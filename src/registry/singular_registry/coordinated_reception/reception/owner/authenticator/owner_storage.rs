pub trait OwnerStorage {
    type OwnerId;
    type OwnerPassword;

    // is on storage because value might want to be hashed etc.
    /// Verify if the password corresponds to the owner
    fn verify(
        &self,
        owner_id: &Self::OwnerId, 
        owner_password: &Self::OwnerPassword
    ) -> bool;
}