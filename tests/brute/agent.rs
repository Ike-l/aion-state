use std::sync::Arc;

use rand::{distr::Alphanumeric, prelude::{Rng, RngExt}};
use rand_chacha::ChaCha8Rng;

use crate::{TestRegistry, brute::command::Command, default::prelude::{Password, ReserverId}};

pub struct Agent {
    id: Option<ReserverId>,
    password: Option<Password>,
}

impl Agent {
    pub fn new(
        rng: &mut ChaCha8Rng,
        chance_to_be_known: f64, 
        agent_amount: u32, 
        id_length: usize
    ) -> Self {
        let is_known = rng.random_bool(chance_to_be_known * agent_amount as f64);
        let (id, password) = if is_known {
            let id: String = rng.sample_iter(&Alphanumeric).take(id_length).map(char::from).collect();
            let id = Some(ReserverId::new(id));
            let password = Some(Password::new(rng.next_u64()));

            (id, password)
        } else { (None, None) };
        Self {
            id,
            password,
        }
    }

    pub fn command<'a>(&self, registry: &'a Arc<TestRegistry>, rng: &mut ChaCha8Rng) -> Command {
        Command::choose(rng, &self.id, &self.password, registry)
    }
}