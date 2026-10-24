//! @id     miyuportail_tests_portail
//! @do     tester_voies_sas_coupure
//! @role   test
//! @layer  toolkit
//! @human  miyuportail — voies à sens unique, sas confiné réel, deuxième validation, CDR, coupure ciblée (EX-44..46, EX-49).

#![allow(clippy::many_single_char_names, clippy::format_collect)]

mod commun;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use commun::*;
use miyuportail::msce::{decoder_trame, sceller, Contexte};
use miyuportail::registre::ErreurInscription;
#[cfg(debug_assertions)]
use miyuportail::sas::Essai;
use miyuportail::sas::ResultatTrame;
use miyuportail::*;

fn reel() -> Monde {
    monde(Box::new(sas_reel()), Box::new(RefusParDefaut))
}

fn rapide() -> Monde {
    monde(Box::new(SasEnProcessus), Box::new(RefusParDefaut))
}

fn erreurs(r: Vec<Result<DeclarationAdmise, ErreurPortail>>) -> Vec<ErreurPortail> {
    r.into_iter().map(|x| x.unwrap_err()).collect()
}

// ---------------------------------------------------------------- voies

#[test]
fn chemin_nominal_par_le_sas_reel() {
    let m = reel();
    let s = m.voie_ouverte("jayoffice.docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    m.portail.deposer(&s, m.trame(&s, 2)).unwrap();
    let r = m.portail.relever(&s, 10).unwrap();
    assert_eq!(r.len(), 2);
    let d = r[0].as_ref().unwrap();
    assert_eq!(d.declaration.champs, champs_simples());
    assert_eq!(d.declaration.sequence, 1);
    assert_eq!(r[1].as_ref().unwrap().declaration.sequence, 2);
    let c = m.portail.compteurs(&s).unwrap();
    assert_eq!((c.deposees, c.relevees, c.admises, c.refusees), (2, 2, 2, 0));
    assert!(matches!(&m.registre.evenements()[0], Evenement::Entree { .. }));
    // File vide : rien à relever.
    assert_eq!(m.portail.relever(&s, 10).unwrap(), Vec::new());
}

#[test]
fn voie_non_declaree_refusee() {
    let m = rapide();
    let s = id("inconnu");
    assert_eq!(m.portail.deposer(&s, m.trame(&s, 1)), Err(ErreurPortail::VoieNonDeclaree));
    assert_eq!(m.portail.relever(&s, 1).unwrap_err(), ErreurPortail::VoieNonDeclaree);
    let p = Publication { type_code: SORTANT, champs: vec![], etiquette: Sensibilite::Publiable, declencheur: Declencheur::Utilisateur };
    assert_eq!(m.portail.publier(&s, p), Err(ErreurPortail::VoieNonDeclaree));
    assert_eq!(m.portail.lire_publication(&s), Err(ErreurPortail::VoieNonDeclaree));
    assert_eq!(m.portail.couper(&s, MotifCoupure::Anomalie), Err(ErreurPortail::VoieNonDeclaree));
}

#[test]
fn exposition_desactivee_par_defaut_et_h0_interdit() {
    let m = rapide();
    assert_eq!(m.portail.declarer(contrat("maia", ClasseExposition::H0)), Err(ErreurPortail::ClasseSansFacade));
    let s = id("site");
    m.portail.declarer(contrat("site", ClasseExposition::H2)).unwrap();
    assert_eq!(m.portail.etat(&s).unwrap(), EtatVoie::Desactivee);
    let cle = m.facade.verifying_key().to_bytes();
    assert_eq!(
        m.portail.ouvrir_session(&s, cle, vec![m.auteur.verifying_key().to_bytes()]),
        Err(ErreurPortail::VoieDesactivee)
    );
    assert_eq!(m.portail.deposer(&s, m.trame(&s, 1)), Err(ErreurPortail::VoieDesactivee));
    assert_eq!(m.portail.declarer(contrat("site", ClasseExposition::H2)), Err(ErreurPortail::VoieDejaDeclaree));
}

#[test]
fn manifeste_tout_ou_rien() {
    let m = rapide();
    let r = m.portail.declarer_manifeste(&[contrat("a", ClasseExposition::H2), contrat("b", ClasseExposition::H0)]);
    assert_eq!(r, Err(ErreurPortail::ClasseSansFacade));
    assert_eq!(m.portail.etat(&id("a")), Err(ErreurPortail::VoieNonDeclaree));
    m.portail.declarer_manifeste(&[contrat("a", ClasseExposition::H2), contrat("b", ClasseExposition::H1)]).unwrap();
    // Un H1 ne peut pas déclarer de type entrant.
    let mut c = contrat("c", ClasseExposition::H1);
    c.traite = traite();
    assert!(matches!(m.portail.declarer(c), Err(ErreurPortail::ContratInvalide(_))));
}

#[test]
fn mauvais_sens_refuse() {
    let m = rapide();
    // H1 : voie publication seule, aucune entrée.
    let s = id("vitrine");
    m.portail.declarer(contrat("vitrine", ClasseExposition::H1)).unwrap();
    m.portail.activer(&s).unwrap();
    assert_eq!(m.portail.deposer(&s, m.trame(&s, 1)), Err(ErreurPortail::SensRefuse));
    assert_eq!(m.portail.relever(&s, 1).unwrap_err(), ErreurPortail::SensRefuse);
    assert_eq!(
        m.portail.ouvrir_session(&s, m.facade.verifying_key().to_bytes(), vec![m.auteur.verifying_key().to_bytes()]),
        Err(ErreurPortail::SensRefuse)
    );
    // H2 : une trame « sortante » déposée sur la voie entrante est refusée.
    let s = m.voie_ouverte("docs");
    let t = sceller(&declaration(&s, Sens::Sortante, SORTANT, 1, champs_simples()), &[&m.facade, &m.auteur]).unwrap();
    m.portail.deposer(&s, t).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::Decodage(RefusDecodage::Sens)]);
    // Type sortant présenté dans le sens entrant.
    let t = sceller(&declaration(&s, Sens::Entrante, SORTANT, 2, champs_simples()), &[&m.facade, &m.auteur]).unwrap();
    m.portail.deposer(&s, t).unwrap();
    assert_eq!(
        erreurs(m.portail.relever(&s, 1).unwrap()),
        vec![ErreurPortail::Decodage(RefusDecodage::TypeDeclaration)]
    );
}

