//! @id     kindmother_service_identite
//! @do     identifier_lignes_normalisees_par_la_base
//! @role   securite
//! @layer  core
//! @human  kindmother-service — identité d'une ligne normalisée par SQLite (clé primaire ou clé unique déclarée, classe de stockage), pour rejouer les oublis sans ambiguïté (A-025).

//! Identité normalisée d'une ligne (A-025, rejeu des oublis fail-closed).
//!
//! Une intention d'oubli désigne ses lignes par une clé **telle que l'opérateur l'a
//! écrite** (`'5'`, `5`, `5.0`…), que SQLite compare selon l'affinité de la colonne.
//! Retrouver ces lignes dans une sauvegarde en hachant la clé de l'intention dépend donc
//! de cette normalisation : une différence (affinité, type) et la ligne oubliée revient.
//!
//! L'identité ci-dessous est celle **que la base elle-même a stockée** :
//!
//! - **colonnes d'identité** : la clé primaire déclarée (ordre de déclaration de la clé),
//!   à défaut la plus petite clé `UNIQUE` déclarée non partielle dont toutes les colonnes
//!   sont `NOT NULL` (à nombre égal, nom d'index le plus petit) ; à défaut, aucune : un
//!   oubli typé sur la table est **refusé** ;
//! - **valeurs** : relues par `SELECT` dans la transaction de l'oubli, **avant** le
//!   `DELETE`/la rédaction, avec leur **classe de stockage** (`typeof` : `integer`,
//!   `real`, `text`, `blob` ; `null` refusé) ;
//! - **empreinte** : SHA-256 de `kindmother/identite/v1 ‖ table (minuscules) ‖ n ‖
//!   (colonne (minuscules) ‖ étiquette de type ‖ octets)…`, chaque champ préfixé de sa
//!   longueur ; entier : 8 octets gros-boutiens ; réel : bits IEEE-754 gros-boutiens
//!   (`-0.0` ramené à `0.0`) ; texte : octets UTF-8 tels que rendus par SQLite ; blob :
//!   octets.
//!
//! Deux lignes ont la même identité si et seulement si SQLite a stocké les mêmes valeurs
//! de même classe dans les mêmes colonnes d'identité : aucune variante n'est devinée.

use std::collections::BTreeSet;

use kindmother::intent::push_len_prefixed;
use rusqlite::types::{Value, ValueRef};
use rusqlite::Connection;

use crate::audit::sha256_hex;

/// Domaine de l'empreinte d'identité d'une ligne.
const DOMAINE_IDENTITE: &[u8] = b"kindmother/identite/v1";

/// Domaine de l'empreinte de l'ensemble des créations d'une intention.
const DOMAINE_CREATIONS: &[u8] = b"kindmother/creations/v1";

/// Colonnes d'identité d'une table et leurs types déclarés.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ColonnesIdentite {
    /// Noms des colonnes, tels que déclarés, dans l'ordre de la clé.
    pub colonnes: Vec<String>,
    /// Types déclarés (majuscules, sans espaces de bord), même ordre.
    pub types: Vec<String>,
}

fn quote(ident: &str) -> String {
    format!("\"{}\"", ident.replace('"', "\"\""))
}

/// Colonne de `pragma_table_info`.
struct InfoColonne {
    nom: String,
    type_declare: String,
    notnull: bool,
    pk: i64,
}

fn info_colonnes(conn: &Connection, table: &str) -> Result<Vec<InfoColonne>, String> {
    let mut stmt = conn
        .prepare("SELECT name, type, \"notnull\", pk FROM pragma_table_info(?1)")
        .map_err(|e| format!("schema lookup on {table} failed: {e}"))?;
    let lignes = stmt
        .query_map([table], |r| {
            Ok(InfoColonne {
                nom: r.get(0)?,
                type_declare: r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                notnull: r.get::<_, i64>(2)? != 0,
                pk: r.get(3)?,
            })
        })
        .map_err(|e| format!("schema lookup on {table} failed: {e}"))?;
    lignes
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("schema lookup on {table} failed: {e}"))
}

