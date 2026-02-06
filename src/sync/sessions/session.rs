use crate::prelude::*;

pub struct SessionSync<'s, E, P> {
    session: ExternalId,
    external: &'s E,
    peacher: &'s P,
}

impl<'s, E: ExternalClient, P: Client> SessionSync<'s, E, P> {
    pub fn new(session: ExternalId, external: &'s E, peacher: &'s P) -> Self {
        Self {
            session,
            external,
            peacher,
        }
    }
    fn mapper(&self) -> ExternalIdQuery<'s, P> {
        ExternalIdQuery::new(self.peacher)
    }

    /// Get a sync instance for a specific chamber of the session.
    pub fn members(&self, chamber: ExternalId) -> MembersSync<'_, E, P> {
        MembersSync::new(&self.session, chamber, self.external, self.peacher)
    }
    pub async fn all_members<F, T, Fut>(&self, mut func: F) -> SyncResult<Vec<T>>
    where
        F: for<'a> FnMut(MembersSync<'a, E, P>) -> Fut,
        Fut: Future<Output = SyncResult<T>>,
    {
        let session = self.mapper().session(&self.session).await?;

        let mut responses = Vec::with_capacity(session.chambers.len());

        for chamber in session.chambers {
            let Some(external) = chamber.external else {
                continue;
            };

            let sync = self.members(external.external_id);

            let result = func(sync).await?;
            responses.push(result);
        }

        Ok(responses)
    }
}
