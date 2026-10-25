//! Tests de bout en bout de l'éditeur du registre de révocation (côté COG).
//!
//! - Origin : un serveur axum **en processus** reproduit la route
//!   `POST /api/auth/revocations/sync` d'Origin (`apps/origin/src/web/revocations.rs`,
//!   binaire sans bibliothèque, donc non importable) : enveloppe vérifiée avec
//!   le format **partagé** (`miyuregistre::enveloppe`, celui qu'Origin
//!   utilise), anti-rejeu, attente par morceaux à position exacte, `409` avec
//!   la position attendue, tête appliquée par `SuiviRevocationsRegistre`.
//! - MiyuCloud : le dépôt est relu par une **copie conforme** de la lecture de
//!   MiyuCloud (`lire_depot` de `apps/miyucloud/.../miyu_revocations.rs` : tête
//!   puis entrées, lectures bornées, copie privée ouverte par
//!   `StockageFichier`), puis appliqué à `SuiviRevocationsRegistre` (le type
//!   qu'utilise MiyuCloud). Une dépendance de test vers `miyucloud-server`
//!   serait trop lourde.
#![cfg(all(feature = "publier", unix))]

use std::collections::HashSet;
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{DefaultBodyLimit, State};
use axum::http::StatusCode;
use axum::routing::post;
use ed25519_dalek::{Signature, VerifyingKey};
use miyauth::http::b64_decoder;
use miyauth::{CibleRevocation, CogId, SourceRevocation};
use miyuregistre::editeur::depot::{DepotRevocations, ErreurDepot, FICHIER_ENTREES, FICHIER_TETE};
use miyuregistre::editeur::http::TransportHttp;
use miyuregistre::editeur::origin::{maintenant_unix, ErreurOrigin, PublicationOrigin, ReglagesOrigin};
use miyuregistre::editeur::Instantane;
use miyuregistre::enveloppe::{
    octets_enveloppe, DUREE_MAX_ENVELOPPE_SECS, MAX_ENTREES_SYNCHRO, ROUTE_REVOCATIONS, TAILLE_MAX_CORPS_REVOCATIONS,
};
use miyuregistre::{
    Cible, Corps, EntreeRegistre, Journal, RefusSuivi, Stockage, StockageFichier, StockageMemoire,
    SuiviRevocationsRegistre,
};
use miyusceau::{sceller, CleSignature};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REGISTRE: [u8; 32] = [0x52; 32];

fn cog(c: char) -> CogId {
    CogId::new(&c.to_string().repeat(26)).unwrap()
}

fn kid(i: u32) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..4].copy_from_slice(&i.to_be_bytes());
    k
}

fn revocation(cle: &CleSignature, i: u32) -> Vec<u8> {
    let e = EntreeRegistre { inscrite_a: 1_800_000_000, corps: Corps::Revocation { cible: Cible::Cle(kid(i)), motif: "test".into() } };
    sceller(&e, cle).unwrap()
}

fn journal_memoire(cle: &CleSignature, n: u32) -> Journal<StockageMemoire> {
    let mut j = Journal::ouvrir(REGISTRE, StockageMemoire::new(), StockageMemoire::new()).unwrap();
    for i in 0..n {
        j.ajouter(&revocation(cle, i)).unwrap();
    }
    j
}

// ---------------------------------------------------------------------------
// Origin de test
// ---------------------------------------------------------------------------

#[derive(Default)]
struct EtatOrigin {
    attente: Vec<Vec<u8>>,
    base: u64,
    vues: HashSet<[u8; 32]>,
}

struct OriginTest {
    cog: CogId,
    cle: VerifyingKey,
    suivi: SuiviRevocationsRegistre,
    etat: Mutex<EtatOrigin>,
    /// Statuts rendus, dans l'ordre.
    statuts: Mutex<Vec<u16>>,
    /// Après ce nombre de morceaux acceptés, l'attente est perdue une fois (redémarrage simulé).
    perte_apres: AtomicU32,
}

