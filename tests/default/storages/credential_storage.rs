use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CredentialStorage<Id: Hash + Eq, Password> {
    inner: HashMap<Id, Password>
}

impl<Id: Hash + Eq, Password> Default for CredentialStorage<Id, Password> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<Id: Eq + Hash, Password: PartialEq> aion_state::prelude::CredentialStorage for CredentialStorage<Id, Password> {
    type Id = Id;
    type Password = Password;

    fn verify(
        &self,
        id: &Self::Id, 
        password: &Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage verify");

        self.inner.get(id).is_some_and(|registered_password| registered_password == password)
    }

    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage register");

        if self.inner.contains_key(&id) {
            return false
        }
        
        self.inner.insert(id, password).is_none()
    }

    fn update_password(
        &mut self,
        id: &Self::Id,
        new_password: Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage update password");

        let Some(old_password) = self.inner.get_mut(id) else { return false };

        *old_password = new_password;

        true
    }

    fn unregister(
        &mut self,
        id: &Self::Id
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage unregister");

        self.inner.remove(id).is_some()
    }
}