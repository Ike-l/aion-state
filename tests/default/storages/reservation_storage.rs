use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

use aion_state::prelude::Accesses;

pub struct ReservationStorage<ReserverId, AccessStorage> {
    inner: HashMap<ReserverId, Accesses<AccessStorage>>
}

impl<ReserverId, AccessStorage> Default for ReservationStorage<ReserverId, AccessStorage> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ReserverId: Eq + Hash, AccessStorage> aion_state::prelude::ReservationStorage for ReservationStorage<ReserverId, AccessStorage> {
    type ReserverId = ReserverId;
    type AccessStorage = AccessStorage;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut aion_state::prelude::Accesses<Self::AccessStorage>> {
        event!(Level::TRACE, "ReservationStorage get mut");

        self.inner.get_mut(key)
    }

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: aion_state::prelude::Accesses<Self::AccessStorage>
    ) -> Option<aion_state::prelude::Accesses<Self::AccessStorage>> {
        event!(Level::TRACE, "ReservationStorage insert");

        self.inner.insert(key, accesses)
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a aion_state::prelude::Accesses<Self::AccessStorage>
    )> 
        where Self: 'a {
        event!(Level::TRACE, "ReservationStorage iter");

        self.inner.iter()
    }
}