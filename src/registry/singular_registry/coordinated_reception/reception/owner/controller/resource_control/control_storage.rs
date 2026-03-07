pub trait ControlStorage {
    type Id;
    type ResourceId;

    /// Returns a boolean;
    /// 
    /// True for the `id` `owns` the `resource_id`
    /// 
    /// False for the `id` does not `own` the `resource_id`
    fn verify(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;

    /// Returns a boolean;
    /// 
    /// True for it existed & removed it
    /// 
    /// False for it didn't exist & didn't remove it
    fn release(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;

    /// Returns a boolean;
    /// 
    /// True for the `id` now successfully owns the `resource_id`
    /// 
    /// False for the `id` now does not own the `resource_id`
    fn own(
        &self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool;
}