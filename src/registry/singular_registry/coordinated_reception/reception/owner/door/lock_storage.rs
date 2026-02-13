pub trait LockStorage {
    type ValueId;

    /// Checks if the value is locked
    fn check(
        &self, 
        value_id: &Self::ValueId
    ) -> bool;

    /// Lock the value
    /// 
    /// returns if it was already locked
    fn lock(
        &self, 
        value_id: &Self::ValueId
    ) -> bool;

    /// Unlock the value
    /// 
    /// returns if it was already locked
    fn unlock(
        &self, 
        value_id: &Self::ValueId
    ) -> bool;
}