fn type_normalise(t: &str) -> String {
    t.trim().to_ascii_uppercase()
}

/// Types déclarés, normalisés, des colonnes `colonnes` de `table` (`None` pour une
/// colonne absente).
pub(crate) fn types_declares(conn: &Connection, table: &str, colonnes: &[String]) -> Result<Vec<Option<String>>, String> {
    let info = info_colonnes(conn, table)?;
    Ok(colonnes
        .iter()
        .map(|c| {
            info.iter()
                .find(|i| i.nom.eq_ignore_ascii_case(c))
                .map(|i| type_normalise(&i.type_declare))
        })
        .collect())
}

/// Colonnes d'identité de `table` (voir le module) ; `None` : ni clé primaire, ni clé
/// unique déclarée utilisable.
pub(crate) fn colonnes_identite(conn: &Connection, table: &str) -> Result<Option<ColonnesIdentite>, String> {
    let info = info_colonnes(conn, table)?;
    let mut pk: Vec<&InfoColonne> = info.iter().filter(|c| c.pk > 0).collect();
    if !pk.is_empty() {
        pk.sort_by_key(|c| c.pk);
        return Ok(Some(ColonnesIdentite {
            colonnes: pk.iter().map(|c| c.nom.clone()).collect(),
            types: pk.iter().map(|c| type_normalise(&c.type_declare)).collect(),
        }));
    }
    // Clés uniques déclarées, non partielles, sur colonnes NOT NULL (pas d'expression).
    let mut index: Vec<String> = {
        let mut stmt = conn
            .prepare("SELECT name FROM pragma_index_list(?1) WHERE \"unique\" = 1 AND partial = 0")
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?;
        let lignes = stmt
            .query_map([table], |r| r.get::<_, String>(0))
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?;
        lignes
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?
    };
    index.sort();
    let mut meilleur: Option<ColonnesIdentite> = None;
    for nom in &index {
        let mut stmt = conn
            .prepare("SELECT cid, name FROM pragma_index_info(?1) ORDER BY seqno")
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?;
        let cols = stmt
            .query_map([nom], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Option<String>>(1)?)))
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("index lookup on {table} failed: {e}"))?;
        let mut colonnes = Vec::with_capacity(cols.len());
        let mut types = Vec::with_capacity(cols.len());
        let mut utilisable = !cols.is_empty();
        for (cid, nom_col) in cols {
            let Some(c) = nom_col.filter(|_| cid >= 0).and_then(|n| info.iter().find(|i| i.nom == n)) else {
                utilisable = false;
                break;
            };
            if !c.notnull {
                utilisable = false;
                break;
            }
            colonnes.push(c.nom.clone());
            types.push(type_normalise(&c.type_declare));
        }
        if utilisable && meilleur.as_ref().is_none_or(|m| colonnes.len() < m.colonnes.len()) {
            meilleur = Some(ColonnesIdentite { colonnes, types });
        }
    }
    Ok(meilleur)
}

/// Encodage canonique typé d'une valeur stockée.
fn encoder(buf: &mut Vec<u8>, v: ValueRef<'_>) {
    match v {
        ValueRef::Null => buf.push(b'n'),
        ValueRef::Integer(i) => {
            buf.push(b'i');
            buf.extend_from_slice(&i.to_be_bytes());
        }
        ValueRef::Real(f) => {
            buf.push(b'r');
            let f = if f == 0.0 { 0.0 } else { f };
            buf.extend_from_slice(&f.to_bits().to_be_bytes());
        }
        ValueRef::Text(t) => {
            buf.push(b't');
            push_len_prefixed(buf, t);
        }
        ValueRef::Blob(b) => {
            buf.push(b'b');
            push_len_prefixed(buf, b);
        }
    }
}

