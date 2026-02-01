# Odoo Live Chat — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Live Chat dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates Rust proposés)
- Schémas de données (canal, règle, session, message, rating)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

---

## 1. Architecture Technique

### 1.1 Structure des Crates (proposition)

```
crates/
├── miyukini-livechat-channel/       # LiveChatChannelOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── channel.rs                # im_livechat.channel
│   │   ├── rule.rs                   # im_livechat.channel.rule
│   │   ├── operator_availability.rs # _get_operator, _get_less_active_operator
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-livechat-session/        # LiveChatSessionOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── session.rs                # discuss.channel (livechat)
│   │   ├── close.rs                  # _close_livechat_session, _gc_empty_livechat_sessions
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-livechat-message/       # LiveChatMessageOperator (ou intégré dans discuss)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── message.rs                # mail.message
│   │   ├── transcript.rs             # _get_channel_history, _email_livechat_transcript
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-livechat-chatbot/       # LiveChatChatbotOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── script.rs                 # chatbot.script
│   │   ├── step.rs                   # chatbot.script.step
│   │   ├── message.rs                # chatbot.message
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-livechat-rating/        # LiveChatRatingOperator (ou miyurating existant)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── rating.rs                 # rating.rating, parent = channel
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-livechat-ui/            # LiveChatUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── channel_kanban.rs
    │   │   ├── channel_form.rs
    │   │   └── discuss_livechat.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

**Façade widget :** Intégrée dans miyuweb (ou module dédié front) ; pas de crate Rust côté client (JS/TS ou WASM selon stack).

### 1.2 Dépendances principales

**Cores Miyukini :**
- miyukini-kernel
- miyukini-central (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- miyunotify (notifications, digest, email)
- miyucontacts (pays, GeoIP si disponible)
- miyumedia (pièces jointes si besoin)
- Module Discuss-like (discuss.channel, mail.message) si déjà présent

---

## 2. Schémas de Données

### 2.1 LiveChatChannel

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatChannel {
    pub id: ChannelId,
    pub name: String,
    pub button_text: String,
    pub default_message: String,
    pub input_placeholder: Option<String>,
    pub header_background_color: String,
    pub title_color: String,
    pub button_background_color: String,
    pub button_text_color: String,
    pub image_128: Option<ImageId>,
    pub user_ids: Vec<UserId>,
    pub rule_ids: Vec<RuleId>,
    pub nbr_channel: u32,
    pub available_operator_ids: Vec<UserId>, // computed
    pub web_page: Option<String>,          // computed
    pub company_id: CompanyId,
}
```

### 2.2 LiveChatChannelRule

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatChannelRule {
    pub id: RuleId,
    pub channel_id: ChannelId,
    pub regex_url: Option<String>,
    pub action: RuleAction, // display_button | display_button_and_text | auto_popup | hide_button
    pub auto_popup_timer: u32,
    pub chatbot_script_id: Option<ChatbotScriptId>,
    pub chatbot_only_if_no_operator: bool,
    pub country_ids: Vec<CountryId>,
    pub sequence: u32,
}
```

### 2.3 LiveChatSession (discuss.channel livechat)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveChatSession {
    pub id: SessionId,
    pub channel_type: ChannelType, // livechat
    pub livechat_channel_id: ChannelId,
    pub livechat_operator_id: PartnerId,
    pub livechat_active: bool,
    pub anonymous_name: Option<String>,
    pub country_id: Option<CountryId>,
    pub chatbot_current_step_id: Option<StepId>,
    pub channel_member_ids: Vec<ChannelMemberId>,
    pub message_ids: Vec<MessageId>,
    pub duration: Option<f64>, // computed
    pub company_id: CompanyId,
}
```

### 2.4 MailMessage (résumé)

```rust
pub struct MailMessage {
    pub id: MessageId,
    pub res_id: ResourceId,
    pub model: String, // "discuss.channel"
    pub author_id: PartnerId,
    pub body: String,
    pub message_type: MessageType,
    pub subtype_xmlid: String,
    pub date: DateTime,
}
```

### 2.5 Rating

```rust
pub struct Rating {
    pub id: RatingId,
    pub parent_id: ChannelId,  // im_livechat.channel
    pub res_id: SessionId,
    pub res_model: String,     // "discuss.channel"
    pub rating: RatingValue,   // satisfied | ok | dissatisfied
    pub feedback: Option<String>,
    pub consumed: bool,
}
```

---

## 3. API et Contrats

### 3.1 LiveChatChannelOperator

- `create_channel(intent, mandate) -> Result<LiveChatChannel>`
- `update_channel(channel_id, intent, mandate) -> Result<()>`
- `get_livechat_info(channel_id, username?) -> Result<LiveChatInfo>` (available, options, server_url)
- `match_rule(channel_id, url, country_id?) -> Result<Option<LiveChatChannelRule>>`
- `add_operator(channel_id, user_id, mandate) -> Result<()>`
- `remove_operator(channel_id, user_id, mandate) -> Result<()>`

### 3.2 LiveChatSessionOperator

