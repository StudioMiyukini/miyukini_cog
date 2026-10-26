//! @id     miyuportail_entree
//! @do     relever_declarations_admises
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — moitié « entrée » du Portail (Franche → zone COG) : dépôt en quarantaine, sas, deuxième validation, CDR, anti-rejeu par auteur, jeton de réponse signé ; aucune capacité de publier (A-022 §3).

//! # Entrée du Portail (A-022 §3)
//!
//! Relève les déclarations déposées par la façade, les fait décoder par le
//! sas isolé, les revalide (deuxième implémentation), les désarme (CDR),
//! tient l'anti-rejeu **par auteur** (A-007) et les inscrit au registre ;
//! chaque déclaration admise reçoit un [`JetonReponse`] signé par la clé de
//! jetons de la session.
//!
//! **Aucune capacité de publier** : ce type n'a ni clé de publication, ni
//! file de publication, ni méthode qui en produise une. Le compilateur le
//! garantit :
//!
//! ```compile_fail
//! # use miyuportail::entree::PortailEntree;
//! # use miyuportail::{IdService, Publication};
//! fn publier_depuis_l_entree(e: &PortailEntree, s: &IdService, p: Publication) {
//!     let _ = e.publier(s, p); // n'existe pas
//! }
//! ```
//!
//! ```compile_fail
//! # use miyuportail::entree::PortailEntree;
//! # use miyuportail::{IdService, JetonReponse, Publication};
//! fn repondre_depuis_l_entree(e: &PortailEntree, s: &IdService, j: &JetonReponse) {
//!     let _ = e.repondre(s, j, Vec::<Publication>::new()); // n'existe pas
//! }
//! ```
//!
//! ## Anti-rejeu par auteur (A-007)
//!
//! Une session épingle **plusieurs** auteurs (appareils, ≤ [`MAX_AUTEURS`]) ;
//! chacun a sa **propre** séquence strictement croissante, indexée par le
//! `kid` de sa clé. Les Cores ajoutent ou retirent un auteur dans la session
//! ouverte ([`PortailEntree::ajouter_auteur`], [`PortailEntree::retirer_auteur`]),
//! sans rouvrir la session ni reconstruire la façade. Un auteur retiré n'est
//! plus admis (même pour les trames déjà en quarantaine ou en vol) ; sa
//! marque haute est **conservée** : réépinglé dans la même session, il
//! reprend au-delà de sa dernière séquence admise (aucun rejeu possible).
//! Le processus `portail-entree` conserve ces marques dans **son** état
//! ([`PortailEntree::marques`], [`PortailEntree::restaurer_marques`]) : un
//! redémarrage ne rouvre pas la porte au rejeu.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use crate::cdr::Desarmement;
use crate::erreurs::{ErreurPortail, RefusDecodage};
use crate::jeton::{CleJetons, IdSession};
use crate::msce::{decoder_interne, kid, Ancres, Contexte, Kid, MAGIC};
use crate::portail::{
    cle_valide, desarmer, empreinte, maintenant, verrouiller_voie, CompteursVoie, ContratVoies, DeclarationAdmise,
    EtatVoie, Fenetre, Identite, PoigneeCoupure, VoieGeree, Voies, MAX_AUTEURS_CONNUS,
};
use crate::registre::{Evenement, Inscription, MotifCoupure};
use crate::sas::{DecodeurIsole, RequeteSas, MAX_AUTEURS, MAX_LOT};
use crate::traite::{IdService, Sens};
use crate::validation2;

/// Session remise à l'entrée par les Cores.
#[derive(Debug)]
pub struct SessionEntree {
    /// Identifiant aléatoire de la session (le même est remis à la sortie).
    pub id: IdSession,
    /// Clé de session de la façade (émise par la zone COG, INV-H6).
    pub cle_facade: [u8; 32],
    /// Auteurs épinglés (1 à [`MAX_AUTEURS`]).
    pub auteurs: Vec<[u8; 32]>,
    /// Clé de signature des jetons de la session (l'entrée seule la détient).
    pub cle_jetons: CleJetons,
}

/// Marque anti-rejeu d'un auteur : `(kid, plus haute séquence admise)`.
pub type Marque = (Kid, u64);

