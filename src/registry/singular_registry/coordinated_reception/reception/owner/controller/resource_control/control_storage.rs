pub trait ControlStorage {
    type Id;
    type ResourceId;

    fn verify(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;
}