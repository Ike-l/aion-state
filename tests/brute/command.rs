use std::{collections::HashSet, sync::Arc};

use rand::{distr::Alphanumeric, prelude::{IteratorRandom, Rng, RngExt}};
use rand_chacha::ChaCha8Rng;

use crate::{TestRegistry, default::prelude::{Access, Password, ReserverId, Resource, ResourceId}};

pub enum Command<'a> {
    CheckedReplacement{
        user_details: Option<(&'a ReserverId, &'a Password)>,
        access: Access,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>,
    },
}

impl<'a> Command<'a> {
    const LEN: usize = 1;

    fn generate_access(rng: &mut ChaCha8Rng) -> Access {
        let r = rng.random_range(0..3);
        match r {
            0 => Access::Replace,
            1 => Access::Unique,
            2 => {
                let s = if rng.random_bool(0.5) {
                    1
                } else {
                    rng.random_range(0..3)
                };
                Access::Shared(s)
            },
            _ => unreachable!()
        }
    }

    fn generate_resource_id(rng: &mut ChaCha8Rng, label_length: usize) -> ResourceId {
        let label: String = rng.sample_iter(&Alphanumeric).take(label_length).map(char::from).collect();
        ResourceId::new_label(label)
    }

    fn get_resource_id(rng: &mut ChaCha8Rng, known_resource_ids: &HashSet<ResourceId>, label_length: usize) -> ResourceId {
        let resources = known_resource_ids.len();
        if resources > 0 && rng.random_bool(0.5) {
            known_resource_ids.iter().choose(rng).unwrap().clone()
        } else {
            Self::generate_resource_id(rng, label_length)
        }
    }

    fn generate_resource(rng: &mut ChaCha8Rng) -> Option<Resource> {
        if rng.random_bool(0.75) {
            let label: String = rng.sample_iter(&Alphanumeric).take(10).map(char::from).collect();
            return Some(Resource::new(label))
        }
        
        None
    }

    fn generate_resource_password(rng: &mut ChaCha8Rng) -> Option<Password> {
        if rng.random_bool(0.95) {
            return None
        }

        return Some(Password::new(rng.next_u64()))
    }

    pub fn choose(
        rng: &mut ChaCha8Rng,
        user_details: Option<(&'a ReserverId, &'a Password)>,
        registry: &Arc<TestRegistry>,
        label_length: usize
    ) -> Self {
        let command_idx = rng.random_range(0..Self::LEN);

        match command_idx {
            0 => {
                Command::CheckedReplacement{ 
                    user_details, 
                    access: Self::generate_access(rng),
                    resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length), 
                    resource: Self::generate_resource(rng), 
                    password: Self::generate_resource_password(rng),
                }
            },
            _ => unreachable!()
        }
    }
}