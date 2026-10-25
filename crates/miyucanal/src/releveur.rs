//! @id     miyucanal_releveur
//! @do     relever_canal_officiel_quarantaine
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — module « releveur » (zone COG : relevé, miroir du registre officiel, quarantaine, installation).

//! Relevé du canal officiel par la zone COG (zones §7 ; menaces §3).
//!
//! # Flux
//!
//! 1. **Heure de confiance** : `HorlogeConfiance::maintenant` (refus sous le MTP).
//! 2. **Tête** relevée, signée par la **clé de journal d'Origin** (seuil 1,
//!    A-006 ; politique N sur N plus stricte possible), registre attendu,
//!    fraîcheur (anti-gel), MTP du registre.
//! 3. **Entrées** nouvelles relevées par lots ; elles doivent reconstruire la
//!    racine signée ; monotonie et cohérence via `EtatRegistre` (miroir).
//! 4. **Tri** des entrées nouvelles : révocations (Origin ou éditeur) ;
//!    annonces 2 sur 2 → inclusion vérifiée contre la tête fraîche →
//!    anti-retour (version > installée, > en attente) → **quarantaine**.
//! 5. **Installation** d'une annonce dont la quarantaine est écoulée, seulement
//!    si : une tête **émise après la fin de la quarantaine** a été acceptée
//!    et est fraîche (toute révocation publiée pendant la quarantaine est
//!    donc vue), aucune révocation ne la vise, la version reste supérieure,
//!    l'inclusion est revérifiée ; l'artefact est relevé **par empreinte**,
//!    taille et SHA-256 contrôlés, puis remis à l'[`Installateur`].
//!
//! Quarantaine : **48 h** (zones §7), comptée depuis la **première vue
//! locale** (heure de confiance), jamais depuis l'horodatage signé (un
//! horodatage antidaté ne raccourcit rien). Elle est **persistée** et scellée
//! ([`crate::quarantaine`], A-006) : après un redémarrage, le décompte reprend ;
//! un état altéré ou illisible relance la quarantaine des annonces concernées.
//! L'exception « correctif critique co-signé » n'est **pas** implémentée
//! (à décider, A-006 : aucun raccourcissement).
//!
//! Sens unique : rien d'autre que des [`Demande`]s génériques ne sort ; le
//! canal n'installe rien lui-même.

use std::collections::{BTreeMap, HashMap, HashSet};

use miyuregistre::{
    Categorie, Cible, Corps, ErreurJournal, HorlogeConfiance, MiroirRegistre, Publication, Stockage, SuiviTetes,
};
use miyusceau::{
    hacher_feuille, verifier_tete, AncresDeConfiance, ErreurSceau, Kid, NiveauConfiance, PolitiqueTete,
    PreuveInclusion, AGE_MAX_TETE_DEFAUT_SECS, TOLERANCE_HORLOGE_SECS,
};
use sha2::{Digest, Sha256};

use crate::annonce::{verifier_annonce, verifier_entree_canal, AncresCanal, AnnonceVerifiee};
use crate::erreurs::{ErreurCanal, ErreurTransport};
use crate::quarantaine::{ouvrir_quarantaine, sceller_quarantaine, CleQuarantaine, VuePersistee};
use crate::transport::{decoder_entrees, Demande, TransportCanal, MAX_ENTREES_PAR_DEMANDE};

/// Quarantaine des mises à jour (zones §7 : 48 h).
pub const QUARANTAINE_SECS: u64 = 48 * 3_600;

/// Nombre maximal d'entrées nouvelles acceptées en un relevé (borne mémoire :
/// au plus ~380 Mio de sceaux reçus avant le contrôle de la racine).
pub const MAX_ENTREES_PAR_RELEVE: u64 = 4_096;

