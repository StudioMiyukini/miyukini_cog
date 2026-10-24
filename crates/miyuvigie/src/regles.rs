//! @id     miyuvigie_regles
//! @do     evaluer_regles_detection
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « regles » : règles de détection pilotées par les données.

//! Règles de détection des Vigies.
//!
//! # Règles intégrées (non désactivables)
//!
//! Appliquées à tout événement de flux, avant les règles du jeu chargé :
//!
//! | id | condition | sévérité | catégorie |
//! |---|---|---|---|
//! | `R-EX12-SORTIE` | `zone_src = cog` et `zone_dst ∉ {cog, portail}` (quel que soit le verdict) | critique | `sortie_zone_cog` |
//! | `R-INV-Z7-ENTREE` | `zone_dst = cog` et `zone_src ∉ {cog, portail}` | critique | `mouvement_lateral` |
//! | `R-INDETERMINE` | zone `inconnue` ou verdict `inconnu` | moyenne | `donnee_indeterminee` |
//!
//! # Format d'un jeu de règles
//!
//! ```text
//! # commentaire
//! vigies-regles 1
//! regle R-FRANCHE-EXT si zone_src=franche et zone_dst=ext et verdict=autorise et octets>1048576 alors elevee flux_hors_contrat
//! ```
//!
//! Conditions : `zone_src=<zone>`, `zone_dst=<zone>`, `verdict=<autorise|bloque>`,
//! `voie=<id|->`, `port_dst=<n>`, `octets><n>`. Analyse **stricte** : la première
//! ligne utile doit être l'en-tête de version, toute ligne invalide ou id en
//! double rejette **tout** le jeu (les règles intégrées restent actives).
//!
//! Jeux signés : [`JeuRegles::charger_signe`] délègue la vérification à un
//! [`VerificateurJeuRegles`] (ex. Sceau v1, `miyusceau`) ; le câblage de la
//! clé d'émission est hors de ce crate.

use std::collections::HashSet;
use std::fmt;

use crate::evenement::{EvenementFlux, Verdict};
use crate::signal::{Categorie, RegleId, Severite, VoieId};
use crate::zone::Zone;

/// En-tête de version obligatoire.
pub const ENTETE: &str = "vigies-regles 1";

/// Identifiant de la règle intégrée EX-12.
pub const REGLE_SORTIE: &str = "R-EX12-SORTIE";
/// Identifiant de la règle intégrée INV-Z7 (entrée directe dans la zone COG).
pub const REGLE_ENTREE: &str = "R-INV-Z7-ENTREE";
/// Identifiant de la règle intégrée « donnée indéterminée ».
pub const REGLE_INDETERMINE: &str = "R-INDETERMINE";

/// Condition élémentaire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// Zone source égale.
    ZoneSrc(Zone),
    /// Zone destination égale.
    ZoneDst(Zone),
    /// Verdict égal.
    Verdict(Verdict),
    /// Voie égale (`None` : flux sans voie).
    Voie(Option<VoieId>),
    /// Port destination égal.
    PortDst(u16),
    /// Volume strictement supérieur.
    OctetsSup(u64),
}

impl Condition {
    fn verifie(&self, e: &EvenementFlux) -> bool {
        match self {
            Condition::ZoneSrc(z) => e.zone_src == *z,
            Condition::ZoneDst(z) => e.zone_dst == *z,
            Condition::Verdict(v) => e.verdict == *v,
            Condition::Voie(v) => e.voie == *v,
            Condition::PortDst(p) => e.port_dst == Some(*p),
            Condition::OctetsSup(n) => e.octets > *n,
        }
    }
}

/// Règle : conjonction de conditions → sévérité, catégorie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Regle {
    /// Identifiant unique dans le jeu.
    pub id: RegleId,
    /// Conditions (toutes requises ; au moins une).
    pub conditions: Vec<Condition>,
    /// Sévérité produite.
    pub severite: Severite,
    /// Catégorie produite.
    pub categorie: Categorie,
}

