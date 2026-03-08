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

    /// Registers `id` with `password`
    /// 
    /// so that if `register` returns `true` then `verify` with the same input returns true
    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool;
}