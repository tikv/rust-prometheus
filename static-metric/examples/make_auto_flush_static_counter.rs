// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!

Use metric enums to reuse possible values of a label.

*/
#[macro_use]
extern crate lazy_static;
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
//

lazy_static! {
    // You can also use default flush duration which is 1 second.
    // pub static ref TLS_HTTP_COUNTER: Lhrs = auto_flush_from!(HTTP_COUNTER_VEC, Lhrs);
    pub static ref TLS_HTTP_COUNTER: Lhrs = auto_flush_from!(HTTP_COUNTER_VEC, Lhrs, std::time::Duration::from_secs(1));
}

fn main() {
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    TLS_HTTP_COUNTER.foo.post.http1.inc();
    //Non-static call
    TLS_HTTP_COUNTER
        .get(FooBar::foo)
        .get(Methods::post)
        .http1
        .inc();

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
        4
    );
}

/*

/// Pseudo macro expanded code of make_auto_flush_static_counter
#[macro_use]
extern crate lazy_static;
extern crate prometheus;
extern crate prometheus_static_metric;

use std::cell::Cell;

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

use ::std::collections::HashMap;
use ::prometheus::*;
use ::prometheus::local::*;
use ::std::cell::Cell;
use ::std::thread::LocalKey;
use std::mem;
use std::mem::MaybeUninit;
#[allow(unused_imports)]
use super::*;
#[allow(missing_copy_implementations)]
pub struct LhrsInner {
    pub foo: Lhrs2Inner,
    pub bar: Lhrs2Inner,
    last_flush: Cell<u64>,
    flush_millis: u64,
}
impl LhrsInner {
    pub fn from(m: &IntCounterVec) -> LhrsInner {
        LhrsInner {
            foo: Lhrs2Inner::from("foo", m),
            bar: Lhrs2Inner::from("bar", m),
            last_flush: Cell::new(timer::now_millis()),
            flush_millis: 1000,
        }
    }
    pub fn flush(&self) {
        self.foo.flush();
        self.bar.flush();
    }
    pub fn with_flush_duration(mut self, duration: Duration) -> Self {
        self.flush_duration = prometheus::timer::duration_to_millis(duration);
        self
    }
}
impl ::prometheus::local::LocalMetric for LhrsInner {
    fn flush(&self) {
        LhrsInner::flush(self);
    }
}
impl ::prometheus::local::MayFlush for LhrsInner {
    fn may_flush(&self) {
        MayFlush::try_flush(self, &self.last_flush, self.flush_duration)
    }
}
#[allow(missing_copy_implementations)]
pub struct LhrsDelegator {
    pub post: Lhrs2Delegator,
    pub get: Lhrs2Delegator,
    pub put: Lhrs2Delegator,
    pub delete: Lhrs2Delegator,
}
impl LhrsDelegator {
    pub fn new(root: &'static LocalKey<LhrsInner>, offset1: usize) -> LhrsDelegator {
        let x = unsafe { MaybeUninit::<Lhrs2Inner>::uninit().assume_init() };
        let branch_offset = (&x as *const Lhrs2Inner) as usize;
        let post = Lhrs2Delegator::new(
            root,
            offset1,
            &(x.post) as *const Lhrs3Inner as usize - branch_offset,
        );
        let get = Lhrs2Delegator::new(
            root,
            offset1,
            &(x.get) as *const Lhrs3Inner as usize - branch_offset,
        );
        let put = Lhrs2Delegator::new(
            root,
            offset1,
            &(x.put) as *const Lhrs3Inner as usize - branch_offset,
        );
        let delete = Lhrs2Delegator::new(
            root,
            offset1,
            &(x.delete) as *const Lhrs3Inner as usize - branch_offset,
        );
        mem::forget(x);
        LhrsDelegator {
            post,
            get,
            put,
            delete,
        }
    }
    pub fn get(&self, enum_value: Methods) -> &Lhrs2Delegator {
        match enum_value {
            Methods::post => &self.post,
            Methods::get => &self.get,
            Methods::put => &self.put,
            Methods::delete => &self.delete,
        }
    }
}
#[allow(missing_copy_implementations)]
pub struct Lhrs2Inner {
    pub post: Lhrs3Inner,
    pub get: Lhrs3Inner,
    pub put: Lhrs3Inner,
    pub delete: Lhrs3Inner,
}
impl Lhrs2Inner {
    pub fn from(label_0: &str, m: &IntCounterVec) -> Lhrs2Inner {
        Lhrs2Inner {
            post: Lhrs3Inner::from(label_0, "post", m),
            get: Lhrs3Inner::from(label_0, "get", m),
            put: Lhrs3Inner::from(label_0, "put", m),
            delete: Lhrs3Inner::from(label_0, "delete", m),
        }
    }
    pub fn flush(&self) {
        self.post.flush();
        self.get.flush();
        self.put.flush();
        self.delete.flush();
    }
}
#[allow(missing_copy_implementations)]
pub struct Lhrs2Delegator {
    pub http1: AFLocalCounter<LhrsInner, LocalIntCounter, Lhrs3Delegator>,
    pub http2: AFLocalCounter<LhrsInner, LocalIntCounter, Lhrs3Delegator>,
}
impl Lhrs2Delegator {
    pub fn new(
        root: &'static LocalKey<LhrsInner>,
        offset1: usize,
        offset2: usize,
    ) -> Lhrs2Delegator {
        let x = unsafe { MaybeUninit::<Lhrs3Inner>::uninit().assume_init() };
        let branch_offset = (&x as *const Lhrs3Inner) as usize;
        let http1 = Lhrs3Delegator::new(
            root,
            offset1,
            offset2,
            &(x.http1) as *const LocalIntCounter as usize - branch_offset,
        );
        let http2 = Lhrs3Delegator::new(
            root,
            offset1,
            offset2,
            &(x.http2) as *const LocalIntCounter as usize - branch_offset,
        );
        mem::forget(x);
        Lhrs2Delegator { http1, http2 }
    }
}
#[allow(missing_copy_implementations)]
pub struct Lhrs3Inner {
    pub http1: LocalIntCounter,
    pub http2: LocalIntCounter,
}
impl Lhrs3Inner {
    pub fn from(label_0: &str, label_1: &str, m: &IntCounterVec) -> Lhrs3Inner {
        Lhrs3Inner {
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
#[allow(missing_copy_implementations)]
pub struct Lhrs3Delegator {
    root: &'static LocalKey<LhrsInner>,
    pub offset1: usize,
    pub offset2: usize,
    pub offset3: usize,
}
impl Lhrs3Delegator {
    pub fn new(
        root: &'static LocalKey<LhrsInner>,
        offset1: usize,
        offset2: usize,
        offset3: usize,
    ) -> AFLocalCounter<LhrsInner, LocalIntCounter, Lhrs3Delegator> {
        let delegator = Lhrs3Delegator {
            root,
            offset1,
            offset2,
            offset3,
        };
        AFLocalCounter::new(delegator)
    }
}
impl CounterDelegator<LhrsInner, LocalIntCounter> for Lhrs3Delegator {
    fn get_root_metric(&self) -> &'static LocalKey<LhrsInner> {
        self.root
    }
    fn get_local<'a>(&self, root_metric: &'a LhrsInner) -> &'a LocalIntCounter {
        unsafe {
            let lhrsinner = root_metric as *const LhrsInner;
            let lhrs2inner = (lhrsinner as usize + self.offset1) as *const Lhrs2Inner;
            let lhrs3inner = (lhrs2inner as usize + self.offset2) as *const Lhrs3Inner;
            let localintcounter =
                (lhrs3inner as usize + self.offset3) as *const LocalIntCounter;
            &*localintcounter
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
            &(x.foo) as *const Lhrs2Inner as usize - branch_offset,
        );
        let bar = LhrsDelegator::new(
            &inner,
            &(x.bar) as *const Lhrs2Inner as usize - branch_offset,
        );
        mem::forget(x);
        Lhrs { inner, foo, bar }
    }
    pub fn get(&self, enum_value: FooBar) -> &LhrsDelegator {
        match enum_value {
            FooBar::foo => &self.foo,
            FooBar::bar => &self.bar,
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