/// Déclenchement d'une règle sur un événement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declenchement {
    /// Règle déclenchée.
    pub regle: RegleId,
    /// Sévérité.
    pub severite: Severite,
    /// Catégorie.
    pub categorie: Categorie,
    /// Explication lisible.
    pub explication: String,
}

/// Erreur de chargement d'un jeu de règles (le jeu entier est refusé).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurRegles {
    /// Numéro de ligne (1-based), 0 si global.
    pub ligne: usize,
    /// Motif.
    pub motif: &'static str,
}

impl fmt::Display for ErreurRegles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "jeu de règles refusé (ligne {}) : {}",
            self.ligne, self.motif
        )
    }
}

impl std::error::Error for ErreurRegles {}

/// Vérification de signature d'un jeu de règles (Sceau v1 ou autre).
pub trait VerificateurJeuRegles {
    /// Vrai si `signature` authentifie `contenu`.
    fn verifier(&self, contenu: &[u8], signature: &[u8]) -> bool;
}

/// Jeu de règles chargé (les règles intégrées s'y ajoutent toujours).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JeuRegles {
    regles: Vec<Regle>,
}

impl JeuRegles {
    /// Jeu vide : seules les règles intégrées s'appliquent.
    pub fn vide() -> Self {
        Self::default()
    }

    /// Règles chargées (hors règles intégrées).
    pub fn regles(&self) -> &[Regle] {
        &self.regles
    }

    /// Charge un jeu signé ; signature invalide = refus.
    pub fn charger_signe(
        texte: &str,
        signature: &[u8],
        verificateur: &dyn VerificateurJeuRegles,
    ) -> Result<Self, ErreurRegles> {
        if !verificateur.verifier(texte.as_bytes(), signature) {
            return Err(ErreurRegles {
                ligne: 0,
                motif: "signature du jeu de règles invalide",
            });
        }
        Self::charger(texte)
    }

    /// Charge un jeu depuis son texte (analyse stricte).
    pub fn charger(texte: &str) -> Result<Self, ErreurRegles> {
        let mut regles = Vec::new();
        let mut ids = HashSet::new();
        let mut entete_vue = false;
        for (i, brute) in texte.lines().enumerate() {
            let n = i + 1;
            let l = brute.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            if !entete_vue {
                if l != ENTETE {
                    return Err(ErreurRegles {
                        ligne: n,
                        motif: "en-tête « vigies-regles 1 » attendu",
                    });
                }
                entete_vue = true;
                continue;
            }
            let regle = analyser_regle(l).map_err(|motif| ErreurRegles { ligne: n, motif })?;
            let id = regle.id.as_str();
            if [REGLE_SORTIE, REGLE_ENTREE, REGLE_INDETERMINE].contains(&id) {
                return Err(ErreurRegles {
                    ligne: n,
                    motif: "identifiant réservé aux règles intégrées",
                });
            }
            if !ids.insert(regle.id.clone()) {
                return Err(ErreurRegles {
                    ligne: n,
                    motif: "identifiant de règle en double",
                });
            }
            regles.push(regle);
        }
        if !entete_vue {
            return Err(ErreurRegles {
                ligne: 0,
                motif: "jeu vide (en-tête absent)",
            });
        }
        Ok(Self { regles })
    }

