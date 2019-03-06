fn main() {
    if !cfg!(feature = "gen") {
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    #[cfg(feature = "gen")]
    {
        use protobuf_build::*;

        check_protoc_version();

        println!("cargo:rerun-if-changed=proto/proto_model.proto");

        generate_protobuf_files(&["proto/proto_model.proto"], "proto");
    }
}
