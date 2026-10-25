//! @id     maia_llm_fuzz_artefact_mmia
//! @do     fuzzer_artefact_modele_canal_officiel
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — artefact « modèle » `MMIA` du canal officiel : décodage, manifeste signé, installation fail-closed (EX-26, A-007, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : artefact brut → `InstallateurModeles::installer_artefact` (dossier
//!   neuf, ancre fixe, composant `modele-fuzz`) ;
//! - autre : `long. nom (1) | nom | licence (1) | altération (3) | modèle` :
//!   manifeste signé par l'ancre pour ce nom et ce modèle, artefact encodé
//!   par `encoder_artefact`, puis éventuellement altéré (position, masque,
//!   troncature). Le composant annoncé est le nom privé de `.gguf` s'il le
//!   porte, `modele-fuzz` sinon.
//!
//! Propriétés : aucune panique ; un artefact installé est exactement
//! `encoder_artefact(manifeste, modèle)` et le fichier écrit contient ces
//! octets du modèle, au nom signé, dans le dossier des modèles ; une même
//! version n'est jamais réinstallée ; un artefact intact dont le nom est le
//! fichier imposé du composant (`<composant>.gguf`, composant `[a-z0-9]`
//! puis `[a-z0-9._-]`, ≤ 64, sans `..` — oracle indépendant) et la licence
//! admissible est installé, tout autre refusé ; une altération n'installe
//! jamais d'octets ni de nom autres que ceux signés.

#![no_main]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use libfuzzer_sys::fuzz_target;
use maia_modeles::canal::{encoder_artefact, InstallateurModeles};
use maia_modeles::{chemin_manifeste, ManifesteModele};
use miyusceau::{sceller, AncresDeConfiance, CleSignature, NiveauConfiance};
use sha2::{Digest, Sha256};

const GRAINE: [u8; 32] = [0x2a; 32];

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

fn ancres() -> AncresDeConfiance {
    let mut a = AncresDeConfiance::new();
    a.ajouter(&CleSignature::depuis_graine(&GRAINE).cle_publique(), NiveauConfiance::Souverain).expect("ancre");
    a
}

fn dossier_neuf() -> PathBuf {
    let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("maia-llm-fuzz-mmia-{}-{n}", std::process::id()))
}

fn maintenant() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// Installe dans un dossier neuf ; en cas de succès, contrôle le résultat
/// puis le refus d'une réinstallation de la même version. Rend
/// `(nom installé, octets installés)`.
fn installer(composant: &str, artefact: &[u8]) -> Option<(String, Vec<u8>)> {
    let dossier = dossier_neuf();
    let inst = InstallateurModeles::new(&dossier, ancres());
    let resultat = inst.installer_artefact(composant, 1, artefact);
    let issue = resultat.ok().map(|chemin| controler_installe(&inst, composant, &dossier, &chemin, artefact));
    let _ = std::fs::remove_dir_all(&dossier);
    issue
}

fn controler_installe(
    inst: &InstallateurModeles,
    composant: &str,
    dossier: &Path,
    chemin: &Path,
    artefact: &[u8],
) -> (String, Vec<u8>) {
    assert_eq!(chemin.parent(), Some(dossier), "modèle écrit hors du dossier des modèles");
    let nom = chemin.file_name().expect("nom").to_string_lossy().into_owned();
    assert_eq!(nom, format!("{composant}.gguf"), "fichier non dérivé du composant");
    let modele = std::fs::read(chemin).expect("modèle écrit lisible");
    let manifeste = std::fs::read(chemin_manifeste(chemin)).expect("manifeste écrit lisible");
    assert_eq!(encoder_artefact(&manifeste, &modele), artefact, "artefact installé ≠ manifeste | modèle écrits");
    assert!(inst.installer_artefact(composant, 1, artefact).is_err(), "même version réinstallée");
    // Aucun temporaire ni sauvegarde laissé.
    let restes = std::fs::read_dir(dossier)
        .expect("dossier lisible")
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().contains(".canal-") && e.file_name() != ".canal-modeles.json")
        .count();
    assert_eq!(restes, 0, "temporaire ou sauvegarde laissé");
    (nom, modele)
}

/// Composant annoncé pour un nom signé : le nom privé de `.gguf`, sinon un composant fixe.
fn composant_pour(nom: &str) -> String {
    nom.strip_suffix(".gguf").filter(|c| !c.is_empty()).unwrap_or("modele-fuzz").to_string()
}

/// Oracle indépendant de la règle de nommage (composant → `<composant>.gguf`).
fn composant_admissible(c: &str) -> bool {
    let o = c.as_bytes();
    !o.is_empty()
        && o.len() <= 64
        && (o[0].is_ascii_lowercase() || o[0].is_ascii_digit())
        && o.iter().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'.' | b'_' | b'-'))
        && !c.contains("..")
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        let _ = installer("modele-fuzz", reste);
        return;
    }
    let Some((&long_nom, reste)) = reste.split_first() else { return };
    let long_nom = usize::from(long_nom);
    if reste.len() < long_nom + 4 {
        return;
    }
    let (nom, reste) = reste.split_at(long_nom);
    let (parametres, modele) = reste.split_at(4);
    let nom = String::from_utf8_lossy(nom).into_owned();
    let licence = if parametres[0] & 1 == 0 { "MIT" } else { "GPL-3.0" };
    let m = ManifesteModele {
        nom_fichier: nom.clone(),
        sha256: Sha256::digest(modele).into(),
        taille: modele.len() as u64,
        licence: licence.into(),
        date_coupure: "2025-06".into(),
        emis_le: maintenant().saturating_sub(60),
    };
    let Ok(manifeste) = sceller(&m, &CleSignature::depuis_graine(&GRAINE)) else { return };
    let mut artefact = encoder_artefact(&manifeste, modele);

    let composant = composant_pour(&nom);
    let (pos, masque, coupe) = (parametres[1], parametres[2], parametres[3]);
    if masque == 0 && coupe & 0x80 == 0 {
        let admissible = composant_admissible(&composant)
            && nom == format!("{composant}.gguf")
            && licence == "MIT"
            && !modele.is_empty();
        let installe = installer(&composant, &artefact);
        if let Some((n, o)) = &installe {
            assert!(admissible, "artefact inadmissible installé : « {nom} »");
            assert_eq!((n.as_str(), o.as_slice()), (nom.as_str(), modele));
        } else {
            assert!(!admissible, "artefact intact admissible refusé : « {nom} »");
        }
        return;
    }
    if masque != 0 {
        let p = usize::from(pos) * 31 % artefact.len();
        artefact[p] ^= masque;
    }
    if coupe & 0x80 != 0 {
        let garde = artefact.len().saturating_sub(usize::from(coupe & 0x7F));
        artefact.truncate(garde);
    }
    if let Some((n, o)) = installer(&composant, &artefact) {
        assert_eq!((n.as_str(), o.as_slice()), (nom.as_str(), modele), "artefact altéré : contenu non signé installé");
    }
});
