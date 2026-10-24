//! @id     miyucdr_tests_saturation
//! @do     tester_plafond_decodages
//! @role   test
//! @layer  toolkit
//! @human  miyucdr — plafond global des désarmements simultanés, fils abandonnés compris (revue de sécurité).

//! Binaire de test séparé : le compteur est global au processus.

use std::io::Cursor;
use std::time::{Duration, Instant};

use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{DynamicImage, ImageFormat, RgbImage};
use miyucdr::{decodages_en_cours, desarmer, Limites, Refus, MAX_DECODAGES_SIMULTANES};

/// Image lente à reconstruire (décodage + réencodage de plusieurs Mpx).
fn image_lente() -> Vec<u8> {
    let img = DynamicImage::ImageRgb8(RgbImage::from_fn(2500, 2500, |x, y| {
        image::Rgb([(x * 7 % 251) as u8, (y * 13 % 241) as u8, ((x ^ y) % 239) as u8])
    }));
    let mut p = Vec::new();
    img.write_with_encoder(PngEncoder::new_with_quality(&mut p, CompressionType::Fast, FilterType::NoFilter))
        .unwrap();
    p
}

fn attendre_vide() {
    let debut = Instant::now();
    while decodages_en_cours() > 0 {
        assert!(debut.elapsed() < Duration::from_secs(120), "fils de décodage jamais terminés");
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Des entrées lentes répétées, abandonnées au délai, n'empilent pas de fils :
/// au-delà du plafond, refus `Sature` ; une fois les fils finis, le CDR répond.
#[test]
fn fils_abandonnes_comptes_puis_saturation() {
    let lente = image_lente();
    let pressee = Limites { delai_max: Duration::from_millis(1), ..Limites::default() };
    let mut delais = 0;
    let mut satures = 0;
    for _ in 0..(MAX_DECODAGES_SIMULTANES * 3) {
        match desarmer("image/png", &lente, &pressee) {
            Err(Refus::DelaiDepasse) => delais += 1,
            Err(Refus::Sature) => satures += 1,
            autre => panic!("inattendu : {autre:?}"),
        }
    }
    assert!(delais >= 1);
    assert!(satures >= 1, "le plafond doit refuser ({delais} délais, {satures} saturés)");
    assert!(decodages_en_cours() <= MAX_DECODAGES_SIMULTANES);

    attendre_vide();
    // Plus rien en cours : un désarmement normal aboutit.
    let mut petite = Vec::new();
    DynamicImage::ImageRgb8(RgbImage::new(4, 4)).write_to(&mut Cursor::new(&mut petite), ImageFormat::Png).unwrap();
    assert!(desarmer("image/png", &petite, &Limites::default()).is_ok());
    assert_eq!(decodages_en_cours(), 0);
}
