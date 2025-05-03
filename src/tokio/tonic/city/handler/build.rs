use std::env;
use std::path::PathBuf;

fn main() {
    //tonic_build::compile_protos("../../proto/city/seoul/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    //tonic_build::compile_protos("../../proto/city/newyork/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    //
    //tonic_build::compile_protos("../../proto/city/tokyo/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let descriptor_path = out_dir.join("descriptor.bin");

    tonic_build::configure()
        .out_dir("src/pb")
        .file_descriptor_set_path(&descriptor_path)
        .compile_protos(
            &[
                "../../proto/city/seoul/city.proto",
                "../../proto/city/newyork/city.proto",
                "../../proto/city/tokyo/city.proto",
            ],
            &["../../proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}
