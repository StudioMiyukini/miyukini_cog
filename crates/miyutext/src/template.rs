//! Tool MiyuText — tool.text.template.apply.
//! Substitue des placeholders (ex. {{ name }}) dans un template avec des données fournies.

use crate::context::GovernedContext;
use crate::errors::MiyuTextError;
use std::collections::HashMap;

/// @id: miyutext_tool_template_apply
/// @role: mutator
/// @layer: tool
/// @human: Substitue des placeholders dans un template avec des données fournies.
/// @do: template_apply_under_governance
/// tool.text.template.apply
pub fn apply(
    ctx: &GovernedContext,
    template: &str,
    data: &HashMap<String, String>,
) -> Result<String, MiyuTextError> {
    if !ctx.has_mandate() {
        return Err(MiyuTextError::NoMandate);
    }
    let mut out = String::with_capacity(template.len());
    let bytes = template.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 4 <= bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
            let end = find_closing(bytes, i + 2);
            if let Some(close) = end {
                let key = template[i + 2..close]
                    .trim()
                    .to_string();
                let val = data.get(&key).map_or("", String::as_str);
                out.push_str(val);
                i = close + 2;
            } else {
                out.push(bytes[i] as char);
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    Ok(out)
}

fn find_closing(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 2 <= bytes.len() {
        if bytes[i] == b'}' && bytes[i + 1] == b'}' {
            return Some(i);
        }
        i += 1;
    }
    None
}
