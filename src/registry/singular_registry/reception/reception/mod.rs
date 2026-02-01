use crate::prelude::{Host, Owner};

pub mod host;
pub mod owner;

pub struct Reception<RS, AS> {
    owner: Owner,
    host: Host<RS, AS>
}