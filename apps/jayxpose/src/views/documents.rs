//! Coffre-fort documentaire (XP-E09 + XP-E10) — liste, upload, partages.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use super::components::{DocumentRow, EmptyState, FormSection, Badge};
use super::JayXposeState;

/// Sous-vue active dans Documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum DocView {
    #[default]
    Liste,
    Upload,
    Partages,
}

#[component]
pub fn Documents(state: Signal<JayXposeState>) -> Element {
    let c = use_palette();
    let db = crate::use_db();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();
    let mut view = use_signal(|| DocView::Liste);

    let documents = db.documents_by_exposant(&exposant_id).unwrap_or_default();
    let partages = db.partages_by_exposant(&exposant_id).unwrap_or_default();
    let doc_len = documents.len();
    let partage_len = partages.len();

    // Alertes expiration
    let expiring = documents.iter().filter(|d| {
        d.status.as_deref() == Some("valide") && d.expires_at.is_some()
    }).count();

    // Pre-calcul styles conditionnels
    let is_partages_view = *view.read() == DocView::Partages;
    let partage_btn_bg = if is_partages_view { c.accent_blue.to_string() } else { c.bg_hover.to_string() };
    let partage_btn_color = if is_partages_view { "white".to_string() } else { c.text_secondary.to_string() };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Header
            div {
                style: "display: flex; align-items: center; justify-content: space-between;",

                div {
                    style: "display: flex; align-items: center; gap: 12px;",
                    h2 { style: "font-size: 24px; color: {c.text_white};", "📁 Mes documents" }
                    span {
                        style: "padding: 4px 10px; background: {c.bg_hover}; border-radius: 12px; font-size: 12px; color: {c.text_secondary};",
                        "{doc_len}"
                    }
                }

                div {
                    style: "display: flex; gap: 8px;",
                    button {
                        style: "padding: 8px 16px; background: {partage_btn_bg}; border: 1px solid {c.border}; border-radius: 4px; color: {partage_btn_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| view.set(if *view.read() == DocView::Partages { DocView::Liste } else { DocView::Partages }),
                        "🔗 Partages ({partage_len})"
                    }
                    button {
                        style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                        onclick: move |_| view.set(DocView::Upload),
                        "➕ Ajouter un document"
                    }
                }
            }

            // Alerte expiration
            if expiring > 0 {
                div {
                    style: "background: {c.accent_orange}15; border: 1px solid {c.accent_orange}40; border-radius: 8px; padding: 12px 16px; display: flex; align-items: center; gap: 12px;",
                    span { style: "font-size: 20px;", "⚠️" }
                    p { style: "font-size: 13px; color: {c.accent_orange};", "{expiring} document(s) expirant prochainement" }
                }
            }

            // Contenu selon la vue
            {
                let current_view = *view.read();
                match current_view {
                    DocView::Liste => rsx! {
                        DocumentsList { exposant_id: exposant_id.clone(), documents: documents.clone() }
                    },
                    DocView::Upload => rsx! {
                        UploadDocument {
                            exposant_id: exposant_id.clone(),
                            on_done: move |()| view.set(DocView::Liste),
                        }
                    },
                    DocView::Partages => rsx! {
                        PartagesList { exposant_id: exposant_id.clone(), partages: partages.clone() }
                    },
                }
            }
        }
    }
}

