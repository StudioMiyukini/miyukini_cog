//! Stockage en mémoire vive (tests, développement, profil sans persistance).
//!
//! Applique les **mêmes** intentions typées et lectures typées que KindMother,
//! sur des tables en mémoire : rien n'est jamais écrit sur disque. Les règles
//! suivent celles du service : colonne inconnue refusée, clé primaire unique,
//! `update` / `delete` bornés par une clé d'égalités, lot atomique (tout ou rien).

// @id service.alicia.memory.vif
// @role memory_volatile_backend
// @layer 7
// @do apply_typed_intents_in_memory
// @human Entrepôt volatile de la mémoire d'Alicia, même contrat que KindMother.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

use kindmother_client::{IntentOperation, SqlValue, TypedSelect, TypedWriteIntent};
use serde_json::Value;

use crate::entrepot::Ligne;
use crate::errors::MemoryError;
use crate::schema::{DefTable, TABLES};

type LigneVive = BTreeMap<String, SqlValue>;

/// Tables en mémoire vive.
#[derive(Default)]
pub(crate) struct EntrepotVif {
    tables: Mutex<HashMap<&'static str, Vec<LigneVive>>>,
}

fn refus(msg: impl Into<String>) -> MemoryError {
    MemoryError::KindMother(msg.into())
}

fn def(table: &str) -> Result<&'static DefTable, MemoryError> {
    TABLES
        .iter()
        .find(|t| t.nom == table)
        .ok_or_else(|| refus(format!("unknown table: {table}")))
}

fn verifier_colonnes<'a>(
    t: &DefTable,
    colonnes: impl IntoIterator<Item = &'a String>,
) -> Result<(), MemoryError> {
    for c in colonnes {
        if !t.colonnes.iter().any(|(n, _)| n == c) {
            return Err(refus(format!("unknown column {}.{c}", t.nom)));
        }
    }
    Ok(())
}

/// Ordre des valeurs (ordre de SQLite : NULL < nombres < texte < blob).
fn comparer(a: &SqlValue, b: &SqlValue) -> Ordering {
    fn rang(v: &SqlValue) -> u8 {
        match v {
            SqlValue::Null => 0,
            SqlValue::Integer(_) | SqlValue::Real(_) => 1,
            SqlValue::Text(_) => 2,
            SqlValue::Blob(_) => 3,
        }
    }
    #[allow(clippy::cast_precision_loss)]
    match (a, b) {
        (SqlValue::Integer(x), SqlValue::Integer(y)) => x.cmp(y),
        (SqlValue::Integer(x), SqlValue::Real(y)) => (*x as f64).total_cmp(y),
        (SqlValue::Real(x), SqlValue::Integer(y)) => x.total_cmp(&(*y as f64)),
        (SqlValue::Real(x), SqlValue::Real(y)) => x.total_cmp(y),
        (SqlValue::Text(x), SqlValue::Text(y)) => x.cmp(y),
        (SqlValue::Blob(x), SqlValue::Blob(y)) => x.cmp(y),
        _ => rang(a).cmp(&rang(b)),
    }
}

/// Égalité d'un filtre (`NULL` : `IS NULL`).
fn egal(v: Option<&SqlValue>, attendu: &SqlValue) -> bool {
    let v = v.unwrap_or(&SqlValue::Null);
    match attendu {
        SqlValue::Null => matches!(v, SqlValue::Null),
        _ => !matches!(v, SqlValue::Null) && comparer(v, attendu) == Ordering::Equal,
    }
}

fn correspond(l: &LigneVive, cle: &BTreeMap<String, SqlValue>) -> bool {
    cle.iter().all(|(c, v)| egal(l.get(c), v))
}

fn en_json(v: &SqlValue) -> Value {
    match v {
        SqlValue::Null => Value::Null,
        SqlValue::Integer(i) => Value::from(*i),
        SqlValue::Real(f) => serde_json::Number::from_f64(*f).map_or(Value::Null, Value::Number),
        SqlValue::Text(s) => Value::String(s.clone()),
        SqlValue::Blob(b) => Value::Array(b.iter().map(|o| Value::from(*o)).collect()),
    }
}

/// Clé primaire unique : refus si deux lignes portent la même.
fn verifier_unicite(t: &DefTable, lignes: &[LigneVive]) -> Result<(), MemoryError> {
    let mut vues: Vec<Vec<&SqlValue>> = Vec::with_capacity(lignes.len());
    for l in lignes {
        let k: Vec<&SqlValue> = t
            .cle
            .iter()
            .map(|c| l.get(*c).unwrap_or(&SqlValue::Null))
            .collect();
        if vues.iter().any(|v| {
            v.iter()
                .zip(&k)
                .all(|(a, b)| comparer(a, b) == Ordering::Equal)
        }) {
            return Err(refus(format!("UNIQUE constraint failed: {}", t.nom)));
        }
        vues.push(k);
    }
    Ok(())
}

