//! @id     kindmother_etiquette
//! @do     definir_etiquette_publiable
//! @role   securite
//! @layer  core
//! @human  kindmother — étiquette `publiable` (EX-47) : référence exacte d'une donnée, intention signée
//! @human  par l'utilisateur (poser / retirer), état lu ; aucune étiquette sur une donnée `nationale`.

//! Étiquette de sensibilité **`publiable`** (IA-FEDEREE §5, EX-47, C-40).
//!
//! Seule une donnée étiquetée `publiable` **par l'utilisateur** peut devenir une
//! Fiche fédérée originale. Ce module ne contient que des types et des
//! contrôles de forme ; la vérification (signature, contenu, sensibilité), le
//! stockage dans le journal d'audit chaîné et la lecture sont faits par
//! `kindmother-service`.
//!
//! ## Modèle
//!
//! - **Donnée** ([`RefDonnee`]) : une cellule exacte — base, table, clé (égalités
//!   qui désignent **une seule** ligne), colonne de texte.
//! - **Empreinte du contenu** : `SHA-256(DOMAINE_CONTENU || octets UTF-8 de la
//!   valeur)` (32 octets, hexadécimal minuscule). Le service la recalcule sur la
//!   valeur courante : une étiquette ne se pose que sur le contenu exact que
//!   l'utilisateur a vu, et une donnée modifiée après coup n'est plus publiable.
//! - **Intention** ([`IntentionEtiquette`]) : poser ou retirer, **signée Ed25519
//!   par une clé de l'utilisateur** épinglée côté service (jamais une clé
//!   d'opérateur). L'opérateur qui la relaie (le pont MAIA) ne peut ni la
//!   fabriquer ni la modifier : MAIA, un outil ou un opérateur sans la clé de
//!   l'utilisateur ne peut pas poser l'étiquette.
//! - **Sensibilité** : toute base `maia-*` (mémoire de MAIA) et toute base
//!   déclarée nationale sont **`nationale`** ; l'étiquette `publiable` y est
//!   refusée et n'y est jamais lue comme vraie. Les noms de base d'une
//!   étiquette sont en **minuscules** ([`base_valide`]) : `MAIA-x` ne peut pas
//!   échapper à la règle `maia-*`.
//! - **Paramètres de publication** ([`ParametresPublication`], intention v2) :
//!   type, langue, licence et audience de la future Fiche sont **choisis par
//!   l'utilisateur** et signés avec la pose ; `maia-federee` les compare
//!   exactement à la Fiche (MAIA ne peut ni élargir l'audience ni s'en servir
//!   de canal).
//!
//! ## Message signé — intention **v2** (format figé, à consigner par amendement)
//!
//! ```text
//! DOMAINE_INTENTION ("miyukini/kindmother/etiquette/v2\0")
//! || clé publique du serveur KindMother (32 o)       -- liaison à l'instance
//! || action (1 o : 0x01 poser, 0x02 retirer)
//! || lp(utilisateur) || clé publique de l'utilisateur (32 o)
//! || lp(base) || lp(table) || n (u64 BE) || (lp(colonne) || valeur canonique)*  -- clé triée
//! || lp(colonne étiquetée)
//! || empreinte du contenu (32 o ; 32 zéros pour retirer)
//! || horodatage (u64 BE, secondes Unix)
//! || paramètres (poser seulement) :
//!      type (1 o : 1 fait, 2 définition, 3 procédure, 4 document, 5 tableau)
//!      || lp(langue) || lp(licence)
//!      || audience (1 o : 1 membres, 2 invités, 3 nommés [|| n (u64 BE) || lp(cog_id)* triés, sans doublon])
//! ```
//!
//! `lp(x)` = longueur u64 BE puis octets ; valeur canonique = [`SqlValue::canonical_bytes`].
//! Le JSON porte `"version": 2` (obligatoire) : une intention v1 (sans ce
//! champ, ou d'une autre version) est **refusée**.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::intent::{check_table_name, is_valid_identifier, push_len_prefixed, SqlValue};

