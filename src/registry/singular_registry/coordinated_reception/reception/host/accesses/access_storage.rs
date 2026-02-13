pub trait AccessStorage {
    // ResourceId
    type AccessId;

    // Value: Accessor
    type Access;

    fn get_mut(
        &mut self, 
        key: &Self::AccessId
    ) -> Option<&mut Self::Access>;

    fn get(
        &self, 
        key: &Self::AccessId
    ) -> Option<&Self::Access>;

    fn insert(
        &mut self,
        key: Self::AccessId,
        value: Self::Access
    ) -> Option<Self::Access>;
}