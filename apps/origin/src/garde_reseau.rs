//! @id     origin_garde_reseau
//! @do     filtrer_cibles_sortantes
//! @role   securite
//! @layer  app
//! @human  origin — garde anti-SSRF : analyse des adresses annoncées et cibles publiques seules.

//! Garde réseau anti-SSRF (constat O-03).
//!
//! - L'adresse annoncée par un COG est analysée (URL ou `hôte:port`, IPv4, IPv6 entre
//!   crochets, nom DNS) ; une adresse locale est remplacée par l'IP du pair vue par le Tracker.
//! - Toute cible du proxy direct est résolue puis **chaque** adresse obtenue doit être
//!   publique : boucle locale (`127/8`, `::1`), privée (RFC 1918), lien local (`169.254/16`,
//!   `fe80::/10`, métadonnées cloud), ULA (`fc00::/7`), multicast, non spécifiée, CGNAT
//!   (100.64/10), plages de documentation/test, IPv4 mappées/compatibles/NAT64/6to4/Teredo.
//! - La connexion se fait vers l'IP vérifiée (jamais une seconde résolution du nom :
//!   pas de « DNS rebinding »), et le contrôle est refait à chaque connexion.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

/// Longueur maximale d'une adresse annoncée (C-13).
pub const LONGUEUR_MAX_ADRESSE: usize = 300;

/// Refus d'une adresse (C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusAdresse {
    /// Adresse vide, trop longue ou syntaxe invalide.
    #[error("adresse mal formée")]
    MalFormee,
    /// Port absent, nul ou invalide.
    #[error("port invalide")]
    PortInvalide,
    /// Schéma autre que http/https.
    #[error("schéma non autorisé")]
    SchemaInterdit,
    /// IP non publique (boucle locale, privée, lien local, etc.).
    #[error("adresse non publique : {0}")]
    NonPublique(IpAddr),
    /// Résolution DNS impossible ou vide.
    #[error("résolution impossible")]
    Resolution,
}

/// Hôte d'une adresse annoncée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Hote {
    /// Adresse IP littérale.
    Ip(IpAddr),
    /// Nom DNS (minuscules, validé).
    Nom(String),
}

/// Adresse analysée : hôte + port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdresseAnalysee {
    /// Hôte.
    pub hote: Hote,
    /// Port (non nul).
    pub port: u16,
}

impl AdresseAnalysee {
    /// Forme canonique `hôte:port` (IPv6 entre crochets).
    #[must_use]
    pub fn canonique(&self) -> String {
        match &self.hote {
            Hote::Ip(IpAddr::V6(v6)) => format!("[{v6}]:{}", self.port),
            Hote::Ip(ip) => format!("{ip}:{}", self.port),
            Hote::Nom(nom) => format!("{nom}:{}", self.port),
        }
    }

    /// Valeur d'en-tête `Host`.
    #[must_use]
    pub fn entete_host(&self) -> String {
        self.canonique()
    }
}

/// Vrai si l'IP est publique (routable sur Internet, hors plages spéciales).
#[must_use]
pub fn ip_publique(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => ipv4_publique(v4),
        IpAddr::V6(v6) => ipv6_publique(v6),
    }
}

fn ipv4_publique(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    !(ip.is_unspecified()             // 0.0.0.0
        || o[0] == 0                  // 0.0.0.0/8
        || ip.is_loopback()           // 127.0.0.0/8
        || ip.is_private()            // 10/8, 172.16/12, 192.168/16
        || ip.is_link_local()         // 169.254/16 (dont métadonnées 169.254.169.254)
        || (o[0] == 100 && (o[1] & 0xc0) == 64) // 100.64/10 CGNAT (dont 100.100.100.200)
        || (o[0] == 192 && o[1] == 0 && o[2] == 0) // 192.0.0.0/24
        || (o[0] == 192 && o[1] == 0 && o[2] == 2) // 192.0.2.0/24 TEST-NET-1
        || (o[0] == 198 && (o[1] & 0xfe) == 18)    // 198.18/15
        || (o[0] == 198 && o[1] == 51 && o[2] == 100) // TEST-NET-2
        || (o[0] == 203 && o[1] == 0 && o[2] == 113)  // TEST-NET-3
        || ip.is_multicast()          // 224/4
        || o[0] >= 240                // 240/4 réservé + diffusion
        || ip.is_broadcast())
}

