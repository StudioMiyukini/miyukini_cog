//! Tool MiyuExport — tool.export.xlsx.generate.
//! Génère un fichier Excel (XLSX) à partir de données et options fournis.

use crate::context::GovernedContext;
use crate::errors::MiyuExportError;
use rust_xlsxwriter::Workbook;

/// Options XLSX (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct XlsxOptions {
    pub sheet_names: Vec<String>,
    pub with_headers: bool,
    pub number_format: Option<String>,
}

/// @id: miyuexport_tool_xlsx_generate
/// @role: mutator
/// @layer: tool
/// @human: Génère un fichier XLSX à partir de données et options fournis.
/// @do: xlsx_generate_under_governance
/// tool.export.xlsx.generate
pub fn generate(
    ctx: &GovernedContext,
    sheets: &[Vec<Vec<String>>],
    options: &XlsxOptions,
) -> Result<Vec<u8>, MiyuExportError> {
    if !ctx.has_mandate() {
        return Err(MiyuExportError::NoMandate);
    }
    let mut workbook = Workbook::new();
    for (idx, sheet_data) in sheets.iter().enumerate() {
        let name = options
            .sheet_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("Sheet{}", idx + 1));
        let name = sanitize_sheet_name(&name);
        let worksheet = workbook.add_worksheet();
        worksheet
            .set_name(&name)
            .map_err(|e| MiyuExportError::Io(e.to_string()))?;
        for (row_idx, row) in sheet_data.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                worksheet
                    .write_string(row_idx as u32, col_idx as u16, cell)
                    .map_err(|e| MiyuExportError::Io(e.to_string()))?;
            }
        }
    }
    workbook
        .save_to_buffer()
        .map_err(|e| MiyuExportError::Io(e.to_string()))
}

fn sanitize_sheet_name(name: &str) -> String {
    let s: String = name
        .chars()
        .take(31)
        .filter(|c| *c != '[' && *c != ']' && *c != '*' && *c != '?' && *c != ':' && *c != '/')
        .collect();
    if s.is_empty() {
        "Sheet".to_string()
    } else {
        s
    }
}
