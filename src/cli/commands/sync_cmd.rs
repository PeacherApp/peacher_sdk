use crate::cli::ResolvedConfig;
use crate::prelude::*;
use anyhow::Result;
use clap::Subcommand;
use tracing::info;

#[derive(Subcommand, Debug)]
pub enum SyncTarget {
    /// Perform a full sync (sessions, members, legislation)
    Full {
        /// Max pages of legislation to sync per session (default: unlimited)
        #[arg(long)]
        max_pages: Option<u64>,
    },
    /// Sync sessions from external API
    Sessions,
    /// Sync members for a session
    Members {
        /// Session ID to sync members for
        #[arg(short, long)]
        session: Option<i32>,
        /// Sync members for all sessions
        #[arg(long)]
        all: bool,
    },
    /// Sync legislation for a session
    Legislation {
        /// Session ID to sync legislation for
        #[arg(short, long)]
        session: Option<i32>,
        /// Max pages to sync (default: unlimited)
        #[arg(long)]
        max_pages: Option<u64>,
        /// Sync legislation for all sessions
        #[arg(long)]
        all: bool,
    },
    /// Sync votes for legislation
    Votes {
        /// External legislation ID to sync votes for
        #[arg(short, long)]
        legislation: Option<String>,
        /// Sync votes for all legislation in a session
        #[arg(long)]
        session: Option<i32>,
        #[arg(long, default_value_t = 0)]
        start_at_page: u64,
    },
}

pub async fn run<C: ExternalClient>(
    target: SyncTarget,
    resolved: &ResolvedConfig,
    client: C,
    injected_peacher: Option<PeacherClient>,
) -> Result<()> {
    let jurisdiction_id = resolved.require_jurisdiction()?;

    // Use injected client if provided, otherwise create from config
    let builder = match injected_peacher {
        Some(peacher_client) => ApiSyncBuilder::new(client, peacher_client),
        None => ApiSyncBuilder::new(client, resolved.to_peacher()),
    };

    let mut sync = builder
        .with_jurisdiction_id(jurisdiction_id)
        .build()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize sync: {:?}", e))?;

    match target {
        SyncTarget::Full { max_pages } => run_full_sync(&mut sync, max_pages).await,
        SyncTarget::Sessions => run_sync_sessions(&mut sync).await,
        SyncTarget::Members { session, all } => {
            if all {
                run_sync_all_members(&mut sync).await
            } else {
                let session_id = session.or(resolved.session_id).ok_or_else(|| {
                    anyhow::anyhow!("No session specified. Use --session or --all")
                })?;
                run_sync_members(&mut sync, session_id).await
            }
        }
        SyncTarget::Legislation {
            session,
            max_pages,
            all,
        } => {
            if all {
                run_sync_all_legislation(&mut sync, max_pages).await
            } else {
                let session_id = session.or(resolved.session_id).ok_or_else(|| {
                    anyhow::anyhow!("No session specified. Use --session or --all")
                })?;
                run_sync_legislation(&mut sync, session_id, max_pages).await
            }
        }
        SyncTarget::Votes {
            legislation,
            session,
            start_at_page,
        } => match (legislation, session) {
            (Some(leg_id), _) => {
                let ext_id = ExternalId::new(leg_id);
                run_sync_votes(&mut sync, &ext_id, true).await
            }
            (None, Some(session_id)) => {
                run_sync_session_votes(&mut sync, session_id, start_at_page).await
            }
            (None, None) => {
                anyhow::bail!("Must specify --legislation <ext_id> or --session <id>")
            }
        },
    }
}

async fn run_full_sync<E>(sync: &mut ApiSync<E>, max_pages: Option<u64>) -> Result<()>
where
    E: ExternalClient,
{
    println!("Starting full sync...\n");

    // Step 1: Sessions
    println!("[1/3] Syncing sessions...");
    sync.sync_sessions()
        .await
        .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;
    let session_count = sync.sessions().len();
    println!("       Synced {} sessions\n", session_count);

    if session_count == 0 {
        println!("No sessions found. Nothing more to sync.");
        return Ok(());
    }

    let session_ids: Vec<i32> = sync.sessions().iter().map(|s| s.id).collect();

    // Step 2: Members for each session
    println!("[2/3] Syncing members...");
    for (i, session_id) in session_ids.iter().enumerate() {
        print!(
            "       Session {} ({}/{})... ",
            session_id,
            i + 1,
            session_ids.len()
        );
        let result = sync
            .update_members(*session_id)
            .await
            .map_err(|e| anyhow::anyhow!("Member sync failed: {:?}", e))?;
        println!(
            "{} new, {} existing",
            result.maybe_new.len(),
            result.duplicates.len()
        );
    }
    println!();

    // Step 3: Legislation for each session
    println!("[3/3] Syncing legislation...");
    for (i, session_id) in session_ids.iter().enumerate() {
        print!(
            "       Session {} ({}/{})... ",
            session_id,
            i + 1,
            session_ids.len()
        );
        let legislation = sync
            .update_legislation_with_pagination(*session_id, max_pages)
            .await
            .map_err(|e| anyhow::anyhow!("Legislation sync failed: {:?}", e))?;
        println!("{} new legislation items", legislation.len());
    }
    println!();

    println!("Full sync completed successfully!");
    println!(
        "Note: Vote sync is not included in full sync. Use 'sync votes --legislation <ext_id>' for granular control."
    );
    Ok(())
}

