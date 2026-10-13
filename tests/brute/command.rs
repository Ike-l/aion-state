use std::{collections::HashSet, fmt::Debug, sync::Arc};

use rand::{distr::Alphanumeric, prelude::{IndexedRandom, IteratorRandom, Rng, RngExt}};
use rand_chacha::ChaCha8Rng;

use crate::{BruteRegistry, default::prelude::{Access, Password, ReserverId, Resource, ResourceId}};

use strum::EnumCount;

#[derive(strum_macros::EnumCount)]
pub enum Command {
    CheckedReplacement {
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        access: Access,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>,
    },
    Register {
        id: ReserverId,
        password: Password,
    },
    Unregister {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
    },
    Own {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId
    },
    UpdatePassword {
        id: ReserverId,
        old_password: Password,
        credentials_match: bool,
        new_password: Password,
    },
    ReleaseResource {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    },
    ReleaseResourceAll {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_ids: Vec<ResourceId>,
    },
    AllowBlacklist {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    },
    AllowWhitelist {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    },
    UnallowBlacklist {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    },
    UnallowWhitelist {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
    },
    CheckAccess {
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
        password: Option<Password>,
    },
    Reserve {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    },
    Unreserve {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
        resource_id: ResourceId,
    },
    DrainReservations {
        id: ReserverId,
        password: Password,
        credentials_match: bool,
    },
    AcquireAccess {
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        access: Access,
        password: Option<Password>,
    },
    AcquireView {
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_access: Vec<(ResourceId, Option<Password>)>,
        access: Access,
    },
    ReallocatingReplacement {
        user_details: Option<(ReserverId, Password)>,
        credentials_match: bool,
        resource_id: ResourceId,
        resource: Option<Resource>,
        password: Option<Password>,
    },
    ContainsResource {
        resource_id: ResourceId,
    },
    IsOwned {
        resource_id: ResourceId,
    },
    CheckOwner {
        id: ReserverId,
        resource_id: ResourceId,
    },
    Len,
    GetAccess {
        resource_id: ResourceId,
    },
    Keys,
    Registered,
    #[cfg(feature = "releaser")]
    Releaser,
    #[cfg(feature = "notifier")]
    Notifier,
    #[cfg(feature = "async")]
    Async,
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::CheckedReplacement { .. } => write!(f, "Checked Replacement"),
            Command::Register { .. } => write!(f, "Register"),
            Command::Unregister { .. } => write!(f, "Unregister"),
            Command::Own { .. } => write!(f, "Own"),
            Command::UpdatePassword { .. } => write!(f, "Update Password"),
            Command::ReleaseResource { .. } => write!(f, "Release Resource"),
            Command::ReleaseResourceAll { .. } => write!(f, "Release Resource All"),
            Command::AllowBlacklist { .. } => write!(f, "Allow Blacklist"),
            Command::AllowWhitelist { .. } => write!(f, "Allow Whitelist"),
            Command::UnallowBlacklist { .. } => write!(f, "Unallow Blacklist"),
            Command::UnallowWhitelist { .. } => write!(f, "Unallow Whitelist"),
            Command::CheckAccess { .. } => write!(f, "Check Access"),
            Command::Reserve { .. } => write!(f, "Reserve"),
            Command::Unreserve { .. } => write!(f, "Unreserve"),
            Command::DrainReservations { .. } => write!(f, "Drain Reservations"),
            Command::AcquireAccess { .. } => write!(f, "Acquire Access"),
            Command::AcquireView { .. } => write!(f, "Acquire View"),
            Command::ReallocatingReplacement { .. } if cfg!(feature = "stable_deref") => write!(f, "Reallocating Replacement"),
            Command::ReallocatingReplacement { .. } => write!(f, "Checked Replacement Fallback"),
            Command::ContainsResource { .. } => write!(f, "Contains Resource"),
            Command::IsOwned { .. } => write!(f, "Is Owned"),
            Command::CheckOwner { .. } => write!(f, "Check Owner"),
            Command::Len => write!(f, "Len"),
            Command::GetAccess { .. } => write!(f, "Get Access"),
            Command::Keys => write!(f, "Keys"),
            Command::Registered => write!(f, "Registered"),
            #[cfg(feature = "releaser")]
            Command::Releaser => write!(f, "Releaser"),
            #[cfg(feature = "notifier")]
            Command::Notifier => write!(f, "Notifier"),
            #[cfg(feature = "async")]
            Command::Async => write!(f, "Async API"),
        }
    }
}

