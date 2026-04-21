use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};
use uuid::Uuid;

use crate::sdk::{AwardTier, MemberView};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAwardResponse {
	/// Stripe Checkout session URL. The frontend should redirect here.
	pub checkout_url: String,
	pub award_id: i32,
}

#[derive(
	Serialize,
	Deserialize,
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	EnumString,
	Display,
	VariantArray,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AwardStatus {
	Pending,
	Succeeded,
	Failed,
	Refunded,
	Canceled,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AwardSummaryView {
	pub acknowledge: i64,
	pub amplify: i64,
	pub elevate: i64,
	pub total: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PublicAwardView {
	pub id: i32,
	pub tier: AwardTier,
	/// `None` when the award was given anonymously.
	pub giver: Option<MemberView>,
	pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GivenAwardView {
	pub id: i32,
	pub tier: AwardTier,
	pub amount_cents: i32,
	pub content_item_id: Uuid,
	pub status: AwardStatus,
	pub is_anonymous: bool,
	pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ReceivedAwardsView {
	pub acknowledge: i64,
	pub amplify: i64,
	pub elevate: i64,
	pub total: i64,
}
