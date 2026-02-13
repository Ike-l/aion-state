use std::rc::Rc;
use crate::prelude::sync::Arc;

/// The instance can be moved with existing references to the underlying data
pub trait StableAddress: sealed::Sealed {}

impl<T> StableAddress for Box<T> {}
impl<T> StableAddress for Arc<T> {}
impl<T> StableAddress for Rc<T> {}

mod sealed {
    use super::{Rc, Arc};

    pub trait Sealed {}

    impl<T> Sealed for Box<T> {}
    impl<T> Sealed for Arc<T> {}
    impl<T> Sealed for Rc<T> {}
}