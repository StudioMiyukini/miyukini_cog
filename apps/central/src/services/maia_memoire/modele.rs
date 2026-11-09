//! @id     central_maia_memoire_modele
//! @do     preparer_vue_memoire_maia
//! @role   orchestration
//! @layer  app
//! @human  central — modèle de l'écran « Mémoire de MAIA » : notes par strate et domaine, règles, candidats,
//! @human  approbation TAMR (empreinte exacte), confirmation d'oubli, messages de refus ; sans rendu, testé.

//! Logique de l'écran « Mémoire de MAIA » (A5), séparée du rendu Dioxus.
//!
//! Formats lus (réponses du pont, `apps/miou-llm-bridge/src/routes_memoire.rs`) :
//! `consulter` → `{"notes":[…, "strate", "domaine"], "candidats":[…]}`,
//! `historique` → `{"versions":[…]}`, `valider` → `{"ecrite"}` ou 202
//! `{"approbation":{intervention_id, empreinte, description, expire_dans_s…}}`,
//! `corriger` / `regles` → `{"ecrite"}` ou 202 `{"candidat"}`, refus →
//! `{"reason"}` ou `{"error"}`. Champs inconnus ignorés (lecture seule).

use serde::Deserialize;
use serde_json::Value;

/// Mention IA (AI Act art. 50) affichée en permanence sur l'écran.
pub const MENTION_IA: &str = "MAIA est une intelligence artificielle qui fonctionne sur votre COG. \
    Les candidats sont des propositions de l'IA : rien n'entre dans sa mémoire sans votre validation. \
    Les notes « déduites » viennent de l'IA et ont été validées par vous.";

/// Format d'export attendu.
pub const FORMAT_EXPORT: &str = "maia-memoire/v1";

/// Strates affichées (N4 = sources, jamais injectées, non affichées).
pub const STRATES: [&str; 4] = ["N0", "N1", "N2", "N3"];

/// Libellé d'une strate.
pub fn libelle_strate(s: &str) -> &'static str {
    match s {
        "N0" => "N0 · Noyau (toujours chargé)",
        "N1" => "N1 · Carte des domaines",
        "N2" => "N2 · Sujets",
        "N3" => "N3 · Notes",
        _ => "Strate inconnue",
    }
}

/// Libellé d'une provenance.
pub fn libelle_provenance(p: &str) -> &'static str {
    match p {
        "utilisateur" => "Dit par vous",
        "deduit" => "Déduit par l'IA",
        "etranger" => "Venu d'ailleurs",
        _ => "Provenance inconnue",
    }
}

/// Libellé d'un auteur de version.
pub fn libelle_auteur(a: &str) -> &'static str {
    match a {
        "utilisateur" => "vous",
        "validation_utilisateur" => "proposition de l'IA validée par vous",
        "maia" => "l'IA (MAIA)",
        _ => "inconnu",
    }
}

/// Version d'une note, telle que rendue par le pont.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NoteVue {
    /// Identifiant hiérarchique.
    pub id: String,
    /// Version.
    pub version: u32,
    /// Texte.
    pub texte: String,
    /// Importance 0–9.
    pub importance: u8,
    /// Confiance 0–9.
    pub confiance: u8,
    /// Provenance.
    pub provenance: String,
    /// Auteur de la version.
    pub auteur: String,
    /// Strate (`N0`…`N3`), notes courantes seulement.
    #[serde(default)]
    pub strate: String,
    /// Domaine (premier segment).
    #[serde(default)]
    pub domaine: String,
    /// Enregistrement de la version.
    #[serde(default)]
    pub enregistre_le: String,
}

impl NoteVue {
    /// Domaine : champ du pont, à défaut premier segment de l'identifiant.
    pub fn domaine(&self) -> &str {
        if self.domaine.is_empty() {
            self.id.split('.').next().unwrap_or_default()
        } else {
            &self.domaine
        }
    }

    /// Vrai si la note est une Règle (suffixe `r<n>`).
    pub fn est_regle(&self) -> bool {
        lettre_type(&self.id) == Some('r')
    }
}

