//! @id     miyucdr_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles du CDR : petites images de chaque format, textes types (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

use std::fs;
use std::io::Cursor;
use std::path::Path;

use image::{ImageFormat, Rgba, RgbaImage};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_type(t: u8, octets: &[u8]) -> Vec<u8> {
    [&[t][..], octets].concat()
}

fn main() {
    let img =
        RgbaImage::from_fn(5, 3, |x, y| Rgba([(x * 50) as u8, (y * 80) as u8, 128, if x == 0 { 0 } else { 255 }]));
    let dynamique = image::DynamicImage::ImageRgba8(img);
    for (t, nom, format) in [
        (0u8, "png", ImageFormat::Png),
        (1, "jpeg", ImageFormat::Jpeg),
        (2, "gif", ImageFormat::Gif),
        (3, "webp", ImageFormat::WebP),
        (4, "bmp", ImageFormat::Bmp),
    ] {
        let source = if format == ImageFormat::Jpeg {
            image::DynamicImage::ImageRgb8(dynamique.to_rgb8())
        } else {
            dynamique.clone()
        };
        let mut o = Cursor::new(Vec::new());
        source.write_to(&mut o, format).expect("encodage d'image");
        ecrire("desarmer_image", nom, &avec_type(t, &o.into_inner()));
    }

    let textes: [(u8, &str, &str); 7] = [
        (0, "texte", "Bonjour\r\nà tous\u{202e}\u{0007}\tfin\n"),
        (1, "csv", "nom,montant\r\n\"Dupont, Jean\",=1+1\r\n@SUM(A1),\"a\"\"b\"\n"),
        (1, "csv_formules", "+cmd|' /C calc'!A0,-2,\t3\n"),
        (2, "markdown", "# Titre\n\n[lien](javascript:alert(1)) <script>x</script>\n"),
        (3, "json", "{\"a\":[1,2.5,{\"b\":\"\\u202e\\u0000c\"}],\"n\":null,\"v\":true}"),
        (3, "json_profond", "[[[[[[[[[[[[[[[[[[[[1]]]]]]]]]]]]]]]]]]]"),
        (0, "nfc", "e\u{0301}\u{feff}\u{200b}"),
    ];
    for (t, nom, texte) in textes {
        ecrire("desarmer_texte", nom, &avec_type(t, texte.as_bytes()));
    }
    ecrire("desarmer_texte", "mime_charset", &[&[0xF0][..], b"text/plain; charset=\"UTF-8\"\0abc"].concat());
    ecrire("desarmer_texte", "mime_refuse", &[&[0xF0][..], b"application/pdf\0%PDF-1.7"].concat());
}
