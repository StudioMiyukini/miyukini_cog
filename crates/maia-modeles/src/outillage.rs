//! @id     maia_modeles_outillage
//! @do     sceller_manifeste_modele
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — outillage de signature : scellement du manifeste d'un modèle, graine de développement (sous-commandes cle-dev et sceller-modele de maia-llm).

//! Outillage de signature (développement / canal officiel).
//!
//! - `maia-llm cle-dev --sortie cle.hex` : génère une graine de signature de
//!   développement (fichier `0600`) et affiche la clé publique à épingler ;
//! - `maia-llm sceller-modele --graine cle.hex --modele X.gguf --licence MIT
//!   --coupure 2025-06` : écrit `X.gguf.sceau`.

use std::path::{Path, PathBuf};

use miyusceau::{sceller, CleSignature};

use crate::fichier::{chemin_manifeste, sha256_fichier};
use crate::manifeste::{ManifesteModele, LICENCES_AUTORISEES};

/// Scelle le manifeste d'un modèle et l'écrit dans `X.gguf.sceau`.
pub fn sceller_modele(
    modele: &Path,
    cle: &CleSignature,
    licence: &str,
    date_coupure: &str,
) -> Result<PathBuf, String> {
    let nom = modele
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .ok_or("chemin sans nom de fichier")?;
    if !LICENCES_AUTORISEES.contains(&licence) {
        return Err(format!(
            "licence « {licence} » non autorisée (Apache-2.0 ou MIT)"
        ));
    }
    let taille = std::fs::metadata(modele).map_err(|e| e.to_string())?.len();
    let sha256 = sha256_fichier(modele).map_err(|e| e.to_string())?;
    let m = ManifesteModele {
        nom_fichier: nom,
        sha256,
        taille,
        licence: licence.to_string(),
        date_coupure: date_coupure.to_string(),
        emis_le: crate::maintenant_secs(),
    };
    let octets = sceller(&m, cle).map_err(|e| e.to_string())?;
    let sortie = chemin_manifeste(modele);
    std::fs::write(&sortie, octets).map_err(|e| e.to_string())?;
    Ok(sortie)
}

/// Lit une graine de signature (64 caractères hexadécimaux) depuis un fichier.
pub fn lire_graine(chemin: &Path) -> Result<CleSignature, String> {
    let texte = std::fs::read_to_string(chemin).map_err(|e| e.to_string())?;
    let octets = hex::decode(texte.trim()).map_err(|e| e.to_string())?;
    let graine: [u8; 32] = octets
        .try_into()
        .map_err(|_| "graine : 32 octets attendus".to_string())?;
    Ok(CleSignature::depuis_graine(&graine))
}

/// Génère une graine de développement dans `sortie` (créée, jamais écrasée,
/// droits `0600` sous Unix). Renvoie la clé publique hexadécimale à épingler.
pub fn generer_graine_dev(sortie: &Path) -> Result<String, String> {
    use std::io::Write;
    let cle = CleSignature::generer();
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut f = opts.open(sortie).map_err(|e| e.to_string())?;
    f.write_all(hex::encode(cle.graine()).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(hex::encode(cle.cle_publique()))
}

/// Sous-commandes d'outillage (`cle-dev`, `sceller-modele`). Renvoie `None`
/// si `args` ne désigne pas une sous-commande d'outillage.
pub fn executer_outil(args: &[String]) -> Option<Result<String, String>> {
    let texte = |nom: &str| {
        args.iter()
            .position(|a| a == nom)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let valeur = |nom: &str| texte(nom).map(PathBuf::from);
    match args.get(1).map(String::as_str) {
        Some("cle-dev") => Some((|| -> Result<String, String> {
            let sortie = valeur("--sortie").ok_or("usage : maia-llm cle-dev --sortie <fichier>")?;
            let publique = generer_graine_dev(&sortie)?;
            Ok(format!(
                "Graine écrite dans {} (secrète, 0600).\nClé publique à épingler dans \
                 model_trust_anchors : {publique}",
                sortie.display()
            ))
        })()),
        Some("sceller-modele") => Some((|| -> Result<String, String> {
            const USAGE: &str =
                "usage : maia-llm sceller-modele --graine <fichier> --modele <X.gguf> \
                                 --licence <Apache-2.0|MIT> --coupure <AAAA-MM>";
            let graine = valeur("--graine").ok_or(USAGE)?;
            let modele = valeur("--modele").ok_or(USAGE)?;
            let licence = texte("--licence").ok_or(USAGE)?;
            let coupure = texte("--coupure").ok_or(USAGE)?;
            let cle = lire_graine(&graine)?;
            let sortie = sceller_modele(&modele, &cle, &licence, &coupure)?;
            Ok(format!(
                "Manifeste écrit : {} (signataire {})",
                sortie.display(),
                hex::encode(cle.cle_publique())
            ))
        })()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ancres_depuis_hex;
    use crate::essai::{cle, dossier, modele};
    use crate::fichier::verifier_modele;

    #[test]
    fn outil_cle_dev_puis_scellement() {
        let d = dossier("outil");
        let p = modele(&d);
        let graine = d.join("cle.hex");
        let publique = generer_graine_dev(&graine).unwrap();
        assert!(generer_graine_dev(&graine).is_err(), "jamais écrasée");
        let args: Vec<String> = [
            "maia-llm",
            "sceller-modele",
            "--graine",
            graine.to_str().unwrap(),
            "--modele",
            p.to_str().unwrap(),
            "--licence",
            "MIT",
            "--coupure",
            "2025-01",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert!(executer_outil(&args).unwrap().is_ok());
        let a = ancres_depuis_hex(&[publique]).unwrap();
        assert!(verifier_modele(&p, &a).is_ok());
        assert!(executer_outil(&["maia-llm".into(), "serve".into()]).is_none());
        assert!(executer_outil(&["maia-llm".into(), "cle-dev".into()])
            .unwrap()
            .is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn licence_refusee_avant_toute_ecriture() {
        let d = dossier("outil");
        let p = modele(&d);
        assert!(sceller_modele(&p, &cle(), "GPL-3.0", "2025-06").is_err());
        assert!(!crate::chemin_manifeste(&p).exists());
        let _ = std::fs::remove_dir_all(&d);
    }
}
