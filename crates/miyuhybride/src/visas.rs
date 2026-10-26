//! @id     miyuhybride_visas
//! @do     guichet_des_visas_zone_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — guichet des visas (A-022 §1) : la façade dépose une demande typée par le Portail, la zone COG vérifie passeport, délégations, révocations et preuve de possession, puis émet un visa signé par la clé `visas` (indice 5) avec sa délégation 0x45.

//! # Guichet des visas (A-022 §1, I3)
//!
//! Le visa est émis **en zone COG** : la façade (Franche) ne détient aucune
//! clé d'émission (INV-H2), elle ne fait que transmettre une demande typée
//! par une voie du Portail et renvoyer la réponse.
//!
//! ```text
//! appareil ──HTTP──▶ façade (Franche) ──MSCE (voie « miyukini.visas »)──▶ entrée ──▶ GuichetVisas (zone COG)
//!                                                                                         │ visa signé (indice 5)
//! appareil ◀──HTTP── façade ◀──publication à jeton (A-027)── sortie ◀─────────────────────┘ + délégation 0x45
//! ```
//!
//! ## Traité « miyukini.visas » v1 (codecs [`crate::message_traite!`], champ inconnu refusé)
//!
//! | sens | code | message | champs (tag : nature, borne) |
//! |---|---|---|---|
//! | entrant | 1 | [`DemandeVisaGuichet`] | 1 `passeport` (octets ≤ 4096) · 2 `delegation_passeport` (octets ≤ 1024, facultatif) · 3 `cle_appareil` (32 o) · 4 `audience` (texte ≤ 64) · 5 `portees` (texte, portées triées séparées par `\n`) · 6 `duree_secs` (u64, 1..=900) · 7 `emise_le` (u64) · 8 `defi` (32 o) · 9 `preuve` (64 o) |
//! | sortant | 101 | [`VisaEmis`] | 1 `demande` (32 o) · 2 `visa` (sceau `0x02`, ≤ 4096) · 3 `delegation` (sceau `0x06`/`0x45` de la clé `visas`, ≤ 1024) · 4 `pas_apres` (u64) |
//! | sortant | 102 | [`VisaRefuse`] | 1 `demande` (32 o) · 2 `motif` ([`MotifRefusVisa`], aucun texte libre) |
//!
//! `demande` = SHA-256 de la trame déposée (corrélation par la façade).
//!
//! ## Preuve de possession de la demande
//!
//! `preuve = Ed25519(cle_appareil, "miyukini/pop/v1" ‖ defi ‖ audience ‖ liaison)`
//! (format de la doc lot 2 §7), avec
//! `liaison = SHA-256("miyukini/guichet-visas/v1" ‖ 0 ‖ SHA-256(passeport) ‖ cle_appareil ‖ lp(audience) ‖ lp(portees) ‖ duree_secs (u64 BE) ‖ emise_le (u64 BE))`
//! ([`liaison_demande_visa`], `lp` = longueur u16 BE + octets) : la preuve
//! couvre tout le contenu de la demande ; la façade ne peut rien en changer.
//!
//! Le `defi` (32 o) est tiré par la façade ou l'appareil : la zone COG ne lui
//! fait pas confiance et tient elle-même l'**usage unique** (registre de
//! nonces persistant, clé = appareil + défi, `miyauth::http::RegistreNonces`)
//! et la **fraîcheur** (`|maintenant − emise_le| ≤ 120 s`). Un rejeu est
//! refusé, y compris après redémarrage.
//!
//! ## Signature d'auteur de la voie
//!
//! Le Portail exige sur toute trame entrante la clé de session de la façade
//! **et** un auteur épinglé. Les appareils demandeurs ne sont pas connus
//! d'avance : la zone COG tire, pour chaque session de la voie des visas,
//! une **clé d'auteur éphémère** remise à la façade avec la clé de session
//! ([`GuichetVisasFranche`], INV-H6, aucun secret durable). Sur cette voie,
//! l'authenticité de la demande ne repose donc **pas** sur la signature
//! d'auteur mais sur le passeport et la preuve de possession, vérifiés en
//! zone COG ; la signature d'auteur sert seulement à l'anti-rejeu de la voie.
//!
//! ## Émission (zone COG, fonctionnalité `visas`, [`GuichetVisas`])
//!
//! Dans l'ordre, premier refus = réponse [`VisaRefuse`] : décodage strict ;
//! fraîcheur de la demande ; durée 1..=900 s ; audience servie par le
//! guichet ; portées canoniques ; délégation du passeport présentée
//! (vérifiée contre la racine épinglée) ; passeport (chaîne A-024,
//! destination, révocations **≤ 15 min** via [`crate::RevocationsFacade`]) ;
//! clé d'appareil = celle du passeport ; preuve de possession ; clé `visas`
//! non révoquée ; défi non rejoué (enregistré) ; fenêtre du visa
//! `[max(maintenant, début du passeport), min(maintenant + durée, fin du
//! passeport, fin de la délégation visas)]` ; portées ⊆ passeport ; visa
//! scellé par la clé `visas` (indice 5), délégation `0x45` jointe.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use ed25519_dalek::SigningKey;
use miyuportail::{ClasseExposition, ContratVoies, Declaration, EtatVoie, IdService, PortailInterne, Traite};
use miyusceau::CleSignature;
use rand_core::OsRng;
use sha2::{Digest, Sha256};

