//! @id     miyuracine_cli
//! @do     conduire_ceremonie_racine
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — outil de cérémonie hors ligne de la racine COG.

//! `miyuracine` : outil de cérémonie **hors ligne** de la racine COG (EX-24, EX-25).
//!
//! À exécuter sur un poste isolé (aucun code réseau dans ce crate). Le secret
//! maître n'est jamais affiché ni écrit ; seules les parts SLIP-39 (fichiers
//! `0600` ou sortie standard) et, sur demande explicite, la graine d'**un**
//! usage (`--exporter-graine-usage`, fichier `0600` neuf) quittent la mémoire,
//! ainsi que le fichier de clé chiffré (`exporter-cle`, format `MYRC` v1).
//!
//! Avant toute lecture de secret, le processus est durci
//! ([`miyuracine::durcir_processus`]) : core dumps désactivés, mémoire
//! verrouillée (`mlockall`) — refus si impossible, sauf `--sans-mlock`
//! (dérogation explicite, avertissement sur la sortie d'erreur).

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use miyuracine::fichier_cle::{lire_parametres, MEMOIRE_ARGON2_MAX};
use miyuracine::{
    durcir_processus, ecrire_fichier_cle, lire_fichier_cle, lire_fichier_secret, verifier_passation, ParametresArgon2,
    PartsRacine, Passation, PolitiqueVerrouillage, Racine, Usage, EXPOSANT_DEFAUT, NOMBRE_PARTS,
};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

/// Option commune : fichier de phrase secrète.
const PHRASE: &str = "--phrase-fichier";
/// Option commune : fichier de clé chiffré (alternative aux deux parts).
const CLE: &str = "--cle";
/// Option commune : fichier du mot de passe du fichier de clé.
const MDP: &str = "--mot-de-passe-fichier";
/// Option d'`exporter-cle` : nouveau mot de passe du fichier écrit.
const NOUVEAU_MDP: &str = "--nouveau-mot-de-passe-fichier";
/// Option globale : dérogation au verrouillage mémoire.
const SANS_MLOCK: &str = "--sans-mlock";

/// Taille maximale d'un fichier de part ou de phrase (octets).
const TAILLE_MAX_FICHIER: u64 = 4096;

const AIDE: &str = "\
miyuracine — cérémonie hors ligne de la racine COG (SLIP-39 2 sur 3, SLIP-10 par usage)

À exécuter sur un poste isolé du réseau. Le secret maître n'est jamais affiché ni écrit.

