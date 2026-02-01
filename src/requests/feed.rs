use std::borrow::Cow;

use crate::prelude::*;

/// Get the current user's feed
#[derive(Default, Clone, Debug)]
pub struct GetFeed(pub FeedParams);

impl GetHandler for GetFeed {
    type ResponseBody = Paginated<FeedItem>;

    fn path(&self) -> Cow<'_, str> {
        "/api/feed".into()
    }

    fn params(&self) -> impl SdkParams {
        self.0.clone()
    }
}
