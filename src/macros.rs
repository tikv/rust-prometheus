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
    ($($KEY: expr => $VALUE: expr ,)*) => {
        {
            use std::collections::HashMap;

            let lbs = HashMap::new();
            $(
                let mut lbs = lbs;
                lbs.insert($KEY, $VALUE);
            )*

            lbs
        }
    }
}

#[macro_export]
macro_rules! opts {
    ($NAME: expr, $HELP: expr) => {
        $crate::Opts::new($NAME, $HELP)
    };

    ($NAME: expr, $HELP: expr $(, $LABELS: expr)+) => {
        {
            use std::collections::HashMap;

            let opts = opts!($NAME, $HELP);
            let mut lbs = HashMap::<String, String>::new();
            $(
                lbs.extend($LABELS.iter().map(|(k, v)| ((*k).into(), (*v).into())));
            )+

            opts.const_labels(lbs)
        }
    }
}

#[macro_export]
macro_rules! register_counter_with {
    ($NAME: expr, $HELP: expr $(, $LABELS: expr)*) => {
        register_counter_with!(opts!($NAME, $HELP $(, $LABELS)*))
    };

    ($OPTS: expr) => {
        {
            let counter = $crate::Counter::with_opts($OPTS).unwrap();
            $crate::register(Box::new(counter.clone())).map(|_| counter)
        }
    }
}

#[macro_export]
macro_rules! register_gauge_with {
    ($NAME: expr, $HELP: expr $(, $LABELS: expr)*) => {
        register_gauge_with!(opts!($NAME, $HELP $(, $LABELS)*))
    };

    ($OPTS: expr) => {
        {
            let gauge = $crate::Gauge::with_opts($OPTS).unwrap();
            $crate::register(Box::new(gauge.clone())).map(|_| gauge)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_macro_labels() {
        let labels = labels!{
            "test" => "hello",
            "foo" => "bar",
        };
        assert_eq!(labels.len(), 2);
        assert!(labels.get("test").is_some());
        assert_eq!(*(labels.get("test").unwrap()), "hello");

        let labels: HashMap<&str, &str> = labels!{};
        assert!(labels.is_empty());
    }

    #[test]
    fn test_macro_opts() {
        let name = "test_opts";
        let help = "test opts help";

        let opts = opts!(name, help);

        assert_eq!(opts.name, name);
        assert_eq!(opts.help, help);

        let opts = opts!(name, help, labels!{"test" => "hello", "foo" => "bar",});
        assert_eq!(opts.const_labels.len(), 2);
        assert!(opts.const_labels.get("foo").is_some());
        assert_eq!(opts.const_labels.get("foo").unwrap(), "bar");

        let opts = opts!(name,
                         help,
                         labels!{"test" => "hello", "foo" => "bar",},
                         labels!{"ans" => "42",});
        assert_eq!(opts.const_labels.len(), 3);
        assert!(opts.const_labels.get("ans").is_some());
        assert_eq!(opts.const_labels.get("ans").unwrap(), "42");
    }

    #[test]
    fn test_macro_counter() {
        let opts = opts!("test_macro_counter_1",
                         "help",
                         labels!{"test" => "hello", "foo" => "bar",});

        let res1 = register_counter_with!(opts);
        assert!(res1.is_ok());

        let res2 = register_counter_with!("test_macro_counter_2", "help");
        assert!(res2.is_ok());

        let res3 = register_counter_with!("test_macro_counter_3", "help", labels!{ "a" => "b",});
        assert!(res3.is_ok());
    }

    #[test]
    fn test_macro_gauge() {
        let opts = opts!("test_macro_gauge",
                         "help",
                         labels!{"test" => "hello", "foo" => "bar",});

        let res1 = register_gauge_with!(opts);
        assert!(res1.is_ok());

        let res2 = register_gauge_with!("test_macro_gauge_2", "help");
        assert!(res2.is_ok());

        let res3 = register_gauge_with!("test_macro_gauge_3", "help", labels!{"a" => "b",});
        assert!(res3.is_ok());
    }
}
