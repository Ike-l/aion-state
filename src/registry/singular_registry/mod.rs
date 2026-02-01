use crate::prelude::{AutomatedRegistry, CoordinatedReception};

pub mod automated_registry;
pub mod reception;
pub mod singular_registry_result;
pub mod singular_registry_input;

pub struct SingularRegistry<S, RS, AS, OS> {
    automated_registry: AutomatedRegistry<S>,
    reception: CoordinatedReception<RS, AS, OS>,
}

// impl<
//     S: RegistryStorage,
//     RS: ReservationStorage,
//     AS: AccessStorage,
//     OS: OwnerStorage<Value = S::Key>,
// > SingularRegistry<S, RS, AS, OS> 
// {

// }