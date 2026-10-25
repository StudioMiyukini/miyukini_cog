//! @id     kindmother_service_etiquettes
//! @do     poser_lire_etiquette_publiable
//! @role   securite
//! @layer  core
//! @human  kindmother-service — étiquette `publiable` (EX-47) : posée / retirée seulement par une intention
//! @human  signée par une clé de l'utilisateur, refusée sur toute donnée `nationale`, tracée dans le
//! @human  journal d'audit chaîné ; lecture typée de l'état (contenu courant comparé au contenu étiqueté).

//! Étiquettes `publiable` côté service (EX-47, C-40, IA-FEDEREE §5).
//!
//! ## Clés de l'utilisateur
//!
//! `<credentials_dir>/utilisateurs/<utilisateur>.pub` : clé publique Ed25519
//! (hex) d'un utilisateur, **déposée par l'administrateur** (le service n'en
//! crée jamais : la clé privée reste sur l'appareil de l'utilisateur). Dossier
//! absent ⇒ aucune étiquette ne peut être posée (fail-closed). Fichier mal
//! formé, nom invalide, ou clé égale à une clé d'opérateur ou du serveur ⇒
//! **refus de démarrer** (la clé de service de MAIA ne peut pas servir de clé
//! d'utilisateur).
//!
//! **Rechargement (B6)** : clés et fichiers `.bases` sont lus **au démarrage
//! seulement**. Ajouter, retirer ou remplacer une clé d'utilisateur (ou
//! modifier ses bases) exige un **redémarrage** du service ; tant qu'il n'a pas
//! eu lieu, une clé retirée reste acceptée. Révoquer d'urgence une clé =
//! retirer le fichier puis redémarrer.
//!
//! ## Bases de l'utilisateur (M5, format à consigner par amendement)
//!
//! `<credentials_dir>/utilisateurs/<utilisateur>.bases` : bases dont
//! l'utilisateur est **propriétaire**, une par ligne (`[a-z0-9_-]{1,64}`, ni
//! `_` ni `-` initial ; ligne vide ou `#` : ignorée). Validé au démarrage :
//! entrée invalide, ou fichier `.bases` sans `.pub` correspondant ⇒ refus de
//! démarrer. Fichier absent ⇒ aucune base (l'utilisateur ne peut rien
//! étiqueter). Poser **ou retirer** une étiquette sur une base hors de cette
//! liste ⇒ refus, même avec une signature valide.
//!
//! ## Bases `nationale`
//!
//! Toute base `maia-*` (mémoire de MAIA), plus les bases listées dans
//! `<data_dir>/bases-nationales` (une par ligne, `#` : commentaire ; entrée
//! invalide ⇒ refus de démarrer). Poser y est refusé ; la lecture n'y rend
//! jamais `publiable`. Comparaisons insensibles à la casse (M4).
//!
//! ## Stockage : le journal chaîné lui-même
//!
//! L'état d'une étiquette n'est **pas** une table modifiable : c'est le dernier
//! événement effectif (`kind = etiquette`, verdict `posee` ou `retiree`) du
//! journal d'audit chaîné pour l'empreinte de la donnée (`params_hash`).
//! Détail d'un événement (format v2) :
//! `v2;<empreinte du contenu | ->;<utilisateur>;<kid de la clé>;<horodatage>;<paramètres | ->`
//! où `<paramètres>` = [`ParametresPublication::texte`] (pose v2). Les
//! événements `v1;…` antérieurs restent lisibles, **sans paramètres** : une
//! pose v1 n'est jamais publiable par `maia-federee`.
//! Identifiant d'intention : `etiquette:<SHA-256 de la signature>` (jamais deux
//! fois : anti-rejeu) ; horodatage signé strictement croissant par donnée
//! (une pose ancienne retenue ne peut pas être rejouée après un retrait).
//!
//! ## Journalisation des lectures et refus, débit (M6)
//!
//! Une lecture **autorisée** n'est pas journalisée (elle ne change rien et
//! inonderait le journal chaîné). Les **refus** sont agrégés par opérateur :
//! le premier refus d'une période de [`PERIODE_REFUS_SECS`] secondes est écrit
//! aussitôt (`refus=<n>;depuis=<t>;dernier=<motif>`), les suivants sont
//! comptés et écrits dans l'entrée suivante (refus après la période, ou juste
//! après la pose / le retrait effectif suivant de cet opérateur). Débit : au plus
//! [`DEBIT_MAX_REQUETES`] requêtes d'étiquette par opérateur et par fenêtre de
//! [`FENETRE_DEBIT_SECS`] secondes ; au-delà, refus `QuotaExceeded`.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::sync::{Mutex, PoisonError};

