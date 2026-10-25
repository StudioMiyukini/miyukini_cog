//! @id     maia_federee_etiquettes
//! @do     attester_fiche_originale_publiable
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « etiquettes » : seul producteur de la capacité `Publiable` ; lit
//! @human  l'étiquette KindMother `publiable` de la donnée source exacte et vérifie que la Fiche
//! @human  en est dérivée (règle stricte) avant d'attester les octets exacts (EX-47).

//! Production de la capacité [`Publiable`] (IA-FEDEREE §5, EX-47, A-013).
//!
//! [`PublieurOriginal`] détient l'[`AutoriteEtiquettes`] (jeton unique par
//! processus). Pour chaque publication originale il :
//!
//! 1. vérifie la Fiche signée (format, signature, normalisation) ; une Fiche à
//!    `reprises` est refusée (republication : TAMR, A-009) ;
//! 2. **lit l'étiquette** de la donnée source exacte auprès de KindMother
//!    ([`LecteurEtiquettes`]) — jamais un état mis en cache : un retrait de
//!    l'étiquette arrête toute nouvelle attestation ;
//! 3. applique la **règle de dérivation stricte** ([`verifier_derivation`]) ;
//! 4. alors seulement construit [`Publiable`] pour l'empreinte des octets
//!    signés exacts, valable [`crate::requete::VALIDITE_PUBLIABLE_SECS`] secondes.
//!
//! ## Règle de dérivation (stricte, à consigner par amendement)
//!
//! Soit `E` l'empreinte du contenu étiqueté lue dans KindMother
//! (`SHA-256("miyukini/kindmother/publiable/contenu/v1\0" || texte)`, la même
//! que `kindmother-service`). La Fiche est publiable si et seulement si :
//!
//! - la donnée n'est pas `nationale` (mémoire de MAIA `maia-*` ou base déclarée) ;
//! - l'étiquette est **posée** (par une intention signée de l'utilisateur) et
//!   le contenu **courant** de la donnée a encore l'empreinte `E` ;
//! - les champs textuels **signés** sont déjà normalisés
//!   ([`FicheVerifiee::canonique`]) : aucun octet signé (invisible, bidi,
//!   balisage) ne porte d'information cachée ;
//! - `SHA-256(domaine || texte de la Fiche) == E` : le **texte** de la Fiche est
//!   octet pour octet le contenu étiqueté ;
//! - le **sujet**, chaque terme de **relation** et chaque **source** sont
//!   dérivables du texte étiqueté : leur clé de comparaison
//!   ([`crate::normalisation::cle_comparaison`]) est non vide et contenue dans
//!   celle du texte. Aucun champ ne publie d'information absente du contenu
//!   étiqueté.
//!
//! Champs **structurels** (revue de sécurité M1 : sans liaison, MAIA pourrait
//! y cacher des données ou élargir l'audience) — tous liés à l'étiquette :
//!
//! - `id` = [`id_fiche_derivee`] de l'identité de la donnée étiquetée
//!   (`PREFIXE_ID_FICHE` + 32 premiers caractères hex de l'empreinte de sa
//!   référence, rendue par KindMother) : aucun choix libre ;
//! - `date` à au plus [`FENETRE_DATE_SECS`] secondes de `maintenant` ;
//! - `version` = dernière version publiée de cet `id` + 1 (fournie par le
//!   service de publication ; [`crate::ServiceRequete`] la revérifie) ;
//! - `type`, `langue`, `licence`, `audience` **égaux exactement** aux
//!   paramètres de publication choisis par l'**utilisateur** et signés avec la
//!   pose (intention d'étiquette v2 de KindMother) ; pose sans paramètres ⇒
//!   jamais publiable ;
//! - au plus [`MAX_ATTESTATIONS_PAR_PERIODE`] attestations par donnée et par
//!   [`PERIODE_ATTESTATIONS_SECS`] secondes (au-delà : [`ErreurFederee::Quota`]).

