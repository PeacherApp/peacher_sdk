use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};
use uuid::Uuid;

use crate::{paginated, prelude::*};

/// Three fixed award tiers backed by Stripe Prices.
#[derive(
	Serialize,
	Deserialize,
	Clone,
	Copy,
	Debug,
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
pub enum AwardTier {
	Acknowledge,
	Amplify,
	Elevate,
}

impl AwardTier {
	pub fn price_cents(self) -> i32 {
		match self {
			Self::Acknowledge => 300,
			Self::Amplify => 1000,
			Self::Elevate => 2500,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAwardRequest {
	pub tier: AwardTier,
	#[serde(default)]
	pub is_anonymous: bool,
}

/// Create a new award on a content item, returning a Stripe Checkout URL.
pub struct CreateAward {
	content_item_id: Uuid,
	body: CreateAwardRequest,
}

impl CreateAward {
	pub fn new(content_item_id: Uuid, tier: AwardTier, is_anonymous: bool) -> Self {
		Self {
			content_item_id,
			body: CreateAwardRequest { tier, is_anonymous },
		}
	}
}

impl Handler for CreateAward {
	type ResponseBody = CreateAwardResponse;

	fn method(&self) -> Method {
		Method::Post
	}

	fn path(&self) -> Cow<'_, str> {
		format!("/api/content_items/{}/awards", self.content_item_id).into()
	}

	fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
		builder.json(&self.body)
	}
}

pub struct GetAwardSummary(pub Uuid);

impl GetHandler for GetAwardSummary {
	type ResponseBody = AwardSummaryView;

	fn path(&self) -> Cow<'_, str> {
		format!("/api/content_items/{}/awards/summary", self.0).into()
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams, utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[serde(default)]
pub struct ListAwardsParams {
	pub page: Option<u64>,
	pub page_size: Option<u64>,
}

paginated!(ListAwardsParams);

pub struct ListContentItemAwards {
	content_item_id: Uuid,
	pub params: ListAwardsParams,
}

impl ListContentItemAwards {
	pub fn new(content_item_id: Uuid) -> Self {
		Self {
			content_item_id,
			params: ListAwardsParams::default(),
		}
	}
}

impl GetHandler for ListContentItemAwards {
	type ResponseBody = Paginated<PublicAwardView>;

	fn path(&self) -> Cow<'_, str> {
		format!("/api/content_items/{}/awards", self.content_item_id).into()
	}

	fn params(&self) -> impl SdkParams {
		self.params.clone()
	}
}

#[derive(Default)]
pub struct ListGivenAwards {
	pub params: ListAwardsParams,
}

impl ListGivenAwards {
	pub fn new() -> Self {
		Self::default()
	}
}

impl GetHandler for ListGivenAwards {
	type ResponseBody = Paginated<GivenAwardView>;

	fn path(&self) -> Cow<'_, str> {
		"/api/account/awards/given".into()
	}

	fn params(&self) -> impl SdkParams {
		self.params.clone()
	}
}

pub struct GetReceivedAwards(pub i32);

impl GetHandler for GetReceivedAwards {
	type ResponseBody = ReceivedAwardsView;

	fn path(&self) -> Cow<'_, str> {
		format!("/api/members/{}/awards/received", self.0).into()
	}
}
