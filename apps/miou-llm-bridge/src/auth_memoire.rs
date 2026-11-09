//! @id     maia_auth_memoire
//! @do     authentifier_utilisateur_memoire_par_visa
//! @role   security
//! @layer  app
//! @human  MAIA — authentification des routes mémoire par passeport et visa (I4) : visa d'indice 5
//! @human  délégué, audience « maia », portée par route, preuve de possession liée à la requête,
//! @human  défis persistants, titulaire du passeport = utilisateur de l'instance.

//! # Authentification par visa des routes mémoire (I4, A-024, A-029)
//!
//! ## En-têtes (lot 2 §5 à §7, A-029)
//!
//! ```text
//! Authorization:      MiyuVisa <b64url(visa)>.<b64url(passeport)>
//! X-Miyu-PoP:         <b64url(id du défi)>.<b64url(signature Ed25519)>
//! X-Miyu-Delegations: <b64url(0x45 visas)>[.<b64url(0x45 emission)>]
//! ```
//!
//! Le défi est tiré par `POST /v1/memoire/defi` (corps vide, aucune
//! authentification, débit limité) : `{"id","defi","audience","expire_a"}`
//! (base64 URL sans remplissage), à usage unique, 60 s, **persistant**
//! ([`miyauth::http::MagasinDefisFichier`]) : consommé avant toute
//! vérification, un défi ne sert qu'une fois, y compris après redémarrage.
//!
//! Preuve de possession : `Ed25519(clé d'appareil, "miyukini/pop/v1" ‖ défi ‖
//! "maia" ‖ liaison_requete(méthode, chemin, corps))` ; le chemin est le
//! chemin **sans** chaîne de requête (une chaîne de requête ⇒ 400).
//!
//! ## Contrôles (dans l'ordre, premier refus = réponse)
//!
//! 1. défi consommé (inconnu, expiré, rejoué ⇒ 401) ;
//! 2. délégations présentées vérifiées contre la **racine épinglée**
//!    (`MAIA_MEMOIRE_RACINE`), **mode strict** : sans délégation `visas`
//!    (indice 5) et `emission` (indice 1) valides, aucun titre n'est accepté ;
//! 3. passeport et visa (chaîne, fenêtres, audience [`AUDIENCE`], liaison
//!    visa ↔ passeport, révocations ≤ 15 min, voir
//!    [`crate::revocations_memoire`]) ; preuve de possession ;
//! 4. rôle `Citizen`, COG d'origine = COG local, **sujet du passeport =
//!    titulaire de l'instance** (`MAIA_MEMOIRE_TITULAIRE`, forme
//!    `pw:<cog_id>:<hex(sujet)>`) ⇒ sinon 403 `autre_utilisateur` ;
//! 5. portée de la route dans le visa ([`Droit`]) ⇒ sinon 403
//!    `portee_insuffisante`.
//!
//! | Portée | Routes |
//! |---|---|
//! | [`PORTEE_LIRE`] `maia:memoire` | consulter, historique, exporter |
//! | [`PORTEE_ECRIRE`] `maia:memoire:ecrire` | corriger, oublier, règles, valider / rejeter un candidat, refuser une approbation |
//! | [`PORTEE_APPROUVER`] `maia:tamr:approuver` | approuver (TAMR) ; la clé d'appareil doit en outre être épinglée dans `MAIA_TAMR_CLES_APPROBATION` (F7) |
//!
//! ## Configuration (tout ou rien ; incomplète ou invalide ⇒ visas désactivés)
//!
//! | Variable | Contenu |
//! |---|---|
//! | `MAIA_MEMOIRE_RACINE` | clé publique d'identité (indice 0) du COG, 64 hex ou base64 URL |
//! | `MAIA_MEMOIRE_TITULAIRE` | `pw:<cog_id>:<64 hex>` : identifiant par paire de l'utilisateur de l'instance |
//! | `MAIA_MEMOIRE_ETAT_DIR` | dossier `0700` des défis (`defis/`), nonces de transition (`nonces/`) et copie du journal |
//! | `MAIA_MEMOIRE_REGISTRE` | identifiant du registre du COG (64 hex) |
//! | `MAIA_MEMOIRE_CLE_JOURNAL` | clé publique de journal du COG |
//! | `MAIA_MEMOIRE_REVOCATIONS_DEPOT` | dépôt des révocations (tête + entrées) |
//!
//! ## Transition (désactivée par défaut)
//!
//! L'ancienne authentification (requête signée `maia/memoire/<instance>/<u>`
//! par une clé épinglée dans `MAIA_MEMOIRE_CLES_UTILISATEUR`, approbations par
//! `MAIA_TAMR_CLES_APPROBATION`) n'est acceptée que si
//! `MAIA_MEMOIRE_TRANSITION=cles-epinglees` ([`VALEUR_TRANSITION`]) ; ses
//! nonces sont alors persistants dans `MAIA_MEMOIRE_ETAT_DIR/nonces` quand le
//! dossier est configuré.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use miyauth::http::{
    verifier_requete_visa_avec_delegations, DefiEmis, MagasinDefis, MagasinDefisFichier, RefusHttp,
    RequeteHttp,
};
use miyauth::{AnnuaireEmetteurs, CogId, IdentityRole, PolitiqueIdentite, SourceRevocation};
use miyusceau::NiveauConfiance;

