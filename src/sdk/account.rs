use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub struct GetAccount;

impl GetHandler for GetAccount {
    type ResponseBody = AccountView;
    fn path(&self) -> std::borrow::Cow<'_, str> {
        "/api/account".into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AccountView {
    pub member: MemberView,
    pub member_location: Option<ViewerLocationResponse>,
    pub ban: Option<BanInfo>,
}
