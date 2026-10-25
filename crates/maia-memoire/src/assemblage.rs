//! @id     maia_memoire_assemblage
//! @do     assembler_contexte_sous_budget
//! @role   orchestration
//! @layer  8
//! @human  maia-memoire — strates N0–N3, format compact une ligne par unité, assemblage sous budget mesuré, zoom `ouvrir` (§2, §5, §7, EX-50, EX-51).

//! Assemblage déterministe (profil S0, sans LLM) :
//!
//! 1. **N0** toujours (règles, identité `moi.*`, préférences fortes), plafonné à 200 jetons ;
//! 2. **N1** : une entrée par domaine, ordre alphabétique (préfixe stable pour le cache Strata) ;
//! 3. **N2 / N3** par score décroissant (pertinence × importance × fraîcheur × confiance),
//!    au plus [`MAX_PAR_SUJET`] notes d'un même sujet ;
//! 4. le reste → une ligne `…` avec les identifiants de branche à ouvrir.
//!
//! Le bloc entier, balises comprises, est mesuré par le [`Tokeniseur`] : jamais au-delà du budget.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::erreurs::ErreurMemoire;
use crate::ident::{Chemin, IdMemoire, TypeUnite};
use crate::jetons::{Tokeniseur, MAX_JETONS_N0, MAX_JETONS_N1_DOMAINE, MAX_JETONS_N2, MAX_JETONS_N3};
use crate::types::{Etiquette, Note, ETIQUETTE};

/// Domaine réservé à l'identité de l'utilisateur (chargé en N0).
pub const DOMAINE_IDENTITE: &str = "moi";
/// Importance à partir de laquelle une préférence est « forte » (N0).
pub const SEUIL_PREFERENCE_FORTE: u8 = 8;
/// Confiance affichée seulement en dessous de ce seuil (§5).
pub const SEUIL_CONFIANCE_AFFICHEE: u8 = 7;
/// Diversité : notes d'un même sujet (`domaine.sujet`) au plus par bloc (§6).
pub const MAX_PAR_SUJET: usize = 3;
/// Balise ouvrante du bloc (données délimitées, §5, §10).
pub const BALISE_OUVRANTE: &str = "<memoire>";
/// Balise fermante du bloc.
pub const BALISE_FERMANTE: &str = "</memoire>";

/// Strate d'une note (§2). N4 (sources) n'est jamais injectée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Strate {
    /// Noyau, toujours chargé.
    N0,
    /// Carte : un résumé par domaine.
    N1,
    /// Sujets : résumés de sujet, projet, personne.
    N2,
    /// Notes atomiques.
    N3,
}

/// Strate d'une note.
pub fn strate(note: &Note) -> Strate {
    let t = note.type_unite();
    if t == TypeUnite::Regle
        || note.id.domaine() == DOMAINE_IDENTITE
        || (t == TypeUnite::Preference && note.importance >= SEUIL_PREFERENCE_FORTE)
    {
        Strate::N0
    } else if t == TypeUnite::Resume && note.id.branche().profondeur() == 1 {
        Strate::N1
    } else if t == TypeUnite::Resume {
        Strate::N2
    } else {
        Strate::N3
    }
}

/// Plafond de texte d'une unité selon son type et sa profondeur (§2).
pub fn plafond_texte(type_unite: TypeUnite, profondeur_branche: usize) -> (&'static str, usize) {
    match (type_unite, profondeur_branche) {
        (TypeUnite::Resume, 1) => ("N1", MAX_JETONS_N1_DOMAINE),
        (TypeUnite::Resume, _) => ("N2", MAX_JETONS_N2),
        _ => ("N3", MAX_JETONS_N3),
    }
}

