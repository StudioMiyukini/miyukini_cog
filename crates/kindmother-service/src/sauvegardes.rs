//! @id     kindmother_service_sauvegardes
//! @do     conserver_purger_restaurer_sauvegardes
//! @role   securite
//! @layer  core
//! @human  kindmother-service — sauvegardes d'avant migration : 30 jours puis effacement sûr, restauration qui réapplique les oublis (A-022 §5).

//! Sauvegardes de KindMother (A-022 §5).
//!
//! **Où et quand** : avant d'appliquer des migrations à une base qui contient des tables,
//! et avant toute restauration (copie de l'état remplacé), KindMother copie la base (et
//! son `-wal` s'il n'est pas vide après le point de contrôle) dans
//! `<data_dir>/_backups/<base>.v<version>-<AAAAMMJJTHHMMSS.f>.bak` (dossier 0700,
//! fichiers 0600). Chaque copie est inscrite au journal d'audit (`migration` /
//! `sauvegarde`, empreinte SHA-256 du contenu, nom du fichier).
//!
//! **Chiffrement** : la copie est **le fichier SQLCipher lui-même** : elle reste chiffrée
//! avec la clé de la base (schéma `v2` : Argon2id(machine-id ‖ secret d'installation ‖
//! `keys/<base>.secret`), sel `miyukini-kindmother-v2-<base>` ; bases antérieures :
//! clé `v1`), voir `kindmother_db_key`. Une copie lisible en clair est refusée et
//! effacée (fail-closed) ; seul le mode de développement en clair (debug,
//! `KINDMOTHER_DEV_PLAINTEXT=1`) produit des copies en clair. Conséquence : seule
//! l'instance KindMother qui détient le secret de la base la lit ; retirer
//! `keys/<base>.secret` rend toutes ses copies illisibles.
//!
//! **Conservation** : [`RetentionSauvegardes`], **30 jours** par défaut, durée réglable
//! par `KINDMOTHER_BACKUP_RETENTION_DAYS` (1 à 30) ; au-delà de 30 jours (au plus
//! [`RETENTION_MAX_JOURS`]), il faut **en plus** `KINDMOTHER_BACKUP_RETENTION_BEYOND_30_DAYS=1`
//! (configuration explicite), sinon KindMother refuse de démarrer. Âge d'une copie =
//! le plus ancien de l'horodatage du nom et de la date de modification du fichier. Une
//! copie plus âgée que la durée est **effacée de façon sûre** ([`effacer_fichier_sur`] :
//! contenu réécrit de zéros, `fsync`, suppression, `fsync` du dossier ; `-wal`/`-shm`
//! compris) au démarrage puis toutes les [`INTERVALLE_PURGE`] ; chaque purge laisse une
//! entrée d'audit `effacement` / `purge-sauvegarde` (nom du fichier, âge, aucun contenu).
//!
//! **Restauration** (`EncryptedDatabase::restaurer_sauvegarde`) : la copie est recopiée
//! dans `<data_dir>/_restauration`, ouverte avec la clé de la base, contrôlée
//! (`quick_check`), puis **tous les oublis postérieurs à la copie** inscrits au journal
//! (entrées `effacement` d'intentions, registre des oublis) y sont rejoués dans une
//! transaction, suivie d'un point de contrôle `TRUNCATE` complet ; alors seulement la
//! copie remplace la base. Journal illisible ou rompu, oubli non rejouable, empreinte de
//! la copie différente de celle inscrite, échec d'audit ⇒ restauration refusée, base
//! courante inchangée (fail-closed).

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};
use kindmother::intent::{is_valid_identifier, IntentOperation, SqlValue};
use rusqlite::types::Value;
use rusqlite::Connection;
use sha2::{Digest, Sha256};

use crate::audit::{empreinte_cle, OubliRejouable};
use crate::errors::ServiceError;
use crate::typed::{table_schema, ObjectKind};

/// Sous-répertoire des sauvegardes.
pub const BACKUP_DIR: &str = "_backups";

/// Sous-répertoire de préparation d'une restauration (jamais servi aux opérateurs).
pub const RESTORE_DIR: &str = "_restauration";