/// Configuration du canal officiel côté zone COG.
#[derive(Debug, Clone)]
pub struct ConfigCanal {
    registre: [u8; 32],
    ancres: AncresCanal,
    journal_origin: Kid,
    ancres_tetes: AncresDeConfiance,
    politique_tete: PolitiqueTete,
    quarantaine_secs: u64,
}

impl ConfigCanal {
    /// Canal officiel (A-006) : registre `registre` ; annonces 2 sur 2
    /// {`origin`, `editeur`} ; révocations Origin **ou** éditeur ; têtes
    /// signées par la clé de journal d'Origin `journal_origin` seule (seuil 1,
    /// 24 h au plus) ; quarantaine de 48 h. Les trois clés doivent être
    /// distinctes.
    pub fn nouvelle(
        registre: [u8; 32],
        origin: &[u8; 32],
        journal_origin: &[u8; 32],
        editeur: &[u8; 32],
    ) -> Result<Self, ErreurCanal> {
        if journal_origin == origin || journal_origin == editeur {
            return Err(ErreurCanal::Configuration("clé de journal d'Origin dédiée exigée"));
        }
        let ancres = AncresCanal::nouvelles(origin, editeur)?;
        let mut ancres_tetes = AncresDeConfiance::new();
        let journal_origin = ancres_tetes.ajouter(journal_origin, NiveauConfiance::Reconnu)?;
        Ok(Self {
            registre,
            ancres,
            journal_origin,
            ancres_tetes,
            politique_tete: PolitiqueTete::journal_cog(),
            quarantaine_secs: QUARANTAINE_SECS,
        })
    }

    /// Politique des têtes **plus stricte** : N sur N d'un ensemble `ancres`
    /// qui contient la clé de journal d'Origin (ex. 2 sur 2 {journal d'Origin,
    /// Origin}), âge ≤ 24 h. Toute politique plus laxiste est refusée.
    pub fn avec_tetes(mut self, ancres: AncresDeConfiance, politique: PolitiqueTete) -> Result<Self, ErreurCanal> {
        let mut sans_journal = ancres.clone();
        if !sans_journal.retirer(&self.journal_origin) {
            return Err(ErreurCanal::Configuration("têtes du canal officiel : clé de journal d'Origin exigée"));
        }
        if politique.seuil() != ancres.len() {
            return Err(ErreurCanal::Configuration("têtes du canal officiel : toutes les ancres doivent signer"));
        }
        if politique.age_max_secs() > AGE_MAX_TETE_DEFAUT_SECS {
            return Err(ErreurCanal::Configuration("têtes du canal officiel : âge maximal de 24 h"));
        }
        self.ancres_tetes = ancres;
        self.politique_tete = politique;
        Ok(self)
    }

    /// `kid` de la clé de journal d'Origin.
    pub fn journal_origin(&self) -> &Kid {
        &self.journal_origin
    }

    /// Identifiant du registre officiel.
    pub fn registre(&self) -> &[u8; 32] {
        &self.registre
    }

    /// Quarantaine plus longue que 48 h ; plus courte : refus.
    pub fn avec_quarantaine(mut self, secs: u64) -> Result<Self, ErreurCanal> {
        if secs < QUARANTAINE_SECS {
            return Err(ErreurCanal::Configuration("quarantaine inférieure à 48 h"));
        }
        self.quarantaine_secs = secs;
        Ok(self)
    }

    /// Durée de quarantaine (secondes).
    pub fn quarantaine_secs(&self) -> u64 {
        self.quarantaine_secs
    }

    /// Ancres des annonces.
    pub fn ancres(&self) -> &AncresCanal {
        &self.ancres
    }

    fn suivi(&self) -> SuiviTetes {
        SuiviTetes::new(self.registre, self.ancres_tetes.clone(), self.politique_tete)
    }
}

/// Mise à jour entièrement vérifiée, remise à l'installateur. Seul le
/// [`Releveur`] en construit une.
#[derive(Debug, Clone)]
pub struct MiseAJourVerifiee {
    annonce: AnnonceVerifiee,
    index: u64,
    artefact: Vec<u8>,
}

