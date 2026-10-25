//! Entrepôt de la mémoire : KindMother (persistance) ou mémoire vive (essais).
//!
//! Les deux reçoivent exactement les mêmes **intentions typées** et **lectures
//! typées** (`kindmother_client`) : le store n'émet jamais de texte SQL.

// @id service.alicia.memory.entrepot
// @role memory_backend
// @layer 7
// @do route_typed_intents_to_backend
// @human Aiguillage des lectures et écritures typées de la mémoire d'Alicia.

use std::collections::HashMap;

use kindmother_client::{TypedSelect, TypedWriteIntent};
use serde_json::Value;

use crate::errors::MemoryError;
use crate::kindmother::EntrepotKindMother;
use crate::vif::EntrepotVif;

/// Ligne lue : colonne → valeur JSON (format des lectures KindMother).
pub(crate) type Ligne = HashMap<String, Value>;

/// Intentions au plus par lot (borne KindMother : 1 000).
pub(crate) const MAX_LOT: usize = 900;

/// Taille de page des lectures complètes.
const PAGE: u32 = 1000;

pub(crate) enum Entrepot {
    /// Mémoire vive : rien sur disque.
    Vif(EntrepotVif),
    /// KindMother, seul écrivain (K-13).
    KindMother(Box<EntrepotKindMother>),
}

impl Entrepot {
    pub(crate) fn select(&self, q: TypedSelect) -> Result<Vec<Ligne>, MemoryError> {
        match self {
            Entrepot::Vif(v) => v.select(&q),
            Entrepot::KindMother(k) => k.select(q),
        }
    }

    /// Lot atomique ; retourne le nombre de lignes touchées par intention.
    pub(crate) fn soumettre(
        &self,
        intents: Vec<TypedWriteIntent>,
    ) -> Result<Vec<u64>, MemoryError> {
        if intents.is_empty() {
            return Ok(Vec::new());
        }
        if intents.len() > MAX_LOT {
            return Err(MemoryError::KindMother(format!(
                "lot trop grand ({} intentions, maximum {MAX_LOT})",
                intents.len()
            )));
        }
        match self {
            Entrepot::Vif(v) => v.soumettre(&intents),
            Entrepot::KindMother(k) => k.soumettre(intents),
        }
    }

    /// Lit toutes les lignes de `table` vérifiant `filtre`, par pages, dans
    /// l'ordre `ordre` (`(colonne, décroissant)`), qui doit se terminer par la
    /// clé primaire pour des pages stables.
    pub(crate) fn toutes(
        &self,
        table: &str,
        ordre: &[(&str, bool)],
        filtre: impl Fn(TypedSelect) -> TypedSelect,
    ) -> Result<Vec<Ligne>, MemoryError> {
        let mut out = Vec::new();
        let mut offset = 0_u32;
        loop {
            let mut q = filtre(TypedSelect::from(table));
            for (c, desc) in ordre {
                q = q.order_by(*c, *desc);
            }
            q = q.limit(PAGE);
            q.offset = Some(offset);
            let page = self.select(q)?;
            let n = page.len();
            out.extend(page);
            if n < PAGE as usize {
                return Ok(out);
            }
            offset = offset
                .checked_add(PAGE)
                .ok_or_else(|| MemoryError::KindMother(format!("table {table} trop grande")))?;
        }
    }
}