use ed25519_dalek::{Signature, VerifyingKey};
use kindmother::etiquette::{
    base_valide, decode_hex, encode_hex, sensibilite_par_regle, ActionEtiquette, EtatEtiquette, IntentionEtiquette,
    ParametresPublication, RefDonnee, Sensibilite, DOMAINE_CONTENU, FENETRE_HORODATAGE_SECS, FICHIER_BASES_NATIONALES,
};
use rusqlite::types::Value;
use sha2::{Digest, Sha256};

use crate::audit::{sha256_hex, AuditKind, AuditRecord, VERDICT_ETIQUETTE_POSEE, VERDICT_ETIQUETTE_RETIREE};
use crate::database::EncryptedDatabase;
use crate::errors::ServiceError;
use crate::sql_guard::{map_prepare_error, AuthorizerGuard, SqlPolicy};
use crate::typed::{table_schema, to_value, ObjectKind};

/// Sous-dossier des clés publiques des utilisateurs dans `<credentials_dir>`.
pub const DOSSIER_CLES_UTILISATEURS: &str = "utilisateurs";

/// Suffixe du fichier des bases dont un utilisateur est propriétaire (M5).
pub const SUFFIXE_BASES: &str = ".bases";

/// Requêtes d'étiquette (poser, retirer, lire) au plus par opérateur et par fenêtre (M6).
pub const DEBIT_MAX_REQUETES: u32 = 120;
/// Fenêtre du débit (secondes).
pub const FENETRE_DEBIT_SECS: u64 = 60;
/// Période d'agrégation des refus par opérateur (secondes).
pub const PERIODE_REFUS_SECS: u64 = 60;

/// Séparation de domaine de l'identité d'une donnée (empreinte de [`RefDonnee`]).
const DOMAINE_DONNEE: &[u8] = b"miyukini/kindmother/etiquette/donnee/v1\0";

/// Empreinte du contenu publiable : `SHA-256(DOMAINE_CONTENU || texte UTF-8)`.
pub fn empreinte_contenu(texte: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOMAINE_CONTENU);
    h.update(texte.as_bytes());
    h.finalize().into()
}

/// Identité d'une donnée dans le journal (hex).
pub fn empreinte_donnee(donnee: &RefDonnee) -> String {
    let mut m = DOMAINE_DONNEE.to_vec();
    m.extend_from_slice(&donnee.canonical_bytes());
    sha256_hex(&m)
}

fn detail_evenement(
    empreinte: Option<&str>,
    utilisateur: &str,
    kid: &str,
    horodatage: u64,
    parametres: Option<&ParametresPublication>,
) -> String {
    format!(
        "v2;{};{utilisateur};{kid};{horodatage};{}",
        empreinte.unwrap_or("-"),
        parametres.map_or_else(|| "-".to_string(), ParametresPublication::texte)
    )
}

/// Événement d'étiquette lu du journal.
#[derive(Debug, PartialEq)]
struct Evenement {
    empreinte: Option<String>,
    utilisateur: String,
    horodatage: u64,
    parametres: Option<ParametresPublication>,
}

/// Détail d'un événement (v2 ; v1 accepté sans paramètres).
fn analyser_detail(detail: &str) -> Option<Evenement> {
    let parts: Vec<&str> = detail.split(';').collect();
    let (empreinte, utilisateur, horodatage, parametres) = match parts.as_slice() {
        ["v1", e, u, _kid, h] => (*e, *u, *h, None),
        ["v2", e, u, _kid, h, "-"] => (*e, *u, *h, None),
        ["v2", e, u, _kid, h, p] => (*e, *u, *h, Some(ParametresPublication::depuis_texte(p)?)),
        _ => return None,
    };
    let empreinte = match empreinte {
        "-" => None,
        e if decode_hex::<32>(e).is_some() => Some(e.to_string()),
        _ => return None,
    };
    Some(Evenement { empreinte, utilisateur: utilisateur.to_string(), horodatage: horodatage.parse().ok()?, parametres })
}

