//! @id     kindmother_heritage_import
//! @do     rejouer_base_heritee_en_intentions_typees
//! @role   donnee
//! @layer  toolkit
//! @human  Moteur d'import : upserts typés par clé primaire en lots atomiques,
//!         vérification ligne à ligne, marqueur écrit seulement après succès complet.

//! Moteur d'import d'une base héritée dans KindMother.
//!
//! Règles de correspondance (explicites, communes à tous les services) :
//! - **Identifiants conservés** : chaque ligne est rejouée en upsert sur sa clé
//!   primaire (`PlanTable::cle`), valeurs typées à l'identique (`NULL` reste `NULL`,
//!   chaîne vide reste chaîne vide).
//! - **Colonnes** : seules les colonnes présentes dans la base héritée sont écrites ;
//!   une colonne absente (base créée par une version plus ancienne) prend la valeur par
//!   défaut du schéma KindMother. Une colonne héritée inconnue de KindMother fait
//!   refuser l'intention par le service : l'import échoue (aucune perte silencieuse).
//! - **Tables** : une table du plan absente de la base héritée compte 0 ligne ; une
//!   table héritée hors du plan (et non déclarée dans `tables_ignorees`) fait échouer
//!   l'import avant toute écriture.
//! - **Clés étrangères** (KindMother active `PRAGMA foreign_keys`, les chemins hérités
//!   pas toujours) : une clé étrangère *nullable* valant `''` est normalisée en `NULL`
//!   (une chaîne vide ne peut désigner aucun parent) ; une ligne dont le parent n'existe
//!   pas dans la base héritée est **orpheline** : elle n'est pas importée (le service
//!   la refuserait), elle est comptée et citée dans le rapport et le marqueur.
//! - **Déclencheurs** : les lignes modifiées après coup par un déclencheur du schéma
//!   (ex. cumul recalculé à l'insertion d'une ligne fille) sont réécrites avec leur
//!   valeur héritée à la vérification. Les colonnes réécrites par déclencheur à chaque
//!   mise à jour (`colonnes_tolerees`, ex. `updated_at`) peuvent alors différer : écart
//!   toléré, compté dans le rapport.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

use kindmother::intent::{
    SchemaMigration, SqlValue, TypedSelect, TypedWriteIntent, MAX_INTENTS_PER_BATCH,
};
use kindmother_client::heritage::chemin_marqueur_import;
use kindmother_client::{ClientError, KindMotherClient};

use crate::lecture::{empreinte_sha256, BaseHeritee};

/// Ligne : colonne → valeur typée.
type Ligne = BTreeMap<String, SqlValue>;

/// Ligne KindMother (lecture typée).
type LigneKm = HashMap<String, serde_json::Value>;

/// Clé canonique (octets) d'une ligne, pour les ensembles et index.
type CleCanonique = Vec<u8>;

/// Taille de page des relectures KindMother.
const PAGE: u32 = 1000;

/// Passes de vérification (la première peut corriger, la dernière doit être propre).
const PASSES_VERIFICATION: usize = 3;

/// Finalité journalisée des intentions d'import.
const FINALITE: &str = "kindmother-import: reprise de la base héritée (K-13)";

/// Erreur d'import (le marqueur n'est alors **pas** écrit).
#[derive(Debug, thiserror::Error)]
pub enum ErreurImport {
    /// Plan d'import incohérent (erreur de programmation du service).
    #[error("plan d'import invalide : {0}")]
    Plan(String),
    /// Base héritée illisible ou refusée.
    #[error("base héritée : {0}")]
    Heritee(String),
    /// Le client n'est pas connecté à la base KindMother attendue.
    #[error("client KindMother connecté à la base {trouvee:?}, attendu {attendue:?}")]
    MauvaiseBase {
        /// Base attendue par le plan.
        attendue: String,
        /// Base du client.
        trouvee: String,
    },
    /// Refus ou échec KindMother (migrations, lot, lecture).
    #[error("KindMother : {0}")]
    KindMother(#[from] ClientError),
    /// Données KindMother différentes des données héritées après import.
    #[error("vérification échouée : {0}")]
    Verification(String),
    /// Le fichier hérité a changé pendant l'import.
    #[error(
        "le fichier hérité {0} a changé pendant l'import : marqueur non écrit, relancer l'import"
    )]
    ModifieePendantImport(PathBuf),
    /// Écriture du marqueur impossible.
    #[error("écriture du marqueur {chemin} impossible : {source}")]
    Marqueur {
        /// Marqueur visé.
        chemin: PathBuf,
        /// Erreur d'origine.
        #[source]
        source: std::io::Error,
    },
}

