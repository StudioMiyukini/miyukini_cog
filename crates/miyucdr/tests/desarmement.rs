//! @id     miyucdr_tests_desarmement
//! @do     verifier_desarmement_reconstruction
//! @role   test
//! @layer  toolkit
//! @human  Tests EX-45 : reconstruction propre, métadonnées retirées, refus fermés par défaut.

use std::io::Cursor;
use std::time::Duration;

use image::codecs::bmp::BmpEncoder;
use image::codecs::gif::GifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ImageFormat, ImageReader, Rgb, RgbImage, Rgba, RgbaImage};
use miyucdr::{desarmer, Cdr, Desarmement, Limites, Refus, TypeContenu, TypeDetecte};

// ── Outils ─────────────────────────────────────────────────────────────

fn crc32(octets: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &o in octets {
        crc ^= u32::from(o);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn bloc_png(type_bloc: [u8; 4], donnees: &[u8]) -> Vec<u8> {
    let mut b = (donnees.len() as u32).to_be_bytes().to_vec();
    b.extend_from_slice(&type_bloc);
    b.extend_from_slice(donnees);
    let mut c = type_bloc.to_vec();
    c.extend_from_slice(donnees);
    b.extend_from_slice(&crc32(&c).to_be_bytes());
    b
}

/// Insère des blocs juste après IHDR (signature 8 + IHDR 25 octets).
fn inserer_blocs_png(png: &[u8], blocs: &[Vec<u8>]) -> Vec<u8> {
    let mut s = png[..33].to_vec();
    for b in blocs {
        s.extend_from_slice(b);
    }
    s.extend_from_slice(&png[33..]);
    s
}

fn image_test(l: u32, h: u32) -> RgbImage {
    RgbImage::from_fn(l, h, |x, y| {
        Rgb([(x * 7) as u8, (y * 5) as u8, ((x + y) * 3) as u8])
    })
}

fn encoder(img: &DynamicImage, format: ImageFormat) -> Vec<u8> {
    let mut v = Vec::new();
    match format {
        ImageFormat::Png => img.write_with_encoder(PngEncoder::new(&mut v)).unwrap(),
        ImageFormat::Jpeg => img
            .write_with_encoder(JpegEncoder::new_with_quality(&mut v, 90))
            .unwrap(),
        ImageFormat::Bmp => img.write_with_encoder(BmpEncoder::new(&mut v)).unwrap(),
        ImageFormat::WebP => img
            .write_with_encoder(WebPEncoder::new_lossless(&mut v))
            .unwrap(),
        ImageFormat::Gif => {
            let mut e = GifEncoder::new(&mut v);
            e.encode_frame(image::Frame::new(img.to_rgba8())).unwrap();
            drop(e);
        }
        _ => unreachable!(),
    }
    v
}

fn png() -> Vec<u8> {
    encoder(
        &DynamicImage::ImageRgb8(image_test(32, 24)),
        ImageFormat::Png,
    )
}

fn jpeg() -> Vec<u8> {
    encoder(
        &DynamicImage::ImageRgb8(image_test(32, 24)),
        ImageFormat::Jpeg,
    )
}

/// Segment APP1 EXIF minimal (TIFF petit-boutiste, orientation = 6 : rotation 90°).
fn app1_exif() -> Vec<u8> {
    let mut tiff = b"II*\0".to_vec();
    tiff.extend_from_slice(&8u32.to_le_bytes()); // IFD0
    tiff.extend_from_slice(&1u16.to_le_bytes()); // 1 entrée
    tiff.extend_from_slice(&0x0112u16.to_le_bytes()); // Orientation
    tiff.extend_from_slice(&3u16.to_le_bytes()); // SHORT
    tiff.extend_from_slice(&1u32.to_le_bytes());
    tiff.extend_from_slice(&6u16.to_le_bytes());
    tiff.extend_from_slice(&[0, 0]);
    tiff.extend_from_slice(&0u32.to_le_bytes()); // pas d'IFD suivant
    tiff.extend_from_slice(b"GPS-SECRET-48.8566N");
    let mut contenu = b"Exif\0\0".to_vec();
    contenu.extend_from_slice(&tiff);
    let mut seg = vec![0xFF, 0xE1];
    seg.extend_from_slice(&((contenu.len() + 2) as u16).to_be_bytes());
    seg.extend_from_slice(&contenu);
    seg
}

/// Segment COM (commentaire JPEG).
fn com(texte: &[u8]) -> Vec<u8> {
    let mut seg = vec![0xFF, 0xFE];
    seg.extend_from_slice(&((texte.len() + 2) as u16).to_be_bytes());
    seg.extend_from_slice(texte);
    seg
}

fn inserer_apres_soi(jpeg: &[u8], segments: &[Vec<u8>]) -> Vec<u8> {
    let mut s = jpeg[..2].to_vec();
    for seg in segments {
        s.extend_from_slice(seg);
    }
    s.extend_from_slice(&jpeg[2..]);
    s
}

fn contient(o: &[u8], motif: &[u8]) -> bool {
    o.windows(motif.len()).any(|f| f == motif)
}

fn relire(octets: &[u8], format: ImageFormat) -> DynamicImage {
    ImageReader::with_format(Cursor::new(octets), format)
        .decode()
        .expect("sortie décodable")
}

fn zip_minimal() -> Vec<u8> {
    let mut z = b"PK\x03\x04".to_vec();
    z.extend_from_slice(&[0u8; 26]);
    z.extend_from_slice(b"payload.sh#!/bin/sh rm -rf /");
    z.extend_from_slice(b"PK\x05\x06");
    z.extend_from_slice(&[0u8; 18]);
    z
}

fn defaut() -> Limites {
    Limites::default()
}

// ── Images ─────────────────────────────────────────────────────────────

#[test]
fn png_reconstruit_sans_blocs_texte_ni_icc() {
    let mut iccp = b"profil\0\0".to_vec();
    iccp.extend_from_slice(&[0x78, 0x9C, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let piege = inserer_blocs_png(
        &png(),
        &[
            bloc_png(*b"tEXt", b"Comment\0<script>alert(1)</script>"),
            bloc_png(
                *b"iTXt",
                b"XML:com.adobe.xmp\0\0\0\0\0<x:xmpmeta>SECRET</x:xmpmeta>",
            ),
            bloc_png(*b"zTXt", b"Note\0\0\x78\x9c\x03\x00\x00\x00\x00\x01"),
            bloc_png(*b"eXIf", b"MM\0*\0\0\0\x08\0\0"),
            bloc_png(*b"iCCP", &iccp),
        ],
    );
    let r = desarmer("image/png", &piege, &defaut()).expect("PNG accepté");
    assert_eq!(r.type_reel, TypeContenu::Png);
    for motif in [
        &b"tEXt"[..],
        b"iTXt",
        b"zTXt",
        b"eXIf",
        b"iCCP",
        b"<script",
        b"SECRET",
    ] {
        assert!(
            !contient(&r.octets, motif),
            "{} subsiste",
            String::from_utf8_lossy(motif)
        );
    }
    assert!(r.rapport.retraits.iter().any(|x| x.contains("tEXt")));
    let sortie = relire(&r.octets, ImageFormat::Png).to_rgb8();
    assert_eq!(
        sortie,
        image_test(32, 24),
        "pixels PNG identiques (sans perte)"
    );
}

#[test]
fn jpeg_reconstruit_sans_exif_ni_commentaire_orientation_appliquee() {
    let piege = inserer_apres_soi(&jpeg(), &[app1_exif(), com(b"<?php system($_GET[0]); ?>")]);
    let r = desarmer("image/jpeg", &piege, &defaut()).expect("JPEG accepté");
    assert_eq!(r.type_reel, TypeContenu::Jpeg);
    assert!(!contient(&r.octets, b"Exif"));
    assert!(!contient(&r.octets, b"GPS-SECRET"));
    assert!(!contient(&r.octets, b"<?php"));
    assert!(r.rapport.retraits.iter().any(|x| x.starts_with("EXIF")));
    // Orientation 6 appliquée : 32×24 devient 24×32.
    assert_eq!(r.rapport.dimensions, Some((24, 32)));
    let sortie = relire(&r.octets, ImageFormat::Jpeg);
    assert_eq!((sortie.width(), sortie.height()), (24, 32));
}

#[test]
fn gif_webp_bmp_reconstruits_en_png() {
    let img = DynamicImage::ImageRgba8(RgbaImage::from_fn(16, 8, |x, y| {
        Rgba([(x * 16) as u8, (y * 32) as u8, 0, 255])
    }));
    for (mime, format) in [
        ("image/gif", ImageFormat::Gif),
        ("image/webp", ImageFormat::WebP),
        ("image/bmp", ImageFormat::Bmp),
    ] {
        let entree = encoder(&img, format);
        let r = desarmer(mime, &entree, &defaut()).unwrap_or_else(|e| panic!("{mime} : {e}"));
        assert_eq!(r.type_reel, TypeContenu::Png, "{mime}");
        assert!(r.octets.starts_with(b"\x89PNG\r\n\x1a\n"));
        let sortie = relire(&r.octets, ImageFormat::Png);
        assert_eq!((sortie.width(), sortie.height()), (16, 8), "{mime}");
    }
}

#[test]
fn polyglotte_png_zip_sortie_sans_charge_finale() {
    let mut poly = png();
    poly.extend_from_slice(&zip_minimal());
    let r = desarmer("image/png", &poly, &defaut()).expect("PNG reconstruit");
    assert!(!contient(&r.octets, b"PK\x03\x04"));
    assert!(!contient(&r.octets, b"PK\x05\x06"));
    assert!(!contient(&r.octets, b"payload"));
    assert!(
        r.octets.ends_with(&bloc_png(*b"IEND", b"")),
        "sortie terminée par IEND"
    );
    assert!(r.rapport.anomalies.iter().any(|a| a.contains("après IEND")));
    assert!(r.rapport.anomalies.iter().any(|a| a.contains("ZIP")));
}

#[test]
fn polyglotte_jpeg_zip_sortie_sans_charge_finale() {
    let mut poly = jpeg();
    poly.extend_from_slice(&zip_minimal());
    let r = desarmer("image/jpeg", &poly, &defaut()).expect("JPEG reconstruit");
    assert!(!contient(&r.octets, b"PK\x03\x04"));
    assert!(!contient(&r.octets, b"payload"));
    assert!(r.octets.ends_with(&[0xFF, 0xD9]), "sortie terminée par EOI");
    assert!(r.rapport.anomalies.iter().any(|a| a.contains("ZIP")));
}

#[test]
fn bombe_dimensions_declarees_refusee_avant_allocation() {
    // IHDR réécrit : 100 000 × 100 000, CRC recalculé.
    let mut p = png();
    p[16..20].copy_from_slice(&100_000u32.to_be_bytes());
    p[20..24].copy_from_slice(&100_000u32.to_be_bytes());
    let crc = crc32(&p[12..29]);
    p[29..33].copy_from_slice(&crc.to_be_bytes());
    let e = desarmer("image/png", &p, &defaut()).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)), "{e:?}");
}

#[test]
fn bombe_decompression_reelle_refusee_par_pixels_max() {
    // 4000 × 4000 uni : quelques kio compressés, 48 Mio décodés.
    let img = DynamicImage::ImageRgb8(RgbImage::new(4000, 4000));
    let mut p = Vec::new();
    img.write_with_encoder(PngEncoder::new_with_quality(
        &mut p,
        CompressionType::Best,
        FilterType::Adaptive,
    ))
    .unwrap();
    assert!(p.len() < 200_000, "entrée compacte ({} octets)", p.len());
    let limites = Limites {
        pixels_max: 1_000_000,
        ..defaut()
    };
    let e = desarmer("image/png", &p, &limites).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)), "{e:?}");
    let limites = Limites {
        allocation_max: 1024 * 1024,
        ..defaut()
    };
    let e = desarmer("image/png", &p, &limites).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)), "{e:?}");
}

