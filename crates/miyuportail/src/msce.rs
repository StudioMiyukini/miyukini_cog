//! @id     miyuportail_msce
//! @do     coder_decoder_declaration_msce
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — trame MSCE « déclaration de voie » (Sceau v1.1, type 0x08), décodeur n° 1 strict et format interne réencodé.

//! # Trame MSCE « déclaration de voie »
//!
//! Seul protocole admis sur une voie du Portail (MODELE-ZONES §5 : liste
//! blanche `MSCE`, Sceau v1). Disposition **identique au Sceau v1.1**
//! (lot 2 §4.1), avec un type dédié :
//!
//! ```text
//! "MSCE" | 0x02 | alg 0x01 | type 0x08 | len u32 BE (≤ 65 536) | charge
//! | nombre (1..=8) | nombre × [ kid (32) | alg (1) | sig_len u16 = 64 | sig (64) ]
//! ```
//!
//! Octets signés : `"miyukini/sceau/v1.1\0" || 0x02 || alg || 0x08 || len || charge`
//! (même domaine que le Sceau ; le type signé sépare les déclarations des
//! titres d'identité, que `miyusceau` refuse de toute façon : type inconnu).
//!
//! Charge TLV canonique (`tag u8 | len u32 BE | valeur`, tags strictement
//! croissants, tous obligatoires, aucun autre) :
//!
//! | tag | champ | forme |
//! |---|---|---|
//! | 1 | service | identifiant `[a-z0-9._:-]`, 1..64 o |
//! | 2 | sens | 1 o : `0x01` entrante, `0x02` sortante |
//! | 3 | version du traité | u64 |
//! | 4 | type de déclaration | u64 |
//! | 5 | séquence | u64 (strictement croissante par session) |
//! | 6 | émise le | u64 (secondes Unix) |
//! | 7 | champs | TLV imbriqué, schéma du traité |
//!
//! Signatures (politique du Portail) : voie entrante = **exactement deux**,
//! la façade (clé de session) **et** un auteur épinglé ; voie sortante =
//! **exactement une**, la clé de publication de la voie.
//!
//! **Rupture protocolaire** : aucun octet de la trame ne traverse. La trame
//! est décodée en [`Declaration`] typée puis **réencodée** dans le format
//! interne (`MPI1`, [`encoder_interne`]) côté zone COG, ou dans une trame
//! neuve signée par la voie côté publication.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

use crate::erreurs::RefusDecodage;
use crate::traite::{IdService, NatureChamp, Sens, Traite, TAILLE_MAX_CHARGE};

/// Magic du Sceau (liste blanche de protocole).
pub const MAGIC: &[u8; 4] = b"MSCE";
/// Version Sceau v1.1.
pub const VERSION: u8 = 0x02;
/// Algorithme Ed25519 (seul implémenté).
pub const ALG_ED25519: u8 = 0x01;
/// Type « déclaration de voie » (à inscrire au tableau du Sceau, amendement A-004 proposé).
pub const TYPE_DECLARATION: u8 = 0x08;
/// Domaine de signature (identique au Sceau v1.1).
pub const DOMAINE: &[u8] = b"miyukini/sceau/v1.1\0";
/// Taille de l'entête (magic, version, alg, type, len).
pub const TAILLE_ENTETE: usize = 4 + 1 + 1 + 1 + 4;
/// Taille d'une entrée de signature Ed25519 (kid, alg, sig_len, sig).
pub const TAILLE_ENTREE_SIGNATURE: usize = 32 + 1 + 2 + 64;
/// Nombre maximal d'entrées de signature (structure Sceau).
pub const MAX_SIGNATURES: usize = 8;
/// Taille maximale d'une trame.
pub const TAILLE_MAX_TRAME: usize = TAILLE_ENTETE + TAILLE_MAX_CHARGE + 1 + MAX_SIGNATURES * TAILLE_ENTREE_SIGNATURE;

/// Magic du format interne réencodé.
pub const MAGIC_INTERNE: &[u8; 4] = b"MPI1";

