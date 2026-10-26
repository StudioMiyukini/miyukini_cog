//! @id     miyuportail_tests_scission
//! @do     tester_portail_scinde_entree_sortie
//! @role   test
//! @layer  toolkit
//! @human  miyuportail — Portail scindé (A-022 §3) : l'entrée refuse toute publication, la sortie toute relève, jeton à usage unique consommé une fois (même après redémarrage), panne de l'un sans arrêt de l'autre, permissions au démarrage.

#![cfg(target_os = "linux")]
#![allow(clippy::many_single_char_names)]

mod commun;

use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use commun::*;
use ed25519_dalek::SigningKey;
use miyuportail::demon::{
    controler_repertoire, nom_cle_publication, verifier_cloisonnement, CheminsEntree, CheminsSortie, DemonEntree,
    DemonSortie, ErreurDemon,
};
use miyuportail::files::{
    ecrire_descripteur, Admise, DescripteurVoie, File, PublicationH1, Reponse, Role, SessionDecrite, EXT_ADMISE,
    EXT_MSCE, EXT_PUBLICATION, EXT_REPONSE,
};
use miyuportail::msce::{decoder_trame, sceller, Contexte};
use miyuportail::*;

static N: AtomicU64 = AtomicU64::new(0);

fn pk(c: &SigningKey) -> [u8; 32] {
    c.verifying_key().to_bytes()
}

/// Arborescence d'essai : un répertoire par file et par état, aucun commun.
struct Banc {
    racine: PathBuf,
    voies_e: PathBuf,
    voies_s: PathBuf,
    etat_e: PathBuf,
    etat_s: PathBuf,
    depot: PathBuf,
    admises: PathBuf,
    reponses: PathBuf,
    publication: PathBuf,
    facade: SigningKey,
    auteur: SigningKey,
}

impl Banc {
    fn new() -> Self {
        let racine = std::env::temp_dir().join(format!(
            "miyuportail-scission-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let d = |n: &str| {
            let p = racine.join(n);
            std::fs::create_dir_all(&p).unwrap();
            std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o700)).unwrap();
            p
        };
        Self {
            voies_e: d("voies-entree"),
            voies_s: d("voies-sortie"),
            etat_e: d("etat-entree"),
            etat_s: d("etat-sortie"),
            depot: d("depot"),
            admises: d("admises"),
            reponses: d("reponses"),
            publication: d("publication"),
            racine,
            facade: cle(7),
            auteur: cle(9),
        }
    }

    fn entree(&self) -> DemonEntree {
        let c = CheminsEntree {
            voies: self.voies_e.clone(),
            etat: self.etat_e.clone(),
            depot: self.depot.clone(),
            admises: self.admises.clone(),
            interdits: vec![self.reponses.clone(), self.publication.clone()],
        };
        DemonEntree::new(c, Box::new(SasEnProcessus), Box::new(RefusParDefaut)).unwrap()
    }

    fn sortie(&self) -> DemonSortie {
        let c = CheminsSortie {
            voies: self.voies_s.clone(),
            etat: self.etat_s.clone(),
            reponses: self.reponses.clone(),
            publication: self.publication.clone(),
            interdits: vec![self.depot.clone(), self.admises.clone()],
        };
        DemonSortie::new(c, Box::new(RefusParDefaut)).unwrap()
    }

    /// Les Cores : un descripteur par moitié (graine des jetons à l'entrée seule).
    fn cores(&self, contrat: ContratVoies, etat: EtatVoie, rotation: u64, session: Option<([u8; 32], &CleJetons)>) {
        let h2 = contrat.classe != ClasseExposition::H1;
        let e = DescripteurVoie {
            role: Role::Entree,
            contrat: contrat.clone(),
            etat,
            rotation,
            session: session.filter(|_| h2).map(|(id, j)| SessionDecrite::Entree {
                id,
                cle_facade: pk(&self.facade),
                auteurs: vec![pk(&self.auteur)],
                graine_jetons: j.graine(),
            }),
        };
        let s = DescripteurVoie {
            role: Role::Sortie,
            contrat,
            etat,
            rotation,
            session: session.filter(|_| h2).map(|(id, j)| SessionDecrite::Sortie { id, ancre_jetons: j.ancre() }),
        };
        if h2 {
            ecrire_descripteur(&self.voies_e, &e, None).unwrap();
        }
        ecrire_descripteur(&self.voies_s, &s, None).unwrap();
    }

    fn deposer(&self, s: &IdService, sequence: u64) -> Vec<u8> {
        let t = sceller(
            &declaration(s, Sens::Entrante, ENTRANT, sequence, champs_simples()),
            &[&self.facade, &self.auteur],
        )
        .unwrap();
        File::new(&self.depot).ecrire(s, EXT_MSCE, &t).unwrap();
        t
    }

    fn noms(dir: &Path, ext: &str) -> Vec<String> {
        File::new(dir).elements(10_000).unwrap().into_iter().filter(|e| e.extension == ext).map(|e| e.nom).collect()
    }

    /// Le moteur : relève les déclarations admises (et les retire de la file).
    fn moteur_relever(&self) -> Vec<Admise> {
        let f = File::new(&self.admises);
        Self::noms(&self.admises, EXT_ADMISE)
            .into_iter()
            .map(|n| {
                let a = Admise::decoder(&f.lire(&n, 1 << 22).unwrap()).unwrap();
                f.retirer(&n).unwrap();
                a
            })
            .collect()
    }

    /// Le moteur : écrit une réponse portant le jeton reçu.
    fn moteur_repondre(&self, jeton_octets: &[u8], texte: &str) {
        let jeton = JetonReponse::decoder(jeton_octets).unwrap();
        let s = jeton.service().clone();
        let mut m =
            declaration(&s, Sens::Sortante, SORTANT, 0, vec![Champ { tag: 1, valeur: Valeur::Texte(texte.into()) }]);
        m.emise_le = 0;
        let r = Reponse { jeton, messages: vec![m] };
        File::new(&self.reponses).ecrire(&s, EXT_REPONSE, &r.encoder().unwrap()).unwrap();
    }

    /// La façade : lit et vérifie les publications (clé publiée par la sortie).
    fn facade_lire(&self, s: &IdService) -> Vec<Declaration> {
        let f = File::new(&self.publication);
        let cle_hex = String::from_utf8(f.lire(&nom_cle_publication(s), 200).unwrap()).unwrap();
        let mut cle = [0u8; 32];
        for (i, o) in cle.iter_mut().enumerate() {
            *o = u8::from_str_radix(&cle_hex[2 * i..2 * i + 2], 16).unwrap();
        }
        let ancres = Ancres { emetteur: cle, auteurs: Vec::new() };
        let traite = traite();
        let ctx = Contexte { service: s, sens: Sens::Sortante, traite: &traite, ancres: &ancres };
        Self::noms(&self.publication, EXT_MSCE)
            .into_iter()
            .map(|n| {
                let d = decoder_trame(&f.lire(&n, 1 << 20).unwrap(), &ctx).unwrap();
                f.retirer(&n).unwrap();
                d
            })
            .collect()
    }
}

