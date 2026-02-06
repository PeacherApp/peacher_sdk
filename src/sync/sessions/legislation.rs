use crate::prelude::*;

pub struct LegislationSync<'s, E, P> {
    session: ExternalId,
    external: &'s E,
    peacher: &'s P,
}

impl<'s, E: ExternalClient, P: Client> LegislationSync<'s, E, P> {
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
}
