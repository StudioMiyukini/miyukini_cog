//! @id     central_service_manager_installer
//! @do     install_package
//! @role   orchestration
//! @layer  app
//! @human  central — module « service manager installer ».

//! Installation et désinstallation de packages service (.msp).
//!
//! **Signature obligatoire (CA-01, EX-09, EX-26)** : un package n'est installé
//! que sous forme de [`PaquetAccepte`], obtenu par [`verifier_paquet_market`]
//! (enveloppe Sceau v1 signée par un éditeur épinglé, voir
//! `miyumarket::signature`). Aucun octet n'est décompressé ni écrit avant cette
//! vérification. Seule exception : build debug + `MIYUKINI_MARKET_DEV_NON_SIGNE=1`.
//!
//! L'archive ZIP signée contient :
//! - `service.manifest.json` : manifeste du service
//! - `bin/` : binaires du service
//! - `assets/` : icônes, ressources statiques (optionnel)

use std::io::Read;

#[cfg(debug_assertions)]
use super::registry::service_install_dir;
use super::gouvernance::{ActionMarket, GouvernanceMarket, OperationMarket};
use super::launcher::Provenance;
use super::registry::{InstalledService, ServiceRegistryData};
use miyumarket::manifest::ServiceManifest;
use miyumarket::signature::{
    hex_encoder, verifier_paquet, ErreurSignaturePaquet, ManifestePaquet, PaquetVerifie,
};
use miyusceau::AncresDeConfiance;

/// Paquet autorisé à l'installation. Ne se construit que par
/// [`verifier_paquet_market`].
#[derive(Debug)]
pub enum PaquetAccepte {
    /// Paquet signé par un éditeur de confiance (cas normal, seul cas en release).
    Signe(PaquetVerifie),
    /// Archive non signée acceptée par dérogation de développement (debug uniquement).
    #[cfg(debug_assertions)]
    NonSigneDev(Vec<u8>),
}

impl PaquetAccepte {
    /// Archive ZIP à extraire.
    fn archive(&self) -> &[u8] {
        match self {
            Self::Signe(p) => p.archive(),
            #[cfg(debug_assertions)]
            Self::NonSigneDev(a) => a,
        }
    }

    /// Titre signé (absent pour une dérogation dev).
    fn titre(&self) -> Option<&ManifestePaquet> {
        match self {
            Self::Signe(p) => Some(p.manifeste()),
            #[cfg(debug_assertions)]
            Self::NonSigneDev(_) => None,
        }
    }

    /// Action StrongFather décrivant ce paquet vérifié (provenance, niveau, signataire).
    pub fn action_market(
        &self,
        operation: OperationMarket,
        id: &str,
        version: Option<&str>,
    ) -> ActionMarket {
        match self {
            Self::Signe(p) => {
                let provenance = if p.niveau() == miyusceau::NiveauConfiance::Souverain {
                    Provenance::SigneeOfficielle
                } else {
                    Provenance::SigneeTierce
                };
                ActionMarket::nouvelle(operation, id, &p.manifeste().version).avec_provenance(
                    provenance,
                    Some(p.niveau()),
                    Some(p.editeur().as_slice()),
                )
            }
            #[cfg(debug_assertions)]
            Self::NonSigneDev(_) => ActionMarket::nouvelle(operation, id, version.unwrap_or(""))
                .avec_provenance(Provenance::NonSigneeDev, None, None),
        }
    }

    /// Sceau à conserver (hex) pour re-vérification au lancement.
    fn sceau_hex(&self) -> Option<String> {
        match self {
            Self::Signe(p) => Some(hex_encoder(p.sceau())),
            #[cfg(debug_assertions)]
            Self::NonSigneDev(_) => None,
        }
    }
}

