//! @id     miyuportail_sortie
//! @do     publier_reponses_et_publications_h1
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — moitié « sortie » du Portail (zone COG → Franche) : réponses portant un jeton à usage unique, publications H1, réencodage signé par la clé de la voie ; aucune capacité de relever (A-022 §3).

//! # Sortie du Portail (A-022 §3)
//!
//! Publie vers la Franche, et **seulement** :
//!
//! - les **réponses** ([`PortailSortie::repondre`]) d'une voie H2/H3, chacune
//!   portant un [`JetonReponse`] signé par l'entrée pour la session en cours,
//!   **consommé** au premier usage ;
//! - les **publications H1** ([`PortailSortie::publier_h1`]), sur une voie
//!   de classe H1 (aucune entrée possible, donc aucun jeton).
//!
//! Dans les deux cas : étiquette `session` ou `publiable` (EX-47), action
//! explicite de l'utilisateur, type déclaré par le traité, CDR, trame MSCE
//! neuve signée par la clé de publication de la voie.
//!
//! **Aucune capacité de relever** : ce type n'a ni quarantaine, ni sas, ni
//! session d'auteurs, ni méthode qui remette une déclaration entrante. Le
//! compilateur le garantit :
//!
//! ```compile_fail
//! # use miyuportail::sortie::PortailSortie;
//! # use miyuportail::IdService;
//! fn relever_depuis_la_sortie(s: &PortailSortie, id: &IdService) {
//!     let _ = s.relever(id, 8); // n'existe pas
//! }
//! ```
//!
//! ```compile_fail
//! # use miyuportail::sortie::PortailSortie;
//! # use miyuportail::IdService;
//! fn deposer_dans_la_sortie(s: &PortailSortie, id: &IdService) {
//!     let _ = s.deposer(id, Vec::new()); // n'existe pas
//! }
//! ```

use std::collections::{BTreeSet, VecDeque};
use std::sync::Arc;

use ed25519_dalek::SigningKey;
use rand_core::OsRng;

use crate::cdr::Desarmement;
use crate::erreurs::ErreurPortail;
use crate::jeton::{IdSession, JetonReponse};
use crate::msce::{conforme_au_traite, sceller, Declaration};
use crate::portail::{
    cle_valide, desarmer, empreinte, maintenant, verrouiller_voie, CompteursVoie, ContratVoies, Declencheur, EtatVoie,
    Fenetre, Identite, PoigneeCoupure, Publication, Sensibilite, VoieGeree, Voies,
};
use crate::registre::{Empreinte, Evenement, Inscription, MotifCoupure};
use crate::traite::{ClasseExposition, IdService, Sens, Traite};

/// Nombre maximal de messages dans une réponse.
pub const MAX_MESSAGES_REPONSE: usize = 32;
/// Nombre maximal de jetons consommés retenus par session : au-delà, la
/// session doit être rouverte (refus, jamais d'oubli d'un jeton consommé).
pub const MAX_JETONS_CONSOMMES: usize = 65_536;

/// Session remise à la sortie par les Cores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionSortie {
    /// Identifiant aléatoire de la session (le même est remis à l'entrée).
    pub id: IdSession,
    /// Clé publique des jetons de la session (la graine reste à l'entrée).
    pub ancre_jetons: [u8; 32],
}

struct Session {
    id: IdSession,
    ancre: [u8; 32],
    /// Empreintes des demandes déjà répondues (jetons consommés).
    consommes: BTreeSet<Empreinte>,
}

/// Voie d'un service, côté sortie.
pub(crate) struct VoieSortie {
    contrat: ContratVoies,
    etat: EtatVoie,
    instance: u64,
    generation: u64,
    session: Option<Session>,
    publication: VecDeque<Vec<u8>>,
    cle_publication: SigningKey,
    sequence_publication: u64,
    debit: Fenetre,
    compteurs: CompteursVoie,
}

impl VoieGeree for VoieSortie {
    fn nouvelle(contrat: ContratVoies) -> Self {
        Self {
            contrat,
            etat: EtatVoie::Desactivee,
            instance: crate::portail::prochaine_instance(),
            generation: 0,
            session: None,
            publication: VecDeque::new(),
            cle_publication: SigningKey::generate(&mut OsRng),
            sequence_publication: 0,
            debit: Fenetre::new(),
            compteurs: CompteursVoie::default(),
        }
    }