fn horodatage_du_detail(detail: &str) -> Option<u64> {
    analyser_detail(detail).map(|e| e.horodatage)
}

fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Lignes de noms de base (`#` : commentaire) ; entrée invalide ⇒ erreur.
fn lire_liste_bases(contenu: &str, origine: &str) -> Result<Vec<String>, ServiceError> {
    let mut bases = Vec::new();
    for ligne in contenu.lines().map(str::trim) {
        if ligne.is_empty() || ligne.starts_with('#') {
            continue;
        }
        if !base_valide(ligne) {
            return Err(ServiceError::Internal(format!(
                "Invalid entry in {origine}: {:?}",
                ligne.chars().take(60).collect::<String>()
            )));
        }
        bases.push(ligne.to_string());
    }
    Ok(bases)
}

/// Lit `<data_dir>/bases-nationales` (absent : aucune base déclarée).
pub fn lire_bases_nationales(data_dir: &Path) -> Result<Vec<String>, ServiceError> {
    let chemin = data_dir.join(FICHIER_BASES_NATIONALES);
    match std::fs::read_to_string(&chemin) {
        Ok(c) => lire_liste_bases(&c, FICHIER_BASES_NATIONALES),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(ServiceError::Internal(format!("Cannot read {chemin:?}: {e}"))),
    }
}

/// Clés publiques des utilisateurs (jamais une clé de service) et bases dont
/// chacun est propriétaire. Chargées au démarrage seulement (B6).
pub struct ClesUtilisateurs {
    cles: HashMap<String, VerifyingKey>,
    bases: HashMap<String, BTreeSet<String>>,
}

impl ClesUtilisateurs {
    /// Charge `dossier/<utilisateur>.pub` et `dossier/<utilisateur>.bases` ;
    /// refuse toute clé de `interdites`.
    pub fn charger(dossier: &Path, interdites: &[[u8; 32]]) -> Result<Self, ServiceError> {
        let mut cles = HashMap::new();
        let mut bases = HashMap::new();
        let entrees = match std::fs::read_dir(dossier) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self { cles, bases }),
            Err(e) => return Err(ServiceError::Internal(format!("Cannot read {dossier:?}: {e}"))),
        };
        for entree in entrees {
            let chemin = entree.map_err(|e| ServiceError::Internal(format!("Cannot read {dossier:?}: {e}")))?.path();
            let Some(nom) = chemin.file_name().and_then(|n| n.to_str()) else {
                return Err(ServiceError::Internal(format!("Invalid user key file name in {dossier:?}")));
            };
            if let Some(utilisateur) = nom.strip_suffix(SUFFIXE_BASES) {
                if !kindmother::etiquette::utilisateur_valide(utilisateur) {
                    return Err(ServiceError::Internal(format!("Invalid user bases file: {nom:?}")));
                }
                let texte = std::fs::read_to_string(&chemin)
                    .map_err(|e| ServiceError::Internal(format!("Cannot read {chemin:?}: {e}")))?;
                let liste: BTreeSet<String> = lire_liste_bases(&texte, nom)?.into_iter().collect();
                bases.insert(utilisateur.to_string(), liste);
                continue;
            }
            let Some(utilisateur) = nom.strip_suffix(".pub") else {
                continue;
            };
            if !kindmother::etiquette::utilisateur_valide(utilisateur) {
                return Err(ServiceError::Internal(format!("Invalid user key file: {nom:?}")));
            }
            let texte = std::fs::read_to_string(&chemin)
                .map_err(|e| ServiceError::Internal(format!("Cannot read {chemin:?}: {e}")))?;
            let octets = decode_hex::<32>(texte.trim())
                .ok_or_else(|| ServiceError::Internal(format!("Malformed user key: {nom:?}")))?;
            if interdites.contains(&octets) {
                return Err(ServiceError::Internal(format!(
                    "User key {nom:?} is also an operator or server key: refused (EX-47)"
                )));
            }
            let cle = VerifyingKey::from_bytes(&octets)
                .map_err(|_| ServiceError::Internal(format!("Invalid user key: {nom:?}")))?;
            cles.insert(utilisateur.to_string(), cle);
        }
        if let Some(orphelin) = bases.keys().find(|u| !cles.contains_key(*u)) {
            return Err(ServiceError::Internal(format!(
                "User bases file {orphelin:?}{SUFFIXE_BASES} without a matching {orphelin:?}.pub"
            )));
        }
        Ok(Self { cles, bases })
    }

    /// Nombre d'utilisateurs connus.
    pub fn len(&self) -> usize {
        self.cles.len()
    }

    /// Vrai si aucun utilisateur n'est connu (aucune étiquette possible).
    pub fn is_empty(&self) -> bool {
        self.cles.is_empty()
    }

    /// Vrai si `utilisateur` est propriétaire de `base` (fichier `.bases`).
    pub fn proprietaire(&self, utilisateur: &str, base: &str) -> bool {
        self.bases.get(utilisateur).is_some_and(|b| b.contains(base))
    }

    /// Vérifie la signature de l'utilisateur ; retourne le `kid` (SHA-256 hex de la clé).
    fn verifier(&self, intention: &IntentionEtiquette, cle_serveur: &[u8; 32]) -> Result<String, ServiceError> {
        let refus = || ServiceError::PermissionDenied("label intent not signed by the user".to_string());
        let cle = self.cles.get(&intention.utilisateur).ok_or_else(refus)?;
        if decode_hex::<32>(&intention.cle_publique) != Some(cle.to_bytes()) {
            return Err(refus());
        }
        let signature = decode_hex::<64>(&intention.signature).ok_or_else(refus)?;
        let message = intention.message_signe(cle_serveur).map_err(ServiceError::Validation)?;
        cle.verify_strict(&message, &Signature::from_bytes(&signature)).map_err(|_| refus())?;
        Ok(sha256_hex(cle.as_bytes()))
    }
}