/// Vérifie un paquet téléchargé AVANT toute écriture (fail-closed).
///
/// `dev_non_signe` n'a d'effet qu'en build debug, et seulement pour une archive
/// sans signature : un paquet signé mais altéré, d'un autre éditeur ou d'un
/// autre service reste refusé.
pub fn verifier_paquet_market(
    octets: &[u8],
    id_attendu: &str,
    version_attendue: Option<&str>,
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
    dev_non_signe: bool,
) -> Result<PaquetAccepte, String> {
    match verifier_paquet(octets, id_attendu, version_attendue, ancres, maintenant_secs) {
        Ok(p) if crate::service_manager::confiance::id_reserve_officiel(id_attendu)
            && p.niveau() != miyusceau::NiveauConfiance::Souverain =>
        {
            Err(format!(
                "Paquet '{id_attendu}' refusé : identifiant réservé aux éditeurs officiels"
            ))
        }
        Ok(p) => {
            tracing::info!(
                "Paquet '{id_attendu}' v{} : signature vérifiée (éditeur {})",
                p.manifeste().version,
                hex_encoder(&p.editeur()[..8])
            );
            Ok(PaquetAccepte::Signe(p))
        }
        Err(ErreurSignaturePaquet::NonSigne | ErreurSignaturePaquet::AucuneAncre)
            if dev_non_signe && cfg!(debug_assertions) && !miyumarket::signature::est_paquet_signe(octets) =>
        {
            #[cfg(debug_assertions)]
            {
                tracing::error!(
                    "⚠⚠ DÉROGATION DEV : paquet '{id_attendu}' NON SIGNÉ accepté \
                     (MIYUKINI_MARKET_DEV_NON_SIGNE=1, build debug). Interdit en production. ⚠⚠"
                );
                Ok(PaquetAccepte::NonSigneDev(octets.to_vec()))
            }
            #[cfg(not(debug_assertions))]
            {
                Err("paquet non signé : installation refusée".to_string())
            }
        }
        Err(e) => {
            tracing::warn!("Paquet '{id_attendu}' refusé : {e}");
            Err(format!("Installation refusée : {e}"))
        }
    }
}

/// Valide le champ `binary_name` d'un manifeste : simple nom de fichier,
/// sans séparateur, sans `..`, sans nom absolu/lecteur, non vide.
/// Fail-closed : tout nom douteux est refusé (audit C3 — évite `../../x`).
pub fn validate_binary_name(name: &str) -> Result<(), String> {
    let invalid = || format!("binary_name invalide '{name}' : un simple nom de fichier est requis");
    if name.is_empty() || name == "." || name == ".." || name.len() > 255 {
        return Err(invalid());
    }
    if name.contains('/') || name.contains('\\') || name.contains(':') || name.contains('\0') {
        return Err(invalid());
    }
    if name.contains("..") {
        return Err(invalid());
    }
    // Contrôle final : un seul composant « normal » côté std::path.
    let mut comps = std::path::Path::new(name).components();
    match (comps.next(), comps.next()) {
        (Some(std::path::Component::Normal(_)), None) => Ok(()),
        _ => Err(invalid()),
    }
}

/// Nom du binaire (validé) pour un manifeste donné.
fn resolve_binary_name(manifest: &ServiceManifest) -> Result<String, String> {
    match manifest.binary_name.as_deref() {
        Some(n) => {
            validate_binary_name(n)?;
            Ok(n.to_string())
        }
        None => Ok(if cfg!(windows) {
            format!("{}.exe", manifest.id)
        } else {
            manifest.id.clone()
        }),
    }
}

/// Calcule le chemin de sortie d'une entrée d'archive, ou refuse (Zip Slip).
/// Rejette : chemin absolu, composant `..`, lecteur Windows, entrée symlink,
/// et tout chemin qui sortirait de `install_dir` après jointure.
fn safe_entry_path(
    file: &zip::read::ZipFile<'_>,
    install_dir: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    let raw = file.name().to_string();
    if file.is_symlink() {
        return Err(format!("Entrée symlink refusée dans le package : '{raw}'"));
    }
    if raw.starts_with('/') || raw.starts_with('\\') || raw.contains(':') {
        return Err(format!("Chemin absolu refusé dans le package : '{raw}'"));
    }
    if raw.split(['/', '\\']).any(|c| c == "..") {
        return Err(format!("Chemin '..' refusé dans le package : '{raw}'"));
    }
    let rel = file
        .enclosed_name()
        .ok_or_else(|| format!("Chemin d'entrée dangereux refusé : '{raw}'"))?;
    let out = install_dir.join(&rel);
    // Défense en profondeur : après jointure, rester sous install_dir.
    if !out.starts_with(install_dir)
        || out
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(format!("Chemin hors du répertoire d'installation : '{raw}'"));
    }
    Ok(out)
}