/// Neutralise un texte pour le bloc : pas de balise, pas de saut de ligne (§10).
pub fn neutraliser(texte: &str) -> String {
    texte
        .chars()
        .map(|c| match c {
            '<' => '‹',
            '>' => '›',
            c if c.is_control() => ' ',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Date relative (§5).
pub fn date_relative(d: DateTime<Utc>, maintenant: DateTime<Utc>) -> String {
    let j = (maintenant - d).num_days();
    if j < 1 {
        "aujourd'hui".to_string()
    } else if j < 60 {
        format!("il y a {j} j")
    } else if j < 730 {
        format!("il y a {} mois", j / 30)
    } else {
        format!("il y a {} ans", j / 365)
    }
}

/// Ligne compacte : `id texte [· c<confiance>] [· <date relative>] [· ↗<ids liés>]`.
pub fn ligne(note: &Note, maintenant: DateTime<Utc>) -> String {
    let mut s = format!("{} {}", note.id, neutraliser(&note.texte));
    if note.confiance < SEUIL_CONFIANCE_AFFICHEE {
        let _ = write!(s, " · c{}", note.confiance);
    }
    if note.type_unite() != TypeUnite::Resume {
        s.push_str(" · ");
        s.push_str(&date_relative(note.cree_le, maintenant));
    }
    if !note.liens.is_empty() {
        let liens: Vec<&str> = note.liens.iter().map(IdMemoire::as_str).collect();
        s.push_str(" · ↗");
        s.push_str(&liens.join(","));
    }
    s
}

/// Bloc mémoire assemblé.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BlocMemoire {
    /// Texte à injecter (données délimitées).
    pub texte: String,
    /// Jetons mesurés du texte entier.
    pub jetons: usize,
    /// Budget demandé.
    pub budget: usize,
    /// Tokenizer utilisé.
    pub tokeniseur: String,
    /// Notes rendues.
    pub inclus: Vec<IdMemoire>,
    /// Notes non rendues (résumées dans la ligne `…`).
    pub omis: Vec<IdMemoire>,
    /// Étiquette (`nationale`).
    pub etiquette: Etiquette,
}

/// Rendu d'un bloc à partir de ses lignes et de la ligne de reste.
fn rendre(lignes: &[String], reste: Option<&str>) -> String {
    let mut t = String::from(BALISE_OUVRANTE);
    for l in lignes {
        t.push('\n');
        t.push_str(l);
    }
    if let Some(r) = reste {
        t.push('\n');
        t.push_str(r);
    }
    t.push('\n');
    t.push_str(BALISE_FERMANTE);
    t
}

/// Ligne `…` la plus longue qui tient : branches des notes omises, avec leur nombre.
fn ligne_reste(
    lignes: &[String],
    omis: &[IdMemoire],
    budget: usize,
    tok: &dyn Tokeniseur,
) -> Option<String> {
    if omis.is_empty() {
        return None;
    }
    let mut par_branche: BTreeMap<String, usize> = BTreeMap::new();
    for id in omis {
        *par_branche.entry(id.branche().as_str().to_string()).or_default() += 1;
    }
    let mut entrees: Vec<String> = par_branche
        .into_iter()
        .map(|(b, n)| format!("{b}({n})"))
        .collect();
    loop {
        let r = if entrees.is_empty() {
            "…".to_string()
        } else {
            format!("… {}", entrees.join(" "))
        };
        if entrees.is_empty() || tok.compter(&rendre(lignes, Some(&r))) <= budget {
            return Some(r);
        }
        entrees.pop();
    }
}

/// Termes de recherche normalisés (minuscules, sans accents, ≥ 3 caractères).
fn termes(texte: &str) -> BTreeSet<String> {
    texte
        .split(|c: char| !c.is_alphanumeric())
        .map(replier)
        .filter(|t| t.chars().count() >= 3)
        .collect()
}

/// Repli en minuscules sans accents (comparaison seulement).
fn replier(mot: &str) -> String {
    mot.to_lowercase()
        .chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            c => c,
        })
        .collect()
}

/// Score de rappel = pertinence × importance × fraîcheur × confiance (§6).
fn score(note: &Note, requete: &BTreeSet<String>, maintenant: DateTime<Utc>) -> f64 {
    let pertinence = if requete.is_empty() {
        1.0
    } else {
        let mots = termes(&format!("{} {}", note.id.as_str().replace('.', " "), note.texte));
        let trouves = requete
            .iter()
            .filter(|t| mots.iter().any(|m| m.starts_with(t.as_str()) || t.starts_with(m.as_str())))
            .count();
        0.1 + 0.9 * (trouves as f64 / requete.len() as f64)
    };
    let importance = (f64::from(note.importance) + 1.0) / 10.0;
    let confiance = (f64::from(note.confiance) + 1.0) / 10.0;
    let reference = note.dernier_usage.map_or(note.enregistre_le, |u| u.max(note.enregistre_le));
    let age = (maintenant - reference).num_days().max(0) as f64;
    let fraicheur = 1.0 / (1.0 + age / 30.0);
    pertinence * importance * fraicheur * confiance
}

