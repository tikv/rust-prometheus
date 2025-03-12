use protobuf::{EnumOrUnknown, MessageField};

use crate::proto::{Counter, Gauge, Histogram, MetricType, Summary};

impl From<Counter> for MessageField<Counter> {
    fn from(value: Counter) -> Self {
        MessageField::some(value)
    }
}

impl From<Gauge> for MessageField<Gauge> {
    fn from(value: Gauge) -> Self {
        MessageField::some(value)
    }
}

impl From<Histogram> for MessageField<Histogram> {
    fn from(value: Histogram) -> Self {
        MessageField::some(value)
    }
}

impl From<Summary> for MessageField<Summary> {
    fn from(value: Summary) -> Self {
        MessageField::some(value)
    }
}

impl From<MetricType> for Option<EnumOrUnknown<MetricType>> {
    fn from(value: MetricType) -> Self {
        Some(EnumOrUnknown::from(value))
    }
}
