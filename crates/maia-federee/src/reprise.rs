//! @id     maia_federee_reprise
//! @do     deriver_fiche_republiee_des_sources
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « reprise » : règle de dérivation stricte d'une republication (A-009) ;
//! @human  le contenu, l'identifiant et les `reprises` sont calculés depuis les Fiches étrangères
//! @human  vérifiées, jamais repris d'une proposition (MAIA propose, le serveur décide).

//! Republication d'une donnée étrangère (A-009) : **règle de dérivation**.
//!
//! Une Fiche republiée ne porte **que** le contenu de ses sources (Fiches
//! étrangères vérifiées). [`deriver_reprise`] la calcule ;
//! [`crate::ServiceRequete`] la recalcule à chaque publication et refuse tout
//! écart ([`ErreurFederee::NonDerivee`]). Seuls deux choix restent à
//! l'utilisateur : le **sujet** (parmi ceux des sources, ou dérivable de leur
//! texte) et une **audience** au plus aussi large que chaque source.
//!
//! | champ | valeur |
//! |---|---|
//! | `id` | [`PREFIXE_ID_REPRISE`] + 32 hex de [`empreinte_ensemble`] (ensemble des `(cog_id, id)` sources, trié) |
//! | `emetteur` | identité du COG local |
//! | `sujet` | choisi : sujet d'une source, ou clé de comparaison contenue dans le texte ; défaut : sujet de la 1ʳᵉ source |
//! | `type`, `langue`, `licence` | **identiques** pour toutes les sources (sinon refus) et repris tels quels |
//! | `texte` | textes des sources, dans l'ordre donné, séparés par une ligne vide ; ≤ 8 000 car. (refus, jamais tronqué) |
//! | `relations` | union des relations des sources, sans doublon, ordre de première apparition ; ≤ 32 |
//! | `sources` | union des sources citées, sans doublon ; ≤ 16 |
//! | `audience` | choisie, incluse dans l'audience de **chaque** source ; défaut : celle de la 1ʳᵉ source |
//! | `reprises` | `Reprise::de(s)` pour chaque source (1 à 16, sans doublon) |
//! | `version`, `date` | fixées par le service de publication |
//!
//! Refus : aucune source ou plus de 16 ; source en double ; source déjà
//! republiée (une republication ne se republie pas) ; source du COG local ;
//! types, langues ou licences différents.

use sha2::{Digest, Sha256};

use crate::erreurs::ErreurFederee;
use crate::fiche::{reprises_distinctes, Audience, Fiche, FicheVerifiee, Relation, Reprise, MAX_RELATIONS, MAX_REPRISES, MAX_SOURCES, MAX_TEXTE};
use crate::normalisation::{cle_comparaison, normaliser_ligne};

/// Préfixe de l'identifiant d'une Fiche republiée (≠ `km.` des originales).
pub const PREFIXE_ID_REPRISE: &str = "rp.";
/// Séparation de domaine de l'empreinte d'un ensemble de sources.
pub const DOMAINE_ENSEMBLE: &[u8] = b"miyukini/maia-federee/reprise/ensemble/v1\0";

/// Empreinte de l'**ensemble** des Fiches sources (ordre indifférent) :
/// `SHA-256(domaine || Σ trié (cog_id || 0x00 || id || 0x00))`.
pub fn empreinte_ensemble(sources: &[FicheVerifiee]) -> [u8; 32] {
    let mut refs: Vec<(&str, &str)> = sources.iter().map(|s| (s.cog().as_str(), s.fiche().id.as_str())).collect();
    refs.sort_unstable();
    refs.dedup();
    let mut h = Sha256::new();
    h.update(DOMAINE_ENSEMBLE);
    for (cog, id) in refs {
        h.update(cog.as_bytes());
        h.update([0]);
        h.update(id.as_bytes());
        h.update([0]);
    }
    h.finalize().into()
}

/// Identifiant imposé de la Fiche qui republie l'ensemble d'empreinte `ensemble`.
pub fn id_fiche_reprise(ensemble: &[u8; 32]) -> String {
    use std::fmt::Write as _;
    let mut id = String::from(PREFIXE_ID_REPRISE);
    for b in &ensemble[..16] {
        let _ = write!(id, "{b:02x}");
    }
    id
}

/// Choix de l'utilisateur pour une republication (tout le reste est calculé).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChoixReprise {
    /// Sujet ; `None` : sujet de la première source.
    pub sujet: Option<String>,
    /// Audience ; `None` : audience de la première source.
    pub audience: Option<Audience>,
}

fn sans_doublon<T: PartialEq + Clone>(listes: impl Iterator<Item = T>) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    for x in listes {
        if !v.contains(&x) {
            v.push(x);
        }
    }
    v
}

