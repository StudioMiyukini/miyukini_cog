//! @id     miyusceau_registre
//! @do     verifier_tete_registre_fraiche
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « registre » (tête signée, fraîcheur, anti-retour).

//! Tête signée du registre (`Sceau<TeteRegistre>`, type `0x05`) et règles
//! d'acceptation côté vérificateur (EX-03, anti-gel ; lot 2 §8).
//!
//! Champs TLV (tous obligatoires, tags croissants) :
//!
//! | Tag | Champ | Valeur |
//! |---|---|---|
//! | 1 | `registre` | identifiant du registre (32 o) |
//! | 2 | `taille` | nombre de feuilles (`u64`) |
//! | 3 | `racine` | racine Merkle RFC 9162 (32 o) ; `SHA-256("")` si `taille = 0` |
//! | 4 | `sequence` | numéro de séquence de la tête (`u64`) |
//! | 5 | `emise_a` | horodatage d'émission (secondes Unix) |
//! | 6 | `validite_max_secs` | durée de validité annoncée, `1..=`[`AGE_MAX_TETE_PLAFOND_SECS`] |
//!
//! Règles (fail-closed) :
//!
//! - **Fraîcheur** : âge effectif = `min(validite_max_secs, politique.age_max)` ;
//!   `maintenant − tolérance > emise_a + âge effectif` ⇒ [`ErreurRegistre::TeteGelee`].
//! - **Seuil** : la politique fixe le nombre de signataires épinglés requis
//!   (1 pour le journal d'un COG, 2 pour le canal officiel).
//! - **Monotonie** (via [`EtatRegistre`]) : séquence en recul ⇒ retour en
//!   arrière ; même séquence et contenu différent ⇒ équivoque ; taille en
//!   recul ou même taille et racine différente ⇒ équivoque ; horodatage en
//!   recul ⇒ retour en arrière ; taille en hausse ⇒ preuve de cohérence exigée.

use crate::ancres::AncresDeConfiance;
use crate::erreurs::{ErreurRegistre, ErreurSceau, PreuveEquivoque};
use crate::format::{decoder_titre, verifier_seuil, Sceau, TypeSceau, MAX_SIGNATURES, TOLERANCE_HORLOGE_SECS};
use crate::merkle::{racine_vide, Empreinte, PreuveCoherence, PreuveInclusion};
use crate::titre::{Titre, Verifie};
use crate::tlv::{EcrivainTlv, LecteurTlv};

/// Âge maximal par défaut d'une tête (24 h, valeur TUF du rôle `timestamp`).
///
/// La doc gelée (lot 2 §8, EX-03) n'en fixe aucun : valeur conservatrice.
pub const AGE_MAX_TETE_DEFAUT_SECS: u64 = 86_400;

/// Plafond absolu de l'âge d'une tête (7 jours) : ni la politique ni la tête
/// ne peuvent l'excéder.
pub const AGE_MAX_TETE_PLAFOND_SECS: u64 = 7 * 86_400;

const TAG_REGISTRE: u8 = 1;
const TAG_TAILLE: u8 = 2;
const TAG_RACINE: u8 = 3;
const TAG_SEQUENCE: u8 = 4;
const TAG_EMISE_A: u8 = 5;
const TAG_VALIDITE: u8 = 6;

/// Tête signée du registre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeteRegistre {
    /// Identifiant du registre (32 octets, défini par l'émetteur).
    pub registre: [u8; 32],
    /// Nombre de feuilles.
    pub taille: u64,
    /// Racine Merkle (RFC 9162).
    pub racine: Empreinte,
    /// Numéro de séquence (croissant à chaque nouvelle tête).
    pub sequence: u64,
    /// Horodatage d'émission (secondes Unix).
    pub emise_a: u64,
    /// Validité maximale annoncée (secondes, `1..=AGE_MAX_TETE_PLAFOND_SECS`).
    pub validite_max_secs: u64,
}

