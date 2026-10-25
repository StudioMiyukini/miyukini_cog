//! @id     maia_memoire_pont_alicia
//! @do     convertir_memoire_alicia
//! @role   orchestration
//! @layer  8
//! @human  maia-memoire — conversion des types de miyualicia-memory (Fact, Preference, LearnedSkill, Episode) en unités MAIA (§11, mémoire commune).

//! Provenance (EX-53) déduite du champ libre `source` d'Alicia, sans confiance
//! par défaut : `user`/`utilisateur` → propos de l'utilisateur ; `inferred`,
//! `deduit`, `alicia`, `service:*` → déduction ; **tout le reste → `etranger`**
//! (refus par défaut : validation requise). Les préférences d'Alicia n'ont pas
//! de source : déductions. Seules les compétences **acceptées** deviennent des
//! procédures (acceptation authentifiée côté miyualicia-api).

use miyualicia_memory::{Episode, EpisodeOutcome, Fact, LearnedSkill, LearnedSkillStatus, Preference};

use crate::erreurs::ErreurMemoire;
use crate::ident::{Branche, TypeUnite};
use crate::memoire::{Ecrit, Proposition};
use crate::types::Provenance;

/// Unité importée : écriture de l'utilisateur ou proposition à valider.
#[derive(Debug, Clone, PartialEq)]
pub enum Import {
    /// Propos de l'utilisateur.
    Ecrit(Ecrit),
    /// Déduction ou contenu `etranger` : candidat.
    Proposition(Proposition),
}

/// Provenance d'une source Alicia.
pub fn provenance_source(source: &str) -> Provenance {
    match source.trim() {
        "user" | "utilisateur" => Provenance::Utilisateur,
        "inferred" | "deduit" | "alicia" => Provenance::Deduit,
        s if s.starts_with("service:") => Provenance::Deduit,
        _ => Provenance::Etranger,
    }
}

/// Segment d'identifiant à partir d'un libellé libre.
fn segment(libre: &str, defaut: &str) -> String {
    let mut s: String = libre
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_lowercase() || c.is_ascii_digit() { c } else { '_' })
        .take(30)
        .collect();
    if !s.starts_with(|c: char| c.is_ascii_lowercase()) {
        s = format!("{defaut}_{s}");
    }
    s.truncate(31);
    // Jamais la forme d'un segment typé (`f12`).
    if Branche::parse(&s).is_err() {
        s.push('_');
    }
    s
}

fn branche(domaine: &str, sujet: &str) -> Result<Branche, ErreurMemoire> {
    Branche::parse(&format!("{}.{}", segment(domaine, "alicia"), segment(sujet, "sujet")))
}

fn import(provenance: Provenance, branche: Branche, t: TypeUnite, texte: String, importance: u8, confiance: u8) -> Import {
    match provenance {
        Provenance::Utilisateur => Import::Ecrit(Ecrit::new(branche, t, texte, importance)),
        p => Import::Proposition(Proposition {
            cible: None,
            branche,
            type_unite: t,
            texte,
            importance,
            confiance,
            provenance: p,
            liens: Vec::new(),
            sources: Vec::new(),
        }),
    }
}

/// Fait Alicia → Fait MAIA (`persona.sujet.fN`).
pub fn depuis_fait(f: &Fact) -> Result<Import, ErreurMemoire> {
    let confiance = (f.confidence.clamp(0.0, 1.0) * 9.0).round() as u8;
    Ok(import(
        provenance_source(&f.source),
        branche(&f.persona, &f.subject)?,
        TypeUnite::Fait,
        f.content.clone(),
        5,
        confiance,
    ))
}

/// Préférence Alicia → Préférence MAIA (déduction : pas de source).
pub fn depuis_preference(p: &Preference) -> Result<Import, ErreurMemoire> {
    Ok(import(
        Provenance::Deduit,
        branche(&p.scope, "preferences")?,
        TypeUnite::Preference,
        format!("{} = {}", p.key, p.value),
        5,
        6,
    ))
}

/// Compétence acceptée → Procédure ; proposée ou refusée : `None`.
pub fn depuis_competence(k: &LearnedSkill) -> Result<Option<Import>, ErreurMemoire> {
    if k.status != LearnedSkillStatus::Accepted {
        return Ok(None);
    }
    Ok(Some(import(
        Provenance::Utilisateur,
        branche(&k.persona, "procedures")?,
        TypeUnite::Procedure,
        format!("« {} » → {}", k.trigger_phrase.trim(), k.qualified_id),
        5,
        9,
    )))
}

/// Épisode Alicia → Épisode MAIA (déduction).
pub fn depuis_episode(e: &Episode) -> Result<Import, ErreurMemoire> {
    let issue = match e.outcome {
        EpisodeOutcome::Ok => "ok",
        EpisodeOutcome::Cancelled => "annulé",
        EpisodeOutcome::Failed => "échec",
    };
    Ok(import(
        Provenance::Deduit,
        branche(&e.persona, &e.service_id)?,
        TypeUnite::Episode,
        format!("{}.{} ({issue})", e.service_id, e.intent),
        3,
        6,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_refus_par_defaut() {
        assert_eq!(provenance_source("user"), Provenance::Utilisateur);
        assert_eq!(provenance_source("service:jaykoa"), Provenance::Deduit);
        assert_eq!(provenance_source("fiche:federation"), Provenance::Etranger);
        assert_eq!(provenance_source(""), Provenance::Etranger);
    }

    #[test]
    fn conversions() {
        let f = Fact::new("work".into(), "cat:mochi".into(), "Le chat s'appelle Mochi".into(), "user".into());
        match depuis_fait(&f).expect("fait") {
            Import::Ecrit(e) => assert_eq!(e.branche.as_str(), "work.cat_mochi"),
            Import::Proposition(_) => panic!("propos de l'utilisateur attendu"),
        }
        let f = Fact::new("1x".into(), "f1".into(), "x".into(), "web".into());
        match depuis_fait(&f).expect("fait") {
            Import::Proposition(p) => {
                assert_eq!(p.provenance, Provenance::Etranger);
                assert_eq!(p.branche.as_str(), "alicia_1x.f1_");
            }
            Import::Ecrit(_) => panic!("etranger attendu"),
        }
        let k = LearnedSkill::proposed("voice".into(), "café".into(), "jaykonta.add".into(), serde_json::json!({}));
        assert!(depuis_competence(&k).expect("k").is_none());
    }
}