/// Clé étrangère d'une table du plan (vers la clé primaire, à une colonne, du parent).
#[derive(Debug, Clone, Copy)]
pub struct ClefEtrangere {
    /// Colonne de la table fille.
    pub colonne: &'static str,
    /// Table parente (déjà importée plus haut dans le plan, ou la table elle-même).
    pub parent: &'static str,
    /// Colonne nullable : `''` y est normalisée en `NULL`.
    pub nullable: bool,
}

impl ClefEtrangere {
    /// Clé étrangère obligatoire (`NOT NULL`).
    pub const fn requise(colonne: &'static str, parent: &'static str) -> Self {
        Self {
            colonne,
            parent,
            nullable: false,
        }
    }

    /// Clé étrangère nullable.
    pub const fn nullable(colonne: &'static str, parent: &'static str) -> Self {
        Self {
            colonne,
            parent,
            nullable: true,
        }
    }
}

/// Table à importer.
#[derive(Debug, Clone)]
pub struct PlanTable {
    /// Nom de la table (identique dans la base héritée et dans KindMother).
    pub table: &'static str,
    /// Colonnes de la clé primaire (clé de l'upsert).
    pub cle: &'static [&'static str],
    /// Clés étrangères (contrôle des orphelines).
    pub cles_etrangeres: &'static [ClefEtrangere],
    /// Colonnes réécrites par un déclencheur à chaque mise à jour : écart toléré si la
    /// ligne a dû être corrigée (voir le module).
    pub colonnes_tolerees: &'static [&'static str],
}

impl PlanTable {
    /// Table sans clé étrangère ni colonne tolérée.
    pub const fn simple(table: &'static str, cle: &'static [&'static str]) -> Self {
        Self {
            table,
            cle,
            cles_etrangeres: &[],
            colonnes_tolerees: &[],
        }
    }
}

/// Plan d'import d'un service.
#[derive(Debug, Clone)]
pub struct PlanImport {
    /// Service (rapport, marqueur).
    pub service: &'static str,
    /// Opérateur KindMother du service (justificatif `<operateur>.key`).
    pub operateur: &'static str,
    /// Base KindMother du service.
    pub base: &'static str,
    /// Migrations KindMother du service (les mêmes que celles du service).
    pub migrations: Vec<SchemaMigration>,
    /// Tables, dans l'ordre des clés étrangères (parents d'abord).
    pub tables: Vec<PlanTable>,
    /// Tables héritées volontairement non importées : `(table, motif)`.
    pub tables_ignorees: &'static [(&'static str, &'static str)],
}

impl PlanImport {
    /// Contrôle de cohérence du plan.
    fn valider(&self) -> Result<(), ErreurImport> {
        let mut vues: HashMap<&str, &PlanTable> = HashMap::new();
        for t in &self.tables {
            if t.cle.is_empty() {
                return Err(ErreurImport::Plan(format!(
                    "{} : clé primaire vide",
                    t.table
                )));
            }
            for fk in t.cles_etrangeres {
                let parent = if fk.parent == t.table {
                    t
                } else {
                    vues.get(fk.parent).copied().ok_or_else(|| {
                        ErreurImport::Plan(format!(
                            "{}.{} : parent {} absent ou placé après dans le plan",
                            t.table, fk.colonne, fk.parent
                        ))
                    })?
                };
                if parent.cle.len() != 1 {
                    return Err(ErreurImport::Plan(format!(
                        "{}.{} : le parent {} doit avoir une clé à une colonne",
                        t.table, fk.colonne, fk.parent
                    )));
                }
            }
            if vues.insert(t.table, t).is_some() {
                return Err(ErreurImport::Plan(format!("{} : table en double", t.table)));
            }
        }
        Ok(())
    }
}

/// Bilan d'une table.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RapportTable {
    /// Table.
    pub table: String,
    /// Table présente dans la base héritée.
    pub presente: bool,
    /// Lignes lues dans la base héritée.
    pub lues: usize,
    /// Lignes orphelines non importées (parent absent).
    pub orphelines: usize,
    /// Clés étrangères `''` normalisées en `NULL`.
    pub normalisees: usize,
    /// Lignes écrites à cette exécution (absentes ou différentes).
    pub ecrites: usize,
    /// Lignes déjà présentes et identiques (reprise).
    pub deja_presentes: usize,
    /// Lignes réécrites à la vérification (modifiées par un déclencheur).
    pub corrigees: usize,
    /// Lignes dont une colonne tolérée diffère de la valeur héritée.
    pub ecarts_toleres: usize,
    /// Lignes de la table KindMother après import.
    pub dans_kindmother: usize,
}