/// Empreinte d'identité d'une ligne (voir le module).
pub(crate) fn empreinte_identite(table: &str, colonnes: &[String], valeurs: &[ValueRef<'_>]) -> String {
    let mut buf = Vec::with_capacity(64);
    push_len_prefixed(&mut buf, DOMAINE_IDENTITE);
    push_len_prefixed(&mut buf, table.to_ascii_lowercase().as_bytes());
    buf.extend_from_slice(&(colonnes.len() as u64).to_be_bytes());
    for (colonne, valeur) in colonnes.iter().zip(valeurs) {
        push_len_prefixed(&mut buf, colonne.to_ascii_lowercase().as_bytes());
        encoder(&mut buf, *valeur);
    }
    sha256_hex(&buf)
}

/// Ligne identifiée : empreinte, et valeurs stockées des colonnes d'identité (pour la
/// désigner exactement), `None` si une valeur n'est pas représentable (texte non UTF-8).
pub(crate) struct LigneIdentifiee {
    pub identite: String,
    pub valeurs: Option<Vec<Value>>,
    pub contient_null: bool,
}

/// Identités des lignes de `table` choisies par `filtre` (clause `WHERE` déjà construite
/// à partir d'identifiants vérifiés, paramètres liés ; vide : toutes les lignes).
pub(crate) fn identites(
    conn: &Connection,
    table: &str,
    colonnes: &[String],
    filtre: &str,
    params: &[Value],
) -> Result<Vec<LigneIdentifiee>, String> {
    let liste: Vec<String> = colonnes.iter().map(|c| quote(c)).collect();
    let mut sql = format!("SELECT {} FROM {}", liste.join(", "), quote(table));
    if !filtre.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(filtre);
    }
    let mut stmt = conn.prepare(&sql).map_err(|e| format!("identity scan on {table} failed: {e}"))?;
    let n = colonnes.len();
    let mut lignes = stmt
        .query(rusqlite::params_from_iter(params.iter()))
        .map_err(|e| format!("identity scan on {table} failed: {e}"))?;
    let mut out = Vec::new();
    while let Some(ligne) = lignes.next().map_err(|e| format!("identity scan on {table} failed: {e}"))? {
        let mut refs = Vec::with_capacity(n);
        for i in 0..n {
            refs.push(ligne.get_ref(i).map_err(|e| format!("identity scan on {table} failed: {e}"))?);
        }
        let contient_null = refs.iter().any(|v| matches!(v, ValueRef::Null));
        let valeurs = refs
            .iter()
            .map(|v| match *v {
                ValueRef::Null => Some(Value::Null),
                ValueRef::Integer(i) => Some(Value::Integer(i)),
                ValueRef::Real(f) => Some(Value::Real(f)),
                ValueRef::Text(t) => std::str::from_utf8(t).ok().map(|s| Value::Text(s.to_string())),
                ValueRef::Blob(b) => Some(Value::Blob(b.to_vec())),
            })
            .collect::<Option<Vec<_>>>();
        out.push(LigneIdentifiee {
            identite: empreinte_identite(table, colonnes, &refs),
            valeurs,
            contient_null,
        });
    }
    Ok(out)
}

/// Identité de la ligne qu'un `INSERT` typé vient de créer (même transaction) : relue par
/// `_rowid_` (`rowid` = `last_insert_rowid()`), ou, pour une table `WITHOUT ROWID`, par
/// les valeurs d'identité de l'intention (clé unique : une ligne au plus). `None` si elle
/// n'est pas déterminable avec certitude (aucune création n'est alors inscrite).
pub(crate) fn identite_inseree(
    conn: &Connection,
    table: &str,
    cols: &ColonnesIdentite,
    valeurs: &std::collections::BTreeMap<String, kindmother::intent::SqlValue>,
    rowid: i64,
) -> Option<String> {
    let unique = |lignes: Vec<LigneIdentifiee>| match lignes.as_slice() {
        [l] if !l.contient_null => Some(l.identite.clone()),
        _ => None,
    };
    let ombre = info_colonnes(conn, table)
        .ok()?
        .iter()
        .any(|c| c.nom.eq_ignore_ascii_case("_rowid_"));
    if !ombre {
        // Échec de préparation : table `WITHOUT ROWID` (pas de `_rowid_`).
        if let Ok(lignes) = identites(conn, table, &cols.colonnes, "_rowid_ = ?1", &[Value::Integer(rowid)]) {
            return unique(lignes);
        }
    }
    let mut filtre = Vec::with_capacity(cols.colonnes.len());
    let mut params = Vec::with_capacity(cols.colonnes.len());
    for colonne in &cols.colonnes {
        let (_, v) = valeurs.iter().find(|(k, _)| k.eq_ignore_ascii_case(colonne))?;
        params.push(crate::typed::to_value(v));
        filtre.push(format!("{} = ?{}", quote(colonne), params.len()));
    }
    unique(identites(conn, table, &cols.colonnes, &filtre.join(" AND "), &params).ok()?)
}