/// Séparation de domaine du message signé par l'utilisateur (intention v2).
pub const DOMAINE_INTENTION: &[u8] = b"miyukini/kindmother/etiquette/v2\0";

/// Version du format d'intention acceptée (v1 refusée).
pub const VERSION_INTENTION: u32 = 2;

/// COG nommés au plus dans l'audience d'une publication.
pub const MAX_AUDIENCE_NOMMES: usize = 64;

/// Séparation de domaine de l'empreinte du contenu publiable (partagée avec `maia-federee`).
pub const DOMAINE_CONTENU: &[u8] = b"miyukini/kindmother/publiable/contenu/v1\0";

/// Préfixe des bases de la mémoire de MAIA : toujours `nationale`.
pub const PREFIXE_NATIONALE: &str = "maia-";

/// Fichier `<data_dir>/bases-nationales` : une base `nationale` par ligne (`#` : commentaire).
pub const FICHIER_BASES_NATIONALES: &str = "bases-nationales";

/// Colonnes au plus dans la clé d'une donnée.
pub const MAX_CLE_COLONNES: usize = 8;

/// Écart toléré entre l'horodatage signé et l'horloge du service (secondes).
pub const FENETRE_HORODATAGE_SECS: u64 = 300;

/// Longueur maximale d'un identifiant d'utilisateur.
pub const MAX_UTILISATEUR_LEN: usize = 48;

/// Longueur maximale d'un nom de base.
pub const MAX_BASE_LEN: usize = 64;

/// Sensibilité d'une donnée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sensibilite {
    /// Mémoire de MAIA ou base déclarée nationale : jamais publiable.
    Nationale,
    /// Donnée ordinaire : publiable seulement si l'utilisateur l'a étiquetée.
    Ordinaire,
}

/// Sensibilité par règle : toute base `maia-*` est `nationale` (comparaison
/// insensible à la casse : `MAIA-x` aussi).
pub fn sensibilite_par_regle(base: &str) -> Sensibilite {
    if base.to_ascii_lowercase().starts_with(PREFIXE_NATIONALE) {
        Sensibilite::Nationale
    } else {
        Sensibilite::Ordinaire
    }
}

/// Nom de base valide (`[a-z0-9_-]{1,64}`, ni `_` ni `-` initial) : minuscules
/// seulement, pour que la règle `maia-*` ⇒ `nationale` ne soit jamais
/// contournée par la casse.
pub fn base_valide(base: &str) -> bool {
    !base.is_empty()
        && base.len() <= MAX_BASE_LEN
        && base.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        && !base.starts_with(['_', '-'])
}

/// Identifiant d'utilisateur valide : `[a-z0-9][a-z0-9_-]{0,47}`.
pub fn utilisateur_valide(u: &str) -> bool {
    let mut c = u.chars();
    matches!(c.next(), Some(x) if x.is_ascii_lowercase() || x.is_ascii_digit())
        && u.len() <= MAX_UTILISATEUR_LEN
        && c.all(|x| x.is_ascii_lowercase() || x.is_ascii_digit() || x == '_' || x == '-')
}

/// Référence exacte d'une donnée : une cellule de texte.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefDonnee {
    /// Base.
    pub base: String,
    /// Table.
    pub table: String,
    /// Égalités qui désignent une seule ligne (triées par colonne).
    pub cle: BTreeMap<String, SqlValue>,
    /// Colonne dont la valeur (texte) est le contenu publiable.
    pub colonne: String,
}

impl RefDonnee {
    /// Référence à la colonne `colonne` de `base.table` (clé à compléter par [`Self::cle`]).
    pub fn new(base: impl Into<String>, table: impl Into<String>, colonne: impl Into<String>) -> Self {
        Self { base: base.into(), table: table.into(), cle: BTreeMap::new(), colonne: colonne.into() }
    }

