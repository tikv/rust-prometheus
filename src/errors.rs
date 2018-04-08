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

use protobuf::error::ProtobufError;
use std::io::Error as IoError;
use std::result;

quick_error!{
    /// The error types for prometheus.
    #[derive(Debug)]
    pub enum Error {
        AlreadyReg {
            description("duplicate metrics collector registration attempted")
        }
        InconsistentCardinality(expect: usize, got: usize) {
            description("inconsistent label cardinality")
            display("expect {} label values, but got {}", expect, got)
        }
        Msg(msg: String) {
            description(msg)
            display("Error: {}", msg)
        }
        Io(err: IoError) {
            from()
            cause(err)
            description(err.description())
            display("Io {}", err)
        }
        Protobuf(err: ProtobufError) {
            from()
            cause(err)
            description(err.description())
            display("Protobuf {}", err)
        }
    }
}

/// A specialized Result type for prometheus.
pub type Result<T> = result::Result<T, Error>;
