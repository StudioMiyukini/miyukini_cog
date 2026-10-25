//! @id     maia_memoire_tests_kindmother_reprise
//! @do     verifier_reprise_apres_coupure_kindmother
//! @role   test
//! @layer  8
//! @human  maia-memoire — reprise après coupure de KindMother : reconnexion authentifiée, lecture rejouée, écriture interrompue jamais dupliquée, authentification refusée jamais réessayée.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use maia_memoire::{BorneOctets, Branche, Ecrit, EntrepotKindMother, Issue, MemoireMaia, TypeUnite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

fn repertoire(utilisateurs: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("maia-memoire-reprise-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).expect("dir");
    std::fs::write(dir.join("maia-utilisateurs"), utilisateurs).expect("utilisateurs");
    dir
}

/// KindMother réel dans son propre runtime : l'arrêter coupe toutes ses connexions,
/// comme un redémarrage du service.
struct Serveur {
    rt: Option<tokio::runtime::Runtime>,
}

impl Serveur {
    /// Démarre sur `addr` (port libre si `None`) avec les données de `dir`.
    async fn demarrer(dir: &Path, addr: Option<SocketAddr>) -> (Self, SocketAddr) {
        let cible = addr.unwrap_or_else(|| "127.0.0.1:0".parse().expect("adresse"));
        let ecoute = std::net::TcpListener::bind(cible).expect("bind");
        ecoute.set_nonblocking(true).expect("non bloquant");
        let addr = ecoute.local_addr().expect("adresse locale");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("runtime");
        let (pret, attente) = tokio::sync::oneshot::channel();
        let dir = dir.to_path_buf();
        rt.spawn(async move {
            let serveur = kindmother_service::KindMotherServer::new(&dir).await.expect("serveur");
            let ecoute = TcpListener::from_std(ecoute).expect("écouteur");
            let _ = pret.send(());
            let _ = serveur.serve_listener(ecoute).await;
        });
        attente.await.expect("serveur prêt");
        (Self { rt: Some(rt) }, addr)
    }

    /// Arrêt brutal : toutes les connexions sont fermées.
    async fn arreter(mut self) {
        if let Some(rt) = self.rt.take() {
            tokio::task::spawn_blocking(move || rt.shutdown_timeout(Duration::from_secs(5)))
                .await
                .expect("arrêt");
        }
    }
}

const NORMAL: u8 = 0;
/// Coupe la connexion à réception d'une écriture, sans la transmettre.
const PERDRE_REQUETE: u8 = 1;
/// Transmet l'écriture au serveur, puis coupe la connexion avant de relayer la réponse.
const PERDRE_REPONSE: u8 = 2;

async fn lire_trame(s: &mut TcpStream) -> Option<Vec<u8>> {
    let mut l = [0u8; 4];
    s.read_exact(&mut l).await.ok()?;
    let mut buf = vec![0u8; usize::try_from(u32::from_be_bytes(l)).ok()?];
    s.read_exact(&mut buf).await.ok()?;
    Some(buf)
}

async fn ecrire_trame(s: &mut TcpStream, t: &[u8]) -> Option<()> {
    let l = u32::try_from(t.len()).ok()?;
    s.write_all(&l.to_be_bytes()).await.ok()?;
    s.write_all(t).await.ok()
}

/// Relais trame à trame (le protocole est requête/réponse) entre MAIA et KindMother,
/// capable de couper une écriture avant ou après son application.
async fn relais(cible: SocketAddr, mode: Arc<AtomicU8>) -> SocketAddr {
    let ecoute = TcpListener::bind("127.0.0.1:0").await.expect("bind relais");
    let addr = ecoute.local_addr().expect("adresse relais");
    tokio::spawn(async move {
        while let Ok((mut client, _)) = ecoute.accept().await {
            let mode = Arc::clone(&mode);
            tokio::spawn(async move {
                let Ok(mut serveur) = TcpStream::connect(cible).await else { return };
                while let Some(req) = lire_trame(&mut client).await {
                    let ecriture = req.windows(13).any(|w| w == b"SubmitIntents");
                    if ecriture
                        && mode.compare_exchange(PERDRE_REQUETE, NORMAL, Ordering::SeqCst, Ordering::SeqCst).is_ok()
                    {
                        return;
                    }
                    if ecrire_trame(&mut serveur, &req).await.is_none() {
                        return;
                    }
                    let Some(rep) = lire_trame(&mut serveur).await else { return };
                    if ecriture
                        && mode.compare_exchange(PERDRE_REPONSE, NORMAL, Ordering::SeqCst, Ordering::SeqCst).is_ok()
                    {
                        return;
                    }
                    if ecrire_trame(&mut client, &rep).await.is_none() {
                        return;
                    }
                }
            });
        }
    });
    addr
}

async fn ouvrir(addr: &str, creds: &Path, utilisateur: &str) -> MemoireMaia<EntrepotKindMother> {
    let e = EntrepotKindMother::connecter(addr, utilisateur, Some(creds)).await.expect("connexion");
    MemoireMaia::ouvrir(e, Arc::new(BorneOctets)).await.expect("ouvrir")
}

fn ecrit(texte: &str) -> Ecrit {
    Ecrit::new(Branche::parse("travail.cog").expect("branche"), TypeUnite::Fait, texte, 7)
}

/// Lignes de `versions` lues directement dans la base (hors entrepôt testé).
async fn versions_en_base(addr: &str, creds: &Path, base: &str) -> usize {
    let cred = kindmother_client::OperatorCredential::load_from_dir(base, creds).expect("cred");
    let c = kindmother_client::KindMotherClient::connect(addr, &cred, base).await.expect("connexion");
    c.select(kindmother_client::TypedSelect::from("versions")).await.expect("versions").len()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn lecture_reussit_apres_redemarrage_de_kindmother() {
    let dir = repertoire("alice\n");
    let (serveur, addr) = Serveur::demarrer(&dir, None).await;
    let creds = dir.join("credentials");
    let alice = ouvrir(&addr.to_string(), &creds, "alice").await;
    let Issue::Ecrite(f) = alice.utilisateur().ecrire(ecrit("Avant la coupure")).await.expect("écrire") else {
        panic!("écrite")
    };

    // Redémarrage de KindMother : la connexion de MAIA est morte (« broken pipe »).
    serveur.arreter().await;
    let (serveur, _) = Serveur::demarrer(&dir, Some(addr)).await;
    let notes = alice.utilisateur().consulter().await.expect("lecture après redémarrage");
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].id, f);

    // KindMother arrêté : erreur explicite, en temps borné (aucune boucle infinie).
    serveur.arreter().await;
    let debut = Instant::now();
    assert!(alice.utilisateur().consulter().await.is_err());
    assert!(debut.elapsed() < Duration::from_secs(10), "backoff borné : {:?}", debut.elapsed());

    // Relancé : l'appel suivant se reconnecte, lectures et écritures reprennent.
    let (serveur, _) = Serveur::demarrer(&dir, Some(addr)).await;
    alice.utilisateur().ecrire(ecrit("Après la coupure")).await.expect("écrire après reprise");
    assert_eq!(alice.utilisateur().consulter().await.expect("lecture").len(), 2);
    serveur.arreter().await;
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ecriture_interrompue_jamais_dupliquee() {
    let dir = repertoire("alice\n");
    let (serveur, cible) = Serveur::demarrer(&dir, None).await;
    let creds = dir.join("credentials");
    let mode = Arc::new(AtomicU8::new(NORMAL));
    let addr = relais(cible, Arc::clone(&mode)).await.to_string();
    let alice = ouvrir(&addr, &creds, "alice").await;

    // Écriture appliquée, réponse perdue : relue présente, non rejouée.
    mode.store(PERDRE_REPONSE, Ordering::SeqCst);
    let Issue::Ecrite(f) = alice.utilisateur().ecrire(ecrit("Réponse perdue")).await.expect("écrire") else {
        panic!("écrite")
    };
    assert_eq!(mode.load(Ordering::SeqCst), NORMAL, "la coupure a bien eu lieu");
    assert_eq!(alice.utilisateur().historique(&f).await.expect("historique").len(), 1);
    assert_eq!(versions_en_base(&addr, &creds, "maia-alice").await, 1, "aucun doublon");

    // Correction (version 2) interrompue après application : une seule version 2.
    mode.store(PERDRE_REPONSE, Ordering::SeqCst);
    alice.utilisateur().corriger(&f, "Réponse perdue, corrigée", None).await.expect("corriger");
    assert_eq!(mode.load(Ordering::SeqCst), NORMAL);
    assert_eq!(alice.utilisateur().historique(&f).await.expect("historique").len(), 2);
    assert_eq!(versions_en_base(&addr, &creds, "maia-alice").await, 2);

    // Écriture jamais parvenue : relue absente, rejouée une seule fois.
    mode.store(PERDRE_REQUETE, Ordering::SeqCst);
    alice.utilisateur().ecrire(ecrit("Requête perdue")).await.expect("écrire");
    assert_eq!(mode.load(Ordering::SeqCst), NORMAL);
    let notes = alice.utilisateur().consulter().await.expect("consulter");
    assert_eq!(notes.len(), 2);
    assert_eq!(notes.iter().filter(|n| n.texte == "Requête perdue").count(), 1);
    assert_eq!(versions_en_base(&addr, &creds, "maia-alice").await, 3);

    serveur.arreter().await;
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn authentification_refusee_jamais_reessayee() {
    let dir = repertoire("alice\n");
    let (serveur, addr) = Serveur::demarrer(&dir, None).await;
    let alice = ouvrir(&addr.to_string(), &dir.join("credentials"), "alice").await;
    serveur.arreter().await;

    // Un autre KindMother (autre clé de serveur) répond à la même adresse : pin refusé.
    let autre = repertoire("alice\n");
    let (imposteur, _) = Serveur::demarrer(&autre, Some(addr)).await;
    let e = alice.utilisateur().consulter().await.expect_err("pin refusé");
    assert!(e.to_string().contains("authentification"), "{e}");
    imposteur.arreter().await;

    // Le vrai serveur revient : l'entrepôt reste verrouillé (aucune reconnexion silencieuse).
    let (serveur, _) = Serveur::demarrer(&dir, Some(addr)).await;
    let e = alice.utilisateur().consulter().await.expect_err("verrouillé");
    assert!(e.to_string().contains("reconnexion suspendue"), "{e}");
    // Une nouvelle connexion explicite fonctionne.
    let alice = ouvrir(&addr.to_string(), &dir.join("credentials"), "alice").await;
    assert_eq!(alice.utilisateur().consulter().await.expect("consulter").len(), 0);
    serveur.arreter().await;
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&autre);
}
