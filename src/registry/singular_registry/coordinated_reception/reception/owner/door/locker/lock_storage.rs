pub trait LockStorage {
    type Item;

    fn check(&self, item: &Self::Item) -> bool;

    // returns if it was already locked
    fn lock(&self, item: &Self::Item) -> bool;

    // returns if it was already locked
    fn unlock(&self, item: &Self::Item) -> bool;
}