mod utils;
pub use utils::*;

use std::borrow::Cow;

use console::{Style, StyledObject};
use indicatif::{ProgressBar, ProgressStyle};
use tabled::{
    Table, Tabled,
    settings::{Alignment, Margin, Modify, Width, style::Style as TableStyle},
};

use crate::prelude::*;
use crate::sync::{
    JurisdictionAndChambersSyncResult, LegislationSyncResult, MembersSyncResult, SessionsSyncResult,
    VotesSyncResult,
};

pub trait AsTable {
    type TableRow<'a>: Tabled
    where
        Self: 'a;
    const NAME: &str;
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>>;
    fn nest(&self) -> usize {
        0
    }
    fn print(&self) {
        let rows = self.to_table_row();

        let spacing = " ".repeat(self.nest());

        if rows.len() == 0 {
            print!("{}{}", spacing, dim(format!("No {} found", Self::NAME)));
        }

        let mut table = Table::new(rows);
        table
            .with(TableStyle::rounded())
            .with(Margin::new(self.nest(), 0, 0, 0));
        println!("{}", table);
    }
}

#[derive(Tabled)]
pub struct SessionRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Current")]
    pub current: String,
    #[tabled(rename = "Start")]
    pub starts_at: String,
    #[tabled(rename = "End")]
    pub ends_at: String,
    #[tabled(rename = "External ID")]
    pub external_id: String,
}

impl AsTable for Vec<GetSessionResponse> {
    type TableRow<'a>
        = SessionRow
    where
        Self: 'a;
    const NAME: &str = "sessions";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|s| SessionRow {
            id: s.id,
            name: s.name.clone(),
            current: if s.current {
                "●".to_string()
            } else {
                String::new()
            },
            starts_at: s
                .starts_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".to_string()),
            ends_at: s
                .ends_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".to_string()),
            external_id: s
                .external
                .as_ref()
                .map(|e| e.external_id.val_str().to_string())
                .unwrap_or_else(|| "-".to_string()),
        })
    }
}

impl GetSessionResponse {
    pub fn print(&self) {
        let current_marker = if self.current {
            format!(" {}", green("● Current"))
        } else {
            String::new()
        };

        let ext_id = self
            .external
            .as_ref()
            .map(|e| e.external_id.val_str())
            .unwrap_or("-");

        println!(
            "{}{} {} {}",
            bold(&self.name),
            current_marker,
            dim(&format!("(ID: {})", self.id)),
            cyan(&format!("[{}]", ext_id))
        );

        println!("  {} {}", dim("Jurisdiction:"), &self.jurisdiction.name);

        let starts = self
            .starts_at
            .map(|d| d.to_string())
            .unwrap_or_else(|| "-".to_string());
        let ends = self
            .ends_at
            .map(|d| d.to_string())
            .unwrap_or_else(|| "-".to_string());
        println!("  {} {} to {}", dim("Dates:"), starts, ends);

        if self.chambers.is_empty() {
            println!("  {}", dim("No chambers"));
        } else {
            println!();
            for chamber in &self.chambers {
                let chamber_ext = chamber
                    .external
                    .as_ref()
                    .map(|e| e.external_id.val_str())
                    .unwrap_or("-");

                println!(
                    "  {} {} {}",
                    bold(&chamber.chamber_name),
                    dim(&format!("(ID: {})", chamber.chamber_id)),
                    cyan(&format!("[{}]", chamber_ext))
                );

                if chamber.members.is_empty() {
                    println!("    {}", dim("No members"));
                } else {
                    let members: Vec<_> =
                        chamber.members.iter().map(|m| m.member.clone()).collect();
                    members.print();
                }
                println!();
            }
        }
    }
}

#[derive(Tabled)]
pub struct ChamberRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "External ID")]
    pub external_id: String,
}

impl AsTable for Vec<ListChamberResponse> {
    type TableRow<'a>
        = ChamberRow
    where
        Self: 'a;
    const NAME: &str = "chambers";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|c| ChamberRow {
            id: c.id,
            name: c.name.clone(),
            external_id: c
                .external
                .as_ref()
                .map(|e| e.external_id.val_str().to_string())
                .unwrap_or_else(|| "-".to_string()),
        })
    }
    fn nest(&self) -> usize {
        2
    }
}

#[derive(Tabled)]
pub struct JurisdictionRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "External ID")]
    pub external_id: String,
    #[tabled(rename = "Chambers")]
    pub chamber_count: usize,
}

#[derive(Tabled)]
pub struct MemberRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name")]
    pub display_name: String,
    #[tabled(rename = "Handle")]
    pub handle: String,
    #[tabled(rename = "Party")]
    pub party: String,
}

#[derive(Tabled)]
pub struct LegislationRow {
    #[tabled(rename = "ID")]
    pub id: i32,
    #[tabled(rename = "Name ID")]
    pub name_id: String,
    #[tabled(rename = "Title")]
    pub title: String,
    #[tabled(rename = "Type")]
    pub legislation_type: String,
    #[tabled(rename = "Status")]
    pub status: String,
    #[tabled(rename = "External ID")]
    pub external_id: String,
}

impl AsTable for Vec<LegislationView> {
    type TableRow<'a>
        = LegislationRow
    where
        Self: 'a;
    const NAME: &str = "legislation";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|l| LegislationRow {
            id: l.id,
            name_id: l.name_id.clone(),
            title: if l.title.len() > 50 {
                format!("{}...", &l.title[..47])
            } else {
                l.title.clone()
            },
            legislation_type: format!("{:?}", l.legislation_type),
            status: l.status.clone(),
            external_id: l
                .external
                .as_ref()
                .map(|e| e.external_id.val_str().to_string())
                .unwrap_or_else(|| "-".to_string()),
        })
    }
}

