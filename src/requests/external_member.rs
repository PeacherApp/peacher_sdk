use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::prelude::*;

pub trait NewMember {
    fn display_name(&self) -> String;
    fn full_name(&self) -> Option<String>;
    fn photo_url(&self) -> Option<String>;
    fn email(&self) -> Option<String>;
    fn public(&self) -> bool;
    fn bio(&self) -> String;
    fn party(&self) -> Option<String>;
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NewExternalMember {
    pub external_id: ExternalId,
    pub url: Option<String>,
    pub display_name: String,
    pub full_name: Option<String>,
    pub bio: String,
    pub party: Option<String>,
    pub photo: Option<String>,
}

impl NewMember for NewExternalMember {
    fn bio(&self) -> String {
        self.bio.to_owned()
    }
    fn display_name(&self) -> String {
        self.display_name.to_owned()
    }
    fn email(&self) -> Option<String> {
        None
    }
    fn full_name(&self) -> Option<String> {
        self.full_name.clone()
    }
    fn party(&self) -> Option<String> {
        self.party.clone()
    }
    fn photo_url(&self) -> Option<String> {
        self.photo.clone()
    }
    fn public(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalMember {
    pub external_id: ExternalId,
    pub external_update_at: Option<DateTime<FixedOffset>>,
    pub display_name: String,
    pub full_name: Option<String>,
    pub bio: String,
    pub url: Option<Url>,
    pub appointed_at: Option<NaiveDate>,
    pub vacated_at: Option<NaiveDate>,
    pub district_number: Option<i32>,
    pub party: String,
    pub photo: Option<Url>,
}

impl ExternalMember {
    pub fn to_update_member_request(&self) -> UpdateMemberRequest {
        let mut req = UpdateMemberRequest::new()
            .display_name(&self.display_name)
            .bio(&self.bio)
            .party(&self.party);

        req = req.display_name(self.display_name.clone());
        req = req.bio(self.bio.clone());
        if let Some(full_name) = &self.full_name {
            req = req.full_name(full_name);
        }
        if let Some(photo) = &self.photo {
            req = req.photo_url(photo.clone());
        }

        req
    }
    pub fn to_create_member_request(&self) -> CreateMemberRequest {
        let mut req = CreateMemberRequest::new(
            self.display_name.clone(),
            self.bio.clone(),
            self.party.clone(),
        );

        if let Some(full_name) = &self.full_name {
            req = req.full_name(full_name.clone());
        }
        if let Some(photo) = &self.photo {
            req = req.photo_url(photo.clone());
        }

        let ext_metadata = ExternalMetadata {
            external_id: self.external_id.clone(),
            externally_updated_at: self.external_update_at,
            url: self.url.clone(),
        };

        req = req.external_metadata(ext_metadata);

        req
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalMemberPhoto {
    pub url: Option<String>,
    pub photo_type: i32,
}
