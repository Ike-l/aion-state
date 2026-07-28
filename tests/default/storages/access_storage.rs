use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

pub struct AccessStorage<ResourceId, Access> {
    inner: HashMap<ResourceId, Access>
}

impl<ResourceId, Access> Default for AccessStorage<ResourceId, Access> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ResourceId: Eq + Hash, Access> crate::prelude::AccessStorage for AccessStorage<ResourceId, Access> {
    type ValueId = ResourceId;
    type Access = Access;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Access> {
        event!(Level::TRACE, "AccessStorage get_mut");

        self.inner.get_mut(value_id)
    }

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Access> {
        event!(Level::TRACE, "AccessStorage get");

        self.inner.get(value_id)
    }

    fn insert(
        &mut self,
        value_id: Self::ValueId,
        access: Self::Access
    ) -> Option<Self::Access> {
        event!(Level::TRACE, "AccessStorage insert");

        self.inner.insert(value_id, access)
    }

    fn drain(&mut self) -> impl Iterator<Item = (
        Self::ValueId, 
        Self::Access
    )> {
        event!(Level::TRACE, "AccessStorage drain");

        self.inner.drain()
    }
}