impl OriginTest {
    fn new(cog: CogId, cle_publique: [u8; 32]) -> Arc<Self> {
        Arc::new(Self {
            suivi: SuiviRevocationsRegistre::new(cog.clone(), REGISTRE, &cle_publique).unwrap(),
            cog,
            cle: VerifyingKey::from_bytes(&cle_publique).unwrap(),
            etat: Mutex::default(),
            statuts: Mutex::default(),
            perte_apres: AtomicU32::new(u32::MAX),
        })
    }

    fn statuts(&self) -> Vec<u16> {
        self.statuts.lock().unwrap().clone()
    }

    /// Même logique que `RevocationsReconnues::{analyser, authentifier, synchroniser}` d'Origin.
    fn traiter(&self, corps: &[u8], maintenant: u64) -> (u16, Value) {
        let refus = |s: u16, motif: &str| (s, json!({"ok": false, "error": motif}));
        if corps.len() > TAILLE_MAX_CORPS_REVOCATIONS {
            return refus(413, "payload_too_large");
        }
        let Ok(v) = serde_json::from_slice::<Value>(corps) else { return refus(400, "invalid_json") };
        let Some(brutes) = v["entrees"].as_array() else { return refus(400, "invalid_json") };
        if brutes.len() > MAX_ENTREES_SYNCHRO {
            return refus(413, "trop_d_entrees");
        }
        if v["cog_id"].as_str() != Some(self.cog.as_str()) {
            return refus(403, "cog_inconnu");
        }
        let (Some(echeance), Some(signature)) = (v["echeance"].as_u64(), v["signature"].as_str()) else {
            return refus(401, "non_authentifie");
        };
        let Some(signature) = b64_decoder(signature).and_then(|s| <[u8; 64]>::try_from(s).ok()) else {
            return refus(401, "non_authentifie");
        };
        let depuis = v["depuis"].as_u64().unwrap_or(u64::MAX);
        let tete = v["tete"].as_str().map(|t| b64_decoder(t).unwrap());
        let entrees: Vec<Vec<u8>> = brutes.iter().map(|e| b64_decoder(e.as_str().unwrap()).unwrap()).collect();
        if echeance < maintenant || echeance > maintenant + DUREE_MAX_ENVELOPPE_SECS + 120 {
            return refus(401, "enveloppe_expiree");
        }
        let message = octets_enveloppe(&self.cog, depuis, tete.as_deref(), &entrees, echeance);
        if self.cle.verify_strict(&message, &Signature::from_bytes(&signature)).is_err() {
            return refus(401, "non_authentifie");
        }

        let mut etat = self.etat.lock().unwrap();
        let taille = self.suivi.taille().unwrap();
        if !etat.attente.is_empty() && etat.base != taille {
            etat.attente.clear();
        }
        let attendu = taille + etat.attente.len() as u64;
        let empreinte: [u8; 32] = Sha256::digest(&message).into();
        if !etat.vues.insert(empreinte) {
            return (200, json!({"ok": true, "taille": taille, "en_attente": etat.attente.len(), "ajoutees": 0, "rejeu": true}));
        }
        let position = |taille: u64, attendu: u64| (409, json!({"ok": false, "error": "position", "taille": taille, "attendu": attendu}));
        let Some(tete) = tete else {
            if depuis != attendu {
                return position(taille, attendu);
            }
            if etat.attente.is_empty() {
                etat.base = taille;
            }
            etat.attente.extend(entrees);
            let en_attente = etat.attente.len();
            if self.perte_apres.fetch_sub(1, Ordering::SeqCst) == 1 {
                etat.attente.clear(); // « redémarrage » après la réponse
            }
            return (200, json!({"ok": true, "taille": taille, "en_attente": en_attente, "ajoutees": 0, "rejeu": false}));
        };
        let resultat = if etat.attente.is_empty() {
            self.suivi.appliquer(&tete, depuis, &entrees, maintenant)
        } else if depuis != attendu {
            return position(taille, attendu);
        } else {
            let mut toutes = std::mem::take(&mut etat.attente);
            toutes.extend(entrees);
            self.suivi.appliquer(&tete, taille, &toutes, maintenant)
        };
        etat.attente.clear();
        match resultat {
            Ok(a) => (200, json!({"ok": true, "taille": a.taille, "en_attente": 0, "ajoutees": a.ajoutees, "rejeu": a.rejeu})),
            Err(RefusSuivi::Couverture { taille, .. }) => position(taille, taille),
            Err(RefusSuivi::Bornes) => refus(413, "bornes"),
            Err(_) => refus(400, "rejected"),
        }
    }
}

