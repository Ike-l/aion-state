pub trait WhitelistStorage {
    type Id;
    type Access;

    fn verify(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool;

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> bool;
}