- `create_session_from_visitor_request(channel_id, anonymous_name, url, country_id, lang, previous_operator_id, chatbot_script_id, mandate) -> Result<LiveChatSession>`
- `close_session(session_id, mandate) -> Result<()>`
- `list_sessions(channel_id?, operator_id?, mandate) -> Result<Vec<LiveChatSession>>`

### 3.3 LiveChatMessageOperator

- `post_message(session_id, author_id, body, mandate) -> Result<MailMessage>`
- `get_history(session_id, mandate) -> Result<Vec<MailMessage>>`
- `get_transcript(session_id, mandate) -> Result<String>`
- `send_transcript_email(session_id, email, mandate) -> Result<()>`

### 3.4 LiveChatRatingOperator

- `create_rating(channel_id, session_id, rating_value, comment?, mandate) -> Result<Rating>`
- `list_ratings_by_channel(channel_id, mandate) -> Result<Vec<Rating>>`
- `satisfaction_stats(channel_id, days: u32, mandate) -> Result<SatisfactionStats>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–4 semaines)

- **Objectif :** Un canal, opérateurs, widget minimal, sessions, messages, clôture.
- **Livrables :**
  - miyukini-livechat-channel (canal + règles basiques : action bouton uniquement)
  - miyukini-livechat-session (création, clôture, attribution opérateur)
  - Intégration messages (module Discuss existant ou miyukini-livechat-message minimal)
  - Façade widget : bouton + fenêtre chat + envoi/réception messages (bus ou polling)
  - LiveChatUI : Kanban canaux, formulaire canal (Opérateurs, Options, Widget)
- **Critères d'acceptation :**
  - Visiteur ouvre widget, envoie message ; opérateur reçoit et répond ; visiteur quitte → session clôturée.
  - Au moins un opérateur disponible ; pas de chatbot ni règles URL/pays.

### Phase 2 — Essentiel (2–3 semaines)

- **Objectif :** Règles URL/pays, notation, transcript email, rapports basiques.
- **Livrables :**
  - Règles complètes (regex_url, country_ids, action, auto_popup_timer)
  - miyukini-livechat-rating (création rating, satisfaction 14 jours)
  - Transcript (texte) + envoi email
  - Rapports canal/opérateur (vues ou exports)
- **Critères d'acceptation :**
  - Règle par URL : bouton affiché/masqué selon page.
  - Clôture → proposition notation → enregistrement ; action "View Rating" depuis canal.
  - Opérateur peut envoyer transcript par email.

### Phase 3 — Complet (2–4 semaines)

- **Objectif :** Chatbot, digest, intégration Helpdesk (optionnel), optimisation temps réel.
- **Livrables :**
  - miyukini-livechat-chatbot (scripts, étapes, réponses, validation email, forward_operator)
  - Règles : chatbot_script_id, chatbot_only_if_no_operator
  - Digest (KPIs live chat, intégration MiyuNotify)
  - Commandes opérateur (/ticket, /help) et liaison Helpdesk si module présent
  - Optimisation bus/WebSocket (versioning, reconnexion)
- **Critères d'acceptation :**
  - Chatbot actif sur une règle ; collecte email ; transfert opérateur.
  - Digest périodique envoyé avec indicateurs live chat.
  - Opérateur peut créer/lier un ticket depuis le chat.

---

## 5. Bornage Fonctionnel

**In scope :**
- Canaux, opérateurs, règles (URL, pays, action, timer, chatbot)
- Sessions (création, attribution, clôture), messages, transcript, notation
- Widget (bouton, fenêtre, couleurs, textes), lien web_page, script externe
- Chatbot (scripts, étapes, questions, forward_operator)
- Rapports canal/opérateur, digest
- Intégration Discuss (conversations en fenêtre opérateur)

**Out of scope (ou phase ultérieure) :**
- Appels RTC/VoIP dans le chat (hors périmètre Odoo Live Chat standard)
- Publication des ratings sur une page publique (implémentable mais optionnel)
- GeoIP détaillé (dépendance externe ; interface uniquement)

---

## 6. Risques et Mitigation

| Risque | Mitigation |
|--------|-------------|
| Disponibilité opérateurs (charge) | Algorithme _get_less_active_operator ; seuils configurables |
| Bus/WebSocket indisponible | Fallback polling pour widget ; reconnexion automatique |
| Données sensibles (messages) | WorrySentinel niveau 2 ; chiffrement si exigé |
| Spam visiteur | Rate limiting Mandat Public d'Accès ; modération côté opérateur |
| Chatbot mal configuré | Validation étapes (au moins une étape) ; tests unitaires scripts |

---

## 7. Correspondance Miyukini

**Service :** LiveChatService (Équipe d'Opérateurs).  
**Correspondance produit :** MiyuLiveChat ou MiyukiniLiveChat = équivalent Odoo Live Chat.

**Réutilisation :** MiyuNotify, MiyuContacts, KindMother, StrongFather, Master Butler, WorrySentinel, Ever Buddy, module Discuss-like.

---

**Document** : Odoo Live Chat — Guide d'Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01
