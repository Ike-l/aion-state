use std::sync::Arc;

use parking_lot::{RawMutex, lock_api::Mutex};
use rand::prelude::{IteratorRandom, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use tracing::{Level, span};

use crate::{brute::{agent::Agent, property_manager::PropertyManager}, create_brute_registry};

mod agent;
mod command;
#[cfg(test)]
mod feature_commands;
mod property_manager;

#[test]
fn run() {
    let seed = 67;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    let max_threads = 20;
    let max_agents = 10;
    let max_ticks = 10000;
    
    let id_length = 10;
    let label_length = 10;
    let chance_to_be_known = 0.9; // 90%
    let capacity = 10000;
    
    let registry = Arc::new(create_brute_registry(Some(capacity)));
    let property_manager = Arc::new(Mutex::<RawMutex, _>::new(PropertyManager::new()));
    
    let threads = rng.random_range(1..=max_threads);
    let mut handles = Vec::with_capacity(threads);
    for thread in 0..threads {
        let registry = Arc::clone(&registry);
        let property_manager = Arc::clone(&property_manager);
        
        let seed: u64 = rng.random();
        
        let handle = std::thread::spawn(move || {
            let span = span!(Level::INFO, "Thread", thread = thread);
            let _enter = span.enter();

            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            
            let ticks: u32 = rng.random_range(1..=max_ticks);
            let agent_amount: u32 =  rng.random_range(1..=max_agents);
        
            let mut agents = Vec::with_capacity(agent_amount as usize);
            for _ in 0..agent_amount {
                agents.push(Agent::new(&mut rng, chance_to_be_known, id_length));
            }

            for _ in 0..ticks {
                let agent = agents.iter_mut().choose(&mut rng).unwrap();

                let mut property_manager = property_manager.lock();
                let command = agent.command(
                    &registry, 
                    &property_manager,
                    &mut rng,
                    label_length
                );

                property_manager.test(&registry, command);
            }
        });
    
        handles.push(handle);
    }

    for handle in handles {
        let _r = handle.join().unwrap();
    }

    let Ok(registry) = Arc::try_unwrap(registry) else { panic!() };
    let Ok(property_manager) = Arc::try_unwrap(property_manager) else { panic!() };
    let property_manager = property_manager.into_inner();
    assert_eq!(registry.keys().len(), registry.len());
    property_manager.assert_consistent(&registry);
}
