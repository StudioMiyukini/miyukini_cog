//! @id     miyuvigie_lib
//! @do     declarer_modules_vigies
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — Vigies : capteurs IDS « salle blanche » sous WorrySentinel (lot 3).

//! # MiyuVigie — Vigies (capteurs IDS)
//!
//! Lot 3 « Frontières » (`docs/implementation/PLAN-IMPLEMENTATION.md` §5) :
//! « Vigies : capteurs IDS, signaux vers CaringNanny → WorrySentinel ; sortie de
//! zone COG = incident critique (EX-12) ». Modèle :
//! `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §12 (les Cores gouvernent,
//! des opérateurs exécutent : Vigies sous WorrySentinel, décision D1).
//!
//! Moteur **salle blanche** en Rust pur, sans dépendance (aucun code Suricata /
//! Snort, C-71). Entrées :
//!
//! - [`SourceEvenements`] → [`EvenementFlux`] : événements de la Douane (adaptateur) ;
//! - [`LecteurNftables`] : journaux nftables au préfixe `MIYU:` (analyse stricte) ;
//! - [`ControleSortie`] / [`analyser_proc_net_tcp`] : connexions de la zone COG
//!   (toute destination hors boucle locale et hors Portail = critique, EX-12).
//!
//! Sortie : [`LotVigie`] (signaux typés, pouls, compteurs), ingéré par
//! CaringNanny puis propagé à WorrySentinel.
//!
//! Les Vigies **détectent** : elles ne bloquent rien et ne décident rien
//! (la coupure d'une voie est une décision de WorrySentinel, EX-49).
//! Aucune horloge n'est lue : le temps est fourni par la source ou l'appelant.

#![forbid(unsafe_code)]

pub mod dedup;
pub mod evenement;
pub mod nftables;
pub mod regles;
pub mod signal;
pub mod sortie;
pub mod vigie;
pub mod zone;

pub use dedup::{Admission, Dedoublonneur};
pub use evenement::{ErreurSource, EvenementFlux, Protocole, SourceEvenements, Verdict};
pub use nftables::{LecteurNftables, ResultatLigne, MARQUEUR_PREFIXE};
pub use regles::{
    Condition, Declenchement, ErreurRegles, JeuRegles, Regle, VerificateurJeuRegles, REGLE_ENTREE,
    REGLE_INDETERMINE, REGLE_SORTIE,
};
pub use signal::{
    CapteurId, Categorie, Compteurs, EtatCapteur, EvenementId, LotVigie, Pouls, RegleId, Severite,
    Signal, VoieId,
};
pub use sortie::{
    analyser_proc_net_tcp, ControleSortie, ExutoirePortail, LigneProc, TentativeConnexion,
    VerdictSortie,
};
pub use vigie::{ConfigVigie, Vigie, REGLE_CAPTEUR_DEGRADE, REGLE_LATERAL};
pub use zone::{est_boucle_locale, PlanAdressage, Prefixe, Zone};
