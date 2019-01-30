pub trait Collector {
    fn describe(&self) {}

    fn collect(&self) {}

    fn box_clone(&self) -> Box<Collector>;
}
