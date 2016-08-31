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

/// Create labes with specify name-value pairs.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # use std::collections::HashMap;
/// # fn main() {
/// let labels = labels!{
///     "test" => "hello",
///     "foo" => "bar",
/// };
/// assert_eq!(labels.len(), 2);
/// assert!(labels.get("test").is_some());
/// assert_eq!(*(labels.get("test").unwrap()), "hello");
///
/// let labels: HashMap<&str, &str> = labels!{};
/// assert!(labels.is_empty());
/// # }
/// ```
#[macro_export]
macro_rules! labels {
    () => {
        {
            use std::collections::HashMap;

            HashMap::new()
        }
    };

    ( $ ( $ KEY : expr => $ VALUE : expr , ) + ) => {
        {
            use std::collections::HashMap;

            let mut lbs = HashMap::new();
            $(
                lbs.insert($KEY, $VALUE);
            )+

            lbs
        }
    }
}

/// Create an Opts.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let name = "test_opts";
/// let help = "test opts help";
///
/// let opts = opts!(name, help);
/// assert_eq!(opts.name, name);
/// assert_eq!(opts.help, help);
///
/// let opts = opts!(name, help, labels!{"test" => "hello", "foo" => "bar",});
/// assert_eq!(opts.const_labels.len(), 2);
/// assert!(opts.const_labels.get("foo").is_some());
/// assert_eq!(opts.const_labels.get("foo").unwrap(), "bar");
///
/// let opts = opts!(name,
///                  help,
///                  labels!{"test" => "hello", "foo" => "bar",},
///                  labels!{"ans" => "42",});
/// assert_eq!(opts.const_labels.len(), 3);
/// assert!(opts.const_labels.get("ans").is_some());
/// assert_eq!(opts.const_labels.get("ans").unwrap(), "42");
/// # }
/// ```
#[macro_export]
macro_rules! opts {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * ) => {
        {
            use std::collections::HashMap;

            let opts = $crate::Opts::new($NAME, $HELP);
            let lbs = HashMap::<String, String>::new();
            $(
                let mut lbs = lbs;
                lbs.extend($CONST_LABELS.iter().map(|(k, v)| ((*k).into(), (*v).into())));
            )*

            opts.const_labels(lbs)
        }
    }
}

/// Create a HistogramOpts
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # use prometheus::{linear_buckets, exponential_buckets};
/// # fn main() {
/// let name = "test_histogram_opts";
/// let help = "test opts help";
///
/// let opts = histogram_opts!(name, help);
/// assert_eq!(opts.common_opts.name, name);
/// assert_eq!(opts.common_opts.help, help);
///
/// let opts = histogram_opts!(name, help, labels!{"test" => "hello", "foo" => "bar",});
/// assert_eq!(opts.common_opts.const_labels.len(), 2);
/// assert!(opts.common_opts.const_labels.get("foo").is_some());
/// assert_eq!(opts.common_opts.const_labels.get("foo").unwrap(), "bar");
///
/// let opts = histogram_opts!(name, help, labels!{"test" => "hello", "foo" => "bar",});
/// assert_eq!(opts.common_opts.const_labels.len(), 2);
/// assert!(opts.common_opts.const_labels.get("test").is_some());
/// assert_eq!(opts.common_opts.const_labels.get("test").unwrap(), "hello");
///
/// let opts = histogram_opts!(name, help, []);
/// assert_eq!(opts.buckets.len(), 0);
///
/// let opts = histogram_opts!(name, help, [Vec::from(&[1.0, 2.0] as &[f64])]);
/// assert_eq!(opts.buckets.len(), 2);
///
/// let opts = histogram_opts!(name,
///                             help,
///                             labels!{"a" => "c",},
///                             [Vec::from(&[1.0, 2.0] as &[f64]), Vec::from(&[3.0] as &[f64])]);
/// assert_eq!(opts.buckets.len(), 3);
///
/// let opts = histogram_opts!(name,
///                             help,
///                             labels!{"a" => "c",},
///                             [linear_buckets(1.0, 0.5, 4).unwrap(),
///                             exponential_buckets(4.0, 1.1, 4).unwrap()]);
/// assert_eq!(opts.buckets.len(), 8);
/// # }
/// ```
#[macro_export]
macro_rules! histogram_opts {
    ( $ NAME : expr , $ HELP : expr , [ $ ( $ BUCKETS : expr ) , * ] ) => {
        {
            let his_opts = $crate::HistogramOpts::new($NAME, $HELP);

            let buckets = Vec::new();
            $(
                let mut buckets = buckets;
                buckets.extend($BUCKETS);
            )*;

            his_opts.buckets(buckets)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , [ $ ( $ BUCKETS : expr ) , + ] ) => {
        {
            use std::collections::HashMap;
            use std::iter::FromIterator;

            let his_opts = histogram_opts!($NAME, $HELP, [ $( $BUCKETS ), + ]);

            his_opts.const_labels(
                HashMap::from_iter($CONST_LABELS.iter().map(|(k, v)| ((*k).into(), (*v).into()))))
        }
    };

    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * ) => {
        {
            let opts = opts!($NAME, $HELP $(, $CONST_LABELS ) *);

            $crate::HistogramOpts::from(opts)
        }
    }
}

