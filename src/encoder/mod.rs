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


use errors::Result;
use proto::MetricFamily;
use std::io::Write;

mod text;
mod pb;

pub use self::pb::{ProtobufEncoder, PROTOBUF_FORMAT};
pub use self::text::{TextEncoder, TEXT_FORMAT};

/// `Encoder` types encode metric families into an underlying wire protocol.
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
