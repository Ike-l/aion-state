use std::sync::Arc;

use parking_lot::{RawMutex, lock_api::Mutex};
use rand::prelude::{IteratorRandom, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{brute::{agent::Agent, property_manager::PropertyManager}, create_registry};

mod agent;
mod command;
mod property_manager;

#[test]
fn run() {
    let seed = 123;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    
    let max_threads = 20;
    // let max_threads = 10;
    let max_agents = 10;
    let max_ticks = 1000;
    
    let id_length = 10;
    let label_length = 10;
    let chance_to_be_known = 0.9; // 90%
    let capacity = 10000;
    
    let registry = Arc::new(create_registry(Some(capacity)));
    let property_manager = Arc::new(PropertyManager::new());
    
    let sync: Arc<Mutex<RawMutex, ()>> = Arc::new(Mutex::new(()));
    
    let threads = rng.random_range(1..=max_threads);
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let registry = Arc::clone(&registry);
        let property_manager = Arc::clone(&property_manager);
        
        let sync = Arc::clone(&sync);
        
        let seed: u64 = rng.random();
        
        let handle = std::thread::spawn(move || {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            
            let ticks: u32 = rng.random_range(1..=max_ticks);
            let agent_amount: u32 =  rng.random_range(1..=max_agents);
        
            let mut agents = Vec::with_capacity(agent_amount as usize);
            for _ in 0..agent_amount {
                agents.push(Agent::new(&mut rng, chance_to_be_known, id_length));
            }

            for _ in 0..ticks {
                let agent = agents.iter().choose(&mut rng).unwrap();

                let command = agent.command(
                    &registry, 
                    &mut rng,
                    label_length
                );

                {
                    let _sync = sync.lock();
                    let _result = property_manager.test(&registry, command);
                }
            }
        });
    
        handles.push(handle);
    }

    for handle in handles {
        let _r = handle.join().unwrap();
    }

    let Ok(registry) = Arc::try_unwrap(registry) else { panic!() };
    panic!("Registry: {}", serde_json::to_string(&registry).unwrap())
}