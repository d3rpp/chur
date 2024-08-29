#![doc = include_str!("../../../README.md")]

#[macro_export]
macro_rules! import_proto {
    ($path: expr) => (
        include!(concat!(env!("__CHUR_DIR"), concat!("/", $path, ".rs")));
    );
}