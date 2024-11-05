pub mod google {
    pub mod r#type {
        include!(
            "/home/d3rpp/code/chur/target/debug/build/chur-example-a6531aff33ada8de/out/google.r#type.rs"
        );
    }
}
pub mod example {
    pub mod hello_world {
        pub mod v1 {
            include!(
                "/home/d3rpp/code/chur/target/debug/build/chur-example-a6531aff33ada8de/out/example.hello_world.v1.rs"
            );
        }
    }
}
pub const FILE_DESCRIPTOR_BYTES: &'static [u8] = include_bytes!(
    "/home/d3rpp/code/chur/target/debug/build/chur-example-a6531aff33ada8de/out/__fd.bin"
);
