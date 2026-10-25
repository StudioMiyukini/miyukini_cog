//! @id     miyucloud_application_services_reclassement_cdr_service
//! @do     reclasser_fichiers_herites_cdr
//! @role   securite
//! @layer  app
//! @human  miyucloud — reclassement des fichiers hérités `inconnu` : chacun repasse par le CDR, désarmable ⇒ désarmé, sinon quarantaine.

//! Reclassement des fichiers hérités (plan §5, décision A-004, EX-45).
//!
//! Les fichiers antérieurs au CDR portent l'état `inconnu` (valeur par défaut
//! de la migration `20261025000001_cdr_desarmement`). Ils ne sont **jamais
//! servis** dans cet état ([`EtatCdr::attend_reclassement`]). Cette tâche les
//! repasse un à un par l'entonnoir unique ([`DesarmementService`]) :
//!
//! - **désarmable** ⇒ `desarme` : les octets reconstruits remplacent le contenu
//!   (nouveau blob, référence de l'ancien retirée) ;
//! - **non désarmable** ⇒ `quarantaine` (décision du propriétaire) : type non
//!   pris en charge, trop volumineux **ou refusé** par le CDR (polyglotte,
//!   malformé…) — un fichier hérité ne peut pas être « refusé à l'écriture »,
//!   il est donc conservé tel quel, non désarmé, téléchargeable seulement sur
//!   accord explicite, jamais ouvert par COG ;
//! - **erreur technique** (blob illisible, base indisponible) ⇒ le fichier
//!   **reste `inconnu`** (donc non servi) et sera repris au passage suivant.
//!
//! # Concurrence (fermé par défaut)
//!
//! Chaque écriture est **conditionnelle** : la ligne doit être encore
//! `inconnu` avec le blob lu. Si l'utilisateur a remplacé le contenu entre
//! temps (nouvelle écriture, déjà passée par le CDR), rien n'est écrit et le
//! blob reconstruit est libéré.
//!
//! Avec changement de contenu, l'ordre est : contenu remplacé (état toujours
//! `inconnu`, donc non servi ; ligne et références des deux blobs écrites
//! dans une seule transaction) → caches dérivés invalidés (hachage, contenu,
//! transcodages, vignettes) → état `desarme`. Une lecture qui voit `desarme`
//! ne peut donc pas tomber sur un cache de l'ancien contenu.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use serde_json::json;
use uuid::Uuid;

use crate::application::services::desarmement_service::DesarmementService;
use crate::application::services::supervision::superviser_tache_periodique;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::etat_cdr::{EtatCdr, MarqueCdr};

/// Nombre de fichiers lus par lot.
pub const TAILLE_LOT_RECLASSEMENT: i64 = 200;

/// Fichier hérité en attente de reclassement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FichierInconnu {
    /// Identifiant de la ligne `storage.files`.
    pub id: Uuid,
    /// Blob lu (condition de l'écriture).
    pub blob_hash: String,
    /// Type MIME déclaré.
    pub mime_type: String,
}

/// Contenu reconstruit par le CDR, à substituer à l'ancien.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContenuReclasse {
    /// Hachage BLAKE3 des octets reconstruits, s'il est déjà connu.
    pub hachage: Option<String>,
    /// Taille des octets reconstruits.
    pub taille: u64,
    /// Type MIME réel de la reconstruction.
    pub mime_type: String,
}

