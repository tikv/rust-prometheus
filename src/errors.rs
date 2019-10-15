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

use thiserror::Error;

/// The error types for prometheus.
#[derive(Debug, Error)]
pub enum Error {
    /// A duplicate metric collector has already been registered.
    #[error("Duplicate metrics collector registration attempted")]
    AlreadyReg,
    /// The label cardinality was inconsistent.
    #[error("Inconsistent label cardinality, expect {expect} label values, but got {got}")]
    InconsistentCardinality {
        /// The expected number of labels.
        expect: usize,
        /// The actual number of labels.
        got: usize,
    },
    /// An error message which is only a string.
    #[error("Error: {0}")]
    Msg(String),
    /// An error containing a [`std::io::Error`].
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    /// An error containing a [`protobuf::Error`].
    #[cfg(feature = "protobuf")]
    #[error("Protobuf error: {0}")]
    Protobuf(#[from] protobuf::error::ProtobufError),
}

/// A specialized Result type for prometheus.
pub type Result<T> = std::result::Result<T, Error>;