/// Create a Counter and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_counter_1",
///                     "help",
///                     labels!{"test" => "hello", "foo" => "bar",});
///
/// let res1 = register_counter!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_counter!("test_macro_counter_2", "help");
/// assert!(res2.is_ok());
///
/// let res3 = register_counter!("test_macro_counter_3", "help", labels!{ "a" => "b",});
/// assert!(res3.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_counter {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * ) => {
        register_counter!(opts!($NAME, $HELP $(, $CONST_LABELS)*))
    };

    ( $ OPTS : expr ) => {
        {
            let counter = $crate::Counter::with_opts($OPTS).unwrap();
            $crate::register(Box::new(counter.clone())).map(|_| counter)
        }
    }
}

/// Create a CounterVec and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_counter_vec_1",
///                   "help",
///                   labels!{"test" => "hello", "foo" => "bar",});
///
/// let counter_vec = register_counter_vec!(opts, &["a", "b"]);
/// assert!(counter_vec.is_ok());
///
/// let counter_vec = register_counter_vec!("test_macro_counter_vec_2", "help", &["a", "b"]);
/// assert!(counter_vec.is_ok());
///
/// let counter_vec = register_counter_vec!("test_macro_counter_vec_3",
///                                         "help",
///                                         labels!{"test" => "hello", "foo" => "bar",},
///                                         &["a", "b"]);
/// assert!(counter_vec.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_counter_vec {
    ( $ OPTS : expr , $ LABELS_NAMES : expr ) => {
        {
            let counter_vec = $crate::CounterVec::new($OPTS, $LABELS_NAMES).unwrap();
            $crate::register(Box::new(counter_vec.clone())).map(|_| counter_vec)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ LABELS_NAMES : expr ) => {
        {
            register_counter_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , $ LABELS_NAMES : expr ) => {
        {
            register_counter_vec!(opts!($NAME, $HELP, $CONST_LABELS), $LABELS_NAMES)
        }
    };
}

/// Create a Gauge and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_gauge",
///                     "help",
///                     labels!{"test" => "hello", "foo" => "bar",});
///
/// let res1 = register_gauge!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_gauge!("test_macro_gauge_2", "help");
/// assert!(res2.is_ok());
///
/// let res3 = register_gauge!("test_macro_gauge_3", "help", labels!{"a" => "b",});
/// assert!(res3.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_gauge {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * ) => {
        register_gauge!(opts!($NAME, $HELP $(, $CONST_LABELS)*))
    };

    ( $ OPTS : expr ) => {
        {
            let gauge = $crate::Gauge::with_opts($OPTS).unwrap();
            $crate::register(Box::new(gauge.clone())).map(|_| gauge)
        }
    }
}

/// Create a GaugeVec and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_gauge_vec_1",
///                  "help",
///                  labels!{"test" => "hello", "foo" => "bar",});
///
/// let gauge_vec = register_gauge_vec!(opts, &["a", "b"]);
/// assert!(gauge_vec.is_ok());
///
/// let gauge_vec = register_gauge_vec!("test_macro_gauge_vec_2", "help", &["a", "b"]);
/// assert!(gauge_vec.is_ok());
///
/// let gauge_vec = register_gauge_vec!("test_macro_gauge_vec_3",
///                                     "help",
///                                     labels!{"test" => "hello", "foo" => "bar",},
///                                     &["a", "b"]);
/// assert!(gauge_vec.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_gauge_vec {
    ( $ OPTS : expr , $ LABELS_NAMES : expr ) => {
        {
            let gauge_vec = $crate::GaugeVec::new($OPTS, $LABELS_NAMES).unwrap();
            $crate::register(Box::new(gauge_vec.clone())).map(|_| gauge_vec)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ LABELS_NAMES : expr ) => {
        {
            register_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , $ LABELS_NAMES : expr ) => {
        {
            register_gauge_vec!(opts!($NAME, $HELP, $CONST_LABELS), $LABELS_NAMES)
        }
    };
}

/// Create a Untyped and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_untyped",
///                     "help",
///                     labels!{"test" => "hello", "foo" => "bar",});
///
/// let res1 = register_untyped!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_untyped!("test_macro_untyped_2", "help");
/// assert!(res2.is_ok());
///
/// let res3 = register_untyped!("test_macro_untyped_3", "help", labels!{"a" => "b",});
/// assert!(res3.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_untyped {
    ( $ NAME : expr , $ HELP : expr $ ( , $ CONST_LABELS : expr ) * ) => {
        register_gauge!(opts!($NAME, $HELP $(, $CONST_LABELS)*))
    };

    ( $ OPTS : expr ) => {
        {
            let gauge = $crate::Untyped::with_opts($OPTS).unwrap();
            $crate::register(Box::new(gauge.clone())).map(|_| gauge)
        }
    }
}