use crate::erreurs::{ErreurHybride, ErreurTraite};
use crate::guichets::{
    declaration_entrante, signer_auteur, AuteursSession, EmpreinteDemande, GuichetFranche, GuichetMoteur,
};
use crate::message::MessageTraite;

/// Service (voie du Portail) du guichet des visas.
pub const SERVICE_VISAS: &str = "miyukini.visas";
/// Version du traité du guichet des visas.
pub const VERSION_TRAITE_VISAS: u64 = 1;
/// Borne d'un titre (passeport, visa) dans le traité.
pub const MAX_TITRE: u32 = 4096;
/// Borne d'une délégation `0x45` dans le traité.
pub const MAX_DELEGATION: u32 = 1024;
/// Borne du texte des portées (32 portées de 128 caractères + séparateurs).
pub const MAX_PORTEES_TEXTE: u32 = 32 * 129;
/// Écart maximal entre `emise_le` et l'horloge de la zone COG.
pub const FENETRE_DEMANDE_SECS: u64 = 120;
/// Durée maximale d'un visa (A-022 §1 : 15 min).
pub const DUREE_MAX_VISA_SECS: u64 = 15 * 60;
/// Domaine de la liaison de la preuve de possession d'une demande de visa.
pub const DOMAINE_LIAISON_VISA: &[u8] = b"miyukini/guichet-visas/v1";
/// Taille maximale d'une trame de la voie des visas.
pub const TAILLE_TRAME_VISAS: usize = 16 * 1024;
/// Débit de la voie des visas (trames par minute et par sens).
pub const DEBIT_VOIE_VISAS: u32 = 600;
/// Profondeur des files de la voie des visas.
pub const QUOTA_FILE_VISAS: usize = 256;

crate::message_traite! {
    /// Demande de visa (façade → zone COG).
    pub struct DemandeVisaGuichet (Entrante, 1) {
        /// Passeport scellé (`0x01`).
        1 => pub passeport: Vec<u8>, max MAX_TITRE;
        /// Délégation `0x45` de la clé `emission` du COG d'origine (facultative).
        2 => pub delegation_passeport: Option<Vec<u8>>, max MAX_DELEGATION;
        /// Clé publique de l'appareil (32 o), égale à celle du passeport.
        3 => pub cle_appareil: Vec<u8>, max 32;
        /// Service visé (audience du visa).
        4 => pub audience: String, max 64;
        /// Portées demandées, triées, séparées par `\n` ([`portees_texte`]).
        5 => pub portees: String, max MAX_PORTEES_TEXTE;
        /// Durée demandée (1..=900 s).
        6 => pub duree_secs: u64, max 0;
        /// Date d'émission de la demande (secondes Unix).
        7 => pub emise_le: u64, max 0;
        /// Défi de la preuve de possession (32 o).
        8 => pub defi: Vec<u8>, max 32;
        /// Signature de possession (64 o).
        9 => pub preuve: Vec<u8>, max 64;
    }
}

