// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

Use metric enums to reuse possible values of a label.

*/
#[macro_use]
extern crate lazy_static;
extern crate coarsetime;
extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus::*;
use prometheus_static_metric::auto_flush_from;
use prometheus_static_metric::make_auto_flush_static_metric;

make_auto_flush_static_metric! {


    pub label_enum FooBar {
        foo,
        bar,
    }

    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub struct Lhrs: LocalIntCounter {
        "product" => FooBar,
        "method" => Methods,
        "version" => {
            http1: "HTTP/1",
            http2: "HTTP/2",
        },
    }
}

lazy_static! {
pub static ref HTTP_COUNTER_VEC: IntCounterVec =
register_int_counter_vec ! (
"http_requests",
"Total number of HTTP requests.",
& ["product", "method", "version"]    // it doesn't matter for the label order
).unwrap();
}

// Macro expanded code of auto_flush_from!
// lazy_static! {
//     pub static ref TLS_HTTP_COUNTER: Lhrs = {
//         thread_local! {
//             pub static TLS_HTTP_COUNTER_INNER: LhrsInner = LhrsInner::from(& HTTP_COUNTER_VEC);
//         }
//         Lhrs::from(&TLS_HTTP_COUNTER_INNER)
//     };
// }

auto_flush_from! {
    TLS_HTTP_COUNTER : Lhrs = HTTP_COUNTER_VEC
}

fn main() {
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    TLS_HTTP_COUNTER.foo.post.http1.inc();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    TLS_HTTP_COUNTER.foo.post.http1.inc();
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        3
    );
}

