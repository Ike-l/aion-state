use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ResourceId {
    Type(TypeId),
    Label(String)
}