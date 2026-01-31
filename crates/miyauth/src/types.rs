//! Types d'identité MiyuAuth (contexte, rôle, artefacts, attestation, vérification).
//! Pas de données sensibles exposées (BOUND-*).

/// Contexte d'identité résolu (citoyen, visiteur, externe).
/// Ne décide pas de la confiance ; validé par KindMother.
#[derive(Debug, Clone)]
pub struct IdentityContext {
    /// Rôle identité : citizen | visitor | external.
    pub role: IdentityRole,
    /// Identifiant opaque (pas d'identité métier exposée).
    pub opaque_id: String,
}

/// Rôle identité aligné Connexion Inter-COG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityRole {
    /// Citoyen (COG d'origine).
    Citizen,
    /// Visiteur (Utilisateur Visiteur).
    Visitor,
    /// Externe (Utilisateur Externe, Façade Publique).
    External,
}

/// Artefacts fournis pour résolution (Passeport, Visa, session, etc.).
/// Contenu fourni par le flux ; le toolkit ne décide pas.
#[derive(Debug, Clone, Default)]
pub struct IdentityArtefacts {
    /// Données brutes Passeport (optionnel).
    pub passport_raw: Option<Vec<u8>>,
    /// Données brutes Visa (optionnel).
    pub visa_raw: Option<Vec<u8>>,
    /// Référence session (optionnel).
    pub session_ref: Option<String>,
}

/// Résultat de vérification Passeport/Visa (valide, invalide, expiré).
/// Ne décide pas de l'autorisation (ALLOW/DENY = StrongFather).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult {
    /// Intégrité et validité OK.
    Valid,
    /// Signature ou champs invalides.
    Invalid,
    /// Dates dépassées.
    Expired,
}

/// Attestation d'identité produite à partir du contexte validé.
/// Ne crée pas la confiance ; capacité d'attestation gouvernée.
#[derive(Debug, Clone)]
pub struct Attestation {
    /// Identifiant opaque de l'attestation (traçabilité).
    pub attestation_id: String,
}
