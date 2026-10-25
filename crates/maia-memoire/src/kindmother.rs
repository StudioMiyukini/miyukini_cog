//! @id     maia_memoire_kindmother
//! @do     stocker_memoire_par_kindmother
//! @role   donnee
//! @layer  8
//! @human  maia-memoire — entrepôt KindMother : une base chiffrée par utilisateur `maia-<utilisateur>`, intentions typées, étiquette `nationale` (§6, §10, EX-55, K-13).

//! KindMother est le seul écrivain (K-13) : schéma par migrations versionnées
//! non destructives, écritures par **intentions typées** (aucun texte SQL),
//! lectures typées. Chaque utilisateur a son opérateur et sa base
//! `maia-<utilisateur>` (clé SQLCipher propre, accès cloisonné par l'arbitrage
//! de KindMother) ; un opérateur MAIA ne peut pas ouvrir la base d'un autre.
//!
//! À l'ouverture : migrations, pose de l'étiquette `nationale` (jamais
//! remplacée), puis contrôle de l'étiquette et du chiffrement — refus sinon.

use std::collections::HashMap;
use std::path::Path;

use chrono::{DateTime, Utc};
use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, SchemaMigration, TypedSelect,
    TypedWriteIntent,
};
use serde_json::Value;
use uuid::Uuid;

use crate::entrepot::{Contenu, Ecriture, Entrepot};
use crate::erreurs::ErreurMemoire;
use crate::ident::{Branche, IdMemoire, TypeUnite};
use crate::types::{Auteur, Candidat, EmpreinteSource, Note, PierreTombale, Provenance, ETIQUETTE};

/// Préfixe des opérateurs et bases MAIA dans KindMother.
pub const PREFIXE_BASE: &str = "maia-";
/// Longueur maximale d'un identifiant d'utilisateur.
pub const MAX_UTILISATEUR: usize = 48;
/// Intentions au plus par lot (borne KindMother : 1 000).
const MAX_LOT: usize = 900;

/// Nom de l'opérateur et de la base KindMother d'un utilisateur.
pub fn nom_base(utilisateur: &str) -> Result<String, ErreurMemoire> {
    let mut cs = utilisateur.chars();
    let valide = matches!(cs.next(), Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit())
        && utilisateur.len() <= MAX_UTILISATEUR
        && cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    if valide {
        Ok(format!("{PREFIXE_BASE}{utilisateur}"))
    } else {
        Err(ErreurMemoire::IdInvalide(format!(
            "utilisateur {:?}",
            utilisateur.chars().take(60).collect::<String>()
        )))
    }
}

fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "mémoire MAIA v1 : versions (ajout seul), pierres tombales, candidats, étiquette",
        &[
            "CREATE TABLE IF NOT EXISTS versions (
                id TEXT NOT NULL, version INTEGER NOT NULL, texte TEXT NOT NULL,
                importance INTEGER NOT NULL, confiance INTEGER NOT NULL,
                provenance TEXT NOT NULL, auteur TEXT NOT NULL,
                liens TEXT NOT NULL, entites TEXT NOT NULL, sources TEXT NOT NULL,
                cree_le TEXT NOT NULL, enregistre_le TEXT NOT NULL, dernier_usage TEXT,
                PRIMARY KEY (id, version))",
            "CREATE TABLE IF NOT EXISTS tombes (
                id TEXT PRIMARY KEY, oublie_le TEXT NOT NULL, motif TEXT NOT NULL,
                versions_effacees INTEGER NOT NULL)",
            "CREATE TABLE IF NOT EXISTS candidats (
                id TEXT PRIMARY KEY, cible TEXT, branche TEXT NOT NULL, type_unite TEXT NOT NULL,
                texte TEXT NOT NULL, importance INTEGER NOT NULL, confiance INTEGER NOT NULL,
                provenance TEXT NOT NULL, liens TEXT NOT NULL, sources TEXT NOT NULL,
                sensible INTEGER NOT NULL, propose_le TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS meta (cle TEXT PRIMARY KEY, valeur TEXT NOT NULL)",
        ],
    )]
}

fn err_client(e: &ClientError) -> ErreurMemoire {
    ErreurMemoire::Entrepot(e.to_string())
}

/// Entrepôt KindMother d'un utilisateur.
pub struct EntrepotKindMother {
    client: KindMotherClient,
    base: String,
}

impl EntrepotKindMother {
    /// Connexion authentifiée (justificatif `maia-<utilisateur>` et pin du serveur
    /// dans `credentials` ; à défaut, emplacement standard de KindMother).
    pub async fn connecter(
        adresse: &str,
        utilisateur: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, ErreurMemoire> {
        let base = nom_base(utilisateur)?;
        let cred = match credentials {
            Some(dir) => OperatorCredential::load_from_dir(&base, dir),
            None => OperatorCredential::load(&base),
        }
        .map_err(|e| err_client(&e))?;
        let client = KindMotherClient::connect(adresse, &cred, &base)
            .await
            .map_err(|e| err_client(&e))?;
        Ok(Self { client, base })
    }

    /// Nom de la base.
    pub fn base(&self) -> &str {
        &self.base
    }

    async fn lignes(&self, table: &str) -> Result<Vec<HashMap<String, Value>>, ErreurMemoire> {
        self.client
            .select(TypedSelect::from(table))
            .await
            .map_err(|e| err_client(&e))
    }
}

fn texte(l: &HashMap<String, Value>, c: &str) -> Result<String, ErreurMemoire> {
    l.get(c)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| ErreurMemoire::Entrepot(format!("colonne {c} illisible")))
}