/// Unités de N0 dans l'ordre stable : règles, identité, préférences fortes.
fn unites_n0<'a>(notes: &[&'a Note]) -> Vec<&'a Note> {
    let rang = |n: &Note| match n.type_unite() {
        TypeUnite::Regle => 0,
        _ if n.id.domaine() == DOMAINE_IDENTITE => 1,
        _ => 2,
    };
    let mut v: Vec<&Note> = notes.iter().copied().filter(|n| strate(n) == Strate::N0).collect();
    v.sort_by(|a, b| {
        rang(a)
            .cmp(&rang(b))
            .then(b.importance.cmp(&a.importance))
            .then(a.id.cmp(&b.id))
    });
    v
}

/// Ligne N0 bornée à [`MAX_JETONS_N0`] ; renvoie aussi les unités qui n'y tiennent pas.
fn ligne_n0<'a>(unites: &[&'a Note], tok: &dyn Tokeniseur) -> (String, Vec<&'a Note>, Vec<&'a Note>) {
    let mut ligne = String::from("N0");
    let mut pris = Vec::new();
    let mut omis = Vec::new();
    for n in unites {
        let mut t = neutraliser(&n.texte);
        if !t.ends_with(['.', '!', '?', '…']) {
            t.push('.');
        }
        let essai = format!("{ligne} {t}");
        if omis.is_empty() && tok.compter(&essai) <= MAX_JETONS_N0 {
            ligne = essai;
            pris.push(*n);
        } else {
            omis.push(*n);
        }
    }
    if pris.is_empty() && omis.is_empty() {
        ligne.push_str(" (vide)");
    }
    (ligne, pris, omis)
}

/// Entrées N1 par domaine, ordre alphabétique (préfixe stable), avec le résumé rendu.
fn entrees_n1(notes: &[&Note]) -> Vec<(String, Option<IdMemoire>)> {
    let mut domaines: BTreeMap<&str, (Option<&Note>, usize)> = BTreeMap::new();
    for n in notes {
        match strate(n) {
            Strate::N0 => {}
            Strate::N1 => {
                let e = domaines.entry(n.id.domaine()).or_default();
                if e.0.is_none_or(|r| n.id < r.id) {
                    e.0 = Some(n);
                }
            }
            Strate::N2 | Strate::N3 => domaines.entry(n.id.domaine()).or_default().1 += 1,
        }
    }
    domaines
        .into_iter()
        .map(|(d, (resume, compte))| match resume {
            Some(r) => (format!("{d}({})", neutraliser(&r.texte)), Some(r.id.clone())),
            None => (format!("{d}({compte} notes)"), None),
        })
        .collect()
}

