use chur_build::{error::ChurError, Config, Dependency};

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
        .dependency(Dependency::github("googleapis/api-common-protos", None))
        .build()
        .unwrap();

    chur_build::execute(cfg)
}
