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
        &self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password>;
}