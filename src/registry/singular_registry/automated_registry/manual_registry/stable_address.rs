// use std::rc::Rc;
// use crate::prelude::sync::Arc;

pub trait StableAddress {}

impl<T> StableAddress for Box<T> {}
// impl<T> StableAddress<T> for Arc<T> {}
// impl<T> StableAddress<T> for Rc<T> {}