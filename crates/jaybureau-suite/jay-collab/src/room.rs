//! @id     jaybureau_suite_jay_collab_room
//! @do     definir_broadcast_capacity
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jay-collab room ».

//! Salle de collaboration côté serveur.
//!
//! Une Room tient l'état Yrs partagé pour un document et broadcaste les updates
//! à tous les participants connectés en WebSocket.
//!
//! ## Contrôle d'accès (R-008)
//!
//! - [`CollabRoom::join`] exige un [`JoinToken`] vérifié par un
//!   [`IdentityVerifier`] **et** un rôle dans l'ACL du document ; sinon refus.
//! - L'état du document et le flux d'événements ne sont remis qu'après un
//!   `join` réussi ([`JoinGrant`]).
//! - [`CollabRoom::apply_and_broadcast`] relit l'ACL à **chaque** modification :
//!   seul un membre connecté dont le rôle permet l'édition peut écrire
//!   (les lecteurs et commentateurs sont refusés).
//! - Toute erreur de contrôle = refus (C-12).

use crate::auth::{IdentityVerifier, JoinToken};
use crate::CollabError;
use jaybureau_core::{CollabEvent, DocId, DocumentAcl, Role, UserId};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use yrs::Doc;

/// Capacité du canal broadcast.
const BROADCAST_CAPACITY: usize = 256;

/// Taille maximale d'un update CRDT accepté (C-13).
pub const MAX_UPDATE_BYTES: usize = 1024 * 1024;

/// Nombre maximal de participants simultanés par salle (C-13).
pub const MAX_PARTICIPANTS: usize = 256;

/// Statistiques d'une salle.
#[derive(Debug, Clone)]
pub struct RoomStats {
    pub doc_id: DocId,
    pub participants: usize,
    pub state_size_bytes: usize,
}

/// Résultat d'un `join` réussi : identité vérifiée, rôle, état et flux.
pub struct JoinGrant {
    /// Identité issue du jeton vérifié (seule identité à utiliser ensuite, C-20).
    pub user_id: UserId,
    pub role: Role,
    /// État complet du document (update Yrs v1).
    pub snapshot: Vec<u8>,
    /// Flux des événements de la salle.
    pub events: broadcast::Receiver<CollabEvent>,
}

/// Salle de collaboration.
pub struct CollabRoom {
    pub doc_id: DocId,
    doc: Arc<Doc>,
    events_tx: broadcast::Sender<CollabEvent>,
    acl: RwLock<DocumentAcl>,
    /// Participants authentifiés.
    members: RwLock<HashSet<UserId>>,
    /// Présences actives (user_id -> Presence).
    presences: RwLock<HashMap<UserId, jaybureau_core::Presence>>,
}

impl CollabRoom {
    /// Crée une salle gouvernée par l'ACL du document.
    pub fn new(doc_id: DocId, acl: DocumentAcl) -> Self {
        let (tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            doc_id,
            doc: Arc::new(crate::new_doc()),
            events_tx: tx,
            acl: RwLock::new(acl),
            members: RwLock::new(HashSet::new()),
            presences: RwLock::new(HashMap::new()),
        }
    }

    /// État complet du document (usage interne : join, stats).
    fn snapshot(&self) -> Vec<u8> {
        use yrs::{ReadTxn, Transact};
        let txn = self.doc.transact();
        txn.encode_state_as_update_v1(&yrs::StateVector::default())
    }

    /// Un participant rejoint la salle.
    ///
    /// Vérifie le jeton (identité), puis l'ACL (au moins lecture). Retourne
    /// l'état complet et un abonnement aux événements.
    pub async fn join(
        &self,
        token: Option<&JoinToken>,
        verifier: &dyn IdentityVerifier,
    ) -> Result<JoinGrant, CollabError> {
        let token = token.ok_or(CollabError::AuthMissing)?;
        let user_id = verifier.verify(token, &self.doc_id)?;
        let role = self
            .acl
            .read()
            .await
            .role_for(&user_id)
            .ok_or(CollabError::AccessDenied)?;
        {
            let mut members = self.members.write().await;
            if !members.contains(&user_id) && members.len() >= MAX_PARTICIPANTS {
                return Err(CollabError::RoomFull);
            }
            members.insert(user_id.clone());
        }
        // Abonnement avant le snapshot : aucun update perdu entre les deux.
        let events = self.events_tx.subscribe();
        Ok(JoinGrant {
            user_id,
            role,
            snapshot: self.snapshot(),
            events,
        })
    }

