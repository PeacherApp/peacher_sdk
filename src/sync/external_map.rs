use crate::prelude::*;

pub struct ExternalIdQuery<'p, P> {
    peacher: &'p P,
}
impl<'p, P: Client> ExternalIdQuery<'p, P> {
    pub(super) fn new(peacher: &'p P) -> Self {
        Self { peacher }
    }

    pub async fn chamber(&self, ext_id: &ExternalId) -> SyncResult<ListChamberResponse> {
        let mut chambers = ListChambers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;
        if chambers.data.is_empty() {
            return Err(SyncError::NotFound(ext_id.clone()));
        } else if chambers.data.len() > 1 {
            return Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, chambers
            )));
        } else {
            let value = chambers.data.swap_remove(0);
            Ok(value)
        }
    }
    pub async fn session(&self, ext_id: &ExternalId) -> SyncResult<GetSessionResponse> {
        let mut sessions =
            ListSessions(SessionParams::default().with_external_id(ext_id.val_str()))
                .request(self.peacher)
                .await?;
        if sessions.data.is_empty() {
            return Err(SyncError::NotFound(ext_id.clone()));
        } else if sessions.data.len() > 1 {
            return Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, sessions
            )));
        } else {
            let value = sessions.data.swap_remove(0);
            Ok(value)
        }
    }

    pub async fn member(&self, ext_id: &ExternalId) -> SyncResult<MemberView> {
        let mut members = ListMembers::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;
        if members.data.is_empty() {
            return Err(SyncError::NotFound(ext_id.clone()));
        } else if members.data.len() > 1 {
            return Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, members
            )));
        } else {
            let value = members.data.swap_remove(0);
            Ok(value)
        }
    }

    pub async fn jurisdiction(&self, ext_id: &ExternalId) -> SyncResult<GetJurisdictionResponse> {
        let mut jurisdictions = ListJurisdictions::default()
            .with_external_id(ext_id.val_str())
            .request(self.peacher)
            .await?;
        if jurisdictions.data.is_empty() {
            return Err(SyncError::NotFound(ext_id.clone()));
        } else if jurisdictions.data.len() > 1 {
            return Err(SyncError::internal(format!(
                "The response for getting a chamber by external id was expected to have one value. Debug:\nExternalId: {}\nResult:{:?}",
                ext_id, jurisdictions
            )));
        } else {
            let value = jurisdictions.data.swap_remove(0);
            Ok(value)
        }
    }
}