crate::message_traite! {
    /// Visa émis (zone COG → façade).
    pub struct VisaEmis (Sortante, 101) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Visa scellé (`0x02`), signé par la clé `visas` (indice 5).
        2 => pub visa: Vec<u8>, max MAX_TITRE;
        /// Délégation `0x45` de la clé `visas`, à présenter avec le visa.
        3 => pub delegation: Vec<u8>, max MAX_DELEGATION;
        /// Fin de validité du visa.
        4 => pub pas_apres: u64, max 0;
    }
}

crate::message_traite! {
    /// Demande refusée (motif par code, aucun texte libre).
    pub struct VisaRefuse (Sortante, 102) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Code de [`MotifRefusVisa`].
        2 => pub motif: u64, max 0;
    }
}

/// Motif de refus d'une demande de visa (codes stables du traité).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum MotifRefusVisa {
    /// Demande mal formée (champs, tailles, portées non canoniques).
    Invalide = 1,
    /// Demande trop ancienne ou datée dans le futur.
    Perimee = 2,
    /// Durée hors de 1..=900 s.
    DureeInvalide = 3,
    /// Audience non servie par ce guichet.
    AudienceRefusee = 4,
    /// Passeport refusé (format, émetteur, destination, fenêtre).
    PasseportRefuse = 5,
    /// Passeport, appareil ou clé révoqués.
    Revoque = 6,
    /// Révocations absentes ou de plus de 15 min.
    RevocationsIndisponibles = 7,
    /// Délégation absente, expirée ou incohérente.
    Delegation = 8,
    /// Preuve de possession invalide.
    Possession = 9,
    /// Portée demandée absente du passeport.
    PorteeHorsPasseport = 10,
    /// Défi déjà utilisé.
    Rejeu = 11,
    /// Guichet saturé ou erreur interne.
    Indisponible = 12,
}

impl MotifRefusVisa {
    /// Motif depuis son code.
    pub const fn depuis_code(code: u64) -> Option<Self> {
        Some(match code {
            1 => Self::Invalide,
            2 => Self::Perimee,
            3 => Self::DureeInvalide,
            4 => Self::AudienceRefusee,
            5 => Self::PasseportRefuse,
            6 => Self::Revoque,
            7 => Self::RevocationsIndisponibles,
            8 => Self::Delegation,
            9 => Self::Possession,
            10 => Self::PorteeHorsPasseport,
            11 => Self::Rejeu,
            12 => Self::Indisponible,
            _ => return None,
        })
    }

    /// Texte court stable (réponses HTTP de la façade).
    pub const fn texte(self) -> &'static str {
        match self {
            Self::Invalide => "invalide",
            Self::Perimee => "perimee",
            Self::DureeInvalide => "duree",
            Self::AudienceRefusee => "audience",
            Self::PasseportRefuse => "passeport",
            Self::Revoque => "revoque",
            Self::RevocationsIndisponibles => "revocations",
            Self::Delegation => "delegation",
            Self::Possession => "possession",
            Self::PorteeHorsPasseport => "portee",
            Self::Rejeu => "rejeu",
            Self::Indisponible => "indisponible",
        }
    }
}

/// Traité du guichet des visas.
pub fn traite_visas() -> Traite {
    crate::message::traite(
        VERSION_TRAITE_VISAS,
        vec![DemandeVisaGuichet::type_declaration(), VisaEmis::type_declaration(), VisaRefuse::type_declaration()],
    )
}

/// Contrat de la voie des visas (H2 : demande entrante, réponse sortante à jeton).
pub fn contrat_voies_visas() -> Result<ContratVoies, ErreurHybride> {
    Ok(ContratVoies {
        service: IdService::new(SERVICE_VISAS)?,
        classe: ClasseExposition::H2,
        traite: traite_visas(),
        taille_max_trame: TAILLE_TRAME_VISAS,
        debit_max_par_minute: DEBIT_VOIE_VISAS,
        quota_file: QUOTA_FILE_VISAS,
    })
}

/// Réponse du guichet, décodée par la façade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReponseVisas {
    /// Visa émis.
    Emis(VisaEmis),
    /// Demande refusée.
    Refuse(VisaRefuse),
}

impl ReponseVisas {
    /// Décodage strict d'une publication de la voie.
    pub fn depuis(d: &Declaration) -> Result<Self, ErreurTraite> {
        match d.type_code {
            VisaEmis::CODE => VisaEmis::depuis_declaration(d).map(Self::Emis),
            VisaRefuse::CODE => VisaRefuse::depuis_declaration(d).map(Self::Refuse),
            _ => Err(ErreurTraite::Type),
        }
    }

