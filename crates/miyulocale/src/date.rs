//! Tool MiyuLocale — tool.locale.date.format.
//! Formate une date/heure selon locale et options fournis (format court/long, timezone).

use crate::context::GovernedContext;
use crate::errors::MiyuLocaleError;
use chrono::DateTime;

/// Options de formatage date (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct DateFormatOptions {
    /// Format court ou long (ex. short, long).
    pub style: String,
    /// Timezone (ex. Europe/Paris) — ignoré pour l’instant, UTC utilisé.
    pub timezone: Option<String>,
}

/// @id: miyulocale_tool_date_format
/// @role: mutator
/// @layer: tool
/// @human: Formate une date/heure selon locale et options fournis.
/// @do: date_format_under_governance
/// tool.locale.date.format
pub fn format(
    ctx: &GovernedContext,
    timestamp_utc_ms: i64,
    locale: &str,
    options: &DateFormatOptions,
) -> Result<String, MiyuLocaleError> {
    if !ctx.has_mandate() {
        return Err(MiyuLocaleError::NoMandate);
    }
    let secs = timestamp_utc_ms / 1000;
    let nsecs = ((timestamp_utc_ms % 1000).unsigned_abs() as u32) * 1_000_000;
    let dt = DateTime::from_timestamp(secs, nsecs).ok_or(MiyuLocaleError::Unimplemented)?;
    let style = options.style.to_lowercase();
    let (fmt, _locale_used) = match (locale.trim().to_lowercase().as_str(), style.as_str()) {
        ("fr" | "fr-fr", "short") => ("%d/%m/%Y %H:%M", ()),
        ("fr" | "fr-fr", "long") => ("%A %d %B %Y à %H:%M", ()),
        ("fr" | "fr-fr", _) => ("%d/%m/%Y %H:%M", ()),
        ("en" | "en-us", "short") => ("%m/%d/%Y %H:%M", ()),
        ("en" | "en-us", "long") => ("%A, %B %d, %Y at %H:%M", ()),
        ("en" | "en-us", _) => ("%m/%d/%Y %H:%M", ()),
        (_, "short") => ("%Y-%m-%d %H:%M", ()),
        _ => ("%Y-%m-%d %H:%M", ()),
    };
    Ok(dt.format(fmt).to_string())
}
