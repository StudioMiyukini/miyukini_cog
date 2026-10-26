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
//!   --coupure 2025-06` : écrit `X.gguf.sceau` ;
//! - `maia-llm emballer-modele --modele X.gguf --ancre <hex> --sortie X.mmia` :
//!   vérifie le couple `X.gguf` + `X.gguf.sceau` puis écrit l'artefact `MMIA`
//!   du canal officiel (écriture en flux, jamais d'écrasement) ;
//! - `maia-llm installer-modele --dossier D --ancre <hex> --composant C
//!   --version N --artefact X.mmia` (fonctionnalité `canal`) : installe
//!   l'artefact comme le canal officiel (signature, nom imposé, taille,
//!   licence, **empreinte vérifiée avant toute écriture**, refus sinon).

use std::path::{Path, PathBuf};

use miyusceau::{sceller, CleSignature};

use crate::artefact::{MAGIC_ARTEFACT, TAILLE_MAX_MANIFESTE, VERSION_ARTEFACT};
use crate::fichier::{chemin_manifeste, lire_manifeste, sha256_fichier, verifier_modele};
use crate::manifeste::{ancres_depuis_hex, ManifesteModele, LICENCES_AUTORISEES};

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

/// Écrit l'artefact `MMIA` de `modele` (manifeste voisin `X.gguf.sceau`)
/// dans `sortie` (créée, jamais écrasée), **après** avoir vérifié le couple
/// contre `ancres`. Les octets du modèle sont copiés en flux.
pub fn emballer_modele(
    modele: &Path,
    ancres: &miyusceau::AncresDeConfiance,
    sortie: &Path,
) -> Result<ManifesteModele, String> {
    use std::io::Write;
    let manifeste_verifie = verifier_modele(modele, ancres).map_err(|e| e.to_string())?;
    let manifeste = lire_manifeste(&chemin_manifeste(modele)).map_err(|e| e.to_string())?;
    if manifeste.is_empty() || manifeste.len() > TAILLE_MAX_MANIFESTE {
        return Err("manifeste vide ou trop gros".into());
    }
    let longueur = u32::try_from(manifeste.len()).map_err(|e| e.to_string())?;
    let mut source = std::fs::File::open(modele).map_err(|e| e.to_string())?;
    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create_new(true);
    let f = opts.open(sortie).map_err(|e| format!("{} : {e}", sortie.display()))?;
    let mut w = std::io::BufWriter::new(f);
    let resultat = (|| -> std::io::Result<u64> {
        w.write_all(MAGIC_ARTEFACT)?;
        w.write_all(&[VERSION_ARTEFACT])?;
        w.write_all(&longueur.to_be_bytes())?;
        w.write_all(&manifeste)?;
        let n = std::io::copy(&mut source, &mut w)?;
        w.flush()?;
        w.get_ref().sync_all()?;
        Ok(n)
    })();
    match resultat {
        Ok(n) if n == manifeste_verifie.taille => Ok(manifeste_verifie),
        Ok(n) => {
            let _ = std::fs::remove_file(sortie);
            Err(format!(
                "modèle modifié pendant l'emballage ({n} o copiés, {} o signés)",
                manifeste_verifie.taille
            ))
        }
        Err(e) => {
            let _ = std::fs::remove_file(sortie);
            Err(format!("écriture de l'artefact : {e}"))
        }
    }
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
        Some("emballer-modele") => Some((|| -> Result<String, String> {
            const USAGE: &str = "usage : maia-llm emballer-modele --modele <X.gguf> \
                                 --ancre <64 hex> --sortie <X.mmia>";
            let modele = valeur("--modele").ok_or(USAGE)?;
            let ancre = texte("--ancre").ok_or(USAGE)?;
            let sortie = valeur("--sortie").ok_or(USAGE)?;
            let ancres = ancres_depuis_hex(&[ancre]).map_err(|e| e.to_string())?;
            let m = emballer_modele(&modele, &ancres, &sortie)?;
            Ok(format!(
                "Artefact MMIA écrit : {} (modèle {}, {} o, SHA-256 {}, licence {}, coupure {})",
                sortie.display(),
                m.nom_fichier,
                m.taille,
                hex::encode(m.sha256),
                m.licence,
                m.date_coupure
            ))
        })()),
        #[cfg(feature = "canal")]
        Some("installer-modele") => Some((|| -> Result<String, String> {
            const USAGE: &str = "usage : maia-llm installer-modele --dossier <D> --ancre <64 hex> \
                                 --composant <c> --version <N> --artefact <X.mmia>";
            let dossier = valeur("--dossier").ok_or(USAGE)?;
            let ancre = texte("--ancre").ok_or(USAGE)?;
            let composant = texte("--composant").ok_or(USAGE)?;
            let version: u64 = texte("--version")
                .ok_or(USAGE)?
                .parse()
                .map_err(|_| "--version : entier attendu".to_string())?;
            let artefact = valeur("--artefact").ok_or(USAGE)?;
            let ancres = ancres_depuis_hex(&[ancre]).map_err(|e| e.to_string())?;
            let octets = std::fs::read(&artefact).map_err(|e| e.to_string())?;
            let installe = crate::canal::InstallateurModeles::new(dossier, ancres)
                .installer_artefact(&composant, version, &octets)?;
            Ok(format!("Modèle installé et vérifié : {}", installe.display()))
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
    use miyusceau::CleSignature;

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
    fn emballage_mmia_verifie_puis_decodable() {
        let d = dossier("emballer");
        let p = modele(&d);
        let cle_essai = cle();
        sceller_modele(&p, &cle_essai, "Apache-2.0", "2024-06").unwrap();
        let ancres_essai = crate::essai::ancres(&cle_essai);
        let sortie = d.join("m.mmia");
        let manifeste_lu = emballer_modele(&p, &ancres_essai, &sortie).unwrap();
        assert_eq!(manifeste_lu.licence, "Apache-2.0");
        let octets = std::fs::read(&sortie).unwrap();
        let (manifeste, corps) = crate::decoder_artefact(&octets).unwrap();
        assert_eq!(corps, std::fs::read(&p).unwrap().as_slice());
        assert_eq!(manifeste, std::fs::read(chemin_manifeste(&p)).unwrap().as_slice());
        // Jamais d'écrasement ; ancre étrangère ou modèle altéré ⇒ refus, rien d'écrit.
        assert!(emballer_modele(&p, &ancres_essai, &sortie).is_err());
        let autre = crate::essai::ancres(&CleSignature::depuis_graine(&[3u8; 32]));
        let s2 = d.join("m2.mmia");
        assert!(emballer_modele(&p, &autre, &s2).is_err());
        assert!(!s2.exists());
        std::fs::write(&p, b"GGUF\x03\x00\x00\x00 FAUX modele de test").unwrap();
        assert!(emballer_modele(&p, &ancres_essai, &s2).is_err());
        assert!(!s2.exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[cfg(feature = "canal")]
    #[test]
    fn installation_par_l_outil_refuse_une_empreinte_differente() {
        let d = dossier("installer");
        let p = modele(&d);
        let cle_essai = cle();
        sceller_modele(&p, &cle_essai, "MIT", "2024-06").unwrap();
        let ancres_essai = crate::essai::ancres(&cle_essai);
        let art = d.join("qwen-mini.mmia");
        emballer_modele(&p, &ancres_essai, &art).unwrap();
        let cible = d.join("canal");
        let ancre = hex::encode(cle_essai.cle_publique());
        let args = |artefact: &Path, version: &str| -> Vec<String> {
            [
                "maia-llm", "installer-modele", "--dossier", cible.to_str().unwrap(), "--ancre",
                &ancre, "--composant", "qwen-mini", "--version", version, "--artefact",
                artefact.to_str().unwrap(),
            ]
            .iter()
            .map(ToString::to_string)
            .collect()
        };
        // Octet du modèle altéré dans l'artefact : empreinte différente ⇒ refus, rien installé.
        let mut altere = std::fs::read(&art).unwrap();
        let dernier = altere.len() - 1;
        altere[dernier] ^= 0x01;
        let art_altere = d.join("altere.mmia");
        std::fs::write(&art_altere, &altere).unwrap();
        let refus = executer_outil(&args(&art_altere, "1")).unwrap();
        assert!(matches!(&refus, Err(e) if e.contains("empreinte")), "{refus:?}");
        assert!(!cible.join("qwen-mini.gguf").exists());
        // Artefact intact : installé puis chargeable.
        assert!(executer_outil(&args(&art, "1")).unwrap().is_ok());
        assert!(verifier_modele(&cible.join("qwen-mini.gguf"), &ancres_essai).is_ok());
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
