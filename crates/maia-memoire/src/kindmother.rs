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
use std::future::Future;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, SchemaMigration, TypedSelect,
    TypedWriteIntent,
};
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
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

/// Erreur d'écriture : contrainte violée = conflit, sinon erreur d'entrepôt.
fn err_ecriture(e: ClientError) -> ErreurMemoire {
    match e {
        ClientError::Service(m) if m.contains("UNIQUE") || m.contains("constraint") => ErreurMemoire::Conflit(m),
        autre => err_client(&autre),
    }
}

/// Erreur de transport : la connexion est perdue (fermée, rompue, réinitialisée, fin
/// de flux) ; une nouvelle poignée de main peut la rétablir. Jamais une erreur
/// d'authentification ni un refus du service.
fn est_transport(e: &ClientError) -> bool {
    matches!(e, ClientError::Transport(_) | ClientError::NotConnected | ClientError::Timeout(_))
}

/// Refus d'un rejeu par KindMother : l'identifiant d'intention figure déjà au journal,
/// donc la première soumission lui est parvenue.
fn est_deja_journalisee(e: &ClientError) -> bool {
    e.to_string().contains("intent id already used")
}

/// Attentes avant chaque tentative de reconnexion d'un même appel (backoff borné :
/// 3 poignées de main au plus, ~0,65 s d'attente cumulée, puis erreur explicite).
const ATTENTES_RECONNEXION: [Duration; 3] =
    [Duration::ZERO, Duration::from_millis(150), Duration::from_millis(500)];

/// État de la connexion partagée.
struct Connexion {
    /// Connexion courante ; `None` après une erreur de transport (rouverte à l'appel suivant).
    client: Option<Arc<KindMotherClient>>,
    /// Authentification refusée lors d'une reconnexion : verrou définitif, aucune
    /// nouvelle poignée de main jusqu'à une nouvelle [`EntrepotKindMother::connecter`].
    auth_refusee: Option<String>,
}

/// Issue d'un lot d'écritures relue dans la base après une coupure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IssueLot {
    Appliquee,
    NonAppliquee,
    /// Ni tout ni rien : impossible pour un lot atomique, refusé.
    Incoherente,
}

/// Ce qu'il faut relire pour savoir si un lot interrompu a été appliqué.
enum Verification<'a> {
    Lot(&'a [Ecriture]),
    Etiquette,
}

