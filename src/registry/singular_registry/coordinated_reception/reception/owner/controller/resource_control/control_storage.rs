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
        &mut self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool;

    /// Returns True for the `id` now successfully owns the `resource_id`
    /// 
    /// Returns False for the `id` now does not own the `resource_id`
    fn own(
        &mut self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool;

    /// Returns True if `resource_id` is owned by someone
    /// 
    /// False if not
    fn is_owned(
        &self,
        resource_id: &Self::ResourceId
    ) -> bool;

    /// Returns an iterator over all `ResourceId` associated with `id`
    /// 
    /// Must act like `id` is being removed from the collection
    fn release_id(
        &mut self,
        id: &Self::Id
    ) -> impl Iterator<Item = Self::ResourceId>;
}