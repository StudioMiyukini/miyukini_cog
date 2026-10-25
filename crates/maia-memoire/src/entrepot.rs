//! @id     maia_memoire_entrepot
//! @do     definir_entrepot_memoire
//! @role   donnee
//! @layer  8
//! @human  maia-memoire — contrat de stockage : ajout seul, oubli = effacement + pierre tombale dans le même lot (EX-52).

//! Le contrat d'écriture est **structurellement non destructif** (EX-52) :
//! [`Ecriture`] ne permet que d'ajouter une version, d'ajouter ou de retirer
//! un candidat (hors mémoire longue) et d'**oublier** — effacement de toutes
//! les versions d'un identifiant indissociable de sa pierre tombale. Il
//! n'existe aucune variante « modifier » ni « supprimer » libre.

use std::collections::BTreeMap;
use std::future::Future;
use std::sync::Mutex;

use uuid::Uuid;

use crate::erreurs::ErreurMemoire;
use crate::ident::IdMemoire;
use crate::types::{Candidat, Note, PierreTombale};

/// Écriture élémentaire d'un lot atomique.
#[derive(Debug, Clone, PartialEq)]
pub enum Ecriture {
    /// Ajoute une version (`version` = dernière + 1 ; 1 pour une nouvelle note).
    AjouterVersion(Note),
    /// Met une proposition en attente de validation.
    AjouterCandidat(Candidat),
    /// Retire un candidat (validé ou rejeté).
    RetirerCandidat(Uuid),
    /// Oubli demandé : efface toutes les versions de `id` et pose la pierre tombale.
    Oublier(PierreTombale),
}

/// Contenu complet d'une mémoire.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Contenu {
    /// Toutes les versions de toutes les notes.
    pub versions: Vec<Note>,
    /// Candidats en attente.
    pub candidats: Vec<Candidat>,
    /// Pierres tombales.
    pub tombes: Vec<PierreTombale>,
}

impl Contenu {
    /// Notes courantes (version la plus haute de chaque identifiant), triées par identifiant.
    pub fn notes_courantes(&self) -> BTreeMap<IdMemoire, &Note> {
        let mut out: BTreeMap<IdMemoire, &Note> = BTreeMap::new();
        for n in &self.versions {
            match out.get(&n.id) {
                Some(cur) if cur.version >= n.version => {}
                _ => {
                    out.insert(n.id.clone(), n);
                }
            }
        }
        out
    }

    /// Dernière version de `id` (0 si absente).
    pub fn derniere_version(&self, id: &IdMemoire) -> u32 {
        self.versions
            .iter()
            .filter(|n| &n.id == id)
            .map(|n| n.version)
            .max()
            .unwrap_or(0)
    }

    /// Vrai si `id` a une pierre tombale.
    pub fn est_oublie(&self, id: &IdMemoire) -> bool {
        self.tombes.iter().any(|t| &t.id == id)
    }

    /// Applique un lot sur une copie (contrôles de cohérence) ; erreur : rien n'est appliqué.
    pub fn appliquer(&self, lot: &[Ecriture]) -> Result<Contenu, ErreurMemoire> {
        let mut c = self.clone();
        for e in lot {
            match e {
                Ecriture::AjouterVersion(n) => {
                    if c.est_oublie(&n.id) {
                        return Err(ErreurMemoire::NoteOubliee(n.id.to_string()));
                    }
                    let attendu = c.derniere_version(&n.id) + 1;
                    if n.version != attendu {
                        return Err(ErreurMemoire::Conflit(format!(
                            "{} : version {} (attendue {attendu})",
                            n.id, n.version
                        )));
                    }
                    c.versions.push(n.clone());
                }
                Ecriture::AjouterCandidat(k) => {
                    if c.candidats.iter().any(|x| x.id == k.id) {
                        return Err(ErreurMemoire::Conflit(format!("candidat {}", k.id)));
                    }
                    c.candidats.push(k.clone());
                }
                Ecriture::RetirerCandidat(id) => {
                    let avant = c.candidats.len();
                    c.candidats.retain(|x| &x.id != id);
                    if c.candidats.len() == avant {
                        return Err(ErreurMemoire::CandidatInconnu(*id));
                    }
                }
                Ecriture::Oublier(t) => {
                    if c.est_oublie(&t.id) {
                        return Err(ErreurMemoire::Conflit(format!("{} déjà oublié", t.id)));
                    }
                    c.versions.retain(|n| n.id != t.id);
                    c.tombes.push(t.clone());
                }
            }
        }
        Ok(c)
    }
}

/// Stockage d'une mémoire (une par utilisateur).
pub trait Entrepot: Send + Sync {
    /// Prépare le stockage (schéma, étiquette `nationale`, chiffrement) ; refus sinon.
    fn preparer(&self) -> impl Future<Output = Result<(), ErreurMemoire>> + Send;
    /// Lit tout le contenu.
    fn lire(&self) -> impl Future<Output = Result<Contenu, ErreurMemoire>> + Send;
    /// Applique un lot **atomiquement** (tout ou rien).
    fn appliquer(&self, lot: Vec<Ecriture>) -> impl Future<Output = Result<(), ErreurMemoire>> + Send;
}

/// Entrepôt en mémoire vive (essais, tests, profil S0 sans persistance) : rien
/// n'est écrit sur disque.
#[derive(Debug, Default)]
pub struct EntrepotMemoire {
    contenu: Mutex<Contenu>,
}

impl EntrepotMemoire {
    /// Entrepôt vide.
    pub fn new() -> Self {
        Self::default()
    }

    fn verrou(&self) -> Result<std::sync::MutexGuard<'_, Contenu>, ErreurMemoire> {
        self.contenu
            .lock()
            .map_err(|_| ErreurMemoire::Entrepot("verrou empoisonné".to_string()))
    }
}

impl Entrepot for EntrepotMemoire {
    fn preparer(&self) -> impl Future<Output = Result<(), ErreurMemoire>> + Send {
        std::future::ready(self.verrou().map(|_| ()))
    }

    fn lire(&self) -> impl Future<Output = Result<Contenu, ErreurMemoire>> + Send {
        std::future::ready(self.verrou().map(|c| c.clone()))
    }

    fn appliquer(&self, lot: Vec<Ecriture>) -> impl Future<Output = Result<(), ErreurMemoire>> + Send {
        let r = self.verrou().and_then(|mut c| {
            let nouveau = c.appliquer(&lot)?;
            *c = nouveau;
            Ok(())
        });
        std::future::ready(r)
    }
}
