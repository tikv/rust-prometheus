use prometheus::{CounterBuilder, Registry};

fn main() {
    let counter = CounterBuilder::new("hello", "world").build().unwrap();

    let counters = CounterBuilder::new("hello", "world")
        .build_vec(["a", "b"])
        .unwrap();

    let counter = counters.with_label_values(["a_value", "b_value"]);
    // let counter = counters.with_label_values(["a_value", "b_value"]);
    // let counter = counters.with_label_values(["a_value", "b_value", "c"]);
    let counter = counters.with_label_values(["a_value".to_owned(), "b_value".to_owned()]);

    let counter = CounterBuilder::new("hello", "world")
        .build_vec(["a".to_owned(), "b".to_owned()])
        .unwrap();

    let mut registry = Registry;

    registry.register(&counter).unwrap();
}