impl RapportTable {
    /// Lignes attendues dans KindMother (lues moins orphelines).
    pub fn attendues(&self) -> usize {
        self.lues - self.orphelines
    }
}

/// Bilan d'un import.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapportImport {
    /// Service.
    pub service: String,
    /// Base KindMother.
    pub base: String,
    /// Fichier hérité.
    pub base_heritee: PathBuf,
    /// Marqueur d'import.
    pub marqueur: PathBuf,
    /// Vrai si le marqueur existait déjà : rien n'a été fait.
    pub deja_importe: bool,
    /// Base héritée chiffrée (SQLCipher).
    pub chiffree: bool,
    /// Empreinte SHA-256 du fichier hérité (vide si `deja_importe`).
    pub sha256: String,
    /// Bilan par table (vide si `deja_importe`).
    pub tables: Vec<RapportTable>,
}

impl RapportImport {
    /// Total des lignes importées (attendues) toutes tables confondues.
    pub fn total_attendues(&self) -> usize {
        self.tables.iter().map(RapportTable::attendues).sum()
    }

    /// Total des lignes orphelines non importées.
    pub fn total_orphelines(&self) -> usize {
        self.tables.iter().map(|t| t.orphelines).sum()
    }
}

impl fmt::Display for RapportImport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.deja_importe {
            return writeln!(
                f,
                "{} : base héritée {} déjà importée (marqueur {}), rien à faire.",
                self.service,
                self.base_heritee.display(),
                self.marqueur.display()
            );
        }
        writeln!(
            f,
            "{} : base héritée {} importée dans la base KindMother « {} »{}.",
            self.service,
            self.base_heritee.display(),
            self.base,
            if self.chiffree { " (SQLCipher)" } else { "" }
        )?;
        writeln!(f, "  sha256 (inchangé) : {}", self.sha256)?;
        writeln!(
            f,
            "  {:<28} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
            "table", "lues", "orph.", "écrites", "déjà", "corr.", "écarts", "KM"
        )?;
        for t in &self.tables {
            writeln!(
                f,
                "  {:<28} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}{}",
                t.table,
                t.lues,
                t.orphelines,
                t.ecrites,
                t.deja_presentes,
                t.corrigees,
                t.ecarts_toleres,
                t.dans_kindmother,
                if t.presente {
                    ""
                } else {
                    "  (absente de la base héritée)"
                }
            )?;
        }
        if self.total_orphelines() > 0 {
            writeln!(
                f,
                "  ATTENTION : {} ligne(s) orpheline(s) (parent absent) non importée(s).",
                self.total_orphelines()
            )?;
        }
        writeln!(f, "  marqueur : {}", self.marqueur.display())
    }
}

/// Table héritée préparée : lignes à importer (ordre d'insertion) et bilan.
struct TablePreparee {
    plan: PlanTable,
    lignes: Vec<Ligne>,
    rapport: RapportTable,
}

