//! @id     central_remote_identite
//! @do     emettre_passeports_visas_appareils
//! @role   security
//! @layer  toolkit
//! @human  central-remote — Central COG d'origine : passeports d'appareil, visas, preuve de possession, révocation.

//! Central en tant que **COG d'origine** des appareils appairés (sortie du lot 2,
//! inventaire #1 à #4 ; doc gelée lot 2 §5 à §8).
//!
//! # Clé d'émission
//!
//! Passeports **et** visas sont scellés par la clé d'émission du COG (doc §3.1,
//! usage `emission`). Sa graine (32 octets bruts) est lue dans un fichier `0600`
//! ([`RemoteServerConfig::cle_emission_path`](crate::server::RemoteServerConfig),
//! défaut [`FICHIER_CLE_EMISSION_DEFAUT`] à côté de l'identité E2E) :
//!
//! - **racine COG configurée** (`miyuracine`, racine hors ligne, décision 2) :
//!   l'opérateur exporte la graine d'émission de la génération courante
//!   (`miyuracine deriver --part … --part … --exporter-graine-usage emission
//!   --generation <g> --sortie <fichier>`), la place à ce chemin et renseigne
//!   `cog_id` (= `cog_id` de la racine). La racine ne vient jamais sur le serveur.
//!   La chaîne racine → clé d'émission (certificat) n'est pas encore spécifiée
//!   (A-003) : la clé d'émission est épinglée directement.
//! - **sans racine** : le fichier est créé une fois (graine aléatoire, `0600`,
//!   création exclusive) et `cog_id` = `base32(SHA-256(clé d'émission))[0..26]`
//!   (même convention que la clé de visas d'Origin, `[forum] cle_emission_path`).
//!
//! Fichier présent mais lien symbolique, non régulier, aux droits ouverts ou de
//! taille ≠ 32 ⇒ le serveur refuse de démarrer.
//!
//! # Chaîne racine → clés d'usage (A-022, A-024) et mode strict
//!
//! Avec un **dossier de chaîne** ([`RemoteServerConfig::chaine_dir`](crate::server::RemoteServerConfig),
//! défaut : variable `CENTRAL_REMOTE_CHAINE_DIR`), la racine du COG est connue
//! et le mode strict est activé (`AnnuaireEmetteurs::exiger_delegations`) :
//!
//! | fichier | contenu |
//! |---|---|
//! | `racine.pub` | clé publique d'identité (indice 0), 64 hex ; `cog_id` = son empreinte |
//! | `emission.delegation` | sceau `0x45` (binaire) de la clé `emission` (indice 1), qui signe les passeports |
//! | `visas.key` | graine (32 o, `0600`) de la clé `visas` (indice 5), qui signe les visas |
//! | `visas.delegation` | sceau `0x45` (binaire) de la clé `visas` |
//!
//! Produits par la cérémonie (`miyuracine deleguer --usage emission|visas …`,
//! `miyuracine deriver … --exporter-graine-usage visas`) ; la graine
//! `emission` reste au chemin de la clé d'émission. Chaque visa est rendu avec
//! l'en-tête `X-Miyu-Delegations` (délégations `visas` puis `emission`) et ne
//! dépasse jamais la fin de la délégation `visas` ; les délégations présentées
//! par l'appareil avec un visa sont vérifiées (`avec_delegations`). Sans
//! dossier de chaîne : mode de transition (clé d'émission épinglée
//! directement, signe passeports et visas).
//!
//! # Défis et nonces (I5)
//!
//! [`AutoriteAppareils::charger`] les persiste dans `central-remote-etat/`
//! (à côté de la clé d'émission, `0700`) : usage unique tenu après un
//! redémarrage et entre instances (`miyauth::http::persistant`).
//!
//! # Titulaires
//!
//! Le passeport porte un identifiant **par paire** (EX-36) du titulaire : le
//! profil Central lié à l'appareil à l'approbation (`profile_id`), à défaut le
//! titulaire par défaut du COG. Secret du titulaire = `HKDF-SHA256(sel =
//! "miyukini/central-remote/titulaire/v1", ikm = graine du fichier
//! [`FICHIER_TITULAIRES_DEFAUT`] (`0600`, à côté de la clé d'émission), info =
//! "profil:" || profile_id | "proprietaire")`. Stable à travers les rotations de
//! la clé d'émission.
//!
//! # Révocation (EX-14, fail-closed)
//!
//! [`AutoriteAppareils`] est la [`SourceRevocation`] de la politique : un titre est
//! refusé si sa série, la clé d'appareil ou la clé d'émission figure dans la
//! [`miyauth::ListeRevocation`] tenue par le [`PairingManager`] (révocation d'un
//! appareil depuis Central) **ou** si la clé d'appareil n'est plus dans la liste
//! blanche persistée (révocation durable, y compris après redémarrage).