/// Contexte de vérification d'un paquet Market (ancres, horloge, dérogation dev).
#[derive(Debug, Clone, Copy)]
pub struct ContexteVerification<'a> {
    /// Éditeurs épinglés.
    pub ancres: &'a AncresDeConfiance,
    /// Horloge Unix (secondes).
    pub maintenant_secs: u64,
    /// Dérogation « non signé » (debug uniquement).
    pub dev_non_signe: bool,
}

/// Vérifie puis installe : chemin unique du Market.
///
/// Ordre : signature Sceau (CA-01) → décision StrongFather `operation` (K-11) →
/// écriture. Refus à l'une ou l'autre étape ⇒ aucun octet écrit.
#[allow(clippy::too_many_arguments)]
pub fn verifier_puis_installer(
    octets: &[u8],
    id_attendu: &str,
    version_attendue: Option<&str>,
    contexte: ContexteVerification<'_>,
    gouvernance: &GouvernanceMarket,
    operation: OperationMarket,
    registry: &mut ServiceRegistryData,
    root: &std::path::Path,
) -> Result<String, String> {
    let paquet = verifier_paquet_market(
        octets,
        id_attendu,
        version_attendue,
        contexte.ancres,
        contexte.maintenant_secs,
        contexte.dev_non_signe,
    )?;
    let action = paquet.action_market(operation, id_attendu, version_attendue);
    gouvernance.executer_action_market(&action, |_jeton| {
        install_package_with_root(&paquet, registry, id_attendu, root)
    })?
}

/// Écriture d'un paquet vérifié. N'est appelée que dans la fermeture autorisée de
/// [`verifier_puis_installer`] (aucun autre chemin d'installation : K-11).
/// `expected_id` : le manifeste du package doit porter exactement cet id (audit C3).
fn install_package_with_root(
    paquet: &PaquetAccepte,
    registry: &mut ServiceRegistryData,
    expected_id: &str,
    root: &std::path::Path,
) -> Result<String, String> {
    let cursor = std::io::Cursor::new(paquet.archive());
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| format!("Package .msp invalide (pas une archive ZIP) : {e}"))?;

    // 1. Lire le manifeste
    let manifest: ServiceManifest = {
        let mut manifest_file = archive
            .by_name("service.manifest.json")
            .map_err(|_| "service.manifest.json manquant dans le package")?;
        let mut content = String::new();
        manifest_file
            .read_to_string(&mut content)
            .map_err(|e| format!("Lecture du manifeste échouée : {e}"))?;
        serde_json::from_str(&content).map_err(|e| format!("Manifeste JSON invalide : {e}"))?
    };

    manifest
        .validate()
        .map_err(|e| format!("Manifeste invalide : {e}"))?;

    // Le manifeste doit correspondre au service demandé.
    if manifest.id != expected_id {
        return Err(format!(
            "Identifiant du manifeste ('{}') différent du service demandé ('{expected_id}') : installation refusée",
            manifest.id
        ));
    }
    // Le manifeste embarqué doit correspondre au titre signé (id, version, binaire).
    if let Some(titre) = paquet.titre() {
        if manifest.id != titre.service_id
            || manifest.version != titre.version
            || manifest.binary_name != titre.nom_binaire
        {
            return Err(format!(
                "Manifeste du package incohérent avec la signature (id/version/binary_name) pour '{expected_id}' : installation refusée"
            ));
        }
    }
    let binary_name = resolve_binary_name(&manifest)?;

    let service_id = manifest.id.clone();
    let install_dir = root.join(&service_id);

    // 2. Pré-validation de TOUTES les entrées avant d'écrire quoi que ce soit.
    let mut plan: Vec<(usize, std::path::PathBuf, bool)> = Vec::new();
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| format!("Erreur lecture archive index {i}: {e}"))?;
        let out_path = safe_entry_path(&file, &install_dir)?;
        plan.push((i, out_path, file.is_dir()));
    }

    tracing::info!(
        "Installation du service '{}' v{} dans {:?}",
        service_id,
        manifest.version,
        install_dir
    );

    // 3. Créer le répertoire d'installation
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Impossible de créer {}: {e}", install_dir.display()))?;
    std::fs::create_dir_all(install_dir.join("bin"))
        .map_err(|e| format!("Impossible de créer bin/ : {e}"))?;
    std::fs::create_dir_all(install_dir.join("data"))
        .map_err(|e| format!("Impossible de créer data/ : {e}"))?;

    // 4. Extraire les fichiers
    for (i, out_path, is_dir) in plan {
        if is_dir {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| format!("Création de '{}' échouée : {e}", out_path.display()))?;
            continue;
        }
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Erreur lecture archive index {i}: {e}"))?;
        let name = file.name().to_string();
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Création de '{}' échouée : {e}", parent.display()))?;
        }
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| format!("Lecture de '{name}' échouée : {e}"))?;
        std::fs::write(&out_path, &buf)
            .map_err(|e| format!("Écriture de '{}' échouée : {e}", out_path.display()))?;
    }

    // 5. Sauvegarder le manifeste séparément
    let manifest_path = install_dir.join("service.manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Sérialisation du manifeste échouée : {e}"))?;
    std::fs::write(&manifest_path, &manifest_json)
        .map_err(|e| format!("Écriture du manifeste échouée : {e}"))?;

    // 6. Chemin du binaire (nom déjà validé)
    let binary_path = install_dir.join("bin").join(&binary_name);

    // 7. Enregistrer dans le registre
    let installed = InstalledService {
        manifest,
        install_dir,
        binary_path,
        installed_at: chrono::Utc::now().to_rfc3339(),
        updated_at: None,
        sceau_paquet: paquet.sceau_hex(),
    };
    registry.services.insert(service_id.clone(), installed);

    tracing::info!("Service '{}' installé avec succès", service_id);
    Ok(service_id)
}