impl Titre for TeteRegistre {
    const TYPE: TypeSceau = TypeSceau::TeteRegistre;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.b32(TAG_REGISTRE, &self.registre)?
            .u64(TAG_TAILLE, self.taille)?
            .b32(TAG_RACINE, &self.racine)?
            .u64(TAG_SEQUENCE, self.sequence)?
            .u64(TAG_EMISE_A, self.emise_a)?
            .u64(TAG_VALIDITE, self.validite_max_secs)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let tete = Self {
            registre: l.exiger_b32(TAG_REGISTRE)?,
            taille: l.exiger_u64(TAG_TAILLE)?,
            racine: l.exiger_b32(TAG_RACINE)?,
            sequence: l.exiger_u64(TAG_SEQUENCE)?,
            emise_a: l.exiger_u64(TAG_EMISE_A)?,
            validite_max_secs: l.exiger_u64(TAG_VALIDITE)?,
        };
        if tete.validite_max_secs == 0 || tete.validite_max_secs > AGE_MAX_TETE_PLAFOND_SECS {
            return Err(ErreurSceau::ValeurInvalide(TAG_VALIDITE));
        }
        if tete.taille == 0 && tete.racine != racine_vide() {
            return Err(ErreurSceau::ValeurInvalide(TAG_RACINE));
        }
        Ok(tete)
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.emise_a, self.emise_a.saturating_add(self.validite_max_secs)))
    }
}

/// Politique d'acceptation d'une tête : seuil de signataires et âge maximal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolitiqueTete {
    seuil: usize,
    age_max_secs: u64,
}

impl PolitiqueTete {
    /// Journal d'un COG : 1 signataire (clé de journal), 24 h.
    pub const fn journal_cog() -> Self {
        Self { seuil: 1, age_max_secs: AGE_MAX_TETE_DEFAUT_SECS }
    }

    /// Registre du canal officiel (Market Origin, mises à jour) : 2 sur 2, 24 h.
    pub const fn canal_officiel() -> Self {
        Self { seuil: 2, age_max_secs: AGE_MAX_TETE_DEFAUT_SECS }
    }

    /// Politique explicite : `seuil` dans `1..=8`, âge dans `1..=AGE_MAX_TETE_PLAFOND_SECS`.
    pub fn nouvelle(seuil: usize, age_max_secs: u64) -> Result<Self, ErreurRegistre> {
        if seuil == 0 || seuil > MAX_SIGNATURES || age_max_secs == 0 || age_max_secs > AGE_MAX_TETE_PLAFOND_SECS {
            return Err(ErreurRegistre::PolitiqueInvalide);
        }
        Ok(Self { seuil, age_max_secs })
    }

    /// Seuil de signataires épinglés requis.
    pub fn seuil(&self) -> usize {
        self.seuil
    }

    /// Âge maximal accepté (secondes).
    pub fn age_max_secs(&self) -> u64 {
        self.age_max_secs
    }
}

/// Contrôle de fraîcheur (anti-gel) d'une tête à l'instant `maintenant_secs`.
pub fn controler_fraicheur(
    tete: &TeteRegistre,
    politique: &PolitiqueTete,
    maintenant_secs: u64,
) -> Result<(), ErreurRegistre> {
    if maintenant_secs.saturating_add(TOLERANCE_HORLOGE_SECS) < tete.emise_a {
        return Err(ErreurRegistre::Sceau(ErreurSceau::PasEncoreValide));
    }
    let age_max_secs = tete.validite_max_secs.min(politique.age_max_secs);
    if maintenant_secs.saturating_sub(TOLERANCE_HORLOGE_SECS) > tete.emise_a.saturating_add(age_max_secs) {
        return Err(ErreurRegistre::TeteGelee { emise_a: tete.emise_a, age_max_secs });
    }
    Ok(())
}

/// Tête vérifiée (signatures, seuil, registre attendu, fraîcheur) avec les
/// octets de son sceau. Seul [`verifier_tete`] en construit une.
#[derive(Debug, Clone)]
pub struct TeteVerifiee {
    verifie: Verifie<TeteRegistre>,
    sceau: Vec<u8>,
}

impl TeteVerifiee {
    /// Tête vérifiée.
    pub fn tete(&self) -> &TeteRegistre {
        self.verifie.titre()
    }