async fn route(State(o): State<Arc<OriginTest>>, corps: axum::body::Bytes) -> (StatusCode, String) {
    let (statut, json) = o.traiter(&corps, maintenant_unix());
    o.statuts.lock().unwrap().push(statut);
    (StatusCode::from_u16(statut).unwrap(), json.to_string())
}

/// Démarre `origin` sur la boucle locale ; rend l'URL de base.
fn demarrer(origin: Arc<OriginTest>) -> String {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(2).enable_all().build().unwrap();
        rt.block_on(async move {
            let ecoute = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            tx.send(ecoute.local_addr().unwrap().port()).unwrap();
            let app = axum::Router::new()
                .route(ROUTE_REVOCATIONS, post(route))
                .layer(DefaultBodyLimit::max(TAILLE_MAX_CORPS_REVOCATIONS + 4096))
                .with_state(origin);
            axum::serve(ecoute, app).await.unwrap();
        });
    });
    format!("http://127.0.0.1:{}", rx.recv_timeout(Duration::from_secs(10)).unwrap())
}

fn client(cog: CogId, cle: &CleSignature, url: &str) -> PublicationOrigin<TransportHttp> {
    let reglages = ReglagesOrigin { intervalle_min: Duration::ZERO, ..ReglagesOrigin::default() };
    PublicationOrigin::new(cog, cle, TransportHttp::new(url).unwrap()).avec_reglages(reglages)
}

#[test]
fn registre_de_5000_entrees_publie_vers_origin_de_test() {
    let cle = CleSignature::generer();
    let mut journal = journal_memoire(&cle, 5000);
    let origin = OriginTest::new(cog('h'), cle.cle_publique());
    let mut c = client(cog('h'), &cle, &demarrer(origin.clone()));

    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();
    let b = c.publier(&inst).unwrap();
    // Sonde (tête seule, 409 attendu 0), morceau de 4096, puis tête + 904 entrées.
    assert_eq!(origin.statuts(), vec![409, 200, 200]);
    assert_eq!((b.envois, b.conflits, b.taille, b.ajoutees), (3, 1, 5000, 5000));
    let t = maintenant_unix();
    assert_eq!(origin.suivi.est_revoque(&cog('h'), &CibleRevocation::Cle(kid(4999)), t), Ok(true));
    assert_eq!(origin.suivi.est_revoque(&cog('h'), &CibleRevocation::Cle(kid(5000)), t), Ok(false));

    // Cycle suivant : position connue, un seul envoi (tête + nouvelle entrée).
    journal.ajouter(&revocation(&cle, 5000)).unwrap();
    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();
    let b = c.publier(&inst).unwrap();
    assert_eq!((b.envois, b.conflits, b.taille, b.ajoutees), (1, 0, 5001, 1));
    assert_eq!(origin.suivi.est_revoque(&cog('h'), &CibleRevocation::Cle(kid(5000)), maintenant_unix()), Ok(true));
    // Sans entrée nouvelle : tête fraîche seule (fraîcheur tenue).
    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();
    assert_eq!(c.publier(&inst).unwrap().envois, 1);
    assert_eq!(origin.suivi.a_jour_au(), Some(inst.emise_a));
}

