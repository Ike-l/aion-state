pub trait OwnershipStorage {
    type OwnerId;
    type ValueId;

    /// Verify if the owner holds ownership over the value
    fn verify(
        &self,
        owner_id: &Self::OwnerId,
        value_id: &Self::ValueId
    ) -> bool;
}