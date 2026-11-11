//! @id     origin_web_media
//! @do     servir_media_embarque
//! @role   adapter
//! @layer  app
//! @human  origin — médias du site (vidéo de présentation et son affiche), embarqués dans le binaire et servis avec prise en charge de Range.

//! Médias statiques du site public.
//!
//! Les fichiers sont embarqués à la compilation (`include_bytes!`) : aucun accès disque,
//! aucun chemin fourni par le client n'atteint le système de fichiers. Les requêtes `Range`
//! (une seule plage) reçoivent une réponse `206`, nécessaire à la lecture vidéo sur Safari.

/// Préfixe des routes média.
pub const PREFIXE: &str = "/media/";

/// Vidéo de présentation (accueil).
const BRAG_MP4: &[u8] = include_bytes!("../../assets/brag.mp4");
/// Même vidéo en VP9/Opus (navigateurs sans décodeur H.264).
const BRAG_WEBM: &[u8] = include_bytes!("../../assets/brag.webm");
/// Affiche de la vidéo (première image).
const BRAG_POSTER: &[u8] = include_bytes!("../../assets/brag-poster.jpg");

/// Associe un nom de média à son contenu et à son type.
fn trouver(nom: &str) -> Option<(&'static [u8], &'static str)> {
    match nom {
        "brag.mp4" => Some((BRAG_MP4, "video/mp4")),
        "brag.webm" => Some((BRAG_WEBM, "video/webm")),
        "brag-poster.jpg" => Some((BRAG_POSTER, "image/jpeg")),
        _ => None,
    }
}

/// En-têtes communs à toutes les réponses média.
const COMMUNS: &str = "Accept-Ranges: bytes\r\n\
    Cache-Control: public, max-age=86400\r\n\
    X-Content-Type-Options: nosniff\r\n\
    Cross-Origin-Resource-Policy: same-origin\r\n\
    Connection: close\r\n";

/// Plage demandée, déjà bornée à la taille du fichier (`debut..=fin`).
#[derive(Debug, PartialEq, Eq)]
enum Plage {
    Entiere,
    Partielle { debut: usize, fin: usize },
    Invalide,
}

/// Lit l'en-tête `Range` (format `bytes=a-b`, `bytes=a-`, `bytes=-n`). Plusieurs plages : on sert tout.
fn lire_plage(entetes: &[String], taille: usize) -> Plage {
    let valeur = entetes.iter().find_map(|l| {
        let (nom, v) = l.split_once(':')?;
        nom.trim().eq_ignore_ascii_case("range").then(|| v.trim().to_string())
    });
    let Some(valeur) = valeur else { return Plage::Entiere };
    let Some(spec) = valeur.strip_prefix("bytes=") else { return Plage::Entiere };
    if spec.contains(',') || taille == 0 {
        return Plage::Entiere;
    }
    let Some((a, b)) = spec.split_once('-') else { return Plage::Invalide };
    let (a, b) = (a.trim(), b.trim());
    let dernier = taille - 1;
    let (debut, fin) = if a.is_empty() {
        // Suffixe : les n derniers octets.
        match b.parse::<usize>() {
            Ok(0) | Err(_) => return Plage::Invalide,
            Ok(n) => (taille.saturating_sub(n), dernier),
        }
    } else {
        let Ok(debut) = a.parse::<usize>() else { return Plage::Invalide };
        let fin = if b.is_empty() {
            dernier
        } else {
            match b.parse::<usize>() {
                Ok(f) => f.min(dernier),
                Err(_) => return Plage::Invalide,
            }
        };
        (debut, fin)
    };
    if debut > fin || debut > dernier {
        return Plage::Invalide;
    }
    Plage::Partielle { debut, fin }
}

