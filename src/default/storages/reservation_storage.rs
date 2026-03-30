use std::{collections::HashMap, hash::Hash};

use crate::prelude::Accesses;

pub struct ReservationStorage<ReserverId, AccessStorage> {
    inner: HashMap<ReserverId, Accesses<AccessStorage>>
}

impl<ReserverId, AccessStorage> Default for ReservationStorage<ReserverId, AccessStorage> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ReserverId: Eq + Hash, AccessStorage> crate::prelude::ReservationStorage for ReservationStorage<ReserverId, AccessStorage> {
    type ReserverId = ReserverId;
    type AccessStorage = AccessStorage;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut crate::prelude::Accesses<Self::AccessStorage>> {
        self.inner.get_mut(key)
    }

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: crate::prelude::Accesses<Self::AccessStorage>
    ) -> Option<crate::prelude::Accesses<Self::AccessStorage>> {
        self.inner.insert(key, accesses)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a crate::prelude::Accesses<Self::AccessStorage>
    )> 
        where Self: 'a {
        self.inner.iter()
    }
}