impl Drop for Banc {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.racine);
    }
}

fn voie_h2(b: &Banc, nom: &str) -> (IdService, CleJetons, [u8; 32]) {
    let jetons = CleJetons::generer();
    let id_session = [nom.len() as u8; 32];
    b.cores(contrat(nom, ClasseExposition::H2), EtatVoie::Active, 0, Some((id_session, &jetons)));
    (id(nom), jetons, id_session)
}

// ---------------------------------------------------------------- bout en bout

#[test]
fn bout_en_bout_par_les_files_a_sens_unique() {
    let b = Banc::new();
    let (s, _, _) = voie_h2(&b, "jayrdv");
    let (mut e, mut so) = (b.entree(), b.sortie());
    so.une_passe();
    e.une_passe();

    let trame = b.deposer(&s, 1);
    let bilan = e.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (1, 1, 0));
    assert!(Banc::noms(&b.depot, EXT_MSCE).is_empty(), "dépôt relevé (acquittement par disparition)");
    let admises = b.moteur_relever();
    assert_eq!(admises.len(), 1);
    assert_eq!(admises[0].declaration.sequence, 1);
    assert_eq!(admises[0].jeton.empreinte(), &sha256(&trame), "jeton lié à la trame déposée");

    b.moteur_repondre(admises[0].jeton.octets(), "créneau 10h");
    let bilan = so.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (1, 1, 0));
    let lues = b.facade_lire(&s);
    assert_eq!(lues.len(), 1);
    assert_eq!(lues[0].type_code, SORTANT);
    assert_eq!(lues[0].sequence, 1);

    // Journaux propres à chaque processus, dans son état seulement.
    let je = std::fs::read_to_string(b.etat_e.join("registre.journal")).unwrap();
    let js = std::fs::read_to_string(b.etat_s.join("registre.journal")).unwrap();
    assert!(je.starts_with("entree jayrdv ") && !je.contains("publication"));
    assert!(js.starts_with("publication jayrdv ") && !js.contains("entree"));
}