use crate::revocations_memoire::RevocationsMemoire;

/// Audience des visas des routes mémoire (le pont MAIA).
pub const AUDIENCE: &str = "maia";
/// Portée de lecture de la mémoire.
pub const PORTEE_LIRE: &str = "maia:memoire";
/// Portée d'écriture (correction, oubli, règles, candidats).
pub const PORTEE_ECRIRE: &str = "maia:memoire:ecrire";
/// Portée d'approbation TAMR.
pub const PORTEE_APPROUVER: &str = "maia:tamr:approuver";

/// Racine du COG (clé d'identité, indice 0).
pub const ENV_RACINE: &str = "MAIA_MEMOIRE_RACINE";
/// Titulaire de l'instance (`pw:<cog_id>:<hex(sujet)>`).
pub const ENV_TITULAIRE: &str = "MAIA_MEMOIRE_TITULAIRE";
/// Dossier d'état (défis, nonces, copie du journal).
pub const ENV_ETAT: &str = "MAIA_MEMOIRE_ETAT_DIR";
/// Identifiant du registre du COG.
pub const ENV_REGISTRE: &str = "MAIA_MEMOIRE_REGISTRE";
/// Clé publique de journal du COG.
pub const ENV_CLE_JOURNAL: &str = "MAIA_MEMOIRE_CLE_JOURNAL";
/// Dépôt des révocations.
pub const ENV_DEPOT_REVOCATIONS: &str = "MAIA_MEMOIRE_REVOCATIONS_DEPOT";
/// Mode de transition (clés épinglées) : seule valeur acceptée [`VALEUR_TRANSITION`].
pub const ENV_TRANSITION: &str = "MAIA_MEMOIRE_TRANSITION";
/// Valeur qui active la transition.
pub const VALEUR_TRANSITION: &str = "cles-epinglees";

/// Droit exigé par une route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Droit {
    /// Lecture.
    Lire,
    /// Écriture.
    Ecrire,
    /// Approbation TAMR.
    Approuver,
}

impl Droit {
    /// Portée de visa correspondante.
    pub fn portee(self) -> &'static str {
        match self {
            Droit::Lire => PORTEE_LIRE,
            Droit::Ecrire => PORTEE_ECRIRE,
            Droit::Approuver => PORTEE_APPROUVER,
        }
    }
}

/// Titulaire de l'instance : COG d'origine et identifiant par paire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Titulaire {
    cog: CogId,
    sujet: [u8; 32],
}

impl Titulaire {
    /// Analyse `pw:<cog_id>:<64 hex>` (forme de `central-remote`).
    pub fn analyser(texte: &str) -> Option<Self> {
        let reste = texte.trim().strip_prefix("pw:")?;
        let (cog, sujet) = reste.split_once(':')?;
        let cog = CogId::new(cog)?;
        let sujet: [u8; 32] = hex::decode(sujet).ok()?.try_into().ok()?;
        Some(Self { cog, sujet })
    }

    /// Titulaire `(cog, sujet)`.
    pub fn new(cog: CogId, sujet: [u8; 32]) -> Self {
        Self { cog, sujet }
    }

    /// Forme textuelle `pw:<cog_id>:<hex(sujet)>`.
    pub fn texte(&self) -> String {
        format!("pw:{}:{}", self.cog, hex::encode(self.sujet))
    }
}

