//! @id     kindmother_intent
//! @do     definir_intentions_ecriture_typees
//! @role   donnee
//! @layer  core
//! @human  kindmother — intentions d'écriture typées et leur cycle de vie (K-13, EX-07).

//! Intentions d'écriture **typées** et requêtes typées (lot C, K-13, EX-07).
//!
//! Un opérateur ne transmet plus de texte SQL : il décrit ce qu'il veut faire
//! (base, table, opération, valeurs typées, clé) et KindMother Service valide puis
//! applique l'intention. Le SQL est construit **côté service** à partir
//! d'identifiants vérifiés contre le schéma ; toutes les valeurs sont liées en
//! paramètres.
//!
//! Cycle de vie (contrat « Write Intent Lifecycle », simplifié) :
//!
//! ```text
//! Créée → Validée → Appliquée → Archivée
//!       ↘ Refusée ──────────────↗
//!         Validée → Échouée ────↗   (erreur à l'application : transaction annulée)
//! ```
//!
//! Un identifiant d'intention ne sert qu'une fois (non-réutilisation, §6 du contrat).
//! Ce module ne contient que des types et des contrôles de forme ; la validation
//! contre le schéma, l'arbitrage et l'audit sont faits par `kindmother-service`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Nombre maximal de colonnes touchées par une intention (C-13).
pub const MAX_INTENT_COLUMNS: usize = 256;

/// Nombre maximal d'intentions dans un lot atomique (C-13).
pub const MAX_INTENTS_PER_BATCH: usize = 1000;

/// Longueur maximale d'un identifiant (table, colonne).
pub const MAX_IDENTIFIER_LEN: usize = 64;

/// Longueur maximale d'une empreinte de décision StrongFather jointe.
pub const MAX_DECISION_DIGEST_LEN: usize = 256;

/// Préfixes réservés : catalogue SQLite et tables système KindMother.
pub const RESERVED_TABLE_PREFIXES: &[&str] = &["sqlite_", "_kindmother_"];

/// Valeur SQL typée (jamais interprétée comme du SQL).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SqlValue {
    /// `NULL`
    Null,
    /// Entier 64 bits
    Integer(i64),
    /// Flottant 64 bits
    Real(f64),
    /// Texte UTF-8
    Text(String),
    /// Octets
    Blob(Vec<u8>),
}

impl SqlValue {
    /// Valeur sous forme d'empreinte canonique (pour le hachage des paramètres d'audit).
    pub fn canonical_bytes(&self, out: &mut Vec<u8>) {
        match self {
            SqlValue::Null => out.push(0),
            SqlValue::Integer(i) => {
                out.push(1);
                out.extend_from_slice(&i.to_be_bytes());
            }
            SqlValue::Real(f) => {
                out.push(2);
                out.extend_from_slice(&f.to_bits().to_be_bytes());
            }
            SqlValue::Text(s) => {
                out.push(3);
                push_len_prefixed(out, s.as_bytes());
            }
            SqlValue::Blob(b) => {
                out.push(4);
                push_len_prefixed(out, b);
            }
        }
    }
}

/// Ajoute `bytes` précédé de sa longueur (encodage sans ambiguïté).
pub fn push_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
    out.extend_from_slice(bytes);
}

impl From<&str> for SqlValue {
    fn from(v: &str) -> Self {
        SqlValue::Text(v.to_string())
    }
}

impl From<String> for SqlValue {
    fn from(v: String) -> Self {
        SqlValue::Text(v)
    }
}

impl From<&String> for SqlValue {
    fn from(v: &String) -> Self {
        SqlValue::Text(v.clone())
    }
}

impl From<i64> for SqlValue {
    fn from(v: i64) -> Self {
        SqlValue::Integer(v)
    }
}

impl From<i32> for SqlValue {
    fn from(v: i32) -> Self {
        SqlValue::Integer(i64::from(v))
    }
}

impl From<u32> for SqlValue {
    fn from(v: u32) -> Self {
        SqlValue::Integer(i64::from(v))
    }
}

impl From<f64> for SqlValue {
    fn from(v: f64) -> Self {
        SqlValue::Real(v)
    }
}

impl From<bool> for SqlValue {
    fn from(v: bool) -> Self {
        SqlValue::Integer(i64::from(v))
    }
}

impl From<Vec<u8>> for SqlValue {
    fn from(v: Vec<u8>) -> Self {
        SqlValue::Blob(v)
    }
}