/// Accès au stockage nécessaire au reclassement (PostgreSQL + magasin de
/// blobs en production, mémoire en test).
pub trait DepotReclassementCdr: Send + Sync + 'static {
    /// Fichiers `inconnu` (corbeille comprise) d'identifiant > `apres`, par
    /// identifiant croissant, au plus `limite`.
    fn lister_inconnus(
        &self,
        apres: Option<Uuid>,
        limite: i64,
    ) -> impl Future<Output = Result<Vec<FichierInconnu>, DomainError>> + Send;

    /// Copie les octets du blob `hash` dans `vers`.
    fn extraire_blob(
        &self,
        hash: &str,
        vers: &Path,
    ) -> impl Future<Output = Result<(), DomainError>> + Send;

    /// Remplace le contenu de `id` par les octets de `depuis` (consommé)
    /// **si** la ligne est encore `inconnu` avec `ancien_hash` ; l'état reste
    /// `inconnu`. **Atomique** : la ligne, la référence du nouveau blob et
    /// le retrait de celle de l'ancien sont écrits dans une seule
    /// transaction (aucune référence perdue ni fantôme, même sur arrêt
    /// brutal). Rend le hachage du nouveau blob si remplacé, `None` si la
    /// ligne a changé entre temps (rien n'est écrit).
    fn remplacer_contenu(
        &self,
        id: Uuid,
        ancien_hash: &str,
        depuis: &Path,
        contenu: &ContenuReclasse,
    ) -> impl Future<Output = Result<Option<String>, DomainError>> + Send;

    /// Nombre de fichiers encore `inconnu` (corbeille comprise).
    fn compter_inconnus(&self) -> impl Future<Output = Result<u64, DomainError>> + Send;

    /// Invalide les données dérivées du contenu de `id` (caches, vignettes).
    fn invalider_derives(&self, id: Uuid) -> impl Future<Output = ()> + Send;

    /// Pose l'état `marque` sur `id` **si** la ligne est encore `inconnu`
    /// avec `hash_attendu`. Vrai si marqué.
    fn marquer(
        &self,
        id: Uuid,
        hash_attendu: &str,
        marque: &MarqueCdr,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
}

/// Issue du reclassement d'un fichier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueReclassement {
    /// Désarmé (contenu reconstruit, éventuellement identique).
    Desarme,
    /// Non désarmable : quarantaine.
    Quarantaine,
    /// Modifié entre temps par une autre écriture : rien écrit.
    Concurrence,
}

/// Bilan d'un passage complet.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BilanReclassement {
    /// Fichiers examinés.
    pub examines: u64,
    /// Classés `desarme`.
    pub desarmes: u64,
    /// Classés `quarantaine`.
    pub quarantaines: u64,
    /// Ignorés (écriture concurrente).
    pub concurrences: u64,
    /// Erreurs techniques (fichier resté `inconnu`, non servi).
    pub erreurs: u64,
    /// Fichiers encore `inconnu` (non servis) à la fin du passage ; `None`
    /// si le décompte a échoué.
    pub restants: Option<u64>,
}

/// Tâche de reclassement des fichiers hérités.
pub struct ReclassementCdrService<D: DepotReclassementCdr> {
    depot: Arc<D>,
    desarmement: Arc<DesarmementService>,
    repertoire_temp: PathBuf,
    /// Fichiers `inconnu` restants au dernier passage (-1 : inconnu).
    restants: AtomicI64,
}

impl<D: DepotReclassementCdr> ReclassementCdrService<D> {
    /// Tâche adossée à `depot`, passant par `desarmement` ; fichiers de
    /// travail sous `repertoire_temp` (même système de fichiers que les blobs
    /// de préférence).
    pub fn new(
        depot: Arc<D>,
        desarmement: Arc<DesarmementService>,
        repertoire_temp: PathBuf,
    ) -> Self {
        Self {
            depot,
            desarmement,
            repertoire_temp,
            restants: AtomicI64::new(-1),
        }
    }

    /// Métrique : fichiers `inconnu` (non servis) restants au dernier
    /// passage terminé ; `None` avant le premier décompte réussi.
    pub fn inconnus_restants(&self) -> Option<u64> {
        u64::try_from(self.restants.load(Ordering::Relaxed)).ok()
    }

