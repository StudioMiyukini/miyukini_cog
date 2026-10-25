//! @id     maia_memoire_memoire
//! @do     ecrire_valider_oublier_memoire
//! @role   securite
//! @layer  8
//! @human  maia-memoire — façade : lecture, notes ordinaires et proposition pour MAIA (A-009), écriture, validation, correction, export et oubli pour l'utilisateur seul (§9, §10, EX-52..56).

//! Deux capacités distinctes :
//!
//! - [`VueMaia`] — ce que MAIA (le modèle, ses outils) peut faire : assembler le
//!   contexte, ouvrir une branche (lecture seule), **noter** et **proposer**.
//!   [`MemoireMaia::noter`] écrit directement une note ordinaire (déduction, hors
//!   domaine sensible, nouvelle ou déjà d'auteur `maia`, quota de
//!   [`MAX_ECRITS_MAIA_24H`]) et en fait un candidat sinon (A-009). Un contenu
//!   `etranger` n'entre jamais en mémoire longue sans validation (EX-53) ; MAIA
//!   ne pose jamais de Règle (EX-54) et n'oublie rien ;
//! - [`AccesUtilisateur`] — réservé aux actions explicites de l'utilisateur :
//!   écrire, poser une règle, corriger, valider ou rejeter, exporter, oublier
//!   (EX-56).
//!
//! Le chemin d'inférence ne reçoit qu'une [`VueMaia`] ; l'[`AccesUtilisateur`]
//! s'obtient de [`MemoireMaia::utilisateur`], à n'appeler que derrière une
//! action authentifiée de l'utilisateur.

use std::collections::BTreeMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::assemblage::{self, plafond_texte, BlocMemoire, BALISE_FERMANTE, BALISE_OUVRANTE};
use crate::entrepot::{Contenu, Ecriture, Entrepot};
use crate::erreurs::ErreurMemoire;
use crate::ident::{Branche, Chemin, IdMemoire, TypeUnite};
use crate::jetons::Tokeniseur;
use crate::types::{
    verifier_echelle, Auteur, Candidat, EmpreinteSource, ExportMemoire, Note, PierreTombale,
    Provenance, ETIQUETTE, FORMAT_EXPORT,
};

/// Domaines sensibles : toute écriture passe par une validation TAMR (§9).
pub const DOMAINES_SENSIBLES: &[&str] = &["sante", "finances", "tiers"];
/// Confiance maximale d'une déduction (§9 : confiance réduite).
pub const CONFIANCE_MAX_DEDUIT: u8 = 6;
/// Confiance maximale d'un contenu `etranger`.
pub const CONFIANCE_MAX_ETRANGER: u8 = 4;
/// Confiance d'un propos explicite de l'utilisateur.
pub const CONFIANCE_UTILISATEUR: u8 = 9;
/// Longueur maximale d'un motif d'oubli.
pub const MAX_MOTIF_OUBLI: usize = 200;
/// Liens, entités ou sources au plus par unité.
pub const MAX_LIENS: usize = 16;
/// Écritures directes de MAIA au plus par 24 h glissantes ; au-delà : candidats (A-009).
pub const MAX_ECRITS_MAIA_24H: usize = 50;

/// Horloge injectable (tests).
pub type Horloge = Arc<dyn Fn() -> DateTime<Utc> + Send + Sync>;

/// Vrai si le domaine est sensible.
pub fn est_sensible(domaine: &str) -> bool {
    DOMAINES_SENSIBLES.contains(&domaine)
}