/// Importe la base héritée `chemin` dans KindMother selon `plan`.
///
/// `client` doit être connecté avec le justificatif de l'opérateur du service, sur sa
/// base (`plan.base`). Le fichier hérité n'est jamais modifié ; le marqueur
/// `<chemin>.importe-kindmother` n'est écrit qu'après une vérification complète.
pub async fn importer(
    chemin: &Path,
    client: &KindMotherClient,
    plan: &PlanImport,
) -> Result<RapportImport, ErreurImport> {
    plan.valider()?;
    if client.database() != plan.base {
        return Err(ErreurImport::MauvaiseBase {
            attendue: plan.base.to_string(),
            trouvee: client.database().to_string(),
        });
    }
    let marqueur = chemin_marqueur_import(chemin);
    let mut rapport = RapportImport {
        service: plan.service.to_string(),
        base: plan.base.to_string(),
        base_heritee: chemin.to_path_buf(),
        marqueur: marqueur.clone(),
        deja_importe: false,
        chiffree: false,
        sha256: String::new(),
        tables: Vec::new(),
    };
    if marqueur.exists() {
        rapport.deja_importe = true;
        return Ok(rapport);
    }

    // 1. Instantané de la base héritée (lecture seule), encadré par son empreinte.
    rapport.sha256 = empreinte_sha256(chemin)?;
    let (mut tables, chiffree) = lire_et_preparer(chemin, plan)?;
    rapport.chiffree = chiffree;

    // 2. Schéma KindMother du service.
    client.apply_migrations(plan.migrations.clone()).await?;

    // 3. Upserts par table, parents d'abord, lots atomiques.
    for t in &mut tables {
        let existantes = lire_km(client, &t.plan).await?;
        let mut intents = Vec::new();
        for ligne in &t.lignes {
            match existantes.get(&cle_canonique(&t.plan, ligne)) {
                Some(km) if ligne_egale(ligne, km, t.plan.colonnes_tolerees) => {
                    t.rapport.deja_presentes += 1;
                }
                _ => intents.push(upsert(&t.plan, ligne)),
            }
        }
        t.rapport.ecrites = intents.len();
        soumettre(client, intents).await?;
    }

    // 4. Vérification (et correction des lignes touchées par un déclencheur).
    verifier(client, &mut tables).await?;

    // 5. Le fichier hérité doit être resté identique.
    if empreinte_sha256(chemin)? != rapport.sha256 {
        return Err(ErreurImport::ModifieePendantImport(chemin.to_path_buf()));
    }

    rapport.tables = tables.into_iter().map(|t| t.rapport).collect();
    ecrire_marqueur(&marqueur, &rapport)?;
    Ok(rapport)
}

/// Lit la base héritée et prépare les lignes (normalisation, orphelines, ordre).
fn lire_et_preparer(
    chemin: &Path,
    plan: &PlanImport,
) -> Result<(Vec<TablePreparee>, bool), ErreurImport> {
    let base = BaseHeritee::ouvrir(chemin)?;
    let heritees = base.tables()?;
    let connues: HashSet<&str> = plan
        .tables
        .iter()
        .map(|t| t.table)
        .chain(plan.tables_ignorees.iter().map(|(t, _)| *t))
        .collect();
    let inconnues: Vec<&String> = heritees
        .iter()
        .filter(|t| !connues.contains(t.as_str()))
        .collect();
    if !inconnues.is_empty() {
        return Err(ErreurImport::Heritee(format!(
            "tables héritées hors du plan d'import de {} : {inconnues:?} (aucune perte silencieuse : \
             compléter le plan ou les déclarer ignorées)",
            plan.service
        )));
    }

    // Clés importées de chaque table (pour les clés étrangères des tables suivantes).
    let mut cles_importees: HashMap<&str, HashSet<CleCanonique>> = HashMap::new();
    let mut tables = Vec::new();
    for t in &plan.tables {
        let mut rapport = RapportTable {
            table: t.table.to_string(),
            ..RapportTable::default()
        };
        let mut lignes = Vec::new();
        if heritees.iter().any(|h| h == t.table) {
            rapport.presente = true;
            let lue = base.lire_table(t.table)?;
            if let Some(c) = t.cle.iter().find(|c| !lue.colonnes.iter().any(|x| x == *c)) {
                return Err(ErreurImport::Heritee(format!(
                    "{} : colonne de clé {c} absente de la base héritée",
                    t.table
                )));
            }
            rapport.lues = lue.lignes.len();
            lignes = lue
                .lignes
                .into_iter()
                .map(|valeurs| lue.colonnes.iter().cloned().zip(valeurs).collect::<Ligne>())
                .collect();
        }
        let lignes = preparer(t, lignes, &cles_importees, &mut rapport);
        if t.cle.len() == 1 {
            cles_importees.insert(
                t.table,
                lignes.iter().map(|l| cle_canonique(t, l)).collect(),
            );
        }
        tables.push(TablePreparee {
            plan: t.clone(),
            lignes,
            rapport,
        });
    }
    Ok((tables, base.est_chiffree()))
}