use crate::pairing::{PairedDevice, PairingManager};
use central_remote_protocol::identite::{
    AUDIENCE, PASSEPORT_ENDPOINT, PORTEE_PILOTAGE, USAGE_PASSEPORT,
};
use hkdf::Hkdf;
use miyauth::http::{
    verifier_requete_passeport_avec_delegations, verifier_requete_signee,
    verifier_requete_visa_avec_delegations, DefiEmis, EntetesSignees, MagasinDefis,
    MagasinDefisFichier, MagasinDefisMemoire, RefusHttp, RegistreNonces, RequeteHttp,
};
use miyauth::titres::DUREE_MAX_VISA_SECS;
use miyauth::{
    emettre_passeport, emettre_visa, AnnuaireEmetteurs, CibleRevocation, CogId, DemandePasseport,
    DemandeVisa, ErreurRevocation, IdentityRole, PolitiqueIdentite, Portees, RefusIdentite,
    SecretTitulaire, SourceRevocation, INDICE_USAGE_EMISSION, INDICE_USAGE_VISAS,
};
use miyusceau::{CleSignature, NiveauConfiance};
use sha2::Sha256;
use std::path::Path;
use std::sync::Arc;
use zeroize::Zeroizing;

/// Fichier par défaut de la graine d'émission (à côté de l'identité E2E).
pub const FICHIER_CLE_EMISSION_DEFAUT: &str = "central-remote-emission.key";
/// Fichier de la graine des secrets de titulaires (à côté de la clé d'émission).
pub const FICHIER_TITULAIRES_DEFAUT: &str = "central-remote-titulaires.key";
/// Durée d'un passeport d'appareil (choix : 24 h ≤ 30 j de la doc ; l'appareil
/// le renouvelle à volonté par requête signée).
pub const DUREE_PASSEPORT_APPAREIL_SECS: u64 = 24 * 3600;
/// Sel de domaine des secrets de titulaires.
pub const SEL_TITULAIRE: &[u8] = b"miyukini/central-remote/titulaire/v1";
/// Dossier (à côté de la clé d'émission) des défis et nonces persistants.
pub const DOSSIER_ETAT_DEFAUT: &str = "central-remote-etat";

/// Chaîne racine → clés d'usage (A-024) : racine connue, mode strict.
#[derive(Debug)]
pub struct ChaineCog {
    /// Clé publique d'identité (indice 0) du COG.
    pub racine: [u8; 32],
    /// Délégation `0x45` de la clé `emission` (indice 1).
    pub delegation_emission: Vec<u8>,
    /// Clé `visas` (indice 5).
    pub cle_visas: CleSignature,
    /// Délégation `0x45` de la clé `visas`.
    pub delegation_visas: Vec<u8>,
}

impl ChaineCog {
    /// Lit le dossier de chaîne (voir l'en-tête du module) ; tout fichier
    /// absent, mal formé ou aux droits ouverts ⇒ erreur (refus de démarrer).
    pub fn charger(dossier: &Path) -> Result<Self, String> {
        let lire = |nom: &str| {
            std::fs::read(dossier.join(nom)).map_err(|e| format!("chaîne {} : {e}", dossier.join(nom).display()))
        };
        let racine_hex = String::from_utf8(lire("racine.pub")?).map_err(|_| "racine.pub : texte attendu")?;
        let racine: [u8; 32] = hex::decode(racine_hex.trim())
            .ok()
            .and_then(|v| v.try_into().ok())
            .ok_or("racine.pub : clé publique (64 hex) attendue")?;
        let cle_visas = miyauth::http::service::charger_cle(&dossier.join("visas.key"))
            .map_err(|e| format!("visas.key : {e}"))?;
        Ok(Self {
            racine,
            delegation_emission: lire("emission.delegation")?,
            cle_visas,
            delegation_visas: lire("visas.delegation")?,
        })
    }
}

/// Accès prouvé d'un appareil appairé (visa + possession vérifiés).
#[derive(Debug, Clone)]
pub struct AccesAppareil {
    /// Identifiant opaque `pw:<cog>:<hex(sujet)>`.
    pub identite: String,
    /// Rôle accordé.
    pub role: IdentityRole,
    /// Fin de validité du visa (secondes Unix).
    pub expire_a: u64,
    /// Clé publique de l'appareil.
    pub cle_appareil: [u8; 32],
    /// Appareil appairé correspondant.
    pub appareil: PairedDevice,
}

/// Visa délivré.
#[derive(Debug, Clone)]
pub struct VisaDelivre {
    /// Visa scellé.
    pub visa: Vec<u8>,
    /// Accès correspondant (fenêtre du visa).
    pub acces: AccesAppareil,
}

/// Texte stable d'un rôle.
pub fn texte_role(role: IdentityRole) -> &'static str {
    match role {
        IdentityRole::Citizen => "citizen",
        IdentityRole::Visitor => "visitor",
        IdentityRole::External => "external",
    }
}

fn identite_opaque(cog: &CogId, sujet: &[u8; 32]) -> String {
    format!("pw:{cog}:{}", hex::encode(sujet))
}

/// Central, COG d'origine des appareils appairés.
pub struct AutoriteAppareils {
    cog: CogId,
    cle: CleSignature,
    /// Clé qui scelle les visas : `visas` (indice 5) en mode chaîné, sinon `cle`.
    cle_visas: CleSignature,
    /// Délégations rendues avec chaque visa (`visas`, `emission`) ; vide en transition.
    delegations: Vec<Vec<u8>>,
    /// Fin de la délégation `visas` (aucun visa au-delà) ; `u64::MAX` en transition.
    fin_visas: u64,
    annuaire: AnnuaireEmetteurs,
    titulaires: Zeroizing<[u8; 32]>,
    pairing: Arc<PairingManager>,
    defis: Box<dyn MagasinDefis>,
    nonces: RegistreNonces,
}

