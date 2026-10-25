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
    verifier_requete_passeport, verifier_requete_signee, verifier_requete_visa, DefiEmis,
    EntetesSignees, MagasinDefis, MagasinDefisMemoire, RefusHttp, RegistreNonces, RequeteHttp,
};
use miyauth::titres::DUREE_MAX_VISA_SECS;
use miyauth::{
    emettre_passeport, emettre_visa, AnnuaireEmetteurs, CibleRevocation, CogId, DemandePasseport,
    DemandeVisa, ErreurRevocation, IdentityRole, PolitiqueIdentite, Portees, RefusIdentite,
    SecretTitulaire, SourceRevocation,
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
    annuaire: AnnuaireEmetteurs,
    titulaires: Zeroizing<[u8; 32]>,
    pairing: Arc<PairingManager>,
    defis: MagasinDefisMemoire,
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
            cle,
            annuaire,
            titulaires: Zeroizing::new(graine_titulaires),
            pairing,
            defis: MagasinDefisMemoire::default(),
            nonces: RegistreNonces::default(),
        })
    }

    /// Charge (ou crée, `0600`) la clé d'émission et la graine des titulaires.
    pub fn charger(
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
        let possession =
            verifier_requete_passeport(requete, AUDIENCE, &self.politique(), &self.defis, maintenant)?;
        let p = possession.passeport().passeport();
        let pas_avant = maintenant.max(p.pas_avant);
        let pas_apres = maintenant.saturating_add(DUREE_MAX_VISA_SECS).min(p.pas_apres);
        let visa = emettre_visa(
            &self.cog,
            &self.cle,
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
        let prouve = verifier_requete_visa(requete, AUDIENCE, &self.politique(), &self.defis, maintenant)?;
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
            if *kid != self.cle.kid() && self.pairing.device_for_key_id(kid).is_none() {
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
}
