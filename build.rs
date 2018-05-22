extern crate protobuf_codegen_pure;

fn main() {
    if cfg!(feature = "gen") {
        protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
            out_dir: "proto",
            input: &["proto/metrics.proto"],
            includes: &["proto"],
            ..Default::default()
        }).unwrap();
    }
}
