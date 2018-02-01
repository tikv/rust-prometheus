// Copyright 2014 The Prometheus Authors
// Copyright 2018 PingCAP, Inc.
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

use errors::{Result, Error};
use value::{Value, ValueType};
use metrics::{Collector, Metric, Opts};
use desc::Desc;
use proto;
use super::*;

/// `Counter` is a Metric that represents a single numerical value that only ever
/// goes up.
#[derive(Clone)]
pub struct Counter {
    v: Arc<Value>,
}

impl Counter {
    /// `new` creates a `Counter` with the `name` and `help` arguments.
    pub fn new<S: Into<String>>(name: S, help: S) -> Result<Counter> {
        let opts = Opts::new(name, help);
        Counter::with_opts(opts)
    }

    /// `with_opts` creates a `Counter` with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Counter> {
        Counter::with_opts_and_label_values(&opts, &[])
    }

    fn with_opts_and_label_values(opts: &Opts, label_values: &[&str]) -> Result<Counter> {
        let v = Value::new(opts, ValueType::Counter, 0.0, label_values)?;
        Ok(Counter { v: Arc::new(v) })
    }

    pub fn local(&self) -> LocalCounter {
        LocalCounter::new(self.clone())
    }
}

impl ICounter for Counter {
    #[inline]
    fn try_inc_by(&mut self, v: f64) -> Result<()> {
        if v < 0.0 {
            return Err(Error::DecreaseCounter(v));
        }
        self.v.inc_by(v);
        Ok(())
    }

    fn get(&self) -> f64 {
        self.v.get()
    }

    fn reset(&mut self) {
        panic("Non-local counter cannot be reset");
    }
}

impl Collector for Counter {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
    }
}

impl Metric for Counter {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}
