use std::{collections::HashMap, hash::Hash};

pub struct RegistryStorage<ResourceId, StoredResource> {
    inner: HashMap<ResourceId, StoredResource>
}

impl<ResourceId, StoragedResource> Default for RegistryStorage<ResourceId, StoragedResource> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ResourceId: Eq + Hash, StoredResource> crate::prelude::RegistryStorage for RegistryStorage<ResourceId, StoredResource> {
    type ValueId = ResourceId;
    type Value = StoredResource;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        self.inner.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value> {
        self.inner.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value> {
        self.inner.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        self.inner.contains_key(value_id)
    }
}