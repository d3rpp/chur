/// This is the equivalent of making this.
///
/// ```rust,ignore
/// pub mod pb {
///     pub mod google {
///         pub mod r#type {
///             chur::import_proto!("google.r#type");
///         }
///     }
///     pub mod example {
///         pub mod hello_world {
///             pub mod v1 {
///                 chur::import_proto!("example.hello_world.v1");
///             }
///         }
///     }
/// }
/// ```
pub mod pb;

#[allow(unused)]
fn main() {
    println!("Hello, World!");

    let desc = pb::FILE_DESCRIPTOR_BYTES;

    let pb = pb::example::hello_world::v1::ExampleRequest {
        ..Default::default()
    };
}
