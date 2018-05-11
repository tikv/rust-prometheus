extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "proto",
        input: &["proto/metrics.proto"],
        includes: &["proto"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
