//! Tool MiyuExport — tool.export.pdf.render.
//! Rend un PDF à partir d'un template et de données fournis ; ne décide pas du contenu.

use crate::context::GovernedContext;
use crate::errors::MiyuExportError;
use genpdf::{elements, fonts, style, Document, Element, SimplePageDecorator};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;

/// Répertoire par défaut pour les polices (LiberationSans : *-Regular.ttf, etc.).
const DEFAULT_FONT_DIR: &str = "./fonts";

/// @id: miyuexport_tool_pdf_render
/// @role: mutator
/// @layer: tool
/// @human: Rend un PDF à partir d'un template et de données fournis.
/// @do: pdf_render_under_governance
/// tool.export.pdf.render — ne décide pas du contenu.
pub fn render(
    ctx: &GovernedContext,
    template: &str,
    data: &HashMap<String, String>,
) -> Result<Vec<u8>, MiyuExportError> {
    if !ctx.has_mandate() {
        return Err(MiyuExportError::NoMandate);
    }
    let font_family = fonts::from_files(Path::new(DEFAULT_FONT_DIR), "LiberationSans", None)
        .map_err(|e| MiyuExportError::Io(e.to_string()))?;
    let mut doc = Document::new(font_family);
    doc.set_title("Export");
    doc.set_minimal_conformance();
    doc.set_page_decorator(SimplePageDecorator::new());
    let content = substitute_template(template, data);
    let paragraph = elements::Paragraph::new(content).styled(style::Style::new().with_font_size(12));
    doc.push(paragraph);
    let mut buf = Cursor::new(Vec::new());
    doc.render(&mut buf).map_err(|e| MiyuExportError::Io(e.to_string()))?;
    Ok(buf.into_inner())
}

fn substitute_template(template: &str, data: &HashMap<String, String>) -> String {
    let mut out = template.to_string();
    for (key, value) in data {
        let placeholder = format!("{{{{{}}}}}", key.trim());
        out = out.replace(&placeholder, value);
    }
    out
}