/// État de garde d'un opérateur : débit et refus en attente d'écriture.
#[derive(Debug, Default)]
struct EtatOperateur {
    debut_fenetre: u64,
    requetes: u32,
    refus_en_attente: u64,
    premier_en_attente: u64,
    dernier_journalise: Option<u64>,
}

/// Refus agrégés à écrire au journal.
#[derive(Debug, PartialEq, Eq)]
struct RefusAgreges {
    nombre: u64,
    depuis: u64,
}

/// Débit et agrégation des refus par opérateur (M6). Les opérateurs sont
/// authentifiés : leur nombre est borné par le magasin de justificatifs.
#[derive(Debug, Default)]
struct Garde {
    etats: Mutex<HashMap<String, EtatOperateur>>,
}

impl Garde {
    /// Compte une requête ; `false` au-delà du débit.
    fn admettre(&self, operateur: &str, t: u64) -> bool {
        let mut etats = self.etats.lock().unwrap_or_else(PoisonError::into_inner);
        let e = etats.entry(operateur.to_string()).or_default();
        if t < e.debut_fenetre || t >= e.debut_fenetre.saturating_add(FENETRE_DEBIT_SECS) {
            e.debut_fenetre = t;
            e.requetes = 0;
        }
        e.requetes = e.requetes.saturating_add(1);
        e.requetes <= DEBIT_MAX_REQUETES
    }

    /// Compte un refus ; `Some` s'il faut écrire une entrée maintenant.
    fn noter_refus(&self, operateur: &str, t: u64) -> Option<RefusAgreges> {
        let mut etats = self.etats.lock().unwrap_or_else(PoisonError::into_inner);
        let e = etats.entry(operateur.to_string()).or_default();
        if e.refus_en_attente == 0 {
            e.premier_en_attente = t;
        }
        e.refus_en_attente = e.refus_en_attente.saturating_add(1);
        let echu = e
            .dernier_journalise
            .is_none_or(|d| t < d || t >= d.saturating_add(PERIODE_REFUS_SECS));
        echu.then(|| Self::prendre(e, t))
    }

    /// Refus en attente (écrits avant un événement effectif de l'opérateur).
    fn vider(&self, operateur: &str, t: u64) -> Option<RefusAgreges> {
        let mut etats = self.etats.lock().unwrap_or_else(PoisonError::into_inner);
        let e = etats.get_mut(operateur)?;
        (e.refus_en_attente > 0).then(|| Self::prendre(e, t))
    }

    fn prendre(e: &mut EtatOperateur, t: u64) -> RefusAgreges {
        let r = RefusAgreges { nombre: e.refus_en_attente, depuis: e.premier_en_attente };
        e.refus_en_attente = 0;
        e.dernier_journalise = Some(t);
        r
    }
}

