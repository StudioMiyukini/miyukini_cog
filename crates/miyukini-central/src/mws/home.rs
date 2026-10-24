//! @id     miyukini_central_mws_home
//! @do     servir_page_home_bornee
//! @role   securite
//! @layer  core
//! @human  miyukini-central — serveur Home MWS durci (taille, délais, connexions bornés).

//! Mini serveur HTTP de la page Home du COG (CA-02, C-13, C-21, EX-48).
//!
//! - Désactivé par défaut (`home_http_bind = None`) ; lorsqu'il est activé
//!   depuis les réglages, il écoute sur `127.0.0.1` (le tunnel MWS suffit).
//! - Entête de requête borné à [`TAILLE_MAX_REQUETE`] octets (sinon 431).
//! - Délai total de lecture de l'entête [`DELAI_LECTURE`] (anti slow-loris),
//!   délai d'écriture [`DELAI_ECRITURE`].
//! - Au plus [`MAX_CONNEXIONS`] connexions simultanées ; au-delà, la
//!   connexion est fermée immédiatement.
//! - Seules `GET` et `HEAD` sont servies (sinon 405).

use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{oneshot, Semaphore};
use tokio::time::{timeout, Duration};
use tracing::{debug, info, warn};

/// Taille maximale de l'entête de requête (8 Kio).
pub const TAILLE_MAX_REQUETE: usize = 8 * 1024;
/// Délai maximal pour recevoir l'entête complet.
pub const DELAI_LECTURE: Duration = Duration::from_secs(5);
/// Délai maximal pour écrire la réponse.
pub const DELAI_ECRITURE: Duration = Duration::from_secs(5);
/// Connexions simultanées maximales.
pub const MAX_CONNEXIONS: usize = 32;

/// Bornes appliquées par le serveur (paramétrables pour les tests).
#[derive(Debug, Clone, Copy)]
pub(crate) struct LimitesHome {
    pub taille_max: usize,
    pub delai_lecture: Duration,
    pub delai_ecriture: Duration,
    pub max_connexions: usize,
}

impl Default for LimitesHome {
    fn default() -> Self {
        Self {
            taille_max: TAILLE_MAX_REQUETE,
            delai_lecture: DELAI_LECTURE,
            delai_ecriture: DELAI_ECRITURE,
            max_connexions: MAX_CONNEXIONS,
        }
    }
}

/// Contenu servi (figé au démarrage du serveur).
#[derive(Debug, Clone)]
pub(crate) struct ContenuHome {
    pub cog_id: String,
    pub core_version: String,
    pub services: Vec<String>,
}

/// Lie l'adresse et sert jusqu'au signal d'arrêt.
pub(crate) async fn run_home_server(
    bind_addr: String,
    contenu: ContenuHome,
    shutdown: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(&bind_addr).await?;
    info!("Serveur Home COG en écoute sur http://{}", bind_addr);
    servir(listener, contenu, LimitesHome::default(), shutdown).await;
    Ok(())
}

/// Boucle d'acceptation bornée.
pub(crate) async fn servir(
    listener: TcpListener,
    contenu: ContenuHome,
    limites: LimitesHome,
    mut shutdown: oneshot::Receiver<()>,
) {
    let contenu = Arc::new(contenu);
    let permis = Arc::new(Semaphore::new(limites.max_connexions));
    loop {
        tokio::select! {
            _ = &mut shutdown => {
                info!("Serveur Home arrêté (déconnexion MWS)");
                break;
            }
            accepte = listener.accept() => {
                let (stream, addr) = match accepte {
                    Ok(x) => x,
                    Err(e) => {
                        warn!("Accept Home: {}", e);
                        continue;
                    }
                };
                let Ok(jeton) = Arc::clone(&permis).try_acquire_owned() else {
                    warn!("[Home Server] Plafond de connexions atteint : {} refusé", addr);
                    drop(stream);
                    continue;
                };
                debug!("[Home Server] Connexion entrante depuis {}", addr);
                let contenu = Arc::clone(&contenu);
                tokio::spawn(async move {
                    if let Err(e) = traiter_connexion(stream, &contenu, limites).await {
                        debug!("Home connection: {}", e);
                    }
                    drop(jeton);
                });
            }
        }
    }
}

/// Résultat de la lecture bornée de l'entête.
enum Lecture {
    Entete(Vec<u8>),
    TropGrand,
    Ferme,
}

/// Lit jusqu'à `\r\n\r\n` sans dépasser `taille_max`.
async fn lire_entete(stream: &mut TcpStream, taille_max: usize) -> std::io::Result<Lecture> {
    let mut tampon = Vec::with_capacity(1024);
    let mut morceau = [0u8; 1024];
    loop {
        let n = stream.read(&mut morceau).await?;
        if n == 0 {
            return Ok(Lecture::Ferme);
        }
        tampon.extend_from_slice(&morceau[..n]);
        if tampon.windows(4).any(|w| w == b"\r\n\r\n") {
            return Ok(Lecture::Entete(tampon));
        }
        if tampon.len() >= taille_max {
            return Ok(Lecture::TropGrand);
        }
    }
}