/// Normalise les clés étrangères nullables `''`, écarte les orphelines et ordonne les
/// lignes d'une table auto-référencée (parents d'abord).
fn preparer(
    t: &PlanTable,
    lignes: Vec<Ligne>,
    cles_importees: &HashMap<&str, HashSet<CleCanonique>>,
    rapport: &mut RapportTable,
) -> Vec<Ligne> {
    let vide = SqlValue::Text(String::new());
    let canon = |v: &SqlValue| {
        let mut out = Vec::new();
        v.canonical_bytes(&mut out);
        out
    };
    let cle_vide = canon(&vide);

    // Clés de la table elle-même (auto-référence).
    let propres: HashSet<CleCanonique> = if t.cle.len() == 1 {
        lignes.iter().map(|l| cle_canonique(t, l)).collect()
    } else {
        HashSet::new()
    };
    let parents_de = |fk: &ClefEtrangere| -> &HashSet<CleCanonique> {
        if fk.parent == t.table {
            &propres
        } else {
            // `valider` garantit que le parent précède : ensemble présent.
            &cles_importees[fk.parent]
        }
    };

    // 1. Normalisation et orphelines vis-à-vis des autres tables.
    let mut retenues = Vec::with_capacity(lignes.len());
    for mut ligne in lignes {
        let mut orpheline = false;
        for fk in t.cles_etrangeres {
            let Some(v) = ligne.get(fk.colonne) else {
                continue;
            };
            if fk.nullable && *v == vide && !parents_de(fk).contains(&cle_vide) {
                ligne.insert(fk.colonne.to_string(), SqlValue::Null);
                rapport.normalisees += 1;
                continue;
            }
            if *v != SqlValue::Null && !parents_de(fk).contains(&canon(v)) {
                orpheline = true;
            }
        }
        if orpheline {
            rapport.orphelines += 1;
        } else {
            retenues.push(ligne);
        }
    }

    // 2. Auto-référence : parents avant enfants ; cycle ou parent écarté = orpheline.
    let auto: Vec<&ClefEtrangere> = t
        .cles_etrangeres
        .iter()
        .filter(|fk| fk.parent == t.table)
        .collect();
    if auto.is_empty() {
        return retenues;
    }
    let mut placees: HashSet<CleCanonique> = HashSet::new();
    let mut ordonnees = Vec::with_capacity(retenues.len());
    let mut restantes = retenues;
    loop {
        let avant = restantes.len();
        let mut suivantes = Vec::new();
        for ligne in restantes {
            let prete = auto.iter().all(|fk| match ligne.get(fk.colonne) {
                None | Some(SqlValue::Null) => true,
                Some(v) => {
                    let c = canon(v);
                    placees.contains(&c) || c == cle_canonique(t, &ligne)
                }
            });
            if prete {
                placees.insert(cle_canonique(t, &ligne));
                ordonnees.push(ligne);
            } else {
                suivantes.push(ligne);
            }
        }
        restantes = suivantes;
        if restantes.is_empty() || restantes.len() == avant {
            break;
        }
    }
    rapport.orphelines += restantes.len();
    ordonnees
}

/// Clé canonique d'une ligne héritée (valeurs des colonnes de clé, dans l'ordre).
fn cle_canonique(t: &PlanTable, ligne: &Ligne) -> CleCanonique {
    let mut out = Vec::new();
    for c in t.cle {
        ligne
            .get(*c)
            .unwrap_or(&SqlValue::Null)
            .canonical_bytes(&mut out);
    }
    out
}

/// Clé canonique d'une ligne KindMother.
fn cle_canonique_km(t: &PlanTable, ligne: &LigneKm) -> CleCanonique {
    let mut out = Vec::new();
    for c in t.cle {
        json_vers_sql(ligne.get(*c).unwrap_or(&serde_json::Value::Null)).canonical_bytes(&mut out);
    }
    out
}

