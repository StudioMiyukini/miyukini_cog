//! Tool MiyuText — tool.text.markdown.render.
//! Rendu markdown en HTML ; options (extensions, safe) fournies ; ne décide pas du contenu.

use crate::context::GovernedContext;
use crate::errors::MiyuTextError;
use pulldown_cmark::{html, Options, Parser};

/// Options de rendu markdown (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct MarkdownOptions {
    /// Extensions activées (ex. tables, footnotes).
    pub extensions: Vec<String>,
    /// Mode safe (échappement HTML strict).
    pub safe: bool,
}

/// @id: miyutext_tool_markdown_render
/// @role: mutator
/// @layer: tool
/// @human: Rend du markdown fourni en HTML ; options (extensions, safe) fournies.
/// @do: markdown_render_under_governance
/// tool.text.markdown.render — ne décide pas du contenu.
pub fn render(
    ctx: &GovernedContext,
    markdown: &str,
    options: &MarkdownOptions,
) -> Result<String, MiyuTextError> {
    if !ctx.has_mandate() {
        return Err(MiyuTextError::NoMandate);
    }
    let mut opts = Options::empty();
    if options.extensions.iter().any(|e| e == "tables") {
        opts.insert(Options::ENABLE_TABLES);
    }
    if options.extensions.iter().any(|e| e == "strikethrough") {
        opts.insert(Options::ENABLE_STRIKETHROUGH);
    }
    let src = if options.safe {
        markdown.replace('<', "&lt;").replace('>', "&gt;")
    } else {
        markdown.to_string()
    };
    let parser = Parser::new_ext(&src, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    Ok(out)
}