/// Liste des documents (XP-E09).
#[component]
fn DocumentsList(
    exposant_id: String,
    documents: Vec<jayxpose::data::DocumentProfessionnel>,
) -> Element {
    let db = crate::use_db();

    if documents.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucun document".to_string(),
                message: "Ajoutez vos documents professionnels dans le coffre-fort.".to_string(),
                icon: "📁".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for doc in documents.iter() {
                {
                    let doc_id = doc.id.clone().unwrap_or_default();
                    let doc_id_del = doc_id.clone();
                    let db_del = db.clone();
                    rsx! {
                        DocumentRow {
                            key: "{doc_id}",
                            doc_type: doc.doc_type.clone().unwrap_or_default(),
                            label: doc.label.clone().unwrap_or_else(|| doc.doc_type.clone().unwrap_or_else(|| "Document".to_string())),
                            file_name: doc.file_name.clone().unwrap_or_else(|| "fichier".to_string()),
                            status: doc.status.clone().unwrap_or_else(|| "en_attente".to_string()),
                            expires_at: doc.expires_at.clone().unwrap_or_default(),
                            version: doc.version.unwrap_or(1),
                            on_view: move |_| {
                                // TODO: ouvrir le document
                            },
                            on_delete: move |_| {
                                let _ = db_del.document_delete(&doc_id_del);
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Upload document (XP-E10).
#[component]
fn UploadDocument(
    exposant_id: String,
    on_done: EventHandler<()>,
) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    let mut doc_type = use_signal(|| "rib".to_string());
    let mut custom_label = use_signal(String::new);
    let mut file_name = use_signal(String::new);
    let mut expires_at = use_signal(String::new);
    let mut save_msg = use_signal(String::new);

    let is_autre = doc_type.read().as_str() == "autre";

    let db_save = db.clone();
    let on_save = move |_| {
        let dt = doc_type.read().clone();
        let label = if dt == "autre" { custom_label.read().clone() } else { dt.clone() };
        let fname = file_name.read().trim().to_string();

        if fname.is_empty() {
            save_msg.set("Le nom de fichier est requis.".to_string());
            return;
        }

        let doc = jayxpose::data::DocumentProfessionnel {
            id: Some(uuid::Uuid::new_v4().to_string()),
            exposant_id: Some(exposant_id.clone()),
            doc_type: Some(dt),
            label: Some(label),
            file_name: Some(fname.clone()),
            file_url: Some(format!("local://{fname}")),
            status: Some("en_attente".to_string()),
            expires_at: Some(expires_at.read().clone()).filter(|s| !s.is_empty()),
            version: Some(1),
            ..Default::default()
        };

        match db_save.document_insert(&doc) {
            Ok(()) => on_done.call(()),
            Err(e) => save_msg.set(format!("Erreur: {e}")),
        }
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 600px;",

            button {
                style: "align-self: flex-start; padding: 8px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                onclick: move |_| on_done.call(()),
                "← Retour"
            }

            h3 { style: "font-size: 20px; color: {c.text_white};", "📤 Ajouter un document" }

            if !save_msg.read().is_empty() {
                div {
                    style: "padding: 10px 16px; background: {c.accent_red}15; border: 1px solid {c.accent_red}40; border-radius: 4px; font-size: 13px; color: {c.accent_red};",
                    "{save_msg}"
                }
            }

            FormSection {
                title: "Type de document".to_string(),

                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",
                    label { style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;", "Type" }
                    select {
                        style: "padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                        onchange: move |evt| doc_type.set(evt.value()),
                        value: "{doc_type}",

                        option { value: "rib", "🏦 RIB" }
                        option { value: "assurance", "🛡️ Assurance" }
                        option { value: "kbis", "📄 KBIS" }
                        option { value: "immatriculation", "📋 Immatriculation" }
                        option { value: "licence", "📜 Licence" }
                        option { value: "urssaf", "📑 URSSAF" }
                        option { value: "carte_pro", "🪪 Carte professionnelle" }
                        option { value: "diplome", "🎓 Diplome" }
                        option { value: "sanitaire", "🏥 Certificat sanitaire" }
                        option { value: "autre", "📎 Autre" }
                    }
                }

                if is_autre {
                    super::components::FormField {
                        label: "Libelle personnalise".to_string(),
                        value: custom_label.read().clone(),
                        placeholder: "Ex: Certificat bio".to_string(),
                        oninput: move |evt: FormEvent| custom_label.set(evt.value()),
                    }
                }
            }

            FormSection {
                title: "Fichier".to_string(),

                super::components::FormField {
                    label: "Nom du fichier".to_string(),
                    value: file_name.read().clone(),
                    placeholder: "document.pdf".to_string(),
                    oninput: move |evt: FormEvent| file_name.set(evt.value()),
                }

                p { style: "font-size: 11px; color: {c.text_muted};", "Formats acceptes : PDF, PNG, JPG. Max 10 Mo." }
            }

            FormSection {
                title: "Expiration".to_string(),

                super::components::FormField {
                    label: "Date d'expiration".to_string(),
                    value: expires_at.read().clone(),
                    field_type: "date".to_string(),
                    optional: true,
                    oninput: move |evt: FormEvent| expires_at.set(evt.value()),
                }
            }

            div {
                style: "display: flex; justify-content: flex-end; gap: 8px;",
                button {
                    style: "padding: 10px 20px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| on_done.call(()),
                    "Annuler"
                }
                button {
                    style: "padding: 10px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; font-weight: 500;",
                    onclick: on_save,
                    "💾 Enregistrer"
                }
            }
        }
    }
}

/// Liste des partages actifs.
#[component]
fn PartagesList(
    exposant_id: String,
    partages: Vec<jayxpose::data::DocumentPartage>,
) -> Element {
    let c = use_palette();
    let db = crate::use_db();

    if partages.is_empty() {
        return rsx! {
            EmptyState {
                title: "Aucun partage".to_string(),
                message: "Les demandes de partage de documents apparaitront ici.".to_string(),
                icon: "🔗".to_string(),
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            for partage in partages.iter() {
                {
                    let p_id = partage.id.clone().unwrap_or_default();
                    let p_id_accept = p_id.clone();
                    let p_id_refuse = p_id.clone();
                    let p_id_revoke = p_id.clone();
                    let status = partage.status.clone().unwrap_or_else(|| "demande".to_string());
                    let status_color = match status.as_str() {
                        "accepte" => c.accent_green.to_string(),
                        "refuse" | "revoque" => c.accent_red.to_string(),
                        _ => c.accent_orange.to_string(),
                    };
                    let doc_id_display = partage.document_id.clone().unwrap_or_else(|| "?".to_string());
                    let context = partage.target_context_type.clone();
                    let is_demande = status == "demande";
                    let is_accepte = status == "accepte";
                    let db_accept = db.clone();
                    let db_refuse = db.clone();
                    let db_revoke = db.clone();

                    rsx! {
                        div {
                            key: "{p_id}",
                            style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 4px;",

                            div {
                                div {
                                    style: "display: flex; align-items: center; gap: 8px;",
                                    span { style: "font-size: 16px;", "🔗" }
                                    p { style: "font-size: 13px; color: {c.text_primary};",
                                        "Document: {doc_id_display}"
                                    }
                                    Badge { text: status.clone(), color: status_color.clone() }
                                }
                                if let Some(ref ctx) = context {
                                    p { style: "font-size: 11px; color: {c.text_muted}; margin-top: 2px;", "Contexte: {ctx}" }
                                }
                            }

                            if is_demande {
                                div {
                                    style: "display: flex; gap: 4px;",
                                    button {
                                        style: "padding: 6px 12px; background: {c.accent_green}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            let _ = db_accept.partage_update_status(&p_id_accept, "accepte");
                                        },
                                        "Accepter"
                                    }
                                    button {
                                        style: "padding: 6px 12px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                        onclick: move |_| {
                                            let _ = db_refuse.partage_update_status(&p_id_refuse, "refuse");
                                        },
                                        "Refuser"
                                    }
                                }
                            }
                            if is_accepte {
                                button {
                                    style: "padding: 6px 12px; background: transparent; border: 1px solid {c.accent_red}; color: {c.accent_red}; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                    onclick: move |_| {
                                        let _ = db_revoke.partage_update_status(&p_id_revoke, "revoque");
                                    },
                                    "Revoquer"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