UTILISATION
  miyuracine [--sans-mlock] <commande> [options]

  miyuracine generer   [--dossier <rép>] [--exposant <0-15>] [--phrase-fichier <f>]
      Génère une racine et ses 3 parts (2 suffisent). Avec --dossier : fichiers
      part-<i>-sur-3.txt créés en 0600 (refus s'ils existent) ; sinon sortie standard.

  miyuracine recuperer --part <f> --part <f> [--phrase-fichier <f>]
      Reconstitue la racine depuis 2 parts ; affiche cog_id, clés publiques et kid.

  miyuracine deriver   --part <f> --part <f> --usage <u> [--generation <n>] [--phrase-fichier <f>]
      Affiche la clé publique et le kid d'un usage.

  miyuracine deriver   --part <f> --part <f> --exporter-graine-usage <u> --generation <n>
                       --sortie <fichier> [--phrase-fichier <f>]
      Écrit la graine Ed25519 (32 octets bruts) d'UN usage dans un fichier neuf en 0600,
      pour l'étape de signature hors ligne. Refus si le fichier existe.

  miyuracine rotation  --part <f> --part <f> --passation <fichier> [--dossier <rép>]
                       [--exposant <0-15>] [--phrase-fichier <f>] [--nouvelle-phrase-fichier <f>]
      Passation : nouvelle racine + 3 nouvelles parts, et sceau « l'ancienne racine se
      porte garante de la nouvelle » signé par les deux identités (fichier neuf).

  miyuracine exporter-cle --part <f> --part <f> --sortie <fichier> --mot-de-passe-fichier <f>
                       [--phrase-fichier <f>] [--argon2-memoire-mio <256-4096>] [--argon2-iterations <3-64>]
                       [--nouveau-mot-de-passe-fichier <f>]
      Écrit la racine chiffrée (Argon2id + XChaCha20-Poly1305, format MYRC v1, 122 octets)
      dans un fichier neuf en 0600 (écriture atomique), puis la relit pour vérification.
      Défaut : 1024 Mio, 4 itérations. Mot de passe : 12 octets au moins.
      Changer de mot de passe : --cle <ancien> --mot-de-passe-fichier <ancien mdp>
      --nouveau-mot-de-passe-fichier <nouveau mdp> --sortie <nouveau fichier>.

  Source de la racine : partout où deux --part sont demandées, on peut donner à la place
  --cle <fichier> --mot-de-passe-fichier <f>.

  Fichiers secrets (parts, phrases, mots de passe, fichier de clé) : fichier ordinaire,
  0600 ou plus strict, propriétaire = utilisateur effectif, pas de lien symbolique ; sinon refus.

DURCISSEMENT
  Core dumps désactivés (RLIMIT_CORE = 0, PR_SET_DUMPABLE = 0) et mémoire verrouillée
  (mlockall) avant toute lecture de secret. Si la mémoire ne peut être verrouillée
  (RLIMIT_MEMLOCK insuffisant : root, CAP_IPC_LOCK ou « ulimit -l unlimited »), refus ;
  --sans-mlock lève ce refus (avertissement, à éviter hors essais).

USAGES
  identite (génération 0 uniquement), emission, journal, editeur, arretes

PHRASE SECRÈTE
  Facultative (ASCII imprimable), lue dans un fichier ; vide par défaut. Sans
  --nouvelle-phrase-fichier, la rotation reprend la phrase de l'ancienne racine.
";

// ─── Arguments ─────────────────────────────────────────────────────────────

#[derive(Default)]
struct Options {
    parts: Vec<PathBuf>,
    dossier: Option<PathBuf>,
    exposant: Option<u8>,
    phrase_fichier: Option<PathBuf>,
    nouvelle_phrase_fichier: Option<PathBuf>,
    usage: Option<Usage>,
    exporter: Option<Usage>,
    generation: Option<u32>,
    sortie: Option<PathBuf>,
    passation: Option<PathBuf>,
    cle: Option<PathBuf>,
    mot_de_passe_fichier: Option<PathBuf>,
    nouveau_mot_de_passe_fichier: Option<PathBuf>,
    argon2_memoire_mio: Option<u32>,
    argon2_iterations: Option<u32>,
}

fn erreur(message: impl Into<String>) -> String {
    message.into()
}

fn definir<T>(champ: &mut Option<T>, valeur: T, nom: &str) -> Result<(), String> {
    if champ.is_some() {
        return Err(erreur(format!("option {nom} répétée")));
    }
    *champ = Some(valeur);
    Ok(())
}

fn analyser(arguments: &[String], autorisees: &[&str]) -> Result<Options, String> {
    let mut o = Options::default();
    let mut iter = arguments.iter();
    while let Some(nom) = iter.next() {
        if !autorisees.contains(&nom.as_str()) {
            return Err(erreur(format!(
                "option inattendue pour cette commande : {nom}"
            )));
        }
        let valeur = iter
            .next()
            .ok_or_else(|| erreur(format!("valeur manquante pour {nom}")))?;
        match nom.as_str() {
            "--part" => o.parts.push(PathBuf::from(valeur)),
            "--dossier" => definir(&mut o.dossier, PathBuf::from(valeur), nom)?,
            "--exposant" => {
                let e: u8 = valeur
                    .parse()
                    .map_err(|_| erreur("exposant invalide (0 à 15)"))?;
                if e > 15 {
                    return Err(erreur("exposant invalide (0 à 15)"));
                }
                definir(&mut o.exposant, e, nom)?;
            }
            "--phrase-fichier" => definir(&mut o.phrase_fichier, PathBuf::from(valeur), nom)?,
            "--nouvelle-phrase-fichier" => {
                definir(&mut o.nouvelle_phrase_fichier, PathBuf::from(valeur), nom)?;
            }
            "--usage" => definir(
                &mut o.usage,
                valeur.parse().map_err(|_| erreur("usage inconnu"))?,
                nom,
            )?,
            "--exporter-graine-usage" => {
                definir(
                    &mut o.exporter,
                    valeur.parse().map_err(|_| erreur("usage inconnu"))?,
                    nom,
                )?;
            }
            "--generation" => {
                let g: u32 = valeur.parse().map_err(|_| erreur("génération invalide"))?;
                definir(&mut o.generation, g, nom)?;
            }
            "--sortie" => definir(&mut o.sortie, PathBuf::from(valeur), nom)?,
            "--passation" => definir(&mut o.passation, PathBuf::from(valeur), nom)?,
            "--cle" => definir(&mut o.cle, PathBuf::from(valeur), nom)?,
            "--mot-de-passe-fichier" => {
                definir(&mut o.mot_de_passe_fichier, PathBuf::from(valeur), nom)?;
            }
            "--nouveau-mot-de-passe-fichier" => {
                definir(&mut o.nouveau_mot_de_passe_fichier, PathBuf::from(valeur), nom)?;
            }
            "--argon2-memoire-mio" => {
                let m: u32 = valeur
                    .parse()
                    .map_err(|_| erreur("mémoire Argon2id invalide (Mio)"))?;
                definir(&mut o.argon2_memoire_mio, m, nom)?;
            }
            "--argon2-iterations" => {
                let t: u32 = valeur
                    .parse()
                    .map_err(|_| erreur("itérations Argon2id invalides"))?;
                definir(&mut o.argon2_iterations, t, nom)?;
            }
            _ => return Err(erreur(format!("option inconnue : {nom}"))),
        }
    }
    Ok(o)
}

// ─── Fichiers ──────────────────────────────────────────────────────────────

/// Lit un fichier secret (part, phrase, mot de passe) : contrôles stricts
/// communs avec le fichier de clé ([`lire_fichier_secret`] : `O_NOFOLLOW`,
/// fichier ordinaire, `0600` ou plus strict, propriétaire = uid effectif,
/// au plus [`TAILLE_MAX_FICHIER`] octets), texte UTF-8.
fn lire_secret(chemin: &Path) -> Result<Zeroizing<String>, String> {
    let mut octets = lire_fichier_secret(chemin, TAILLE_MAX_FICHIER)
        .map_err(|e| erreur(format!("lecture de {} : {e}", chemin.display())))?;
    match String::from_utf8(std::mem::take(&mut *octets)) {
        Ok(texte) => Ok(Zeroizing::new(texte)),
        Err(e) => {
            // Octets invalides effacés avant libération.
            drop(Zeroizing::new(e.into_bytes()));
            Err(erreur(format!("lecture de {} : texte non UTF-8", chemin.display())))
        }
    }
}

fn lire_phrase(chemin: Option<&Path>) -> Result<Zeroizing<Vec<u8>>, String> {
    let Some(chemin) = chemin else {
        return Ok(Zeroizing::new(Vec::new()));
    };
    let texte = lire_secret(chemin)?;
    let sans_fin = texte.strip_suffix('\n').unwrap_or(&texte);
    let sans_fin = sans_fin.strip_suffix('\r').unwrap_or(sans_fin);
    Ok(Zeroizing::new(sans_fin.as_bytes().to_vec()))
}

/// Crée un fichier **neuf** (refus s'il existe) avec les droits donnés.
#[cfg(unix)]
fn creer_neuf(chemin: &Path, mode: u32) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(chemin)
}

