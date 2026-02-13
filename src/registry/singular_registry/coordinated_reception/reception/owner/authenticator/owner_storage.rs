pub trait OwnerStorage {
    type OwnerId;
    type OwnerPassword;

    // is on storage because value might want to be hashed etc.
    fn verify(
        &self,
        key: &Self::OwnerId, 
        value: &Self::OwnerPassword
    ) -> bool;
}