/// Identifiant de clé : SHA-256 de la clé publique (comme le Sceau).
pub type Kid = [u8; 32];

/// Valeur typée d'un champ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Valeur {
    /// Entier.
    Entier(u64),
    /// Booléen.
    Booleen(bool),
    /// Texte contrôlé.
    Texte(String),
    /// Octets opaques typés.
    Octets(Vec<u8>),
    /// Document : désarmé (CDR) avant toute remise au moteur.
    Document(Vec<u8>),
}

impl Valeur {
    fn code(&self) -> u8 {
        match self {
            Self::Entier(_) => 0x01,
            Self::Booleen(_) => 0x02,
            Self::Texte(_) => 0x03,
            Self::Octets(_) => 0x04,
            Self::Document(_) => 0x05,
        }
    }

    fn octets(&self) -> Vec<u8> {
        match self {
            Self::Entier(v) => v.to_be_bytes().to_vec(),
            Self::Booleen(b) => vec![u8::from(*b)],
            Self::Texte(t) => t.as_bytes().to_vec(),
            Self::Octets(o) | Self::Document(o) => o.clone(),
        }
    }
}

/// Champ d'une déclaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Champ {
    /// Tag défini par le traité.
    pub tag: u8,
    /// Valeur typée.
    pub valeur: Valeur,
}

/// Déclaration typée (« parse, don't validate »).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    /// Service de la voie.
    pub service: IdService,
    /// Sens.
    pub sens: Sens,
    /// Version du traité.
    pub version_traite: u64,
    /// Code du type de déclaration.
    pub type_code: u64,
    /// Numéro de séquence (anti-rejeu).
    pub sequence: u64,
    /// Date d'émission (secondes Unix, déclarative).
    pub emise_le: u64,
    /// Champs, tags strictement croissants.
    pub champs: Vec<Champ>,
    /// `kid` des signataires vérifiés, strictement croissants (vide avant scellement).
    pub signataires: Vec<Kid>,
}

/// Ancres de vérification d'une voie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ancres {
    /// Émetteur : clé de session de la façade (entrante) ou clé de publication de la voie (sortante).
    pub emetteur: [u8; 32],
    /// Auteurs (participants) épinglés ; vide = un seul signataire attendu (l'émetteur).
    pub auteurs: Vec<[u8; 32]>,
}

/// Contexte de décodage attendu par la voie.
#[derive(Debug, Clone, Copy)]
pub struct Contexte<'a> {
    /// Service de la voie.
    pub service: &'a IdService,
    /// Sens de la voie.
    pub sens: Sens,
    /// Traité du service.
    pub traite: &'a Traite,
    /// Ancres.
    pub ancres: &'a Ancres,
}

/// `kid` d'une clé publique.
pub fn kid(cle_publique: &[u8; 32]) -> Kid {
    miyusceau::empreinte_cle(cle_publique)
}

/// Octets signés (domaine Sceau v1.1, type 0x08).
pub fn octets_signes(charge: &[u8]) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE.len() + 7 + charge.len());
    m.extend_from_slice(DOMAINE);
    m.push(VERSION);
    m.push(ALG_ED25519);
    m.push(TYPE_DECLARATION);
    m.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    m.extend_from_slice(charge);
    m
}

fn tlv(sortie: &mut Vec<u8>, tag: u8, valeur: &[u8]) {
    sortie.push(tag);
    sortie.extend_from_slice(&(valeur.len() as u32).to_be_bytes());
    sortie.extend_from_slice(valeur);
}

