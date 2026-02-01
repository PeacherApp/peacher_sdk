use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtChamberSessionMember {
    pub member: ExternalMember,
    pub appointed_at: Option<NaiveDate>,
    pub vacated_at: Option<NaiveDate>,
    pub district_number: Option<i32>,
}
impl ExtChamberSessionMember {
    pub fn vacated_at(mut self, vacated_at: NaiveDate) -> Self {
        self.vacated_at = Some(vacated_at);
        self
    }
    pub fn appointed_at(mut self, appointed_at: NaiveDate) -> Self {
        self.appointed_at = Some(appointed_at);
        self
    }
    pub fn district(mut self, district_number: i32) -> Self {
        self.district_number = Some(district_number);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternalMember {
    pub external_id: ExternalId,
    pub display_name: Option<String>,
    pub full_name: Option<Option<String>>,
    pub bio: Option<String>,
    pub url: Option<Option<String>>,
    //pub chamber_id: Option<ExternalId>,
    //pub district_number: Option<Option<i32>>,
    pub party: Option<String>,
    pub photo: Option<Option<String>>,
}

impl ExternalMember {
    pub fn new(external_id: impl Into<ExternalId>) -> Self {
        Self {
            external_id: external_id.into(),
            display_name: None,
            full_name: None,
            bio: None,
            url: None,
            //chamber_id: None,
            //district_number: None,
            party: None,
            photo: None,
        }
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn full_name(mut self, full_name: Option<String>) -> Self {
        self.full_name = Some(full_name);
        self
    }

    pub fn bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn url(mut self, url: Option<String>) -> Self {
        self.url = Some(url);
        self
    }

    pub fn appointed_at(self, appointed_at: Option<NaiveDate>) -> ExtChamberSessionMember {
        ExtChamberSessionMember {
            member: self,
            district_number: None,
            appointed_at,
            vacated_at: None,
        }
    }
    pub fn district_number(self, district_number: Option<i32>) -> ExtChamberSessionMember {
        ExtChamberSessionMember {
            member: self,
            district_number,
            appointed_at: None,
            vacated_at: None,
        }
    }
    pub fn vacated_at(self, vacated_at: Option<NaiveDate>) -> ExtChamberSessionMember {
        ExtChamberSessionMember {
            member: self,
            district_number: None,
            appointed_at: None,
            vacated_at,
        }
    }

    pub fn party(mut self, party: impl Into<String>) -> Self {
        self.party = Some(party.into());
        self
    }

    pub fn photo(mut self, photo: Option<String>) -> Self {
        self.photo = Some(photo);
        self
    }

    pub fn to_update_member_request(&self) -> UpdateMemberRequest {
        let mut req = UpdateMemberRequest::new();

        if let Some(display_name) = &self.display_name {
            req = req.display_name(display_name.clone());
        }
        if let Some(bio) = &self.bio {
            req = req.bio(bio.clone());
        }
        if let Some(party) = &self.party {
            req = req.party(party.clone());
        }
        if let Some(Some(full_name)) = &self.full_name {
            req = req.full_name(full_name.clone());
        }
        if let Some(Some(photo)) = &self.photo {
            req = req.photo_url(photo.clone());
        }

        req
    }
    pub fn to_create_member_request(&self) -> CreateMemberRequest {
        let mut req = CreateMemberRequest::new(
            self.display_name.clone().unwrap_or_default(),
            self.bio.clone().unwrap_or_default(),
            self.party.clone().unwrap_or_default(),
        );

        if let Some(Some(full_name)) = &self.full_name {
            req = req.full_name(full_name.clone());
        }
        if let Some(Some(photo)) = &self.photo {
            req = req.photo_url(photo.clone());
        }

        let mut ext_metadata = ExternalMetadata::new(self.external_id.clone());
        if let Some(Some(url)) = &self.url {
            ext_metadata.set_url(url.clone());
        }
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
