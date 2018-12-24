// Copyright 2017 PingCAP, Inc.
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

use std::io::Write;

use errors::{Error, Result};
use proto::MetricFamily;

mod pb;
mod text;

pub use self::pb::{ProtobufEncoder, PROTOBUF_FORMAT};
pub use self::text::{TextEncoder, TEXT_FORMAT};

/// An interface for encoding metric families into an underlying wire protocol.
pub trait Encoder {
    /// `encode` converts a slice of MetricFamily proto messages into target
    /// format and writes the resulting lines to `writer`. It returns the number
    /// of bytes written and any error encountered. This function does not
    /// perform checks on the content of the metric and label names,
    /// i.e. invalid metric or label names will result in invalid text format
    /// output.
    fn encode<W: Write>(&self, &[MetricFamily], &mut W) -> Result<()>;

    /// `format_type` returns target format.
    fn format_type(&self) -> &str;
}

fn check_metric_family(mf: &MetricFamily) -> Result<()> {
    if mf.get_metric().is_empty() {
        return Err(Error::Msg(format!("MetricFamily has no metrics: {:?}", mf)));
    }
    if mf.get_name().is_empty() {
        return Err(Error::Msg(format!("MetricFamily has no name: {:?}", mf)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use counter::CounterVec;
    use encoder::Encoder;
    use metrics::Collector;
    use metrics::Opts;

    #[test]
    fn test_bad_metrics() {
        let mut writer = Vec::<u8>::new();
        let pb_encoder = ProtobufEncoder::new();
        let text_encoder = TextEncoder::new();
        let cv = CounterVec::new(
            Opts::new("test_counter_vec", "help information"),
            &["labelname"],
        )
        .unwrap();

        // Empty metrics
        let mfs = cv.collect();
        check_metric_family(&mfs[0]).unwrap_err();
        pb_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);
        text_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);

        // Add a sub metric
        cv.with_label_values(&["foo"]).inc();
        let mut mfs = cv.collect();

        // Empty name
        (&mut mfs[0]).clear_name();
        check_metric_family(&mfs[0]).unwrap_err();
        pb_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);
        text_encoder.encode(&mfs, &mut writer).unwrap_err();
        assert_eq!(writer.len(), 0);
    }
}
