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

use std::io::Error as IoError;
use std::result;

#[cfg(feature = "protobuf")]
use protobuf::error::ProtobufError;

#[cfg(feature = "protobuf")]
quick_error! {
    /// The error types for prometheus.
    #[derive(Debug)]
    pub enum Error {
        /// A duplicate metric collector has already been registered.
        AlreadyReg {
            description("duplicate metrics collector registration attempted")
        }
        /// The label cardinality was inconsistent.
        InconsistentCardinality(expect: usize, got: usize) {
            description("inconsistent label cardinality")
            display("expect {} label values, but got {}", expect, got)
        }
        /// An error message which is only a string.
        Msg(msg: String) {
            description(msg)
            display("Error: {}", msg)
        }
        /// An error containing a [`std::io::Error`].
        Io(err: IoError) {
            from()
            cause(err)
            description(err.description())
            display("Io {}", err)
        }
        /// An error containing a [`protobuf::Error`].
        Protobuf(err: ProtobufError) {
            from()
            cause(err)
            description(err.description())
            display("Protobuf {}", err)
        }
    }
}

#[cfg(not(feature = "protobuf"))]
quick_error! {
    /// The error types for prometheus.
    #[derive(Debug)]
    pub enum Error {
        /// A duplicate metric collector has already been registered.
        AlreadyReg {
            description("duplicate metrics collector registration attempted")
        }
        /// The label cardinality was inconsistent.
        InconsistentCardinality(expect: usize, got: usize) {
            description("inconsistent label cardinality")
            display("expect {} label values, but got {}", expect, got)
        }
        /// An error message which is only a string.
        Msg(msg: String) {
            description(msg)
            display("Error: {}", msg)
        }
        /// An error containing a [`std::io::Error`].
        Io(err: IoError) {
            from()
            cause(err)
            description(err.description())
            display("Io {}", err)
        }
    }
}

/// A specialized Result type for prometheus.
pub type Result<T> = result::Result<T, Error>;