#[test]
fn reprise_apres_409_et_arret_sur_401_403() {
    let cle = CleSignature::generer();
    let mut journal = journal_memoire(&cle, 5000);
    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();

    // Origin perd son attente après le premier morceau (redémarrage) : la tête
    // reçoit 409 (attendu 0), le client reprend à 0 et aboutit.
    let origin = OriginTest::new(cog('h'), cle.cle_publique());
    origin.perte_apres.store(1, Ordering::SeqCst);
    let mut c = client(cog('h'), &cle, &demarrer(origin.clone()));
    let b = c.publier(&inst).unwrap();
    assert_eq!(origin.statuts(), vec![409, 200, 409, 200, 200]);
    assert_eq!((b.conflits, b.taille, b.ajoutees), (2, 5000, 5000));
    assert_eq!(c.position(), Some(5000));

    // Origin qui n'a pas épinglé cette clé : 401, arrêt immédiat (aucun nouvel essai).
    let autre = OriginTest::new(cog('h'), CleSignature::generer().cle_publique());
    let mut c = client(cog('h'), &cle, &demarrer(autre.clone()));
    let e = c.publier(&inst).unwrap_err();
    assert!(e.est_definitive() && matches!(e, ErreurOrigin::Refusee { statut: 401, .. }), "{e}");
    assert_eq!(autre.statuts(), vec![401]);
    assert_eq!(autre.suivi.taille(), Some(0));

    // COG non déclaré : 403, arrêt.
    let mut c = client(cog('z'), &cle, &demarrer(origin.clone()));
    assert!(matches!(c.publier(&inst), Err(ErreurOrigin::Refusee { statut: 403, .. })));
}

// ---------------------------------------------------------------------------
// Dépôt MiyuCloud
// ---------------------------------------------------------------------------

/// Copie conforme de `lire_borne` (MiyuCloud).
fn lire_borne(chemin: &Path, max: u64, sortie: &mut impl std::io::Write) -> Result<u64, String> {
    let fichier = std::fs::File::open(chemin).map_err(|e| e.to_string())?;
    let copies = std::io::copy(&mut fichier.take(max.saturating_add(1)), sortie).map_err(|e| e.to_string())?;
    if copies > max {
        return Err("taille".into());
    }
    Ok(copies)
}

