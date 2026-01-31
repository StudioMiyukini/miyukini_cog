//! Tool expression : tool.calc.expression.evaluate.
//!
//! Évalue une expression mathématique fournie (sans décision métier — BOUND-2).
//! Évaluateur arithmétique sûr : uniquement nombres, + - * / ( ). Pas d'appel externe ni eval.

use crate::context::GovernedContext;
use crate::errors::MiyuCalcError;
use std::iter::Peekable;
use std::str::Chars;

/// @id: miyucalc_tool_expression_evaluate
/// @role: mutator
/// @layer: tool
/// @human: Évalue une expression mathématique fournie.
/// @do: evaluate_expression_under_governance
/// tool.calc.expression.evaluate — ne décide pas du contenu ni de l'autorisation.
pub fn evaluate(
    ctx: &GovernedContext,
    expression: &str,
) -> Result<f64, MiyuCalcError> {
    if !ctx.has_mandate() {
        return Err(MiyuCalcError::NoMandate);
    }
    let expr = expression.trim();
    if expr.is_empty() {
        return Err(MiyuCalcError::InvalidExpression("empty expression".to_string()));
    }
    let mut it = expr.chars().peekable();
    let v = parse_expr(&mut it)?;
    if it.next().is_some() {
        return Err(MiyuCalcError::InvalidExpression("trailing input".to_string()));
    }
    Ok(v)
}

fn parse_expr(it: &mut Peekable<Chars>) -> Result<f64, MiyuCalcError> {
    let mut left = parse_term(it)?;
    loop {
        match it.peek().copied() {
            Some('+') => {
                it.next();
                left += parse_term(it)?;
            }
            Some('-') => {
                it.next();
                left -= parse_term(it)?;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_term(it: &mut Peekable<Chars>) -> Result<f64, MiyuCalcError> {
    let mut left = parse_factor(it)?;
    loop {
        match it.peek().copied() {
            Some('*') => {
                it.next();
                left *= parse_factor(it)?;
            }
            Some('/') => {
                it.next();
                let right = parse_factor(it)?;
                if right == 0.0 {
                    return Err(MiyuCalcError::InvalidExpression("division by zero".to_string()));
                }
                left /= right;
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_factor(it: &mut Peekable<Chars>) -> Result<f64, MiyuCalcError> {
    skip_ws(it);
    match it.peek().copied() {
        Some('(') => {
            it.next();
            let v = parse_expr(it)?;
            skip_ws(it);
            if it.next() != Some(')') {
                return Err(MiyuCalcError::InvalidExpression("missing ')'".to_string()));
            }
            Ok(v)
        }
        Some('+') => {
            it.next();
            parse_factor(it)
        }
        Some('-') => {
            it.next();
            parse_factor(it).map(|x| -x)
        }
        Some(c) if c.is_ascii_digit() || c == '.' => parse_number(it),
        _ => Err(MiyuCalcError::InvalidExpression("expected number or '('".to_string())),
    }
}

fn skip_ws(it: &mut Peekable<Chars>) {
    while it.peek().copied().map_or(false, |c| c == ' ' || c == '\t') {
        it.next();
    }
}

fn parse_number(it: &mut Peekable<Chars>) -> Result<f64, MiyuCalcError> {
    let mut s = String::new();
    while let Some(c) = it.peek().copied() {
        if c.is_ascii_digit() || c == '.' {
            s.push(c);
            it.next();
        } else {
            break;
        }
    }
    s.parse::<f64>().map_err(|_| MiyuCalcError::InvalidExpression(format!("invalid number: {}", s)))
}
