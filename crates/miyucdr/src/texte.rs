//! @id     miyucdr_texte
//! @do     nettoyer_texte_utf8
//! @role   securite
//! @layer  toolkit
//! @human  Texte : UTF-8 strict, contrôles et contrôles bidi retirés, fins de ligne, NFC.

use unicode_normalization::UnicodeNormalization;

use crate::{Limites, Rapport, Refus, TypeContenu};

/// Compteurs de caractères retirés.
#[derive(Debug, Default)]
pub(crate) struct Compteurs {
    pub controles: usize,
    pub bidi: usize,
    pub bom: usize,
    pub fins_de_ligne: usize,
}

impl Compteurs {
    pub(crate) fn consigner(&self, rapport: &mut Rapport) {
        if self.bom > 0 {
            rapport
                .retraits
                .push(format!("{} BOM / U+FEFF retirés", self.bom));
        }
        if self.controles > 0 {
            rapport
                .retraits
                .push(format!("{} caractères de contrôle retirés", self.controles));
        }
        if self.bidi > 0 {
            rapport.retraits.push(format!(
                "{} contrôles bidirectionnels retirés (Trojan Source)",
                self.bidi
            ));
        }
        if self.fins_de_ligne > 0 {
            rapport.retraits.push(format!(
                "{} fins de ligne CR/CRLF normalisées",
                self.fins_de_ligne
            ));
        }
    }
}

/// Contrôles bidirectionnels d'incorporation, de forçage et d'isolement.
fn est_bidi(c: char) -> bool {
    matches!(c, '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}' | '\u{061C}')
}

/// Nettoie une chaîne.
///
/// - garde `\n` et `\t` ; `\r\n` et `\r` isolé deviennent `\n` si
///   `fins_de_ligne`, sinon `\r` est conservé ;
/// - retire les autres contrôles C0, DEL, C1, U+FEFF et les contrôles bidi ;
/// - normalise en NFC si demandé.
pub(crate) fn nettoyer(s: &str, nfc: bool, fins_de_ligne: bool, c: &mut Compteurs) -> String {
    let mut sortie = String::with_capacity(s.len());
    let mut car = s.chars().peekable();
    while let Some(ch) = car.next() {
        match ch {
            '\r' if fins_de_ligne => {
                if car.peek() == Some(&'\n') {
                    car.next();
                }
                c.fins_de_ligne += 1;
                sortie.push('\n');
            }
            '\n' | '\t' | '\r' => sortie.push(ch),
            '\u{FEFF}' => c.bom += 1,
            ch if ch.is_control() => c.controles += 1,
            ch if est_bidi(ch) => c.bidi += 1,
            ch => sortie.push(ch),
        }
    }
    if nfc {
        sortie.nfc().collect()
    } else {
        sortie
    }
}

/// UTF-8 strict.
pub(crate) fn utf8(octets: &[u8]) -> Result<&str, Refus> {
    std::str::from_utf8(octets).map_err(|e| Refus::Malforme(format!("UTF-8 invalide : {e}")))
}

/// Texte brut et Markdown.
pub(crate) fn reconstruire(
    declare: TypeContenu,
    octets: &[u8],
    limites: &Limites,
    rapport: &mut Rapport,
) -> Result<Vec<u8>, Refus> {
    let texte = utf8(octets)?;
    let mut c = Compteurs::default();
    let propre = nettoyer(texte, limites.normaliser_nfc, true, &mut c);
    c.consigner(rapport);
    if declare == TypeContenu::Markdown {
        rapport
            .retraits
            .push("Markdown transmis comme texte : le rendu doit filtrer HTML et liens".into());
    }
    Ok(propre.into_bytes())
}