/// Configuration des visas (voir l'en-tête du module).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigVisa {
    /// Clé d'identité du COG.
    pub racine: [u8; 32],
    /// Titulaire de l'instance.
    pub titulaire: Titulaire,
    /// Dossier d'état.
    pub etat: PathBuf,
    /// Identifiant du registre.
    pub registre: [u8; 32],
    /// Clé de journal.
    pub cle_journal: [u8; 32],
    /// Dépôt des révocations.
    pub depot: PathBuf,
}

fn nettoyer(v: Option<&str>) -> Option<&str> {
    v.map(str::trim).filter(|s| !s.is_empty())
}

impl ConfigVisa {
    /// Variables d'environnement (voir l'en-tête du module).
    pub fn depuis_env() -> Result<Option<Self>, String> {
        let v = |n: &str| std::env::var(n).ok();
        Self::depuis_valeurs([
            v(ENV_RACINE).as_deref(),
            v(ENV_TITULAIRE).as_deref(),
            v(ENV_ETAT).as_deref(),
            v(ENV_REGISTRE).as_deref(),
            v(ENV_CLE_JOURNAL).as_deref(),
            v(ENV_DEPOT_REVOCATIONS).as_deref(),
        ])
    }

    /// Variante pure : `[racine, titulaire, etat, registre, cle_journal, depot]`.
    /// Toutes absentes ⇒ `Ok(None)` ; un sous-ensemble ou une valeur invalide ⇒ erreur.
    pub fn depuis_valeurs(valeurs: [Option<&str>; 6]) -> Result<Option<Self>, String> {
        let v = valeurs.map(nettoyer);
        if v.iter().all(Option::is_none) {
            return Ok(None);
        }
        let [Some(racine), Some(titulaire), Some(etat), Some(registre), Some(journal), Some(depot)] =
            v
        else {
            return Err(format!(
                "{ENV_RACINE}, {ENV_TITULAIRE}, {ENV_ETAT}, {ENV_REGISTRE}, {ENV_CLE_JOURNAL} et \
                 {ENV_DEPOT_REVOCATIONS} vont ensemble"
            ));
        };
        let cle = |t: &str, nom: &str| {
            miyauth::http::service::analyser_cle_publique(t).ok_or_else(|| {
                format!("{nom} : clé publique Ed25519 (64 hex ou base64 URL) attendue")
            })
        };
        Ok(Some(Self {
            racine: cle(racine, ENV_RACINE)?,
            titulaire: Titulaire::analyser(titulaire)
                .ok_or_else(|| format!("{ENV_TITULAIRE} : forme pw:<cog_id>:<64 hex> attendue"))?,
            etat: PathBuf::from(etat),
            registre: hex::decode(registre)
                .ok()
                .and_then(|o| o.try_into().ok())
                .ok_or_else(|| format!("{ENV_REGISTRE} : 64 caractères hexadécimaux attendus"))?,
            cle_journal: cle(journal, ENV_CLE_JOURNAL)?,
            depot: PathBuf::from(depot),
        }))
    }
}

/// Vrai si la transition (clés épinglées) est demandée explicitement.
pub fn transition_demandee() -> bool {
    std::env::var(ENV_TRANSITION).is_ok_and(|v| v.trim() == VALEUR_TRANSITION)
}

/// Accès accordé par un visa vérifié.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccesVisa {
    /// Clé d'appareil (preuve de possession faite).
    pub cle_appareil: [u8; 32],
    /// Fin de validité du visa.
    pub expire_a: u64,
}

/// Refus d'un visa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusVisa {
    /// Titre, défi, délégation, révocation ou possession refusés.
    Http(RefusHttp),
    /// Passeport d'un autre titulaire (ou d'un autre COG, ou visiteur).
    AutreTitulaire,
    /// Portée absente du visa.
    PorteeInsuffisante,
}

/// Vérificateur des visas des routes mémoire.
pub struct AuthVisa {
    cog: CogId,
    annuaire: AnnuaireEmetteurs,
    titulaire: Titulaire,
    defis: Box<dyn MagasinDefis>,
    revocations: Arc<dyn SourceRevocation + Send + Sync>,
}

impl std::fmt::Debug for AuthVisa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthVisa")
            .field("cog", &self.cog)
            .finish_non_exhaustive()
    }
}