/// Valeur JSON KindMother → valeur typée (clés : texte ou entier).
fn json_vers_sql(v: &serde_json::Value) -> SqlValue {
    match v {
        serde_json::Value::Number(n) => n.as_i64().map_or_else(
            || SqlValue::Real(n.as_f64().unwrap_or(f64::NAN)),
            SqlValue::Integer,
        ),
        serde_json::Value::String(s) => SqlValue::Text(s.clone()),
        serde_json::Value::Bool(b) => SqlValue::Integer(i64::from(*b)),
        _ => SqlValue::Null,
    }
}

/// Upsert typé d'une ligne héritée sur sa clé primaire.
fn upsert(t: &PlanTable, ligne: &Ligne) -> TypedWriteIntent {
    let mut intent = TypedWriteIntent::upsert(t.table).purpose(FINALITE);
    for (colonne, valeur) in ligne {
        if t.cle.contains(&colonne.as_str()) {
            intent = intent.key(colonne.clone(), valeur.clone());
        } else {
            intent = intent.value(colonne.clone(), valeur.clone());
        }
    }
    intent
}

/// Soumet des intentions en lots atomiques de taille maximale.
async fn soumettre(
    client: &KindMotherClient,
    mut intents: Vec<TypedWriteIntent>,
) -> Result<(), ErreurImport> {
    while !intents.is_empty() {
        let reste = intents.split_off(intents.len().min(MAX_INTENTS_PER_BATCH));
        client.submit_batch(intents).await?;
        intents = reste;
    }
    Ok(())
}

/// Lit toute une table KindMother (pages triées par clé), indexée par clé canonique.
async fn lire_km(
    client: &KindMotherClient,
    t: &PlanTable,
) -> Result<HashMap<CleCanonique, LigneKm>, ErreurImport> {
    let mut lignes = HashMap::new();
    let mut offset: u32 = 0;
    loop {
        let mut q = TypedSelect::from(t.table).limit(PAGE);
        for c in t.cle {
            q = q.order_by(*c, false);
        }
        q.offset = Some(offset);
        let page = client.select(q).await?;
        let n = page.len();
        for l in page {
            lignes.insert(cle_canonique_km(t, &l), l);
        }
        if n < PAGE as usize {
            return Ok(lignes);
        }
        offset = offset.checked_add(PAGE).ok_or_else(|| {
            ErreurImport::Verification(format!("{} : table trop grande", t.table))
        })?;
    }
}

/// Vrai si la ligne KindMother porte les valeurs héritées (hors colonnes `ignorer`).
fn ligne_egale(ligne: &Ligne, km: &LigneKm, ignorer: &[&str]) -> bool {
    ligne
        .iter()
        .filter(|(c, _)| !ignorer.contains(&c.as_str()))
        .all(|(c, v)| valeur_egale(v, km.get(c)))
}

/// Égalité d'une valeur héritée et de sa relecture KindMother (JSON).
pub(crate) fn valeur_egale(heritee: &SqlValue, km: Option<&serde_json::Value>) -> bool {
    use serde_json::Value as J;
    let Some(km) = km else {
        return false;
    };
    match (heritee, km) {
        (SqlValue::Null, J::Null) => true,
        (SqlValue::Integer(a), J::Number(n)) => match n.as_i64() {
            Some(b) => *a == b,
            None => n.as_f64().is_some_and(|b| reels_egaux(*a as f64, b)),
        },
        (SqlValue::Real(a), J::Number(n)) => n.as_f64().is_some_and(|b| reels_egaux(*a, b)),
        // KindMother relit un réel non fini en NULL.
        (SqlValue::Real(a), J::Null) => !a.is_finite(),
        (SqlValue::Text(a), J::String(b)) => a == b,
        (SqlValue::Blob(a), J::String(b)) => base64(a) == *b,
        _ => false,
    }
}

/// Égalité de réels à l'arrondi du transport JSON près.
fn reels_egaux(a: f64, b: f64) -> bool {
    a == b || (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.0)
}