/// Contrôle d'un texte d'unité : une ligne, sans balise de bloc, sous le plafond de sa strate.
pub fn controler_texte(
    tok: &dyn Tokeniseur,
    texte: &str,
    type_unite: TypeUnite,
    profondeur: usize,
) -> Result<String, ErreurMemoire> {
    let t = texte.trim();
    if t.is_empty() {
        return Err(ErreurMemoire::TexteInvalide("texte vide"));
    }
    if t.chars().any(char::is_control) {
        return Err(ErreurMemoire::TexteInvalide("saut de ligne ou caractère de contrôle"));
    }
    let bas = t.to_lowercase();
    if bas.contains("<memoire") || bas.contains("</memoire") || bas.contains(BALISE_OUVRANTE) || bas.contains(BALISE_FERMANTE) {
        return Err(ErreurMemoire::TexteInvalide("balise de bloc mémoire"));
    }
    let (strate, max) = plafond_texte(type_unite, profondeur);
    let jetons = tok.compter(t);
    if jetons > max {
        return Err(ErreurMemoire::TexteTropLong { strate, jetons, max });
    }
    Ok(t.to_string())
}

fn controler_listes(liens: &[IdMemoire], entites: &[IdMemoire], sources: &[EmpreinteSource]) -> Result<(), ErreurMemoire> {
    if liens.len() > MAX_LIENS || entites.len() > MAX_LIENS || sources.len() > MAX_LIENS {
        return Err(ErreurMemoire::TexteInvalide("trop de liens, d'entités ou de sources"));
    }
    Ok(())
}

/// Prochain numéro libre de `branche.<type>` (jamais un identifiant oublié, §9).
fn prochain_numero(c: &Contenu, branche: &Branche, t: TypeUnite, deja: &BTreeMap<(String, TypeUnite), u32>) -> u32 {
    let meme = |id: &IdMemoire| id.branche() == *branche && id.type_unite() == t;
    let max = c
        .versions
        .iter()
        .map(|n| &n.id)
        .chain(c.tombes.iter().map(|x| &x.id))
        .filter(|id| meme(id))
        .map(IdMemoire::numero)
        .max()
        .unwrap_or(0);
    max.max(deja.get(&(branche.as_str().to_string(), t)).copied().unwrap_or(0)) + 1
}

/// Proposition de MAIA (déduction ou contenu `etranger`).
#[derive(Debug, Clone, PartialEq)]
pub struct Proposition {
    /// Note existante à corriger (`None` : nouvelle note dans `branche`).
    pub cible: Option<IdMemoire>,
    /// Branche de la nouvelle note (ignorée si `cible`).
    pub branche: Branche,
    /// Type d'unité (jamais `Regle`).
    pub type_unite: TypeUnite,
    /// Texte.
    pub texte: String,
    /// Importance 0–9.
    pub importance: u8,
    /// Confiance 0–9 (plafonnée selon la provenance).
    pub confiance: u8,
    /// `deduit` ou `etranger` (obligatoire, EX-53).
    pub provenance: Provenance,
    /// Liens.
    pub liens: Vec<IdMemoire>,
    /// Sources N4.
    pub sources: Vec<EmpreinteSource>,
}

/// Écriture explicite de l'utilisateur.
#[derive(Debug, Clone, PartialEq)]
pub struct Ecrit {
    /// Branche.
    pub branche: Branche,
    /// Type d'unité.
    pub type_unite: TypeUnite,
    /// Texte.
    pub texte: String,
    /// Importance 0–9.
    pub importance: u8,
    /// Liens.
    pub liens: Vec<IdMemoire>,
    /// Entités citées.
    pub entites: Vec<IdMemoire>,
    /// Sources N4.
    pub sources: Vec<EmpreinteSource>,
}

impl Ecrit {
    /// Écrit simple (sans liens).
    pub fn new(branche: Branche, type_unite: TypeUnite, texte: impl Into<String>, importance: u8) -> Self {
        Self {
            branche,
            type_unite,
            texte: texte.into(),
            importance,
            liens: Vec::new(),
            entites: Vec::new(),
            sources: Vec::new(),
        }
    }
}

/// Issue d'une écriture de l'utilisateur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Issue {
    /// Écrite en mémoire longue.
    Ecrite(IdMemoire),
    /// Branche sensible : en attente de validation TAMR (§9).
    EnAttente(Uuid),
}