impl MiseAJourVerifiee {
    /// Catégorie (Cores, OS, modèle IA).
    pub fn categorie(&self) -> Categorie {
        self.annonce.publication().categorie
    }

    /// Composant.
    pub fn composant(&self) -> &str {
        &self.annonce.publication().composant
    }

    /// Version.
    pub fn version(&self) -> u64 {
        self.annonce.publication().version
    }

    /// Empreinte SHA-256 vérifiée de l'artefact.
    pub fn empreinte(&self) -> &[u8; 32] {
        &self.annonce.publication().empreinte
    }

    /// Octets de l'artefact (empreinte et taille vérifiées).
    pub fn artefact(&self) -> &[u8] {
        &self.artefact
    }

    /// Annonce 2 sur 2 (sceau conservable par l'installateur).
    pub fn annonce(&self) -> &AnnonceVerifiee {
        &self.annonce
    }

    /// Index de l'annonce dans le registre officiel.
    pub fn index(&self) -> u64 {
        self.index
    }
}

/// Chemin d'installation existant (Central, StrongFather) : le canal ne fait
/// que lui remettre des mises à jour vérifiées.
pub trait Installateur {
    /// Version installée d'un composant (`None` : absent).
    fn version_installee(&self, categorie: Categorie, composant: &str) -> Option<u64>;
    /// Installe ; une erreur laisse la mise à jour en attente.
    fn installer(&mut self, maj: &MiseAJourVerifiee) -> Result<(), String>;
}

/// Motif de refus d'une annonce ou d'une installation (journalisé ; à
/// remonter à WorrySentinel).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Motif {
    /// Entrée non signée 2 sur 2 par le canal, ou mal formée.
    Signature(ErreurSceau),
    /// Autre refus de l'annonce.
    Invalide(String),
    /// Version non supérieure à celle installée ou en attente.
    Retrogradation {
        /// Version de référence.
        reference: u64,
        /// Version proposée.
        proposee: u64,
    },
    /// Visée par une révocation.
    Revoquee,
    /// Inclusion non prouvée contre une tête fraîche.
    Inclusion(String),
    /// Artefact indisponible au dépôt.
    ArtefactIndisponible(ErreurTransport),
    /// Artefact de taille ou d'empreinte différente de l'annonce.
    ArtefactAltere,
    /// Refus du chemin d'installation (StrongFather, écriture…).
    Installateur(String),
}

/// Événement d'un relevé.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    /// Miroir synchronisé : nouvelles entrées `de..a`.
    Synchronise {
        /// Premier nouvel index.
        de: u64,
        /// Taille après synchronisation.
        a: u64,
    },
    /// Entrée sans rapport avec le canal (autre signataire, autre nature).
    EntreeIgnoree {
        /// Index.
        index: u64,
    },
    /// Annonce mise en quarantaine.
    EnQuarantaine {
        /// Composant.
        composant: String,
        /// Version.
        version: u64,
        /// Heure de confiance à partir de laquelle elle est installable.
        installable_a: u64,
    },
    /// Annonce refusée à l'entrée du registre.
    AnnonceRefusee {
        /// Index.
        index: u64,
        /// Motif.
        motif: Motif,
    },
    /// Révocation enregistrée.
    Revocation {
        /// Cible.
        cible: Cible,
    },
    /// Annonce en attente annulée (révoquée, remplacée par une version supérieure).
    Annulee {
        /// Composant.
        composant: String,
        /// Version.
        version: u64,
    },
    /// Quarantaine non écoulée ou tête postérieure à la quarantaine pas encore vue.
    EnAttente {
        /// Composant.
        composant: String,
        /// Version.
        version: u64,
    },
    /// Mise à jour installée.
    Installee {
        /// Composant.
        composant: String,
        /// Version.
        version: u64,
    },
    /// État de quarantaine persisté refusé en tout ou partie (altéré,
    /// illisible, autre registre) : les annonces concernées repartent pour une
    /// quarantaine complète (à remonter à WorrySentinel).
    QuarantaineAlteree {
        /// Enregistrements refusés (MAC faux).
        refuses: u64,
        /// État entier refusé.
        illisible: bool,
    },
    /// Installation refusée (l'annonce reste en attente sauf rétrogradation / révocation).
    InstallationRefusee {
        /// Composant.
        composant: String,
        /// Version.
        version: u64,
        /// Motif.
        motif: Motif,
    },
}

