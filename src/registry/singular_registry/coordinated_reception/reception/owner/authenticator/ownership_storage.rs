pub trait OwnershipStorage {
    type OwnerId;
    type ValueId;

    fn owns(
        &self,
        owner_id: &Self::OwnerId,
        value_id: &Self::ValueId
    ) -> bool;
}