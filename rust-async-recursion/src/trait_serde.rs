// trait_serde.rs

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::fmt::Debug;

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Debug,
{
    value
        .fmt(serializer)
        .map_err(|e| serde::ser::Error::custom(format!("Failed to format: {:?}", e)))
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Debug,
{
    Debug::fmt(&T::deserialize(deserializer)?, Default::default())
        .map_err(|e| serde::de::Error::custom(format!("Failed to format: {:?}", e)))
}
