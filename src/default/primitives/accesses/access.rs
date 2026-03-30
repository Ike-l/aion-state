use crate::default::{AccessResult, Resource, StoredResource};

pub enum Access {

}

impl crate::accessor::Accessor for Access {
    type StoredValue = StoredResource;
    type Value = Resource;

    type AccessResult<'a> = AccessResult;

    fn accepts_incoming(&self, incoming_access: &Self) -> bool {
        todo!()
    }

    fn can_insert_resource(&self) -> bool {
        todo!()
    }

    fn can_remove_resource(&self) -> bool {
        todo!()
    }

    fn acquire<'a>(
        &self, 
        stored_value: &'a Self::StoredValue
    ) -> Self::AccessResult<'a> {
        todo!()
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