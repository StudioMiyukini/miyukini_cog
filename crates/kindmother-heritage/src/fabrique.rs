//! @id     kindmother_heritage_fabrique
//! @do     fabriquer_base_heritee_de_test
//! @role   test
//! @layer  toolkit
//! @human  Fabrique de bases SQLite « héritées » pour les tests d'import des services
//!         (feature `fabrique-test`, jamais utilisée en production).

//! Fabrique de bases héritées (tests).
//!
//! Les services n'embarquent plus `rusqlite` sur leur chemin par défaut : leurs tests
//! d'import construisent la base héritée avec le DDL hérité brut et des lignes typées.

use std::path::Path;

use kindmother_client::{KindMotherClient, TypedSelect};

use kindmother::intent::{is_valid_identifier, SqlValue};
use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection};

use crate::PlanImport;

/// Crée la base `chemin` : `ddl` puis insertion de `lignes` (`(table, [(colonne, valeur)])`).
///
/// # Errors
/// Erreur SQLite ou identifiant invalide (message).
pub fn creer_base_heritee(
    chemin: &Path,
    ddl: &str,
    lignes: &[(&str, Vec<(&str, SqlValue)>)],
) -> Result<(), String> {
    let conn = Connection::open(chemin).map_err(|e| e.to_string())?;
    // Le SQLite embarqué de rusqlite applique les clés étrangères par défaut ; une base
    // héritée peut pourtant contenir des orphelines (autre build, autre version) : la
    // fabrique les autorise pour pouvoir les tester.
    conn.pragma_update(None, "foreign_keys", false)
        .map_err(|e| e.to_string())?;
    conn.execute_batch(ddl).map_err(|e| e.to_string())?;
    for (table, valeurs) in lignes {
        if !is_valid_identifier(table) || valeurs.iter().any(|(c, _)| !is_valid_identifier(c)) {
            return Err(format!("identifiant invalide dans {table}"));
        }
        let colonnes: Vec<String> = valeurs.iter().map(|(c, _)| format!("\"{c}\"")).collect();
        let marques: Vec<String> = (1..=valeurs.len()).map(|i| format!("?{i}")).collect();
        let sql = format!(
            "INSERT INTO \"{table}\" ({}) VALUES ({})",
            colonnes.join(", "),
            marques.join(", ")
        );
        let params = valeurs.iter().map(|(_, v)| match v {
            SqlValue::Null => Value::Null,
            SqlValue::Integer(i) => Value::Integer(*i),
            SqlValue::Real(f) => Value::Real(*f),
            SqlValue::Text(s) => Value::Text(s.clone()),
            SqlValue::Blob(b) => Value::Blob(b.clone()),
        });
        conn.execute(&sql, params_from_iter(params))
            .map_err(|e| format!("{table} : {e}"))?;
    }
    Ok(())
}

/// Vérifie que chaque ligne de `lignes` existe dans KindMother avec les mêmes valeurs
/// (clé : celle du plan ; colonnes `(table, colonne)` de `ignorer` non comparées).
/// Retourne le nombre de lignes vérifiées.
///
/// # Errors
/// Première différence trouvée (message).
pub async fn verifier_dans_kindmother(
    client: &KindMotherClient,
    plan: &PlanImport,
    lignes: &[(&str, Vec<(&str, SqlValue)>)],
    ignorer: &[(&str, &str)],
) -> Result<usize, String> {
    for (table, valeurs) in lignes {
        let t = plan
            .tables
            .iter()
            .find(|t| t.table == *table)
            .ok_or_else(|| format!("{table} hors plan"))?;
        let mut q = TypedSelect::from(*table);
        for c in t.cle {
            let v = valeurs
                .iter()
                .find(|(n, _)| n == c)
                .ok_or_else(|| format!("{table} : clé {c} absente de la ligne"))?;
            q = q.filter(*c, v.1.clone());
        }
        let trouvees = client.select(q).await.map_err(|e| e.to_string())?;
        let [km] = trouvees.as_slice() else {
            return Err(format!(
                "{table} {valeurs:?} : {} ligne(s) trouvée(s)",
                trouvees.len()
            ));
        };
        for (c, v) in valeurs {
            if ignorer.contains(&(*table, *c)) {
                continue;
            }
            if !crate::import::valeur_egale(v, km.get(*c)) {
                return Err(format!(
                    "{table}.{c} : hérité {v:?}, KindMother {:?}",
                    km.get(*c)
                ));
            }
        }
    }
    Ok(lignes.len())
}
