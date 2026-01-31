//! Tool MiyuText — tool.text.sanitize.
//! Sanitise une chaîne pour affichage sécurisé (XSS, échappement HTML) ; politique fournie.

use crate::context::GovernedContext;
use crate::errors::MiyuTextError;

/// Politique de sanitization (fournie dans le flux).
#[derive(Debug, Clone, Default)]
pub struct SanitizePolicy {
    /// Échappement HTML (tags, attributs).
    pub escape_html: bool,
    /// Autoriser certains tags (whitelist vide = tout échapper).
    pub allowed_tags: Vec<String>,
}

/// @id: miyutext_tool_sanitize
/// @role: security
/// @layer: tool
/// @human: Sanitise une chaîne pour affichage sécurisé (XSS, échappement HTML).
/// @do: sanitize_under_governance
/// tool.text.sanitize — politique fournie.
pub fn sanitize(
    ctx: &GovernedContext,
    input: &str,
    policy: &SanitizePolicy,
) -> Result<String, MiyuTextError> {
    if !ctx.has_mandate() {
        return Err(MiyuTextError::NoMandate);
    }
    let mut s = input.to_string();
    if policy.allowed_tags.is_empty() {
        s = strip_all_tags(&s);
    }
    if policy.escape_html {
        s = escape_html(&s);
    }
    Ok(s)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn strip_all_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            while i < bytes.len() && bytes[i] != b'>' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}
