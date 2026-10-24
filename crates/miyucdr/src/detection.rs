//! @id     miyucdr_detection
//! @do     detecter_type_reel_par_signature
//! @role   securite
//! @layer  toolkit
//! @human  Détection du type réel par octets magiques, signaux de polyglotte.

//! Le type réel est déduit du **contenu** (signatures), jamais du nom ni du type
//! déclaré. Les formats refusés sont reconnus aussi, pour un refus explicite.

use std::fmt;

/// Type détecté à partir des octets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeDetecte {
    /// PNG
    Png,
    /// JPEG
    Jpeg,
    /// GIF 87a / 89a
    Gif,
    /// WebP (conteneur RIFF)
    Webp,
    /// BMP (en-tête DIB reconnu)
    Bmp,
    /// Texte UTF-8 valide, sans octet nul ni signature binaire
    Texte,
    /// PDF
    Pdf,
    /// RTF
    Rtf,
    /// ZIP (dont OOXML, ODF, JAR…)
    Zip,
    /// Conteneur OLE2 (anciens documents Office)
    Ole,
    /// gzip
    Gzip,
    /// 7z
    SeptZip,
    /// RAR
    Rar,
    /// tar
    Tar,
    /// TIFF
    Tiff,
    /// ICO
    Ico,
    /// Exécutable (ELF, PE, Mach-O)
    Executable,
    /// Binaire inconnu ou texte non UTF-8
    Inconnu,
}

impl fmt::Display for TypeDetecte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nom = match self {
            Self::Png => "PNG",
            Self::Jpeg => "JPEG",
            Self::Gif => "GIF",
            Self::Webp => "WebP",
            Self::Bmp => "BMP",
            Self::Texte => "texte UTF-8",
            Self::Pdf => "PDF",
            Self::Rtf => "RTF",
            Self::Zip => "archive ZIP",
            Self::Ole => "document OLE2",
            Self::Gzip => "archive gzip",
            Self::SeptZip => "archive 7z",
            Self::Rar => "archive RAR",
            Self::Tar => "archive tar",
            Self::Tiff => "TIFF",
            Self::Ico => "ICO",
            Self::Executable => "exécutable",
            Self::Inconnu => "binaire inconnu ou texte non UTF-8",
        };
        f.write_str(nom)
    }
}

const MACHO: [[u8; 4]; 6] = [
    [0xFE, 0xED, 0xFA, 0xCE],
    [0xFE, 0xED, 0xFA, 0xCF],
    [0xCE, 0xFA, 0xED, 0xFE],
    [0xCF, 0xFA, 0xED, 0xFE],
    [0xCA, 0xFE, 0xBA, 0xBE],
    [0xBE, 0xBA, 0xFE, 0xCA],
];

/// Tailles d'en-tête DIB connues (BITMAPCOREHEADER … BITMAPV5HEADER).
const TAILLES_DIB: [u32; 6] = [12, 40, 52, 56, 108, 124];

/// Les lecteurs PDF acceptent `%PDF-` dans le premier kibioctet.
const FENETRE_PDF: usize = 1024;

fn est_bmp(o: &[u8]) -> bool {
    if o.len() < 26 || !o.starts_with(b"BM") {
        return false;
    }
    let taille_dib = u32::from_le_bytes([o[14], o[15], o[16], o[17]]);
    TAILLES_DIB.contains(&taille_dib)
}

/// Signature binaire en tête, sans considérer le texte.
fn signature(o: &[u8]) -> Option<TypeDetecte> {
    let t = if o.starts_with(b"\x89PNG\r\n\x1a\n") {
        TypeDetecte::Png
    } else if o.starts_with(&[0xFF, 0xD8, 0xFF]) {
        TypeDetecte::Jpeg
    } else if o.starts_with(b"GIF87a") || o.starts_with(b"GIF89a") {
        TypeDetecte::Gif
    } else if o.len() >= 12 && o.starts_with(b"RIFF") && &o[8..12] == b"WEBP" {
        TypeDetecte::Webp
    } else if est_bmp(o) {
        TypeDetecte::Bmp
    } else if o.starts_with(b"%PDF-") {
        TypeDetecte::Pdf
    } else if o.starts_with(b"{\\rtf") {
        TypeDetecte::Rtf
    } else if o.starts_with(b"PK\x03\x04")
        || o.starts_with(b"PK\x05\x06")
        || o.starts_with(b"PK\x07\x08")
    {
        TypeDetecte::Zip
    } else if o.starts_with(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
        TypeDetecte::Ole
    } else if o.starts_with(&[0x1F, 0x8B]) {
        TypeDetecte::Gzip
    } else if o.starts_with(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]) {
        TypeDetecte::SeptZip
    } else if o.starts_with(b"Rar!\x1a\x07") {
        TypeDetecte::Rar
    } else if o.len() >= 262 && &o[257..262] == b"ustar" {
        TypeDetecte::Tar
    } else if o.starts_with(b"II*\0") || o.starts_with(b"MM\0*") {
        TypeDetecte::Tiff
    } else if o.starts_with(&[0, 0, 1, 0]) {
        TypeDetecte::Ico
    } else if o.starts_with(b"\x7fELF")
        || o.starts_with(b"MZ")
        || (o.len() >= 4 && MACHO.iter().any(|m| o[..4] == m[..]))
    {
        TypeDetecte::Executable
    } else {
        return None;
    };
    Some(t)
}