/// Base64 standard avec remplissage (encodage des BLOB relus par KindMother).
fn base64(data: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(char::from(ALPHABET[(n >> 18) as usize & 63]));
        out.push(char::from(ALPHABET[(n >> 12) as usize & 63]));
        out.push(if chunk.len() > 1 {
            char::from(ALPHABET[(n >> 6) as usize & 63])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(ALPHABET[n as usize & 63])
        } else {
            '='
        });
    }
    out
}

/// Vérifie chaque table ; réécrit les lignes différentes (déclencheurs) puis revérifie.
async fn verifier(
    client: &KindMotherClient,
    tables: &mut [TablePreparee],
) -> Result<(), ErreurImport> {
    for passe in 1..=PASSES_VERIFICATION {
        let derniere = passe == PASSES_VERIFICATION;
        let mut corrections = 0;
        for t in tables.iter_mut() {
            let km = lire_km(client, &t.plan).await?;
            let mut intents = Vec::new();
            let mut ecarts_toleres = 0;
            let mut exemple = None;
            for ligne in &t.lignes {
                let cle = cle_canonique(&t.plan, ligne);
                let egale = km
                    .get(&cle)
                    .is_some_and(|l| ligne_egale(ligne, l, t.plan.colonnes_tolerees));
                if egale {
                    if !km.get(&cle).is_some_and(|l| ligne_egale(ligne, l, &[])) {
                        ecarts_toleres += 1;
                    }
                } else {
                    exemple.get_or_insert_with(|| {
                        t.plan
                            .cle
                            .iter()
                            .map(|c| format!("{c}={:?}", ligne.get(*c)))
                            .collect::<Vec<_>>()
                            .join(", ")
                    });
                    intents.push(upsert(&t.plan, ligne));
                }
            }
            if !intents.is_empty() {
                if derniere {
                    return Err(ErreurImport::Verification(format!(
                        "{} : {} ligne(s) absente(s) ou différente(s) de la base héritée (ex. {})",
                        t.plan.table,
                        intents.len(),
                        exemple.unwrap_or_default()
                    )));
                }
                corrections += intents.len();
                t.rapport.corrigees += intents.len();
                soumettre(client, intents).await?;
                continue;
            }
            t.rapport.ecarts_toleres = ecarts_toleres;
            t.rapport.dans_kindmother = km.len();
            if km.len() < t.rapport.attendues() {
                return Err(ErreurImport::Verification(format!(
                    "{} : {} ligne(s) dans KindMother pour {} attendue(s)",
                    t.plan.table,
                    km.len(),
                    t.rapport.attendues()
                )));
            }
        }
        if corrections == 0 {
            return Ok(());
        }
    }
    Ok(())
}

