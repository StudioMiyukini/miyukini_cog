//! @id     miyucdr_json
//! @do     reserialiser_json_analyse
//! @role   securite
//! @layer  toolkit
//! @human  JSON : analyse stricte, profondeur bornée, chaînes nettoyées, resérialisation.

use serde_json::{Map, Value};

use crate::texte::{nettoyer, utf8, Compteurs};
use crate::{Limites, Rapport, Refus};

/// Profondeur maximale d'imbrication.
fn profondeur(v: &Value) -> usize {
    match v {
        Value::Array(a) => 1 + a.iter().map(profondeur).max().unwrap_or(0),
        Value::Object(o) => 1 + o.values().map(profondeur).max().unwrap_or(0),
        _ => 0,
    }
}

fn assainir(v: Value, nfc: bool, c: &mut Compteurs) -> Value {
    match v {
        Value::String(s) => Value::String(nettoyer(&s, nfc, false, c)),
        Value::Array(a) => Value::Array(a.into_iter().map(|e| assainir(e, nfc, c)).collect()),
        Value::Object(o) => {
            let mut m = Map::new();
            for (k, e) in o {
                let cle = nettoyer(&k, nfc, false, c);
                m.insert(cle, assainir(e, nfc, c));
            }
            Value::Object(m)
        }
        autre => autre,
    }
}

/// Analyse puis resérialise : seule la valeur analysée traverse
/// (commentaires, espaces, clés dupliquées, BOM et octets finaux écartés).
pub(crate) fn reconstruire(
    octets: &[u8],
    limites: &Limites,
    rapport: &mut Rapport,
) -> Result<Vec<u8>, Refus> {
    let texte = utf8(octets)?;
    let texte = texte.strip_prefix('\u{FEFF}').unwrap_or(texte);
    // serde_json borne déjà la récursion (128) ; la limite propre est vérifiée ensuite.
    let valeur: Value =
        serde_json::from_str(texte).map_err(|e| Refus::Malforme(format!("JSON : {e}")))?;
    let p = profondeur(&valeur);
    if p > limites.json_profondeur_max {
        return Err(Refus::LimiteDepassee(format!(
            "profondeur JSON {p} > {}",
            limites.json_profondeur_max
        )));
    }
    let mut c = Compteurs::default();
    let valeur = assainir(valeur, limites.normaliser_nfc, &mut c);
    c.consigner(rapport);
    rapport
        .retraits
        .push("JSON resérialisé (forme compacte)".into());
    serde_json::to_vec(&valeur).map_err(|e| Refus::Interne(format!("JSON : {e}")))
}