use std::collections::{BTreeMap, VecDeque};
use std::future::Future;
use std::sync::Mutex;

use crate::erreurs::ErreurFederee;
use crate::fiche::{verifier_fiche, Audience, FicheVerifiee, TypeFiche};
use crate::normalisation::cle_comparaison;
use crate::requete::{AutoriteEtiquettes, Publiable};

/// Préfixe fixe de l'identifiant d'une Fiche originale (dérivé de la donnée).
pub const PREFIXE_ID_FICHE: &str = "km.";
/// Caractères hexadécimaux de l'empreinte de la donnée gardés dans l'`id` (128 bits).
pub const LONGUEUR_HEX_ID: usize = 32;
/// Écart maximal entre la `date` de la Fiche et l'horloge (secondes).
pub const FENETRE_DATE_SECS: u64 = 300;
/// Attestations au plus par donnée et par période.
pub const MAX_ATTESTATIONS_PAR_PERIODE: usize = 8;
/// Période du plafond d'attestations (secondes : 24 h).
pub const PERIODE_ATTESTATIONS_SECS: u64 = 86_400;

/// Identifiant imposé de la Fiche originale dérivée de la donnée d'identité
/// `empreinte_donnee` (empreinte de la référence exacte, calculée par KindMother).
pub fn id_fiche_derivee(empreinte_donnee: &[u8; 32]) -> String {
    use std::fmt::Write as _;
    let mut id = String::from(PREFIXE_ID_FICHE);
    for b in &empreinte_donnee[..LONGUEUR_HEX_ID / 2] {
        let _ = write!(id, "{b:02x}");
    }
    id
}

/// Paramètres de publication choisis par l'utilisateur et signés avec la pose
/// (`kindmother::etiquette::ParametresPublication`, intention v2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametresLus {
    /// Nature imposée de la Fiche.
    pub type_fiche: TypeFiche,
    /// Langue imposée.
    pub langue: String,
    /// Licence imposée.
    pub licence: String,
    /// Audience imposée (comparée exactement, ordre compris).
    pub audience: Audience,
}

/// Séparation de domaine de l'empreinte du contenu publiable (identique à
/// `kindmother::etiquette::DOMAINE_CONTENU`).
pub const DOMAINE_CONTENU: &[u8] = b"miyukini/kindmother/publiable/contenu/v1\0";

/// Empreinte du contenu publiable d'un texte.
pub fn empreinte_contenu(texte: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(DOMAINE_CONTENU);
    h.update(texte.as_bytes());
    h.finalize().into()
}

/// État de l'étiquette de la donnée source, tel que lu auprès de KindMother
/// (`kindmother::etiquette::EtatEtiquette`). Le drapeau `publiable` calculé par
/// KindMother n'est pas repris : la décision est recalculée ici.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtiquetteLue {
    /// Donnée `nationale` (mémoire de MAIA ou base déclarée).
    pub nationale: bool,
    /// Étiquette `publiable` posée par l'utilisateur et non retirée.
    pub posee: bool,
    /// Empreinte du contenu étiqueté.
    pub empreinte_contenu: Option<[u8; 32]>,
    /// Empreinte du contenu courant de la donnée.
    pub empreinte_actuelle: Option<[u8; 32]>,
    /// Identité de la donnée (empreinte de sa référence exacte) : base de l'`id` imposé.
    pub empreinte_donnee: Option<[u8; 32]>,
    /// Paramètres de publication signés par l'utilisateur (absents : jamais publiable).
    pub parametres: Option<ParametresLus>,
}

fn hex32(texte: &str) -> Option<[u8; 32]> {
    let b = texte.as_bytes();
    if b.len() != 64 {
        return None;
    }
    let val = |c: u8| (c as char).to_digit(16).and_then(|d| u8::try_from(d).ok());
    let mut out = [0u8; 32];
    for (i, o) in out.iter_mut().enumerate() {
        *o = (val(b[2 * i])? << 4) | val(b[2 * i + 1])?;
    }
    Some(out)
}

