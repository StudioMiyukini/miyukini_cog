//! @id     maia_memoire_ident
//! @do     adresser_memoire_par_chemin
//! @role   donnee
//! @layer  8
//! @human  maia-memoire — identifiants hiérarchiques stables `domaine.sujet.note` (§4, EX-51).

//! Identifiants hiérarchiques (§4).
//!
//! - chemin pointé, alphabet `[a-z0-9_]`, segment commençant par une lettre,
//!   32 caractères au plus par segment (convention des `@id` MSCM, ASCII) ;
//! - une **note** se termine par un segment typé : lettre de type (`f` fait,
//!   `p` préférence, `r` règle, `e` épisode, `k` procédure, `n` entité,
//!   `s` résumé) suivie d'un numéro sans zéro initial (`f12`) ;
//! - une **branche** est un préfixe sans segment typé : `travail.cog` désigne
//!   tout le sous-arbre ;
//! - un segment de branche ne peut pas avoir la forme d'un segment typé (pas
//!   d'ambiguïté entre `travail.f1` note et `travail.f1` branche).

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::erreurs::ErreurMemoire;

/// Segments de branche au plus (une note en a un de plus).
pub const MAX_SEGMENTS_BRANCHE: usize = 5;
/// Longueur maximale d'un segment.
pub const MAX_LONGUEUR_SEGMENT: usize = 32;
/// Chiffres du numéro d'une note au plus.
pub const MAX_CHIFFRES_NUMERO: usize = 6;

/// Type d'unité de mémoire (§3) et sa lettre de suffixe (§4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeUnite {
    /// Affirmation sur le monde ou l'utilisateur (`f`).
    Fait,
    /// Goût, habitude, réglage (`p`).
    Preference,
    /// Consigne durable posée explicitement par l'utilisateur (`r`).
    Regle,
    /// Événement daté (`e`).
    Episode,
    /// Savoir-faire appris (`k`).
    Procedure,
    /// Fiche personne / lieu / objet / projet (`n`).
    Entite,
    /// Nœud de synthèse N1 / N2 (`s`).
    Resume,
}

impl TypeUnite {
    /// Tous les types.
    pub const TOUS: [TypeUnite; 7] = [
        TypeUnite::Fait,
        TypeUnite::Preference,
        TypeUnite::Regle,
        TypeUnite::Episode,
        TypeUnite::Procedure,
        TypeUnite::Entite,
        TypeUnite::Resume,
    ];

    /// Lettre de suffixe.
    pub fn lettre(self) -> char {
        match self {
            TypeUnite::Fait => 'f',
            TypeUnite::Preference => 'p',
            TypeUnite::Regle => 'r',
            TypeUnite::Episode => 'e',
            TypeUnite::Procedure => 'k',
            TypeUnite::Entite => 'n',
            TypeUnite::Resume => 's',
        }
    }

    /// Type d'une lettre de suffixe.
    pub fn depuis_lettre(c: char) -> Option<Self> {
        TypeUnite::TOUS.into_iter().find(|t| t.lettre() == c)
    }
}

/// Segment ordinaire : `[a-z][a-z0-9_]{0,31}`.
fn segment_valide(s: &str) -> bool {
    let mut cs = s.chars();
    matches!(cs.next(), Some(c) if c.is_ascii_lowercase())
        && s.len() <= MAX_LONGUEUR_SEGMENT
        && cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Segment typé : lettre de type + numéro sans zéro initial.
fn segment_type(s: &str) -> Option<(TypeUnite, u32)> {
    let mut cs = s.chars();
    let t = TypeUnite::depuis_lettre(cs.next()?)?;
    let chiffres = cs.as_str();
    if chiffres.is_empty()
        || chiffres.len() > MAX_CHIFFRES_NUMERO
        || chiffres.starts_with('0')
        || !chiffres.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    chiffres.parse().ok().map(|n| (t, n))
}

/// Branche (sous-arbre) : `travail`, `travail.cog`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Branche(String);

impl Branche {
    /// Analyse une branche.
    pub fn parse(texte: &str) -> Result<Self, ErreurMemoire> {
        let segments: Vec<&str> = texte.split('.').collect();
        if segments.is_empty()
            || segments.len() > MAX_SEGMENTS_BRANCHE
            || !segments
                .iter()
                .all(|s| segment_valide(s) && segment_type(s).is_none())
        {
            return Err(ErreurMemoire::IdInvalide(texte.chars().take(80).collect()));
        }
        Ok(Self(texte.to_string()))
    }

    /// Texte de la branche.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Premier segment (domaine).
    pub fn domaine(&self) -> &str {
        self.0.split('.').next().unwrap_or_default()
    }

    /// Nombre de segments.
    pub fn profondeur(&self) -> usize {
        self.0.split('.').count()
    }

    /// Vrai si l'identifiant est dans le sous-arbre (frontière de segment).
    pub fn contient(&self, id: &IdMemoire) -> bool {
        id.as_str()
            .strip_prefix(self.0.as_str())
            .is_some_and(|reste| reste.starts_with('.'))
    }

    /// Branche tronquée à `n` segments (au moins un).
    pub fn tronquer(&self, n: usize) -> Branche {
        let n = n.max(1);
        Branche(self.0.split('.').take(n).collect::<Vec<_>>().join("."))
    }
}

impl TryFrom<String> for Branche {
    type Error = ErreurMemoire;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        Branche::parse(&v)
    }
}

