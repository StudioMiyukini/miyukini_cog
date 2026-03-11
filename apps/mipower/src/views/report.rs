use crate::models::SequenceMeta;
use crate::render::md_to_html;
use crate::state::{load_sequences, MipowerCtx, Notification, View};
use dioxus::prelude::*;
use std::path::PathBuf;

// ── Phases : onglets du menu top ─────────────────────────────────────────────
const PHASE_ORDER: &[&str] = &["brief", "p0", "p1", "p2", "p3", "p4", "p5", "p6", "agents", "docs"];
const PHASE_LABELS: &[(&str, &str)] = &[
    ("brief",  "Brief"),
    ("p0",     "P0"),
    ("p1",     "P1"),
    ("p2",     "P2"),
    ("p3",     "P3"),
    ("p4",     "P4"),
    ("p5",     "P5"),
    ("p6",     "P6"),
    ("agents", "Agents"),
    ("docs",   "Docs"),
];

fn phase_label(key: &str) -> &str {
    PHASE_LABELS.iter().find(|(k, _)| *k == key).map(|(_, v)| *v).unwrap_or(key)
}

// ── Sections de la colonne gauche ─────────────────────────────────────────────
// Dérivées du nom du fichier / sous-dossier
const SECTION_ORDER: &[&str] = &["plan", "etapes", "buf", "temps", "volets", "specs", "autre"];
const SECTION_LABELS: &[(&str, &str)] = &[
    ("plan",   "Plan"),
    ("etapes", "Etapes"),
    ("buf",    "BUF"),
    ("temps",  "Temps"),
    ("volets", "Volets"),
    ("specs",  "Specs"),
    ("autre",  "Autres"),
];

fn section_label(key: &str) -> &str {
    SECTION_LABELS.iter().find(|(k, _)| *k == key).map(|(_, v)| *v).unwrap_or(key)
}

/// Déduit la section sémantique d'un fichier à partir de son nom.
fn detect_section(filename: &str) -> &'static str {
    let n = filename.to_lowercase();
    if n.contains("buf") || n.contains("buffer")               { return "buf"; }
    if n.starts_with("etape") || n.contains("etape")           { return "etapes"; }
    if n.starts_with("t0")   || n.starts_with("temps")
       || n.contains("-t0")  || n.contains("temps-")           { return "temps"; }
    if n.contains("volet")                                      { return "volets"; }
    if n.contains("spec")    || n.contains("req")               { return "specs"; }
    if n == "index.md"  || n.contains("plan")
       || n.contains("roadmap") || n.contains("brief")
       || n.contains("travail") || n.contains("trace")          { return "plan"; }
    "autre"
}

#[derive(Clone, PartialEq)]
struct Artefact {
    path:    String,
    label:   String,
    phase:   String,   // onglet top (p0, p1, brief…)
    section: String,   // groupe colonne gauche
    done:    bool,
}

