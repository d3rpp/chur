use chur::{error::ChurError, Config, Dependency};

fn main() -> Result<(), ChurError> {
    // println!(
    //     "cargo::warning=OUT_DIR is {}",
    //     std::env::var("OUT_DIR").unwrap()
    // );

    let cfg = Config::builder()
        .root_dir("example/proto")
        .protos([
            "example/hello_world/v1/hello_world_service.proto",
            "example/hello_world/v1/foo_service.proto",
        ])
        .dependency(Dependency::github(
			"googleapis/api-common-protos",
			"3332dec527759859840a3a2ff108c67a54708130".to_string(),
            None
		))
		.dependency(Dependency::github(
			"grpc/grpc-proto",
			"a9c639a9a4bddf74bcbd819acc871fa8ad2b8a81".to_string(),
            None
		))
        .file_descriptors(true)
        .codegen("example/src/pb.rs".to_string())
        .build()
        .unwrap();

    chur::execute(cfg)
}