    /// Rôle courant d'un membre connecté (ACL relue à chaque appel).
    async fn member_role(&self, user_id: &str) -> Result<Role, CollabError> {
        if !self.members.read().await.contains(user_id) {
            return Err(CollabError::NotJoined);
        }
        self.acl
            .read()
            .await
            .role_for(user_id)
            .ok_or(CollabError::AccessDenied)
    }

    /// Applique un update reçu d'un participant authentifié et le broadcaste.
    ///
    /// `user_id` doit être celui du [`JoinGrant`] de la connexion, jamais un
    /// champ du message reçu.
    pub async fn apply_and_broadcast(
        &self,
        user_id: &str,
        update: Vec<u8>,
    ) -> Result<(), CollabError> {
        if update.len() > MAX_UPDATE_BYTES {
            return Err(CollabError::TooLarge(update.len()));
        }
        let role = self.member_role(user_id).await?;
        if !role.can_edit() {
            return Err(CollabError::ReadOnly);
        }
        crate::apply_update(&self.doc, &update)?;
        let _ = self.events_tx.send(CollabEvent::Update { update });
        Ok(())
    }

    /// Met à jour la présence d'un membre authentifié et la broadcaste.
    pub async fn update_presence(
        &self,
        user_id: UserId,
        presence: jaybureau_core::Presence,
    ) -> Result<(), CollabError> {
        self.member_role(&user_id).await?;
        self.presences
            .write()
            .await
            .insert(user_id.clone(), presence.clone());
        let _ = self.events_tx.send(CollabEvent::Presence { user_id, presence });
        Ok(())
    }

    /// Un utilisateur quitte la salle.
    pub async fn leave(&self, user_id: UserId) {
        let was_member = self.members.write().await.remove(&user_id);
        self.presences.write().await.remove(&user_id);
        if was_member {
            let _ = self.events_tx.send(CollabEvent::Leave { user_id });
        }
    }

    /// Remplace l'ACL (partage modifié). Les membres qui ont perdu tout accès
    /// sont retirés de la salle ; les droits d'édition sont réévalués au
    /// prochain update.
    pub async fn set_acl(&self, acl: DocumentAcl) {
        let removed: Vec<UserId> = {
            let mut members = self.members.write().await;
            let removed: Vec<UserId> = members
                .iter()
                .filter(|u| acl.role_for(u).is_none())
                .cloned()
                .collect();
            for u in &removed {
                members.remove(u);
            }
            removed
        };
        *self.acl.write().await = acl;
        for u in removed {
            self.presences.write().await.remove(&u);
            let _ = self.events_tx.send(CollabEvent::Leave { user_id: u });
        }
    }

    /// Retourne les stats de la salle.
    pub fn stats(&self) -> RoomStats {
        let participants = self.members.try_read().map_or(0, |r| r.len());
        let state_size = self.snapshot().len();
        RoomStats {
            doc_id: self.doc_id.clone(),
            participants,
            state_size_bytes: state_size,
        }
    }
}

/// Gestionnaire global des salles (côté serveur).
pub struct RoomRegistry {
    rooms: RwLock<HashMap<DocId, Arc<CollabRoom>>>,
}

impl RoomRegistry {
    pub fn new() -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
        }
    }

    /// Retourne ou crée la salle pour un doc_id donné.
    ///
    /// `acl` n'est utilisée qu'à la création ; pour une salle existante,
    /// propager les changements de partage via [`CollabRoom::set_acl`].
    pub async fn get_or_create(&self, doc_id: DocId, acl: DocumentAcl) -> Arc<CollabRoom> {
        {
            let rooms = self.rooms.read().await;
            if let Some(r) = rooms.get(&doc_id) {
                return r.clone();
            }
        }
        let mut rooms = self.rooms.write().await;
        rooms
            .entry(doc_id.clone())
            .or_insert_with(|| Arc::new(CollabRoom::new(doc_id, acl)))
            .clone()
    }

    /// Nombre total de salles actives.
    pub async fn count(&self) -> usize {
        self.rooms.read().await.len()
    }

    /// Stats de toutes les salles.
    pub async fn all_stats(&self) -> Vec<RoomStats> {
        self.rooms.read().await.values().map(|r| r.stats()).collect()
    }
}