/// Copie conforme de `lire_depot` (MiyuCloud) : tête puis entrées.
fn lire_depot(depot: &Path, depuis: u64) -> Result<(Vec<u8>, Vec<Vec<u8>>), String> {
    let mut tete = Vec::new();
    if lire_borne(&depot.join(FICHIER_TETE), 1 << 20, &mut tete)? == 0 {
        return Err("tête vide".into());
    }
    let mut copie = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    lire_borne(&depot.join(FICHIER_ENTREES), 256 << 20, copie.as_file_mut())?;
    copie.as_file_mut().sync_all().map_err(|e| e.to_string())?;
    let stockage = StockageFichier::ouvrir(copie.path()).map_err(|e| e.to_string())?;
    let entrees = (depuis..stockage.taille()).map(|k| stockage.lire(k)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok((tete, entrees))
}

/// Relevé MiyuCloud : lecture du dépôt puis application au suivi.
fn relever(suivi: &SuiviRevocationsRegistre, depot: &Path) -> Result<usize, String> {
    let depuis = suivi.taille().unwrap();
    let (tete, entrees) = lire_depot(depot, depuis)?;
    suivi.appliquer(&tete, depuis, &entrees, maintenant_unix()).map(|a| a.ajoutees).map_err(|e| e.to_string())
}

fn mode(p: &Path) -> u32 {
    std::fs::symlink_metadata(p).unwrap().mode() & 0o7777
}

#[test]
fn depot_relu_par_le_protocole_de_miyucloud() {
    let cle = CleSignature::generer();
    let mut journal = journal_memoire(&cle, 50);
    let racine = tempfile::tempdir().unwrap();
    let chemin = racine.path().join("depot");
    let mut depot = DepotRevocations::ouvrir(&chemin).unwrap();
    assert_eq!(mode(&chemin), 0o750);
    // Un second éditeur sur le même dépôt : refusé (verrou).
    assert!(matches!(DepotRevocations::ouvrir(&chemin), Err(ErreurDepot::Refuse(_))));

    // Lien symbolique posé à la place de la tête : remplacé, jamais suivi.
    let cible = racine.path().join("cible");
    std::fs::write(&cible, b"intact").unwrap();
    std::os::unix::fs::symlink(&cible, chemin.join(FICHIER_TETE)).unwrap();

    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();
    depot.publier(&inst).unwrap();
    assert_eq!(std::fs::read(&cible).unwrap(), b"intact");
    assert_eq!((mode(&chemin.join(FICHIER_TETE)), mode(&chemin.join(FICHIER_ENTREES))), (0o640, 0o640));
    let noms: Vec<String> = std::fs::read_dir(&chemin).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
    assert_eq!(noms.len(), 2, "aucun temporaire laissé : {noms:?}");

    let suivi = SuiviRevocationsRegistre::new(cog('h'), REGISTRE, &cle.cle_publique()).unwrap();
    assert_eq!(relever(&suivi, &chemin).unwrap(), 50);
    assert_eq!(suivi.est_revoque(&cog('h'), &CibleRevocation::Cle(kid(49)), maintenant_unix()), Ok(true));

    // Sans entrée nouvelle : seules la tête change, le fichier d'entrées n'est pas réécrit.
    let ino = std::fs::metadata(chemin.join(FICHIER_ENTREES)).unwrap().ino();
    let inst = Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap();
    depot.publier(&inst).unwrap();
    assert_eq!(std::fs::metadata(chemin.join(FICHIER_ENTREES)).unwrap().ino(), ino);
    assert_eq!(relever(&suivi, &chemin).unwrap(), 0);
    // Entrées nouvelles : relevé incrémental (depuis la taille du miroir).
    journal.ajouter(&revocation(&cle, 50)).unwrap();
    depot.publier(&Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap()).unwrap();
    assert_eq!(relever(&suivi, &chemin).unwrap(), 1);
    assert_eq!(suivi.taille(), Some(51));
    drop(depot);

    // Dossier trop ouvert, ou lien symbolique : refusés.
    let large = racine.path().join("large");
    std::fs::create_dir(&large).unwrap();
    std::fs::set_permissions(&large, std::fs::Permissions::from_mode(0o770)).unwrap();
    assert!(matches!(DepotRevocations::ouvrir(&large), Err(ErreurDepot::Refuse(_))));
    let lien = racine.path().join("lien");
    std::os::unix::fs::symlink(&chemin, &lien).unwrap();
    assert!(matches!(DepotRevocations::ouvrir(&lien), Err(ErreurDepot::Refuse(_))));
}

#[test]
fn lecteur_concurrent_ne_voit_jamais_un_couple_incoherent() {
    let cle = CleSignature::generer();
    let mut journal = journal_memoire(&cle, 10);
    let racine = tempfile::tempdir().unwrap();
    let chemin = racine.path().join("depot");
    let mut depot = DepotRevocations::ouvrir(&chemin).unwrap();
    depot.publier(&Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap()).unwrap();

    let fini = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let lecteur = {
        let (chemin, fini, publique) = (chemin.clone(), fini.clone(), cle.cle_publique());
        std::thread::spawn(move || {
            let mut releves = 0u32;
            while !fini.load(Ordering::SeqCst) || releves == 0 {
                // Lecteur neuf à chaque fois : il exige que les entrées couvrent toute la tête.
                let suivi = SuiviRevocationsRegistre::new(cog('h'), REGISTRE, &publique).unwrap();
                relever(&suivi, &chemin).unwrap();
                releves += 1;
            }
            releves
        })
    };
    for i in 10..110 {
        journal.ajouter(&revocation(&cle, i)).unwrap();
        depot.publier(&Instantane::emettre(&mut journal, &cle, maintenant_unix()).unwrap()).unwrap();
    }
    fini.store(true, Ordering::SeqCst);
    assert!(lecteur.join().unwrap() > 0);
}

// ---------------------------------------------------------------------------
// Binaire
// ---------------------------------------------------------------------------

fn ecrire_cle(dossier: &Path, cle: &CleSignature, mode: u32) -> PathBuf {
    let p = dossier.join(format!("cle-{mode:o}"));
    let hex: String = cle.graine().iter().fold(String::new(), |mut s, b| {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
        s
    });
    std::fs::write(&p, hex).unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(mode)).unwrap();
    p
}

