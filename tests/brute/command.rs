use std::sync::Arc;

use rand::prelude::RngExt;
use rand_chacha::ChaCha8Rng;

use crate::{TestRegistry, default::prelude::{Password, ReserverId}};

#[derive(small_iter_fields::LenFields)]
pub enum Command {
    Dummy,
}

impl Command {
    pub fn execute(&self) {
        todo!()
    }

    pub fn choose(
        rng: &mut ChaCha8Rng,
        id: &Option<ReserverId>,
        password: &Option<Password>,
        registry: &Arc<TestRegistry>
    ) -> Self {
        let command_idx = rng.random_range(0..Self::len());

        match command_idx {
            0 => Command::Dummy,
            _ => unreachable!()
        }
    }
}