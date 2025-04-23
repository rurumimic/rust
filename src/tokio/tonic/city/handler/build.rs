fn main() {
    //tonic_build::compile_protos("../../proto/city/seoul/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    //tonic_build::compile_protos("../../proto/city/newyork/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
    //
    //tonic_build::compile_protos("../../proto/city/tokyo/city.proto")
    //    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    tonic_build::configure()
        .out_dir("src/pb")
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
