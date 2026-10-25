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
//! ## Bases `nationale`
//!
//! Toute base `maia-*` (mémoire de MAIA), plus les bases listées dans
//! `<data_dir>/bases-nationales` (une par ligne, `#` : commentaire ; entrée
//! invalide ⇒ refus de démarrer). Poser y est refusé ; la lecture n'y rend
//! jamais `publiable`.
//!
//! ## Stockage : le journal chaîné lui-même
//!
//! L'état d'une étiquette n'est **pas** une table modifiable : c'est le dernier
//! événement effectif (`kind = etiquette`, verdict `posee` ou `retiree`) du
//! journal d'audit chaîné pour l'empreinte de la donnée (`params_hash`).
//! Détail d'un événement : `v1;<empreinte du contenu | ->;<utilisateur>;<kid de la clé>;<horodatage>`.
//! Identifiant d'intention : `etiquette:<SHA-256 de la signature>` (jamais deux
//! fois : anti-rejeu) ; horodatage signé strictement croissant par donnée
//! (une pose ancienne retenue ne peut pas être rejouée après un retrait).
//! Refus et lectures sont aussi journalisés (verdicts `refusee`, `autorisee`).

use std::collections::HashMap;
use std::path::Path;

use ed25519_dalek::{Signature, VerifyingKey};
use kindmother::etiquette::{
    decode_hex, encode_hex, sensibilite_par_regle, ActionEtiquette, EtatEtiquette, IntentionEtiquette,
    RefDonnee, Sensibilite, DOMAINE_CONTENU, FENETRE_HORODATAGE_SECS, FICHIER_BASES_NATIONALES,
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

fn detail_evenement(empreinte: Option<&str>, utilisateur: &str, kid: &str, horodatage: u64) -> String {
    format!("v1;{};{utilisateur};{kid};{horodatage}", empreinte.unwrap_or("-"))
}

/// `(empreinte, utilisateur, horodatage)` d'un détail d'événement.
fn analyser_detail(detail: &str) -> Option<(Option<String>, String, u64)> {
    let parts: Vec<&str> = detail.split(';').collect();
    let ["v1", empreinte, utilisateur, _kid, horodatage] = parts.as_slice() else {
        return None;
    };
    let empreinte = match *empreinte {
        "-" => None,
        e if decode_hex::<32>(e).is_some() => Some((*e).to_string()),
        _ => return None,
    };
    Some((empreinte, (*utilisateur).to_string(), horodatage.parse().ok()?))
}

fn horodatage_du_detail(detail: &str) -> Option<u64> {
    analyser_detail(detail).map(|(_, _, h)| h)
}

fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Lit `<data_dir>/bases-nationales` (absent : aucune base déclarée).
pub fn lire_bases_nationales(data_dir: &Path) -> Result<Vec<String>, ServiceError> {
    let chemin = data_dir.join(FICHIER_BASES_NATIONALES);
    let contenu = match std::fs::read_to_string(&chemin) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(ServiceError::Internal(format!("Cannot read {chemin:?}: {e}"))),
    };
    let mut bases = Vec::new();
    for ligne in contenu.lines().map(str::trim) {
        if ligne.is_empty() || ligne.starts_with('#') {
            continue;
        }
        if !kindmother::etiquette::base_valide(ligne) {
            return Err(ServiceError::Internal(format!(
                "Invalid entry in {FICHIER_BASES_NATIONALES}: {:?}",
                ligne.chars().take(60).collect::<String>()
            )));
        }
        bases.push(ligne.to_string());
    }
    Ok(bases)
}

/// Clés publiques des utilisateurs (jamais une clé de service).
pub struct ClesUtilisateurs {
    cles: HashMap<String, VerifyingKey>,
}