#[test]
fn liste_blanche_taille_quota_debit() {
    let m = rapide();
    let s = m.voie_ouverte("docs");
    assert_eq!(m.portail.deposer(&s, b"GET / HTTP/1.1\r\n\r\n".to_vec()), Err(ErreurPortail::ProtocoleRefuse));
    let mut grand = b"MSCE".to_vec();
    grand.resize(8193, 0);
    assert_eq!(m.portail.deposer(&s, grand), Err(ErreurPortail::TropGrand));

    let q = id("quota");
    let mut c = contrat("quota", ClasseExposition::H2);
    c.quota_file = 1;
    c.debit_max_par_minute = 2;
    m.portail.declarer(c).unwrap();
    m.portail.activer(&q).unwrap();
    m.portail.ouvrir_session(&q, m.facade.verifying_key().to_bytes(), vec![m.auteur.verifying_key().to_bytes()]).unwrap();
    m.portail.deposer(&q, m.trame(&q, 1)).unwrap();
    assert_eq!(m.portail.deposer(&q, m.trame(&q, 2)), Err(ErreurPortail::FilePleine));
    m.portail.relever(&q, 1).unwrap();
    m.portail.deposer(&q, m.trame(&q, 2)).unwrap();
    m.portail.relever(&q, 1).unwrap();
    assert_eq!(m.portail.deposer(&q, m.trame(&q, 3)), Err(ErreurPortail::DebitDepasse));
}

