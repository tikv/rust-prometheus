// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

/*!
This crate provides staticly built metrics to your Prometheus application.

This is useful since it reduces the amount of branching and processing needed at runtime to collect metrics.

```rust
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;
use prometheus::{self, IntCounter, TextEncoder, Encoder};

lazy_static! {
    static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
}

HIGH_FIVE_COUNTER.inc();
assert_eq!(HIGH_FIVE_COUNTER.get(), 1);
```

Is it reccomended that you consult the [`prometheus` documentation for more information.](https://docs.rs/prometheus/)
*/

extern crate static_metric_proc_macros;

pub use static_metric_proc_macros::*;
