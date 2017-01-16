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

use protobuf::Message;

use errors::Result;
use proto::MetricFamily;

use super::Encoder;

/// The protocol buffer format of metric family.
pub const PROTOBUF_FORMAT: &'static str =
    "application/vnd.google.protobuf; proto=io.prometheus.client.MetricFamily; encoding=delimited";

/// Implementation of an `Encoder` that converts a `MetricFamily` proto message
/// into the binary wire format of protobuf.
#[derive(Debug, Default)]
pub struct ProtobufEncoder;

impl ProtobufEncoder {
    pub fn new() -> ProtobufEncoder {
        ProtobufEncoder
    }
}

impl Encoder for ProtobufEncoder {
    fn encode(&self, metric_familys: &[MetricFamily], writer: &mut Write) -> Result<()> {
        for mf in metric_familys {
            try!(mf.write_length_delimited_to_writer(writer));
        }
        Ok(())
    }

    fn format_type(&self) -> &str {
        PROTOBUF_FORMAT
    }
}

#[cfg(test)]
mod tests {
    // TODO: add tests.
}
