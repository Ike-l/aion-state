pub trait CredentialStorage {
    type Id;
    type Password;

    // is on storage because value might want to be hashed etc.
    /// Verify if the `Password` corresponds to the `Id` 
    fn verify(
        &self,
        id: &Self::Id, 
        password: &Self::Password
    ) -> bool;

    fn registered(&self) -> impl Iterator<Item = &Self::Id>;

    /// Registers `id` with `password`
    /// 
    /// so that if `register` returns `true` then `verify` with the same input returns true
    /// 
    /// returns false if the register failed
    /// 
    /// should fail if `id` already exists
    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool;

    /// Set the password for `id` as `new_password`
    /// 
    /// True for success
    /// 
    /// False for fail
    fn update_password(
        &mut self,
        id: &Self::Id,
        new_password: Self::Password
    ) -> bool;

    /// Unregister the user
    /// 
    /// The opposite of `register`
    /// 
    /// True for success
    /// 
    /// False for fail
    fn unregister(
        &mut self,
        id: &Self::Id
    ) -> bool;
}