/// Étiquettes `publiable` : clés des utilisateurs, bases nationales, clé du serveur.
pub struct Etiquettes {
    cles: ClesUtilisateurs,
    nationales: Vec<String>,
    cle_serveur: [u8; 32],
    garde: Garde,
}

impl Etiquettes {
    /// Assemble la couche d'étiquettes.
    pub fn new(cles: ClesUtilisateurs, nationales: Vec<String>, cle_serveur: [u8; 32]) -> Self {
        Self { cles, nationales, cle_serveur, garde: Garde::default() }
    }

    /// Clés des utilisateurs.
    pub fn cles(&self) -> &ClesUtilisateurs {
        &self.cles
    }

    /// Sensibilité d'une base : `maia-*` ou déclarée ⇒ `nationale` (insensible à la casse).
    pub fn sensibilite(&self, base: &str) -> Sensibilite {
        if sensibilite_par_regle(base) == Sensibilite::Nationale
            || self.nationales.iter().any(|b| b.eq_ignore_ascii_case(base))
        {
            Sensibilite::Nationale
        } else {
            Sensibilite::Ordinaire
        }
    }

    /// Débit des requêtes d'étiquette de `operateur` (M6) ; refus agrégé.
    pub fn admettre(&self, db: &EncryptedDatabase, operateur: &str, donnee: &RefDonnee, operation: &str) -> Result<(), ServiceError> {
        if self.garde.admettre(operateur, maintenant()) {
            return Ok(());
        }
        let e = ServiceError::QuotaExceeded("label request rate exceeded for this operator".to_string());
        self.refuser(db, operateur, donnee, operation, &e);
        Err(e)
    }

    /// Note un refus de `operateur` ; agrégé (voir la documentation du module).
    pub fn refuser(&self, db: &EncryptedDatabase, operateur: &str, donnee: &RefDonnee, operation: &str, e: &ServiceError) {
        if let Some(r) = self.garde.noter_refus(operateur, maintenant()) {
            ecrire_refus(db, operateur, donnee, operation, &r, &e.to_string());
        }
    }

    /// Applique une intention signée par l'utilisateur ; tout refus est noté (agrégé).
    pub async fn etiqueter(
        &self,
        db: &EncryptedDatabase,
        operateur: &str,
        intention: &IntentionEtiquette,
    ) -> Result<EtatEtiquette, ServiceError> {
        let ref_hex = empreinte_donnee(&intention.donnee);
        match self.etiqueter_inner(db, operateur, intention, &ref_hex).await {
            Ok(()) => self.etat(db, &intention.donnee, &ref_hex).await,
            Err(e) => {
                self.refuser(db, operateur, &intention.donnee, intention.action.as_str(), &e);
                Err(e)
            }
        }
    }

    async fn etiqueter_inner(
        &self,
        db: &EncryptedDatabase,
        operateur: &str,
        intention: &IntentionEtiquette,
        ref_hex: &str,
    ) -> Result<(), ServiceError> {
        intention.check_shape().map_err(ServiceError::Validation)?;
        let donnee = &intention.donnee;
        if intention.action == ActionEtiquette::Poser && self.sensibilite(&donnee.base) == Sensibilite::Nationale {
            return Err(ServiceError::PermissionDenied(
                "nationale data: the publiable label is refused (EX-47)".to_string(),
            ));
        }
        let kid = self.cles.verifier(intention, &self.cle_serveur)?;
        if !self.cles.proprietaire(&intention.utilisateur, &donnee.base) {
            return Err(ServiceError::PermissionDenied(
                "database not owned by the user (bases file): label refused".to_string(),
            ));
        }
        if maintenant().abs_diff(intention.horodatage) > FENETRE_HORODATAGE_SECS {
            return Err(ServiceError::PermissionDenied("label intent timestamp outside the window".to_string()));
        }
        let verdict = match intention.action {
            ActionEtiquette::Poser => {
                let actuelle = contenu_actuel(db, donnee).await?;
                if actuelle.is_none() || actuelle != intention.empreinte_contenu.as_ref().map(|e| e.to_ascii_lowercase()) {
                    return Err(ServiceError::PermissionDenied(
                        "content digest does not match the current value of the data".to_string(),
                    ));
                }
                VERDICT_ETIQUETTE_POSEE
            }
            ActionEtiquette::Retirer => VERDICT_ETIQUETTE_RETIREE,
        };
        let signature = decode_hex::<64>(&intention.signature).unwrap_or([0; 64]);
        let empreinte = intention.empreinte_contenu.as_ref().map(|e| e.to_ascii_lowercase());
        let record = AuditRecord {
            kind: AuditKind::Etiquette,
            intent_id: Some(format!("etiquette:{}", sha256_hex(&signature))),
            operator_id: operateur.to_string(),
            database: donnee.base.clone(),
            operation: intention.action.as_str().to_string(),
            table: Some(donnee.table.clone()),
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash: ref_hex.to_string(),
            detail: detail_evenement(
                empreinte.as_deref(),
                &intention.utilisateur,
                &kid,
                intention.horodatage,
                intention.parametres.as_ref(),
            ),
        };
        let seq = db.journal().append_etiquette(&record, intention.horodatage, horodatage_du_detail)?;
        // Refus en attente de cet opérateur : écrits juste après l'événement effectif.
        if let Some(r) = self.garde.vider(operateur, maintenant()) {
            ecrire_refus(db, operateur, donnee, intention.action.as_str(), &r, "refus agrégés");
        }
        tracing::info!(
            target: "kindmother::etiquette",
            seq, operateur, base = %donnee.base, table = %donnee.table, action = intention.action.as_str(),
            utilisateur = %intention.utilisateur, "étiquette publiable : intention de l'utilisateur appliquée"
        );
        Ok(())
    }