impl EtiquetteLue {
    /// Depuis les champs hexadécimaux rendus par KindMother ; une empreinte mal
    /// formée est traitée comme absente (fail-closed). Identité de la donnée et
    /// paramètres : [`Self::avec_donnee`], [`Self::avec_parametres`].
    pub fn depuis_hex(nationale: bool, posee: bool, empreinte_contenu: Option<&str>, empreinte_actuelle: Option<&str>) -> Self {
        Self {
            nationale,
            posee,
            empreinte_contenu: empreinte_contenu.and_then(hex32),
            empreinte_actuelle: empreinte_actuelle.and_then(hex32),
            empreinte_donnee: None,
            parametres: None,
        }
    }

    /// Identité de la donnée (hex rendu par KindMother ; mal formée ⇒ absente).
    #[must_use]
    pub fn avec_donnee(mut self, empreinte_donnee: Option<&str>) -> Self {
        self.empreinte_donnee = empreinte_donnee.and_then(hex32);
        self
    }

    /// Paramètres de publication signés par l'utilisateur.
    #[must_use]
    pub fn avec_parametres(mut self, parametres: Option<ParametresLus>) -> Self {
        self.parametres = parametres;
        self
    }
}

/// Lecture de l'étiquette d'une donnée auprès de KindMother (requête typée
/// `LireEtiquette` de `kindmother-client`). Implémentée par l'appelant qui
/// détient la connexion KindMother ; une erreur de lecture est un refus.
pub trait LecteurEtiquettes {
    /// Référence exacte de la donnée source (`kindmother::etiquette::RefDonnee`).
    type Donnee: ?Sized + Sync;

    /// Lit l'état **courant** de l'étiquette de `donnee`.
    fn lire(&self, donnee: &Self::Donnee) -> impl Future<Output = Result<EtiquetteLue, ErreurFederee>> + Send;
}

/// Règle de dérivation stricte (voir la documentation du module) ;
/// `derniere_version` : dernière version publiée de l'`id` imposé (0 : jamais).
pub fn verifier_derivation(
    fiche: &FicheVerifiee,
    etiquette: &EtiquetteLue,
    derniere_version: u64,
    maintenant: u64,
) -> Result<(), ErreurFederee> {
    if etiquette.nationale {
        return Err(ErreurFederee::Interdit("donnée nationale : jamais publiable"));
    }
    if fiche.est_reprise() {
        return Err(ErreurFederee::ValidationRequise);
    }
    let Some(etiquete) = etiquette.empreinte_contenu.filter(|_| etiquette.posee) else {
        return Err(ErreurFederee::NonPubliable);
    };
    if etiquette.empreinte_actuelle != Some(etiquete) {
        return Err(ErreurFederee::NonPubliable);
    }
    if !fiche.canonique() {
        return Err(ErreurFederee::NonDerivee("champs signés non normalisés"));
    }
    let f = fiche.fiche();
    if empreinte_contenu(&f.texte) != etiquete {
        return Err(ErreurFederee::NonDerivee("texte différent du contenu étiqueté"));
    }
    let corpus = cle_comparaison(&f.texte);
    let derive = |champ: &str| {
        let c = cle_comparaison(champ);
        !c.is_empty() && corpus.contains(&c)
    };
    if !derive(&f.sujet) {
        return Err(ErreurFederee::NonDerivee("sujet absent du texte étiqueté"));
    }
    if !f.relations.iter().all(|r| derive(&r.sujet) && derive(&r.predicat) && derive(&r.objet)) {
        return Err(ErreurFederee::NonDerivee("relation absente du texte étiqueté"));
    }
    if !f.sources.iter().all(|s| derive(s)) {
        return Err(ErreurFederee::NonDerivee("source absente du texte étiqueté"));
    }
    verifier_structure(fiche, etiquette, derniere_version, maintenant)
}

