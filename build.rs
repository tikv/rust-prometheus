fn main() {
    if !cfg!(feature = "gen") {
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    #[cfg(feature = "gen")]
    {
        use protobuf_build::*;
        use std::fs;

        check_protoc_version();

        println!("cargo:rerun-if-changed=proto/proto_model.proto");

        // Delete rust files in proto directory
        fs::read_dir("proto")
            .expect("Couldn't read directory")
            .filter_map(|e| {
                let file_name = e.expect("Couldn't list file").file_name();
                let file_name = file_name.to_string_lossy();
                if !file_name.ends_with(".rs") {
                    return None;
                }
                Some(format!("proto/{}", file_name))
            })
            .for_each(|f| fs::remove_file(f).expect("Couldn't remove file"));

        // Prost
        generate_prost_files(&["proto/proto_model.proto".to_owned()], "proto");
        let mod_names = module_names_for_dir("proto");
        generate_wrappers(
            &mod_names
                .iter()
                .map(|m| format!("proto/{}.rs", m))
                .collect::<Vec<_>>(),
            "proto",
        );

        // Rust-protobuf
        generate_protobuf_files(&["proto/proto_model.proto"], "proto");
    }
}