fn publier(args: &[&str]) -> std::process::Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_miyuregistre-publier")).args(args).output().unwrap()
}

#[test]
fn binaire_publie_vers_le_depot_et_origin() {
    let cle = CleSignature::generer();
    let racine = tempfile::tempdir().unwrap();
    let dossier_journal = racine.path().join("journal");
    std::fs::create_dir(&dossier_journal).unwrap();
    std::fs::set_permissions(&dossier_journal, std::fs::Permissions::from_mode(0o700)).unwrap();
    {
        let mut entrees = StockageFichier::ouvrir(dossier_journal.join("entrees.mrgj")).unwrap();
        for i in 0..300 {
            entrees.ajouter(&revocation(&cle, i)).unwrap();
        }
    }
    let fichier_cle = ecrire_cle(racine.path(), &cle, 0o600);
    let depot = racine.path().join("depot");
    let origin = OriginTest::new(cog('h'), cle.cle_publique());
    let url = demarrer(origin.clone());
    let registre: String = "52".repeat(32);
    let base = |cle: &Path| {
        vec![
            "--journal".to_string(),
            dossier_journal.display().to_string(),
            "--registre".into(),
            registre.clone(),
            "--cle-journal".into(),
            cle.display().to_string(),
            "--une-fois".into(),
        ]
    };
    let mut args = base(&fichier_cle);
    args.extend(["--depot".into(), depot.display().to_string(), "--origin".into(), url.clone(), "--cog".into(), cog('h').to_string()]);
    let sortie = publier(&args.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(sortie.status.success(), "{}", String::from_utf8_lossy(&sortie.stderr));
    assert_eq!(origin.suivi.taille(), Some(300));
    let suivi = SuiviRevocationsRegistre::new(cog('h'), REGISTRE, &cle.cle_publique()).unwrap();
    assert_eq!(relever(&suivi, &depot).unwrap(), 300);
    // Second cycle : la séquence des têtes reprend depuis le journal (têtes persistées).
    let sortie = publier(&args.iter().map(String::as_str).collect::<Vec<_>>());
    assert!(sortie.status.success(), "{}", String::from_utf8_lossy(&sortie.stderr));
    assert_eq!(relever(&suivi, &depot).unwrap(), 0);

    // Clé lisible par le groupe : refus de démarrer (code 2).
    let large = ecrire_cle(racine.path(), &cle, 0o640);
    let mut a = base(&large);
    a.extend(["--depot".into(), depot.display().to_string()]);
    assert_eq!(publier(&a.iter().map(String::as_str).collect::<Vec<_>>()).status.code(), Some(2));
    // Origin seul, clé non épinglée : 401 ⇒ arrêt (code 3).
    let autre = OriginTest::new(cog('h'), CleSignature::generer().cle_publique());
    let mut a = base(&fichier_cle);
    a.extend(["--origin".into(), demarrer(autre), "--cog".into(), cog('h').to_string()]);
    assert_eq!(publier(&a.iter().map(String::as_str).collect::<Vec<_>>()).status.code(), Some(3));
    // Intervalle au-delà de 5 min : refusé.
    let mut a = base(&fichier_cle);
    a.extend(["--depot".into(), depot.display().to_string(), "--intervalle".into(), "600".into()]);
    assert_eq!(publier(&a.iter().map(String::as_str).collect::<Vec<_>>()).status.code(), Some(2));
}
