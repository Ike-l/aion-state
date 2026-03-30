use crate::default::{AccessResult, Resource, StoredResource};

enum BorrowType {
    Held, Instant
}

#[derive(PartialEq)]
pub enum Access {
    Shared(usize),
    Unique,
    Replace,
}

impl Access {
    fn borrow_type(&self) -> BorrowType {
        match self {
            Access::Replace |
            Access::Shared(0) => BorrowType::Instant,
            Access::Unique => BorrowType::Held,
            Access::Shared(_) => BorrowType::Held,
        }
    }
}

impl crate::accessor::Accessor for Access {
    type StoredValue = StoredResource;
    type Value = Resource;

    type AccessResult<'a> = AccessResult<'a, Resource>;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        match (self.borrow_type(), incoming_access.borrow_type()) {
            (BorrowType::Held, BorrowType::Held) => {
                match (self, incoming_access) {
                    (Access::Shared(_), Access::Shared(_)) => true,
                    _ => false
                }
            },
            (BorrowType::Held, BorrowType::Instant) => *incoming_access != Access::Replace,
            (BorrowType::Instant, _) => true,
        }
    }

    fn can_insert_resource(&self) -> bool {
        *self == Access::Replace
    }

    fn can_remove_resource(&self) -> bool {
        *self == Access::Replace
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a mut Self::StoredValue
    ) -> Self::AccessResult<'a> {
        match self {
            Access::Shared(_) => AccessResult::Shared(stored_value.get()),
            Access::Unique => AccessResult::Unique(stored_value.get_mut()),
            Access::Replace => unreachable!(),
        }
    }

    fn merge(
        &mut self,
        incoming_access: Self
    ) {
        todo!()
    }

    fn release(
        &mut self,
        other: &Self
    ) {
        todo!()
    }

    fn insert<'a>(
        &self,
        value: Self::Value
    ) -> Self::StoredValue {
        todo!()
    }

    fn remove<'a>(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue {
        todo!()
    }
}