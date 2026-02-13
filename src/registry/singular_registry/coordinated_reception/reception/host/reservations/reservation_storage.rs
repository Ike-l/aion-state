use crate::prelude::Accesses;

pub trait ReservationStorage {
    // ReserverId
    type ReserverId;

    // AccessStorage: AccessStorage
    type AccessStorage;

    fn get_mut(
        &mut self, 
        key: &Self::ReserverId
    ) -> Option<&mut Accesses<Self::AccessStorage>>;

    fn insert(
        &mut self,
        key: Self::ReserverId,
        accesses: Accesses<Self::AccessStorage>
    ) -> Option<Accesses<Self::AccessStorage>>;

    type Iter<'a>: Iterator<Item = (&'a Self::ReserverId, &'a Accesses<Self::AccessStorage>)>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}