#[test]
fn session_requise_et_cles_controlees() {
    let m = rapide();
    let s = id("docs");
    m.portail.declarer(contrat("docs", ClasseExposition::H2)).unwrap();
    m.portail.activer(&s).unwrap();
    assert_eq!(m.portail.deposer(&s, m.trame(&s, 1)), Err(ErreurPortail::SessionAbsente));
    assert_eq!(m.portail.relever(&s, 1).unwrap_err(), ErreurPortail::SessionAbsente);
    let f = m.facade.verifying_key().to_bytes();
    assert_eq!(m.portail.ouvrir_session(&s, f, vec![]), Err(ErreurPortail::CleInvalide));
    assert_eq!(m.portail.ouvrir_session(&s, f, vec![f]), Err(ErreurPortail::CleInvalide));
    // Point de petit ordre (clé faible) refusé.
    let mut faible = [0u8; 32];
    faible[0] = 1;
    assert_eq!(m.portail.ouvrir_session(&s, faible, vec![m.auteur.verifying_key().to_bytes()]), Err(ErreurPortail::CleInvalide));
    // Fermeture : quarantaine purgée.
    m.portail.ouvrir_session(&s, f, vec![m.auteur.verifying_key().to_bytes()]).unwrap();
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    m.portail.fermer_session(&s).unwrap();
    assert_eq!(m.portail.relever(&s, 1).unwrap_err(), ErreurPortail::SessionAbsente);
}

#[test]
fn rejeu_refuse() {
    let m = rapide();
    let s = m.voie_ouverte("docs");
    let t = m.trame(&s, 5);
    m.portail.deposer(&s, t.clone()).unwrap();
    m.portail.deposer(&s, t).unwrap();
    m.portail.deposer(&s, m.trame(&s, 4)).unwrap();
    let r = m.portail.relever(&s, 10).unwrap();
    assert!(r[0].is_ok());
    assert_eq!(r[1], Err(ErreurPortail::Rejeu));
    assert_eq!(r[2], Err(ErreurPortail::Rejeu));
}

#[test]
fn signature_auteur_obligatoire() {
    let m = rapide();
    let s = m.voie_ouverte("docs");
    let d = declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples());
    m.portail.deposer(&s, sceller(&d, &[&m.facade]).unwrap()).unwrap();
    m.portail.deposer(&s, sceller(&d, &[&m.facade, &cle(11)]).unwrap()).unwrap();
    assert_eq!(
        erreurs(m.portail.relever(&s, 10).unwrap()),
        vec![ErreurPortail::Decodage(RefusDecodage::Signataires); 2]
    );
}

// ---------------------------------------------------------------- coupure (EX-49)

#[test]
fn voie_coupee_refuse_les_autres_continuent() {
    let m = rapide();
    let a = m.voie_ouverte("jayoffice.docs");
    let b = m.voie_ouverte("jaymessage");
    m.portail.deposer(&a, m.trame(&a, 1)).unwrap();
    m.portail.deposer(&b, m.trame(&b, 1)).unwrap();

    let poignee: Box<dyn CoupureVoie> = Box::new(m.portail.poignee_coupure());
    poignee.couper(&a, MotifCoupure::Anomalie).unwrap();

    assert_eq!(m.portail.etat(&a).unwrap(), EtatVoie::Coupee(MotifCoupure::Anomalie));
    assert_eq!(m.portail.deposer(&a, m.trame(&a, 2)), Err(ErreurPortail::VoieCoupee));
    assert_eq!(m.portail.relever(&a, 1).unwrap_err(), ErreurPortail::VoieCoupee);
    let p = || Publication {
        type_code: SORTANT,
        champs: vec![Champ { tag: 1, valeur: Valeur::Texte("x".into()) }],
        etiquette: Sensibilite::Publiable,
        declencheur: Declencheur::Utilisateur,
    };
    assert_eq!(m.portail.publier(&a, p()), Err(ErreurPortail::VoieCoupee));

    // L'autre voie n'est pas touchée.
    assert!(m.portail.relever(&b, 1).unwrap()[0].is_ok());
    m.portail.deposer(&b, m.trame(&b, 2)).unwrap();
    assert!(m.portail.relever(&b, 1).unwrap()[0].is_ok());
    assert!(m.portail.publier(&b, p()).is_ok());

    assert!(m
        .registre
        .evenements()
        .contains(&Evenement::Coupure { service: a.clone(), motif: MotifCoupure::Anomalie }));

    // Rétablissement explicite : désactivée, nouvelle clé de publication, nouvelle session requise.
    let ancienne = m.portail.cle_publication(&a).unwrap();
    assert_eq!(m.portail.activer(&a), Err(ErreurPortail::TransitionRefusee));
    m.portail.retablir(&a).unwrap();
    assert_eq!(m.portail.etat(&a).unwrap(), EtatVoie::Desactivee);
    assert_ne!(m.portail.cle_publication(&a).unwrap(), ancienne);
    m.portail.activer(&a).unwrap();
    assert_eq!(m.portail.deposer(&a, m.trame(&a, 2)), Err(ErreurPortail::SessionAbsente));
}