/*

/// Pseudo macro expanded code of make_auto_flush_static_metric.rs
#[macro_use]
extern crate lazy_static;
extern crate coarsetime;
extern crate prometheus;
extern crate prometheus_static_metric;

use std::cell::Cell;

use coarsetime::Instant;
#[allow(unused_imports)]
use prometheus::local::*;
use prometheus::*;
use std::collections::HashMap;
use std::mem;
use std::mem::MaybeUninit;
use std::thread::LocalKey;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum Methods {
    post,
    get,
    put,
    delete,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum FooBar {
    foo,
    bar,
}

#[allow(missing_copy_implementations)]
pub struct LhrsInner {
    pub foo: LhrsInner2,
    pub bar: LhrsInner2,
    last_flush: Cell<Instant>,
}

#[allow(missing_copy_implementations)]
pub struct LhrsInner2 {
    pub post: LhrsInner3,
    pub get: LhrsInner3,
    pub put: LhrsInner3,
    pub delete: LhrsInner3,
}

#[allow(missing_copy_implementations)]
pub struct LhrsInner3 {
    pub http1: LocalIntCounter,
    pub http2: LocalIntCounter,
}

pub struct LhrsDelegator {
    pub post: LhrsDelegator2,
    pub get: LhrsDelegator2,
    pub put: LhrsDelegator2,
    pub delete: LhrsDelegator2,
}

pub struct LhrsDelegator2 {
    pub http1: AFLocalCounter<LhrsInner, LocalIntCounter, LhrsDelegator3>,
    pub http2: AFLocalCounter<LhrsInner, LocalIntCounter, LhrsDelegator3>,
}

pub struct LhrsDelegator3 {
    root: &'static LocalKey<LhrsInner>,
    offset1: usize,
    offset2: usize,
    offset3: usize,
}

impl LhrsInner {
    pub fn from(m: &IntCounterVec) -> LhrsInner {
        LhrsInner {
            foo: LhrsInner2::from("foo", m),
            bar: LhrsInner2::from("bar", m),
            last_flush: Cell::new(Instant::now()),
        }
    }

    pub fn flush(&self) {
        self.foo.flush();
        self.bar.flush();
    }
}

impl LhrsInner2 {
    pub fn from(label_0: &str, m: &IntCounterVec) -> LhrsInner2 {
        LhrsInner2 {
            post: LhrsInner3::from(label_0, "post", m),
            get: LhrsInner3::from(label_0, "get", m),
            put: LhrsInner3::from(label_0, "put", m),
            delete: LhrsInner3::from(label_0, "delete", m),
        }
    }

    pub fn flush(&self) {
        self.post.flush();
        self.get.flush();
        self.put.flush();
        self.delete.flush();
    }
}

impl LhrsInner3 {
    pub fn from(label_0: &str, label_1: &str, m: &IntCounterVec) -> LhrsInner3 {
        LhrsInner3 {
            http1: m
                .with(&{
                    let mut coll = HashMap::new();
                    coll.insert("product", label_0);
                    coll.insert("method", label_1);
                    coll.insert("version", "HTTP/1");
                    coll
                })
                .local(),
            http2: m
                .with(&{
                    let mut coll = HashMap::new();
                    coll.insert("product", label_0);
                    coll.insert("method", label_1);
                    coll.insert("version", "HTTP/2");
                    coll
                })
                .local(),
        }
    }

    pub fn flush(&self) {
        self.http1.flush();
        self.http2.flush();
    }
}

impl ::prometheus::local::LocalMetric for LhrsInner {
    fn flush(&self) {
        LhrsInner::flush(self);
    }
}

impl ::prometheus::local::MayFlush for LhrsInner {
    fn may_flush(&self) {
        MayFlush::try_flush(self, &self.last_flush, 1.0)
    }
}

impl LhrsDelegator {
    fn new(root: &'static LocalKey<LhrsInner>, offset: usize) -> LhrsDelegator {
        let x = unsafe { MaybeUninit::<LhrsInner2>::uninit().assume_init() };
        let branch_offset = &x as *const LhrsInner2 as usize;
        let post = LhrsDelegator2::new(
            root,
            offset,
            &(x.post) as *const LhrsInner3 as usize - branch_offset,
        );
        let get = LhrsDelegator2::new(
            root,
            offset,
            &(x.get) as *const LhrsInner3 as usize - branch_offset,
        );
        let put = LhrsDelegator2::new(
            root,
            offset,
            &(x.put) as *const LhrsInner3 as usize - branch_offset,
        );
        let delete = LhrsDelegator2::new(
            root,
            offset,
            &(x.delete) as *const LhrsInner3 as usize - branch_offset,
        );
        mem::forget(x);
        LhrsDelegator {
            post,
            get,
            put,
            delete,
        }
    }

    pub fn get(&self, value: Methods) -> &LhrsDelegator2 {
        match value {
            Methods::post => &self.post,
            Methods::get => &self.get,
            Methods::put => &self.put,
            Methods::delete => &self.delete,
        }
    }
}

impl LhrsDelegator2 {
    fn new(root: &'static LocalKey<LhrsInner>, offset1: usize, offset2: usize) -> LhrsDelegator2 {
        let x = unsafe { MaybeUninit::<LhrsInner3>::uninit().assume_init() };
        let branch_offset = (&x as *const LhrsInner3) as usize;
        let http1 = LhrsDelegator3::new(
            root,
            offset1,
            offset2,
            &(x.http1) as *const LocalIntCounter as usize - branch_offset,
        );
        let http2 = LhrsDelegator3::new(
            root,
            offset1,
            offset2,
            &(x.http2) as *const LocalIntCounter as usize - branch_offset,
        );
        mem::forget(x);
        LhrsDelegator2 { http1, http2 }
    }
}

impl LhrsDelegator3 {
    fn new(
        root: &'static LocalKey<LhrsInner>,
        offset1: usize,
        offset2: usize,
        offset3: usize,
    ) -> AFLocalCounter<LhrsInner, LocalIntCounter, LhrsDelegator3> {
        let delegator = LhrsDelegator3 {
            root,
            offset1,
            offset2,
            offset3,
        };

        AFLocalCounter::new(delegator)
    }
}

impl CounterDelegator<LhrsInner, LocalIntCounter> for LhrsDelegator3 {
    fn get_root_metric(&self) -> &'static LocalKey<LhrsInner> {
        self.root
    }

    fn get_local<'a>(&self, root_metric: &'a LhrsInner) -> &'a LocalIntCounter {
        unsafe {
            let inner1 = root_metric as *const LhrsInner;
            let inner2 = (inner1 as usize + self.offset1) as *const LhrsInner2;
            let inner3 = (inner2 as usize + self.offset2) as *const LhrsInner3;
            let counter = (inner3 as usize + self.offset3) as *const LocalIntCounter;
            &*counter
        }
    }
}

pub struct Lhrs {
    inner: &'static LocalKey<LhrsInner>,
    pub foo: LhrsDelegator,
    pub bar: LhrsDelegator,
}

impl Lhrs {
    pub fn from(inner: &'static LocalKey<LhrsInner>) -> Lhrs {
        let x = unsafe { MaybeUninit::<LhrsInner>::uninit().assume_init() };
        let branch_offset = &x as *const LhrsInner as usize;
        let foo = LhrsDelegator::new(
            &inner,
            &(x.foo) as *const LhrsInner2 as usize - branch_offset,
        );
        let bar = LhrsDelegator::new(
            &inner,
            &(x.bar) as *const LhrsInner2 as usize - branch_offset,
        );
        mem::forget(x);
        Lhrs { inner, foo, bar }
    }

    pub fn try_get(&self, value: &str) -> Option<&LhrsDelegator> {
        match value {
            "foo" => Some(&self.foo),
            "bar" => Some(&self.bar),
            _ => None,
        }
    }

    pub fn flush(&self) {
        self.inner.with(|m| m.flush())
    }
}

lazy_static! {
pub static ref HTTP_COUNTER_VEC: IntCounterVec =
register_int_counter_vec ! (
"http_requests",
"Total number of HTTP requests.",
& ["product", "method", "version"]    // it doesn't matter for the label order
).unwrap();
}

thread_local! {
pub static TLS_HTTP_COUNTER_INNER: LhrsInner = LhrsInner::from(& HTTP_COUNTER_VEC);
}

lazy_static! {
    pub static ref TLS_HTTP_COUNTER: Lhrs = Lhrs::from(&TLS_HTTP_COUNTER_INNER);
}

fn main() {
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    TLS_HTTP_COUNTER.foo.post.http1.inc();

    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        0
    );

    ::std::thread::sleep(::std::time::Duration::from_secs(2));

    TLS_HTTP_COUNTER.foo.post.http1.inc();
    assert_eq!(
        HTTP_COUNTER_VEC
            .with_label_values(&["foo", "post", "HTTP/1"])
            .get(),
        3
    );
}

*/