impl std::fmt::Debug for AutoriteAppareils {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoriteAppareils").field("cog", &self.cog).finish_non_exhaustive()
    }
}

impl AutoriteAppareils {
    /// Construit l'autorité à partir de la clé d'émission et de la graine des
    /// titulaires ; active la liste de révocation du [`PairingManager`].
    /// `cog_id` : celui de la racine si elle existe, sinon dérivé de la clé.
    pub fn new(
        cle: CleSignature,
        cog_id: Option<&str>,
        graine_titulaires: [u8; 32],
        pairing: Arc<PairingManager>,
    ) -> Result<Self, String> {
        let cog = match cog_id {
            Some(c) => CogId::new(c.trim()).ok_or_else(|| format!("cog_id invalide « {c} »"))?,
            None => CogId::new(&miyuracine::cog_id_depuis_cle(&cle.cle_publique()))
                .ok_or("cog_id dérivé invalide")?,
        };
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire
            .ajouter(cog.clone(), &cle.cle_publique(), NiveauConfiance::Souverain)
            .map_err(|e| format!("clé d'émission : {e}"))?;
        pairing
            .activer_revocations(cog.clone())
            .map_err(|e| format!("révocations : {e}"))?;
        Ok(Self {
            cog,
            cle_visas: CleSignature::depuis_graine(&cle.graine()),
            cle,
            delegations: Vec::new(),
            fin_visas: u64::MAX,
            annuaire,
            titulaires: Zeroizing::new(graine_titulaires),
            pairing,
            defis: Box::new(MagasinDefisMemoire::default()),
            nonces: RegistreNonces::default(),
        })
    }

    /// Autorité en **mode chaîné** (racine connue, mode strict) : passeports
    /// signés par `cle` (clé `emission`, indice 1), visas par la clé `visas`
    /// (indice 5), chacune certifiée par sa délégation ; `cog_id` = celui de la racine.
    pub fn new_chaine(
        cle: CleSignature,
        chaine: ChaineCog,
        graine_titulaires: [u8; 32],
        pairing: Arc<PairingManager>,
        maintenant: u64,
    ) -> Result<Self, String> {
        let cog = CogId::new(&miyuracine::cog_id_depuis_cle(&chaine.racine)).ok_or("cog_id de la racine invalide")?;
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire
            .epingler_racine(cog.clone(), &chaine.racine, NiveauConfiance::Souverain)
            .map_err(|e| format!("racine : {e}"))?;
        for (octets, cle_attendue, indice) in [
            (&chaine.delegation_emission, &cle, INDICE_USAGE_EMISSION),
            (&chaine.delegation_visas, &chaine.cle_visas, INDICE_USAGE_VISAS),
        ] {
            let kid = annuaire.ajouter_delegation(octets, maintenant).map_err(|e| format!("délégation : {e}"))?;
            let d = annuaire.delegation(&kid).map(|(_, d)| d.clone()).ok_or("délégation absente")?;
            if kid != cle_attendue.kid() || d.indice() != indice {
                return Err(format!("délégation d'indice {} : clé ou usage incohérents", d.indice()));
            }
        }
        let fin_visas = annuaire
            .delegation(&chaine.cle_visas.kid())
            .map(|(_, d)| d.pas_apres())
            .ok_or("délégation visas absente")?;
        annuaire.exiger_delegations();
        pairing.activer_revocations(cog.clone()).map_err(|e| format!("révocations : {e}"))?;
        Ok(Self {
            cog,
            cle,
            cle_visas: chaine.cle_visas,
            delegations: vec![chaine.delegation_visas, chaine.delegation_emission],
            fin_visas,
            annuaire,
            titulaires: Zeroizing::new(graine_titulaires),
            pairing,
            defis: Box::new(MagasinDefisMemoire::default()),
            nonces: RegistreNonces::default(),
        })
    }

    /// Défis et nonces **persistants et partagés** dans `dossier` (I5).
    pub fn avec_etat_persistant(mut self, dossier: &Path) -> Result<Self, String> {
        self.defis = Box::new(
            MagasinDefisFichier::ouvrir_defaut(&dossier.join("defis")).map_err(|e| format!("défis : {e}"))?,
        );
        self.nonces = RegistreNonces::persistant(
            &dossier.join("nonces"),
            miyauth::http::CAPACITE_NONCES_DEFAUT,
            miyauth::http::FENETRE_HORODATAGE_SECS,
        )
        .map_err(|e| format!("nonces : {e}"))?;
        Ok(self)
    }

    /// Vrai si l'autorité est en mode chaîné (racine connue, délégations exigées).
    pub fn mode_strict(&self) -> bool {
        self.annuaire.est_strict()
    }

    /// Délégations à rendre avec un visa (`X-Miyu-Delegations`) ; vide en transition.
    pub fn delegations_visa(&self) -> &[Vec<u8>] {
        &self.delegations
    }

