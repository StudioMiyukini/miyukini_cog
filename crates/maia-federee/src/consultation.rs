//! @id     maia_federee_consultation
//! @do     orchestrer_double_maia_consultation
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « consultation » (double MAIA : Planificateur sans contenu étranger, Lecteur sans outil, réponse sourcée déterministe marquée etranger).

//! Mode consultation et double MAIA (IA-FEDEREE §3, EX-59, EX-60).
//!
//! 1. le **Planificateur** (détient les outils) ne reçoit que la question de
//!    l'utilisateur et rend un [`Plan`] typé : hors fédération, ou
//!    [`RequeteTypee`] — il ne voit **jamais** une Fiche ni un extrait ;
//! 2. la base étrangère sélectionne les Fiches (émetteurs re-qualifiés) ;
//! 3. le **Lecteur** (sans outil) lit chaque Fiche et rend des extraits
//!    typés, ancrés ([`crate::extraire`]) ;
//! 4. corroboration et plafond par source ([`crate::corroborer`]) ;
//! 5. la réponse est **rendue par du code** (aucun modèle) : chaque
//!    affirmation cite la Fiche, sa version et le COG émetteur, et son degré
//!    (établi / source unique) ; le tout est un [`Etranger`] : **sans
//!    outil**, pire cas = une mauvaise réponse.

use std::fmt::Write as _;

use crate::base::{BaseEtrangere, Qualite, VueFederale};
use crate::corroboration::{corroborer, Affirmation, Etablissement, PolitiqueCorroboration, Source};
use crate::lecteur::{extraire, ModeleLecteur};
use crate::marquage::{Etranger, Provenance};
use crate::requete::RequeteTypee;

/// Plan typé rendu par le Planificateur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// La question ne relève pas des connaissances fédérées.
    HorsFederation,
    /// Consulter la base étrangère avec cette requête typée.
    ConsulterFedere(RequeteTypee),
}

/// MAIA-Planificateur : ne reçoit **que** la question de l'utilisateur.
pub trait Planificateur {
    /// Plan pour la question.
    fn planifier(&self, question: &str) -> Plan;
}

/// Réponse sourcée sur des connaissances fédérées (contenu `etranger`).
#[derive(Debug, Clone)]
pub struct ReponseSourcee {
    sujet: String,
    affirmations: Etranger<Vec<Affirmation>>,
}

fn court(cog: &str) -> &str {
    cog.get(..10).unwrap_or(cog)
}

impl ReponseSourcee {
    /// Affirmations corroborées (marquées `etranger`).
    pub fn affirmations(&self) -> &Etranger<Vec<Affirmation>> {
        &self.affirmations
    }

    /// Provenance (toujours `etranger`).
    pub const fn provenance(&self) -> Provenance {
        Provenance::Etranger
    }

    /// Vrai si aucune affirmation n'a été retenue.
    pub fn est_vide(&self) -> bool {
        self.affirmations.valeur_non_fiable().is_empty()
    }

    /// Rendu déterministe pour l'utilisateur (reste `etranger`).
    pub fn texte(&self) -> Etranger<String> {
        let sujet = self.sujet.clone();
        self.affirmations.clone().map(|affirmations| {
            if affirmations.is_empty() {
                return format!("Aucune connaissance fédérée pertinente sur « {sujet} ».");
            }
            let mut t = format!(
                "Connaissances fédérées sur « {sujet} » (contenu étranger, non fiable, sans action possible) :\n"
            );
            for a in &affirmations {
                let degre = match a.etablissement {
                    Etablissement::Etabli { confirmateurs } => format!("établi ({confirmateurs} COG membres)"),
                    Etablissement::SourceUnique => "source unique".to_string(),
                };
                let sources: Vec<String> = a
                    .sources
                    .iter()
                    .map(|s: &Source| {
                        let q = if s.qualite == Qualite::Membre { "membre" } else { "invité" };
                        format!("fiche {} v{} (COG {}…, {q})", s.fiche, s.version, court(s.cog.as_str()))
                    })
                    .collect();
                let _ = writeln!(
                    t,
                    "- {} — {} — {} — {degre} — sources : {}",
                    a.relation.sujet,
                    a.relation.predicat,
                    a.relation.objet,
                    sources.join(", ")
                );
            }
            t
        })
    }
}

/// Issue d'une consultation.
#[derive(Debug, Clone)]
pub enum Consultation {
    /// Le Planificateur n'a pas demandé de consultation fédérée.
    HorsFederation,
    /// Réponse sourcée (contenu `etranger`).
    Reponse(ReponseSourcee),
}

/// Consultation fédérée « double MAIA ».
pub fn consulter(
    question: &str,
    planificateur: &dyn Planificateur,
    lecteur: &dyn ModeleLecteur,
    base: &BaseEtrangere,
    vue: &dyn VueFederale,
    politique: &PolitiqueCorroboration,
    maintenant: u64,
) -> Consultation {
    let Plan::ConsulterFedere(requete) = planificateur.planifier(question) else {
        return Consultation::HorsFederation;
    };
    let mut extraits = Vec::new();
    for (fiche, qualite) in base.rechercher(&requete, vue, maintenant) {
        for extrait in extraire(lecteur, &requete, fiche) {
            let source = Source {
                cog: fiche.cog().clone(),
                fiche: fiche.fiche().id.clone(),
                version: fiche.fiche().version,
                empreinte: fiche.empreinte(),
                qualite,
                ancrage: extrait.ancrage,
                reprise: fiche.est_reprise(),
            };
            extraits.push((source, extrait));
        }
    }
    Consultation::Reponse(ReponseSourcee {
        sujet: requete.sujet().to_string(),
        affirmations: Etranger::marquer(corroborer(extraits, politique)),
    })
}
