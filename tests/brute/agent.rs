use std::sync::Arc;

use rand::{distr::Alphanumeric, prelude::{Rng, RngExt}};
use rand_chacha::ChaCha8Rng;

use crate::{TestRegistry, brute::command::Command, default::prelude::{Password, ReserverId}};

pub struct Agent {
    user_details: Option<(ReserverId, Password)>
}

impl Agent {
    pub fn new(
        rng: &mut ChaCha8Rng,
        chance_to_be_known: f64, 
        id_length: usize
    ) -> Self {
        let is_known = rng.random_bool(chance_to_be_known);
        let user_details = if is_known {
            let id: String = rng.sample_iter(&Alphanumeric).take(id_length).map(char::from).collect();
            let id = ReserverId::new(id);
            let password = Password::new(rng.next_u64());

            Some((id, password))
        } else { None };
        Self {
            user_details
        }
    }

    pub fn command<'a>(
        &'a self, 
        registry: &Arc<TestRegistry>, 
        rng: &mut ChaCha8Rng,
        label_length: usize,
    ) -> Command<'a> {
        Command::choose(
            rng, 
            self.user_details.as_ref().map(|(i, p)| (i, p)), 
            registry,
            label_length
        )
    }
}