impl Default for RoomRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::tests::new_key;
    use crate::auth::{unix_now, KeyDirectory};
    use ed25519_dalek::SigningKey;
    use yrs::{Text, Transact};

    struct Fixture {
        room: CollabRoom,
        dir: KeyDirectory,
        alice: SigningKey,
        bob: SigningKey,
        carol: SigningKey,
        mallory: SigningKey,
    }

    /// alice : propriétaire ; bob : lecteur ; carol : éditrice ; mallory : hors ACL.
    fn fixture() -> Fixture {
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.grant("bob".into(), Role::Viewer);
        acl.grant("carol".into(), Role::Editor);
        let room = CollabRoom::new("doc-1".into(), acl);
        let dir = KeyDirectory::new();
        let (alice, bob, carol, mallory) = (new_key(), new_key(), new_key(), new_key());
        dir.register("alice".into(), alice.verifying_key()).unwrap();
        dir.register("bob".into(), bob.verifying_key()).unwrap();
        dir.register("carol".into(), carol.verifying_key()).unwrap();
        dir.register("mallory".into(), mallory.verifying_key()).unwrap();
        Fixture {
            room,
            dir,
            alice,
            bob,
            carol,
            mallory,
        }
    }

    fn token(k: &SigningKey, user: &str) -> JoinToken {
        JoinToken::sign(k, user, "doc-1", unix_now().unwrap()).unwrap()
    }

    fn sample_update() -> Vec<u8> {
        let doc = crate::new_doc();
        let text = doc.get_or_insert_text("content");
        {
            let mut txn = doc.transact_mut();
            text.insert(&mut txn, 0, "Bonjour");
        }
        let sv = crate::encode_state(&crate::new_doc());
        crate::encode_update_since(&doc, &sv).unwrap()
    }

    #[tokio::test]
    async fn join_without_token_refused() {
        let f = fixture();
        assert!(matches!(f.room.join(None, &f.dir).await, Err(CollabError::AuthMissing)));
    }

    #[tokio::test]
    async fn join_with_forged_token_refused() {
        let f = fixture();
        // Mallory signe au nom d'alice.
        let t = token(&f.mallory, "alice");
        assert!(f.room.join(Some(&t), &f.dir).await.is_err());
        assert_eq!(f.room.stats().participants, 0);
    }

    #[tokio::test]
    async fn join_outside_acl_refused() {
        let f = fixture();
        let t = token(&f.mallory, "mallory");
        assert!(matches!(
            f.room.join(Some(&t), &f.dir).await,
            Err(CollabError::AccessDenied)
        ));
    }

    #[tokio::test]
    async fn edit_without_join_refused() {
        let f = fixture();
        assert!(matches!(
            f.room.apply_and_broadcast("alice", sample_update()).await,
            Err(CollabError::NotJoined)
        ));
    }

    #[tokio::test]
    async fn viewer_cannot_edit() {
        let f = fixture();
        let grant = f.room.join(Some(&token(&f.bob, "bob")), &f.dir).await.unwrap();
        assert_eq!(grant.role, Role::Viewer);
        assert!(matches!(
            f.room.apply_and_broadcast(&grant.user_id, sample_update()).await,
            Err(CollabError::ReadOnly)
        ));
    }

    #[tokio::test]
    async fn editor_can_edit_and_others_receive() {
        let f = fixture();
        let viewer = f.room.join(Some(&token(&f.bob, "bob")), &f.dir).await.unwrap();
        let mut events = viewer.events;
        let editor = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        f.room
            .apply_and_broadcast(&editor.user_id, sample_update())
            .await
            .unwrap();
        assert!(matches!(events.recv().await, Ok(CollabEvent::Update { .. })));
        let owner = f.room.join(Some(&token(&f.alice, "alice")), &f.dir).await.unwrap();
        // Un document vide s'encode en 2 octets : le snapshot contient le texte.
        assert!(owner.snapshot.len() > 2);
    }

    #[tokio::test]
    async fn revoked_editor_cannot_edit() {
        let f = fixture();
        let editor = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        // Carol rétrogradée en lectrice : l'ACL est relue à chaque update.
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.grant("carol".into(), Role::Viewer);
        f.room.set_acl(acl).await;
        assert!(matches!(
            f.room.apply_and_broadcast(&editor.user_id, sample_update()).await,
            Err(CollabError::ReadOnly)
        ));
        // Accès retiré : membre expulsé.
        f.room.set_acl(DocumentAcl::owner_only("alice".into())).await;
        assert!(matches!(
            f.room.apply_and_broadcast(&editor.user_id, sample_update()).await,
            Err(CollabError::NotJoined)
        ));
    }

    #[tokio::test]
    async fn presence_requires_join() {
        let f = fixture();
        assert!(f
            .room
            .update_presence("alice".into(), jaybureau_core::Presence::default())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn oversized_update_refused() {
        let f = fixture();
        let editor = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        assert!(matches!(
            f.room
                .apply_and_broadcast(&editor.user_id, vec![0u8; MAX_UPDATE_BYTES + 1])
                .await,
            Err(CollabError::TooLarge(_))
        ));
    }
}
