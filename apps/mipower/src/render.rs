//! Rendu Markdown → HTML avec pulldown-cmark + ammonia.
//! Zéro JavaScript : tout se passe côté Rust avant injection dans le WebView.

use pulldown_cmark::{html, Options, Parser};

/// Convertit du Markdown en HTML sécurisé (pas de scripts, XSS neutralisé).
pub fn md_to_html(markdown: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, opts);
    let mut raw_html = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut raw_html, parser);

    // Sanitisation : supprime scripts, event handlers, etc.
    let clean = ammonia::Builder::default()
        .add_tags(&["table", "thead", "tbody", "tr", "th", "td", "input"])
        .add_tag_attributes("th", &["align"])
        .add_tag_attributes("td", &["align"])
        // Autoriser les cases à cocher des task lists (type=checkbox, disabled)
        .add_tag_attributes("input", &["type", "checked", "disabled"])
        .add_generic_attributes(&["class", "id"])
        .clean(&raw_html)
        .to_string();

    clean
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown() {
        let html = md_to_html("# Titre\n\nParagraphe.");
        assert!(html.contains("<h1"));
        assert!(html.contains("Titre"));
        assert!(html.contains("<p>"));
    }

    #[test]
    fn test_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = md_to_html(md);
        assert!(html.contains("<table>"));
    }

    #[test]
    fn test_script_stripped() {
        let md = "<script>alert('xss')</script>";
        let html = md_to_html(md);
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn test_tasklist() {
        let md = "- [x] Fait\n- [ ] À faire";
        let html = md_to_html(md);
        assert!(html.contains("checkbox"));
    }
}
