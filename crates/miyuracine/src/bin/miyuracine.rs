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
//! usage (`--exporter-graine-usage`, fichier `0600` neuf) quittent la mémoire.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use miyuracine::{
    verifier_passation, PartsRacine, Passation, Racine, Usage, EXPOSANT_DEFAUT, NOMBRE_PARTS,
};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

/// Option commune : fichier de phrase secrète.
const PHRASE: &str = "--phrase-fichier";

/// Taille maximale d'un fichier de part ou de phrase (octets).
const TAILLE_MAX_FICHIER: u64 = 4096;

const AIDE: &str = "\
miyuracine — cérémonie hors ligne de la racine COG (SLIP-39 2 sur 3, SLIP-10 par usage)

À exécuter sur un poste isolé du réseau. Le secret maître n'est jamais affiché ni écrit.

UTILISATION
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

USAGES
  identite (génération 0 uniquement), emission, journal, editeur

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
            _ => return Err(erreur(format!("option inconnue : {nom}"))),
        }
    }
    Ok(o)
}

// ─── Fichiers ──────────────────────────────────────────────────────────────

fn lire_secret(chemin: &Path) -> Result<Zeroizing<String>, String> {
    let fichier =
        File::open(chemin).map_err(|e| erreur(format!("lecture de {} : {e}", chemin.display())))?;
    let mut texte = Zeroizing::new(String::new());
    fichier
        .take(TAILLE_MAX_FICHIER + 1)
        .read_to_string(&mut texte)
        .map_err(|e| erreur(format!("lecture de {} : {e}", chemin.display())))?;
    if texte.len() as u64 > TAILLE_MAX_FICHIER {
        return Err(erreur(format!("{} : fichier trop grand", chemin.display())));
    }
    Ok(texte)
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
    let mut f = creer_neuf(chemin, mode)
        .map_err(|e| erreur(format!("création de {} : {e}", chemin.display())))?;
    f.write_all(octets)
        .and_then(|()| f.sync_all())
        .map_err(|e| erreur(format!("écriture de {} : {e}", chemin.display())))
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

fn recuperer_racine(o: &Options) -> Result<Racine, String> {
    if o.parts.len() != 2 {
        return Err(erreur("exactement deux --part sont requises"));
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

fn executer(arguments: &[String]) -> Result<(), String> {
    let Some((commande, reste)) = arguments.split_first() else {
        return Err(erreur("commande manquante (voir « miyuracine aide »)"));
    };
    match commande.as_str() {
        "generer" => generer(&analyser(reste, &["--dossier", "--exposant", PHRASE])?),
        "recuperer" => recuperer(&analyser(reste, &["--part", PHRASE])?),
        "deriver" => deriver(&analyser(
            reste,
            &[
                "--part",
                PHRASE,
                "--usage",
                "--generation",
                "--exporter-graine-usage",
                "--sortie",
            ],
        )?),
        "rotation" => rotation(&analyser(
            reste,
            &[
                "--part",
                PHRASE,
                "--nouvelle-phrase-fichier",
                "--dossier",
                "--exposant",
                "--passation",
            ],
        )?),
        "aide" | "--aide" | "-h" | "--help" => {
            print!("{AIDE}");
            Ok(())
        }
        autre => Err(erreur(format!(
            "commande inconnue : {autre} (voir « miyuracine aide »)"
        ))),
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
