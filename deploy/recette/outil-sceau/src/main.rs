//! Outil de signature du MANIFESTE des paquets COG de recette.
//!
//! Sous-commandes :
//! - `generer-cle --sortie FICHIER` : crée une clé d'éditeur (graine Ed25519 en hexadécimal, mode 0600,
//!   refuse d'écraser un fichier existant) et affiche sa clé publique et son empreinte (kid).
//! - `info-cle --cle FICHIER` : affiche la clé publique et l'empreinte d'une graine.
//! - `sceller --cle FICHIER --manifeste M --paquet N --version V --commit C --arbre T --sortie S`
//!   : produit un sceau Sceau v1.1 de type ManifestePaquet lié au SHA-256 de M.
//! - `signer-brut --cle FICHIER --fichier F --sortie S` : signature Ed25519 détachée brute (64 octets).
//! - `cle-publique-pem --cle FICHIER --sortie PEM` : clé publique au format PEM pour openssl.
//! - `verifier --cle-publique HEX --manifeste M --sceau S` : vérifie le sceau contre UNE clé d'éditeur
//!   épinglée (tout autre signataire est refusé), puis le SHA-256 de M. Code 0 si valide.
//!
//! Aucun secret n'est écrit sur la sortie standard.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use miyusceau::{
    sceller, verifier, AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, LecteurTlv, NiveauConfiance, Titre,
    TypeSceau,
};
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

/// Titre signé : identité d'un paquet et empreinte de son MANIFESTE.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifesteTitre {
    paquet: String,
    version: String,
    commit: String,
    arbre: String,
    manifeste_sha256: [u8; 32],
    date_epoch: u64,
}

impl Titre for ManifesteTitre {
    const TYPE: TypeSceau = TypeSceau::ManifestePaquet;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.texte(1, &self.paquet)?;
        e.texte(2, &self.version)?;
        e.texte(3, &self.commit)?;
        e.texte(4, &self.arbre)?;
        e.b32(5, &self.manifeste_sha256)?;
        e.u64(6, self.date_epoch)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        Ok(Self {
            paquet: l.exiger_texte(1)?.to_owned(),
            version: l.exiger_texte(2)?.to_owned(),
            commit: l.exiger_texte(3)?.to_owned(),
            arbre: l.exiger_texte(4)?.to_owned(),
            manifeste_sha256: l.exiger_b32(5)?,
            date_epoch: l.exiger_u64(6)?,
        })
    }
}

fn usage() -> &'static str {
    "usage : outil-sceau generer-cle --sortie FICHIER\n\
     \x20      outil-sceau info-cle --cle FICHIER\n\
     \x20      outil-sceau sceller --cle FICHIER --manifeste M --paquet N --version V --commit C --arbre T --sortie S\n\
     \x20      outil-sceau verifier --cle-publique HEX --manifeste M --sceau S"
}

fn hex_vers(octets: &[u8]) -> String {
    octets.iter().map(|o| format!("{o:02x}")).collect()
}