impl Command {
    pub fn index(&self) -> usize {
        match self {
            Self::CheckedReplacement { .. } => 0,
            Self::Register { .. } => 1,
            Self::Unregister { .. } => 2,
            Self::Own { .. } => 3,
            Self::UpdatePassword { .. } => 4,
            Self::ReleaseResource { .. } => 5,
            Self::ReleaseResourceAll { .. } => 6,
            Self::AllowBlacklist { .. } => 7,
            Self::AllowWhitelist { .. } => 8,
            Self::UnallowBlacklist { .. } => 9,
            Self::UnallowWhitelist { .. } => 10,
            Self::CheckAccess { .. } => 11,
            Self::Reserve { .. } => 12,
            Self::Unreserve { .. } => 13,
            Self::DrainReservations { .. } => 14,
            Self::AcquireAccess { .. } => 15,
            Self::AcquireView { .. } => 16,
            Self::ReallocatingReplacement { .. } => 17,
            Self::ContainsResource { .. } => 18,
            Self::IsOwned { .. } => 19,
            Self::CheckOwner { .. } => 20,
            Self::Len => 21,
            Self::GetAccess { .. } => 22,
            Self::Keys => 23,
            Self::Registered => 24,
            #[cfg(feature = "releaser")]
            Self::Releaser => 25,
            #[cfg(feature = "notifier")]
            Self::Notifier => 25 + cfg!(feature = "releaser") as usize,
            #[cfg(feature = "async")]
            Self::Async => 25 + cfg!(feature = "releaser") as usize + cfg!(feature = "notifier") as usize,
        }
    }

    pub const LEN: usize = Command::COUNT;

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