fn ipv6_publique(ip: Ipv6Addr) -> bool {
    // IPv4 mappée (::ffff:a.b.c.d) : on juge l'IPv4 incluse (refus de ::ffff:127.0.0.1).
    if let Some(v4) = ip.to_ipv4_mapped() {
        return ipv4_publique(v4);
    }
    let s = ip.segments();
    !(ip.is_unspecified()                       // ::
        || ip.is_loopback()                     // ::1
        || (s[0..6] == [0, 0, 0, 0, 0, 0])      // ::/96 compatible IPv4 (déprécié)
        || (s[0] & 0xfe00) == 0xfc00            // fc00::/7 ULA (dont fd00:ec2::254)
        || (s[0] & 0xffc0) == 0xfe80            // fe80::/10 lien local
        || (s[0] & 0xffc0) == 0xfec0            // fec0::/10 site local (déprécié)
        || (s[0] & 0xff00) == 0xff00            // ff00::/8 multicast
        || (s[0] == 0x64 && s[1] == 0xff9b)     // 64:ff9b::/96 NAT64
        || (s[0] == 0x0100 && s[1..4] == [0, 0, 0]) // 100::/64 discard
        || (s[0] == 0x2001 && s[1] == 0x0db8)   // 2001:db8::/32 documentation
        || (s[0] == 0x2001 && s[1] == 0)        // 2001::/32 Teredo
        || s[0] == 0x2002) // 2002::/16 6to4
}

/// Analyse une adresse annoncée : `hôte:port`, `[v6]:port`, `http(s)://hôte:port[/]`.
pub fn analyser_adresse(brute: &str) -> Result<AdresseAnalysee, RefusAdresse> {
    let brute = brute.trim();
    if brute.is_empty() || brute.len() > LONGUEUR_MAX_ADRESSE {
        return Err(RefusAdresse::MalFormee);
    }
    if brute.chars().any(|c| c.is_control() || c.is_whitespace()) {
        return Err(RefusAdresse::MalFormee);
    }
    let minuscule = brute.to_ascii_lowercase();
    let (sans_schema, schema_explicite) = if let Some(r) = minuscule.strip_prefix("http://") {
        (r, true)
    } else if let Some(r) = minuscule.strip_prefix("https://") {
        (r, true)
    } else if minuscule.contains("://") {
        return Err(RefusAdresse::SchemaInterdit);
    } else {
        (minuscule.as_str(), false)
    };
    // Autorité seule : chemin final « / » toléré, rien d'autre (pas d'identifiants, requête…).
    let autorite = sans_schema.strip_suffix('/').unwrap_or(sans_schema);
    if autorite.is_empty() || autorite.contains(['/', '?', '#', '@', '\\', '%']) {
        return Err(RefusAdresse::MalFormee);
    }

    let (hote, port) = if let Some(reste) = autorite.strip_prefix('[') {
        let (v6, apres) = reste.split_once(']').ok_or(RefusAdresse::MalFormee)?;
        let ip: Ipv6Addr = v6.parse().map_err(|_| RefusAdresse::MalFormee)?;
        let port = match apres.strip_prefix(':') {
            Some(p) => p,
            None if apres.is_empty() && schema_explicite => "",
            None => return Err(RefusAdresse::PortInvalide),
        };
        (Hote::Ip(IpAddr::V6(ip)), port)
    } else {
        let (h, p) = match autorite.rsplit_once(':') {
            Some((h, p)) => (h, p),
            None if schema_explicite => (autorite, ""),
            None => return Err(RefusAdresse::PortInvalide),
        };
        if h.contains(':') {
            // IPv6 sans crochets : ambigu, refusé.
            return Err(RefusAdresse::MalFormee);
        }
        let hote = match h.parse::<Ipv4Addr>() {
            Ok(v4) => Hote::Ip(IpAddr::V4(v4)),
            Err(_) => Hote::Nom(valider_nom(h)?),
        };
        (hote, p)
    };

    let port = if port.is_empty() {
        if minuscule.starts_with("https://") {
            443
        } else {
            80
        }
    } else {
        if !port.bytes().all(|b| b.is_ascii_digit()) || port.len() > 5 {
            return Err(RefusAdresse::PortInvalide);
        }
        port.parse::<u16>().map_err(|_| RefusAdresse::PortInvalide)?
    };
    if port == 0 {
        return Err(RefusAdresse::PortInvalide);
    }
    Ok(AdresseAnalysee { hote, port })
}

fn valider_nom(nom: &str) -> Result<String, RefusAdresse> {
    let nom = nom.strip_suffix('.').unwrap_or(nom);
    if nom.is_empty() || nom.len() > 253 {
        return Err(RefusAdresse::MalFormee);
    }
    for etiquette in nom.split('.') {
        if etiquette.is_empty()
            || etiquette.len() > 63
            || etiquette.starts_with('-')
            || etiquette.ends_with('-')
            || !etiquette
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err(RefusAdresse::MalFormee);
        }
    }
    if nom == "localhost" {
        // Remplacé par l'IP du pair (voir `hote_local`).
        return Ok(nom.to_string());
    }
    // Nom pleinement qualifié exigé (un nom court passerait par les domaines de recherche
    // internes). Le dernier label doit commencer par une lettre : refuse les formes
    // numériques ambiguës interprétées comme IP par certains résolveurs
    // (« 2130706433 », « 0x7f.1 », « 127.1 »).
    let derniere = nom.rsplit('.').next().unwrap_or(nom);
    if !nom.contains('.') || !derniere.starts_with(|c: char| c.is_ascii_lowercase()) {
        return Err(RefusAdresse::MalFormee);
    }
    Ok(nom.to_string())
}

