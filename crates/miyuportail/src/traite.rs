//! @id     miyuportail_traite
//! @do     decrire_traite_service
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — traité d'un service : types de déclarations autorisés par sens, schéma des champs (INV-H3, INV-H4).

//! Le **traité** est le schéma versionné des déclarations échangées moteur ⇄
//! façade (SERVICES-HYBRIDES-ET-PORTAIL §2). Les décodeurs des deux côtés sont
//! dérivés de ce schéma (INV-H4) : aucun type ni champ hors traité ne traverse
//! (INV-H3).

use crate::erreurs::{ErreurPortail, RefusDecodage};

/// Taille maximale de la charge d'une déclaration (celle du Sceau v1.1).
pub const TAILLE_MAX_CHARGE: usize = miyusceau::TAILLE_MAX_CHARGE;
/// Nombre maximal de types de déclarations par traité.
pub const MAX_TYPES: usize = 64;
/// Nombre maximal de champs par type de déclaration.
pub const MAX_CHAMPS: usize = 32;
/// Longueur maximale d'un identifiant de service.
pub const MAX_ID_SERVICE: usize = 64;
/// Longueur maximale d'un type MIME déclaré par le traité.
pub const MAX_TYPE_MIME: usize = 64;

/// Identifiant de service : `[a-z0-9._:-]`, 1 à 64 octets, commence par une lettre.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdService(String);

impl IdService {
    /// Valide et construit un identifiant.
    pub fn new(id: &str) -> Result<Self, ErreurPortail> {
        if Self::valide(id.as_bytes()) {
            Ok(Self(id.to_owned()))
        } else {
            Err(ErreurPortail::IdentifiantInvalide)
        }
    }

    /// Règle de forme (partagée par les deux décodeurs : c'est une règle du
    /// traité, pas un parseur).
    pub fn valide(octets: &[u8]) -> bool {
        !octets.is_empty()
            && octets.len() <= MAX_ID_SERVICE
            && octets[0].is_ascii_lowercase()
            && octets
                .iter()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b':' | b'-'))
    }

    /// Forme textuelle.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for IdService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Sens d'une voie (diode logicielle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Sens {
    /// Quarantaine (Franche) → zone COG.
    Entrante = 0x01,
    /// Zone COG → publication (Franche).
    Sortante = 0x02,
}

impl Sens {
    /// Décodage strict d'un octet.
    pub const fn depuis_octet(o: u8) -> Option<Self> {
        match o {
            0x01 => Some(Self::Entrante),
            0x02 => Some(Self::Sortante),
            _ => None,
        }
    }
}

/// Classe d'exposition (SERVICES-HYBRIDES-ET-PORTAIL §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClasseExposition {
    /// Interne pur : jamais de façade, jamais de voie.
    H0,
    /// Hybride diffusion : voie publication seule.
    H1,
    /// Hybride collaboratif : quarantaine + publication.
    H2,
    /// Hybride actif : comme H2 (+ liste blanche sortante et intention StrongFather, hors Portail).
    H3,
}

impl ClasseExposition {
    /// Sens ouverts pour la classe.
    pub const fn autorise(self, sens: Sens) -> bool {
        match self {
            Self::H0 => false,
            Self::H1 => matches!(sens, Sens::Sortante),
            Self::H2 | Self::H3 => true,
        }
    }
}

/// Nature d'un document soumis au désarmement (CDR, §3 contrôle 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NatureDocument {
    /// Document bureautique → format interne sans macro, script ni objet embarqué.
    Document = 0x01,
    /// Image → réencodée.
    Image = 0x02,
    /// Archive → refusée, ou dépaquetée et contrôlée une à une.
    Archive = 0x03,
}

impl NatureDocument {
    /// Décodage strict.
    pub const fn depuis_octet(o: u8) -> Option<Self> {
        match o {
            0x01 => Some(Self::Document),
            0x02 => Some(Self::Image),
            0x03 => Some(Self::Archive),
            _ => None,
        }
    }
}

