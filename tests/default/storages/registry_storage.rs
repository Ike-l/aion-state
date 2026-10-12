use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

use crate::default::primitives::accesses::access_result::Transmutable;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegistryStorage<ResourceId: Hash + Eq, StoredResource> {
    inner: HashMap<ResourceId, StoredResource>,
    calculated_len: usize,
    capacity: usize
}

impl<ResourceId: Hash + Eq, StoragedResource> Default for RegistryStorage<ResourceId, StoragedResource> {
    fn default() -> Self {
        let capacity = 1000;
        Self::new(capacity)
    }
}

impl<ResourceId: Hash + Eq, StoredResource> RegistryStorage<ResourceId, StoredResource> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
            calculated_len: 0,
            capacity
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ResourceWrapper<'a, T> {
    Unique(&'a mut T),
    Shared(&'a T)
}

impl<'a, T> ResourceWrapper<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self::Unique(value)
    }
}

impl<'a, T> Transmutable for ResourceWrapper<'a, T> {
    fn transmute(self) -> Self {
        let Self::Unique(value) = self else { unreachable!() };
        Self::Shared(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum ResourceView<'a, ResourceId: Hash + Eq, StoredResource> {
    Unique(HashMap<ResourceId, &'a mut StoredResource>),
    Shared(HashMap<ResourceId, &'a StoredResource>)
}

impl<'a, ResourceId: Hash + Eq, StoredResource> ResourceView<'a, ResourceId, StoredResource> {
    pub fn new(values: HashMap<ResourceId, &'a mut StoredResource>) -> Self {
        Self::Unique(values)
    }

    pub fn get(&self, key: &ResourceId) -> Option<&StoredResource> {
        match self {
            Self::Unique(values) => values.get(key).map(|value| &**value),
            Self::Shared(values) => values.get(key).copied(),
        }
    }
}

impl<'a, ResourceId: Hash + Eq, StoredResource> Transmutable for ResourceView<'a, ResourceId, StoredResource> {
    fn transmute(self) -> Self {
        let Self::Unique(values) = self else { unreachable!() };
        let shared = values.into_iter().map(|(key, value)| (key, &*value)).collect();
        Self::Shared(shared)
    }
}

impl<ResourceId: Eq + Hash + Clone, StoredResource> aion_state::prelude::RegistryStorage for RegistryStorage<ResourceId, StoredResource> {
    type ValueId = ResourceId;
    type OwnedValue = StoredResource;
    type ReferencedValue<'a> = ResourceWrapper<'a, StoredResource> where Self: 'a;
    type StorageView<'a> = ResourceView<'a, ResourceId, StoredResource> where Self: 'a;

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId> {
        self.inner.keys()
    }
    
    fn get_mut(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>>
    {
        event!(Level::TRACE, "RegistryStorage get mut");

        self.inner.get_mut(value_id).map(ResourceWrapper::new)
    }

    fn get_view(
        &mut self,
        value_ids: &[&Self::ValueId]
    ) -> Option<Self::StorageView<'_>>
    {
        event!(Level::TRACE, "RegistryStorage get view");

        if self.calculated_len == 0 {
            return None;
        }

        for value_id in value_ids {
            if !self.inner.contains_key(*value_id) {
                return None;
            }
        }

        let values = self.inner
            .iter_mut()
            .filter(|(key, _)| value_ids.iter().any(|value_id| *value_id == *key))
            .map(|(key, value)| (key.clone(), value))
            .collect::<HashMap<ResourceId, &mut StoredResource>>();

        Some(ResourceView::new(values))
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue> {
        event!(Level::TRACE, "RegistryStorage insert");

        let r = self.inner.insert(value_id, value);

        if r.is_none() {
            self.calculated_len += 1;
        }

        r
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue> {
        event!(Level::TRACE, "RegistryStorage remove");

        let r = self.inner.remove(value_id);

        if r.is_some() {
            self.calculated_len -= 1;
        }

        r
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        event!(Level::TRACE, "RegistryStorage contains key");

        self.inner.contains_key(value_id)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    unsafe fn next_insert_may_reallocates(&self) -> bool {
        self.calculated_len >= self.capacity
    }

    unsafe fn next_removal_may_reallocates(&self) -> bool {
        false
    }
}