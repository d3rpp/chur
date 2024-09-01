#[macro_export]
macro_rules! import_proto {
    ($path: expr) => {
        include!(concat!(env!("__CHUR_DIR"), concat!("/", $path, ".rs")));
    };
}

pub use chur_macros::*;
