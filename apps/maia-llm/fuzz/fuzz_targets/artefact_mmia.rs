//! @id     maia_llm_fuzz_artefact_mmia
//! @do     fuzzer_artefact_modele_canal_officiel
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — artefact « modèle » `MMIA` du canal officiel : décodage, manifeste signé, installation fail-closed (EX-26, A-007, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : artefact brut → `InstallateurModeles::installer_artefact` (dossier
//!   neuf, ancre fixe) ;
//! - autre : `long. nom (1) | nom | licence (1) | altération (3) | modèle` :
//!   manifeste signé par l'ancre pour ce nom et ce modèle, artefact encodé
//!   par `encoder_artefact`, puis éventuellement altéré (position, masque,
//!   troncature).
//!
//! Propriétés : aucune panique ; un artefact installé est exactement
//! `encoder_artefact(manifeste, modèle)` et le fichier écrit contient ces
//! octets du modèle, au nom signé, dans le dossier des modèles ; une même
//! version n'est jamais réinstallée ; un artefact intact au nom et à la
//! licence admissibles est installé, tout autre refusé ; une altération
//! n'installe jamais d'octets ni de nom autres que ceux signés.

#![no_main]

#[path = "../../src/provenance.rs"]
#[allow(dead_code)]
mod provenance;

#[path = "../../src/canal.rs"]
#[allow(dead_code)]
mod canal;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use canal::{encoder_artefact, InstallateurModeles};
use libfuzzer_sys::fuzz_target;
use miyusceau::{sceller, AncresDeConfiance, CleSignature, NiveauConfiance};
use provenance::ManifesteModele;
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
fn installer(artefact: &[u8]) -> Option<(String, Vec<u8>)> {
    let dossier = dossier_neuf();
    let inst = InstallateurModeles::new(&dossier, ancres());
    let resultat = inst.installer_artefact("modele-fuzz", 1, artefact);
    let issue = resultat.ok().map(|chemin| controler_installe(&inst, &dossier, &chemin, artefact));
    let _ = std::fs::remove_dir_all(&dossier);
    issue
}

fn controler_installe(inst: &InstallateurModeles, dossier: &Path, chemin: &Path, artefact: &[u8]) -> (String, Vec<u8>) {
    assert_eq!(chemin.parent(), Some(dossier), "modèle écrit hors du dossier des modèles");
    let nom = chemin.file_name().expect("nom").to_string_lossy().into_owned();
    let modele = std::fs::read(chemin).expect("modèle écrit lisible");
    let manifeste = std::fs::read(provenance::chemin_manifeste(chemin)).expect("manifeste écrit lisible");
    assert_eq!(encoder_artefact(&manifeste, &modele), artefact, "artefact installé ≠ manifeste | modèle écrits");
    assert!(inst.installer_artefact("modele-fuzz", 1, artefact).is_err(), "même version réinstallée");
    (nom, modele)
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        let _ = installer(reste);
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

    let (pos, masque, coupe) = (parametres[1], parametres[2], parametres[3]);
    if masque == 0 && coupe & 0x80 == 0 {
        let admissible = !nom.starts_with('.')
            && !nom.to_ascii_lowercase().ends_with(".sceau")
            && licence == "MIT"
            && !modele.is_empty();
        let installe = installer(&artefact);
        if let Some((n, o)) = &installe {
            assert!(admissible, "artefact inadmissible installé : « {nom} »");
            assert_eq!((n.as_str(), o.as_slice()), (nom.as_str(), modele));
        } else {
            // Un nom trop long pour le système de fichiers (fichier temporaire
            // compris) est un refus légitime.
            assert!(!admissible || nom.len() > 200, "artefact intact admissible refusé : « {nom} »");
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
    if let Some((n, o)) = installer(&artefact) {
        assert_eq!((n.as_str(), o.as_slice()), (nom.as_str(), modele), "artefact altéré : contenu non signé installé");
    }
});