impl AuthVisa {
    /// Vérificateur en **mode strict** pour la racine `racine` ; le titulaire
    /// doit être un citoyen de ce COG.
    pub fn new(
        racine: &[u8; 32],
        titulaire: Titulaire,
        defis: Box<dyn MagasinDefis>,
        revocations: Arc<dyn SourceRevocation + Send + Sync>,
    ) -> Result<Self, String> {
        let cog = CogId::new(&miyusceau::delegation::cog_id_depuis_cle(racine))
            .ok_or("cog_id de la racine invalide")?;
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire
            .epingler_racine(cog.clone(), racine, NiveauConfiance::Souverain)
            .map_err(|e| format!("racine : {e}"))?;
        annuaire.exiger_delegations();
        if titulaire.cog != cog {
            return Err(format!(
                "{ENV_TITULAIRE} : le titulaire doit être un citoyen du COG {cog}"
            ));
        }
        Ok(Self {
            cog,
            annuaire,
            titulaire,
            defis,
            revocations,
        })
    }

    /// Charge le vérificateur depuis `config` : défis persistants dans
    /// `<etat>/defis`, révocations relevées depuis le dépôt.
    pub fn charger(config: &ConfigVisa) -> Result<(Self, RevocationsMemoire), String> {
        let defis = MagasinDefisFichier::ouvrir_defaut(&config.etat.join("defis"))
            .map_err(|e| format!("défis : {e}"))?;
        let cog = CogId::new(&miyusceau::delegation::cog_id_depuis_cle(&config.racine))
            .ok_or("cog_id de la racine invalide")?;
        let revocations = RevocationsMemoire::new(
            cog,
            config.registre,
            &config.cle_journal,
            Some(config.depot.clone()),
            Some(&config.etat),
        )?;
        let auth = Self::new(
            &config.racine,
            config.titulaire.clone(),
            Box::new(defis),
            Arc::new(revocations.clone()),
        )?;
        Ok((auth, revocations))
    }

    /// COG local.
    pub fn cog(&self) -> &CogId {
        &self.cog
    }

    /// Émet un défi de possession (audience [`AUDIENCE`]).
    pub fn emettre_defi(&self, maintenant: u64) -> Result<DefiEmis, RefusHttp> {
        self.defis.emettre(AUDIENCE, maintenant)
    }

    /// Vérifie la requête (voir l'en-tête du module) pour `droit`.
    pub fn verifier(
        &self,
        requete: &RequeteHttp<'_>,
        delegations: Option<&str>,
        droit: Droit,
        maintenant: u64,
    ) -> Result<AccesVisa, RefusVisa> {
        let politique = PolitiqueIdentite {
            cog_local: self.cog.clone(),
            annuaire: &self.annuaire,
            revocations: &*self.revocations,
        };
        let prouve = verifier_requete_visa_avec_delegations(
            requete,
            delegations,
            AUDIENCE,
            &politique,
            self.defis.as_ref(),
            maintenant,
        )
        .map_err(RefusVisa::Http)?;
        if prouve.role() != IdentityRole::Citizen
            || prouve.cog_origine() != &self.titulaire.cog
            || prouve.sujet() != &self.titulaire.sujet
        {
            return Err(RefusVisa::AutreTitulaire);
        }
        let visa = prouve.acces().visa();
        if !visa.portees.contient(droit.portee()) {
            return Err(RefusVisa::PorteeInsuffisante);
        }
        Ok(AccesVisa {
            cle_appareil: visa.cle_appareil,
            expire_a: visa.pas_apres,
        })
    }
}

/// Dossier des nonces de transition (persistants) sous le dossier d'état.
pub fn dossier_nonces(etat: &Path) -> PathBuf {
    etat.join("nonces")
}

#[cfg(test)]
pub(crate) mod banc {
    //! COG d'essai : racine, délégations `emission` et `visas`, passeports et visas.
    use super::*;
    use miyauth::http::{
        formater_autorisation_visa, formater_delegations, formater_pop, liaison_requete, signer_pop,
    };
    use miyauth::{
        emettre_passeport, emettre_visa, identifiant_pairwise, prouver_possession,
        verifier_passeport, DemandePasseport, DemandeVisa, ListeRevocation, Portees,
        SecretTitulaire,
    };
    use miyuracine::{Racine, Usage};
    use miyusceau::CleSignature;