/// Type MIME déclaré par le traité pour un champ `Document` (A-004 : le CDR
/// exige le type déclaré, puis vérifie que le contenu réel lui correspond).
///
/// Forme stricte `type/sous-type`, minuscules ASCII `[a-z0-9.+-]`, sans
/// paramètre, au plus [`MAX_TYPE_MIME`] octets. Valeur copiable (le traité
/// reste `Copy` champ par champ).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeMime {
    octets: [u8; MAX_TYPE_MIME],
    longueur: u8,
}

impl TypeMime {
    /// Valide et construit un type MIME.
    pub fn new(mime: &str) -> Result<Self, ErreurPortail> {
        Self::depuis_octets(mime.as_bytes()).ok_or(ErreurPortail::ContratInvalide("type MIME invalide"))
    }

    fn depuis_octets(o: &[u8]) -> Option<Self> {
        let partie = |p: &[u8]| {
            !p.is_empty()
                && (p[0].is_ascii_lowercase() || p[0].is_ascii_digit())
                && p.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'+' | b'-'))
        };
        if o.len() > MAX_TYPE_MIME {
            return None;
        }
        let mut morceaux = o.split(|b| *b == b'/');
        let (Some(t), Some(st), None) = (morceaux.next(), morceaux.next(), morceaux.next()) else {
            return None;
        };
        if !partie(t) || !partie(st) {
            return None;
        }
        let mut octets = [0u8; MAX_TYPE_MIME];
        octets[..o.len()].copy_from_slice(o);
        Some(Self { octets, longueur: o.len() as u8 })
    }

    /// Forme textuelle.
    pub fn as_str(&self) -> &str {
        // ASCII validé à la construction.
        std::str::from_utf8(&self.octets[..usize::from(self.longueur)]).unwrap_or("")
    }

    fn octets(&self) -> &[u8] {
        &self.octets[..usize::from(self.longueur)]
    }
}

impl std::fmt::Debug for TypeMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeMime({:?})", self.as_str())
    }
}

impl std::fmt::Display for TypeMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Nature d'un champ du traité.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatureChamp {
    /// Entier non signé, exactement 8 octets big-endian.
    Entier,
    /// Booléen, exactement 1 octet `0x00` ou `0x01`.
    Booleen,
    /// Texte UTF-8 de `max` octets au plus, sans caractère de contrôle hors `\t` et `\n`.
    Texte {
        /// Taille maximale en octets.
        max: u32,
    },
    /// Octets opaques typés par le traité, `max` octets au plus.
    Octets {
        /// Taille maximale en octets.
        max: u32,
    },
    /// Fichier ou document : **désarmement (CDR) obligatoire** (EX-45).
    Document {
        /// Taille maximale en octets (entrée reçue **et** sortie reconstruite).
        max: u32,
        /// Nature attendue.
        nature: NatureDocument,
        /// Type MIME déclaré : **obligatoire** pour [`NatureDocument::Document`]
        /// (le CDR désarme selon le type déclaré), **absent** pour les images
        /// (famille imposée, format détecté) et les archives (refusées).
        mime: Option<TypeMime>,
    },
}

impl NatureChamp {
    fn code(self) -> u8 {
        match self {
            Self::Entier => 0x01,
            Self::Booleen => 0x02,
            Self::Texte { .. } => 0x03,
            Self::Octets { .. } => 0x04,
            Self::Document { .. } => 0x05,
        }
    }

    fn max(self) -> u32 {
        match self {
            Self::Entier => 8,
            Self::Booleen => 1,
            Self::Texte { max } | Self::Octets { max } | Self::Document { max, .. } => max,
        }
    }

    fn nature_document(self) -> u8 {
        match self {
            Self::Document { nature, .. } => nature as u8,
            _ => 0,
        }
    }

    fn mime(self) -> Option<TypeMime> {
        match self {
            Self::Document { mime, .. } => mime,
            _ => None,
        }
    }
}

