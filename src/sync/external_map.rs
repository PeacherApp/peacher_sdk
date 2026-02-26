use std::sync::Arc;

use ahash::HashMap;

use crate::prelude::*;
pub struct ClientMapper<'p, P> {
    peacher: &'p P,

    jurisdiction: Option<Arc<JurisdictionView>>,
    chambers: HashMap<ExternalId, Arc<GetChamberView>>,
    sessions: HashMap<ExternalId, Arc<GetSessionView>>,
    members: HashMap<ExternalId, Arc<MemberWithPartyView>>,
}

impl<'p, P: Client> ClientMapper<'p, P> {
    pub(super) fn new(peacher: &'p P) -> Self {
        Self {
            peacher,

            jurisdiction: None,
            chambers: Default::default(),
            sessions: Default::default(),
            members: Default::default(),
        }
    }
    pub fn client(&self) -> &'p P {
        self.peacher
    }

    pub async fn chamber(&mut self, ext_id: &ExternalId) -> SyncResult<Arc<GetChamberView>> {
        if let Some(chamber) = self.chambers.get(ext_id) {
            return Ok(chamber.clone());
        }

        let mut chambers = ListChambers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;
        if chambers.data.is_empty() {
            Err(SyncError::NotFound(ext_id.clone()))
        } else if chambers.data.len() > 1 {
            Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, chambers
            )))
        } else {
            let value = Arc::new(chambers.data.swap_remove(0));

            self.chambers.insert(ext_id.clone(), value.clone());
            Ok(value)
        }
    }
    pub async fn session(&mut self, ext_id: &ExternalId) -> SyncResult<Arc<GetSessionView>> {
        if let Some(session) = self.sessions.get(ext_id) {
            return Ok(session.clone());
        }

        let mut sessions =
            ListSessions(SessionParams::default().with_external_id(ext_id.val_str()))
                .request(self.peacher)
                .await?;
        if sessions.data.is_empty() {
            Err(SyncError::NotFound(ext_id.clone()))
        } else if sessions.data.len() > 1 {
            Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, sessions
            )))
        } else {
            let value = Arc::new(sessions.data.swap_remove(0));
            self.sessions.insert(ext_id.clone(), value.clone());
            Ok(value)
        }
    }

    pub async fn member(&mut self, ext_id: &ExternalId) -> SyncResult<Arc<MemberWithPartyView>> {
        if let Some(member) = self.members.get(ext_id) {
            return Ok(member.clone());
        }

        let mut members = ListMembers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;
        if members.data.is_empty() {
            Err(SyncError::NotFound(ext_id.clone()))
        } else if members.data.len() > 1 {
            Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, members
            )))
        } else {
            let value = Arc::new(members.data.swap_remove(0));
            self.members.insert(ext_id.clone(), value.clone());
            Ok(value)
        }
    }

    pub fn store_member(
        &mut self,
        id: ExternalId,
        member: MemberWithPartyView,
    ) -> Arc<MemberWithPartyView> {
        let m = Arc::new(member);
        self.members.insert(id, m.clone());
        m
    }

    pub fn store_jurisdiction(&mut self, jurisdiction: JurisdictionView) -> Arc<JurisdictionView> {
        let j = Arc::new(jurisdiction);
        self.jurisdiction = Some(j.clone());
        j
    }

    pub async fn jurisdiction(&mut self, ext_id: &ExternalId) -> SyncResult<Arc<JurisdictionView>> {
        if let Some(jurisdiction) = self.jurisdiction.as_ref() {
            return Ok(jurisdiction.clone());
        }
        let mut jurisdictions =
            ListJurisdictions(JurisdictionParams::default().with_external_id(ext_id.val_str()))
                .request(self.peacher)
                .await?;
        if jurisdictions.data.is_empty() {
            Err(SyncError::NotFound(ext_id.clone()))
        } else if jurisdictions.data.len() > 1 {
            Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, jurisdictions
            )))
        } else {
            let value = Arc::new(jurisdictions.data.swap_remove(0).into_jurisdiction_view());
            self.jurisdiction = Some(value.clone());
            Ok(value)
        }
    }
}