/// Lettre de type du dernier segment (`f12` ⇒ `f`).
pub fn lettre_type(id: &str) -> Option<char> {
    let dernier = id.rsplit('.').next()?;
    let mut c = dernier.chars();
    let l = c.next()?;
    let reste = c.as_str();
    (!reste.is_empty() && reste.bytes().all(|b| b.is_ascii_digit())).then_some(l)
}

/// Candidat en attente (proposition de l'IA).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CandidatVue {
    /// Identifiant (UUID).
    pub id: String,
    /// Note corrigée (`None` : nouvelle note).
    #[serde(default)]
    pub cible: Option<String>,
    /// Branche.
    pub branche: String,
    /// Type d'unité.
    pub type_unite: String,
    /// Texte proposé.
    pub texte: String,
    /// Importance.
    pub importance: u8,
    /// Confiance.
    pub confiance: u8,
    /// Provenance.
    pub provenance: String,
    /// Branche sensible : approbation TAMR exigée.
    pub sensible: bool,
}

/// Réponse de `consulter`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct Consultation {
    /// Notes courantes.
    pub notes: Vec<NoteVue>,
    /// Candidats en attente.
    pub candidats: Vec<CandidatVue>,
}

/// Demande d'approbation TAMR rendue par `valider` (202).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DemandeApprobation {
    /// Intervention.
    pub intervention_id: String,
    /// Empreinte exacte de l'action à approuver (SHA-256 hexadécimal).
    pub empreinte: String,
    /// Description exacte de l'action.
    pub description: String,
    /// Secondes restantes.
    #[serde(default)]
    pub expire_dans_s: u64,
}

/// Issue d'une écriture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// Note écrite (identifiant).
    Ecrite(String),
    /// Branche sensible : un candidat attend (à valider avec TAMR).
    Candidat(String),
    /// Validation sensible : approbation TAMR demandée.
    Approbation(DemandeApprobation),
}

/// Interprète la réponse d'une écriture (corriger, règle, valider).
pub fn interpreter_issue(statut: u16, corps: &Value) -> Result<Issue, String> {
    match statut {
        200 => corps["ecrite"]
            .as_str()
            .map(|s| Issue::Ecrite(s.to_string()))
            .ok_or_else(|| "réponse inattendue du pont".to_string()),
        202 => {
            if let Some(k) = corps["candidat"].as_str() {
                return Ok(Issue::Candidat(k.to_string()));
            }
            let d: DemandeApprobation = serde_json::from_value(corps["approbation"].clone())
                .map_err(|_| "demande d'approbation illisible".to_string())?;
            if !empreinte_valide(&d.empreinte) {
                return Err("empreinte d'approbation invalide".to_string());
            }
            Ok(Issue::Approbation(d))
        }
        _ => Err(message_refus(statut, corps)),
    }
}

