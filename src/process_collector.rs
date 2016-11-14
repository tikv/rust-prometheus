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

use proto;
use desc::Desc;
use metrics::{Opts, Collector};
use counter::Counter;
use gauge::Gauge;
use errors::{Error, Result};

/// The `pid_t` data type represents process IDs.
pub use libc::pid_t;

/// `ProcessCollector` a collector which exports the current state of
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

        let cpu_total = Counter::with_opts(Opts::new("process_cpu_seconds_total",
                                                     "Total user and system CPU time spent in \
                                                      seconds.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(cpu_total.desc().into_iter().cloned());

        let open_fds = Gauge::with_opts(Opts::new("process_open_fds",
                                                  "Number of open file descriptors.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(open_fds.desc().into_iter().cloned());

        let max_fds = Gauge::with_opts(Opts::new("process_max_fds",
                                                 "Maximum number of open file descriptors.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(max_fds.desc().into_iter().cloned());

        let vsize = Gauge::with_opts(Opts::new("process_virtual_memory_bytes",
                                               "Virtual memory size in bytes.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(vsize.desc().into_iter().cloned());

        let rss = Gauge::with_opts(Opts::new("process_resident_memory_bytes",
                                             "Resident memory size in bytes.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(rss.desc().into_iter().cloned());

        let start_time = Gauge::with_opts(Opts::new("process_start_time_seconds",
                                                    "Start time of the process since unix epoch \
                                                     in seconds.")
                .namespace(namespace.clone()))
            .unwrap();
        descs.extend(start_time.desc().into_iter().cloned());

        ProcessCollector {
            pid: pid,
            descs: descs,
            cpu_total: Mutex::new(cpu_total),
            open_fds: open_fds,
            max_fds: max_fds,
            vsize: vsize,
            rss: rss,
            start_time: start_time,
        }
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
        if let Ok((vm_size, vm_rss)) = mem_status(self.pid) {
            self.vsize.set(vm_size);
            self.rss.set(vm_rss);
        }

        let pid_stat = pid_info::stat(self.pid).ok();

        // proc_start_time
        if let Some(ref stat) = pid_stat {
            let start_time = (stat.start_time as f64) / *CLK_TCK + BOOT_TIME.unwrap();
            self.start_time.set(start_time);
        }

        // cpu
        let cpu_total_mfs = {
            let cpu_total = self.cpu_total.lock().unwrap();
            if let Some(ref stat) = pid_stat {
                let total = (stat.utime + stat.stime) as f64 / *CLK_TCK;
                let past = cpu_total.get();
                let delta = total - past;
                if delta > 0.0 {
                    cpu_total.inc_by(delta).unwrap();
                }
            }

            cpu_total.collect()
        };

        // proc_start_time
        if let (Some(ref stat), Some(boot_time)) = (pid_stat, *BOOT_TIME) {
            self.start_time.set(stat.start_time as f64 / *CLK_TCK + boot_time);
        }

        // collect MetricFamilys.
        let mut mfs = Vec::new();
        mfs.extend(cpu_total_mfs);
        mfs.extend(self.open_fds.collect());
        mfs.extend(self.max_fds.collect());
        mfs.extend(self.vsize.collect());
        mfs.extend(self.rss.collect());
        mfs.extend(self.start_time.collect());
        mfs
    }
}

impl Default for ProcessCollector {
    /// Returns a `ProcessCollector` of the calling process.
    fn default() -> Self {
        let pid = unsafe { libc::getpid() };
        ProcessCollector::new(pid, "")
    }
}

fn open_fds(pid: pid_t) -> Result<usize> {
    let path = format!("/proc/{}/fd", pid);
    try!(fs::read_dir(path)).fold(Ok(0), |acc, i| {
        let mut acc = try!(acc);
        let ty = try!(try!(i).file_type());
        if !ty.is_dir() {
            acc += 1;
        }

        Ok(acc)
    })
}

// `find_statistic` match lines in pattern pat, it takes the first matching line,
// and parse the first number literal in the matching line.
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
//
// pub for tests.
pub fn find_statistic(all: &str, pat: &str) -> Result<f64> {
    let literal = all.lines()
        .find(|line| line.contains(pat))
        .and_then(|line| {
            (&line[pat.len()..line.len()])
                .split_whitespace()
                .find(|s| !s.is_empty())
        });
    match literal {
        Some(literal) => {
            literal.parse().map_err(|e| Error::Msg(format!("read statistic {} failed: {}", pat, e)))
        }
        None => Err(Error::Msg(format!("read statistic {} failed", pat))),
    }

}

// See more `man 5 proc`, `/proc/[pid]/limits`
const MAX_FD_PATTERN: &'static str = "Max open files";

fn max_fds(pid: pid_t) -> Result<f64> {
    let path = format!("/proc/{}/limits", pid);
    let mut buffer = String::new();
    try!(fs::File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)));
    find_statistic(&buffer, MAX_FD_PATTERN)
}

// 1 KB = 1024 Byte
const KB_TO_BYTE: f64 = 1024.0;

// See more `man 5 proc`, `/proc/[pid]/status`
const VM_SIZE_PATTERN: &'static str = "VmSize:";
const VM_RSS_PATTERN: &'static str = "VmRSS:";

fn mem_status(pid: pid_t) -> Result<(f64, f64)> {
    let path = format!("/proc/{}/status", pid);
    let mut buffer = String::new();
    try!(fs::File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)));

    let vm_size = try!(find_statistic(&buffer, VM_SIZE_PATTERN));
    let vm_rss = try!(find_statistic(&buffer, VM_RSS_PATTERN));

    Ok((vm_size * KB_TO_BYTE, vm_rss * KB_TO_BYTE))
}

lazy_static! {
    static ref CLK_TCK: f64 = {
        unsafe {
            libc::sysconf(libc::_SC_CLK_TCK) as f64
        }
    };
}

// See more `man 5 proc`, `/proc/stat`
const BOOT_TIME_PATTERN: &'static str = "btime";

lazy_static! {
    static ref BOOT_TIME: Option<f64> = {
        let mut buffer = String::new();
        fs::File::open("/proc/stat")
            .and_then(|mut f| f.read_to_string(&mut buffer)).ok().and_then(|_| {
               find_statistic(&buffer, BOOT_TIME_PATTERN).ok()
            })
    };
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use registry;
    use metrics::Collector;

    use super::*;

    #[test]
    fn test_process_collector() {
        let pc = ProcessCollector::default();
        {
            // 6 metrics per process collector.
            let descs = pc.desc();
            assert_eq!(descs.len(), 6);
            let mfs = pc.collect();
            assert_eq!(mfs.len(), 6);
        }

        let r = registry::Registry::new();
        let res = r.register(Box::new(pc));
        assert!(res.is_ok());
    }

    const STATUS_LITERAL: &'static str = r#"Name:	compiz
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

    const VM_RSS: f64 = 112884.0;
    const VM_SIZE: f64 = 1362696.0;

    #[test]
    fn test_find_statistic() {
        let rss = super::find_statistic(STATUS_LITERAL, super::VM_RSS_PATTERN);
        assert!(rss.is_ok());
        assert!((rss.unwrap() - VM_RSS) < EPSILON);

        let rss = find_statistic(STATUS_LITERAL, super::VM_SIZE_PATTERN);
        assert!(rss.is_ok());
        assert!((rss.unwrap() - VM_SIZE) < EPSILON);
    }
}
