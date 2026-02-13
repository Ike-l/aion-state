pub trait LockStorage {
    type ValueId;

    fn check(&self, value_id: &Self::ValueId) -> bool;

    // returns if it was already locked
    fn lock(&self, value_id: &Self::ValueId) -> bool;

    // returns if it was already locked
    fn unlock(&self, value_id: &Self::ValueId) -> bool;
}