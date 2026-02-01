use tracing::{error, info};

use crate::prelude::*;

pub struct LegislationSync {
    session_ext_id: ExternalId,
    page: u64,
    page_size: u64,
    order_by: ExtOrder,
    max_page: Option<u64>,
    /// Legislation that needs to be synced
    new_legislation: Vec<ExternalLegislation>,
}

impl LegislationSync {
    pub fn new(session_id: ExternalId, max_page: Option<u64>) -> Self {
        Self {
            session_ext_id: session_id,
            page: 0,
            max_page,
            order_by: ExtOrder::Latest,
            page_size: 20,
            new_legislation: Vec::new(),
        }
    }

    /// Run one iteration of the sync loop.
    ///
    /// Returns `Status::Finished` when all pages have been fetched.
    /// Returns `Status::NeedsAnotherLoop` if more pages should be fetched.
    ///
    /// Note: This method no longer queries Peacher for existing legislation.
    /// The API handles upsert, so all fetched legislation is added and
    /// will be created or updated as needed.
    pub async fn run_loop<E>(
        &mut self,
        external_client: &mut E,
        _peacher_client: &PeacherClient,
    ) -> SdkResult<Status>
    where
        E: ExternalClient,
    {
        info!(
            "Fetching page {} with page_size {}",
            self.page, self.page_size
        );
        let ext_legislation = match external_client
            .fetch_legislation(&ExtLegislationQuery {
                session_id: self.session_ext_id.clone(),
                order_by: self.order_by,
                page: self.page,
                page_size: self.page_size,
            })
            .await
        {
            Ok(r) => r,
            Err(SdkError::Unsupported(_)) if matches!(self.order_by, ExtOrder::Latest) => {
                self.order_by = ExtOrder::Earliest;
                external_client
                    .fetch_legislation(&ExtLegislationQuery {
                        session_id: self.session_ext_id.clone(),
                        order_by: self.order_by,
                        page: self.page,
                        page_size: self.page_size,
                    })
                    .await?
            }
            Err(e) => {
                error!(
                    "Error fetching external legislation on page {}, {e}",
                    self.page
                );
                return Err(e);
            }
        };

        if ext_legislation.data.is_empty() {
            return Ok(Status::Finished);
        }

        // For now, add all legislation to the new_legislation list
        // The API handles upsert, so duplicates will be updated
        for ext_leg in ext_legislation.data {
            self.new_legislation.push(ext_leg);
        }

        // Check if we've reached the end or max page
        if ext_legislation.page >= ext_legislation.num_pages.saturating_sub(1)
            || self
                .max_page
                .is_some_and(|max_page| ext_legislation.page >= max_page)
        {
            return Ok(Status::Finished);
        }

        self.page += 1;
        Ok(Status::NeedsAnotherLoop)
    }

    pub fn drain_new_legislation(&mut self) -> impl Iterator<Item = ExternalLegislation> {
        self.new_legislation.drain(..)
    }

    /// Get a reference to the new legislation collected so far
    #[expect(dead_code)]
    pub fn new_legislation(&self) -> &[ExternalLegislation] {
        &self.new_legislation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Finished,
    NeedsAnotherLoop,
}
