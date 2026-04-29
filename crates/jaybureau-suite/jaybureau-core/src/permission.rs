//! Permissions et partage de documents (ACL).

use crate::UserId;
use serde::{Deserialize, Serialize};

/// Rôle d'un utilisateur sur un document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Propriétaire : tous les droits.
    Owner,
    /// Éditeur : modification + partage.
    Editor,
    /// Commentateur : lecture + commentaires.
    Commenter,
    /// Lecteur : lecture seule.
    Viewer,
}

impl Role {
    pub fn can_edit(&self) -> bool {
        matches!(self, Self::Owner | Self::Editor)
    }
    pub fn can_comment(&self) -> bool {
        !matches!(self, Self::Viewer)
    }
    pub fn can_share(&self) -> bool {
        matches!(self, Self::Owner | Self::Editor)
    }
    pub fn can_delete(&self) -> bool {
        matches!(self, Self::Owner)
    }
}

/// Une entrée ACL pour un utilisateur ou un lien.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclEntry {
    pub user_id: UserId,
    pub role: Role,
    pub granted_at: chrono::DateTime<chrono::Utc>,
}

/// Type d'accès par lien partagé.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    /// Lien désactivé — seules les personnes ajoutées peuvent accéder.
    Restricted,
    /// N'importe qui avec le lien peut voir.
    AnyoneView,
    /// N'importe qui avec le lien peut commenter.
    AnyoneComment,
    /// N'importe qui avec le lien peut éditer.
    AnyoneEdit,
}

impl Access {
    pub fn role(&self) -> Option<Role> {
        match self {
            Self::Restricted => None,
            Self::AnyoneView => Some(Role::Viewer),
            Self::AnyoneComment => Some(Role::Commenter),
            Self::AnyoneEdit => Some(Role::Editor),
        }
    }
}

/// Liste de contrôle d'accès pour un document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAcl {
    pub owner: UserId,
    pub entries: Vec<AclEntry>,
    pub link_access: Access,
}

impl DocumentAcl {
    pub fn owner_only(owner: UserId) -> Self {
        Self {
            owner,
            entries: Vec::new(),
            link_access: Access::Restricted,
        }
    }

    /// Retourne le rôle d'un utilisateur sur ce document.
    pub fn role_for(&self, user_id: &str) -> Option<Role> {
        if user_id == self.owner {
            return Some(Role::Owner);
        }
        if let Some(e) = self.entries.iter().find(|e| e.user_id == user_id) {
            return Some(e.role);
        }
        self.link_access.role()
    }

    /// Ajoute ou remplace une entrée ACL.
    pub fn grant(&mut self, user_id: UserId, role: Role) {
        self.entries.retain(|e| e.user_id != user_id);
        self.entries.push(AclEntry {
            user_id,
            role,
            granted_at: chrono::Utc::now(),
        });
    }

    /// Révoque l'accès d'un utilisateur.
    pub fn revoke(&mut self, user_id: &str) {
        self.entries.retain(|e| e.user_id != user_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_has_all_rights() {
        let acl = DocumentAcl::owner_only("alice".into());
        let role = acl.role_for("alice").unwrap();
        assert!(role.can_edit());
        assert!(role.can_delete());
        assert!(role.can_share());
    }

    #[test]
    fn viewer_cannot_edit() {
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.grant("bob".into(), Role::Viewer);
        let role = acl.role_for("bob").unwrap();
        assert!(!role.can_edit());
        assert!(!role.can_delete());
    }

    #[test]
    fn link_access_applies_to_strangers() {
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.link_access = Access::AnyoneView;
        assert!(matches!(acl.role_for("stranger"), Some(Role::Viewer)));
    }

    #[test]
    fn revoke_removes_entry() {
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.grant("bob".into(), Role::Editor);
        assert!(acl.role_for("bob").is_some());
        acl.revoke("bob");
        assert!(acl.role_for("bob").is_none());
    }
}