#[cfg(not(unix))]
fn creer_neuf(_chemin: &Path, _mode: u32) -> io::Result<File> {
    // Droits 0600 non garantis hors Unix : refus (fail-closed).
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "écriture de secrets réservée aux systèmes Unix",
    ))
}

fn ecrire_neuf(chemin: &Path, mode: u32, octets: &[u8]) -> Result<(), String> {
    ecrire_neuf_avec(chemin, mode, |f| f.write_all(octets).and_then(|()| f.sync_all()))
}

/// Crée `chemin` (neuf) et y écrit par `ecrire` ; en cas d'échec d'écriture,
/// le fichier **partiel** (créé ici même) est supprimé.
fn ecrire_neuf_avec(
    chemin: &Path,
    mode: u32,
    ecrire: impl FnOnce(&mut File) -> io::Result<()>,
) -> Result<(), String> {
    let mut f = creer_neuf(chemin, mode)
        .map_err(|e| erreur(format!("création de {} : {e}", chemin.display())))?;
    if let Err(e) = ecrire(&mut f) {
        drop(f);
        let suppression = match fs::remove_file(chemin) {
            Ok(()) => "fichier partiel supprimé".to_string(),
            Err(s) => format!("fichier partiel NON supprimé ({s}) : à effacer"),
        };
        return Err(erreur(format!(
            "écriture de {} : {e} ; {suppression}",
            chemin.display()
        )));
    }
    Ok(())
}