    /// Empreinte de la demande à laquelle elle répond.
    pub fn demande(&self) -> &[u8] {
        match self {
            Self::Emis(v) => &v.demande,
            Self::Refuse(r) => &r.demande,
        }
    }
}

/// Texte canonique des portées : triées, sans doublon, séparées par `\n`.
pub fn portees_texte<S: AsRef<str>>(portees: &[S]) -> String {
    let mut v: Vec<&str> = portees.iter().map(AsRef::as_ref).collect();
    v.sort_unstable();
    v.dedup();
    v.join("\n")
}

fn lp(h: &mut Sha256, o: &[u8]) {
    let n = u16::try_from(o.len()).unwrap_or(u16::MAX);
    h.update(n.to_be_bytes());
    h.update(o);
}

/// Liaison de la preuve de possession d'une demande de visa (voir l'en-tête).
pub fn liaison_demande_visa(
    passeport: &[u8],
    cle_appareil: &[u8; 32],
    audience: &str,
    portees: &str,
    duree_secs: u64,
    emise_le: u64,
) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOMAINE_LIAISON_VISA);
    h.update([0u8]);
    h.update(Sha256::digest(passeport));
    h.update(cle_appareil);
    lp(&mut h, audience.as_bytes());
    lp(&mut h, portees.as_bytes());
    h.update(duree_secs.to_be_bytes());
    h.update(emise_le.to_be_bytes());
    h.finalize().into()
}

// ---------------------------------------------------------------- Cores : voie

/// Voie du guichet des visas, tenue par les Cores (zone COG) : déclarée
/// **désactivée** (EX-48), activée explicitement, une session par façade.
pub struct VoieVisas {
    portail: Arc<PortailInterne>,
    service: IdService,
    traite: Traite,
    epoque: Arc<AtomicU64>,
}

impl std::fmt::Debug for VoieVisas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VoieVisas").field("service", &self.service).finish_non_exhaustive()
    }
}

impl VoieVisas {
    /// Déclare la voie (désactivée).
    pub fn installer(portail: Arc<PortailInterne>) -> Result<Self, ErreurHybride> {
        let contrat = contrat_voies_visas()?;
        let service = contrat.service.clone();
        let traite = contrat.traite.clone();
        portail.declarer(contrat)?;
        Ok(Self { portail, service, traite, epoque: Arc::new(AtomicU64::new(0)) })
    }

    /// Active l'exposition (décision explicite).
    pub fn activer(&self) -> Result<(), ErreurHybride> {
        Ok(self.portail.activer(&self.service)?)
    }

    /// État de la voie.
    pub fn etat(&self) -> Result<EtatVoie, ErreurHybride> {
        Ok(self.portail.etat(&self.service)?)
    }

    /// Ouvre une session : clé de session **et** clé d'auteur éphémères,
    /// tirées ici (zone COG) et remises à la façade (INV-H6).
    pub fn ouvrir_session(&self) -> Result<GuichetVisasFranche, ErreurHybride> {
        let cle_session = SigningKey::generate(&mut OsRng);
        let auteur = CleSignature::generer();
        let pk = auteur.cle_publique();
        self.portail.ouvrir_session(&self.service, cle_session.verifying_key().to_bytes(), vec![pk])?;
        self.epoque.fetch_add(1, Ordering::SeqCst);
        let auteurs: AuteursSession = Arc::new(RwLock::new(vec![pk]));
        let guichet = GuichetFranche::new(Arc::clone(&self.portail), self.service.clone(), self.traite.clone(), cle_session, auteurs)?;
        Ok(GuichetVisasFranche { guichet, auteur, sequence: AtomicU64::new(0) })
    }

    /// Ferme la session (réponses en attente périmées).
    pub fn fermer_session(&self) -> Result<(), ErreurHybride> {
        self.epoque.fetch_add(1, Ordering::SeqCst);
        Ok(self.portail.fermer_session(&self.service)?)
    }

    /// Guichet du moteur (zone COG) de la voie.
    pub fn guichet_moteur(&self) -> GuichetMoteur {
        GuichetMoteur::new(Arc::clone(&self.portail), self.service.clone(), Arc::clone(&self.epoque))
    }
}