/// Décodeur qui coupe la voie pendant le décodage : le lot en cours est refusé.
struct CoupeEnRoute {
    poignee: Mutex<Option<PoigneeCoupure>>,
    service: IdService,
}

impl DecodeurIsole for CoupeEnRoute {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        if let Some(p) = self.poignee.lock().unwrap().as_ref() {
            p.couper(&self.service, MotifCoupure::HorsContrat).unwrap();
        }
        SasEnProcessus.decoder_lot(requete)
    }
}

#[test]
fn coupure_pendant_le_decodage() {
    let decodeur = Arc::new(CoupeEnRoute { poignee: Mutex::new(None), service: id("docs") });
    struct Partage(Arc<CoupeEnRoute>);
    impl DecodeurIsole for Partage {
        fn decoder_lot(&self, r: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
            self.0.decoder_lot(r)
        }
    }
    let m = monde(Box::new(Partage(decodeur.clone())), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    *decodeur.poignee.lock().unwrap() = Some(m.portail.poignee_coupure());
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::VoieCoupee]);
}

// ---------------------------------------------------------------- sas (EX-46)

#[test]
fn sas_refuse_entree_malformee_ou_surdimensionnee() {
    let m = reel();
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, b"MSCE\x02\x01\x08\xff\xff\xff\xff".to_vec()).unwrap();
    m.portail.deposer(&s, b"MSCE".to_vec()).unwrap();
    let mut t = m.trame(&s, 1);
    t.push(0);
    m.portail.deposer(&s, t).unwrap();
    let r = erreurs(m.portail.relever(&s, 10).unwrap());
    assert_eq!(
        r,
        vec![
            ErreurPortail::Decodage(RefusDecodage::Taille),
            ErreurPortail::Decodage(RefusDecodage::Taille),
            ErreurPortail::Decodage(RefusDecodage::Taille),
        ]
    );
    // Une trame valide passe ensuite : le refus d'un lot n'empoisonne pas la voie.
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    assert!(m.portail.relever(&s, 1).unwrap()[0].is_ok());
}

/// Mode d'essai : compilation de débogage uniquement.
#[cfg(debug_assertions)]
#[test]
fn sas_plantage_refuse_le_lot() {
    let m = monde(Box::new(sas_reel().avec_essai(Essai::Plantage)), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    m.portail.deposer(&s, m.trame(&s, 2)).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 10).unwrap()), vec![ErreurPortail::Sas(ErreurSas::FinAnormale); 2]);
    assert_eq!(m.portail.compteurs(&s).unwrap().refusees, 2);
}

/// Mode d'essai : compilation de débogage uniquement.
#[cfg(debug_assertions)]
#[test]
fn sas_delai_depasse_tue_et_refuse() {
    let sas = sas_reel().avec_essai(Essai::Boucle).avec_delai(Duration::from_millis(300));
    let m = monde(Box::new(sas), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    let debut = Instant::now();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::Sas(ErreurSas::Delai)]);
    assert!(debut.elapsed() < Duration::from_secs(3));
}

/// Mode d'essai : compilation de débogage uniquement.
#[cfg(debug_assertions)]
#[test]
fn deuxieme_validation_divergente_refusee() {
    let m = monde(Box::new(sas_reel().avec_essai(Essai::Divergence)), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::ValidationDivergente]);
    assert_eq!(m.registre.evenements(), Vec::new());
}

/// Sas « compromis » qui déclare admise une trame que la zone COG refuse.
struct SasComplaisant;

impl DecodeurIsole for SasComplaisant {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let d = declaration(&requete.service, Sens::Entrante, ENTRANT, 1, champs_simples());
        Ok(requete.trames.iter().map(|_| Ok(miyuportail::msce::encoder_interne(&d))).collect())
    }
}

