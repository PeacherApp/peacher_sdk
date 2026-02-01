use std::borrow::Cow;

use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// List jurisdictions with optional filters
pub struct ListJurisdictions {
    page: u64,
    page_size: u64,
    external_id: Option<String>,
}

impl ListJurisdictions {
    pub fn new() -> Self {
        Self {
            page: 1,
            page_size: 20,
            external_id: None,
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn with_external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }
}

impl Default for ListJurisdictions {
    fn default() -> Self {
        Self::new()
    }
}

impl GetHandler for ListJurisdictions {
    type ResponseBody = Paginated<GetJurisdictionResponse>;

    fn path(&self) -> Cow<'_, str> {
        "/api/jurisdictions".into()
    }

    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            page: u64,
            page_size: u64,
            #[serde(skip_serializing_if = "Option::is_none")]
            external_id: Option<String>,
        }
        Params {
            page: self.page,
            page_size: self.page_size,
            external_id: self.external_id.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateJurisdiction {
    pub name: String,
    pub external_metadata: Option<ExternalMetadata>,
}

impl CreateJurisdiction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            external_metadata: None,
        }
    }
    pub fn external_metadata(mut self, metadata: ExternalMetadata) -> Self {
        self.external_metadata = Some(metadata);
        self
    }
}

impl Handler for CreateJurisdiction {
    type ResponseBody = CreateJurisdictionResponse;

    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> Cow<'_, str> {
        "/api/jurisdictions".into()
    }
    fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
        builder.json(self)
    }
}

pub struct GetAccountJurisdictions;
impl GetHandler for GetAccountJurisdictions {
    type ResponseBody = Vec<JurisdictionView>;
    fn path(&self) -> Cow<'_, str> {
        "/api/account/jurisdictions".into()
    }
}

// #[derive(Serialize, Deserialize)]
// #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
// pub struct CreateSession {
//     pub name: String,
//     pub current: bool,
//     pub external_id: ExternalId,
//     pub url: Option<String>,
// }

// pub struct CreateSessionMember {
//     chamber_id: i32,
//     session_id: i32,
//     member: ExternalMember,
// }
// impl CreateSessionMember {
//     pub fn new(chamber_id: i32, session_id: i32, member: ExternalMember) -> Self {
//         Self {
//             chamber_id,
//             session_id,
//             member,
//         }
//     }
// }

// impl Handler for CreateSessionMember {
//     type ResponseBody = ChamberSessionMember;
//     fn method(&self) -> Method {
//         Method::Post
//     }
//     fn path(&self) -> Cow<'_, str> {
//         format!(
//             "/chamber/{}/session/{}/members",
//             self.chamber_id, self.session_id
//         )
//         .into()
//     }

//     fn request_body(&self, builder: BodyBuilder) -> BodyBuilder {
//         builder.json(&self.member)
//     }
// }

/// Get a jurisdiction by ID
pub struct GetJurisdiction(pub i32);

impl GetHandler for GetJurisdiction {
    type ResponseBody = GetJurisdictionResponse;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}", self.0).into()
    }
}

/// Get jurisdiction details with session-aware data
pub struct GetJurisdictionDetails {
    pub id: i32,
    pub session: Option<i32>,
}

impl GetJurisdictionDetails {
    pub fn new(id: i32) -> Self {
        Self { id, session: None }
    }

    pub fn with_session(mut self, session_id: i32) -> Self {
        self.session = Some(session_id);
        self
    }
}

impl GetHandler for GetJurisdictionDetails {
    type ResponseBody = GetJurisdictionDetailsResponse;
    fn path(&self) -> Cow<'_, str> {
        format!("/api/jurisdictions/{}/details", self.id).into()
    }
    fn params(&self) -> impl SdkParams {
        #[derive(Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            session: Option<i32>,
        }
        Params {
            session: self.session,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct JurisdictionChamber {
    pub name: String,
    pub external_id: ExternalId,
    pub url: Option<String>,
}