/// Suivi anti-rejeu d'un auteur dans une session.
struct SuiviAuteur {
    /// Épinglé (admis) ; `false` = retiré, marque haute conservée.
    epingle: bool,
    /// Plus haute séquence admise pour cet auteur.
    derniere_sequence: Option<u64>,
}

pub(crate) struct Session {
    id: IdSession,
    /// Clé de façade et auteurs **épinglés** (remis au sas et à la deuxième validation).
    ancres: Ancres,
    /// Suivi par `kid` d'auteur : épinglés et retirés (≤ [`MAX_AUTEURS_CONNUS`]).
    auteurs: BTreeMap<Kid, SuiviAuteur>,
    jetons: CleJetons,
}

impl Session {
    fn new(s: SessionEntree) -> Self {
        let suivi =
            s.auteurs.iter().map(|a| (kid(a), SuiviAuteur { epingle: true, derniere_sequence: None })).collect();
        Self {
            id: s.id,
            ancres: Ancres { emetteur: s.cle_facade, auteurs: s.auteurs },
            auteurs: suivi,
            jetons: s.cle_jetons,
        }
    }

    /// `kid` de l'unique auteur signataire d'une déclaration admise par les
    /// validations ; il doit être épinglé **maintenant** (retrait en vol = refus).
    fn auteur_epingle(&self, signataires: &[Kid]) -> Result<Kid, ErreurPortail> {
        let kid_emetteur = kid(&self.ancres.emetteur);
        if !signataires.contains(&kid_emetteur) {
            return Err(ErreurPortail::AuteurNonEpingle);
        }
        let mut autres = signataires.iter().filter(|k| **k != kid_emetteur);
        let (Some(auteur), None) = (autres.next(), autres.next()) else {
            return Err(ErreurPortail::AuteurNonEpingle);
        };
        match self.auteurs.get(auteur) {
            Some(s) if s.epingle => Ok(*auteur),
            _ => Err(ErreurPortail::AuteurNonEpingle),
        }
    }
}

/// Voie d'un service, côté entrée.
pub struct VoieEntree {
    contrat: ContratVoies,
    etat: EtatVoie,
    /// Instance unique de la déclaration (une voie supprimée puis
    /// redéclarée est une **autre** voie : lots en vol refusés).
    instance: u64,
    generation: u64,
    pub(crate) session: Option<Session>,
    quarantaine: VecDeque<Vec<u8>>,
    debit: Fenetre,
    compteurs: CompteursVoie,
}

impl VoieGeree for VoieEntree {
    fn nouvelle(contrat: ContratVoies) -> Self {
        Self {
            contrat,
            etat: EtatVoie::Desactivee,
            instance: crate::portail::prochaine_instance(),
            generation: 0,
            session: None,
            quarantaine: VecDeque::new(),
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
        self.quarantaine.clear();
    }
}

impl VoieEntree {
    fn identite(&self) -> Identite {
        Identite { instance: self.instance, generation: self.generation }
    }
}

/// Moitié entrée du Portail : relève, jamais ne publie.
pub struct PortailEntree {
    voies: Voies<VoieEntree>,
    sas: Box<dyn DecodeurIsole>,
    cdr: Box<dyn Desarmement>,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PortailEntree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortailEntree").finish_non_exhaustive()
    }
}

impl crate::portail::CoupureVoie for PortailEntree {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        self.poignee_coupure().couper(service, motif)
    }
}

impl PortailEntree {
    /// Entrée sans voie. `sas` : en production, [`crate::sas::SasProcessus`].
    pub fn new(sas: Box<dyn DecodeurIsole>, cdr: Box<dyn Desarmement>, registre: Arc<dyn Inscription>) -> Self {
        Self { voies: Voies::new(), sas, cdr, registre }
    }

    pub(crate) fn table(&self) -> Voies<VoieEntree> {
        self.voies.clone()
    }

    /// Poignée de coupure (ce sens seulement).
    pub fn poignee_coupure(&self) -> PoigneeCoupure {
        PoigneeCoupure::new(Some(self.table()), None, Arc::clone(&self.registre))
    }