impl<T: Into<SqlValue>> From<Option<T>> for SqlValue {
    fn from(v: Option<T>) -> Self {
        v.map_or(SqlValue::Null, Into::into)
    }
}

/// Opération d'une intention d'écriture typée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentOperation {
    /// Insertion d'une ligne (`values`).
    Insert,
    /// Mise à jour des lignes désignées par `key` (`values`, `increments`).
    Update,
    /// Suppression des lignes désignées par `key`.
    Delete,
    /// Insertion, ou mise à jour si la clé (`key`, contrainte d'unicité) existe déjà.
    Upsert,
}

impl IntentOperation {
    /// Nom stable (audit).
    pub fn as_str(self) -> &'static str {
        match self {
            IntentOperation::Insert => "insert",
            IntentOperation::Update => "update",
            IntentOperation::Delete => "delete",
            IntentOperation::Upsert => "upsert",
        }
    }
}

/// Intention d'écriture typée (aucun texte SQL).
///
/// `operator_id`, `database` et `intent_id` peuvent être laissés vides : le client
/// les remplit (identité authentifiée, base par défaut, identifiant aléatoire).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedWriteIntent {
    /// Identifiant unique, jamais réutilisé.
    pub intent_id: String,
    /// Opérateur émetteur (doit égaler l'identité authentifiée).
    pub operator_id: String,
    /// Base visée.
    pub database: String,
    /// Table visée (jamais `_kindmother_*` ni `sqlite_*`).
    pub table: String,
    /// Opération.
    pub operation: IntentOperation,
    /// Valeurs affectées (`Insert`, `Upsert`, `Update`).
    #[serde(default)]
    pub values: BTreeMap<String, SqlValue>,
    /// Incréments (`colonne = colonne + valeur`, `Update` seulement).
    #[serde(default)]
    pub increments: BTreeMap<String, SqlValue>,
    /// Clé : égalités qui désignent les lignes (`Update`, `Delete`) ou colonnes
    /// de la contrainte d'unicité (`Upsert`).
    #[serde(default)]
    pub key: BTreeMap<String, SqlValue>,
    /// Finalité déclarée (libre, journalisée).
    #[serde(default)]
    pub purpose: String,
    /// Empreinte opaque d'une décision StrongFather jointe (journalisée si présente).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_digest: Option<String>,
    /// **Effacement** (oubli, droit à l'effacement) : opération typée explicite.
    ///
    /// Permis sur `Delete` (forme normale, `TypedWriteIntent::forget`), ainsi que sur
    /// `Update`/`Upsert` qui écrasent une valeur à oublier (caviardage) ; refusé sur
    /// `Insert`. Un lot qui contient au moins une intention d'effacement est **toujours**
    /// suivi par KindMother d'un point de contrôle `wal_checkpoint(TRUNCATE)` (même si
    /// aucune ligne n'a été touchée) et chaque intention d'effacement appliquée laisse
    /// une entrée d'audit `effacement` (empreinte de la clé, jamais la valeur).
    ///
    /// Format filaire : champ JSON `"erasure": true`, **absent** quand faux (les
    /// intentions ordinaires gardent leur encodage et leur empreinte d'audit).
    #[serde(default, skip_serializing_if = "is_false")]
    pub erasure: bool,
}

// Signature imposée par `serde(skip_serializing_if)`.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(v: &bool) -> bool {
    !*v
}

impl TypedWriteIntent {
    /// Intention vierge pour `table`.
    pub fn new(operation: IntentOperation, table: impl Into<String>) -> Self {
        Self {
            intent_id: String::new(),
            operator_id: String::new(),
            database: String::new(),
            table: table.into(),
            operation,
            values: BTreeMap::new(),
            increments: BTreeMap::new(),
            key: BTreeMap::new(),
            purpose: String::new(),
            decision_digest: None,
            erasure: false,
        }
    }

    /// Insertion dans `table`.
    pub fn insert(table: impl Into<String>) -> Self {
        Self::new(IntentOperation::Insert, table)
    }

    /// Mise à jour dans `table`.
    pub fn update(table: impl Into<String>) -> Self {
        Self::new(IntentOperation::Update, table)
    }

    /// Suppression dans `table`.
    pub fn delete(table: impl Into<String>) -> Self {
        Self::new(IntentOperation::Delete, table)
    }

