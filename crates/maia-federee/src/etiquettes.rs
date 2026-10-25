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
//! Champs structurels non couverts (choisis par l'utilisateur dans sa
//! demande) : `id`, `type`, `date`, `version`, `langue`, `licence`, `audience`.

use std::future::Future;

use crate::erreurs::ErreurFederee;
use crate::fiche::{verifier_fiche, FicheVerifiee};
use crate::normalisation::cle_comparaison;
use crate::requete::{AutoriteEtiquettes, Publiable};

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
    /// formée est traitée comme absente (fail-closed).
    pub fn depuis_hex(nationale: bool, posee: bool, empreinte_contenu: Option<&str>, empreinte_actuelle: Option<&str>) -> Self {
        Self {
            nationale,
            posee,
            empreinte_contenu: empreinte_contenu.and_then(hex32),
            empreinte_actuelle: empreinte_actuelle.and_then(hex32),
        }
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

/// Règle de dérivation stricte (voir la documentation du module).
pub fn verifier_derivation(fiche: &FicheVerifiee, etiquette: &EtiquetteLue) -> Result<(), ErreurFederee> {
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
    Ok(())
}

/// Seul producteur de [`Publiable`] : détient l'[`AutoriteEtiquettes`].
#[derive(Debug)]
pub struct PublieurOriginal {
    autorite: AutoriteEtiquettes,
}

impl PublieurOriginal {
    /// Publieur détenteur de l'autorité (obtenue par [`AutoriteEtiquettes::instituer`]).
    pub fn new(autorite: AutoriteEtiquettes) -> Self {
        Self { autorite }
    }

    /// Atteste que les octets signés `octets` sont publiables : étiquette de
    /// `donnee` lue **maintenant** auprès de KindMother, puis règle de dérivation.
    pub async fn attester<L: LecteurEtiquettes>(
        &self,
        lecteur: &L,
        donnee: &L::Donnee,
        octets: &[u8],
        maintenant: u64,
    ) -> Result<Publiable, ErreurFederee> {
        let fiche = verifier_fiche(octets, maintenant)?;
        if fiche.est_reprise() {
            return Err(ErreurFederee::ValidationRequise);
        }
        let etiquette = lecteur.lire(donnee).await?;
        if let Err(e) = verifier_derivation(&fiche, &etiquette) {
            tracing::warn!(target: "maia_federee::audit", fiche = %fiche.fiche().id, motif = %e, "publication originale refusée");
            return Err(e);
        }
        tracing::info!(target: "maia_federee::audit", fiche = %fiche.fiche().id, "Fiche originale attestée publiable (étiquette de l'utilisateur)");
        Publiable::attestee(&self.autorite, &fiche, maintenant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fiche::essais::fiche;
    use crate::fiche::{sceller_fiche, Fiche, Relation};
    use crate::requete::{Publication, ServiceRequete, VALIDITE_PUBLIABLE_SECS};
    use miyusceau::CleSignature;
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

    /// KindMother factice : état d'étiquette par identifiant de donnée.
    struct KindMotherFactice(Mutex<Vec<(&'static str, EtiquetteLue)>>);

    impl KindMotherFactice {
        fn poser(&self, id: &'static str, texte: &str) {
            let e = empreinte_contenu(texte);
            self.fixer(id, EtiquetteLue { nationale: id.starts_with("maia-"), posee: true, empreinte_contenu: Some(e), empreinte_actuelle: Some(e) });
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
            std::future::ready(Ok(self.0.lock().unwrap().iter().find(|(i, _)| *i == donnee).map_or(
                EtiquetteLue { nationale: donnee.starts_with("maia-"), posee: false, empreinte_contenu: None, empreinte_actuelle: None },
                |(_, e)| e.clone(),
            )))
        }
    }

    fn publieur() -> PublieurOriginal {
        PublieurOriginal::new(AutoriteEtiquettes::pour_essais())
    }

    fn fiche_derivee(cle: &CleSignature) -> Fiche {
        Fiche {
            sujet: "capitale de la France".into(),
            sources: Vec::new(),
            ..fiche(cle, "geo.paris", TEXTE, &[("Paris", "capitale de", "France")])
        }
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
    }

    #[test]
    fn etiquetee_par_l_utilisateur_publiable_puis_retrait() {
        let cle = CleSignature::generer();
        let octets = sceller_fiche(&fiche_derivee(&cle), &cle).unwrap();
        let km = KindMotherFactice(Mutex::new(Vec::new()));
        let p = publieur();
        // Non étiquetée : refus.
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, T)), Err(ErreurFederee::NonPubliable));
        // Étiquetée par l'utilisateur : publication possible.
        km.poser("notes.n1", TEXTE);
        let capacite = bloquer(p.attester(&km, "notes.n1", &octets, T)).unwrap();
        let mut s = ServiceRequete::new(cog(&cle));
        s.publier(octets.clone(), Publication::Originale(capacite), T + 1).unwrap();
        // Retrait : plus de nouvelle publication (nouvelle version refusée).
        km.retirer("notes.n1");
        let v2 = sceller_fiche(&Fiche { version: 2, ..fiche_derivee(&cle) }, &cle).unwrap();
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &v2, T)), Err(ErreurFederee::NonPubliable));
        // Capacité attestée avant le retrait : périmée après VALIDITE_PUBLIABLE_SECS.
        km.poser("notes.n1", TEXTE);
        let ancienne = bloquer(p.attester(&km, "notes.n1", &v2, T)).unwrap();
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
        assert!(matches!(bloquer(p.attester(&km, "maia-alice.n1", &octets, T)), Err(ErreurFederee::Interdit(_))));
        // Contenu courant modifié depuis la pose.
        let e = empreinte_contenu(TEXTE);
        km.fixer("notes.n1", EtiquetteLue { nationale: false, posee: true, empreinte_contenu: Some(e), empreinte_actuelle: Some(empreinte_contenu("autre")) });
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, T)), Err(ErreurFederee::NonPubliable));
        // Étiquette d'une autre donnée (autre contenu) : la Fiche n'en dérive pas.
        km.poser("notes.n2", "Note privée : code 1234.");
        assert!(matches!(bloquer(p.attester(&km, "notes.n2", &octets, T)), Err(ErreurFederee::NonDerivee(_))));
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
            assert!(matches!(bloquer(p.attester(&km, "notes.n1", &octets, T)), Err(ErreurFederee::NonDerivee(_))), "{f:?}");
        }
        // Fiche à reprises : jamais par ce chemin.
        let source = verifier_fiche(&sceller_fiche(&base, &cle).unwrap(), T).unwrap();
        let reprise = Fiche { reprises: vec![crate::fiche::Reprise::de(&source)], id: "geo.reprise".into(), ..base.clone() };
        let octets = sceller_fiche(&reprise, &cle).unwrap();
        assert_eq!(bloquer(p.attester(&km, "notes.n1", &octets, T)), Err(ErreurFederee::ValidationRequise));
        // La Fiche dérivée exacte passe.
        assert!(bloquer(p.attester(&km, "notes.n1", &sceller_fiche(&base, &cle).unwrap(), T)).is_ok());
    }
}
