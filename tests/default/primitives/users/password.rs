#[cfg(test)]
#[derive(Debug)]
#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Password {
    inner: String
}

impl From<u64> for Password {
    fn from(value: u64) -> Self {
        Self { inner: value.to_string() }
    }
}

impl Password {
    pub fn new(value: u64) -> Self {
        Self { inner: value.to_string() }
    }
}