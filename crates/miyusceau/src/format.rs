//! @id     miyusceau_format
//! @do     sceller_verifier_sceau_v1_1
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « format ».

//! Format binaire « Sceau v1.1 » (big-endian, amendement A-002) :
//!
//! ```text
//! magic "MSCE" (4) | version 0x02 (1) | alg (1) | type (1) | len (4) | charge (len)
//! | nombre (1, 1..=8) | nombre × [ kid (32) | alg (1) | sig_len (2) | sig (sig_len) ]
//! ```
//!
//! Octets signés (communs à tous les signataires) :
//! `"miyukini/sceau/v1.1" || 0x00 || version || alg || type || len || charge`.
//!
//! - `kid` strictement croissants (ni désordre ni doublon)
//! - `alg` de chaque entrée = `alg` de l'entête ; `sig_len` imposée par l'algorithme
//! - Sceau v1.0 (version `0x01`) abandonné : refusé, sans voie héritée

use ed25519_dalek::Signature;

use crate::ancres::{AncresDeConfiance, NiveauConfiance};
use crate::cles::{CleSignature, Kid};
use crate::erreurs::ErreurSceau;
use crate::titre::{Titre, Verifie};
use crate::tlv::{EcrivainTlv, LecteurTlv};

const MAGIC: &[u8; 4] = b"MSCE";
/// Octet de version du format Sceau v1.1.
pub const VERSION: u8 = 0x02;
const DOMAINE: &[u8] = b"miyukini/sceau/v1.1\0";
/// magic + version + alg + type + len.
const TAILLE_ENTETE: usize = 4 + 1 + 1 + 1 + 4;
/// kid + alg + sig_len.
const TAILLE_ENTETE_SIGNATURE: usize = 32 + 1 + 2;

/// Taille maximale de la charge utile (64 Kio).
pub const TAILLE_MAX_CHARGE: usize = 65_536;

/// Nombre maximal de signatures dans un sceau.
pub const MAX_SIGNATURES: usize = 8;

/// Taille maximale d'un sceau encodé (charge maximale, 8 signatures de la
/// taille la plus grande parmi les algorithmes connus).
pub const TAILLE_MAX_SCEAU: usize = TAILLE_ENTETE
    + TAILLE_MAX_CHARGE
    + 1
    + MAX_SIGNATURES * (TAILLE_ENTETE_SIGNATURE + Algorithme::HybrideEd25519MlDsa65.taille_signature());

/// Tolérance d'horloge appliquée aux fenêtres de validité (secondes).
pub const TOLERANCE_HORLOGE_SECS: u64 = 120;

/// Types de titres (séparation de domaine).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeSceau {
    /// Passeport (personne + appareil).
    Passeport = 0x01,
    /// Visa (droit d'entrée ciblé).
    Visa = 0x02,
    /// Manifeste de paquet.
    ManifestePaquet = 0x03,
    /// Entrée du registre.
    EntreeRegistre = 0x04,
    /// Tête signée du registre.
    TeteRegistre = 0x05,
    /// Certificat (clé d'émission, clé éditeur).
    Certificat = 0x06,
    /// Reconnaissance entre COG.
    Reconnaissance = 0x07,
}

impl TryFrom<u8> for TypeSceau {
    type Error = ErreurSceau;

    fn try_from(v: u8) -> Result<Self, ErreurSceau> {
        Ok(match v {
            0x01 => Self::Passeport,
            0x02 => Self::Visa,
            0x03 => Self::ManifestePaquet,
            0x04 => Self::EntreeRegistre,
            0x05 => Self::TeteRegistre,
            0x06 => Self::Certificat,
            0x07 => Self::Reconnaissance,
            autre => return Err(ErreurSceau::TypeInconnu(autre)),
        })
    }
}

/// Algorithme de signature (identifiant signé, amendement A-002).
///
/// Seul Ed25519 est implémenté ; les identifiants post-quantiques sont
/// réservés et **refusés** à la vérification (fail-closed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Algorithme {
    /// Ed25519, vérifié avec `verify_strict`.
    Ed25519 = 0x01,
    /// ML-DSA-65 (FIPS 204) — réservé, refusé.
    MlDsa65 = 0x02,
    /// Hybride Ed25519 ‖ ML-DSA-65 — réservé, refusé.
    HybrideEd25519MlDsa65 = 0x03,
}

impl Algorithme {
    /// Longueur exacte d'une signature pour cet algorithme.
    pub const fn taille_signature(self) -> usize {
        match self {
            Self::Ed25519 => 64,
            Self::MlDsa65 => 3_309,
            Self::HybrideEd25519MlDsa65 => 64 + 3_309,
        }
    }