/// Champs structurels liés à l'étiquette (M1).
fn verifier_structure(
    fiche: &FicheVerifiee,
    etiquette: &EtiquetteLue,
    derniere_version: u64,
    maintenant: u64,
) -> Result<(), ErreurFederee> {
    let f = fiche.fiche();
    let Some(donnee) = etiquette.empreinte_donnee else {
        return Err(ErreurFederee::NonPubliable);
    };
    if f.id != id_fiche_derivee(&donnee) {
        return Err(ErreurFederee::NonDerivee("id non dérivé de la donnée étiquetée"));
    }
    if f.date.abs_diff(maintenant) > FENETRE_DATE_SECS {
        return Err(ErreurFederee::NonDerivee("date hors de la fenêtre"));
    }
    if Some(f.version) != derniere_version.checked_add(1) {
        return Err(ErreurFederee::NonDerivee("version ≠ dernière version publiée + 1"));
    }
    let Some(p) = &etiquette.parametres else {
        return Err(ErreurFederee::NonPubliable);
    };
    if f.type_fiche != p.type_fiche {
        return Err(ErreurFederee::NonDerivee("type différent du choix de l'utilisateur"));
    }
    if f.langue != p.langue {
        return Err(ErreurFederee::NonDerivee("langue différente du choix de l'utilisateur"));
    }
    if f.licence != p.licence {
        return Err(ErreurFederee::NonDerivee("licence différente du choix de l'utilisateur"));
    }
    if f.audience != p.audience {
        return Err(ErreurFederee::NonDerivee("audience différente du choix de l'utilisateur"));
    }
    Ok(())
}

/// Seul producteur de [`Publiable`] : détient l'[`AutoriteEtiquettes`].
#[derive(Debug)]
pub struct PublieurOriginal {
    autorite: AutoriteEtiquettes,
    /// Instants des attestations récentes, par identité de donnée (plafond M1).
    attestations: Mutex<BTreeMap<[u8; 32], VecDeque<u64>>>,
}

impl PublieurOriginal {
    /// Publieur détenteur de l'autorité (obtenue par [`AutoriteEtiquettes::instituer`]).
    pub fn new(autorite: AutoriteEtiquettes) -> Self {
        Self { autorite, attestations: Mutex::new(BTreeMap::new()) }
    }

    /// Atteste que les octets signés `octets` sont publiables : étiquette de
    /// `donnee` lue **maintenant** auprès de KindMother, puis règle de
    /// dérivation (`derniere_version` : dernière version publiée de l'`id`
    /// imposé, 0 si aucune), puis plafond d'attestations de la donnée.
    pub async fn attester<L: LecteurEtiquettes>(
        &self,
        lecteur: &L,
        donnee: &L::Donnee,
        octets: &[u8],
        derniere_version: u64,
        maintenant: u64,
    ) -> Result<Publiable, ErreurFederee> {
        let fiche = verifier_fiche(octets, maintenant)?;
        if fiche.est_reprise() {
            return Err(ErreurFederee::ValidationRequise);
        }
        let etiquette = lecteur.lire(donnee).await?;
        let resultat = verifier_derivation(&fiche, &etiquette, derniere_version, maintenant)
            .and_then(|()| self.compter_attestation(etiquette.empreinte_donnee, maintenant));
        if let Err(e) = resultat {
            tracing::warn!(target: "maia_federee::audit", fiche = %fiche.fiche().id, motif = %e, "publication originale refusée");
            return Err(e);
        }
        tracing::info!(target: "maia_federee::audit", fiche = %fiche.fiche().id, "Fiche originale attestée publiable (étiquette de l'utilisateur)");
        Publiable::attestee(&self.autorite, &fiche, maintenant)
    }

