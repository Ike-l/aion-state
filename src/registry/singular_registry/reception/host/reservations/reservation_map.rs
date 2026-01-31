use std::fmt::Debug;

use crate::prelude::AccessMap;

pub trait ReservationMap<S> {
    type Reserver: Debug + PartialEq;

    type Iter<'a>: Iterator<Item = (&'a Self::Reserver, &'a AccessMap<S>)>
    where
        Self: 'a, S: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}