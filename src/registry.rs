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
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use proto;
use metrics::Collector;
use errors::{Result, Error};

struct RegistryCore {
    pub colloctors_by_id: HashMap<u64, Box<Collector>>,
    pub dim_hashes_by_name: HashMap<String, u64>,
}

impl RegistryCore {
    fn register(&mut self, c: Box<Collector>) -> Result<()> {
        // TODO: should simplify later.
        let id = {
            let desc = c.desc();

            if let Some(hash) = self.dim_hashes_by_name.get(&desc.fq_name) {
                if *hash != desc.dim_hash {
                    return Err(Error::Msg(format!("a previously registered descriptor with the \
                                                   same fully-qualified name as {:?} has \
                                                   different label names or a different help \
                                                   string",
                                                  desc)));
                }
            }

            if self.colloctors_by_id.contains_key(&desc.id) {
                return Err(Error::AlreadyReg);
            }

            self.dim_hashes_by_name.insert(desc.fq_name.clone(), desc.dim_hash);

            desc.id
        };

        self.colloctors_by_id.insert(id, c);
        Ok(())
    }

    fn unregister(&mut self, c: Box<Collector>) -> Result<()> {
        let desc = c.desc();
        if self.colloctors_by_id.remove(&desc.id).is_none() {
            return Err(Error::Msg(format!("collector {:?} is not registered", desc)));
        }

        // dim_hashes_by_name is left untouched as those must be consistent
        // throughout the lifetime of a program.
        Ok(())
    }

    fn gather(&self) -> Vec<proto::MetricFamily> {
        let mut mf_by_name = HashMap::new();

        for c in self.colloctors_by_id.values() {
            let mut mf = c.collect();
            let name = mf.get_name().to_owned();

            match mf_by_name.entry(name) {
                Entry::Vacant(entry) => {
                    entry.insert(mf);
                }
                Entry::Occupied(mut entry) => {
                    let mut existent_mf = entry.get_mut();
                    let mut existent_metrics = existent_mf.mut_metric();

                    // TODO: check type.
                    // TODO: check consistency.
                    for metric in mf.take_metric().into_iter() {
                        existent_metrics.push(metric);
                    }
                }
            }
        }

        // TODO: metric_family injection hook.
        // TODO: sort metrics.
        mf_by_name.into_iter().map(|(_, m)| m).collect()
    }
}

#[derive(Clone)]
pub struct Registry {
    r: Arc<RwLock<RegistryCore>>,
}

impl Default for Registry {
    fn default() -> Registry {
        let r = RegistryCore {
            colloctors_by_id: HashMap::new(),
            dim_hashes_by_name: HashMap::new(),
        };

        Registry { r: Arc::new(RwLock::new(r)) }
    }
}

impl Registry {
    pub fn new() -> Registry {
        Registry::default()
    }

    pub fn register(&self, c: Box<Collector>) -> Result<()> {
        self.r.write().unwrap().register(c)
    }

    pub fn unregister(&self, c: Box<Collector>) -> Result<()> {
        self.r.write().unwrap().unregister(c)
    }

    pub fn gather(&self) -> Vec<proto::MetricFamily> {
        self.r.read().unwrap().gather()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use counter::Counter;

    use super::*;

    #[test]
    fn test_registry() {
        let r = Registry::new();

        let counter = Counter::new("test", "test help").unwrap();
        r.register(Box::new(counter.clone())).unwrap();
        counter.inc();

        let r1 = r.clone();
        let handler = thread::spawn(move || {
            let metric_familys = r1.gather();
            assert_eq!(metric_familys.len(), 1);
        });

        assert!(r.register(Box::new(counter.clone())).is_err());

        assert!(handler.join().is_ok());

        assert!(r.unregister(Box::new(counter.clone())).is_ok());
        assert!(r.unregister(Box::new(counter.clone())).is_err());
    }
}