/// Vrai si `e` est une empreinte SHA-256 hexadécimale minuscule.
pub fn empreinte_valide(e: &str) -> bool {
    e.len() == 64
        && e.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Empreinte lisible : groupes de 8 caractères (la valeur exacte est
/// conservée et envoyée telle quelle).
pub fn empreinte_affichable(e: &str) -> String {
    e.as_bytes()
        .chunks(8)
        .map(|c| String::from_utf8_lossy(c).into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Message d'un refus du pont (motif `reason`, sinon `error`, sinon statut).
pub fn message_refus(statut: u16, corps: &Value) -> String {
    let motif = corps["reason"].as_str().unwrap_or_default();
    let texte = match motif {
        "visa_refuse" => {
            "Visa refusé : il a peut-être expiré (15 min au plus). Renouvelez-le puis réessayez."
        }
        "visa_requis" => "Un visa est requis pour accéder à votre mémoire.",
        "visa_desactive" => "Le pont MAIA n'accepte pas encore les visas (configuration du COG).",
        "portee_insuffisante" => "Votre visa ne permet pas cette action (portée insuffisante).",
        "autre_utilisateur" => {
            "Ce visa n'est pas celui du titulaire de cette mémoire : accès refusé."
        }
        "appareil_approbation_requis" => {
            "Cette approbation doit venir de votre appareil d'approbation."
        }
        "approbations_desactivees" => "Les approbations TAMR sont désactivées sur ce COG.",
        "memoire_http_desactivee" => {
            "La mémoire de MAIA n'est pas disponible (désactivée ou non ouverte)."
        }
        "debit_depasse" => "Trop de requêtes : patientez une minute.",
        "signature_utilisateur_requise" | "signature_approbation_requise" => {
            "Signature refusée (mode de transition)."
        }
        "requete_refusee" | "corps_trop_volumineux" | "cookie_refuse" => {
            "Requête refusée par le pont."
        }
        _ => "",
    };
    if !texte.is_empty() {
        return texte.to_string();
    }
    match corps["error"].as_str() {
        Some(e) if statut == 403 && corps["tamr"] == "refus" => {
            format!("Approbation refusée : {e}")
        }
        Some(e) => format!("Refus du pont ({statut}) : {e}"),
        None => format!("Refus du pont ({statut})."),
    }
}

/// Domaines présents, triés.
pub fn domaines(notes: &[NoteVue]) -> Vec<String> {
    let mut d: Vec<String> = notes.iter().map(|n| n.domaine().to_string()).collect();
    d.sort();
    d.dedup();
    d
}

/// Notes d'un domaine dans une strate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupeDomaine {
    /// Domaine.
    pub domaine: String,
    /// Notes (triées par identifiant).
    pub notes: Vec<NoteVue>,
}

/// Notes d'une strate, par domaine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupeStrate {
    /// Strate.
    pub strate: &'static str,
    /// Domaines.
    pub domaines: Vec<GroupeDomaine>,
}

/// Groupe les notes par strate (N0 → N3) puis domaine, après filtres
/// (`None` : tout). Les strates vides sont omises.
pub fn grouper(
    notes: &[NoteVue],
    strate: Option<&str>,
    domaine: Option<&str>,
) -> Vec<GroupeStrate> {
    STRATES
        .iter()
        .filter(|s| strate.is_none_or(|f| f == **s))
        .filter_map(|s| {
            let mut dans: Vec<&NoteVue> = notes
                .iter()
                .filter(|n| n.strate == *s && domaine.is_none_or(|d| n.domaine() == d))
                .collect();
            if dans.is_empty() {
                return None;
            }
            dans.sort_by(|a, b| a.id.cmp(&b.id));
            let mut groupes: Vec<GroupeDomaine> = Vec::new();
            for n in dans {
                match groupes.last_mut() {
                    Some(g) if g.domaine == n.domaine() => g.notes.push(n.clone()),
                    _ => groupes.push(GroupeDomaine {
                        domaine: n.domaine().to_string(),
                        notes: vec![n.clone()],
                    }),
                }
            }
            groupes.sort_by(|a, b| a.domaine.cmp(&b.domaine));
            Some(GroupeStrate {
                strate: s,
                domaines: groupes,
            })
        })
        .collect()
}

/// Règles posées par l'utilisateur.
pub fn regles(notes: &[NoteVue]) -> Vec<NoteVue> {
    let mut r: Vec<NoteVue> = notes.iter().filter(|n| n.est_regle()).cloned().collect();
    r.sort_by(|a, b| a.id.cmp(&b.id));
    r
}

/// Vrai si `id` est `chemin` ou dans son sous-arbre.
pub fn sous_chemin(id: &str, chemin: &str) -> bool {
    id == chemin || id.strip_prefix(chemin).is_some_and(|r| r.starts_with('.'))
}

/// Confirmation explicite d'un oubli : l'utilisateur doit **retaper le
/// chemin exact** et donner un motif.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationOubli {
    /// Chemin à oublier (note ou branche).
    pub chemin: String,
    /// Saisie de confirmation.
    pub saisie: String,
    /// Motif (conservé dans la pierre tombale, sans contenu).
    pub motif: String,
}

impl ConfirmationOubli {
    /// Nouvelle confirmation pour `chemin`.
    pub fn new(chemin: &str) -> Self {
        Self {
            chemin: chemin.to_string(),
            saisie: String::new(),
            motif: String::new(),
        }
    }

    /// Vrai si l'oubli peut être envoyé.
    pub fn peut_confirmer(&self) -> bool {
        !self.chemin.is_empty()
            && self.saisie.trim() == self.chemin
            && !self.motif.trim().is_empty()
    }

    /// Effet annoncé avant l'oubli.
    pub fn effet(&self, notes: &[NoteVue]) -> String {
        let n = notes
            .iter()
            .filter(|x| sous_chemin(&x.id, &self.chemin))
            .count();
        format!(
            "Oubli définitif de « {} » : {n} note(s) et toutes leurs versions seront effacées \
             de la base chiffrée. Seule une pierre tombale sans contenu (identifiant, date, motif) \
             restera. Cette action ne peut pas être annulée.",
            self.chemin
        )
    }

    /// Corps de la requête d'oubli (seulement si confirmée).
    pub fn corps(&self) -> Option<Value> {
        self.peut_confirmer()
            .then(|| serde_json::json!({ "chemin": self.chemin, "motif": self.motif.trim() }))
    }
}

/// Vérifie l'effet d'un oubli sur la consultation suivante : plus aucune
/// note du chemin.
pub fn oubli_effectif(chemin: &str, apres: &[NoteVue]) -> bool {
    !apres.iter().any(|n| sous_chemin(&n.id, chemin))
}

/// Résumé d'un export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumeExport {
    /// Notes courantes.
    pub notes: usize,
    /// Versions.
    pub versions: usize,
    /// Candidats.
    pub candidats: usize,
    /// Pierres tombales.
    pub pierres_tombales: usize,
}

