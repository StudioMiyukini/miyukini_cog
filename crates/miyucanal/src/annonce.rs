//! @id     miyucanal_annonce
//! @do     verifier_annonce_deux_sur_deux
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — module « annonce » (annonces de mise à jour 2 sur 2 Origin + éditeur, révocations).

//! Annonces du canal officiel (EX-01, EX-26).
//!
//! Une **annonce** est une entrée de registre ([`miyuregistre::EntreeRegistre`],
//! Sceau `0x04`) de nature `Publication`, signée **2 sur 2** par l'ensemble
//! exact {Origin, clé éditeur hors ligne} : les ancres de vérification ne
//! contiennent que ces deux clés, un troisième signataire (même épinglé
//! ailleurs) n'est jamais compté.
//!
//! Une **révocation** du canal est acceptée signée par **l'un ou l'autre**
//! (seuil 1 sur les mêmes ancres) : elle ne peut qu'empêcher une
//! installation (décision du propriétaire, amendement A-006). La clé de
//! journal d'Origin (têtes du registre) n'est pas une ancre des entrées.
//!
//! Signatures vérifiées **avant** tout décodage de la charge (`verifier_seuil`).

use miyuregistre::{identite_sceau, Cible, Corps, EntreeRegistre, Publication};
use miyusceau::{
    sceller_multi, verifier_seuil, AncresDeConfiance, CleSignature, ErreurSceau, Kid, NiveauConfiance,
    TOLERANCE_HORLOGE_SECS,
};

use crate::erreurs::ErreurCanal;

/// Ancres du canal officiel : exactement {Origin, éditeur}.
#[derive(Debug, Clone)]
pub struct AncresCanal {
    ancres: AncresDeConfiance,
    origin: Kid,
    editeur: Kid,
}

impl AncresCanal {
    /// Épingle Origin et la clé éditeur (clés distinctes exigées).
    pub fn nouvelles(origin: &[u8; 32], editeur: &[u8; 32]) -> Result<Self, ErreurCanal> {
        if origin == editeur {
            return Err(ErreurCanal::Configuration("Origin et l'éditeur doivent avoir des clés distinctes"));
        }
        let mut ancres = AncresDeConfiance::new();
        let origin = ancres.ajouter(origin, NiveauConfiance::Reconnu)?;
        let editeur = ancres.ajouter(editeur, NiveauConfiance::Reconnu)?;
        Ok(Self { ancres, origin, editeur })
    }

    /// Ancres (deux clés).
    pub fn ancres(&self) -> &AncresDeConfiance {
        &self.ancres
    }

    /// `kid` d'Origin.
    pub fn origin(&self) -> &Kid {
        &self.origin
    }

    /// `kid` de l'éditeur.
    pub fn editeur(&self) -> &Kid {
        &self.editeur
    }
}

/// Annonce vérifiée 2 sur 2. Seul [`verifier_annonce`] en construit une.
#[derive(Debug, Clone)]
pub struct AnnonceVerifiee {
    publication: Publication,
    inscrite_a: u64,
    identite: [u8; 32],
    signataires: Vec<Kid>,
    sceau: Vec<u8>,
}

impl AnnonceVerifiee {
    /// Publication annoncée.
    pub fn publication(&self) -> &Publication {
        &self.publication
    }

    /// Horodatage d'inscription signé.
    pub fn inscrite_a(&self) -> u64 {
        self.inscrite_a
    }

    /// Identité de l'annonce (SHA-256 des octets signés).
    pub fn identite(&self) -> &[u8; 32] {
        &self.identite
    }

    /// Signataires comptés (Origin et éditeur).
    pub fn signataires(&self) -> &[Kid] {
        &self.signataires
    }

    /// Octets du sceau.
    pub fn sceau(&self) -> &[u8] {
        &self.sceau
    }

    /// Vrai si l'annonce est visée par l'une des révocations.
    pub fn est_revoquee<'a>(&self, mut revocations: impl Iterator<Item = &'a Cible>) -> bool {
        revocations.any(|c| match c {
            Cible::Annonce(id) => id == &self.identite,
            Cible::Artefact(h) => h == &self.publication.empreinte,
            Cible::Cle(kid) => self.signataires.contains(kid),
            Cible::Serie(_) => false,
        })
    }
}