    /// Évalue un événement : règles intégrées puis règles du jeu.
    pub fn evaluer(&self, e: &EvenementFlux) -> Vec<Declenchement> {
        let mut d = Vec::new();
        if e.zone_src == Zone::Cog && !matches!(e.zone_dst, Zone::Cog | Zone::Portail) {
            d.push(integree(
                REGLE_SORTIE,
                Severite::Critique,
                Categorie::SortieZoneCog,
                format!(
                    "tentative de sortie de la zone COG vers « {} » (verdict Douane : {}) — EX-12",
                    e.zone_dst,
                    e.verdict.nom()
                ),
            ));
        }
        if e.zone_dst == Zone::Cog && !matches!(e.zone_src, Zone::Cog | Zone::Portail) {
            d.push(integree(
                REGLE_ENTREE,
                Severite::Critique,
                Categorie::MouvementLateral,
                format!(
                    "flux direct « {} » → zone COG hors Portail (verdict Douane : {}) — INV-Z7",
                    e.zone_src,
                    e.verdict.nom()
                ),
            ));
        }
        if e.zone_src == Zone::Inconnue
            || e.zone_dst == Zone::Inconnue
            || e.verdict == Verdict::Inconnu
        {
            d.push(integree(
                REGLE_INDETERMINE,
                Severite::Moyenne,
                Categorie::DonneeIndeterminee,
                "zone ou verdict indéterminé : non bénin par défaut".to_string(),
            ));
        }
        for r in &self.regles {
            if r.conditions.iter().all(|c| c.verifie(e)) {
                d.push(Declenchement {
                    regle: r.id.clone(),
                    severite: r.severite,
                    categorie: r.categorie,
                    explication: format!("règle {} déclenchée", r.id),
                });
            }
        }
        d
    }
}

fn integree(
    id: &str,
    severite: Severite,
    categorie: Categorie,
    explication: String,
) -> Declenchement {
    Declenchement {
        // Identifiants constants valides par construction.
        regle: RegleId::new(id).unwrap_or_else(|| unreachable!()),
        severite,
        categorie,
        explication,
    }
}

fn analyser_regle(l: &str) -> Result<Regle, &'static str> {
    let jetons: Vec<&str> = l.split_whitespace().collect();
    // regle ID si C [et C]* alors SEV CAT
    if jetons.len() < 7 || jetons[0] != "regle" || jetons[2] != "si" {
        return Err(
            "forme « regle <id> si <cond> [et <cond>]* alors <sévérité> <catégorie> » attendue",
        );
    }
    let id = RegleId::new(jetons[1]).ok_or("identifiant de règle invalide")?;
    let n = jetons.len();
    if jetons[n - 3] != "alors" {
        return Err("« alors <sévérité> <catégorie> » attendu en fin de règle");
    }
    let severite = Severite::depuis_nom(jetons[n - 2]).ok_or("sévérité inconnue")?;
    let categorie = Categorie::depuis_nom(jetons[n - 1]).ok_or("catégorie inconnue")?;
    let corps = &jetons[3..n - 3];
    if corps.is_empty() || corps.len().is_multiple_of(2) {
        return Err("conditions mal formées");
    }
    let mut conditions = Vec::new();
    for (k, j) in corps.iter().enumerate() {
        if k % 2 == 1 {
            if *j != "et" {
                return Err("conditions séparées par « et » attendues");
            }
            continue;
        }
        conditions.push(analyser_condition(j)?);
    }
    Ok(Regle {
        id,
        conditions,
        severite,
        categorie,
    })
}