async fn ecrire(
    stream: &mut TcpStream,
    statut: &str,
    corps: &str,
    avec_corps: bool,
    delai: Duration,
) -> std::io::Result<()> {
    let entete = format!(
        "HTTP/1.1 {statut}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\
         X-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        corps.len()
    );
    let ecriture = async {
        stream.write_all(entete.as_bytes()).await?;
        if avec_corps {
            stream.write_all(corps.as_bytes()).await?;
        }
        stream.flush().await
    };
    timeout(delai, ecriture)
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "écriture trop lente"))?
}

async fn traiter_connexion(
    mut stream: TcpStream,
    contenu: &ContenuHome,
    limites: LimitesHome,
) -> std::io::Result<()> {
    let Ok(lecture) = timeout(limites.delai_lecture, lire_entete(&mut stream, limites.taille_max)).await
    else {
        // Slow-loris : on ferme sans attendre davantage.
        let _ = ecrire(&mut stream, "408 Request Timeout", "", false, limites.delai_ecriture).await;
        return Ok(());
    };
    let lecture = lecture?;
    let entete = match lecture {
        Lecture::Entete(e) => e,
        Lecture::Ferme => return Ok(()),
        Lecture::TropGrand => {
            return ecrire(
                &mut stream,
                "431 Request Header Fields Too Large",
                "",
                false,
                limites.delai_ecriture,
            )
            .await;
        }
    };

    // Ligne de requête : "METHODE /chemin HTTP/1.x"
    let premiere = entete.split(|&b| b == b'\n').next().unwrap_or_default();
    let premiere = String::from_utf8_lossy(premiere);
    let mut parties = premiere.split_whitespace();
    let methode = parties.next().unwrap_or_default();
    let chemin = parties.next().unwrap_or("/");
    let avec_corps = match methode {
        "GET" => true,
        "HEAD" => false,
        _ => {
            return ecrire(&mut stream, "405 Method Not Allowed", "", false, limites.delai_ecriture).await;
        }
    };

    let (statut, corps) =
        super::route_request(chemin, &contenu.cog_id, &contenu.core_version, &contenu.services);
    ecrire(&mut stream, statut, &corps, avec_corps, limites.delai_ecriture).await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn demarrer(limites: LimitesHome) -> (std::net::SocketAddr, oneshot::Sender<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let contenu = ContenuHome {
            cog_id: "cog-test".into(),
            core_version: "1.0.0".into(),
            services: vec![],
        };
        tokio::spawn(servir(listener, contenu, limites, rx));
        (addr, tx)
    }

    async fn lire_tout(stream: &mut TcpStream) -> String {
        let mut sortie = Vec::new();
        let _ = timeout(Duration::from_secs(5), stream.read_to_end(&mut sortie)).await;
        String::from_utf8_lossy(&sortie).into_owned()
    }

    #[tokio::test]
    async fn requete_valide_servie() {
        let (addr, _arret) = demarrer(LimitesHome::default()).await;
        let mut s = TcpStream::connect(addr).await.unwrap();
        s.write_all(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
        let r = lire_tout(&mut s).await;
        assert!(r.starts_with("HTTP/1.1 200 OK"), "{r}");
    }

    #[tokio::test]
    async fn requete_trop_grande_refusee() {
        let (addr, _arret) = demarrer(LimitesHome::default()).await;
        let mut s = TcpStream::connect(addr).await.unwrap();
        // Flux sans fin de ligne au-delà de la borne : refus 431, pas de croissance infinie.
        let gros = vec![b'A'; TAILLE_MAX_REQUETE + 1024];
        let _ = s.write_all(&gros).await;
        let r = lire_tout(&mut s).await;
        assert!(r.starts_with("HTTP/1.1 431"), "{r}");
    }

    #[tokio::test]
    async fn slow_loris_coupe_par_le_delai() {
        let limites = LimitesHome { delai_lecture: Duration::from_millis(200), ..Default::default() };
        let (addr, _arret) = demarrer(limites).await;
        let mut s = TcpStream::connect(addr).await.unwrap();
        s.write_all(b"GET / HTTP/1.1\r\nX-Lent: ").await.unwrap();
        let debut = std::time::Instant::now();
        let r = lire_tout(&mut s).await;
        assert!(r.starts_with("HTTP/1.1 408"), "{r}");
        assert!(debut.elapsed() < Duration::from_secs(3));
    }

    #[tokio::test]
    async fn plafond_de_connexions() {
        let limites = LimitesHome { max_connexions: 1, delai_lecture: Duration::from_secs(2), ..Default::default() };
        let (addr, _arret) = demarrer(limites).await;
        // La 1re connexion muette occupe l'unique place.
        let _occupe = TcpStream::connect(addr).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        let mut s = TcpStream::connect(addr).await.unwrap();
        let _ = s.write_all(b"GET / HTTP/1.1\r\n\r\n").await;
        let r = lire_tout(&mut s).await;
        assert!(r.is_empty(), "connexion au-delà du plafond fermée sans réponse : {r}");
    }

    #[tokio::test]
    async fn methode_non_get_refusee() {
        let (addr, _arret) = demarrer(LimitesHome::default()).await;
        let mut s = TcpStream::connect(addr).await.unwrap();
        s.write_all(b"POST / HTTP/1.1\r\n\r\n").await.unwrap();
        let r = lire_tout(&mut s).await;
        assert!(r.starts_with("HTTP/1.1 405"), "{r}");
    }
}