/// Suffixe d'une sauvegarde.
pub const BACKUP_SUFFIX: &str = ".bak";

/// Durée de conservation par défaut (A-022 §5).
pub const RETENTION_PAR_DEFAUT_JOURS: u32 = 30;

/// Durée maximale acceptée, même avec la configuration explicite.
pub const RETENTION_MAX_JOURS: u32 = 3650;

/// Durée de conservation en jours (1 à 30 sans autre réglage).
pub const ENV_RETENTION_JOURS: &str = "KINDMOTHER_BACKUP_RETENTION_DAYS";

/// Autorisation explicite d'une durée de plus de 30 jours (`1`).
pub const ENV_RETENTION_AU_DELA: &str = "KINDMOTHER_BACKUP_RETENTION_BEYOND_30_DAYS";

/// Intervalle entre deux purges périodiques (en plus de celle du démarrage).
pub const INTERVALLE_PURGE: Duration = Duration::from_secs(6 * 3600);

/// Format de l'horodatage dans le nom d'une sauvegarde (UTC).
const FORMAT_HORODATAGE: &str = "%Y%m%dT%H%M%S%.f";

/// Durée de conservation des sauvegardes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetentionSauvegardes {
    jours: u32,
}

impl Default for RetentionSauvegardes {
    fn default() -> Self {
        Self::par_defaut()
    }
}

impl RetentionSauvegardes {
    /// 30 jours.
    pub const fn par_defaut() -> Self {
        Self { jours: RETENTION_PAR_DEFAUT_JOURS }
    }

    /// Nombre de jours.
    pub fn jours(self) -> u32 {
        self.jours
    }

    /// Durée.
    pub fn duree(self) -> chrono::Duration {
        chrono::Duration::days(i64::from(self.jours))
    }

    /// Lit la durée depuis l'environnement ([`ENV_RETENTION_JOURS`],
    /// [`ENV_RETENTION_AU_DELA`]).
    pub fn depuis_env() -> Result<Self, ServiceError> {
        Self::depuis_valeurs(
            std::env::var(ENV_RETENTION_JOURS).ok().as_deref(),
            std::env::var(ENV_RETENTION_AU_DELA).ok().as_deref(),
        )
    }

    /// Règle de configuration : absente ⇒ 30 ; `1..=30` ; au-delà seulement avec
    /// `au_dela == Some("1")` et jamais plus de [`RETENTION_MAX_JOURS`] ; tout le reste
    /// est refusé (fail-closed : une valeur mal écrite ne doit pas allonger la durée).
    pub fn depuis_valeurs(jours: Option<&str>, au_dela: Option<&str>) -> Result<Self, ServiceError> {
        let Some(brut) = jours.map(str::trim).filter(|s| !s.is_empty()) else {
            return Ok(Self::par_defaut());
        };
        let jours: u32 = brut.parse().map_err(|_| {
            ServiceError::Validation(format!("{ENV_RETENTION_JOURS}={brut:?}: whole number of days expected"))
        })?;
        if jours == 0 {
            return Err(ServiceError::Validation(format!(
                "{ENV_RETENTION_JOURS}=0 refused: at least 1 day"
            )));
        }
        if jours > RETENTION_PAR_DEFAUT_JOURS {
            if au_dela != Some("1") {
                return Err(ServiceError::Validation(format!(
                    "{ENV_RETENTION_JOURS}={jours} exceeds {RETENTION_PAR_DEFAUT_JOURS} days: \
                     set {ENV_RETENTION_AU_DELA}=1 to keep backups longer (A-022 §5)"
                )));
            }
            if jours > RETENTION_MAX_JOURS {
                return Err(ServiceError::Validation(format!(
                    "{ENV_RETENTION_JOURS}={jours} exceeds the maximum of {RETENTION_MAX_JOURS} days"
                )));
            }
        }
        Ok(Self { jours })
    }
}

/// Nom de sauvegarde analysé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NomSauvegarde {
    pub database: String,
    pub version: u32,
    pub horodatage: DateTime<Utc>,
}

/// Nom de fichier d'une nouvelle sauvegarde.
pub(crate) fn nom_sauvegarde(database: &str, version: u32, quand: DateTime<Utc>) -> String {
    format!("{database}.v{version}-{}{BACKUP_SUFFIX}", quand.format(FORMAT_HORODATAGE))
}