impl AsTable for Vec<MemberView> {
    type TableRow<'a>
        = MemberRow
    where
        Self: 'a;
    const NAME: &str = "members";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|m| MemberRow {
            id: m.id,
            display_name: m.display_name.clone(),
            handle: m.handle.clone(),
            party: m.party.name.clone(),
        })
    }
    fn nest(&self) -> usize {
        4
    }
}

impl AsTable for Vec<JurisdictionChamberView> {
    type TableRow<'a>
        = ChamberRow
    where
        Self: 'a;
    const NAME: &str = "chambers";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|c| ChamberRow {
            id: c.id,
            name: c.name.clone(),
            external_id: c
                .external
                .as_ref()
                .map(|e| e.external_id.val_str().to_string())
                .unwrap_or_else(|| "-".to_string()),
        })
    }
}

impl AsTable for Vec<GetJurisdictionResponse> {
    type TableRow<'a>
        = JurisdictionRow
    where
        Self: 'a;
    const NAME: &str = "jurisdictions";
    fn to_table_row<'a>(&'a self) -> impl ExactSizeIterator<Item = Self::TableRow<'a>> {
        self.iter().map(|j| JurisdictionRow {
            id: j.id,
            name: j.name.clone(),
            external_id: j
                .external
                .as_ref()
                .map(|e| e.external_id.to_string())
                .unwrap_or("-".to_string()),
            chamber_count: j.chambers.len(),
        })
    }

    fn print(&self) {
        if self.is_empty() {
            println!("{}", dim("No jurisdictions found"));
            return;
        }

        for j in self.iter() {
            let ext_id = j
                .external
                .as_ref()
                .map(|e| e.external_id.val_str())
                .unwrap_or("-");

            println!(
                "{} {} {}",
                bold(&j.name),
                dim(&format!("(ID: {})", j.id)),
                cyan(&format!("[{}]", ext_id))
            );

            j.chambers.print();

            println!();
        }
    }
    fn nest(&self) -> usize {
        2
    }
}

impl JurisdictionAndChambersSyncResult {
    pub fn print(&self) {
        println!();
        if self.jurisdiction_created {
            println!(
                "{} Created jurisdiction {} {}",
                green("✓"),
                bold(&self.jurisdiction_name),
                dim(&format!("(ID: {})", self.jurisdiction_id))
            );
        } else {
            println!(
                "{} Jurisdiction {} already exists {}",
                green("✓"),
                bold(&self.jurisdiction_name),
                dim(&format!("(ID: {})", self.jurisdiction_id))
            );
        }

        if !self.chambers_created.is_empty() {
            println!();
            println!(
                "{} Created {} chamber(s):",
                green("✓"),
                self.chambers_created.len()
            );
            self.chambers_created.print();
        }

        if !self.chambers_updated.is_empty() {
            println!();
            println!(
                "{} {} existing chamber(s)",
                dim("●"),
                self.chambers_updated.len()
            );
        }

        if self.chambers_created.is_empty() && self.chambers_updated.is_empty() {
            println!("{}", dim("No chamber changes"));
        }
    }
}

impl SessionsSyncResult {
    pub fn print(&self) {
        println!();
        println!("{} Sessions synced!", green("✓"));

        if !self.created.is_empty() {
            println!();
            println!("Created {} session(s):", self.created.len());
            self.created.print();
        }

        if !self.updated.is_empty() {
            println!();
            println!("Existing {} session(s):", self.updated.len());
            self.updated.print();
        }

        if self.created.is_empty() && self.updated.is_empty() {
            println!("{}", dim("No changes."));
        }
    }
}

impl MembersSyncResult {
    pub fn print(&self) {
        println!();
        println!("{} Members synced!", green("✓"));

        if !self.created.is_empty() {
            println!();
            println!("Created {} member(s):", self.created.len());
            self.created.print();
        }

        if !self.updated.is_empty() {
            println!();
            println!("Existing {} member(s):", self.updated.len());
            self.updated.print();
        }

        if self.created.is_empty() && self.updated.is_empty() {
            println!("{}", dim("No changes."));
        }
    }
}

impl LegislationSyncResult {
    pub fn print(&self) {
        println!();
        println!("{} Legislation synced!", green("✓"));

        if self.stopped_early {
            println!("{}", dim("(Stopped early - reached known items)"));
        }

        if !self.created.is_empty() {
            println!();
            println!("Created {} legislation(s):", self.created.len());
            self.created.print();
        }

        if !self.updated.is_empty() {
            println!();
            println!("Existing {} legislation(s):", self.updated.len());
            self.updated.print();
        }

        if self.created.is_empty() && self.updated.is_empty() {
            println!("{}", dim("No changes."));
        }
    }
}

impl VotesSyncResult {
    pub fn print(&self) {
        println!();
        println!("{} Votes synced!", green("✓"));

        if !self.created.is_empty() {
            println!();
            println!("Created {} vote(s): {}", self.created.len(), dim(&format!("{:?}", self.created)));
        }

        if !self.updated.is_empty() {
            println!();
            println!("Existing {} vote(s): {}", self.updated.len(), dim(&format!("{:?}", self.updated)));
        }

        if self.created.is_empty() && self.updated.is_empty() {
            println!("{}", dim("No changes."));
        }
    }
}