    /// Résultat complet de la vérification (signataires, niveau).
    pub fn verifie(&self) -> &Verifie<TeteRegistre> {
        &self.verifie
    }

    /// Octets du sceau vérifié.
    pub fn sceau(&self) -> &[u8] {
        &self.sceau
    }
}

/// Vérifie une tête : sceau à seuil (politique), registre attendu, fraîcheur.
///
/// Ne contrôle pas la monotonie : utiliser [`EtatRegistre::verifier_et_accepter`].
pub fn verifier_tete(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    politique: &PolitiqueTete,
    registre_attendu: &[u8; 32],
    maintenant_secs: u64,
) -> Result<TeteVerifiee, ErreurRegistre> {
    let verifie = match verifier_seuil::<TeteRegistre>(octets, ancres, politique.seuil, maintenant_secs) {
        Ok(v) => v,
        Err(ErreurSceau::Expire) => {
            // `Expire` n'est produit qu'après signatures et seuil vérifiés :
            // fenêtre annoncée par la tête dépassée = registre gelé.
            let tete = decoder_titre::<TeteRegistre>(&Sceau::decoder(octets)?.charge)?;
            let age_max_secs = tete.validite_max_secs.min(politique.age_max_secs);
            return Err(ErreurRegistre::TeteGelee { emise_a: tete.emise_a, age_max_secs });
        }
        Err(e) => return Err(e.into()),
    };
    if &verifie.titre().registre != registre_attendu {
        return Err(ErreurRegistre::RegistreInattendu);
    }
    controler_fraicheur(verifie.titre(), politique, maintenant_secs)?;
    Ok(TeteVerifiee { verifie, sceau: octets.to_vec() })
}

/// Issue d'une acceptation réussie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Acceptation {
    /// Nouvelle tête retenue (séquence supérieure).
    Nouvelle,
    /// Même tête que la dernière acceptée (état inchangé).
    Identique,
}

const MAGIC_ETAT: &[u8; 4] = b"MSRG";
const VERSION_ETAT: u8 = 0x01;

/// Dernière tête acceptée d'un registre : à persister par l'appelant (format
/// binaire indépendant du stockage) pour imposer la monotonie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtatRegistre {
    tete: TeteRegistre,
    sceau: Vec<u8>,
}

impl EtatRegistre {
    /// État initial à partir d'une première tête vérifiée (confiance au
    /// premier usage : l'appelant décide de l'amorçage).
    pub fn initial(tete: TeteVerifiee) -> Self {
        Self { tete: tete.verifie.into_titre(), sceau: tete.sceau }
    }

    /// Dernière tête acceptée.
    pub fn tete(&self) -> &TeteRegistre {
        &self.tete
    }

    /// Sceau de la dernière tête acceptée.
    pub fn sceau(&self) -> &[u8] {
        &self.sceau
    }

    /// Vérifie `octets` (voir [`verifier_tete`], registre de l'état) puis
    /// l'accepte selon les règles de monotonie. L'état n'est modifié qu'en
    /// cas de succès.
    pub fn verifier_et_accepter(
        &mut self,
        octets: &[u8],
        ancres: &AncresDeConfiance,
        politique: &PolitiqueTete,
        maintenant_secs: u64,
        coherence: Option<&PreuveCoherence>,
    ) -> Result<Acceptation, ErreurRegistre> {
        let registre = self.tete.registre;
        let tete = verifier_tete(octets, ancres, politique, &registre, maintenant_secs)?;
        self.accepter(tete, coherence)
    }