    /// **Oubli** dans `table` : suppression typée explicite d'effacement (`Delete` +
    /// `erasure`). KindMother garantit suppression, point de contrôle `TRUNCATE` du WAL
    /// (pages remises à zéro par `secure_delete` recopiées, `-wal` vidé) et entrée
    /// d'audit `effacement` sans la valeur. À utiliser par tout service d'oubli
    /// (MAIA `maia-memoire`, Alicia `miyualicia-memory`) plutôt qu'un `delete` reconnu
    /// par sa seule finalité textuelle.
    pub fn forget(table: impl Into<String>) -> Self {
        Self::new(IntentOperation::Delete, table).erasure()
    }

    /// Marque l'intention comme effacement (`Delete`, ou `Update`/`Upsert` qui écrase
    /// une valeur à oublier).
    pub fn erasure(mut self) -> Self {
        self.erasure = true;
        self
    }

    /// Insertion ou mise à jour dans `table`.
    pub fn upsert(table: impl Into<String>) -> Self {
        Self::new(IntentOperation::Upsert, table)
    }

    /// Affecte une valeur à une colonne.
    pub fn value(mut self, column: impl Into<String>, value: impl Into<SqlValue>) -> Self {
        self.values.insert(column.into(), value.into());
        self
    }

    /// Incrémente une colonne (`Update`).
    pub fn increment(mut self, column: impl Into<String>, value: impl Into<SqlValue>) -> Self {
        self.increments.insert(column.into(), value.into());
        self
    }

    /// Ajoute une égalité de clé.
    pub fn key(mut self, column: impl Into<String>, value: impl Into<SqlValue>) -> Self {
        self.key.insert(column.into(), value.into());
        self
    }

    /// Finalité déclarée.
    pub fn purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = purpose.into();
        self
    }

    /// Base visée (sinon : base par défaut du client).
    pub fn database(mut self, database: impl Into<String>) -> Self {
        self.database = database.into();
        self
    }

    /// Joint l'empreinte d'une décision StrongFather.
    pub fn decision(mut self, digest: impl Into<String>) -> Self {
        self.decision_digest = Some(digest.into());
        self
    }

    /// Empreinte canonique des paramètres (valeurs, incréments, clé) pour l'audit.
    pub fn canonical_params(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (tag, map) in [(b'v', &self.values), (b'i', &self.increments), (b'k', &self.key)] {
            out.push(tag);
            out.extend_from_slice(&(map.len() as u64).to_be_bytes());
            for (column, value) in map {
                push_len_prefixed(&mut out, column.as_bytes());
                value.canonical_bytes(&mut out);
            }
        }
        // Ajouté seulement pour un effacement : l'empreinte des intentions ordinaires
        // ne change pas.
        if self.erasure {
            out.push(b'e');
        }
        out
    }

    /// Contrôle de forme, indépendant du schéma (le service refait ce contrôle).
    pub fn check_shape(&self) -> Result<(), String> {
        check_table_name(&self.table)?;
        let total = self.values.len() + self.increments.len() + self.key.len();
        if total > MAX_INTENT_COLUMNS {
            return Err(format!("too many columns ({total} > {MAX_INTENT_COLUMNS})"));
        }
        for column in self
            .values
            .keys()
            .chain(self.increments.keys())
            .chain(self.key.keys())
        {
            if !is_valid_identifier(column) {
                return Err(format!("invalid column name: {column:?}"));
            }
        }
        if let Some(digest) = &self.decision_digest {
            if digest.is_empty() || digest.len() > MAX_DECISION_DIGEST_LEN {
                return Err("invalid decision digest length".to_string());
            }
        }
        if self.erasure && self.operation == IntentOperation::Insert {
            return Err("erasure applies to delete, update or upsert, not insert".to_string());
        }
        match self.operation {
            IntentOperation::Insert => {
                if self.values.is_empty() {
                    return Err("insert requires values".to_string());
                }
                if !self.increments.is_empty() || !self.key.is_empty() {
                    return Err("insert accepts values only".to_string());
                }
            }
            IntentOperation::Update => {
                if self.values.is_empty() && self.increments.is_empty() {
                    return Err("update requires values or increments".to_string());
                }
                if self.key.is_empty() {
                    return Err("update requires a key (no unbounded update)".to_string());
                }
                if self
                    .values
                    .keys()
                    .any(|c| self.increments.contains_key(c))
                {
                    return Err("a column cannot be both set and incremented".to_string());
                }
            }
            IntentOperation::Delete => {
                if self.key.is_empty() {
                    return Err("delete requires a key (no unbounded delete)".to_string());
                }
                if !self.values.is_empty() || !self.increments.is_empty() {
                    return Err("delete accepts a key only".to_string());
                }
            }
            IntentOperation::Upsert => {
                if self.key.is_empty() {
                    return Err("upsert requires the conflict key".to_string());
                }
                if !self.increments.is_empty() {
                    return Err("upsert does not accept increments".to_string());
                }
                if self.values.keys().any(|c| self.key.contains_key(c)) {
                    return Err("a key column cannot also be a value".to_string());
                }
            }
        }
        Ok(())
    }
}