/// Champ d'un type de déclaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChampTraite {
    /// Tag TLV (strictement croissant dans le type, ≥ 1).
    pub tag: u8,
    /// Nature et bornes.
    pub nature: NatureChamp,
    /// Champ obligatoire.
    pub obligatoire: bool,
}

/// Type de déclaration autorisé par le traité.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclaration {
    /// Code du type (unique par sens).
    pub code: u64,
    /// Sens dans lequel ce type peut traverser.
    pub sens: Sens,
    /// Champs, tags strictement croissants.
    pub champs: Vec<ChampTraite>,
}

impl TypeDeclaration {
    /// Champ de tag donné.
    pub fn champ(&self, tag: u8) -> Option<&ChampTraite> {
        self.champs.iter().find(|c| c.tag == tag)
    }
}

/// Traité versionné d'un service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Traite {
    /// Version (égalité stricte exigée : pas de compatibilité implicite).
    pub version: u64,
    /// Types autorisés.
    pub types: Vec<TypeDeclaration>,
}

impl Traite {
    /// Type autorisé pour un sens et un code.
    pub fn type_pour(&self, sens: Sens, code: u64) -> Option<&TypeDeclaration> {
        self.types.iter().find(|t| t.sens == sens && t.code == code)
    }

    /// Restreint le traité à un sens (le sas ne reçoit que les types entrants).
    pub fn restreindre(&self, sens: Sens) -> Self {
        Self { version: self.version, types: self.types.iter().filter(|t| t.sens == sens).cloned().collect() }
    }

