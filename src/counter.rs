// Copyright 2014 The Prometheus Authors
// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::{Arc, RwLock};

use proto;
use metrics::{Metric, Opts};
use value::{Value, ValueType};
use desc::Desc;
use errors::Result;

/// `Counter` is a Metric that represents a single numerical value that only ever
/// goes up.
#[derive(Clone)]
pub struct Counter {
    desc: Arc<Desc>,
    v: Arc<RwLock<Value>>,
}

impl Counter {
    pub fn new<S: Into<String>>(namespace: S, sub_system: S, name: S, help: S) -> Result<Counter> {
        let opts = Opts::new(namespace, sub_system, name, help);
        Counter::with_opts(opts)
    }

    pub fn with_opts(opts: Opts) -> Result<Counter> {
        let desc = try!(Desc::new(opts.fq_name(), opts.help, vec![], opts.const_labels));
        let v = try!(Value::new(&desc, ValueType::Counter, 0.0, vec![]));
        Ok(Counter {
            desc: Arc::new(desc),
            v: Arc::new(RwLock::new(v)),
        })
    }
}

impl Counter {
    /// `add` adds the given value to the counter. It panics if the value is <
    /// 0.
    #[inline]
    pub fn add(&mut self, v: f64) {
        assert!(v >= 0.0, "counter cannot decrease in value");
        self.v.write().unwrap().add(v)
    }

    /// `inc` increments the counter by 1.
    #[inline]
    pub fn inc(&mut self) {
        self.add(1.0)
    }

    /// `value` returns the counter value.
    #[inline]
    pub fn value(&self) -> f64 {
        self.v.read().unwrap().val
    }
}

impl Metric for Counter {
    fn desc(&self) -> &Desc {
        &self.desc
    }

    fn metric(&self) -> proto::Metric {
        self.v.read().unwrap().metric()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    use metrics::{Opts, Metric};

    #[test]
    fn test_counter() {
        let mut const_labels = HashMap::new();
        const_labels.insert("a".to_owned(), "1".to_owned());
        const_labels.insert("b".to_owned(), "2".to_owned());
        let opts = Opts::with_label("", "", "test", "test help", const_labels);
        let mut counter = Counter::with_opts(opts).unwrap();
        counter.inc();
        assert_eq!(counter.value() as u64, 1);
        counter.add(42.0);
        assert_eq!(counter.value() as u64, 43);

        let m = counter.metric();
        assert_eq!(m.get_label().len(), 2);
        assert_eq!(m.get_counter().get_value() as u64, 43);
    }
}
