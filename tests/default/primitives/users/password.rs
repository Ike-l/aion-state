#[derive(PartialEq, Clone)]
pub struct Password {
    inner: u64
}

impl From<u64> for Password {
    fn from(value: u64) -> Self {
        Self { inner: value }
    }
}