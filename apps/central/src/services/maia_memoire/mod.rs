//! @id     central_maia_memoire
//! @do     afficher_memoire_maia
//! @role   orchestration
//! @layer  app
//! @human  central — écran « Mémoire de MAIA » (A5) : notes par strate et domaine, historique, correction,
//! @human  export maia-memoire/v1, oubli confirmé, candidats et approbation TAMR, règles ; mention IA.

//! Écran « Mémoire de MAIA » (A5, EX-56).
//!
//! Pourquoi dans Central (et non `cog-web-portal`) : la mémoire est une donnée
//! `nationale` de l'utilisateur, servie par le pont MAIA en zone COG sur
//! l'adresse locale ; Central est l'application locale de l'utilisateur sur
//! son poste et appelle déjà MAIA. Le portail web expose des services à
//! des utilisateurs **externes** (Franche) : la mémoire n'y a pas sa place.
//!
//! - Authentification : [`client`] (visa par défaut, transition explicite) ;
//! - logique (groupes, confirmations, messages) : [`modele`], testée ;
//! - accessibilité : onglets `tablist`, champs étiquetés, messages
//!   `role=status` / `role=alert`, confirmation d'oubli en `alertdialog`
//!   (Échap pour annuler), tout est atteignable au clavier (boutons et
//!   champs natifs, ordre du document).

pub mod client;
pub mod modele;

use dioxus::prelude::*;
use serde_json::json;

use client::{ClientMemoire, ConfigMemoire, Methode};
use modele::{
    domaines, empreinte_affichable, grouper, interpreter_issue, libelle_auteur, libelle_provenance,
    libelle_strate, message_refus, regles, ConfirmationOubli, Consultation, DemandeApprobation,
    Issue, NoteVue, MENTION_IA, STRATES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Onglet {
    #[default]
    Notes,
    Candidats,
    Regles,
    Export,
}

impl Onglet {
    const TOUS: [Onglet; 4] = [
        Onglet::Notes,
        Onglet::Candidats,
        Onglet::Regles,
        Onglet::Export,
    ];

    fn libelle(self) -> &'static str {
        match self {
            Onglet::Notes => "Notes",
            Onglet::Candidats => "Candidats",
            Onglet::Regles => "Règles",
            Onglet::Export => "Exporter",
        }
    }

    fn id(self) -> &'static str {
        match self {
            Onglet::Notes => "memoire-onglet-notes",
            Onglet::Candidats => "memoire-onglet-candidats",
            Onglet::Regles => "memoire-onglet-regles",
            Onglet::Export => "memoire-onglet-export",
        }
    }
}

/// Message affiché (succès ou erreur).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Message {
    texte: String,
    erreur: bool,
}

// Valeurs directes du signal `Option<Message>` (d'où l'enveloppe `Some`).
#[allow(clippy::unnecessary_wraps)]
fn ok(t: impl Into<String>) -> Option<Message> {
    Some(Message {
        texte: t.into(),
        erreur: false,
    })
}

#[allow(clippy::unnecessary_wraps)]
fn ko(t: impl Into<String>) -> Option<Message> {
    Some(Message {
        texte: t.into(),
        erreur: true,
    })
}

/// Recharge la consultation ; rend les notes lues (pour vérifier un effet).
async fn recharger(
    client: &ClientMemoire,
    mut consultation: Signal<Option<Consultation>>,
    mut message: Signal<Option<Message>>,
) -> Option<Vec<NoteVue>> {
    match client
        .envoyer(Methode::Post, "consulter", None, false)
        .await
    {
        Ok(r) if r.statut == 200 => {
            if let Ok(c) = serde_json::from_value::<Consultation>(r.corps) {
                let notes = c.notes.clone();
                consultation.set(Some(c));
                Some(notes)
            } else {
                message.set(ko("Réponse illisible du pont MAIA."));
                None
            }
        }
        Ok(r) => {
            message.set(ko(message_refus(r.statut, &r.corps)));
            None
        }
        Err(e) => {
            message.set(ko(e.0));
            None
        }
    }
}

