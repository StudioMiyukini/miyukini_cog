//! @id     strongfather_expr
//! @do     evaluer_expressions_de_politique
//! @role   domain_logic
//! @layer  core
//! @human  Petit évaluateur d'expressions sûr pour les conditions et règles de politique.
//!
//! # StrongFather — Évaluateur d'expressions
//!
//! Langage volontairement minuscule, sans `eval`, sans appel de fonction arbitraire,
//! sans boucle ; taille, nombre de jetons et profondeur bornés.
//!
//! ## Grammaire
//!
//! ```text
//! expr    := or
//! or      := and ( "||" and )*
//! and     := unary ( "&&" unary )*
//! unary   := "!" unary | cmp
//! cmp     := primary ( op primary )?      op ∈ == != < <= > >= in
//! primary := "(" expr ")" | "has(" path ")" | liste | littéral | chemin
//! liste   := "[" littéral ( "," littéral )* "]"
//! littéral:= 'texte' | "texte" | entier | true | false
//! ```
//!
//! ## Attributs d'intention (chemins)
//!
//! - `action` (texte canonique : creation, modification, deletion, read, evaluation)
//! - `subject`, `caller`, `origin`, `instance` (texte)
//! - `priority` (entier ; absent ⇒ attribut inconnu)
//! - `data.<clé>`, `meta.<clé>` (texte ; absent ⇒ attribut inconnu)
//! - `constraints` (liste des identifiants de contraintes)
//!
//! ## Typage et échecs
//!
//! - `==`/`!=` : même type ; texte vs entier ⇒ le texte doit s'analyser en entier, sinon erreur
//! - `<`, `<=`, `>`, `>=` : entiers uniquement (texte converti si possible), sinon erreur
//! - `in` : scalaire à gauche, liste à droite
//! - `&&` / `||` court-circuitent (permet `has(data.x) && data.x == '1'`)
//! - Attribut inconnu, erreur de type, expression mal formée ⇒ [`ExprError`] ; l'appelant
//!   (le moteur) traite toute erreur comme un refus (fail-closed, C-12)

use std::fmt;

use crate::intent::Intent;

/// Longueur maximale d'une expression (octets).
pub const MAX_EXPR_LEN: usize = 2048;
/// Nombre maximal de jetons.
pub const MAX_TOKENS: usize = 256;
/// Profondeur maximale d'imbrication.
pub const MAX_DEPTH: usize = 32;
/// Nombre maximal d'éléments d'une liste littérale.
pub const MAX_LIST_LEN: usize = 64;

/// @id strongfather_expr_error
/// @role error
/// @layer core
/// @do represent_expression_error
//@human / @human Erreur d'analyse ou d'évaluation d'une expression. Toujours traitée comme un refus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprError {
    /// Expression mal formée ou dépassant les bornes.
    Syntax(String),
    /// Attribut inconnu ou absent de l'intention.
    UnknownAttribute(String),
    /// Opération entre types incompatibles.
    Type(String),
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprError::Syntax(m) => write!(f, "expression mal formée : {m}"),
            ExprError::UnknownAttribute(a) => write!(f, "attribut inconnu : {a}"),
            ExprError::Type(m) => write!(f, "erreur de type : {m}"),
        }
    }
}

impl std::error::Error for ExprError {}

/// Valeur typée manipulée par l'évaluateur.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
    List(Vec<Value>),
}

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Value::Str(_) => "texte",
            Value::Int(_) => "entier",
            Value::Bool(_) => "booléen",
            Value::List(_) => "liste",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    In,
}

/// Arbre syntaxique.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Lit(Value),
    Path(Vec<String>),
    Has(Vec<String>),
    List(Vec<Value>),
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Cmp(CmpOp, Box<Node>, Box<Node>),
}

/// @id strongfather_expr_compiled
/// @role domain_model
/// @layer core
/// @do hold_compiled_expression
//@human / @human Expression analysée une fois, évaluable sur toute intention (pure, déterministe).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    root: Node,
}