#[test]
fn images_tronquees_ou_malformees_refusees() {
    for (mime, octets) in [("image/png", png()), ("image/jpeg", jpeg())] {
        let tronque = &octets[..octets.len() / 2];
        let e = desarmer(mime, tronque, &defaut()).unwrap_err();
        assert!(matches!(e, Refus::Malforme(_)), "{mime} tronqué : {e:?}");
    }
    let mut corrompu = png();
    let n = corrompu.len();
    corrompu[n - 20] ^= 0xFF; // données IDAT altérées (CRC faux)
    assert!(desarmer("image/png", &corrompu, &defaut()).is_err());
    let e = desarmer("image/gif", b"GIF89a\x01", &defaut()).unwrap_err();
    assert!(matches!(e, Refus::Malforme(_)), "{e:?}");
}

#[test]
fn taille_entree_bornee() {
    let limites = Limites {
        octets_max: 100,
        ..defaut()
    };
    let e = desarmer("image/png", &png(), &limites).unwrap_err();
    assert!(matches!(e, Refus::TropVolumineux { .. }), "{e:?}");
}

#[test]
fn delai_borne() {
    let limites = Limites {
        delai_max: Duration::from_nanos(1),
        ..defaut()
    };
    let img = DynamicImage::ImageRgb8(image_test(2000, 2000));
    let p = encoder(&img, ImageFormat::Png);
    let e = desarmer("image/png", &p, &limites).unwrap_err();
    assert_eq!(e, Refus::DelaiDepasse);
}