/// Écrit l'export dans `chemin` (droits `0600`, donnée `nationale`).
fn ecrire_export(chemin: &std::path::Path, octets: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(chemin)?;
    f.write_all(octets)?;
    f.sync_all()
}

#[component]
pub fn MemoireMaiaView() -> Element {
    let palette = crate::state::use_app_state().read().current_theme.palette();
    let client: Result<ClientMemoire, String> = use_hook(|| {
        ConfigMemoire::depuis_env()
            .and_then(ClientMemoire::new)
            .map_err(|e| e.0)
    });

    let consultation = use_signal(|| None::<Consultation>);
    let mut message = use_signal(|| None::<Message>);
    let mut onglet = use_signal(Onglet::default);
    let mut filtre_strate = use_signal(String::new);
    let mut filtre_domaine = use_signal(String::new);
    let mut historique = use_signal(|| None::<(String, Vec<NoteVue>)>);
    let mut correction = use_signal(|| None::<(String, String, String)>);
    let mut oubli = use_signal(|| None::<ConfirmationOubli>);
    let mut approbation = use_signal(|| None::<DemandeApprobation>);
    let mut regle = use_signal(|| ("moi".to_string(), String::new(), "9".to_string()));
    let mut occupe = use_signal(|| false);

    let client_init = client.clone();
    use_hook(move || {
        if let Ok(c) = client_init {
            spawn(async move {
                recharger(&c, consultation, message).await;
            });
        }
    });

    let Ok(client) = client else {
        let erreur = client.err().unwrap_or_default();
        return rsx! {
            section {
                aria_label: "Mémoire de MAIA",
                style: "padding: 18px; display: flex; flex-direction: column; gap: 12px; color: {palette.text_primary};",
                h2 { style: "margin: 0;", "Mémoire de MAIA" }
                p { role: "note", style: "margin: 0; font-size: 13px;", "{MENTION_IA}" }
                p { role: "alert", style: "color: {palette.accent_red};", "{erreur}" }
            }
        };
    };

    let c_ref = consultation.read();
    let notes: Vec<NoteVue> = c_ref.as_ref().map(|c| c.notes.clone()).unwrap_or_default();
    let candidats = c_ref
        .as_ref()
        .map(|c| c.candidats.clone())
        .unwrap_or_default();
    drop(c_ref);
    let fs = filtre_strate.read().clone();
    let fd = filtre_domaine.read().clone();
    let groupes = grouper(
        &notes,
        (!fs.is_empty()).then_some(fs.as_str()),
        (!fd.is_empty()).then_some(fd.as_str()),
    );
    let liste_domaines = domaines(&notes);
    let liste_regles = regles(&notes);
    let nb_candidats = candidats.len();
    let approbation_possible = client.approbation_configuree();
    let actif = *onglet.read();
    let est_occupe = *occupe.read();

    let carte = format!(
        "border: 1px solid {}; border-radius: 10px; padding: 12px; background: {};",
        palette.border, palette.bg_card
    );
    let bouton = format!(
        "padding: 6px 12px; border-radius: 6px; border: 1px solid {}; background: {}; color: {}; cursor: pointer;",
        palette.border, palette.bg_secondary, palette.text_primary
    );
    let bouton_danger = format!(
        "padding: 6px 12px; border-radius: 6px; border: 1px solid {0}; background: transparent; color: {0}; cursor: pointer;",
        palette.accent_red
    );
    let champ = format!(
        "padding: 6px; border-radius: 6px; border: 1px solid {}; background: {}; color: {};",
        palette.border, palette.bg_main, palette.text_primary
    );

    // Actions (clones bon marché du client, signaux copiables).
    let c_valider = client.clone();
    let valider = move |id: String| {
        let c = c_valider.clone();
        spawn(async move {
            if !client::segment_valide(&id) {
                message.set(ko("Identifiant de candidat invalide."));
                return;
            }
            occupe.set(true);
            match c
                .envoyer(
                    Methode::Post,
                    &format!("candidats/{id}/valider"),
                    None,
                    false,
                )
                .await
            {
                Ok(r) => match interpreter_issue(r.statut, &r.corps) {
                    Ok(Issue::Ecrite(n)) => {
                        message.set(ok(format!("Candidat validé : note {n} écrite.")));
                        recharger(&c, consultation, message).await;
                    }
                    Ok(Issue::Approbation(d)) => {
                        message.set(ok("Branche sensible : approbation TAMR requise. Vérifiez l'empreinte exacte ci-dessous."));
                        approbation.set(Some(d));
                    }
                    Ok(Issue::Candidat(k)) => message.set(ok(format!("Candidat {k} en attente."))),
                    Err(e) => message.set(ko(e)),
                },
                Err(e) => message.set(ko(e.0)),
            }
            occupe.set(false);
        });
    };
    let c_rejeter = client.clone();
    let rejeter = move |id: String| {
        let c = c_rejeter.clone();
        spawn(async move {
            if !client::segment_valide(&id) {
                message.set(ko("Identifiant de candidat invalide."));
                return;
            }
            match c
                .envoyer(Methode::Delete, &format!("candidats/{id}"), None, false)
                .await
            {
                Ok(r) if r.statut == 200 => {
                    message.set(ok("Candidat refusé : rien n'est entré en mémoire."));
                    recharger(&c, consultation, message).await;
                }
                Ok(r) => message.set(ko(message_refus(r.statut, &r.corps))),
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_approuver = client.clone();
    let approuver = move |d: DemandeApprobation| {
        let c = c_approuver.clone();
        spawn(async move {
            if !client::segment_valide(&d.intervention_id) {
                message.set(ko("Identifiant d'intervention invalide."));
                return;
            }
            let corps = json!({ "empreinte": d.empreinte });
            match c
                .envoyer(
                    Methode::Post,
                    &format!("approbations/{}", d.intervention_id),
                    Some(&corps),
                    true,
                )
                .await
            {
                Ok(r) => match interpreter_issue(r.statut, &r.corps) {
                    Ok(Issue::Ecrite(n)) => {
                        message.set(ok(format!(
                            "Approbation accordée par StrongFather : note {n} écrite."
                        )));
                        approbation.set(None);
                        recharger(&c, consultation, message).await;
                    }
                    Ok(_) => message.set(ko("Réponse inattendue du pont.")),
                    Err(e) => {
                        message.set(ko(e));
                        approbation.set(None);
                    }
                },
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_refuser = client.clone();
    let refuser_approbation = move |d: DemandeApprobation| {
        let c = c_refuser.clone();
        spawn(async move {
            if !client::segment_valide(&d.intervention_id) {
                return;
            }
            match c
                .envoyer(
                    Methode::Delete,
                    &format!("approbations/{}", d.intervention_id),
                    None,
                    false,
                )
                .await
            {
                Ok(r) if r.statut == 200 => {
                    message.set(ok("Approbation refusée : rien n'est écrit."))
                }
                Ok(r) => message.set(ko(message_refus(r.statut, &r.corps))),
                Err(e) => message.set(ko(e.0)),
            }
            approbation.set(None);
        });
    };
    let c_historique = client.clone();
    let voir_historique = move |id: String| {
        let c = c_historique.clone();
        spawn(async move {
            match c
                .envoyer(
                    Methode::Post,
                    "historique",
                    Some(&json!({ "id": id })),
                    false,
                )
                .await
            {
                Ok(r) if r.statut == 200 => {
                    let v: Vec<NoteVue> =
                        serde_json::from_value(r.corps["versions"].clone()).unwrap_or_default();
                    historique.set(Some((id, v)));
                }
                Ok(r) => message.set(ko(message_refus(r.statut, &r.corps))),
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_corriger = client.clone();
    let enregistrer_correction = move |_| {
        let c = c_corriger.clone();
        let Some((id, texte, importance)) = correction.read().clone() else {
            return;
        };
        spawn(async move {
            let corps = match modele::corps_correction(&id, &texte, Some(&importance)) {
                Ok(v) => v,
                Err(e) => {
                    message.set(ko(e));
                    return;
                }
            };
            match c
                .envoyer(Methode::Post, "corriger", Some(&corps), false)
                .await
            {
                Ok(r) => match interpreter_issue(r.statut, &r.corps) {
                    Ok(Issue::Ecrite(n)) => {
                        message.set(ok(format!("Note {n} corrigée (nouvelle version).")));
                        correction.set(None);
                        recharger(&c, consultation, message).await;
                    }
                    Ok(Issue::Candidat(_)) => {
                        message.set(ok("Branche sensible : la correction attend votre validation avec approbation TAMR (onglet Candidats)."));
                        correction.set(None);
                        recharger(&c, consultation, message).await;
                    }
                    Ok(Issue::Approbation(_)) => message.set(ko("Réponse inattendue du pont.")),
                    Err(e) => message.set(ko(e)),
                },
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_oublier = client.clone();
    let confirmer_oubli = move |_| {
        let c = c_oublier.clone();
        let Some(conf) = oubli.read().clone() else {
            return;
        };
        let Some(corps) = conf.corps() else {
            message.set(ko(
                "Confirmation incomplète : retapez le chemin exact et donnez un motif.",
            ));
            return;
        };
        spawn(async move {
            match c
                .envoyer(Methode::Delete, "notes", Some(&corps), false)
                .await
            {
                Ok(r) if r.statut == 200 => {
                    oubli.set(None);
                    match recharger(&c, consultation, message).await {
                        Some(apres) if modele::oubli_effectif(&conf.chemin, &apres) => {
                            message.set(ok(format!("« {} » est oublié : effacement vérifié, pierre tombale sans contenu.", conf.chemin)));
                        }
                        Some(_) => message.set(ko(
                            "L'oubli n'a pas été constaté : des notes restent. Réessayez.",
                        )),
                        None => {}
                    }
                }
                Ok(r) => message.set(ko(message_refus(r.statut, &r.corps))),
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_regle = client.clone();
    let poser_regle = move |_| {
        let c = c_regle.clone();
        let (b, t, i) = regle.read().clone();
        spawn(async move {
            let corps = match modele::corps_regle(&b, &t, &i) {
                Ok(v) => v,
                Err(e) => {
                    message.set(ko(e));
                    return;
                }
            };
            match c
                .envoyer(Methode::Post, "regles", Some(&corps), false)
                .await
            {
                Ok(r) => match interpreter_issue(r.statut, &r.corps) {
                    Ok(Issue::Ecrite(n)) => {
                        message.set(ok(format!("Règle {n} posée.")));
                        regle.set((b, String::new(), "9".into()));
                        recharger(&c, consultation, message).await;
                    }
                    Ok(_) => message.set(ok("Règle en attente de validation.")),
                    Err(e) => message.set(ko(e)),
                },
                Err(e) => message.set(ko(e.0)),
            }
        });
    };
    let c_export = client.clone();
    let exporter = move |_| {
        let c = c_export.clone();
        spawn(async move {
            let r = match c.envoyer(Methode::Post, "exporter", None, false).await {
                Ok(r) if r.statut == 200 => r,
                Ok(r) => {
                    message.set(ko(message_refus(r.statut, &r.corps)));
                    return;
                }
                Err(e) => {
                    message.set(ko(e.0));
                    return;
                }
            };
            let resume = match modele::verifier_export(&r.corps) {
                Ok(s) => s,
                Err(e) => {
                    message.set(ko(e));
                    return;
                }
            };
            let nom = modele::nom_fichier_export(
                &c.config().utilisateur,
                &chrono::Local::now().format("%Y-%m-%d").to_string(),
            );
            let Some(fichier) = rfd::AsyncFileDialog::new()
                .set_file_name(&nom)
                .add_filter("JSON", &["json"])
                .save_file()
                .await
            else {
                message.set(ok("Export annulé."));
                return;
            };
            let octets = serde_json::to_vec_pretty(&r.corps).unwrap_or_default();
            match ecrire_export(fichier.path(), &octets) {
                Ok(()) => message.set(ok(format!(
                    "Export maia-memoire/v1 enregistré ({} notes, {} versions, {} candidats, {} pierres tombales) : {}",
                    resume.notes,
                    resume.versions,
                    resume.candidats,
                    resume.pierres_tombales,
                    fichier.path().display()
                ))),
                Err(e) => message.set(ko(format!("Écriture de l'export impossible : {e}"))),
            }
        });
    };
    let c_actualiser = client.clone();
    let actualiser = move |_| {
        let c = c_actualiser.clone();
        spawn(async move {
            if recharger(&c, consultation, message).await.is_some() {
                message.set(ok("Mémoire actualisée."));
            }
        });
    };

    let msg = message.read().clone();
    let demande = approbation.read().clone();
    let conf_oubli = oubli.read().clone();
    let hist = historique.read().clone();
    let corr = correction.read().clone();
    let (regle_b, regle_t, regle_i) = regle.read().clone();

    rsx! {
        section {
            aria_label: "Mémoire de MAIA",
            style: "padding: 18px; display: flex; flex-direction: column; gap: 12px; color: {palette.text_primary};",

            div { style: "display: flex; align-items: center; gap: 12px;",
                h2 { style: "margin: 0; font-size: 20px; flex: 1;", "Mémoire de MAIA" }
                span { style: "font-size: 12px; color: {palette.text_secondary};", "Utilisateur : {client.config().utilisateur}" }
                button { style: "{bouton}", onclick: actualiser, "Actualiser" }
            }
            p {
                role: "note",
                style: "margin: 0; padding: 8px 12px; border-left: 4px solid {palette.accent_blue}; background: {palette.bg_secondary}; font-size: 13px;",
                strong { "IA · " }
                "{MENTION_IA}"
            }

            if let Some(m) = msg {
                if m.erreur {
                    p { role: "alert", style: "margin: 0; color: {palette.accent_red};", "{m.texte}" }
                } else {
                    p { role: "status", aria_live: "polite", style: "margin: 0; color: {palette.accent_green};", "{m.texte}" }
                }
            }

            div { role: "tablist", aria_label: "Sections de la mémoire", style: "display: flex; gap: 6px;",
                for o in Onglet::TOUS {
                    button {
                        key: "{o.id()}",
                        id: "{o.id()}",
                        role: "tab",
                        aria_selected: if actif == o { "true" } else { "false" },
                        aria_controls: "memoire-panneau",
                        style: if actif == o { format!("{bouton} border-color: {}; font-weight: 600;", palette.accent_blue) } else { bouton.clone() },
                        onclick: move |_| onglet.set(o),
                        if o == Onglet::Candidats { "{o.libelle()} ({nb_candidats})" } else { "{o.libelle()}" }
                    }
                }
            }

            div { id: "memoire-panneau", role: "tabpanel", aria_labelledby: "{actif.id()}",
                style: "display: flex; flex-direction: column; gap: 10px;",

                match actif {
                    Onglet::Notes => rsx! {
                        div { style: "display: flex; gap: 12px; align-items: center; flex-wrap: wrap;",
                            label { r#for: "memoire-filtre-strate", "Strate" }
                            select { id: "memoire-filtre-strate", style: "{champ}", value: "{fs}",
                                onchange: move |e| filtre_strate.set(e.value()),
                                option { value: "", "Toutes" }
                                for s in STRATES { option { key: "{s}", value: "{s}", "{libelle_strate(s)}" } }
                            }
                            label { r#for: "memoire-filtre-domaine", "Domaine" }
                            select { id: "memoire-filtre-domaine", style: "{champ}", value: "{fd}",
                                onchange: move |e| filtre_domaine.set(e.value()),
                                option { value: "", "Tous" }
                                for d in liste_domaines.clone() { option { key: "{d}", value: "{d}", "{d}" } }
                            }
                        }
                        if groupes.is_empty() {
                            p { style: "color: {palette.text_muted};", "Aucune note." }
                        }
                        for g in groupes {
                            section { key: "{g.strate}", aria_label: "{libelle_strate(g.strate)}", style: "{carte}",
                                h3 { style: "margin: 0 0 8px 0; font-size: 15px;", "{libelle_strate(g.strate)}" }
                                for d in g.domaines {
                                    div { key: "{g.strate}-{d.domaine}", style: "margin-bottom: 8px;",
                                        div { style: "display: flex; align-items: center; gap: 8px;",
                                            h4 { style: "margin: 4px 0; font-size: 13px; flex: 1;", "Domaine « {d.domaine} »" }
                                            button {
                                                style: "{bouton_danger}",
                                                aria_label: "Oublier tout le domaine {d.domaine}",
                                                onclick: { let ch = d.domaine.clone(); move |_| oubli.set(Some(ConfirmationOubli::new(&ch))) },
                                                "Oublier le domaine"
                                            }
                                        }
                                        ul { style: "list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 6px;",
                                            for n in d.notes {
                                                li { key: "{n.id}", style: "padding: 8px; border-radius: 6px; background: {palette.bg_secondary};",
                                                    div { style: "display: flex; gap: 8px; align-items: baseline; flex-wrap: wrap;",
                                                        code { "{n.id}" }
                                                        span { style: "flex: 1;", "{n.texte}" }
                                                    }
                                                    div { style: "font-size: 12px; color: {palette.text_secondary}; margin-top: 4px;",
                                                        "{libelle_provenance(&n.provenance)} · version {n.version} · importance {n.importance}/9 · confiance {n.confiance}/9 · par {libelle_auteur(&n.auteur)}"
                                                    }
                                                    div { style: "display: flex; gap: 6px; margin-top: 6px;",
                                                        button { style: "{bouton}", aria_label: "Historique de {n.id}",
                                                            onclick: { let id = n.id.clone(); let f = voir_historique.clone(); move |_| f(id.clone()) },
                                                            "Historique"
                                                        }
                                                        button { style: "{bouton}", aria_label: "Corriger {n.id}",
                                                            onclick: { let n2 = n.clone(); move |_| correction.set(Some((n2.id.clone(), n2.texte.clone(), n2.importance.to_string()))) },
                                                            "Corriger"
                                                        }
                                                        button { style: "{bouton_danger}", aria_label: "Oublier {n.id}",
                                                            onclick: { let id = n.id.clone(); move |_| oubli.set(Some(ConfirmationOubli::new(&id))) },
                                                            "Oublier"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Onglet::Candidats => rsx! {
                        p { style: "margin: 0; font-size: 13px; color: {palette.text_secondary};",
                            "Propositions de l'IA en attente : rien n'entre en mémoire sans votre décision. Les branches sensibles exigent en plus une approbation TAMR."
                        }
                        if candidats.is_empty() { p { style: "color: {palette.text_muted};", "Aucun candidat." } }
                        ul { style: "list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px;",
                            for k in candidats {
                                li { key: "{k.id}", style: "{carte}",
                                    div { style: "display: flex; gap: 8px; align-items: baseline; flex-wrap: wrap;",
                                        code { "{k.branche}" }
                                        span { style: "flex: 1;", "{k.texte}" }
                                        if k.sensible {
                                            span { style: "font-size: 12px; padding: 2px 6px; border-radius: 4px; border: 1px solid {palette.accent_orange}; color: {palette.accent_orange};",
                                                "Sensible · approbation TAMR exigée"
                                            }
                                        }
                                    }
                                    div { style: "font-size: 12px; color: {palette.text_secondary}; margin-top: 4px;",
                                        "{libelle_provenance(&k.provenance)} · confiance {k.confiance}/9 · importance {k.importance}/9"
                                        if let Some(cible) = &k.cible { " · corrige {cible}" }
                                    }
                                    div { style: "display: flex; gap: 6px; margin-top: 6px;",
                                        button { style: "{bouton}", disabled: est_occupe, aria_label: "Valider le candidat {k.texte}",
                                            onclick: { let id = k.id.clone(); let f = valider.clone(); move |_| f(id.clone()) },
                                            "Valider"
                                        }
                                        button { style: "{bouton_danger}", aria_label: "Refuser le candidat {k.texte}",
                                            onclick: { let id = k.id.clone(); let f = rejeter.clone(); move |_| f(id.clone()) },
                                            "Refuser"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Onglet::Regles => rsx! {
                        p { style: "margin: 0; font-size: 13px; color: {palette.text_secondary};",
                            "Les règles sont des consignes durables que vous seul posez ; elles sont toujours chargées (N0)."
                        }
                        if liste_regles.is_empty() { p { style: "color: {palette.text_muted};", "Aucune règle." } }
                        ul { style: "margin: 0; padding-left: 18px;",
                            for r in liste_regles {
                                li { key: "{r.id}", code { "{r.id}" } " — {r.texte} (importance {r.importance}/9)" }
                            }
                        }
                        form { style: "{carte} display: grid; grid-template-columns: max-content 1fr; gap: 8px; align-items: center;",
                            aria_label: "Poser une règle",
                            onsubmit: move |e| { e.prevent_default(); },
                            label { r#for: "regle-branche", "Branche" }
                            input { id: "regle-branche", style: "{champ}", value: "{regle_b}",
                                oninput: move |e| regle.with_mut(|r| r.0 = e.value()) }
                            label { r#for: "regle-texte", "Consigne" }
                            input { id: "regle-texte", style: "{champ}", value: "{regle_t}",
                                oninput: move |e| regle.with_mut(|r| r.1 = e.value()) }
                            label { r#for: "regle-importance", "Importance (0–9)" }
                            input { id: "regle-importance", r#type: "number", min: "0", max: "9", style: "{champ}", value: "{regle_i}",
                                oninput: move |e| regle.with_mut(|r| r.2 = e.value()) }
                            span {}
                            button { r#type: "button", style: "{bouton}", onclick: poser_regle, "Poser la règle" }
                        }
                    },
                    Onglet::Export => rsx! {
                        p { style: "margin: 0; font-size: 13px; color: {palette.text_secondary};",
                            "Export intégral au format maia-memoire/v1 : notes, historique complet, candidats et pierres tombales. Le fichier est écrit avec des droits privés (0600) : il contient votre mémoire en clair."
                        }
                        div { button { style: "{bouton}", onclick: exporter, "Exporter ma mémoire" } }
                    },
                }
            }

            if let Some((id, versions)) = hist {
                section { aria_label: "Historique de {id}", style: "{carte}",
                    div { style: "display: flex; align-items: center;",
                        h3 { style: "margin: 0; font-size: 15px; flex: 1;", "Historique de " code { "{id}" } }
                        button { style: "{bouton}", onclick: move |_| historique.set(None), "Fermer l'historique" }
                    }
                    ol { style: "margin: 8px 0 0 0;",
                        for v in versions {
                            li { key: "{v.version}",
                                "v{v.version} · {v.enregistre_le} · {libelle_auteur(&v.auteur)} · {libelle_provenance(&v.provenance)} — {v.texte}"
                            }
                        }
                    }
                }
            }

            if let Some((id, texte, importance)) = corr {
                form { aria_label: "Corriger {id}", style: "{carte} display: grid; grid-template-columns: max-content 1fr; gap: 8px; align-items: center;",
                    onsubmit: move |e| { e.prevent_default(); },
                    onkeydown: move |e: KeyboardEvent| if e.key() == Key::Escape { correction.set(None) },
                    h3 { style: "grid-column: 1 / 3; margin: 0; font-size: 15px;", "Corriger " code { "{id}" } }
                    label { r#for: "correction-texte", "Nouveau texte" }
                    textarea { id: "correction-texte", autofocus: true, style: "{champ} min-height: 60px;", value: "{texte}",
                        oninput: move |e| correction.with_mut(|c| if let Some(c) = c { c.1 = e.value() }) }
                    label { r#for: "correction-importance", "Importance (0–9)" }
                    input { id: "correction-importance", r#type: "number", min: "0", max: "9", style: "{champ}", value: "{importance}",
                        oninput: move |e| correction.with_mut(|c| if let Some(c) = c { c.2 = e.value() }) }
                    span {}
                    div { style: "display: flex; gap: 6px;",
                        button { r#type: "button", style: "{bouton}", onclick: enregistrer_correction, "Enregistrer la correction" }
                        button { r#type: "button", style: "{bouton}", onclick: move |_| correction.set(None), "Annuler" }
                    }
                }
            }

            if let Some(conf) = conf_oubli {
                div {
                    role: "alertdialog",
                    aria_modal: "true",
                    aria_labelledby: "oubli-titre",
                    aria_describedby: "oubli-effet",
                    style: "{carte} border-color: {palette.accent_red}; display: flex; flex-direction: column; gap: 8px;",
                    onkeydown: move |e: KeyboardEvent| if e.key() == Key::Escape { oubli.set(None) },
                    h3 { id: "oubli-titre", style: "margin: 0; font-size: 15px; color: {palette.accent_red};", "Oublier « {conf.chemin} » ?" }
                    p { id: "oubli-effet", style: "margin: 0; font-size: 13px;", "{conf.effet(&notes)}" }
                    label { r#for: "oubli-saisie", "Pour confirmer, retapez exactement : " code { "{conf.chemin}" } }
                    input { id: "oubli-saisie", autofocus: true, style: "{champ}", value: "{conf.saisie}",
                        oninput: move |e| oubli.with_mut(|o| if let Some(o) = o { o.saisie = e.value() }) }
                    label { r#for: "oubli-motif", "Motif (conservé sans le contenu)" }
                    input { id: "oubli-motif", style: "{champ}", value: "{conf.motif}",
                        oninput: move |e| oubli.with_mut(|o| if let Some(o) = o { o.motif = e.value() }) }
                    div { style: "display: flex; gap: 6px;",
                        button { style: "{bouton_danger}", disabled: !conf.peut_confirmer(), onclick: confirmer_oubli, "Oublier définitivement" }
                        button { style: "{bouton}", onclick: move |_| oubli.set(None), "Annuler" }
                    }
                }
            }

            if let Some(d) = demande {
                div {
                    role: "alertdialog",
                    aria_modal: "true",
                    aria_labelledby: "tamr-titre",
                    aria_describedby: "tamr-description",
                    style: "{carte} border-color: {palette.accent_orange}; display: flex; flex-direction: column; gap: 8px;",
                    h3 { id: "tamr-titre", style: "margin: 0; font-size: 15px;", "Approbation TAMR requise" }
                    p { id: "tamr-description", style: "margin: 0; white-space: pre-wrap;", "{d.description}" }
                    p { style: "margin: 0; font-size: 13px;", "Empreinte exacte de l'action à approuver (SHA-256) :" }
                    code {
                        aria_label: "Empreinte {d.empreinte}",
                        style: "font-size: 14px; padding: 8px; background: {palette.bg_main}; border-radius: 6px; user-select: all;",
                        "{empreinte_affichable(&d.empreinte)}"
                    }
                    p { style: "margin: 0; font-size: 12px; color: {palette.text_secondary};",
                        "La décision est prise par StrongFather ; expiration dans {d.expire_dans_s} s. Vérifiez que l'empreinte affichée par votre appareil d'approbation est identique."
                    }
                    if !approbation_possible {
                        p { role: "alert", style: "margin: 0; color: {palette.accent_red};",
                            "Aucune identité d'approbation n'est configurée sur ce poste (dossier « approbation » du visa)."
                        }
                    }
                    div { style: "display: flex; gap: 6px;",
                        button { style: "{bouton}", disabled: !approbation_possible,
                            onclick: { let d2 = d.clone(); let f = approuver.clone(); move |_| f(d2.clone()) },
                            "Approuver cette empreinte"
                        }
                        button { style: "{bouton_danger}",
                            onclick: { let d2 = d.clone(); let f = refuser_approbation.clone(); move |_| f(d2.clone()) },
                            "Refuser"
                        }
                    }
                }
            }
        }
    }
}