    /// Vrai si la vérification est implémentée.
    pub const fn est_implemente(self) -> bool {
        matches!(self, Self::Ed25519)
    }
}

impl TryFrom<u8> for Algorithme {
    type Error = ErreurSceau;

    fn try_from(v: u8) -> Result<Self, ErreurSceau> {
        Ok(match v {
            0x01 => Self::Ed25519,
            0x02 => Self::MlDsa65,
            0x03 => Self::HybrideEd25519MlDsa65,
            autre => return Err(ErreurSceau::AlgorithmeInconnu(autre)),
        })
    }
}

/// Une entrée du bloc de signatures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureSceau {
    /// `kid` du signataire.
    pub kid: Kid,
    /// Algorithme (égal à celui de l'entête).
    pub algorithme: Algorithme,
    /// Octets de la signature (longueur imposée par l'algorithme).
    pub octets: Vec<u8>,
}

/// Sceau décodé (signatures non vérifiées : usage interne et outillage).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sceau {
    /// Algorithme de signature du sceau.
    pub algorithme: Algorithme,
    /// Type du titre.
    pub type_sceau: TypeSceau,
    /// Charge utile TLV.
    pub charge: Vec<u8>,
    /// Signatures, `kid` strictement croissants.
    pub signatures: Vec<SignatureSceau>,
}

/// Lecture bornée : renvoie `n` octets à partir de `*pos`, ou `TailleInvalide`.
fn lire<'a>(octets: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8], ErreurSceau> {
    let fin = pos.checked_add(n).ok_or(ErreurSceau::TailleInvalide)?;
    let tranche = octets.get(*pos..fin).ok_or(ErreurSceau::TailleInvalide)?;
    *pos = fin;
    Ok(tranche)
}

/// Contrôle l'ordre canonique des signatures : `kid` strictement croissants.
fn controler_ordre(signatures: &[SignatureSceau]) -> Result<(), ErreurSceau> {
    for paire in signatures.windows(2) {
        match paire[0].kid.cmp(&paire[1].kid) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => return Err(ErreurSceau::SignataireDuplique),
            std::cmp::Ordering::Greater => return Err(ErreurSceau::SignaturesNonTriees),
        }
    }
    Ok(())
}

impl Sceau {
    /// Décode strictement des octets : taille exacte, entête, algorithme et
    /// type connus, 1 à 8 signatures bien formées, `kid` strictement croissants.
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurSceau> {
        if octets.len() > TAILLE_MAX_SCEAU {
            return Err(ErreurSceau::TailleInvalide);
        }
        let mut pos = 0usize;
        if lire(octets, &mut pos, 4)? != MAGIC {
            return Err(ErreurSceau::MagicInvalide);
        }
        let version = lire(octets, &mut pos, 1)?[0];
        if version != VERSION {
            return Err(ErreurSceau::VersionNonSupportee(version));
        }
        let algorithme = Algorithme::try_from(lire(octets, &mut pos, 1)?[0])?;
        let type_sceau = TypeSceau::try_from(lire(octets, &mut pos, 1)?[0])?;
        let longueur = lire(octets, &mut pos, 4)?;
        let longueur = u32::from_be_bytes([longueur[0], longueur[1], longueur[2], longueur[3]]) as usize;
        if longueur > TAILLE_MAX_CHARGE {
            return Err(ErreurSceau::ChargeTropGrande);
        }
        let charge = lire(octets, &mut pos, longueur)?.to_vec();

        let nombre = lire(octets, &mut pos, 1)?[0];
        if nombre == 0 || usize::from(nombre) > MAX_SIGNATURES {
            return Err(ErreurSceau::NombreSignaturesInvalide(nombre));
        }
        let mut signatures = Vec::with_capacity(usize::from(nombre));
        for _ in 0..nombre {
            let mut kid = [0u8; 32];
            kid.copy_from_slice(lire(octets, &mut pos, 32)?);
            let alg_entree = Algorithme::try_from(lire(octets, &mut pos, 1)?[0])?;
            if alg_entree != algorithme {
                return Err(ErreurSceau::AlgorithmeIncoherent);
            }
            let sig_len = lire(octets, &mut pos, 2)?;
            let sig_len = usize::from(u16::from_be_bytes([sig_len[0], sig_len[1]]));
            if sig_len != algorithme.taille_signature() {
                return Err(ErreurSceau::TailleInvalide);
            }
            let sig = lire(octets, &mut pos, sig_len)?.to_vec();
            signatures.push(SignatureSceau { kid, algorithme: alg_entree, octets: sig });
        }
        if pos != octets.len() {
            return Err(ErreurSceau::TailleInvalide);
        }
        controler_ordre(&signatures)?;
        Ok(Self { algorithme, type_sceau, charge, signatures })
    }

