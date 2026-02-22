use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreferencesResponse {
    pub email: NotificationPreference<EmailNotificationPreferences>,
    pub in_app: NotificationPreference<InAppNotificationPreferences>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreferenceMeta {
    pub id: i32,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NotificationPreference<P> {
    /// if this is None, then the preference has never been recorded.
    pub meta: Option<NotificationPreferenceMeta>,
    pub preference: P,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum NotificationPreferenceKind {
    InApp(InAppNotificationPreferences),
    Email(EmailNotificationPreferences),
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateNotificationPreferenceResponse {
    pub updated_at: DateTime<FixedOffset>,
    pub preference: NotificationPreferenceKind,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PreferenceType {
    InApp,
    Email,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InAppNotificationPreferences {
    enabled: bool,
}
impl Default for InAppNotificationPreferences {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EmailNotificationPreferences {
    enabled: bool,
}

#[expect(clippy::derivable_impls)]
impl Default for EmailNotificationPreferences {
    fn default() -> Self {
        Self { enabled: false }
    }
}
