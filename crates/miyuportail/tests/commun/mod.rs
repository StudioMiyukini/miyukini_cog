//! @id     miyuportail_tests_commun
//! @do     outiller_tests_portail
//! @role   test
//! @layer  toolkit
//! @human  miyuportail — outillage commun des tests (traité, contrats, clés, trames).

#![allow(dead_code)]

use std::sync::Arc;

use ed25519_dalek::SigningKey;
use miyuportail::msce::sceller;
use miyuportail::sas::{traiter_requete, ResultatTrame};
use miyuportail::*;

pub const ENTRANT: u64 = 10;
pub const SORTANT: u64 = 20;

pub fn sas_reel() -> SasProcessus {
    SasProcessus::new(env!("CARGO_BIN_EXE_portail-sas"))
}

/// Décodeur « en processus » : même cœur que le sas, sans confinement (tests de logique).
pub struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let reponse = traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?)
            .map_err(|_| ErreurSas::FinAnormale)?;
        miyuportail::sas::decoder_reponse(&reponse, requete.trames.len())
    }
}

pub fn traite() -> Traite {
    Traite {
        version: 1,
        types: vec![
            TypeDeclaration {
                code: ENTRANT,
                sens: Sens::Entrante,
                champs: vec![
                    ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 128 }, obligatoire: true },
                    ChampTraite { tag: 2, nature: NatureChamp::Entier, obligatoire: false },
                    ChampTraite {
                        tag: 3,
                        nature: NatureChamp::Document {
                            max: 1024,
                            nature: NatureDocument::Document,
                            mime: Some(TypeMime::new("text/plain").unwrap()),
                        },
                        obligatoire: false,
                    },
                ],
            },
            TypeDeclaration {
                code: SORTANT,
                sens: Sens::Sortante,
                champs: vec![
                    ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 128 }, obligatoire: true },
                    ChampTraite {
                        tag: 2,
                        nature: NatureChamp::Document { max: 1024, nature: NatureDocument::Image, mime: None },
                        obligatoire: false,
                    },
                ],
            },
        ],
    }
}

pub fn id(nom: &str) -> IdService {
    IdService::new(nom).unwrap()
}

pub fn contrat(nom: &str, classe: ClasseExposition) -> ContratVoies {
    let mut traite = traite();
    if classe == ClasseExposition::H1 {
        traite.types.retain(|t| t.sens == Sens::Sortante);
    }
    ContratVoies {
        service: id(nom),
        classe,
        traite,
        taille_max_trame: 8192,
        debit_max_par_minute: 1000,
        quota_file: 64,
    }
}

pub struct Monde {
    pub portail: PortailInterne,
    pub registre: Arc<RegistreMemoire>,
    pub facade: SigningKey,
    pub auteur: SigningKey,
}

pub fn cle(graine: u8) -> SigningKey {
    SigningKey::from_bytes(&[graine; 32])
}

pub fn monde(sas: Box<dyn DecodeurIsole>, cdr: Box<dyn Desarmement>) -> Monde {
    let registre = Arc::new(RegistreMemoire::new());
    Monde { portail: PortailInterne::new(sas, cdr, registre.clone()), registre, facade: cle(7), auteur: cle(9) }
}

impl Monde {
    /// Déclare, active et ouvre une session sur une voie H2.
    pub fn voie_ouverte(&self, nom: &str) -> IdService {
        let s = id(nom);
        self.portail.declarer(contrat(nom, ClasseExposition::H2)).unwrap();
        self.portail.activer(&s).unwrap();
        self.portail
            .ouvrir_session(&s, self.facade.verifying_key().to_bytes(), vec![self.auteur.verifying_key().to_bytes()])
            .unwrap();
        s
    }

    pub fn trame(&self, service: &IdService, sequence: u64) -> Vec<u8> {
        sceller(&declaration(service, Sens::Entrante, ENTRANT, sequence, champs_simples()), &[&self.facade, &self.auteur])
            .unwrap()
    }
}

pub fn champs_simples() -> Vec<Champ> {
    vec![Champ { tag: 1, valeur: Valeur::Texte("bonjour".into()) }, Champ { tag: 2, valeur: Valeur::Entier(42) }]
}

pub fn declaration(service: &IdService, sens: Sens, type_code: u64, sequence: u64, champs: Vec<Champ>) -> Declaration {
    Declaration {
        service: service.clone(),
        sens,
        version_traite: 1,
        type_code,
        sequence,
        emise_le: 1_790_000_000,
        champs,
        signataires: Vec::new(),
    }
}
