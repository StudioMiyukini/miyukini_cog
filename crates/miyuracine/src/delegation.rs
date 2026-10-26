//! @id     miyuracine_delegation
//! @do     deleguer_cles_usage
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — délégations 0x45 des clés d'usage, signées par l'identité pendant la cérémonie (A-022 §2).

//! Délégation de clé d'usage (A-022 §2) : pendant la cérémonie hors ligne,
//! la clé d'**identité** (indice 0) signe, pour chaque clé d'usage d'indice
//! ≥ 1 et de génération `g`, un certificat `0x06` de nature `0x45`
//! ([`miyusceau::DelegationCle`]) : `cog_id`, indice, génération, clé
//! publique dérivée, fenêtre de validité (≤ 90 j).
//!
//! Tout vérificateur remonte ainsi de la clé d'usage à la racine épinglée ;
//! la clé d'identité ne quitte jamais la cérémonie (A-022 §4).

use miyusceau::{verifier_delegation, DelegationCle, NiveauConfiance};

use crate::erreurs::ErreurRacine;
use crate::racine::Racine;
use crate::usages::Usage;

impl Racine {
    /// Produit le sceau de délégation `0x45` de la clé `usage`/`generation`,
    /// valide de `pas_avant` à `pas_apres` (au plus 90 j), signé par
    /// l'identité de cette racine.
    ///
    /// Contrôle avant de rendre les octets : le sceau se vérifie contre la
    /// clé d'identité (fenêtre comprise, à l'instant `pas_avant`) et certifie
    /// exactement la clé dérivée. Refus pour l'identité elle-même (elle ne se
    /// délègue pas).
    pub fn deleguer(
        &self,
        usage: Usage,
        generation: u32,
        pas_avant: u64,
        pas_apres: u64,
    ) -> Result<Vec<u8>, ErreurRacine> {
        if usage == Usage::Identite {
            return Err(ErreurRacine::DelegationIdentite);
        }
        let cle = self.cle_publique(usage, generation)?;
        let delegation = DelegationCle::nouvelle(
            &self.cog_id(),
            usage.indice(),
            generation,
            cle,
            pas_avant,
            pas_apres,
        )?;
        let identite = self.cle(Usage::Identite, 0)?;
        let sceau = delegation.sceller(&identite)?;
        let relue = verifier_delegation(
            &sceau,
            &identite.cle_publique(),
            NiveauConfiance::Souverain,
            pas_avant,
        )?
        .into_titre();
        if relue != delegation {
            return Err(ErreurRacine::Sceau(
                miyusceau::ErreurSceau::EncodageNonCanonique,
            ));
        }
        Ok(sceau)
    }
}
