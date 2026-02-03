pub trait LockStorage {
    type Item;

    fn check(&self, item: &Self::Item) -> bool;
}