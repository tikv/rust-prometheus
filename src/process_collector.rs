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

//! Monitor a process.
//!
//! This module only supports **Linux** platform.

use std::fs;
use std::io::Read;
use std::sync::Mutex;

use libc;
use procinfo::pid as pid_info;

use crate::counter::Counter;
use crate::desc::Desc;
use crate::errors::{Error, Result};
use crate::gauge::Gauge;
use crate::metrics::{Collector, Opts};
use crate::proto;

/// The `pid_t` data type represents process IDs.
pub use libc::pid_t;

// Six metrics per ProcessCollector.
const MERTICS_NUMBER: usize = 6;

/// A collector which exports the current state of
/// process metrics including cpu, memory and file descriptor usage as well as
/// the process start time for the given process id.
pub struct ProcessCollector {
    pid: pid_t,
    descs: Vec<Desc>,
    cpu_total: Mutex<Counter>,
    open_fds: Gauge,
    max_fds: Gauge,
    vsize: Gauge,
    rss: Gauge,
    start_time: Gauge,
}

impl ProcessCollector {
    /// Create a `ProcessCollector` with the given process id and namespace.
    pub fn new<S: Into<String>>(pid: pid_t, namespace: S) -> ProcessCollector {
        let namespace = namespace.into();
        let mut descs = Vec::new();

        let cpu_total = Counter::with_opts(
            Opts::new(
                "process_cpu_seconds_total",
                "Total user and system CPU time spent in \
                 seconds.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(cpu_total.desc().into_iter().cloned());

        let open_fds = Gauge::with_opts(
            Opts::new("process_open_fds", "Number of open file descriptors.")
                .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(open_fds.desc().into_iter().cloned());

        let max_fds = Gauge::with_opts(
            Opts::new(
                "process_max_fds",
                "Maximum number of open file descriptors.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(max_fds.desc().into_iter().cloned());

        let vsize = Gauge::with_opts(
            Opts::new(
                "process_virtual_memory_bytes",
                "Virtual memory size in bytes.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(vsize.desc().into_iter().cloned());

        let rss = Gauge::with_opts(
            Opts::new(
                "process_resident_memory_bytes",
                "Resident memory size in bytes.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(rss.desc().into_iter().cloned());

        let start_time = Gauge::with_opts(
            Opts::new(
                "process_start_time_seconds",
                "Start time of the process since unix epoch \
                 in seconds.",
            )
            .namespace(namespace.clone()),
        )
        .unwrap();
        descs.extend(start_time.desc().into_iter().cloned());

        ProcessCollector {
            pid,
            descs,
            cpu_total: Mutex::new(cpu_total),
            open_fds,
            max_fds,
            vsize,
            rss,
            start_time,
        }
    }

    /// Return a `ProcessCollector` of the calling process.
    pub fn for_self() -> ProcessCollector {
        let pid = unsafe { libc::getpid() };
        ProcessCollector::new(pid, "")
    }
}

impl Collector for ProcessCollector {
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        // file descriptors
        if let Ok(num) = open_fds(self.pid) {
            self.open_fds.set(num as f64);
        }
        if let Ok(max) = max_fds(self.pid) {
            self.max_fds.set(max)
        }

        // memory
        if let Ok(statm) = pid_info::statm(self.pid) {
            self.vsize.set(statm.size as f64 * *PAGESIZE);
            self.rss.set(statm.resident as f64 * *PAGESIZE);
        }

        let pid_stat = pid_info::stat(self.pid);

        // proc_start_time
        if let (&Ok(ref stat), Some(boot_time)) = (&pid_stat, *BOOT_TIME) {
            self.start_time
                .set(stat.start_time as f64 / *CLK_TCK + boot_time);
        }

        // cpu
        let cpu_total_mfs = {
            let cpu_total = self.cpu_total.lock().unwrap();
            if let Ok(stat) = pid_stat {
                let total = (stat.utime + stat.stime) as f64 / *CLK_TCK;
                let past = cpu_total.get();
                let delta = total - past;
                if delta > 0.0 {
                    cpu_total.inc_by(delta);
                }
            }

            cpu_total.collect()
        };

        // collect MetricFamilys.
        let mut mfs = Vec::with_capacity(MERTICS_NUMBER);
        mfs.extend(cpu_total_mfs);
        mfs.extend(self.open_fds.collect());
        mfs.extend(self.max_fds.collect());
        mfs.extend(self.vsize.collect());
        mfs.extend(self.rss.collect());
        mfs.extend(self.start_time.collect());
        mfs
    }
}

fn open_fds(pid: pid_t) -> Result<usize> {
    let path = format!("/proc/{}/fd", pid);
    fs::read_dir(path)?.fold(Ok(0), |acc, i| {
        let mut acc = acc?;
        let ty = i?.file_type()?;
        if !ty.is_dir() {
            acc += 1;
        }

        Ok(acc)
    })
}

// `find_statistic` matches lines in pattern pat, it takes the first matching line,
// and parses the first number literal in the matching line.
//
// Example:
//  * all:
// ```
// ctxt 789298306
// btime 1477460662
// processes 302136
// procs_running 1
// procs_blocked 0
// ```
//  * pat: "btime"
//
// then it returns `Ok(1477460662.0)`
fn find_statistic(all: &str, pat: &str) -> Result<f64> {
    if let Some(idx) = all.find(pat) {
        let mut iter = (all[idx + pat.len()..]).split_whitespace();
        if let Some(v) = iter.next() {
            return v
                .parse()
                .map_err(|e| Error::Msg(format!("read statistic {} failed: {}", pat, e)));
        }
    }

    Err(Error::Msg(format!("read statistic {} failed", pat)))
}

const MAXFD_PATTERN: &str = "Max open files";

fn max_fds(pid: pid_t) -> Result<f64> {
    let mut buffer = String::new();
    fs::File::open(&format!("/proc/{}/limits", pid))
        .and_then(|mut f| f.read_to_string(&mut buffer))?;

    find_statistic(&buffer, MAXFD_PATTERN)
}

lazy_static! {
    // getconf CLK_TCK
    static ref CLK_TCK: f64 = {
        unsafe {
            libc::sysconf(libc::_SC_CLK_TCK) as f64
        }
    };

    // getconf PAGESIZE
    static ref PAGESIZE: f64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as f64
        }
    };
}

// See more `man 5 proc`, `/proc/stat`
const BOOT_TIME_PATTERN: &str = "btime";

lazy_static! {
    static ref BOOT_TIME: Option<f64> = {
        let mut buffer = String::new();
        fs::File::open("/proc/stat")
            .and_then(|mut f| f.read_to_string(&mut buffer))
            .ok()
            .and_then(|_| find_statistic(&buffer, BOOT_TIME_PATTERN).ok())
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::metrics::Collector;
    use crate::registry;
    use std::f64::EPSILON;

    #[test]
    fn test_process_collector() {
        let pc = ProcessCollector::for_self();
        {
            // Six metrics per process collector.
            let descs = pc.desc();
            assert_eq!(descs.len(), super::MERTICS_NUMBER);
            let mfs = pc.collect();
            assert_eq!(mfs.len(), super::MERTICS_NUMBER);
        }

        let r = registry::Registry::new();
        let res = r.register(Box::new(pc));
        assert!(res.is_ok());
    }

    const STATUS_LITERAL: &str = r#"Name:	compiz
State:	S (sleeping)
Tgid:	3124
Ngid:	0
Pid:	3124
PPid:	3038
TracerPid:	0
Uid:	1000	1000	1000	1000
Gid:	1000	1000	1000	1000
FDSize:	64
Groups:	4 24 27 30 46 108 124 126 999 1000
NStgid:	3124
NSpid:	3124
NSpgid:	3038
NSsid:	3038
VmPeak:	 1452388 kB
VmSize:	 1362696 kB
VmLck:	       0 kB
VmPin:	       0 kB
VmHWM:	  134316 kB
VmRSS:	  112884 kB
VmData:	  780020 kB
VmStk:	     152 kB
VmExe:	      12 kB
VmLib:	   77504 kB
VmPTE:	    1116 kB
VmPMD:	      16 kB
VmSwap:	       0 kB
HugetlbPages:	       0 kB
Threads:	8
SigQ:	0/31457
SigPnd:	0000000000000000
ShdPnd:	0000000000000000
SigBlk:	0000000000000000
SigIgn:	0000000000001000
SigCgt:	0000000180014003
CapInh:	0000000000000000
CapPrm:	0000000000000000
CapEff:	0000000000000000
CapBnd:	0000003fffffffff
CapAmb:	0000000000000000
Seccomp:	0
Cpus_allowed:	f
Cpus_allowed_list:	0-3
Mems_allowed:	00000000,00000001
Mems_allowed_list:	0
voluntary_ctxt_switches:	1713183
nonvoluntary_ctxt_switches:	68606
"#;

    const VM_SIZE_PATTERN: &str = "VmSize:";
    const VM_RSS_PATTERN: &str = "VmRSS:";
    const VM_RSS: f64 = 112884.0;
    const VM_SIZE: f64 = 1362696.0;

    const LIMITS_LITERAL: &str = r#"
Limit                     Soft Limit           Hard Limit           Units
Max cpu time              unlimited            unlimited            seconds
Max file size             unlimited            unlimited            bytes
Max data size             unlimited            unlimited            bytes
Max stack size            8388608              unlimited            bytes
Max core file size        0                    unlimited            bytes
Max resident set          unlimited            unlimited            bytes
Max processes             31454                31454                processes
Max open files            1024                 4096                 files
Max locked memory         65536                65536                bytes
Max address space         unlimited            unlimited            bytes
Max file locks            unlimited            unlimited            locks
Max pending signals       31454                31454                signals
Max msgqueue size         819200               819200               bytes
Max nice priority         0                    0
Max realtime priority     0                    0
Max realtime timeout      unlimited            unlimited            us "#;

    const MAXFD: f64 = 1024.0;

    #[test]
    fn test_find_statistic() {
        let rss = super::find_statistic(STATUS_LITERAL, VM_RSS_PATTERN);
        assert!(rss.is_ok());
        assert!((rss.unwrap() - VM_RSS) < EPSILON);

        let size = super::find_statistic(STATUS_LITERAL, VM_SIZE_PATTERN);
        assert!(size.is_ok());
        assert!((size.unwrap() - VM_SIZE) < EPSILON);

        let maxfd = super::find_statistic(LIMITS_LITERAL, super::MAXFD_PATTERN).unwrap();
        assert!((maxfd - MAXFD) < EPSILON);
    }
}
