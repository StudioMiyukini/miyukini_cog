//! @id     maia_federee_normalisation
//! @do     normaliser_texte_etranger
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « normalisation » (invisibles, bidi, homoglyphes, balisage, sauts de mise en forme).

//! Normalisation à l'entrée de tout texte étranger (IA-FEDEREE §2, EX-57).
//!
//! Ordre appliqué (déterministe, idempotent) :
//!
//! 1. **NFKC** : pleine chasse, ligatures, alphanumériques mathématiques,
//!    espaces typographiques → formes canoniques ;
//! 2. **suppression** des caractères invisibles et de format (espaces de
//!    largeur nulle, joints, sélecteurs de variante, étiquettes Unicode,
//!    trait d'union conditionnel, BOM), des **contrôles bidi**
//!    (LRE/RLE/PDF/LRO/RLO, isolats, marques LRM/RLM/ALM), des contrôles C0/C1
//!    et des zones à usage privé ;
//! 3. **balisage** retiré : caractères `< > { } [ ] ` | * # ~ \` supprimés
//!    (aucun HTML, Markdown ni gabarit ne survit ; choix fail-closed à
//!    consigner par amendement) ;
//! 4. **homoglyphes** : lettres cyrilliques et grecques confusables avec le
//!    latin ramenées au latin dans tout mot mixte, et partout si l'écriture
//!    dominante du texte est le latin ;
//! 5. **sauts de mise en forme** : séparateurs de ligne / paragraphe → saut
//!    de ligne, espaces repliés, au plus une ligne vide consécutive.
//!
//! Aucune URL n'est interprétée : une source reste du texte inerte.

use unicode_normalization::UnicodeNormalization;

/// Vrai si le caractère est invisible, de format, bidi, de contrôle ou privé.
fn est_a_supprimer(c: char) -> bool {
    let u = u32::from(c);
    matches!(u,
        0x0000..=0x0008 | 0x000B..=0x001F | 0x007F..=0x009F // contrôles C0 / C1 (hors \t \n)
        | 0x00AD // trait d'union conditionnel
        | 0x034F // joint de graphèmes
        | 0x061C // ALM
        | 0x115F | 0x1160 | 0x17B4 | 0x17B5 | 0x3164 | 0xFFA0 // remplisseurs
        | 0x180B..=0x180F // sélecteurs mongols
        | 0x200B..=0x200F // ZWSP, ZWNJ, ZWJ, LRM, RLM
        | 0x202A..=0x202E // enchâssements et forçages bidi
        | 0x2060..=0x206F // joints de mots, invisibles, isolats bidi
        | 0xFE00..=0xFE0F // sélecteurs de variante
        | 0xFEFF // BOM
        | 0xFFF0..=0xFFFB // annotations interlinéaires
        | 0x1D173..=0x1D17A // formats musicaux
        | 0xE000..=0xF8FF | 0xF0000..=0x10FFFF // usage privé
        | 0xE0000..=0xE0FFF // étiquettes et sélecteurs de variante supplémentaires
    )
}

/// Caractères de balisage retirés (aucun balisage, IA-FEDEREE §2).
fn est_balisage(c: char) -> bool {
    matches!(c, '<' | '>' | '{' | '}' | '[' | ']' | '`' | '|' | '*' | '#' | '~' | '\\')
}