    /// Lecture typée de l'état. Lecture autorisée : non journalisée ; refus : agrégé (M6).
    pub async fn lire(
        &self,
        db: &EncryptedDatabase,
        operateur: &str,
        donnee: &RefDonnee,
    ) -> Result<EtatEtiquette, ServiceError> {
        let ref_hex = empreinte_donnee(donnee);
        let resultat = match donnee.check_shape() {
            Ok(()) => self.etat(db, donnee, &ref_hex).await,
            Err(e) => Err(ServiceError::Validation(e)),
        };
        if let Err(e) = &resultat {
            self.refuser(db, operateur, donnee, "publiable.lire", e);
        }
        resultat
    }

    async fn etat(&self, db: &EncryptedDatabase, donnee: &RefDonnee, ref_hex: &str) -> Result<EtatEtiquette, ServiceError> {
        let sensibilite = self.sensibilite(&donnee.base);
        let dernier = db.journal().dernier_evenement_etiquette(ref_hex)?;
        let evenement = match &dernier {
            Some((seq, verdict, detail)) => {
                let e = analyser_detail(detail)
                    .ok_or_else(|| ServiceError::Internal("audit journal corrupt: label event".to_string()))?;
                Some((*seq, verdict == VERDICT_ETIQUETTE_POSEE, e))
            }
            None => None,
        };
        let pose = evenement.as_ref().filter(|e| e.1);
        let posee = pose.is_some();
        let empreinte_actuelle = contenu_actuel(db, donnee).await?;
        let empreinte_contenu = pose.and_then(|e| e.2.empreinte.clone());
        let publiable = posee
            && sensibilite == Sensibilite::Ordinaire
            && empreinte_contenu.is_some()
            && empreinte_actuelle == empreinte_contenu;
        Ok(EtatEtiquette {
            sensibilite,
            posee,
            publiable,
            empreinte_contenu,
            empreinte_actuelle,
            utilisateur: pose.map(|e| e.2.utilisateur.clone()),
            horodatage: evenement.as_ref().map(|e| e.2.horodatage),
            seq: pose.map(|e| e.0),
            parametres: pose.and_then(|e| e.2.parametres.clone()),
            empreinte_donnee: Some(ref_hex.to_string()),
        })
    }
}

/// Écrit une entrée de refus agrégés (échec de journalisation : signalé, le refus reste un refus).
fn ecrire_refus(db: &EncryptedDatabase, operateur: &str, donnee: &RefDonnee, operation: &str, r: &RefusAgreges, motif: &str) {
    let record = AuditRecord {
        kind: AuditKind::Etiquette,
        intent_id: None,
        operator_id: operateur.to_string(),
        database: donnee.base.clone(),
        operation: operation.to_string(),
        table: Some(donnee.table.clone()),
        verdict: "refusee".to_string(),
        decision_digest: None,
        params_hash: empreinte_donnee(donnee),
        detail: format!("refus={};depuis={};dernier={motif}", r.nombre, r.depuis),
    };
    if let Err(err) = db.journal().append(&record) {
        tracing::error!("AUDIT FAILURE while recording label refusals: {err}");
    }
    tracing::warn!(
        target: "kindmother::etiquette",
        operateur, base = %donnee.base, refus = r.nombre, depuis = r.depuis, motif = %motif,
        "étiquette publiable : refus (agrégés)"
    );
}