// ── Discordances et types refusés ──────────────────────────────────────

#[test]
fn discordance_type_declare_refusee() {
    let cas: [(&str, Vec<u8>, TypeDetecte); 6] = [
        ("image/png", jpeg(), TypeDetecte::Jpeg),
        ("image/jpeg", png(), TypeDetecte::Png),
        ("image/png", zip_minimal(), TypeDetecte::Zip),
        ("text/plain", png(), TypeDetecte::Png),
        (
            "text/plain",
            b"%PDF-1.7\n1 0 obj".to_vec(),
            TypeDetecte::Pdf,
        ),
        (
            "application/json",
            b"\x7fELF\x02\x01\x01".to_vec(),
            TypeDetecte::Executable,
        ),
    ];
    for (mime, octets, attendu) in cas {
        let e = desarmer(mime, &octets, &defaut()).unwrap_err();
        assert!(
            matches!(e, Refus::TypeDiscordant { detecte, .. } if detecte == attendu),
            "{mime} : {e:?}"
        );
    }
    // PDF caché après un préambule texte (fenêtre de 1 kio des lecteurs PDF).
    let e = desarmer("text/plain", b"bonjour\n%PDF-1.4 ...", &defaut()).unwrap_err();
    assert!(matches!(
        e,
        Refus::TypeDiscordant {
            detecte: TypeDetecte::Pdf,
            ..
        }
    ));
    // Texte non UTF-8 / binaire.
    let e = desarmer("text/plain", b"caf\xe9", &defaut()).unwrap_err();
    assert!(matches!(
        e,
        Refus::TypeDiscordant {
            detecte: TypeDetecte::Inconnu,
            ..
        }
    ));
}