    pub(crate) fn maintenant() -> u64 {
        miyauth::http::service::maintenant_unix()
    }

    /// COG d'essai.
    pub(crate) struct MondeVisa {
        pub racine: Racine,
        pub cog: CogId,
        pub deleg_emission: Vec<u8>,
        pub deleg_visas: Vec<u8>,
    }

    impl MondeVisa {
        pub(crate) fn new() -> Self {
            let racine = Racine::depuis_secret(&[0x5a; 32]);
            let t = maintenant();
            let cog = CogId::new(&miyusceau::delegation::cog_id_depuis_cle(
                &racine.cle_publique_identite(),
            ))
            .expect("cog");
            Self {
                deleg_emission: racine
                    .deleguer(Usage::Emission, 0, t - 86_400, t + 30 * 86_400)
                    .expect("délégation emission"),
                deleg_visas: racine
                    .deleguer(Usage::Visas, 0, t - 86_400, t + 30 * 86_400)
                    .expect("délégation visas"),
                racine,
                cog,
            }
        }

        pub(crate) fn racine_publique(&self) -> [u8; 32] {
            self.racine.cle_publique_identite()
        }

        pub(crate) fn secret(nom: &str) -> SecretTitulaire {
            let mut o = [0u8; 32];
            for (i, b) in nom.bytes().enumerate().take(32) {
                o[i] = b;
            }
            SecretTitulaire::depuis_octets(o)
        }

        pub(crate) fn titulaire(&self, nom: &str) -> Titulaire {
            Titulaire::new(
                self.cog.clone(),
                identifiant_pairwise(&Self::secret(nom), &self.cog),
            )
        }

        /// Révocations fraîches et vides pour ce COG.
        pub(crate) fn revocations(&self) -> Arc<dyn SourceRevocation + Send + Sync> {
            Arc::new(ListeRevocation::new(self.cog.clone(), maintenant(), 3600))
        }

        /// Délégations à présenter (`visas`, `emission`).
        pub(crate) fn entete_delegations(&self) -> String {
            formater_delegations(&[&self.deleg_visas, &self.deleg_emission])
        }

        /// Passeport de `nom` pour l'appareil `appareil`.
        pub(crate) fn passeport(
            &self,
            nom: &str,
            appareil: &CleSignature,
            portees: &[&str],
        ) -> Vec<u8> {
            let t = maintenant();
            emettre_passeport(
                &self.cog,
                &self.racine.cle(Usage::Emission, 0).expect("clé emission"),
                &DemandePasseport {
                    secret: &Self::secret(nom),
                    destination: self.cog.clone(),
                    cle_appareil: appareil.cle_publique(),
                    portees: Portees::new(portees.iter().copied()).expect("portées"),
                    pas_avant: t - 3600,
                    pas_apres: t + 86_400,
                },
            )
            .expect("passeport")
        }

        /// Visa (fenêtre `[debut, fin]`) scellé par `cle_visas` (défaut : clé `visas`).
        pub(crate) fn visa_fenetre(
            &self,
            passeport: &[u8],
            appareil: &CleSignature,
            portees: &[&str],
            fenetre: (u64, u64),
            cle_visas: Option<CleSignature>,
        ) -> Vec<u8> {
            let t = maintenant();
            let mut annuaire = AnnuaireEmetteurs::new();
            annuaire
                .epingler_racine(
                    self.cog.clone(),
                    &self.racine_publique(),
                    NiveauConfiance::Souverain,
                )
                .expect("racine");
            annuaire
                .ajouter_delegation(&self.deleg_emission, t)
                .expect("délégation");
            let rev = ListeRevocation::new(self.cog.clone(), t, 3600);
            let politique = PolitiqueIdentite {
                cog_local: self.cog.clone(),
                annuaire: &annuaire,
                revocations: &rev,
            };
            let pv = verifier_passeport(passeport, &politique, t).expect("passeport vérifié");
            let defi = [3u8; 32];
            let liaison = [4u8; 32];
            let sig = signer_pop(appareil, &defi, "guichet", &liaison);
            let possession =
                prouver_possession(pv, &defi, "guichet", &liaison, &sig).expect("possession");
            emettre_visa(
                &self.cog,
                &cle_visas.unwrap_or_else(|| self.racine.cle(Usage::Visas, 0).expect("visas")),
                &possession,
                &DemandeVisa {
                    audience: AUDIENCE.into(),
                    portees: Portees::new(portees.iter().copied()).expect("portées"),
                    pas_avant: fenetre.0,
                    pas_apres: fenetre.1,
                },
            )
            .expect("visa")
        }

