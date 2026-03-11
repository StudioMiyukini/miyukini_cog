use crate::state::{MipowerCtx, Notification, View};
use dioxus::prelude::*;

const VALID_AGENTS: &[&str] = &["Maria", "Denis", "Lise", "Victor", "Hugo", "Fabrice", "George", "Jean", "Arianne", "Francois"];

#[component]
pub fn Starter(
    prefill_title: Option<String>,
    prefill_desc:  Option<String>,
    mut view: Signal<View>,
    mut notification: Signal<Option<Notification>>,
) -> Element {
    let ctx = use_context::<MipowerCtx>();

    let mut title       = use_signal(|| prefill_title.clone().unwrap_or_default());
    let mut task_class  = use_signal(|| "T5".to_string());
    let mut complexity  = use_signal(|| "C5".to_string());
    let mut domain      = use_signal(|| "fullstack".to_string());
    let mut autonomy    = use_signal(|| String::new());
    let mut stack       = use_signal(|| String::new());
    let mut description = use_signal(|| prefill_desc.clone().unwrap_or_default());
    let mut constraints = use_signal(|| String::new());
    let mut urgency     = use_signal(|| false);
    let mut sensitive   = use_signal(|| false);
    let mut msw         = use_signal(|| false);
    let mut agents: Signal<Vec<String>> = use_signal(Vec::new);
    let mut tags:   Signal<Vec<String>> = use_signal(Vec::new);
    let mut tag_input   = use_signal(|| String::new());

    let mut init_status = use_signal(|| String::new());

    // Generation du prompt (reactive)
    let prompt = build_prompt(
        &title.read(),
        &task_class.read(),
        &domain.read(),
        &stack.read(),
        &constraints.read(),
        &autonomy.read(),
        &description.read(),
        &agents.read(),
        &tags.read(),
        *urgency.read(),
        *sensitive.read(),
        *msw.read(),
    );

    let prompt_display = prompt.clone();

    rsx! {
        div { class: "view-content",
            div { class: "view-header",
                h1 { "MIPOWER Starter" }
            }

            div { class: "starter-layout",

                // ── Formulaire ──────────────────────────────────────────────
                div { class: "starter-form",
                    label { class: "field-group",
                        span { class: "field-label", "Titre de la sequence *" }
                        input {
                            r#type: "text",
                            class: "text-input",
                            placeholder: "ex: mipower-workflow-editor",
                            maxlength: 200,
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }

                    div { class: "field-row",
                        label { class: "field-group",
                            span { class: "field-label", "Classe de tache" }
                            select {
                                class: "select-input",
                                value: "{task_class}",
                                onchange: move |e| task_class.set(e.value()),
                                option { value: "T1", "T1 — Micro-fix" }
                                option { value: "T2", "T2 — Fix cible" }
                                option { value: "T3", "T3 — Feature moderee" }
                                option { value: "T4", "T4 — Feature majeure" }
                                option { value: "T5", selected: true, "T5 — Chantier strategique" }
                            }
                        }
                        label { class: "field-group",
                            span { class: "field-label", "Complexite" }
                            select {
                                class: "select-input",
                                value: "{complexity}",
                                onchange: move |e| complexity.set(e.value()),
                                option { value: "C1", "C1 — Mineure" }
                                option { value: "C2", "C2 — Faible" }
                                option { value: "C3", "C3 — Moyenne" }
                                option { value: "C4", "C4 — Elevee" }
                                option { value: "C5", selected: true, "C5 — Strategique" }
                            }
                        }
                    }

                    div { class: "field-row",
                        label { class: "field-group",
                            span { class: "field-label", "Domaine" }
                            select {
                                class: "select-input",
                                value: "{domain}",
                                onchange: move |e| domain.set(e.value()),
                                option { value: "back",      "Back-end" }
                                option { value: "front",     "Front-end" }
                                option { value: "fullstack", selected: true, "Full-stack" }
                                option { value: "infra",     "Infra / DevOps" }
                                option { value: "ai-ml",     "AI / ML" }
                                option { value: "securite",  "Securite" }
                                option { value: "data",      "Data" }
                                option { value: "autre",     "Autre" }
                            }
                        }
                        label { class: "field-group",
                            span { class: "field-label", "Mode autonomie" }
                            select {
                                class: "select-input",
                                value: "{autonomy}",
                                onchange: move |e| autonomy.set(e.value()),
                                option { value: "", "Non specifie" }
                                option { value: "FULL",      "FULL — Autopilot" }
                                option { value: "BIG_STEPS", "BIG_STEPS — Par phase" }
                                option { value: "GUIDED",    "GUIDED — Supervise" }
                            }
                        }
                    }

                    label { class: "field-group",
                        span { class: "field-label", "Stack technique" }
                        div { class: "stack-input-group",
                            select {
                                class: "select-input",
                                onchange: move |e| {
                                    let v = e.value();
                                    if v != "__autre__" && !v.is_empty() { stack.set(v); }
                                },
                                option { value: "", "Choisir une stack..." }
                                option { value: "Rust + axum + SQLite",        "Rust + axum + SQLite" }
                                option { value: "Rust + Dioxus",               "Rust + Dioxus" }
                                option { value: "Rust + axum + Dioxus",        "Rust + axum + Dioxus (fullstack)" }
                                option { value: "TypeScript + React",          "TypeScript + React" }
                                option { value: "TypeScript + Next.js",        "TypeScript + Next.js" }
                                option { value: "Python + FastAPI",            "Python + FastAPI" }
                                option { value: "Go + Fiber",                  "Go + Fiber" }
                                option { value: "__autre__",                   "Autre (saisie libre)" }
                            }
                            input {
                                r#type: "text",
                                class: "text-input",
                                placeholder: "ou saisie libre...",
                                maxlength: 200,
                                value: "{stack}",
                                oninput: move |e| stack.set(e.value()),
                            }
                        }
                    }

                    label { class: "field-group",
                        span { class: "field-label", "Description *" }
                        textarea {
                            class: "textarea-input",
                            rows: 5,
                            maxlength: 2000,
                            placeholder: "Decris ce que tu veux construire...",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }
                    }

                    label { class: "field-group",
                        span { class: "field-label", "Contraintes techniques (optionnel)" }
                        input {
                            r#type: "text",
                            class: "text-input",
                            placeholder: "ex: Rust uniquement, pas de cloud...",
                            maxlength: 500,
                            value: "{constraints}",
                            oninput: move |e| constraints.set(e.value()),
                        }
                    }

                    // ── Options avancees ──────────────────────────────────
                    details { class: "advanced-section",
                        summary { "Options avancees" }
                        div { class: "advanced-body",

                            div { class: "field-label", "Agents MIP actifs" }
                            div { class: "agents-grid",
                                for agent in VALID_AGENTS {
                                    {
                                        let agent_str = agent.to_string();
                                        let agent_str2 = agent.to_string();
                                        let is_checked = agents.read().contains(&agent_str);
                                        rsx! {
                                            label { class: "agent-label",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: is_checked,
                                                    onchange: move |e| {
                                                        let mut a = agents.write();
                                                        if e.checked() { if !a.contains(&agent_str) { a.push(agent_str.clone()); } }
                                                        else { a.retain(|x| x != &agent_str); }
                                                    },
                                                }
                                                " {agent_str2}"
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "field-label", "Tags" }
                            div { class: "tags-input-row",
                                input {
                                    r#type: "text",
                                    class: "text-input",
                                    placeholder: "Ajouter un tag...",
                                    maxlength: 50,
                                    value: "{tag_input}",
                                    oninput: move |e| tag_input.set(e.value()),
                                }
                                button {
                                    class: "btn-secondary",
                                    onclick: move |_| {
                                        let t = tag_input.read().trim().to_string();
                                        if !t.is_empty() && tags.read().len() < 10 && !tags.read().contains(&t) {
                                            tags.write().push(t);
                                            tag_input.set(String::new());
                                        }
                                    },
                                    "+"
                                }
                            }
                            div { class: "tags-list",
                                for (i, tag) in tags.read().clone().into_iter().enumerate() {
                                    span {
                                        class: "tag-chip",
                                        title: "Clic pour supprimer",
                                        onclick: move |_| { tags.write().remove(i); },
                                        "{tag} x"
                                    }
                                }
                            }

                            div { class: "toggles-row",
                                label { class: "toggle-label",
                                    input {
                                        r#type: "checkbox",
                                        checked: *urgency.read(),
                                        onchange: move |e| urgency.set(e.checked()),
                                    }
                                    " Urgent"
                                }
                                label { class: "toggle-label",
                                    input {
                                        r#type: "checkbox",
                                        checked: *sensitive.read(),
                                        onchange: move |e| sensitive.set(e.checked()),
                                    }
                                    " Donnees sensibles"
                                }
                                label { class: "toggle-label",
                                    input {
                                        r#type: "checkbox",
                                        checked: *msw.read(),
                                        onchange: move |e| msw.set(e.checked()),
                                    }
                                    " Mode Sans Web (offline)"
                                }
                            }
                        }
                    }
                }

                // ── Preview & actions ────────────────────────────────────────
                div { class: "starter-preview",
                    div { class: "preview-header",
                        h3 { "Preview" }
                        div { class: "preview-actions",
                            button {
                                class: "btn-secondary",
                                onclick: move |_| {
                                    let text = prompt.clone();
                                    spawn(async move {
                                        if let Ok(mut cb) = arboard::Clipboard::new() {
                                            let _ = cb.set_text(text);
                                        }
                                    });
                                },
                                "Copier"
                            }
                            button {
                                class: "btn-primary",
                                onclick: {
                                    let title_val  = title.read().clone();
                                    let complexity_val = complexity.read().clone();
                                    let mip_root   = ctx.mip_root.read().clone();
                                    move |_| {
                                        let slug = title_val
                                            .to_lowercase()
                                            .chars()
                                            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                                            .collect::<String>();
                                        let slug = slug.trim_matches('-').to_string();
                                        if slug.is_empty() {
                                            init_status.set("Remplis le titre avant d initialiser.".into());
                                            return;
                                        }
                                        let complexity_num: u8 = complexity_val.chars().nth(1)
                                            .and_then(|c| c.to_digit(10))
                                            .unwrap_or(5) as u8;
                                        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                                        let folder = format!("{today}-{slug}");
                                        let seq_path = std::path::PathBuf::from(&mip_root)
                                            .join(".mip").join("sequences").join(&folder);
                                        let template_root = std::path::PathBuf::from(&mip_root)
                                            .join(".mip").join("sequences").join("_template");

                                        match std::fs::create_dir_all(&seq_path) {
                                            Err(e) => { init_status.set(format!("Erreur mkdir : {e}")); return; }
                                            Ok(()) => {}
                                        }

                                        let log1 = crate::init::init_base(&seq_path, &template_root);
                                        let log2 = crate::init::init_by_complexity(&seq_path, &template_root, complexity_num);
                                        let _ = crate::init::rebuild_index(std::path::Path::new(&mip_root));

                                        match (log1, log2) {
                                            (Ok(l1), Ok(l2)) => {
                                                init_status.set(format!(
                                                    "Sequence cree : {} | base: {} | C{}: {}",
                                                    folder, l1.summary(), complexity_num, l2.summary()
                                                ));
                                                notification.set(Some(crate::state::Notification {
                                                    message: format!("Sequence {folder} initialisee."),
                                                    is_error: false,
                                                }));
                                            }
                                            (Err(e), _) | (_, Err(e)) => {
                                                init_status.set(format!("Erreur : {e}"));
                                            }
                                        }
                                    }
                                },
                                "Init sequence"
                            }
                        }
                    }

                    if !init_status.read().is_empty() {
                        div { class: "init-status", "{init_status}" }
                    }

                    textarea {
                        class: "prompt-textarea",
                        readonly: true,
                        value: "{prompt_display}",
                    }
                }
            }
        }
    }
}

fn build_prompt(
    title: &str, task_class: &str, domain: &str, stack: &str,
    constraints: &str, autonomy: &str, description: &str,
    agents: &[String], tags: &[String],
    urgency: bool, sensitive: bool, msw: bool,
) -> String {
    let stack       = if stack.is_empty()       { "A definir en P0" } else { stack };
    let constraints = if constraints.is_empty() { "Aucune contrainte specifique" } else { constraints };

    let mut lines = vec![
        format!("Lance une sequence MIP pour : {title}"),
        String::new(),
        format!("Classe estimee : {task_class}"),
        format!("Domaine : {domain}"),
        format!("Stack : {stack}"),
        format!("Contraintes : {constraints}"),
    ];
    if !autonomy.is_empty() { lines.push(format!("Mode autonomie : {autonomy}")); }
    if urgency             { lines.push("Urgence : Oui".into()); }
    if sensitive           { lines.push("Donnees sensibles : Oui".into()); }
    if msw                 { lines.push("Mode Sans Web : Oui".into()); }
    if !agents.is_empty()  { lines.push(format!("Agents actifs : {}", agents.join(", "))); }
    if !tags.is_empty()    { lines.push(format!("Tags : {}", tags.join(", "))); }
    lines.push(String::new());
    lines.push(format!("Description :\n{description}"));
    lines.push(String::new());
    lines.push("---".into());
    lines.push("Maria, classe cette demande et lance P0.".into());
    lines.join("\n")
}