    /// Encode en octets (aucun contrôle : outillage et tests ; les sceaux
    /// produits par [`sceller`] et [`cosigner`] sont toujours canoniques).
    pub fn encoder(&self) -> Vec<u8> {
        let taille_sigs: usize =
            self.signatures.iter().map(|s| TAILLE_ENTETE_SIGNATURE + s.octets.len()).sum();
        let mut sortie = Vec::with_capacity(TAILLE_ENTETE + self.charge.len() + 1 + taille_sigs);
        sortie.extend_from_slice(MAGIC);
        sortie.push(VERSION);
        sortie.push(self.algorithme as u8);
        sortie.push(self.type_sceau as u8);
        sortie.extend_from_slice(&(self.charge.len() as u32).to_be_bytes());
        sortie.extend_from_slice(&self.charge);
        sortie.push(self.signatures.len() as u8);
        for s in &self.signatures {
            sortie.extend_from_slice(&s.kid);
            sortie.push(s.algorithme as u8);
            sortie.extend_from_slice(&(s.octets.len() as u16).to_be_bytes());
            sortie.extend_from_slice(&s.octets);
        }
        sortie
    }

    /// Octets couverts par chaque signature (identiques pour tous les
    /// signataires). Leur SHA-256 identifie le titre indépendamment du bloc
    /// de signatures.
    pub fn octets_signes(&self) -> Vec<u8> {
        Self::octets_signes_de(self.algorithme, self.type_sceau, &self.charge)
    }

    fn octets_signes_de(algorithme: Algorithme, type_sceau: TypeSceau, charge: &[u8]) -> Vec<u8> {
        let mut m = Vec::with_capacity(DOMAINE.len() + 3 + 4 + charge.len());
        m.extend_from_slice(DOMAINE);
        m.push(VERSION);
        m.push(algorithme as u8);
        m.push(type_sceau as u8);
        m.extend_from_slice(&(charge.len() as u32).to_be_bytes());
        m.extend_from_slice(charge);
        m
    }

    /// Ajoute une signature Ed25519 en respectant l'ordre canonique.
    fn ajouter_signature(&mut self, cle: &CleSignature) -> Result<(), ErreurSceau> {
        if self.algorithme != Algorithme::Ed25519 {
            return Err(ErreurSceau::AlgorithmeNonPrisEnCharge(self.algorithme as u8));
        }
        if self.signatures.len() >= MAX_SIGNATURES {
            return Err(ErreurSceau::NombreSignaturesInvalide((MAX_SIGNATURES + 1) as u8));
        }
        let kid = cle.kid();
        let Err(position) = self.signatures.binary_search_by(|s| s.kid.cmp(&kid)) else {
            return Err(ErreurSceau::SignataireDuplique);
        };
        let octets = cle.signer(&self.octets_signes()).to_vec();
        self.signatures.insert(position, SignatureSceau { kid, algorithme: Algorithme::Ed25519, octets });
        Ok(())
    }
}

fn encoder_charge<T: Titre>(titre: &T) -> Result<Vec<u8>, ErreurSceau> {
    let mut ecrivain = EcrivainTlv::new();
    titre.encoder(&mut ecrivain)?;
    let charge = ecrivain.finir();
    if charge.len() > TAILLE_MAX_CHARGE {
        return Err(ErreurSceau::ChargeTropGrande);
    }
    Ok(charge)
}

/// Scelle un titre avec une clé de signature (Ed25519, une signature).
pub fn sceller<T: Titre>(titre: &T, cle: &CleSignature) -> Result<Vec<u8>, ErreurSceau> {
    sceller_multi(titre, &[cle])
}

/// Scelle un titre avec plusieurs clés (1 à 8, distinctes) : chaque clé signe
/// les mêmes octets signés.
pub fn sceller_multi<T: Titre>(titre: &T, cles: &[&CleSignature]) -> Result<Vec<u8>, ErreurSceau> {
    if cles.is_empty() || cles.len() > MAX_SIGNATURES {
        return Err(ErreurSceau::NombreSignaturesInvalide(u8::try_from(cles.len()).unwrap_or(u8::MAX)));
    }
    let mut sceau = Sceau {
        algorithme: Algorithme::Ed25519,
        type_sceau: T::TYPE,
        charge: encoder_charge(titre)?,
        signatures: Vec::with_capacity(cles.len()),
    };
    for cle in cles {
        sceau.ajouter_signature(cle)?;
    }
    Ok(sceau.encoder())
}