fn hex_depuis(texte: &str) -> Result<Vec<u8>, String> {
    let t = texte.trim();
    if t.len() % 2 != 0 || !t.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("hexadécimal invalide".into());
    }
    (0..t.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&t[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

/// Lit une option `--nom valeur` ; `None` si absente.
fn option(args: &[String], nom: &str) -> Result<Option<String>, String> {
    match args.iter().position(|a| a == nom) {
        None => Ok(None),
        Some(i) => args
            .get(i + 1)
            .cloned()
            .map(Some)
            .ok_or_else(|| format!("{nom} : valeur attendue")),
    }
}

fn requise(args: &[String], nom: &str) -> Result<String, String> {
    option(args, nom)?.ok_or_else(|| format!("option obligatoire absente : {nom}"))
}

fn charger_cle(chemin: &Path) -> Result<CleSignature, String> {
    let texte = fs::read_to_string(chemin).map_err(|e| format!("lecture de la clé : {e}"))?;
    let octets = hex_depuis(&texte)?;
    let graine: [u8; 32] = octets.as_slice().try_into().map_err(|_| "graine : 32 octets attendus".to_string())?;
    Ok(CleSignature::depuis_graine(&graine))
}

fn lire_cle_publique(valeur: &str) -> Result<[u8; 32], String> {
    let texte = match fs::read_to_string(Path::new(valeur)) {
        Ok(t) => t,
        Err(_) => valeur.to_string(),
    };
    let octets = hex_depuis(&texte)?;
    octets.as_slice().try_into().map_err(|_| "clé publique : 32 octets attendus".to_string())
}

fn generer_cle(args: &[String]) -> Result<(), String> {
    let sortie = PathBuf::from(requise(args, "--sortie")?);
    let mut fichier = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&sortie)
        .map_err(|e| format!("création de {} refusée (existe déjà ?) : {e}", sortie.display()))?;
    let cle = CleSignature::generer();
    writeln!(fichier, "{}", hex_vers(&cle.graine())).map_err(|e| e.to_string())?;
    println!("cle_publique {}", hex_vers(&cle.cle_publique()));
    println!("empreinte {}", hex_vers(&cle.kid()));
    Ok(())
}

fn info_cle(args: &[String]) -> Result<(), String> {
    let cle = charger_cle(Path::new(&requise(args, "--cle")?))?;
    println!("cle_publique {}", hex_vers(&cle.cle_publique()));
    println!("empreinte {}", hex_vers(&cle.kid()));
    Ok(())
}

fn maintenant() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn sha256_fichier(chemin: &Path) -> Result<[u8; 32], String> {
    let octets = fs::read(chemin).map_err(|e| format!("lecture de {} : {e}", chemin.display()))?;
    Ok(Sha256::digest(&octets).into())
}

fn sceller_manifeste(args: &[String]) -> Result<(), String> {
    let cle = charger_cle(Path::new(&requise(args, "--cle")?))?;
    let manifeste = PathBuf::from(requise(args, "--manifeste")?);
    let titre = ManifesteTitre {
        paquet: requise(args, "--paquet")?,
        version: requise(args, "--version")?,
        commit: requise(args, "--commit")?,
        arbre: requise(args, "--arbre")?,
        manifeste_sha256: sha256_fichier(&manifeste)?,
        date_epoch: maintenant(),
    };
    let sceau = sceller(&titre, &cle).map_err(|e| format!("scellement refusé : {e}"))?;
    let sortie = PathBuf::from(requise(args, "--sortie")?);
    fs::write(&sortie, &sceau).map_err(|e| format!("écriture de {} : {e}", sortie.display()))?;
    println!("sceau {} octets, signataire {}", sceau.len(), hex_vers(&cle.kid()));
    Ok(())
}

fn verifier_manifeste(args: &[String]) -> Result<(), String> {
    let cle_pub = lire_cle_publique(&requise(args, "--cle-publique")?)?;
    let manifeste = PathBuf::from(requise(args, "--manifeste")?);
    let sceau_chemin = PathBuf::from(requise(args, "--sceau")?);
    let sceau = fs::read(&sceau_chemin).map_err(|e| format!("lecture du sceau : {e}"))?;

    let mut ancres = AncresDeConfiance::new();
    ancres
        .ajouter(&cle_pub, NiveauConfiance::Souverain)
        .map_err(|e| format!("ancre refusée : {e}"))?;
    let verifie = verifier::<ManifesteTitre>(&sceau, &ancres, maintenant())
        .map_err(|e| format!("sceau refusé : {e}"))?;
    let titre = verifie.titre();
    let empreinte = sha256_fichier(&manifeste)?;
    if empreinte != titre.manifeste_sha256 {
        return Err("MANIFESTE altéré : SHA-256 différent de celui scellé".into());
    }
    println!("sceau valide : signataire {}", hex_vers(verifie.signataire()));
    println!("paquet {} version {}", titre.paquet, titre.version);
    println!("commit {} arbre {}", titre.commit, titre.arbre);
    println!("manifeste_sha256 {}", hex_vers(&titre.manifeste_sha256));
    Ok(())
}

/// Signature détachée Ed25519 brute (64 octets, RFC 8032, sans contexte) : vérifiable par
/// `openssl pkeyutl -verify -rawin`. Sert à authentifier le MANIFESTE avant toute exécution.
fn signer_brut(args: &[String]) -> Result<(), String> {
    let graine = charger_graine(Path::new(&requise(args, "--cle")?))?;
    let fichier = PathBuf::from(requise(args, "--fichier")?);
    let message = fs::read(&fichier).map_err(|e| format!("lecture de {} : {e}", fichier.display()))?;
    let signature = SigningKey::from_bytes(&graine).sign(&message);
    let sortie = PathBuf::from(requise(args, "--sortie")?);
    fs::write(&sortie, signature.to_bytes()).map_err(|e| format!("écriture de {} : {e}", sortie.display()))?;
    Ok(())
}

/// Clé publique Ed25519 au format SubjectPublicKeyInfo PEM (lisible par openssl).
fn cle_publique_pem(args: &[String]) -> Result<(), String> {
    let graine = charger_graine(Path::new(&requise(args, "--cle")?))?;
    let pub_octets = SigningKey::from_bytes(&graine).verifying_key().to_bytes();
    let mut der = vec![0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00];
    der.extend_from_slice(&pub_octets);
    let b64 = base64::engine::general_purpose::STANDARD.encode(&der);
    let mut pem = String::from("-----BEGIN PUBLIC KEY-----\n");
    for morceau in b64.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(morceau).map_err(|e| e.to_string())?);
        pem.push('\n');
    }
    pem.push_str("-----END PUBLIC KEY-----\n");
    let sortie = PathBuf::from(requise(args, "--sortie")?);
    fs::write(&sortie, pem).map_err(|e| format!("écriture de {} : {e}", sortie.display()))?;
    Ok(())
}

fn charger_graine(chemin: &Path) -> Result<[u8; 32], String> {
    let texte = fs::read_to_string(chemin).map_err(|e| format!("lecture de la clé : {e}"))?;
    let octets = hex_depuis(&texte)?;
    octets.as_slice().try_into().map_err(|_| "graine : 32 octets attendus".to_string())
}

fn principal() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("generer-cle") => generer_cle(&args),
        Some("info-cle") => info_cle(&args),
        Some("sceller") => sceller_manifeste(&args),
        Some("verifier") => verifier_manifeste(&args),
        Some("signer-brut") => signer_brut(&args),
        Some("cle-publique-pem") => cle_publique_pem(&args),
        _ => Err(usage().to_string()),
    }
}

fn main() -> ExitCode {
    match principal() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("outil-sceau : {e}");
            ExitCode::from(1)
        }
    }
}
