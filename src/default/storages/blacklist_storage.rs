use std::{collections::HashMap, hash::Hash};

pub struct BlacklistStorage<ResourceId, Access, Password> {
    inner: HashMap<ResourceId, Vec<(Access, Password)>>
}

impl<ResourceId, Access, Password> Default for BlacklistStorage<ResourceId, Access, Password> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ResourceId, Access, Password> BlacklistStorage<ResourceId, Access, Password> {
    pub fn generate_password(&mut self) -> Password {
        todo!()
    }
}

impl<ResourceId: Eq + Hash, Access: PartialEq, Password: PartialEq + Clone> crate::prelude::BlacklistStorage for BlacklistStorage<ResourceId, Access, Password> {
    type Id = ResourceId;
    type Access = Access;
    type Password = Password;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        allowed_accesses.iter().any(|(allowed_access, access_password)| allowed_access == access && access_password == password)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password> {
        let generated_password = self.generate_password();

        self.inner.entry(id).or_default().push((access, generated_password.clone()));

        Some(generated_password)
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };

        let Some(position) = allowed_accesses.iter().position(|(allowed_access, _)| allowed_access == access) else { return false };

        allowed_accesses.remove(position);

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
    ) -> bool where <Self as crate::prelude::BlacklistStorage>::Id: 'a {
        !ids.any(|resource_id| !self.release(resource_id))
    }
}
