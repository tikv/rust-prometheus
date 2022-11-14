use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};

#[cfg(test)]
use mock_instant::Instant;

#[cfg(not(test))]
use std::time::Instant;

use crate::{core::Collector, Error, PullingGauge};

/// A prometheus gauge that exposes the maximum value of a gauge over an interval.
///
/// Used to expose instantaneous values that tend to move a lot within a small interval.
///
/// # Examples
/// ```
/// # use std::time::Duration;
/// # use prometheus::{Registry, MaximumOverIntervalGauge};
///
/// let registry = Registry::new();
/// let gauge = MaximumOverIntervalGauge::new(
///     "maximum_queue_size_30s",
///     "The high watermark queue size in the last 30 seconds.",
///     Duration::from_secs(30)
/// ).unwrap();
/// registry.register(Box::new(gauge.clone()));
///
/// gauge.add(30);
/// gauge.sub(10);
///
/// // For the next 30 seconds, the metric will be 30 as that was the maximum value.
/// // Afterwards, it will drop to 10.
/// ```
#[derive(Clone, Debug)]
pub struct MaximumOverIntervalGauge {
    // The current real-time value.
    value: Arc<AtomicI64>,
    // The maximum value in the current interval.
    maximum_value: Arc<AtomicI64>,

    // The length of a given interval.
    interval_duration: Duration,
    // The time at which the current interval will expose.
    interval_expiry: Arc<RwLock<Instant>>,

    gauge: PullingGauge,
}

impl MaximumOverIntervalGauge {
    /// Create a new [`MaximumOverIntervalGauge`].
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        interval: Duration,
    ) -> Result<Self, Error> {
        let maximum_value = Arc::new(AtomicI64::new(0));

        Ok(Self {
            value: Arc::new(AtomicI64::new(0)),
            maximum_value: maximum_value.clone(),

            interval_expiry: Arc::new(RwLock::new(Instant::now() + interval)),
            interval_duration: interval,
            gauge: PullingGauge::new(
                name,
                help,
                Box::new(move || maximum_value.load(Ordering::Relaxed) as f64),
            )?,
        })
    }

    /// Increments the gauge by 1.
    pub fn inc(&self) {
        self.apply_delta(1);
    }

    /// Decrements the gauge by 1.
    pub fn dec(&self) {
        self.apply_delta(-1);
    }

    /// Add the given value to the gauge.
    ///
    /// (The value can be negative, resulting in a decrement of the gauge.)
    pub fn add(&self, v: i64) {
        self.apply_delta(v);
    }

    /// Subtract the given value from the gauge.
    ///
    /// (The value can be negative, resulting in an increment of the gauge.)
    pub fn sub(&self, v: i64) {
        self.apply_delta(-v);
    }

    fn apply_delta(&self, delta: i64) {
        let previous_value = self.value.fetch_add(delta, Ordering::Relaxed);
        let new_value = previous_value + delta;

        let now = Instant::now();
        let interval_expiry = self.interval_expiry.upgradable_read();
        let loaded_interval_expiry = *interval_expiry;

        // Check whether we've crossed into the new interval.
        if loaded_interval_expiry < now {
            // There's a possible optimization here of using try_upgrade in a loop. Need to write
            // benchmarks to verify.
            let mut interval_expiry = RwLockUpgradableReadGuard::upgrade(interval_expiry);

            // Did we get to be the thread that actually started the new interval? Other threads
            // could have updated the value before we got the exclusive lock.
            if *interval_expiry == loaded_interval_expiry {
                *interval_expiry = now + self.interval_duration;
                self.maximum_value.store(new_value, Ordering::Relaxed);

                return;
            }
        }

        // Set the maximum_value to the max of the current value & previous max.
        self.maximum_value.fetch_max(new_value, Ordering::Relaxed);
    }
}

impl Collector for MaximumOverIntervalGauge {
    fn desc(&self) -> Vec<&crate::core::Desc> {
        self.gauge.desc()
    }

    fn collect(&self) -> Vec<crate::proto::MetricFamily> {
        // Apply a delta of '0' to ensure that the reset-value-if-interval-expired-logic kicks in.
        self.apply_delta(0);

        self.gauge.collect()
    }
}

#[cfg(test)]
mod test {
    use mock_instant::MockClock;

    use super::*;

    static INTERVAL: Duration = Duration::from_secs(30);

    #[test]
    fn test_correct_behaviour() {
        let gauge = MaximumOverIntervalGauge::new(
            "test_counter".to_string(),
            "This won't help you".to_string(),
            INTERVAL,
        )
        .unwrap();

        assert_metric_value(&gauge, 0.0);

        gauge.add(5);

        assert_metric_value(&gauge, 5.0);

        gauge.dec();

        // The value should still be five after we decreased it as the max within the interval was 5.
        assert_metric_value(&gauge, 5.0);

        MockClock::advance(INTERVAL + Duration::from_secs(1));

        // The value should be 4 now as the next interval has started.
        assert_metric_value(&gauge, 4.0);
    }

    #[test]
    fn test_cloning() {
        let gauge = MaximumOverIntervalGauge::new(
            "test_counter".to_string(),
            "This won't help you".to_string(),
            INTERVAL,
        )
        .unwrap();

        let same_gauge = gauge.clone();

        assert_metric_value(&gauge, 0.0);

        gauge.add(5);

        // Read from the cloned gauge to veriy that they share data.
        assert_metric_value(&same_gauge, 5.0);
    }

    fn assert_metric_value(gauge: &MaximumOverIntervalGauge, val: f64) {
        let result = gauge.collect();

        let metric_family = result
            .first()
            .expect("expected one MetricFamily to be returned");

        let metric = metric_family
            .get_metric()
            .first()
            .expect("expected one Metric to be returned");

        assert_eq!(val, metric.get_gauge().get_value());
    }
}