#[test]
fn sas_compromis_contredit_par_la_zone_cog() {
    let m = monde(Box::new(SasComplaisant), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, b"MSCE n'importe quoi".to_vec()).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::ValidationDivergente]);
}

/// Sas qui renvoie un nombre de résultats incohérent.
struct SasIncoherent;

impl DecodeurIsole for SasIncoherent {
    fn decoder_lot(&self, _: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        Ok(Vec::new())
    }
}

#[test]
fn sas_reponse_incoherente_refusee() {
    let m = monde(Box::new(SasIncoherent), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::Sas(ErreurSas::Reponse)]);
}

#[test]
fn sas_binaire_absent_refuse() {
    let m = monde(Box::new(SasProcessus::new("/nonexistent/portail-sas")), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    assert_eq!(erreurs(m.portail.relever(&s, 1).unwrap()), vec![ErreurPortail::Sas(ErreurSas::Lancement)]);
}

#[cfg(target_os = "linux")]
mod seccomp {
    use std::os::unix::process::ExitStatusExt;
    use std::process::{Command, Stdio};

    const SIGSYS: i32 = 31;

    fn lancer(argument: &str) -> std::process::ExitStatus {
        Command::new(env!("CARGO_BIN_EXE_portail-sas"))
            .arg(argument)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
    }

    #[cfg(debug_assertions)]
    #[test]
    fn seccomp_tue_le_sas_qui_ouvre_une_socket() {
        assert_eq!(lancer("--essai=socket").signal(), Some(SIGSYS));
    }

    #[cfg(debug_assertions)]
    #[test]
    fn seccomp_tue_le_sas_qui_ecrit_un_fichier() {
        assert_eq!(lancer("--essai=ecriture").signal(), Some(SIGSYS));
        assert!(!std::path::Path::new("/tmp/portail-sas-essai-ecriture").exists());
    }

    #[test]
    fn argument_inconnu_refuse() {
        assert_eq!(lancer("--debug").code(), Some(miyuportail::sas::SORTIE_ARGUMENTS));
    }
}

// ---------------------------------------------------------------- CDR (EX-45)

fn trame_avec_document(m: &Monde, s: &IdService, seq: u64, doc: &[u8]) -> Vec<u8> {
    let mut c = champs_simples();
    c.push(Champ { tag: 3, valeur: Valeur::Document(doc.to_vec()) });
    sceller(&declaration(s, Sens::Entrante, ENTRANT, seq, c), &[&m.facade, &m.auteur]).unwrap()
}

#[test]
fn cdr_par_defaut_refuse_tout_document() {
    let m = rapide();
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, trame_avec_document(&m, &s, 1, b"%PDF-1.7 /JavaScript")).unwrap();
    m.portail.deposer(&s, m.trame(&s, 2)).unwrap();
    let r = m.portail.relever(&s, 10).unwrap();
    assert_eq!(r[0], Err(ErreurPortail::Desarmement));
    assert!(r[1].is_ok(), "un message sans document traverse");
}

/// CDR d'essai : « reconstruit » en préfixant (montre que les octets d'origine ne traversent pas).
struct CdrEssai;

impl Desarmement for CdrEssai {
    fn desarmer(&self, o: &ObjetADesarmer<'_>) -> Result<ObjetDesarme, RefusDesarmement> {
        if o.octets.windows(10).any(|w| w == b"JavaScript") {
            return Err(RefusDesarmement);
        }
        let mut octets = b"RECONSTRUIT:".to_vec();
        octets.extend_from_slice(o.octets);
        Ok(ObjetDesarme { octets })
    }
}

#[test]
fn cdr_branche_remplace_le_document() {
    let m = monde(Box::new(SasEnProcessus), Box::new(CdrEssai));
    let s = m.voie_ouverte("docs");
    m.portail.deposer(&s, trame_avec_document(&m, &s, 1, b"texte")).unwrap();
    m.portail.deposer(&s, trame_avec_document(&m, &s, 2, b"/JavaScript")).unwrap();
    let r = m.portail.relever(&s, 10).unwrap();
    let d = &r[0].as_ref().unwrap().declaration;
    assert_eq!(d.champs[2].valeur, Valeur::Document(b"RECONSTRUIT:texte".to_vec()));
    assert_eq!(r[1], Err(ErreurPortail::Desarmement));
}

