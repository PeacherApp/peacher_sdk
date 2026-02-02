use std::borrow::Cow;

use console::{Style, StyledObject};
use indicatif::{ProgressBar, ProgressStyle};

pub fn dim<D>(text: D) -> String
where
    StyledObject<D>: ToString,
{
    Style::new().dim().apply_to(text).to_string()
}

pub fn green<D>(text: D) -> String
where
    StyledObject<D>: ToString,
{
    Style::new().green().apply_to(text).to_string()
}

pub fn yellow<D>(text: D) -> String
where
    StyledObject<D>: ToString,
{
    Style::new().yellow().apply_to(text).to_string()
}

pub fn cyan<D>(text: D) -> String
where
    StyledObject<D>: ToString,
{
    Style::new().cyan().apply_to(text).to_string()
}

pub fn bold<D>(text: D) -> String
where
    StyledObject<D>: ToString,
{
    Style::new().bold().apply_to(text).to_string()
}

pub fn spinner(message: impl Into<Cow<'static, str>>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(message);
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

/// Finish spinner with success message
pub fn spinner_success(pb: &ProgressBar, message: &str) {
    pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", green("✓"), message));
}

/// Finish spinner with error message
pub fn spinner_error(pb: &ProgressBar, message: &str) {
    pb.set_style(ProgressStyle::default_spinner().template("{msg}").unwrap());
    pb.finish_with_message(format!("{} {}", Style::new().red().apply_to("✗"), message));
}