/// Installe un service directement depuis un binaire (sans package .msp).
/// Développement uniquement : absent des builds release (pas de signature).
#[cfg(debug_assertions)]
pub fn install_from_binary(
    manifest: ServiceManifest,
    binary_source: &std::path::Path,
    registry: &mut ServiceRegistryData,
) -> Result<String, String> {
    manifest
        .validate()
        .map_err(|e| format!("Manifeste invalide : {e}"))?;
    let service_id = manifest.id.clone();
    let install_dir = service_install_dir(&service_id);

    tracing::info!(
        "Installation directe du service '{}' depuis {:?}",
        service_id,
        binary_source
    );

    // Créer les répertoires
    std::fs::create_dir_all(install_dir.join("bin"))
        .map_err(|e| format!("Impossible de créer bin/ : {e}"))?;
    std::fs::create_dir_all(install_dir.join("data"))
        .map_err(|e| format!("Impossible de créer data/ : {e}"))?;

    // Copier le binaire
    let binary_name = manifest.binary_name.clone().unwrap_or_else(|| {
        binary_source
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                if cfg!(windows) {
                    format!("{}.exe", service_id)
                } else {
                    service_id.clone()
                }
            })
    });
    validate_binary_name(&binary_name)?;
    let binary_path = install_dir.join("bin").join(&binary_name);
    std::fs::copy(binary_source, &binary_path)
        .map_err(|e| format!("Copie du binaire échouée : {e}"))?;

    // Sauvegarder le manifeste
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Sérialisation du manifeste : {e}"))?;
    std::fs::write(install_dir.join("service.manifest.json"), &manifest_json)
        .map_err(|e| format!("Écriture du manifeste : {e}"))?;

    // Enregistrer
    let installed = InstalledService {
        manifest,
        install_dir: install_dir.clone(),
        binary_path: binary_path.clone(),
        installed_at: chrono::Utc::now().to_rfc3339(),
        updated_at: None,
        sceau_paquet: None,
    };
    registry.services.insert(service_id.clone(), installed);

    tracing::info!(
        "Service '{}' installé (binaire: {:?})",
        service_id,
        binary_path
    );
    Ok(service_id)
}

/// Désinstalle un service : supprime le répertoire d'installation et l'entrée du registre.
pub fn uninstall_service(
    service_id: &str,
    registry: &mut ServiceRegistryData,
) -> Result<(), String> {
    let service = registry
        .services
        .remove(service_id)
        .ok_or_else(|| format!("Service '{service_id}' non trouvé dans le registre"))?;

    tracing::info!(
        "Désinstallation du service '{}' depuis {:?}",
        service_id,
        service.install_dir
    );

    // Supprimer le répertoire d'installation
    if service.install_dir.exists() {
        std::fs::remove_dir_all(&service.install_dir).map_err(|e| {
            format!(
                "Suppression de {} échouée : {e}. Supprimez manuellement.",
                service.install_dir.display()
            )
        })?;
    }

    tracing::info!("Service '{}' désinstallé", service_id);
    Ok(())
}