    fn generate_reserver_id(rng: &mut ChaCha8Rng, id_length: usize) -> ReserverId {
        let label: String = rng.sample_iter(&Alphanumeric).take(id_length).map(char::from).collect();
        ReserverId::new(label)
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

    fn get_owned_resource_id(
        rng: &mut ChaCha8Rng,
        owned_resource_ids: &[ResourceId],
        known_resource_ids: &HashSet<ResourceId>,
        label_length: usize,
    ) -> ResourceId {
        if !owned_resource_ids.is_empty() && rng.random_bool(0.75) {
            owned_resource_ids.choose(rng).unwrap().clone()
        } else {
            Self::get_resource_id(rng, known_resource_ids, label_length)
        }
    }

    fn generate_user_details(
        rng: &mut ChaCha8Rng,
        user_details: Option<(&ReserverId, &Password)>,
    ) -> (Option<(ReserverId, Password)>, bool) {
        let Some((id, correct_password)) = user_details else {
            return (None, true)
        };

        let credentials_match = rng.random_bool(0.9);
        let password = if credentials_match {
            correct_password.clone()
        } else {
            loop {
                let password = Password::new(rng.next_u64());
                if password != *correct_password {
                    break password
                }
            }
        };

        (Some((id.clone(), password)), credentials_match)
    }

    pub fn choose(
        rng: &mut ChaCha8Rng,
        user_details: Option<(&ReserverId, &Password)>,
        registry: &Arc<BruteRegistry>,
        owned_resource_ids: &[ResourceId],
        label_length: usize
    ) -> Self {
        #[cfg(any(feature = "releaser", feature = "notifier", feature = "async"))]
        if rng.random_bool(0.005) {
            let feature_idx = rng.random_range(0..Self::LEN - 25);
            return match feature_idx {
                #[cfg(feature = "releaser")]
                0 => Command::Releaser,
                #[cfg(feature = "notifier")]
                index if index == cfg!(feature = "releaser") as usize => Command::Notifier,
                #[cfg(feature = "async")]
                index if index == cfg!(feature = "releaser") as usize + cfg!(feature = "notifier") as usize => Command::Async,
                _ => unreachable!("Feature command index is out of range"),
            }
        }

        let command_idx = rng.random_range(0..25);

        match command_idx {
            0 => {
                let (user_details, credentials_match) = Self::generate_user_details(rng, user_details);
                Command::CheckedReplacement{ 
                    user_details, 
                    credentials_match,
                    access: Self::generate_access(rng),
                    resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length), 
                    resource: Self::generate_resource(rng), 
                    password: Self::generate_resource_password(rng),
                }
            },
            1 => {
                if let Some((id, password)) = user_details {
                    Command::Register { id: id.clone(), password: password.clone() }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            2 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::Own {
                        id,
                        password,
                        credentials_match,
                        resource_id: Self::get_owned_resource_id(rng, owned_resource_ids, &registry.keys().into_iter().collect(), label_length),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            3 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::Unregister { id, password, credentials_match }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            4 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, old_password) = user_details.unwrap();
                    Command::UpdatePassword {
                        id,
                        old_password,
                        credentials_match,
                        new_password: Password::new(rng.next_u64()),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            5 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::ReleaseResource {
                        id,
                        password,
                        credentials_match,
                        resource_id: Self::get_owned_resource_id(rng, owned_resource_ids, &registry.keys().into_iter().collect(), label_length),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            6 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    let keys = if owned_resource_ids.is_empty() { registry.keys() } else { owned_resource_ids.to_vec() };
                    let amount = rng.random_range(0..=keys.len().min(3));
                    Command::ReleaseResourceAll {
                        id,
                        password,
                        credentials_match,
                        resource_ids: keys.into_iter().take(amount).collect(),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            7 | 8 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    let resource_id = Self::get_owned_resource_id(rng, owned_resource_ids, &registry.keys().into_iter().collect(), label_length);
                    let access = Self::generate_access(rng);
                    if command_idx == 7 {
                        Command::AllowBlacklist { id, password, credentials_match, resource_id, access }
                    } else {
                        Command::AllowWhitelist { id, password, credentials_match, resource_id, access }
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            9 | 10 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    let resource_id = Self::get_owned_resource_id(rng, owned_resource_ids, &registry.keys().into_iter().collect(), label_length);
                    let access = Self::generate_access(rng);
                    if command_idx == 9 {
                        Command::UnallowBlacklist { id, password, credentials_match, resource_id, access }
                    } else {
                        Command::UnallowWhitelist { id, password, credentials_match, resource_id, access }
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            11 => {
                let (user_details, credentials_match) = Self::generate_user_details(rng, user_details);
                Command::CheckAccess {
                    user_details,
                    credentials_match,
                    resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
                    access: Self::generate_access(rng),
                    password: Self::generate_resource_password(rng),
                }
            },
            12 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::Reserve {
                        id,
                        password,
                        credentials_match,
                        resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            13 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::Unreserve {
                        id,
                        password,
                        credentials_match,
                        resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
                    }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            14 => {
                if let Some((id, password)) = user_details {
                    let (user_details, credentials_match) = Self::generate_user_details(rng, Some((id, password)));
                    let (id, password) = user_details.unwrap();
                    Command::DrainReservations { id, password, credentials_match }
                } else {
                    Self::choose(rng, user_details, registry, owned_resource_ids, label_length)
                }
            },
            15 => {
                let (user_details, credentials_match) = Self::generate_user_details(rng, user_details);
                let resource_id = Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length);
                let access = if rng.random_bool(0.15) { Access::Shared(0) } else { Access::Shared(1) };
                Command::AcquireAccess {
                    user_details,
                    credentials_match,
                    resource_id,
                    access,
                    password: Self::generate_resource_password(rng),
                }
            },
            16 => {
                let (user_details, credentials_match) = Self::generate_user_details(rng, user_details);
                let keys = registry.keys();
                let amount = rng.random_range(1..=3);
                let mut resource_access: Vec<(ResourceId, Option<Password>)> = Vec::with_capacity(amount);
                for _ in 0..amount {
                    let resource_id = if !resource_access.is_empty() && rng.random_bool(0.2) {
                        resource_access[rng.random_range(0..resource_access.len())].0.clone()
                    } else if keys.is_empty() {
                        Self::generate_resource_id(rng, label_length)
                    } else {
                        keys[rng.random_range(0..keys.len())].clone()
                    };
                    resource_access.push((resource_id, Self::generate_resource_password(rng)));
                }
                Command::AcquireView { user_details, credentials_match, resource_access, access: Access::Shared(1) }
            },
            17 => {
                let (user_details, credentials_match) = Self::generate_user_details(rng, user_details);
                Command::ReallocatingReplacement {
                    user_details,
                    credentials_match,
                    resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
                    resource: Self::generate_resource(rng),
                    password: Self::generate_resource_password(rng),
                }
            },
            18 => Command::ContainsResource {
                resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
            },
            19 => Command::IsOwned {
                resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
            },
            20 => {
                let id = user_details.map(|(id, _)| id.clone()).unwrap_or_else(|| Self::generate_reserver_id(rng, label_length));
                Command::CheckOwner {
                    id,
                    resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
                }
            },
            21 => Command::Len,
            22 => Command::GetAccess {
                resource_id: Self::get_resource_id(rng, &registry.keys().into_iter().collect(), label_length),
            },
            23 => Command::Keys,
            24 => Command::Registered,
            _ => unreachable!()
        }
    }
}
