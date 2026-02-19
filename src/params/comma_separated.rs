use std::fmt;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use ahash::HashSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

pub trait CommaSeparatable: Eq + Hash + fmt::Display + FromStr {}
impl<T: Eq + Hash + fmt::Display + FromStr> CommaSeparatable for T {}

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
pub struct CommaSeparated<T: CommaSeparatable>(pub HashSet<T>);

impl<T: CommaSeparatable> Default for CommaSeparated<T> {
    fn default() -> Self {
        Self(HashSet::default())
    }
}

impl<T: CommaSeparatable> CommaSeparated<T> {
    pub fn new(values: impl IntoIterator<Item = T>) -> Self {
        Self(values.into_iter().collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: CommaSeparatable> PartialEq for CommaSeparated<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: CommaSeparatable> Deref for CommaSeparated<T> {
    type Target = HashSet<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: CommaSeparatable> DerefMut for CommaSeparated<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: CommaSeparatable> FromIterator<T> for CommaSeparated<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: CommaSeparatable + fmt::Display> Serialize for CommaSeparated<T> {
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

impl<'de, T: CommaSeparatable + FromStr> Deserialize<'de> for CommaSeparated<T>
where
    T::Err: fmt::Display,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(Self::default());
        }
        let set: Result<HashSet<T>, _> = s
            .split(',')
            .map(|item| T::from_str(item.trim()).map_err(de::Error::custom))
            .collect();
        Ok(Self(set?))
    }
}

#[cfg(feature = "utoipa")]
impl<T: CommaSeparatable> utoipa::PartialSchema for CommaSeparated<T> {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::Object::builder()
            .schema_type(utoipa::openapi::schema::Type::String)
            .description(Some("Comma-separated list of values"))
            .into()
    }
}

#[cfg(feature = "utoipa")]
impl<T: CommaSeparatable> utoipa::ToSchema for CommaSeparated<T> {}

#[test]
fn comma_separated_param() {
    use pretty_assertions::assert_eq;
    use strum::{Display, EnumString};
    #[derive(Serialize, Debug, PartialEq, Eq, Deserialize, EnumString, Display, Hash)]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    enum TestEnum {
        ValueA,
        Valueb,
        #[expect(clippy::upper_case_acronyms)]
        ABC,
    }

    #[derive(Serialize, Deserialize)]
    struct TestParams {
        q: CommaSeparated<TestEnum>,
    }
    let mut enums = HashSet::default();
    enums.insert(TestEnum::ValueA);
    enums.insert(TestEnum::Valueb);

    let serialize = |params: TestParams| serde_qs::to_string(&params).unwrap();

    let params = serialize(TestParams {
        q: CommaSeparated(enums),
    });

    assert!(
        params == "q=value_a%2Cvalueb" || params == "q=valueb%2Cvalue_a",
        "unexpected serialization: {params}"
    );

    let params = serialize(TestParams {
        q: CommaSeparated::default(),
    });

    assert_eq!(params, "");

    assert_eq!(
        serialize(TestParams {
            q: CommaSeparated::new([TestEnum::ABC])
        }),
        "q=abc"
    );
}
