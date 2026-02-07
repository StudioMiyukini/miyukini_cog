//! Tools MiyuFeeds — tool.feed.atom.board, forum, topic.
//! Lecture KindMother ; décision accès = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyufeedsError;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn atom_feed_bytes(id: &str, title: &str, feed_type: &str) -> Vec<u8> {
    let id_esc = escape_xml(id);
    let title_esc = escape_xml(title);
    let now = rfc3339_utc_now();
    let xml = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{title_esc}</title>
  <id>urn:miyufeeds:{feed_type}:{id_esc}</id>
  <updated>{now}</updated>
  <generator uri="https://miukini.github.io">MiyuFeeds</generator>
</feed>"#
    );
    xml.into_bytes()
}

fn rfc3339_utc_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH")
        .as_secs();
    let days = (secs / 86400) as i64;
    let (y, mo, d) = unix_days_to_ymd(days);
    let rem = (secs % 86400) as i64;
    let h = rem / 3600;
    let mi = (rem % 3600) / 60;
    let s = rem % 60;
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

#[allow(clippy::many_single_char_names)]
fn unix_days_to_ymd(days: i64) -> (i64, i64, i64) {
    let z = days + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = mp + (if mp < 10 { 3 } else { -9 });
    let y = y + i64::from(mo <= 2);
    (y, mo, d)
}

/// @id: miyufeeds_tool_atom_board
/// @role: mutator
/// @layer: tool
/// @human: Génère un flux ATOM pour le board.
/// @do: feed_atom_board_under_governance
/// tool.feed.atom.board
pub fn atom_board(ctx: &GovernedContext, board_id: &str) -> Result<Vec<u8>, MiyufeedsError> {
    if !ctx.has_mandate() {
        return Err(MiyufeedsError::NoMandate);
    }
    let id = board_id.trim();
    let title = format!("Board {id}");
    Ok(atom_feed_bytes(id, &title, "board"))
}

/// @id: miyufeeds_tool_atom_forum
/// @role: mutator
/// @layer: tool
/// @human: Génère un flux ATOM pour un forum.
/// @do: feed_atom_forum_under_governance
/// tool.feed.atom.forum
pub fn atom_forum(ctx: &GovernedContext, forum_id: &str) -> Result<Vec<u8>, MiyufeedsError> {
    if !ctx.has_mandate() {
        return Err(MiyufeedsError::NoMandate);
    }
    let id = forum_id.trim();
    let title = format!("Forum {id}");
    Ok(atom_feed_bytes(id, &title, "forum"))
}

/// @id: miyufeeds_tool_atom_topic
/// @role: mutator
/// @layer: tool
/// @human: Génère un flux ATOM pour un topic.
/// @do: feed_atom_topic_under_governance
/// tool.feed.atom.topic
pub fn atom_topic(ctx: &GovernedContext, topic_id: &str) -> Result<Vec<u8>, MiyufeedsError> {
    if !ctx.has_mandate() {
        return Err(MiyufeedsError::NoMandate);
    }
    let id = topic_id.trim();
    let title = format!("Topic {id}");
    Ok(atom_feed_bytes(id, &title, "topic"))
}
