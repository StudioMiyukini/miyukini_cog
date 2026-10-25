//! @id     miyuhybride_message
//! @do     generer_codecs_traite
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — messages typés du traité : schéma et codecs générés d'une seule déclaration, des deux côtés (INV-H3, INV-H4, EX-42).

//! # Messages du traité
//!
//! Un service hybride déclare **une fois** chacun de ses messages avec la
//! macro [`message_traite!`] : la même déclaration produit
//!
//! - la structure Rust typée ;
//! - le [`TypeDeclaration`] inscrit dans le traité du manifeste signé ;
//! - l'encodeur vers les [`Champ`] de la trame MSCE et le décodeur strict.
//!
//! Moteur (zone COG) et façade (Franche) compilent la **même** crate de
//! traité : leurs décodeurs sont générés depuis le même schéma, aucune
//! divergence possible (INV-H4). Tout champ inconnu, absent ou de nature
//! différente est refusé (INV-H3).
//!
//! Natures admises pour un champ : `u64` (entier), `bool`, `String` (texte
//! borné), `Vec<u8>` (octets bornés) et leurs `Option<…>` (champ facultatif).
//! Les documents (CDR obligatoire) ne sont pas exprimables ici : un service
//! qui en échange les déclare directement dans son traité.

use miyuportail::{Champ, ChampTraite, Declaration, NatureChamp, Sens, Traite, TypeDeclaration, Valeur};

use crate::erreurs::ErreurTraite;

/// Nature d'un champ de message (implémentée pour les types admis).
pub trait ChampType: Sized {
    /// Champ obligatoire (faux pour `Option<…>`).
    const OBLIGATOIRE: bool;
    /// Nature du champ dans le traité ; `max` borne textes et octets.
    fn nature(max: u32) -> NatureChamp;
    /// Valeur à encoder (`None` : champ facultatif absent).
    fn en_valeur(&self) -> Option<Valeur>;
    /// Décodage strict depuis la valeur du champ `tag` (absente = `None`).
    fn depuis_valeur(tag: u8, valeur: Option<&Valeur>) -> Result<Self, ErreurTraite>;
}

/// Nature scalaire (obligatoire) ; `Option<T>` en dérive.
pub trait ChampScalaire: Sized {
    /// Nature du champ dans le traité.
    fn nature(max: u32) -> NatureChamp;
    /// Valeur MSCE.
    fn valeur(&self) -> Valeur;
    /// Décodage strict.
    fn depuis(tag: u8, valeur: &Valeur) -> Result<Self, ErreurTraite>;
}

impl ChampScalaire for u64 {
    fn nature(_: u32) -> NatureChamp {
        NatureChamp::Entier
    }
    fn valeur(&self) -> Valeur {
        Valeur::Entier(*self)
    }
    fn depuis(tag: u8, valeur: &Valeur) -> Result<Self, ErreurTraite> {
        match valeur {
            Valeur::Entier(v) => Ok(*v),
            _ => Err(ErreurTraite::Nature(tag)),
        }
    }
}

impl ChampScalaire for bool {
    fn nature(_: u32) -> NatureChamp {
        NatureChamp::Booleen
    }
    fn valeur(&self) -> Valeur {
        Valeur::Booleen(*self)
    }
    fn depuis(tag: u8, valeur: &Valeur) -> Result<Self, ErreurTraite> {
        match valeur {
            Valeur::Booleen(v) => Ok(*v),
            _ => Err(ErreurTraite::Nature(tag)),
        }
    }
}

impl ChampScalaire for String {
    fn nature(max: u32) -> NatureChamp {
        NatureChamp::Texte { max }
    }
    fn valeur(&self) -> Valeur {
        Valeur::Texte(self.clone())
    }
    fn depuis(tag: u8, valeur: &Valeur) -> Result<Self, ErreurTraite> {
        match valeur {
            Valeur::Texte(v) => Ok(v.clone()),
            _ => Err(ErreurTraite::Nature(tag)),
        }
    }
}

