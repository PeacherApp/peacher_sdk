use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<HooksInputs>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HooksInputs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<HooksTaxInput>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HooksTaxInput {
    pub calculation: String,
}