    /// Charge (ou crée, `0600`) la clé d'émission et la graine des titulaires ;
    /// défis et nonces persistés dans `central-remote-etat/` ; mode chaîné si
    /// `chaine` est donné.
    pub fn charger_avec_chaine(
        cle_emission: &Path,
        cog_id: Option<&str>,
        chaine: Option<&Path>,
        pairing: Arc<PairingManager>,
    ) -> Result<Self, String> {
        let dossier = cle_emission.parent().unwrap_or_else(|| Path::new(""));
        let autorite = match chaine {
            None => Self::charger_transition(cle_emission, cog_id, pairing)?,
            Some(c) => {
                let cle = miyauth::http::service::charger_cle(cle_emission)
                    .map_err(|e| format!("clé d'émission ({}) : {e}", cle_emission.display()))?;
                let chemin_titulaires = dossier.join(FICHIER_TITULAIRES_DEFAUT);
                let titulaires = miyauth::http::service::charger_ou_creer_cle(&chemin_titulaires)
                    .map_err(|e| format!("secrets de titulaires ({}) : {e}", chemin_titulaires.display()))?;
                let a = Self::new_chaine(
                    cle,
                    ChaineCog::charger(c)?,
                    titulaires.graine(),
                    pairing,
                    miyauth::http::service::maintenant_unix(),
                )?;
                if let Some(attendu) = cog_id {
                    if attendu.trim() != a.cog.as_str() {
                        return Err("cog_id configuré différent de celui de la racine".into());
                    }
                }
                a
            }
        };
        autorite.avec_etat_persistant(&dossier.join(DOSSIER_ETAT_DEFAUT))
    }

    /// Charge (ou crée, `0600`) la clé d'émission et la graine des titulaires
    /// (mode de transition) ; défis et nonces persistés.
    pub fn charger(
        cle_emission: &Path,
        cog_id: Option<&str>,
        pairing: Arc<PairingManager>,
    ) -> Result<Self, String> {
        Self::charger_avec_chaine(cle_emission, cog_id, None, pairing)
    }

    fn charger_transition(
        cle_emission: &Path,
        cog_id: Option<&str>,
        pairing: Arc<PairingManager>,
    ) -> Result<Self, String> {
        let cle = miyauth::http::service::charger_ou_creer_cle(cle_emission)
            .map_err(|e| format!("clé d'émission ({}) : {e}", cle_emission.display()))?;
        let chemin_titulaires = cle_emission
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(FICHIER_TITULAIRES_DEFAUT);
        let titulaires = miyauth::http::service::charger_ou_creer_cle(&chemin_titulaires)
            .map_err(|e| format!("secrets de titulaires ({}) : {e}", chemin_titulaires.display()))?;
        Self::new(cle, cog_id, titulaires.graine(), pairing)
    }

    /// `cog_id` du COG.
    pub fn cog(&self) -> &CogId {
        &self.cog
    }

    /// Clé publique d'émission (à épingler par les COG qui reconnaissent celui-ci).
    pub fn cle_publique_emission(&self) -> [u8; 32] {
        self.cle.cle_publique()
    }

    fn politique(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite { cog_local: self.cog.clone(), annuaire: &self.annuaire, revocations: self }
    }

    fn secret_titulaire(&self, profile_id: Option<&str>) -> Result<SecretTitulaire, RefusIdentite> {
        let hk = Hkdf::<Sha256>::new(Some(SEL_TITULAIRE), self.titulaires.as_slice());
        let info = match profile_id {
            Some(p) => format!("profil:{p}"),
            None => "proprietaire".to_string(),
        };
        let mut sortie = [0u8; 32];
        // 32 o ≤ 255 × 32 o : l'expansion HKDF ne peut pas échouer ; refus par
        // principe (fail-closed) si la bibliothèque l'indiquait quand même.
        hk.expand(info.as_bytes(), &mut sortie)
            .map_err(|_| RefusIdentite::ParametreInvalide("hkdf"))?;
        let s = SecretTitulaire::depuis_octets(sortie);
        zeroize::Zeroize::zeroize(&mut sortie);
        Ok(s)
    }

    /// Émet un défi de possession (audience [`AUDIENCE`], usage unique, 60 s).
    pub fn emettre_defi(&self, maintenant: u64) -> Result<DefiEmis, RefusHttp> {
        self.defis.emettre(AUDIENCE, maintenant)
    }

    /// Contrôle précoce d'une demande de passeport (avant lecture du corps) :
    /// signataire = clé d'un appareil appairé non révoqué, horodatage dans la fenêtre.
    pub fn controler_demande_passeport(
        &self,
        entetes: &EntetesSignees,
        maintenant: u64,
    ) -> Result<PairedDevice, RefusHttp> {
        let appareil = self
            .pairing
            .device_for_identity_key(&hex::encode(entetes.signataire))
            .ok_or(RefusHttp::SignataireNonAutorise)?;
        if self.pairing.is_key_revoked(&appareil.e2e_identity_key) {
            return Err(RefusHttp::SignataireNonAutorise);
        }
        entetes.controler_horodatage(maintenant, self.nonces.fenetre())?;
        Ok(appareil)
    }

