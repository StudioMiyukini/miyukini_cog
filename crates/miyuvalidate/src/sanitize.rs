//! Tool MiyuValidate — tool.validate.sanitize.
//! Sanitise une valeur selon type et politique fournis (string, nombre, liste, échappement HTML, trim).

use crate::context::GovernedContext;
use crate::errors::MiyuValidateError;

/// Taille maximale d'une valeur à sanitiser (octets, C-13).
pub const MAX_VALUE_BYTES: usize = 256 * 1024;
/// Nombre maximal d'éléments d'une liste.
pub const MAX_LIST_ITEMS: usize = 1024;

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

/// @id miyuvalidate_tool_sanitize
/// @role security
/// @layer tool
/// @do sanitize_under_governance
//@human / @human Sanitise une valeur selon type et politique fournis.
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
    if value.len() > MAX_VALUE_BYTES {
        return Err(MiyuValidateError::InputTooLarge {
            what: "value",
            max: MAX_VALUE_BYTES,
        });
    }
    let mut s = if policy.trim {
        value.trim().to_string()
    } else {
        value.to_string()
    };
    match value_type {
        SanitizeValueType::String => {}
        SanitizeValueType::Number => {
            let n = s
                .trim()
                .parse::<f64>()
                .map_err(|_| MiyuValidateError::InvalidData("not a number".to_string()))?;
            // `f64::from_str` accepte « NaN », « inf », « infinity » : refusés.
            if !n.is_finite() {
                return Err(MiyuValidateError::InvalidData(
                    "number is not finite".to_string(),
                ));
            }
        }
        SanitizeValueType::List => {
            let items: Vec<&str> = s.split(',').collect();
            if items.len() > MAX_LIST_ITEMS {
                return Err(MiyuValidateError::InputTooLarge {
                    what: "list items",
                    max: MAX_LIST_ITEMS,
                });
            }
            s = items
                .into_iter()
                .map(|x| if policy.trim { x.trim() } else { x })
                .collect::<Vec<_>>()
                .join(",");
        }
    }
    if policy.escape_html {
        s = escape_html(&s);
    }
    Ok(s)
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 1)
    }

    fn pol(escape_html: bool, trim: bool) -> SanitizePolicy {
        SanitizePolicy { escape_html, trim }
    }

    #[test]
    fn refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(
            sanitize(&empty, "x", SanitizeValueType::String, &pol(true, true)),
            Err(MiyuValidateError::NoMandate)
        );
    }

    #[test]
    fn chaine_trim_et_echappement() {
        let r = sanitize(
            &ctx(),
            "  <a href=\"x\">O'Neil & co</a> ",
            SanitizeValueType::String,
            &pol(true, true),
        )
        .unwrap();
        assert_eq!(
            r,
            "&lt;a href=&quot;x&quot;&gt;O&#39;Neil &amp; co&lt;/a&gt;"
        );
        let r = sanitize(&ctx(), " a ", SanitizeValueType::String, &pol(false, false)).unwrap();
        assert_eq!(r, " a ");
    }

    #[test]
    fn nombre_valide_et_invalide() {
        assert_eq!(
            sanitize(
                &ctx(),
                " 42.5 ",
                SanitizeValueType::Number,
                &pol(false, true)
            )
            .unwrap(),
            "42.5"
        );
        for bad in ["abc", "NaN", "inf", "-infinity", ""] {
            assert!(
                matches!(
                    sanitize(&ctx(), bad, SanitizeValueType::Number, &pol(false, true)),
                    Err(MiyuValidateError::InvalidData(_))
                ),
                "accepté à tort : {bad}"
            );
        }
    }

    #[test]
    fn liste_trim() {
        let r = sanitize(
            &ctx(),
            " a , b,c ",
            SanitizeValueType::List,
            &pol(false, true),
        )
        .unwrap();
        assert_eq!(r, "a,b,c");
    }

    #[test]
    fn bornes_appliquees() {
        let big = "x".repeat(MAX_VALUE_BYTES + 1);
        assert!(matches!(
            sanitize(&ctx(), &big, SanitizeValueType::String, &pol(false, false)),
            Err(MiyuValidateError::InputTooLarge { what: "value", .. })
        ));
        let many = ",".repeat(MAX_LIST_ITEMS);
        assert!(matches!(
            sanitize(&ctx(), &many, SanitizeValueType::List, &pol(false, false)),
            Err(MiyuValidateError::InputTooLarge {
                what: "list items",
                ..
            })
        ));
    }
}
