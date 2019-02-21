// Copyright 2018 PingCAP, Inc.
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

extern crate prometheus;
extern crate prometheus_static_metric;

use prometheus_static_metric::make_static_metric;

make_static_metric! {
    pub label_enum Methods {
        post,
        get,
        put,
        delete,
    }

    pub label_enum MethodsWithName {
        post: "post_name",
    }
}

#[test]
fn test_format() {
    assert_eq!("post", Methods::post.get_str());
    assert_eq!("post", format!("{}", Methods::post));
    assert_eq!("post", format!("{:?}", Methods::post));
    assert_eq!("delete", Methods::delete.get_str());
    assert_eq!("delete", format!("{}", Methods::delete));
    assert_eq!("delete", format!("{:?}", Methods::delete));
    assert_eq!("post_name", MethodsWithName::post.get_str());
    assert_eq!("post_name", format!("{}", MethodsWithName::post));
    assert_eq!("post_name", format!("{:?}", MethodsWithName::post));
}

#[test]
fn test_equal() {
    assert_eq!(Methods::post, Methods::post);
    assert_ne!(Methods::post, Methods::get);
}