fn analyser_condition(j: &str) -> Result<Condition, &'static str> {
    if let Some(n) = j.strip_prefix("octets>") {
        return n
            .parse()
            .map(Condition::OctetsSup)
            .map_err(|_| "octets : entier attendu");
    }
    let (cle, val) = j
        .split_once('=')
        .ok_or("condition « clé=valeur » attendue")?;
    match cle {
        "zone_src" => val
            .parse()
            .map(Condition::ZoneSrc)
            .map_err(|()| "zone inconnue"),
        "zone_dst" => val
            .parse()
            .map(Condition::ZoneDst)
            .map_err(|()| "zone inconnue"),
        "verdict" => Verdict::depuis_nom(val)
            .map(Condition::Verdict)
            .ok_or("verdict inconnu"),
        "voie" if val == "-" => Ok(Condition::Voie(None)),
        "voie" => VoieId::new(val)
            .map(|v| Condition::Voie(Some(v)))
            .ok_or("voie invalide"),
        "port_dst" => val
            .parse()
            .map(Condition::PortDst)
            .map_err(|_| "port invalide"),
        _ => Err("clé de condition inconnue"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::EvenementId;

    fn evt(src: Zone, dst: Zone, verdict: Verdict, octets: u64) -> EvenementFlux {
        EvenementFlux {
            id: EvenementId::new("e1").unwrap(),
            horodatage_ms: 1,
            zone_src: src,
            zone_dst: dst,
            voie: None,
            verdict,
            octets,
            ip_src: None,
            ip_dst: None,
            port_dst: Some(443),
            protocole: None,
        }
    }

    #[test]
    fn regle_integree_sortie_meme_bloquee() {
        let j = JeuRegles::vide();
        let d = j.evaluer(&evt(Zone::Cog, Zone::Exterieur, Verdict::Bloque, 0));
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].regle.as_str(), REGLE_SORTIE);
        assert_eq!(d[0].severite, Severite::Critique);
        // COG -> Portail : légitime.
        assert_eq!(
            j.evaluer(&evt(Zone::Cog, Zone::Portail, Verdict::Autorise, 0)),
            Vec::new()
        );
    }

    #[test]
    fn regle_integree_indeterminee() {
        let d = JeuRegles::vide().evaluer(&evt(Zone::Franche, Zone::Franche, Verdict::Inconnu, 0));
        assert_eq!(d[0].regle.as_str(), REGLE_INDETERMINE);
    }

    #[test]
    fn jeu_charge_et_evalue() {
        let j = JeuRegles::charger(
            "# essai\nvigies-regles 1\nregle R-GROS si zone_src=franche et zone_dst=ext et octets>1000 alors elevee flux_hors_contrat\n",
        )
        .unwrap();
        let d = j.evaluer(&evt(
            Zone::Franche,
            Zone::Exterieur,
            Verdict::Autorise,
            5000,
        ));
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].severite, Severite::Elevee);
        assert_eq!(
            j.evaluer(&evt(Zone::Franche, Zone::Exterieur, Verdict::Autorise, 10)),
            Vec::new()
        );
    }

    #[test]
    fn jeux_invalides_refuses() {
        for t in [
            "",
            "regle R si zone_src=cog alors critique sortie_zone_cog",
            "vigies-regles 2\n",
            "vigies-regles 1\nregle R si zone_src=lune alors critique sortie_zone_cog",
            "vigies-regles 1\nregle R si zone_src=cog et alors critique sortie_zone_cog",
            "vigies-regles 1\nregle R si zone_src=cog ou zone_dst=ext alors critique sortie_zone_cog",
            "vigies-regles 1\nregle R si zone_src=cog alors grave sortie_zone_cog",
            "vigies-regles 1\nregle R si zone_src=cog alors critique autre",
            "vigies-regles 1\nregle R si zone_src=cog alors critique sortie_zone_cog\nregle R si zone_dst=ext alors faible flux_hors_contrat",
            "vigies-regles 1\nregle R-EX12-SORTIE si zone_src=cog alors info flux_hors_contrat",
            "vigies-regles 1\nregle R si foo=bar alors info flux_hors_contrat",
        ] {
            assert!(JeuRegles::charger(t).is_err(), "accepté à tort : {t:?}");
        }
    }

    struct Verif(bool);
    impl VerificateurJeuRegles for Verif {
        fn verifier(&self, _: &[u8], _: &[u8]) -> bool {
            self.0
        }
    }

    #[test]
    fn jeu_signe() {
        let t = "vigies-regles 1\n";
        assert!(JeuRegles::charger_signe(t, b"sig", &Verif(true)).is_ok());
        assert!(JeuRegles::charger_signe(t, b"sig", &Verif(false)).is_err());
    }
}
