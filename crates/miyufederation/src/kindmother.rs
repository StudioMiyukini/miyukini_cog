//! @id     miyufederation_kindmother
//! @do     stocker_etat_federal_par_kindmother
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — support KindMother de l'état fédéral (base dédiée `miyufederation`, intentions typées, morceaux d'une génération rendus visibles par un pointeur atomique).

//! Support KindMother de l'état fédéral scellé (fonctionnalité `kindmother`).
//!
//! KindMother est le **seul écrivain** du COG (K-13) : schéma par migration
//! versionnée, écritures par **intentions typées** (aucun texte SQL), lectures
//! typées. L'état reste l'enveloppe `MFED` scellée (HMAC du secret local,
//! titres signés revérifiés au chargement) : KindMother ne voit que des octets
//! opaques et n'est **pas** une source de confiance — un état altéré dans la
//! base est refusé au chargement comme un fichier altéré.
//!
//! # Base (à déclarer côté KindMother)
//!
//! Opérateur et base **`miyufederation`** ([`OPERATEUR_DEFAUT`], propriétaire
//! de sa seule base : `OperatorConfig::owner("miyufederation", "miyufederation")`),
//! **non** `nationale` : jamais dans un fichier `.bases`, jamais étiquetable ;
//! une base `maia-*` ou `cog-federee` est refusée ([`nom_valide`]). La base
//! doit être chiffrée (SQLCipher) : sinon refus à l'ouverture.
//!
//! # Schéma (migration 1)
//!
//! | Table | Colonnes | Rôle |
//! |---|---|---|
//! | `etat_courant` | `cle` TEXT PK (`'courant'`), `generation` INTEGER, `morceaux` INTEGER, `taille` INTEGER, `empreinte` TEXT (SHA-256, hex) | pointeur de l'état courant |
//! | `etat_morceaux` | `generation` INTEGER, `rang` INTEGER, `octets` TEXT (hex, ≤ 256 Kio d'état), PK (`generation`, `rang`) | morceaux de l'état |
//!
//! # Écriture atomique
//!
//! Nouvelle génération `g + 1` : suppression des restes de `g + 1` (écriture
//! interrompue) et de `g − 1` (purge interrompue d'une écriture antérieure),
//! insertion des morceaux de `g + 1`, puis **bascule du pointeur** (`Update`
//! gardé par `generation = g`, ou `Insert` au premier état) : un lot
//! KindMother (transaction) quand l'état tient en ≤ 4 morceaux (≤ 1 Mio),
//! sinon des lots de morceaux puis un lot final de bascule. Tant que le
//! pointeur n'a pas basculé, la lecture rend l'état `g` intact. Bascule sans
//! effet (pointeur déplacé par un autre écrivain) ⇒ refus, rien n'est purgé.
//! **Aussitôt la bascule réussie**, les morceaux de `g` sont supprimés (lot
//! suivant : une suppression ne peut pas être conditionnée à la bascule dans
//! le même lot, et une bascule sans effet ne doit jamais purger l'état
//! courant) ; une interruption entre les deux laisse `g` jusqu'à l'écriture
//! suivante, qui le purge. Un seul écrivain par base (l'opérateur
//! `miyufederation` est propre au processus de fédération).
//!
//! # Retour arrière
//!
//! KindMother n'est pas une source de confiance : un acteur capable d'écrire
//! dans la base pourrait y restaurer un ancien pointeur et ses morceaux
//! (copiés avant leur purge). L'état restauré est authentique (HMAC) mais
//! **périmé** : la vue persistante le refuse par sa génération scellée,
//! confrontée au compteur de fraîcheur tenu hors de KindMother
//! ([`crate::fraicheur`]).
//!
//! # Lecture
//!
//! Pointeur absent = aucun état ; sinon chaque morceau `(g, rang)` est relu et
//! [`assembler_morceaux`] contrôle nombre, rangs, tailles et SHA-256 avant de
//! rendre un octet ; tout écart ⇒ [`ErreurEtat::Illisible`]. Erreurs de
//! transport ou de service ⇒ [`ErreurEtat::Support`] (aucune reconnexion
//! implicite : l'appelant recharge, fail-closed).