        /// Visa valide 10 min.
        pub(crate) fn visa(
            &self,
            passeport: &[u8],
            appareil: &CleSignature,
            portees: &[&str],
        ) -> Vec<u8> {
            let t = maintenant();
            self.visa_fenetre(passeport, appareil, portees, (t - 60, t + 600), None)
        }
    }

    /// Titres présentés par un client.
    pub(crate) struct Titres {
        pub visa: Vec<u8>,
        pub passeport: Vec<u8>,
        pub delegations: Option<String>,
        pub appareil: CleSignature,
    }

    impl Titres {
        /// En-têtes `(nom, valeur)` d'une requête prouvée avec le défi `(id, defi)`.
        pub(crate) fn entetes(
            &self,
            defi: (&[u8; 16], &[u8; 32]),
            methode: &str,
            chemin: &str,
            corps: &[u8],
        ) -> Vec<(&'static str, String)> {
            let sig = signer_pop(
                &self.appareil,
                defi.1,
                AUDIENCE,
                &liaison_requete(methode, chemin, corps),
            );
            let mut v = vec![
                (
                    "authorization",
                    formater_autorisation_visa(&self.visa, &self.passeport),
                ),
                ("x-miyu-pop", formater_pop(defi.0, &sig)),
            ];
            if let Some(d) = &self.delegations {
                v.push(("x-miyu-delegations", d.clone()));
            }
            v
        }
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::banc::*;
    use super::*;
    use miyauth::http::MagasinDefisMemoire;
    use miyusceau::CleSignature;

    #[test]
    fn titulaire_analyse() {
        let t = Titulaire::analyser(&format!(
            "pw:abcdefghijklmnopqrstuvwxyz:{}",
            "ab".repeat(32)
        ))
        .expect("titulaire");
        assert_eq!(t.sujet, [0xab; 32]);
        assert_eq!(Titulaire::analyser(&t.texte()), Some(t));
        for faux in [
            "abcdefghijklmnopqrstuvwxyz:00",
            "pw:court:00",
            &format!("pw:abcdefghijklmnopqrstuvwxyz:{}", "ab".repeat(31)),
            &format!("pw:abcdefghijklmnopqrstuvwxyz:{}", "zz".repeat(32)),
        ] {
            assert!(Titulaire::analyser(faux).is_none(), "{faux}");
        }
    }

    #[test]
    fn configuration_tout_ou_rien() {
        assert_eq!(ConfigVisa::depuis_valeurs([None; 6]), Ok(None));
        let racine = hex::encode([7u8; 32]);
        let titulaire = format!("pw:abcdefghijklmnopqrstuvwxyz:{}", "01".repeat(32));
        let registre = hex::encode([1u8; 32]);
        let complet = [
            Some(racine.as_str()),
            Some(titulaire.as_str()),
            Some("/etat"),
            Some(registre.as_str()),
            Some(racine.as_str()),
            Some("/depot"),
        ];
        let c = ConfigVisa::depuis_valeurs(complet)
            .expect("ok")
            .expect("config");
        assert_eq!(c.registre, [1; 32]);
        for i in 0..6 {
            let mut partiel = complet;
            partiel[i] = None;
            assert!(ConfigVisa::depuis_valeurs(partiel).is_err(), "{i}");
        }
        let mut faux = complet;
        faux[1] = Some("alice");
        assert!(ConfigVisa::depuis_valeurs(faux).is_err());
    }

    #[test]
    fn titulaire_d_un_autre_cog_refuse() {
        let m = MondeVisa::new();
        let autre = Titulaire::new(
            CogId::new("abcdefghijklmnopqrstuvwxyz").expect("c"),
            [1; 32],
        );
        assert!(AuthVisa::new(
            &m.racine_publique(),
            autre,
            Box::new(MagasinDefisMemoire::default()),
            m.revocations()
        )
        .is_err());
    }