    /// Services déclarés.
    pub fn services(&self) -> Result<Vec<IdService>, ErreurPortail> {
        self.voies.services()
    }

    /// Déclare les voies d'un manifeste : tout ou rien.
    pub fn declarer_manifeste(&self, contrats: &[ContratVoies]) -> Result<(), ErreurPortail> {
        self.voies.declarer_manifeste(contrats)
    }

    /// Déclare la voie d'un service.
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

    /// Désactive l'exposition : session close, quarantaine purgée.
    pub fn desactiver(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.voies.desactiver(service)
    }

    /// Rétablit une voie coupée : elle repasse désactivée.
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

    /// Ouvre une session : épingle la clé de session de la façade et les
    /// auteurs ; installe la clé de jetons de la session.
    pub fn ouvrir_session(&self, service: &IdService, session: SessionEntree) -> Result<(), ErreurPortail> {
        if session.auteurs.is_empty() || session.auteurs.len() > MAX_AUTEURS {
            return Err(ErreurPortail::CleInvalide);
        }
        cle_valide(&session.cle_facade)?;
        for (i, a) in session.auteurs.iter().enumerate() {
            cle_valide(a)?;
            if *a == session.cle_facade || session.auteurs[..i].contains(a) {
                return Err(ErreurPortail::CleInvalide);
            }
        }
        cle_valide(&session.cle_jetons.ancre())?;
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.exiger(Sens::Entrante)?;
        voie.nouvelle_generation();
        voie.quarantaine.clear();
        voie.session = Some(Session::new(session));
        Ok(())
    }

