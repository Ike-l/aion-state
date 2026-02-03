pub trait RegistryStorage {
    type Key;
    type Value;

    fn get(
        &self, 
        key: &Self::Key
    ) -> Option<&Self::Value>;

    fn insert(
        &mut self, 
        key: Self::Key, 
        value: Self::Value
    ) -> Option<Self::Value>; 

    fn remove(
        &mut self, 
        key: &Self::Key
    ) -> Option<Self::Value>;

    fn contains_key(
        &self, 
        key: &Self::Key
    ) -> bool;


    fn reallocates_on_next_new_insert(&self) -> bool;
}

