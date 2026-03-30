use std::{collections::HashMap, hash::Hash};

pub struct WhitelistStorage<ResourceId, Access> {
    inner: HashMap<ResourceId, Vec<Access>>
}

impl<ResourceId, Access> Default for WhitelistStorage<ResourceId, Access> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ResourceId: Eq + Hash, Access: PartialEq> crate::prelude::WhitelistStorage for WhitelistStorage<ResourceId, Access> {
    type Id = ResourceId;
    type Access = Access;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        allowed_accesses.iter().any(|allowed_access| allowed_access == access)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> bool {
        self.inner.entry(id).or_default().push(access);

        true
    }

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        self.inner.remove(id).is_some()
    }

    fn release_all<'a>(
        &mut self,
        mut ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as crate::prelude::WhitelistStorage>::Id: 'a {
        !ids.any(|resource_id| !self.release(resource_id))
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };

        let Some(position) = allowed_accesses.iter().position(|allowed_access| allowed_access == access) else { return false };

        allowed_accesses.remove(position);

        true
    }
}