/// Analyse `<base>.v<version>-<horodatage>.bak` (nom simple, sans chemin).
pub(crate) fn analyser_nom(fichier: &str) -> Option<NomSauvegarde> {
    let corps = fichier.strip_suffix(BACKUP_SUFFIX)?;
    let (database, reste) = corps.split_once(".v")?;
    let (version, horodatage) = reste.split_once('-')?;
    let valide = !database.is_empty()
        && database
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    if !valide {
        return None;
    }
    let horodatage = NaiveDateTime::parse_from_str(horodatage, FORMAT_HORODATAGE).ok()?;
    Some(NomSauvegarde {
        database: database.to_string(),
        version: version.parse().ok()?,
        horodatage: horodatage.and_utc(),
    })
}

/// Chemin d'un fichier annexe SQLite (`-wal`, `-shm`).
pub(crate) fn annexe(path: &Path, suffixe: &str) -> PathBuf {
    PathBuf::from(format!("{}{suffixe}", path.display()))
}

/// Fichiers d'une base ou d'une sauvegarde : principal, `-wal`, `-shm`.
pub(crate) fn fichiers_de(path: &Path) -> [PathBuf; 3] {
    [path.to_path_buf(), annexe(path, "-wal"), annexe(path, "-shm")]
}

/// Crée (si besoin) un dossier privé 0700.
pub(crate) fn dossier_prive(dir: &Path) -> Result<(), ServiceError> {
    std::fs::create_dir_all(dir)
        .map_err(|e| ServiceError::Internal(format!("Cannot create {}: {e}", dir.display())))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
            .map_err(|e| ServiceError::Internal(format!("Cannot restrict {}: {e}", dir.display())))?;
    }
    Ok(())
}

/// Synchronise un dossier (les suppressions et renommages y deviennent durables).
pub(crate) fn sync_dossier(dir: &Path) -> Result<(), ServiceError> {
    #[cfg(unix)]
    {
        std::fs::File::open(dir)
            .and_then(|d| d.sync_all())
            .map_err(|e| ServiceError::Internal(format!("Cannot sync {}: {e}", dir.display())))?;
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
    }
    Ok(())
}

/// **Effacement sûr** d'un fichier : contenu réécrit de zéros sur toute sa longueur,
/// `fsync`, puis suppression. Un lien symbolique est retiré sans toucher sa cible.
/// Retourne `false` si le fichier n'existait pas.
///
/// Limite : la rémanence matérielle (SSD, copie sur écriture, instantanés du système de
/// fichiers) n'est couverte que par le chiffrement (A-020).
pub(crate) fn effacer_fichier_sur(path: &Path) -> Result<bool, ServiceError> {
    let echec = |quoi: &str, e: std::io::Error| {
        ServiceError::Internal(format!("Secure erase of {} failed ({quoi}): {e}", path.display()))
    };
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => return Err(echec("metadata", e)),
    };
    if meta.file_type().is_symlink() {
        std::fs::remove_file(path).map_err(|e| echec("unlink symlink", e))?;
        return Ok(true);
    }
    if !meta.is_file() {
        return Err(ServiceError::Internal(format!(
            "Secure erase of {} refused: not a regular file",
            path.display()
        )));
    }
    let mut fichier = std::fs::OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|e| echec("open", e))?;
    #[cfg(unix)]
    {
        // Le fichier ouvert doit être celui inspecté (pas de substitution entre-temps).
        use std::os::unix::fs::MetadataExt;
        let ouvert = fichier.metadata().map_err(|e| echec("fstat", e))?;
        if ouvert.ino() != meta.ino() || ouvert.dev() != meta.dev() {
            return Err(ServiceError::Internal(format!(
                "Secure erase of {} refused: file replaced during erase",
                path.display()
            )));
        }
    }
    let zeros = vec![0u8; 64 * 1024];
    let mut reste = meta.len();
    while reste > 0 {
        let n = usize::try_from(reste.min(zeros.len() as u64)).unwrap_or(zeros.len());
        fichier.write_all(&zeros[..n]).map_err(|e| echec("overwrite", e))?;
        reste -= n as u64;
    }
    fichier.flush().map_err(|e| echec("flush", e))?;
    fichier.sync_all().map_err(|e| echec("fsync", e))?;
    drop(fichier);
    std::fs::remove_file(path).map_err(|e| echec("unlink", e))?;
    Ok(true)
}