fn sha256(o: &[u8]) -> [u8; 32] {
    use sha2::Digest;
    sha2::Sha256::digest(o).into()
}

// ---------------------------------------------------------------- l'entrée ne publie pas

#[test]
fn l_entree_refuse_toute_tentative_de_publication() {
    let b = Banc::new();
    let (s, jetons, id_session) = voie_h2(&b, "jayrdv");
    let mut e = b.entree();
    e.une_passe();

    // Une réponse valide, une publication H1 et une trame d'auteur glissées
    // dans le dépôt : l'entrée ne sait que relever des trames MSCE.
    let jeton = jetons.emettre(&s, &id_session, &[3; 32], 1);
    let m = declaration(&s, Sens::Sortante, SORTANT, 0, vec![Champ { tag: 1, valeur: Valeur::Texte("x".into()) }]);
    let mut m0 = m.clone();
    m0.emise_le = 0;
    let rep = Reponse { jeton, messages: vec![m0.clone()] }.encoder().unwrap();
    let pubh1 = PublicationH1 { etiquette: Sensibilite::Publiable, declencheur: Declencheur::Utilisateur, message: m0 }
        .encoder()
        .unwrap();
    let depot = File::new(&b.depot);
    depot.ecrire(&s, EXT_REPONSE, &rep).unwrap();
    depot.ecrire(&s, EXT_PUBLICATION, &pubh1).unwrap();
    depot.ecrire(&s, EXT_MSCE, &rep).unwrap(); // réponse déguisée en trame
    let bilan = e.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (0, 0, 3));
    assert!(std::fs::read_dir(&b.depot).unwrap().next().is_none(), "tout retiré du dépôt");
    assert!(std::fs::read_dir(&b.admises).unwrap().next().is_none(), "rien remis au moteur");
    assert!(std::fs::read_dir(&b.publication).unwrap().next().is_none(), "rien publié");

    // L'entrée ne démarre pas si elle peut écrire dans une file de publication.
    assert_eq!(e.verifier_cloisonnement(), Err(ErreurDemon::Cloisonnement(b.reponses.clone())));
    if rustix::process::geteuid().as_raw() != 0 {
        for d in [&b.reponses, &b.publication] {
            std::fs::set_permissions(d, std::fs::Permissions::from_mode(0o500)).unwrap();
        }
        assert_eq!(e.verifier_cloisonnement(), Ok(()));
        assert_eq!(e.verifier_au_demarrage(), Ok(()));
    }
}

// ---------------------------------------------------------------- la sortie ne relève pas

#[test]
fn la_sortie_refuse_toute_releve() {
    let b = Banc::new();
    let (s, jetons, id_session) = voie_h2(&b, "jayrdv");
    let mut so = b.sortie();
    so.une_passe();

    // Une trame d'auteur valide, une déclaration admise : la sortie n'en remet rien.
    let t = sceller(&declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples()), &[&b.facade, &b.auteur]).unwrap();
    let admise = Admise {
        jeton: jetons.emettre(&s, &id_session, &[1; 32], 1),
        declaration: declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples()),
    }
    .encoder();
    let rep = File::new(&b.reponses);
    rep.ecrire(&s, EXT_MSCE, &t).unwrap();
    rep.ecrire(&s, EXT_ADMISE, &admise).unwrap();
    rep.ecrire(&s, EXT_REPONSE, &t).unwrap(); // trame déguisée en réponse
    let bilan = so.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (0, 0, 3));
    assert!(std::fs::read_dir(&b.reponses).unwrap().next().is_none());
    assert!(std::fs::read_dir(&b.admises).unwrap().next().is_none(), "rien vers le moteur");
    assert!(Banc::noms(&b.publication, EXT_MSCE).is_empty(), "rien publié");

    assert_eq!(so.verifier_cloisonnement(), Err(ErreurDemon::Cloisonnement(b.depot.clone())));
}

// ---------------------------------------------------------------- jeton à usage unique

