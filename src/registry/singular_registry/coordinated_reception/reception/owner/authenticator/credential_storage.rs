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
}