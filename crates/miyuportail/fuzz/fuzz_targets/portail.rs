//! @id     miyuportail_fuzz_portail
//! @do     fuzzer_portail_avec_modele
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — Portail interne à états contre un modèle : dépôt, relève par le sas, anti-rejeu par auteur, auteurs, coupure, rétablissement, publication (EX-10, EX-44, EX-49, A-007).

//! Entrée : suite d'opérations (`op (1) | arguments`). Le sas est le cœur
//! réel ([`traiter_requete`], sans confinement) derrière son protocole ; le
//! CDR rend le document tel quel, sauf s'il commence par `0xFF` (refus).
//!
//! Le modèle connaît, pour chaque trame **construite** en quarantaine, si
//! elle doit passer (déclaration conforme, auteur épinglé au moment de la
//! relève, séquence au-delà de la marque haute de l'auteur, document admis
//! par le CDR) ; la relève doit lui donner raison **exactement**. Une trame
//! brute (charge quelconque signée) ne peut passer qu'en respectant les
//! mêmes invariants (auteur épinglé, séquence croissante, conformité).
//!
//! Autres propriétés : aucune panique ; voie coupée, désactivée ou sans
//! session = rien n'est admis ; compteurs cohérents ; toute publication lue
//! est une trame `0x08` neuve, signée par la clé de la voie, à séquence
//! strictement croissante, identique à ce qui a été publié.

#![no_main]

#[path = "commun.rs"]
mod commun;

use std::collections::VecDeque;
use std::sync::Arc;

use ed25519_dalek::SigningKey;
use libfuzzer_sys::fuzz_target;
use miyuportail::msce::{self, conforme_au_traite, decoder_trame, Kid};
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{
    Ancres, ClasseExposition, ContratVoies, CoupureVoie, Declaration, Declencheur, DecodeurIsole, Desarmement,
    ErreurPortail, ErreurSas, EtatVoie, MotifCoupure, ObjetADesarmer, ObjetDesarme, PortailInterne, Publication,
    RefusDesarmement, RegistreMemoire, RequeteSas, Sens, Sensibilite, Valeur,
};

const QUOTA: usize = 16;

struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let reponse = traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?)
            .map_err(|_| ErreurSas::FinAnormale)?;
        decoder_reponse(&reponse, requete.trames.len())
    }
}

/// CDR d'essai : identité, refus si le document commence par 0xFF.
struct CdrEssai;

impl Desarmement for CdrEssai {
    fn desarmer(&self, o: &ObjetADesarmer<'_>) -> Result<ObjetDesarme, RefusDesarmement> {
        if o.octets.first() == Some(&0xFF) {
            return Err(RefusDesarmement);
        }
        Ok(ObjetDesarme { octets: o.octets.to_vec() })
    }
}

fn cdr_admet(d: &Declaration) -> bool {
    d.champs.iter().all(|c| !matches!(&c.valeur, Valeur::Document(o) if o.first() == Some(&0xFF)))
}

fn auteurs() -> [SigningKey; 4] {
    [9u8, 11, 13, 15].map(|g| SigningKey::from_bytes(&[g; 32]))
}

/// Attente du modèle pour une trame en quarantaine.
enum Attente {
    /// Construite : auteur (indice), séquence, admissible hors état (conforme, CDR).
    Construite { auteur: usize, sequence: u64, admissible: bool },
    /// Brute : aucun pronostic, seulement des invariants.
    Brute,
}

#[derive(Default)]
struct Modele {
    active: bool,
    coupee: bool,
    session: bool,
    epingles: [bool; 4],
    /// Marque haute par auteur dans la session (conservée au retrait).
    marques: [Option<u64>; 4],
    quarantaine: VecDeque<Attente>,
    publications: VecDeque<Declaration>,
    sequence_publication: u64,
}

impl Modele {
    fn purger_session(&mut self) {
        self.session = false;
        self.epingles = [false; 4];
        self.marques = [None; 4];
        self.quarantaine.clear();
    }
}

