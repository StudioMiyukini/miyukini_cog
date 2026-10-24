//! @id     origin_admin_acces
//! @do     authentifier_admin_par_passeport
//! @role   securite
//! @layer  app
//! @human  origin — connexion admin : passeport + preuve de possession → visa `origin-admin`, liste d'administrateurs.

//! Connexion au panneau admin d'Origin sans mot de passe (inventaire des secrets partagés
//! #15, sortie du lot 2).
//!
//! Origin agit comme COG hôte avec la même clé que le forum (`[forum] cle_emission_path`)
//! et les mêmes COG reconnus (`[[forum.cogs_reconnus]]`), mais avec sa propre audience
//! (`origin-admin`) et son propre magasin de défis.
//!
//! 1. `POST /admin/defi` → `{defi_id, defi, audience: "origin-admin", expire_a, cog_id}`.
//! 2. Le COG de l'administrateur émet un passeport pour la destination Origin portant la
//!    portée `origin-admin` (durée ≤ 15 min) ; l'appareil signe la preuve de possession
//!    liée à `POST /admin/login/passeport` (méthode, chemin, SHA-256 du corps).
//! 3. `POST /admin/login/passeport` avec `Authorization: MiyuPasseport …` et
//!    `X-Miyu-PoP: …` → visa `origin-admin` (≤ 15 min) si l'identité par paire
//!    `pw:<cog>:<sujet>` figure dans `[admin] administrateurs`.
//!
//! Choix fail-closed :
//! - liste d'administrateurs vide ou clé d'hôte absente : connexion par passeport refusée ;
//! - entrée de liste invalide : toute la connexion par passeport est désactivée ;
//! - portée `origin-admin` exigée dans le passeport (un passeport « forum » ne suffit pas) ;
//! - passeport de plus de 15 min refusé (aucune liste de révocation distribuée à Origin).

use std::collections::HashSet;

use miyauth::http::{verifier_requete_passeport, DefiEmis, MagasinDefis, RefusHttp, RequeteHttp};
use miyauth::titres::DUREE_MAX_VISA_SECS;
use miyauth::{emettre_visa, CogId, DemandeVisa, Portees};

use crate::config::OriginConfig;
use crate::web::forum_auth::{
    identite_opaque, HoteForum, ParametresForum, DUREE_MAX_PASSEPORT_FORUM_SECS,
};

/// Audience des visas admin (et des défis de possession associés).
pub const AUDIENCE_ADMIN: &str = "origin-admin";
/// Portée exigée dans le passeport et recopiée dans le visa.
pub const PORTEE_ADMIN: &str = "origin-admin";
/// Durée maximale d'un passeport accepté pour l'admin (révocation non vérifiable au-delà).
pub const DUREE_MAX_PASSEPORT_ADMIN_SECS: u64 = DUREE_MAX_PASSEPORT_FORUM_SECS;

/// Refus de connexion admin par passeport.
#[derive(Debug)]
pub enum RefusAdmin {
    /// Vérification passeport / preuve de possession en échec.
    Http(RefusHttp),
    /// Passeport de plus de 15 min.
    PasseportTropLong,
    /// Portée `origin-admin` absente du passeport.
    PorteeAbsente,
    /// Identité valide mais absente de `[admin] administrateurs`.
    NonAdministrateur(String),
    /// Fenêtre de visa vide ou émission refusée.
    Emission(String),
}

impl std::fmt::Display for RefusAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(e) => write!(f, "{e}"),
            Self::PasseportTropLong => f.write_str("passeport de plus de 15 min"),
            Self::PorteeAbsente => f.write_str("portée origin-admin absente du passeport"),
            Self::NonAdministrateur(id) => write!(f, "identité non administratrice : {id}"),
            Self::Emission(e) => write!(f, "émission du visa refusée : {e}"),
        }
    }
}

/// Connexion admin réussie.
#[derive(Debug)]
pub struct ConnexionAdmin {
    /// Visa `origin-admin` scellé par Origin.
    pub visa: Vec<u8>,
    /// Expiration du visa (secondes Unix) : la session admin expire avec lui.
    pub expire_a: u64,
    /// Identité par paire `pw:<cog>:<sujet hex>`.
    pub identite: String,
}

/// Analyse une entrée `pw:<cog_id>:<sujet hex 32 o>`.
pub fn analyser_administrateur(texte: &str) -> Result<(CogId, [u8; 32]), String> {
    let texte = texte.trim();
    let reste = texte
        .strip_prefix("pw:")
        .ok_or_else(|| format!("admin.administrateurs : « {texte} » ne commence pas par pw:"))?;
    let (cog, sujet) = reste.split_once(':').ok_or_else(|| {
        format!("admin.administrateurs : « {texte} » : pw:<cog_id>:<sujet hex> attendu")
    })?;
    let cog = CogId::new(cog)
        .ok_or_else(|| format!("admin.administrateurs : cog_id invalide « {cog} »"))?;
    let sujet: [u8; 32] = hex::decode(sujet)
        .ok()
        .and_then(|v| v.try_into().ok())
        .ok_or_else(|| {
            format!(
                "admin.administrateurs : sujet hexadécimal de 32 octets attendu dans « {texte} »"
            )
        })?;
    Ok((cog, sujet))
}

