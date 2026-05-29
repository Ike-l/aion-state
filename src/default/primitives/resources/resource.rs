#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    inner: String
}

impl Resource {
    pub fn new(resource: String) -> Self {
        Self { inner: resource }
    }

    pub fn get(&self) -> &str {
        &self.inner
    }
}