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
use std::io::Read;
use std::sync::Mutex;

use procinfo::pid as Pid;
use libc::{self, pid_t};

use proto;
use desc::Desc;
use metrics::{Opts, Collector};
use counter::Counter;
use gauge::Gauge;
use errors::{Error, Result};

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
    /// `desc` returns descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    /// `collect` collects metrics.
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

        // cpu
        let cpu_total_mfs = {
            let cpu_total = self.cpu_total.lock().unwrap();
            if let Ok(total) = total_cpu_sec(self.pid) {
                let past = cpu_total.get();
                let delta = total - past;
                if delta > 0.0 {
                    cpu_total.inc_by(delta).unwrap();
                }
            }

            cpu_total.collect()
        };

        // start time
        if let Ok(start_time) = proc_start_time(self.pid) {
            self.start_time.set(start_time);
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

/// getpid() returns the process ID of the calling process.
pub fn get_pid() -> pid_t {
    unsafe { libc::getpid() }
}

fn open_fds(pid: pid_t) -> Result<usize> {
    // FIXME: linux only?
    let path = format!("/proc/{}/fd", pid);
    try!(fs::read_dir(path)).fold(Ok(0), |acc, i| {
        let mut acc = try!(acc);

        if !i.unwrap().file_type().unwrap().is_dir() {
            acc += 1;
        }

        Ok(acc)
    })
}

fn find_in_lines<'a>(all: &'a str, pat: &str) -> Option<&'a str> {
    all.lines()
        .find(|line| line.contains(pat))
        .and_then(|line| Some(&line[pat.len()..line.len()]))
}

const MAX_FD_PATTERN: &'static str = "Max open files";

fn max_fds(pid: pid_t) -> Result<f64> {
    let path = format!("/proc/{}/limits", pid);
    let mut buffer = String::new();
    try!(fs::File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)));
    buffer.lines()
        .find(|line| line.contains(MAX_FD_PATTERN))
        .and_then(|line| {
            (&line[MAX_FD_PATTERN.len()..line.len()])
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .next()
                .and_then(|max| max.parse().ok())
        })
        .ok_or(Error::Msg("read max open files failed".to_owned()))
}

const VM_SIZE_PATTERN: &'static str = "VmSize:";
const VM_RSS_PATTERN: &'static str = "VmRSS:";
const KB_TO_BYTE: f64 = 1024.0; // 1 KB = 1024 Byte

fn mem_status(pid: pid_t) -> Result<(f64, f64)> {
    let path = format!("/proc/{}/status", pid);
    let mut buffer = String::new();
    try!(fs::File::open(path).and_then(|mut f| f.read_to_string(&mut buffer)));

    let vm_size: f64 = try!(buffer.lines()
        .find(|line| line.contains(VM_SIZE_PATTERN))
        .and_then(|line| {
            (&line[VM_SIZE_PATTERN.len()..line.len()])
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .next()
                .and_then(|size| size.parse().ok())
        })
        .ok_or(Error::Msg("read virtual memory size failed".to_owned())));

    let vm_rss: f64 = try!(buffer.lines()
        .find(|line| line.contains(VM_RSS_PATTERN))
        .and_then(|line| {
            (&line[VM_RSS_PATTERN.len()..line.len()])
                .split(char::is_whitespace)
                .filter(|s| !s.is_empty())
                .next()
                .and_then(|size| size.parse().ok())
        })
        .ok_or(Error::Msg("read resident set size failed".to_owned())));

    Ok((vm_size * KB_TO_BYTE, vm_rss * KB_TO_BYTE))
}

// This module only supports linux platform. It is safe to
// retrieve USER_HZ value via a sysconf call.
lazy_static! {
    static ref CLK_TCK: f64 = {
        unsafe {
            libc::sysconf(libc::_SC_CLK_TCK) as f64
        }
    };
}

fn total_cpu_sec(pid: pid_t) -> Result<f64> {
    let stat = try!(Pid::stat(pid));
    Ok((stat.utime + stat.stime) as f64 / *CLK_TCK)
}


const BOOT_TIME_PATTERN: &'static str = "btime";

lazy_static! {
    static ref BOOT_TIME: Option<f64> = {
        unsafe {
            let mut info = ::std::mem::zeroed();
            match libc::sysinfo(&mut info) {
                0 => {
                    let mut buffer = String::new();
                    if let Err(_) = fs::File::open("/proc/stat")
                                            .and_then(|mut f| f.read_to_string(&mut buffer))
                    {
                        return None;
                    }
                    buffer.lines()
                        .find(|line| line.contains(BOOT_TIME_PATTERN))
                        .and_then(|line| {
                            (&line[BOOT_TIME_PATTERN.len()..line.len()])
                                .split(char::is_whitespace)
                                .filter(|s| !s.is_empty())
                                .next()
                                .and_then(|time| time.parse().ok())
                        })
                }
                _ => None,
            }
        }
    };
}

fn proc_start_time(pid: pid_t) -> Result<f64> {
    let stat = try!(Pid::stat(pid));
    Ok((stat.start_time as f64) / *CLK_TCK + BOOT_TIME.unwrap())
}
