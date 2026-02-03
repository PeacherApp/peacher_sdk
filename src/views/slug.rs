use std::{borrow::Cow, fmt, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RawSlug {
    normalized: bool,
    inner: String,
}

impl RawSlug {
    pub fn new(val: impl Into<String>) -> Self {
        let mut this = RawSlug::new_raw(val);
        this.normalize();
        this
    }

    pub fn new_raw(val: impl Into<String>) -> Self {
        RawSlug {
            normalized: false,
            inner: val.into(),
        }
    }

    pub fn normalize(&mut self) {
        self.inner = normalize(&self.inner);
        self.normalized = true;
    }

    pub fn as_str(&self) -> Cow<'_, str> {
        let mut value = Cow::Borrowed(self.inner.as_str());

        if value.is_empty() {
            let str = value.to_mut();
            *str = "_".to_string();
        } else if !self.normalized {
            let normalized = normalize(&value);
            *value.to_mut() = normalized;
        }

        value
    }

    pub fn to_slug(&self) -> Slug {
        Slug(self.as_str().into_owned())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Slug(String);

impl Deref for Slug {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl From<Slug> for String {
    fn from(value: Slug) -> Self {
        value.0
    }
}

impl Slug {
    pub fn new(val: impl Into<String>) -> Self {
        RawSlug::new(val).to_slug()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn normalize(val: impl AsRef<str>) -> String {
    val.as_ref()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}