    /// Ajoute une égalité de clé.
    pub fn cle(mut self, colonne: impl Into<String>, valeur: impl Into<SqlValue>) -> Self {
        self.cle.insert(colonne.into(), valeur.into());
        self
    }

    /// Contrôle de forme (le service le refait).
    pub fn check_shape(&self) -> Result<(), String> {
        if !base_valide(&self.base) {
            return Err(format!("invalid database name: {:?}", self.base));
        }
        check_table_name(&self.table)?;
        if !is_valid_identifier(&self.colonne) {
            return Err(format!("invalid column name: {:?}", self.colonne));
        }
        if self.cle.is_empty() || self.cle.len() > MAX_CLE_COLONNES {
            return Err(format!("key requires 1 to {MAX_CLE_COLONNES} columns"));
        }
        for (colonne, valeur) in &self.cle {
            if !is_valid_identifier(colonne) {
                return Err(format!("invalid key column: {colonne:?}"));
            }
            if matches!(valeur, SqlValue::Null | SqlValue::Real(_)) {
                return Err(format!("key value of {colonne} must be an integer, a text or a blob"));
            }
        }
        Ok(())
    }

    /// Encodage canonique (sans ambiguïté) : identité de la donnée.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(128);
        push_len_prefixed(&mut out, self.base.as_bytes());
        push_len_prefixed(&mut out, self.table.as_bytes());
        out.extend_from_slice(&(self.cle.len() as u64).to_be_bytes());
        for (colonne, valeur) in &self.cle {
            push_len_prefixed(&mut out, colonne.as_bytes());
            valeur.canonical_bytes(&mut out);
        }
        push_len_prefixed(&mut out, self.colonne.as_bytes());
        out
    }
}

/// Action d'une intention d'étiquette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionEtiquette {
    /// Pose l'étiquette `publiable` sur le contenu exact (empreinte).
    Poser,
    /// Retire l'étiquette : plus aucune nouvelle publication.
    Retirer,
}

impl ActionEtiquette {
    /// Octet du message signé.
    pub fn octet(self) -> u8 {
        match self {
            ActionEtiquette::Poser => 0x01,
            ActionEtiquette::Retirer => 0x02,
        }
    }

    /// Nom stable (audit).
    pub fn as_str(self) -> &'static str {
        match self {
            ActionEtiquette::Poser => "publiable.poser",
            ActionEtiquette::Retirer => "publiable.retirer",
        }
    }
}

/// Nature de la Fiche publiée (même codage que `maia_federee::TypeFiche`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypePublication {
    /// Fait.
    Fait,
    /// Définition.
    Definition,
    /// Procédure.
    Procedure,
    /// Document.
    Document,
    /// Tableau.
    Tableau,
}

impl TypePublication {
    /// Octet du message signé (= code `TypeFiche` de la Fiche).
    pub fn octet(self) -> u8 {
        match self {
            Self::Fait => 1,
            Self::Definition => 2,
            Self::Procedure => 3,
            Self::Document => 4,
            Self::Tableau => 5,
        }
    }

    /// Depuis l'octet.
    pub fn depuis_octet(o: u8) -> Option<Self> {
        Some(match o {
            1 => Self::Fait,
            2 => Self::Definition,
            3 => Self::Procedure,
            4 => Self::Document,
            5 => Self::Tableau,
            _ => return None,
        })
    }
}

/// Audience de la Fiche publiée, choisie par l'utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudiencePublication {
    /// Membres de la fédération.
    Membres,
    /// Membres et invités.
    Invites,
    /// COG nommés (`cog_id`, triés, sans doublon).
    Nommes(Vec<String>),
}

