use serde::{Deserialize, Serialize};

use crate::prelude::GetHandler;

pub struct GetAccount;

impl GetHandler for GetAccount {
    type ResponseBody = AccountView;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account".into()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateAccountRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub full_name: Option<String>,
    pub handle: Option<String>,
    pub address: Option<SetLocation>,
    pub public: Option<bool>,
    pub email: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
pub struct AddressSearchParams {
    pub input: String,
}
