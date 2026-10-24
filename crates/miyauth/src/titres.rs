//! @id     miyauth_titres
//! @do     definir_passeport_visa
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — titres Passeport et Visa au format Sceau v1.1 (doc gelée lot 2, §5 et §6).

//! Titres d'identité scellés : [`Passeport`] (type `0x01`) et [`Visa`] (type `0x02`).
//!
//! Encodage TLV canonique de `miyusceau` : tags strictement croissants, tout champ
//! inconnu ou manquant est un refus, valeurs de taille exacte. Les contrôles de forme
//! (`cog_id`, portées, audience) sont faits au décodage : un titre mal formé n'est
//! jamais exposé, même correctement signé.

use miyusceau::{EcrivainTlv, ErreurSceau, LecteurTlv, Titre, TypeSceau};

use crate::types::IdentityRole;

/// Longueur d'un `cog_id` : `base32(SHA-256(clé racine))[0..26]`, minuscules (doc §3).
pub const LONGUEUR_COG_ID: usize = 26;
/// Durée maximale d'un passeport (décision 3 : 30 jours).
pub const DUREE_MAX_PASSEPORT_SECS: u64 = 30 * 24 * 3600;
/// Durée maximale d'un visa (décision 3 : 15 minutes).
pub const DUREE_MAX_VISA_SECS: u64 = 15 * 60;
/// Nombre maximal de portées par titre (choix fail-closed : borne la charge).
pub const MAX_PORTEES: usize = 32;
/// Longueur maximale d'une portée.
pub const LONGUEUR_MAX_PORTEE: usize = 128;
/// Longueur maximale d'une audience (nom de service).
pub const LONGUEUR_MAX_AUDIENCE: usize = 64;

const ALPHABET_BASE32: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";

/// Numéro de série d'un titre : 128 bits aléatoires (« identifiant NFT », doc §5).
pub type Serie = [u8; 16];

/// Identifiant d'un COG (empreinte de sa clé racine), forme validée.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CogId(String);

impl CogId {
    /// Valide un `cog_id` : 26 caractères de l'alphabet base32 minuscule (RFC 4648).
    pub fn new(valeur: &str) -> Option<Self> {
        (valeur.len() == LONGUEUR_COG_ID && valeur.bytes().all(|b| ALPHABET_BASE32.contains(&b)))
            .then(|| Self(valeur.to_string()))
    }

    /// Forme textuelle.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Portées (droits) d'un titre : ensemble trié, sans doublon, borné.
///
/// Une seule représentation valide : l'ordre lexicographique strict est imposé au
/// décodage. miyauth ne les interprète pas (l'autorisation relève de StrongFather).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Portees(Vec<String>);

impl Portees {
    /// Construit un ensemble canonique (tri + dédoublonnage). Refuse toute portée
    /// mal formée (vide, trop longue, caractère hors ASCII visible) ou un excès.
    pub fn new<I, S>(portees: I) -> Option<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut v: Vec<String> = portees.into_iter().map(Into::into).collect();
        v.sort();
        v.dedup();
        (v.len() <= MAX_PORTEES && v.iter().all(|p| portee_valide(p))).then_some(Self(v))
    }

    /// Portées, triées.
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// Vrai si la portée exacte est présente.
    pub fn contient(&self, portee: &str) -> bool {
        self.0.binary_search_by(|p| p.as_str().cmp(portee)).is_ok()
    }

    fn encoder(&self) -> Result<Vec<u8>, ErreurSceau> {
        let mut sous = EcrivainTlv::new();
        for (i, p) in self.0.iter().enumerate() {
            let tag = u8::try_from(i).map_err(|_| ErreurSceau::ChargeTropGrande)?;
            sous.texte(tag, p)?;
        }
        Ok(sous.finir())
    }

    fn decoder(tag: u8, valeur: &[u8]) -> Result<Self, ErreurSceau> {
        let mut lecteur = LecteurTlv::new(valeur);
        let mut v: Vec<String> = Vec::new();
        for i in 0..=MAX_PORTEES {
            let sous_tag = u8::try_from(i).map_err(|_| ErreurSceau::ValeurInvalide(tag))?;
            let Some(octets) = lecteur.optionnel(sous_tag).map_err(|_| ErreurSceau::ValeurInvalide(tag))? else {
                break;
            };
            if i == MAX_PORTEES {
                return Err(ErreurSceau::ValeurInvalide(tag));
            }
            let p = std::str::from_utf8(octets).map_err(|_| ErreurSceau::ValeurInvalide(tag))?;
            if !portee_valide(p) || v.last().is_some_and(|precedente| precedente.as_str() >= p) {
                return Err(ErreurSceau::ValeurInvalide(tag));
            }
            v.push(p.to_string());
        }
        lecteur.terminer().map_err(|_| ErreurSceau::ValeurInvalide(tag))?;
        Ok(Self(v))
    }
}

fn portee_valide(p: &str) -> bool {
    !p.is_empty() && p.len() <= LONGUEUR_MAX_PORTEE && p.bytes().all(|b| b.is_ascii_graphic())
}

