//! @id     miyuhybride_db_secondaire
//! @do     borner_db_secondaire_facade
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — DB secondaire d'une façade : vierge à chaque session, bornée par le contrat, purgée à la destruction, jamais autoritaire (EX-29, EX-43).

//! # DB secondaire
//!
//! MODELE-ZONES §9 : la DB secondaire en Franche sert de **tampon** ; elle ne
//! contient que les données partagées dans la session, est **purgée** en fin
//! de session, et rien de ce qu'elle contient ne fait foi (la validation qui
//! compte est faite en zone COG).
//!
//! Implémentation en mémoire, bornée à la taille déclarée par le contrat
//! (clés + valeurs). Le chemin disque de la sandbox Franche
//! (`miyudouane::Sandbox::db_secondaire`) est le seul emplacement inscriptible
//! d'une façade ; un stockage persistant y viendra derrière la même interface.

use std::collections::BTreeMap;

use crate::erreurs::ErreurHybride;

/// Base secondaire d'une session de façade.
#[derive(Debug)]
pub struct DbSecondaire {
    capacite: usize,
    utilise: usize,
    lignes: BTreeMap<(String, Vec<u8>), Vec<u8>>,
}

impl DbSecondaire {
    /// Base **vierge** de `capacite` octets.
    pub fn vierge(capacite: usize) -> Self {
        Self { capacite, utilise: 0, lignes: BTreeMap::new() }
    }

    fn poids(table: &str, cle: &[u8], valeur: &[u8]) -> usize {
        table.len() + cle.len() + valeur.len()
    }

    /// Écrit (ou remplace) une ligne ; refus si la capacité serait dépassée.
    pub fn ecrire(&mut self, table: &str, cle: &[u8], valeur: Vec<u8>) -> Result<(), ErreurHybride> {
        let k = (table.to_string(), cle.to_vec());
        let ancien = self.lignes.get(&k).map_or(0, |v| Self::poids(table, cle, v));
        let nouveau = Self::poids(table, cle, &valeur);
        let total = self.utilise - ancien + nouveau;
        if total > self.capacite {
            return Err(ErreurHybride::BaseSecondairePleine);
        }
        self.utilise = total;
        self.lignes.insert(k, valeur);
        Ok(())
    }

    /// Lit une ligne.
    pub fn lire(&self, table: &str, cle: &[u8]) -> Option<&[u8]> {
        self.lignes.get(&(table.to_string(), cle.to_vec())).map(Vec::as_slice)
    }

    /// Retire une ligne.
    pub fn retirer(&mut self, table: &str, cle: &[u8]) -> Option<Vec<u8>> {
        let v = self.lignes.remove(&(table.to_string(), cle.to_vec()))?;
        self.utilise -= Self::poids(table, cle, &v);
        Some(v)
    }

    /// Lignes d'une table, par clé croissante.
    pub fn table<'a>(&'a self, table: &'a str) -> impl Iterator<Item = (&'a [u8], &'a [u8])> + 'a {
        self.lignes.iter().filter(move |((t, _), _)| t == table).map(|((_, k), v)| (k.as_slice(), v.as_slice()))
    }

    /// Vide une table.
    pub fn vider_table(&mut self, table: &str) {
        let cles: Vec<Vec<u8>> = self.table(table).map(|(k, _)| k.to_vec()).collect();
        for k in cles {
            self.retirer(table, &k);
        }
    }

    /// Purge complète (fin de session, coupure).
    pub fn purger(&mut self) {
        self.lignes.clear();
        self.utilise = 0;
    }

    /// Octets utilisés.
    pub fn utilise(&self) -> usize {
        self.utilise
    }

    /// Vrai si vide.
    pub fn est_vide(&self) -> bool {
        self.lignes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bornee_et_purgeable() {
        let mut db = DbSecondaire::vierge(16);
        db.ecrire("t", b"a", vec![0; 10]).unwrap();
        assert_eq!(db.utilise(), 12);
        assert_eq!(db.ecrire("t", b"b", vec![0; 10]), Err(ErreurHybride::BaseSecondairePleine));
        db.ecrire("t", b"a", vec![1; 4]).unwrap();
        assert_eq!(db.lire("t", b"a"), Some(&[1u8; 4][..]));
        db.ecrire("u", b"b", vec![2]).unwrap();
        db.vider_table("t");
        assert!(db.lire("t", b"a").is_none());
        db.purger();
        assert!(db.est_vide());
        assert_eq!(db.utilise(), 0);
    }
}