/// Scanne le répertoire des services pour détecter des services installés manuellement
/// (par ex. en copiant un dossier avec un service.manifest.json + binaire).
pub fn scan_installed_services(registry: &mut ServiceRegistryData) {
    let root = super::registry::services_root_dir();
    if !root.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(&root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let manifest_path = path.join("service.manifest.json");
        if !manifest_path.exists() {
            continue;
        }

        // Lire le manifeste
        let Ok(content) = std::fs::read_to_string(&manifest_path) else {
            continue;
        };
        let Ok(manifest): Result<ServiceManifest, _> = serde_json::from_str(&content) else {
            continue;
        };

        let service_id = manifest.id.clone();

        // Ne pas écraser un service déjà enregistré
        if registry.services.contains_key(&service_id) {
            continue;
        }

        // Déduire le chemin du binaire
        let binary_name = manifest.binary_name.clone().unwrap_or_else(|| {
            if cfg!(windows) {
                format!("{}.exe", service_id)
            } else {
                service_id.clone()
            }
        });
        if let Err(e) = validate_binary_name(&binary_name) {
            tracing::warn!("Service '{service_id}' ignoré par le scan : {e}");
            continue;
        }
        let binary_path = path.join("bin").join(&binary_name);

        if binary_path.exists() {
            tracing::info!(
                "Service détecté par scan : '{}' dans {:?}",
                service_id,
                path
            );
            registry.services.insert(
                service_id,
                InstalledService {
                    manifest,
                    install_dir: path,
                    binary_path,
                    installed_at: String::new(),
                    updated_at: None,
                    // Service copié à la main : aucun sceau, il ne se lance
                    // qu'avec la dérogation dev (voir launcher).
                    sceau_paquet: None,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn manifest_json(id: &str, binary_name: Option<&str>) -> String {
        let bn = binary_name
            .map(|b| format!(r#", "binary_name": "{b}""#))
            .unwrap_or_default();
        format!(
            r#"{{"id":"{id}","name":"Test","version":"1.0.0","developer":"dev","description":"d","icon":"x","service_type":"interne_cog","source":"tiers","min_central_version":"0.1.0"{bn}}}"#
        )
    }

    fn build_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                w.write_all(data).unwrap();
            }
            w.finish().unwrap();
        }
        buf
    }

    fn tmp_root(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("miyu-installer-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn cle_test() -> miyusceau::CleSignature {
        miyusceau::CleSignature::depuis_graine(&[7u8; 32])
    }

    fn ancres(cle: &miyusceau::CleSignature) -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle.cle_publique(), miyusceau::NiveauConfiance::Reconnu).unwrap();
        a
    }

    fn signer(zip: &[u8], id: &str, bin: Option<&str>, cle: &miyusceau::CleSignature) -> Vec<u8> {
        miyumarket::signature::signer_paquet(zip, id, "1.0.0", bin, cle).unwrap()
    }

    /// Signe l'archive pour `expected` (clé de test épinglée) puis vérifie et installe.
    fn try_install(entries: &[(&str, &[u8])], expected: &str, tag: &str) -> (Result<String, String>, std::path::PathBuf) {
        let root = tmp_root(tag);
        let cle = cle_test();
        let bytes = signer(&build_zip(entries), expected, None, &cle);
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&bytes, expected, Some("1.0.0"), &ancres(&cle), false, &mut reg, &root);
        (r, root)
    }

    /// Politiques StrongFather de base du dépôt.
    fn gouvernance_base() -> GouvernanceMarket {
        GouvernanceMarket::depuis_fichier(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/strongfather/market.toml"),
        )
    }

    /// Vérifie puis installe avec la politique de base (opération `installer`).
    fn vpi(
        octets: &[u8],
        id: &str,
        version: Option<&str>,
        ancres: &AncresDeConfiance,
        dev_non_signe: bool,
        reg: &mut ServiceRegistryData,
        root: &std::path::Path,
    ) -> Result<String, String> {
        let contexte = ContexteVerification { ancres, maintenant_secs: 0, dev_non_signe };
        verifier_puis_installer(octets, id, version, contexte, &gouvernance_base(), OperationMarket::Installer, reg, root)
    }

    fn racine_vide(root: &std::path::Path) -> bool {
        std::fs::read_dir(root).map_or(true, |mut d| d.next().is_none())
    }

    #[test]
    fn paquet_non_signe_refuse_sans_ecriture() {
        let m = manifest_json("svc", None);
        let zip = build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]);
        let root = tmp_root("nonsigne");
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&zip, "svc", Some("1.0.0"), &ancres(&cle_test()), false, &mut reg, &root);
        assert!(r.unwrap_err().contains("non signé"));
        assert!(racine_vide(&root), "rien ne doit être écrit");
        assert!(reg.services.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn paquet_altere_refuse_sans_ecriture() {
        let m = manifest_json("svc", None);
        let cle = cle_test();
        let mut paquet = signer(
            &build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]),
            "svc",
            None,
            &cle,
        );
        let n = paquet.len();
        paquet[n - 30] ^= 0xff;
        let root = tmp_root("altere");
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&paquet, "svc", Some("1.0.0"), &ancres(&cle), false, &mut reg, &root);
        assert!(r.is_err());
        assert!(racine_vide(&root));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn mauvais_editeur_refuse_sans_ecriture() {
        let m = manifest_json("svc", None);
        let autre = miyusceau::CleSignature::depuis_graine(&[9u8; 32]);
        let paquet = signer(
            &build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]),
            "svc",
            None,
            &autre,
        );
        let root = tmp_root("editeur");
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&paquet, "svc", Some("1.0.0"), &ancres(&cle_test()), false, &mut reg, &root);
        assert!(r.is_err());
        assert!(racine_vide(&root));
        // Aucune ancre : refus aussi.
        let r = vpi(&paquet, "svc", None, &AncresDeConfiance::new(), false, &mut reg, &root);
        assert!(r.is_err());
        assert!(racine_vide(&root));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn manifeste_incoherent_avec_signature_refuse() {
        // Signature pour binary_name absent, manifeste embarqué déclarant un autre binaire.
        let m = manifest_json("svc", Some("autre.bin"));
        let cle = cle_test();
        let paquet = signer(&build_zip(&[("service.manifest.json", m.as_bytes())]), "svc", None, &cle);
        let root = tmp_root("incoherent");
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&paquet, "svc", Some("1.0.0"), &ancres(&cle), false, &mut reg, &root);
        assert!(r.unwrap_err().contains("incohérent"));
        assert!(racine_vide(&root));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn paquet_signe_valide_conserve_le_sceau() {
        let m = manifest_json("svc", None);
        let cle = cle_test();
        let paquet = signer(
            &build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]),
            "svc",
            None,
            &cle,
        );
        let root = tmp_root("sceau");
        let mut reg = ServiceRegistryData::default();
        vpi(&paquet, "svc", Some("1.0.0"), &ancres(&cle), false, &mut reg, &root).unwrap();
        let sceau = reg.services["svc"].sceau_paquet.clone().expect("sceau conservé");
        let octets = miyumarket::signature::hex_decoder(&sceau).unwrap();
        assert!(miyumarket::signature::verifier_sceau_installe(&octets, "svc", &ancres(&cle), 0).is_ok());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn derogation_dev_accepte_seulement_le_non_signe() {
        let m = manifest_json("svc", None);
        let zip = build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]);
        let root = tmp_root("dev");
        let mut reg = ServiceRegistryData::default();
        let r = vpi(&zip, "svc", None, &AncresDeConfiance::new(), true, &mut reg, &root);
        assert_eq!(r.unwrap(), "svc");
        assert!(reg.services["svc"].sceau_paquet.is_none());
        // Un paquet signé par un éditeur inconnu reste refusé même avec la dérogation.
        let autre = miyusceau::CleSignature::depuis_graine(&[9u8; 32]);
        let paquet = signer(&zip, "svc", None, &autre);
        let r = vpi(&paquet, "svc", None, &ancres(&cle_test()), true, &mut reg, &root);
        assert!(r.is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_valide_installe() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")],
            "svc",
            "ok",
        );
        assert_eq!(r.unwrap(), "svc");
        assert!(root.join("svc/bin/svc").exists());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_slip_dotdot_refuse() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("../evil", b"x")],
            "svc",
            "dotdot",
        );
        assert!(r.is_err(), "{r:?}");
        assert!(!root.join("evil").exists());
        assert!(!root.join("svc").exists(), "rien ne doit être écrit");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_chemin_absolu_refuse() {
        let m = manifest_json("svc", None);
        let (r, root) = try_install(
            &[("service.manifest.json", m.as_bytes()), ("/tmp/miyu-evil-abs", b"x")],
            "svc",
            "abs",
        );
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn zip_symlink_refuse() {
        let m = manifest_json("svc", None);
        let mut buf = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
            let opts = zip::write::SimpleFileOptions::default();
            w.start_file("service.manifest.json", opts).unwrap();
            w.write_all(m.as_bytes()).unwrap();
            w.add_symlink("bin/link", "/etc/passwd", opts).unwrap();
            w.finish().unwrap();
        }
        let root = tmp_root("symlink");
        let mut reg = ServiceRegistryData::default();
        let cle = cle_test();
        let paquet = signer(&buf, "svc", None, &cle);
        let r = vpi(&paquet, "svc", None, &ancres(&cle), false, &mut reg, &root);
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn binary_name_invalide_refuse() {
        for bad in ["../x", "a/b", "a\\b", "..", "", "C:x"] {
            assert!(validate_binary_name(bad).is_err(), "{bad:?} devrait être refusé");
        }
        assert!(validate_binary_name("svc.exe").is_ok());
        let m = manifest_json("svc", Some("../../evil"));
        let (r, root) = try_install(&[("service.manifest.json", m.as_bytes())], "svc", "bn");
        assert!(r.is_err(), "{r:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn id_manifeste_different_refuse() {
        let m = manifest_json("autre", None);
        let (r, root) = try_install(&[("service.manifest.json", m.as_bytes())], "svc", "id");
        assert!(r.unwrap_err().contains("différent"));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Paquet valide et signé, mais StrongFather refuse : rien n'est écrit.
    #[test]
    fn refus_strongfather_aucune_ecriture() {
        let m = manifest_json("svc", None);
        let cle = cle_test();
        let paquet = signer(
            &build_zip(&[("service.manifest.json", m.as_bytes()), ("bin/svc", b"x")]),
            "svc",
            None,
            &cle,
        );
        let a = ancres(&cle);
        let contexte = ContexteVerification { ancres: &a, maintenant_secs: 0, dev_non_signe: false };
        // Porte absente (fichier manquant) : refus fail-closed.
        let absente = GouvernanceMarket::depuis_fichier(std::path::Path::new("/inexistant/market.toml"));
        let root = tmp_root("sf-absente");
        let mut reg = ServiceRegistryData::default();
        let r = verifier_puis_installer(&paquet, "svc", Some("1.0.0"), contexte, &absente, OperationMarket::Installer, &mut reg, &root);
        assert!(r.unwrap_err().contains("refusée"));
        assert!(racine_vide(&root) && reg.services.is_empty());
        // Politique qui n'autorise que la désinstallation : l'installation reste refusée.
        let restrictive = bondingbrother::strongfather::PolicySet::from_toml_str(
            "version = 1\n[[policy]]\nid = \"seulement-desinstaller\"\ntype = \"permission\"\neffect = \"authorize\"\nrule = \"data.operation == 'desinstaller'\"\n",
        )
        .unwrap();
        let g = GouvernanceMarket::depuis_porte(bondingbrother::PorteStrongFather::depuis_politiques(restrictive).unwrap());
        let r = verifier_puis_installer(&paquet, "svc", Some("1.0.0"), contexte, &g, OperationMarket::Installer, &mut reg, &root);
        assert!(r.unwrap_err().contains("StrongFather"));
        assert!(racine_vide(&root) && reg.services.is_empty());
        // Politique de base : installation autorisée (éditeur épinglé localement).
        let r = verifier_puis_installer(&paquet, "svc", Some("1.0.0"), contexte, &gouvernance_base(), OperationMarket::MettreAJour, &mut reg, &root);
        assert_eq!(r.unwrap(), "svc");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn action_market_reflete_le_paquet_verifie() {
        let m = manifest_json("svc", None);
        let cle = cle_test();
        let paquet = signer(&build_zip(&[("service.manifest.json", m.as_bytes())]), "svc", None, &cle);
        let p = verifier_paquet_market(&paquet, "svc", Some("1.0.0"), &ancres(&cle), 0, false).unwrap();
        let i = p.action_market(OperationMarket::Installer, "svc", Some("1.0.0")).intention();
        assert_eq!(i.data.content["provenance"], "signee_tierce");
        assert_eq!(i.data.content["niveau_confiance"], "reconnu");
        assert_eq!(i.data.content["version"], "1.0.0");
        assert_ne!(i.data.content["editeur"], "aucun");
    }
}