/// Vrai si l'hôte annoncé désigne la machine du COG elle-même (à remplacer par l'IP du pair).
fn hote_local(hote: &Hote) -> bool {
    match hote {
        Hote::Ip(ip) => ip.is_loopback() || ip.is_unspecified(),
        Hote::Nom(n) => n == "localhost" || n.ends_with(".localhost"),
    }
}

/// Adresse effective à enregistrer pour un ANNOUNCE.
///
/// - Hôte local (`127.x`, `::1`, `0.0.0.0`, `localhost`) → IP du pair + port annoncé.
/// - IP littérale → doit être publique.
/// - Nom DNS → conservé (vérifié à chaque connexion après résolution).
pub fn adresse_annoncee_effective(
    brute: &str,
    ip_pair: IpAddr,
) -> Result<AdresseAnalysee, RefusAdresse> {
    let mut analysee = analyser_adresse(brute)?;
    if hote_local(&analysee.hote) {
        analysee.hote = Hote::Ip(ip_pair);
    }
    if let Hote::Ip(ip) = analysee.hote {
        if !ip_publique(ip) {
            return Err(RefusAdresse::NonPublique(ip));
        }
    }
    Ok(analysee)
}

/// Résout la cible et retourne une adresse **publique vérifiée** à laquelle se connecter.
/// Refus si une seule des adresses résolues n'est pas publique.
pub async fn resoudre_cible_publique(
    adresse: &AdresseAnalysee,
) -> Result<SocketAddr, RefusAdresse> {
    let candidats: Vec<SocketAddr> = match &adresse.hote {
        Hote::Ip(ip) => vec![SocketAddr::new(*ip, adresse.port)],
        Hote::Nom(nom) => tokio::net::lookup_host((nom.as_str(), adresse.port))
            .await
            .map_err(|_| RefusAdresse::Resolution)?
            .take(16)
            .collect(),
    };
    verifier_candidats(&candidats)
}

/// Contrôle une liste de candidats : tous publics, sinon refus ; retourne le premier.
pub fn verifier_candidats(candidats: &[SocketAddr]) -> Result<SocketAddr, RefusAdresse> {
    let premier = *candidats.first().ok_or(RefusAdresse::Resolution)?;
    for c in candidats {
        if !ip_publique(c.ip()) {
            return Err(RefusAdresse::NonPublique(c.ip()));
        }
    }
    Ok(premier)
}

