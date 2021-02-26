use crate::atomic64::Atomic;
use crate::timer;
use crate::proto::LabelPair;

/// An OpenMetrics Exemplar
///
/// https://github.com/OpenObservability/OpenMetrics/blob/master/specification/OpenMetrics.md#exemplars
#[derive(Debug)]
pub struct Exemplar<P: Atomic> {
    pub value: P,
    pub labels: Vec<LabelPair>,
    pub timestamp_ms: i64,
}

impl<P: Atomic> Exemplar<P> {
    fn new(val: P::T) -> Self {
        Self {
            value: P::new(val),
            labels: vec![],
            timestamp_ms: (timer::now_millis() / 1000) as i64,
        }
    }

    fn new_with_labels(val: P::T, labels: Vec<LabelPair>) -> Self {
        let ex = Self {
            value: P::new(val),
            labels: labels,
            timestamp_ms: (timer::now_millis() / 1000) as i64,
        };
        // TODO: verify length of labelset + values as <= 128 UTF8 chars
        ex
    }

    fn set(&mut self, val: P::T, labels: Vec<LabelPair>) {
        self.value.set(val);
        self.labels = labels;
        self.timestamp_ms = (timer::now_millis() / 1000) as i64;
        // TODO: verify length of labelset + values as <= 128 UTF8 chars
    }
}