#[test]
fn types_non_pris_en_charge_refuses() {
    for mime in [
        "application/pdf",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/vnd.oasis.opendocument.text",
        "application/msword",
        "application/zip",
        "application/x-7z-compressed",
        "image/svg+xml",
        "image/tiff",
        "text/html",
        "application/octet-stream",
        "",
        "n'importe quoi",
        "text/plain; charset=iso-8859-1",
    ] {
        let e = desarmer(mime, b"contenu", &defaut()).unwrap_err();
        assert!(matches!(e, Refus::NonPrisEnCharge { .. }), "{mime} : {e:?}");
    }
    assert!(desarmer("Text/Plain; Charset=UTF-8", b"ok", &defaut()).is_ok());
}

// ── Texte, CSV, Markdown, JSON ─────────────────────────────────────────

#[test]
fn texte_controles_bidi_bom_retires_crlf_nfc() {
    let entree = "\u{FEFF}ligne1\r\nA\u{0007}B\u{0000}\r\u{202E}evil\u{2066}\u{0085}\te\u{0301}\n";
    // Un octet nul rend l'entrée binaire : vérifié séparément.
    let sans_nul = entree.replace('\u{0000}', "");
    let r = desarmer("text/plain", sans_nul.as_bytes(), &defaut()).unwrap();
    let s = String::from_utf8(r.octets).unwrap();
    assert_eq!(s, "ligne1\nAB\nevil\t\u{00E9}\n");
    assert!(r
        .rapport
        .retraits
        .iter()
        .any(|x| x.contains("bidirectionnels")));
    let e = desarmer("text/plain", entree.as_bytes(), &defaut()).unwrap_err();
    assert!(
        matches!(e, Refus::TypeDiscordant { .. }),
        "octet nul refusé"
    );
}

