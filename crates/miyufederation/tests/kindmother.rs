//! @id     miyufederation_test_kindmother
//! @do     verifier_persistance_federale_kindmother
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — support KindMother sur un serveur réel (SQLCipher) : redémarrage après chaque transition, morceaux et générations, altération en base refusée.

//! Support KindMother de l'état fédéral sur un **KindMother réel**
//! (`kindmother_service::spawn_ephemeral`, SQLCipher).
//!
//! L'opérateur de production `miyufederation` n'est pas encore déclaré par
//! KindMother (`register_default_operators`, hors de cette crate) : les tests
//! empruntent un opérateur propriétaire de sa base déclaré par le fichier
//! `maia-utilisateurs` (`maia-federation-test`), via
//! [`SupportKindMother::depuis_client`] qui ne contrôle pas le nom.

#![cfg(feature = "kindmother")]
#![allow(clippy::many_single_char_names)]

mod commun;

use std::path::{Path, PathBuf};

use commun::scenarios::{self, fondateurs, premier, registre, SECRET};
use commun::{CogTest, T0};
use kindmother_client::{KindMotherClient, OperatorCredential, TypedSelect, TypedWriteIntent};
use miyufederation::kindmother::{nom_valide, OPERATEUR_DEFAUT};
use miyufederation::support::TAILLE_MORCEAU;
use miyufederation::{CompteurFichier, ErreurEtat, FederationPersistante, SupportEtat, SupportKindMother};

const OPERATEUR: &str = "maia-federation-test";

struct Serveur {
    adresse: String,
    credentials: PathBuf,
    dossier: PathBuf,
}

impl Drop for Serveur {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dossier);
    }
}

async fn serveur() -> Serveur {
    let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dossier = std::env::temp_dir().join(format!("miyufederation-km-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dossier).unwrap();
    std::fs::write(dossier.join("maia-utilisateurs"), "federation-test\n").unwrap();
    let (addr, credentials) = kindmother_service::spawn_ephemeral(&dossier).await.unwrap();
    Serveur { adresse: addr.to_string(), credentials, dossier }
}

async fn client(s: &Serveur) -> KindMotherClient {
    let cred = OperatorCredential::load_from_dir(OPERATEUR, &s.credentials).unwrap();
    KindMotherClient::connect(&s.adresse, &cred, OPERATEUR).await.unwrap()
}

/// Nouvelle connexion (le « redémarrage » du processus de fédération).
async fn support(s: &Serveur) -> SupportKindMother {
    SupportKindMother::depuis_client(client(s).await).await.unwrap()
}

/// Compteur de fraîcheur local, **hors de KindMother** (dossier du COG).
fn compteur(s: &Serveur) -> CompteurFichier {
    CompteurFichier::ouvrir(&s.dossier.join("federation-local"), &SECRET).unwrap()
}

/// Redémarrage : nouvelle connexion KindMother et compteur local rouvert.
async fn ouvrir(s: &Serveur) -> (SupportKindMother, CompteurFichier) {
    (support(s).await, compteur(s))
}

macro_rules! sur_kindmother {
    ($($nom:ident),* $(,)?) => {
        $(
            #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
            async fn $nom() {
                let s = serveur().await;
                scenarios::$nom(async || ouvrir(&s).await).await;
            }
        )*
    };
}

sur_kindmother!(admission, sanction, contestation, vue_note_carnet, horloge_en_recul);

async fn lignes(c: &KindMotherClient, table: &str) -> usize {
    c.select(TypedSelect::from(table)).await.unwrap().len()
}

/// Octets opaques : aller-retour sur 1, 2 et 6 morceaux (lots multiples),
/// générations successives, morceaux périmés purgés.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn morceaux_generations_et_purge() {
    let s = serveur().await;
    let sup = support(&s).await;
    assert_eq!(sup.base(), OPERATEUR);
    assert_eq!(SupportEtat::lire(&sup).await.unwrap(), None);
    let c = client(&s).await;
    for (i, taille) in [41usize, TAILLE_MORCEAU + 1, 5 * TAILLE_MORCEAU + 3, 7].into_iter().enumerate() {
        let o: Vec<u8> = (0..taille).map(|k| (k * 31 + i) as u8).collect();
        SupportEtat::ecrire(&sup, &o).await.unwrap();
        assert_eq!(SupportEtat::lire(&sup).await.unwrap().as_deref(), Some(o.as_slice()), "taille {taille}");
        // Nouvelle connexion : même état.
        assert_eq!(SupportEtat::lire(&support(&s).await).await.unwrap().as_deref(), Some(o.as_slice()));
        let p = c.select(TypedSelect::from("etat_courant")).await.unwrap();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].get("generation").and_then(serde_json::Value::as_i64), Some(i64::try_from(i).unwrap() + 1));
    }
    // Seule la génération courante reste : la précédente est purgée dès la bascule.
    assert_eq!(lignes(&c, "etat_morceaux").await, 1);
}

