#[cfg(not(feature = "serde"))]
pub trait DeserializeOwnedAlias {}

#[cfg(feature = "serde")]
pub trait DeserializeOwnedAlias: serde::de::DeserializeOwned {}

#[cfg(feature = "serde")]
impl<T> DeserializeOwnedAlias for T where T: serde::de::DeserializeOwned {}

#[cfg(not(feature = "serde"))]
pub trait SerializeAlias {}

#[cfg(feature = "serde")]
pub trait SerializeAlias: serde::ser::Serialize {}

#[cfg(feature = "serde")]
impl<T> SerializeAlias for T where T: serde::ser::Serialize {}