/// Ouvre une connexion TCP vers la cible après vérification (à chaque connexion).
pub async fn connecter_cible_publique(
    adresse: &AdresseAnalysee,
    delai: std::time::Duration,
) -> Result<tokio::net::TcpStream, Box<dyn std::error::Error + Send + Sync>> {
    let cible = resoudre_cible_publique(adresse).await?;
    let flux = tokio::time::timeout(delai, tokio::net::TcpStream::connect(cible))
        .await
        .map_err(|_| "connection timeout")??;
    // Contrôle final sur le pair effectivement connecté.
    let pair = flux.peer_addr()?;
    if !ip_publique(pair.ip()) {
        return Err(Box::new(RefusAdresse::NonPublique(pair.ip())));
    }
    Ok(flux)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair_public() -> IpAddr {
        "8.8.8.8".parse().expect("ip")
    }

    #[tokio::test]
    async fn table_ssrf_refusee() {
        let refusees = [
            "127.0.0.1:8080",
            "127.0.0.2:8080",
            "127.255.255.254:80",
            "[::1]:8080",
            "[::ffff:127.0.0.1]:8080",
            "[::ffff:7f00:1]:8080",
            "[::ffff:10.0.0.1]:80",
            "[::127.0.0.1]:80",
            "[64:ff9b::7f00:1]:80",
            "[2002:7f00:1::]:80",
            "169.254.169.254:80",
            "http://169.254.169.254/",
            "10.0.0.5:80",
            "10.255.255.255:443",
            "172.16.0.1:80",
            "172.31.255.255:80",
            "192.168.1.10:8080",
            "100.64.0.1:80",
            "100.100.100.200:80",
            "0.0.0.0:80",
            "0.1.2.3:80",
            "224.0.0.1:80",
            "255.255.255.255:80",
            "240.0.0.1:80",
            "192.0.2.1:80",
            "198.18.0.1:80",
            "[fe80::1]:80",
            "[fc00::1]:80",
            "[fd00:ec2::254]:80",
            "[ff02::1]:80",
            "[::]:80",
            "[2001:db8::1]:80",
        ];
        for a in refusees {
            // 1. Contrôle à la connexion : l'IP analysée n'est jamais une cible acceptée.
            let analysee = analyser_adresse(a).expect(a);
            assert!(
                matches!(
                    resoudre_cible_publique(&analysee).await,
                    Err(RefusAdresse::NonPublique(_))
                ),
                "{a} aurait dû être refusée à la connexion"
            );
            // 2. À l'ANNOUNCE : refusée, ou (hôte local) remplacée par l'IP du pair ;
            //    l'IP non publique n'est jamais enregistrée.
            match adresse_annoncee_effective(a, pair_public()) {
                Err(RefusAdresse::NonPublique(_)) => {}
                Ok(eff) => assert_eq!(eff.hote, Hote::Ip(pair_public()), "{a}"),
                autre => panic!("{a} : résultat inattendu {autre:?}"),
            }
            // 3. Pair non public : toujours refusée.
            assert!(
                adresse_annoncee_effective(a, "10.1.2.3".parse().expect("ip")).is_err(),
                "{a} avec pair privé"
            );
        }
    }

    #[test]
    fn adresses_locales_remplacees_par_ip_du_pair() {
        for a in ["127.0.0.1:8090", "0.0.0.0:8090", "localhost:8090", "[::1]:8090", "http://127.0.0.1:8090/"] {
            let r = adresse_annoncee_effective(a, pair_public()).expect(a);
            assert_eq!(r.canonique(), "8.8.8.8:8090", "{a}");
        }
        // Pair privé : l'adresse locale devient privée, refusée.
        let r = adresse_annoncee_effective("127.0.0.1:8090", "10.0.0.2".parse().expect("ip"));
        assert!(matches!(r, Err(RefusAdresse::NonPublique(_))));
    }

    #[test]
    fn syntaxes_invalides_refusees() {
        for a in [
            "",
            "8.8.8.8",
            "8.8.8.8:0",
            "8.8.8.8:99999",
            "8.8.8.8:80/admin",
            "http://user@8.8.8.8:80",
            "ftp://8.8.8.8:21",
            "gopher://x:70",
            "8.8.8.8:80\r\nHost: x",
            "::1:80",
            "2130706433:80",
            "0x7f.1:80",
            "exa mple.com:80",
            "-bad.example:80",
            "intranet:80",
            "127.1:80",
        ] {
            assert!(analyser_adresse(a).is_err() || adresse_annoncee_effective(a, pair_public()).is_err(), "{a}");
        }
    }

    #[test]
    fn adresses_publiques_acceptees() {
        assert_eq!(
            adresse_annoncee_effective("8.8.4.4:8443", pair_public()).expect("v4").canonique(),
            "8.8.4.4:8443"
        );
        assert_eq!(
            adresse_annoncee_effective("[2606:4700::1111]:80", pair_public()).expect("v6").canonique(),
            "[2606:4700::1111]:80"
        );
        assert_eq!(
            adresse_annoncee_effective("https://Cog.Example.org/", pair_public()).expect("nom").canonique(),
            "cog.example.org:443"
        );
    }

    #[tokio::test]
    async fn nom_resolvant_vers_privee_refuse() {
        // « localhost » est connu sans réseau (/etc/hosts) et résout vers la boucle locale.
        let a = AdresseAnalysee {
            hote: Hote::Nom("localhost".to_string()),
            port: 80,
        };
        assert!(matches!(
            resoudre_cible_publique(&a).await,
            Err(RefusAdresse::NonPublique(_) | RefusAdresse::Resolution)
        ));
        // Réponse DNS mixte (une publique, une privée) : refus.
        let mixte: Vec<SocketAddr> = vec![
            "8.8.8.8:80".parse().expect("a"),
            "10.0.0.1:80".parse().expect("b"),
        ];
        assert!(verifier_candidats(&mixte).is_err());
        let rebinding: Vec<SocketAddr> = vec!["169.254.169.254:80".parse().expect("c")];
        assert!(verifier_candidats(&rebinding).is_err());
        assert!(verifier_candidats(&[]).is_err());
    }

    #[tokio::test]
    async fn connexion_refusee_vers_boucle_locale() {
        // Même si une entrée de pool contenait une IP locale, la connexion est refusée.
        let a = AdresseAnalysee {
            hote: Hote::Ip("127.0.0.2".parse().expect("ip")),
            port: 9,
        };
        assert!(connecter_cible_publique(&a, std::time::Duration::from_millis(200))
            .await
            .is_err());
    }
}