fn chemins_parts(dossier: &Path) -> Result<Vec<PathBuf>, String> {
    if !dossier.is_dir() {
        return Err(erreur(format!(
            "{} n'est pas un répertoire",
            dossier.display()
        )));
    }
    let chemins: Vec<PathBuf> = (1..=NOMBRE_PARTS)
        .map(|i| dossier.join(format!("part-{i}-sur-{NOMBRE_PARTS}.txt")))
        .collect();
    if let Some(existant) = chemins.iter().find(|c| fs::symlink_metadata(c).is_ok()) {
        return Err(erreur(format!(
            "{} existe déjà : refus d'écraser une part",
            existant.display()
        )));
    }
    Ok(chemins)
}

fn emettre_parts(parts: &PartsRacine, chemins: Option<&[PathBuf]>) -> Result<(), String> {
    let t = parts.toutes();
    match chemins {
        Some(chemins) => {
            for (i, (part, chemin)) in t.iter().zip(chemins).enumerate() {
                let mut contenu = Zeroizing::new(String::with_capacity(part.len() + 1));
                contenu.push_str(part);
                contenu.push('\n');
                ecrire_neuf(chemin, 0o600, contenu.as_bytes())?;
                println!(
                    "part {}/{NOMBRE_PARTS} écrite : {} (0600)",
                    i + 1,
                    chemin.display()
                );
            }
        }
        None => {
            for (i, part) in t.iter().enumerate() {
                println!("part {}/{NOMBRE_PARTS} : {part}", i + 1);
            }
        }
    }
    eprintln!(
        "Rangez chaque part sur un support distinct (USB, papier) ; deux parts suffisent à reconstituer la racine."
    );
    Ok(())
}

// ─── Affichage ─────────────────────────────────────────────────────────────

