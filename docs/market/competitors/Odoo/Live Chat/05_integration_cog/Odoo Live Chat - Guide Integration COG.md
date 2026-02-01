# Odoo Live Chat — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Live Chat dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG (LiveChatService)
- Patterns WriteIntent et Mandates (canal, session, message, rating)
- Exemples de code pseudo-Rust
- Gestion des gouvernances et façade widget

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
LiveChatWidget (Façade) → BondingBrother → LiveChatChannelOperator → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
LiveChatUI → BondingBrother → LiveChatChannelOperator / LiveChatSessionOperator / ...
```

### 1.2 Flux typique — Ouverture session (visiteur)

1. **Intention visiteur** → LiveChatWidget (get_livechat_info puis demande session).
2. **Traduction intention** → BondingBrother (pas d’entrée COG ; requête gouvernée côté backend).
3. **Backend** : LiveChatChannelOperator.match_rule(channel_id, url, country_id) → règle.
4. **Backend** : LiveChatChannelOperator._get_operator(previous_operator_id, lang, country_id) ou chatbot.
5. **Décision** → StrongFather (création session autorisée selon règles et disponibilité).
6. **Persistance** → KindMother (WriteIntent discuss.channel / session).
7. **Notification** → MiyuNotify, bus (façade).

### 1.3 Flux typique — Opérateur envoie un message

1. **Intention opérateur** → LiveChatUI (Discuss) → BondingBrother.
2. **Mandat** → Mandat de Permission (opérateur membre du canal, session active).
3. **Décision** → StrongFather (envoi message autorisé).
4. **Persistance** → KindMother (WriteIntent mail.message).
5. **Notification** → Bus (façade), MiyuNotify si besoin.

---

## 2. Patterns d'Intégration

### 2.1 Création canal

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateLiveChatChannelIntent {
    pub name: String,
    pub button_text: Option<String>,
    pub default_message: Option<String>,
    pub header_background_color: Option<String>,
    pub button_background_color: Option<String>,
    pub user_ids: Vec<UserId>,
}

impl LiveChatChannelOperator {
    pub async fn create_channel(
        &self,
        intent: CreateLiveChatChannelIntent,
        mandate: Mandate,
    ) -> Result<LiveChatChannel, LiveChatError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["channel.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_channel",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(LiveChatError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "channel.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(LiveChatError::PermissionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "im_livechat.channel",
            operation: WriteOperation::Create,
            data: LiveChatChannelData {
                name: intent.name,
                button_text: intent.button_text.unwrap_or_default(),
                default_message: intent.default_message.unwrap_or_default(),
                header_background_color: intent.header_background_color.unwrap_or_else(|| "#875A7B".into()),
                button_background_color: intent.button_background_color.unwrap_or_else(|| "#875A7B".into()),
                user_ids: intent.user_ids,
                company_id: self.get_company_id().await?,
            },
            security_level: SecurityLevel::Sensitive,
        };

        let channel = self.kind_mother.execute_write_intent(write_intent).await?;
        Ok(channel)
    }
}
```

### 2.2 Création session (visiteur)

**Pattern :** Façade → traduction intention → WriteIntent (côté backend uniquement ; visiteur ne pénètre pas le COG)

```rust
// Côté backend : traduction de la requête widget en création de session gouvernée
impl LiveChatSessionOperator {
    pub async fn create_session_from_visitor_request(
        &self,
        channel_id: ChannelId,
        anonymous_name: String,
        url: Option<String>,
        country_id: Option<CountryId>,
        lang: Option<String>,
        previous_operator_id: Option<PartnerId>,
        chatbot_script_id: Option<ChatbotScriptId>,
        mandate: Mandate, // Mandat Public d'Accès ou Mandat interne selon appelant
    ) -> Result<LiveChatSession, LiveChatError> {
        let channel_op = self.get_live_chat_channel_operator().await?;
        let rule = channel_op.match_rule(channel_id, url.as_deref(), country_id).await?;
        let operator_or_chatbot = channel_op.resolve_operator_or_chatbot(
            channel_id,
            previous_operator_id,
            lang.as_deref(),
            country_id,
            chatbot_script_id,
            &rule,
        ).await?;

        if operator_or_chatbot.is_none() {
            return Err(LiveChatError::NoOperatorAvailable);
        }

        mandate.validate_flows(&["session.create"])?;
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_session",
                context: &CreateSessionContext { channel_id, anonymous_name: &anonymous_name },
            })
            .await?;
        if !decision.allowed {
            return Err(LiveChatError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "discuss.channel",
            operation: WriteOperation::Create,
            data: SessionData {
                channel_type: ChannelType::Livechat,
                livechat_channel_id: channel_id,
                livechat_operator_id: operator_or_chatbot.operator_partner_id,
                chatbot_current_step_id: operator_or_chatbot.chatbot_step_id,
                anonymous_name: Some(anonymous_name),
                country_id,
                channel_member_ids: operator_or_chatbot.members_to_add,
                livechat_active: true,
            },
            security_level: SecurityLevel::Sensitive,
        };

        let session = self.kind_mother.execute_write_intent(write_intent).await?;
        Ok(session)
    }
}
```

