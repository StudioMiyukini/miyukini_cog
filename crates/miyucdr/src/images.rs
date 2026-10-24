//! @id     miyucdr_images
//! @do     reencoder_images_sans_metadonnees
//! @role   securite
//! @layer  toolkit
//! @human  Images : décodage complet borné puis réencodage PNG/JPEG, métadonnées retirées.

//! Seuls les pixels traversent : le réencodeur n'écrit ni EXIF, ni XMP, ni
//! profil ICC, ni bloc texte, ni commentaire. L'orientation EXIF est appliquée
//! aux pixels avant d'être écartée.

use std::io::Cursor;

use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{DynamicImage, ImageDecoder, ImageError, ImageFormat, ImageReader, Limits};

use crate::detection::blocs_png;
use crate::{Limites, Rapport, Refus, TypeContenu};

fn format_image(t: TypeContenu) -> ImageFormat {
    match t {
        TypeContenu::Jpeg => ImageFormat::Jpeg,
        TypeContenu::Gif => ImageFormat::Gif,
        TypeContenu::Webp => ImageFormat::WebP,
        TypeContenu::Bmp => ImageFormat::Bmp,
        _ => ImageFormat::Png,
    }
}

fn refus_image(e: ImageError) -> Refus {
    match e {
        ImageError::Limits(l) => Refus::LimiteDepassee(l.to_string()),
        autre => Refus::Malforme(autre.to_string()),
    }
}

/// Décode `octets` (format imposé, jamais deviné) et réencode.
pub(crate) fn reconstruire(
    declare: TypeContenu,
    octets: &[u8],
    limites: &Limites,
    rapport: &mut Rapport,
) -> Result<(Vec<u8>, TypeContenu), Refus> {
    let mut lim = Limits::default();
    lim.max_image_width = Some(limites.cote_max);
    lim.max_image_height = Some(limites.cote_max);
    lim.max_alloc = Some(limites.allocation_max);

    let mut lecteur = ImageReader::with_format(Cursor::new(octets), format_image(declare));
    lecteur.limits(lim);
    let mut decodeur = lecteur.into_decoder().map_err(refus_image)?;

    // Bornage avant toute allocation de pixels (bombe de décompression).
    let (largeur, hauteur) = decodeur.dimensions();
    if largeur == 0 || hauteur == 0 {
        return Err(Refus::Malforme("image de dimension nulle".into()));
    }
    let pixels = u64::from(largeur) * u64::from(hauteur);
    if largeur > limites.cote_max || hauteur > limites.cote_max || pixels > limites.pixels_max {
        return Err(Refus::LimiteDepassee(format!(
            "{largeur}×{hauteur} ({pixels} pixels ; max {} par côté, {} au total)",
            limites.cote_max, limites.pixels_max
        )));
    }

    // `from_decoder` ne réserve pas sur `max_alloc` : tampon de sortie borné ici.
    let tampon = decodeur.total_bytes();
    if tampon > limites.allocation_max {
        return Err(Refus::LimiteDepassee(format!(
            "tampon décodé de {tampon} octets > {}",
            limites.allocation_max
        )));
    }

    // Inventaire de ce qui sera écarté.
    if let Ok(Some(exif)) = decodeur.exif_metadata() {
        rapport
            .retraits
            .push(format!("EXIF ({} octets)", exif.len()));
    }
    if let Ok(Some(icc)) = decodeur.icc_profile() {
        rapport
            .retraits
            .push(format!("profil ICC ({} octets)", icc.len()));
    }
    if declare == TypeContenu::Png {
        for bloc in blocs_png(octets) {
            rapport.retraits.push(format!("bloc PNG {bloc}"));
        }
    }
    let orientation = decodeur.orientation().ok();

    let mut image = DynamicImage::from_decoder(decodeur).map_err(refus_image)?;
    if let Some(o) = orientation {
        if o != image::metadata::Orientation::NoTransforms {
            image.apply_orientation(o);
            rapport
                .retraits
                .push(format!("orientation EXIF appliquée aux pixels ({o:?})"));
        }
    }
    if declare == TypeContenu::Gif {
        rapport
            .retraits
            .push("GIF : seule la première image est conservée".into());
    }

    rapport.dimensions = Some((image.width(), image.height()));

    let mut sortie = Vec::new();
    let type_reel = if declare == TypeContenu::Jpeg {
        let image = match image {
            DynamicImage::ImageLuma8(_) => image,
            autre => DynamicImage::ImageRgb8(autre.to_rgb8()),
        };
        image
            .write_with_encoder(JpegEncoder::new_with_quality(
                &mut sortie,
                limites.qualite_jpeg.clamp(1, 100),
            ))
            .map_err(|e| Refus::Interne(format!("réencodage JPEG : {e}")))?;
        TypeContenu::Jpeg
    } else {
        let image = match image {
            DynamicImage::ImageLuma8(_)
            | DynamicImage::ImageLumaA8(_)
            | DynamicImage::ImageRgb8(_)
            | DynamicImage::ImageRgba8(_)
            | DynamicImage::ImageLuma16(_)
            | DynamicImage::ImageLumaA16(_)
            | DynamicImage::ImageRgb16(_)
            | DynamicImage::ImageRgba16(_) => image,
            autre => DynamicImage::ImageRgba8(autre.to_rgba8()),
        };
        image
            .write_with_encoder(PngEncoder::new_with_quality(
                &mut sortie,
                CompressionType::Default,
                FilterType::Adaptive,
            ))
            .map_err(|e| Refus::Interne(format!("réencodage PNG : {e}")))?;
        TypeContenu::Png
    };
    Ok((sortie, type_reel))
}