async fn run_sync_sessions<E>(sync: &mut ApiSync<E>) -> Result<()>
where
    E: ExternalClient,
{
    println!("Syncing sessions...");
    sync.sync_sessions()
        .await
        .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;

    let sessions = sync.sessions();
    println!("Synced {} sessions:", sessions.len());

    for s in sessions {
        let current = if s.current { " (current)" } else { "" };
        println!("  - [{}] {}{}", s.id, s.name, current);
    }

    Ok(())
}

async fn run_sync_members<E>(sync: &mut ApiSync<E>, session_id: i32) -> Result<()>
where
    E: ExternalClient,
{
    // Ensure sessions are synced first
    if sync.sessions().is_empty() {
        println!("Syncing sessions first...");
        sync.sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;
    }

    println!("Syncing members for session {}...", session_id);
    let result = sync
        .update_members(session_id)
        .await
        .map_err(|e| anyhow::anyhow!("Member sync failed: {:?}", e))?;

    println!("Results:");
    println!("  New members:      {}", result.maybe_new.len());
    println!("  Existing members: {}", result.duplicates.len());

    Ok(())
}

async fn run_sync_all_members<E>(sync: &mut ApiSync<E>) -> Result<()>
where
    E: ExternalClient,
{
    // Ensure sessions are synced first
    if sync.sessions().is_empty() {
        println!("Syncing sessions first...");
        sync.sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;
    }

    let session_ids: Vec<i32> = sync.sessions().iter().map(|s| s.id).collect();

    println!("Syncing members for {} sessions...\n", session_ids.len());

    let mut total_new = 0;
    let mut total_existing = 0;

    for session_id in session_ids {
        print!("  Session {}... ", session_id);
        let result = sync
            .update_members(session_id)
            .await
            .map_err(|e| anyhow::anyhow!("Member sync failed: {:?}", e))?;
        println!(
            "{} new, {} existing",
            result.maybe_new.len(),
            result.duplicates.len()
        );
        total_new += result.maybe_new.len();
        total_existing += result.duplicates.len();
    }

    println!("\nTotal: {} new, {} existing", total_new, total_existing);
    Ok(())
}

async fn run_sync_legislation<E>(
    sync: &mut ApiSync<E>,
    session_id: i32,
    max_pages: Option<u64>,
) -> Result<()>
where
    E: ExternalClient,
{
    // Ensure sessions are synced first
    if sync.sessions().is_empty() {
        println!("Syncing sessions first...");
        sync.sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;
    }

    println!("Syncing legislation for session {}...", session_id);
    if let Some(max) = max_pages {
        println!("  (limiting to {} pages)", max);
    }

    let legislation = sync
        .update_legislation_with_pagination(session_id, max_pages)
        .await
        .map_err(|e| anyhow::anyhow!("Legislation sync failed: {:?}", e))?;

    println!("Synced {} new legislation items", legislation.len());

    Ok(())
}

async fn run_sync_all_legislation<E>(sync: &mut ApiSync<E>, max_pages: Option<u64>) -> Result<()>
where
    E: ExternalClient,
{
    // Ensure sessions are synced first
    if sync.sessions().is_empty() {
        println!("Syncing sessions first...");
        sync.sync_sessions()
            .await
            .map_err(|e| anyhow::anyhow!("Session sync failed: {:?}", e))?;
    }

    let session_ids: Vec<i32> = sync.sessions().iter().map(|s| s.id).collect();

    println!(
        "Syncing legislation for {} sessions...\n",
        session_ids.len()
    );

    let mut total = 0;

    for session_id in session_ids {
        print!("  Session {}... ", session_id);
        let legislation = sync
            .update_legislation_with_pagination(session_id, max_pages)
            .await
            .map_err(|e| anyhow::anyhow!("Legislation sync failed: {:?}", e))?;
        println!("{} new items", legislation.len());
        total += legislation.len();
    }

    println!("\nTotal: {} new legislation items", total);
    Ok(())
}

async fn run_sync_session_votes<E>(
    sync: &mut ApiSync<E>,
    session_id: i32,
    mut page: u64,
) -> Result<()>
where
    E: ExternalClient,
{
    info!("Syncing session votes...");
    let page_size = 20;
    let mut max_page = page + 1;

    loop {
        if page > max_page {
            break;
        }
        info!("querying page {page}/{max_page}...");
        let legislation = sync.list_legislation(session_id, page, page_size).await?;

        for legislation in legislation.data {
            if legislation.votes.is_empty()
                && let Some(external_owner) = legislation.external
            {
                run_sync_votes(sync, &external_owner.external_id, false).await?;
                info!(
                    "Updated votes for {} (id: {}; ext: {})",
                    legislation.name_id, legislation.id, external_owner.external_id,
                );
            }
        }

        max_page = legislation.num_pages;
        page += 1;
    }
    Ok(())
}

async fn run_sync_votes<E>(
    sync: &mut ApiSync<E>,
    legislation_id: &ExternalId,
    print: bool,
) -> Result<()>
where
    E: ExternalClient,
{
    if print {
        println!("Syncing votes for legislation {}...", legislation_id);
    }

    sync.update_legislation_votes(legislation_id)
        .await
        .map_err(|e| anyhow::anyhow!("Vote sync failed: {:?}", e))?;
    if print {
        println!("Votes synced successfully");
    }
    Ok(())
}