fn entier(l: &HashMap<String, Value>, c: &str) -> Result<i64, ErreurMemoire> {
    l.get(c)
        .and_then(Value::as_i64)
        .ok_or_else(|| ErreurMemoire::Entrepot(format!("colonne {c} illisible")))
}

fn echelle(l: &HashMap<String, Value>, c: &str) -> Result<u8, ErreurMemoire> {
    u8::try_from(entier(l, c)?)
        .ok()
        .filter(|v| *v <= 9)
        .ok_or_else(|| ErreurMemoire::Entrepot(format!("colonne {c} hors échelle")))
}

fn date(l: &HashMap<String, Value>, c: &str) -> Result<DateTime<Utc>, ErreurMemoire> {
    DateTime::parse_from_rfc3339(&texte(l, c)?)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| ErreurMemoire::Entrepot(format!("colonne {c} : date illisible")))
}

fn json<T: serde::de::DeserializeOwned>(l: &HashMap<String, Value>, c: &str) -> Result<T, ErreurMemoire> {
    serde_json::from_str(&texte(l, c)?).map_err(|_| ErreurMemoire::Entrepot(format!("colonne {c} illisible")))
}

fn provenance(l: &HashMap<String, Value>) -> Result<Provenance, ErreurMemoire> {
    Provenance::depuis_str(&texte(l, "provenance")?)
        .ok_or_else(|| ErreurMemoire::Entrepot("provenance inconnue".to_string()))
}

fn liste<T: serde::Serialize>(v: &[T]) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string())
}

fn type_unite(s: &str) -> Option<TypeUnite> {
    TypeUnite::TOUS
        .into_iter()
        .find(|t| serde_json::to_value(t).ok().and_then(|v| v.as_str().map(|x| x == s)) == Some(true))
}

