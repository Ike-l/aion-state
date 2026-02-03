// use std::rc::Rc;
// use crate::prelude::sync::Arc;

pub trait StableAddress: sealed::Sealed {}

impl<T> StableAddress for Box<T> {}
// impl<T> StableAddress<T> for Arc<T> {}
// impl<T> StableAddress<T> for Rc<T> {}

mod sealed {
    pub trait Sealed {}

    impl<T> Sealed for Box<T> {}
}