    /// Reclasse tous les fichiers `inconnu` présents (un passage). Les
    /// fichiers en erreur restent `inconnu` (non servis) et sont repris au
    /// passage suivant.
    pub async fn reclasser_tout(&self) -> Result<BilanReclassement, DomainError> {
        tokio::fs::create_dir_all(&self.repertoire_temp)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "ReclassementCdr",
                    format!("répertoire de travail : {e}"),
                )
            })?;
        let mut bilan = BilanReclassement::default();
        let mut apres = None;
        loop {
            let lot = self
                .depot
                .lister_inconnus(apres, TAILLE_LOT_RECLASSEMENT)
                .await?;
            let Some(dernier) = lot.last() else {
                break;
            };
            apres = Some(dernier.id);
            let plein = lot.len() as i64 >= TAILLE_LOT_RECLASSEMENT;
            for fichier in &lot {
                bilan.examines += 1;
                match self.reclasser(fichier).await {
                    Ok(IssueReclassement::Desarme) => bilan.desarmes += 1,
                    Ok(IssueReclassement::Quarantaine) => bilan.quarantaines += 1,
                    Ok(IssueReclassement::Concurrence) => bilan.concurrences += 1,
                    Err(e) => {
                        bilan.erreurs += 1;
                        tracing::warn!(
                            "Reclassement CDR : fichier {} resté « inconnu » (non servi) : {}",
                            fichier.id,
                            e
                        );
                    }
                }
            }
            if !plein {
                break;
            }
        }
        bilan.restants = match self.depot.compter_inconnus().await {
            Ok(n) => {
                self.restants
                    .store(i64::try_from(n).unwrap_or(i64::MAX), Ordering::Relaxed);
                Some(n)
            }
            Err(e) => {
                tracing::warn!(
                    "Reclassement CDR : décompte des fichiers « inconnu » impossible : {e}"
                );
                None
            }
        };
        Ok(bilan)
    }

    /// Reclasse un fichier : CDR, puis écriture conditionnelle.
    pub async fn reclasser(
        &self,
        fichier: &FichierInconnu,
    ) -> Result<IssueReclassement, DomainError> {
        let temp = tempfile::Builder::new()
            .prefix("reclassement-cdr-")
            .tempfile_in(&self.repertoire_temp)
            .map_err(|e| {
                DomainError::internal_error("ReclassementCdr", format!("fichier de travail : {e}"))
            })?
            .into_temp_path();
        self.depot.extraire_blob(&fichier.blob_hash, &temp).await?;

        let issue = self
            .desarmement
            .desarmer_fichier(&temp, &fichier.mime_type, Some(fichier.blob_hash.clone()))
            .await;
        match issue {
            Ok(issue) if issue.etat == EtatCdr::Desarme => {
                let marque = Self::marque_reclassee(issue.marque());
                let nouveau = issue.hachage.clone();
                if nouveau.as_deref() == Some(fichier.blob_hash.as_str()) {
                    // Contenu déjà sain (reconstruction identique).
                    return self
                        .marquer(fichier, &fichier.blob_hash, &marque, EtatCdr::Desarme)
                        .await;
                }
                let contenu = ContenuReclasse {
                    hachage: nouveau,
                    taille: issue.taille,
                    mime_type: issue.type_mime.clone(),
                };
                let Some(hash) = self
                    .depot
                    .remplacer_contenu(fichier.id, &fichier.blob_hash, &temp, &contenu)
                    .await?
                else {
                    return Ok(IssueReclassement::Concurrence);
                };
                self.depot.invalider_derives(fichier.id).await;
                self.marquer(fichier, &hash, &marque, EtatCdr::Desarme)
                    .await
            }
            Ok(issue) => {
                // Quarantaine : octets d'origine conservés.
                let marque = Self::marque_reclassee(issue.marque());
                self.marquer(fichier, &fichier.blob_hash, &marque, EtatCdr::Quarantaine)
                    .await
            }
            Err(e) if e.kind == ErrorKind::InvalidInput => {
                // Refusé par le CDR (polyglotte, malformé…) : non désarmable
                // ⇒ quarantaine (décision du propriétaire), jamais servi tel quel.
                let marque = MarqueCdr {
                    etat: EtatCdr::Quarantaine,
                    rapport: Some(json!({
                        "issue": "quarantaine",
                        "type_declare": fichier.mime_type,
                        "motif": format!("non désarmable : {}", e.message),
                        "reclassement": true,
                    })),
                };
                self.marquer(fichier, &fichier.blob_hash, &marque, EtatCdr::Quarantaine)
                    .await
            }
            Err(e) => Err(e),
        }
    }

    async fn marquer(
        &self,
        fichier: &FichierInconnu,
        hash: &str,
        marque: &MarqueCdr,
        etat: EtatCdr,
    ) -> Result<IssueReclassement, DomainError> {
        if !self.depot.marquer(fichier.id, hash, marque).await? {
            return Ok(IssueReclassement::Concurrence);
        }
        tracing::info!(
            "Reclassement CDR : fichier {} classé « {} »",
            fichier.id,
            etat
        );
        Ok(if etat == EtatCdr::Desarme {
            IssueReclassement::Desarme
        } else {
            IssueReclassement::Quarantaine
        })
    }

    fn marque_reclassee(mut marque: MarqueCdr) -> MarqueCdr {
        if let Some(serde_json::Value::Object(o)) = marque.rapport.as_mut() {
            o.insert("reclassement".to_string(), serde_json::Value::Bool(true));
        }
        marque
    }
}