/// Issue d'une note de MAIA (A-009).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Notee {
    /// Note ordinaire écrite directement (auteur `maia`).
    Ecrite(IdMemoire),
    /// Candidat en attente de validation de l'utilisateur.
    Candidat(Uuid),
}

/// Mode de validation d'un candidat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validation {
    /// Validation explicite de l'utilisateur.
    Utilisateur,
    /// Validation de l'utilisateur confirmée par TAMR (branches sensibles).
    UtilisateurEtTamr,
}

/// Mémoire d'un utilisateur.
pub struct MemoireMaia<E: Entrepot> {
    entrepot: E,
    tokeniseur: Arc<dyn Tokeniseur>,
    horloge: Horloge,
    ecriture: tokio::sync::Mutex<()>,
}

impl<E: Entrepot> MemoireMaia<E> {
    /// Ouvre la mémoire : l'entrepôt est préparé (schéma, étiquette, chiffrement) ou refusé.
    pub async fn ouvrir(entrepot: E, tokeniseur: Arc<dyn Tokeniseur>) -> Result<Self, ErreurMemoire> {
        entrepot.preparer().await?;
        Ok(Self {
            entrepot,
            tokeniseur,
            horloge: Arc::new(Utc::now),
            ecriture: tokio::sync::Mutex::new(()),
        })
    }

    /// Remplace l'horloge (tests).
    pub fn avec_horloge(mut self, horloge: Horloge) -> Self {
        self.horloge = horloge;
        self
    }

    /// Tokenizer actif.
    pub fn tokeniseur(&self) -> &dyn Tokeniseur {
        self.tokeniseur.as_ref()
    }

    fn maintenant(&self) -> DateTime<Utc> {
        (self.horloge)()
    }