/// Fiche republiant `sources` (vérifiées, dans l'ordre voulu pour le texte)
/// pour l'émetteur `emetteur` (identité du COG local) ; voir la règle dans
/// l'en-tête du module.
pub fn deriver_reprise(
    sources: &[FicheVerifiee],
    emetteur: [u8; 32],
    choix: &ChoixReprise,
    version: u64,
    date: u64,
) -> Result<Fiche, ErreurFederee> {
    let (Some(premiere), true) = (sources.first(), sources.len() <= MAX_REPRISES) else {
        return Err(ErreurFederee::ChampInvalide("reprises : 1 à 16 sources"));
    };
    let reprises: Vec<Reprise> = sources.iter().map(Reprise::de).collect();
    if !reprises_distinctes(&reprises) {
        return Err(ErreurFederee::ChampInvalide("reprises"));
    }
    let local = crate::fiche::cog_de_cle(&emetteur).ok_or(ErreurFederee::EmetteurIncoherent)?;
    let p = premiere.fiche();
    for s in sources {
        let f = s.fiche();
        if s.cog() == &local {
            return Err(ErreurFederee::ChampInvalide("reprises : source de son propre COG"));
        }
        if s.est_reprise() {
            return Err(ErreurFederee::ChampInvalide("reprises : source déjà republiée"));
        }
        if f.licence != p.licence {
            return Err(ErreurFederee::ChampInvalide("licence différente de la source"));
        }
        if f.type_fiche != p.type_fiche || f.langue != p.langue {
            return Err(ErreurFederee::ChampInvalide("sources de types ou de langues différents"));
        }
    }
    let audience = choix.audience.clone().unwrap_or_else(|| p.audience.clone());
    if !sources.iter().all(|s| audience.incluse_dans(&s.fiche().audience)) {
        return Err(ErreurFederee::ChampInvalide("audience plus large que la source"));
    }
    let texte = sources.iter().map(|s| s.fiche().texte.as_str()).collect::<Vec<_>>().join("\n\n");
    if texte.chars().count() > MAX_TEXTE {
        return Err(ErreurFederee::ChampInvalide("texte des sources trop long"));
    }
    let sujet = match &choix.sujet {
        None => p.sujet.clone(),
        Some(brut) => {
            let sujet = normaliser_ligne(brut);
            let c = cle_comparaison(&sujet);
            let d_une_source = sources.iter().any(|s| s.fiche().sujet == sujet);
            if sujet.is_empty() || !(d_une_source || (!c.is_empty() && cle_comparaison(&texte).contains(&c))) {
                return Err(ErreurFederee::NonDerivee("sujet absent des sources"));
            }
            sujet
        }
    };
    let relations: Vec<Relation> = sans_doublon(sources.iter().flat_map(|s| s.fiche().relations.iter().cloned()));
    if relations.len() > MAX_RELATIONS {
        return Err(ErreurFederee::ChampInvalide("relations des sources trop nombreuses"));
    }
    let citees: Vec<String> = sans_doublon(sources.iter().flat_map(|s| s.fiche().sources.iter().cloned()));
    if citees.len() > MAX_SOURCES {
        return Err(ErreurFederee::ChampInvalide("sources citées trop nombreuses"));
    }
    Ok(Fiche {
        id: id_fiche_reprise(&empreinte_ensemble(sources)),
        emetteur,
        sujet,
        type_fiche: p.type_fiche,
        texte,
        relations,
        sources: citees,
        date,
        version,
        langue: p.langue.clone(),
        licence: p.licence.clone(),
        audience,
        reprises,
    })
}

