pub trait WhitelistStorage {
    type Id;
    type Access;

    /// Verify if the `id` `access` pair corresponds to a registered element
    fn check_access(
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

    /// Release all ids simultaneously
    /// 
    /// True if successful for all
    /// 
    /// False if fails for any
    /// 
    /// There is no invariant (yet) that requires this to be atomic
    /// 
    /// However if it was not atomic then may as well use `release` iteratively
    fn release_all<'a>(
        &mut self,
        ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as WhitelistStorage>::Id: 'a;

    /// Acts as inverse to `allow`
    /// 
    /// returns True if successful
    /// 
    /// returns False if unsuccessful
    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool;
}