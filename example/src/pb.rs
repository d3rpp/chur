pub mod google {
    pub mod r#type {
        include!(concat!(env!("OUT_DIR"), "/", "google.r#type.rs"));
    }
}
pub mod example {
    pub mod hello_world {
        pub mod v1 {
            include!(concat!(env!("OUT_DIR"), "/", "example.hello_world.v1.rs"));
        }
    }
}
pub const FILE_DESCRIPTOR_BYTES: &'static [u8] = include_bytes!(
    concat!(env!("OUT_DIR"), "/", "__fd.bin")
);