/// Charge TLV canonique d'une déclaration (les signataires sont ignorés).
pub fn encoder_charge(d: &Declaration) -> Result<Vec<u8>, RefusDecodage> {
    let mut champs = Vec::new();
    let mut precedent = 0u8;
    for c in &d.champs {
        if c.tag == 0 || c.tag <= precedent {
            return Err(RefusDecodage::Tlv);
        }
        precedent = c.tag;
        tlv(&mut champs, c.tag, &c.valeur.octets());
    }
    let mut s = Vec::new();
    tlv(&mut s, 1, d.service.as_str().as_bytes());
    tlv(&mut s, 2, &[d.sens as u8]);
    tlv(&mut s, 3, &d.version_traite.to_be_bytes());
    tlv(&mut s, 4, &d.type_code.to_be_bytes());
    tlv(&mut s, 5, &d.sequence.to_be_bytes());
    tlv(&mut s, 6, &d.emise_le.to_be_bytes());
    tlv(&mut s, 7, &champs);
    if s.len() > TAILLE_MAX_CHARGE {
        return Err(RefusDecodage::Taille);
    }
    Ok(s)
}

/// Scelle une déclaration : trame MSCE signée par chaque clé (1 à 8, distinctes).
pub fn sceller(d: &Declaration, cles: &[&SigningKey]) -> Result<Vec<u8>, RefusDecodage> {
    if cles.is_empty() || cles.len() > MAX_SIGNATURES {
        return Err(RefusDecodage::Signataires);
    }
    let charge = encoder_charge(d)?;
    let message = octets_signes(&charge);
    let mut entrees: Vec<(Kid, [u8; 64])> =
        cles.iter().map(|c| (kid(&c.verifying_key().to_bytes()), c.sign(&message).to_bytes())).collect();
    entrees.sort_by_key(|e| e.0);
    if entrees.windows(2).any(|p| p[0].0 == p[1].0) {
        return Err(RefusDecodage::Signataires);
    }
    let mut s = Vec::with_capacity(TAILLE_ENTETE + charge.len() + 1 + entrees.len() * TAILLE_ENTREE_SIGNATURE);
    s.extend_from_slice(MAGIC);
    s.push(VERSION);
    s.push(ALG_ED25519);
    s.push(TYPE_DECLARATION);
    s.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    s.extend_from_slice(&charge);
    s.push(entrees.len() as u8);
    for (k, sig) in &entrees {
        s.extend_from_slice(k);
        s.push(ALG_ED25519);
        s.extend_from_slice(&64u16.to_be_bytes());
        s.extend_from_slice(sig);
    }
    Ok(s)
}

/// Lecteur à position (décodeur n° 1).
struct Lecteur<'a> {
    octets: &'a [u8],
    pos: usize,
}

