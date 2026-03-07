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

    /// Acts a `block` for all accesses associated with the given `id`
    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool;

    /// Acts as inverse to `allow`
    fn block(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool;
}