/// Altération dans la base (morceau, pointeur, suppression) : refus au
/// rechargement, jamais de vue.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn alteration_en_base_refusee() {
    let s = serveur().await;
    let reg = registre();
    let f = fondateurs(4);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier(ouvrir(&s).await, &reg).await;
    for x in &f {
        p.ajouter_fondateur(&reg, &x.candidature, T0).await.unwrap();
    }
    p.recevoir_candidature(&reg, &a.candidature, T0).await.unwrap();
    for x in &f[..2] {
        p.recevoir_confirmation(&reg, &x.confirmer(&a, T0 + 60), T0 + 60).await.unwrap();
    }
    p.actualiser(&reg, T0 + 60).await.unwrap();
    drop(p);
    let c = client(&s).await;
    let pointeur = c.select(TypedSelect::from("etat_courant")).await.unwrap().remove(0);
    let g = pointeur.get("generation").and_then(serde_json::Value::as_i64).unwrap();
    let morceau = c.select(TypedSelect::from("etat_morceaux").filter("generation", g)).await.unwrap().remove(0);
    let hexa = morceau.get("octets").and_then(serde_json::Value::as_str).unwrap().to_string();
    let charger = async |t: u64| {
        scenarios::charger_avec(ouvrir(&s).await, t, &reg, true).await.map(|r| r.0)
    };
    let modifier = async |table: &str, cle: (&str, i64), colonne: &str, valeur: kindmother_client::SqlValue| {
        c.submit(TypedWriteIntent::update(table).key(cle.0, cle.1).value(colonne, valeur).purpose("test_alteration"))
            .await
            .unwrap();
    };

    // Un caractère du morceau changé, empreinte recalculée par l'attaquant : HMAC faux.
    let mut faux = hexa.clone().into_bytes();
    faux[200] = if faux[200] == b'0' { b'1' } else { b'0' };
    let faux = String::from_utf8(faux).unwrap();
    let octets_faux = miyufederation::support::de_hex(&faux, TAILLE_MORCEAU).unwrap();
    let empreinte = {
        use sha2::Digest;
        miyufederation::support::hex(&sha2::Sha256::digest(&octets_faux))
    };
    modifier("etat_morceaux", ("generation", g), "octets", faux.as_str().into()).await;
    assert!(matches!(charger(T0 + 61).await, Err(ErreurEtat::Illisible(_))), "empreinte non recalculée");
    modifier("etat_courant", ("generation", g), "empreinte", empreinte.as_str().into()).await;
    assert!(matches!(charger(T0 + 61).await, Err(ErreurEtat::Altere)));
    // Pointeur menteur.
    modifier("etat_morceaux", ("generation", g), "octets", hexa.as_str().into()).await;
    modifier("etat_courant", ("generation", g), "taille", 3_i64.into()).await;
    assert!(charger(T0 + 61).await.is_err());
    // Suppression du pointeur alors qu'un état était attendu.
    c.submit(TypedWriteIntent::delete("etat_courant").key("cle", "courant").purpose("test_suppression")).await.unwrap();
    assert!(matches!(charger(T0 + 61).await, Err(ErreurEtat::EtatAbsent)));
}

