//! Validation du mot de passe Miyukini COG.
//!
//! Règles : 8 caractères minimum, au moins une lettre, un chiffre,
//! un caractère spécial, une majuscule et une minuscule.

/// Règles de complexité du mot de passe (Miyukini COG).
pub const PASSWORD_MIN_LEN: usize = 8;

/// Vérifie qu'un mot de passe respecte les règles de complexité :
/// - 8 caractères minimum
/// - Au moins une lettre (a-z ou A-Z)
/// - Au moins un chiffre (0-9)
/// - Au moins un caractère spécial (hors lettres et chiffres)
/// - Au moins une majuscule (A-Z)
/// - Au moins une minuscule (a-z)
pub fn validate_password(password: &str) -> Result<(), PasswordError> {
    if password.len() < PASSWORD_MIN_LEN {
        return Err(PasswordError::TooShort);
    }
    let mut has_letter = false;
    let mut has_digit = false;
    let mut has_special = false;
    let mut has_upper = false;
    let mut has_lower = false;
    for c in password.chars() {
        if c.is_ascii_alphabetic() {
            has_letter = true;
            if c.is_ascii_uppercase() {
                has_upper = true;
            } else {
                has_lower = true;
            }
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else {
            has_special = true;
        }
    }
    if !has_letter {
        return Err(PasswordError::NoLetter);
    }
    if !has_digit {
        return Err(PasswordError::NoDigit);
    }
    if !has_special {
        return Err(PasswordError::NoSpecial);
    }
    if !has_upper {
        return Err(PasswordError::NoUppercase);
    }
    if !has_lower {
        return Err(PasswordError::NoLowercase);
    }
    Ok(())
}

/// Erreur de validation du mot de passe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    TooShort,
    NoLetter,
    NoDigit,
    NoSpecial,
    NoUppercase,
    NoLowercase,
}

impl std::fmt::Display for PasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordError::TooShort => write!(f, "Au moins {} caractères requis.", PASSWORD_MIN_LEN),
            PasswordError::NoLetter => write!(f, "Au moins une lettre requise."),
            PasswordError::NoDigit => write!(f, "Au moins un chiffre requis."),
            PasswordError::NoSpecial => write!(f, "Au moins un caractère spécial requis."),
            PasswordError::NoUppercase => write!(f, "Au moins une majuscule requise."),
            PasswordError::NoLowercase => write!(f, "Au moins une minuscule requise."),
        }
    }
}

impl std::error::Error for PasswordError {}

/// Message d'aide pour l'utilisateur (règles affichées dans la fenêtre).
pub fn password_rules_hint() -> &'static str {
    "8 caractères min., 1 lettre, 1 chiffre, 1 caractère spécial, 1 majuscule, 1 minuscule."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_short() {
        assert!(matches!(validate_password("Ab1!xyz"), Err(PasswordError::TooShort)));
    }

    #[test]
    fn no_digit() {
        assert!(matches!(validate_password("Abcdefg!"), Err(PasswordError::NoDigit)));
    }

    #[test]
    fn no_special() {
        assert!(matches!(validate_password("Abcdefg1"), Err(PasswordError::NoSpecial)));
    }

    #[test]
    fn no_upper() {
        assert!(matches!(validate_password("abcdefg1!"), Err(PasswordError::NoUppercase)));
    }

    #[test]
    fn no_lower() {
        assert!(matches!(validate_password("ABCDEFG1!"), Err(PasswordError::NoLowercase)));
    }

    #[test]
    fn valid() {
        assert!(validate_password("Abcdefg1!").is_ok());
        assert!(validate_password("Miyukini1@").is_ok());
    }
}