    /// Identifiant de la session ouverte, s'il y en a une.
    pub fn session(&self, service: &IdService) -> Result<Option<IdSession>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let id = verrouiller_voie(&partagee)?.session.as_ref().map(|s| s.id);
        Ok(id)
    }

    /// Épingle un auteur (appareil) de plus dans la session ouverte, sans la
    /// rouvrir (A-007). Appel réservé aux Cores.
    ///
    /// Auteur nouveau pour la session : séquence neuve. Auteur retiré puis
    /// réépinglé : sa marque haute est **conservée** (aucun rejeu possible).
    /// Refus : clé invalide ou égale à la clé de façade, auteur déjà épinglé,
    /// plus de [`MAX_AUTEURS`] épinglés ou de [`MAX_AUTEURS_CONNUS`] connus.
    pub fn ajouter_auteur(&self, service: &IdService, auteur: [u8; 32]) -> Result<(), ErreurPortail> {
        cle_valide(&auteur)?;
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        voie.exiger(Sens::Entrante)?;
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        if auteur == session.ancres.emetteur {
            return Err(ErreurPortail::CleInvalide);
        }
        let k = kid(&auteur);
        match session.auteurs.get(&k) {
            Some(s) if s.epingle => return Err(ErreurPortail::AuteurDejaEpingle),
            None if session.auteurs.len() >= MAX_AUTEURS_CONNUS => return Err(ErreurPortail::TropDAuteurs),
            _ => {}
        }
        if session.ancres.auteurs.len() >= MAX_AUTEURS {
            return Err(ErreurPortail::TropDAuteurs);
        }
        session.ancres.auteurs.push(auteur);
        session
            .auteurs
            .entry(k)
            .and_modify(|s| s.epingle = true)
            .or_insert(SuiviAuteur { epingle: true, derniere_sequence: None });
        Ok(())
    }

    /// Retire un auteur de la session ouverte (A-007) : ses trames sont
    /// refusées dès maintenant, y compris celles déjà en quarantaine ou en
    /// cours de décodage ; les autres auteurs continuent. Sa marque haute est
    /// conservée. Le dernier auteur ne se retire pas (fermer la session).
    pub fn retirer_auteur(&self, service: &IdService, auteur: &[u8; 32]) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        let suivi =
            session.auteurs.get_mut(&kid(auteur)).filter(|s| s.epingle).ok_or(ErreurPortail::AuteurNonEpingle)?;
        if session.ancres.auteurs.len() <= 1 {
            return Err(ErreurPortail::TransitionRefusee);
        }
        suivi.epingle = false;
        session.ancres.auteurs.retain(|a| a != auteur);
        Ok(())
    }

    /// Clés des auteurs épinglés de la session ouverte (côté Cores).
    pub fn auteurs(&self, service: &IdService) -> Result<Vec<[u8; 32]>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let voie = verrouiller_voie(&partagee)?;
        Ok(voie.session.as_ref().ok_or(ErreurPortail::SessionAbsente)?.ancres.auteurs.clone())
    }

    /// Marques anti-rejeu de la session ouverte (persistées par le processus
    /// `portail-entree` dans son propre état) : `(session, marques)`.
    pub fn marques(&self, service: &IdService) -> Result<Option<(IdSession, Vec<Marque>)>, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let voie = verrouiller_voie(&partagee)?;
        Ok(voie
            .session
            .as_ref()
            .map(|s| (s.id, s.auteurs.iter().filter_map(|(k, v)| v.derniere_sequence.map(|q| (*k, q))).collect())))
    }

    /// Restaure des marques anti-rejeu persistées pour la **même** session
    /// (redémarrage du processus). Une marque ne fait que monter ; un auteur
    /// marqué mais non épinglé est connu comme **retiré**. Autre session :
    /// refus (les marques d'une session ne valent que pour elle).
    pub fn restaurer_marques(
        &self,
        service: &IdService,
        session: &IdSession,
        marques: &[Marque],
    ) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let s = garde.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        if s.id != *session {
            return Err(ErreurPortail::TransitionRefusee);
        }
        for (k, q) in marques {
            if !s.auteurs.contains_key(k) && s.auteurs.len() >= MAX_AUTEURS_CONNUS {
                return Err(ErreurPortail::TropDAuteurs);
            }
            let suivi = s.auteurs.entry(*k).or_insert(SuiviAuteur { epingle: false, derniere_sequence: None });
            suivi.derniere_sequence = Some(suivi.derniere_sequence.map_or(*q, |d| d.max(*q)));
        }
        Ok(())
    }

    /// Ferme la session : quarantaine purgée.
    pub fn fermer_session(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.nouvelle_generation();
        voie.session = None;
        voie.quarantaine.clear();
        Ok(())
    }

    /// **Côté Franche** : la façade dépose une trame en quarantaine.
    /// Contrôles : voie déclarée, sens ouvert, voie active, session, taille,
    /// liste blanche `MSCE`, débit, quota.
    pub fn deposer(&self, service: &IdService, trame: Vec<u8>) -> Result<(), ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        voie.exiger(Sens::Entrante)?;
        if voie.session.is_none() {
            return Err(ErreurPortail::SessionAbsente);
        }
        if trame.len() > voie.contrat.taille_max_trame {
            return Err(ErreurPortail::TropGrand);
        }
        if !trame.starts_with(MAGIC) {
            return Err(ErreurPortail::ProtocoleRefuse);
        }
        if voie.quarantaine.len() >= voie.contrat.quota_file {
            return Err(ErreurPortail::FilePleine);
        }
        voie.debit.consommer(voie.contrat.debit_max_par_minute)?;
        voie.quarantaine.push_back(trame);
        voie.compteurs.deposees += 1;
        Ok(())
    }

    /// **Côté zone COG** : relève jusqu'à `max` trames (au plus [`MAX_LOT`]).
    ///
    /// Erreur globale si la voie n'est pas relevable ; sinon un résultat par
    /// trame relevée, dans l'ordre de dépôt.
    pub fn relever(
        &self,
        service: &IdService,
        max: usize,
    ) -> Result<Vec<Result<DeclarationAdmise, ErreurPortail>>, ErreurPortail> {
        // 1. Prise du lot (verrou court).
        let (requete, identite) = {
            let partagee = self.voies.voie(service)?;
            let mut garde = verrouiller_voie(&partagee)?;
            let voie = &mut *garde;
            voie.exiger(Sens::Entrante)?;
            let ancres = voie.session.as_ref().ok_or(ErreurPortail::SessionAbsente)?.ancres.clone();
            let n = max.min(MAX_LOT).min(voie.quarantaine.len());
            if n == 0 {
                return Ok(Vec::new());
            }
            let trames: Vec<Vec<u8>> = voie.quarantaine.drain(..n).collect();
            voie.compteurs.relevees += n as u64;
            let requete = RequeteSas {
                service: service.clone(),
                ancres,
                traite: voie.contrat.traite.restreindre(Sens::Entrante),
                trames,
            };
            (requete, voie.identite())
        };

        // 2. Sas isolé, deuxième validation, CDR (hors verrou : les autres voies continuent).
        let candidats: Vec<Result<Candidat, ErreurPortail>> = match self.sas.decoder_lot(&requete) {
            Err(e) => requete.trames.iter().map(|_| Err(ErreurPortail::Sas(e.clone()))).collect(),
            Ok(resultats) if resultats.len() == requete.trames.len() => requete
                .trames
                .iter()
                .zip(resultats)
                .map(|(trame, resultat)| self.controler(&requete, trame, resultat))
                .collect(),
            Ok(_) => {
                requete.trames.iter().map(|_| Err(ErreurPortail::Sas(crate::erreurs::ErreurSas::Reponse))).collect()
            }
        };

        // 3. Anti-rejeu, inscription, jeton (verrou) ; voie coupée entre-temps = tout refusé.
        let partagee = self.voies.voie(service)?;
        let mut garde = verrouiller_voie(&partagee)?;
        let voie = &mut *garde;
        let toujours_valide = voie.etat == EtatVoie::Active && voie.identite() == identite;
        let mut sortie = Vec::with_capacity(candidats.len());
        for candidat in candidats {
            let resultat = if toujours_valide {
                candidat.and_then(|c| self.admettre(voie, c))
            } else {
                Err(ErreurPortail::VoieCoupee)
            };
            if resultat.is_ok() {
                voie.compteurs.admises += 1;
            } else {
                voie.compteurs.refusees += 1;
            }
            sortie.push(resultat);
        }
        Ok(sortie)
    }

    fn controler(
        &self,
        requete: &RequeteSas,
        trame: &[u8],
        resultat: Result<Vec<u8>, RefusDecodage>,
    ) -> Result<Candidat, ErreurPortail> {
        let interne = resultat?;
        let selon_sas = decoder_interne(&interne).map_err(|_| ErreurPortail::ValidationDivergente)?;
        let ctx = Contexte {
            service: &requete.service,
            sens: Sens::Entrante,
            traite: &requete.traite,
            ancres: &requete.ancres,
        };
        let selon_zone_cog = validation2::valider(trame, &ctx).map_err(|_| ErreurPortail::ValidationDivergente)?;
        if selon_sas != selon_zone_cog {
            return Err(ErreurPortail::ValidationDivergente);
        }
        let declaration = desarmer(self.cdr.as_ref(), selon_zone_cog, &requete.traite)?;
        Ok(Candidat { declaration, empreinte: empreinte(trame) })
    }

    fn admettre(&self, voie: &mut VoieEntree, c: Candidat) -> Result<DeclarationAdmise, ErreurPortail> {
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        let auteur = session.auteur_epingle(&c.declaration.signataires)?;
        let suivi = session.auteurs.get_mut(&auteur).ok_or(ErreurPortail::AuteurNonEpingle)?;
        if suivi.derniere_sequence.is_some_and(|s| c.declaration.sequence <= s) {
            return Err(ErreurPortail::Rejeu);
        }
        self.registre
            .inscrire(&Evenement::Entree { service: c.declaration.service.clone(), empreinte: c.empreinte })
            .map_err(|_| ErreurPortail::Inscription)?;
        suivi.derniere_sequence = Some(c.declaration.sequence);
        let jeton = session.jetons.emettre(&c.declaration.service, &session.id, &c.empreinte, maintenant());
        Ok(DeclarationAdmise { declaration: c.declaration, empreinte: c.empreinte, jeton })
    }

    /// Compteurs du sens entrant (`publiees` et `lues` restent à zéro : ils
    /// appartiennent à la sortie).
    pub fn compteurs(&self, service: &IdService) -> Result<CompteursVoie, ErreurPortail> {
        let partagee = self.voies.voie(service)?;
        let compteurs = verrouiller_voie(&partagee)?.compteurs;
        Ok(compteurs)
    }
}