    /// Délivre un passeport à l'appareil signataire d'une requête signée
    /// (usage [`USAGE_PASSEPORT`], `POST` [`PASSEPORT_ENDPOINT`]). Rend le
    /// passeport scellé et sa fin de validité.
    pub fn delivrer_passeport(
        &self,
        entetes: &EntetesSignees,
        corps: &[u8],
        maintenant: u64,
    ) -> Result<(Vec<u8>, u64), RefusHttp> {
        let appareil = self.controler_demande_passeport(entetes, maintenant)?;
        verifier_requete_signee(
            entetes,
            USAGE_PASSEPORT,
            "POST",
            PASSEPORT_ENDPOINT,
            corps,
            &self.nonces,
            maintenant,
        )?;
        let pas_apres = maintenant.saturating_add(DUREE_PASSEPORT_APPAREIL_SECS);
        let secret = self.secret_titulaire(appareil.profile_id.as_deref())?;
        let portees = Portees::new([PORTEE_PILOTAGE])
            .ok_or(RefusIdentite::ParametreInvalide("portees"))?;
        let passeport = emettre_passeport(
            &self.cog,
            &self.cle,
            &DemandePasseport {
                secret: &secret,
                destination: self.cog.clone(),
                cle_appareil: entetes.signataire,
                portees,
                pas_avant: maintenant,
                pas_apres,
            },
        )?;
        Ok((passeport, pas_apres))
    }

    fn appareil_de(&self, cle: &[u8; 32]) -> Result<PairedDevice, RefusHttp> {
        // Doublé par la source de révocation ; garde un résultat cohérent si
        // l'appareil est retiré entre la vérification et la lecture.
        self.pairing
            .device_for_identity_key(&hex::encode(cle))
            .ok_or(RefusHttp::Identite(RefusIdentite::Revoque(CibleRevocation::appareil(cle))))
    }

    /// Échange `MiyuPasseport` + `X-Miyu-PoP` contre un visa (audience
    /// [`AUDIENCE`], ≤ 15 min, dans la fenêtre du passeport).
    pub fn delivrer_visa(&self, requete: &RequeteHttp<'_>, maintenant: u64) -> Result<VisaDelivre, RefusHttp> {
        self.delivrer_visa_avec_delegations(requete, None, maintenant)
    }

    /// Comme [`Self::delivrer_visa`], avec les délégations présentées par
    /// l'appareil (`X-Miyu-Delegations`). En mode chaîné, le visa est scellé
    /// par la clé `visas` et ne dépasse pas la fin de sa délégation.
    pub fn delivrer_visa_avec_delegations(
        &self,
        requete: &RequeteHttp<'_>,
        delegations: Option<&str>,
        maintenant: u64,
    ) -> Result<VisaDelivre, RefusHttp> {
        let possession = verifier_requete_passeport_avec_delegations(
            requete,
            delegations,
            AUDIENCE,
            &self.politique(),
            self.defis.as_ref(),
            maintenant,
        )?;
        let p = possession.passeport().passeport();
        let pas_avant = maintenant.max(p.pas_avant);
        let pas_apres = maintenant.saturating_add(DUREE_MAX_VISA_SECS).min(p.pas_apres).min(self.fin_visas);
        if pas_apres <= pas_avant {
            return Err(RefusIdentite::DelegationHorsFenetre.into());
        }
        let visa = emettre_visa(
            &self.cog,
            &self.cle_visas,
            &possession,
            &DemandeVisa {
                audience: AUDIENCE.to_string(),
                portees: p.portees.clone(),
                pas_avant,
                pas_apres,
            },
        )?;
        let appareil = self.appareil_de(&p.cle_appareil)?;
        Ok(VisaDelivre {
            visa,
            acces: AccesAppareil {
                identite: identite_opaque(&self.cog, &p.sujet),
                role: IdentityRole::Citizen,
                expire_a: pas_apres,
                cle_appareil: p.cle_appareil,
                appareil,
            },
        })
    }

    /// Vérifie `MiyuVisa` + `X-Miyu-PoP` (défi neuf, consommé) pour [`AUDIENCE`].
    pub fn verifier_acces(&self, requete: &RequeteHttp<'_>, maintenant: u64) -> Result<AccesAppareil, RefusHttp> {
        self.verifier_acces_avec_delegations(requete, None, maintenant)
    }

    /// Comme [`Self::verifier_acces`], avec les délégations présentées
    /// (`X-Miyu-Delegations`) : vérifiées contre la racine épinglée.
    pub fn verifier_acces_avec_delegations(
        &self,
        requete: &RequeteHttp<'_>,
        delegations: Option<&str>,
        maintenant: u64,
    ) -> Result<AccesAppareil, RefusHttp> {
        let prouve = verifier_requete_visa_avec_delegations(
            requete,
            delegations,
            AUDIENCE,
            &self.politique(),
            self.defis.as_ref(),
            maintenant,
        )?;
        let visa = prouve.acces().visa();
        let appareil = self.appareil_de(&visa.cle_appareil)?;
        Ok(AccesAppareil {
            identite: identite_opaque(prouve.cog_origine(), prouve.sujet()),
            role: prouve.role(),
            expire_a: visa.pas_apres,
            cle_appareil: visa.cle_appareil,
            appareil,
        })
    }