impl ClesUtilisateurs {
    /// Charge `dossier/<utilisateur>.pub` ; refuse toute clé de `interdites`.
    pub fn charger(dossier: &Path, interdites: &[[u8; 32]]) -> Result<Self, ServiceError> {
        let mut cles = HashMap::new();
        let entrees = match std::fs::read_dir(dossier) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Self { cles }),
            Err(e) => return Err(ServiceError::Internal(format!("Cannot read {dossier:?}: {e}"))),
        };
        for entree in entrees {
            let chemin = entree.map_err(|e| ServiceError::Internal(format!("Cannot read {dossier:?}: {e}")))?.path();
            let Some(nom) = chemin.file_name().and_then(|n| n.to_str()) else {
                return Err(ServiceError::Internal(format!("Invalid user key file name in {dossier:?}")));
            };
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
        Ok(Self { cles })
    }

    /// Nombre d'utilisateurs connus.
    pub fn len(&self) -> usize {
        self.cles.len()
    }

    /// Vrai si aucun utilisateur n'est connu (aucune étiquette possible).
    pub fn is_empty(&self) -> bool {
        self.cles.is_empty()
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

/// Étiquettes `publiable` : clés des utilisateurs, bases nationales, clé du serveur.
pub struct Etiquettes {
    cles: ClesUtilisateurs,
    nationales: Vec<String>,
    cle_serveur: [u8; 32],
}

impl Etiquettes {
    /// Assemble la couche d'étiquettes.
    pub fn new(cles: ClesUtilisateurs, nationales: Vec<String>, cle_serveur: [u8; 32]) -> Self {
        Self { cles, nationales, cle_serveur }
    }

    /// Clés des utilisateurs.
    pub fn cles(&self) -> &ClesUtilisateurs {
        &self.cles
    }

    /// Sensibilité d'une base : `maia-*` ou déclarée ⇒ `nationale`.
    pub fn sensibilite(&self, base: &str) -> Sensibilite {
        if sensibilite_par_regle(base) == Sensibilite::Nationale || self.nationales.iter().any(|b| b == base) {
            Sensibilite::Nationale
        } else {
            Sensibilite::Ordinaire
        }
    }

    /// Applique une intention signée par l'utilisateur ; tout refus est journalisé.
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
                refuser(db, operateur, &intention.donnee, intention.action.as_str(), &ref_hex, &e);
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
            detail: detail_evenement(empreinte.as_deref(), &intention.utilisateur, &kid, intention.horodatage),
        };
        let seq = db.journal().append_etiquette(&record, intention.horodatage, horodatage_du_detail)?;
        tracing::info!(
            target: "kindmother::etiquette",
            seq, operateur, base = %donnee.base, table = %donnee.table, action = intention.action.as_str(),
            utilisateur = %intention.utilisateur, "étiquette publiable : intention de l'utilisateur appliquée"
        );
        Ok(())
    }

    /// Lecture typée de l'état (journalisée ; non journalisée = refusée).
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
        let (verdict, detail) = match &resultat {
            Ok(etat) => ("autorisee".to_string(), format!("publiable={}", etat.publiable)),
            Err(e) => ("refusee".to_string(), e.to_string()),
        };
        db.journal().append(&AuditRecord {
            kind: AuditKind::Etiquette,
            intent_id: None,
            operator_id: operateur.to_string(),
            database: donnee.base.clone(),
            operation: "publiable.lire".to_string(),
            table: Some(donnee.table.clone()),
            verdict,
            decision_digest: None,
            params_hash: ref_hex,
            detail,
        })?;
        resultat
    }

    async fn etat(&self, db: &EncryptedDatabase, donnee: &RefDonnee, ref_hex: &str) -> Result<EtatEtiquette, ServiceError> {
        let sensibilite = self.sensibilite(&donnee.base);
        let dernier = db.journal().dernier_evenement_etiquette(ref_hex)?;
        let evenement = match &dernier {
            Some((seq, verdict, detail)) => {
                let (empreinte, utilisateur, horodatage) = analyser_detail(detail)
                    .ok_or_else(|| ServiceError::Internal("audit journal corrupt: label event".to_string()))?;
                Some((*seq, verdict == VERDICT_ETIQUETTE_POSEE, empreinte, utilisateur, horodatage))
            }
            None => None,
        };
        let posee = evenement.as_ref().is_some_and(|e| e.1);
        let empreinte_actuelle = contenu_actuel(db, donnee).await?;
        let empreinte_contenu = evenement.as_ref().filter(|e| e.1).and_then(|e| e.2.clone());
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
            utilisateur: evenement.as_ref().filter(|e| e.1).map(|e| e.3.clone()),
            horodatage: evenement.as_ref().map(|e| e.4),
            seq: evenement.as_ref().filter(|e| e.1).map(|e| e.0),
        })
    }
}

/// Journalise un refus (échec de journalisation : signalé, le refus reste un refus).
fn refuser(db: &EncryptedDatabase, operateur: &str, donnee: &RefDonnee, operation: &str, ref_hex: &str, e: &ServiceError) {
    let record = AuditRecord {
        kind: AuditKind::Etiquette,
        intent_id: None,
        operator_id: operateur.to_string(),
        database: donnee.base.clone(),
        operation: operation.to_string(),
        table: Some(donnee.table.clone()),
        verdict: "refusee".to_string(),
        decision_digest: None,
        params_hash: ref_hex.to_string(),
        detail: e.to_string(),
    };
    if let Err(err) = db.journal().append(&record) {
        tracing::error!("AUDIT FAILURE while recording a label refusal: {err}");
    }
    tracing::warn!(target: "kindmother::etiquette", operateur, base = %donnee.base, motif = %e, "étiquette publiable refusée");
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

    #[test]
    fn vecteur_empreinte_contenu() {
        // Vecteur partagé avec `maia-federee::etiquettes` (même domaine, même règle).
        assert_eq!(
            encode_hex(&empreinte_contenu("Paris est la capitale de la France.")),
            VECTEUR_PARIS
        );
    }

    pub(crate) const VECTEUR_PARIS: &str = "fc13b5df90b60dd6b4fcc79af479c7c6e6eb0b5f791fd19845d523897f865284";

    #[test]
    fn detail_aller_retour() {
        let e = "ab".repeat(32);
        let d = detail_evenement(Some(&e), "alice", "kid", 42);
        assert_eq!(analyser_detail(&d), Some((Some(e), "alice".into(), 42)));
        assert_eq!(analyser_detail(&detail_evenement(None, "alice", "kid", 7)), Some((None, "alice".into(), 7)));
        assert_eq!(analyser_detail("v2;x"), None);
    }
}
