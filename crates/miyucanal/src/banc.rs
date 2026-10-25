//! @id     miyucanal_banc
//! @do     simuler_canal_officiel_essai
//! @role   test
//! @layer  toolkit
//! @human  miyucanal — module « banc » (fonctionnalité `banc-essai` : Origin, éditeur et dépôt simulés pour les tests des installateurs).

//! Banc d'essai du canal officiel (fonctionnalité `banc-essai`, **tests
//! seulement**) : Origin (clé de signature + clé de journal dédiée), éditeur,
//! registre officiel en mémoire et dépôt [`DepotMemoire`].
//!
//! Sert aux tests des installateurs branchés sur le canal (Central,
//! maia-llm) : une [`crate::MiseAJourVerifiee`] ne se construit que par le
//! [`crate::Releveur`], le banc produit donc de vraies annonces 2 sur 2 et de
//! vraies têtes. Clés générées à chaque banc, jamais persistées.

use miyuregistre::{Categorie, Cible, Journal, Publication, StockageMemoire};
use miyusceau::CleSignature;
use sha2::{Digest, Sha256};

use crate::annonce::{sceller_annonce, sceller_revocation};
use crate::releveur::ConfigCanal;
use crate::transport::DepotMemoire;

/// Validité annoncée des têtes émises par le banc (24 h).
pub const VALIDITE_TETE_SECS: u64 = 86_400;

/// Origin, éditeur, registre officiel et dépôt simulés.
pub struct BancCanal {
    registre: [u8; 32],
    origin: CleSignature,
    journal_origin: CleSignature,
    editeur: CleSignature,
    journal: Journal<StockageMemoire>,
    /// Dépôt officiel simulé (transport du relevé).
    pub depot: DepotMemoire,
}

impl std::fmt::Debug for BancCanal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BancCanal").field("taille", &self.journal.taille()).finish_non_exhaustive()
    }
}

impl BancCanal {
    /// Banc neuf pour le registre `registre` (clés aléatoires).
    pub fn new(registre: [u8; 32]) -> Self {
        Self {
            registre,
            origin: CleSignature::generer(),
            journal_origin: CleSignature::generer(),
            editeur: CleSignature::generer(),
            // Stockage en mémoire : l'ouverture ne peut pas échouer.
            journal: Journal::ouvrir(registre, StockageMemoire::new(), StockageMemoire::new())
                .unwrap_or_else(|_| unreachable!()),
            depot: DepotMemoire::new(),
        }
    }

    /// Identifiant du registre officiel.
    pub fn registre(&self) -> [u8; 32] {
        self.registre
    }

    /// Clés publiques (Origin, journal d'Origin, éditeur).
    pub fn cles_publiques(&self) -> ([u8; 32], [u8; 32], [u8; 32]) {
        (self.origin.cle_publique(), self.journal_origin.cle_publique(), self.editeur.cle_publique())
    }

    /// Clé de signature de l'éditeur (révocations, essais de signature unique).
    pub fn editeur(&self) -> &CleSignature {
        &self.editeur
    }

    /// Configuration du canal côté zone COG pour ce banc.
    pub fn config(&self) -> ConfigCanal {
        let (o, j, e) = self.cles_publiques();
        ConfigCanal::nouvelle(self.registre, &o, &j, &e).unwrap_or_else(|_| unreachable!())
    }

    /// Inscrit des octets bruts au registre officiel (entrée quelconque).
    pub fn inscrire(&mut self, sceau: &[u8]) -> u64 {
        self.journal.ajouter(sceau).unwrap_or_else(|e| panic!("inscription refusée : {e}"))
    }

    /// Publie `artefact` (annonce 2 sur 2 inscrite, artefact déposé). Renvoie l'annonce.
    pub fn publier(&mut self, categorie: Categorie, composant: &str, version: u64, artefact: &[u8], t: u64) -> Vec<u8> {
        self.depot.deposer_artefact(artefact.to_vec());
        let publication = Publication {
            categorie,
            composant: composant.to_string(),
            version,
            empreinte: Sha256::digest(artefact).into(),
            taille: artefact.len() as u64,
        };
        let annonce = sceller_annonce(publication, t, &self.origin, &self.editeur)
            .unwrap_or_else(|e| panic!("annonce refusée : {e}"));
        self.inscrire(&annonce);
        annonce
    }

    /// Inscrit une révocation signée par l'éditeur.
    pub fn revoquer(&mut self, cible: Cible, t: u64) {
        let r = sceller_revocation(cible, "essai", t, &[&self.editeur])
            .unwrap_or_else(|e| panic!("révocation refusée : {e}"));
        self.inscrire(&r);
    }

    /// Émet une tête (clé de journal d'Origin) et met le dépôt à jour.
    pub fn emettre(&mut self, t: u64) {
        let tete = self
            .journal
            .emettre_tete(&[&self.journal_origin], t, VALIDITE_TETE_SECS)
            .unwrap_or_else(|e| panic!("tête refusée : {e}"));
        self.depot.deposer_tete(tete);
        self.depot.remplacer_entrees(self.journal.entrees(0, u64::MAX).unwrap_or_else(|_| unreachable!()));
    }

    /// Horodatage de la dernière tête émise.
    pub fn derniere_emission(&self) -> Option<u64> {
        self.journal.derniere_tete().map(|t| t.emise_a)
    }
}
