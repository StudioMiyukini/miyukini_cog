//! Tool MiyuExport — tool.export.csv.generate.
//! Génère un fichier CSV à partir de données (tableau) et options fournis.

use crate::context::GovernedContext;
use crate::errors::MiyuExportError;

/// Options CSV (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct CsvOptions {
    pub delimiter: char,
    pub encoding: String,
    pub locale: Option<String>,
}

/// @id: miyuexport_tool_csv_generate
/// @role: mutator
/// @layer: tool
/// @human: Génère un fichier CSV à partir de données et options fournis.
/// @do: csv_generate_under_governance
/// tool.export.csv.generate
pub fn generate(
    ctx: &GovernedContext,
    rows: &[Vec<String>],
    options: &CsvOptions,
) -> Result<Vec<u8>, MiyuExportError> {
    if !ctx.has_mandate() {
        return Err(MiyuExportError::NoMandate);
    }
    let delim = if options.delimiter == '\0' {
        ','
    } else {
        options.delimiter
    };
    let mut out = String::new();
    for row in rows {
        let escaped: Vec<String> = row
            .iter()
            .map(|cell| escape_csv_field(cell, delim))
            .collect();
        out.push_str(&escaped.join(&delim.to_string()));
        out.push('\n');
    }
    Ok(out.into_bytes())
}

fn escape_csv_field(s: &str, delimiter: char) -> String {
    let needs_quote =
        s.contains(delimiter) || s.contains('"') || s.contains('\n') || s.contains('\r');
    if needs_quote {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