/// Écrit le marqueur (fichier temporaire puis renommage : jamais de marqueur partiel).
fn ecrire_marqueur(marqueur: &Path, rapport: &RapportImport) -> Result<(), ErreurImport> {
    let erreur = |source| ErreurImport::Marqueur {
        chemin: marqueur.to_path_buf(),
        source,
    };
    let mut contenu = String::new();
    contenu.push_str(
        "# Base héritée importée dans KindMother (K-13), par kindmother-import.\n\
         # Ne pas supprimer : sans ce fichier, le service refuse de démarrer sur cette base.\n\
         # Le fichier hérité n'a pas été modifié (empreinte ci-dessous, avant et après import).\n",
    );
    contenu.push_str(&format!("service: {}\n", rapport.service));
    contenu.push_str(&format!("base_kindmother: {}\n", rapport.base));
    contenu.push_str(&format!(
        "base_heritee: {}\n",
        rapport.base_heritee.display()
    ));
    contenu.push_str(&format!("date: {}\n", chrono::Utc::now().to_rfc3339()));
    contenu.push_str(&format!("sha256: {}\n", rapport.sha256));
    contenu.push_str(&format!(
        "chiffree: {}\n",
        if rapport.chiffree { "oui" } else { "non" }
    ));
    for t in &rapport.tables {
        contenu.push_str(&format!(
            "table {}: lues={} importees={} orphelines={} normalisees={} dans_kindmother={}{}\n",
            t.table,
            t.lues,
            t.attendues(),
            t.orphelines,
            t.normalisees,
            t.dans_kindmother,
            if t.presente { "" } else { " (absente)" }
        ));
    }
    let mut tmp = marqueur.as_os_str().to_os_string();
    tmp.push(format!(".tmp-{}", std::process::id()));
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, contenu).map_err(erreur)?;
    std::fs::rename(&tmp, marqueur).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        erreur(e)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_comme_le_service() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(&[0, 255, 1, 2]), "AP8BAg==");
    }

    #[test]
    fn egalite_des_valeurs() {
        use serde_json::json;
        assert!(valeur_egale(&SqlValue::Null, Some(&json!(null))));
        assert!(!valeur_egale(&SqlValue::Null, Some(&json!(""))));
        assert!(!valeur_egale(
            &SqlValue::Text(String::new()),
            Some(&json!(null))
        ));
        assert!(valeur_egale(&SqlValue::Integer(3), Some(&json!(3))));
        assert!(valeur_egale(
            &SqlValue::Real(0.1 + 0.2),
            Some(&json!(0.30000000000000004))
        ));
        assert!(!valeur_egale(&SqlValue::Real(1.0), Some(&json!(1.5))));
        assert!(valeur_egale(
            &SqlValue::Blob(vec![1, 2]),
            Some(&json!("AQI="))
        ));
        assert!(!valeur_egale(&SqlValue::Integer(1), None));
    }

    fn ligne(paires: &[(&str, SqlValue)]) -> Ligne {
        paires
            .iter()
            .map(|(c, v)| ((*c).to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn orphelines_normalisation_et_auto_reference() {
        const FK: &[ClefEtrangere] = &[
            ClefEtrangere::nullable("parent_id", "cat"),
            ClefEtrangere::requise("compte", "comptes"),
        ];
        let t = PlanTable {
            table: "cat",
            cle: &["id"],
            cles_etrangeres: FK,
            colonnes_tolerees: &[],
        };
        let txt = |s: &str| SqlValue::Text(s.to_string());
        let mut comptes = HashMap::new();
        let mut c = Vec::new();
        txt("a").canonical_bytes(&mut c);
        comptes.insert("comptes", HashSet::from([c]));
        let lignes = vec![
            ligne(&[
                ("id", txt("enfant")),
                ("parent_id", txt("racine")),
                ("compte", txt("a")),
            ]),
            ligne(&[
                ("id", txt("racine")),
                ("parent_id", txt("")),
                ("compte", txt("a")),
            ]),
            ligne(&[
                ("id", txt("perdue")),
                ("parent_id", txt("absente")),
                ("compte", txt("a")),
            ]),
            ligne(&[
                ("id", txt("sans_compte")),
                ("parent_id", SqlValue::Null),
                ("compte", txt("z")),
            ]),
            ligne(&[
                ("id", txt("c1")),
                ("parent_id", txt("c2")),
                ("compte", txt("a")),
            ]),
            ligne(&[
                ("id", txt("c2")),
                ("parent_id", txt("c1")),
                ("compte", txt("a")),
            ]),
        ];
        let mut rapport = RapportTable::default();
        let sorties = preparer(&t, lignes, &comptes, &mut rapport);
        let ids: Vec<_> = sorties.iter().map(|l| l["id"].clone()).collect();
        assert_eq!(
            ids,
            vec![txt("racine"), txt("enfant")],
            "parent avant enfant"
        );
        assert_eq!(
            sorties[0]["parent_id"],
            SqlValue::Null,
            "'' normalisée en NULL"
        );
        assert_eq!(rapport.normalisees, 1);
        // perdue (parent absent), sans_compte (compte absent), c1/c2 (cycle).
        assert_eq!(rapport.orphelines, 4);
    }

    #[test]
    fn plan_parent_apres_refuse() {
        const FK: &[ClefEtrangere] = &[ClefEtrangere::requise("p", "parent")];
        let plan = PlanImport {
            service: "s",
            operateur: "s",
            base: "s",
            migrations: Vec::new(),
            tables: vec![
                PlanTable {
                    table: "fille",
                    cle: &["id"],
                    cles_etrangeres: FK,
                    colonnes_tolerees: &[],
                },
                PlanTable::simple("parent", &["id"]),
            ],
            tables_ignorees: &[],
        };
        assert!(matches!(plan.valider(), Err(ErreurImport::Plan(_))));
    }
}