    fn contrat(&self) -> &ContratVoies {
        &self.contrat
    }

    fn etat(&self) -> EtatVoie {
        self.etat
    }

    fn poser_etat(&mut self, etat: EtatVoie) {
        self.etat = etat;
    }

    fn nouvelle_generation(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    fn purger(&mut self) {
        self.session = None;
        self.publication.clear();
    }

    fn au_retablissement(&mut self) {
        self.cle_publication = SigningKey::generate(&mut OsRng);
        self.sequence_publication = 0;
    }
}

impl VoieSortie {
    fn identite(&self) -> Identite {
        Identite { instance: self.instance, generation: self.generation }
    }
}

/// État persistable de la clé de publication d'une voie : `(graine, dernière séquence)`.
pub type EtatCle = ([u8; 32], u64);

/// Moitié sortie du Portail : publie, jamais ne relève.
pub struct PortailSortie {
    voies: Voies<VoieSortie>,
    cdr: Box<dyn Desarmement>,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PortailSortie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortailSortie").finish_non_exhaustive()
    }
}

impl crate::portail::CoupureVoie for PortailSortie {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        self.poignee_coupure().couper(service, motif)
    }
}

impl PortailSortie {
    /// Sortie sans voie.
    pub fn new(cdr: Box<dyn Desarmement>, registre: Arc<dyn Inscription>) -> Self {
        Self { voies: Voies::new(), cdr, registre }
    }

    pub(crate) fn table(&self) -> Voies<VoieSortie> {
        self.voies.clone()
    }

    /// Poignée de coupure (ce sens seulement).
    pub fn poignee_coupure(&self) -> PoigneeCoupure {
        PoigneeCoupure::new(None, Some(self.table()), Arc::clone(&self.registre))
    }

    /// Services déclarés.
    pub fn services(&self) -> Result<Vec<IdService>, ErreurPortail> {
        self.voies.services()
    }

    /// Déclare les voies d'un manifeste : tout ou rien.
    pub fn declarer_manifeste(&self, contrats: &[ContratVoies]) -> Result<(), ErreurPortail> {
        self.voies.declarer_manifeste(contrats)
    }

    /// Déclare la voie d'un service (clé de publication neuve).
    pub fn declarer(&self, contrat: ContratVoies) -> Result<(), ErreurPortail> {
        self.declarer_manifeste(std::slice::from_ref(&contrat))
    }

