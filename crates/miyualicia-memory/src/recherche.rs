//! Recherche plein texte de la mémoire (remplace l'index FTS5 de l'ancienne base).
//!
//! Même découpage que le tokenizer `unicode61` de FTS5 par défaut : jetons =
//! suites de caractères alphanumériques, casse et diacritiques latins ignorés
//! (`métro` = `Metro`). Requête : tous les termes doivent figurer (ET
//! implicite) ; `terme*` cherche un préfixe. La syntaxe avancée de FTS5
//! (`OR`, `NEAR`, guillemets…) n'est pas interprétée : tout est terme.
//! Rang : fréquence des termes rapportée à la longueur du texte (décroissant).

// @id service.alicia.memory.recherche
// @role memory_full_text_search
// @layer 7
// @do match_memory_full_text
// @human Recherche plein texte des faits et épisodes d'Alicia, sans index SQL.

/// Ramène une lettre latine accentuée à sa base (minuscule).
fn plier(c: char) -> char {
    match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => 'a',
        'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => 'c',
        'ď' | 'đ' => 'd',
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => 'e',
        'ĝ' | 'ğ' | 'ġ' | 'ģ' => 'g',
        'ĥ' | 'ħ' => 'h',
        'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => 'i',
        'ĵ' => 'j',
        'ķ' => 'k',
        'ĺ' | 'ļ' | 'ľ' | 'ŀ' | 'ł' => 'l',
        'ñ' | 'ń' | 'ņ' | 'ň' => 'n',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => 'o',
        'ŕ' | 'ŗ' | 'ř' => 'r',
        'ś' | 'ŝ' | 'ş' | 'š' => 's',
        'ţ' | 'ť' | 'ŧ' => 't',
        'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => 'u',
        'ŵ' => 'w',
        'ý' | 'ÿ' | 'ŷ' => 'y',
        'ź' | 'ż' | 'ž' => 'z',
        autre => autre,
    }
}

/// Jetons normalisés d'un texte.
fn jetons(texte: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut courant = String::new();
    for c in texte.chars() {
        if c.is_alphanumeric() {
            courant.extend(c.to_lowercase().map(plier));
        } else if !courant.is_empty() {
            out.push(std::mem::take(&mut courant));
        }
    }
    if !courant.is_empty() {
        out.push(courant);
    }
    out
}

/// Terme de requête : mot exact ou préfixe (`terme*`).
struct Terme {
    texte: String,
    prefixe: bool,
}

/// Requête analysée (vide si aucun terme).
pub(crate) struct Requete {
    termes: Vec<Terme>,
}

impl Requete {
    pub(crate) fn analyser(requete: &str) -> Self {
        let mut termes = Vec::new();
        for mot in requete.split_whitespace() {
            let prefixe = mot.ends_with('*');
            let js = jetons(mot);
            let n = js.len();
            for (i, j) in js.into_iter().enumerate() {
                termes.push(Terme {
                    texte: j,
                    prefixe: prefixe && i + 1 == n,
                });
            }
        }
        Self { termes }
    }

    pub(crate) fn est_vide(&self) -> bool {
        self.termes.is_empty()
    }

    /// Score de `texte` (`None` si un terme manque).
    pub(crate) fn score(&self, texte: &str) -> Option<f64> {
        if self.termes.is_empty() {
            return None;
        }
        let js = jetons(texte);
        if js.is_empty() {
            return None;
        }
        let mut total = 0_usize;
        for t in &self.termes {
            let n = js
                .iter()
                .filter(|j| {
                    if t.prefixe {
                        j.starts_with(&t.texte)
                    } else {
                        **j == t.texte
                    }
                })
                .count();
            if n == 0 {
                return None;
            }
            total += n;
        }
        #[allow(clippy::cast_precision_loss)]
        Some(total as f64 / js.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casse_et_accents_ignores() {
        let q = Requete::analyser("METRO");
        assert!(q.score("Jean préfère le métro").is_some());
        assert!(Requete::analyser("préfère").score("prefere").is_some());
    }

    #[test]
    fn tous_les_termes_et_prefixe() {
        let q = Requete::analyser("chat mochi");
        assert!(q.score("le chat s'appelle Mochi").is_some());
        assert!(q.score("le chat s'appelle Tama").is_none());
        assert!(Requete::analyser("moch*").score("Mochi-San").is_some());
        assert!(Requete::analyser("moch").score("Mochi-San").is_none());
        assert!(Requete::analyser("   ").est_vide());
    }

    #[test]
    fn rang_plus_dense_d_abord() {
        let q = Requete::analyser("réunion");
        let court = q.score("réunion demain").unwrap_or_default();
        let long = q
            .score("rappelle moi la réunion de demain matin au bureau")
            .unwrap_or_default();
        assert!(court > long);
    }
}