/// Empreinte de l'ensemble des identités créées par une intention (portée par le détail
/// de son entrée chaînée : la chaîne couvre le registre des créations).
pub(crate) fn empreinte_creations(database: &str, table: &str, identites: &BTreeSet<String>) -> String {
    let mut buf = Vec::with_capacity(64 + identites.len() * 72);
    push_len_prefixed(&mut buf, DOMAINE_CREATIONS);
    push_len_prefixed(&mut buf, database.as_bytes());
    push_len_prefixed(&mut buf, table.to_ascii_lowercase().as_bytes());
    buf.extend_from_slice(&(identites.len() as u64).to_be_bytes());
    for i in identites {
        push_len_prefixed(&mut buf, i.as_bytes());
    }
    sha256_hex(&buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conn(sql: &str) -> Connection {
        let c = Connection::open_in_memory().expect("mem");
        c.execute_batch(sql).expect("schema");
        c
    }

    #[test]
    fn colonnes_cle_primaire_puis_unique_sinon_aucune() {
        let c = conn(
            "CREATE TABLE a (x TEXT, y INTEGER, v TEXT, PRIMARY KEY (y, x));
             CREATE TABLE b (v TEXT, code text NOT NULL UNIQUE, autre TEXT UNIQUE, n INTEGER NOT NULL, m INTEGER NOT NULL,
                             UNIQUE (n, m));
             CREATE TABLE c (v TEXT, w TEXT UNIQUE);
             CREATE TABLE d (v TEXT);",
        );
        let a = colonnes_identite(&c, "a").expect("a").expect("pk");
        assert_eq!(a.colonnes, vec!["y", "x"]);
        assert_eq!(a.types, vec!["INTEGER", "TEXT"]);
        let b = colonnes_identite(&c, "b").expect("b").expect("unique");
        assert_eq!(b.colonnes, vec!["code"]);
        assert_eq!(b.types, vec!["TEXT"]);
        assert!(colonnes_identite(&c, "c").expect("c").is_none(), "UNIQUE sur colonne nullable");
        assert!(colonnes_identite(&c, "d").expect("d").is_none());
    }

    #[test]
    fn identite_suit_la_classe_de_stockage() {
        let c = conn(
            "CREATE TABLE t (id TEXT PRIMARY KEY, n INTEGER);
             CREATE TABLE u (id PRIMARY KEY);
             INSERT INTO t VALUES (5, '7');
             INSERT INTO u VALUES (5), ('5');",
        );
        let cols = vec!["id".to_string()];
        // Affinité TEXT : 5 est stocké '5'.
        let t = identites(&c, "t", &cols, "", &[]).expect("t");
        assert_eq!(t[0].identite, empreinte_identite("T", &cols, &[ValueRef::Text(b"5")]));
        // Sans affinité : 5 et '5' sont deux identités distinctes.
        let u = identites(&c, "u", &cols, "", &[]).expect("u");
        assert_eq!(u.len(), 2);
        assert_ne!(u[0].identite, u[1].identite);
        assert_ne!(
            empreinte_identite("t", &cols, &[ValueRef::Real(0.0)]),
            empreinte_identite("t", &cols, &[ValueRef::Integer(0)])
        );
        assert_eq!(
            empreinte_identite("t", &cols, &[ValueRef::Real(-0.0)]),
            empreinte_identite("t", &cols, &[ValueRef::Real(0.0)])
        );
    }
}