// ---------------------------------------------------------------- voie sortante (EX-47)

#[test]
fn publication_controlee_et_reencodee() {
    let m = rapide();
    let s = m.voie_ouverte("docs");
    let p = |etiquette, declencheur| Publication {
        type_code: SORTANT,
        champs: vec![Champ { tag: 1, valeur: Valeur::Texte("compte rendu".into()) }],
        etiquette,
        declencheur,
    };
    assert_eq!(
        m.portail.publier(&s, p(Sensibilite::Nationale, Declencheur::Utilisateur)),
        Err(ErreurPortail::SortieNationale)
    );
    assert_eq!(
        m.portail.publier(&s, p(Sensibilite::Publiable, Declencheur::Maia)),
        Err(ErreurPortail::ActionUtilisateurRequise)
    );
    assert_eq!(
        m.portail.publier(&s, p(Sensibilite::Session, Declencheur::Service)),
        Err(ErreurPortail::ActionUtilisateurRequise)
    );
    let mut hors_traite = p(Sensibilite::Publiable, Declencheur::Utilisateur);
    hors_traite.type_code = ENTRANT;
    assert_eq!(m.portail.publier(&s, hors_traite), Err(ErreurPortail::Decodage(RefusDecodage::TypeDeclaration)));
    let mut doc = p(Sensibilite::Publiable, Declencheur::Utilisateur);
    doc.champs.push(Champ { tag: 2, valeur: Valeur::Document(b"png".to_vec()) });
    assert_eq!(m.portail.publier(&s, doc), Err(ErreurPortail::Desarmement));

    assert_eq!(m.portail.publier(&s, p(Sensibilite::Session, Declencheur::Utilisateur)), Ok(1));
    assert_eq!(m.portail.publier(&s, p(Sensibilite::Publiable, Declencheur::Utilisateur)), Ok(2));

    // La façade lit une trame MSCE neuve, signée par la seule clé de la voie.
    let trame = m.portail.lire_publication(&s).unwrap().unwrap();
    let traite = traite();
    let ancres = Ancres { emetteur: m.portail.cle_publication(&s).unwrap(), auteurs: vec![] };
    let ctx = Contexte { service: &s, sens: Sens::Sortante, traite: &traite, ancres: &ancres };
    let d = decoder_trame(&trame, &ctx).unwrap();
    assert_eq!(d.sequence, 1);
    assert_eq!(d.champs[0].valeur, Valeur::Texte("compte rendu".into()));
    assert_eq!(m.portail.compteurs(&s).unwrap().lues, 1);
    assert!(matches!(m.registre.evenements()[0], Evenement::Publication { .. }));
}

// ---------------------------------------------------------------- registre

struct RegistreEnPanne;

impl Inscription for RegistreEnPanne {
    fn inscrire(&self, _: &Evenement) -> Result<(), ErreurInscription> {
        Err(ErreurInscription)
    }
}

#[test]
fn inscription_impossible_refuse_le_passage() {
    let portail = PortailInterne::new(Box::new(SasEnProcessus), Box::new(RefusParDefaut), Arc::new(RegistreEnPanne));
    let (f, a) = (cle(7), cle(9));
    let s = id("docs");
    portail.declarer(contrat("docs", ClasseExposition::H2)).unwrap();
    portail.activer(&s).unwrap();
    portail.ouvrir_session(&s, f.verifying_key().to_bytes(), vec![a.verifying_key().to_bytes()]).unwrap();
    let t = sceller(&declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples()), &[&f, &a]).unwrap();
    portail.deposer(&s, t).unwrap();
    assert_eq!(erreurs(portail.relever(&s, 1).unwrap()), vec![ErreurPortail::Inscription]);
    // La coupure reste effective même si l'incident ne peut pas être inscrit.
    assert_eq!(portail.couper(&s, MotifCoupure::Decision), Err(ErreurPortail::Inscription));
    assert_eq!(portail.etat(&s).unwrap(), EtatVoie::Coupee(MotifCoupure::Decision));
}
