use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{paginated, prelude::*};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct LegislationParams {
    #[serde(default)]
    pub id: Vec<i32>,

    pub freetext: Option<String>,
    pub legislation_type: Option<String>,

    #[serde(default)]
    pub external_id: Vec<ExternalId>,
    pub session_id: Option<i32>,
    /// Filter by active status (derived from outcome)
    pub is_active: Option<bool>,
    /// Filter by specific outcomes
    #[serde(default)]
    pub outcome: Vec<String>,
    /// id | external_id
    #[serde(default)]
    pub order_by: LegislationOrder,
    /// asc | desc
    #[serde(default)]
    pub order: Ordering,

    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

paginated!(LegislationParams);

impl LegislationParams {
    pub fn set_ids(mut self, ids: impl IntoIterator<Item = i32>) -> Self {
        self.id = ids.into_iter().collect();
        self
    }
    pub fn set_external_ids(mut self, external_ids: impl IntoIterator<Item = ExternalId>) -> Self {
        self.external_id = external_ids.into_iter().collect();
        self
    }
    pub fn set_order_by(mut self, order: LegislationOrder) -> Self {
        self.order_by = order;
        self
    }
    pub fn set_order(mut self, order: Ordering) -> Self {
        self.order = order;
        self
    }

    pub fn legislation_type(&self) -> Option<LegislationType> {
        self.legislation_type
            .as_ref()
            .and_then(|t| LegislationType::from_str(t).ok())
    }

    pub fn set_is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn set_outcomes(mut self, outcomes: impl IntoIterator<Item = LegislationStatus>) -> Self {
        self.outcome = outcomes.into_iter().map(|o| o.to_string()).collect();
        self
    }

    pub fn outcomes(&self) -> Vec<LegislationStatus> {
        self.outcome
            .iter()
            .filter_map(|o| LegislationStatus::from_str(o).ok())
            .collect()
    }
}

/// How the legislation should be ordered
#[derive(
    Serialize, Deserialize, Default, Clone, EnumString, Display, Debug, PartialEq, Eq, Copy,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LegislationOrder {
    #[default]
    Id,
    ExternalId,
}