#[derive(Debug, Clone)]
struct EnAttente {
    annonce: AnnonceVerifiee,
    index: u64,
    vue_le: u64,
}

type Cle = (Categorie, String);

/// Releveur du canal officiel (zone COG).
#[derive(Debug)]
pub struct Releveur<S: Stockage> {
    config: ConfigCanal,
    miroir: MiroirRegistre<S>,
    en_attente: BTreeMap<Cle, EnAttente>,
    revocations: HashSet<Cible>,
    a_rescanner: bool,
    /// Premières vues restaurées (reprise), consommées au premier relevé.
    vues_persistees: HashMap<[u8; 32], VuePersistee>,
    /// Refus de l'état de quarantaine persisté, signalé au premier relevé.
    alteration: Option<(u64, bool)>,
}

impl<S: Stockage> Releveur<S> {
    /// Premier démarrage : aucune tête acceptée (la première tête valide de la
    /// clé de journal d'Origin est retenue, premier usage ; amorçage épinglé
    /// dans l'image signée : à décider, A-006).
    pub fn new(config: ConfigCanal, stockage: S) -> Result<Self, ErreurCanal> {
        let miroir = MiroirRegistre::ouvrir(config.suivi(), stockage)?;
        Ok(Self {
            config,
            miroir,
            en_attente: BTreeMap::new(),
            revocations: HashSet::new(),
            a_rescanner: true,
            vues_persistees: HashMap::new(),
            alteration: None,
        })
    }

    /// Reprise après redémarrage (`etat`, `temps` : voir [`Self::etat_persistant`] ;
    /// `quarantaine` : voir [`Self::etat_quarantaine`], vide s'il est absent).
    ///
    /// Les annonces en attente sont reconstruites au prochain relevé ; celles
    /// dont l'enregistrement de quarantaine est authentique **reprennent** leur
    /// décompte, les autres repartent pour une quarantaine complète
    /// (fail-closed, jamais raccourcie).
    pub fn reprendre(
        config: ConfigCanal,
        stockage: S,
        etat: &[u8],
        temps: &[u8],
        quarantaine: &[u8],
        cle: &CleQuarantaine,
    ) -> Result<Self, ErreurCanal> {
        let suivi = SuiviTetes::reprendre(
            config.registre,
            config.ancres_tetes.clone(),
            config.politique_tete,
            etat,
            temps,
        )?;
        let miroir = MiroirRegistre::ouvrir(suivi, stockage)?;
        let lu = ouvrir_quarantaine(quarantaine, &config.registre, cle);
        let alteration = lu.altere().then_some((lu.refuses, lu.illisible));
        Ok(Self {
            config,
            miroir,
            en_attente: BTreeMap::new(),
            revocations: HashSet::new(),
            a_rescanner: true,
            vues_persistees: lu.vues,
            alteration,
        })
    }

    /// État de quarantaine scellé à persister après chaque relevé : annonces
    /// en attente et premières vues restaurées non encore consommées.
    pub fn etat_quarantaine(&self, cle: &CleQuarantaine) -> Vec<u8> {
        let mut vues: Vec<VuePersistee> = self
            .en_attente
            .values()
            .map(|a| VuePersistee { identite: *a.annonce.identite(), index: a.index, vue_le: a.vue_le })
            .collect();
        vues.extend(self.vues_persistees.values().copied());
        sceller_quarantaine(&self.config.registre, &vues, cle)
    }