/// Audience (service visé) : `[a-z0-9._-]`, 1 à 64 caractères.
pub fn audience_valide(a: &str) -> bool {
    !a.is_empty()
        && a.len() <= LONGUEUR_MAX_AUDIENCE
        && a.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-'))
}

fn role_vers_code(role: IdentityRole) -> Result<u64, ErreurSceau> {
    match role {
        IdentityRole::Citizen => Ok(1),
        IdentityRole::Visitor => Ok(2),
        // Un titre n'accorde jamais le rôle « externe » : refus à l'encodage.
        IdentityRole::External => Err(ErreurSceau::ValeurInvalide(0)),
    }
}

fn code_vers_role(tag: u8, code: u64) -> Result<IdentityRole, ErreurSceau> {
    match code {
        1 => Ok(IdentityRole::Citizen),
        2 => Ok(IdentityRole::Visitor),
        _ => Err(ErreurSceau::ValeurInvalide(tag)),
    }
}

fn lire_serie(lecteur: &mut LecteurTlv<'_>, tag: u8) -> Result<Serie, ErreurSceau> {
    lecteur.exiger(tag)?.try_into().map_err(|_| ErreurSceau::ValeurInvalide(tag))
}

fn lire_cog(lecteur: &mut LecteurTlv<'_>, tag: u8) -> Result<CogId, ErreurSceau> {
    CogId::new(lecteur.exiger_texte(tag)?).ok_or(ErreurSceau::ValeurInvalide(tag))
}

fn controler_fenetre(tag: u8, pas_avant: u64, pas_apres: u64, duree_max: u64) -> Result<(), ErreurSceau> {
    if pas_avant >= pas_apres || pas_apres - pas_avant > duree_max {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Passeport (doc §5)
// ═══════════════════════════════════════════════════════════════════════════════

mod tag_passeport {
    pub const SERIE: u8 = 0x01;
    pub const EMETTEUR_COG: u8 = 0x02;
    pub const EMETTEUR_KID: u8 = 0x03;
    pub const SUJET: u8 = 0x04;
    pub const DESTINATION: u8 = 0x05;
    pub const CLE_APPAREIL: u8 = 0x06;
    pub const ROLE: u8 = 0x07;
    pub const PORTEES: u8 = 0x08;
    pub const PAS_AVANT: u8 = 0x09;
    pub const PAS_APRES: u8 = 0x0A;
}

/// Passeport : identité personne + appareil, émis par la clé d'émission du COG d'origine.
///
/// EX-36 : un passeport est émis **pour un COG destination** ; `sujet` est
/// l'identifiant par paire (pairwise) de la personne pour ce COG
/// ([`crate::pairwise::identifiant_pairwise`]). Deux COG visités reçoivent donc deux
/// sujets différents, non reliables sans le secret du titulaire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passeport {
    /// Numéro de série aléatoire (128 bits).
    pub serie: Serie,
    /// `cog_id` du COG d'origine (émetteur).
    pub emetteur_cog: CogId,
    /// `kid` de la clé d'émission qui scelle le passeport.
    pub emetteur_kid: [u8; 32],
    /// Identifiant par paire du titulaire pour `destination` (32 octets).
    pub sujet: [u8; 32],
    /// COG pour lequel le passeport est émis (le COG d'origine lui-même pour un usage local).
    pub destination: CogId,
    /// Clé publique Ed25519 de l'appareil titulaire (preuve de possession, doc §7).
    pub cle_appareil: [u8; 32],
    /// Toujours `Citizen` (refus au décodage sinon).
    pub role: IdentityRole,
    /// Droits de base.
    pub portees: Portees,
    /// Début de validité (secondes Unix).
    pub pas_avant: u64,
    /// Fin de validité (secondes Unix), au plus 30 jours après `pas_avant`.
    pub pas_apres: u64,
}

impl Titre for Passeport {
    const TYPE: TypeSceau = TypeSceau::Passeport;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        use tag_passeport::*;
        if self.role != IdentityRole::Citizen {
            return Err(ErreurSceau::ValeurInvalide(ROLE));
        }
        controler_fenetre(PAS_APRES, self.pas_avant, self.pas_apres, DUREE_MAX_PASSEPORT_SECS)?;
        e.octets(SERIE, &self.serie)?
            .texte(EMETTEUR_COG, self.emetteur_cog.as_str())?
            .b32(EMETTEUR_KID, &self.emetteur_kid)?
            .b32(SUJET, &self.sujet)?
            .texte(DESTINATION, self.destination.as_str())?
            .b32(CLE_APPAREIL, &self.cle_appareil)?
            .u64(ROLE, role_vers_code(self.role)?)?
            .octets(PORTEES, &self.portees.encoder()?)?
            .u64(PAS_AVANT, self.pas_avant)?
            .u64(PAS_APRES, self.pas_apres)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        use tag_passeport::*;
        let serie = lire_serie(l, SERIE)?;
        let emetteur_cog = lire_cog(l, EMETTEUR_COG)?;
        let emetteur_kid = l.exiger_b32(EMETTEUR_KID)?;
        let sujet = l.exiger_b32(SUJET)?;
        let destination = lire_cog(l, DESTINATION)?;
        let cle_appareil = l.exiger_b32(CLE_APPAREIL)?;
        let role = code_vers_role(ROLE, l.exiger_u64(ROLE)?)?;
        if role != IdentityRole::Citizen {
            return Err(ErreurSceau::ValeurInvalide(ROLE));
        }
        let portees = Portees::decoder(PORTEES, l.exiger(PORTEES)?)?;
        let pas_avant = l.exiger_u64(PAS_AVANT)?;
        let pas_apres = l.exiger_u64(PAS_APRES)?;
        controler_fenetre(PAS_APRES, pas_avant, pas_apres, DUREE_MAX_PASSEPORT_SECS)?;
        Ok(Self { serie, emetteur_cog, emetteur_kid, sujet, destination, cle_appareil, role, portees, pas_avant, pas_apres })
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.pas_avant, self.pas_apres))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Visa (doc §6)
// ═══════════════════════════════════════════════════════════════════════════════