/// Identifiant SQL accepté : `[A-Za-z_][A-Za-z0-9_]*`, borné.
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && name.len() <= MAX_IDENTIFIER_LEN
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Vrai si la table est réservée (catalogue SQLite, tables système KindMother).
pub fn is_reserved_table(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    RESERVED_TABLE_PREFIXES
        .iter()
        .any(|prefix| lower.starts_with(prefix))
}

/// Contrôle d'un nom de table d'opérateur.
pub fn check_table_name(table: &str) -> Result<(), String> {
    if !is_valid_identifier(table) {
        return Err(format!("invalid table name: {table:?}"));
    }
    if is_reserved_table(table) {
        return Err(format!("reserved table: {table}"));
    }
    Ok(())
}

/// État d'une intention dans son cycle de vie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentState {
    /// Reçue, non encore validée.
    Creee,
    /// Toutes les validations sont passées.
    Validee,
    /// Une validation a échoué (terminal avant archivage).
    Refusee,
    /// Appliquée et validée en base (terminal avant archivage).
    Appliquee,
    /// Validée mais l'application a échoué ; transaction annulée (terminal avant archivage).
    Echouee,
    /// Scellée dans le journal d'audit (état final).
    Archivee,
}

impl IntentState {
    /// Nom stable (audit).
    pub fn as_str(self) -> &'static str {
        match self {
            IntentState::Creee => "creee",
            IntentState::Validee => "validee",
            IntentState::Refusee => "refusee",
            IntentState::Appliquee => "appliquee",
            IntentState::Echouee => "echouee",
            IntentState::Archivee => "archivee",
        }
    }

    /// Transitions permises.
    pub fn can_transition_to(self, next: IntentState) -> bool {
        use IntentState::{Appliquee, Archivee, Creee, Echouee, Refusee, Validee};
        matches!(
            (self, next),
            (Creee, Validee | Refusee)
                | (Validee, Appliquee | Echouee | Refusee)
                | (Refusee | Appliquee | Echouee, Archivee)
        )
    }
}

/// Transition refusée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleError {
    /// État courant.
    pub from: IntentState,
    /// État demandé.
    pub to: IntentState,
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid intent transition {} -> {}",
            self.from.as_str(),
            self.to.as_str()
        )
    }
}

impl std::error::Error for LifecycleError {}

/// Cycle de vie d'une intention : état courant et historique des transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentLifecycle {
    state: IntentState,
    history: Vec<IntentState>,
}

impl Default for IntentLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentLifecycle {
    /// Nouvelle intention (état `Créée`).
    pub fn new() -> Self {
        Self {
            state: IntentState::Creee,
            history: vec![IntentState::Creee],
        }
    }

    /// État courant.
    pub fn state(&self) -> IntentState {
        self.state
    }

    /// Historique (du premier au dernier état).
    pub fn history(&self) -> &[IntentState] {
        &self.history
    }

    /// Passe à `next` si la transition est permise.
    pub fn transition(&mut self, next: IntentState) -> Result<(), LifecycleError> {
        if !self.state.can_transition_to(next) {
            return Err(LifecycleError {
                from: self.state,
                to: next,
            });
        }
        self.state = next;
        self.history.push(next);
        Ok(())
    }

    /// Dernier état terminal atteint avant l'archivage (`Appliquée`, `Refusée`, `Échouée`).
    pub fn terminal(&self) -> Option<IntentState> {
        self.history.iter().rev().copied().find(|s| {
            matches!(
                s,
                IntentState::Appliquee | IntentState::Refusee | IntentState::Echouee
            )
        })
    }
}

