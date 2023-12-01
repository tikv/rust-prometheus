// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(feature = "gen")]
fn generate_protobuf_binding_file() {
    protobuf_codegen::Codegen:: default()
    
    .protoc()
    // Use `protoc-bin-vendored` bundled protoc command, optional.
    .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())

    .include("proto").input("proto/metrics.proto").input("proto/gogo.proto").out_dir("proto").run_from_script()
    // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    //     out_dir: "proto",
    //     input: &["proto/metrics.proto"],
    //     includes: &["proto"],
    //     ..Default::default()
    // })
    // .unwrap();
}

#[cfg(not(feature = "gen"))]
fn generate_protobuf_binding_file() {}

fn main() {
    generate_protobuf_binding_file()
}