use std::collections::HashMap;
use std::future::Future;
use std::path::Path;

use kindmother_client::{ClientError, KindMotherClient, OperatorCredential, SchemaMigration, TypedSelect, TypedWriteIntent};
use serde_json::Value;

use crate::persistance::ErreurEtat;
use crate::support::{assembler_morceaux, de_hex, decouper, hex, Pointeur, SupportEtat, TAILLE_MORCEAU};

/// Opérateur et base KindMother de l'état fédéral.
pub const OPERATEUR_DEFAUT: &str = "miyufederation";
/// Morceaux au plus par lot (≈ 2 Mio de JSON : sous la trame de 4 Mio).
pub const MORCEAUX_PAR_LOT: usize = 4;
const CLE_POINTEUR: &str = "courant";
const LONGUEUR_MAX_NOM: usize = 64;

fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "état fédéral v1 : pointeur de génération, morceaux de l'état MFED scellé",
        &[
            "CREATE TABLE IF NOT EXISTS etat_courant (
                cle TEXT PRIMARY KEY, generation INTEGER NOT NULL, morceaux INTEGER NOT NULL,
                taille INTEGER NOT NULL, empreinte TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS etat_morceaux (
                generation INTEGER NOT NULL, rang INTEGER NOT NULL, octets TEXT NOT NULL,
                PRIMARY KEY (generation, rang))",
        ],
    )]
}

/// Nom d'opérateur / de base admis pour l'état fédéral : `[a-z0-9][a-z0-9_-]{0,63}`,
/// ni base `nationale` de mémoire MAIA (`maia-*`), ni base réservée `cog-federee`.
pub fn nom_valide(nom: &str) -> bool {
    let mut cs = nom.chars();
    matches!(cs.next(), Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit())
        && nom.len() <= LONGUEUR_MAX_NOM
        && cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        && !nom.starts_with("maia-")
        && nom != "cog-federee"
}

fn support(e: &ClientError) -> ErreurEtat {
    ErreurEtat::Support(format!("KindMother : {e}"))
}

fn entier(l: &HashMap<String, Value>, c: &'static str) -> Result<u64, ErreurEtat> {
    l.get(c)
        .and_then(Value::as_i64)
        .and_then(|v| u64::try_from(v).ok())
        .ok_or(ErreurEtat::Illisible("colonne entière"))
}

fn texte<'a>(l: &'a HashMap<String, Value>, c: &'static str) -> Result<&'a str, ErreurEtat> {
    l.get(c).and_then(Value::as_str).ok_or(ErreurEtat::Illisible("colonne texte"))
}

fn i64_de(v: u64) -> Result<i64, ErreurEtat> {
    i64::try_from(v).map_err(|_| ErreurEtat::TropGrand)
}

/// Pointeur relu d'une ligne `etat_courant`.
fn pointeur_de(l: &HashMap<String, Value>) -> Result<Pointeur, ErreurEtat> {
    let empreinte: [u8; 32] = de_hex(texte(l, "empreinte")?, 32)
        .and_then(|v| v.try_into().ok())
        .ok_or(ErreurEtat::Illisible("empreinte"))?;
    let generation = entier(l, "generation")?;
    if generation == 0 {
        return Err(ErreurEtat::Illisible("génération"));
    }
    Ok(Pointeur {
        generation,
        morceaux: u32::try_from(entier(l, "morceaux")?).map_err(|_| ErreurEtat::Illisible("morceaux"))?,
        taille: entier(l, "taille")?,
        empreinte,
    })
}

/// Support KindMother de l'état fédéral (voir module).
pub struct SupportKindMother {
    client: KindMotherClient,
}

impl std::fmt::Debug for SupportKindMother {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SupportKindMother").field("base", &self.client.database()).finish_non_exhaustive()
    }
}