/// Issue d'une intention, renvoyée à l'opérateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentOutcome {
    /// Identifiant de l'intention.
    pub intent_id: String,
    /// État terminal (`Appliquee`, `Refusee`, `Echouee`).
    pub state: IntentState,
    /// Vrai si l'issue est scellée dans le journal d'audit.
    pub archived: bool,
    /// Lignes affectées (0 si non appliquée).
    pub rows_affected: u64,
    /// Dernier rowid inséré (insertion appliquée).
    pub last_insert_id: Option<i64>,
    /// Motif de refus ou d'échec.
    pub reason: Option<String>,
}

/// Tri d'une lecture typée.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderBy {
    /// Colonne.
    pub column: String,
    /// Tri décroissant.
    #[serde(default)]
    pub descending: bool,
}

/// Lecture typée (aucun texte SQL) : égalités, tri, limite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedSelect {
    /// Table lue.
    pub table: String,
    /// Colonnes retournées (vide : toutes).
    #[serde(default)]
    pub columns: Vec<String>,
    /// Égalités (`colonne = valeur`, `NULL` : `IS NULL`).
    #[serde(default)]
    pub filter: BTreeMap<String, SqlValue>,
    /// Tri.
    #[serde(default)]
    pub order_by: Vec<OrderBy>,
    /// Limite.
    #[serde(default)]
    pub limit: Option<u32>,
    /// Décalage.
    #[serde(default)]
    pub offset: Option<u32>,
}

impl TypedSelect {
    /// Lecture de toutes les colonnes de `table`.
    pub fn from(table: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            columns: Vec::new(),
            filter: BTreeMap::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// Restreint les colonnes retournées.
    pub fn columns<I, S>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.columns = columns.into_iter().map(Into::into).collect();
        self
    }

    /// Ajoute une égalité.
    pub fn filter(mut self, column: impl Into<String>, value: impl Into<SqlValue>) -> Self {
        self.filter.insert(column.into(), value.into());
        self
    }

    /// Ajoute un tri.
    pub fn order_by(mut self, column: impl Into<String>, descending: bool) -> Self {
        self.order_by.push(OrderBy {
            column: column.into(),
            descending,
        });
        self
    }

    /// Fixe la limite.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Contrôle de forme, indépendant du schéma.
    pub fn check_shape(&self) -> Result<(), String> {
        check_table_name(&self.table)?;
        for column in self
            .columns
            .iter()
            .chain(self.filter.keys())
            .chain(self.order_by.iter().map(|o| &o.column))
        {
            if !is_valid_identifier(column) {
                return Err(format!("invalid column name: {column:?}"));
            }
        }
        if self.columns.len() + self.filter.len() + self.order_by.len() > MAX_INTENT_COLUMNS {
            return Err("too many columns".to_string());
        }
        Ok(())
    }
}

/// Étape de migration de schéma versionnée (`version - 1 → version`).
///
/// Appliquée par le service dans une transaction, sous une politique **non
/// destructive** : aucune table ni colonne existante ne peut disparaître, aucune
/// ligne ne peut être supprimée (C-41, EX-08).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigration {
    /// Version atteinte après l'étape (≥ 1, contiguë).
    pub version: u32,
    /// Description journalisée.
    pub description: String,
    /// Scripts DDL (une ou plusieurs instructions par entrée, chacune contrôlée).
    pub statements: Vec<String>,
}

impl SchemaMigration {
    /// Nouvelle étape.
    pub fn new(version: u32, description: impl Into<String>, statements: &[&str]) -> Self {
        Self {
            version,
            description: description.into(),
            statements: statements.iter().map(|s| (*s).to_string()).collect(),
        }
    }
}