/// Paramètres de publication signés par l'utilisateur avec la pose (v2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParametresPublication {
    /// Nature de la Fiche.
    pub type_fiche: TypePublication,
    /// Langue (`[a-z]{2,3}(-[A-Za-z0-9]{2,8})?`).
    pub langue: String,
    /// Licence (identifiant SPDX, `[A-Za-z0-9.+-]{1,64}`).
    pub licence: String,
    /// Audience.
    pub audience: AudiencePublication,
}

fn langue_valide(l: &str) -> bool {
    let mut parts = l.splitn(2, '-');
    let primaire = parts.next().unwrap_or("");
    (2..=3).contains(&primaire.len())
        && primaire.bytes().all(|b| b.is_ascii_lowercase())
        && parts.next().is_none_or(|r| (2..=8).contains(&r.len()) && r.bytes().all(|b| b.is_ascii_alphanumeric()))
}

fn licence_valide(l: &str) -> bool {
    (1..=64).contains(&l.len()) && l.bytes().all(|b| b.is_ascii_alphanumeric() || b".+-".contains(&b))
}

/// `cog_id` : 26 caractères base32 minuscule (RFC 4648).
fn cog_valide(c: &str) -> bool {
    c.len() == 26 && c.bytes().all(|b| b.is_ascii_lowercase() || (b'2'..=b'7').contains(&b))
}

impl ParametresPublication {
    /// Contrôle de forme (alphabets de la Fiche ; COG nommés triés sans doublon).
    pub fn check_shape(&self) -> Result<(), String> {
        if !langue_valide(&self.langue) {
            return Err("invalid publication language".to_string());
        }
        if !licence_valide(&self.licence) {
            return Err("invalid publication licence".to_string());
        }
        if let AudiencePublication::Nommes(cogs) = &self.audience {
            if cogs.is_empty() || cogs.len() > MAX_AUDIENCE_NOMMES || !cogs.iter().all(|c| cog_valide(c)) {
                return Err("invalid named audience".to_string());
            }
            if !cogs.windows(2).all(|w| w[0] < w[1]) {
                return Err("named audience must be sorted without duplicates".to_string());
            }
        }
        Ok(())
    }

    /// Encodage canonique (message signé).
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = vec![self.type_fiche.octet()];
        push_len_prefixed(&mut out, self.langue.as_bytes());
        push_len_prefixed(&mut out, self.licence.as_bytes());
        match &self.audience {
            AudiencePublication::Membres => out.push(1),
            AudiencePublication::Invites => out.push(2),
            AudiencePublication::Nommes(cogs) => {
                out.push(3);
                out.extend_from_slice(&(cogs.len() as u64).to_be_bytes());
                for c in cogs {
                    push_len_prefixed(&mut out, c.as_bytes());
                }
            }
        }
        out
    }

    /// Forme textuelle compacte (journal d'audit) :
    /// `<type>,<langue>,<licence>,<membres|invites|n:cog.cog…>`.
    pub fn texte(&self) -> String {
        let audience = match &self.audience {
            AudiencePublication::Membres => "membres".to_string(),
            AudiencePublication::Invites => "invites".to_string(),
            AudiencePublication::Nommes(c) => format!("n:{}", c.join(".")),
        };
        format!("{},{},{},{audience}", self.type_fiche.octet(), self.langue, self.licence)
    }

    /// Inverse de [`Self::texte`] (forme contrôlée).
    pub fn depuis_texte(t: &str) -> Option<Self> {
        let [ty, langue, licence, audience] = t.split(',').collect::<Vec<_>>()[..] else {
            return None;
        };
        let audience = match audience {
            "membres" => AudiencePublication::Membres,
            "invites" => AudiencePublication::Invites,
            a => AudiencePublication::Nommes(a.strip_prefix("n:")?.split('.').map(str::to_string).collect()),
        };
        let p = Self {
            type_fiche: TypePublication::depuis_octet(ty.parse().ok()?)?,
            langue: langue.to_string(),
            licence: licence.to_string(),
            audience,
        };
        p.check_shape().ok()?;
        Some(p)
    }
}