/// Create a UntypedVec and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = opts!("test_macro_untyped_vec_1",
///                  "help",
///                  labels!{"test" => "hello", "foo" => "bar",});
///
/// let untyped_vec = register_untyped_vec!(opts, &["a", "b"]);
/// assert!(untyped_vec.is_ok());
///
/// let untyped_vec = register_untyped_vec!("test_macro_untyped_vec_2", "help", &["a", "b"]);
/// assert!(untyped_vec.is_ok());
///
/// let untyped_vec = register_untyped_vec!("test_macro_untyped_vec_3",
///                                     "help",
///                                     labels!{"test" => "hello", "foo" => "bar",},
///                                     &["a", "b"]);
/// assert!(untyped_vec.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_untyped_vec {
    ( $ OPTS : expr , $ LABELS_NAMES : expr ) => {
        {
            let gauge_vec = $crate::UntypedVec::new($OPTS, $LABELS_NAMES).unwrap();
            $crate::register(Box::new(gauge_vec.clone())).map(|_| gauge_vec)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ LABELS_NAMES : expr ) => {
        {
            register_gauge_vec!(opts!($NAME, $HELP), $LABELS_NAMES)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , $ LABELS_NAMES : expr ) => {
        {
            register_gauge_vec!(opts!($NAME, $HELP, $CONST_LABELS), $LABELS_NAMES)
        }
    };
}

/// Create a Histogram and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = histogram_opts!("test_macro_histogram",
///                             "help",
///                             labels!{"test" => "hello", "foo" => "bar",});
///
/// let res1 = register_histogram!(opts);
/// assert!(res1.is_ok());
///
/// let res2 = register_histogram!("test_macro_histogram_2", "help");
/// assert!(res2.is_ok());
///
/// let res3 = register_histogram!("test_macro_histogram_3", "help", labels!{"a" => "b",});
/// assert!(res3.is_ok());
///
/// let res4 = register_histogram!("test_macro_histogram_4",
///                                 "help",
///                                 labels!{"a" => "b",},
///                                 [Vec::from(&[1.0, 2.0] as &[f64])]);
/// assert!(res4.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_histogram {
    ( $ NAME : expr , $ HELP : expr ) => {
        register_histogram!(histogram_opts!($NAME, $HELP))
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr ) => {
        register_histogram!(histogram_opts!($NAME, $HELP, $CONST_LABELS))
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , [ $ ( $ BUCKETS : expr ) , + ] ) => {
        register_histogram!(
            histogram_opts!($NAME, $HELP, $CONST_LABELS, [ $($BUCKETS), + ]))
    };

    ( $ OPTS : expr ) => {
        {
            let histogram = $crate::Histogram::with_opts($OPTS).unwrap();
            $crate::register(Box::new(histogram.clone())).map(|_| histogram)
        }
    }
}

/// Create a HistogramVec and register to default registry.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate prometheus;
/// # fn main() {
/// let opts = histogram_opts!("test_macro_histogram_vec_1",
///                            "help",
///                            labels!{"test" => "hello", "foo" => "bar",});
///
/// let histogram_vec = register_histogram_vec!(opts, &["a", "b"]);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec =
///     register_histogram_vec!("test_macro_histogram_vec_2", "help", &["a", "b"]);
/// assert!(histogram_vec.is_ok());
///
/// let histogram_vec = register_histogram_vec!("test_macro_histogram_vec_3",
///                                             "help",
///                                             labels!{"test" => "hello", "foo" => "bar",},
///                                             &["a", "b"]);
/// assert!(histogram_vec.is_ok());
/// # }
/// ```
#[macro_export]
macro_rules! register_histogram_vec {
    ( $ OPTS : expr , $ LABELS_NAMES : expr ) => {
        {
            let histogram_vec = $crate::HistogramVec::new($OPTS, $LABELS_NAMES).unwrap();
            $crate::register(Box::new(histogram_vec.clone())).map(|_| histogram_vec)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ LABELS_NAMES : expr ) => {
        {
            register_histogram_vec!(histogram_opts!($NAME, $HELP), $LABELS_NAMES)
        }
    };

    ( $ NAME : expr , $ HELP : expr , $ CONST_LABELS : expr , $ LABELS_NAMES : expr ) => {
        {
            register_histogram_vec!(histogram_opts!($NAME, $HELP, $CONST_LABELS), $LABELS_NAMES)
        }
    };
}
