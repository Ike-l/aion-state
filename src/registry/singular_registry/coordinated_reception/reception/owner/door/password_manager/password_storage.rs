pub trait PasswordStorage {
    type Password;
    type Access;

    // i.e If want "Global" semantics for an access, but still want it to be locked
    // ^i.e if Owner wants to restrict access so noone can get a "Unique" access can make the "Shared" access password `None`
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