impl From<Branche> for String {
    fn from(b: Branche) -> Self {
        b.0
    }
}

impl fmt::Display for Branche {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifiant stable d'une note : `travail.cog.f12`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct IdMemoire {
    texte: String,
    type_unite: TypeUnite,
    numero: u32,
}

impl IdMemoire {
    /// Analyse un identifiant de note.
    pub fn parse(texte: &str) -> Result<Self, ErreurMemoire> {
        let invalide = || ErreurMemoire::IdInvalide(texte.chars().take(80).collect());
        let (branche, dernier) = texte.rsplit_once('.').ok_or_else(invalide)?;
        Branche::parse(branche).map_err(|_| invalide())?;
        let (type_unite, numero) = segment_type(dernier).ok_or_else(invalide)?;
        Ok(Self {
            texte: texte.to_string(),
            type_unite,
            numero,
        })
    }

    /// Identifiant `branche.<type><numero>`.
    pub fn nouveau(branche: &Branche, type_unite: TypeUnite, numero: u32) -> Result<Self, ErreurMemoire> {
        IdMemoire::parse(&format!("{}.{}{numero}", branche.as_str(), type_unite.lettre()))
    }

    /// Texte de l'identifiant.
    pub fn as_str(&self) -> &str {
        &self.texte
    }

    /// Type d'unité (suffixe).
    pub fn type_unite(&self) -> TypeUnite {
        self.type_unite
    }

    /// Numéro (suffixe).
    pub fn numero(&self) -> u32 {
        self.numero
    }

    /// Branche parente (lien « parent » du §3).
    pub fn branche(&self) -> Branche {
        let (b, _) = self.texte.rsplit_once('.').unwrap_or_default();
        Branche(b.to_string())
    }

    /// Domaine (premier segment).
    pub fn domaine(&self) -> &str {
        self.texte.split('.').next().unwrap_or_default()
    }
}

impl TryFrom<String> for IdMemoire {
    type Error = ErreurMemoire;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        IdMemoire::parse(&v)
    }
}

impl From<IdMemoire> for String {
    fn from(id: IdMemoire) -> Self {
        id.texte
    }
}

impl fmt::Display for IdMemoire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.texte)
    }
}

/// Cible d'une ouverture (`memoire.ouvrir(id)`) : une note ou une branche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chemin {
    /// Une note précise.
    Note(IdMemoire),
    /// Un sous-arbre.
    Branche(Branche),
}

impl Chemin {
    /// Analyse un chemin : note si le dernier segment est typé, branche sinon.
    pub fn parse(texte: &str) -> Result<Self, ErreurMemoire> {
        IdMemoire::parse(texte)
            .map(Chemin::Note)
            .or_else(|_| Branche::parse(texte).map(Chemin::Branche))
    }

    /// Vrai si la note est désignée (elle-même ou dans le sous-arbre).
    pub fn designe(&self, id: &IdMemoire) -> bool {
        match self {
            Chemin::Note(n) => n == id,
            Chemin::Branche(b) => b.contient(id),
        }
    }

    /// Domaine visé.
    pub fn domaine(&self) -> &str {
        match self {
            Chemin::Note(n) => n.domaine(),
            Chemin::Branche(b) => b.domaine(),
        }
    }

    /// Texte du chemin.
    pub fn as_str(&self) -> &str {
        match self {
            Chemin::Note(n) => n.as_str(),
            Chemin::Branche(b) => b.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemples_du_document() {
        for (texte, t, n) in [
            ("travail.cog.f12", TypeUnite::Fait, 12),
            ("famille.lea.p3", TypeUnite::Preference, 3),
            ("sante.sommeil.e7", TypeUnite::Episode, 7),
            ("travail.cog.s1", TypeUnite::Resume, 1),
        ] {
            let id = IdMemoire::parse(texte).expect(texte);
            assert_eq!((id.type_unite(), id.numero(), id.as_str()), (t, n, texte));
        }
        let id = IdMemoire::parse("travail.cog.f12").expect("id");
        assert_eq!(id.branche().as_str(), "travail.cog");
        assert_eq!(id.domaine(), "travail");
        assert!(Branche::parse("travail.cog").expect("b").contient(&id));
        assert!(Branche::parse("travail").expect("b").contient(&id));
        assert!(!Branche::parse("travail.co").expect("b").contient(&id));
    }

    #[test]
    fn refus() {
        for texte in [
            "", "travail", "Travail.f1", "travail.f0", "travail.f01", "travail..f1",
            "travail.x1", "1travail.f1", "travail.cog-x.f1", "a.b.c.d.e.f.f1",
            "travail.f1.f2", "travail.f1234567", "travail. f1", "trävail.f1",
        ] {
            assert!(IdMemoire::parse(texte).is_err(), "{texte:?}");
        }
        assert!(Branche::parse("travail.f1").is_err());
        assert!(Branche::parse(&"a".repeat(33)).is_err());
        assert!(matches!(Chemin::parse("travail.cog"), Ok(Chemin::Branche(_))));
        assert!(matches!(Chemin::parse("travail.cog.k2"), Ok(Chemin::Note(_))));
    }

    #[test]
    fn serde_valide() {
        let id: IdMemoire = serde_json::from_str("\"travail.cog.f12\"").expect("ok");
        assert_eq!(serde_json::to_string(&id).expect("ser"), "\"travail.cog.f12\"");
        assert!(serde_json::from_str::<IdMemoire>("\"travail.cog\"").is_err());
    }
}
