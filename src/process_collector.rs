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

use procinfo::pid;
use libc::{self, pid_t};

use proto;
use errors::Result;
use desc::Desc;
use metrics::{Opts, Collector};
use counter::{Counter, CounterVec};
use gauge::{Gauge, GaugeVec};
use registry::Registry;

pub struct ProcessCollector {
    descs: Vec<Desc>,
    counters: Vec<Counter>,
    gauges: Vec<Gauge>,
}

impl ProcessCollector {
    pub fn new<S: Into<String>>(pid: pid_t, namespace: S) -> ProcessCollector {
        let namespace = namespace.into();
        let mut counters = Vec::new();
        let mut gauges = Vec::new();

        let cpu_total = Counter::with_opts(Opts::new("process_cpu_seconds_total",
                                                     "Total user and system CPU time spent in \
                                                      seconds.")
                .namespace(namespace.as_str()))
            .unwrap();
        counters.push(cpu_total);

        let open_fds = Gauge::with_opts(Opts::new("process_open_fds",
                                                  "Number of open file descriptors.")
                .namespace(namespace.as_str()))
            .unwrap();
        gauges.push(open_fds);

        let max_fds = Gauge::with_opts(Opts::new("process_max_fds",
                                                 "Maximum number of open file descriptors.")
                .namespace(namespace.as_str()))
            .unwrap();
        gauges.push(max_fds);

        let vsize = Gauge::with_opts(Opts::new("process_virtual_memory_bytes",
                                               "Virtual memory size in bytes.")
                .namespace(namespace.as_str()))
            .unwrap();
        gauges.push(vsize);

        let rss = Gauge::with_opts(Opts::new("process_resident_memory_bytes",
                                             "Resident memory size in bytes.")
                .namespace(namespace.as_str()))
            .unwrap();
        gauges.push(rss);

        let start_time = Gauge::with_opts(Opts::new("process_start_time_seconds",
                                                    "Start time of the process since unix epoch \
                                                     in seconds.")
                .namespace(namespace.as_str()))
            .unwrap();
        gauges.push(start_time);

        let mut descs = Vec::new();
        for c in &counters {
            for d in c.desc().into_iter().cloned() {
                descs.push(d);
            }
        }

        for g in &gauges {
            for d in g.desc().into_iter().cloned() {
                descs.push(d);
            }
        }

        ProcessCollector {
            descs: descs,
            counters: counters,
            gauges: gauges,
        }
    }
}

impl Collector for ProcessCollector {
    /// `desc` returns descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    /// `collect` collects metrics.
    fn collect(&self) -> Vec<proto::MetricFamily> {
        // FIXME: collect process info.
        unimplemented!();

        // collect MetricFamilys.
        let mut mfs = Vec::new();
        for c in &self.counters {
            for mf in c.collect() {
                mfs.push(mf.clone());
            }
        }

        for g in &self.gauges {
            for mf in g.collect() {
                mfs.push(mf.clone());
            }
        }

        mfs
    }
}

/// getpid() returns the process ID of the calling process.
pub fn get_pid() -> pid_t {
    unsafe { libc::getpid() }
}