    /// Contrôle de cohérence : bornes, tags strictement croissants, codes uniques par sens.
    pub fn valider(&self) -> Result<(), ErreurPortail> {
        if self.types.len() > MAX_TYPES {
            return Err(ErreurPortail::ContratInvalide("trop de types"));
        }
        for (i, t) in self.types.iter().enumerate() {
            if self.types[..i].iter().any(|u| u.sens == t.sens && u.code == t.code) {
                return Err(ErreurPortail::ContratInvalide("code de type dupliqué"));
            }
            if t.champs.len() > MAX_CHAMPS {
                return Err(ErreurPortail::ContratInvalide("trop de champs"));
            }
            let mut precedent = 0u8;
            for c in &t.champs {
                if c.tag == 0 || c.tag <= precedent {
                    return Err(ErreurPortail::ContratInvalide("tags de champs non croissants"));
                }
                precedent = c.tag;
                let max = c.nature.max() as usize;
                if max == 0 || max > TAILLE_MAX_CHARGE {
                    return Err(ErreurPortail::ContratInvalide("borne de champ invalide"));
                }
                match c.nature {
                    NatureChamp::Document { nature: NatureDocument::Document, mime: None, .. } => {
                        return Err(ErreurPortail::ContratInvalide("document sans type MIME déclaré"));
                    }
                    NatureChamp::Document { nature: NatureDocument::Image | NatureDocument::Archive, mime: Some(_), .. } => {
                        return Err(ErreurPortail::ContratInvalide("type MIME réservé aux documents"));
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Encodage binaire canonique (transmission au sas).
    ///
    /// `version u64 | n u8 | n × [code u64 | sens u8 | m u8 | m × [tag u8 | nature u8 | max u32 | doc u8 | oblig u8 | lmime u8 | mime]]`
    ///
    /// (`lmime` = 0 sans type MIME.) Protocole interne parent ⇄ sas : ne fait
    /// pas partie de la trame MSCE (vecteurs figés inchangés).
    pub fn encoder(&self) -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(&self.version.to_be_bytes());
        s.push(self.types.len() as u8);
        for t in &self.types {
            s.extend_from_slice(&t.code.to_be_bytes());
            s.push(t.sens as u8);
            s.push(t.champs.len() as u8);
            for c in &t.champs {
                s.push(c.tag);
                s.push(c.nature.code());
                s.extend_from_slice(&c.nature.max().to_be_bytes());
                s.push(c.nature.nature_document());
                s.push(u8::from(c.obligatoire));
                let mime = c.nature.mime();
                let mime = mime.as_ref().map_or(&[][..], TypeMime::octets);
                s.push(mime.len() as u8);
                s.extend_from_slice(mime);
            }
        }
        s
    }

    /// Décodage strict (taille exacte, valeurs connues, puis [`Traite::valider`]).
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        let mut l = Curseur::new(octets);
        let version = l.u64()?;
        let n = usize::from(l.u8()?);
        if n > MAX_TYPES {
            return Err(RefusDecodage::Requete);
        }
        let mut types = Vec::with_capacity(n);
        for _ in 0..n {
            let code = l.u64()?;
            let sens = Sens::depuis_octet(l.u8()?).ok_or(RefusDecodage::Requete)?;
            let m = usize::from(l.u8()?);
            if m > MAX_CHAMPS {
                return Err(RefusDecodage::Requete);
            }
            let mut champs = Vec::with_capacity(m);
            for _ in 0..m {
                let tag = l.u8()?;
                let code_nature = l.u8()?;
                let max = l.u32()?;
                let doc = l.u8()?;
                let obligatoire = match l.u8()? {
                    0 => false,
                    1 => true,
                    _ => return Err(RefusDecodage::Requete),
                };
                let lmime = usize::from(l.u8()?);
                let mime = if lmime == 0 {
                    None
                } else {
                    Some(TypeMime::depuis_octets(l.prendre(lmime)?).ok_or(RefusDecodage::Requete)?)
                };
                if mime.is_some() && code_nature != 0x05 {
                    return Err(RefusDecodage::Requete);
                }
                let nature = match (code_nature, doc) {
                    (0x01, 0) if max == 8 => NatureChamp::Entier,
                    (0x02, 0) if max == 1 => NatureChamp::Booleen,
                    (0x03, 0) => NatureChamp::Texte { max },
                    (0x04, 0) => NatureChamp::Octets { max },
                    (0x05, d) => NatureChamp::Document {
                        max,
                        nature: NatureDocument::depuis_octet(d).ok_or(RefusDecodage::Requete)?,
                        mime,
                    },
                    _ => return Err(RefusDecodage::Requete),
                };
                champs.push(ChampTraite { tag, nature, obligatoire });
            }
            types.push(TypeDeclaration { code, sens, champs });
        }
        l.fin()?;
        let traite = Self { version, types };
        traite.valider().map_err(|_| RefusDecodage::Requete)?;
        Ok(traite)
    }
}

/// Curseur de lecture bornée (protocole interne parent ⇄ sas).
pub(crate) struct Curseur<'a> {
    reste: &'a [u8],
}

impl<'a> Curseur<'a> {
    pub(crate) fn new(octets: &'a [u8]) -> Self {
        Self { reste: octets }
    }

    pub(crate) fn prendre(&mut self, n: usize) -> Result<&'a [u8], RefusDecodage> {
        if n > self.reste.len() {
            return Err(RefusDecodage::Requete);
        }
        let (tete, queue) = self.reste.split_at(n);
        self.reste = queue;
        Ok(tete)
    }

    pub(crate) fn u8(&mut self) -> Result<u8, RefusDecodage> {
        Ok(self.prendre(1)?[0])
    }

    pub(crate) fn u32(&mut self) -> Result<u32, RefusDecodage> {
        let o = self.prendre(4)?;
        Ok(u32::from_be_bytes([o[0], o[1], o[2], o[3]]))
    }

    pub(crate) fn u64(&mut self) -> Result<u64, RefusDecodage> {
        let o = self.prendre(8)?;
        let mut t = [0u8; 8];
        t.copy_from_slice(o);
        Ok(u64::from_be_bytes(t))
    }

    pub(crate) fn b32(&mut self) -> Result<[u8; 32], RefusDecodage> {
        let o = self.prendre(32)?;
        let mut t = [0u8; 32];
        t.copy_from_slice(o);
        Ok(t)
    }

    pub(crate) fn fin(&self) -> Result<(), RefusDecodage> {
        if self.reste.is_empty() {
            Ok(())
        } else {
            Err(RefusDecodage::Requete)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exemple() -> Traite {
        Traite {
            version: 3,
            types: vec![
                TypeDeclaration {
                    code: 1,
                    sens: Sens::Entrante,
                    champs: vec![
                        ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 256 }, obligatoire: true },
                        ChampTraite { tag: 2, nature: NatureChamp::Entier, obligatoire: false },
                        ChampTraite {
                            tag: 5,
                            nature: NatureChamp::Document { max: 4096, nature: NatureDocument::Image, mime: None },
                            obligatoire: false,
                        },
                    ],
                },
                TypeDeclaration {
                    code: 1,
                    sens: Sens::Sortante,
                    champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Booleen, obligatoire: true }],
                },
            ],
        }
    }

    #[test]
    fn aller_retour_traite() {
        let t = exemple();
        t.valider().unwrap();
        assert_eq!(Traite::decoder(&t.encoder()).unwrap(), t);
    }

    #[test]
    fn traite_octet_en_trop_refuse() {
        let mut o = exemple().encoder();
        o.push(0);
        assert_eq!(Traite::decoder(&o), Err(RefusDecodage::Requete));
    }

    #[test]
    fn traite_tags_non_croissants_refuse() {
        let mut t = exemple();
        t.types[0].champs.swap(0, 1);
        assert!(t.valider().is_err());
        assert!(Traite::decoder(&t.encoder()).is_err());
    }

    #[test]
    fn document_exige_un_type_mime_declare() {
        let mut t = exemple();
        t.types[0].champs.push(ChampTraite {
            tag: 6,
            nature: NatureChamp::Document { max: 1024, nature: NatureDocument::Document, mime: None },
            obligatoire: false,
        });
        assert!(t.valider().is_err(), "document sans type MIME");
        t.types[0].champs[3].nature = NatureChamp::Document {
            max: 1024,
            nature: NatureDocument::Document,
            mime: Some(TypeMime::new("text/csv").unwrap()),
        };
        t.valider().unwrap();
        assert_eq!(Traite::decoder(&t.encoder()).unwrap(), t);
        // Type MIME sur une image : refusé.
        t.types[0].champs[2].nature =
            NatureChamp::Document { max: 4096, nature: NatureDocument::Image, mime: Some(TypeMime::new("image/png").unwrap()) };
        assert!(t.valider().is_err());
    }

    #[test]
    fn type_mime_strict() {
        assert_eq!(TypeMime::new("text/plain").unwrap().as_str(), "text/plain");
        assert!(TypeMime::new("application/vnd.oasis.opendocument.text").is_ok());
        for faux in ["", "text", "Text/plain", "text/plain; charset=utf-8", "a/b/c", "/x", "x/", "text/pl ain"] {
            assert!(TypeMime::new(faux).is_err(), "{faux}");
        }
        assert!(TypeMime::new(&format!("a/{}", "b".repeat(MAX_TYPE_MIME))).is_err());
    }

    #[test]
    fn identifiant_service() {
        assert!(IdService::new("jayoffice.docs").is_ok());
        assert!(IdService::new("").is_err());
        assert!(IdService::new("Majuscule").is_err());
        assert!(IdService::new("1abc").is_err());
        assert!(IdService::new("a/b").is_err());
        assert!(IdService::new(&"a".repeat(65)).is_err());
    }

    #[test]
    fn classes() {
        assert!(!ClasseExposition::H0.autorise(Sens::Sortante));
        assert!(!ClasseExposition::H1.autorise(Sens::Entrante));
        assert!(ClasseExposition::H1.autorise(Sens::Sortante));
        assert!(ClasseExposition::H2.autorise(Sens::Entrante));
    }
}
