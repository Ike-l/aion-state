pub trait OwnerStorage {
    type Key;
    type Value;

    // is on storage because value might want to be hashed etc.
    fn verify(
        &self,
        key: &Self::Key, 
        value: &Self::Value
    ) -> bool;
}