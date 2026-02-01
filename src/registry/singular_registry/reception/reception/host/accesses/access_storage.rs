pub trait AccessStorage {
    type Key;
    type Value;

    fn get_mut(
        &mut self, 
        key: &Self::Key
    ) -> Option<&mut Self::Value>;

    fn get(
        &self, 
        key: &Self::Key
    ) -> Option<&Self::Value>;

    fn insert(
        &mut self,
        key: Self::Key,
        value: Self::Value
    ) -> Option<Self::Value>;
}