#[test]
fn texte_nfc_optionnel_et_taille_bornee() {
    let limites = Limites {
        normaliser_nfc: false,
        ..defaut()
    };
    let r = desarmer("text/markdown", "e\u{0301}".as_bytes(), &limites).unwrap();
    assert_eq!(r.octets, "e\u{0301}".as_bytes());
    assert_eq!(r.type_reel, TypeContenu::Markdown);
    let limites = Limites {
        texte_octets_max: 10,
        ..defaut()
    };
    let e = desarmer("text/plain", &[b'a'; 11], &limites).unwrap_err();
    assert!(matches!(e, Refus::TropVolumineux { .. }));
}

#[test]
fn markdown_script_signale() {
    let r = desarmer("text/markdown", b"# Titre\n<SCRIPT>x</script>", &defaut()).unwrap();
    assert!(r.rapport.anomalies.iter().any(|a| a.contains("script")));
}

#[test]
fn csv_reecrit_formules_neutralisees() {
    let entree = "nom,montant,note\r\nalice,-12.5,\"=HYPERLINK(\"\"http://x\"\")\"\r\n\"bob, jr\",+3,@SUM(A1)\r\n\"multi\nligne\",\"a\"\"b\",-cmd|' /C calc'!A0\r\n";
    let r = desarmer("text/csv", entree.as_bytes(), &defaut()).unwrap();
    let s = String::from_utf8(r.octets).unwrap();
    assert_eq!(
        s,
        "nom,montant,note\nalice,-12.5,\"'=HYPERLINK(\"\"http://x\"\")\"\n\"bob, jr\",+3,'@SUM(A1)\n\"multi\nligne\",\"a\"\"b\",'-cmd|' /C calc'!A0\n"
    );
    // Point-virgule (tableurs français).
    let r = desarmer("text/csv", "a;b\n1;=2+2\n".as_bytes(), &defaut()).unwrap();
    assert_eq!(r.octets, b"a;b\n1;'=2+2\n");
}

#[test]
fn csv_malforme_ou_hors_limites_refuse() {
    for mauvais in ["a,\"b\nc", "a,b\"c\n", "\"a\"x,b\n"] {
        let e = desarmer("text/csv", mauvais.as_bytes(), &defaut()).unwrap_err();
        assert!(matches!(e, Refus::Malforme(_)), "{mauvais:?} : {e:?}");
    }
    let limites = Limites {
        csv_enregistrements_max: 2,
        ..defaut()
    };
    let e = desarmer("text/csv", b"a\nb\nc\n", &limites).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)));
    let limites = Limites {
        csv_champs_max: 3,
        ..defaut()
    };
    let e = desarmer("text/csv", b"a,b,c,d\n", &limites).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)));
}