impl<'a> Lecteur<'a> {
    fn lire(&mut self, n: usize) -> Result<&'a [u8], RefusDecodage> {
        let fin = self.pos.checked_add(n).ok_or(RefusDecodage::Taille)?;
        let t = self.octets.get(self.pos..fin).ok_or(RefusDecodage::Taille)?;
        self.pos = fin;
        Ok(t)
    }

    fn octet(&mut self) -> Result<u8, RefusDecodage> {
        Ok(self.lire(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, RefusDecodage> {
        let o = self.lire(2)?;
        Ok(u16::from_be_bytes([o[0], o[1]]))
    }

    fn u32(&mut self) -> Result<u32, RefusDecodage> {
        let o = self.lire(4)?;
        Ok(u32::from_be_bytes([o[0], o[1], o[2], o[3]]))
    }

    fn termine(&self) -> bool {
        self.pos == self.octets.len()
    }
}

fn u64_exact(o: &[u8]) -> Result<u64, RefusDecodage> {
    let t: [u8; 8] = o.try_into().map_err(|_| RefusDecodage::Valeur)?;
    Ok(u64::from_be_bytes(t))
}

/// Texte admis : UTF-8 valide, aucun caractère de contrôle hors `\t` et `\n`.
fn texte_admis(o: &[u8]) -> Result<String, RefusDecodage> {
    let s = std::str::from_utf8(o).map_err(|_| RefusDecodage::Valeur)?;
    if s.chars().any(|c| c.is_control() && c != '\t' && c != '\n') {
        return Err(RefusDecodage::Valeur);
    }
    Ok(s.to_owned())
}

/// Lit un TLV canonique complet (tags strictement croissants).
fn lire_tlv(octets: &[u8]) -> Result<Vec<(u8, &[u8])>, RefusDecodage> {
    let mut l = Lecteur { octets, pos: 0 };
    let mut sortie = Vec::new();
    let mut precedent: Option<u8> = None;
    while !l.termine() {
        let tag = l.octet().map_err(|_| RefusDecodage::Tlv)?;
        if precedent.is_some_and(|p| tag <= p) {
            return Err(RefusDecodage::Tlv);
        }
        precedent = Some(tag);
        let n = l.u32().map_err(|_| RefusDecodage::Tlv)? as usize;
        let v = l.lire(n).map_err(|_| RefusDecodage::Tlv)?;
        sortie.push((tag, v));
    }
    Ok(sortie)
}

/// Contrôle et vérifie le bloc de signatures selon la politique de la voie.
fn verifier_signatures(
    message: &[u8],
    signatures: &[(Kid, &[u8])],
    ancres: &Ancres,
) -> Result<Vec<Kid>, RefusDecodage> {
    let attendu = if ancres.auteurs.is_empty() { 1 } else { 2 };
    if signatures.len() != attendu {
        return Err(RefusDecodage::Signataires);
    }
    let kid_emetteur = kid(&ancres.emetteur);
    let mut emetteur_vu = false;
    let mut auteur_vu = false;
    for (k, sig) in signatures {
        let cle = if *k == kid_emetteur {
            emetteur_vu = true;
            ancres.emetteur
        } else if let Some(a) = ancres.auteurs.iter().find(|a| kid(a) == *k) {
            auteur_vu = true;
            *a
        } else {
            return Err(RefusDecodage::Signataires);
        };
        let vk = VerifyingKey::from_bytes(&cle).map_err(|_| RefusDecodage::Signataires)?;
        let s = Signature::from_slice(sig).map_err(|_| RefusDecodage::Signature)?;
        vk.verify_strict(message, &s).map_err(|_| RefusDecodage::Signature)?;
    }
    if !emetteur_vu || (attendu == 2 && !auteur_vu) {
        return Err(RefusDecodage::Signataires);
    }
    Ok(signatures.iter().map(|(k, _)| *k).collect())
}

/// Décode les champs imbriqués selon le schéma du type.
fn decoder_champs(octets: &[u8], ty: &crate::traite::TypeDeclaration) -> Result<Vec<Champ>, RefusDecodage> {
    let mut champs = Vec::new();
    for (tag, v) in lire_tlv(octets)? {
        let schema = ty.champ(tag).ok_or(RefusDecodage::Champ)?;
        let valeur = match schema.nature {
            NatureChamp::Entier => Valeur::Entier(u64_exact(v)?),
            NatureChamp::Booleen => match v {
                [0] => Valeur::Booleen(false),
                [1] => Valeur::Booleen(true),
                _ => return Err(RefusDecodage::Valeur),
            },
            NatureChamp::Texte { max } => {
                if v.len() > max as usize {
                    return Err(RefusDecodage::Champ);
                }
                Valeur::Texte(texte_admis(v)?)
            }
            NatureChamp::Octets { max } => {
                if v.len() > max as usize {
                    return Err(RefusDecodage::Champ);
                }
                Valeur::Octets(v.to_vec())
            }
            NatureChamp::Document { max, .. } => {
                if v.len() > max as usize {
                    return Err(RefusDecodage::Champ);
                }
                Valeur::Document(v.to_vec())
            }
        };
        champs.push(Champ { tag, valeur });
    }
    for c in ty.champs.iter().filter(|c| c.obligatoire) {
        if !champs.iter().any(|d| d.tag == c.tag) {
            return Err(RefusDecodage::Champ);
        }
    }
    Ok(champs)
}

/// **Décodeur n° 1** (exécuté dans le sas) : cadre strict, puis signatures
/// vérifiées **avant** tout décodage de la charge (EX-09, C-23), puis charge
/// et champs selon le traité.
pub fn decoder_trame(octets: &[u8], ctx: &Contexte<'_>) -> Result<Declaration, RefusDecodage> {
    if octets.len() > TAILLE_MAX_TRAME {
        return Err(RefusDecodage::Taille);
    }
    let mut l = Lecteur { octets, pos: 0 };
    if l.lire(4)? != MAGIC {
        return Err(RefusDecodage::Magic);
    }
    if l.octet()? != VERSION {
        return Err(RefusDecodage::Version);
    }
    if l.octet()? != ALG_ED25519 {
        return Err(RefusDecodage::Algorithme);
    }
    if l.octet()? != TYPE_DECLARATION {
        return Err(RefusDecodage::TypeTrame);
    }
    let len = l.u32()? as usize;
    if len > TAILLE_MAX_CHARGE {
        return Err(RefusDecodage::Taille);
    }
    let charge = l.lire(len)?;
    let nombre = usize::from(l.octet()?);
    if nombre == 0 || nombre > MAX_SIGNATURES {
        return Err(RefusDecodage::Signataires);
    }
    let mut signatures: Vec<(Kid, &[u8])> = Vec::with_capacity(nombre);
    for _ in 0..nombre {
        let k: Kid = l.lire(32)?.try_into().map_err(|_| RefusDecodage::Taille)?;
        if l.octet()? != ALG_ED25519 {
            return Err(RefusDecodage::Algorithme);
        }
        if l.u16()? != 64 {
            return Err(RefusDecodage::Taille);
        }
        let sig = l.lire(64)?;
        if signatures.last().is_some_and(|(p, _)| *p >= k) {
            return Err(RefusDecodage::Signataires);
        }
        signatures.push((k, sig));
    }
    if !l.termine() {
        return Err(RefusDecodage::Taille);
    }

    // Signatures d'abord.
    let signataires = verifier_signatures(&octets_signes(charge), &signatures, ctx.ancres)?;

    // Puis la charge.
    let tlv = lire_tlv(charge)?;
    let tags: Vec<u8> = tlv.iter().map(|(t, _)| *t).collect();
    if tags != [1, 2, 3, 4, 5, 6, 7] {
        return Err(RefusDecodage::Tlv);
    }
    let service_octets = tlv[0].1;
    if !IdService::valide(service_octets) {
        return Err(RefusDecodage::Valeur);
    }
    let service = IdService::new(std::str::from_utf8(service_octets).map_err(|_| RefusDecodage::Valeur)?)
        .map_err(|_| RefusDecodage::Valeur)?;
    if &service != ctx.service {
        return Err(RefusDecodage::Service);
    }
    let sens = match tlv[1].1 {
        [o] => Sens::depuis_octet(*o).ok_or(RefusDecodage::Valeur)?,
        _ => return Err(RefusDecodage::Valeur),
    };
    if sens != ctx.sens {
        return Err(RefusDecodage::Sens);
    }
    let version_traite = u64_exact(tlv[2].1)?;
    if version_traite != ctx.traite.version {
        return Err(RefusDecodage::VersionTraite);
    }
    let type_code = u64_exact(tlv[3].1)?;
    let ty = ctx.traite.type_pour(sens, type_code).ok_or(RefusDecodage::TypeDeclaration)?;
    let sequence = u64_exact(tlv[4].1)?;
    let emise_le = u64_exact(tlv[5].1)?;
    let champs = decoder_champs(tlv[6].1, ty)?;
    Ok(Declaration { service, sens, version_traite, type_code, sequence, emise_le, champs, signataires })
}

/// Valide une déclaration construite en zone COG contre le traité (voie sortante).
pub fn conforme_au_traite(d: &Declaration, traite: &Traite) -> Result<(), RefusDecodage> {
    if d.version_traite != traite.version {
        return Err(RefusDecodage::VersionTraite);
    }
    let ty = traite.type_pour(d.sens, d.type_code).ok_or(RefusDecodage::TypeDeclaration)?;
    // Aller-retour par le décodeur des champs : même règle qu'à l'entrée.
    let charge = encoder_charge(d)?;
    let tlv = lire_tlv(&charge)?;
    let champs = decoder_champs(tlv.get(6).ok_or(RefusDecodage::Tlv)?.1, ty)?;
    if champs != d.champs {
        return Err(RefusDecodage::Champ);
    }
    Ok(())
}

/// Encode le **format interne** réencodé (sortie du sas, entrée du moteur).
///
/// `"MPI1" | ls u8 | service | sens u8 | version u64 | type u64 | séquence u64
/// | émise u64 | ns u8 | ns × kid | nc u8 | nc × [tag u8 | nature u8 | len u32 | valeur]`
pub fn encoder_interne(d: &Declaration) -> Vec<u8> {
    let mut s = Vec::new();
    s.extend_from_slice(MAGIC_INTERNE);
    s.push(d.service.as_str().len() as u8);
    s.extend_from_slice(d.service.as_str().as_bytes());
    s.push(d.sens as u8);
    s.extend_from_slice(&d.version_traite.to_be_bytes());
    s.extend_from_slice(&d.type_code.to_be_bytes());
    s.extend_from_slice(&d.sequence.to_be_bytes());
    s.extend_from_slice(&d.emise_le.to_be_bytes());
    s.push(d.signataires.len() as u8);
    for k in &d.signataires {
        s.extend_from_slice(k);
    }
    s.push(d.champs.len() as u8);
    for c in &d.champs {
        let v = c.valeur.octets();
        s.push(c.tag);
        s.push(c.valeur.code());
        s.extend_from_slice(&(v.len() as u32).to_be_bytes());
        s.extend_from_slice(&v);
    }
    s
}

/// Décode strictement le format interne (contrôle de forme uniquement : la
/// conformité est établie par la comparaison avec la deuxième validation).
pub fn decoder_interne(octets: &[u8]) -> Result<Declaration, RefusDecodage> {
    if octets.len() > TAILLE_MAX_TRAME {
        return Err(RefusDecodage::Taille);
    }
    let mut l = Lecteur { octets, pos: 0 };
    if l.lire(4)? != MAGIC_INTERNE {
        return Err(RefusDecodage::Magic);
    }
    let ls = usize::from(l.octet()?);
    let so = l.lire(ls)?;
    if !IdService::valide(so) {
        return Err(RefusDecodage::Valeur);
    }
    let service = IdService::new(std::str::from_utf8(so).map_err(|_| RefusDecodage::Valeur)?)
        .map_err(|_| RefusDecodage::Valeur)?;
    let sens = Sens::depuis_octet(l.octet()?).ok_or(RefusDecodage::Valeur)?;
    let version_traite = u64_exact(l.lire(8)?)?;
    let type_code = u64_exact(l.lire(8)?)?;
    let sequence = u64_exact(l.lire(8)?)?;
    let emise_le = u64_exact(l.lire(8)?)?;
    let ns = usize::from(l.octet()?);
    if ns > MAX_SIGNATURES {
        return Err(RefusDecodage::Signataires);
    }
    let mut signataires = Vec::with_capacity(ns);
    for _ in 0..ns {
        let k: Kid = l.lire(32)?.try_into().map_err(|_| RefusDecodage::Taille)?;
        signataires.push(k);
    }
    let nc = usize::from(l.octet()?);
    let mut champs = Vec::with_capacity(nc);
    for _ in 0..nc {
        let tag = l.octet()?;
        let nature = l.octet()?;
        let n = l.u32()? as usize;
        let v = l.lire(n)?;
        let valeur = match nature {
            0x01 => Valeur::Entier(u64_exact(v)?),
            0x02 => match v {
                [0] => Valeur::Booleen(false),
                [1] => Valeur::Booleen(true),
                _ => return Err(RefusDecodage::Valeur),
            },
            0x03 => Valeur::Texte(texte_admis(v)?),
            0x04 => Valeur::Octets(v.to_vec()),
            0x05 => Valeur::Document(v.to_vec()),
            _ => return Err(RefusDecodage::Valeur),
        };
        champs.push(Champ { tag, valeur });
    }
    if !l.termine() {
        return Err(RefusDecodage::Taille);
    }
    Ok(Declaration { service, sens, version_traite, type_code, sequence, emise_le, champs, signataires })
}
