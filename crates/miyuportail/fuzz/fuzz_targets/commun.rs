//! @id     miyuportail_fuzz_commun
//! @do     outiller_cibles_fuzzing_portail
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles du Portail : traité, clés fixes, encadrement MSCE signé (EX-10).

//! Le décodeur vérifie les signatures **avant** la charge : une trame
//! aléatoire s'arrête au bloc de signatures. Pour atteindre le décodage de la
//! charge, les cibles signent elles-mêmes une charge quelconque avec des clés
//! fixes ([`encadrer`]) ; le mode est choisi par le premier octet de l'entrée.

#![allow(dead_code)]

use ed25519_dalek::{Signer, SigningKey};
use miyuportail::msce::{self, Ancres, Contexte};
use miyuportail::{ChampTraite, IdService, NatureChamp, NatureDocument, Sens, Traite, TypeDeclaration, TypeMime};

/// Service de la voie fuzzée.
pub const SERVICE: &str = "jayoffice";
/// Version du traité.
pub const VERSION_TRAITE: u64 = 1;
/// Codes de types (entrant, sortant).
pub const ENTRANT: u64 = 10;
pub const SORTANT: u64 = 20;

/// Clé de l'émetteur (façade ou publication de la voie).
pub fn cle_emetteur() -> SigningKey {
    SigningKey::from_bytes(&[7; 32])
}

/// Clé d'un auteur épinglé.
pub fn cle_auteur() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

pub fn service() -> IdService {
    IdService::new(SERVICE).expect("service valide")
}

/// Traité couvrant toutes les natures de champ, dans les deux sens.
pub fn traite() -> Traite {
    let mime = TypeMime::new("text/plain").expect("mime valide");
    let champs = |doc: NatureChamp| {
        vec![
            ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 128 }, obligatoire: true },
            ChampTraite { tag: 2, nature: NatureChamp::Entier, obligatoire: false },
            ChampTraite { tag: 3, nature: NatureChamp::Booleen, obligatoire: false },
            ChampTraite { tag: 5, nature: NatureChamp::Octets { max: 64 }, obligatoire: true },
            ChampTraite { tag: 9, nature: doc, obligatoire: false },
            ChampTraite { tag: 200, nature: NatureChamp::Entier, obligatoire: false },
        ]
    };
    let t = Traite {
        version: VERSION_TRAITE,
        types: vec![
            TypeDeclaration {
                code: ENTRANT,
                sens: Sens::Entrante,
                champs: champs(NatureChamp::Document { max: 1024, nature: NatureDocument::Document, mime: Some(mime) }),
            },
            TypeDeclaration {
                code: SORTANT,
                sens: Sens::Sortante,
                champs: champs(NatureChamp::Document { max: 1024, nature: NatureDocument::Image, mime: None }),
            },
            TypeDeclaration {
                code: ENTRANT + 1,
                sens: Sens::Entrante,
                champs: vec![ChampTraite {
                    tag: 1,
                    nature: NatureChamp::Document { max: 256, nature: NatureDocument::Archive, mime: None },
                    obligatoire: false,
                }],
            },
        ],
    };
    t.valider().expect("traité de fuzzing valide");
    t
}

/// Ancres : émetteur seul, ou émetteur + auteur épinglé.
pub fn ancres(avec_auteur: bool) -> Ancres {
    Ancres {
        emetteur: cle_emetteur().verifying_key().to_bytes(),
        auteurs: if avec_auteur { vec![cle_auteur().verifying_key().to_bytes()] } else { Vec::new() },
    }
}

/// Construit une trame MSCE autour d'une charge **quelconque**, signée par
/// `cles` (entrées triées par `kid`, comme le scelleur).
pub fn encadrer(charge: &[u8], cles: &[&SigningKey]) -> Vec<u8> {
    let message = msce::octets_signes(charge);
    let mut entrees: Vec<([u8; 32], [u8; 64])> =
        cles.iter().map(|c| (msce::kid(&c.verifying_key().to_bytes()), c.sign(&message).to_bytes())).collect();
    entrees.sort_by_key(|e| e.0);
    let mut s =
        Vec::with_capacity(msce::TAILLE_ENTETE + charge.len() + 1 + entrees.len() * msce::TAILLE_ENTREE_SIGNATURE);
    s.extend_from_slice(msce::MAGIC);
    s.push(msce::VERSION);
    s.push(msce::ALG_ED25519);
    s.push(msce::TYPE_DECLARATION);
    s.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    s.extend_from_slice(charge);
    s.push(entrees.len() as u8);
    for (k, sig) in &entrees {
        s.extend_from_slice(k);
        s.push(msce::ALG_ED25519);
        s.extend_from_slice(&64u16.to_be_bytes());
        s.extend_from_slice(sig);
    }
    s
}

/// Cas fuzzé : trame, sens, ancres, et clés ayant signé (vide = trame brute).
pub struct Cas {
    pub trame: Vec<u8>,
    pub sens: Sens,
    pub avec_auteur: bool,
    pub signataires: Vec<SigningKey>,
}

/// Interprète l'entrée : `mode (1) | reste`.
///
/// - bits 0-1 : 0 = trame brute ; 1 = charge signée par l'émetteur ;
///   2 = charge cosignée émetteur + auteur ; 3 = charge signée par l'auteur seul ;
/// - bit 2 : sens sortant ;
/// - bit 3 : ancres avec auteur épinglé (forcé pour le mode 2).
pub fn cas(entree: &[u8]) -> Option<Cas> {
    let (&mode, reste) = entree.split_first()?;
    let sens = if mode & 0x04 != 0 { Sens::Sortante } else { Sens::Entrante };
    let (signataires, avec_auteur) = match mode & 0x03 {
        0 => (Vec::new(), mode & 0x08 != 0),
        1 => (vec![cle_emetteur()], mode & 0x08 != 0),
        2 => (vec![cle_emetteur(), cle_auteur()], true),
        _ => (vec![cle_auteur()], mode & 0x08 != 0),
    };
    let trame = if signataires.is_empty() {
        reste.to_vec()
    } else {
        let refs: Vec<&SigningKey> = signataires.iter().collect();
        encadrer(reste, &refs)
    };
    Some(Cas { trame, sens, avec_auteur, signataires })
}

/// Contexte de décodage d'un cas (le traité et les ancres sont fournis par l'appelant).
pub fn contexte<'a>(service: &'a IdService, sens: Sens, traite: &'a Traite, ancres: &'a Ancres) -> Contexte<'a> {
    Contexte { service, sens, traite, ancres }
}