    /// Accès utilisateur (écriture, correction, export, oubli) : à n'obtenir que
    /// derrière une action authentifiée de l'utilisateur.
    pub fn utilisateur(&self) -> AccesUtilisateur<'_, E> {
        AccesUtilisateur { m: self }
    }

    /// Assemble le contexte sous `budget` jetons mesurés (EX-50). Lecture seule.
    pub async fn assembler(&self, requete: &str, budget: usize) -> Result<BlocMemoire, ErreurMemoire> {
        let c = self.entrepot.lire().await?;
        let courantes = c.notes_courantes();
        let notes: Vec<&Note> = courantes.values().copied().collect();
        let bloc = assemblage::assembler(&notes, requete, budget, self.tokeniseur.as_ref(), self.maintenant())?;
        let sensibles: Vec<&str> = bloc.inclus.iter().filter(|id| est_sensible(id.domaine())).map(IdMemoire::as_str).collect();
        if !sensibles.is_empty() {
            tracing::info!(target: "maia_memoire::audit", ids = ?sensibles, "notes sensibles injectées");
        }
        Ok(bloc)
    }

    /// `memoire.ouvrir(id)` : zoom sur une note ou une branche, lecture seule, sans effet de bord (EX-51).
    pub async fn ouvrir_chemin(&self, chemin: &str, budget: usize) -> Result<BlocMemoire, ErreurMemoire> {
        let chemin = Chemin::parse(chemin)?;
        let c = self.entrepot.lire().await?;
        if let Chemin::Note(id) = &chemin {
            if c.est_oublie(id) {
                return Err(ErreurMemoire::NoteOubliee(id.to_string()));
            }
        }
        if est_sensible(chemin.domaine()) {
            tracing::info!(target: "maia_memoire::audit", chemin = chemin.as_str(), "ouverture de branche sensible");
        }
        let courantes = c.notes_courantes();
        let notes: Vec<&Note> = courantes.values().copied().collect();
        assemblage::ouvrir(&notes, &chemin, budget, self.tokeniseur.as_ref(), self.maintenant())
    }

    /// Note de MAIA (A-009) : écrite directement si c'est une note ordinaire —
    /// provenance `deduit`, type ≠ Règle, domaine non sensible, nouvelle note ou
    /// correction d'une note dont la version courante est d'auteur `maia`, moins
    /// de [`MAX_ECRITS_MAIA_24H`] écritures directes sur 24 h ; sinon candidat
    /// ([`Self::proposer`]).
    pub async fn noter(&self, p: Proposition) -> Result<Notee, ErreurMemoire> {
        if p.provenance != Provenance::Deduit || p.type_unite == TypeUnite::Regle {
            return self.proposer(p).await.map(Notee::Candidat);
        }
        verifier_echelle(p.importance)?;
        let confiance = verifier_echelle(p.confiance)?.min(CONFIANCE_MAX_DEDUIT);
        controler_listes(&p.liens, &[], &p.sources)?;
        let maintenant = self.maintenant();
        {
            let _g = self.ecriture.lock().await;
            let c = self.entrepot.lire().await?;
            let courantes = c.notes_courantes();
            let depuis = maintenant - chrono::Duration::hours(24);
            let recents = c.versions.iter().filter(|n| n.auteur == Auteur::Maia && n.enregistre_le > depuis).count();
            let actuelle = match &p.cible {
                Some(id) => {
                    cible_courante(&c, id)?;
                    if id.type_unite() != p.type_unite {
                        return Err(ErreurMemoire::IdInvalide(format!("{id} : type différent de la proposition")));
                    }
                    Some(*courantes.get(id).ok_or_else(|| ErreurMemoire::NoteInconnue(id.to_string()))?)
                }
                None => None,
            };
            let branche = actuelle.map_or_else(|| p.branche.clone(), |n| n.id.branche());
            let directe = !est_sensible(branche.domaine())
                && actuelle.is_none_or(|n| n.auteur == Auteur::Maia)
                && recents < MAX_ECRITS_MAIA_24H;
            if directe {
                let texte = controler_texte(self.tokeniseur.as_ref(), &p.texte, p.type_unite, branche.profondeur())?;
                let note = match actuelle {
                    Some(n) => Note {
                        version: n.version + 1,
                        texte,
                        importance: p.importance,
                        confiance,
                        provenance: Provenance::Deduit,
                        auteur: Auteur::Maia,
                        liens: if p.liens.is_empty() { n.liens.clone() } else { p.liens },
                        sources: if p.sources.is_empty() { n.sources.clone() } else { p.sources },
                        enregistre_le: maintenant,
                        ..n.clone()
                    },
                    None => Note {
                        id: IdMemoire::nouveau(&branche, p.type_unite, prochain_numero(&c, &branche, p.type_unite, &BTreeMap::new()))?,
                        version: 1,
                        texte,
                        importance: p.importance,
                        confiance,
                        provenance: Provenance::Deduit,
                        auteur: Auteur::Maia,
                        liens: p.liens,
                        entites: Vec::new(),
                        sources: p.sources,
                        cree_le: maintenant,
                        enregistre_le: maintenant,
                        dernier_usage: None,
                    },
                };
                let id = note.id.clone();
                let version = note.version;
                self.entrepot.appliquer(vec![Ecriture::AjouterVersion(note)]).await?;
                tracing::info!(target: "maia_memoire::audit", id = id.as_str(), version, "note écrite par MAIA");
                return Ok(Notee::Ecrite(id));
            }
        }
        self.proposer(p).await.map(Notee::Candidat)
    }

    /// Proposition de MAIA : toujours un candidat, jamais une écriture en mémoire longue (EX-53, EX-54).
    pub async fn proposer(&self, p: Proposition) -> Result<Uuid, ErreurMemoire> {
        if p.type_unite == TypeUnite::Regle {
            return Err(ErreurMemoire::RegleReserveeUtilisateur);
        }
        let plafond = match p.provenance {
            Provenance::Utilisateur => {
                return Err(ErreurMemoire::ProvenanceRefusee(
                    "MAIA ne peut pas attribuer une proposition à l'utilisateur",
                ))
            }
            Provenance::Deduit => CONFIANCE_MAX_DEDUIT,
            Provenance::Etranger => CONFIANCE_MAX_ETRANGER,
        };
        verifier_echelle(p.importance)?;
        let confiance = verifier_echelle(p.confiance)?.min(plafond);
        controler_listes(&p.liens, &[], &p.sources)?;
        let _g = self.ecriture.lock().await;
        let c = self.entrepot.lire().await?;
        let branche = match &p.cible {
            Some(id) => {
                cible_courante(&c, id)?;
                if id.type_unite() != p.type_unite {
                    return Err(ErreurMemoire::IdInvalide(format!("{id} : type différent de la proposition")));
                }
                id.branche()
            }
            None => p.branche.clone(),
        };
        let texte = controler_texte(self.tokeniseur.as_ref(), &p.texte, p.type_unite, branche.profondeur())?;
        let candidat = Candidat {
            id: Uuid::new_v4(),
            cible: p.cible,
            sensible: est_sensible(branche.domaine()),
            branche,
            type_unite: p.type_unite,
            texte,
            importance: p.importance,
            confiance,
            provenance: p.provenance,
            liens: p.liens,
            sources: p.sources,
            propose_le: self.maintenant(),
        };
        let id = candidat.id;
        self.entrepot.appliquer(vec![Ecriture::AjouterCandidat(candidat)]).await?;
        Ok(id)
    }
}

