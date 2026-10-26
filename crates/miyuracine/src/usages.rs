//! @id     miyuracine_usages
//! @do     declarer_chemins_usage
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « usages ».

//! Registre des chemins de dérivation : **une clé par usage** (EX-25),
//! dérivée de la racine en SLIP-10 durcie.
//!
//! Usages tirés de la hiérarchie de clés du document gelé
//! `docs/securite/ARCHITECTURE-lot2-identite-signee.md` §3. Le document ne
//! fixe ni indices ni chemins : la table ci-dessous est définie ici
//! (choix de ce crate, à reporter dans la documentation par amendement).
//!
//! | Usage       | Rôle (doc §3)                                   | Chemin SLIP-10              | Rotation |
//! |-------------|-------------------------------------------------|-----------------------------|----------|
//! | `identite`  | clé publique racine (`cog_id`), passation, reconnaissances, certificats, **délégations `0x45`** | `m/1296256839'/0'`          | nouvelle racine (passation) |
//! | `emission`  | signe les **passeports** (seulement, A-022 §1)  | `m/1296256839'/1'/g'`       | 90 j (g = génération) |
//! | `journal`   | signe les têtes du registre                     | `m/1296256839'/2'/g'`       | 90 j |
//! | `editeur`   | certificats éditeur / paquets publiés par le COG | `m/1296256839'/3'/g'`       | par génération |
//! | `arretes`   | arrêtés de frontière de BorderGuard (A-004)     | `m/1296256839'/4'/g'`       | par génération |
//! | `visas`     | signe les **visas** (guichet en zone COG, A-022 §1) | `m/1296256839'/5'/g'`   | 90 j |
//! | `fiches`    | signe les Fiches (pont MAIA, A-022 §4)          | `m/1296256839'/6'/g'`       | 90 j |
//!
//! `1296256839` = `0x4D434F47` = « MCOG » en ASCII (but, inférieur à 2³¹).
//! L'indice 4 « arrêtés » est fixé par l'amendement A-004 (doc v1.4) ; les
//! indices 5 « visas » et 6 « fiches » par A-022 (doc v1.22) ; les indices 7
//! et suivants sont réservés ; un indice d'usage n'est jamais réattribué.
//! `g` (génération) est un entier < 2³¹ incrémenté à chaque rotation de la
//! clé d'usage.
//!
//! Depuis A-022 §1, l'indice 1 `emission` ne signe plus les visas : un visa
//! signé par une clé déléguée d'indice 1 est refusé par `miyauth` (aucune
//! transition pour les clés déléguées : un visa vit au plus 15 min et aucune
//! délégation `0x45` n'existait avant A-022). Chaque clé d'usage est
//! certifiée par une délégation `0x45` signée par l'identité
//! ([`crate::Racine::deleguer`], sous-commande `deleguer` du CLI).

use std::fmt;
use std::str::FromStr;

use crate::erreurs::ErreurRacine;
use crate::slip10::DURCI;

/// But (« purpose ») commun à tous les chemins de la racine COG : « MCOG ».
pub const BUT_MIYUKINI_COG: u32 = 0x4D43_4F47;

// Le but doit rester un indice non durci (< 2³¹) : contrôlé à la compilation.
const _: () = assert!(BUT_MIYUKINI_COG < DURCI);

/// Usage d'une clé dérivée de la racine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Usage {
    /// Clé publique racine du COG : `cog_id`, passation, reconnaissances, certificats.
    Identite,
    /// Clé d'émission : passeports seulement depuis A-022 §1 (rotation 90 j).
    Emission,
    /// Clé de journal : têtes du registre Merkle (rotation 90 j).
    Journal,
    /// Clé éditeur du COG : certificats éditeur et paquets publiés par le COG.
    Editeur,
    /// Clé des arrêtés : arrêtés de frontière signés par BorderGuard (A-004, indice 4).
    Arretes,
    /// Clé des visas : visas émis par le guichet en zone COG (A-022 §1, indice 5, rotation 90 j).
    Visas,
    /// Clé des Fiches : Fiches signées par le pont MAIA (A-022 §4, indice 6).
    Fiches,
}

impl Usage {
    /// Tous les usages, dans l'ordre des indices.
    pub const TOUS: [Self; 7] = [
        Self::Identite,
        Self::Emission,
        Self::Journal,
        Self::Editeur,
        Self::Arretes,
        Self::Visas,
        Self::Fiches,
    ];

    /// Indice d'usage dans le chemin (jamais réattribué).
    pub const fn indice(self) -> u32 {
        match self {
            Self::Identite => 0,
            Self::Emission => 1,
            Self::Journal => 2,
            Self::Editeur => 3,
            Self::Arretes => 4,
            Self::Visas => 5,
            Self::Fiches => 6,
        }
    }