/// Intention d'étiquette signée par l'utilisateur (format v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentionEtiquette {
    /// Version du format : [`VERSION_INTENTION`] (obligatoire ; v1 refusée).
    pub version: u32,
    /// Poser ou retirer.
    pub action: ActionEtiquette,
    /// Donnée exacte.
    pub donnee: RefDonnee,
    /// Utilisateur signataire.
    pub utilisateur: String,
    /// Empreinte du contenu (hex, 64 car.) : obligatoire pour poser, absente pour retirer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empreinte_contenu: Option<String>,
    /// Horodatage signé (secondes Unix), strictement croissant par donnée.
    pub horodatage: u64,
    /// Paramètres de publication : obligatoires pour poser, absents pour retirer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parametres: Option<ParametresPublication>,
    /// Clé publique Ed25519 de l'utilisateur (hex, 64 car.).
    pub cle_publique: String,
    /// Signature Ed25519 (hex, 128 car.) de [`Self::message_signe`].
    pub signature: String,
}

/// Décode un hexadécimal de longueur fixe (minuscules ou majuscules).
pub fn decode_hex<const N: usize>(texte: &str) -> Option<[u8; N]> {
    let b = texte.as_bytes();
    if b.len() != N * 2 {
        return None;
    }
    let val = |c: u8| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    };
    let mut out = [0u8; N];
    for (i, o) in out.iter_mut().enumerate() {
        *o = (val(b[2 * i])? << 4) | val(b[2 * i + 1])?;
    }
    Some(out)
}

/// Encode en hexadécimal minuscule.
pub fn encode_hex(octets: &[u8]) -> String {
    const D: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(octets.len() * 2);
    for b in octets {
        s.push(char::from(D[usize::from(b >> 4)]));
        s.push(char::from(D[usize::from(b & 0x0f)]));
    }
    s
}

impl IntentionEtiquette {
    /// Contrôle de forme (le service le refait).
    pub fn check_shape(&self) -> Result<(), String> {
        if self.version != VERSION_INTENTION {
            return Err(format!("label intent version {} refused (v{VERSION_INTENTION} required)", self.version));
        }
        self.donnee.check_shape()?;
        match (self.action, &self.parametres) {
            (ActionEtiquette::Poser, Some(p)) => p.check_shape()?,
            (ActionEtiquette::Poser, None) => return Err("poser requires publication parameters".to_string()),
            (ActionEtiquette::Retirer, None) => {}
            (ActionEtiquette::Retirer, Some(_)) => return Err("retirer takes no publication parameters".to_string()),
        }
        if !utilisateur_valide(&self.utilisateur) {
            return Err("invalid user id".to_string());
        }
        if decode_hex::<32>(&self.cle_publique).is_none() {
            return Err("invalid user public key".to_string());
        }
        if decode_hex::<64>(&self.signature).is_none() {
            return Err("invalid signature".to_string());
        }
        match (self.action, &self.empreinte_contenu) {
            (ActionEtiquette::Poser, Some(e)) if decode_hex::<32>(e).is_some() => Ok(()),
            (ActionEtiquette::Poser, _) => Err("poser requires a content digest (64 hex)".to_string()),
            (ActionEtiquette::Retirer, None) => Ok(()),
            (ActionEtiquette::Retirer, Some(_)) => Err("retirer takes no content digest".to_string()),
        }
    }

    /// Message signé par l'utilisateur, lié à la clé publique du serveur KindMother.
    pub fn message_signe(&self, cle_serveur: &[u8; 32]) -> Result<Vec<u8>, String> {
        self.check_shape()?;
        let cle = decode_hex::<32>(&self.cle_publique).ok_or("invalid user public key")?;
        let empreinte = match &self.empreinte_contenu {
            Some(e) => decode_hex::<32>(e).ok_or("invalid content digest")?,
            None => [0u8; 32],
        };
        let mut m = Vec::with_capacity(256);
        m.extend_from_slice(DOMAINE_INTENTION);
        m.extend_from_slice(cle_serveur);
        m.push(self.action.octet());
        push_len_prefixed(&mut m, self.utilisateur.as_bytes());
        m.extend_from_slice(&cle);
        m.extend_from_slice(&self.donnee.canonical_bytes());
        m.extend_from_slice(&empreinte);
        m.extend_from_slice(&self.horodatage.to_be_bytes());
        if let Some(p) = &self.parametres {
            m.extend_from_slice(&p.canonical_bytes());
        }
        Ok(m)
    }
}