/// Homoglyphe cyrillique ou grec → lettre latine.
#[allow(clippy::match_same_arms)]
fn homoglyphe(c: char) -> Option<char> {
    Some(match c {
        // Cyrillique minuscules
        'а' => 'a', 'е' => 'e', 'о' => 'o', 'р' => 'p', 'с' => 'c', 'у' => 'y', 'х' => 'x',
        'і' => 'i', 'ј' => 'j', 'ѕ' => 's', 'ԁ' => 'd', 'һ' => 'h', 'ԛ' => 'q', 'ԝ' => 'w', 'ӏ' => 'l',
        // Cyrillique majuscules
        'А' => 'A', 'В' => 'B', 'Е' => 'E', 'К' => 'K', 'М' => 'M', 'Н' => 'H', 'О' => 'O',
        'Р' => 'P', 'С' => 'C', 'Т' => 'T', 'Х' => 'X', 'У' => 'Y', 'І' => 'I', 'Ј' => 'J', 'Ѕ' => 'S',
        // Grec
        'ο' => 'o', 'ν' => 'v', 'ρ' => 'p', 'ι' => 'i', 'κ' => 'k', 'τ' => 't', 'υ' => 'u',
        'Α' => 'A', 'Β' => 'B', 'Ε' => 'E', 'Ζ' => 'Z', 'Η' => 'H', 'Ι' => 'I', 'Κ' => 'K',
        'Μ' => 'M', 'Ν' => 'N', 'Ο' => 'O', 'Ρ' => 'P', 'Τ' => 'T', 'Υ' => 'Y', 'Χ' => 'X',
        _ => return None,
    })
}

fn est_latin(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(u32::from(c), 0x00C0..=0x024F)
}

fn est_cyrillique_ou_grec(c: char) -> bool {
    matches!(u32::from(c), 0x0370..=0x052F)
}

/// Ramène les homoglyphes au latin (mots mixtes ; tout le texte si le latin domine).
fn normaliser_homoglyphes(texte: &str) -> String {
    let latins = texte.chars().filter(|c| est_latin(*c)).count();
    let autres = texte.chars().filter(|c| est_cyrillique_ou_grec(*c)).count();
    let latin_dominant = latins > autres;
    let mut sortie = String::with_capacity(texte.len());
    let mut mot = String::new();
    let vider = |mot: &mut String, sortie: &mut String| {
        let mixte = mot.chars().any(est_latin) && mot.chars().any(est_cyrillique_ou_grec);
        if mixte || latin_dominant {
            sortie.extend(mot.chars().map(|c| homoglyphe(c).unwrap_or(c)));
        } else {
            sortie.push_str(mot);
        }
        mot.clear();
    };
    for c in texte.chars() {
        if c.is_alphanumeric() {
            mot.push(c);
        } else {
            vider(&mut mot, &mut sortie);
            sortie.push(c);
        }
    }
    vider(&mut mot, &mut sortie);
    sortie
}

/// Passes au plus du point fixe de [`nettoyer`] (2 suffisent en pratique).
const MAX_PASSES: usize = 8;

/// Étapes 1 à 4, sans mise en forme des espaces, **jusqu'au point fixe** :
/// une passe peut en appeler une autre (suppression qui forme un mot mixte,
/// homoglyphes ramenés qui rendent le latin dominant, marque combinante
/// rapprochée de sa base) ; sans cela la normalisation n'est pas idempotente
/// et une requête construite par `RequeteTypee::nouvelle` est refusée comme
/// « non canonique » par `RequeteTypee::decoder` (trouvé par fuzzing).
fn nettoyer(texte: &str) -> String {
    let mut courant = nettoyer_une_passe(texte);
    for _ in 1..MAX_PASSES {
        let suivant = nettoyer_une_passe(&courant);
        if suivant == courant {
            break;
        }
        courant = suivant;
    }
    courant
}

/// Une passe des étapes 1 à 4.
fn nettoyer_une_passe(texte: &str) -> String {
    let nfkc: String = texte.nfkc().collect();
    let filtre: String = nfkc
        .chars()
        .filter_map(|c| match c {
            '\u{2028}' | '\u{2029}' | '\u{0085}' | '\r' => Some('\n'),
            '\t' => Some(' '),
            c if est_a_supprimer(c) || est_balisage(c) => None,
            c if c.is_whitespace() && c != '\n' => Some(' '),
            c => Some(c),
        })
        .collect();
    normaliser_homoglyphes(&filtre)
}