#[test]
fn jeton_consomme_une_fois_meme_apres_redemarrage() {
    let b = Banc::new();
    let (s, _, _) = voie_h2(&b, "jayrdv");
    let (mut e, mut so) = (b.entree(), b.sortie());
    so.une_passe();
    e.une_passe();
    b.deposer(&s, 1);
    e.une_passe();
    let a = b.moteur_relever().remove(0);

    b.moteur_repondre(a.jeton.octets(), "un");
    b.moteur_repondre(a.jeton.octets(), "deux");
    let bilan = so.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (1, 1, 1), "second usage refusé");

    // Redémarrage de la sortie : le jeton consommé le reste.
    drop(so);
    let mut so = b.sortie();
    so.une_passe();
    b.moteur_repondre(a.jeton.octets(), "trois");
    let bilan = so.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (0, 0, 1));
    let lues = b.facade_lire(&s);
    assert_eq!(lues.len(), 1);
    assert_eq!(lues[0].champs[0].valeur, Valeur::Texte("un".into()));
}

#[test]
fn jeton_verifie_par_la_sortie() {
    let registre = std::sync::Arc::new(RegistreMemoire::new());
    let sortie = PortailSortie::new(Box::new(RefusParDefaut), registre);
    let s = id("jayrdv");
    sortie.declarer(contrat("jayrdv", ClasseExposition::H2)).unwrap();
    sortie.activer(&s).unwrap();
    let jetons = CleJetons::generer();
    let session = [5; 32];
    sortie.ouvrir_session(&s, SessionSortie { id: session, ancre_jetons: jetons.ancre() }).unwrap();
    let p = || {
        vec![Publication {
            type_code: SORTANT,
            champs: vec![Champ { tag: 1, valeur: Valeur::Texte("r".into()) }],
            etiquette: Sensibilite::Session,
            declencheur: Declencheur::Utilisateur,
        }]
    };

    // Clé étrangère, autre session, autre service : refusés.
    let faux = CleJetons::generer().emettre(&s, &session, &[1; 32], 1);
    assert_eq!(sortie.repondre(&s, &faux, p()), Err(ErreurPortail::JetonInvalide));
    let autre_session = jetons.emettre(&s, &[6; 32], &[1; 32], 1);
    assert_eq!(sortie.repondre(&s, &autre_session, p()), Err(ErreurPortail::JetonInvalide));
    let autre_service = jetons.emettre(&id("autre"), &session, &[1; 32], 1);
    assert_eq!(sortie.repondre(&s, &autre_service, p()), Err(ErreurPortail::JetonInvalide));

    // Bon jeton : une fois.
    let bon = jetons.emettre(&s, &session, &[1; 32], 1);
    assert_eq!(sortie.repondre(&s, &bon, p()), Ok(vec![1]));
    assert_eq!(sortie.repondre(&s, &bon, p()), Err(ErreurPortail::JetonDejaUtilise));
    let recopie = JetonReponse::decoder(bon.octets()).unwrap();
    assert_eq!(sortie.repondre(&s, &recopie, p()), Err(ErreurPortail::JetonDejaUtilise));

    // H2 : aucune publication sans jeton ; session close : jetons périmés.
    assert_eq!(sortie.publier_h1(&s, p().remove(0)), Err(ErreurPortail::JetonRequis));
    sortie.fermer_session(&s).unwrap();
    let apres = jetons.emettre(&s, &session, &[2; 32], 1);
    assert_eq!(sortie.repondre(&s, &apres, p()), Err(ErreurPortail::JetonInvalide));
    assert_eq!(sortie.compteurs(&s).unwrap().publiees, 1);
}

#[test]
fn publication_h1_sans_jeton() {
    let b = Banc::new();
    b.cores(contrat("site", ClasseExposition::H1), EtatVoie::Active, 0, None);
    let s = id("site");
    let mut so = b.sortie();
    so.une_passe();
    let mut m =
        declaration(&s, Sens::Sortante, SORTANT, 0, vec![Champ { tag: 1, valeur: Valeur::Texte("page".into()) }]);
    m.emise_le = 0;
    let f = File::new(&b.reponses);
    for (etiquette, declencheur) in [
        (Sensibilite::Publiable, Declencheur::Utilisateur),
        (Sensibilite::Nationale, Declencheur::Utilisateur),
        (Sensibilite::Publiable, Declencheur::Maia),
    ] {
        let p = PublicationH1 { etiquette, declencheur, message: m.clone() };
        f.ecrire(&s, EXT_PUBLICATION, &p.encoder().unwrap()).unwrap();
    }
    let bilan = so.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (1, 1, 2));
    assert_eq!(b.facade_lire(&s).len(), 1);
}

// ---------------------------------------------------------------- panne de l'un, l'autre continue