/// Vérifie une annonce : 2 sur 2 {Origin, éditeur} puis nature `Publication`,
/// inscription non datée dans le futur.
pub fn verifier_annonce(octets: &[u8], ancres: &AncresCanal, maintenant: u64) -> Result<AnnonceVerifiee, ErreurCanal> {
    let verifie = verifier_seuil::<EntreeRegistre>(octets, &ancres.ancres, 2, maintenant)?;
    let signataires = verifie.signataires().to_vec();
    let entree = verifie.into_titre();
    let Corps::Publication(publication) = entree.corps else {
        return Err(ErreurSceau::ValeurInvalide(1).into());
    };
    if entree.inscrite_a > maintenant.saturating_add(TOLERANCE_HORLOGE_SECS) {
        return Err(ErreurSceau::PasEncoreValide.into());
    }
    Ok(AnnonceVerifiee {
        publication,
        inscrite_a: entree.inscrite_a,
        identite: identite_sceau(octets)?,
        signataires,
        sceau: octets.to_vec(),
    })
}

/// Entrée du canal vérifiée au seuil 1 (Origin **ou** éditeur) : sert à
/// trier les entrées du registre officiel avant tout traitement.
pub fn verifier_entree_canal(octets: &[u8], ancres: &AncresCanal, maintenant: u64) -> Result<EntreeRegistre, ErreurCanal> {
    Ok(verifier_seuil::<EntreeRegistre>(octets, &ancres.ancres, 1, maintenant)?.into_titre())
}

/// Outillage (Origin + éditeur) : scelle une annonce 2 sur 2.
pub fn sceller_annonce(
    publication: Publication,
    inscrite_a: u64,
    origin: &CleSignature,
    editeur: &CleSignature,
) -> Result<Vec<u8>, ErreurCanal> {
    let entree = EntreeRegistre { inscrite_a, corps: Corps::Publication(publication) };
    Ok(sceller_multi(&entree, &[origin, editeur])?)
}

/// Outillage : scelle une révocation (une ou deux clés du canal).
pub fn sceller_revocation(
    cible: Cible,
    motif: &str,
    inscrite_a: u64,
    cles: &[&CleSignature],
) -> Result<Vec<u8>, ErreurCanal> {
    let entree = EntreeRegistre { inscrite_a, corps: Corps::Revocation { cible, motif: motif.to_string() } };
    Ok(sceller_multi(&entree, cles)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuregistre::Categorie;

    fn publication() -> Publication {
        Publication { categorie: Categorie::Os, composant: "noyau-linux".into(), version: 2, empreinte: [1; 32], taille: 3 }
    }

    #[test]
    fn deux_sur_deux_exige() {
        let origin = CleSignature::generer();
        let editeur = CleSignature::generer();
        let intrus = CleSignature::generer();
        let ancres = AncresCanal::nouvelles(&origin.cle_publique(), &editeur.cle_publique()).unwrap();

        let ok = sceller_annonce(publication(), 10, &origin, &editeur).unwrap();
        let a = verifier_annonce(&ok, &ancres, 10).unwrap();
        assert_eq!(a.signataires().len(), 2);
        assert_eq!(a.publication(), &publication());

        let entree = EntreeRegistre { inscrite_a: 10, corps: Corps::Publication(publication()) };
        let seule = sceller_multi(&entree, &[&origin]).unwrap();
        assert!(matches!(
            verifier_annonce(&seule, &ancres, 10),
            Err(ErreurCanal::Sceau(ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 }))
        ));
        let avec_intrus = sceller_multi(&entree, &[&origin, &intrus]).unwrap();
        assert!(verifier_annonce(&avec_intrus, &ancres, 10).is_err());
        // Signature altérée d'un signataire épinglé : refus du sceau entier.
        let mut altere = ok.clone();
        let n = altere.len();
        altere[n - 1] ^= 1;
        assert!(verifier_annonce(&altere, &ancres, 10).is_err());
        // Inscription datée dans le futur.
        let futur = sceller_annonce(publication(), 10_000, &origin, &editeur).unwrap();
        assert!(verifier_annonce(&futur, &ancres, 10).is_err());
        assert!(AncresCanal::nouvelles(&origin.cle_publique(), &origin.cle_publique()).is_err());
    }

    #[test]
    fn revocations_visent_annonce_artefact_ou_cle() {
        let origin = CleSignature::generer();
        let editeur = CleSignature::generer();
        let ancres = AncresCanal::nouvelles(&origin.cle_publique(), &editeur.cle_publique()).unwrap();
        let a = verifier_annonce(&sceller_annonce(publication(), 1, &origin, &editeur).unwrap(), &ancres, 1).unwrap();
        assert!(!a.est_revoquee([Cible::Serie([0; 32])].iter()));
        assert!(a.est_revoquee([Cible::Annonce(*a.identite())].iter()));
        assert!(a.est_revoquee([Cible::Artefact([1; 32])].iter()));
        assert!(a.est_revoquee([Cible::Cle(editeur.kid())].iter()));
        let r = sceller_revocation(Cible::Artefact([1; 32]), "faille", 2, &[&editeur]).unwrap();
        assert!(verifier_entree_canal(&r, &ancres, 2).is_ok());
    }
}
