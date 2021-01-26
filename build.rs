// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "gen")]
fn generate_protobuf_binding_file() {
    use prost_build::Config;
    use std::path::Path;

    let mut cfg = Config::new();
    cfg.out_dir(Path::new("proto"));
    cfg.compile_protos(&["proto/proto_model.proto"], &["proto"]).unwrap();
}

#[cfg(not(feature = "gen"))]
fn generate_protobuf_binding_file() {}

fn main() {
    generate_protobuf_binding_file()
}