    fn requete<'a>(
        entetes: &'a [(&'static str, String)],
        chemin: &'a str,
    ) -> (RequeteHttp<'a>, Option<&'a str>) {
        let v = |n: &str| {
            entetes
                .iter()
                .find(|(k, _)| *k == n)
                .map(|(_, v)| v.as_str())
        };
        (
            RequeteHttp {
                methode: "POST",
                chemin,
                corps: b"",
                autorisation: v("authorization"),
                pop: v("x-miyu-pop"),
                liaison: None,
            },
            v("x-miyu-delegations"),
        )
    }

    #[test]
    fn visa_valide_puis_portee_titulaire_et_rejeu() {
        let m = MondeVisa::new();
        let auth = AuthVisa::new(
            &m.racine_publique(),
            m.titulaire("alice"),
            Box::new(MagasinDefisMemoire::default()),
            m.revocations(),
        )
        .expect("auth");
        let appareil = CleSignature::depuis_graine(&[0x21; 32]);
        let p = m.passeport("alice", &appareil, &[PORTEE_LIRE]);
        let titres = Titres {
            visa: m.visa(&p, &appareil, &[PORTEE_LIRE]),
            passeport: p,
            delegations: Some(m.entete_delegations()),
            appareil,
        };
        let chemin = "/v1/memoire/alice/consulter";
        let d = auth.emettre_defi(maintenant()).expect("défi");
        let e = titres.entetes((&d.id, &d.defi), "POST", chemin, b"");
        let (r, del) = requete(&e, chemin);
        let acces = auth
            .verifier(&r, del, Droit::Lire, maintenant())
            .expect("accès");
        assert_eq!(acces.cle_appareil, titres.appareil.cle_publique());
        // Rejeu : défi consommé.
        assert_eq!(
            auth.verifier(&r, del, Droit::Lire, maintenant()),
            Err(RefusVisa::Http(RefusHttp::DefiInvalide))
        );
        // Portée d'écriture absente.
        let d = auth.emettre_defi(maintenant()).expect("défi");
        let e = titres.entetes((&d.id, &d.defi), "POST", chemin, b"");
        let (r, del) = requete(&e, chemin);
        assert_eq!(
            auth.verifier(&r, del, Droit::Ecrire, maintenant()),
            Err(RefusVisa::PorteeInsuffisante)
        );
        // Autre titulaire (bob, même COG).
        let auth_bob = AuthVisa::new(
            &m.racine_publique(),
            m.titulaire("bob"),
            Box::new(MagasinDefisMemoire::default()),
            m.revocations(),
        )
        .expect("auth");
        let d = auth_bob.emettre_defi(maintenant()).expect("défi");
        let e = titres.entetes((&d.id, &d.defi), "POST", chemin, b"");
        let (r, del) = requete(&e, chemin);
        assert_eq!(
            auth_bob.verifier(&r, del, Droit::Lire, maintenant()),
            Err(RefusVisa::AutreTitulaire)
        );
    }

    #[test]
    fn appareil_revoque_ou_revocations_perimees_refuses() {
        use miyauth::{CibleRevocation, ListeRevocation, RefusIdentite};
        let m = MondeVisa::new();
        let appareil = CleSignature::depuis_graine(&[0x22; 32]);
        let p = m.passeport("alice", &appareil, &[PORTEE_LIRE]);
        let titres = Titres {
            visa: m.visa(&p, &appareil, &[PORTEE_LIRE]),
            passeport: p,
            delegations: Some(m.entete_delegations()),
            appareil,
        };
        let mut revoquee = ListeRevocation::new(m.cog.clone(), maintenant(), 3600);
        revoquee.revoquer(CibleRevocation::appareil(&titres.appareil.cle_publique()));
        let perimee = ListeRevocation::new(m.cog.clone(), maintenant() - 7200, 900);
        for liste in [revoquee, perimee] {
            let auth = AuthVisa::new(
                &m.racine_publique(),
                m.titulaire("alice"),
                Box::new(MagasinDefisMemoire::default()),
                Arc::new(liste),
            )
            .expect("auth");
            let chemin = "/v1/memoire/alice/consulter";
            let d = auth.emettre_defi(maintenant()).expect("défi");
            let e = titres.entetes((&d.id, &d.defi), "POST", chemin, b"");
            let (r, del) = requete(&e, chemin);
            assert!(matches!(
                auth.verifier(&r, del, Droit::Lire, maintenant()),
                Err(RefusVisa::Http(RefusHttp::Identite(
                    RefusIdentite::Revoque(_) | RefusIdentite::Revocation(_)
                )))
            ));
        }
    }
}
