pub trait AccessStorage {
    type ValueId;

    // Value: Accessor
    type Access;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Access>;

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Access>;

    fn insert(
        &mut self,
        value_id: Self::ValueId,
        access: Self::Access
    ) -> Option<Self::Access>;
}