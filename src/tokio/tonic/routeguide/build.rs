fn main() {
    tonic_build::compile_protos("proto/route_guide.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    tonic_build::configure()
        .build_client(false)
        .out_dir("another_crate/src/pb")
        .compile_protos(&["proto/route_guide.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