/// Vrai si `fiche` (vérifiée, normalisée) est exactement la dérivation de
/// `sources` pour ses propres choix (sujet, audience), version et date.
pub fn verifier_derivation_reprise(fiche: &FicheVerifiee, sources: &[FicheVerifiee]) -> Result<(), ErreurFederee> {
    let f = fiche.fiche();
    let choix = ChoixReprise { sujet: Some(f.sujet.clone()), audience: Some(f.audience.clone()) };
    let attendue = deriver_reprise(sources, f.emetteur, &choix, f.version, f.date)?;
    // `reprises` : même ensemble (l'ordre des sources fixe celui du texte).
    let memes_reprises = attendue.reprises.len() == f.reprises.len() && attendue.reprises.iter().all(|r| f.reprises.contains(r));
    let sans_reprises = |x: &Fiche| Fiche { reprises: Vec::new(), ..x.clone() };
    if !memes_reprises || sans_reprises(&attendue) != sans_reprises(f) {
        return Err(ErreurFederee::NonDerivee("Fiche republiée non dérivée de ses sources"));
    }
    if !fiche.canonique() {
        return Err(ErreurFederee::NonDerivee("champs signés non normalisés"));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;
    use crate::fiche::essais::fiche;
    use crate::fiche::{sceller_fiche, verifier_fiche};
    use miyusceau::CleSignature;

    const T: u64 = 1_800_000_100;

    fn source(cle: &CleSignature, id: &str, texte: &str) -> FicheVerifiee {
        verifier_fiche(&sceller_fiche(&fiche(cle, id, texte, &[("Paris", "capitale de", "France")]), cle).unwrap(), T).unwrap()
    }

    #[test]
    fn derivation_deterministe_et_identifiant_d_ensemble() {
        let a = CleSignature::generer();
        let b = CleSignature::generer();
        let local = CleSignature::generer();
        let s1 = source(&a, "geo.paris", "Paris est la capitale.");
        let s2 = source(&b, "geo.capitale", "La capitale est Paris.");
        let f = deriver_reprise(&[s1.clone(), s2.clone()], local.cle_publique(), &ChoixReprise::default(), 1, T).unwrap();
        assert!(f.id.starts_with(PREFIXE_ID_REPRISE) && f.id.len() == 35);
        assert_eq!(f.texte, "Paris est la capitale.\n\nLa capitale est Paris.");
        assert_eq!(f.relations.len(), 1, "relations sans doublon");
        assert_eq!(f.reprises, vec![Reprise::de(&s1), Reprise::de(&s2)]);
        // Ordre indifférent pour l'identifiant.
        let g = deriver_reprise(&[s2.clone(), s1.clone()], local.cle_publique(), &ChoixReprise::default(), 1, T).unwrap();
        assert_eq!(f.id, g.id);
        // La Fiche scellée se revérifie comme dérivée de ses sources.
        let v = verifier_fiche(&sceller_fiche(&f, &local).unwrap(), T).unwrap();
        assert!(verifier_derivation_reprise(&v, &[s1.clone(), s2.clone()]).is_ok());
        // Texte modifié : non dérivée.
        let h = Fiche { texte: format!("{} Code 1234.", f.texte), ..f.clone() };
        let v = verifier_fiche(&sceller_fiche(&h, &local).unwrap(), T).unwrap();
        assert!(matches!(verifier_derivation_reprise(&v, &[s1, s2]), Err(ErreurFederee::NonDerivee(_))));
    }

    #[test]
    fn refus_licence_audience_source_locale_reprise_sujet() {
        let a = CleSignature::generer();
        let local = CleSignature::generer();
        let s1 = source(&a, "geo.paris", "Paris est la capitale.");
        let mit = verifier_fiche(&sceller_fiche(&Fiche { licence: "MIT".into(), ..fiche(&a, "x", "Lyon.", &[]) }, &a).unwrap(), T).unwrap();
        let e = local.cle_publique();
        assert!(deriver_reprise(&[], e, &ChoixReprise::default(), 1, T).is_err());
        assert!(deriver_reprise(&[s1.clone(), mit], e, &ChoixReprise::default(), 1, T).is_err());
        let elargie = ChoixReprise { audience: Some(Audience::Invites), ..ChoixReprise::default() };
        assert_eq!(
            deriver_reprise(std::slice::from_ref(&s1), e, &elargie, 1, T),
            Err(ErreurFederee::ChampInvalide("audience plus large que la source"))
        );
        let propre = source(&local, "geo.local", "Paris.");
        assert!(deriver_reprise(&[propre], e, &ChoixReprise::default(), 1, T).is_err());
        assert!(deriver_reprise(&[s1.clone(), s1.clone()], e, &ChoixReprise::default(), 1, T).is_err());
        let sujet = ChoixReprise { sujet: Some("code d'accès 1234".into()), ..ChoixReprise::default() };
        assert!(matches!(deriver_reprise(std::slice::from_ref(&s1), e, &sujet, 1, T), Err(ErreurFederee::NonDerivee(_))));
        let sujet = ChoixReprise { sujet: Some("la capitale".into()), ..ChoixReprise::default() };
        assert_eq!(deriver_reprise(std::slice::from_ref(&s1), e, &sujet, 1, T).unwrap().sujet, "la capitale");
        // Une republication ne se republie pas.
        let r = deriver_reprise(std::slice::from_ref(&s1), a.cle_publique(), &ChoixReprise::default(), 1, T);
        assert!(r.is_err(), "source de son propre COG");
        let b = CleSignature::generer();
        let rep = deriver_reprise(std::slice::from_ref(&s1), b.cle_publique(), &ChoixReprise::default(), 1, T).unwrap();
        let rep = verifier_fiche(&sceller_fiche(&rep, &b).unwrap(), T).unwrap();
        assert_eq!(
            deriver_reprise(&[rep], e, &ChoixReprise::default(), 1, T),
            Err(ErreurFederee::ChampInvalide("reprises : source déjà republiée"))
        );
    }
}
