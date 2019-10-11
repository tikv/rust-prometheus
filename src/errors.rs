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

use failure::Fail;

/// A specialized Result type for prometheus.
pub type Result<T> = std::result::Result<T, Error>;

/// The error types for prometheus.
#[derive(Debug, Fail)]
pub enum Error {
    /// A duplicate metric collector has already been registered.
    #[fail(display = "Duplicate metrics collector registration attempted")]
    AlreadyReg,
    /// The label cardinality was inconsistent.
    #[fail(
        display = "Inconsistent label cardinality, expect {} label values, but got {}",
        _0, _1
    )]
    InconsistentCardinality(usize, usize),
    /// An error message which is only a string.
    #[fail(display = "Error: {}", _0)]
    Msg(String),
    /// An error containing a [`std::io::Error`].
    #[fail(display = "Io error: {}", _0)]
    Io(std::io::Error),
    /// An error containing a [`protobuf::Error`].
    #[cfg(feature = "protobuf")]
    #[fail(display = "Protobuf error: {}", _0)]
    Protobuf(protobuf::error::ProtobufError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

#[cfg(feature = "protobuf")]
impl From<protobuf::error::ProtobufError> for Error {
    fn from(err: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}