/// Journalise le bilan d'un passage, dont la métrique « fichiers `inconnu`
/// restants » (non servis tant qu'ils ne sont pas reclassés).
pub fn journaliser_bilan(b: &BilanReclassement) {
    if b.examines > 0 {
        tracing::info!(
            "Reclassement CDR : {} examinés, {} désarmés, {} en quarantaine, {} concurrents, {} en erreur (restés « inconnu »)",
            b.examines,
            b.desarmes,
            b.quarantaines,
            b.concurrences,
            b.erreurs
        );
    }
    match b.restants {
        Some(0) => tracing::debug!("Reclassement CDR : aucun fichier « inconnu » restant"),
        Some(n) => tracing::warn!(
            "Reclassement CDR : {n} fichier(s) « inconnu » restant(s) (non servis jusqu'au reclassement)"
        ),
        None => {}
    }
}

/// Lance la tâche de reclassement **supervisée** : un passage immédiat puis
/// toutes les `periode` ; si la tâche s'arrête (panique comprise), l'erreur
/// est journalisée et la tâche relancée après `delai_relance` (voir
/// [`superviser_tache_periodique`]).
pub fn superviser_reclassement<D: DepotReclassementCdr>(
    service: Arc<ReclassementCdrService<D>>,
    periode: Duration,
    delai_relance: Duration,
) -> tokio::task::JoinHandle<()> {
    superviser_tache_periodique("Reclassement CDR", periode, delai_relance, move || {
        let service = service.clone();
        async move {
            match service.reclasser_tout().await {
                Ok(b) => journaliser_bilan(&b),
                Err(e) => tracing::warn!(
                    "Reclassement CDR interrompu (nouvel essai au prochain passage) : {e}"
                ),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::services::desarmement_service::tests::png_avec_metadonnees;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// Ligne de fichier en mémoire.
    #[derive(Debug, Clone)]
    struct Ligne {
        blob: String,
        mime: String,
        etat: EtatCdr,
        rapport: Option<serde_json::Value>,
    }

    /// Dépôt mémoire : mêmes conditions que PostgreSQL.
    #[derive(Default)]
    struct DepotMemoire {
        lignes: Mutex<HashMap<Uuid, Ligne>>,
        blobs: Mutex<HashMap<String, (Vec<u8>, i64)>>,
        invalides: Mutex<Vec<Uuid>>,
        /// Simule une écriture concurrente juste avant `remplacer_contenu`.
        concurrence: Mutex<Option<Uuid>>,
        /// Nombre de paniques simulées au début de `lister_inconnus`.
        paniques: std::sync::atomic::AtomicI64,
    }

    impl DepotMemoire {
        fn ajouter(&self, octets: &[u8], mime: &str) -> Uuid {
            let hash = blake3::hash(octets).to_hex().to_string();
            let mut blobs = self.blobs.lock().unwrap();
            blobs.entry(hash.clone()).or_insert((octets.to_vec(), 0)).1 += 1;
            let id = Uuid::new_v4();
            self.lignes.lock().unwrap().insert(
                id,
                Ligne {
                    blob: hash,
                    mime: mime.to_string(),
                    etat: EtatCdr::Inconnu,
                    rapport: None,
                },
            );
            id
        }

        fn ligne(&self, id: Uuid) -> Ligne {
            self.lignes.lock().unwrap()[&id].clone()
        }

        fn refs(&self, hash: &str) -> i64 {
            self.blobs.lock().unwrap().get(hash).map_or(0, |b| b.1)
        }

        fn octets(&self, hash: &str) -> Vec<u8> {
            self.blobs.lock().unwrap()[hash].0.clone()
        }
    }

    impl DepotReclassementCdr for DepotMemoire {
        async fn lister_inconnus(
            &self,
            apres: Option<Uuid>,
            limite: i64,
        ) -> Result<Vec<FichierInconnu>, DomainError> {
            if self
                .paniques
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
                > 0
            {
                panic!("panique simulée du dépôt");
            }
            let lignes = self.lignes.lock().unwrap();
            let mut v: Vec<FichierInconnu> = lignes
                .iter()
                .filter(|(id, l)| l.etat == EtatCdr::Inconnu && apres.is_none_or(|a| **id > a))
                .map(|(id, l)| FichierInconnu {
                    id: *id,
                    blob_hash: l.blob.clone(),
                    mime_type: l.mime.clone(),
                })
                .collect();
            v.sort_by_key(|f| f.id);
            v.truncate(limite as usize);
            Ok(v)
        }

        async fn extraire_blob(&self, hash: &str, vers: &Path) -> Result<(), DomainError> {
            let octets = self
                .blobs
                .lock()
                .unwrap()
                .get(hash)
                .map(|b| b.0.clone())
                .ok_or_else(|| DomainError::not_found("Blob", hash))?;
            std::fs::write(vers, octets).unwrap();
            Ok(())
        }

        async fn remplacer_contenu(
            &self,
            id: Uuid,
            ancien_hash: &str,
            depuis: &Path,
            contenu: &ContenuReclasse,
        ) -> Result<Option<String>, DomainError> {
            let octets = std::fs::read(depuis).unwrap();
            let hash = blake3::hash(&octets).to_hex().to_string();
            if let Some(h) = &contenu.hachage {
                assert_eq!(*h, hash, "hachage fourni = hachage des octets rangés");
            }
            let mut lignes = self.lignes.lock().unwrap();
            if self.concurrence.lock().unwrap().take() == Some(id)
                && let Some(l) = lignes.get_mut(&id)
            {
                // Écriture utilisateur concurrente (déjà passée par le CDR).
                l.blob = "autre".to_string();
                l.etat = EtatCdr::Desarme;
            }
            match lignes.get_mut(&id) {
                Some(l) if l.etat == EtatCdr::Inconnu && l.blob == ancien_hash => {
                    // Transaction unique : ligne + référence du nouveau blob
                    // + retrait de celle de l'ancien.
                    let mut blobs = self.blobs.lock().unwrap();
                    blobs.entry(hash.clone()).or_insert((octets, 0)).1 += 1;
                    if let Some(b) = blobs.get_mut(ancien_hash) {
                        b.1 -= 1;
                    }
                    l.blob = hash.clone();
                    l.mime = contenu.mime_type.clone();
                    Ok(Some(hash))
                }
                _ => Ok(None),
            }
        }

        async fn compter_inconnus(&self) -> Result<u64, DomainError> {
            Ok(self
                .lignes
                .lock()
                .unwrap()
                .values()
                .filter(|l| l.etat == EtatCdr::Inconnu)
                .count() as u64)
        }

        async fn invalider_derives(&self, id: Uuid) {
            self.invalides.lock().unwrap().push(id);
        }

        async fn marquer(
            &self,
            id: Uuid,
            hash_attendu: &str,
            marque: &MarqueCdr,
        ) -> Result<bool, DomainError> {
            let mut lignes = self.lignes.lock().unwrap();
            match lignes.get_mut(&id) {
                Some(l) if l.etat == EtatCdr::Inconnu && l.blob == hash_attendu => {
                    l.etat = marque.etat;
                    l.rapport = marque.rapport.clone();
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }

    fn service(
        depot: &Arc<DepotMemoire>,
    ) -> (ReclassementCdrService<DepotMemoire>, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let svc = ReclassementCdrService::new(
            depot.clone(),
            Arc::new(DesarmementService::default()),
            dir.path().join("travail"),
        );
        (svc, dir)
    }

    #[tokio::test]
    async fn image_desarmable_reclassee_desarmee_avec_contenu_reconstruit() {
        let depot = Arc::new(DepotMemoire::default());
        let png = png_avec_metadonnees();
        let id = depot.ajouter(&png, "image/png");
        let ancien = depot.ligne(id).blob;
        let (svc, _dir) = service(&depot);

        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.examines, 1);
        assert_eq!(bilan.desarmes, 1);

        let l = depot.ligne(id);
        assert_eq!(l.etat, EtatCdr::Desarme);
        assert_ne!(l.blob, ancien, "octets reconstruits substitués");
        let reconstruit = depot.octets(&l.blob);
        assert!(
            !reconstruit
                .windows(b"SECRET-METADONNEE".len())
                .any(|w| w == b"SECRET-METADONNEE"),
            "métadonnée retirée par le CDR"
        );
        assert_eq!(depot.refs(&ancien), 0, "référence de l'ancien blob retirée");
        assert_eq!(depot.refs(&l.blob), 1);
        assert_eq!(*depot.invalides.lock().unwrap(), vec![id]);
        assert_eq!(l.rapport.unwrap()["reclassement"], true);
    }

    #[tokio::test]
    async fn type_non_pris_en_charge_mis_en_quarantaine_sans_toucher_au_contenu() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"%PDF-1.7\n...", "application/pdf");
        let ancien = depot.ligne(id).blob;
        let (svc, _dir) = service(&depot);

        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.quarantaines, 1);
        let l = depot.ligne(id);
        assert_eq!(l.etat, EtatCdr::Quarantaine);
        assert_eq!(l.blob, ancien);
        assert_eq!(depot.refs(&ancien), 1);
    }

    /// Refus du CDR (contenu discordant : polyglotte, renommage) : un fichier
    /// hérité non désarmable va en quarantaine (décision du propriétaire).
    #[tokio::test]
    async fn refus_du_cdr_mis_en_quarantaine() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"ceci n'est pas une image", "image/png");
        let (svc, _dir) = service(&depot);

        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.quarantaines, 1);
        let l = depot.ligne(id);
        assert_eq!(l.etat, EtatCdr::Quarantaine);
        let motif = l.rapport.unwrap()["motif"].as_str().unwrap().to_string();
        assert!(motif.starts_with("non désarmable"), "{motif}");
    }

    #[tokio::test]
    async fn texte_sain_desarme_sans_nouveau_blob() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"bonjour\n", "text/plain");
        let ancien = depot.ligne(id).blob;
        let (svc, _dir) = service(&depot);

        svc.reclasser_tout().await.unwrap();
        let l = depot.ligne(id);
        assert_eq!(l.etat, EtatCdr::Desarme);
        assert_eq!(l.blob, ancien);
        assert_eq!(depot.refs(&ancien), 1);
    }

    #[tokio::test]
    async fn blob_illisible_reste_inconnu_donc_non_servi() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"x", "text/plain");
        let hash = depot.ligne(id).blob;
        depot.blobs.lock().unwrap().remove(&hash);
        let (svc, _dir) = service(&depot);

        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.erreurs, 1);
        let etat = depot.ligne(id).etat;
        assert_eq!(etat, EtatCdr::Inconnu);
        assert!(!etat.autorise_telechargement(true));
        assert!(!etat.autorise_lecture_dav());
    }

    #[tokio::test]
    async fn ecriture_concurrente_rien_n_est_ecrase_et_le_blob_libere() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(&png_avec_metadonnees(), "image/png");
        *depot.concurrence.lock().unwrap() = Some(id);
        let (svc, _dir) = service(&depot);

        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.concurrences, 1);
        let l = depot.ligne(id);
        assert_eq!(l.blob, "autre", "écriture de l'utilisateur conservée");
        // Le blob reconstruit a été libéré : plus aucune référence vivante
        // hormis l'ancien (toujours compté par le dépôt mémoire).
        let vivants: i64 = depot
            .blobs
            .lock()
            .unwrap()
            .iter()
            .filter(|(h, _)| **h != "autre")
            .map(|(_, b)| b.1)
            .sum();
        assert_eq!(vivants, 1);
        assert!(depot.invalides.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn plusieurs_lots_tous_reclasses() {
        let depot = Arc::new(DepotMemoire::default());
        let n = TAILLE_LOT_RECLASSEMENT as usize + 3;
        for i in 0..n {
            depot.ajouter(format!("ligne {i}\n").as_bytes(), "text/plain");
        }
        let (svc, _dir) = service(&depot);
        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(bilan.examines, n as u64);
        assert_eq!(bilan.desarmes, n as u64);
        assert!(
            depot
                .lignes
                .lock()
                .unwrap()
                .values()
                .all(|l| l.etat == EtatCdr::Desarme)
        );
        // Second passage : rien à faire.
        assert_eq!(
            svc.reclasser_tout().await.unwrap(),
            BilanReclassement {
                restants: Some(0),
                ..BilanReclassement::default()
            }
        );
        assert_eq!(svc.inconnus_restants(), Some(0));
    }

    #[tokio::test]
    async fn restants_comptes_apres_un_passage() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"x", "text/plain");
        let hash = depot.ligne(id).blob;
        depot.blobs.lock().unwrap().remove(&hash); // illisible : reste inconnu
        depot.ajouter(b"bonjour\n", "text/plain");
        let (svc, _dir) = service(&depot);
        assert_eq!(svc.inconnus_restants(), None, "avant tout passage");
        let bilan = svc.reclasser_tout().await.unwrap();
        assert_eq!(
            (bilan.desarmes, bilan.erreurs, bilan.restants),
            (1, 1, Some(1))
        );
        assert_eq!(svc.inconnus_restants(), Some(1));
    }

    /// La boucle de reclassement est supervisée : une panique est
    /// journalisée et la tâche relancée, le travail finit par être fait.
    #[tokio::test]
    async fn supervision_relance_apres_panique() {
        let depot = Arc::new(DepotMemoire::default());
        let id = depot.ajouter(b"bonjour\n", "text/plain");
        depot.paniques.store(2, std::sync::atomic::Ordering::SeqCst);
        let (svc, _dir) = service(&depot);
        let svc = Arc::new(svc);
        let handle = superviser_reclassement(
            svc.clone(),
            Duration::from_secs(3600),
            Duration::from_millis(10),
        );
        let debut = std::time::Instant::now();
        while depot.ligne(id).etat == EtatCdr::Inconnu {
            assert!(
                debut.elapsed() < Duration::from_secs(10),
                "la tâche n'a pas été relancée"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert_eq!(depot.ligne(id).etat, EtatCdr::Desarme);
        assert!(!handle.is_finished(), "la supervision continue");
        handle.abort();
    }
}