/// Assemble le contexte mémoire sous `budget` jetons (EX-50).
pub fn assembler(
    courantes: &[&Note],
    requete: &str,
    budget: usize,
    tok: &dyn Tokeniseur,
    maintenant: DateTime<Utc>,
) -> Result<BlocMemoire, ErreurMemoire> {
    let n0 = unites_n0(courantes);
    let (ligne0, pris0, omis0) = ligne_n0(&n0, tok);
    let mut lignes = vec![ligne0];
    let mut inclus: Vec<IdMemoire> = pris0.iter().map(|n| n.id.clone()).collect();
    let mut omis: Vec<IdMemoire> = omis0.iter().map(|n| n.id.clone()).collect();

    // N0 toujours présent : sans lui, aucun bloc (fail-closed).
    let minimal = tok.compter(&rendre(&lignes, Some("…")));
    if minimal > budget {
        return Err(ErreurMemoire::BudgetInsuffisant { budget, requis: minimal });
    }
    let tient = |lignes: &[String]| tok.compter(&rendre(lignes, Some("…"))) <= budget;

    // N1 compressée : autant de domaines que le budget le permet.
    let mut ligne1 = String::from("N1");
    let mut sep = " ";
    let mut resumes_n1 = Vec::new();
    let mut n1_omis = false;
    for (e, resume) in entrees_n1(courantes) {
        let essai = format!("{ligne1}{sep}{e}");
        let mut avec = lignes.clone();
        avec.push(essai.clone());
        if n1_omis || !tient(&avec) {
            n1_omis = true;
            omis.extend(resume);
            continue;
        }
        ligne1 = essai;
        sep = " · ";
        resumes_n1.extend(resume);
    }
    if ligne1 == "N1" {
        ligne1.push_str(" (vide)");
    }
    let mut avec = lignes.clone();
    avec.push(ligne1);
    if tient(&avec) {
        lignes = avec;
        inclus.extend(resumes_n1);
    } else {
        omis.extend(resumes_n1);
    }

    // N2 / N3 par score décroissant, diversité par sujet.
    let q = termes(requete);
    let mut pool: Vec<(&Note, f64)> = courantes
        .iter()
        .copied()
        .filter(|n| matches!(strate(n), Strate::N2 | Strate::N3))
        .map(|n| (n, score(n, &q, maintenant)))
        .collect();
    pool.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.id.cmp(&b.0.id)));
    let mut par_sujet: BTreeMap<String, usize> = BTreeMap::new();
    for (n, _) in pool {
        let sujet = n.id.branche().tronquer(2).as_str().to_string();
        let compte = par_sujet.entry(sujet).or_default();
        if *compte >= MAX_PAR_SUJET {
            omis.push(n.id.clone());
            continue;
        }
        let mut avec = lignes.clone();
        avec.push(ligne(n, maintenant));
        if tient(&avec) {
            lignes = avec;
            *compte += 1;
            inclus.push(n.id.clone());
        } else {
            omis.push(n.id.clone());
        }
    }

    omis.sort();
    let reste = ligne_reste(&lignes, &omis, budget, tok);
    let texte = rendre(&lignes, reste.as_deref());
    let jetons = tok.compter(&texte);
    if jetons > budget {
        // Garde finale : impossible par construction, refus plutôt que dépassement.
        return Err(ErreurMemoire::BudgetInsuffisant { budget, requis: jetons });
    }
    Ok(BlocMemoire {
        texte,
        jetons,
        budget,
        tokeniseur: tok.nom().to_string(),
        inclus,
        omis,
        etiquette: ETIQUETTE,
    })
}

/// Zoom à la demande sur une note ou une branche (`memoire.ouvrir(id)`, EX-51) :
/// lecture seule, sous budget.
pub fn ouvrir(
    courantes: &[&Note],
    chemin: &Chemin,
    budget: usize,
    tok: &dyn Tokeniseur,
    maintenant: DateTime<Utc>,
) -> Result<BlocMemoire, ErreurMemoire> {
    let mut designees: Vec<&Note> = courantes.iter().copied().filter(|n| chemin.designe(&n.id)).collect();
    designees.sort_by(|a, b| a.id.cmp(&b.id));
    if designees.is_empty() {
        return Err(ErreurMemoire::NoteInconnue(chemin.as_str().to_string()));
    }
    let mut lignes = vec![format!("ouverture {}", chemin.as_str())];
    let minimal = tok.compter(&rendre(&lignes, Some("…")));
    if minimal > budget {
        return Err(ErreurMemoire::BudgetInsuffisant { budget, requis: minimal });
    }
    let mut inclus = Vec::new();
    let mut omis = Vec::new();
    for n in designees {
        let mut avec = lignes.clone();
        avec.push(ligne(n, maintenant));
        if omis.is_empty() && tok.compter(&rendre(&avec, Some("…"))) <= budget {
            lignes = avec;
            inclus.push(n.id.clone());
        } else {
            omis.push(n.id.clone());
        }
    }
    let reste = ligne_reste(&lignes, &omis, budget, tok);
    let texte = rendre(&lignes, reste.as_deref());
    let jetons = tok.compter(&texte);
    Ok(BlocMemoire {
        texte,
        jetons,
        budget,
        tokeniseur: tok.nom().to_string(),
        inclus,
        omis,
        etiquette: ETIQUETTE,
    })
}
