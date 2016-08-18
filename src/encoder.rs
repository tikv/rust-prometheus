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
    fn encode(&self, &MetricFamily, &mut Write) -> Result<usize>;
}

pub const TEXT_FORMAT: &'static str = "text/plain; version=0.0.4";

#[derive(Debug)]
pub struct TextEncoder;

impl Encoder for TextEncoder {
    /// encode converts a MetricFamily proto message into text format and
    /// writes the resulting lines to 'out'. It returns the number of bytes written
    /// and any error encountered.  This function does not perform checks on the
    /// content of the metric and label names, i.e. invalid metric or label names
    /// will result in invalid text format output.
    fn encode(&self, metric_family: &MetricFamily, writer: &mut Write) -> Result<usize> {
        if metric_family.get_metric().len() == 0 {
            return Err(Error::Msg(format!("MetricFamily has no metrics")));
        }

        let name = metric_family.get_name();
        if name == "" {
            return Err(Error::Msg(format!("MetricFamily has no name")));
        }

        let mut written = 0;
        let help = metric_family.get_help();
        if help.len() != 0 {
            written += try!(
                writer.write((format!("# HELP {} {}\n", name, escape_string(help, false)))
                        .as_bytes()).or_else(|e| Err(Error::Io(e))));
        }

        let metric_type = metric_family.get_field_type();

        let lowercase_type = format!("{:?}", metric_type).to_owned().to_lowercase();
        written += try!(writer.write((format!("# TYPE {} {}\n", name, lowercase_type)).as_bytes())
            .or_else(|e| Err(Error::Io(e))));

        for m in metric_family.get_metric() {
            match metric_type {
                MetricType::COUNTER => {
                    written +=
                        try!(write_sample(name, m, "", "", m.get_counter().get_value(), writer));
                }
                MetricType::GAUGE => {
                    written +=
                        try!(write_sample(name, m, "", "", m.get_gauge().get_value(), writer));
                }
                MetricType::SUMMARY => {}
                MetricType::HISTOGRAM => {}
                MetricType::UNTYPED => {}
            }
        }
        Ok(written)
    }
}

/// write_sample writes a single sample in text format to out, given the metric
/// name, the metric proto message itself, optionally an additional label name
/// and value (use empty strings if not required), and the value. The function
/// returns the number of bytes written and any error encountered.
fn write_sample(name: &str,
                mc: &proto::Metric,
                additional_label_name: &str,
                additional_label_value: &str,
                value: f64,
                writer: &mut Write)
                -> Result<usize> {
    let mut written = 0;
    written += try!(writer.write(name.as_bytes()).or_else(|e| Err(Error::Io(e))));

    written += try!(label_pairs_to_text(mc.get_label(),
                                        additional_label_name,
                                        additional_label_value,
                                        writer));

    written += try!(writer.write((format!(" {}", value)).as_bytes())
        .or_else(|e| Err(Error::Io(e))));
    let time_stamp = mc.get_timestamp_ms();
    if time_stamp != 0 {
        written += try!(writer.write(format!(" {}", time_stamp).as_bytes())
            .or_else(|e| Err(Error::Io(e))));
    }

    written += try!(writer.write(b"\n").or_else(|e| Err(Error::Io(e))));;
    Ok(written)
}

/// label_pairs_to_text converts a slice of LabelPair proto messages plus the
/// explicitly given additional label pair into text formatted as required by the
/// text format and writes it to 'out'. An empty slice in combination with an
/// empty string 'additionalLabelName' results in nothing being
/// written. Otherwise, the label pairs are written, escaped as required by the
/// text format, and enclosed in '{...}'. The function returns the number of
/// bytes written and any error encountered.
fn label_pairs_to_text(pairs: &[proto::LabelPair],
                       additional_label_name: &str,
                       additional_label_value: &str,
                       writer: &mut Write)
                       -> Result<usize> {
    if pairs.len() == 0 && additional_label_name == "" {
        return Ok(0);
    }

    let mut written = 0;

    let mut separator = "{";
    for lp in pairs {
        written += try!(writer.write((format!("{}{}=\"{}\"",
                            separator,
                            lp.get_name(),
                            escape_string(lp.get_value(), true)))
                .as_bytes())
            .or_else(|e| Err(Error::Io(e))));

        separator = ",";
    }

    if additional_label_name != "" {
        written += try!(writer.write((format!("{}{}=\"{}\"",
                            separator,
                            additional_label_name,
                            escape_string(additional_label_value, true)))
                .as_bytes())
            .or_else(|e| Err(Error::Io(e))));
    }

    written += try!(writer.write(b"}").or_else(|e| Err(Error::Io(e))));
    Ok(written)
}

/// escape_string replaces '\' by '\\', new line character by '\n', and - if
/// include_double_quote is true - '"' by '\"'.
pub fn escape_string(v: &str, include_double_quote: bool) -> String {
    let mut escaped = String::with_capacity(v.len() * 2);

    for c in v.chars() {
        match c {
            '\\' => {
                escaped.push_str(r"\\");
            }
            '\n' => {
                escaped.push_str(r"\n");
            }
            '"' if include_double_quote => {
                escaped.push_str("\"");
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
    use super::*;

    #[test]
    fn test_ecape_string() {
        assert_eq!(r"\\", escape_string("\\", false));
        assert_eq!(r"a\\", escape_string("a\\", false));
        assert_eq!(r"\n", escape_string("\n", false));
        assert_eq!(r"a\n", escape_string("a\n", false));
        assert_eq!(r"\\n", escape_string("\\n", false));

        assert_eq!("\\\\n\"", escape_string("\\n\"", true));
    }
}