/// Déclaration validée, avant anti-rejeu et jeton.
struct Candidat {
    declaration: crate::msce::Declaration,
    empreinte: crate::registre::Empreinte,
}

#[cfg(test)]
mod tests {
    //! Anti-rejeu par auteur (A-007) : plusieurs appareils dans une session.

    use std::sync::Mutex;

    use ed25519_dalek::SigningKey;

    use super::*;
    use crate::cdr::RefusParDefaut;
    use crate::erreurs::ErreurSas;
    use crate::jeton::nouvel_id_session;
    use crate::msce::{sceller, Champ, Declaration, Valeur};
    use crate::registre::RegistreMemoire;
    use crate::sas::{decoder_reponse, traiter_requete, ResultatTrame};
    use crate::traite::{ChampTraite, ClasseExposition, NatureChamp, Traite, TypeDeclaration};

    const ENTRANT: u64 = 10;

    struct SasEnProcessus;

    impl DecodeurIsole for SasEnProcessus {
        fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
            let r = traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?)
                .map_err(|_| ErreurSas::FinAnormale)?;
            decoder_reponse(&r, requete.trames.len())
        }
    }

    fn cle(graine: u8) -> SigningKey {
        SigningKey::from_bytes(&[graine; 32])
    }

    fn pk(c: &SigningKey) -> [u8; 32] {
        c.verifying_key().to_bytes()
    }

    struct Monde {
        portail: PortailEntree,
        service: IdService,
        facade: SigningKey,
    }

    fn session(facade: &SigningKey, auteurs: Vec<[u8; 32]>) -> SessionEntree {
        SessionEntree { id: nouvel_id_session(), cle_facade: pk(facade), auteurs, cle_jetons: CleJetons::generer() }
    }

    fn monde(auteurs: &[&SigningKey]) -> Monde {
        let service = IdService::new("docs").unwrap();
        let traite = Traite {
            version: 1,
            types: vec![TypeDeclaration {
                code: ENTRANT,
                sens: Sens::Entrante,
                champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Entier, obligatoire: true }],
            }],
        };
        let contrat = ContratVoies {
            service: service.clone(),
            classe: ClasseExposition::H2,
            traite,
            taille_max_trame: 8192,
            debit_max_par_minute: 1000,
            quota_file: 64,
        };
        let portail =
            PortailEntree::new(Box::new(SasEnProcessus), Box::new(RefusParDefaut), Arc::new(RegistreMemoire::new()));
        portail.declarer(contrat).unwrap();
        portail.activer(&service).unwrap();
        let facade = cle(7);
        portail.ouvrir_session(&service, session(&facade, auteurs.iter().map(|a| pk(a)).collect())).unwrap();
        Monde { portail, service, facade }
    }

    impl Monde {
        fn trame(&self, auteur: &SigningKey, sequence: u64) -> Vec<u8> {
            let d = Declaration {
                service: self.service.clone(),
                sens: Sens::Entrante,
                version_traite: 1,
                type_code: ENTRANT,
                sequence,
                emise_le: 1_790_000_000,
                champs: vec![Champ { tag: 1, valeur: Valeur::Entier(sequence) }],
                signataires: Vec::new(),
            };
            sceller(&d, &[&self.facade, auteur]).unwrap()
        }

        /// Dépose puis relève : un résultat par trame.
        fn passer(&self, trames: Vec<Vec<u8>>) -> Vec<Result<u64, ErreurPortail>> {
            for t in trames {
                self.portail.deposer(&self.service, t).unwrap();
            }
            self.portail
                .relever(&self.service, MAX_LOT)
                .unwrap()
                .into_iter()
                .map(|r| r.map(|a| a.declaration.sequence))
                .collect()
        }
    }

    #[test]
    fn sequences_independantes_par_auteur() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a, &b]);
        assert_eq!(
            m.passer(vec![m.trame(&a, 1), m.trame(&b, 1), m.trame(&b, 2), m.trame(&a, 2), m.trame(&b, 3)]),
            vec![Ok(1), Ok(1), Ok(2), Ok(2), Ok(3)]
        );
        let t_a2 = m.trame(&a, 2);
        assert_eq!(
            m.passer(vec![t_a2.clone(), m.trame(&a, 1), m.trame(&b, 4), m.trame(&a, 3), t_a2]),
            vec![Err(ErreurPortail::Rejeu), Err(ErreurPortail::Rejeu), Ok(4), Ok(3), Err(ErreurPortail::Rejeu)]
        );
    }

    #[test]
    fn jeton_signe_par_la_session() {
        let a = cle(9);
        let m = monde(&[&a]);
        m.portail.deposer(&m.service, m.trame(&a, 1)).unwrap();
        let admise = m.portail.relever(&m.service, 1).unwrap().remove(0).unwrap();
        let id = m.portail.session(&m.service).unwrap().unwrap();
        assert_eq!(admise.jeton.session(), &id);
        assert_eq!(admise.jeton.empreinte(), &admise.empreinte);
        assert_eq!(admise.jeton.service(), &m.service);
    }

    #[test]
    fn auteur_non_epingle_refuse() {
        let (a, intrus) = (cle(9), cle(13));
        let m = monde(&[&a]);
        assert_eq!(m.passer(vec![m.trame(&intrus, 1)]), vec![Err(ErreurPortail::Decodage(RefusDecodage::Signataires))]);
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&intrus)), Err(ErreurPortail::AuteurNonEpingle));
    }

    #[test]
    fn ajout_et_retrait_dans_la_session_ouverte() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a]);
        assert_eq!(m.passer(vec![m.trame(&a, 5)]), vec![Ok(5)]);

        m.portail.ajouter_auteur(&m.service, pk(&b)).unwrap();
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&b)), Err(ErreurPortail::AuteurDejaEpingle));
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&m.facade)), Err(ErreurPortail::CleInvalide));
        assert_eq!(m.passer(vec![m.trame(&b, 1)]), vec![Ok(1)]);

        m.portail.deposer(&m.service, m.trame(&a, 6)).unwrap();
        m.portail.retirer_auteur(&m.service, &pk(&a)).unwrap();
        assert_eq!(
            m.passer(vec![m.trame(&a, 7), m.trame(&b, 2)]),
            vec![
                Err(ErreurPortail::Decodage(RefusDecodage::Signataires)),
                Err(ErreurPortail::Decodage(RefusDecodage::Signataires)),
                Ok(2)
            ]
        );
        assert_eq!(m.portail.auteurs(&m.service).unwrap(), vec![pk(&b)]);
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&b)), Err(ErreurPortail::TransitionRefusee));

        m.portail.ajouter_auteur(&m.service, pk(&a)).unwrap();
        assert_eq!(
            m.passer(vec![m.trame(&a, 5), m.trame(&a, 1), m.trame(&a, 6)]),
            vec![Err(ErreurPortail::Rejeu), Err(ErreurPortail::Rejeu), Ok(6)]
        );
    }

    #[test]
    fn marques_restaurees_pour_la_meme_session_seulement() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a, &b]);
        assert_eq!(m.passer(vec![m.trame(&a, 4), m.trame(&b, 2)]), vec![Ok(4), Ok(2)]);
        let (id, marques) = m.portail.marques(&m.service).unwrap().unwrap();
        assert_eq!(marques.len(), 2);

        // « Redémarrage » : nouvelle entrée, même session (même id), marques restaurées.
        let m2 = monde(&[&a, &b]);
        let s =
            SessionEntree { id, cle_facade: pk(&m.facade), auteurs: vec![pk(&a)], cle_jetons: CleJetons::generer() };
        m2.portail.ouvrir_session(&m2.service, s).unwrap();
        m2.portail.restaurer_marques(&m2.service, &id, &marques).unwrap();
        assert_eq!(m2.passer(vec![m2.trame(&a, 4), m2.trame(&a, 5)]), vec![Err(ErreurPortail::Rejeu), Ok(5)]);
        // b marqué mais non épinglé : connu comme retiré, réépinglé il reprend au-delà de 2.
        m2.portail.ajouter_auteur(&m2.service, pk(&b)).unwrap();
        assert_eq!(m2.passer(vec![m2.trame(&b, 2), m2.trame(&b, 3)]), vec![Err(ErreurPortail::Rejeu), Ok(3)]);
        // Autre session : refus.
        assert_eq!(
            m2.portail.restaurer_marques(&m2.service, &[0; 32], &marques),
            Err(ErreurPortail::TransitionRefusee)
        );
    }

    #[test]
    fn retrait_pendant_le_decodage_refuse_l_auteur_retire() {
        /// Sas qui retire A pendant le décodage du lot (retrait « en vol »).
        type Retrait = Option<(Voies<VoieEntree>, IdService, [u8; 32])>;
        struct SasRetirant(Arc<Mutex<Retrait>>);
        impl DecodeurIsole for SasRetirant {
            fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
                if let Some((voies, service, auteur)) = self.0.lock().unwrap().take() {
                    let partagee = voies.voie(&service).unwrap();
                    let mut voie = partagee.lock().unwrap();
                    let session = voie.session.as_mut().unwrap();
                    session.auteurs.get_mut(&kid(&auteur)).unwrap().epingle = false;
                    session.ancres.auteurs.retain(|a| *a != auteur);
                }
                SasEnProcessus.decoder_lot(requete)
            }
        }
        let (a, b) = (cle(9), cle(11));
        let mut m = monde(&[&a, &b]);
        let declencheur = Arc::new(Mutex::new(None));
        m.portail.sas = Box::new(SasRetirant(Arc::clone(&declencheur)));
        *declencheur.lock().unwrap() = Some((m.portail.table(), m.service.clone(), pk(&a)));
        assert_eq!(m.passer(vec![m.trame(&a, 1), m.trame(&b, 1)]), vec![Err(ErreurPortail::AuteurNonEpingle), Ok(1)]);
    }

    #[test]
    fn bornes_des_auteurs() {
        let premiers: Vec<SigningKey> = (0..MAX_AUTEURS as u8).map(|i| cle(100 + i)).collect();
        let refs: Vec<&SigningKey> = premiers.iter().collect();
        let m = monde(&refs);
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&cle(50))), Err(ErreurPortail::TropDAuteurs));
        let a = cle(9);
        assert_eq!(
            m.portail.ouvrir_session(&m.service, session(&m.facade, vec![pk(&a), pk(&a)])),
            Err(ErreurPortail::CleInvalide)
        );
        m.portail.ouvrir_session(&m.service, session(&m.facade, vec![pk(&a)])).unwrap();
        let mut refus = None;
        for i in 0..=MAX_AUTEURS_CONNUS as u16 {
            let mut graine = [0xA5; 32];
            graine[..2].copy_from_slice(&i.to_be_bytes());
            let c = SigningKey::from_bytes(&graine);
            match m.portail.ajouter_auteur(&m.service, pk(&c)) {
                Ok(()) => m.portail.retirer_auteur(&m.service, &pk(&c)).unwrap(),
                Err(e) => {
                    refus = Some(e);
                    break;
                }
            }
        }
        assert_eq!(refus, Some(ErreurPortail::TropDAuteurs));
    }

    #[test]
    fn coupure_ferme_tout_et_session_neuve_repart_de_zero() {
        use crate::portail::CoupureVoie;
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a, &b]);
        assert_eq!(m.passer(vec![m.trame(&a, 3), m.trame(&b, 3)]), vec![Ok(3), Ok(3)]);
        m.portail.couper(&m.service, MotifCoupure::Anomalie).unwrap();
        assert_eq!(m.portail.deposer(&m.service, m.trame(&a, 4)), Err(ErreurPortail::VoieCoupee));
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&cle(13))), Err(ErreurPortail::VoieCoupee));
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&a)), Err(ErreurPortail::SessionAbsente));
        m.portail.retablir(&m.service).unwrap();
        m.portail.activer(&m.service).unwrap();
        m.portail.ouvrir_session(&m.service, session(&m.facade, vec![pk(&a)])).unwrap();
        assert_eq!(m.passer(vec![m.trame(&a, 1)]), vec![Ok(1)]);
    }
}