fuzz_target!(|entree: &[u8]| {
    let mut f = commun::Flux::new(entree);
    let service = commun::service();
    let traite = commun::traite();
    let facade = commun::cle_emetteur();
    let cles = auteurs();
    let kids: Vec<Kid> = cles.iter().map(|c| msce::kid(&c.verifying_key().to_bytes())).collect();
    let portail = PortailInterne::new(Box::new(SasEnProcessus), Box::new(CdrEssai), Arc::new(RegistreMemoire::new()));
    portail
        .declarer(ContratVoies {
            service: service.clone(),
            classe: ClasseExposition::H2,
            traite: traite.clone(),
            taille_max_trame: msce::TAILLE_MAX_TRAME,
            debit_max_par_minute: 1_000_000,
            quota_file: QUOTA,
        })
        .expect("contrat de fuzzing valide");
    let mut m = Modele::default();
    let mut operations = 0;

    while !f.vide() && operations < 64 {
        operations += 1;
        match f.u8() % 12 {
            // Activer.
            0 => {
                let r = portail.activer(&service);
                assert_eq!(r.is_ok(), !m.coupee);
                if r.is_ok() {
                    m.active = true;
                }
            }
            // Ouvrir une session (auteurs choisis par masque, au moins un).
            1 => {
                let masque = (f.u8() & 0x0F).max(1);
                let choisis: Vec<[u8; 32]> = (0..4)
                    .filter(|i| masque & (1 << i) != 0)
                    .map(|i| cles[i].verifying_key().to_bytes())
                    .collect();
                let r = portail.ouvrir_session(&service, facade.verifying_key().to_bytes(), choisis);
                assert_eq!(r.is_ok(), m.active && !m.coupee, "ouverture de session : {r:?}");
                if r.is_ok() {
                    m.purger_session();
                    m.session = true;
                    for i in 0..4 {
                        m.epingles[i] = masque & (1 << i) != 0;
                    }
                }
            }
            // Déposer une trame construite (façade + auteur i).
            2..=4 => {
                let i = usize::from(f.u8() % 4);
                let d = commun::declaration_tiree(&mut f, Sens::Entrante);
                let Ok(trame) = msce::sceller(&d, &[&facade, &cles[i]]) else { continue };
                // Admissible hors état de session : ce que le décodeur n° 1 admet
                // avec cet auteur épinglé, puis le CDR.
                let ancres = Ancres {
                    emetteur: facade.verifying_key().to_bytes(),
                    auteurs: vec![cles[i].verifying_key().to_bytes()],
                };
                let admissible = decoder_trame(&trame, &commun::contexte(&service, Sens::Entrante, &traite, &ancres))
                    .is_ok_and(|lue| cdr_admet(&lue));
                let r = portail.deposer(&service, trame);
                let attendu = m.active && !m.coupee && m.session && m.quarantaine.len() < QUOTA;
                assert_eq!(r.is_ok(), attendu, "dépôt : {r:?}");
                if r.is_ok() {
                    m.quarantaine.push_back(Attente::Construite { auteur: i, sequence: d.sequence, admissible });
                }
            }
            // Déposer une charge brute signée (façade + auteur i), ou des octets bruts.
            5 => {
                let i = usize::from(f.u8() % 4);
                let n = usize::from(f.u8());
                let brut = f.prendre(n);
                let trame = if i == 3 { brut.to_vec() } else { commun::encadrer(brut, &[&facade, &cles[i]]) };
                let msce_ok = trame.starts_with(msce::MAGIC);
                let r = portail.deposer(&service, trame);
                let attendu = m.active && !m.coupee && m.session && m.quarantaine.len() < QUOTA && msce_ok;
                assert_eq!(r.is_ok(), attendu, "dépôt brut : {r:?}");
                if r.is_ok() {
                    m.quarantaine.push_back(Attente::Brute);
                }
            }
            // Relever.
            6 | 7 => {
                let max = usize::from(f.u8() % 20);
                let avant = portail.compteurs(&service).expect("compteurs");
                let r = portail.relever(&service, max);
                let relevable = m.active && !m.coupee && m.session;
                assert_eq!(r.is_ok(), relevable, "relève : {:?}", r.as_ref().err());
                let Ok(resultats) = r else { continue };
                let n = max.min(m.quarantaine.len());
                assert_eq!(resultats.len(), n);
                for resultat in resultats {
                    let attente = m.quarantaine.pop_front().expect("modèle synchronisé");
                    // Pronostic exact pour une trame construite, avant mise à jour des marques.
                    if let Attente::Construite { auteur, sequence, admissible } = attente {
                        let attendu =
                            admissible && m.epingles[auteur] && m.marques[auteur].is_none_or(|s| sequence > s);
                        let refus = resultat.as_ref().err();
                        assert_eq!(resultat.is_ok(), attendu, "pronostic du modèle contredit : {refus:?}");
                    }
                    // Invariants de toute trame admise (construite ou brute).
                    if let Ok(admise) = &resultat {
                        let d = &admise.declaration;
                        assert_eq!(d.service, service);
                        assert_eq!(d.sens, Sens::Entrante);
                        conforme_au_traite(d, &traite).expect("admise non conforme");
                        let kid_facade = msce::kid(&facade.verifying_key().to_bytes());
                        assert!(d.signataires.contains(&kid_facade), "admise sans la façade");
                        let i = kids.iter().position(|k| d.signataires.contains(k)).expect("admise sans auteur connu");
                        assert!(m.epingles[i], "auteur non épinglé admis");
                        assert!(m.marques[i].is_none_or(|s| d.sequence > s), "rejeu admis");
                        m.marques[i] = Some(d.sequence);
                    }
                }
                let apres = portail.compteurs(&service).expect("compteurs");
                assert_eq!(apres.relevees - avant.relevees, n as u64);
                assert_eq!(apres.admises + apres.refusees, apres.relevees, "compteurs incohérents");
                assert!(apres.deposees >= apres.relevees);
            }
            // Épingler un auteur dans la session ouverte.
            8 => {
                let i = usize::from(f.u8() % 4);
                let r = portail.ajouter_auteur(&service, cles[i].verifying_key().to_bytes());
                let attendu = m.active && !m.coupee && m.session && !m.epingles[i];
                assert_eq!(r.is_ok(), attendu, "épinglage : {r:?}");
                if r.is_ok() {
                    m.epingles[i] = true;
                }
            }
            // Retirer un auteur (jamais le dernier).
            9 => {
                let i = usize::from(f.u8() % 4);
                let r = portail.retirer_auteur(&service, &cles[i].verifying_key().to_bytes());
                let restants = m.epingles.iter().filter(|e| **e).count();
                let attendu = m.session && m.epingles[i] && restants > 1;
                assert_eq!(r.is_ok(), attendu, "retrait : {r:?}");
                if r.is_ok() {
                    m.epingles[i] = false;
                }
            }
            // Couper, fermer, désactiver, rétablir.
            10 => match f.u8() % 4 {
                0 => {
                    portail.couper(&service, MotifCoupure::Anomalie).expect("coupure");
                    m.coupee = true;
                    m.active = false;
                    m.purger_session();
                    m.publications.clear();
                    assert!(matches!(portail.etat(&service), Ok(EtatVoie::Coupee(_))));
                }
                1 => {
                    portail.fermer_session(&service).expect("fermeture");
                    m.purger_session();
                }
                2 => {
                    let r = portail.desactiver(&service);
                    assert_eq!(r.is_ok(), !m.coupee);
                    if r.is_ok() {
                        m.active = false;
                        m.purger_session();
                        m.publications.clear();
                    }
                }
                _ => {
                    let r = portail.retablir(&service);
                    assert_eq!(r.is_ok(), m.coupee);
                    if r.is_ok() {
                        m.coupee = false;
                        m.sequence_publication = 0;
                    }
                }
            },
            // Publier, ou lire une publication.
            _ => {
                if f.u8() & 1 == 0 {
                    let d = commun::declaration_tiree(&mut f, Sens::Sortante);
                    let etiquette = [Sensibilite::Session, Sensibilite::Publiable, Sensibilite::Nationale]
                        [usize::from(f.u8() % 3)];
                    let declencheur =
                        [Declencheur::Utilisateur, Declencheur::Maia, Declencheur::Service][usize::from(f.u8() % 3)];
                    let publication =
                        Publication { type_code: d.type_code, champs: d.champs.clone(), etiquette, declencheur };
                    // Le Portail fixe lui-même sens et version du brouillon.
                    let brouillon =
                        Declaration { sens: Sens::Sortante, version_traite: commun::VERSION_TRAITE, ..d.clone() };
                    let conforme = conforme_au_traite(&brouillon, &traite).is_ok();
                    let r = portail.publier(&service, publication);
                    let attendu = etiquette != Sensibilite::Nationale
                        && declencheur == Declencheur::Utilisateur
                        && m.active
                        && !m.coupee
                        && conforme
                        && cdr_admet(&d)
                        && m.publications.len() < QUOTA;
                    assert_eq!(r.is_ok(), attendu, "publication : {r:?}");
                    if let Ok(sequence) = r {
                        m.sequence_publication += 1;
                        assert_eq!(sequence, m.sequence_publication);
                        m.publications.push_back(Declaration { sequence, ..d });
                    }
                } else {
                    let r = portail.lire_publication(&service);
                    assert_eq!(r.is_ok(), m.active && !m.coupee, "lecture : {r:?}");
                    let Ok(lue) = r else { continue };
                    let attendue = m.publications.pop_front();
                    assert_eq!(lue.is_some(), attendue.is_some());
                    let (Some(trame), Some(attendue)) = (lue, attendue) else { continue };
                    let ancres =
                        Ancres { emetteur: portail.cle_publication(&service).expect("clé"), auteurs: Vec::new() };
                    let ctx = commun::contexte(&service, Sens::Sortante, &traite, &ancres);
                    let d = decoder_trame(&trame, &ctx).expect("publication relisible par la façade");
                    assert_eq!(d.sequence, attendue.sequence);
                    assert_eq!(d.type_code, attendue.type_code);
                    assert_eq!(d.champs, attendue.champs, "publication altérée");
                }
            }
        }
        // Aucune erreur d'état interne, jamais.
        assert_ne!(portail.compteurs(&service).err(), Some(ErreurPortail::EtatIndisponible));
    }
});
