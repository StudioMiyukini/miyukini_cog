//! @id     miyusceau_tlv
//! @do     encoder_decoder_tlv_canonique
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « tlv ».

//! Encodage TLV canonique : `tag (1 o) || longueur (4 o, big-endian) || valeur`.
//!
//! Règles (une seule représentation valide) :
//! - tags strictement croissants (ni doublon, ni désordre) ;
//! - tout tag non lu par le décodeur du titre est un refus ;
//! - entiers sur exactement 8 octets, clés/empreintes sur exactement 32 octets,
//!   texte en UTF-8 valide.

use crate::erreurs::ErreurSceau;

/// Écrivain TLV : impose l'ordre strictement croissant des tags.
#[derive(Debug, Default)]
pub struct EcrivainTlv {
    tampon: Vec<u8>,
    dernier: Option<u8>,
}

impl EcrivainTlv {
    /// Nouvel écrivain vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Champ brut. Erreur si `tag` n'est pas supérieur au précédent.
    pub fn octets(&mut self, tag: u8, valeur: &[u8]) -> Result<&mut Self, ErreurSceau> {
        if self.dernier.is_some_and(|d| tag <= d) {
            return Err(ErreurSceau::TagsNonCroissants);
        }
        let longueur = u32::try_from(valeur.len()).map_err(|_| ErreurSceau::ChargeTropGrande)?;
        self.tampon.push(tag);
        self.tampon.extend_from_slice(&longueur.to_be_bytes());
        self.tampon.extend_from_slice(valeur);
        self.dernier = Some(tag);
        Ok(self)
    }

    /// Entier non signé sur 8 octets.
    pub fn u64(&mut self, tag: u8, valeur: u64) -> Result<&mut Self, ErreurSceau> {
        self.octets(tag, &valeur.to_be_bytes())
    }

    /// Texte UTF-8.
    pub fn texte(&mut self, tag: u8, valeur: &str) -> Result<&mut Self, ErreurSceau> {
        self.octets(tag, valeur.as_bytes())
    }

    /// Valeur de 32 octets (clé publique, empreinte).
    pub fn b32(&mut self, tag: u8, valeur: &[u8; 32]) -> Result<&mut Self, ErreurSceau> {
        self.octets(tag, valeur)
    }

    /// Octets encodés.
    pub fn finir(self) -> Vec<u8> {
        self.tampon
    }
}

/// Valide la structure complète d'une charge avant tout décodage : chaque
/// champ est entier (pas de débordement) et les tags sont strictement
/// croissants. Défense en profondeur : ne dépend pas du décodeur du titre.
pub(crate) fn valider_structure(donnees: &[u8]) -> Result<(), ErreurSceau> {
    let mut position = 0usize;
    let mut dernier: Option<u8> = None;
    while position < donnees.len() {
        let tag = donnees[position];
        if dernier.is_some_and(|d| tag <= d) {
            return Err(ErreurSceau::TagsNonCroissants);
        }
        let entete = donnees.get(position + 1..position + 5).ok_or(ErreurSceau::ChampTronque)?;
        let longueur = u32::from_be_bytes(entete.try_into().map_err(|_| ErreurSceau::ChampTronque)?) as usize;
        position = (position + 5).checked_add(longueur).ok_or(ErreurSceau::ChampTronque)?;
        if position > donnees.len() {
            return Err(ErreurSceau::ChampTronque);
        }
        dernier = Some(tag);
    }
    Ok(())
}

/// Lecteur TLV : les champs doivent être demandés dans l'ordre croissant des tags.
#[derive(Debug)]
pub struct LecteurTlv<'a> {
    donnees: &'a [u8],
    position: usize,
    dernier: Option<u8>,
}

impl<'a> LecteurTlv<'a> {
    /// Lecteur sur une charge utile.
    pub fn new(donnees: &'a [u8]) -> Self {
        Self { donnees, position: 0, dernier: None }
    }

    /// Tag du prochain champ sans le consommer (et contrôle d'ordre).
    fn prochain_tag(&self) -> Result<Option<u8>, ErreurSceau> {
        let Some(&tag) = self.donnees.get(self.position) else {
            return Ok(None);
        };
        if self.dernier.is_some_and(|d| tag <= d) {
            return Err(ErreurSceau::TagsNonCroissants);
        }
        Ok(Some(tag))
    }

    /// Champ optionnel `tag`. Un champ de tag inférieur non demandé est inconnu.
    pub fn optionnel(&mut self, tag: u8) -> Result<Option<&'a [u8]>, ErreurSceau> {
        match self.prochain_tag()? {
            None => Ok(None),
            Some(t) if t < tag => Err(ErreurSceau::ChampInconnu(t)),
            Some(t) if t > tag => Ok(None),
            Some(_) => {
                let debut = self.position + 5;
                let entete = self.donnees.get(self.position + 1..debut).ok_or(ErreurSceau::ChampTronque)?;
                let longueur = u32::from_be_bytes(entete.try_into().map_err(|_| ErreurSceau::ChampTronque)?) as usize;
                let fin = debut.checked_add(longueur).ok_or(ErreurSceau::ChampTronque)?;
                let valeur = self.donnees.get(debut..fin).ok_or(ErreurSceau::ChampTronque)?;
                self.position = fin;
                self.dernier = Some(tag);
                Ok(Some(valeur))
            }
        }
    }

    /// Champ obligatoire.
    pub fn exiger(&mut self, tag: u8) -> Result<&'a [u8], ErreurSceau> {
        self.optionnel(tag)?.ok_or(ErreurSceau::ChampManquant(tag))
    }

    /// Entier obligatoire (exactement 8 octets).
    pub fn exiger_u64(&mut self, tag: u8) -> Result<u64, ErreurSceau> {
        let v = self.exiger(tag)?;
        Ok(u64::from_be_bytes(v.try_into().map_err(|_| ErreurSceau::ValeurInvalide(tag))?))
    }

    /// Texte obligatoire (UTF-8 valide).
    pub fn exiger_texte(&mut self, tag: u8) -> Result<&'a str, ErreurSceau> {
        let v = self.exiger(tag)?;
        std::str::from_utf8(v).map_err(|_| ErreurSceau::ValeurInvalide(tag))
    }

    /// Valeur obligatoire de 32 octets.
    pub fn exiger_b32(&mut self, tag: u8) -> Result<[u8; 32], ErreurSceau> {
        let v = self.exiger(tag)?;
        v.try_into().map_err(|_| ErreurSceau::ValeurInvalide(tag))
    }

    /// Vérifie que toute la charge a été consommée.
    pub fn terminer(self) -> Result<(), ErreurSceau> {
        match self.donnees.get(self.position) {
            None => Ok(()),
            Some(&t) => Err(ErreurSceau::ChampInconnu(t)),
        }
    }
}