/// Construit la réponse HTTP brute pour un média, ou `None` si le chemin n'est pas un média connu.
///
/// `tete` : requête `HEAD` (en-têtes seuls).
pub fn reponse(chemin: &str, entetes: &[String], tete: bool) -> Option<Vec<u8>> {
    let nom = chemin.split_once('?').map_or(chemin, |(p, _)| p).strip_prefix(PREFIXE)?;
    let (donnees, type_contenu) = trouver(nom)?;
    let taille = donnees.len();
    let (statut, corps, plage) = match lire_plage(entetes, taille) {
        Plage::Entiere => ("200 OK", donnees, String::new()),
        Plage::Partielle { debut, fin } => (
            "206 Partial Content",
            &donnees[debut..=fin],
            format!("Content-Range: bytes {debut}-{fin}/{taille}\r\n"),
        ),
        Plage::Invalide => {
            let entete = format!(
                "HTTP/1.1 416 Range Not Satisfiable\r\nContent-Range: bytes */{taille}\r\nContent-Length: 0\r\n{COMMUNS}\r\n"
            );
            return Some(entete.into_bytes());
        }
    };
    let mut sortie = format!(
        "HTTP/1.1 {statut}\r\nContent-Type: {type_contenu}\r\nContent-Length: {}\r\n{plage}{COMMUNS}\r\n",
        corps.len()
    )
    .into_bytes();
    if !tete {
        sortie.extend_from_slice(corps);
    }
    Some(sortie)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(v: &str) -> Vec<String> {
        vec![format!("Range: {v}\r\n")]
    }

    #[test]
    fn plages() {
        assert_eq!(lire_plage(&[], 100), Plage::Entiere);
        assert_eq!(lire_plage(&r("bytes=0-"), 100), Plage::Partielle { debut: 0, fin: 99 });
        assert_eq!(lire_plage(&r("bytes=10-19"), 100), Plage::Partielle { debut: 10, fin: 19 });
        assert_eq!(lire_plage(&r("bytes=90-500"), 100), Plage::Partielle { debut: 90, fin: 99 });
        assert_eq!(lire_plage(&r("bytes=-10"), 100), Plage::Partielle { debut: 90, fin: 99 });
        assert_eq!(lire_plage(&r("bytes=100-"), 100), Plage::Invalide);
        assert_eq!(lire_plage(&r("bytes=20-10"), 100), Plage::Invalide);
        assert_eq!(lire_plage(&r("bytes=-0"), 100), Plage::Invalide);
        assert_eq!(lire_plage(&r("bytes=x-1"), 100), Plage::Invalide);
        assert_eq!(lire_plage(&r("bytes=0-1,5-6"), 100), Plage::Entiere);
        assert_eq!(lire_plage(&["range: bytes=1-2\r\n".into()], 100), Plage::Partielle { debut: 1, fin: 2 });
    }

    #[test]
    fn reponses() {
        assert!(reponse("/media/inconnu.mp4", &[], false).is_none());
        assert!(reponse("/media/brag.webm", &[], true).is_some());
        assert!(reponse("/media/../Cargo.toml", &[], false).is_none());
        assert!(reponse("/autre", &[], false).is_none());

        let complete = reponse("/media/brag.mp4", &[], false).expect("média connu");
        let texte = String::from_utf8_lossy(&complete[..300]);
        assert!(texte.starts_with("HTTP/1.1 200 OK"));
        assert!(texte.contains("Content-Type: video/mp4"));
        assert!(texte.contains("Accept-Ranges: bytes"));

        let partielle = reponse("/media/brag.mp4", &r("bytes=0-1"), false).expect("média connu");
        let texte = String::from_utf8_lossy(&partielle);
        assert!(texte.starts_with("HTTP/1.1 206 Partial Content"));
        assert!(texte.contains(&format!("Content-Range: bytes 0-1/{}", BRAG_MP4.len())));
        assert!(texte.contains("Content-Length: 2\r\n"));

        let sans_corps = reponse("/media/brag-poster.jpg?v=1", &[], true).expect("média connu");
        assert!(sans_corps.ends_with(b"\r\n\r\n"));

        let refus = reponse("/media/brag.mp4", &r("bytes=999999999-"), false).expect("média connu");
        assert!(String::from_utf8_lossy(&refus).starts_with("HTTP/1.1 416"));
    }
}
