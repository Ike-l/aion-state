pub trait PasswordStorage {
    type Password;
    type Access;

    type GenerationPolicy;

    fn check(
        &self,
        password: Option<&Self::Password>,
        access: &Self::Access
    ) -> bool;

    fn generate_password(
        &mut self,
        access: &Self::Access,
        policy: &Self::GenerationPolicy
    ) -> Option<Self::Password>;
}