impl ChampScalaire for Vec<u8> {
    fn nature(max: u32) -> NatureChamp {
        NatureChamp::Octets { max }
    }
    fn valeur(&self) -> Valeur {
        Valeur::Octets(self.clone())
    }
    fn depuis(tag: u8, valeur: &Valeur) -> Result<Self, ErreurTraite> {
        match valeur {
            Valeur::Octets(v) => Ok(v.clone()),
            _ => Err(ErreurTraite::Nature(tag)),
        }
    }
}

impl<T: ChampScalaire> ChampType for T {
    const OBLIGATOIRE: bool = true;
    fn nature(max: u32) -> NatureChamp {
        <T as ChampScalaire>::nature(max)
    }
    fn en_valeur(&self) -> Option<Valeur> {
        Some(self.valeur())
    }
    fn depuis_valeur(tag: u8, valeur: Option<&Valeur>) -> Result<Self, ErreurTraite> {
        T::depuis(tag, valeur.ok_or(ErreurTraite::ChampAbsent(tag))?)
    }
}

impl<T: ChampScalaire> ChampType for Option<T> {
    const OBLIGATOIRE: bool = false;
    fn nature(max: u32) -> NatureChamp {
        <T as ChampScalaire>::nature(max)
    }
    fn en_valeur(&self) -> Option<Valeur> {
        self.as_ref().map(ChampScalaire::valeur)
    }
    fn depuis_valeur(tag: u8, valeur: Option<&Valeur>) -> Result<Self, ErreurTraite> {
        valeur.map(|v| T::depuis(tag, v)).transpose()
    }
}

/// Message typé du traité (implémenté par [`message_traite!`]).
pub trait MessageTraite: Sized {
    /// Sens de traversée.
    const SENS: Sens;
    /// Code du type de déclaration (unique par sens).
    const CODE: u64;
    /// Schéma inscrit dans le traité.
    fn type_declaration() -> TypeDeclaration;
    /// Champs MSCE (tags strictement croissants).
    fn en_champs(&self) -> Vec<Champ>;
    /// Décodage strict des champs.
    fn depuis_champs(champs: &[Champ]) -> Result<Self, ErreurTraite>;

    /// Décodage strict d'une déclaration (sens et type contrôlés).
    fn depuis_declaration(d: &Declaration) -> Result<Self, ErreurTraite> {
        if d.sens != Self::SENS {
            return Err(ErreurTraite::Sens);
        }
        if d.type_code != Self::CODE {
            return Err(ErreurTraite::Type);
        }
        Self::depuis_champs(&d.champs)
    }
}

/// Recherche stricte d'un champ : tags strictement croissants exigés.
#[doc(hidden)]
pub fn champs_canoniques(champs: &[Champ], connus: &[u8]) -> Result<(), ErreurTraite> {
    if champs.windows(2).any(|p| p[0].tag >= p[1].tag) {
        return Err(ErreurTraite::NonCanonique);
    }
    if let Some(c) = champs.iter().find(|c| !connus.contains(&c.tag)) {
        return Err(ErreurTraite::ChampInconnu(c.tag));
    }
    Ok(())
}

/// Valeur du champ `tag`, s'il est présent.
#[doc(hidden)]
pub fn valeur_de(champs: &[Champ], tag: u8) -> Option<&Valeur> {
    champs.iter().find(|c| c.tag == tag).map(|c| &c.valeur)
}

#[doc(hidden)]
pub fn champ_traite<T: ChampType>(tag: u8, max: u32) -> ChampTraite {
    ChampTraite { tag, nature: T::nature(max), obligatoire: T::OBLIGATOIRE }
}

/// Assemble un traité versionné depuis les schémas des messages.
pub fn traite(version: u64, types: Vec<TypeDeclaration>) -> Traite {
    Traite { version, types }
}