/// Vérifie que `id` est une note courante (ni oubliée, ni inconnue).
fn cible_courante(c: &Contenu, id: &IdMemoire) -> Result<u32, ErreurMemoire> {
    if c.est_oublie(id) {
        return Err(ErreurMemoire::NoteOubliee(id.to_string()));
    }
    match c.derniere_version(id) {
        0 => Err(ErreurMemoire::NoteInconnue(id.to_string())),
        v => Ok(v),
    }
}

/// Capacité de MAIA : lecture, notes ordinaires et propositions ; ni Règle, ni oubli.
pub struct VueMaia<E: Entrepot> {
    m: Arc<MemoireMaia<E>>,
}

impl<E: Entrepot> Clone for VueMaia<E> {
    fn clone(&self) -> Self {
        Self { m: Arc::clone(&self.m) }
    }
}

impl<E: Entrepot> VueMaia<E> {
    /// Vue MAIA d'une mémoire partagée.
    pub fn new(m: Arc<MemoireMaia<E>>) -> Self {
        Self { m }
    }

    /// Voir [`MemoireMaia::assembler`].
    pub async fn assembler(&self, requete: &str, budget: usize) -> Result<BlocMemoire, ErreurMemoire> {
        self.m.assembler(requete, budget).await
    }

    /// Voir [`MemoireMaia::ouvrir_chemin`].
    pub async fn ouvrir(&self, chemin: &str, budget: usize) -> Result<BlocMemoire, ErreurMemoire> {
        self.m.ouvrir_chemin(chemin, budget).await
    }

    /// Voir [`MemoireMaia::noter`].
    pub async fn noter(&self, p: Proposition) -> Result<Notee, ErreurMemoire> {
        self.m.noter(p).await
    }

    /// Voir [`MemoireMaia::proposer`].
    pub async fn proposer(&self, p: Proposition) -> Result<Uuid, ErreurMemoire> {
        self.m.proposer(p).await
    }
}

/// Capacité de l'utilisateur sur sa mémoire (EX-56).
pub struct AccesUtilisateur<'a, E: Entrepot> {
    m: &'a MemoireMaia<E>,
}