/// Entrepôt KindMother d'un utilisateur.
///
/// # Reprise après coupure
///
/// La connexion à KindMother peut tomber (redémarrage du service, réseau). Sur une
/// erreur de **transport** (connexion fermée, *broken pipe*, *reset*, fin de flux),
/// l'entrepôt abandonne la connexion et en rouvre une — nouvelle poignée de main
/// authentifiée avec le même justificatif — en 3 tentatives au plus par appel
/// (backoff borné) ; au-delà, erreur explicite, et l'appel suivant réessaie.
///
/// - **Lectures et migrations** (idempotentes) : rejouées une fois sur la connexion rouverte.
/// - **Écritures** : jamais rejouées à l'aveugle. Les identifiants d'intention sont
///   fixés avant l'envoi ; après la coupure, l'état est **relu** :
///   lot présent → succès (pas de doublon) ; lot absent → rejoué **une** fois avec les
///   **mêmes** identifiants, que KindMother refuse s'il a déjà journalisé la première
///   soumission (non-réutilisation des identifiants : aucun double effet possible,
///   même si l'original était encore en vol) ; état incohérent, relecture impossible
///   ou rejeu refusé sans lot présent → erreur explicite « issue inconnue ».
/// - **Authentification refusée** à la reconnexion : jamais réessayée ; l'entrepôt se
///   verrouille (toutes les opérations échouent) jusqu'à une nouvelle connexion explicite.
///
/// Le justificatif de l'opérateur reste en mémoire pour la durée de vie de l'entrepôt
/// (nécessaire à la reconnexion ; effacé à la destruction, `ZeroizeOnDrop`).
pub struct EntrepotKindMother {
    adresse: String,
    justificatif: OperatorCredential,
    base: String,
    connexion: Mutex<Connexion>,
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
        let justificatif = match credentials {
            Some(dir) => OperatorCredential::load_from_dir(&base, dir),
            None => OperatorCredential::load(&base),
        }
        .map_err(|e| err_client(&e))?;
        let client = KindMotherClient::connect(adresse, &justificatif, &base)
            .await
            .map_err(|e| err_client(&e))?;
        Ok(Self {
            adresse: adresse.to_string(),
            justificatif,
            base,
            connexion: Mutex::new(Connexion { client: Some(Arc::new(client)), auth_refusee: None }),
        })
    }

    /// Nom de la base.
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Connexion courante, rouverte au besoin (backoff borné). Une authentification
    /// refusée verrouille l'entrepôt : aucune poignée de main n'est retentée.
    async fn client(&self) -> Result<Arc<KindMotherClient>, ErreurMemoire> {
        let mut cx = self.connexion.lock().await;
        if let Some(motif) = &cx.auth_refusee {
            return Err(ErreurMemoire::Entrepot(format!(
                "authentification KindMother refusée à la reconnexion ({motif}) : \
                 reconnexion suspendue, nouvelle connexion explicite requise"
            )));
        }
        if let Some(c) = &cx.client {
            return Ok(Arc::clone(c));
        }
        let mut derniere = String::new();
        for attente in ATTENTES_RECONNEXION {
            if !attente.is_zero() {
                tokio::time::sleep(attente).await;
            }
            match KindMotherClient::connect(&self.adresse, &self.justificatif, &self.base).await {
                Ok(c) => {
                    info!(base = %self.base, "connexion KindMother rétablie");
                    let c = Arc::new(c);
                    cx.client = Some(Arc::clone(&c));
                    return Ok(c);
                }
                Err(ClientError::Authentication(motif)) => {
                    error!(base = %self.base, %motif, "authentification KindMother refusée à la reconnexion : entrepôt verrouillé");
                    cx.auth_refusee = Some(motif.clone());
                    return Err(ErreurMemoire::Entrepot(format!(
                        "authentification KindMother refusée à la reconnexion : {motif}"
                    )));
                }
                Err(e) => {
                    warn!(base = %self.base, erreur = %e, "reconnexion à KindMother impossible");
                    derniere = e.to_string();
                }
            }
        }
        Err(ErreurMemoire::Entrepot(format!(
            "KindMother injoignable ({} tentatives de reconnexion) : {derniere}",
            ATTENTES_RECONNEXION.len()
        )))
    }

    /// Abandonne la connexion `c` après une erreur de transport (sauf si une autre
    /// tâche l'a déjà remplacée).
    async fn abandonner(&self, c: &Arc<KindMotherClient>, cause: &ClientError) {
        let mut cx = self.connexion.lock().await;
        if cx.client.as_ref().is_some_and(|x| Arc::ptr_eq(x, c)) {
            warn!(base = %self.base, %cause, "connexion KindMother perdue : elle sera rouverte");
            cx.client = None;
        }
    }

    /// Opération **idempotente** (lecture, migrations versionnées) : sur erreur de
    /// transport, reconnexion puis **un** rejeu.
    async fn idempotente<T, F, Fut>(&self, op: F) -> Result<T, ErreurMemoire>
    where
        F: Fn(Arc<KindMotherClient>) -> Fut,
        Fut: Future<Output = Result<T, ClientError>>,
    {
        let c = self.client().await?;
        match op(Arc::clone(&c)).await {
            Err(e) if est_transport(&e) => {
                self.abandonner(&c, &e).await;
                let c = self.client().await?;
                match op(Arc::clone(&c)).await {
                    Err(e) if est_transport(&e) => {
                        self.abandonner(&c, &e).await;
                        Err(err_client(&e))
                    }
                    r => r.map_err(|e| err_client(&e)),
                }
            }
            r => r.map_err(|e| err_client(&e)),
        }
    }

    async fn lignes(&self, table: &str) -> Result<Vec<HashMap<String, Value>>, ErreurMemoire> {
        self.idempotente(|c| async move { c.select(TypedSelect::from(table)).await })
            .await
    }

    /// Soumet un lot atomique d'intentions ; après une coupure, relit l'état au lieu
    /// de rejouer à l'aveugle (voir la documentation du type).
    async fn ecrire(
        &self,
        mut intentions: Vec<TypedWriteIntent>,
        verification: Verification<'_>,
    ) -> Result<(), ErreurMemoire> {
        // Identifiants fixés avant l'envoi : un rejeu porte les mêmes (anti-doublon côté KindMother).
        for i in &mut intentions {
            if i.intent_id.is_empty() {
                i.intent_id = Uuid::new_v4().to_string();
            }
        }
        let c = self.client().await?;
        let cause = match c.submit_batch(intentions.clone()).await {
            Ok(_) => return Ok(()),
            Err(e) if est_transport(&e) => e,
            Err(e) => return Err(err_ecriture(e)),
        };
        self.abandonner(&c, &cause).await;
        let inconnue = |detail: String| {
            ErreurMemoire::Entrepot(format!(
                "écriture interrompue ({cause}) : issue inconnue, non rejouée ({detail})"
            ))
        };
        let c = self.client().await.map_err(|e| inconnue(e.to_string()))?;
        match self.issue(&c, &verification).await.map_err(|e| inconnue(e.to_string()))? {
            IssueLot::Appliquee => {
                info!(base = %self.base, "écriture interrompue déjà appliquée : non rejouée");
                Ok(())
            }
            IssueLot::Incoherente => Err(inconnue("état relu incohérent".to_string())),
            IssueLot::NonAppliquee => match c.submit_batch(intentions).await {
                Ok(_) => {
                    info!(base = %self.base, "écriture interrompue non appliquée : rejouée une fois");
                    Ok(())
                }
                // La première soumission avait atteint le journal : l'état fait foi.
                Err(e) if est_deja_journalisee(&e) => {
                    match self.issue(&c, &verification).await.map_err(|e| inconnue(e.to_string()))? {
                        IssueLot::Appliquee => Ok(()),
                        _ => Err(inconnue("première soumission journalisée mais non appliquée".to_string())),
                    }
                }
                Err(e) if est_transport(&e) => {
                    self.abandonner(&c, &e).await;
                    Err(inconnue(format!("rejeu interrompu : {e}")))
                }
                Err(e) => Err(err_ecriture(e)),
            },
        }
    }

    /// Relit la base pour savoir si un lot a été appliqué (lot atomique : tout ou rien).
    async fn issue(
        &self,
        c: &KindMotherClient,
        verification: &Verification<'_>,
    ) -> Result<IssueLot, ClientError> {
        let lire = |table: &'static str, cle: &'static str, valeur: String| {
            c.select(TypedSelect::from(table).filter(cle, valeur))
        };
        let mut presences = Vec::new();
        match verification {
            Verification::Etiquette => {
                let lignes = lire("meta", "cle", "etiquette".to_string()).await?;
                presences.push(
                    lignes
                        .iter()
                        .any(|l| l.get("valeur").and_then(Value::as_str) == Some(ETIQUETTE.as_str())),
                );
            }
            Verification::Lot(lot) => {
                for e in *lot {
                    presences.push(match e {
                        Ecriture::AjouterVersion(n) => lire("versions", "id", n.id.as_str().to_string())
                            .await?
                            .iter()
                            .any(|l| {
                                l.get("version").and_then(Value::as_i64) == Some(i64::from(n.version))
                                    && l.get("enregistre_le").and_then(Value::as_str)
                                        == Some(n.enregistre_le.to_rfc3339().as_str())
                            }),
                        Ecriture::AjouterCandidat(k) => {
                            !lire("candidats", "id", k.id.to_string()).await?.is_empty()
                        }
                        Ecriture::RetirerCandidat(id) => {
                            lire("candidats", "id", id.to_string()).await?.is_empty()
                        }
                        Ecriture::Oublier(t) => lire("tombes", "id", t.id.as_str().to_string())
                            .await?
                            .iter()
                            .any(|l| {
                                l.get("oublie_le").and_then(Value::as_str)
                                    == Some(t.oublie_le.to_rfc3339().as_str())
                            }),
                    });
                }
            }
        }
        Ok(if presences.iter().all(|p| *p) {
            IssueLot::Appliquee
        } else if presences.iter().any(|p| *p) {
            IssueLot::Incoherente
        } else {
            IssueLot::NonAppliquee
        })
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
        // Migrations versionnées : idempotentes, rejouables après une coupure.
        self.idempotente(|c| async move { c.apply_migrations(migrations()).await })
            .await?;
        // Étiquette posée à la création, jamais remplacée ; toute autre valeur : refus.
        let meta = self.lignes("meta").await?;
        let etiquette = meta
            .iter()
            .find(|l| l.get("cle").and_then(Value::as_str) == Some("etiquette"));
        match etiquette {
            None => {
                self.ecrire(
                    vec![TypedWriteIntent::insert("meta")
                        .value("cle", "etiquette")
                        .value("valeur", ETIQUETTE.as_str())
                        .purpose("memoire_etiquette")],
                    Verification::Etiquette,
                )
                .await?;
            }
            Some(l) => match l.get("valeur").and_then(Value::as_str) {
                Some(v) if v == ETIQUETTE.as_str() => {}
                autre => return Err(ErreurMemoire::EtiquetteInvalide(autre.unwrap_or_default().to_string())),
            },
        }
        let info = self
            .idempotente(|c| async move { c.database_info().await })
            .await?;
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
        self.ecrire(intentions, Verification::Lot(&lot)).await
    }
}
