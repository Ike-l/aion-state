use crate::prelude::Accesses;

pub trait ReservationStorage {
    type ReserverId;

    /// Needs `AccessStorage: AccessStorage` to use crate functionality
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

    fn iter<'a>(&self) -> impl Iterator<Item = (
        &'a Self::ReserverId, 
        &'a Accesses<Self::AccessStorage>
    )> 
        where Self: 'a;
}
