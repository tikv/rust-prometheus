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

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::io::Write;

use metrics::Metric;
use errors::{Result, Error};

struct RegistryCore {
    pub metrics_by_id: HashMap<u64, Box<Metric>>,
    pub desc_ids: HashSet<u64>,
    pub dim_hashes_by_name: HashMap<String, u64>,
}

impl RegistryCore {
    pub fn register(&mut self, metric: Box<Metric>) -> Result<()> {
        // TODO: should simplify later.
        let id = {
            let desc = metric.desc();
            if self.desc_ids.contains(&desc.id) {
                return Err(Error::Msg(format!("descriptor {:?} already exists with the same \
                                               fully-qualified name and const label values",
                                              desc)));
            }

            if let Some(hash) = self.dim_hashes_by_name.get(&desc.fq_name) {
                if *hash != desc.dim_hash {
                    return Err(Error::Msg(format!("a previously registered descriptor with the \
                                                   same fully-qualified name as {:?} has \
                                                   different label names or a different help \
                                                   string",
                                                  desc)));
                }
            }

            if self.metrics_by_id.contains_key(&desc.id) {
                return Err(Error::AlreadyReg);
            }

            self.desc_ids.insert(desc.id);
            self.dim_hashes_by_name.insert(desc.fq_name.clone(), desc.dim_hash);

            desc.id
        };

        self.metrics_by_id.insert(id, metric);
        Ok(())
    }
}

#[derive(Clone)]
pub struct Registry {
    r: Arc<RwLock<RegistryCore>>,
}

impl Registry {
    pub fn new() -> Registry {
        let r = RegistryCore {
            metrics_by_id: HashMap::new(),
            desc_ids: HashSet::new(),
            dim_hashes_by_name: HashMap::new(),
        };

        Registry { r: Arc::new(RwLock::new(r)) }
    }

    pub fn register(&mut self, metric: Box<Metric>) -> Result<()> {
        self.r.write().unwrap().register(metric)
    }

    pub fn write_pb<T: Write>(&self, _: &mut T) -> Result<()> {
        Ok(())
    }

    pub fn write_test<T: Write>(&self, _: &mut T) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;
    use counter::Counter;

    #[test]
    fn test_registry() {
        let mut r = Registry::new();

        let r1 = r.clone();
        thread::spawn(move || {
            let mut w = vec![];
            r1.write_pb(&mut w).unwrap();
        });

        let counter = Counter::new("", "", "test", "test help").unwrap();

        r.register(Box::new(counter.clone())).unwrap();
        counter.inc();

        assert!(r.register(Box::new(counter.clone())).is_err());
    }
}
