use crate::prelude::*;

pub struct SessionSync<'c, 's, E, P> {
    session: ExternalId,
    external: &'s E,

    mapper: &'c mut ClientMapper<'s, P>,
}

impl<'c, 's, E: ExternalClient, P: Client> SessionSync<'c, 's, E, P> {
    pub fn new(session: ExternalId, external: &'s E, mapper: &'c mut ClientMapper<'s, P>) -> Self {
        Self {
            session,
            external,
            mapper,
        }
    }

    /// Get a sync instance for a specific chamber of the session.
    pub fn members<'m>(&'m mut self, chamber: ExternalId) -> MembersSync<'m, 's, E, P>
    where
        'c: 's,
        'm: 's,
    {
        MembersSync::new(&self.session, chamber, self.external, &mut self.mapper)
    }
    pub async fn all_members<'m, F, T, Fut>(&'m mut self, mut func: F) -> SyncResult<Vec<T>>
    where
        F: FnMut(MembersSync<'m, 's, E, P>) -> Fut,
        Fut: Future<Output = SyncResult<T>>,
        'm: 'c,
        'c: 's,
    {
        let session = self.mapper.session(&self.session).await?;

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