/// Déclare un message du traité : structure, schéma et codecs générés.
///
/// ```
/// miyuhybride::message_traite! {
///     /// Demande de démonstration.
///     pub struct Demande (Entrante, 1) {
///         /// Texte borné à 64 octets.
///         1 => pub texte: String, max 64;
///         /// Entier facultatif.
///         2 => pub rang: Option<u64>, max 0;
///     }
/// }
/// use miyuhybride::MessageTraite;
/// let d = Demande { texte: "a".into(), rang: None };
/// assert_eq!(Demande::depuis_champs(&d.en_champs()).unwrap(), d);
/// ```
#[macro_export]
macro_rules! message_traite {
    (
        $(#[$m:meta])*
        $vis:vis struct $nom:ident ($sens:ident, $code:expr) {
            $( $(#[$fm:meta])* $tag:literal => $fvis:vis $champ:ident : $ty:ty, max $max:expr; )*
        }
    ) => {
        $(#[$m])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $nom {
            $( $(#[$fm])* $fvis $champ: $ty, )*
        }

        impl $crate::MessageTraite for $nom {
            const SENS: $crate::Sens = $crate::Sens::$sens;
            const CODE: u64 = $code;

            fn type_declaration() -> $crate::TypeDeclaration {
                $crate::TypeDeclaration {
                    code: $code,
                    sens: $crate::Sens::$sens,
                    champs: vec![ $( $crate::message::champ_traite::<$ty>($tag, $max) ),* ],
                }
            }

            fn en_champs(&self) -> Vec<$crate::Champ> {
                let mut champs = Vec::new();
                $(
                    if let Some(valeur) = <$ty as $crate::message::ChampType>::en_valeur(&self.$champ) {
                        champs.push($crate::Champ { tag: $tag, valeur });
                    }
                )*
                champs
            }

            fn depuis_champs(champs: &[$crate::Champ]) -> Result<Self, $crate::ErreurTraite> {
                $crate::message::champs_canoniques(champs, &[ $( $tag ),* ])?;
                Ok(Self {
                    $(
                        $champ: <$ty as $crate::message::ChampType>::depuis_valeur(
                            $tag,
                            $crate::message::valeur_de(champs, $tag),
                        )?,
                    )*
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::message_traite! {
        /// Message d'essai.
        pub struct Essai (Entrante, 7) {
            1 => pub nom: String, max 16;
            2 => pub age: u64, max 0;
            3 => pub note: Option<String>, max 8;
            4 => pub drapeau: bool, max 0;
            5 => pub empreinte: Option<Vec<u8>>, max 32;
        }
    }

    #[test]
    fn aller_retour_et_schema() {
        let m = Essai { nom: "a".into(), age: 3, note: None, drapeau: true, empreinte: Some(vec![1; 32]) };
        let champs = m.en_champs();
        assert_eq!(champs.iter().map(|c| c.tag).collect::<Vec<_>>(), vec![1, 2, 4, 5]);
        assert_eq!(Essai::depuis_champs(&champs).unwrap(), m);
        let t = Essai::type_declaration();
        assert_eq!((t.code, t.sens), (7, Sens::Entrante));
        assert_eq!(t.champs[0].nature, NatureChamp::Texte { max: 16 });
        assert!(t.champs[0].obligatoire && !t.champs[2].obligatoire);
        traite(1, vec![t]).valider().unwrap();
    }

    #[test]
    fn decodage_strict() {
        let m = Essai { nom: "a".into(), age: 3, note: None, drapeau: false, empreinte: None };
        let mut champs = m.en_champs();
        // Champ inconnu.
        let mut inconnu = champs.clone();
        inconnu.push(Champ { tag: 9, valeur: Valeur::Entier(1) });
        assert_eq!(Essai::depuis_champs(&inconnu), Err(ErreurTraite::ChampInconnu(9)));
        // Nature différente.
        champs[1].valeur = Valeur::Texte("3".into());
        assert_eq!(Essai::depuis_champs(&champs), Err(ErreurTraite::Nature(2)));
        // Obligatoire absent.
        let sans_nom: Vec<Champ> = m.en_champs().into_iter().filter(|c| c.tag != 1).collect();
        assert_eq!(Essai::depuis_champs(&sans_nom), Err(ErreurTraite::ChampAbsent(1)));
        // Désordre.
        let mut desordre = m.en_champs();
        desordre.swap(0, 1);
        assert_eq!(Essai::depuis_champs(&desordre), Err(ErreurTraite::NonCanonique));
    }
}
