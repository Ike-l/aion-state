use crate::prelude::{Authenticator, PasswordManager};

pub mod authenticator;
pub mod password_manager;

pub struct Owner {
    authenticator: Authenticator,
    password_manager: PasswordManager
}
