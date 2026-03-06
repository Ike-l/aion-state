use crate::prelude::{Authenticator, Controller};

pub mod authenticator;
pub mod controller;

pub mod owner_result;
pub mod owner_input;

/// Applies `Authentication` semantics when ownership of the door is required, then `Door` semantics 
pub struct Owner<OS, WS, BS, CS> {
    authenticator: Authenticator<OS>,
    controller: Controller<WS, BS, CS>,
}