    /// Accepte une tête déjà vérifiée selon les règles de monotonie.
    pub fn accepter(
        &mut self,
        nouvelle: TeteVerifiee,
        coherence: Option<&PreuveCoherence>,
    ) -> Result<Acceptation, ErreurRegistre> {
        let a = &self.tete;
        let n = nouvelle.tete();
        let equivoque = |etat: &Self| {
            ErreurRegistre::Equivoque(Box::new(PreuveEquivoque {
                acceptee: etat.sceau.clone(),
                contradictoire: nouvelle.sceau.clone(),
            }))
        };
        if n.registre != a.registre {
            return Err(ErreurRegistre::RegistreInattendu);
        }
        if n.sequence < a.sequence {
            return Err(ErreurRegistre::RetourArriere);
        }
        if n.sequence == a.sequence {
            return if n == a { Ok(Acceptation::Identique) } else { Err(equivoque(self)) };
        }
        if n.taille < a.taille || (n.taille == a.taille && n.racine != a.racine) {
            return Err(equivoque(self));
        }
        if n.emise_a < a.emise_a {
            return Err(ErreurRegistre::RetourArriere);
        }
        if n.taille > a.taille && a.taille > 0 {
            let preuve = coherence.ok_or(ErreurRegistre::CoherenceManquante)?;
            if preuve.taille1 != a.taille || preuve.taille2 != n.taille {
                return Err(ErreurRegistre::PreuveInvalide);
            }
            preuve.verifier(&a.racine, &n.racine)?;
        }
        *self = Self::initial(nouvelle);
        Ok(Acceptation::Nouvelle)
    }

    /// Exige que la dernière tête acceptée soit encore fraîche (registre non gelé).
    pub fn exiger_fraicheur(&self, politique: &PolitiqueTete, maintenant_secs: u64) -> Result<&TeteRegistre, ErreurRegistre> {
        controler_fraicheur(&self.tete, politique, maintenant_secs)?;
        Ok(&self.tete)
    }

    /// Vérifie l'inclusion d'une feuille (empreinte RFC 9162) dans la dernière
    /// tête acceptée, à condition qu'elle soit fraîche.
    pub fn verifier_inclusion(
        &self,
        feuille: &Empreinte,
        preuve: &PreuveInclusion,
        politique: &PolitiqueTete,
        maintenant_secs: u64,
    ) -> Result<(), ErreurRegistre> {
        let tete = self.exiger_fraicheur(politique, maintenant_secs)?;
        if preuve.taille != tete.taille {
            return Err(ErreurRegistre::PreuveInvalide);
        }
        preuve.verifier(feuille, &tete.racine)
    }

    /// Encodage : `"MSRG" | version 0x01 | longueur (4, BE) | sceau de la tête`.
    pub fn encoder(&self) -> Vec<u8> {
        let mut sortie = Vec::with_capacity(9 + self.sceau.len());
        sortie.extend_from_slice(MAGIC_ETAT);
        sortie.push(VERSION_ETAT);
        sortie.extend_from_slice(&(self.sceau.len() as u32).to_be_bytes());
        sortie.extend_from_slice(&self.sceau);
        sortie
    }

    /// Décodage strict d'un état persistant. Les signatures du sceau sont
    /// **revérifiées** (ancres et seuil de la politique, défense en profondeur
    /// contre un stockage altéré) ; la fraîcheur ne l'est pas (voir
    /// [`Self::exiger_fraicheur`]).
    pub fn decoder(
        octets: &[u8],
        ancres: &AncresDeConfiance,
        politique: &PolitiqueTete,
        registre_attendu: &[u8; 32],
    ) -> Result<Self, ErreurRegistre> {
        if octets.len() < 9 || &octets[0..4] != MAGIC_ETAT || octets[4] != VERSION_ETAT {
            return Err(ErreurRegistre::EtatInvalide);
        }
        let longueur = u32::from_be_bytes(octets[5..9].try_into().map_err(|_| ErreurRegistre::EtatInvalide)?) as usize;
        let sceau = &octets[9..];
        if sceau.len() != longueur {
            return Err(ErreurRegistre::EtatInvalide);
        }
        let tete = decoder_titre::<TeteRegistre>(&Sceau::decoder(sceau)?.charge)?;
        // Instant d'émission : toujours dans la fenêtre de la tête.
        let verifie = verifier_seuil::<TeteRegistre>(sceau, ancres, politique.seuil, tete.emise_a)?;
        // Un état stocké d'un autre registre (même ancres) ne doit jamais être
        // repris comme référence de monotonie pour celui-ci.
        if &verifie.titre().registre != registre_attendu {
            return Err(ErreurRegistre::RegistreInattendu);
        }
        Ok(Self { tete: verifie.into_titre(), sceau: sceau.to_vec() })
    }
}