impl SupportKindMother {
    /// Connexion authentifiée de l'opérateur [`OPERATEUR_DEFAUT`] à sa base
    /// (justificatif et pin du serveur dans `credentials`, à défaut
    /// emplacement standard de KindMother), puis préparation ([`Self::depuis_client`]).
    pub async fn connecter(adresse: &str, credentials: Option<&Path>) -> Result<Self, ErreurEtat> {
        Self::connecter_operateur(adresse, OPERATEUR_DEFAUT, credentials).await
    }

    /// Comme [`Self::connecter`] pour un opérateur propriétaire de la base du
    /// même nom ; nom refusé par [`nom_valide`] ⇒ refus sans connexion.
    pub async fn connecter_operateur(
        adresse: &str,
        operateur: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, ErreurEtat> {
        if !nom_valide(operateur) {
            return Err(ErreurEtat::Support(format!(
                "KindMother : base {:?} refusée pour l'état fédéral",
                operateur.chars().take(LONGUEUR_MAX_NOM).collect::<String>()
            )));
        }
        let justificatif = match credentials {
            Some(dir) => OperatorCredential::load_from_dir(operateur, dir),
            None => OperatorCredential::load(operateur),
        }
        .map_err(|e| support(&e))?;
        let client = KindMotherClient::connect(adresse, &justificatif, operateur).await.map_err(|e| support(&e))?;
        Self::depuis_client(client).await
    }

    /// Support sur un client déjà connecté (base par défaut du client) :
    /// migration du schéma, puis contrôle du chiffrement de la base (base en
    /// clair ⇒ refus). Ne contrôle pas le nom de la base (intégration, tests).
    pub async fn depuis_client(client: KindMotherClient) -> Result<Self, ErreurEtat> {
        client.apply_migrations(migrations()).await.map_err(|e| support(&e))?;
        let info = client.database_info().await.map_err(|e| support(&e))?;
        if !info.encrypted {
            return Err(ErreurEtat::Support("KindMother : base de l'état fédéral non chiffrée".to_string()));
        }
        Ok(Self { client })
    }

    /// Nom de la base.
    pub fn base(&self) -> &str {
        self.client.database()
    }

    async fn pointeur(&self) -> Result<Option<Pointeur>, ErreurEtat> {
        let lignes = self
            .client
            .select(TypedSelect::from("etat_courant").filter("cle", CLE_POINTEUR))
            .await
            .map_err(|e| support(&e))?;
        match lignes.as_slice() {
            [] => Ok(None),
            [l] => pointeur_de(l).map(Some),
            _ => Err(ErreurEtat::Illisible("pointeur multiple")),
        }
    }

    async fn lire_etat(&self) -> Result<Option<Vec<u8>>, ErreurEtat> {
        let Some(p) = self.pointeur().await? else { return Ok(None) };
        // Bornes du pointeur contrôlées avant toute lecture de morceau.
        if p.morceaux == 0 || p.morceaux as usize > crate::support::MAX_MORCEAUX {
            return Err(ErreurEtat::Illisible("pointeur hors bornes"));
        }
        let generation = i64_de(p.generation)?;
        let mut morceaux: Vec<(u64, String)> = Vec::with_capacity(p.morceaux as usize);
        for rang in 0..p.morceaux {
            let lignes = self
                .client
                .select(
                    TypedSelect::from("etat_morceaux")
                        .filter("generation", generation)
                        .filter("rang", i64::from(rang)),
                )
                .await
                .map_err(|e| support(&e))?;
            let [l] = lignes.as_slice() else { return Err(ErreurEtat::Illisible("morceau absent ou multiple")) };
            if entier(l, "generation")? != p.generation || entier(l, "rang")? != u64::from(rang) {
                return Err(ErreurEtat::Illisible("morceau d'une autre génération"));
            }
            let t = texte(l, "octets")?;
            if t.len() > 2 * TAILLE_MORCEAU {
                return Err(ErreurEtat::Illisible("morceau trop grand"));
            }
            morceaux.push((u64::from(rang), t.to_string()));
        }
        let vue: Vec<(u64, &str)> = morceaux.iter().map(|(r, t)| (*r, t.as_str())).collect();
        assembler_morceaux(&p, &vue).map(Some)
    }

    async fn soumettre(&self, lot: Vec<TypedWriteIntent>) -> Result<Vec<kindmother_client::IntentOutcome>, ErreurEtat> {
        if lot.is_empty() {
            return Ok(Vec::new());
        }
        self.client.submit_batch(lot).await.map_err(|e| support(&e))
    }

    async fn ecrire_etat(&self, octets: &[u8]) -> Result<(), ErreurEtat> {
        let courant = self.pointeur().await?;
        let ancienne = courant.map_or(0, |p| p.generation);
        let nouvelle = ancienne.checked_add(1).ok_or(ErreurEtat::TropGrand)?;
        let (p, morceaux) = decouper(octets, nouvelle)?;
        let g = i64_de(nouvelle)?;

        let mut nettoyage =
            vec![TypedWriteIntent::delete("etat_morceaux").key("generation", g).purpose("federation_etat_restes")];
        if ancienne >= 2 {
            nettoyage.push(
                TypedWriteIntent::delete("etat_morceaux")
                    .key("generation", i64_de(ancienne - 1)?)
                    .purpose("federation_etat_perime"),
            );
        }
        // Rangs < MAX_MORCEAUX (borné par `decouper`) : la conversion ne peut pas échouer.
        let inserts: Vec<TypedWriteIntent> = (0u32..)
            .zip(morceaux)
            .map(|(rang, hexa)| {
                TypedWriteIntent::insert("etat_morceaux")
                    .value("generation", g)
                    .value("rang", i64::from(rang))
                    .value("octets", hexa)
                    .purpose("federation_etat_morceau")
            })
            .collect();
        let bascule = match courant {
            None => TypedWriteIntent::insert("etat_courant").value("cle", CLE_POINTEUR),
            Some(_) => TypedWriteIntent::update("etat_courant")
                .key("cle", CLE_POINTEUR)
                .key("generation", i64_de(ancienne)?),
        }
        .value("generation", g)
        .value("morceaux", i64::from(p.morceaux))
        .value("taille", i64_de(p.taille)?)
        .value("empreinte", hex(&p.empreinte))
        .purpose("federation_etat_bascule");

        let issues = if inserts.len() <= MORCEAUX_PAR_LOT {
            // Tout dans une transaction.
            let mut lot = nettoyage;
            lot.extend(inserts);
            lot.push(bascule);
            self.soumettre(lot).await?
        } else {
            self.soumettre(nettoyage).await?;
            let mut inserts = inserts;
            while !inserts.is_empty() {
                let reste = inserts.split_off(inserts.len().min(MORCEAUX_PAR_LOT));
                self.soumettre(std::mem::replace(&mut inserts, reste)).await?;
            }
            self.soumettre(vec![bascule]).await?
        };
        // La bascule (dernière intention) doit avoir déplacé exactement le pointeur.
        if issues.last().map(|o| o.rows_affected) != Some(1) {
            return Err(ErreurEtat::Support(
                "KindMother : pointeur de l'état déplacé par un autre écrivain".to_string(),
            ));
        }
        // Bascule faite : la génération précédente n'est plus lisible, ses
        // morceaux sont purgés sans attendre (voir module).
        if ancienne >= 1 {
            self.soumettre(vec![TypedWriteIntent::delete("etat_morceaux")
                .key("generation", i64_de(ancienne)?)
                .purpose("federation_etat_perime")])
                .await?;
        }
        Ok(())
    }
}

impl SupportEtat for SupportKindMother {
    fn lire(&self) -> impl Future<Output = Result<Option<Vec<u8>>, ErreurEtat>> + Send {
        self.lire_etat()
    }

    fn ecrire(&self, octets: &[u8]) -> impl Future<Output = Result<(), ErreurEtat>> + Send {
        self.ecrire_etat(octets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noms_de_base() {
        assert!(nom_valide(OPERATEUR_DEFAUT));
        for refuse in ["", "maia-alice", "cog-federee", "Federation", "-a", "a/b", &"a".repeat(65)] {
            assert!(!nom_valide(refuse), "{refuse:?}");
        }
    }
}
