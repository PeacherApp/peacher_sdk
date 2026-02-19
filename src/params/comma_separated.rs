use std::fmt;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use ahash::HashSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// A `HashSet<T>` that serializes and deserializes as a comma-separated string.
///
/// In query parameters, this appears as `field=value1,value2,value3` rather than
/// the default array-indexed format (`field[0]=value1&field[1]=value2`).
///
/// Items must implement `Display` (for serialization) and `FromStr` (for deserialization).
///
/// When the set is empty, serialization emits `None` which causes query-string
/// serializers (like `serde_qs`) to omit the field entirely.
#[derive(Debug, Clone)]
pub struct CommaSeparated<T: Eq + Hash>(pub HashSet<T>);

impl<T: Eq + Hash> CommaSeparated<T> {
    pub fn new() -> Self {
        Self(HashSet::default())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Eq + Hash> Default for CommaSeparated<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash + PartialEq> PartialEq for CommaSeparated<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq + Hash> Deref for CommaSeparated<T> {
    type Target = HashSet<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Eq + Hash> DerefMut for CommaSeparated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Eq + Hash> FromIterator<T> for CommaSeparated<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Eq + Hash + fmt::Display> Serialize for CommaSeparated<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.is_empty() {
            serializer.serialize_none()
        } else {
            let s: String = self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            serializer.serialize_str(&s)
        }
    }
}

impl<'de, T: Eq + Hash + FromStr> Deserialize<'de> for CommaSeparated<T>
where
    T::Err: fmt::Display,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Self::new());
        }
        let set: Result<HashSet<T>, _> = s
            .split(',')
            .map(|item| T::from_str(item.trim()).map_err(de::Error::custom))
            .collect();
        Ok(Self(set?))
    }
}

#[cfg(feature = "utoipa")]
impl<T: Eq + Hash> utoipa::PartialSchema for CommaSeparated<T> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::Object::builder()
            .schema_type(utoipa::openapi::schema::Type::String)
            .description(Some("Comma-separated list of values"))
            .into()
    }
}

#[cfg(feature = "utoipa")]
impl<T: Eq + Hash> utoipa::ToSchema for CommaSeparated<T> {}
