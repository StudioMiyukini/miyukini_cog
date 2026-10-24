//! @id     miyucdr_tableur
//! @do     reecrire_csv_strict
//! @role   securite
//! @layer  toolkit
//! @human  CSV : analyse stricte RFC 4180, formules neutralisées, réécriture canonique.

use crate::texte::{nettoyer, utf8, Compteurs};
use crate::{Limites, Rapport, Refus};

/// Séparateur le plus fréquent sur la première ligne parmi `,` `;` tabulation.
fn separateur(texte: &str) -> char {
    let ligne = texte.lines().next().unwrap_or("");
    let mut meilleur = (',', 0usize);
    for s in [',', ';', '\t'] {
        let n = ligne.matches(s).count();
        if n > meilleur.1 {
            meilleur = (s, n);
        }
    }
    meilleur.0
}

/// Analyse stricte : guillemet non fermé, guillemet au milieu d'un champ non
/// cité ou caractère après un guillemet fermant ⇒ refus.
fn analyser(texte: &str, sep: char, limites: &Limites) -> Result<Vec<Vec<String>>, Refus> {
    let mut lignes: Vec<Vec<String>> = Vec::new();
    let mut ligne: Vec<String> = Vec::new();
    let mut champ = String::new();
    let mut car = texte.chars().peekable();
    let mut numero = 1usize;

    let pousser_champ = |ligne: &mut Vec<String>, champ: &mut String| -> Result<(), Refus> {
        if ligne.len() >= limites.csv_champs_max {
            return Err(Refus::LimiteDepassee(format!(
                "plus de {} champs par enregistrement CSV",
                limites.csv_champs_max
            )));
        }
        ligne.push(std::mem::take(champ));
        Ok(())
    };

    while car.peek().is_some() {
        // Début de champ.
        if car.peek() == Some(&'"') {
            car.next();
            loop {
                match car.next() {
                    None => {
                        return Err(Refus::Malforme(format!(
                            "CSV : guillemet non fermé (ligne {numero})"
                        )))
                    }
                    Some('"') if car.peek() == Some(&'"') => {
                        car.next();
                        champ.push('"');
                    }
                    Some('"') => break,
                    Some(c) => {
                        if c == '\n' {
                            numero += 1;
                        }
                        champ.push(c);
                    }
                }
            }
            match car.peek() {
                None | Some('\n') => {}
                Some(&c) if c == sep => {}
                Some(_) => {
                    return Err(Refus::Malforme(format!(
                        "CSV : caractère après un guillemet fermant (ligne {numero})"
                    )))
                }
            }
        } else {
            while let Some(&c) = car.peek() {
                if c == sep || c == '\n' {
                    break;
                }
                if c == '"' {
                    return Err(Refus::Malforme(format!(
                        "CSV : guillemet dans un champ non cité (ligne {numero})"
                    )));
                }
                champ.push(c);
                car.next();
            }
        }
        // Fin de champ.
        match car.next() {
            Some('\n') | None => {
                pousser_champ(&mut ligne, &mut champ)?;
                if lignes.len() >= limites.csv_enregistrements_max {
                    return Err(Refus::LimiteDepassee(format!(
                        "plus de {} enregistrements CSV",
                        limites.csv_enregistrements_max
                    )));
                }
                lignes.push(std::mem::take(&mut ligne));
                numero += 1;
            }
            Some(_) => {
                pousser_champ(&mut ligne, &mut champ)?;
                // Séparateur final : champ vide en fin de ligne.
                if matches!(car.peek(), None | Some('\n')) {
                    pousser_champ(&mut ligne, &mut champ)?;
                    car.next();
                    lignes.push(std::mem::take(&mut ligne));
                    numero += 1;
                }
            }
        }
    }
    Ok(lignes)
}

/// Une cellule commençant par `=`, `+`, `-`, `@` ou tabulation est interprétée
/// comme formule par les tableurs (injection CSV / DDE) ; les nombres sont exemptés.
fn neutraliser(champ: &str) -> Option<String> {
    let premier = champ.chars().next()?;
    if !matches!(premier, '=' | '+' | '-' | '@' | '\t') {
        return None;
    }
    if matches!(premier, '+' | '-') && champ.parse::<f64>().is_ok() {
        return None;
    }
    Some(format!("'{champ}"))
}

fn ecrire_champ(sortie: &mut String, champ: &str, sep: char) {
    if champ.contains(sep) || champ.contains('"') || champ.contains('\n') || champ.contains('\r') {
        sortie.push('"');
        sortie.push_str(&champ.replace('"', "\"\""));
        sortie.push('"');
    } else {
        sortie.push_str(champ);
    }
}

/// CSV : texte nettoyé, analysé strictement puis réécrit champ par champ.
pub(crate) fn reconstruire(
    octets: &[u8],
    limites: &Limites,
    rapport: &mut Rapport,
) -> Result<Vec<u8>, Refus> {
    let texte = utf8(octets)?;
    let mut c = Compteurs::default();
    let propre = nettoyer(texte, limites.normaliser_nfc, true, &mut c);
    c.consigner(rapport);

    let sep = separateur(&propre);
    let lignes = analyser(&propre, sep, limites)?;

    let mut neutralisees = 0usize;
    let mut sortie = String::with_capacity(propre.len() + 16);
    for ligne in &lignes {
        for (i, champ) in ligne.iter().enumerate() {
            if i > 0 {
                sortie.push(sep);
            }
            match neutraliser(champ) {
                Some(n) => {
                    neutralisees += 1;
                    ecrire_champ(&mut sortie, &n, sep);
                }
                None => ecrire_champ(&mut sortie, champ, sep),
            }
        }
        sortie.push('\n');
    }
    if neutralisees > 0 {
        rapport.retraits.push(format!(
            "{neutralisees} cellules débutant par une formule neutralisées (préfixe ')"
        ));
    }
    rapport.retraits.push(format!(
        "CSV réécrit : {} enregistrements, séparateur {:?}",
        lignes.len(),
        sep
    ));
    Ok(sortie.into_bytes())
}
