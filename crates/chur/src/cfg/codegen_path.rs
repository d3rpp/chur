#[derive(Debug)]
pub enum CodegenPath {
    Relative(String),
    Absolute
}

impl From<String> for CodegenPath {
    fn from(value: String) -> Self {
        Self::Relative(value)
    }
}