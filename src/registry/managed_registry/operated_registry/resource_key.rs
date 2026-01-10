use std::{hash::Hash, fmt::Debug};

pub trait ResourceKey: Debug + Hash + PartialEq + Eq {}