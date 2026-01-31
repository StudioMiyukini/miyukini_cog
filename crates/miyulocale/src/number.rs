//! Tool MiyuLocale — tool.locale.number.format.
//! Formate un nombre selon locale et options fournis (devise, décimales, séparateurs).

use crate::context::GovernedContext;
use crate::errors::MiyuLocaleError;

/// Options de formatage nombre (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct LocaleNumberOptions {
    /// Code devise (ex. EUR, USD) si applicable.
    pub currency: Option<String>,
    /// Nombre de décimales.
    pub decimals: Option<u32>,
}

/// @id: miyulocale_tool_number_format
/// @role: mutator
/// @layer: tool
/// @human: Formate un nombre selon locale et options fournis.
/// @do: number_format_under_governance
/// tool.locale.number.format
pub fn format(
    ctx: &GovernedContext,
    value: f64,
    locale: &str,
    options: &LocaleNumberOptions,
) -> Result<String, MiyuLocaleError> {
    if !ctx.has_mandate() {
        return Err(MiyuLocaleError::NoMandate);
    }
    let decimals = options.decimals.unwrap_or(2);
    let (sep_decimal, sep_thousands) = match locale.trim().to_lowercase().as_str() {
        "fr" | "fr-fr" => (',', Some(' ')),
        "en" | "en-us" | "en-gb" => ('.', Some(',')),
        _ => ('.', None),
    };
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
            if i > 0 && (n - i) % 3 == 0 {
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
    if let Some(ref code) = options.currency {
        let sym = match code.to_uppercase().as_str() {
            "EUR" => " €",
            "USD" => " $",
            "GBP" => " £",
            "CHF" => " CHF",
            "JPY" => " ¥",
            _ => "",
        };
        out.push_str(sym);
    }
    Ok(out)
}
