use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ReserverId {
    label: String
}

impl Serialize for ReserverId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(&self.label)
    }
}

impl<'de> serde::Deserialize<'de> for ReserverId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        Ok(Self { label })
    }
}

impl ReserverId {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            label: label.into()
        }
    }
}