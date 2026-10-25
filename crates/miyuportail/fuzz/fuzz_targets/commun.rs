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

/// Lecteur d'octets de l'entrée fuzzée : au-delà de la fin, renvoie 0.
pub struct Flux<'a> {
    octets: &'a [u8],
}

impl<'a> Flux<'a> {
    pub fn new(octets: &'a [u8]) -> Self {
        Self { octets }
    }

    pub fn vide(&self) -> bool {
        self.octets.is_empty()
    }

    pub fn u8(&mut self) -> u8 {
        match self.octets.split_first() {
            Some((&o, reste)) => {
                self.octets = reste;
                o
            }
            None => 0,
        }
    }

    pub fn u64(&mut self) -> u64 {
        (0..8).fold(0u64, |a, _| (a << 8) | u64::from(self.u8()))
    }

    /// Prend au plus `n` octets.
    pub fn prendre(&mut self, n: usize) -> &'a [u8] {
        let n = n.min(self.octets.len());
        let (tete, reste) = self.octets.split_at(n);
        self.octets = reste;
        tete
    }
}

/// Champs **conformes** au traité de fuzzing pour `code` (types `ENTRANT`,
/// `ENTRANT + 1`, `SORTANT`), tirés de l'entrée : optionnels présents ou non,
/// tailles dans les bornes, texte `[a-z ]`.
pub fn champs_conformes(f: &mut Flux<'_>, code: u64) -> Vec<miyuportail::Champ> {
    use miyuportail::{Champ, Valeur};
    let presence = f.u8();
    let mut champs = Vec::new();
    if code == ENTRANT + 1 {
        if presence & 1 != 0 {
            let n = usize::from(f.u8());
            champs.push(Champ { tag: 1, valeur: Valeur::Document(f.prendre(n).to_vec()) });
        }
        return champs;
    }
    let n = usize::from(f.u8() % 129);
    let lettre = |o: &u8| if o % 27 == 26 { ' ' } else { char::from(b'a' + o % 27) };
    let texte: String = f.prendre(n).iter().map(lettre).collect();
    champs.push(Champ { tag: 1, valeur: Valeur::Texte(texte) });
    if presence & 1 != 0 {
        champs.push(Champ { tag: 2, valeur: Valeur::Entier(f.u64()) });
    }
    if presence & 2 != 0 {
        champs.push(Champ { tag: 3, valeur: Valeur::Booleen(f.u8() & 1 != 0) });
    }
    let n = usize::from(f.u8() % 65);
    champs.push(Champ { tag: 5, valeur: Valeur::Octets(f.prendre(n).to_vec()) });
    if presence & 4 != 0 {
        let n = usize::from(f.u8()) * 4;
        champs.push(Champ { tag: 9, valeur: Valeur::Document(f.prendre(n.min(1024)).to_vec()) });
    }
    if presence & 8 != 0 {
        champs.push(Champ { tag: 200, valeur: Valeur::Entier(f.u64()) });
    }
    champs
}

/// Déclaration de la voie fuzzée, conforme au traité ou non selon les octets
/// (version, type ou champs perturbés quand `perturbation` le demande).
pub fn declaration_tiree(f: &mut Flux<'_>, sens: Sens) -> miyuportail::Declaration {
    use miyuportail::{Champ, Declaration, Valeur};
    let perturbation = f.u8();
    let code = match (sens, f.u8() % 2) {
        (Sens::Entrante, 0) => ENTRANT,
        (Sens::Entrante, _) => ENTRANT + 1,
        (Sens::Sortante, _) => SORTANT,
    };
    let mut d = Declaration {
        service: service(),
        sens,
        version_traite: VERSION_TRAITE,
        type_code: code,
        sequence: u64::from(f.u8()),
        emise_le: 1_800_000_000,
        champs: champs_conformes(f, code),
        signataires: Vec::new(),
    };
    match perturbation {
        0xF0 => d.version_traite = f.u64(),
        0xF1 => d.type_code = f.u64(),
        0xF2 => d.sens = if sens == Sens::Entrante { Sens::Sortante } else { Sens::Entrante },
        0xF3 => d.champs.push(Champ { tag: 250, valeur: Valeur::Entier(0) }),
        0xF4 => d.champs.retain(|c| c.tag != 1),
        0xF5 => {
            let n = usize::from(f.u8());
            d.champs.retain(|c| c.tag != 2);
            d.champs.insert(1.min(d.champs.len()), Champ { tag: 2, valeur: Valeur::Octets(f.prendre(n).to_vec()) });
        }
        0xF6 => {
            if let Some(Champ { valeur: Valeur::Texte(t), .. }) = d.champs.first_mut() {
                t.push('\u{7}');
            }
        }
        _ => {}
    }
    d
}
