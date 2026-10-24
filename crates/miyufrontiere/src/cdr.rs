//! @id     miyufrontiere_cdr
//! @do     brancher_cdr_sur_portail
//! @role   integration
//! @layer  operateur
//! @human  miyufrontiere — adaptateur Portail → CDR : chaque champ Document du traité est désarmé par miyucdr (EX-45).

//! # Portail → CDR
//!
//! [`CdrPortail`] implémente `miyuportail::Desarmement` avec `miyucdr` :
//!
//! | nature (traité) | désarmement |
//! |---|---|
//! | `Image` | famille imposée, format **détecté** sur le contenu (PNG, JPEG, GIF, WebP, BMP), puis `desarmer(mime détecté)` ; tout autre contenu refusé |
//! | `Document` | type MIME **déclaré par le traité** obligatoire, `desarmer(mime déclaré)` (le CDR vérifie que le contenu réel lui correspond) |
//! | `Archive` | **refusée** (dépaquetage non prévu, A-004) |
//!
//! La sortie reconstruite doit tenir dans `taille_max` du champ, sinon refus.
//! Le refus rendu au Portail reste **opaque** (aucun détail vers la façade) ;
//! son motif est compté ici ([`CdrPortail::refus`]) pour l'observation.

use std::collections::BTreeMap;
use std::sync::Mutex;

use miyucdr::{detecter, Cdr, Limites, Refus, TypeContenu};
use miyuportail::{Desarmement, NatureDocument, ObjetADesarmer, ObjetDesarme, RefusDesarmement};

/// Motif d'un refus de désarmement (observation locale, jamais transmis à la façade).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MotifRefusCdr {
    /// Nature `Archive` : refusée.
    Archive,
    /// Document sans type MIME déclaré par le traité.
    MimeAbsent,
    /// Image dont le format détecté n'est pas pris en charge.
    ImageNonReconnue,
    /// Type déclaré non pris en charge par le CDR.
    NonPrisEnCharge,
    /// Contenu réel ≠ type déclaré.
    TypeDiscordant,
    /// Entrée trop volumineuse.
    TropVolumineux,
    /// Limites de décodage dépassées.
    LimiteDepassee,
    /// Contenu malformé.
    Malforme,
    /// Délai dépassé.
    Delai,
    /// CDR saturé.
    Sature,
    /// Défaillance interne.
    Interne,
    /// Sortie reconstruite au-delà de `taille_max`.
    SortieTropGrande,
}

impl MotifRefusCdr {
    fn depuis(refus: &Refus) -> Self {
        match refus {
            Refus::NonPrisEnCharge { .. } => Self::NonPrisEnCharge,
            Refus::TypeDiscordant { .. } => Self::TypeDiscordant,
            Refus::TropVolumineux { .. } => Self::TropVolumineux,
            Refus::LimiteDepassee(_) => Self::LimiteDepassee,
            Refus::Malforme(_) => Self::Malforme,
            Refus::DelaiDepasse => Self::Delai,
            Refus::Sature => Self::Sature,
            Refus::Interne(_) => Self::Interne,
        }
    }
}

/// CDR du Portail (`miyucdr`), fermé par défaut.
#[derive(Debug, Default)]
pub struct CdrPortail {
    cdr: Cdr,
    refus: Mutex<BTreeMap<MotifRefusCdr, u64>>,
}

impl CdrPortail {
    /// CDR avec des limites explicites.
    pub fn new(limites: Limites) -> Self {
        Self { cdr: Cdr::new(limites), refus: Mutex::new(BTreeMap::new()) }
    }

    /// Refus comptés par motif.
    pub fn refus(&self) -> BTreeMap<MotifRefusCdr, u64> {
        self.refus.lock().map(|r| r.clone()).unwrap_or_default()
    }

    fn refuser(&self, motif: MotifRefusCdr) -> RefusDesarmement {
        if let Ok(mut r) = self.refus.lock() {
            *r.entry(motif).or_insert(0) += 1;
        }
        RefusDesarmement
    }

