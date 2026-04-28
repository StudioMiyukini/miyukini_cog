//! Types pour Jay Sheets (tableur collaboratif).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Données d'une feuille de calcul.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SheetData {
    /// Cellules indexées par (row, col).
    #[serde(with = "cell_map")]
    pub cells: HashMap<(u32, u32), Cell>,
    /// Nombre de colonnes affichées.
    pub num_cols: u32,
    /// Nombre de lignes affichées.
    pub num_rows: u32,
    /// Nom de la feuille (pour multi-feuilles dans le futur).
    pub name: String,
}

/// Cellule.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cell {
    pub value: CellValue,
    /// Formule si présente (commençant par `=`).
    pub formula: Option<String>,
    pub format: CellFormat,
}

/// Valeur d'une cellule (typée).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum CellValue {
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "number")]
    Number(f64),
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "date")]
    Date(chrono::NaiveDate),
    #[serde(rename = "error")]
    Error(String),
}

impl Default for CellValue {
    fn default() -> Self {
        Self::Empty
    }
}

/// Formatage d'une cellule (style d'affichage).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CellFormat {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Option<String>,
    pub background: Option<String>,
    pub align: Option<String>,
    /// Pattern de format numérique (ex: "#,##0.00", "0.00%", "YYYY-MM-DD").
    pub number_format: Option<String>,
}

impl SheetData {
    /// Crée une feuille vide avec les dimensions données.
    pub fn new(name: impl Into<String>, rows: u32, cols: u32) -> Self {
        Self {
            cells: HashMap::new(),
            num_rows: rows,
            num_cols: cols,
            name: name.into(),
        }
    }

    /// Récupère une cellule (retourne vide si inexistante).
    pub fn get(&self, row: u32, col: u32) -> Cell {
        self.cells.get(&(row, col)).cloned().unwrap_or_default()
    }

    /// Définit une cellule.
    pub fn set(&mut self, row: u32, col: u32, cell: Cell) {
        self.cells.insert((row, col), cell);
        if row >= self.num_rows {
            self.num_rows = row + 1;
        }
        if col >= self.num_cols {
            self.num_cols = col + 1;
        }
    }

    /// Convertit une colonne en nom A, B, C, ..., AA, AB...
    pub fn col_name(col: u32) -> String {
        let mut name = String::new();
        let mut c = col;
        loop {
            name.insert(0, (b'A' + (c % 26) as u8) as char);
            if c < 26 {
                break;
            }
            c = c / 26 - 1;
        }
        name
    }
}

/// Serde helper pour la HashMap à clés tuple.
mod cell_map {
    use super::Cell;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    pub fn serialize<S: Serializer>(
        map: &HashMap<(u32, u32), Cell>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        let v: Vec<(u32, u32, Cell)> =
            map.iter().map(|(&(r, c), v)| (r, c, v.clone())).collect();
        v.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<HashMap<(u32, u32), Cell>, D::Error> {
        let v: Vec<(u32, u32, Cell)> = Vec::deserialize(d)?;
        Ok(v.into_iter().map(|(r, c, cell)| ((r, c), cell)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_cell() {
        let mut sheet = SheetData::new("Sheet1", 10, 5);
        let cell = Cell {
            value: CellValue::Number(42.0),
            ..Default::default()
        };
        sheet.set(3, 2, cell);
        let got = sheet.get(3, 2);
        assert!(matches!(got.value, CellValue::Number(n) if n == 42.0));
    }

    #[test]
    fn col_names() {
        assert_eq!(SheetData::col_name(0), "A");
        assert_eq!(SheetData::col_name(25), "Z");
        assert_eq!(SheetData::col_name(26), "AA");
        assert_eq!(SheetData::col_name(27), "AB");
        assert_eq!(SheetData::col_name(51), "AZ");
        assert_eq!(SheetData::col_name(52), "BA");
    }

    #[test]
    fn auto_grow_dimensions() {
        let mut sheet = SheetData::new("S", 5, 5);
        sheet.set(
            10,
            10,
            Cell {
                value: CellValue::Text("x".into()),
                ..Default::default()
            },
        );
        assert_eq!(sheet.num_rows, 11);
        assert_eq!(sheet.num_cols, 11);
    }
}
