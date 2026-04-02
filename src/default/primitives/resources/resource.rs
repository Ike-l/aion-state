pub struct Resource {
    inner: String
}

impl Resource {
    pub fn get(&self) -> &str {
        &self.inner
    }
}