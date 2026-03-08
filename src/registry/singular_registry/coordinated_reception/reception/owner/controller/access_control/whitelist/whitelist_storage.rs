pub trait WhitelistStorage {
    type Id;
    type Access;

    /// Verify if the `id` `access` pair corresponds to a registered element
    fn verify(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool;

    /// Register the `id` `access` pair and return 
    /// 
    /// True if successful
    /// 
    /// False if unsuccessful
    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> bool;

    /// Acts a `block` for all accesses associated with the given `id`
    /// 
    /// returns True if successful
    /// 
    /// returns False if unsuccessful
    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool;

    /// Acts as inverse to `allow`
    /// 
    /// returns True if successful
    /// 
    /// returns False if unsuccessful
    fn block(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool;
}