/// Contrôle un export (`format` = [`FORMAT_EXPORT`], étiquette `nationale`).
pub fn verifier_export(v: &Value) -> Result<ResumeExport, String> {
    if v["format"] != FORMAT_EXPORT {
        return Err("format d'export inattendu".to_string());
    }
    if v["etiquette"] != "nationale" {
        return Err("étiquette d'export inattendue".to_string());
    }
    let n = |k: &str| v[k].as_array().map_or(0, Vec::len);
    Ok(ResumeExport {
        notes: n("notes"),
        versions: n("historique"),
        candidats: n("candidats"),
        pierres_tombales: n("pierres_tombales"),
    })
}

/// Nom de fichier proposé pour l'export.
pub fn nom_fichier_export(utilisateur: &str, date: &str) -> String {
    format!("maia-memoire-{utilisateur}-{date}.json")
}

/// Corps de correction (`importance` facultative, 0–9).
pub fn corps_correction(id: &str, texte: &str, importance: Option<&str>) -> Result<Value, String> {
    let texte = texte.trim();
    if texte.is_empty() {
        return Err("Le texte ne peut pas être vide.".to_string());
    }
    let mut c = serde_json::json!({ "id": id, "texte": texte });
    if let Some(i) = importance.map(str::trim).filter(|i| !i.is_empty()) {
        c["importance"] = importance_valide(i)?.into();
    }
    Ok(c)
}

/// Corps d'une nouvelle Règle.
pub fn corps_regle(branche: &str, texte: &str, importance: &str) -> Result<Value, String> {
    let (branche, texte) = (branche.trim(), texte.trim());
    if branche.is_empty() || texte.is_empty() {
        return Err("Branche et texte sont requis.".to_string());
    }
    if !branche.split('.').all(|s| {
        s.starts_with(|c: char| c.is_ascii_lowercase())
            && s.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    }) {
        return Err(
            "Branche : segments en minuscules séparés par des points (ex. moi.style).".to_string(),
        );
    }
    Ok(
        serde_json::json!({ "branche": branche, "texte": texte, "importance": importance_valide(importance)? }),
    )
}