/// Empreinte (hex) du contenu courant de la donnée : `None` si la base n'existe
/// pas, si la clé ne désigne pas exactement une ligne, ou si la valeur n'est pas
/// du texte.
async fn contenu_actuel(db: &EncryptedDatabase, donnee: &RefDonnee) -> Result<Option<String>, ServiceError> {
    donnee.check_shape().map_err(ServiceError::Validation)?;
    if !db.database_path(&donnee.base).exists() {
        return Ok(None);
    }
    let conn = db.get_ro_connection(&donnee.base).await?;
    let conn = conn.lock().await;
    let Some(schema) = table_schema(&conn, &donnee.table)? else {
        return Ok(None);
    };
    if schema.kind != ObjectKind::Table {
        return Err(ServiceError::Validation(format!("not a table: {}", donnee.table)));
    }
    for colonne in donnee.cle.keys().chain(std::iter::once(&donnee.colonne)) {
        if !schema.has_column(colonne) {
            return Err(ServiceError::Validation(format!("unknown column {}.{colonne}", donnee.table)));
        }
    }
    // Identifiants validés (`[A-Za-z_][A-Za-z0-9_]*`) ; valeurs liées.
    let mut params: Vec<Value> = Vec::with_capacity(donnee.cle.len());
    let filtre: Vec<String> = donnee
        .cle
        .iter()
        .map(|(c, v)| {
            params.push(to_value(v));
            format!("\"{c}\" = ?{}", params.len())
        })
        .collect();
    let sql = format!(
        "SELECT \"{}\" FROM \"{}\" WHERE {} LIMIT 2",
        donnee.colonne,
        donnee.table,
        filtre.join(" AND ")
    );
    let _guard = AuthorizerGuard::install(&conn, SqlPolicy::READ_ONLY);
    let mut stmt = conn.prepare(&sql).map_err(|e| map_prepare_error(&e))?;
    let valeurs: Vec<Value> = stmt
        .query_map(rusqlite::params_from_iter(params), |r| r.get::<_, Value>(0))
        .map_err(|e| ServiceError::Database(format!("Label content read failed: {e}")))?
        .collect::<Result<_, _>>()
        .map_err(|e| ServiceError::Database(format!("Label content read failed: {e}")))?;
    Ok(match valeurs.as_slice() {
        [Value::Text(t)] => Some(encode_hex(&empreinte_contenu(t))),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use kindmother::etiquette::{AudiencePublication, TypePublication};

    #[test]
    fn vecteur_empreinte_contenu() {
        // Vecteur partagé avec `maia-federee::etiquettes` (même domaine, même règle).
        assert_eq!(
            encode_hex(&empreinte_contenu("Paris est la capitale de la France.")),
            VECTEUR_PARIS
        );
    }

    pub(crate) const VECTEUR_PARIS: &str = "fc13b5df90b60dd6b4fcc79af479c7c6e6eb0b5f791fd19845d523897f865284";

    fn parametres() -> ParametresPublication {
        ParametresPublication {
            type_fiche: TypePublication::Fait,
            langue: "fr".into(),
            licence: "CC-BY-4.0".into(),
            audience: AudiencePublication::Membres,
        }
    }

    #[test]
    fn detail_aller_retour() {
        let e = "ab".repeat(32);
        let d = detail_evenement(Some(&e), "alice", "kid", 42, Some(&parametres()));
        assert!(d.starts_with("v2;"));
        assert_eq!(
            analyser_detail(&d),
            Some(Evenement { empreinte: Some(e.clone()), utilisateur: "alice".into(), horodatage: 42, parametres: Some(parametres()) })
        );
        let r = analyser_detail(&detail_evenement(None, "alice", "kid", 7, None));
        assert_eq!(r, Some(Evenement { empreinte: None, utilisateur: "alice".into(), horodatage: 7, parametres: None }));
        // Ancien format v1 : lu, sans paramètres (jamais publiable par maia-federee).
        let v1 = analyser_detail(&format!("v1;{e};alice;kid;9")).expect("v1");
        assert_eq!((v1.horodatage, v1.parametres), (9, None));
        assert_eq!(analyser_detail("v2;x"), None);
        assert_eq!(analyser_detail(&format!("v2;{e};alice;kid;9;n:pas-un-cog")), None);
    }

    /// M6 : débit borné par opérateur et par fenêtre.
    #[test]
    fn debit_par_operateur() {
        let g = Garde::default();
        for _ in 0..DEBIT_MAX_REQUETES {
            assert!(g.admettre("jaykonta", 1_000));
        }
        assert!(!g.admettre("jaykonta", 1_000 + FENETRE_DEBIT_SECS - 1));
        assert!(g.admettre("jaykoa", 1_000), "opérateurs indépendants");
        assert!(g.admettre("jaykonta", 1_000 + FENETRE_DEBIT_SECS), "fenêtre suivante");
    }

    /// M6 : refus agrégés par opérateur (premier écrit, suivants comptés).
    #[test]
    fn refus_agreges_par_operateur() {
        let g = Garde::default();
        assert_eq!(g.noter_refus("op", 100), Some(RefusAgreges { nombre: 1, depuis: 100 }));
        for t in 101..150 {
            assert_eq!(g.noter_refus("op", t), None);
        }
        assert_eq!(g.noter_refus("autre", 120), Some(RefusAgreges { nombre: 1, depuis: 120 }));
        // Période échue : une entrée porte tous les refus en attente.
        assert_eq!(g.noter_refus("op", 100 + PERIODE_REFUS_SECS), Some(RefusAgreges { nombre: 50, depuis: 101 }));
        // Vidage avant un événement effectif.
        assert_eq!(g.noter_refus("op", 170), None);
        assert_eq!(g.vider("op", 171), Some(RefusAgreges { nombre: 1, depuis: 170 }));
        assert_eq!(g.vider("op", 172), None);
    }

    /// M5 : fichier `.bases` validé au démarrage.
    #[test]
    fn fichier_bases_valide_au_demarrage() {
        let dir = std::env::temp_dir().join(format!("km-bases-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("dossier");
        let cle = ed25519_dalek::SigningKey::from_bytes(&[7; 32]);
        std::fs::write(dir.join("alice.pub"), encode_hex(cle.verifying_key().as_bytes())).expect("pub");
        std::fs::write(dir.join("alice.bases"), "# bases d'Alice\njaykonta\n\nnotes-alice\n").expect("bases");
        let c = ClesUtilisateurs::charger(&dir, &[]).expect("charger");
        assert!(c.proprietaire("alice", "jaykonta") && c.proprietaire("alice", "notes-alice"));
        assert!(!c.proprietaire("alice", "jaykoa") && !c.proprietaire("bob", "jaykonta"));
        // Entrée invalide (majuscules, M4) : refus de démarrer.
        std::fs::write(dir.join("alice.bases"), "Jaykonta\n").expect("bases");
        assert!(ClesUtilisateurs::charger(&dir, &[]).is_err());
        // `.bases` sans `.pub` : refus de démarrer.
        std::fs::write(dir.join("alice.bases"), "jaykonta\n").expect("bases");
        std::fs::write(dir.join("bob.bases"), "jaykonta\n").expect("bases");
        assert!(ClesUtilisateurs::charger(&dir, &[]).is_err());
        std::fs::remove_file(dir.join("bob.bases")).expect("rm");
        // Sans `.bases` : aucune base.
        std::fs::remove_file(dir.join("alice.bases")).expect("rm");
        assert!(!ClesUtilisateurs::charger(&dir, &[]).expect("charger").proprietaire("alice", "jaykonta"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn nationales_insensibles_a_la_casse() {
        let e = Etiquettes::new(
            ClesUtilisateurs { cles: HashMap::new(), bases: HashMap::new() },
            vec!["jaykoa".into()],
            [0; 32],
        );
        assert_eq!(e.sensibilite("MAIA-alice"), Sensibilite::Nationale);
        assert_eq!(e.sensibilite("JayKoa"), Sensibilite::Nationale);
        assert_eq!(e.sensibilite("jaykonta"), Sensibilite::Ordinaire);
    }
}
