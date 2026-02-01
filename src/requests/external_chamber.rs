use crate::prelude::*;
use std::borrow::Cow;

pub struct GetChamberSession {
    session_id: i32,
    chamber_id: i32,
}
impl GetChamberSession {
    pub fn new(session_id: i32, chamber_id: i32) -> Self {
        Self {
            session_id,
            chamber_id,
        }
    }
}
impl GetHandler for GetChamberSession {
    type ResponseBody = ChamberSessionView;
    fn path(&self) -> Cow<'_, str> {
        format!(
            "/api/sessions/{}/chambers/{}",
            self.session_id, self.chamber_id
        )
        .into()
    }
}