#[test]
fn la_panne_de_l_un_n_arrete_pas_l_autre() {
    let b = Banc::new();
    let (s, _, _) = voie_h2(&b, "jayrdv");
    let (mut e, mut so) = (b.entree(), b.sortie());
    so.une_passe();
    e.une_passe();
    let t1 = b.deposer(&s, 1);
    b.deposer(&s, 2);
    e.une_passe();
    let admises = b.moteur_relever();
    assert_eq!(admises.len(), 2);

    // L'entrée tombe : la sortie répond aux déclarations déjà admises.
    drop(e);
    b.moteur_repondre(admises[0].jeton.octets(), "a");
    assert_eq!(so.une_passe().emis, 1);

    // La sortie tombe : l'entrée (relancée) admet encore, sans rejeu.
    drop(so);
    let mut e = b.entree();
    e.une_passe();
    File::new(&b.depot).ecrire(&s, EXT_MSCE, &t1).unwrap(); // rejeu d'une trame déjà admise
    b.deposer(&s, 3);
    let bilan = e.une_passe();
    assert_eq!((bilan.recus, bilan.emis, bilan.refuses), (2, 1, 1), "marques persistées : rejeu refusé");
    assert_eq!(b.moteur_relever()[0].declaration.sequence, 3);

    // La sortie revient : la réponse en attente passe, avec la même clé.
    let mut so = b.sortie();
    so.une_passe();
    b.moteur_repondre(admises[1].jeton.octets(), "b");
    assert_eq!(so.une_passe().emis, 1);
    let lues = b.facade_lire(&s);
    assert_eq!(lues.iter().map(|d| d.sequence).collect::<Vec<_>>(), vec![1, 2], "séquence de publication continue");
}

// ---------------------------------------------------------------- coupure, rétablissement

#[test]
fn coupure_purge_et_rotation_de_cle() {
    let b = Banc::new();
    let (s, jetons, id_session) = voie_h2(&b, "jayrdv");
    let (mut e, mut so) = (b.entree(), b.sortie());
    so.une_passe();
    e.une_passe();
    let cle_avant = File::new(&b.publication).lire(&nom_cle_publication(&s), 200).unwrap();
    b.deposer(&s, 1);
    e.une_passe();
    let a = b.moteur_relever().remove(0);

    b.cores(contrat("jayrdv", ClasseExposition::H2), EtatVoie::Coupee(MotifCoupure::Anomalie), 0, None);
    e.une_passe();
    so.une_passe();
    b.deposer(&s, 2);
    b.moteur_repondre(a.jeton.octets(), "après coupure");
    assert_eq!(e.une_passe().refuses, 1);
    assert_eq!(so.une_passe().refuses, 1);
    assert!(!File::new(&b.publication).existe(&nom_cle_publication(&s)), "clé retirée à la coupure");

    // Rétablie puis réactivée, même session : nouvelle clé, ancien jeton périmé (session neuve exigée).
    b.cores(contrat("jayrdv", ClasseExposition::H2), EtatVoie::Active, 1, Some((id_session, &jetons)));
    so.une_passe();
    let cle_apres = File::new(&b.publication).lire(&nom_cle_publication(&s), 200).unwrap();
    assert_ne!(cle_avant, cle_apres);
    let _ = jetons;
}

// ---------------------------------------------------------------- permissions au démarrage

#[test]
fn repertoire_trop_ouvert_refuse_au_demarrage() {
    let b = Banc::new();
    std::fs::set_permissions(&b.admises, std::fs::Permissions::from_mode(0o3735)).unwrap();
    let e = b.entree();
    assert!(matches!(e.verifier_au_demarrage(), Err(ErreurDemon::Permissions(p, _)) if p == b.admises));
    std::fs::set_permissions(&b.admises, std::fs::Permissions::from_mode(0o3730)).unwrap();
    std::fs::set_permissions(&b.voies_s, std::fs::Permissions::from_mode(0o751)).unwrap();
    assert!(matches!(b.sortie().verifier_au_demarrage(), Err(ErreurDemon::Permissions(p, _)) if p == b.voies_s));

    // Mauvais propriétaire attendu, lien symbolique : refusés.
    let moi = std::fs::metadata(&b.etat_e).unwrap().uid();
    assert!(controler_repertoire(&b.etat_e, Some(moi.wrapping_add(1))).is_err());
    let lien = b.racine.join("lien");
    std::os::unix::fs::symlink(&b.etat_e, &lien).unwrap();
    assert!(controler_repertoire(&lien, None).is_err());
    assert!(controler_repertoire(&b.etat_e, Some(moi)).is_ok());
    assert_eq!(verifier_cloisonnement(&[b.racine.join("absent")]), Ok(()));
}