    fn type_a_desarmer(&self, objet: &ObjetADesarmer<'_>) -> Result<&'static str, RefusDesarmement> {
        match objet.nature {
            NatureDocument::Archive => Err(self.refuser(MotifRefusCdr::Archive)),
            NatureDocument::Image => TypeContenu::image_depuis_detecte(detecter(objet.octets))
                .map(TypeContenu::mime)
                .ok_or_else(|| self.refuser(MotifRefusCdr::ImageNonReconnue)),
            NatureDocument::Document => {
                let mime = objet.mime.ok_or_else(|| self.refuser(MotifRefusCdr::MimeAbsent))?;
                TypeContenu::depuis_mime(mime.as_str())
                    .map(TypeContenu::mime)
                    .map_err(|e| self.refuser(MotifRefusCdr::depuis(&e)))
            }
        }
    }
}

impl Desarmement for CdrPortail {
    fn desarmer(&self, objet: &ObjetADesarmer<'_>) -> Result<ObjetDesarme, RefusDesarmement> {
        let mime = self.type_a_desarmer(objet)?;
        let reconstruit =
            self.cdr.desarmer_detaille(mime, objet.octets).map_err(|e| self.refuser(MotifRefusCdr::depuis(&e)))?;
        if reconstruit.octets.len() > objet.taille_max {
            return Err(self.refuser(MotifRefusCdr::SortieTropGrande));
        }
        Ok(ObjetDesarme { octets: reconstruit.octets })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use image::{DynamicImage, ImageFormat, RgbImage};
    use miyuportail::{IdService, Sens, TypeMime};

    fn png() -> Vec<u8> {
        let mut o = Vec::new();
        DynamicImage::ImageRgb8(RgbImage::from_pixel(8, 8, image::Rgb([200, 10, 10])))
            .write_to(&mut Cursor::new(&mut o), ImageFormat::Png)
            .unwrap();
        // Données après IEND : jamais recopiées.
        o.extend_from_slice(b"<script>charge cachee</script>");
        o
    }

    fn objet<'a>(
        service: &'a IdService,
        nature: NatureDocument,
        mime: Option<&str>,
        taille_max: usize,
        octets: &'a [u8],
    ) -> ObjetADesarmer<'a> {
        ObjetADesarmer {
            service,
            sens: Sens::Entrante,
            type_code: 10,
            tag: 3,
            nature,
            mime: mime.map(|m| TypeMime::new(m).unwrap()),
            taille_max,
            octets,
        }
    }

    #[test]
    fn image_format_detecte_et_reconstruite() {
        let s = IdService::new("docs").unwrap();
        let cdr = CdrPortail::default();
        let entree = png();
        let sortie = cdr.desarmer(&objet(&s, NatureDocument::Image, None, 65_536, &entree)).unwrap();
        assert!(sortie.octets.starts_with(b"\x89PNG"));
        assert!(!sortie.octets.windows(8).any(|w| w == b"<script>"));
        // Contenu non image sous la nature Image : refusé.
        assert_eq!(cdr.desarmer(&objet(&s, NatureDocument::Image, None, 65_536, b"bonjour")), Err(RefusDesarmement));
        assert_eq!(cdr.refus().get(&MotifRefusCdr::ImageNonReconnue), Some(&1));
    }

    #[test]
    fn document_exige_le_type_declare() {
        let s = IdService::new("docs").unwrap();
        let cdr = CdrPortail::default();
        let texte = "ligne\u{202e} 1\r\nligne 2".as_bytes();
        let r = cdr.desarmer(&objet(&s, NatureDocument::Document, Some("text/plain"), 1024, texte)).unwrap();
        assert_eq!(r.octets, "ligne 1\nligne 2".as_bytes());
        assert_eq!(cdr.desarmer(&objet(&s, NatureDocument::Document, None, 1024, texte)), Err(RefusDesarmement));
        assert_eq!(
            cdr.desarmer(&objet(&s, NatureDocument::Document, Some("application/pdf"), 1024, b"%PDF-1.7")),
            Err(RefusDesarmement)
        );
        // Type déclaré JSON, contenu PNG : discordant.
        let image = png();
        assert_eq!(
            cdr.desarmer(&objet(&s, NatureDocument::Document, Some("application/json"), 65_536, &image)),
            Err(RefusDesarmement)
        );
        let refus = cdr.refus();
        assert_eq!(refus.get(&MotifRefusCdr::MimeAbsent), Some(&1));
        assert_eq!(refus.get(&MotifRefusCdr::NonPrisEnCharge), Some(&1));
        assert_eq!(refus.get(&MotifRefusCdr::TypeDiscordant), Some(&1));
    }

    #[test]
    fn archive_refusee_et_sortie_bornee() {
        let s = IdService::new("docs").unwrap();
        let cdr = CdrPortail::default();
        assert_eq!(cdr.desarmer(&objet(&s, NatureDocument::Archive, None, 1024, b"PK\x03\x04")), Err(RefusDesarmement));
        // Sortie reconstruite (PNG réencodé) plus grande que la borne du champ.
        let image = png();
        assert_eq!(cdr.desarmer(&objet(&s, NatureDocument::Image, None, 16, &image)), Err(RefusDesarmement));
        let refus = cdr.refus();
        assert_eq!(refus.get(&MotifRefusCdr::Archive), Some(&1));
        assert_eq!(refus.get(&MotifRefusCdr::SortieTropGrande), Some(&1));
    }
}
