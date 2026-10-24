//! @id     miyucloud_domain_entities_calendar_access
//! @do     decider_droits_calendrier
//! @role   securite
//! @layer  app
//! @human  miyucloud — droits d'un utilisateur sur un calendrier (lecture / écriture / gestion), M-01.

//! Décision d'accès à un calendrier.
//!
//! M-01 : la lecture et l'écriture sont décidées séparément.
//! - lecture : propriétaire, partage (tout niveau) ou calendrier public ;
//! - écriture (événements, métadonnées) : propriétaire ou partage `write` / `owner` ;
//! - gestion (suppression du calendrier) : propriétaire ou partage `owner`.
//!
//! `is_public` n'accorde **jamais** l'écriture. Tout niveau de partage inconnu
//! est traité comme `read` (fail-closed, C-12).

/// Niveau de partage d'un calendrier (colonne `caldav.calendar_shares.access_level`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarShareLevel {
    Read,
    Write,
    Owner,
}

impl CalendarShareLevel {
    /// Interprète la valeur stockée ; toute valeur inconnue retombe sur `Read`.
    pub fn from_db(value: &str) -> Self {
        match value.trim() {
            "write" => Self::Write,
            "owner" => Self::Owner,
            _ => Self::Read,
        }
    }
}

/// Faits d'accès d'un utilisateur sur un calendrier, lus en base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CalendarAccess {
    /// Le calendrier existe.
    pub exists: bool,
    /// L'utilisateur est le propriétaire.
    pub is_owner: bool,
    /// Le calendrier est public.
    pub is_public: bool,
    /// Partage accordé à l'utilisateur, s'il existe.
    pub share: Option<CalendarShareLevel>,
}

impl CalendarAccess {
    /// Aucun droit (calendrier absent ou erreur).
    pub fn none() -> Self {
        Self::default()
    }

    /// Lecture : propriétaire, partage quelconque ou calendrier public.
    pub fn can_read(&self) -> bool {
        self.exists && (self.is_owner || self.share.is_some() || self.is_public)
    }

    /// Écriture : propriétaire ou partage `write` / `owner`. Jamais via `is_public`.
    pub fn can_write(&self) -> bool {
        self.exists
            && (self.is_owner
                || matches!(
                    self.share,
                    Some(CalendarShareLevel::Write | CalendarShareLevel::Owner)
                ))
    }

    /// Gestion (suppression du calendrier) : propriétaire ou partage `owner`.
    pub fn can_manage(&self) -> bool {
        self.exists && (self.is_owner || matches!(self.share, Some(CalendarShareLevel::Owner)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn access(
        is_owner: bool,
        is_public: bool,
        share: Option<CalendarShareLevel>,
    ) -> CalendarAccess {
        CalendarAccess {
            exists: true,
            is_owner,
            is_public,
            share,
        }
    }

    #[test]
    fn owner_has_all_rights() {
        let a = access(true, false, None);
        assert!(a.can_read() && a.can_write() && a.can_manage());
    }

    #[test]
    fn read_only_share_cannot_write_or_delete() {
        let a = access(false, false, Some(CalendarShareLevel::Read));
        assert!(a.can_read());
        assert!(!a.can_write());
        assert!(!a.can_manage());
    }

    #[test]
    fn write_share_can_write_but_not_delete_calendar() {
        let a = access(false, false, Some(CalendarShareLevel::Write));
        assert!(a.can_read());
        assert!(a.can_write());
        assert!(!a.can_manage());
    }

    #[test]
    fn owner_share_can_manage() {
        let a = access(false, false, Some(CalendarShareLevel::Owner));
        assert!(a.can_write() && a.can_manage());
    }

    #[test]
    fn public_calendar_is_read_only_for_non_owner() {
        let a = access(false, true, None);
        assert!(a.can_read());
        assert!(!a.can_write());
        assert!(!a.can_manage());
    }

    #[test]
    fn public_calendar_with_read_share_still_read_only() {
        let a = access(false, true, Some(CalendarShareLevel::Read));
        assert!(!a.can_write());
        assert!(!a.can_manage());
    }

    #[test]
    fn stranger_has_no_rights() {
        let a = access(false, false, None);
        assert!(!a.can_read() && !a.can_write() && !a.can_manage());
    }

    #[test]
    fn missing_calendar_grants_nothing() {
        let a = CalendarAccess::none();
        assert!(!a.can_read() && !a.can_write() && !a.can_manage());
    }

    #[test]
    fn unknown_share_level_is_read_only() {
        assert_eq!(
            CalendarShareLevel::from_db("admin"),
            CalendarShareLevel::Read
        );
        assert_eq!(CalendarShareLevel::from_db(""), CalendarShareLevel::Read);
        assert_eq!(
            CalendarShareLevel::from_db("write"),
            CalendarShareLevel::Write
        );
        assert_eq!(
            CalendarShareLevel::from_db("owner"),
            CalendarShareLevel::Owner
        );
    }
}
