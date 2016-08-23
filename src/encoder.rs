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

use std::io::Write;

use errors::{Result, Error};
use proto::MetricFamily;
use proto::{self, MetricType};

pub trait Encoder {
    /// `encode` converts a slice of MetricFamily proto messages into target
    /// format and writes the resulting lines to `writer`. It returns the number
    /// of bytes written and any error encountered. This function does not
    /// perform checks on the content of the metric and label names,
    /// i.e. invalid metric or label names will result in invalid text format
    /// output.
    fn encode(&self, &[MetricFamily], &mut Write) -> Result<usize>;

    /// `format_type` returns target format.
    fn format_type(&self) -> &str;
}

macro_rules! write_writer {
    ($writer: expr, $($arg: tt)*) => {{
        let to_write = format!($($arg)*);
        try!($writer.write_all(to_write.as_bytes()));

        to_write.len()
    }}
}

pub type Format = &'static str;

pub const TEXT_FORMAT: Format = "text/plain; version=0.0.4";

/// Implementation of an `Encoder` that converts a `MetricFamily` proto message
/// into text format.
#[derive(Debug, Default)]
pub struct TextEncoder;

impl TextEncoder {
    pub fn new() -> TextEncoder {
        TextEncoder
    }
}

impl Encoder for TextEncoder {
    fn encode(&self, metric_familys: &[MetricFamily], writer: &mut Write) -> Result<usize> {
        let mut written = 0;

        for mf in metric_familys {
            if mf.get_metric().is_empty() {
                return Err(Error::Msg("MetricFamily has no metrics".to_owned()));
            }

            let name = mf.get_name();
            if name.is_empty() {
                return Err(Error::Msg("MetricFamily has no name".to_owned()));
            }

            let help = mf.get_help();
            if !help.is_empty() {
                written +=
                    write_writer!(writer, "# HELP {} {}\n", name, escape_string(help, false));
            }

            let metric_type = mf.get_field_type();
            let lowercase_type = format!("{:?}", metric_type).to_lowercase();
            written += write_writer!(writer, "# TYPE {} {}\n", name, lowercase_type);

            for m in mf.get_metric() {
                match metric_type {
                    MetricType::COUNTER => {
                        written += try!(write_sample(name,
                                                     m,
                                                     "",
                                                     "",
                                                     m.get_counter().get_value(),
                                                     writer));
                    }
                    MetricType::GAUGE | MetricType::SUMMARY | MetricType::HISTOGRAM |
                    MetricType::UNTYPED => unimplemented!(),
                }
            }
        }

        Ok(written)
    }

    fn format_type(&self) -> &str {
        TEXT_FORMAT
    }
}

/// `write_sample` writes a single sample in text format to `writer`, given the
/// metric name, the metric proto message itself, optionally an additional label
/// name and value (use empty strings if not required), and the value.
/// The function returns the number of bytes written and any error encountered.
fn write_sample(name: &str,
                mc: &proto::Metric,
                additional_label_name: &str,
                additional_label_value: &str,
                value: f64,
                writer: &mut Write)
                -> Result<usize> {
    let mut written = 0;

    try!(writer.write_all(name.as_bytes()));
    written += name.len();

    written += try!(label_pairs_to_text(mc.get_label(),
                                        additional_label_name,
                                        additional_label_value,
                                        writer));

    written += write_writer!(writer, " {}", value);


    let timestamp = mc.get_timestamp_ms();
    if timestamp != 0 {
        written += write_writer!(writer, " {}", timestamp);
    }

    try!(writer.write_all(b"\n"));
    written += 1;

    Ok(written)
}

/// `label_pairs_to_text` converts a slice of `LabelPair` proto messages plus
/// the explicitly given additional label pair into text formatted as required
/// by the text format and writes it to `writer`. An empty slice in combination
/// with an empty string `additional_label_name` results in nothing being
/// written. Otherwise, the label pairs are written, escaped as required by the
/// text format, and enclosed in '{...}'. The function returns the number of
/// bytes written and any error encountered.
fn label_pairs_to_text(pairs: &[proto::LabelPair],
                       additional_label_name: &str,
                       additional_label_value: &str,
                       writer: &mut Write)
                       -> Result<usize> {
    if pairs.is_empty() && additional_label_name.is_empty() {
        return Ok(0);
    }

    let mut written = 0;

    let mut separator = "{";
    for lp in pairs {
        written += write_writer!(writer,
                                 "{}{}=\"{}\"",
                                 separator,
                                 lp.get_name(),
                                 escape_string(lp.get_value(), true));

        separator = ",";
    }

    if !additional_label_name.is_empty() {
        written += write_writer!(writer,
                                 "{}{}=\"{}\"",
                                 separator,
                                 additional_label_name,
                                 escape_string(additional_label_value, true));
    }

    try!(writer.write_all(b"}"));
    written += 1;

    Ok(written)
}

/// `escape_string` replaces '\' by '\\', new line character by '\n', and - if
/// `include_double_quote` is true - '"' by '\"'.
pub fn escape_string(v: &str, include_double_quote: bool) -> String {
    let mut escaped = String::with_capacity(v.len() * 2);

    for c in v.chars() {
        match c {
            '\\' | '\n' => {
                escaped.extend(c.escape_default());
            }
            '"' if include_double_quote => {
                escaped.extend(c.escape_default());
            }
            _ => {
                escaped.push(c);
            }
        }
    }

    escaped.shrink_to_fit();

    escaped
}

#[cfg(test)]
mod tests {
    use counter::Counter;
    use metrics::{Opts, Collector};

    use super::*;

    #[test]
    fn test_ecape_string() {
        assert_eq!(r"\\", escape_string("\\", false));
        assert_eq!(r"a\\", escape_string("a\\", false));
        assert_eq!(r"\n", escape_string("\n", false));
        assert_eq!(r"a\n", escape_string("a\n", false));
        assert_eq!(r"\\n", escape_string("\\n", false));

        assert_eq!(r##"\\n\""##, escape_string("\\n\"", true));
        assert_eq!(r##"\\\n\""##, escape_string("\\\n\"", true));
        assert_eq!(r##"\\\\n\""##, escape_string("\\\\n\"", true));
        assert_eq!(r##"\"\\n\""##, escape_string("\"\\n\"", true));
    }

    #[test]
    fn test_text_encoder() {
        let opts = Opts::new("test", "test help").const_label("a", "1").const_label("b", "2");
        let counter = Counter::with_opts(opts).unwrap();
        counter.inc();

        let mf = counter.collect();
        let mut writer = Vec::<u8>::new();
        let encoder = TextEncoder::new();
        let txt = encoder.encode(&[mf], &mut writer);
        assert!(txt.is_ok());

        let ans = r##"# HELP test test help
# TYPE test counter
test{a="1",b="2"} 1
"##;

        assert_eq!(ans.as_bytes(), writer.as_slice());
        assert_eq!(ans.len(), txt.unwrap());
    }
}