    /// État à persister après chaque relevé : (`EtatRegistre`, fenêtre MTP).
    pub fn etat_persistant(&self) -> Option<(Vec<u8>, Vec<u8>)> {
        let suivi = self.miroir.suivi();
        suivi.etat().map(|e| (e.encoder(), suivi.temps().encoder()))
    }

    /// Miroir du registre officiel.
    pub fn miroir(&self) -> &MiroirRegistre<S> {
        &self.miroir
    }

    /// Annonces en quarantaine : (composant, version, installable à).
    pub fn en_attente(&self) -> Vec<(String, u64, u64)> {
        self.en_attente
            .values()
            .map(|a| {
                let p = a.annonce.publication();
                (p.composant.clone(), p.version, a.vue_le.saturating_add(self.config.quarantaine_secs))
            })
            .collect()
    }

    /// Relevé complet puis installation des mises à jour prêtes.
    pub fn cycle<T: TransportCanal, I: Installateur>(
        &mut self,
        transport: &mut T,
        installateur: &mut I,
        horloge: &mut HorlogeConfiance,
        mural: u64,
    ) -> Result<Vec<Evenement>, ErreurCanal> {
        let mut evenements = self.relever(transport, installateur, horloge, mural)?;
        evenements.extend(self.installer_pretes(transport, installateur, horloge, mural)?);
        Ok(evenements)
    }

    /// Relève la tête et les entrées nouvelles, trie les entrées.
    pub fn relever<T: TransportCanal, I: Installateur>(
        &mut self,
        transport: &mut T,
        installateur: &I,
        horloge: &mut HorlogeConfiance,
        mural: u64,
    ) -> Result<Vec<Evenement>, ErreurCanal> {
        let maintenant = horloge.maintenant(mural)?;
        let sceau_tete = transport.relever(&Demande::Tete)?;
        // Signature et fraîcheur de la tête avant toute autre demande.
        let n = verifier_tete(
            &sceau_tete,
            &self.config.ancres_tetes,
            &self.config.politique_tete,
            &self.config.registre,
            maintenant,
        )?
        .tete()
        .taille;
        let m = self.miroir.taille();
        let manquantes = n.saturating_sub(m);
        if manquantes > MAX_ENTREES_PAR_RELEVE {
            return Err(ErreurCanal::TropDEntrees(manquantes));
        }
        let mut entrees = Vec::with_capacity(usize::try_from(manquantes).unwrap_or(0));
        let mut depuis = m;
        while depuis < n {
            let nombre = u32::try_from((n - depuis).min(u64::from(MAX_ENTREES_PAR_DEMANDE))).unwrap_or(1);
            let reponse = transport.relever(&Demande::Entrees { depuis, nombre })?;
            entrees.extend(decoder_entrees(&reponse, nombre)?);
            depuis += u64::from(nombre);
        }
        let nouvelles = self.miroir.synchroniser(&sceau_tete, &entrees, horloge, maintenant)?;
        let mut evenements = Vec::new();
        if !nouvelles.is_empty() {
            evenements.push(Evenement::Synchronise { de: nouvelles.start, a: nouvelles.end });
        }
        let rescan = self.a_rescanner;
        if let Some((refuses, illisible)) = self.alteration.take() {
            evenements.push(Evenement::QuarantaineAlteree { refuses, illisible });
        }
        let a_trier = if rescan { 0..nouvelles.end } else { nouvelles };
        self.a_rescanner = false;
        for index in a_trier {
            let octets = self.miroir.entree(index).map_err(ErreurJournal::from)?;
            self.trier(index, &octets, installateur, maintenant, &mut evenements);
        }
        if rescan {
            // Premières vues non reprises : annonces révoquées, remplacées ou installées.
            self.vues_persistees.clear();
        }
        Ok(evenements)
    }