### 2.3 Envoi message (opérateur ou chatbot)

**Pattern :** Mandate + WriteIntent

```rust
impl LiveChatMessageOperator {
    pub async fn post_message(
        &self,
        session_id: SessionId,
        author_id: PartnerId,
        body: String,
        mandate: Mandate,
    ) -> Result<MailMessage, LiveChatError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["message.post"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "post_message",
                context: &PostMessageContext { session_id, author_id: &author_id },
            })
            .await?;
        if !decision.allowed {
            return Err(LiveChatError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "mail.message",
            operation: WriteOperation::Create,
            data: MailMessageData {
                res_id: session_id.into(),
                model: "discuss.channel".into(),
                author_id,
                body,
                message_type: MessageType::Comment,
                subtype_xmlid: "mail.mt_comment",
            },
            security_level: SecurityLevel::Sensitive,
        };

        let message = self.kind_mother.execute_write_intent(write_intent).await?;
        // Notify bus (façade) for real-time
        self.bus_publish_message(session_id, &message).await?;
        Ok(message)
    }
}
```

### 2.4 Clôture session et notation

**Pattern :** WriteIntent (livechat_active = False) + WriteIntent (rating)

```rust
impl LiveChatSessionOperator {
    pub async fn close_session(&self, session_id: SessionId, mandate: Mandate) -> Result<(), LiveChatError> {
        mandate.validate_flows(&["session.close"])?;
        let write_intent = WriteIntent {
            entity_type: "discuss.channel",
            operation: WriteOperation::Update,
            data: UpdateSessionData {
                id: session_id,
                livechat_active: false,
                member_fold_state: FoldState::Closed,
            },
            security_level: SecurityLevel::Sensitive,
        };
        self.kind_mother.execute_write_intent(write_intent).await?;
        self.notify_visitor_left(session_id).await?;
        Ok(())
    }
}

impl LiveChatRatingOperator {
    pub async fn create_rating(
        &self,
        channel_id: ChannelId,
        session_id: SessionId,
        rating_value: RatingValue, // satisfied / ok / dissatisfied
        comment: Option<String>,
        mandate: Mandate,
    ) -> Result<Rating, LiveChatError> {
        mandate.validate_flows(&["rating.create"])?;
        let write_intent = WriteIntent {
            entity_type: "rating.rating",
            operation: WriteOperation::Create,
            data: RatingData {
                parent_id: channel_id.into(),
                res_id: session_id.into(),
                res_model: "discuss.channel".into(),
                rating: rating_value,
                consumed: true,
                feedback: comment,
            },
            security_level: SecurityLevel::Standard,
        };
        self.kind_mother.execute_write_intent(write_intent).await?;
        Ok(())
    }
}
```

---

## 3. Façade Widget (Utilisateur Externe)

**Règle :** Le widget ne reçoit que des données exposées par Mandat Public d'Accès ; aucune décision métier ni WriteIntent n’est exécutée côté client.

- **get_livechat_info** : lecture seule (available, options, server_url) — appel API publique gouvernée par Mandat Public (quotas, rate limits).
- **Demande session** : requête HTTP/WebSocket vers backend ; BondingBrother traduit en intention ; création session côté COG avec Mandat Public (pas de Mandat de Permission utilisateur interne).
- **Messages** : envoi/réception via bus en façade ; persistance côté COG via LiveChatMessageOperator sous Mandat.

---

## 4. Gestion des Erreurs et Rollback

- **NoOperatorAvailable** : get_livechat_info.available = false ou message explicite ; pas de session créée.
- **DecisionDenied** : refus StrongFather → erreur utilisateur (permission ou règle).
- **PermissionDenied** : Master Butler → erreur permission.
- **SecurityLevelExceeded** : WorrySentinel → refus ou dégradation.
- Rollback : KindMother annule la transaction WriteIntent en cas d’échec post-décision.

---

## 5. Intégration avec Cores

| Core | Rôle Live Chat |
|------|-----------------|
| StrongFather | Décision création canal/session/message/rating, clôture |
| KindMother | WriteIntent canal, session, message, rating, chatbot |
| Master Butler | Permissions opérateurs, canaux, sessions |
| WorrySentinel | Niveau sécurité 1–2, données sensibles (messages, email chatbot) |
| Ever Buddy | Cycle de vie canal, session |
| BondingBrother | Traduction intention UI/Widget → Opérateurs |
| Caring Nanny | État système (dégradé = pas de nouvelle session si configuré) |
| Border Guard | Frontière visiteur = Façade Publique Gouvernée |

---

**Document** : Odoo Live Chat — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