fn importance_valide(i: &str) -> Result<u8, String> {
    i.trim()
        .parse::<u8>()
        .ok()
        .filter(|v| *v <= 9)
        .ok_or_else(|| "Importance : un nombre de 0 à 9.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(id: &str, strate: &str) -> NoteVue {
        NoteVue {
            id: id.into(),
            version: 1,
            texte: format!("texte {id}"),
            importance: 5,
            confiance: 7,
            provenance: "utilisateur".into(),
            auteur: "utilisateur".into(),
            strate: strate.into(),
            domaine: String::new(),
            enregistre_le: String::new(),
        }
    }

    #[test]
    fn consultation_lue_depuis_le_pont() {
        let v = serde_json::json!({
            "notes": [{
                "id": "travail.f1", "version": 2, "texte": "Lot 8", "importance": 5, "confiance": 9,
                "provenance": "utilisateur", "auteur": "utilisateur", "liens": [], "entites": [],
                "sources": [], "cree_le": "2026-10-26T10:00:00Z", "enregistre_le": "2026-10-26T11:00:00Z",
                "strate": "N3", "domaine": "travail"
            }],
            "candidats": [{
                "id": "8d4f7c1e-0000-4000-8000-000000000001", "cible": null, "branche": "sante.sommeil",
                "type_unite": "fait", "texte": "Dort 6 h", "importance": 6, "confiance": 5,
                "provenance": "utilisateur", "liens": [], "sources": [], "sensible": true,
                "propose_le": "2026-10-26T10:00:00Z"
            }]
        });
        let c: Consultation = serde_json::from_value(v).expect("consultation");
        assert_eq!(c.notes[0].strate, "N3");
        assert_eq!(c.notes[0].domaine(), "travail");
        assert!(c.candidats[0].sensible);
    }

    #[test]
    fn groupes_par_strate_puis_domaine_et_filtres() {
        let notes = vec![
            note("moi.r1", "N0"),
            note("travail.cog.f2", "N3"),
            note("famille.s1", "N1"),
            note("travail.cog.f1", "N3"),
            note("famille.lea.f1", "N3"),
            note("travail.cog.s1", "N2"),
        ];
        let g = grouper(&notes, None, None);
        assert_eq!(
            g.iter().map(|s| s.strate).collect::<Vec<_>>(),
            ["N0", "N1", "N2", "N3"]
        );
        let n3 = &g[3];
        assert_eq!(
            n3.domaines
                .iter()
                .map(|d| d.domaine.as_str())
                .collect::<Vec<_>>(),
            ["famille", "travail"]
        );
        assert_eq!(n3.domaines[1].notes[0].id, "travail.cog.f1", "trié");
        let g = grouper(&notes, Some("N3"), Some("travail"));
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].domaines.len(), 1);
        assert_eq!(g[0].domaines[0].notes.len(), 2);
        assert_eq!(grouper(&notes, Some("N1"), Some("travail")).len(), 0);
        assert_eq!(domaines(&notes), ["famille", "moi", "travail"]);
    }

    #[test]
    fn regles_reconnues_par_leur_suffixe() {
        let notes = vec![
            note("moi.r2", "N0"),
            note("moi.p1", "N0"),
            note("travail.regles.f1", "N3"),
            note("moi.r1", "N0"),
        ];
        assert_eq!(
            regles(&notes)
                .iter()
                .map(|n| n.id.as_str())
                .collect::<Vec<_>>(),
            ["moi.r1", "moi.r2"]
        );
        assert_eq!(lettre_type("travail.cog.f12"), Some('f'));
        assert_eq!(lettre_type("travail.cog"), None);
        assert_eq!(lettre_type("r"), None);
    }

    #[test]
    fn oubli_exige_le_chemin_exact_et_un_motif() {
        let notes = vec![
            note("travail.cog.f1", "N3"),
            note("travail.cog.f2", "N3"),
            note("travail.cogito.f1", "N3"),
        ];
        let mut c = ConfirmationOubli::new("travail.cog");
        assert!(!c.peut_confirmer());
        assert!(c.corps().is_none());
        c.saisie = "travail.co".into();
        c.motif = "plus utile".into();
        assert!(!c.peut_confirmer(), "saisie différente");
        c.saisie = "travail.cog".into();
        c.motif = "   ".into();
        assert!(!c.peut_confirmer(), "motif vide");
        c.motif = "plus utile".into();
        assert!(c.peut_confirmer());
        assert_eq!(
            c.corps(),
            Some(serde_json::json!({"chemin": "travail.cog", "motif": "plus utile"}))
        );
        let effet = c.effet(&notes);
        assert!(
            effet.contains("2 note(s)") && effet.contains("ne peut pas être annulée"),
            "{effet}"
        );
        assert!(!oubli_effectif("travail.cog", &notes));
        assert!(
            oubli_effectif("travail.cog", &notes[2..]),
            "travail.cogito n'est pas sous travail.cog"
        );
    }

    #[test]
    fn issues_et_approbation_tamr() {
        assert_eq!(
            interpreter_issue(200, &serde_json::json!({"ecrite": "travail.f1"})),
            Ok(Issue::Ecrite("travail.f1".into()))
        );
        assert_eq!(
            interpreter_issue(202, &serde_json::json!({"candidat": "k"})),
            Ok(Issue::Candidat("k".into()))
        );
        let e = "ab".repeat(32);
        let v = serde_json::json!({"approbation": {
            "intervention_id": "iv", "type_intervention": "APPROVAL", "objet": "memoire.valider_candidat",
            "empreinte": e, "description": "Valider « Dort 6 h »", "expire_dans_s": 300
        }});
        let Ok(Issue::Approbation(d)) = interpreter_issue(202, &v) else {
            panic!("approbation attendue")
        };
        assert_eq!(d.empreinte, e, "empreinte exacte conservée");
        assert_eq!(empreinte_affichable(&d.empreinte).split(' ').count(), 8);
        assert_eq!(empreinte_affichable(&d.empreinte).replace(' ', ""), e);
        let mauvaise = serde_json::json!({"approbation": {"intervention_id": "iv", "empreinte": "XYZ", "description": ""}});
        assert!(interpreter_issue(202, &mauvaise).is_err());
        let refus = serde_json::json!({"error": "refus StrongFather", "tamr": "refus"});
        assert_eq!(
            interpreter_issue(403, &refus),
            Err("Approbation refusée : refus StrongFather".into())
        );
    }

    #[test]
    fn messages_de_refus() {
        for (motif, attendu) in [
            ("visa_refuse", "expiré"),
            ("portee_insuffisante", "portée"),
            ("autre_utilisateur", "titulaire"),
            ("appareil_approbation_requis", "appareil d'approbation"),
        ] {
            assert!(
                message_refus(403, &serde_json::json!({"reason": motif})).contains(attendu),
                "{motif}"
            );
        }
        assert_eq!(
            message_refus(500, &serde_json::json!({})),
            "Refus du pont (500)."
        );
        assert!(
            message_refus(404, &serde_json::json!({"error": "note inconnue"}))
                .contains("note inconnue")
        );
    }

    #[test]
    fn export_controle() {
        let v = serde_json::json!({"format": "maia-memoire/v1", "etiquette": "nationale", "notes": [1, 2],
            "historique": [1, 2, 3], "candidats": [], "pierres_tombales": [1]});
        assert_eq!(
            verifier_export(&v),
            Ok(ResumeExport {
                notes: 2,
                versions: 3,
                candidats: 0,
                pierres_tombales: 1
            })
        );
        assert!(
            verifier_export(&serde_json::json!({"format": "autre", "etiquette": "nationale"}))
                .is_err()
        );
        assert!(verifier_export(&serde_json::json!({"format": "maia-memoire/v1"})).is_err());
        assert_eq!(
            nom_fichier_export("alice", "2026-10-26"),
            "maia-memoire-alice-2026-10-26.json"
        );
    }

    #[test]
    fn corps_des_ecritures() {
        assert_eq!(
            corps_correction("travail.f1", " Lot 8 ", None),
            Ok(serde_json::json!({"id": "travail.f1", "texte": "Lot 8"}))
        );
        assert_eq!(
            corps_correction("travail.f1", "x", Some("7")),
            Ok(serde_json::json!({"id": "travail.f1", "texte": "x", "importance": 7}))
        );
        assert!(corps_correction("travail.f1", "  ", None).is_err());
        assert!(corps_correction("travail.f1", "x", Some("10")).is_err());
        assert_eq!(
            corps_regle("moi", "Tutoie-moi", "9"),
            Ok(serde_json::json!({"branche": "moi", "texte": "Tutoie-moi", "importance": 9}))
        );
        assert!(corps_regle("Moi", "x", "9").is_err());
        assert!(corps_regle("moi..x", "x", "9").is_err());
        assert!(corps_regle("moi", "", "9").is_err());
        assert!(corps_regle("moi", "x", "neuf").is_err());
    }

    #[test]
    fn libelles() {
        assert!(MENTION_IA.contains("intelligence artificielle"));
        assert_eq!(libelle_provenance("deduit"), "Déduit par l'IA");
        assert_eq!(libelle_strate("N0"), "N0 · Noyau (toujours chargé)");
        assert_eq!(libelle_auteur("maia"), "l'IA (MAIA)");
    }
}
