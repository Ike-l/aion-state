pub trait BlacklistStorage {
    type Id;
    type Access;
    type Password;

    /// Verify if the `id` `access` pair corresponds and with the password
    fn verify(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool;

    /// Register the `id` `access` pair and return the associated password
    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password>;

    /// Unregister the `id` `access` pair
    /// 
    /// Returns True if successfully found & removed the registry element
    /// 
    /// Returns False if not found & not removed the registry element
    fn block(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool;

    /// Unregestier all `access` corresponding with `id`
    /// 
    /// Returns True if successfully found & removed all registry elements
    /// 
    /// Returns False if unsuccessful
    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool;
}