/// Normalise un texte multiligne : espaces repliés, lignes rognées, au plus
/// une ligne vide consécutive, ni en tête ni en fin.
pub fn normaliser(texte: &str) -> String {
    let propre = nettoyer(texte);
    let mut lignes: Vec<String> = Vec::new();
    let mut vide_precedente = true;
    for ligne in propre.split('\n') {
        let ligne = ligne.split(' ').filter(|m| !m.is_empty()).collect::<Vec<_>>().join(" ");
        if ligne.is_empty() {
            if !vide_precedente {
                lignes.push(String::new());
            }
            vide_precedente = true;
        } else {
            lignes.push(ligne);
            vide_precedente = false;
        }
    }
    while lignes.last().is_some_and(String::is_empty) {
        lignes.pop();
    }
    lignes.join("\n")
}

/// Normalise un champ d'une ligne (sujet, relation, source…) : tout saut de
/// ligne devient une espace.
pub fn normaliser_ligne(texte: &str) -> String {
    nettoyer(texte).split([' ', '\n']).filter(|m| !m.is_empty()).collect::<Vec<_>>().join(" ")
}

/// Clé de comparaison (corroboration, ancrage, recherche) : ligne normalisée,
/// minuscules, ponctuation finale retirée.
pub fn cle_comparaison(texte: &str) -> String {
    normaliser_ligne(texte).to_lowercase().trim_end_matches(['.', '!', '?', ';', ':', ',']).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invisibles_et_bidi_supprimes() {
        let piege = "Pa\u{200B}ris\u{202E}gnp.exe\u{202C} est\u{FEFF} la\u{2066} capitale\u{E0041}";
        assert_eq!(normaliser(piege), "Parisgnp.exe est la capitale");
    }

    #[test]
    fn homoglyphes_ramenes_au_latin() {
        // « Pаris » avec un « а » cyrillique : mot mixte.
        assert_eq!(normaliser("P\u{0430}ris"), "Paris");
        // Mot entièrement cyrillique dans un texte latin : ramené aussi.
        assert_eq!(normaliser("ignore \u{0441}\u{0435} texte"), "ignore ce texte");
        // Texte russe : intact.
        assert_eq!(normaliser("Москва столица"), "Москва столица");
    }

    #[test]
    fn nfkc_et_balisage() {
        assert_eq!(normaliser("ｓｙｓｔｅｍ"), "system");
        assert_eq!(normaliser("𝐒𝐘𝐒𝐓𝐄𝐌"), "SYSTEM");
        assert_eq!(normaliser("<script>alerte</script> **gras** `code` {{x}}"), "scriptalerte/script gras code x");
    }

    #[test]
    fn sauts_de_mise_en_forme() {
        assert_eq!(normaliser("  a\t\tb \r\n\n\n\n c\u{2028}d  \n\n"), "a b\n\nc\nd");
        assert_eq!(normaliser_ligne("a\nb\u{2029}c"), "a b c");
    }

    #[test]
    fn idempotente() {
        for t in ["P\u{0430}ris\u{200B}  <b>x</b>\n\n\nfin", "Москва", "ｓｙｓｔｅｍ"] {
            let une = normaliser(t);
            assert_eq!(normaliser(&une), une);
        }
    }

    #[test]
    fn idempotente_quand_une_passe_en_appelle_une_autre() {
        // Non-régression (fuzzing, cible `requete`) : la suppression des
        // contrôles forme le mot mixte « PНPН », ramené au latin, ce qui rend
        // le latin dominant ; le « Н » isolé ne l'était qu'à la 2e passe.
        let t = "\u{041D}-P\u{1b}\u{041D}\u{1b}P\u{1b}\u{041D}";
        let une = normaliser_ligne(t);
        assert_eq!(une, "H-PHPH");
        assert_eq!(normaliser_ligne(&une), une);
        assert_eq!(normaliser(&normaliser(t)), normaliser(t));
        // Marque combinante séparée de sa base par un invisible : composée.
        let e = normaliser("e\u{200B}\u{0301}");
        assert_eq!(e, "\u{e9}");
        assert_eq!(normaliser(&e), e);
    }

    #[test]
    fn cle_insensible_casse_et_ponctuation() {
        assert_eq!(cle_comparaison("  Paris. "), cle_comparaison("paris"));
    }
}