    fn reference(&self, cle: &Cle, installateur: &impl Installateur) -> Option<u64> {
        let installee = installateur.version_installee(cle.0, &cle.1);
        let attente = self.en_attente.get(cle).map(|a| a.annonce.publication().version);
        installee.max(attente)
    }

    fn trier(
        &mut self,
        index: u64,
        octets: &[u8],
        installateur: &impl Installateur,
        maintenant: u64,
        evenements: &mut Vec<Evenement>,
    ) {
        // Seuil 1 d'abord : sépare les entrées du canal des autres, signature
        // vérifiée avant tout décodage.
        let Ok(entree) = verifier_entree_canal(octets, &self.config.ancres, maintenant) else {
            evenements.push(Evenement::EntreeIgnoree { index });
            return;
        };
        match entree.corps {
            Corps::Revocation { cible, .. } => self.revoquer(cible, evenements),
            Corps::Publication(_) => self.mettre_en_quarantaine(index, octets, installateur, maintenant, evenements),
            _ => evenements.push(Evenement::EntreeIgnoree { index }),
        }
    }

    fn revoquer(&mut self, cible: Cible, evenements: &mut Vec<Evenement>) {
        self.revocations.insert(cible);
        evenements.push(Evenement::Revocation { cible });
        let revoquees: Vec<Cle> = self
            .en_attente
            .iter()
            .filter(|(_, a)| a.annonce.est_revoquee(std::iter::once(&cible)))
            .map(|(k, _)| k.clone())
            .collect();
        for k in revoquees {
            if let Some(a) = self.en_attente.remove(&k) {
                let p = a.annonce.publication();
                evenements.push(Evenement::Annulee { composant: p.composant.clone(), version: p.version });
            }
        }
    }

    fn mettre_en_quarantaine(
        &mut self,
        index: u64,
        octets: &[u8],
        installateur: &impl Installateur,
        maintenant: u64,
        evenements: &mut Vec<Evenement>,
    ) {
        let refus = |motif| Evenement::AnnonceRefusee { index, motif };
        let annonce = match verifier_annonce(octets, &self.config.ancres, maintenant) {
            Ok(a) => a,
            Err(ErreurCanal::Sceau(e)) => return evenements.push(refus(Motif::Signature(e))),
            Err(e) => return evenements.push(refus(Motif::Invalide(e.to_string()))),
        };
        if annonce.est_revoquee(self.revocations.iter()) {
            return evenements.push(refus(Motif::Revoquee));
        }
        let p: &Publication = annonce.publication();
        let cle = (p.categorie, p.composant.clone());
        if let Some(reference) = self.reference(&cle, installateur) {
            if p.version <= reference {
                return evenements.push(refus(Motif::Retrogradation { reference, proposee: p.version }));
            }
        }
        if let Err(e) = self.miroir.verifier_inclusion(index, maintenant) {
            return evenements.push(refus(Motif::Inclusion(e.to_string())));
        }
        if let Some(ancienne) = self.en_attente.remove(&cle) {
            let a = ancienne.annonce.publication();
            evenements.push(Evenement::Annulee { composant: a.composant.clone(), version: a.version });
        }
        // Reprise : première vue authentique restaurée (même annonce, même
        // index), sinon maintenant. Une vue restaurée postérieure à maintenant
        // est conservée (jamais raccourcir).
        let vue_le = match self.vues_persistees.remove(annonce.identite()) {
            Some(v) if v.index == index => v.vue_le,
            _ => maintenant,
        };
        evenements.push(Evenement::EnQuarantaine {
            composant: p.composant.clone(),
            version: p.version,
            installable_a: vue_le.saturating_add(self.config.quarantaine_secs),
        });
        self.en_attente.insert(cle, EnAttente { annonce, index, vue_le });
    }

