//! @id     jaybureau_suite_jay_collab_room
//! @do     definir_broadcast_capacity
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jay-collab room » : salle CRDT, ACL, révocation des flux (R-008, R-020).

//! Salle de collaboration côté serveur.
//!
//! Une Room tient l'état Yrs partagé pour un document et broadcaste les updates
//! à tous les participants connectés en WebSocket.
//!
//! ## Contrôle d'accès (R-008, R-021)
//!
//! - [`CollabRoom::join`] exige un [`JoinToken`] vérifié par un
//!   [`IdentityVerifier`] **et** un rôle dans l'ACL du document ; l'ACL est
//!   contrôlée avant l'enregistrement du nonce (un principal sans droit ne
//!   consomme pas le cache anti-rejeu).
//! - L'état du document et le flux d'événements ne sont remis qu'après un
//!   `join` réussi ([`JoinGrant`]).
//! - [`CollabRoom::apply_and_broadcast`] relit l'ACL à **chaque** modification :
//!   seul un membre connecté dont le rôle permet l'édition peut écrire
//!   (les lecteurs et commentateurs sont refusés).
//! - Toute erreur de contrôle = refus (C-12).
//!
//! ## Révocation (R-020)
//!
//! - Chaque connexion admise reçoit un jeton d'annulation
//!   (`tokio::sync::watch`) conservé dans `members`.
//! - [`CollabRoom::set_acl`] (accès retiré) et [`CollabRoom::leave`]
//!   déclenchent ce jeton : le flux [`MemberEvents`] cesse immédiatement de
//!   produire des événements et retourne [`CollabError::Revoked`] ; la boucle
//!   de relais (serveur) qui le consomme se termine.
//! - Défense en profondeur : l'appartenance de la connexion est revérifiée
//!   avant la remise de **chaque** événement.

use crate::auth::{IdentityVerifier, JoinToken};
use crate::CollabError;
use jaybureau_core::{CollabEvent, DocId, DocumentAcl, Role, UserId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, watch, RwLock};
use yrs::Doc;

/// Capacité du canal broadcast.
const BROADCAST_CAPACITY: usize = 256;

/// Taille maximale d'un update CRDT accepté (C-13).
pub const MAX_UPDATE_BYTES: usize = 1024 * 1024;

/// Nombre maximal de participants simultanés par salle (C-13).
pub const MAX_PARTICIPANTS: usize = 256;

/// Nombre maximal de connexions simultanées d'un même participant (C-13).
pub const MAX_CONNECTIONS_PER_USER: usize = 16;

/// Connexions actives : user_id → (identifiant de connexion → annulation).
type Members = HashMap<UserId, HashMap<u64, watch::Sender<bool>>>;

/// Statistiques d'une salle.
#[derive(Debug, Clone)]
pub struct RoomStats {
    pub doc_id: DocId,
    pub participants: usize,
    pub state_size_bytes: usize,
}

/// Flux d'événements d'une connexion admise, révocable (R-020).
///
/// Cesse définitivement de produire des événements dès que la connexion est
/// annulée ([`CollabRoom::leave`], [`CollabRoom::set_acl`]) ou qu'elle n'est
/// plus membre de la salle.
pub struct MemberEvents {
    rx: broadcast::Receiver<CollabEvent>,
    cancel: watch::Receiver<bool>,
    members: Arc<RwLock<Members>>,
    user_id: UserId,
    connection_id: u64,
    ended: bool,
}

impl MemberEvents {
    /// Connexion encore membre et non annulée ?
    async fn still_member(&self) -> bool {
        if *self.cancel.borrow() {
            return false;
        }
        self.members
            .read()
            .await
            .get(&self.user_id)
            .is_some_and(|conns| conns.contains_key(&self.connection_id))
    }

    /// Prochain événement de la salle.
    ///
    /// - [`CollabError::Revoked`] : connexion annulée ou retirée — définitif,
    ///   la boucle de relais doit se terminer.
    /// - [`CollabError::Lagged`] : événements perdus (récepteur trop lent) —
    ///   le client doit se resynchroniser.
    /// - [`CollabError::Disconnected`] : salle fermée.
    pub async fn recv(&mut self) -> Result<CollabEvent, CollabError> {
        if self.ended || *self.cancel.borrow() {
            self.ended = true;
            return Err(CollabError::Revoked);
        }
        let received = tokio::select! {
            biased;
            // Annulation (ou émetteur supprimé) : prioritaire sur les événements.
            _ = self.cancel.changed() => None,
            r = self.rx.recv() => Some(r),
        };
        match received {
            None => {
                self.ended = true;
                Err(CollabError::Revoked)
            }
            Some(Ok(evt)) => {
                // Défense en profondeur : appartenance revérifiée à chaque remise.
                if self.still_member().await {
                    Ok(evt)
                } else {
                    self.ended = true;
                    Err(CollabError::Revoked)
                }
            }
            Some(Err(broadcast::error::RecvError::Lagged(n))) => Err(CollabError::Lagged(n)),
            Some(Err(broadcast::error::RecvError::Closed)) => {
                self.ended = true;
                Err(CollabError::Disconnected)
            }
        }
    }