    /// Supprime la voie (aucun résidu).
    pub fn supprimer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.voies.supprimer(service)
    }

    /// Active l'exposition (EX-48).
    pub fn activer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.voies.activer(service)
    }

    /// Désactive l'exposition : session close (jetons périmés), file purgée.
    pub fn desactiver(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.voies.desactiver(service)
    }

    /// Rétablit une voie coupée : désactivée, **nouvelle** clé de publication.
    pub fn retablir(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.voies.retablir(service)
    }

    /// État d'une voie.
    pub fn etat(&self, service: &IdService) -> Result<EtatVoie, ErreurPortail> {
        self.voies.etat(service)
    }

    /// Contrat d'une voie.
    pub fn contrat(&self, service: &IdService) -> Result<ContratVoies, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let contrat = verrouiller_voie(&partagee)?.contrat.clone();
        Ok(contrat)
    }

    /// Ouvre une session : les jetons d'une autre session sont refusés.
    pub fn ouvrir_session(&self, service: &IdService, session: SessionSortie) -> Result<(), ErreurPortail> {
        cle_valide(&session.ancre_jetons)?;
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.exiger(Sens::Sortante)?;
        voie.nouvelle_generation();
        voie.session = Some(Session { id: session.id, ancre: session.ancre_jetons, consommes: BTreeSet::new() });
        Ok(())
    }

    /// Identifiant de la session ouverte, s'il y en a une.
    pub fn session(&self, service: &IdService) -> Result<Option<IdSession>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let id = verrouiller_voie(&partagee)?.session.as_ref().map(|s| s.id);
        Ok(id)
    }

    /// Ferme la session : tous ses jetons sont périmés.
    pub fn fermer_session(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.nouvelle_generation();
        voie.session = None;
        Ok(())
    }

    /// Jetons consommés de la session ouverte (persistés par le processus
    /// `portail-sortie` dans son propre état).
    pub fn jetons_consommes(&self, service: &IdService) -> Result<Option<(IdSession, Vec<Empreinte>)>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let voie = verrouiller_voie(&partagee)?;
        Ok(voie.session.as_ref().map(|s| (s.id, s.consommes.iter().copied().collect())))
    }

    /// Restaure les jetons consommés de la **même** session (redémarrage) :
    /// un jeton déjà consommé le reste. Autre session : refus.
    pub fn restaurer_jetons(
        &self,
        service: &IdService,
        session: &IdSession,
        consommes: &[Empreinte],
    ) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        let s = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        if s.id != *session {
            return Err(ErreurPortail::TransitionRefusee);
        }
        if s.consommes.len().saturating_add(consommes.len()) > MAX_JETONS_CONSOMMES {
            return Err(ErreurPortail::JetonsEpuises);
        }
        s.consommes.extend(consommes.iter().copied());
        Ok(())
    }

    /// Clé de publication et dernière séquence (persistées par `portail-sortie`).
    pub fn etat_cle(&self, service: &IdService) -> Result<EtatCle, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let voie = verrouiller_voie(&partagee)?;
        Ok((voie.cle_publication.to_bytes(), voie.sequence_publication))
    }

    /// Installe une clé de publication persistée (redémarrage de `portail-sortie`).
    pub fn installer_cle(&self, service: &IdService, (graine, sequence): EtatCle) -> Result<(), ErreurPortail> {
        let cle = SigningKey::from_bytes(&graine);
        cle_valide(&cle.verifying_key().to_bytes())?;
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.cle_publication = cle;
        voie.sequence_publication = sequence;
        voie.publication.clear();
        voie.nouvelle_generation();
        Ok(())
    }

    /// Réponse à une déclaration admise (voie H2/H3) : jeton vérifié
    /// (signature de la session en cours, service), **consommé** ; 1 à
    /// [`MAX_MESSAGES_REPONSE`] messages, publiés tous ou aucun. Rend les
    /// numéros de publication.
    pub fn repondre(
        &self,
        service: &IdService,
        jeton: &JetonReponse,
        publications: Vec<Publication>,
    ) -> Result<Vec<u64>, ErreurPortail> {
        if publications.is_empty() || publications.len() > MAX_MESSAGES_REPONSE {
            return Err(ErreurPortail::ReponseHorsBornes);
        }
        if jeton.service() != service {
            return Err(ErreurPortail::JetonInvalide);
        }
        self.publier_lot(service, publications, Some(jeton))
    }

    /// Publication d'une voie **H1** (diffusion, aucun jeton possible). Une
    /// voie H2/H3 ne publie qu'en réponse ([`Self::repondre`]).
    pub fn publier_h1(&self, service: &IdService, publication: Publication) -> Result<u64, ErreurPortail> {
        if self.contrat(service)?.classe != ClasseExposition::H1 {
            return Err(ErreurPortail::JetonRequis);
        }
        self.publier_sans_jeton(service, publication)
    }

    /// Publication sans jeton : réservée à [`crate::PortailInterne`]
    /// (processus unique) et à [`Self::publier_h1`].
    pub(crate) fn publier_sans_jeton(
        &self,
        service: &IdService,
        publication: Publication,
    ) -> Result<u64, ErreurPortail> {
        let mut numeros = self.publier_lot(service, vec![publication], None)?;
        numeros.pop().ok_or(ErreurPortail::EtatIndisponible)
    }

    fn publier_lot(
        &self,
        service: &IdService,
        publications: Vec<Publication>,
        jeton: Option<&JetonReponse>,
    ) -> Result<Vec<u64>, ErreurPortail> {
        for p in &publications {
            if p.etiquette == Sensibilite::Nationale {
                return Err(ErreurPortail::SortieNationale);
            }
            if p.declencheur != Declencheur::Utilisateur {
                return Err(ErreurPortail::ActionUtilisateurRequise);
            }
        }
        // 1. Contrôles de voie et instantané du traité.
        let (traite, identite) = {
            let partagee = self.voies.voie(service)?;
            let voie = verrouiller_voie(&partagee)?;
            voie.exiger(Sens::Sortante)?;
            if let Some(j) = jeton {
                Self::verifier_jeton(&voie, j)?;
            }
            (voie.contrat.traite.clone(), voie.identite())
        };
        // 2. Conformité au traité et CDR (hors verrou).
        let mut brouillons = Vec::with_capacity(publications.len());
        for p in publications {
            brouillons.push(self.preparer(service, &traite, p)?);
        }

        // 3. Jeton consommé, réencodage signé, quota, débit, inscription (verrou) : tout ou rien.
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        voie.exiger(Sens::Sortante)?;
        if voie.identite() != identite {
            return Err(ErreurPortail::VoieCoupee);
        }
        if let Some(j) = jeton {
            Self::verifier_jeton(voie, j)?;
        }
        if voie.publication.len().saturating_add(brouillons.len()) > voie.contrat.quota_file {
            return Err(ErreurPortail::FilePleine);
        }
        let emise_le = maintenant();
        let mut trames = Vec::with_capacity(brouillons.len());
        let mut sequence = voie.sequence_publication;
        for mut d in brouillons {
            sequence = sequence.checked_add(1).ok_or(ErreurPortail::Rejeu)?;
            d.sequence = sequence;
            d.emise_le = emise_le;
            let trame = sceller(&d, &[&voie.cle_publication])?;
            if trame.len() > voie.contrat.taille_max_trame {
                return Err(ErreurPortail::TropGrand);
            }
            trames.push((d.sequence, trame));
        }
        voie.debit.consommer_n(voie.contrat.debit_max_par_minute, trames.len() as u32)?;
        if let Some(j) = jeton {
            let session = voie.session.as_mut().ok_or(ErreurPortail::JetonInvalide)?;
            if session.consommes.len() >= MAX_JETONS_CONSOMMES {
                return Err(ErreurPortail::JetonsEpuises);
            }
            // Consommé avant toute publication : un échec d'inscription ne le rend pas réutilisable.
            session.consommes.insert(*j.empreinte());
        }
        let mut numeros = Vec::with_capacity(trames.len());
        for (numero, trame) in trames {
            self.registre
                .inscrire(&Evenement::Publication { service: service.clone(), empreinte: empreinte(&trame) })
                .map_err(|_| ErreurPortail::Inscription)?;
            voie.sequence_publication = numero;
            voie.publication.push_back(trame);
            voie.compteurs.publiees += 1;
            numeros.push(numero);
        }
        Ok(numeros)
    }

    fn verifier_jeton(voie: &VoieSortie, jeton: &JetonReponse) -> Result<(), ErreurPortail> {
        let session = voie.session.as_ref().ok_or(ErreurPortail::JetonInvalide)?;
        if jeton.session() != &session.id || jeton.service() != &voie.contrat.service {
            return Err(ErreurPortail::JetonInvalide);
        }
        jeton.verifier(&session.ancre)?;
        if session.consommes.contains(jeton.empreinte()) {
            return Err(ErreurPortail::JetonDejaUtilise);
        }
        Ok(())
    }

    fn preparer(&self, service: &IdService, traite: &Traite, p: Publication) -> Result<Declaration, ErreurPortail> {
        let brouillon = Declaration {
            service: service.clone(),
            sens: Sens::Sortante,
            version_traite: traite.version,
            type_code: p.type_code,
            sequence: 0,
            emise_le: 0,
            champs: p.champs,
            signataires: Vec::new(),
        };
        conforme_au_traite(&brouillon, traite)?;
        desarmer(self.cdr.as_ref(), brouillon, traite)
    }

    /// **Côté Franche** : la façade lit la prochaine publication (trame MSCE neuve).
    pub fn lire_publication(&self, service: &IdService) -> Result<Option<Vec<u8>>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        voie.exiger(Sens::Sortante)?;
        let trame = voie.publication.pop_front();
        if trame.is_some() {
            voie.compteurs.lues += 1;
        }
        Ok(trame)
    }

    /// Clé publique de publication de la voie (vérification par la façade).
    pub fn cle_publication(&self, service: &IdService) -> Result<[u8; 32], ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let cle = verrouiller_voie(&partagee)?.cle_publication.verifying_key().to_bytes();
        Ok(cle)
    }

    /// Compteurs du sens sortant (seuls `publiees` et `lues` sont tenus ici).
    pub fn compteurs(&self, service: &IdService) -> Result<CompteursVoie, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let compteurs = verrouiller_voie(&partagee)?.compteurs;
        Ok(compteurs)
    }
}