mod tag_visa {
    pub const SERIE: u8 = 0x01;
    pub const PASSEPORT: u8 = 0x02;
    pub const EMETTEUR_COG: u8 = 0x03;
    pub const SUJET: u8 = 0x04;
    pub const CLE_APPAREIL: u8 = 0x05;
    pub const AUDIENCE: u8 = 0x06;
    pub const ROLE: u8 = 0x07;
    pub const PORTEES: u8 = 0x08;
    pub const PAS_AVANT: u8 = 0x09;
    pub const PAS_APRES: u8 = 0x0A;
}

/// Visa : droit d'entrée ciblé, émis par le COG **hôte** sur présentation d'un passeport.
///
/// Lié indissociablement au passeport (`passeport` = SHA-256 de ses octets signés),
/// au sujet par paire et à la clé d'appareil du titulaire (redondance volontaire :
/// chaque lien est contrôlé séparément).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Visa {
    /// Numéro de série aléatoire (128 bits).
    pub serie: Serie,
    /// SHA-256 des octets signés du passeport présenté.
    pub passeport: [u8; 32],
    /// `cog_id` du COG hôte (émetteur du visa).
    pub emetteur_cog: CogId,
    /// Sujet par paire, recopié du passeport.
    pub sujet: [u8; 32],
    /// Clé d'appareil, recopiée du passeport.
    pub cle_appareil: [u8; 32],
    /// Service visé.
    pub audience: String,
    /// `Citizen` si le passeport vient du COG hôte, `Visitor` sinon.
    pub role: IdentityRole,
    /// Droits restreints au service.
    pub portees: Portees,
    /// Début de validité (secondes Unix).
    pub pas_avant: u64,
    /// Fin de validité, au plus 15 minutes après `pas_avant`.
    pub pas_apres: u64,
}

impl Titre for Visa {
    const TYPE: TypeSceau = TypeSceau::Visa;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        use tag_visa::*;
        if !audience_valide(&self.audience) {
            return Err(ErreurSceau::ValeurInvalide(AUDIENCE));
        }
        controler_fenetre(PAS_APRES, self.pas_avant, self.pas_apres, DUREE_MAX_VISA_SECS)?;
        e.octets(SERIE, &self.serie)?
            .b32(PASSEPORT, &self.passeport)?
            .texte(EMETTEUR_COG, self.emetteur_cog.as_str())?
            .b32(SUJET, &self.sujet)?
            .b32(CLE_APPAREIL, &self.cle_appareil)?
            .texte(AUDIENCE, &self.audience)?
            .u64(ROLE, role_vers_code(self.role)?)?
            .octets(PORTEES, &self.portees.encoder()?)?
            .u64(PAS_AVANT, self.pas_avant)?
            .u64(PAS_APRES, self.pas_apres)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        use tag_visa::*;
        let serie = lire_serie(l, SERIE)?;
        let passeport = l.exiger_b32(PASSEPORT)?;
        let emetteur_cog = lire_cog(l, EMETTEUR_COG)?;
        let sujet = l.exiger_b32(SUJET)?;
        let cle_appareil = l.exiger_b32(CLE_APPAREIL)?;
        let audience = l.exiger_texte(AUDIENCE)?;
        if !audience_valide(audience) {
            return Err(ErreurSceau::ValeurInvalide(AUDIENCE));
        }
        let audience = audience.to_string();
        let role = code_vers_role(ROLE, l.exiger_u64(ROLE)?)?;
        let portees = Portees::decoder(PORTEES, l.exiger(PORTEES)?)?;
        let pas_avant = l.exiger_u64(PAS_AVANT)?;
        let pas_apres = l.exiger_u64(PAS_APRES)?;
        controler_fenetre(PAS_APRES, pas_avant, pas_apres, DUREE_MAX_VISA_SECS)?;
        Ok(Self { serie, passeport, emetteur_cog, sujet, cle_appareil, audience, role, portees, pas_avant, pas_apres })
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.pas_avant, self.pas_apres))
    }
}
