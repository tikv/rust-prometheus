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

#[macro_export]
macro_rules! labels {
    ($($KEY: expr => $VALUE: expr ,)+) => {
        {
            use std::collections::HashMap;

            let mut lbs = HashMap::<String, String>::new();
            $(
                lbs.insert(($KEY).into(), ($VALUE).into());
            )*

            lbs
        }
    };

    () => {
        HashMap::<String, String>::new()
    }
}

#[macro_export]
macro_rules! opts {
    ($NAME: expr, $HELP: expr, $LABELS: expr) => {
        $crate::Opts::new($NAME, $HELP).const_labels($LABELS)
    }
}

#[macro_export]
macro_rules! register_counter_with {
    ($OPTS: expr) => {
        {
            let counter = $crate::Counter::with_opts($OPTS).unwrap();
            $crate::register(Box::new(counter.clone())).unwrap();

            counter
        }
    }
}

#[macro_export]
macro_rules! register_gauge_with {
    ($OPTS: expr) => {
        {
            let gauge = $crate::Gauge::with_opts($OPTS).unwrap();
            $crate::register(Box::new(gauge.clone())).unwrap();

            gauge
        }
    }
}
