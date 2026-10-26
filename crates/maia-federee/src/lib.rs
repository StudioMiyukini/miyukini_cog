//! @id     maia_federee_lib
//! @do     declarer_modules_ia_federee
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — IA fédérée : Fiches signées, double MAIA, marquage etranger, corroboration, requêtes déterministes.

//! # MAIA fédérée — le web pour IA entre COG (lot 7)
//!
//! Spécification : `docs/architecture/IA-FEDEREE.md` (EX-57 à EX-63),
//! `docs/architecture/MEMOIRE-MAIA.md` §10, `docs/implementation/PLAN-IMPLEMENTATION.md` §9.
//!
//! Principe : aucun modèle n'est immunisé contre l'injection ; la défense est
//! **architecturale** — une injection peut arriver, elle ne peut pas **agir**.
//!
//! - [`fiche`] : titre Sceau v1.1 de type `0x09` ([`miyusceau::TypeSceau::Fiche`]),
//!   champs typés, aucun champ de consignes, tout champ inconnu refusé (EX-57) ;
//!   signé par la clé d'usage `fiches` (indice 6) qui porte sa délégation
//!   `0x45` ([`SignataireFiches`], A-022 §4, A-024), Fiches signées par
//!   l'identité encore lisibles (transition)
//! - [`normalisation`] : invisibles, contrôles bidi, homoglyphes, balisage,
//!   sauts de mise en forme (EX-57)
//! - [`base`] : base de connaissances **étrangère**, séparée de la mémoire de
//!   MAIA ; émetteur admis par la vue fédérale (invité ou membre)
//! - [`plongement`] : plongements toujours **recalculés localement** (EX-62)
//! - [`requete`] : requête typée et service de requête **déterministe** — une
//!   IA étrangère n'interroge jamais MAIA (EX-58)
//! - [`lecteur`] : MAIA-Lecteur, **sans outil**, renvoie des extraits typés
//!   ancrés dans la Fiche signée (EX-59)
//! - [`corroboration`] : « établi » si ≥ *k* membres distincts concordent,
//!   plafond d'extraits par source (EX-61)
//! - [`consultation`] : double MAIA — le Planificateur ne voit **jamais** le
//!   contenu étranger ; réponse sourcée rendue de façon déterministe (EX-59)
//! - [`etiquettes`] : seul producteur de [`Publiable`] — lit l'étiquette
//!   KindMother `publiable` de la donnée source exacte et vérifie que la Fiche
//!   en est dérivée (règle stricte, EX-47)
//! - [`reprise`] : règle de dérivation stricte d'une republication (A-009) :
//!   contenu, identifiant et `reprises` calculés depuis les sources vérifiées
//! - [`marquage`] : `etranger` propagé ; aucune action, publication, mémoire
//!   longue sans validation ; aucune Règle ni apprentissage (EX-60, EX-63)

pub mod base;
pub mod consultation;
pub mod corroboration;
pub mod erreurs;
pub mod etiquettes;
pub mod fiche;
pub mod lecteur;
pub mod marquage;
pub mod normalisation;
pub mod plongement;
pub mod reprise;
pub mod requete;

pub use base::{BaseEtrangere, Qualite, VueFederale, MAX_FICHES_PAR_EMETTEUR};
pub use consultation::{consulter, Consultation, Plan, Planificateur, ReponseSourcee};
pub use corroboration::{corroborer, Affirmation, Etablissement, PolitiqueCorroboration, Source, K_DEFAUT, PLAFOND_DEFAUT};
pub use erreurs::ErreurFederee;
pub use etiquettes::{
    empreinte_contenu, id_fiche_derivee, verifier_derivation, EtiquetteLue, LecteurEtiquettes, ParametresLus,
    PublieurOriginal, FENETRE_DATE_SECS, MAX_ATTESTATIONS_PAR_PERIODE, PERIODE_ATTESTATIONS_SECS, PREFIXE_ID_FICHE,
};
pub use fiche::{
    cog_de_cle, sceller_fiche, verifier_fiche, verifier_fiche_deleguee, Audience, Fiche, FicheVerifiee, Relation, Reprise,
    SignataireFiches, SignatureFiche, TypeFiche, INDICE_FICHES, MAX_DELEGATION, TYPE_FICHE,
};
pub use lecteur::{extraire, Ancrage, ExtraitType, ModeleLecteur};
pub use marquage::{autoriser, AutoriteConfirmation, Effet, Etranger, Provenance, ValidationUtilisateur};
pub use plongement::PlongementLocal;
pub use reprise::{
    deriver_reprise, empreinte_ensemble, id_fiche_reprise, verifier_derivation_reprise, ChoixReprise, PREFIXE_ID_REPRISE,
};
pub use requete::{
    AutoriteEtiquettes, Publiable, Publication, RequeteTypee, ServiceRequete, PERIODE_QUOTA_SECS, QUOTA_REQUETES,
    VALIDITE_PUBLIABLE_SECS,
};