fn nom_type(t: TypeUnite) -> String {
    serde_json::to_value(t)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

fn note_de(l: &HashMap<String, Value>) -> Result<Note, ErreurMemoire> {
    Ok(Note {
        id: IdMemoire::parse(&texte(l, "id")?)?,
        version: u32::try_from(entier(l, "version")?)
            .map_err(|_| ErreurMemoire::Entrepot("version illisible".to_string()))?,
        texte: texte(l, "texte")?,
        importance: echelle(l, "importance")?,
        confiance: echelle(l, "confiance")?,
        provenance: provenance(l)?,
        auteur: Auteur::depuis_str(&texte(l, "auteur")?)
            .ok_or_else(|| ErreurMemoire::Entrepot("auteur inconnu".to_string()))?,
        liens: json(l, "liens")?,
        entites: json(l, "entites")?,
        sources: json::<Vec<EmpreinteSource>>(l, "sources")?,
        cree_le: date(l, "cree_le")?,
        enregistre_le: date(l, "enregistre_le")?,
        dernier_usage: match l.get("dernier_usage") {
            None | Some(Value::Null) => None,
            Some(_) => Some(date(l, "dernier_usage")?),
        },
    })
}

fn candidat_de(l: &HashMap<String, Value>) -> Result<Candidat, ErreurMemoire> {
    Ok(Candidat {
        id: Uuid::parse_str(&texte(l, "id")?).map_err(|_| ErreurMemoire::Entrepot("candidat illisible".to_string()))?,
        cible: match l.get("cible") {
            None | Some(Value::Null) => None,
            Some(_) => Some(IdMemoire::parse(&texte(l, "cible")?)?),
        },
        branche: Branche::parse(&texte(l, "branche")?)?,
        type_unite: type_unite(&texte(l, "type_unite")?)
            .ok_or_else(|| ErreurMemoire::Entrepot("type inconnu".to_string()))?,
        texte: texte(l, "texte")?,
        importance: echelle(l, "importance")?,
        confiance: echelle(l, "confiance")?,
        provenance: provenance(l)?,
        liens: json(l, "liens")?,
        sources: json(l, "sources")?,
        sensible: entier(l, "sensible")? != 0,
        propose_le: date(l, "propose_le")?,
    })
}

fn tombe_de(l: &HashMap<String, Value>) -> Result<PierreTombale, ErreurMemoire> {
    Ok(PierreTombale {
        id: IdMemoire::parse(&texte(l, "id")?)?,
        oublie_le: date(l, "oublie_le")?,
        motif: texte(l, "motif")?,
        versions_effacees: u32::try_from(entier(l, "versions_effacees")?)
            .map_err(|_| ErreurMemoire::Entrepot("tombe illisible".to_string()))?,
    })
}

fn intention(e: &Ecriture) -> Vec<TypedWriteIntent> {
    match e {
        Ecriture::AjouterVersion(n) => vec![TypedWriteIntent::insert("versions")
            .value("id", n.id.as_str())
            .value("version", n.version)
            .value("texte", n.texte.as_str())
            .value("importance", i64::from(n.importance))
            .value("confiance", i64::from(n.confiance))
            .value("provenance", n.provenance.as_str())
            .value("auteur", n.auteur.as_str())
            .value("liens", liste(&n.liens))
            .value("entites", liste(&n.entites))
            .value("sources", liste(&n.sources))
            .value("cree_le", n.cree_le.to_rfc3339())
            .value("enregistre_le", n.enregistre_le.to_rfc3339())
            .value("dernier_usage", n.dernier_usage.map(|d| d.to_rfc3339()))
            .purpose("memoire_ajouter_version")],
        Ecriture::AjouterCandidat(k) => vec![TypedWriteIntent::insert("candidats")
            .value("id", k.id.to_string())
            .value("cible", k.cible.as_ref().map(|c| c.as_str().to_string()))
            .value("branche", k.branche.as_str())
            .value("type_unite", nom_type(k.type_unite))
            .value("texte", k.texte.as_str())
            .value("importance", i64::from(k.importance))
            .value("confiance", i64::from(k.confiance))
            .value("provenance", k.provenance.as_str())
            .value("liens", liste(&k.liens))
            .value("sources", liste(&k.sources))
            .value("sensible", k.sensible)
            .value("propose_le", k.propose_le.to_rfc3339())
            .purpose("memoire_proposer")],
        Ecriture::RetirerCandidat(id) => vec![TypedWriteIntent::delete("candidats")
            .key("id", id.to_string())
            .purpose("memoire_retirer_candidat")],
        // Oubli : effacement de toutes les versions et pierre tombale dans le même lot.
        Ecriture::Oublier(t) => vec![
            TypedWriteIntent::delete("versions")
                .key("id", t.id.as_str())
                .purpose("memoire_oubli_demande"),
            TypedWriteIntent::insert("tombes")
                .value("id", t.id.as_str())
                .value("oublie_le", t.oublie_le.to_rfc3339())
                .value("motif", t.motif.as_str())
                .value("versions_effacees", t.versions_effacees)
                .purpose("memoire_pierre_tombale"),
        ],
    }
}

impl Entrepot for EntrepotKindMother {
    async fn preparer(&self) -> Result<(), ErreurMemoire> {
        self.client
            .apply_migrations(migrations())
            .await
            .map_err(|e| err_client(&e))?;
        // Étiquette posée à la création, jamais remplacée ; toute autre valeur : refus.
        let meta = self.lignes("meta").await?;
        let etiquette = meta
            .iter()
            .find(|l| l.get("cle").and_then(Value::as_str) == Some("etiquette"));
        match etiquette {
            None => {
                self.client
                    .submit(
                        TypedWriteIntent::insert("meta")
                            .value("cle", "etiquette")
                            .value("valeur", ETIQUETTE.as_str())
                            .purpose("memoire_etiquette"),
                    )
                    .await
                    .map_err(|e| err_client(&e))?;
            }
            Some(l) => match l.get("valeur").and_then(Value::as_str) {
                Some(v) if v == ETIQUETTE.as_str() => {}
                autre => return Err(ErreurMemoire::EtiquetteInvalide(autre.unwrap_or_default().to_string())),
            },
        }
        let info = self.client.database_info().await.map_err(|e| err_client(&e))?;
        if !info.encrypted {
            return Err(ErreurMemoire::NonChiffree);
        }
        Ok(())
    }

    async fn lire(&self) -> Result<Contenu, ErreurMemoire> {
        Ok(Contenu {
            versions: self.lignes("versions").await?.iter().map(note_de).collect::<Result<_, _>>()?,
            candidats: self.lignes("candidats").await?.iter().map(candidat_de).collect::<Result<_, _>>()?,
            tombes: self.lignes("tombes").await?.iter().map(tombe_de).collect::<Result<_, _>>()?,
        })
    }

    async fn appliquer(&self, lot: Vec<Ecriture>) -> Result<(), ErreurMemoire> {
        let intentions: Vec<TypedWriteIntent> = lot.iter().flat_map(intention).collect();
        if intentions.is_empty() {
            return Ok(());
        }
        if intentions.len() > MAX_LOT {
            return Err(ErreurMemoire::Conflit(format!("lot trop grand ({} intentions)", intentions.len())));
        }
        // Lot atomique côté KindMother. L'existence des candidats retirés est vérifiée par
        // la façade sur un état relu sous verrou d'écriture.
        self.client
            .submit_batch(intentions)
            .await
            .map_err(|e| match e {
                ClientError::Service(m) if m.contains("UNIQUE") || m.contains("constraint") => ErreurMemoire::Conflit(m),
                autre => err_client(&autre),
            })?;
        Ok(())
    }
}
