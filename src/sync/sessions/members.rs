use crate::prelude::*;

pub struct MembersSync<'s, E, P> {
    session: &'s GetSessionResponse,
    external: &'s E,
    peacher: &'s P,
}
impl<'s, E: ExternalClient, P: Client> MembersSync<'s, E, P> {
    pub fn new(session: &'s GetSessionResponse, external: &'s E, peacher: &'s P) -> Self {
        Self {
            session,
            external,
            peacher,
        }
    }

    fn mapper(&self) -> ExternalIdQuery<'s, P> {
        ExternalIdQuery::new(self.peacher)
    }
}
