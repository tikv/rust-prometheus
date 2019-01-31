/// A trait for metrics collectors.
///
/// Normal users should use [`Counter`], [`Gauge`] and [`Histogram`].
pub trait Collector: Send + Sync {
    fn describe(&self) {}

    fn collect(&self) {}

    /// Clones and creates a boxed collector.
    fn box_clone(&self) -> Box<Collector>;
}
