//! @id     miyuracine_racine
//! @do     gerer_racine_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « racine ».

//! Racine COG : secret maître de 256 bits, généré par `OsRng`, sauvegardé en
//! SLIP-39 **2 sur 3** (un groupe, deux parts sur trois), jamais écrit en clair.
//!
//! Les clés d'usage sont dérivées en SLIP-10 durcie (graine SLIP-10 = secret
//! maître, comme le prévoit SLIP-39 pour les portefeuilles hiérarchiques), selon
//! le registre de [`crate::usages`].

use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::erreurs::ErreurRacine;
use crate::slip10::NoeudSlip10;
use crate::slip39::{self, Groupe};
use crate::usages::Usage;

/// Taille du secret maître (octets) : 256 bits.
pub const TAILLE_SECRET: usize = 32;
/// Seuil du partage de la racine.
pub const SEUIL: u8 = 2;
/// Nombre de parts de la racine.
pub const NOMBRE_PARTS: u8 = 3;
/// Exposant d'itération par défaut : 10 000 × 2⁵ = 320 000 itérations PBKDF2.
pub const EXPOSANT_DEFAUT: u8 = 5;
/// Longueur d'un `cog_id` (26 caractères base32 = 130 bits).
pub const LONGUEUR_COG_ID: usize = 26;

const ALPHABET_BASE32: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

/// Les trois parts mnémoniques de la racine (effacées à la destruction).
pub struct PartsRacine {
    parts: [Zeroizing<String>; 3],
}

impl PartsRacine {
    /// Part `i` (0, 1 ou 2), en mots SLIP-39.
    pub fn part(&self, i: usize) -> Option<&str> {
        self.parts.get(i).map(|p| p.as_str())
    }

    /// Les trois parts.
    pub fn toutes(&self) -> [&str; 3] {
        [&self.parts[0], &self.parts[1], &self.parts[2]]
    }
}

impl std::fmt::Debug for PartsRacine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PartsRacine { <masqué> }")
    }
}

/// Racine COG en mémoire (effacée à la destruction ; ni `Clone`, ni affichage).
pub struct Racine {
    secret: Zeroizing<[u8; TAILLE_SECRET]>,
}

impl std::fmt::Debug for Racine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Racine")
            .field("cog_id", &self.cog_id())
            .field("secret", &"<masqué>")
            .finish()
    }
}

/// Calcule le `cog_id` d'une clé publique : `base32(SHA-256(clé))[0..26]`
/// (RFC 4648, minuscules, sans remplissage), comme `miyuwebway_participant`.
pub fn cog_id_depuis_cle(cle_publique: &[u8; 32]) -> String {
    let empreinte: [u8; 32] = Sha256::digest(cle_publique).into();
    let mut sortie = String::with_capacity(LONGUEUR_COG_ID);
    let mut tampon: u32 = 0;
    let mut bits: u32 = 0;
    for octet in empreinte {
        tampon = (tampon << 8) | u32::from(octet);
        bits += 8;
        while bits >= 5 && sortie.len() < LONGUEUR_COG_ID {
            bits -= 5;
            sortie.push(char::from(
                ALPHABET_BASE32[((tampon >> bits) & 0x1f) as usize],
            ));
        }
        tampon &= (1 << bits) - 1;
        if sortie.len() >= LONGUEUR_COG_ID {
            break;
        }
    }
    sortie
}

impl Racine {
    /// Génère une nouvelle racine depuis l'aléa du système (`OsRng`).
    pub fn generer() -> Result<Self, ErreurRacine> {
        let mut secret = Zeroizing::new([0u8; TAILLE_SECRET]);
        OsRng
            .try_fill_bytes(secret.as_mut_slice())
            .map_err(|_| ErreurRacine::AleaIndisponible)?;
        Ok(Self { secret })
    }

    /// Reconstruit une racine depuis un secret maître connu (tests, migration).
    pub fn depuis_secret(secret: &[u8; TAILLE_SECRET]) -> Self {
        Self {
            secret: Zeroizing::new(*secret),
        }
    }

    /// Secret maître (usage interne au crate : fichier de clé chiffré).
    pub(crate) fn secret(&self) -> &[u8; TAILLE_SECRET] {
        &self.secret
    }

    /// Partage la racine en 3 parts SLIP-39 dont 2 suffisent (un groupe).
    ///
    /// La phrase secrète (ASCII imprimable, éventuellement vide) chiffre le
    /// secret maître avant partage ; elle n'est vérifiable par aucun moyen.
    pub fn partager(&self, phrase: &[u8], exposant: u8) -> Result<PartsRacine, ErreurRacine> {
        let mut groupes = slip39::generer_parts(
            1,
            &[Groupe {
                seuil: SEUIL,
                nombre: NOMBRE_PARTS,
            }],
            self.secret.as_slice(),
            phrase,
            exposant,
        )?;
        let mut membres = groupes
            .pop()
            .ok_or(ErreurRacine::ParametresInvalides("aucun groupe produit"))?;
        if membres.len() != usize::from(NOMBRE_PARTS) {
            return Err(ErreurRacine::ParametresInvalides(
                "nombre de parts inattendu",
            ));
        }
        let p2 = membres.pop().ok_or(ErreurRacine::AucunePart)?;
        let p1 = membres.pop().ok_or(ErreurRacine::AucunePart)?;
        let p0 = membres.pop().ok_or(ErreurRacine::AucunePart)?;
        let parts = PartsRacine {
            parts: [p0, p1, p2],
        };
        // Contrôle de cérémonie : chaque paire reconstitue exactement cette racine.
        self.verifier_parts(&parts, phrase)?;
        Ok(parts)
    }