    /// Plafond : au plus [`MAX_ATTESTATIONS_PAR_PERIODE`] par donnée sur
    /// [`PERIODE_ATTESTATIONS_SECS`] ; l'attestation est comptée si admise.
    fn compter_attestation(&self, donnee: Option<[u8; 32]>, maintenant: u64) -> Result<(), ErreurFederee> {
        let donnee = donnee.ok_or(ErreurFederee::NonPubliable)?;
        let mut table = self
            .attestations
            .lock()
            .map_err(|_| ErreurFederee::Interdit("état des attestations corrompu"))?;
        let debut = maintenant.saturating_sub(PERIODE_ATTESTATIONS_SECS);
        table.retain(|_, instants| {
            while instants.front().is_some_and(|&t| t <= debut) {
                instants.pop_front();
            }
            !instants.is_empty()
        });
        let instants = table.entry(donnee).or_default();
        if instants.len() >= MAX_ATTESTATIONS_PAR_PERIODE {
            return Err(ErreurFederee::Quota);
        }
        instants.push_back(maintenant);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiche::essais::fiche;
    use crate::fiche::{sceller_fiche, Fiche, Relation};
    use crate::requete::{Publication, ServiceRequete, VALIDITE_PUBLIABLE_SECS};
    use miyusceau::CleSignature;
    use sha2::{Digest, Sha256};
    use std::sync::Mutex;

    const T: u64 = 1_800_000_100;
    const TEXTE: &str = "Paris est la capitale de la France.";

    /// Exécute un futur sans exécuteur (les lecteurs factices sont prêts tout de suite).
    fn bloquer<F: Future>(f: F) -> F::Output {
        let mut f = std::pin::pin!(f);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        loop {
            if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
                return v;
            }
        }
    }

    /// Identité factice d'une donnée (KindMother : empreinte de la référence).
    fn identite(id: &str) -> [u8; 32] {
        Sha256::digest(id.as_bytes()).into()
    }

    /// Paramètres choisis par l'utilisateur (ceux de la Fiche d'essai).
    fn parametres() -> ParametresLus {
        ParametresLus { type_fiche: TypeFiche::Fait, langue: "fr".into(), licence: "CC-BY-4.0".into(), audience: Audience::Membres }
    }

    fn lue(id: &str, posee: bool, contenu: Option<[u8; 32]>, actuelle: Option<[u8; 32]>) -> EtiquetteLue {
        EtiquetteLue {
            nationale: id.starts_with("maia-"),
            posee,
            empreinte_contenu: contenu,
            empreinte_actuelle: actuelle,
            empreinte_donnee: Some(identite(id)),
            parametres: Some(parametres()),
        }
    }