#[component]
pub fn Report(
    slug: String,
    initial_file: Option<String>,
    initial_tab:  Option<String>,
    mut view:         Signal<View>,
    mut sequences:    Signal<Vec<SequenceMeta>>,
    mut notification: Signal<Option<Notification>>,
) -> Element {
    let ctx = use_context::<MipowerCtx>();

    let mut artefacts:    Signal<Vec<Artefact>>    = use_signal(Vec::new);
    let mut active_file:  Signal<Option<String>>   = use_signal(|| initial_file.clone());
    let mut html_content: Signal<String>           = use_signal(String::new);
    let mut raw_content:  Signal<String>           = use_signal(String::new);
    let mut edit_mode:    Signal<bool>             = use_signal(|| false);
    let mut edit_buffer:  Signal<String>           = use_signal(String::new);
    let mut active_phase: Signal<String>           = use_signal(|| initial_tab.clone().unwrap_or_else(|| "brief".to_string()));
    let mut save_status:  Signal<String>           = use_signal(String::new);

    let mip_root   = ctx.mip_root.read().clone();
    let slug_clone = slug.clone();

    // Charger artefacts
    use_effect(move || {
        let root = ctx.mip_root.read().clone();
        let arts = collect_artefacts(&root, &slug_clone);
        if active_file.read().is_none() {
            // auto-select premier brief
            if let Some(f) = arts.iter().find(|a| a.phase == "brief") {
                active_file.set(Some(f.path.clone()));
            }
        }
        artefacts.set(arts);
    });

    // Charger contenu fichier actif
    let _watch = active_file.read().clone();
    use_effect(move || {
        let Some(ref path) = *active_file.read() else { return; };
        let full = full_path(&ctx.mip_root.read(), path);
        match std::fs::read_to_string(&full) {
            Ok(md) => {
                html_content.set(md_to_html(&md));
                raw_content.set(md.clone());
                edit_buffer.set(md);
            }
            Err(e) => html_content.set(format!("<p style='color:var(--error)'>Erreur : {e}</p>")),
        }
    });

    let arts  = artefacts.read().clone();
    let phase = active_phase.read().clone();

    // Phases présentes (pour le menu top)
    let phases_present: Vec<String> = PHASE_ORDER.iter()
        .filter(|p| arts.iter().any(|a| &a.phase == *p))
        .map(|p| p.to_string())
        .chain(arts.iter().map(|a| a.phase.clone())
            .filter(|p| !PHASE_ORDER.contains(&p.as_str()))
            .collect::<std::collections::HashSet<_>>().into_iter())
        .collect();

    // Fichiers de la phase courante, groupés par section
    let phase_arts: Vec<&Artefact> = arts.iter().filter(|a| a.phase == phase).collect();
    let mut sections: std::collections::HashMap<String, Vec<&Artefact>> = std::collections::HashMap::new();
    for a in &phase_arts {
        sections.entry(a.section.clone()).or_default().push(a);
    }
    let section_keys: Vec<String> = SECTION_ORDER.iter()
        .filter(|s| sections.contains_key(**s))
        .map(|s| s.to_string())
        .chain(sections.keys().filter(|s| !SECTION_ORDER.contains(&s.as_str())).cloned())
        .collect();

    // Stats par phase pour le badge count
    let phase_stats = |p: &str| -> (usize, usize) {
        let v: Vec<_> = arts.iter().filter(|a| a.phase == p).collect();
        let done = v.iter().filter(|a| a.done).count();
        (done, v.len())
    };

    let seq_meta   = sequences.read().iter().find(|s| s.slug == slug).cloned();
    let seq_status = seq_meta.as_ref().map(|s| s.status.as_str()).unwrap_or("active").to_string();

    rsx! {
        div { class: "report-view",

            // ── Header ─────────────────────────────────────────────────────
            div { class: "report-header",
                div { class: "report-header-left",
                    button {
                        class: "btn-back",
                        onclick: move |_| view.set(View::Dashboard),
                        "← Retour"
                    }
                    span { class: "report-title", "{slug}" }
                    {status_badge(&seq_status)}
                }
                div { class: "report-header-right",
                    if seq_status == "active" {
                        button {
                            class: "btn-danger",
                            onclick: {
                                let slug2 = slug.clone();
                                let root2 = mip_root.clone();
                                let mut seqs = sequences;
                                move |_| {
                                    if let Err(e) = cloture_sequence(&root2, &slug2) {
                                        notification.set(Some(Notification { message: format!("Erreur : {e}"), is_error: true }));
                                    } else {
                                        seqs.set(load_sequences(&root2));
                                        notification.set(Some(Notification { message: format!("{slug2} cloturee."), is_error: false }));
                                    }
                                }
                            },
                            "Cloturer"
                        }
                    }
                    if active_file.read().is_some() {
                        button {
                            class: "btn-secondary",
                            onclick: {
                                let raw   = raw_content.read().clone();
                                let slug3 = slug.clone();
                                move |_| view.set(View::Starter {
                                    prefill_title: Some(format!("reprise-{slug3}")),
                                    prefill_desc:  Some(raw.clone()),
                                })
                            },
                            "Reprendre comme brief"
                        }
                    }
                }
            }

            // ── Menu top : phases ───────────────────────────────────────────
            div { class: "phase-tabs",
                for p in &phases_present {
                    {
                        let key   = p.clone();
                        let key2  = p.clone();
                        let is_act = *active_phase.read() == key;
                        let (done, total) = phase_stats(&key2);
                        let all_done = done == total && total > 0;
                        let cls = if is_act       { "phase-tab phase-tab-active" }
                                  else if all_done { "phase-tab phase-tab-done" }
                                  else             { "phase-tab" };
                        rsx! {
                            button {
                                class: "{cls}",
                                onclick: move |_| active_phase.set(key.clone()),
                                span { class: "phase-tab-label", "{phase_label(&key2)}" }
                                if total > 0 {
                                    span { class: "phase-tab-count", "{done}/{total}" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Corps : colonne gauche + contenu ────────────────────────────
            div { class: "report-body",

                // ── Colonne gauche : sections ──────────────────────────────
                aside { class: "report-sidebar",
                    if phase_arts.is_empty() {
                        div { class: "sidebar-empty", "Aucun fichier." }
                    }
                    for sec_key in &section_keys {
                        {
                            let sec_files = sections.get(sec_key).cloned().unwrap_or_default();
                            let sec_label = section_label(sec_key);
                            rsx! {
                                div { class: "sidebar-section",
                                    div { class: "sidebar-section-title", "{sec_label}" }
                                    for art in sec_files {
                                        {
                                            let path    = art.path.clone();
                                            let label   = art.label.clone();
                                            let section = sec_key.clone();
                                            let is_act  = active_file.read().as_deref() == Some(&path);
                                            let is_done = art.done;
                                            rsx! {
                                                {sidebar_item_btn(path, label, section, is_act, is_done, active_file, edit_mode, save_status)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Contenu principal ──────────────────────────────────────
                div { class: "report-content",
                    if active_file.read().is_none() {
                        div { class: "report-empty", "Selectionnez un fichier dans la colonne de gauche." }
                    } else if *edit_mode.read() {
                        div { class: "editor-zone",
                            div { class: "editor-toolbar",
                                button {
                                    class: "btn-primary",
                                    onclick: {
                                        let root4 = mip_root.clone();
                                        move |_| {
                                            let Some(path) = active_file.read().clone() else { return; };
                                            let full    = full_path(&root4, &path);
                                            let content = edit_buffer.read().clone();
                                            match std::fs::write(&full, content.as_bytes()) {
                                                Ok(()) => {
                                                    html_content.set(md_to_html(&edit_buffer.read()));
                                                    raw_content.set(edit_buffer.read().clone());
                                                    edit_mode.set(false);
                                                    save_status.set("Sauvegarde.".into());
                                                }
                                                Err(e) => save_status.set(format!("Erreur : {e}")),
                                            }
                                        }
                                    },
                                    "Sauvegarder"
                                }
                                button {
                                    class: "btn-secondary",
                                    onclick: move |_| {
                                        edit_buffer.set(raw_content.read().clone());
                                        edit_mode.set(false);
                                        save_status.set(String::new());
                                    },
                                    "Annuler"
                                }
                                if !save_status.read().is_empty() {
                                    span { class: "editor-status", "{save_status}" }
                                }
                            }
                            textarea {
                                class: "md-editor",
                                value: "{edit_buffer}",
                                oninput: move |e| edit_buffer.set(e.value()),
                            }
                        }
                    } else {
                        div { class: "content-toolbar",
                            if let Some(path) = active_file.read().clone() {
                                span { class: "content-path", "{path}" }
                            }
                            div { class: "toolbar-actions",
                                button {
                                    class: "btn-secondary",
                                    onclick: move |_| {
                                        edit_buffer.set(raw_content.read().clone());
                                        edit_mode.set(true);
                                        save_status.set(String::new());
                                    },
                                    "Editer"
                                }
                                button {
                                    class: "btn-icon",
                                    title: "Ouvrir dans l editeur systeme",
                                    onclick: {
                                        let root5 = mip_root.clone();
                                        move |_| {
                                            let Some(path) = active_file.read().clone() else { return; };
                                            let full = full_path(&root5, &path);
                                            let _ = std::process::Command::new("cmd")
                                                .args(["/C", "start", "", &full.to_string_lossy()])
                                                .spawn();
                                        }
                                    },
                                    "Ouvrir ↗"
                                }
                            }
                        }
                        div {
                            class: "prose",
                            dangerous_inner_html: "{html_content}",
                        }
                    }
                }
            }
        }
    }
}

/// Rendu d'un item de la colonne gauche.
/// Les "temps" (T01, T02…) affichent un bouton stylisé sans l'extension .md.
fn sidebar_item_btn(
    path: String, label: String, section: String,
    is_act: bool, is_done: bool,
    mut active_file: Signal<Option<String>>,
    mut edit_mode:   Signal<bool>,
    mut save_status: Signal<String>,
) -> Element {
    let path2 = path.clone();
    let path3 = path.clone();

    if section == "temps" {
        // Extraire "T01" et le reste du nom sans extension
        let stem = label.trim_end_matches(".md");
        // Cherche un pattern Txx ou txx au début
        let (num, name) = if let Some(rest) = stem.strip_prefix(|c: char| c == 't' || c == 'T') {
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            let tail   = &rest[digits.len()..];
            let tail   = tail.trim_start_matches(['-', '_', ' ']).to_string();
            (format!("T{digits}"), if tail.is_empty() { stem.to_string() } else { tail })
        } else {
            // cherche "-t0N" ou "-temps" dans le nom complet
            let n = stem.to_lowercase();
            if let Some(idx) = n.find("-t0").or_else(|| n.find("-t1")).or_else(|| n.find("-t2")) {
                let num_part = &stem[idx+1..].chars().take(3).collect::<String>();
                let name_part = stem[idx + 1 + num_part.len()..].trim_start_matches(['-', '_']).to_string();
                (num_part.to_uppercase(), if name_part.is_empty() { stem.to_string() } else { name_part })
            } else {
                ("—".into(), stem.to_string())
            }
        };
        let mut cls = "sidebar-item temps-btn".to_string();
        if is_act  { cls.push_str(" sidebar-item-active"); }
        if is_done { cls.push_str(" sidebar-item-done"); }
        rsx! {
            button {
                class: "{cls}",
                title: "{path2}",
                onclick: move |_| {
                    active_file.set(Some(path3.clone()));
                    edit_mode.set(false);
                    save_status.set(String::new());
                },
                span { class: "temps-num", "{num}" }
                span { class: "temps-name", "{name}" }
                if is_done { span { class: "sidebar-item-check", "✓" } }
            }
        }
    } else {
        // Afficher sans l'extension .md
        let display = label.trim_end_matches(".md").to_string();
        let mut cls = "sidebar-item".to_string();
        if is_act  { cls.push_str(" sidebar-item-active"); }
        if is_done { cls.push_str(" sidebar-item-done"); }
        rsx! {
            button {
                class: "{cls}",
                title: "{path2}",
                onclick: move |_| {
                    active_file.set(Some(path3.clone()));
                    edit_mode.set(false);
                    save_status.set(String::new());
                },
                span { class: "sidebar-item-label", "{display}" }
                if is_done { span { class: "sidebar-item-check", "✓" } }
            }
        }
    }
}

fn status_badge(status: &str) -> Element {
    match status {
        "active"   => rsx! { span { class: "badge badge-active",   "Actif"   } },
        "done"     => rsx! { span { class: "badge badge-done",     "Termine" } },
        "archived" => rsx! { span { class: "badge badge-archived", "Archive" } },
        _          => rsx! { span { class: "badge",                "{status}"} },
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn full_path(mip_root: &str, rel_path: &str) -> PathBuf {
    PathBuf::from(mip_root).join(rel_path.replace('/', std::path::MAIN_SEPARATOR_STR))
}

/// Détermine la phase MIP d'un fichier à partir de son chemin relatif à la séquence.
fn detect_phase(rel_to_seq: &str) -> String {
    let parts: Vec<&str> = rel_to_seq.splitn(3, '/').collect();
    if parts.len() == 1 { return "_root".to_string(); }
    let folder = parts[0];
    let fname  = parts.last().unwrap_or(&"").to_lowercase();

    match folder {
        "briefs"          => "brief".into(),
        "agents"          => "agents".into(),
        "specs"           => "docs".into(),
        "rapports_finaux" => "docs".into(),
        "ressources"      => "docs".into(),
        "gpi"             => "docs".into(),
        "plans_p3"        => "p3".into(),
        "audits"          => "p4".into(),
        "phases" => {
            // grouper par préfixe de fichier : p0-*.md → "p0"
            for px in &["p0","p1","p2","p3","p4","p5","p6"] {
                if fname.starts_with(px) { return px.to_string(); }
            }
            "docs".into()
        }
        _ => folder.to_string(),
    }
}

fn collect_artefacts(mip_root: &str, slug: &str) -> Vec<Artefact> {
    let base = PathBuf::from(mip_root).join(".mip").join("sequences");
    let Ok(entries) = std::fs::read_dir(&base) else { return vec![]; };
    let seq_dir = entries.filter_map(|e| e.ok())
        .find(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.ends_with(&format!("-{slug}")) || n == slug
        })
        .map(|e| e.path());
    let Some(seq_dir) = seq_dir else { return vec![]; };
    let root = PathBuf::from(mip_root);
    let mut out = Vec::new();
    walk_md(&seq_dir, &root, &seq_dir, &mut out);
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

fn walk_md(cur: &PathBuf, root: &PathBuf, seq_dir: &PathBuf, out: &mut Vec<Artefact>) {
    let Ok(entries) = std::fs::read_dir(cur) else { return; };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if name != "ui" && name != "node_modules" && !name.starts_with('.') {
                walk_md(&path, root, seq_dir, out);
            }
        } else if path.extension().is_some_and(|e| e == "md") {
            let Ok(rel) = path.strip_prefix(root) else { continue; };
            let rel_path = rel.to_string_lossy().replace('\\', "/");
            let rel_seq  = path.strip_prefix(seq_dir)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();

            let phase   = detect_phase(&rel_seq);
            let label   = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let section = detect_section(&label);
            let done    = std::fs::read_to_string(&path)
                .map(|c| c.contains("Etat : TERMINE") || c.contains("Statut : Termine"))
                .unwrap_or(false);

            out.push(Artefact { path: rel_path, label, phase, section: section.to_string(), done });
        }
    }
}

fn cloture_sequence(mip_root: &str, slug: &str) -> anyhow::Result<()> {
    let base = PathBuf::from(mip_root).join(".mip").join("sequences");
    let seq_dir = std::fs::read_dir(&base)?
        .filter_map(|e| e.ok())
        .find(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.ends_with(&format!("-{slug}")) || n == slug
        })
        .map(|e| e.path())
        .ok_or_else(|| anyhow::anyhow!("Introuvable : {slug}"))?;

    let trace = seq_dir.join("phases").join("p6-trace.md");
    std::fs::create_dir_all(trace.parent().unwrap())?;
    let ts = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let content = if trace.exists() {
        format!("{}\n\n---\nEtat : TERMINE\nCLOTUREE -- SUCCES\nDate : {ts}\n",
            std::fs::read_to_string(&trace)?)
    } else {
        format!("# P6 — Cloture\n\nEtat : TERMINE\nCLOTUREE -- SUCCES\nDate : {ts}\n")
    };
    std::fs::write(&trace, content.as_bytes())?;
    crate::init::rebuild_index(std::path::Path::new(mip_root)).ok();
    Ok(())
}