/// Origin en tant que COG hôte de l'admin + liste des administrateurs.
pub struct AccesAdmin {
    hote: HoteForum,
    administrateurs: HashSet<(CogId, [u8; 32])>,
}

impl AccesAdmin {
    /// Construit l'accès depuis la configuration. `Ok(None)` : clé d'hôte absente ou liste
    /// d'administrateurs vide (connexion par passeport désactivée). Toute entrée invalide
    /// est une erreur (fail-closed).
    pub fn depuis_config(config: &OriginConfig) -> Result<Option<Self>, String> {
        if config.admin.administrateurs.is_empty() {
            return Ok(None);
        }
        let administrateurs = config
            .admin
            .administrateurs
            .iter()
            .map(|a| analyser_administrateur(a))
            .collect::<Result<HashSet<_>, _>>()?;
        let parametres = ParametresForum::depuis_config(&config.forum)?;
        let Some(cle) = parametres.cle_hote else {
            return Ok(None);
        };
        Ok(Some(Self::new(
            HoteForum::new(cle, &parametres.cogs_reconnus)?,
            administrateurs,
        )))
    }

    pub(crate) fn new(hote: HoteForum, administrateurs: HashSet<(CogId, [u8; 32])>) -> Self {
        Self {
            hote,
            administrateurs,
        }
    }

    /// `cog_id` d'Origin (destination des passeports admin).
    pub fn cog(&self) -> &CogId {
        &self.hote.cog
    }

    /// Nombre d'administrateurs autorisés.
    pub fn nb_administrateurs(&self) -> usize {
        self.administrateurs.len()
    }

    /// Émet un défi de possession (audience `origin-admin`, usage unique, 60 s).
    pub fn emettre_defi(&self, maintenant: u64) -> Result<DefiEmis, RefusHttp> {
        self.hote.defis.emettre(AUDIENCE_ADMIN, maintenant)
    }

    /// Vérifie passeport + preuve de possession, la portée, la durée et la liste des
    /// administrateurs, puis émet le visa `origin-admin` (≤ 15 min, borné par le passeport).
    pub fn connecter(
        &self,
        requete: &RequeteHttp<'_>,
        maintenant: u64,
    ) -> Result<ConnexionAdmin, RefusAdmin> {
        let possession = verifier_requete_passeport(
            requete,
            AUDIENCE_ADMIN,
            &self.hote.politique(),
            &self.hote.defis,
            maintenant,
        )
        .map_err(RefusAdmin::Http)?;
        let p = possession.passeport().passeport();
        if p.pas_apres.saturating_sub(p.pas_avant) > DUREE_MAX_PASSEPORT_ADMIN_SECS {
            return Err(RefusAdmin::PasseportTropLong);
        }
        if !p.portees.contient(PORTEE_ADMIN) {
            return Err(RefusAdmin::PorteeAbsente);
        }
        let identite = identite_opaque(&p.emetteur_cog, &p.sujet);
        if !self
            .administrateurs
            .contains(&(p.emetteur_cog.clone(), p.sujet))
        {
            return Err(RefusAdmin::NonAdministrateur(identite));
        }
        let pas_avant = maintenant.max(p.pas_avant);
        let pas_apres = maintenant
            .saturating_add(DUREE_MAX_VISA_SECS)
            .min(p.pas_apres);
        if pas_apres <= pas_avant {
            return Err(RefusAdmin::Emission("fenêtre vide".into()));
        }
        let portees =
            Portees::new([PORTEE_ADMIN]).ok_or_else(|| RefusAdmin::Emission("portée".into()))?;
        let demande = DemandeVisa {
            audience: AUDIENCE_ADMIN.to_string(),
            portees,
            pas_avant,
            pas_apres,
        };
        let visa = emettre_visa(&self.hote.cog, &self.hote.cle, &possession, &demande)
            .map_err(|e| RefusAdmin::Emission(e.to_string()))?;
        Ok(ConnexionAdmin {
            visa,
            expire_a: pas_apres,
            identite,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn administrateur_analyse_stricte() {
        let cog = "a".repeat(26);
        let sujet = "0f".repeat(32);
        let (c, s) = analyser_administrateur(&format!("pw:{cog}:{sujet}")).unwrap();
        assert_eq!(c.as_str(), cog);
        assert_eq!(s, [0x0f; 32]);
        for faux in [
            format!("{cog}:{sujet}"),
            format!("pw:{cog}"),
            format!("pw:{cog}:{}", "0f".repeat(31)),
            format!("pw:pas-un-cog:{sujet}"),
            format!("pw:{cog}:{}", "zz".repeat(32)),
        ] {
            assert!(analyser_administrateur(&faux).is_err(), "{faux}");
        }
    }

    #[test]
    fn liste_vide_ou_entree_invalide() {
        let mut c = OriginConfig::default();
        assert!(AccesAdmin::depuis_config(&c).unwrap().is_none());
        c.admin.administrateurs = vec!["pw:invalide".into()];
        assert!(AccesAdmin::depuis_config(&c).is_err());
        // Liste valide mais clé d'hôte absente : désactivé.
        c.admin.administrateurs = vec![format!("pw:{}:{}", "a".repeat(26), "00".repeat(32))];
        assert!(AccesAdmin::depuis_config(&c).unwrap().is_none());
    }
}