    /// Nom stable (CLI, journaux).
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Identite => "identite",
            Self::Emission => "emission",
            Self::Journal => "journal",
            Self::Editeur => "editeur",
            Self::Arretes => "arretes",
            Self::Visas => "visas",
            Self::Fiches => "fiches",
        }
    }

    /// Vrai si la clé tourne par générations (sinon : unique, génération 0).
    pub const fn a_generations(self) -> bool {
        !matches!(self, Self::Identite)
    }

    /// Chemin SLIP-10 (indices non durcis ; tous sont durcis à la dérivation).
    ///
    /// L'identité n'accepte que la génération 0 : sa rotation est une
    /// nouvelle racine (passation), jamais un nouvel indice.
    pub fn chemin(self, generation: u32) -> Result<Vec<u32>, ErreurRacine> {
        if generation >= DURCI {
            return Err(ErreurRacine::IndiceHorsPlage);
        }
        if self.a_generations() {
            Ok(vec![BUT_MIYUKINI_COG, self.indice(), generation])
        } else if generation == 0 {
            Ok(vec![BUT_MIYUKINI_COG, self.indice()])
        } else {
            Err(ErreurRacine::GenerationInvalide)
        }
    }

    /// Chemin lisible, par exemple `m/1296256839'/1'/0'`.
    pub fn chemin_texte(self, generation: u32) -> Result<String, ErreurRacine> {
        use std::fmt::Write as _;
        let mut s = String::from("m");
        for i in self.chemin(generation)? {
            let _ = write!(s, "/{i}'");
        }
        Ok(s)
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.nom())
    }
}

impl FromStr for Usage {
    type Err = ErreurRacine;

    fn from_str(s: &str) -> Result<Self, ErreurRacine> {
        Self::TOUS
            .into_iter()
            .find(|u| u.nom() == s)
            .ok_or(ErreurRacine::UsageInconnu)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn but_est_mcog() {
        assert_eq!(&BUT_MIYUKINI_COG.to_be_bytes(), b"MCOG");
    }

    #[test]
    fn chemins_distincts_et_stables() {
        assert_eq!(Usage::Identite.chemin_texte(0).unwrap(), "m/1296256839'/0'");
        assert_eq!(
            Usage::Emission.chemin_texte(0).unwrap(),
            "m/1296256839'/1'/0'"
        );
        assert_eq!(
            Usage::Journal.chemin_texte(3).unwrap(),
            "m/1296256839'/2'/3'"
        );
        assert_eq!(
            Usage::Editeur.chemin_texte(0).unwrap(),
            "m/1296256839'/3'/0'"
        );
        assert_eq!(
            Usage::Identite.chemin(1).unwrap_err(),
            ErreurRacine::GenerationInvalide
        );
        assert_eq!(
            Usage::Emission.chemin(DURCI).unwrap_err(),
            ErreurRacine::IndiceHorsPlage
        );
        assert_eq!(
            Usage::Arretes.chemin_texte(2).unwrap(),
            "m/1296256839'/4'/2'"
        );
        let indices: Vec<u32> = Usage::TOUS.iter().map(|u| u.indice()).collect();
        assert_eq!(indices, vec![0, 1, 2, 3, 4, 5, 6]);
    }

    /// A-004 : indice 4 = arrêtés ; les indices existants ne bougent pas.
    #[test]
    fn indice_arretes_a004_et_indices_stables() {
        assert_eq!(Usage::Arretes.indice(), 4);
        assert_eq!(Usage::Arretes.nom(), "arretes");
        assert!(Usage::Arretes.a_generations());
        assert_eq!(
            Usage::Arretes.chemin(0).unwrap(),
            vec![BUT_MIYUKINI_COG, 4, 0]
        );
        let attendus = [
            (Usage::Identite, 0),
            (Usage::Emission, 1),
            (Usage::Journal, 2),
            (Usage::Editeur, 3),
            (Usage::Arretes, 4),
            (Usage::Visas, 5),
            (Usage::Fiches, 6),
        ];
        for (u, i) in attendus {
            assert_eq!(u.indice(), i, "{u}");
        }
    }

    /// A-022 : indice 5 = visas, indice 6 = fiches (rotation par générations).
    #[test]
    fn indices_visas_fiches_a022() {
        assert_eq!(Usage::Visas.chemin_texte(0).unwrap(), "m/1296256839'/5'/0'");
        assert_eq!(Usage::Fiches.chemin_texte(3).unwrap(), "m/1296256839'/6'/3'");
        assert!(Usage::Visas.a_generations() && Usage::Fiches.a_generations());
        assert_eq!("visas".parse::<Usage>().unwrap(), Usage::Visas);
        assert_eq!("fiches".parse::<Usage>().unwrap(), Usage::Fiches);
    }

    #[test]
    fn noms_aller_retour() {
        for u in Usage::TOUS {
            assert_eq!(u.nom().parse::<Usage>().unwrap(), u);
        }
        assert_eq!(
            "racine".parse::<Usage>().unwrap_err(),
            ErreurRacine::UsageInconnu
        );
    }
}
