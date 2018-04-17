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

use syn::Ident;

pub fn get_metric_vec_type(metric_type: &Ident) -> Ident {
    Ident::from(format!("{}Vec", metric_type))
}

pub fn get_label_struct_name(struct_name: &Ident, label_index: usize) -> Ident {
    let mut struct_name = struct_name.to_string();
    if label_index > 0 {
        struct_name.push_str(&(label_index + 1).to_string());
    }
    Ident::from(struct_name)
}

pub fn get_member_type(
    struct_name: &Ident,
    label_index: usize,
    metric_type: &Ident,
    is_last_label: bool,
) -> Ident {
    if is_last_label {
        metric_type.clone()
    } else {
        get_label_struct_name(struct_name, label_index + 1)
    }
}
