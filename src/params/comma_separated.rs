use std::fmt;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use ahash::HashSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Adds supertrait requirements for things that can be in [`CommaSeparated`].
///
///
/// With the `utoipa` feature, this provides OpenAPI schema metadata for types used inside [`CommaSeparated<T>`].
///
/// Automatically implemented for all types implementing [`strum::VariantArray`],
/// which generates the description and example from the enum's variants.
///
/// This is typically implemented via the the [`commaparam`] macro
pub trait CommaSeparatable: Eq + Hash + fmt::Display + FromStr {
    /// A description fragment appended to the schema description.
    /// For enums this lists valid values; for primitives this can be empty.
    #[cfg(feature = "utoipa")]
    fn description() -> Option<String>;

    /// An example value for the comma-separated string (e.g. `"bill,resolution"`).
    #[cfg(feature = "utoipa")]
    fn example() -> String;
}

/// Implement [`ParamExample`] for an enum that derives [`strum::VariantArray`].
/// Generates schema description and example from the enum's variants.
///
/// ```ignore
/// impl_param_example_variants!(LegislationType);
/// ```
///
/// ```ignore
/// impl_param_example!(i32, "1,2,3");
/// impl_param_example!(String, "foo,bar,baz");
/// ```
#[macro_export]
macro_rules! commaparam {
    ($ty:ty) => {
        impl $crate::params::CommaSeparatable for $ty {
            #[cfg(feature = "utoipa")]
            fn description() -> Option<String> {
                let variants: Vec<String> = <$ty as strum::VariantArray>::VARIANTS
                    .iter()
                    .map(|v| v.to_string())
                    .collect();
                Some(format!("Valid values: {}", variants.join(", ")))
            }

            #[cfg(feature = "utoipa")]
            fn example() -> String {
                <$ty as strum::VariantArray>::VARIANTS
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            }
        }
    };

    ($ty:ty, $example:literal) => {
        impl $crate::params::CommaSeparatable for $ty {
            #[cfg(feature = "utoipa")]
            fn description() -> Option<String> {
                Some(format!("a $ty"))
            }

            #[cfg(feature = "utoipa")]
            fn example() -> String {
                $example.into()
            }
        }
    };
}

commaparam!(i32, "1,2,3");
commaparam!(i64, "1,2,3");
commaparam!(String, "foo,bar,baz");

/// Construct a [`CommaSeparated<T>`] from a list of values, similar to [`vec!`].
///
/// ```
/// let types = commasep![LegislationType::Bill, LegislationType::Resolution];
/// let ids = commasep![1, 2, 3];
/// let empty: CommaSeparated<i32> = commasep![];
/// ```
#[macro_export]
macro_rules! commasep {
    () => {
        $crate::params::CommaSeparated::default()
    };
    ($($val:expr),+ $(,)?) => {
        $crate::params::CommaSeparated::new([$($val),+])
    };
}

/// A `HashSet<T>` that serializes and deserializes as a comma-separated string.
///
/// The easiest way to create this struct is with the [`commasep`] macro:
/// ```
/// let types: CommaSeparated<LegislationType> = commasep![LegislationType::Bill, LegislationType::Resolution];
/// let ids: CommaSeparated<i32> = commasep![1, 2, 3];
/// let empty: CommaSeparated<i32> = commasep![];
/// ```
///
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

impl<T: CommaSeparatable + Clone> CommaSeparated<T> {
    /// helper function commonly used in the api
    pub fn iter_owned(&self) -> impl Iterator<Item = T> {
        self.0.iter().cloned()
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

impl<T: CommaSeparatable> Serialize for CommaSeparated<T> {
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

impl<'de, T: CommaSeparatable> Deserialize<'de> for CommaSeparated<T>
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
        let mut description = "Comma-separated set of values. \
             Duplicate values are ignored. Order is not guaranteed."
            .to_string();

        if let Some(extra) = T::description() {
            description.push_str("\n\n");
            description.push_str(&extra);
        }

        utoipa::openapi::schema::Object::builder()
            .schema_type(utoipa::openapi::schema::Type::String)
            .description(Some(description))
            .examples([T::example()])
            .pattern(Some("^[^,]+(,[^,]+)*$"))
            .into()
    }
}

#[cfg(feature = "utoipa")]
impl<T: CommaSeparatable> utoipa::ToSchema for CommaSeparated<T> {}

#[test]
fn comma_separated_param() {
    use pretty_assertions::assert_eq;
    use strum::{Display, EnumString};
    #[derive(
        Serialize, Debug, PartialEq, Eq, Deserialize, EnumString, Display, Hash, strum::VariantArray,
    )]
    #[serde(rename_all = "snake_case")]
    #[strum(serialize_all = "snake_case")]
    enum TestEnum {
        ValueA,
        Valueb,
        #[expect(clippy::upper_case_acronyms)]
        ABC,
    }
    commaparam!(TestEnum);

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