fn inserer(
    t: &DefTable,
    lignes: &mut Vec<LigneVive>,
    valeurs: &BTreeMap<String, SqlValue>,
) -> Result<u64, MemoryError> {
    let mut l = LigneVive::new();
    for (c, d) in t.colonnes {
        let v = if let Some(v) = valeurs.get(*c) {
            v.clone()
        } else {
            d.valeur()
                .ok_or_else(|| refus(format!("NOT NULL constraint failed: {}.{c}", t.nom)))?
        };
        l.insert((*c).to_string(), v);
    }
    if let Some((c, _)) = l.iter().find(|(c, v)| {
        matches!(v, SqlValue::Null)
            && t.colonnes
                .iter()
                .any(|(n, d)| n == c && matches!(d, crate::schema::Defaut::Requise))
    }) {
        return Err(refus(format!("NOT NULL constraint failed: {}.{c}", t.nom)));
    }
    lignes.push(l);
    verifier_unicite(t, lignes)?;
    Ok(1)
}

fn appliquer(
    tables: &mut HashMap<&'static str, Vec<LigneVive>>,
    i: &TypedWriteIntent,
) -> Result<u64, MemoryError> {
    i.check_shape().map_err(refus)?;
    let t = def(&i.table)?;
    verifier_colonnes(
        t,
        i.values
            .keys()
            .chain(i.key.keys())
            .chain(i.increments.keys()),
    )?;
    if !i.increments.is_empty() {
        return Err(refus("increments are not used by the Alicia memory"));
    }
    let lignes = tables.entry(t.nom).or_default();
    match i.operation {
        IntentOperation::Insert => inserer(t, lignes, &i.values),
        IntentOperation::Update => {
            let mut n = 0_u64;
            for l in lignes.iter_mut().filter(|l| correspond(l, &i.key)) {
                for (c, v) in &i.values {
                    l.insert(c.clone(), v.clone());
                }
                n += 1;
            }
            verifier_unicite(t, lignes)?;
            Ok(n)
        }
        IntentOperation::Delete => {
            let avant = lignes.len();
            lignes.retain(|l| !correspond(l, &i.key));
            Ok((avant - lignes.len()) as u64)
        }
        IntentOperation::Upsert => {
            if let Some(l) = lignes.iter_mut().find(|l| correspond(l, &i.key)) {
                for (c, v) in &i.values {
                    l.insert(c.clone(), v.clone());
                }
                Ok(1)
            } else {
                let mut valeurs = i.values.clone();
                valeurs.extend(i.key.iter().map(|(c, v)| (c.clone(), v.clone())));
                inserer(t, lignes, &valeurs)
            }
        }
    }
}

impl EntrepotVif {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    fn verrou(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, HashMap<&'static str, Vec<LigneVive>>>, MemoryError> {
        self.tables.lock().map_err(|_| MemoryError::LockPoisoned)
    }

    /// Lecture typée (égalités, tri, décalage, limite, projection).
    pub(crate) fn select(&self, q: &TypedSelect) -> Result<Vec<Ligne>, MemoryError> {
        q.check_shape().map_err(refus)?;
        let t = def(&q.table)?;
        verifier_colonnes(
            t,
            q.columns
                .iter()
                .chain(q.filter.keys())
                .chain(q.order_by.iter().map(|o| &o.column)),
        )?;
        let tables = self.verrou()?;
        let mut lignes: Vec<&LigneVive> = tables
            .get(t.nom)
            .map(|v| v.iter().filter(|l| correspond(l, &q.filter)).collect())
            .unwrap_or_default();
        lignes.sort_by(|a, b| {
            for o in &q.order_by {
                let ord = comparer(
                    a.get(&o.column).unwrap_or(&SqlValue::Null),
                    b.get(&o.column).unwrap_or(&SqlValue::Null),
                );
                let ord = if o.descending { ord.reverse() } else { ord };
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            Ordering::Equal
        });
        let debut = q.offset.map_or(0, |o| o as usize);
        let fin = q
            .limit
            .map_or(usize::MAX, |l| debut.saturating_add(l as usize));
        Ok(lignes
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i >= debut && *i < fin)
            .map(|(_, l)| {
                l.iter()
                    .filter(|(c, _)| q.columns.is_empty() || q.columns.contains(c))
                    .map(|(c, v)| (c.clone(), en_json(v)))
                    .collect()
            })
            .collect())
    }

    /// Lot atomique : appliqué sur une copie, remplacée seulement si tout réussit.
    pub(crate) fn soumettre(&self, intents: &[TypedWriteIntent]) -> Result<Vec<u64>, MemoryError> {
        let mut tables = self.verrou()?;
        let mut copie = tables.clone();
        let mut lignes = Vec::with_capacity(intents.len());
        for i in intents {
            lignes.push(appliquer(&mut copie, i)?);
        }
        *tables = copie;
        Ok(lignes)
    }
}
