pub trait ControlStorage {
    type Id;
    type ResourceId;

    fn verify(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;

    /// Returns a booleans;
    /// 
    /// True for it existed & removed it
    /// 
    /// False for it didn't exist & didn't remove it
    fn release(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;
}