#[test]
fn json_reserialise_et_nettoye() {
    let entree = "\u{FEFF}{ \"a\" : [1, 2.5, true, null],\n \"a\": \"doublon\", \"t\": \"x\\u0000y\\u202Ez\" }";
    let r = desarmer("application/json", entree.as_bytes(), &defaut()).unwrap();
    let v: serde_json::Value = serde_json::from_slice(&r.octets).unwrap();
    assert_eq!(v["a"], "doublon");
    assert_eq!(v["t"], "xyz");
    assert!(!r.octets.contains(&b' '));
}

#[test]
fn json_malforme_profond_ou_avec_queue_refuse() {
    for mauvais in [
        "{\"a\":1",
        "{\"a\":1} PK\u{3}\u{4}",
        "[1,]",
        "// c\n{}",
        "NaN",
    ] {
        let e = desarmer("application/json", mauvais.as_bytes(), &defaut()).unwrap_err();
        assert!(matches!(e, Refus::Malforme(_)), "{mauvais:?} : {e:?}");
    }
    let profond = format!("{}{}", "[".repeat(100), "]".repeat(100));
    let e = desarmer("application/json", profond.as_bytes(), &defaut()).unwrap_err();
    assert!(matches!(e, Refus::LimiteDepassee(_)), "{e:?}");
    let tres_profond = format!("{}{}", "[".repeat(100_000), "]".repeat(100_000));
    assert!(desarmer("application/json", tres_profond.as_bytes(), &defaut()).is_err());
}

// ── Trait consommé par le Portail ──────────────────────────────────────

#[test]
fn trait_desarmement() {
    let cdr: Box<dyn Desarmement> = Box::new(Cdr::default());
    let sortie = cdr.desarmer("image/png", &png()).unwrap();
    assert!(sortie.starts_with(b"\x89PNG"));
    let refus = cdr.desarmer("application/pdf", b"%PDF-1.7").unwrap_err();
    assert!(refus.contains("non pris en charge"), "{refus}");
    let detaille = Cdr::new(defaut())
        .desarmer_detaille("text/plain", b"ok")
        .unwrap();
    assert_eq!(detaille.rapport.type_detecte, TypeDetecte::Texte);
}

#[test]
fn famille_image_par_detection() {
    assert_eq!(
        TypeContenu::image_depuis_detecte(miyucdr::detecter(&jpeg())),
        Some(TypeContenu::Jpeg)
    );
    assert_eq!(
        TypeContenu::image_depuis_detecte(miyucdr::detecter(b"%PDF-1.7")),
        None
    );
}

/// Constats du fuzzing (lot 8) : le nettoyage ne doit pas reconstituer une
/// signature de fichier, et le JSON doit être idempotent.
#[test]
fn nettoyage_ne_reconstitue_aucune_signature() {
    let lim = miyucdr::Limites::default();
    for (mime, entree) in [
        ("text/plain", &b"\x01GIF89a"[..]),
        ("text/plain", &b"\x01{\\rtf"[..]),
        ("text/csv", &b"\x16MZ"[..]),
        ("text/plain", &b"\x0c%P\x03DF-M"[..]),
    ] {
        match miyucdr::desarmer(mime, entree, &lim) {
            Ok(r) => assert_eq!(miyucdr::detecter(&r.octets), miyucdr::TypeDetecte::Texte, "{entree:?}"),
            Err(_) => {}
        }
    }
}

#[test]
fn json_idempotent() {
    let lim = miyucdr::Limites::default();
    let une = miyucdr::desarmer("application/json", b"1976619766666696.86", &lim).unwrap().octets;
    let deux = miyucdr::desarmer("application/json", &une, &lim).unwrap().octets;
    assert_eq!(une, deux);
}