    /// Vérifie une annonce livrée hors registre avec sa preuve d'inclusion
    /// (lot 2 §8 : « preuve d'inclusion livrée avec chaque paquet ») contre
    /// la dernière tête acceptée, fraîche.
    pub fn verifier_annonce_incluse(
        &self,
        sceau: &[u8],
        preuve: &PreuveInclusion,
        horloge: &mut HorlogeConfiance,
        mural: u64,
    ) -> Result<AnnonceVerifiee, ErreurCanal> {
        let maintenant = horloge.maintenant(mural)?;
        let annonce = verifier_annonce(sceau, &self.config.ancres, maintenant)?;
        self.miroir.suivi().verifier_inclusion(&hacher_feuille(sceau), preuve, maintenant)?;
        if annonce.est_revoquee(self.revocations.iter()) {
            return Err(ErreurCanal::Revoquee);
        }
        Ok(annonce)
    }

    /// Installe les annonces dont la quarantaine est écoulée (voir module).
    /// Une tête gelée bloque toute installation (refus global).
    pub fn installer_pretes<T: TransportCanal, I: Installateur>(
        &mut self,
        transport: &mut T,
        installateur: &mut I,
        horloge: &mut HorlogeConfiance,
        mural: u64,
    ) -> Result<Vec<Evenement>, ErreurCanal> {
        let maintenant = horloge.maintenant(mural)?;
        let tete_emise_a = self.miroir.suivi().tete_fraiche(maintenant)?.emise_a;
        let mut evenements = Vec::new();
        let cles: Vec<Cle> = self.en_attente.keys().cloned().collect();
        for cle in cles {
            let Some(attente) = self.en_attente.get(&cle).cloned() else { continue };
            let p = attente.annonce.publication().clone();
            let fin = attente.vue_le.saturating_add(self.config.quarantaine_secs);
            // Quarantaine écoulée ET tête émise après sa fin (révocations vues).
            if maintenant < fin || tete_emise_a.saturating_add(TOLERANCE_HORLOGE_SECS) < fin {
                evenements.push(Evenement::EnAttente { composant: p.composant.clone(), version: p.version });
                continue;
            }
            let refus = |motif| Evenement::InstallationRefusee {
                composant: p.composant.clone(),
                version: p.version,
                motif,
            };
            if attente.annonce.est_revoquee(self.revocations.iter()) {
                self.en_attente.remove(&cle);
                evenements.push(refus(Motif::Revoquee));
                continue;
            }
            if let Some(reference) = installateur.version_installee(p.categorie, &p.composant) {
                if p.version <= reference {
                    self.en_attente.remove(&cle);
                    evenements.push(refus(Motif::Retrogradation { reference, proposee: p.version }));
                    continue;
                }
            }
            if let Err(e) = self.miroir.verifier_inclusion(attente.index, maintenant) {
                evenements.push(refus(Motif::Inclusion(e.to_string())));
                continue;
            }
            let artefact = match transport.relever(&Demande::Artefact { empreinte: p.empreinte, taille: p.taille }) {
                Ok(a) => a,
                Err(e) => {
                    evenements.push(refus(Motif::ArtefactIndisponible(e)));
                    continue;
                }
            };
            let empreinte: [u8; 32] = Sha256::digest(&artefact).into();
            if artefact.len() as u64 != p.taille || empreinte != p.empreinte {
                evenements.push(refus(Motif::ArtefactAltere));
                continue;
            }
            let maj = MiseAJourVerifiee { annonce: attente.annonce, index: attente.index, artefact };
            match installateur.installer(&maj) {
                Ok(()) => {
                    self.en_attente.remove(&cle);
                    evenements.push(Evenement::Installee { composant: p.composant.clone(), version: p.version });
                }
                Err(e) => evenements.push(refus(Motif::Installateur(e))),
            }
        }
        Ok(evenements)
    }

    /// `kid` des signataires du canal (Origin, éditeur).
    pub fn signataires_canal(&self) -> [Kid; 2] {
        [*self.config.ancres.origin(), *self.config.ancres.editeur()]
    }
}
