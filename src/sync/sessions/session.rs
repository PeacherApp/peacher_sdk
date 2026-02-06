use crate::prelude::*;

pub struct SessionSync<'s, E, P> {
    session: GetSessionResponse,
    chamber: Option<i32>,
    external: &'s E,
    peacher: &'s P,
}

impl<'s, E: ExternalClient, P: Client> SessionSync<'s, E, P> {
    pub fn new(session: GetSessionResponse, external: &'s E, peacher: &'s P) -> Self {
        Self {
            session,
            chamber: None,
            external,
            peacher,
        }
    }
    fn mapper(&self) -> ExternalIdQuery<'s, P> {
        ExternalIdQuery::new(self.peacher)
    }

    fn members(&self) -> MembersSync<'_, E, P> {
        MembersSync::new(&self.session, self.external, self.peacher)
    }
}
