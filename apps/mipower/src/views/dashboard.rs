use crate::models::SequenceMeta;
use crate::state::{View, load_sequences, MipowerCtx};
use dioxus::prelude::*;

const CLASS_ORDER: &[&str] = &["T1", "T2", "T3", "T4", "T5"];
const STATUS_ORDER: &[&str] = &["active", "done", "archived"];

#[component]
pub fn Dashboard(
    mut view: Signal<View>,
    mut sequences: Signal<Vec<SequenceMeta>>,
) -> Element {
    let ctx     = use_context::<MipowerCtx>();
    let mut search  = use_signal(|| String::new());
    let mut status_filter = use_signal(|| String::new());
    let mut sort_by = use_signal(|| "date-desc".to_string());

    // Recharger les sequences a l'ouverture
    use_effect(move || {
        let root = ctx.mip_root.read().clone();
        let loaded = load_sequences(&root);
        sequences.set(loaded);
    });

    let seqs = sequences.read().clone();
    let search_val  = search.read().to_lowercase();
    let status_val  = status_filter.read().clone();
    let sort_val    = sort_by.read().clone();

    let mut filtered: Vec<&SequenceMeta> = seqs.iter().filter(|s| {
        (search_val.is_empty() || s.slug.to_lowercase().contains(&search_val) || s.name.to_lowercase().contains(&search_val))
        && (status_val.is_empty() || s.status == status_val)
    }).collect();

    filtered.sort_by(|a, b| match sort_val.as_str() {
        "date-asc"  => a.date.cmp(&b.date),
        "name-asc"  => a.name.cmp(&b.name),
        "class-asc" => {
            let ia = CLASS_ORDER.iter().position(|&x| Some(x) == a.task_class.as_deref()).unwrap_or(99);
            let ib = CLASS_ORDER.iter().position(|&x| Some(x) == b.task_class.as_deref()).unwrap_or(99);
            ia.cmp(&ib)
        },
        "status" => {
            let ia = STATUS_ORDER.iter().position(|&x| x == a.status).unwrap_or(99);
            let ib = STATUS_ORDER.iter().position(|&x| x == b.status).unwrap_or(99);
            ia.cmp(&ib)
        },
        _ => b.date.cmp(&a.date), // date-desc
    });

    rsx! {
        div { class: "view-content",
            div { class: "view-header",
                h1 { "Sequences MIP" }
                div { class: "header-actions",
                    input {
                        r#type: "text",
                        class: "search-input",
                        placeholder: "Rechercher une sequence...",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                    }
                    select {
                        class: "filter-select",
                        value: "{status_filter}",
                        onchange: move |e| status_filter.set(e.value()),
                        option { value: "", "Tous les statuts" }
                        option { value: "active", "Actif" }
                        option { value: "done", "Termine" }
                        option { value: "archived", "Archive" }
                    }
                    select {
                        class: "filter-select",
                        value: "{sort_by}",
                        onchange: move |e| sort_by.set(e.value()),
                        option { value: "date-desc", "Date (recent)" }
                        option { value: "date-asc",  "Date (ancien)" }
                        option { value: "name-asc",  "Nom A->Z" }
                        option { value: "class-asc", "Classe T1->T5" }
                        option { value: "status",    "Statut" }
                    }
                    button {
                        class: "btn-icon",
                        title: "Recharger",
                        onclick: move |_| {
                            let root = ctx.mip_root.read().clone();
                            sequences.set(load_sequences(&root));
                        },
                        "↺"
                    }
                }
            }

            if filtered.is_empty() {
                div { class: "empty-state",
                    if seqs.is_empty() {
                        p { "Aucune sequence trouvee. Configurez le chemin MIP dans les Parametres." }
                    } else {
                        p { "Aucun resultat pour ce filtre." }
                    }
                }
            } else {
                div { class: "sequence-grid",
                    for seq in filtered {
                        {sequence_card(seq, view)}
                    }
                }
            }
        }
    }
}

fn sequence_card(seq: &SequenceMeta, mut view: Signal<View>) -> Element {
    let slug = seq.slug.clone();
    let name = seq.name.clone();
    let date = seq.date.clone();
    let status = seq.status.clone();
    let task_class = seq.task_class.clone().unwrap_or_default();
    let complexity = seq.complexity.clone().unwrap_or_default();
    let current_phase = seq.current_phase.clone().unwrap_or_default();
    let status_cls = format!("badge badge-{status}");
    let class_cls  = format!("badge badge-{}", task_class.to_lowercase());

    rsx! {
        div {
            class: "sequence-card",
            onclick: move |_| {
                view.set(View::Report { slug: slug.clone(), file_path: None, tab: None });
            },
            div { class: "card-header",
                span { class: "card-slug", "{slug}" }
                span { class: "card-date", "{date}" }
            }
            div { class: "card-name", "{name}" }
            div { class: "card-badges",
                if !task_class.is_empty() {
                    span { class: "{class_cls}", "{task_class}" }
                }
                if !complexity.is_empty() {
                    span { class: "badge", "{complexity}" }
                }
                if !current_phase.is_empty() {
                    span { class: "badge", "{current_phase}" }
                }
                span { class: "{status_cls}", "{status}" }
            }
        }
    }
}