/// État de l'étiquette d'une donnée, lu par une requête typée.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EtatEtiquette {
    /// Sensibilité de la base.
    pub sensibilite: Sensibilite,
    /// Étiquette `publiable` posée par l'utilisateur et non retirée.
    pub posee: bool,
    /// Vrai seulement si : posée, base `Ordinaire`, contenu courant égal au contenu étiqueté.
    pub publiable: bool,
    /// Empreinte du contenu étiqueté (hex), si posée.
    pub empreinte_contenu: Option<String>,
    /// Empreinte du contenu courant (hex), si la clé désigne une seule ligne de texte.
    pub empreinte_actuelle: Option<String>,
    /// Utilisateur qui a posé l'étiquette.
    pub utilisateur: Option<String>,
    /// Horodatage signé du dernier événement (pose ou retrait).
    pub horodatage: Option<u64>,
    /// Numéro de l'entrée du journal d'audit chaîné qui porte la pose.
    pub seq: Option<i64>,
    /// Paramètres de publication signés par l'utilisateur avec la pose (v2) ;
    /// absents pour une pose v1 (jamais publiable par `maia-federee`).
    #[serde(default)]
    pub parametres: Option<ParametresPublication>,
    /// Identité de la donnée (hex de l'empreinte de sa référence) : base de
    /// l'identifiant de Fiche imposé par `maia-federee`.
    #[serde(default)]
    pub empreinte_donnee: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parametres() -> ParametresPublication {
        ParametresPublication {
            type_fiche: TypePublication::Fait,
            langue: "fr".into(),
            licence: "CC-BY-4.0".into(),
            audience: AudiencePublication::Membres,
        }
    }

    fn intention() -> IntentionEtiquette {
        IntentionEtiquette {
            version: VERSION_INTENTION,
            parametres: Some(parametres()),
            action: ActionEtiquette::Poser,
            donnee: RefDonnee::new("notes", "notes", "body").cle("id", "n1"),
            utilisateur: "alice".into(),
            empreinte_contenu: Some("ab".repeat(32)),
            horodatage: 1_800_000_000,
            cle_publique: "11".repeat(32),
            signature: "22".repeat(64),
        }
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn forme_et_message() {
        let i = intention();
        assert!(i.check_shape().is_ok());
        let m = i.message_signe(&[9; 32]).expect("message");
        assert!(m.starts_with(DOMAINE_INTENTION));
        // Lié au serveur, à l'action et à l'horodatage.
        assert_ne!(m, i.message_signe(&[8; 32]).expect("m"));
        let r = IntentionEtiquette { action: ActionEtiquette::Retirer, empreinte_contenu: None, parametres: None, ..i.clone() };
        assert_ne!(m, r.message_signe(&[9; 32]).expect("m"));
        let t = IntentionEtiquette { horodatage: 1, ..i.clone() };
        assert_ne!(m, t.message_signe(&[9; 32]).expect("m"));
        // Lié aux paramètres de publication (audience élargie ⇒ autre message).
        let a = IntentionEtiquette {
            parametres: Some(ParametresPublication { audience: AudiencePublication::Invites, ..parametres() }),
            ..i.clone()
        };
        assert_ne!(m, a.message_signe(&[9; 32]).expect("m"));
    }

    /// M1 : intention v1 (sans `version`, ou version ≠ 2) refusée.
    #[test]
    fn intention_v1_refusee() {
        let i = intention();
        let mut json = serde_json::to_value(&i).expect("json");
        json.as_object_mut().expect("objet").remove("version");
        assert!(serde_json::from_value::<IntentionEtiquette>(json).is_err(), "v1 sans version");
        assert!(IntentionEtiquette { version: 1, ..i.clone() }.check_shape().is_err());
        assert!(IntentionEtiquette { version: 1, ..i }.message_signe(&[9; 32]).is_err());
    }

    #[test]
    fn parametres_forme_et_texte() {
        let cog = |c: char| c.to_string().repeat(26);
        let p = ParametresPublication { audience: AudiencePublication::Nommes(vec![cog('a'), cog('b')]), ..parametres() };
        assert!(p.check_shape().is_ok());
        assert_eq!(ParametresPublication::depuis_texte(&p.texte()), Some(p.clone()));
        assert_eq!(ParametresPublication::depuis_texte(&parametres().texte()), Some(parametres()));
        for mauvais in [
            ParametresPublication { audience: AudiencePublication::Nommes(vec![cog('b'), cog('a')]), ..parametres() },
            ParametresPublication { audience: AudiencePublication::Nommes(vec![cog('a'), cog('a')]), ..parametres() },
            ParametresPublication { audience: AudiencePublication::Nommes(vec![]), ..parametres() },
            ParametresPublication { langue: "fr,x".into(), ..parametres() },
            ParametresPublication { licence: "MIT;".into(), ..parametres() },
        ] {
            assert!(mauvais.check_shape().is_err(), "{mauvais:?}");
        }
        let i = intention();
        assert!(IntentionEtiquette { parametres: None, ..i.clone() }.check_shape().is_err());
        let r = IntentionEtiquette { action: ActionEtiquette::Retirer, empreinte_contenu: None, ..i };
        assert!(r.check_shape().is_err(), "retirer sans paramètres");
    }

    #[test]
    fn refus_de_forme() {
        let i = intention();
        for mauvaise in [
            IntentionEtiquette { empreinte_contenu: None, ..i.clone() },
            IntentionEtiquette { action: ActionEtiquette::Retirer, parametres: None, ..i.clone() },
            IntentionEtiquette { utilisateur: "Alice".into(), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("notes", "notes", "body"), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("_audit", "t", "c").cle("id", 1_i64), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("b", "_kindmother_meta", "c").cle("id", 1_i64), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("b", "t", "c; DROP").cle("id", 1_i64), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("b", "t", "c").cle("id", SqlValue::Null), ..i.clone() },
            IntentionEtiquette { signature: "zz".repeat(64), ..i.clone() },
            // M4 : majuscules refusées (MAIA-x ne contourne pas maia-*).
            IntentionEtiquette { donnee: RefDonnee::new("MAIA-alice", "t", "c").cle("id", 1_i64), ..i.clone() },
            IntentionEtiquette { donnee: RefDonnee::new("Notes", "t", "c").cle("id", 1_i64), ..i.clone() },
        ] {
            assert!(mauvaise.check_shape().is_err(), "{mauvaise:?}");
        }
    }

    #[test]
    fn sensibilite_maia_nationale() {
        assert_eq!(sensibilite_par_regle("maia-alice"), Sensibilite::Nationale);
        assert_eq!(sensibilite_par_regle("MAIA-alice"), Sensibilite::Nationale);
        assert_eq!(sensibilite_par_regle("Maia-alice"), Sensibilite::Nationale);
        assert!(!base_valide("MAIA-alice"));
        assert_eq!(sensibilite_par_regle("notes"), Sensibilite::Ordinaire);
    }

    #[test]
    fn hex_aller_retour() {
        let o = [0u8, 1, 0xab, 0xff];
        assert_eq!(decode_hex::<4>(&encode_hex(&o)), Some(o));
        assert_eq!(decode_hex::<4>("00"), None);
    }
}