fn hex(octets: &[u8]) -> String {
    use std::fmt::Write as _;
    octets
        .iter()
        .fold(String::with_capacity(octets.len() * 2), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

fn afficher_cle(racine: &Racine, usage: Usage, generation: u32) -> Result<(), String> {
    let cle = racine.cle(usage, generation).map_err(|e| e.to_string())?;
    let chemin = usage.chemin_texte(generation).map_err(|e| e.to_string())?;
    println!(
        "{usage:<9} g{generation:<3} {chemin:<26} clé {}  kid {}",
        hex(&cle.cle_publique()),
        hex(&cle.kid())
    );
    Ok(())
}

fn afficher_identite(racine: &Racine) -> Result<(), String> {
    println!("cog_id : {}", racine.cog_id());
    afficher_cle(racine, Usage::Identite, 0)
}

// ─── Commandes ─────────────────────────────────────────────────────────────

fn lire_mot_de_passe(o: &Options) -> Result<Zeroizing<Vec<u8>>, String> {
    let chemin = o
        .mot_de_passe_fichier
        .as_deref()
        .ok_or_else(|| erreur("--mot-de-passe-fichier <f> est requis"))?;
    lire_phrase(Some(chemin))
}

/// Mot de passe du fichier écrit par `exporter-cle` : le nouveau s'il est
/// donné (changement de mot de passe), sinon celui de `--mot-de-passe-fichier`.
fn lire_mot_de_passe_sortie(o: &Options) -> Result<Zeroizing<Vec<u8>>, String> {
    match o.nouveau_mot_de_passe_fichier.as_deref() {
        Some(chemin) => lire_phrase(Some(chemin)),
        None => lire_mot_de_passe(o),
    }
}

fn recuperer_racine(o: &Options) -> Result<Racine, String> {
    if let Some(cle) = o.cle.as_deref() {
        if !o.parts.is_empty() || o.phrase_fichier.is_some() {
            return Err(erreur(
                "--cle exclut --part et --phrase-fichier (la phrase SLIP-39 ne s'applique qu'aux parts)",
            ));
        }
        let mot_de_passe = lire_mot_de_passe(o)?;
        return lire_fichier_cle(cle, &mot_de_passe)
            .map_err(|e| erreur(format!("{} : {e}", cle.display())));
    }
    if o.parts.len() != 2 {
        return Err(erreur(
            "exactement deux --part sont requises (ou --cle et --mot-de-passe-fichier)",
        ));
    }
    let phrase = lire_phrase(o.phrase_fichier.as_deref())?;
    let textes = o
        .parts
        .iter()
        .map(|p| lire_secret(p))
        .collect::<Result<Vec<_>, _>>()?;
    let vues: Vec<&str> = textes.iter().map(|t| t.as_str()).collect();
    Racine::recuperer(&vues, &phrase).map_err(|e| erreur(format!("récupération refusée : {e}")))
}

fn generer(o: &Options) -> Result<(), String> {
    let phrase = lire_phrase(o.phrase_fichier.as_deref())?;
    let chemins = o.dossier.as_deref().map(chemins_parts).transpose()?;
    let racine = Racine::generer().map_err(|e| e.to_string())?;
    let parts = racine
        .partager(&phrase, o.exposant.unwrap_or(EXPOSANT_DEFAUT))
        .map_err(|e| e.to_string())?;
    println!("Racine COG générée ; chaque paire de parts a été vérifiée.");
    afficher_identite(&racine)?;
    emettre_parts(&parts, chemins.as_deref())
}

fn recuperer(o: &Options) -> Result<(), String> {
    let racine = recuperer_racine(o)?;
    println!("Racine reconstituée (clés publiques, génération 0) :");
    afficher_identite(&racine)?;
    for usage in Usage::TOUS.into_iter().filter(|u| u.a_generations()) {
        afficher_cle(&racine, usage, 0)?;
    }
    Ok(())
}

fn deriver(o: &Options) -> Result<(), String> {
    match (o.usage, o.exporter) {
        (Some(usage), None) => {
            if o.sortie.is_some() {
                return Err(erreur(
                    "--sortie n'a de sens qu'avec --exporter-graine-usage",
                ));
            }
            let racine = recuperer_racine(o)?;
            afficher_cle(&racine, usage, o.generation.unwrap_or(0))
        }
        (None, Some(usage)) => {
            let sortie = o
                .sortie
                .as_deref()
                .ok_or_else(|| erreur("--sortie <fichier> est requis pour l'export"))?;
            let generation = match (usage.a_generations(), o.generation) {
                (true, None) => {
                    return Err(erreur(
                        "--generation est requis pour exporter une clé d'usage",
                    ))
                }
                (_, g) => g.unwrap_or(0),
            };
            if fs::symlink_metadata(sortie).is_ok() {
                return Err(erreur(format!("{} existe déjà : refus", sortie.display())));
            }
            let racine = recuperer_racine(o)?;
            let graine = racine
                .graine(usage, generation)
                .map_err(|e| e.to_string())?;
            ecrire_neuf(sortie, 0o600, graine.as_slice())?;
            if usage == Usage::Identite {
                eprintln!("ATTENTION : graine de l'IDENTITÉ racine exportée ; ne jamais la copier sur un serveur.");
            }
            println!(
                "graine écrite : {} (0600, 32 octets bruts)",
                sortie.display()
            );
            afficher_cle(&racine, usage, generation)
        }
        _ => Err(erreur(
            "préciser exactement une option : --usage ou --exporter-graine-usage",
        )),
    }
}

fn rotation(o: &Options) -> Result<(), String> {
    let fichier_passation = o
        .passation
        .as_deref()
        .ok_or_else(|| erreur("--passation <fichier> est requis"))?;
    if fs::symlink_metadata(fichier_passation).is_ok() {
        return Err(erreur(format!(
            "{} existe déjà : refus",
            fichier_passation.display()
        )));
    }
    let chemins = o.dossier.as_deref().map(chemins_parts).transpose()?;
    let ancienne = recuperer_racine(o)?;
    let nouvelle_phrase = match o.nouvelle_phrase_fichier.as_deref() {
        Some(f) => lire_phrase(Some(f))?,
        None => lire_phrase(o.phrase_fichier.as_deref())?,
    };

    let nouvelle = Racine::generer().map_err(|e| e.to_string())?;
    let parts = nouvelle
        .partager(&nouvelle_phrase, o.exposant.unwrap_or(EXPOSANT_DEFAUT))
        .map_err(|e| e.to_string())?;
    let maintenant = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| erreur("horloge système antérieure à 1970 : refus"))?
        .as_secs();
    let passation = Passation {
        ancienne_racine: ancienne.cle_publique_identite(),
        nouvelle_racine: nouvelle.cle_publique_identite(),
        emise_le: maintenant,
    };
    let cle_ancienne = ancienne
        .cle(Usage::Identite, 0)
        .map_err(|e| e.to_string())?;
    let cle_nouvelle = nouvelle
        .cle(Usage::Identite, 0)
        .map_err(|e| e.to_string())?;
    let sceau = passation
        .sceller(&cle_ancienne, &cle_nouvelle)
        .map_err(|e| e.to_string())?;
    // Contrôle avant toute écriture : le sceau se vérifie avec l'ancienne racine épinglée.
    verifier_passation(&sceau, &passation.ancienne_racine, maintenant)
        .map_err(|e| e.to_string())?;

    println!("Passation de racine (rotation) :");
    println!(
        "ancienne : cog_id {}  clé {}",
        ancienne.cog_id(),
        hex(&passation.ancienne_racine)
    );
    println!(
        "nouvelle : cog_id {}  clé {}",
        nouvelle.cog_id(),
        hex(&passation.nouvelle_racine)
    );
    emettre_parts(&parts, chemins.as_deref())?;
    ecrire_neuf(fichier_passation, 0o644, &sceau)?;
    println!(
        "sceau de passation écrit : {} ({} octets, SHA-256 {})",
        fichier_passation.display(),
        sceau.len(),
        hex(&Sha256::digest(&sceau))
    );
    eprintln!("Détruisez les anciennes parts une fois la passation publiée et les nouvelles parts rangées.");
    Ok(())
}

fn parametres_argon2(o: &Options) -> Result<ParametresArgon2, String> {
    let defaut = ParametresArgon2::DEFAUT;
    let m_kio = match o.argon2_memoire_mio {
        Some(mio) => mio
            .checked_mul(1024)
            .ok_or_else(|| erreur("mémoire Argon2id hors bornes"))?,
        None => defaut.memoire_kio(),
    };
    ParametresArgon2::nouveaux(
        m_kio,
        o.argon2_iterations.unwrap_or(defaut.iterations()),
        defaut.parallelisme(),
    )
    .map_err(|e| e.to_string())
}

fn exporter_cle(o: &Options) -> Result<(), String> {
    let sortie = o
        .sortie
        .as_deref()
        .ok_or_else(|| erreur("--sortie <fichier> est requis"))?;
    let params = parametres_argon2(o)?;
    if fs::symlink_metadata(sortie).is_ok() {
        return Err(erreur(format!("{} existe déjà : refus", sortie.display())));
    }
    let racine = recuperer_racine(o)?;
    let mot_de_passe = lire_mot_de_passe_sortie(o)?;
    ecrire_fichier_cle(sortie, &racine, &mot_de_passe, params)
        .map_err(|e| erreur(format!("{} : {e}", sortie.display())))?;
    // Relecture complète avant d'annoncer le succès (fail-closed).
    let relue = lire_fichier_cle(sortie, &mot_de_passe)
        .map_err(|e| erreur(format!("relecture de {} : {e}", sortie.display())))?;
    if relue.cle_publique_identite() != racine.cle_publique_identite() {
        let _ = fs::remove_file(sortie);
        return Err(erreur("relecture : racine différente, fichier supprimé"));
    }
    println!(
        "fichier de clé écrit et vérifié : {} (0600, MYRC v1, Argon2id {} Mio, t = {}, p = {})",
        sortie.display(),
        params.memoire_kio() / 1024,
        params.iterations(),
        params.parallelisme()
    );
    afficher_identite(&racine)
}

/// Mémoire Argon2id que la commande pourra demander (budget de `mlockall`).
fn budget_argon2(commande: &str, o: &Options) -> Result<u64, String> {
    let lecture = match o.cle.as_deref() {
        Some(cle) => lire_parametres(cle)
            .map_err(|e| erreur(format!("{} : {e}", cle.display())))?
            .memoire_octets(),
        None => 0,
    };
    let ecriture = if commande == "exporter-cle" {
        parametres_argon2(o)?.memoire_octets()
    } else {
        0
    };
    Ok(lecture.max(ecriture).min(MEMOIRE_ARGON2_MAX))
}

fn durcir(commande: &str, o: &Options, politique: PolitiqueVerrouillage) -> Result<(), String> {
    let budget = budget_argon2(commande, o)?;
    let etat = durcir_processus(politique, budget).map_err(|e| e.to_string())?;
    if !etat.memoire_verrouillee {
        eprintln!(
            "AVERTISSEMENT : --sans-mlock — mémoire NON verrouillée, les secrets peuvent atteindre le swap. \
             Core dumps désactivés. À éviter hors essais."
        );
    }
    Ok(())
}

fn executer(arguments: &[String]) -> Result<(), String> {
    let (politique, arguments) = match arguments.split_first() {
        Some((premier, reste)) if premier == SANS_MLOCK => (PolitiqueVerrouillage::Avertir, reste),
        _ => (PolitiqueVerrouillage::Exiger, arguments),
    };
    let Some((commande, reste)) = arguments.split_first() else {
        return Err(erreur("commande manquante (voir « miyuracine aide »)"));
    };
    let autorisees: &[&str] = match commande.as_str() {
        "generer" => &["--dossier", "--exposant", PHRASE],
        "recuperer" => &["--part", PHRASE, CLE, MDP],
        "deriver" => &[
            "--part",
            PHRASE,
            CLE,
            MDP,
            "--usage",
            "--generation",
            "--exporter-graine-usage",
            "--sortie",
        ],
        "rotation" => &[
            "--part",
            PHRASE,
            CLE,
            MDP,
            "--nouvelle-phrase-fichier",
            "--dossier",
            "--exposant",
            "--passation",
        ],
        "exporter-cle" => &[
            "--part",
            PHRASE,
            CLE,
            MDP,
            NOUVEAU_MDP,
            "--sortie",
            "--argon2-memoire-mio",
            "--argon2-iterations",
        ],
        "aide" | "--aide" | "-h" | "--help" => {
            print!("{AIDE}");
            return Ok(());
        }
        autre => {
            return Err(erreur(format!(
                "commande inconnue : {autre} (voir « miyuracine aide »)"
            )))
        }
    };
    let o = analyser(reste, autorisees)?;
    // Durcissement AVANT toute lecture de part, phrase ou mot de passe.
    durcir(commande, &o, politique)?;
    match commande.as_str() {
        "generer" => generer(&o),
        "recuperer" => recuperer(&o),
        "deriver" => deriver(&o),
        "rotation" => rotation(&o),
        "exporter-cle" => exporter_cle(&o),
        _ => unreachable!("commande filtrée ci-dessus"),
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match executer(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("miyuracine : {message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn dossier(nom: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("miyuracine-bin-{nom}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    /// B5 : échec d'écriture ⇒ le fichier partiel (créé ici) est supprimé ;
    /// un fichier préexistant n'est jamais touché.
    #[test]
    fn ecrire_neuf_supprime_le_fichier_partiel() {
        let d = dossier("partiel");
        let chemin = d.join("part-1-sur-3.txt");
        let e = ecrire_neuf_avec(&chemin, 0o600, |f| {
            f.write_all(b"debut de part")?;
            Err(io::Error::other("disque plein"))
        })
        .unwrap_err();
        assert!(e.contains("fichier partiel supprimé"), "{e}");
        assert!(!chemin.exists());

        fs::write(&chemin, b"existant").unwrap();
        assert!(ecrire_neuf(&chemin, 0o600, b"neuf").is_err());
        assert_eq!(fs::read(&chemin).unwrap(), b"existant");

        let neuf = d.join("neuf");
        ecrire_neuf(&neuf, 0o600, b"contenu").unwrap();
        assert_eq!(fs::read(&neuf).unwrap(), b"contenu");
        fs::remove_dir_all(&d).unwrap();
    }
}
