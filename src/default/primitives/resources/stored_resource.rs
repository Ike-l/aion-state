use crate::default::Resource;

pub struct StoredResource {
    inner: Resource
}

impl StoredResource {
    pub fn get(&self) -> &Resource {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut Resource {
        &mut self.inner
    }
}