/// Retour arrière dans KindMother : un ancien couple pointeur + morceaux
/// (copié par un acteur capable d'écrire dans la base, puis restauré
/// directement) est authentique mais refusé par le compteur local ; même
/// génération et contenu différent : refusé ; compteur local absent : refus
/// explicite.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn retour_arriere_dans_kindmother_refuse() {
    use kindmother_client::SqlValue;
    let s = serveur().await;
    let reg = registre();
    let f = fondateurs(4);
    let a = CogTest::nouveau(30, T0);
    let c = client(&s).await;
    let instantane = async || {
        let p = c.select(TypedSelect::from("etat_courant")).await.unwrap().remove(0);
        let g = p.get("generation").and_then(serde_json::Value::as_i64).unwrap();
        let m = c.select(TypedSelect::from("etat_morceaux").filter("generation", g)).await.unwrap();
        (p, m)
    };
    let restaurer = async |(p, m): &(
        std::collections::HashMap<String, serde_json::Value>,
        Vec<std::collections::HashMap<String, serde_json::Value>>,
    )| {
        let entier = |l: &std::collections::HashMap<String, serde_json::Value>, k: &str| l[k].as_i64().unwrap();
        let mut lot = vec![TypedWriteIntent::delete("etat_courant").key("cle", "courant").purpose("test_restauration")];
        for l in m {
            lot.push(
                TypedWriteIntent::upsert("etat_morceaux")
                    .key("generation", entier(l, "generation"))
                    .key("rang", entier(l, "rang"))
                    .value("octets", SqlValue::Text(l["octets"].as_str().unwrap().to_string()))
                    .purpose("test_restauration"),
            );
        }
        lot.push(
            TypedWriteIntent::insert("etat_courant")
                .value("cle", "courant")
                .value("generation", entier(p, "generation"))
                .value("morceaux", entier(p, "morceaux"))
                .value("taille", entier(p, "taille"))
                .value("empreinte", SqlValue::Text(p["empreinte"].as_str().unwrap().to_string()))
                .purpose("test_restauration"),
        );
        c.submit_batch(lot).await.unwrap();
    };

    let mut p = premier(ouvrir(&s).await, &reg).await;
    for x in &f {
        p.ajouter_fondateur(&reg, &x.candidature, T0).await.unwrap();
    }
    p.recevoir_candidature(&reg, &a.candidature, T0).await.unwrap();
    let candidat = instantane().await;
    for x in &f[..2] {
        p.recevoir_confirmation(&reg, &x.confirmer(&a, T0 + 60), T0 + 60).await.unwrap();
    }
    p.actualiser(&reg, T0 + 60).await.unwrap();
    assert_eq!(p.federation().statut(&a.cog), Some(miyufederation::Statut::Invite));
    drop(p);
    let invite = instantane().await;
    let invite_octets = SupportEtat::lire(&support(&s).await).await.unwrap().unwrap();
    // Morceaux de l'état candidat purgés dès la bascule suivante.
    let g_candidat = candidat.0["generation"].as_i64().unwrap();
    let restes = c.select(TypedSelect::from("etat_morceaux").filter("generation", g_candidat)).await.unwrap();
    assert_eq!(restes.len(), 0, "morceaux de la génération précédente encore présents");

    // Restauration de l'ancien couple pointeur + morceaux : refus.
    restaurer(&candidat).await;
    let r = scenarios::charger_avec(ouvrir(&s).await, T0 + 61, &reg, true).await.map(|r| r.0);
    assert!(matches!(r, Err(ErreurEtat::EtatPerime { .. })), "{r:?}");

    // Même génération, contenu différent : la branche écrite depuis l'état
    // candidat avec un compteur rejoué, puis remplacée par l'état « invité ».
    let dossier_branche = s.dossier.join("branche");
    {
        // Le compteur rejoué pointe l'état candidat (copie de l'époque).
        let cb = CompteurFichier::ouvrir(&dossier_branche, &SECRET).unwrap();
        let octets = SupportEtat::lire(&support(&s).await).await.unwrap().unwrap();
        let (_, _, _, gc) =
            miyufederation::Federation::reprendre_generation(&octets, &scenarios::cle(), scenarios::parametres(), None, T0 + 61)
                .unwrap();
        miyufederation::CompteurFraicheur::ecrire(&cb, &miyufederation::Fraicheur::de_etat(gc, &octets)).unwrap();
        let mut q = scenarios::charger_avec((support(&s).await, cb), T0 + 61, &reg, true).await.unwrap().0;
        let mut k = 0;
        let g_mfed_invite = {
            let o = invite_octets.clone();
            miyufederation::Federation::reprendre_generation(&o, &scenarios::cle(), scenarios::parametres(), None, T0 + 61)
                .unwrap()
                .3
        };
        while q.generation() < g_mfed_invite {
            q.actualiser(&reg, T0 + 61 + k).await.unwrap();
            k += 1;
        }
    }
    restaurer(&invite).await;
    let cb = CompteurFichier::ouvrir(&dossier_branche, &SECRET).unwrap();
    let r = scenarios::charger_avec((support(&s).await, cb), T0 + 70, &reg, true).await.map(|r| r.0);
    assert!(matches!(r, Err(ErreurEtat::EtatDivergent(_))), "{r:?}");

    // L'état « invité » restauré est accepté par le compteur d'origine.
    let p = scenarios::charger_avec(ouvrir(&s).await, T0 + 70, &reg, true).await.unwrap().0;
    assert_eq!(p.federation().statut(&a.cog), Some(miyufederation::Statut::Invite));
    drop(p);

    // Compteur local perdu : refus explicite, aucune reprise silencieuse.
    std::fs::remove_file(s.dossier.join("federation-local").join(miyufederation::FICHIER_COMPTEUR)).unwrap();
    let r = scenarios::charger_avec(ouvrir(&s).await, T0 + 71, &reg, true).await.map(|r| r.0);
    assert!(matches!(r, Err(ErreurEtat::CompteurAbsent)), "{r:?}");
}

/// Production : opérateur `miyufederation` (déclaré par défaut dans
/// KindMother, A-019), jamais une base `maia-*` ni `cog-federee`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn noms_de_base_et_operateur_non_declare() {
    let s = serveur().await;
    assert!(nom_valide(OPERATEUR_DEFAUT));
    for nom in [OPERATEUR, "cog-federee", "../x"] {
        let e = SupportKindMother::connecter_operateur(&s.adresse, nom, Some(&s.credentials)).await.unwrap_err();
        assert!(matches!(e, ErreurEtat::Support(_)), "{nom} : {e:?}");
    }
    assert!(SupportKindMother::connecter(&s.adresse, Some(Path::new(&s.credentials))).await.is_ok());
}

/// Les opérations du support KindMother sont des futurs `Send`.
#[allow(dead_code)]
fn transitions_send(p: &mut FederationPersistante<SupportKindMother>, reg: &scenarios::Registre) {
    fn send<T: Send>(_: T) {}
    send(p.actualiser(reg, T0));
}
