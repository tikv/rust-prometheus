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

use std::fs;
use std::path::Path;

use procinfo::pid;
use libc::{self, pid_t};

use proto;
use desc::Desc;
use metrics::{Opts, Collector};
use counter::Counter;
use gauge::Gauge;
use errors::Result;

pub struct ProcessCollector {
    pid: pid_t,
    descs: Vec<Desc>,
    cpu_total: Counter,
    open_fds: Gauge,
    max_fds: Gauge,
    vsize: Gauge,
    rss: Gauge,
    start_time: Gauge,
}

impl ProcessCollector {
    pub fn new<S: Into<String>>(pid: pid_t, namespace: S) -> ProcessCollector {
        let namespace = namespace.into();
        let mut descs = Vec::new();

        let cpu_total = Counter::with_opts(Opts::new("process_cpu_seconds_total",
                                                     "Total user and system CPU time spent in \
                                                      seconds.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(cpu_total.desc().into_iter().cloned());

        let open_fds = Gauge::with_opts(Opts::new("process_open_fds",
                                                  "Number of open file descriptors.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(open_fds.desc().into_iter().cloned());

        let max_fds = Gauge::with_opts(Opts::new("process_max_fds",
                                                 "Maximum number of open file descriptors.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(max_fds.desc().into_iter().cloned());

        let vsize = Gauge::with_opts(Opts::new("process_virtual_memory_bytes",
                                               "Virtual memory size in bytes.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(vsize.desc().into_iter().cloned());

        let rss = Gauge::with_opts(Opts::new("process_resident_memory_bytes",
                                             "Resident memory size in bytes.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(rss.desc().into_iter().cloned());

        let start_time = Gauge::with_opts(Opts::new("process_start_time_seconds",
                                                    "Start time of the process since unix epoch \
                                                     in seconds.")
                .namespace(namespace.as_str()))
            .unwrap();
        descs.extend(start_time.desc().into_iter().cloned());

        ProcessCollector {
            pid: pid,
            descs: descs,
            cpu_total: cpu_total,
            open_fds: open_fds,
            max_fds: max_fds,
            vsize: vsize,
            rss: rss,
            start_time: start_time,
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

        // fds
        if let Ok(num) = open_fds_num(self.pid) {
            self.open_fds.set(num as f64);
        }

        if let Ok(statm) = pid::statm(self.pid) {
            self.vsize.set(statm.size as f64);
            self.rss.set(statm.resident as f64)
        }

        // collect MetricFamilys.
        let mut mfs = Vec::new();
        mfs.extend(self.cpu_total.collect());
        mfs.extend(self.open_fds.collect());
        mfs.extend(self.max_fds.collect());
        mfs.extend(self.vsize.collect());
        mfs.extend(self.rss.collect());
        mfs.extend(self.start_time.collect());
        mfs
    }
}

/// getpid() returns the process ID of the calling process.
pub fn get_pid() -> pid_t {
    unsafe { libc::getpid() }
}

fn open_fds_num(pid: pid_t) -> Result<usize> {
    // FIXME: path
    try!(fs::read_dir("./")).fold(Ok(0), |acc, i| {
        let mut acc = try!(acc);

        if !i.unwrap().file_type().unwrap().is_dir() {
            acc += 1;
        }

        Ok(acc)
    })
}