impl Expr {
    /// @id strongfather_expr_parse
    /// @role parser
    /// @layer core
    /// @do parse_expression
    //@human / @human Analyse une expression ; bornes de taille, jetons et profondeur appliquées.
    pub fn parse(source: &str) -> Result<Self, ExprError> {
        if source.len() > MAX_EXPR_LEN {
            return Err(ExprError::Syntax(format!(
                "expression trop longue ({} > {MAX_EXPR_LEN} octets)",
                source.len()
            )));
        }
        let tokens = tokenize(source)?;
        if tokens.is_empty() {
            return Err(ExprError::Syntax("expression vide".to_string()));
        }
        let mut p = Parser {
            tokens,
            pos: 0,
            depth: 0,
        };
        let root = p.parse_or()?;
        if p.pos != p.tokens.len() {
            return Err(ExprError::Syntax(format!(
                "jeton inattendu : {:?}",
                p.tokens[p.pos]
            )));
        }
        Ok(Self { root })
    }

    /// @id strongfather_expr_eval
    /// @role evaluator
    /// @layer core
    /// @do evaluate_expression_on_intent
    //@human / @human Évalue l'expression sur l'intention ; le résultat doit être un booléen.
    pub fn eval(&self, intent: &Intent) -> Result<bool, ExprError> {
        match eval_node(&self.root, intent)? {
            Value::Bool(b) => Ok(b),
            autre => Err(ExprError::Type(format!(
                "l'expression doit produire un booléen, pas un {}",
                autre.type_name()
            ))),
        }
    }
}

// --------------------------------------------------------------------------
// Analyse lexicale
// --------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tok {
    Ident(String),
    Str(String),
    Int(i64),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Not,
    And,
    Or,
    Op(CmpOp),
}

fn tokenize(src: &str) -> Result<Vec<Tok>, ExprError> {
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        if out.len() > MAX_TOKENS {
            return Err(ExprError::Syntax(format!(
                "trop de jetons (> {MAX_TOKENS})"
            )));
        }
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        match c {
            c if c.is_whitespace() => i += 1,
            '(' => {
                out.push(Tok::LParen);
                i += 1;
            }
            ')' => {
                out.push(Tok::RParen);
                i += 1;
            }
            '[' => {
                out.push(Tok::LBracket);
                i += 1;
            }
            ']' => {
                out.push(Tok::RBracket);
                i += 1;
            }
            ',' => {
                out.push(Tok::Comma);
                i += 1;
            }
            '.' => {
                out.push(Tok::Dot);
                i += 1;
            }
            '&' if next == Some('&') => {
                out.push(Tok::And);
                i += 2;
            }
            '|' if next == Some('|') => {
                out.push(Tok::Or);
                i += 2;
            }
            '=' if next == Some('=') => {
                out.push(Tok::Op(CmpOp::Eq));
                i += 2;
            }
            '!' if next == Some('=') => {
                out.push(Tok::Op(CmpOp::Ne));
                i += 2;
            }
            '!' => {
                out.push(Tok::Not);
                i += 1;
            }
            '<' if next == Some('=') => {
                out.push(Tok::Op(CmpOp::Le));
                i += 2;
            }
            '<' => {
                out.push(Tok::Op(CmpOp::Lt));
                i += 1;
            }
            '>' if next == Some('=') => {
                out.push(Tok::Op(CmpOp::Ge));
                i += 2;
            }
            '>' => {
                out.push(Tok::Op(CmpOp::Gt));
                i += 1;
            }
            '\'' | '"' => {
                let quote = c;
                i += 1;
                let mut s = String::new();
                loop {
                    match chars.get(i) {
                        None => return Err(ExprError::Syntax("chaîne non terminée".to_string())),
                        Some('\\') => match chars.get(i + 1) {
                            Some(&e) if e == quote || e == '\\' => {
                                s.push(e);
                                i += 2;
                            }
                            _ => {
                                return Err(ExprError::Syntax(
                                    "échappement invalide dans une chaîne".to_string(),
                                ))
                            }
                        },
                        Some(&ch) if ch == quote => {
                            i += 1;
                            break;
                        }
                        Some(&ch) => {
                            s.push(ch);
                            i += 1;
                        }
                    }
                }
                out.push(Tok::Str(s));
            }
            c if c.is_ascii_digit() || (c == '-' && next.is_some_and(|n| n.is_ascii_digit())) => {
                let start = i;
                i += 1;
                while chars.get(i).is_some_and(char::is_ascii_digit) {
                    i += 1;
                }
                let txt: String = chars[start..i].iter().collect();
                let n = txt
                    .parse::<i64>()
                    .map_err(|_| ExprError::Syntax(format!("entier invalide : {txt}")))?;
                out.push(Tok::Int(n));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = i;
                while chars
                    .get(i)
                    .is_some_and(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || *ch == '-')
                {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if word == "in" {
                    out.push(Tok::Op(CmpOp::In));
                } else {
                    out.push(Tok::Ident(word));
                }
            }
            autre => {
                return Err(ExprError::Syntax(format!(
                    "caractère inattendu : '{autre}'"
                )));
            }
        }
    }
    if out.len() > MAX_TOKENS {
        return Err(ExprError::Syntax(format!(
            "trop de jetons (> {MAX_TOKENS})"
        )));
    }
    Ok(out)
}

