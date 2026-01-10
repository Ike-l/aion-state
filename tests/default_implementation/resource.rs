#[derive(Debug, PartialEq, Clone)]
pub struct StoredResource(i32);

impl StoredResource {
    pub fn new(resource: i32) -> Self {
        Self(resource)
    }

    pub fn get(&self) -> &i32 {
        &self.0
    }

    pub fn consume(self) -> i32 {
        self.0
    }
}