/// Contrôle de contiguïté d'une suite de migrations (`1, 2, …, n`).
pub fn check_migrations(migrations: &[SchemaMigration]) -> Result<(), String> {
    for (i, m) in migrations.iter().enumerate() {
        let expected = u32::try_from(i + 1).map_err(|_| "too many migrations".to_string())?;
        if m.version != expected {
            return Err(format!(
                "migration versions must be contiguous from 1 (expected {expected}, got {})",
                m.version
            ));
        }
        if m.statements.is_empty() {
            return Err(format!("migration {} has no statement", m.version));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_de_vie_nominal_et_refus() {
        let mut lc = IntentLifecycle::new();
        lc.transition(IntentState::Validee).expect("validée");
        lc.transition(IntentState::Appliquee).expect("appliquée");
        lc.transition(IntentState::Archivee).expect("archivée");
        assert_eq!(lc.terminal(), Some(IntentState::Appliquee));
        assert!(lc.transition(IntentState::Appliquee).is_err(), "archivée est final");

        let mut lc = IntentLifecycle::new();
        assert!(lc.transition(IntentState::Appliquee).is_err(), "pas d'application sans validation");
        lc.transition(IntentState::Refusee).expect("refusée");
        assert!(lc.transition(IntentState::Validee).is_err());
        lc.transition(IntentState::Archivee).expect("archivée");
        assert_eq!(lc.terminal(), Some(IntentState::Refusee));
    }

    #[test]
    fn forme_des_intentions() {
        assert!(TypedWriteIntent::insert("t").value("a", 1_i64).check_shape().is_ok());
        assert!(TypedWriteIntent::insert("t").check_shape().is_err());
        assert!(TypedWriteIntent::update("t").value("a", 1_i64).check_shape().is_err());
        assert!(TypedWriteIntent::delete("t").check_shape().is_err());
        assert!(TypedWriteIntent::delete("t").key("id", "x").check_shape().is_ok());
        assert!(TypedWriteIntent::insert("_kindmother_audit").value("a", 1_i64).check_shape().is_err());
        assert!(TypedWriteIntent::insert("sqlite_master").value("a", 1_i64).check_shape().is_err());
        assert!(TypedWriteIntent::insert("t; DROP TABLE t").value("a", 1_i64).check_shape().is_err());
        assert!(TypedWriteIntent::insert("t").value("a\"b", 1_i64).check_shape().is_err());
        assert!(TypedWriteIntent::upsert("t").key("id", "x").value("id", "y").check_shape().is_err());
        assert!(TypedWriteIntent::update("t")
            .key("id", "x")
            .value("n", 1_i64)
            .increment("n", 1_i64)
            .check_shape()
            .is_err());
    }

    #[test]
    fn oubli_type_forme_et_encodage() {
        let oubli = TypedWriteIntent::forget("t").key("id", "x");
        assert_eq!(oubli.operation, IntentOperation::Delete);
        assert!(oubli.erasure);
        assert!(oubli.check_shape().is_ok());
        assert!(TypedWriteIntent::forget("t").check_shape().is_err(), "clé exigée");
        assert!(TypedWriteIntent::update("t").key("id", "x").value("v", "").erasure().check_shape().is_ok());
        assert!(TypedWriteIntent::insert("t").value("a", 1_i64).erasure().check_shape().is_err());

        let json = serde_json::to_string(&oubli).expect("json");
        assert!(json.contains("\"erasure\":true"), "{json}");
        let back: TypedWriteIntent = serde_json::from_str(&json).expect("back");
        assert_eq!(back, oubli);
        // Intention ordinaire : champ absent, empreinte inchangée.
        let ordinaire = TypedWriteIntent::delete("t").key("id", "x");
        assert!(!serde_json::to_string(&ordinaire).expect("json").contains("erasure"));
        assert_ne!(ordinaire.canonical_params(), oubli.canonical_params());
        let ancien: TypedWriteIntent = serde_json::from_str(
            r#"{"intent_id":"i","operator_id":"o","database":"d","table":"t","operation":"Delete","key":{"id":{"type":"Text","value":"x"}}}"#,
        )
        .expect("ancien format");
        assert!(!ancien.erasure);
    }

    #[test]
    fn serialisation_aller_retour() {
        let intent = TypedWriteIntent::upsert("t")
            .key("id", "x")
            .value("n", 1.5_f64)
            .value("b", vec![0_u8, 255])
            .value("z", Option::<String>::None)
            .decision("sf:abc");
        let json = serde_json::to_string(&intent).expect("json");
        let back: TypedWriteIntent = serde_json::from_str(&json).expect("back");
        assert_eq!(back, intent);
    }

    #[test]
    fn migrations_contigues() {
        let m1 = SchemaMigration::new(1, "init", &["CREATE TABLE t (id TEXT)"]);
        let m2 = SchemaMigration::new(2, "idx", &["CREATE INDEX i ON t (id)"]);
        assert!(check_migrations(&[m1.clone(), m2.clone()]).is_ok());
        assert!(check_migrations(&[m2]).is_err());
        assert!(check_migrations(&[SchemaMigration::new(1, "vide", &[])]).is_err());
        let _ = m1;
    }
}