// --------------------------------------------------------------------------
// Analyse syntaxique (descente récursive bornée)
// --------------------------------------------------------------------------

struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    depth: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Tok> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, t: &Tok) -> Result<(), ExprError> {
        match self.bump() {
            Some(ref x) if x == t => Ok(()),
            autre => Err(ExprError::Syntax(format!(
                "attendu {t:?}, trouvé {autre:?}"
            ))),
        }
    }

    fn enter(&mut self) -> Result<(), ExprError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(ExprError::Syntax(format!(
                "imbrication trop profonde (> {MAX_DEPTH})"
            )));
        }
        Ok(())
    }

    fn parse_or(&mut self) -> Result<Node, ExprError> {
        self.enter()?;
        let mut left = self.parse_and()?;
        while self.peek() == Some(&Tok::Or) {
            self.bump();
            let right = self.parse_and()?;
            left = Node::Or(Box::new(left), Box::new(right));
        }
        self.depth -= 1;
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Node, ExprError> {
        let mut left = self.parse_unary()?;
        while self.peek() == Some(&Tok::And) {
            self.bump();
            let right = self.parse_unary()?;
            left = Node::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Node, ExprError> {
        if self.peek() == Some(&Tok::Not) {
            self.bump();
            self.enter()?;
            let inner = self.parse_unary()?;
            self.depth -= 1;
            return Ok(Node::Not(Box::new(inner)));
        }
        self.parse_cmp()
    }

    fn parse_cmp(&mut self) -> Result<Node, ExprError> {
        let left = self.parse_primary()?;
        if let Some(Tok::Op(op)) = self.peek() {
            let op = *op;
            self.bump();
            let right = self.parse_primary()?;
            if matches!(self.peek(), Some(Tok::Op(_))) {
                return Err(ExprError::Syntax(
                    "comparaisons enchaînées interdites (utiliser des parenthèses)".to_string(),
                ));
            }
            return Ok(Node::Cmp(op, Box::new(left), Box::new(right)));
        }
        Ok(left)
    }

    fn parse_path(&mut self, first: String) -> Result<Vec<String>, ExprError> {
        let mut parts = vec![first];
        while self.peek() == Some(&Tok::Dot) {
            self.bump();
            match self.bump() {
                Some(Tok::Ident(s)) => parts.push(s),
                autre => {
                    return Err(ExprError::Syntax(format!(
                        "segment de chemin attendu, trouvé {autre:?}"
                    )))
                }
            }
            if parts.len() > 4 {
                return Err(ExprError::Syntax("chemin trop long".to_string()));
            }
        }
        Ok(parts)
    }

    fn parse_literal(&mut self) -> Result<Value, ExprError> {
        match self.bump() {
            Some(Tok::Str(s)) => Ok(Value::Str(s)),
            Some(Tok::Int(n)) => Ok(Value::Int(n)),
            Some(Tok::Ident(w)) if w == "true" => Ok(Value::Bool(true)),
            Some(Tok::Ident(w)) if w == "false" => Ok(Value::Bool(false)),
            autre => Err(ExprError::Syntax(format!(
                "littéral attendu dans la liste, trouvé {autre:?}"
            ))),
        }
    }

    fn parse_primary(&mut self) -> Result<Node, ExprError> {
        match self.bump() {
            Some(Tok::LParen) => {
                let inner = self.parse_or()?;
                self.expect(&Tok::RParen)?;
                Ok(inner)
            }
            Some(Tok::LBracket) => {
                let mut items = Vec::new();
                if self.peek() == Some(&Tok::RBracket) {
                    self.bump();
                    return Ok(Node::List(items));
                }
                loop {
                    items.push(self.parse_literal()?);
                    if items.len() > MAX_LIST_LEN {
                        return Err(ExprError::Syntax(format!(
                            "liste trop longue (> {MAX_LIST_LEN})"
                        )));
                    }
                    match self.bump() {
                        Some(Tok::Comma) => {}
                        Some(Tok::RBracket) => break,
                        autre => {
                            return Err(ExprError::Syntax(format!(
                                "',' ou ']' attendu, trouvé {autre:?}"
                            )))
                        }
                    }
                }
                Ok(Node::List(items))
            }
            Some(Tok::Str(s)) => Ok(Node::Lit(Value::Str(s))),
            Some(Tok::Int(n)) => Ok(Node::Lit(Value::Int(n))),
            Some(Tok::Ident(w)) => match w.as_str() {
                "true" => Ok(Node::Lit(Value::Bool(true))),
                "false" => Ok(Node::Lit(Value::Bool(false))),
                "has" => {
                    self.expect(&Tok::LParen)?;
                    let first = match self.bump() {
                        Some(Tok::Ident(s)) => s,
                        autre => {
                            return Err(ExprError::Syntax(format!(
                                "chemin attendu dans has(), trouvé {autre:?}"
                            )))
                        }
                    };
                    let path = self.parse_path(first)?;
                    self.expect(&Tok::RParen)?;
                    Ok(Node::Has(path))
                }
                _ => Ok(Node::Path(self.parse_path(w)?)),
            },
            autre => Err(ExprError::Syntax(format!(
                "terme attendu, trouvé {autre:?}"
            ))),
        }
    }
}

// --------------------------------------------------------------------------
// Évaluation
// --------------------------------------------------------------------------

fn resolve(path: &[String], intent: &Intent) -> Result<Value, ExprError> {
    let unknown = || ExprError::UnknownAttribute(path.join("."));
    match path {
        [a] => match a.as_str() {
            "action" => Ok(Value::Str(intent.action_type.as_str().to_string())),
            "subject" => Ok(Value::Str(intent.subject.clone())),
            "caller" => Ok(Value::Str(intent.call_context.caller_identity.clone())),
            "origin" => Ok(Value::Str(intent.call_context.origin.clone())),
            "instance" => Ok(Value::Str(intent.call_context.instance.clone())),
            "priority" => intent
                .requested_priority
                .map(|p| Value::Int(i64::from(p)))
                .ok_or_else(unknown),
            "constraints" => Ok(Value::List(
                intent
                    .constraints
                    .iter()
                    .map(|c| Value::Str(c.id.clone()))
                    .collect(),
            )),
            _ => Err(unknown()),
        },
        [ns, key] => {
            let table = match ns.as_str() {
                "data" => &intent.data.content,
                "meta" => &intent.metadata,
                _ => return Err(unknown()),
            };
            table
                .get(key)
                .map(|v| Value::Str(v.clone()))
                .ok_or_else(unknown)
        }
        _ => Err(unknown()),
    }
}

fn as_int(v: &Value) -> Option<i64> {
    match v {
        Value::Int(n) => Some(*n),
        Value::Str(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

fn scalar_eq(a: &Value, b: &Value) -> Result<bool, ExprError> {
    match (a, b) {
        (Value::Str(x), Value::Str(y)) => Ok(x == y),
        (Value::Int(x), Value::Int(y)) => Ok(x == y),
        (Value::Bool(x), Value::Bool(y)) => Ok(x == y),
        (Value::Str(_), Value::Int(_)) | (Value::Int(_), Value::Str(_)) => {
            match (as_int(a), as_int(b)) {
                (Some(x), Some(y)) => Ok(x == y),
                _ => Err(ExprError::Type(
                    "comparaison texte/entier sur un texte non numérique".to_string(),
                )),
            }
        }
        _ => Err(ExprError::Type(format!(
            "comparaison {} / {} interdite",
            a.type_name(),
            b.type_name()
        ))),
    }
}

fn eval_node(node: &Node, intent: &Intent) -> Result<Value, ExprError> {
    match node {
        Node::Lit(v) => Ok(v.clone()),
        Node::List(items) => Ok(Value::List(items.clone())),
        Node::Path(p) => resolve(p, intent),
        Node::Has(p) => match resolve(p, intent) {
            Ok(_) => Ok(Value::Bool(true)),
            Err(ExprError::UnknownAttribute(_)) if is_known_path_shape(p) => Ok(Value::Bool(false)),
            Err(e) => Err(e),
        },
        Node::Not(inner) => Ok(Value::Bool(!expect_bool(&eval_node(inner, intent)?)?)),
        Node::And(l, r) => {
            if !expect_bool(&eval_node(l, intent)?)? {
                return Ok(Value::Bool(false));
            }
            Ok(Value::Bool(expect_bool(&eval_node(r, intent)?)?))
        }
        Node::Or(l, r) => {
            if expect_bool(&eval_node(l, intent)?)? {
                return Ok(Value::Bool(true));
            }
            Ok(Value::Bool(expect_bool(&eval_node(r, intent)?)?))
        }
        Node::Cmp(op, l, r) => {
            let gauche = eval_node(l, intent)?;
            let droite = eval_node(r, intent)?;
            let res = match op {
                CmpOp::Eq => scalar_eq(&gauche, &droite)?,
                CmpOp::Ne => !scalar_eq(&gauche, &droite)?,
                CmpOp::In => match (&gauche, &droite) {
                    (Value::List(_), _) => {
                        return Err(ExprError::Type("'in' : liste à gauche".to_string()))
                    }
                    (_, Value::List(items)) => {
                        let mut found = false;
                        for it in items {
                            if scalar_eq(&gauche, it)? {
                                found = true;
                                break;
                            }
                        }
                        found
                    }
                    _ => {
                        return Err(ExprError::Type(
                            "'in' attend une liste à droite".to_string(),
                        ))
                    }
                },
                CmpOp::Lt | CmpOp::Le | CmpOp::Gt | CmpOp::Ge => {
                    let (Some(x), Some(y)) = (as_int(&gauche), as_int(&droite)) else {
                        return Err(ExprError::Type(format!(
                            "ordre {} / {} : entiers requis",
                            gauche.type_name(),
                            droite.type_name()
                        )));
                    };
                    match op {
                        CmpOp::Lt => x < y,
                        CmpOp::Le => x <= y,
                        CmpOp::Gt => x > y,
                        _ => x >= y,
                    }
                }
            };
            Ok(Value::Bool(res))
        }
    }
}

/// `has()` n'accepte que des chemins de forme connue : une faute de frappe (`dta.x`) reste une erreur.
fn is_known_path_shape(p: &[String]) -> bool {
    match p {
        [a] => a == "priority",
        [ns, _] => ns == "data" || ns == "meta",
        _ => false,
    }
}

fn expect_bool(v: &Value) -> Result<bool, ExprError> {
    match v {
        Value::Bool(b) => Ok(*b),
        autre => Err(ExprError::Type(format!(
            "booléen attendu, trouvé {}",
            autre.type_name()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{ActionType, CallContext, Constraint, IntentData};
    use std::collections::HashMap;

    fn intent() -> Intent {
        let mut data = HashMap::new();
        data.insert("montant".to_string(), "42".to_string());
        data.insert("devise".to_string(), "EUR".to_string());
        let mut meta = HashMap::new();
        meta.insert("zone".to_string(), "H0".to_string());
        Intent::new(
            "i-1".to_string(),
            ActionType::Modification,
            "jaykonta".to_string(),
            CallContext {
                caller_identity: "op-1".to_string(),
                origin: "maia".to_string(),
                instance: "cog-1".to_string(),
            },
            IntentData { content: data },
        )
        .with_metadata(meta)
        .with_priority(5)
        .with_constraints(vec![Constraint {
            id: "c1".to_string(),
            description: String::new(),
        }])
    }

    fn ev(src: &str) -> Result<bool, ExprError> {
        Expr::parse(src)?.eval(&intent())
    }

    /// @id strongfather_expr_test_true_false
    /// @role test
    /// @layer core
    /// @do test_basic_evaluation
    //@human / @human Évaluation de conditions vraies et fausses sur les attributs d'intention.
    #[test]
    fn test_conditions_vraies_et_fausses() {
        assert_eq!(ev("true"), Ok(true));
        assert_eq!(ev("false"), Ok(false));
        assert_eq!(ev("action == 'modification'"), Ok(true));
        assert_eq!(ev("action == 'read'"), Ok(false));
        assert_eq!(ev("subject == \"jaykonta\" && caller == 'op-1'"), Ok(true));
        assert_eq!(ev("subject == 'jaykonta' && caller == 'op-2'"), Ok(false));
        assert_eq!(ev("origin != 'maia' || instance == 'cog-1'"), Ok(true));
        assert_eq!(ev("!(origin == 'maia')"), Ok(false));
        assert_eq!(ev("data.montant <= 100"), Ok(true));
        assert_eq!(ev("data.montant > 100"), Ok(false));
        assert_eq!(ev("data.montant == 42"), Ok(true));
        assert_eq!(ev("priority >= 5"), Ok(true));
        assert_eq!(ev("meta.zone in ['H0', 'H1']"), Ok(true));
        assert_eq!(ev("meta.zone in ['H2']"), Ok(false));
        assert_eq!(ev("'c1' in constraints"), Ok(true));
        assert_eq!(ev("has(data.absent)"), Ok(false));
        assert_eq!(ev("has(data.absent) && data.absent == 'x'"), Ok(false));
        assert_eq!(ev("has(data.devise) && data.devise == 'EUR'"), Ok(true));
    }

    /// @id strongfather_expr_test_no_substring
    /// @role test
    /// @layer core
    /// @do test_exact_equality
    //@human / @human L'égalité est exacte : pas de correspondance par sous-chaîne ni par casse.
    #[test]
    fn test_egalite_exacte() {
        assert_eq!(ev("subject == 'jaykont'"), Ok(false));
        assert_eq!(ev("subject == 'jaykonta-evil'"), Ok(false));
        assert_eq!(ev("subject == 'JayKonta'"), Ok(false));
        assert_eq!(ev("subject in ['jaykonta-evil', 'kont']"), Ok(false));
    }

    /// @id strongfather_expr_test_errors
    /// @role test
    /// @layer core
    /// @do test_errors_fail
    //@human / @human Attribut inconnu, type incompatible ou expression mal formée produisent une erreur.
    #[test]
    fn test_erreurs() {
        assert!(matches!(
            ev("user.role == 'admin'"),
            Err(ExprError::UnknownAttribute(_))
        ));
        assert!(matches!(
            ev("data.absent == 'x'"),
            Err(ExprError::UnknownAttribute(_))
        ));
        assert!(matches!(
            ev("has(dta.x)"),
            Err(ExprError::UnknownAttribute(_))
        ));
        assert!(matches!(ev("data.devise > 3"), Err(ExprError::Type(_))));
        assert!(matches!(ev("data.devise == 3"), Err(ExprError::Type(_))));
        assert!(matches!(ev("subject"), Err(ExprError::Type(_))));
        assert!(matches!(ev("true && 'x'"), Err(ExprError::Type(_))));
        for bad in [
            "",
            "action ==",
            "(true",
            "true)",
            "action = 'x'",
            "'non terminé",
            "a == b == c",
            "action == 'x' ; drop",
            "[1, 2",
        ] {
            assert!(
                matches!(ev(bad), Err(ExprError::Syntax(_))),
                "doit échouer en syntaxe : {bad:?}"
            );
        }
    }

    /// @id strongfather_expr_test_bounds
    /// @role test
    /// @layer core
    /// @do test_resource_bounds
    //@human / @human Les bornes de taille, de jetons et de profondeur sont appliquées.
    #[test]
    fn test_bornes() {
        let long = format!("subject == '{}'", "x".repeat(MAX_EXPR_LEN));
        assert!(matches!(Expr::parse(&long), Err(ExprError::Syntax(_))));
        let profond = format!(
            "{}true{}",
            "(".repeat(MAX_DEPTH + 1),
            ")".repeat(MAX_DEPTH + 1)
        );
        assert!(matches!(Expr::parse(&profond), Err(ExprError::Syntax(_))));
        let nots = format!("{}true", "!".repeat(MAX_DEPTH + 1));
        assert!(matches!(Expr::parse(&nots), Err(ExprError::Syntax(_))));
        let jetons = vec!["true"; MAX_TOKENS].join(" && ");
        assert!(matches!(Expr::parse(&jetons), Err(ExprError::Syntax(_))));
    }
}