/// Ajoute la signature de `cle` à un sceau existant (cosignature).
///
/// Le sceau doit être strictement bien formé, du type `T`, et sa charge doit
/// se décoder canoniquement en `T` (on ne cosigne pas ce qu'on ne sait pas
/// lire). Les signatures existantes ne sont **pas** vérifiées ici : c'est le
/// rôle du vérificateur. Un signataire déjà présent est refusé.
pub fn cosigner<T: Titre>(octets: &[u8], cle: &CleSignature) -> Result<Vec<u8>, ErreurSceau> {
    let mut sceau = Sceau::decoder(octets)?;
    if sceau.type_sceau != T::TYPE {
        return Err(ErreurSceau::TypeInattendu { attendu: T::TYPE as u8, trouve: sceau.type_sceau as u8 });
    }
    decoder_titre::<T>(&sceau.charge)?;
    sceau.ajouter_signature(cle)?;
    Ok(sceau.encoder())
}

fn decoder_titre<T: Titre>(charge: &[u8]) -> Result<T, ErreurSceau> {
    crate::tlv::valider_structure(charge)?;
    let mut lecteur = LecteurTlv::new(charge);
    let titre = T::decoder(&mut lecteur)?;
    lecteur.terminer()?;
    Ok(titre)
}

/// Le plus faible de deux niveaux (`Reconnu` < `Souverain`).
fn plus_faible(a: NiveauConfiance, b: NiveauConfiance) -> NiveauConfiance {
    if a == NiveauConfiance::Reconnu || b == NiveauConfiance::Reconnu {
        NiveauConfiance::Reconnu
    } else {
        NiveauConfiance::Souverain
    }
}

/// Vérifie un sceau à un signataire (seuil 1) et décode son titre (fail-closed).
///
/// Équivaut à [`verifier_seuil`] avec `seuil = 1`.
pub fn verifier<T: Titre>(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
) -> Result<Verifie<T>, ErreurSceau> {
    verifier_seuil(octets, ancres, 1, maintenant_secs)
}

/// Vérifie un sceau à seuil et décode son titre (fail-closed).
///
/// Politique : `seuil` (≥ 1) signataires **distincts** épinglés dans `ancres`.
///
/// Ordre : format strict → type attendu → algorithme implémenté → pour chaque
/// entrée épinglée, `verify_strict` (une seule invalide = refus du sceau
/// entier) → seuil → décodage canonique complet → fenêtre de validité.
///
/// Les entrées non épinglées (bien formées) ne sont pas comptées. Le niveau
/// de confiance retenu est le plus faible des signataires comptés.
pub fn verifier_seuil<T: Titre>(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    seuil: usize,
    maintenant_secs: u64,
) -> Result<Verifie<T>, ErreurSceau> {
    if seuil == 0 || seuil > MAX_SIGNATURES {
        return Err(ErreurSceau::SeuilInvalide);
    }
    let sceau = Sceau::decoder(octets)?;
    if sceau.type_sceau != T::TYPE {
        return Err(ErreurSceau::TypeInattendu { attendu: T::TYPE as u8, trouve: sceau.type_sceau as u8 });
    }
    if !sceau.algorithme.est_implemente() {
        return Err(ErreurSceau::AlgorithmeNonPrisEnCharge(sceau.algorithme as u8));
    }

    let message = sceau.octets_signes();
    let mut signataires: Vec<Kid> = Vec::with_capacity(sceau.signatures.len());
    let mut niveau: Option<NiveauConfiance> = None;
    for entree in &sceau.signatures {
        let Some((cle, niveau_ancre)) = ancres.trouver(&entree.kid) else {
            continue;
        };
        let brute: [u8; 64] = entree.octets.as_slice().try_into().map_err(|_| ErreurSceau::TailleInvalide)?;
        cle.verify_strict(&message, &Signature::from_bytes(&brute))
            .map_err(|_| ErreurSceau::SignatureInvalide)?;
        // `kid` uniques (décodage canonique) : chaque signataire compte une fois.
        signataires.push(entree.kid);
        niveau = Some(niveau.map_or(*niveau_ancre, |n| plus_faible(n, *niveau_ancre)));
    }
    let Some(niveau) = niveau else {
        return Err(ErreurSceau::SignataireInconnu);
    };
    if signataires.len() < seuil {
        return Err(ErreurSceau::SeuilNonAtteint { requis: seuil, obtenu: signataires.len() });
    }

    let titre = decoder_titre::<T>(&sceau.charge)?;

    if let Some((pas_avant, pas_apres)) = titre.fenetre_validite() {
        if maintenant_secs.saturating_add(TOLERANCE_HORLOGE_SECS) < pas_avant {
            return Err(ErreurSceau::PasEncoreValide);
        }
        if maintenant_secs.saturating_sub(TOLERANCE_HORLOGE_SECS) > pas_apres {
            return Err(ErreurSceau::Expire);
        }
    }
    Ok(Verifie::nouveau(titre, signataires, niveau))
}
