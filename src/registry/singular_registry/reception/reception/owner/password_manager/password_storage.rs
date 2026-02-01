pub trait PasswordStorage {
    type Password;
    type Access;

    fn check(
        &self,
        password: &Self::Password,
        access: &Self::Access
    ) -> bool;

    fn generate_password(
        &mut self,
        access: &Self::Access
    ) -> Self::Password;
}