    /// Le flux est-il terminé (révoqué ou fermé) ?
    pub fn is_ended(&self) -> bool {
        self.ended
    }
}

/// Résultat d'un `join` réussi : identité vérifiée, rôle, état et flux.
pub struct JoinGrant {
    /// Identité issue du jeton vérifié (seule identité à utiliser ensuite, C-20).
    pub user_id: UserId,
    pub role: Role,
    /// Identifiant de cette connexion dans la salle.
    pub connection_id: u64,
    /// État complet du document (update Yrs v1).
    pub snapshot: Vec<u8>,
    /// Flux révocable des événements de la salle.
    pub events: MemberEvents,
}

/// Salle de collaboration.
pub struct CollabRoom {
    pub doc_id: DocId,
    doc: Arc<Doc>,
    events_tx: broadcast::Sender<CollabEvent>,
    acl: RwLock<DocumentAcl>,
    /// Connexions authentifiées. Ordre de verrouillage : `members` puis `acl`.
    members: Arc<RwLock<Members>>,
    next_connection: AtomicU64,
    /// Présences actives (user_id -> Presence).
    presences: RwLock<HashMap<UserId, jaybureau_core::Presence>>,
}

/// Annule toutes les connexions d'un membre retiré.
fn cancel_all(conns: HashMap<u64, watch::Sender<bool>>) {
    for (_, tx) in conns {
        // Échec = plus aucun récepteur : rien à annuler.
        let _ = tx.send(true);
    }
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
            members: Arc::new(RwLock::new(HashMap::new())),
            next_connection: AtomicU64::new(1),
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
    /// Vérifie le jeton (audience, identité, signature), puis l'ACL (au moins
    /// lecture) avant l'enregistrement du nonce. Retourne l'état complet et un
    /// abonnement révocable aux événements.
    pub async fn join(
        &self,
        token: Option<&JoinToken>,
        verifier: &dyn IdentityVerifier,
    ) -> Result<JoinGrant, CollabError> {
        let token = token.ok_or(CollabError::AuthMissing)?;
        let user_id = {
            let acl = self.acl.read().await;
            verifier.verify(token, &self.doc_id, &|u| acl.role_for(u).is_some())?
        };
        // Même ordre de verrouillage que `set_acl` (members puis acl) : l'ACL
        // relue ici est celle en vigueur au moment de l'admission.
        let mut members = self.members.write().await;
        let role = self
            .acl
            .read()
            .await
            .role_for(&user_id)
            .ok_or(CollabError::AccessDenied)?;
        if !members.contains_key(&user_id) && members.len() >= MAX_PARTICIPANTS {
            return Err(CollabError::RoomFull);
        }
        let conns = members.entry(user_id.clone()).or_default();
        if conns.len() >= MAX_CONNECTIONS_PER_USER {
            return Err(CollabError::RoomFull);
        }
        let connection_id = self.next_connection.fetch_add(1, Ordering::Relaxed);
        let (cancel_tx, cancel_rx) = watch::channel(false);
        conns.insert(connection_id, cancel_tx);
        // Abonnement avant le snapshot : aucun update perdu entre les deux.
        let rx = self.events_tx.subscribe();
        drop(members);
        Ok(JoinGrant {
            user_id: user_id.clone(),
            role,
            connection_id,
            snapshot: self.snapshot(),
            events: MemberEvents {
                rx,
                cancel: cancel_rx,
                members: self.members.clone(),
                user_id,
                connection_id,
                ended: false,
            },
        })
    }

    /// Rôle courant d'un membre connecté (ACL relue à chaque appel).
    async fn member_role(&self, user_id: &str) -> Result<Role, CollabError> {
        if !self.members.read().await.contains_key(user_id) {
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

    /// Un utilisateur quitte la salle : toutes ses connexions sont annulées
    /// (leurs flux d'événements se terminent, R-020).
    pub async fn leave(&self, user_id: UserId) {
        let removed = self.members.write().await.remove(&user_id);
        self.presences.write().await.remove(&user_id);
        if let Some(conns) = removed {
            // Annulation avant la diffusion du départ.
            cancel_all(conns);
            let _ = self.events_tx.send(CollabEvent::Leave { user_id });
        }
    }

    /// Remplace l'ACL (partage modifié). Les membres qui ont perdu tout accès
    /// sont retirés de la salle et leurs flux d'événements annulés
    /// immédiatement (R-020) ; les droits d'édition sont réévalués au
    /// prochain update.
    pub async fn set_acl(&self, acl: DocumentAcl) {
        let removed: Vec<(UserId, HashMap<u64, watch::Sender<bool>>)> = {
            // members puis acl, tenus ensemble : aucun `join` ne peut
            // s'intercaler entre le calcul des retraits et le remplacement.
            let mut members = self.members.write().await;
            let mut current = self.acl.write().await;
            let gone: Vec<UserId> = members
                .keys()
                .filter(|u| acl.role_for(u).is_none())
                .cloned()
                .collect();
            *current = acl;
            gone.into_iter()
                .filter_map(|u| members.remove(&u).map(|c| (u, c)))
                .collect()
        };
        for (u, conns) in removed {
            cancel_all(conns);
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
    use crate::auth::tests::{new_dir, new_key, AUD};
    use crate::auth::{unix_now, KeyDirectory};
    use std::time::Duration;
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
        let dir = new_dir();
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
        JoinToken::sign(k, AUD, user, "doc-1", unix_now().unwrap()).unwrap()
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

    /// Attend la fin du flux : au plus quelques événements, puis `Revoked`.
    async fn expect_revoked(events: &mut MemberEvents) {
        let r = tokio::time::timeout(Duration::from_secs(1), events.recv())
            .await
            .expect("le flux doit se terminer sans attendre");
        assert!(matches!(r, Err(CollabError::Revoked)), "obtenu: {r:?}");
        assert!(events.is_ended());
        // Définitif : plus aucun événement ensuite.
        assert!(matches!(events.recv().await, Err(CollabError::Revoked)));
    }

    #[tokio::test]
    async fn revoked_member_stops_receiving_events() {
        let f = fixture();
        let mut bob = f.room.join(Some(&token(&f.bob, "bob")), &f.dir).await.unwrap();
        let editor = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        // Bob perd tout accès au document.
        let mut acl = DocumentAcl::owner_only("alice".into());
        acl.grant("carol".into(), Role::Editor);
        f.room.set_acl(acl).await;
        // Les modifications suivantes ne lui parviennent plus.
        f.room
            .apply_and_broadcast(&editor.user_id, sample_update())
            .await
            .unwrap();
        expect_revoked(&mut bob.events).await;
        // L'éditrice, elle, continue de recevoir (départ de bob puis update).
        let mut carol_events = editor.events;
        assert!(matches!(carol_events.recv().await, Ok(CollabEvent::Leave { .. })));
        assert!(matches!(carol_events.recv().await, Ok(CollabEvent::Update { .. })));
    }

    #[tokio::test]
    async fn leave_ends_event_stream() {
        let f = fixture();
        let mut carol = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        // Deux connexions de carol : toutes deux annulées.
        let mut carol2 = f.room.join(Some(&token(&f.carol, "carol")), &f.dir).await.unwrap();
        let owner = f.room.join(Some(&token(&f.alice, "alice")), &f.dir).await.unwrap();
        f.room.leave("carol".into()).await;
        f.room
            .apply_and_broadcast(&owner.user_id, sample_update())
            .await
            .unwrap();
        expect_revoked(&mut carol.events).await;
        expect_revoked(&mut carol2.events).await;
    }

    #[tokio::test]
    async fn membership_rechecked_before_each_event() {
        let f = fixture();
        let mut bob = f.room.join(Some(&token(&f.bob, "bob")), &f.dir).await.unwrap();
        let owner = f.room.join(Some(&token(&f.alice, "alice")), &f.dir).await.unwrap();
        // Retrait sans déclencher l'annulation (simule un oubli) : la
        // revérification d'appartenance suffit à couper le flux.
        let detached = f.room.members.write().await.remove("bob");
        f.room
            .apply_and_broadcast(&owner.user_id, sample_update())
            .await
            .unwrap();
        expect_revoked(&mut bob.events).await;
        drop(detached);
    }

    #[tokio::test]
    async fn member_without_acl_does_not_consume_nonce() {
        let f = fixture();
        for _ in 0..=crate::auth::MAX_NONCES_PER_USER {
            let t = token(&f.mallory, "mallory");
            assert!(matches!(
                f.room.join(Some(&t), &f.dir).await,
                Err(CollabError::AccessDenied)
            ));
        }
        assert_eq!(f.dir.replay_cache_len(), 0);
    }

    #[tokio::test]
    async fn token_for_other_server_refused_by_room() {
        let f = fixture();
        let t = JoinToken::sign(&f.alice, "server-b", "alice", "doc-1", unix_now().unwrap()).unwrap();
        assert!(matches!(
            f.room.join(Some(&t), &f.dir).await,
            Err(CollabError::WrongAudience)
        ));
        assert_eq!(f.room.stats().participants, 0);
        // Un serveur B configuré accepte ce jeton pour sa propre salle.
        let dir_b = KeyDirectory::new("server-b").unwrap();
        dir_b.register("alice".into(), f.alice.verifying_key()).unwrap();
        let room_b = CollabRoom::new("doc-1".into(), DocumentAcl::owner_only("alice".into()));
        assert!(room_b.join(Some(&t), &dir_b).await.is_ok());
    }

    #[tokio::test]
    async fn connections_per_user_bounded() {
        let f = fixture();
        let mut grants = Vec::new();
        for _ in 0..MAX_CONNECTIONS_PER_USER {
            grants.push(f.room.join(Some(&token(&f.alice, "alice")), &f.dir).await.unwrap());
        }
        assert!(matches!(
            f.room.join(Some(&token(&f.alice, "alice")), &f.dir).await,
            Err(CollabError::RoomFull)
        ));
    }
}