impl<E: Entrepot> AccesUtilisateur<'_, E> {
    /// Propos explicite : accepté directement, sauf branche sensible (TAMR, §9).
    pub async fn ecrire(&self, e: Ecrit) -> Result<Issue, ErreurMemoire> {
        let m = self.m;
        verifier_echelle(e.importance)?;
        controler_listes(&e.liens, &e.entites, &e.sources)?;
        let texte = controler_texte(m.tokeniseur.as_ref(), &e.texte, e.type_unite, e.branche.profondeur())?;
        let maintenant = m.maintenant();
        let _g = m.ecriture.lock().await;
        if est_sensible(e.branche.domaine()) {
            let candidat = Candidat {
                id: Uuid::new_v4(),
                cible: None,
                branche: e.branche,
                type_unite: e.type_unite,
                texte,
                importance: e.importance,
                confiance: CONFIANCE_UTILISATEUR,
                provenance: Provenance::Utilisateur,
                liens: e.liens,
                sources: e.sources,
                sensible: true,
                propose_le: maintenant,
            };
            let id = candidat.id;
            m.entrepot.appliquer(vec![Ecriture::AjouterCandidat(candidat)]).await?;
            return Ok(Issue::EnAttente(id));
        }
        let c = m.entrepot.lire().await?;
        let numero = prochain_numero(&c, &e.branche, e.type_unite, &BTreeMap::new());
        let id = IdMemoire::nouveau(&e.branche, e.type_unite, numero)?;
        let note = Note {
            id: id.clone(),
            version: 1,
            texte,
            importance: e.importance,
            confiance: CONFIANCE_UTILISATEUR,
            provenance: Provenance::Utilisateur,
            auteur: Auteur::Utilisateur,
            liens: e.liens,
            entites: e.entites,
            sources: e.sources,
            cree_le: maintenant,
            enregistre_le: maintenant,
            dernier_usage: None,
        };
        m.entrepot.appliquer(vec![Ecriture::AjouterVersion(note)]).await?;
        Ok(Issue::Ecrite(id))
    }

    /// Pose une Règle (seul chemin de création d'une règle, EX-54).
    pub async fn poser_regle(&self, branche: Branche, texte: &str, importance: u8) -> Result<Issue, ErreurMemoire> {
        self.ecrire(Ecrit::new(branche, TypeUnite::Regle, texte, importance)).await
    }

    /// Correction : nouvelle version qui remplace l'ancienne, l'historique reste (EX-52, EX-56).
    pub async fn corriger(&self, id: &IdMemoire, texte: &str, importance: Option<u8>) -> Result<Issue, ErreurMemoire> {
        let m = self.m;
        if let Some(i) = importance {
            verifier_echelle(i)?;
        }
        let texte = controler_texte(m.tokeniseur.as_ref(), texte, id.type_unite(), id.branche().profondeur())?;
        let maintenant = m.maintenant();
        let _g = m.ecriture.lock().await;
        let c = m.entrepot.lire().await?;
        let version = cible_courante(&c, id)?;
        let courantes = c.notes_courantes();
        let Some(actuelle) = courantes.get(id) else {
            return Err(ErreurMemoire::NoteInconnue(id.to_string()));
        };
        if est_sensible(id.domaine()) {
            let candidat = Candidat {
                id: Uuid::new_v4(),
                cible: Some(id.clone()),
                branche: id.branche(),
                type_unite: id.type_unite(),
                texte,
                importance: importance.unwrap_or(actuelle.importance),
                confiance: CONFIANCE_UTILISATEUR,
                provenance: Provenance::Utilisateur,
                liens: actuelle.liens.clone(),
                sources: actuelle.sources.clone(),
                sensible: true,
                propose_le: maintenant,
            };
            let cid = candidat.id;
            m.entrepot.appliquer(vec![Ecriture::AjouterCandidat(candidat)]).await?;
            return Ok(Issue::EnAttente(cid));
        }
        let note = Note {
            version: version + 1,
            texte,
            importance: importance.unwrap_or(actuelle.importance),
            confiance: CONFIANCE_UTILISATEUR,
            provenance: Provenance::Utilisateur,
            auteur: Auteur::Utilisateur,
            enregistre_le: maintenant,
            ..(*actuelle).clone()
        };
        m.entrepot.appliquer(vec![Ecriture::AjouterVersion(note)]).await?;
        Ok(Issue::Ecrite(id.clone()))
    }

    /// Valide un candidat : il entre en mémoire longue (EX-53).
    pub async fn valider(&self, candidat: Uuid, mode: Validation) -> Result<IdMemoire, ErreurMemoire> {
        let mut ids = self.valider_tous(&[candidat], mode).await?;
        ids.pop().ok_or(ErreurMemoire::CandidatInconnu(candidat))
    }

    /// Validation en lot (§9), atomique : un refus et rien n'est validé.
    pub async fn valider_lot(&self, candidats: &[Uuid]) -> Result<Vec<IdMemoire>, ErreurMemoire> {
        self.valider_tous(candidats, Validation::Utilisateur).await
    }

    async fn valider_tous(&self, candidats: &[Uuid], mode: Validation) -> Result<Vec<IdMemoire>, ErreurMemoire> {
        let m = self.m;
        let maintenant = m.maintenant();
        let _g = m.ecriture.lock().await;
        let c = m.entrepot.lire().await?;
        let courantes = c.notes_courantes();
        let mut alloues: BTreeMap<(String, TypeUnite), u32> = BTreeMap::new();
        let mut lot = Vec::new();
        let mut ids = Vec::new();
        for uid in candidats {
            let k = c
                .candidats
                .iter()
                .find(|k| &k.id == uid)
                .ok_or(ErreurMemoire::CandidatInconnu(*uid))?;
            if k.type_unite == TypeUnite::Regle && k.provenance != Provenance::Utilisateur {
                return Err(ErreurMemoire::RegleReserveeUtilisateur);
            }
            if k.sensible && mode != Validation::UtilisateurEtTamr {
                return Err(ErreurMemoire::TamrRequis(k.branche.to_string()));
            }
            let auteur = if k.provenance == Provenance::Utilisateur {
                Auteur::Utilisateur
            } else {
                Auteur::ValidationUtilisateur
            };
            let note = if let Some(id) = &k.cible {
                    let version = cible_courante(&c, id)?;
                    let actuelle = courantes.get(id).ok_or_else(|| ErreurMemoire::NoteInconnue(id.to_string()))?;
                    Note {
                        version: version + 1,
                        texte: k.texte.clone(),
                        importance: k.importance,
                        confiance: k.confiance,
                        provenance: k.provenance,
                        auteur,
                        liens: if k.liens.is_empty() { actuelle.liens.clone() } else { k.liens.clone() },
                        sources: if k.sources.is_empty() { actuelle.sources.clone() } else { k.sources.clone() },
                        enregistre_le: maintenant,
                        ..(*actuelle).clone()
                    }
                } else {
                    let numero = prochain_numero(&c, &k.branche, k.type_unite, &alloues);
                    alloues.insert((k.branche.as_str().to_string(), k.type_unite), numero);
                    Note {
                        id: IdMemoire::nouveau(&k.branche, k.type_unite, numero)?,
                        version: 1,
                        texte: k.texte.clone(),
                        importance: k.importance,
                        confiance: k.confiance,
                        provenance: k.provenance,
                        auteur,
                        liens: k.liens.clone(),
                        entites: Vec::new(),
                        sources: k.sources.clone(),
                        cree_le: maintenant,
                        enregistre_le: maintenant,
                        dernier_usage: None,
                    }
            };
            ids.push(note.id.clone());
            lot.push(Ecriture::RetirerCandidat(*uid));
            lot.push(Ecriture::AjouterVersion(note));
        }
        m.entrepot.appliquer(lot).await?;
        Ok(ids)
    }

    /// Rejette un candidat : effacé, il n'entre jamais en mémoire.
    pub async fn rejeter(&self, candidat: Uuid) -> Result<(), ErreurMemoire> {
        let _g = self.m.ecriture.lock().await;
        self.m.entrepot.appliquer(vec![Ecriture::RetirerCandidat(candidat)]).await
    }

    /// Oubli d'une note ou d'une branche : effacement réel de toutes les versions
    /// (et des candidats qui la visent), pierre tombale sans contenu (§9, EX-52, EX-56).
    pub async fn oublier(&self, chemin: &str, motif: &str) -> Result<Vec<PierreTombale>, ErreurMemoire> {
        let motif = motif.trim();
        if motif.is_empty() || motif.chars().count() > MAX_MOTIF_OUBLI || motif.chars().any(char::is_control) {
            return Err(ErreurMemoire::OubliInvalide("motif vide, trop long ou multiligne"));
        }
        let chemin = Chemin::parse(chemin)?;
        let m = self.m;
        let maintenant = m.maintenant();
        let _g = m.ecriture.lock().await;
        let c = m.entrepot.lire().await?;
        let mut versions: BTreeMap<&IdMemoire, u32> = BTreeMap::new();
        for n in c.versions.iter().filter(|n| chemin.designe(&n.id)) {
            *versions.entry(&n.id).or_default() += 1;
        }
        let candidats: Vec<Uuid> = c
            .candidats
            .iter()
            .filter(|k| {
                if let Some(id) = &k.cible {
                    return chemin.designe(id);
                }
                if let Chemin::Branche(b) = &chemin {
                    return &k.branche == b || k.branche.as_str().starts_with(&format!("{}.", b.as_str()));
                }
                false
            })
            .map(|k| k.id)
            .collect();
        if versions.is_empty() && candidats.is_empty() {
            if let Chemin::Note(id) = &chemin {
                if c.est_oublie(id) {
                    return Err(ErreurMemoire::NoteOubliee(id.to_string()));
                }
            }
            return Err(ErreurMemoire::OubliInvalide("rien à oublier"));
        }
        let tombes: Vec<PierreTombale> = versions
            .into_iter()
            .map(|(id, n)| PierreTombale {
                id: id.clone(),
                oublie_le: maintenant,
                motif: motif.to_string(),
                versions_effacees: n,
            })
            .collect();
        let mut lot: Vec<Ecriture> = candidats.into_iter().map(Ecriture::RetirerCandidat).collect();
        lot.extend(tombes.iter().cloned().map(Ecriture::Oublier));
        m.entrepot.appliquer(lot).await?;
        tracing::info!(target: "maia_memoire::audit", chemin = chemin.as_str(), notes = tombes.len(), "oubli demandé par l'utilisateur");
        Ok(tombes)
    }

    /// Notes courantes (consultation, EX-56).
    pub async fn consulter(&self) -> Result<Vec<Note>, ErreurMemoire> {
        let c = self.m.entrepot.lire().await?;
        Ok(c.notes_courantes().into_values().cloned().collect())
    }

    /// Historique complet d'une note (versions croissantes).
    pub async fn historique(&self, id: &IdMemoire) -> Result<Vec<Note>, ErreurMemoire> {
        let c = self.m.entrepot.lire().await?;
        cible_courante(&c, id)?;
        let mut v: Vec<Note> = c.versions.into_iter().filter(|n| &n.id == id).collect();
        v.sort_by_key(|n| n.version);
        Ok(v)
    }

    /// Candidats en attente de validation.
    pub async fn candidats(&self) -> Result<Vec<Candidat>, ErreurMemoire> {
        Ok(self.m.entrepot.lire().await?.candidats)
    }

    /// Export intégral (EX-56), étiqueté `nationale`.
    pub async fn exporter(&self) -> Result<ExportMemoire, ErreurMemoire> {
        let c = self.m.entrepot.lire().await?;
        let notes = c.notes_courantes().into_values().cloned().collect();
        let mut historique = c.versions;
        historique.sort_by(|a, b| a.id.cmp(&b.id).then(a.version.cmp(&b.version)));
        Ok(ExportMemoire {
            format: FORMAT_EXPORT.to_string(),
            etiquette: ETIQUETTE,
            exporte_le: self.m.maintenant(),
            notes,
            historique,
            candidats: c.candidats,
            pierres_tombales: c.tombes,
        })
    }
}