    /// Vrai si l'accès est encore valable à `maintenant` : visa non expiré,
    /// appareil toujours appairé et non révoqué (revérification des sessions WS).
    pub fn acces_toujours_valide(&self, acces: &AccesAppareil, maintenant: u64) -> bool {
        maintenant < acces.expire_a
            && self.est_revoque(&self.cog, &CibleRevocation::appareil(&acces.cle_appareil), maintenant)
                == Ok(false)
    }
}

impl SourceRevocation for AutoriteAppareils {
    fn est_revoque(
        &self,
        emetteur: &CogId,
        cible: &CibleRevocation,
        maintenant_secs: u64,
    ) -> Result<bool, ErreurRevocation> {
        if self.pairing.est_revoque(emetteur, cible, maintenant_secs)? {
            return Ok(true);
        }
        // Révocation durable : une clé (autre que la clé d'émission) doit être
        // celle d'un appareil encore appairé.
        if let CibleRevocation::Cle(kid) = cible {
            if *kid != self.cle.kid()
                && *kid != self.cle_visas.kid()
                && self.pairing.device_for_key_id(kid).is_none()
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyauth::http::{
        formater_autorisation_passeport, formater_autorisation_visa, formater_pop, liaison_requete,
        signer_pop, signer_requete,
    };
    use miyukini_cog_bridge::{
        parse_commitment_hex, parse_pairing_signature_hex, DevicePairing, E2eIdentity, PairingNonce,
    };

    fn maintenant() -> u64 {
        miyauth::http::service::maintenant_unix()
    }

    /// Appaire `device` (engagement, révélation, approbation) sur `mgr`.
    fn appairer(mgr: &PairingManager, device: &E2eIdentity, id: &str) {
        let cog_key = E2eIdentity::from_secret_bytes(&[7; 32]).public_key_bytes();
        let key = device.public_key_bytes();
        let pairing = DevicePairing::start(key);
        let c = parse_commitment_hex(&pairing.commitment_hex()).unwrap();
        let sig = parse_pairing_signature_hex(&pairing.commit_signature_hex(device, id).unwrap()).unwrap();
        let src = "10.0.0.2".parse().unwrap();
        let nc = mgr.request_pairing(id, "Pixel", &hex::encode(key), c, &sig, src).unwrap();
        let revealed = pairing.receive_cog_nonce(cog_key, nc).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let rsig = parse_pairing_signature_hex(&revealed.reveal_signature_hex(device, id).unwrap()).unwrap();
        let code = mgr.reveal_pairing(id, &nd, &rsig, src, &cog_key).unwrap();
        mgr.approve_device_for(id, &code, Some("prof-1")).unwrap();
    }

    fn monde() -> (AutoriteAppareils, Arc<PairingManager>, E2eIdentity, CleSignature) {
        let mgr = Arc::new(PairingManager::new());
        let device = E2eIdentity::generate();
        appairer(&mgr, &device, "device-identite-01");
        let autorite = AutoriteAppareils::new(CleSignature::generer(), None, [3; 32], mgr.clone()).unwrap();
        let cle = CleSignature::depuis_graine(&device.secret_bytes());
        (autorite, mgr, device, cle)
    }

    fn passeport(a: &AutoriteAppareils, cle: &CleSignature) -> Vec<u8> {
        let e = signer_requete(cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        a.delivrer_passeport(&e, b"", maintenant()).unwrap().0
    }

    fn pop(a: &AutoriteAppareils, cle: &CleSignature, methode: &str, chemin: &str) -> String {
        let d = a.emettre_defi(maintenant()).unwrap();
        let sig = signer_pop(cle, &d.defi, AUDIENCE, &liaison_requete(methode, chemin, b""));
        formater_pop(&d.id, &sig)
    }

    fn visa(a: &AutoriteAppareils, cle: &CleSignature, p: &[u8]) -> VisaDelivre {
        let auth = formater_autorisation_passeport(p);
        let pop = pop(a, cle, "POST", "/api/bridge/visa");
        let r = RequeteHttp { methode: "POST", chemin: "/api/bridge/visa", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
        a.delivrer_visa(&r, maintenant()).unwrap()
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn passeport_visa_acces_puis_revocation() {
        let (a, mgr, _device, cle) = monde();
        let p = passeport(&a, &cle);
        let v = visa(&a, &cle, &p);
        assert!(v.acces.expire_a <= maintenant() + DUREE_MAX_VISA_SECS);
        assert_eq!(v.acces.appareil.profile_id.as_deref(), Some("prof-1"));
        let auth = formater_autorisation_visa(&v.visa, &p);
        let pop1 = pop(&a, &cle, "GET", "/ws");
        let r = RequeteHttp { methode: "GET", chemin: "/ws", corps: b"", autorisation: Some(&auth), pop: Some(&pop1), liaison: None };
        let acces = a.verifier_acces(&r, maintenant()).unwrap();
        assert_eq!(acces.identite, v.acces.identite);
        assert!(a.acces_toujours_valide(&acces, maintenant()));
        // Rejeu de la même preuve : défi consommé.
        assert!(matches!(a.verifier_acces(&r, maintenant()), Err(RefusHttp::DefiInvalide)));
        // Preuve liée à une autre requête : refus.
        let pop2 = pop(&a, &cle, "GET", "/autre");
        let r2 = RequeteHttp { pop: Some(&pop2), ..r };
        assert!(a.verifier_acces(&r2, maintenant()).is_err());

        // Révocation : la session et tout nouveau titre sont refusés.
        assert!(mgr.revoke_device("device-identite-01").unwrap());
        assert!(!a.acces_toujours_valide(&acces, maintenant()));
        let pop3 = pop(&a, &cle, "GET", "/ws");
        let r3 = RequeteHttp { pop: Some(&pop3), ..r };
        assert!(matches!(
            a.verifier_acces(&r3, maintenant()),
            Err(RefusHttp::Identite(RefusIdentite::Revoque(_)))
        ));
        let e = signer_requete(&cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        assert_eq!(a.delivrer_passeport(&e, b"", maintenant()).unwrap_err(), RefusHttp::SignataireNonAutorise);
    }

    #[test]
    fn passeport_refuse_hors_liste_blanche_et_rejeu() {
        let (a, _mgr, _device, cle) = monde();
        let inconnu = CleSignature::generer();
        let e = signer_requete(&inconnu, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        assert_eq!(a.delivrer_passeport(&e, b"", maintenant()).unwrap_err(), RefusHttp::SignataireNonAutorise);
        let e = signer_requete(&cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        assert!(a.delivrer_passeport(&e, b"", maintenant()).is_ok());
        assert_eq!(a.delivrer_passeport(&e, b"", maintenant()).unwrap_err(), RefusHttp::Rejeu);
        // Autre usage : signature invalide.
        let e = signer_requete(&cle, "autre/usage", "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        assert_eq!(a.delivrer_passeport(&e, b"", maintenant()).unwrap_err(), RefusHttp::SignatureInvalide);
    }

    #[test]
    fn titulaire_stable_par_profil() {
        let (a, _mgr, _d, cle) = monde();
        let s1 = visa(&a, &cle, &passeport(&a, &cle)).acces.identite;
        let s2 = visa(&a, &cle, &passeport(&a, &cle)).acces.identite;
        assert_eq!(s1, s2);
        assert_ne!(
            a.secret_titulaire(Some("prof-1")).unwrap().octets(),
            a.secret_titulaire(None).unwrap().octets()
        );
    }

    #[test]
    fn cle_emission_creee_0600_et_cog_id_configurable() {
        let dir = std::env::temp_dir().join(format!("cr-identite-{}", uuid::Uuid::new_v4()));
        let chemin = dir.join(FICHIER_CLE_EMISSION_DEFAUT);
        let a1 = AutoriteAppareils::charger(&chemin, None, Arc::new(PairingManager::new())).unwrap();
        let a2 = AutoriteAppareils::charger(&chemin, None, Arc::new(PairingManager::new())).unwrap();
        assert_eq!(a1.cle_publique_emission(), a2.cle_publique_emission());
        assert_eq!(a1.cog(), a2.cog());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for f in [&chemin, &dir.join(FICHIER_TITULAIRES_DEFAUT)] {
                assert_eq!(std::fs::metadata(f).unwrap().permissions().mode() & 0o777, 0o600);
            }
            // Droits ouverts : refus de démarrer.
            std::fs::set_permissions(&chemin, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(AutoriteAppareils::charger(&chemin, None, Arc::new(PairingManager::new())).is_err());
            std::fs::set_permissions(&chemin, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let racine = "abcdefghijklmnopqrstuvwxyz";
        let a3 = AutoriteAppareils::charger(&chemin, Some(racine), Arc::new(PairingManager::new())).unwrap();
        assert_eq!(a3.cog().as_str(), racine);
        assert!(AutoriteAppareils::charger(&chemin, Some("invalide"), Arc::new(PairingManager::new())).is_err());
    }

    /// Mode chaîné : racine connue, délégations `emission` et `visas`, strict.
    fn monde_chaine(fin_visas: u64) -> (AutoriteAppareils, CleSignature, miyuracine::Racine) {
        use miyuracine::{Racine, Usage};
        let mgr = Arc::new(PairingManager::new());
        let device = E2eIdentity::generate();
        appairer(&mgr, &device, "device-identite-02");
        let racine = Racine::depuis_secret(&[0x77; 32]);
        let t = maintenant();
        let chaine = ChaineCog {
            racine: racine.cle_publique_identite(),
            delegation_emission: racine.deleguer(Usage::Emission, 0, t - 60, t + 30 * 86_400).unwrap(),
            cle_visas: racine.cle(Usage::Visas, 0).unwrap(),
            delegation_visas: racine.deleguer(Usage::Visas, 0, t - 60, fin_visas).unwrap(),
        };
        let a = AutoriteAppareils::new_chaine(racine.cle(Usage::Emission, 0).unwrap(), chaine, [3; 32], mgr, t).unwrap();
        (a, CleSignature::depuis_graine(&device.secret_bytes()), racine)
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn mode_chaine_visas_par_l_indice_5_et_strict() {
        let (a, cle, racine) = monde_chaine(maintenant() + 300);
        assert!(a.mode_strict());
        assert_eq!(a.cog().as_str(), miyuracine::cog_id_depuis_cle(&racine.cle_publique_identite()));
        assert_eq!(a.delegations_visa().len(), 2, "délégations `visas` et `emission` rendues avec le visa");
        let p = passeport(&a, &cle);
        let v = visa(&a, &cle, &p);
        assert!(v.acces.expire_a <= maintenant() + 300, "jamais au-delà de la délégation `visas`");
        // Accès accepté, délégations présentées ou non (déjà connues de l'annuaire).
        let auth = formater_autorisation_visa(&v.visa, &p);
        let deleg: Vec<&[u8]> = a.delegations_visa().iter().map(Vec::as_slice).collect();
        let entete = miyauth::http::formater_delegations(&deleg);
        let pop1 = pop(&a, &cle, "GET", "/ws");
        let r = RequeteHttp { methode: "GET", chemin: "/ws", corps: b"", autorisation: Some(&auth), pop: Some(&pop1), liaison: None };
        assert!(a.verifier_acces_avec_delegations(&r, Some(&entete), maintenant()).is_ok());
        // Délégation présentée d'une autre racine : refus.
        let autre = miyuracine::Racine::depuis_secret(&[0x78; 32]);
        let t = maintenant();
        let etrangere = autre.deleguer(miyuracine::Usage::Visas, 0, t - 60, t + 600).unwrap();
        let pop2 = pop(&a, &cle, "GET", "/ws");
        let r2 = RequeteHttp { pop: Some(&pop2), ..r };
        let e2 = miyauth::http::formater_delegations(&[&etrangere]);
        assert!(a.verifier_acces_avec_delegations(&r2, Some(&e2), maintenant()).is_err());
        // Visa scellé par la clé `emission` (indice 1) : refusé (strict, usage incorrect).
        let possession = {
            let auth = formater_autorisation_passeport(&p);
            let pop = pop(&a, &cle, "POST", "/x");
            let r = RequeteHttp { methode: "POST", chemin: "/x", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
            miyauth::http::verifier_requete_passeport(&r, AUDIENCE, &a.politique(), a.defis.as_ref(), maintenant()).unwrap()
        };
        let faux = emettre_visa(
            &a.cog,
            &racine.cle(miyuracine::Usage::Emission, 0).unwrap(),
            &possession,
            &DemandeVisa {
                audience: AUDIENCE.into(),
                portees: Portees::new([PORTEE_PILOTAGE]).unwrap(),
                pas_avant: maintenant(),
                pas_apres: maintenant() + 60,
            },
        )
        .unwrap();
        let auth = formater_autorisation_visa(&faux, &p);
        let pop3 = pop(&a, &cle, "GET", "/ws");
        let r3 = RequeteHttp { methode: "GET", chemin: "/ws", corps: b"", autorisation: Some(&auth), pop: Some(&pop3), liaison: None };
        assert!(matches!(
            a.verifier_acces(&r3, maintenant()),
            Err(RefusHttp::Identite(RefusIdentite::UsageIncorrect { attendu: 5, delegue: 1 }))
        ));
    }

    #[test]
    fn delegation_visas_expiree_refuse_de_demarrer() {
        use miyuracine::{Racine, Usage};
        let racine = Racine::depuis_secret(&[0x79; 32]);
        let t = maintenant();
        let chaine = ChaineCog {
            racine: racine.cle_publique_identite(),
            delegation_emission: racine.deleguer(Usage::Emission, 0, t - 60, t + 86_400).unwrap(),
            cle_visas: racine.cle(Usage::Visas, 0).unwrap(),
            delegation_visas: racine.deleguer(Usage::Visas, 0, t - 86_400, t - 600).unwrap(),
        };
        let mgr = Arc::new(PairingManager::new());
        assert!(AutoriteAppareils::new_chaine(racine.cle(Usage::Emission, 0).unwrap(), chaine, [3; 32], mgr, t).is_err());
    }

    #[test]
    fn nonces_et_defis_persistants_apres_redemarrage() {
        let dir = std::env::temp_dir().join(format!("cr-etat-{}", uuid::Uuid::new_v4()));
        let mgr = Arc::new(PairingManager::new());
        let device = E2eIdentity::generate();
        appairer(&mgr, &device, "device-identite-03");
        let cle = CleSignature::depuis_graine(&device.secret_bytes());
        let emission = CleSignature::generer();
        let a = AutoriteAppareils::new(CleSignature::depuis_graine(&emission.graine()), None, [3; 32], mgr.clone())
            .unwrap()
            .avec_etat_persistant(&dir)
            .unwrap();
        let e = signer_requete(&cle, USAGE_PASSEPORT, "POST", PASSEPORT_ENDPOINT, b"", maintenant());
        assert!(a.delivrer_passeport(&e, b"", maintenant()).is_ok());
        let d = a.emettre_defi(maintenant()).unwrap();
        drop(a);
        // Redémarrage : même dossier d'état.
        let b = AutoriteAppareils::new(emission, None, [3; 32], mgr).unwrap().avec_etat_persistant(&dir).unwrap();
        assert_eq!(b.delivrer_passeport(&e, b"", maintenant()).unwrap_err(), RefusHttp::Rejeu, "nonce rejoué");
        // Le défi émis avant le redémarrage reste consommable une fois.
        assert!(b.defis.consommer(&d.id, AUDIENCE, maintenant()).is_ok());
        assert!(b.defis.consommer(&d.id, AUDIENCE, maintenant()).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

}
