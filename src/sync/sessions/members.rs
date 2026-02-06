use ahash::HashMap;
use tracing::info;

use crate::prelude::*;

pub struct MembersSync<'caller, 'chamber, 'clients, E, P> {
    session_external_id: &'caller ExternalId,
    chamber_external_id: &'chamber ExternalId,
    external: &'caller E,
    mapper: &'caller mut ClientMapper<'clients, P>,
}
impl<'caller, 'chamber, 'client, E: ExternalClient, P: Client>
    MembersSync<'caller, 'chamber, 'client, E, P>
{
    pub fn new(
        session_external_id: &'caller ExternalId,
        chamber_external_id: &'chamber ExternalId,
        external: &'caller E,
        mapper: &'caller mut ClientMapper<'client, P>,
    ) -> Self {
        Self {
            session_external_id,
            chamber_external_id,
            external,
            mapper,
        }
    }

    /// Sync the members for this session and chamber pair
    pub async fn sync(&mut self) -> SyncResult<MembersSyncResult> {
        let session = self.mapper.session(self.session_external_id).await?;
        let chamber = self.mapper.chamber(self.chamber_external_id).await?;

        info!(
            "Syncing members for session {} chamber {}",
            session.id, chamber.id
        );

        // Get members from external source
        let external_members = self
            .external
            .list_members(self.session_external_id, self.chamber_external_id)
            .await?;

        info!("here 1.5");
        // Get existing members - we need to check by external_id
        // Note: The current API doesn't have a session/chamber filter on ListMembers
        // We'll get all members and filter, or use the chamber session endpoint
        let chamber_session = GetChamberSession::new(chamber.id, session.id)
            .request(self.mapper.client())
            .await?;
        info!("here 2");

        let known_members = chamber_session
            .members
            .into_iter()
            .filter_map(|m| {
                let external_id = m.external.as_ref()?.external_id.clone();

                Some((external_id, m))
            })
            .collect::<HashMap<_, _>>();

        let mut created = Vec::new();
        let mut updated = Vec::new();

        for ext_member in external_members {
            match known_members.get(&ext_member.member.external_id) {
                Some(member) => {
                    let update_req = ext_member.member.to_update_member_request();
                    let member = UpdateMember::new(member.member.id, update_req)
                        .request(self.mapper.client())
                        .await?;

                    let member = self
                        .mapper
                        .store_member(ext_member.member.external_id.clone(), member);

                    //TODO: need to update appointed at, expunged at, and district id.
                    updated.push(member);
                }
                None => {
                    let (member, is_new) =
                        match self.mapper.member(&ext_member.member.external_id).await {
                            Ok(member) => (member, false),
                            Err(SyncError::NotFound(id)) => {
                                // Create new member
                                let create_req = ext_member.member.to_create_member_request();
                                let member = CreateMember::new(create_req)
                                    .request(self.mapper.client())
                                    .await?;

                                let member = self.mapper.store_member(id.clone(), member);

                                info!(
                                    "Created member '{}' (id: {}, ext_id: {})",
                                    member.display_name, member.id, id
                                );
                                (member, true)
                            }
                            Err(e) => return Err(e),
                        };

                    // since the member is not known, they need to be linked.
                    let mut link_req = LinkMemberToChamber::new(chamber.id, session.id, member.id);

                    link_req = link_req
                        .appointed_at(ext_member.appointed_at)
                        .expunged_at(ext_member.vacated_at);
                    link_req.set_district(ext_member.district_number);
                    link_req.request(self.mapper.client()).await?;

                    if is_new {
                        created.push(member);
                    } else {
                        let update_req = ext_member.member.to_update_member_request();

                        let member = UpdateMember::new(member.id, update_req)
                            .request(self.mapper.client())
                            .await?;

                        let member = self
                            .mapper
                            .store_member(ext_member.member.external_id.clone(), member);

                        updated.push(member);
                    }
                }
            }
        }

        info!(
            "Members sync complete: {} created, {} updated",
            created.len(),
            updated.len()
        );

        Ok(MembersSyncResult { created, updated })
    }
}