    /// KindMother factice : état d'étiquette par identifiant de donnée.
    struct KindMotherFactice(Mutex<Vec<(&'static str, EtiquetteLue)>>);

    impl KindMotherFactice {
        fn poser(&self, id: &'static str, texte: &str) {
            let e = empreinte_contenu(texte);
            self.fixer(id, lue(id, true, Some(e), Some(e)));
        }
        fn fixer(&self, id: &'static str, e: EtiquetteLue) {
            let mut v = self.0.lock().unwrap();
            v.retain(|(i, _)| *i != id);
            v.push((id, e));
        }
        fn retirer(&self, id: &'static str) {
            let mut v = self.0.lock().unwrap();
            for (i, e) in v.iter_mut() {
                if *i == id {
                    e.posee = false;
                    e.empreinte_contenu = None;
                }
            }
        }
    }

    impl LecteurEtiquettes for KindMotherFactice {
        type Donnee = str;
        fn lire(&self, donnee: &str) -> impl Future<Output = Result<EtiquetteLue, ErreurFederee>> + Send {
            std::future::ready(Ok(self
                .0
                .lock()
                .unwrap()
                .iter()
                .find(|(i, _)| *i == donnee)
                .map_or(lue(donnee, false, None, None), |(_, e)| e.clone())))
        }
    }

    fn publieur() -> PublieurOriginal {
        PublieurOriginal::new(AutoriteEtiquettes::pour_essais())
    }

    /// Fiche dérivée de la donnée `donnee` (id imposé), version `version`.
    fn derivee(cle: &CleSignature, donnee: &str, version: u64) -> Fiche {
        Fiche {
            sujet: "capitale de la France".into(),
            sources: Vec::new(),
            version,
            ..fiche(cle, &id_fiche_derivee(&identite(donnee)), TEXTE, &[("Paris", "capitale de", "France")])
        }
    }

    fn fiche_derivee(cle: &CleSignature) -> Fiche {
        derivee(cle, "notes.n1", 1)
    }

    fn cog(cle: &CleSignature) -> miyauth::CogId {
        crate::fiche::cog_de_cle(&cle.cle_publique()).unwrap()
    }

    #[test]
    fn vecteur_partage_avec_kindmother() {
        // Même vecteur que `kindmother-service::etiquettes` (domaine et règle identiques).
        let hex = empreinte_contenu(TEXTE).iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        });
        assert_eq!(hex, "fc13b5df90b60dd6b4fcc79af479c7c6e6eb0b5f791fd19845d523897f865284");
        assert_eq!(EtiquetteLue::depuis_hex(false, true, Some(&hex), Some(&hex)).empreinte_contenu, Some(empreinte_contenu(TEXTE)));
        assert_eq!(EtiquetteLue::depuis_hex(false, true, Some("zz"), None).empreinte_contenu, None);
        let l = EtiquetteLue::depuis_hex(false, true, None, None).avec_donnee(Some(&hex));
        assert_eq!(l.empreinte_donnee, Some(empreinte_contenu(TEXTE)));
        assert_eq!(id_fiche_derivee(&empreinte_contenu(TEXTE)), format!("km.{}", &hex[..32]));
    }