    /// Vérifie que **chaque** paire de parts reconstitue cette racine.
    pub fn verifier_parts(&self, parts: &PartsRacine, phrase: &[u8]) -> Result<(), ErreurRacine> {
        let t = parts.toutes();
        for (a, b) in [(0, 1), (0, 2), (1, 2)] {
            let r = Self::recuperer(&[t[a], t[b]], phrase)?;
            if !r.egale(self) {
                return Err(ErreurRacine::EmpreinteInvalide);
            }
        }
        Ok(())
    }

    /// Reconstitue la racine depuis exactement 2 parts.
    ///
    /// Refus : une seule part, plus de deux parts, part corrompue (somme de
    /// contrôle), parts d'origines différentes, schéma autre que
    /// « 1 groupe, 2 sur N, 256 bits », empreinte invalide.
    pub fn recuperer(parts: &[&str], phrase: &[u8]) -> Result<Self, ErreurRacine> {
        let decodees = parts
            .iter()
            .map(|t| slip39::decoder_part(t))
            .collect::<Result<Vec<_>, _>>()?;
        if decodees.is_empty() {
            return Err(ErreurRacine::AucunePart);
        }
        for p in &decodees {
            let q = p.parametres;
            if q.seuil_groupes != 1
                || q.nombre_groupes != 1
                || q.seuil_membres != SEUIL
                || q.longueur_valeur != TAILLE_SECRET
            {
                return Err(ErreurRacine::SchemaRacineInattendu);
            }
        }
        if decodees.len() != usize::from(SEUIL) {
            return Err(ErreurRacine::NombrePartsInvalide);
        }
        let secret = slip39::combiner_decodees(&decodees, phrase)?;
        let tableau: [u8; TAILLE_SECRET] = secret
            .as_slice()
            .try_into()
            .map_err(|_| ErreurRacine::LongueurSecretInvalide)?;
        let racine = Self::depuis_secret(&tableau);
        let mut tableau = tableau;
        zeroize::Zeroize::zeroize(&mut tableau);
        Ok(racine)
    }

    /// Égalité des secrets, sans sortie anticipée.
    fn egale(&self, autre: &Self) -> bool {
        self.secret
            .iter()
            .zip(autre.secret.iter())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
    }

    fn noeud_maitre(&self) -> Result<NoeudSlip10, ErreurRacine> {
        NoeudSlip10::maitre(self.secret.as_slice())
    }

    /// Nœud SLIP-10 d'un usage (clé privée + code de chaîne).
    pub fn noeud(&self, usage: Usage, generation: u32) -> Result<NoeudSlip10, ErreurRacine> {
        self.noeud_maitre()?.deriver(&usage.chemin(generation)?)
    }

    /// Clé de signature d'un usage, prête pour `miyusceau::sceller`.
    pub fn cle(
        &self,
        usage: Usage,
        generation: u32,
    ) -> Result<miyusceau::CleSignature, ErreurRacine> {
        Ok(self.noeud(usage, generation)?.cle_signature())
    }

    /// Graine Ed25519 d'un usage, pour l'export hors ligne explicite (à effacer).
    pub fn graine(
        &self,
        usage: Usage,
        generation: u32,
    ) -> Result<Zeroizing<[u8; 32]>, ErreurRacine> {
        Ok(self.noeud(usage, generation)?.cle_privee())
    }

    /// Clé publique d'un usage.
    pub fn cle_publique(&self, usage: Usage, generation: u32) -> Result<[u8; 32], ErreurRacine> {
        Ok(self.cle(usage, generation)?.cle_publique())
    }

    /// Clé publique racine du COG (usage `identite`).
    pub fn cle_publique_identite(&self) -> [u8; 32] {
        // Le chemin d'identité, génération 0, est toujours valide.
        self.cle_publique(Usage::Identite, 0)
            .unwrap_or_else(|_| unreachable!())
    }

    /// `cog_id` = empreinte de la clé publique racine.
    pub fn cog_id(&self) -> String {
        cog_id_depuis_cle(&self.cle_publique_identite())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cog_id_vecteur_connu() {
        // Même méthode que `miyuwebway_participant` : bits de SHA-256 par paquets de 5.
        let h: [u8; 32] = Sha256::digest([0u8; 32]).into();
        let bits = h.iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:08b}");
            s
        });
        let attendu: String = bits
            .as_bytes()
            .chunks(5)
            .take(LONGUEUR_COG_ID)
            .map(|c| {
                char::from(
                    ALPHABET_BASE32
                        [usize::from_str_radix(std::str::from_utf8(c).unwrap(), 2).unwrap()],
                )
            })
            .collect();
        assert_eq!(cog_id_depuis_cle(&[0u8; 32]), attendu);
        assert_eq!(attendu.len(), LONGUEUR_COG_ID);
    }

    #[test]
    fn debug_ne_montre_pas_le_secret() {
        let r = Racine::depuis_secret(&[0xab; 32]);
        let d = format!("{r:?}");
        assert!(d.contains("<masqué>"));
        assert!(!d.contains("171"));
    }
}
