pub trait BlacklistStorage {
    type Id;
    type Access;
    type Password;

    fn verify(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool;

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password>;

    fn block(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool;

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool;
}