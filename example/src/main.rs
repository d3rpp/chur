pub(crate) mod pb {
    pub(crate) mod google {
        pub(crate) mod r#type {
            chur::import_proto!("google.r#type");
        }
    }
    pub(crate) mod example {
        pub(crate) mod hello_world {
            pub(crate) mod v1 {
                chur::import_proto!("example.hello_world.v1");
            }
        }
    }
}



fn main() {
    println!("Hello, World!");
}