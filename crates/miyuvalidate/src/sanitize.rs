//! Tool MiyuValidate — tool.validate.sanitize.
//! Sanitise une valeur selon type et politique fournis (string, nombre, liste, échappement HTML, trim).

use crate::context::GovernedContext;
use crate::errors::MiyuValidateError;

/// Type de valeur à sanitiser (fourni dans le flux).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanitizeValueType {
    String,
    Number,
    List,
}

/// Politique de sanitization (fournie dans le flux).
#[derive(Debug, Clone, Default)]
pub struct SanitizePolicy {
    pub escape_html: bool,
    pub trim: bool,
}

/// @id: miyuvalidate_tool_sanitize
/// @role: security
/// @layer: tool
/// @human: Sanitise une valeur selon type et politique fournis.
/// @do: sanitize_under_governance
/// tool.validate.sanitize
pub fn sanitize(
    ctx: &GovernedContext,
    value: &str,
    value_type: SanitizeValueType,
    policy: &SanitizePolicy,
) -> Result<String, MiyuValidateError> {
    if !ctx.has_mandate() {
        return Err(MiyuValidateError::NoMandate);
    }
    let mut s = value.to_string();
    if policy.trim {
        s = s.trim().to_string();
    }
    match value_type {
        SanitizeValueType::String => {}
        SanitizeValueType::Number => {
            let _ = s.parse::<f64>().map_err(|_| MiyuValidateError::InvalidData("not a number".to_string()))?;
        }
        SanitizeValueType::List => {
            s = s
                .split(',')
                .map(|x| if policy.trim { x.trim() } else { x })
                .collect::<Vec<_>>()
                .join(",");
        }
    }
    if policy.escape_html {
        s = s
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;");
    }
    Ok(s)
}