/// Empreinte SHA-256 (hex) du contenu d'une sauvegarde : fichier principal puis `-wal`
/// (absent ⇔ vide), chacun précédé de sa longueur.
pub(crate) fn empreinte_sauvegarde(path: &Path) -> Result<String, ServiceError> {
    let mut hasher = Sha256::new();
    hasher.update(b"kindmother/sauvegarde/v1");
    for fichier in [path.to_path_buf(), annexe(path, "-wal")] {
        let longueur = std::fs::metadata(&fichier).map(|m| m.len()).unwrap_or(0);
        hasher.update(longueur.to_be_bytes());
        if longueur > 0 {
            let mut f = std::fs::File::open(&fichier).map_err(|e| {
                ServiceError::Internal(format!("Cannot read backup {}: {e}", fichier.display()))
            })?;
            std::io::copy(&mut f, &mut hasher).map_err(|e| {
                ServiceError::Internal(format!("Cannot read backup {}: {e}", fichier.display()))
            })?;
        }
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Âge effectif d'une sauvegarde : le plus ancien de l'horodatage du nom et de la date
/// de modification (un nom daté dans le futur ne la soustrait pas à la purge).
pub(crate) fn date_effective(nom: Option<&NomSauvegarde>, modifie: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
    match (nom.map(|n| n.horodatage), modifie) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

// ----------------------------------------------------------------------------------
// Rejeu des oublis sur une copie restaurée
// ----------------------------------------------------------------------------------

/// Oubli à rejouer.
pub(crate) enum ARejouer<'a> {
    /// Ligne du registre des oublis (entrées écrites par cette version).
    Registre(&'a OubliRejouable),
    /// Entrée `effacement` antérieure au registre : seule l'empreinte des paramètres est
    /// connue (`Delete` seulement) ; colonnes de clé recherchées parmi les colonnes et
    /// paires de colonnes de la table.
    Ancien {
        /// Table.
        table: &'a str,
        /// Empreinte SHA-256 des paramètres canoniques de l'intention.
        params_hash: &'a str,
    },
}

/// Nombre maximal de colonnes de clé pour lesquelles les variantes de type
/// (entier ↔ texte) sont essayées (2^n combinaisons).
const MAX_COLONNES_VARIANTES: usize = 6;

fn vers_sql(v: &Value) -> SqlValue {
    match v {
        Value::Null => SqlValue::Null,
        Value::Integer(i) => SqlValue::Integer(*i),
        Value::Real(f) => SqlValue::Real(*f),
        Value::Text(s) => SqlValue::Text(s.clone()),
        Value::Blob(b) => SqlValue::Blob(b.clone()),
    }
}

/// Valeurs qu'une intention a pu porter pour une valeur stockée (affinité de colonne :
/// `'5'` comparé à une colonne entière vaut `5`, et inversement).
fn variantes(v: &Value) -> Vec<SqlValue> {
    let mut out = vec![vers_sql(v)];
    match v {
        Value::Integer(i) => out.push(SqlValue::Text(i.to_string())),
        Value::Text(s) => {
            if let Ok(i) = s.parse::<i64>() {
                if i.to_string() == *s {
                    out.push(SqlValue::Integer(i));
                }
            }
        }
        _ => {}
    }
    out
}

/// Toutes les combinaisons de variantes d'un n-uplet de clé.
fn combinaisons(colonnes: &[String], valeurs: &[Value]) -> Vec<BTreeMap<String, SqlValue>> {
    let mut out: Vec<BTreeMap<String, SqlValue>> = vec![BTreeMap::new()];
    let avec_variantes = colonnes.len() <= MAX_COLONNES_VARIANTES;
    for (colonne, valeur) in colonnes.iter().zip(valeurs) {
        let choix = if avec_variantes { variantes(valeur) } else { vec![vers_sql(valeur)] };
        let mut suivant = Vec::with_capacity(out.len() * choix.len());
        for partiel in &out {
            for c in &choix {
                let mut m = partiel.clone();
                m.insert(colonne.clone(), c.clone());
                suivant.push(m);
            }
        }
        out = suivant;
    }
    out
}

/// Empreinte des paramètres canoniques d'un `Delete` d'effacement de clé `cle`
/// (identique à `TypedWriteIntent::canonical_params` haché par l'audit).
fn params_hash_oubli(cle: &BTreeMap<String, SqlValue>) -> String {
    let mut out = Vec::new();
    for tag in *b"vi" {
        out.push(tag);
        out.extend_from_slice(&0u64.to_be_bytes());
    }
    out.push(b'k');
    out.extend_from_slice(&(cle.len() as u64).to_be_bytes());
    for (colonne, valeur) in cle {
        kindmother::intent::push_len_prefixed(&mut out, colonne.as_bytes());
        valeur.canonical_bytes(&mut out);
    }
    out.push(b'e');
    crate::audit::sha256_hex(&out)
}

fn quote(ident: &str) -> String {
    format!("\"{ident}\"")
}

/// N-uplets distincts des colonnes `colonnes` de `table`.
fn nuplets(conn: &Connection, table: &str, colonnes: &[String]) -> Result<Vec<Vec<Value>>, String> {
    let liste: Vec<String> = colonnes.iter().map(|c| quote(c)).collect();
    let sql = format!("SELECT DISTINCT {} FROM {}", liste.join(", "), quote(table));
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("cannot scan {table}: {e}"))?;
    let n = colonnes.len();
    let lignes = stmt
        .query_map([], |r| (0..n).map(|i| r.get::<_, Value>(i)).collect::<Result<Vec<_>, _>>())
        .map_err(|e| format!("cannot scan {table}: {e}"))?;
    lignes
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("cannot scan {table}: {e}"))
}

/// Applique l'oubli aux lignes de clé `valeurs_cle` (valeurs telles que stockées).
fn appliquer(
    conn: &Connection,
    table: &str,
    operation: IntentOperation,
    colonnes: &[String],
    valeurs_cle: &[Value],
    remplacement: &BTreeMap<String, SqlValue>,
) -> Result<u64, String> {
    let mut params: Vec<Value> = Vec::new();
    let mut sets = Vec::new();
    for (colonne, valeur) in remplacement {
        params.push(crate::typed::to_value(valeur));
        sets.push(format!("{} = ?{}", quote(colonne), params.len()));
    }
    let mut filtre = Vec::new();
    for (colonne, valeur) in colonnes.iter().zip(valeurs_cle) {
        params.push(valeur.clone());
        filtre.push(format!("{} IS ?{}", quote(colonne), params.len()));
    }
    let sql = match operation {
        IntentOperation::Delete => format!("DELETE FROM {} WHERE {}", quote(table), filtre.join(" AND ")),
        IntentOperation::Update | IntentOperation::Upsert => {
            if sets.is_empty() {
                return Ok(0);
            }
            format!("UPDATE {} SET {} WHERE {}", quote(table), sets.join(", "), filtre.join(" AND "))
        }
        IntentOperation::Insert => return Err("insert is never an erasure".to_string()),
    };
    conn.execute(&sql, rusqlite::params_from_iter(params.iter()))
        .map(|n| n as u64)
        .map_err(|e| format!("replay on {table} failed: {e}"))
}

/// Rejoue un oubli sur `conn` (copie en cours de restauration). Retourne le nombre de
/// lignes effacées ou caviardées. `Err` ⇒ oubli non rejouable : restauration refusée.
pub(crate) fn rejouer(conn: &Connection, oubli: &ARejouer<'_>) -> Result<u64, String> {
    let table = match oubli {
        ARejouer::Registre(o) => o.table.as_str(),
        ARejouer::Ancien { table, .. } => table,
    };
    kindmother::intent::check_table_name(table).map_err(|e| format!("invalid table in journal: {e}"))?;
    let Some(schema) = table_schema(conn, table).map_err(|e| e.to_string())? else {
        // Table absente de la copie : aucune ligne à oublier.
        return Ok(0);
    };
    if schema.kind != ObjectKind::Table {
        return Err(format!("{table} is not a table in the restored copy"));
    }
    match oubli {
        ARejouer::Registre(o) => {
            if o.operation == IntentOperation::Insert || o.cle_colonnes.is_empty() {
                return Err(format!("erasure on {table}: invalid registry entry"));
            }
            for colonne in o.cle_colonnes.iter().chain(o.valeurs.keys()) {
                if !is_valid_identifier(colonne) {
                    return Err(format!("erasure on {table}: invalid column name in registry"));
                }
                if !schema.has_column(colonne) {
                    return Err(format!(
                        "erasure on {table} cannot be replayed: column {colonne} absent from the restored copy"
                    ));
                }
            }
            let mut total = 0;
            for nuplet in nuplets(conn, table, &o.cle_colonnes)? {
                let correspond = combinaisons(&o.cle_colonnes, &nuplet)
                    .iter()
                    .any(|cle| empreinte_cle(cle) == o.cle_empreinte);
                if correspond {
                    total += appliquer(conn, table, o.operation, &o.cle_colonnes, &nuplet, &o.valeurs)?;
                }
            }
            Ok(total)
        }
        ARejouer::Ancien { params_hash, .. } => {
            let mut colonnes = schema.columns.clone();
            colonnes.sort();
            let mut ensembles: Vec<Vec<String>> = colonnes.iter().map(|c| vec![c.clone()]).collect();
            for (i, a) in colonnes.iter().enumerate() {
                for b in &colonnes[i + 1..] {
                    ensembles.push(vec![a.clone(), b.clone()]);
                }
            }
            let mut total = 0;
            for ensemble in ensembles.iter().filter(|e| e.iter().all(|c| is_valid_identifier(c))) {
                for nuplet in nuplets(conn, table, ensemble)? {
                    let correspond = combinaisons(ensemble, &nuplet)
                        .iter()
                        .any(|cle| params_hash_oubli(cle) == *params_hash);
                    if correspond {
                        total += appliquer(conn, table, IntentOperation::Delete, ensemble, &nuplet, &BTreeMap::new())?;
                    }
                }
            }
            Ok(total)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retention_par_defaut_et_bornes() {
        assert_eq!(RetentionSauvegardes::depuis_valeurs(None, None).expect("defaut").jours(), 30);
        assert_eq!(RetentionSauvegardes::depuis_valeurs(Some(" "), None).expect("vide").jours(), 30);
        assert_eq!(RetentionSauvegardes::depuis_valeurs(Some("7"), None).expect("7").jours(), 7);
        assert_eq!(RetentionSauvegardes::depuis_valeurs(Some("30"), None).expect("30").jours(), 30);
        assert!(RetentionSauvegardes::depuis_valeurs(Some("0"), None).is_err());
        assert!(RetentionSauvegardes::depuis_valeurs(Some("-1"), None).is_err());
        assert!(RetentionSauvegardes::depuis_valeurs(Some("trente"), None).is_err());
        // Au-delà de 30 jours : configuration explicite exigée.
        assert!(RetentionSauvegardes::depuis_valeurs(Some("31"), None).is_err());
        assert!(RetentionSauvegardes::depuis_valeurs(Some("31"), Some("oui")).is_err());
        assert_eq!(RetentionSauvegardes::depuis_valeurs(Some("90"), Some("1")).expect("90").jours(), 90);
        assert!(RetentionSauvegardes::depuis_valeurs(Some("3651"), Some("1")).is_err());
    }

    #[test]
    fn nom_de_sauvegarde_aller_retour() {
        let quand = Utc::now();
        let nom = nom_sauvegarde("maia-memoire", 3, quand);
        let lu = analyser_nom(&nom).expect("nom");
        assert_eq!(lu.database, "maia-memoire");
        assert_eq!(lu.version, 3);
        assert_eq!(lu.horodatage, quand);
        assert!(analyser_nom("x.v1-20260101T000000.bak").is_some(), "sans fraction");
        for faux in ["x.bak", "X.v1-20260101T000000.bak", "x.v1-hier.bak", "x.v1-20260101T000000", "../x.v1-20260101T000000.bak"] {
            assert!(analyser_nom(faux).is_none(), "{faux}");
        }
    }

    #[test]
    fn date_effective_la_plus_ancienne() {
        let nom = analyser_nom("x.v1-20260101T000000.bak").expect("nom");
        let futur = analyser_nom("x.v1-29990101T000000.bak").expect("nom");
        let maintenant = Utc::now();
        assert_eq!(date_effective(Some(&nom), Some(maintenant)), Some(nom.horodatage));
        assert_eq!(date_effective(Some(&futur), Some(maintenant)), Some(maintenant));
        assert_eq!(date_effective(None, Some(maintenant)), Some(maintenant));
        assert_eq!(date_effective(None, None), None);
    }

    #[test]
    fn effacement_sur_reecrit_puis_supprime() {
        let dir = std::env::temp_dir().join(format!("km-sauv-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("dir");
        let f = dir.join("a.bak");
        std::fs::write(&f, b"SECRET-SECRET-SECRET").expect("write");
        let lien = dir.join("lien");
        std::fs::hard_link(&f, &lien).expect("hard link");
        assert!(effacer_fichier_sur(&f).expect("erase"));
        assert!(!f.exists());
        let reste = std::fs::read(&lien).expect("read");
        assert_eq!(reste.len(), 20);
        assert!(reste.iter().all(|&b| b == 0), "contenu réécrit de zéros");
        assert!(!effacer_fichier_sur(&f).expect("absent"), "absent ⇒ false");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn rejeu_par_registre_et_ancien() {
        let c = Connection::open_in_memory().expect("mem");
        c.execute_batch(
            "CREATE TABLE t (id TEXT PRIMARY KEY, n INTEGER, v TEXT);
             INSERT INTO t VALUES ('a', 1, 'x'), ('b', 2, 'y'), ('c', 3, 'z');",
        )
        .expect("schema");
        let intent = kindmother::intent::TypedWriteIntent::forget("t").key("id", "a");
        let oubli = OubliRejouable::depuis_intention(&intent, "db");
        assert_eq!(rejouer(&c, &ARejouer::Registre(&oubli)).expect("rejeu"), 1);
        assert_eq!(rejouer(&c, &ARejouer::Registre(&oubli)).expect("idempotent"), 0);
        // Clé texte '2' sur colonne entière.
        let intent = kindmother::intent::TypedWriteIntent::forget("t").key("n", "2");
        let oubli = OubliRejouable::depuis_intention(&intent, "db");
        assert_eq!(rejouer(&c, &ARejouer::Registre(&oubli)).expect("variante"), 1);
        // Caviardage.
        let intent = kindmother::intent::TypedWriteIntent::update("t").key("id", "c").value("v", "[oublie]").erasure();
        let oubli = OubliRejouable::depuis_intention(&intent, "db");
        assert_eq!(rejouer(&c, &ARejouer::Registre(&oubli)).expect("caviardage"), 1);
        let v: String = c.query_row("SELECT v FROM t WHERE id = 'c'", [], |r| r.get(0)).expect("v");
        assert_eq!(v, "[oublie]");
        // Colonne de clé absente : non rejouable.
        let intent = kindmother::intent::TypedWriteIntent::forget("t").key("absente", "c");
        let oubli = OubliRejouable::depuis_intention(&intent, "db");
        assert!(rejouer(&c, &ARejouer::Registre(&oubli)).is_err());
        // Table absente : rien à faire.
        let intent = kindmother::intent::TypedWriteIntent::forget("autre").key("id", "c");
        let oubli = OubliRejouable::depuis_intention(&intent, "db");
        assert_eq!(rejouer(&c, &ARejouer::Registre(&oubli)).expect("absente"), 0);
        // Entrée antérieure : empreinte des paramètres seulement.
        let intent = kindmother::intent::TypedWriteIntent::forget("t").key("id", "c");
        let h = crate::audit::sha256_hex(&intent.canonical_params());
        assert_eq!(rejouer(&c, &ARejouer::Ancien { table: "t", params_hash: &h }).expect("ancien"), 1);
        let n: i64 = c.query_row("SELECT count(*) FROM t", [], |r| r.get(0)).expect("count");
        assert_eq!(n, 0);
    }
}
