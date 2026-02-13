pub trait LockStorage {
    type ValueId;

    fn check(&self, item: &Self::ValueId) -> bool;

    // returns if it was already locked
    fn lock(&self, item: &Self::ValueId) -> bool;

    // returns if it was already locked
    fn unlock(&self, item: &Self::ValueId) -> bool;
}