    #[test]
    fn etiquetee_par_l_utilisateur_publiable_puis_retrait() {
        let cle = CleSignature::generer();
        let octets = sceller_fiche(&fiche_derivee(&cle), &cle).unwrap();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        let p = publieur();
        // Non étiquetée : refus.
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonPubliable));
        // Étiquetée par l'utilisateur : publication possible.
        km.poser("notes.n1", TEXTE);
        let capacite = bloquer(p.attester(&km, "notes.n1", &octets, 0, T)).unwrap();
        let mut s = ServiceRequete::new(cog(&cle));
        s.publier(octets.clone(), Publication::Originale(capacite), T + 1).unwrap();
        assert_eq!(s.derniere_version(&fiche_derivee(&cle).id), 1);
        // Retrait : plus de nouvelle publication (nouvelle version refusée).
        km.retirer("notes.n1");
        let v2 = sceller_fiche(&derivee(&cle, "notes.n1", 2), &cle).unwrap();
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &v2, 1, T)), Err(ErreurFederee::NonPubliable));
        // Capacité attestée avant le retrait : périmée après VALIDITE_PUBLIABLE_SECS.
        km.poser("notes.n1", TEXTE);
        let ancienne = bloquer(p.attester(&km, "notes.n1", &v2, 1, T)).unwrap();
        km.retirer("notes.n1");
        assert_eq!(
            s.publier(v2, Publication::Originale(ancienne), T + VALIDITE_PUBLIABLE_SECS + 1),
            Err(ErreurFederee::NonPubliable)
        );
    }

    #[test]
    fn nationale_contenu_modifie_ou_autre_donnee_refuses() {
        let cle = CleSignature::generer();
        let octets = sceller_fiche(&fiche_derivee(&cle), &cle).unwrap();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        let p = publieur();
        // Mémoire de MAIA (`nationale`) : refus même « posée ».
        km.poser("maia-alice.n1", TEXTE);
        assert!(matches!(bloquer(p.attester(&km, "maia-alice.n1", &octets, 0, T)), Err(ErreurFederee::Interdit(_))));
        // Contenu courant modifié depuis la pose.
        let e = empreinte_contenu(TEXTE);
        km.fixer("notes.n1", lue("notes.n1", true, Some(e), Some(empreinte_contenu("autre"))));
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonPubliable));
        // Étiquette d'une autre donnée (autre contenu) : la Fiche n'en dérive pas.
        km.poser("notes.n2", "Note privée : code 1234.");
        assert!(matches!(bloquer(p.attester(&km, "notes.n2", &octets, 0, T)), Err(ErreurFederee::NonDerivee(_))));
        // Autre donnée au même contenu : l'id imposé diffère.
        km.poser("notes.n3", TEXTE);
        assert_eq!(
            bloquer(p.attester(&km, "notes.n3", &octets, 0, T)),
            Err(ErreurFederee::NonDerivee("id non dérivé de la donnée étiquetée"))
        );
    }

    #[test]
    fn fiche_dont_le_contenu_ne_correspond_pas_refusee() {
        let cle = CleSignature::generer();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        km.poser("notes.n1", TEXTE);
        let p = publieur();
        let base = fiche_derivee(&cle);
        let refusees = [
            // Texte différent (ajout d'une information privée).
            Fiche { texte: format!("{TEXTE} Code 1234."), ..base.clone() },
            // Sujet, relation ou source qui publient une information hors du texte étiqueté.
            Fiche { sujet: "Code d'accès 1234".into(), ..base.clone() },
            Fiche { relations: vec![Relation { sujet: "Alice".into(), predicat: "habite".into(), objet: "Lyon".into() }], ..base.clone() },
            Fiche { sources: vec!["https://exemple.org/alice-privé".into()], ..base.clone() },
            // Octets signés non normalisés (canal caché par invisibles).
            Fiche { texte: "Paris est la\u{200B} capitale de la France.".into(), ..base.clone() },
        ];
        for f in refusees {
            let octets = sceller_fiche(&f, &cle).unwrap();
            assert!(matches!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonDerivee(_))), "{f:?}");
        }
        // Fiche à reprises : jamais par ce chemin.
        let source = verifier_fiche(&sceller_fiche(&base, &cle).unwrap(), T).unwrap();
        let reprise = Fiche { reprises: vec![crate::fiche::Reprise::de(&source)], id: "geo.reprise".into(), ..base.clone() };
        let octets = sceller_fiche(&reprise, &cle).unwrap();
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::ValidationRequise));
        // La Fiche dérivée exacte passe.
        assert!(bloquer(p.attester(&km, "notes.n1", &sceller_fiche(&base, &cle).unwrap(), 0, T)).is_ok());
    }

    /// M1 : champs structurels (id, date, version, type, langue, licence,
    /// audience) liés à l'étiquette ; aucun ne sert de canal ni n'élargit l'audience.
    #[test]
    fn champs_structurels_lies_a_l_etiquette() {
        let cle = CleSignature::generer();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        km.poser("notes.n1", TEXTE);
        let p = publieur();
        let base = fiche_derivee(&cle);
        let autre_cog = crate::fiche::cog_de_cle(&[9; 32]).unwrap();
        let refusees = [
            // id libre (canal d'exfiltration) ou d'une autre donnée.
            Fiche { id: "code.1234".into(), ..base.clone() },
            Fiche { id: id_fiche_derivee(&identite("notes.n2")), ..base.clone() },
            // date hors fenêtre (canal : bits de la date).
            Fiche { date: T - FENETRE_DATE_SECS - 1, ..base.clone() },
            Fiche { date: 1, ..base.clone() },
            // version ≠ dernière + 1 (canal : saut de version).
            Fiche { version: 7, ..base.clone() },
            // type, langue, licence, audience ≠ choix de l'utilisateur.
            Fiche { type_fiche: TypeFiche::Document, ..base.clone() },
            Fiche { langue: "en".into(), ..base.clone() },
            Fiche { licence: "MIT".into(), ..base.clone() },
            Fiche { audience: Audience::Invites, ..base.clone() },
            Fiche { audience: Audience::Nommes(vec![autre_cog]), ..base.clone() },
        ];
        for f in refusees {
            let octets = sceller_fiche(&f, &cle).unwrap();
            assert!(
                matches!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonDerivee(_))),
                "{f:?}"
            );
        }
        // Date au-delà de la fenêtre dans le futur : refusée (DateFuture ou NonDerivee).
        let future = Fiche { date: T + FENETRE_DATE_SECS + 1, ..base.clone() };
        assert!(bloquer(p.attester(&km, "notes.n1", &sceller_fiche(&future, &cle).unwrap(), 0, T)).is_err());
        // Date dans la fenêtre (passé proche) : admise.
        let ok = Fiche { date: T - FENETRE_DATE_SECS, ..base.clone() };
        assert!(bloquer(p.attester(&km, "notes.n1", &sceller_fiche(&ok, &cle).unwrap(), 0, T)).is_ok());
        // Version 2 seulement après la version 1.
        let v2 = sceller_fiche(&derivee(&cle, "notes.n1", 2), &cle).unwrap();
        assert!(matches!(bloquer(p.attester(&km, "notes.n1", &v2, 0, T)), Err(ErreurFederee::NonDerivee(_))));
        assert!(bloquer(p.attester(&km, "notes.n1", &v2, 1, T)).is_ok());
        // Pose sans paramètres (intention v1) ou sans identité : jamais publiable.
        let e = empreinte_contenu(TEXTE);
        km.fixer("notes.n1", EtiquetteLue { parametres: None, ..lue("notes.n1", true, Some(e), Some(e)) });
        let octets = sceller_fiche(&base, &cle).unwrap();
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonPubliable));
        km.fixer("notes.n1", EtiquetteLue { empreinte_donnee: None, ..lue("notes.n1", true, Some(e), Some(e)) });
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::NonPubliable));
    }

    /// M1 : le service de publication revérifie version = dernière + 1.
    #[test]
    fn service_exige_version_suivante() {
        let cle = CleSignature::generer();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        km.poser("notes.n1", TEXTE);
        let p = publieur();
        let mut s = ServiceRequete::new(cog(&cle));
        let v1 = sceller_fiche(&fiche_derivee(&cle), &cle).unwrap();
        // Attestation obtenue en annonçant à tort « dernière = 2 » pour une v3.
        let v3 = sceller_fiche(&derivee(&cle, "notes.n1", 3), &cle).unwrap();
        let triche = bloquer(p.attester(&km, "notes.n1", &v3, 2, T)).unwrap();
        assert_eq!(s.publier(v3, Publication::Originale(triche), T), Err(ErreurFederee::VersionObsolete));
        let c = bloquer(p.attester(&km, "notes.n1", &v1, s.derniere_version(&fiche_derivee(&cle).id), T)).unwrap();
        s.publier(v1, Publication::Originale(c), T).unwrap();
    }

    /// M1 : plafond d'attestations par donnée et par 24 h.
    #[test]
    fn plafond_d_attestations_par_donnee() {
        let cle = CleSignature::generer();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        km.poser("notes.n1", TEXTE);
        let p = publieur();
        let octets = sceller_fiche(&fiche_derivee(&cle), &cle).unwrap();
        for _ in 0..MAX_ATTESTATIONS_PAR_PERIODE {
            assert!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)).is_ok());
        }
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, 0, T)), Err(ErreurFederee::Quota));
        // Autre donnée : plafond distinct.
        km.poser("notes.n4", TEXTE);
        let autre = sceller_fiche(&derivee(&cle, "notes.n4", 1), &cle).unwrap();
        assert!(bloquer(p.attester(&km, "notes.n4", &autre, 0, T)).is_ok());
        // Après 24 h : de nouveau admise (Fiche redatée).
        let t2 = T + PERIODE_ATTESTATIONS_SECS + 1;
        let redatee = sceller_fiche(&Fiche { date: t2, ..fiche_derivee(&cle) }, &cle).unwrap();
        assert!(bloquer(p.attester(&km, "notes.n1", &redatee, 0, t2)).is_ok());
    }
}