fn contient(o: &[u8], motif: &[u8]) -> bool {
    !motif.is_empty() && o.windows(motif.len()).any(|f| f == motif)
}

/// Détecte le type réel de `octets`.
///
/// Un texte est retenu seulement s'il est UTF-8 valide, sans octet nul, sans
/// signature binaire en tête ni `%PDF-` dans le premier kibioctet.
pub fn detecter(octets: &[u8]) -> TypeDetecte {
    if let Some(t) = signature(octets) {
        return t;
    }
    let tete = &octets[..octets.len().min(FENETRE_PDF)];
    if contient(tete, b"%PDF-") {
        return TypeDetecte::Pdf;
    }
    if !octets.contains(&0) && std::str::from_utf8(octets).is_ok() {
        TypeDetecte::Texte
    } else {
        TypeDetecte::Inconnu
    }
}

/// Fin du flux PNG (après `IEND`), ou `None` si la structure est illisible.
pub(crate) fn fin_png(o: &[u8]) -> Option<usize> {
    let mut pos = 8usize;
    loop {
        let entete = o.get(pos..pos.checked_add(8)?)?;
        let longueur = u32::from_be_bytes([entete[0], entete[1], entete[2], entete[3]]) as usize;
        let fin = pos.checked_add(12)?.checked_add(longueur)?;
        if fin > o.len() {
            return None;
        }
        if &entete[4..8] == b"IEND" {
            return Some(fin);
        }
        pos = fin;
    }
}

/// Blocs PNG auxiliaires présents (types de 4 lettres), dans l'ordre.
pub(crate) fn blocs_png(o: &[u8]) -> Vec<String> {
    let mut blocs = Vec::new();
    let mut pos = 8usize;
    while let Some(entete) = pos.checked_add(8).and_then(|f| o.get(pos..f)) {
        let longueur = u32::from_be_bytes([entete[0], entete[1], entete[2], entete[3]]) as usize;
        let type_bloc = String::from_utf8_lossy(&entete[4..8]).into_owned();
        if &entete[4..8] == b"IEND" {
            break;
        }
        // Bloc auxiliaire : première lettre minuscule.
        if entete[4].is_ascii_lowercase() && !blocs.contains(&type_bloc) {
            blocs.push(type_bloc);
        }
        match pos.checked_add(12).and_then(|p| p.checked_add(longueur)) {
            Some(fin) if fin <= o.len() => pos = fin,
            _ => break,
        }
    }
    blocs
}

/// Signaux suspects : données après la fin, signatures d'autres formats incluses.
///
/// Sans effet sur la sortie (reconstruite), mais consignés pour les Vigies.
pub(crate) fn anomalies(octets: &[u8], detecte: TypeDetecte) -> Vec<String> {
    let mut a = Vec::new();
    if detecte == TypeDetecte::Png {
        if let Some(fin) = fin_png(octets) {
            if fin < octets.len() {
                a.push(format!(
                    "{} octets après IEND (polyglotte possible), écartés",
                    octets.len() - fin
                ));
            }
        }
    }
    if detecte == TypeDetecte::Texte {
        let minuscule = octets
            .iter()
            .take(1 << 20)
            .map(u8::to_ascii_lowercase)
            .collect::<Vec<u8>>();
        if contient(&minuscule, b"<script") || contient(&minuscule, b"javascript:") {
            a.push("texte contenant <script ou javascript: (à échapper au rendu)".into());
        }
    } else {
        if contient(octets, b"PK\x03\x04") || contient(octets, b"PK\x05\x06") {
            a.push("signature ZIP incluse (polyglotte possible), écartée".into());
        }
        if contient(&octets[..octets.len().min(FENETRE_PDF)], b"%PDF-") {
            a.push("signature PDF dans le premier kibioctet (polyglotte possible)".into());
        }
    }
    a
}