// ---------------------------------------------------------------- Franche

/// Côté façade de la voie des visas : dépose des demandes, lit les réponses.
/// Aucune clé durable : clé de session et clé d'auteur éphémères de la session.
pub struct GuichetVisasFranche {
    guichet: GuichetFranche,
    auteur: CleSignature,
    sequence: AtomicU64,
}

impl std::fmt::Debug for GuichetVisasFranche {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuichetVisasFranche").field("service", self.guichet.service()).finish_non_exhaustive()
    }
}

impl GuichetVisasFranche {
    /// Dépose une demande (déclaration `emise_le`) ; rend l'empreinte de la trame.
    pub fn deposer(&self, demande: &DemandeVisaGuichet, emise_le: u64) -> Result<EmpreinteDemande, ErreurHybride> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst).saturating_add(1);
        let d = declaration_entrante(self.guichet.service(), self.guichet.version_traite(), demande, sequence, emise_le)?;
        let signature = signer_auteur(&self.auteur, &d)?;
        self.guichet.deposer(&d, &signature)
    }

    /// Lit les réponses publiées (au plus `max`), vérifiées (clé de la voie,
    /// traité) ; une publication hors traité est écartée.
    pub fn lire_reponses(&self, max: usize) -> Result<Vec<ReponseVisas>, ErreurHybride> {
        let mut r = Vec::new();
        for _ in 0..max {
            match self.guichet.lire_publication()? {
                Some(d) => {
                    if let Ok(rep) = ReponseVisas::depuis(&d) {
                        r.push(rep);
                    }
                }
                None => break,
            }
        }
        Ok(r)
    }

    /// État de la voie.
    pub fn etat(&self) -> Result<EtatVoie, ErreurHybride> {
        self.guichet.etat()
    }
}

// ---------------------------------------------------------------- zone COG : émission

#[cfg(feature = "visas")]
pub use emission::{BilanGuichet, ConfigGuichetVisas, GuichetVisas};

#[cfg(feature = "visas")]
mod emission {
    use std::collections::BTreeSet;

    use miyauth::http::{RefusHttp, RegistreNonces};
    use miyauth::{
        emettre_visa, passeport::kid_appareil, prouver_possession, verifier_passeport, AnnuaireEmetteurs, CibleRevocation,
        CogId, DemandeVisa, PolitiqueIdentite, Portees, RefusIdentite, SourceRevocation, INDICE_USAGE_VISAS,
    };
    use miyusceau::CleSignature;

    use super::{
        liaison_demande_visa, DemandeVisaGuichet, MotifRefusVisa, VisaEmis, VisaRefuse, DUREE_MAX_VISA_SECS,
        FENETRE_DEMANDE_SECS,
    };
    use crate::erreurs::ErreurHybride;
    use crate::guichets::{GuichetMoteur, MessageSortant};
    use crate::message::MessageTraite;
    use crate::revocations::RevocationsFacade;
    use miyuportail::Declaration;

