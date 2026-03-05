//! Tool number : tool.calc.number.format.
//!
//! Formate un nombre selon options fournies (devise, pourcentage, décimales, locale) — BOUND-2.

use crate::context::GovernedContext;
use crate::errors::MiyuCalcError;

/// Options de formatage (fournies dans le flux, pas de choix métier).
#[derive(Debug, Clone, Default)]
pub struct FormatOptions {
    /// Nombre de décimales (optionnel).
    pub decimals: Option<u32>,
    /// Code devise (ex. EUR, USD) si format devise.
    pub currency: Option<String>,
    /// true si format pourcentage.
    pub percent: bool,
    /// Locale (ex. fr-FR) pour séparateurs.
    pub locale: Option<String>,
}

/// @id: miyucalc_tool_number_format
/// @role: mutator
/// @layer: tool
/// @human: Formate un nombre selon options fournies.
/// @do: format_number_under_governance
/// tool.calc.number.format
pub fn format(
    ctx: &GovernedContext,
    value: f64,
    options: &FormatOptions,
) -> Result<String, MiyuCalcError> {
    if !ctx.has_mandate() {
        return Err(MiyuCalcError::NoMandate);
    }
    let decimals = options.decimals.unwrap_or(2);
    let (val, suffix, prefix) = if options.percent {
        (value * 100.0, "%", "")
    } else if let Some(ref code) = options.currency {
        let sym = currency_symbol(code);
        (value, "", sym)
    } else {
        (value, "", "")
    };
    let sep_decimal = options
        .locale
        .as_deref()
        .and_then(|l| {
            if l.starts_with("fr") {
                Some(',')
            } else if l.starts_with("en") {
                Some('.')
            } else {
                None
            }
        })
        .unwrap_or('.');
    let sep_thousands = options.locale.as_deref().and_then(|l| {
        if l.starts_with("fr") {
            Some(' ')
        } else if l.starts_with("en") {
            Some(',')
        } else {
            None
        }
    });
    let s = format_decimal(val, decimals, sep_decimal, sep_thousands);
    Ok(format!("{prefix}{s}{suffix}"))
}

fn currency_symbol(code: &str) -> &'static str {
    match code.to_uppercase().as_str() {
        "EUR" => "€",
        "USD" => "$",
        "GBP" => "£",
        "CHF" => "CHF ",
        "JPY" => "¥",
        _ => "",
    }
}

fn format_decimal(
    value: f64,
    decimals: u32,
    sep_decimal: char,
    sep_thousands: Option<char>,
) -> String {
    let int_part = value.abs().trunc() as i64;
    let frac = (value.abs().fract() * 10_f64.powi(decimals as i32)).round() as u64;
    let int_str = int_part.unsigned_abs().to_string();
    let mut out = String::new();
    if value < 0.0 {
        out.push('-');
    }
    if let Some(sep) = sep_thousands {
        let chars: Vec<char> = int_str.chars().collect();
        let n = chars.len();
        for (i, c) in chars.into_iter().enumerate() {
            if i > 0 && (n - i).is_multiple_of(3) {
                out.push(sep);
            }
            out.push(c);
        }
    } else {
        out.push_str(&int_str);
    }
    if decimals > 0 {
        out.push(sep_decimal);
        let frac_str = format!("{:0>width$}", frac, width = decimals as usize);
        out.push_str(&frac_str);
    }
    out
}