    /// Bilan d'une relève du guichet.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct BilanGuichet {
        /// Visas émis.
        pub emis: u64,
        /// Demandes refusées (réponse [`VisaRefuse`]).
        pub refusees: u64,
        /// Trames refusées par le Portail ou réponses non publiées.
        pub erreurs: u64,
    }

    /// Configuration du guichet (voir [`GuichetVisas::new`]).
    #[derive(Debug)]
    pub struct ConfigGuichetVisas<'a> {
        /// COG local (émetteur des visas).
        pub cog_local: CogId,
        /// Annuaire : racine du COG local épinglée, COG reconnus.
        pub annuaire: &'a AnnuaireEmetteurs,
        /// Clé d'usage `visas` (indice 5), détenue en zone COG seulement.
        pub cle_visas: CleSignature,
        /// Délégation `0x45` de `cle_visas` (sceau complet).
        pub delegation_visas: Vec<u8>,
        /// Services pour lesquels un visa peut être émis.
        pub audiences: &'a [&'a str],
    }

    /// Guichet des visas (zone COG) : seul détenteur de la clé `visas`.
    pub struct GuichetVisas {
        cog_local: CogId,
        annuaire: AnnuaireEmetteurs,
        cle_visas: CleSignature,
        delegation_visas: Vec<u8>,
        fin_delegation: u64,
        audiences: BTreeSet<String>,
        revocations: RevocationsFacade,
        nonces: RegistreNonces,
    }

    impl std::fmt::Debug for GuichetVisas {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("GuichetVisas").field("cog_local", &self.cog_local).finish_non_exhaustive()
        }
    }

    fn motif_identite(r: &RefusIdentite) -> MotifRefusVisa {
        match r {
            RefusIdentite::Revoque(_) => MotifRefusVisa::Revoque,
            RefusIdentite::Revocation(_) => MotifRefusVisa::RevocationsIndisponibles,
            RefusIdentite::SansDelegation
            | RefusIdentite::UsageIncorrect { .. }
            | RefusIdentite::DelegationHorsFenetre
            | RefusIdentite::DelegationIncoherente
            | RefusIdentite::RacineIncoherente => MotifRefusVisa::Delegation,
            RefusIdentite::PossessionInvalide => MotifRefusVisa::Possession,
            RefusIdentite::PorteeHorsPasseport => MotifRefusVisa::PorteeHorsPasseport,
            _ => MotifRefusVisa::PasseportRefuse,
        }
    }

    /// Vrai si le passeport (lu **sans** vérification, seulement pour choisir le
    /// motif d'un refus déjà acquis) vient d'un COG à racine épinglée dont la
    /// clé signataire n'a aucune délégation connue.
    fn sans_delegation(annuaire: &AnnuaireEmetteurs, octets: &[u8]) -> bool {
        let Ok(sceau) = miyusceau::Sceau::decoder(octets) else {
            return false;
        };
        let Ok(p) = miyusceau::decoder_titre::<miyauth::Passeport>(&sceau.charge) else {
            return false;
        };
        annuaire.racine_epinglee(&p.emetteur_cog) && annuaire.delegation(&p.emetteur_kid).is_none()
    }

    impl GuichetVisas {
        /// Guichet du COG `config.cog_local` :
        ///
        /// - `annuaire` doit épingler la **racine** de `cog_local` (sinon
        ///   [`RefusIdentite::RacineIncoherente`]) ; sans ancre directe, il
        ///   passe en mode strict ;
        /// - `delegation_visas` (sceau `0x45`) doit certifier `cle_visas` pour
        ///   l'indice 5 de `cog_local` et couvrir `maintenant` ;
        /// - `audiences` : services pour lesquels un visa peut être émis ;
        /// - `revocations` : listes remises par les Cores (≤ 15 min) ;
        /// - `nonces` : registre des défis consommés, **persistant** en
        ///   production ([`RegistreNonces::persistant`]).
        pub fn new(
            config: ConfigGuichetVisas<'_>,
            revocations: RevocationsFacade,
            nonces: RegistreNonces,
            maintenant: u64,
        ) -> Result<Self, RefusIdentite> {
            let ConfigGuichetVisas { cog_local, annuaire, cle_visas, delegation_visas, audiences } = config;
            if !annuaire.racine_epinglee(&cog_local) {
                return Err(RefusIdentite::RacineIncoherente);
            }
            let mut annuaire = annuaire.avec_delegations(&[&delegation_visas], maintenant)?;
            let (cog, d) = annuaire.delegation(&cle_visas.kid()).ok_or(RefusIdentite::SansDelegation)?;
            if cog != &cog_local {
                return Err(RefusIdentite::DelegationIncoherente);
            }
            if d.indice() != INDICE_USAGE_VISAS {
                return Err(RefusIdentite::UsageIncorrect { attendu: INDICE_USAGE_VISAS, delegue: d.indice() });
            }
            let fin_delegation = d.pas_apres();
            if !annuaire.a_des_ancres_directes() {
                annuaire.exiger_delegations();
            }
            if audiences.is_empty() || !audiences.iter().all(|a| miyauth::titres::audience_valide(a)) {
                return Err(RefusIdentite::ParametreInvalide("audiences"));
            }
            Ok(Self {
                cog_local,
                annuaire,
                cle_visas,
                delegation_visas,
                fin_delegation,
                audiences: audiences.iter().map(ToString::to_string).collect(),
                revocations,
                nonces,
            })
        }

        /// Fin de validité de la délégation de la clé `visas` (rotation à prévoir avant).
        pub fn fin_delegation(&self) -> u64 {
            self.fin_delegation
        }

        /// Traite une déclaration admise : rend la réponse à publier (jamais d'erreur :
        /// tout refus devient un [`VisaRefuse`]).
        pub fn traiter_declaration(&self, d: &Declaration, demande: &[u8; 32], maintenant: u64) -> MessageSortant {
            let sortant = match self.emettre(d, demande, maintenant) {
                Ok(v) => MessageSortant::de(&v),
                Err(motif) => MessageSortant::de(&VisaRefuse { demande: demande.to_vec(), motif: motif as u64 }),
            };
            // Les deux messages sont sortants : `de` ne peut pas échouer ; repli défensif.
            sortant.unwrap_or_else(|_| MessageSortant::refus_interne(demande))
        }

        fn emettre(&self, d: &Declaration, demande: &[u8; 32], maintenant: u64) -> Result<VisaEmis, MotifRefusVisa> {
            let m = DemandeVisaGuichet::depuis_declaration(d).map_err(|_| MotifRefusVisa::Invalide)?;
            if maintenant.abs_diff(m.emise_le) > FENETRE_DEMANDE_SECS {
                return Err(MotifRefusVisa::Perimee);
            }
            if m.duree_secs == 0 || m.duree_secs > DUREE_MAX_VISA_SECS {
                return Err(MotifRefusVisa::DureeInvalide);
            }
            if !self.audiences.contains(&m.audience) {
                return Err(MotifRefusVisa::AudienceRefusee);
            }
            let cle_appareil: [u8; 32] = m.cle_appareil.as_slice().try_into().map_err(|_| MotifRefusVisa::Invalide)?;
            let defi: [u8; 32] = m.defi.as_slice().try_into().map_err(|_| MotifRefusVisa::Invalide)?;
            let preuve: [u8; 64] = m.preuve.as_slice().try_into().map_err(|_| MotifRefusVisa::Invalide)?;
            let portees = Portees::new(m.portees.split('\n')).ok_or(MotifRefusVisa::Invalide)?;
            if portees.as_slice().join("\n") != m.portees {
                return Err(MotifRefusVisa::Invalide);
            }
            let presentees: Vec<&[u8]> = m.delegation_passeport.iter().map(Vec::as_slice).collect();
            let annuaire =
                self.annuaire.avec_delegations(&presentees, maintenant).map_err(|_| MotifRefusVisa::Delegation)?;
            let politique =
                PolitiqueIdentite { cog_local: self.cog_local.clone(), annuaire: &annuaire, revocations: &self.revocations };
            let pv = verifier_passeport(&m.passeport, &politique, maintenant).map_err(|r| match r {
                // Signataire inconnu d'un COG à racine épinglée : délégation `emission` absente ou refusée.
                RefusIdentite::Sceau(_) if sans_delegation(&annuaire, &m.passeport) => MotifRefusVisa::Delegation,
                r => motif_identite(&r),
            })?;
            if pv.passeport().cle_appareil != cle_appareil {
                return Err(MotifRefusVisa::Possession);
            }
            let liaison = liaison_demande_visa(&m.passeport, &cle_appareil, &m.audience, &m.portees, m.duree_secs, m.emise_le);
            let possession =
                prouver_possession(pv, &defi, &m.audience, &liaison, &preuve).map_err(|r| motif_identite(&r))?;
            match self.revocations.est_revoque(&self.cog_local, &CibleRevocation::Cle(self.cle_visas.kid()), maintenant) {
                Ok(false) => {}
                Ok(true) => return Err(MotifRefusVisa::Revoque),
                Err(_) => return Err(MotifRefusVisa::RevocationsIndisponibles),
            }
            // Usage unique du défi, tenu en zone COG (persistant), après toutes les vérifications.
            self.nonces.consommer(kid_appareil(&cle_appareil), &defi, maintenant).map_err(|e| match e {
                RefusHttp::Rejeu => MotifRefusVisa::Rejeu,
                _ => MotifRefusVisa::Indisponible,
            })?;
            let p = possession.passeport().passeport();
            let pas_avant = maintenant.max(p.pas_avant);
            let pas_apres = maintenant.saturating_add(m.duree_secs).min(p.pas_apres).min(self.fin_delegation);
            if pas_apres <= pas_avant || maintenant >= self.fin_delegation {
                return Err(MotifRefusVisa::Delegation);
            }
            let visa = emettre_visa(
                &self.cog_local,
                &self.cle_visas,
                &possession,
                &DemandeVisa { audience: m.audience.clone(), portees, pas_avant, pas_apres },
            )
            .map_err(|r| motif_identite(&r))?;
            Ok(VisaEmis { demande: demande.to_vec(), visa, delegation: self.delegation_visas.clone(), pas_apres })
        }

        /// Relève jusqu'à `max` demandes admises et répond à chacune (jeton à usage unique).
        pub fn traiter(&self, moteur: &GuichetMoteur, max: usize, maintenant: u64) -> Result<BilanGuichet, ErreurHybride> {
            let mut b = BilanGuichet::default();
            for r in moteur.relever(max)? {
                let Ok(admise) = r else {
                    b.erreurs += 1;
                    continue;
                };
                let demande = *admise.jeton.empreinte();
                let sortant = self.traiter_declaration(&admise.declaration, &demande, maintenant);
                let emis = sortant.type_code() == VisaEmis::CODE;
                match moteur.repondre(admise.jeton, std::slice::from_ref(&sortant)) {
                    Ok(_) if emis => b.emis += 1,
                    Ok(_) => b.refusees += 1,
                    Err(_) => b.erreurs += 1,
                }
            }
            Ok(b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuportail::{Champ, Valeur};

    fn demande() -> DemandeVisaGuichet {
        DemandeVisaGuichet {
            passeport: vec![1; 300],
            delegation_passeport: None,
            cle_appareil: vec![2; 32],
            audience: "jayrdv".into(),
            portees: portees_texte(&["b", "a", "a"]),
            duree_secs: 900,
            emise_le: 5,
            defi: vec![3; 32],
            preuve: vec![4; 64],
        }
    }

    #[test]
    fn traite_et_contrat_valides() {
        traite_visas().valider().unwrap();
        contrat_voies_visas().unwrap().valider().unwrap();
        assert_eq!(portees_texte(&["b", "a", "a"]), "a\nb");
    }

    #[test]
    fn codecs_stricts_champ_inconnu_refuse() {
        let d = demande();
        let mut champs = d.en_champs();
        assert_eq!(DemandeVisaGuichet::depuis_champs(&champs).unwrap(), d);
        champs.push(Champ { tag: 10, valeur: Valeur::Entier(1) });
        assert_eq!(DemandeVisaGuichet::depuis_champs(&champs), Err(ErreurTraite::ChampInconnu(10)));
        let sans_preuve: Vec<Champ> = d.en_champs().into_iter().filter(|c| c.tag != 9).collect();
        assert_eq!(DemandeVisaGuichet::depuis_champs(&sans_preuve), Err(ErreurTraite::ChampAbsent(9)));
        let r = VisaRefuse { demande: vec![0; 32], motif: MotifRefusVisa::Rejeu as u64 };
        assert_eq!(VisaRefuse::depuis_champs(&r.en_champs()).unwrap(), r);
        assert_eq!(MotifRefusVisa::depuis_code(11), Some(MotifRefusVisa::Rejeu));
        assert_eq!(MotifRefusVisa::depuis_code(99), None);
    }

    #[test]
    fn liaison_couvre_tout_le_contenu() {
        let base = liaison_demande_visa(b"p", &[1; 32], "jayrdv", "a", 900, 5);
        assert_ne!(base, liaison_demande_visa(b"q", &[1; 32], "jayrdv", "a", 900, 5));
        assert_ne!(base, liaison_demande_visa(b"p", &[2; 32], "jayrdv", "a", 900, 5));
        assert_ne!(base, liaison_demande_visa(b"p", &[1; 32], "jaykoa", "a", 900, 5));
        assert_ne!(base, liaison_demande_visa(b"p", &[1; 32], "jayrdv", "b", 900, 5));
        assert_ne!(base, liaison_demande_visa(b"p", &[1; 32], "jayrdv", "a", 901, 5));
        assert_ne!(base, liaison_demande_visa(b"p", &[1; 32], "jayrdv", "a", 900, 6));
    }
}
