# Odoo Live Chat — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Live Chat** d'Odoo (module im_livechat), identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Code source GitHub Odoo 18.0 (im_livechat `__manifest__.py`, models, assets)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances explicites (mail, rating, digest, utm)
- Intégration Discuss / Mail
- Intégration Website
- Intégration Helpdesk (optionnelle)
- Bus et temps réel
- Recommandations Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (`__manifest__.py`)

- **mail** : Discuss, discuss.channel, mail.message, notifications, bus (temps réel).
- **rating** : notation satisfaction (rating.parent.mixin sur im_livechat.channel, rating.rating).
- **digest** : résumés périodiques activité live chat (digest_data, digest_views).
- **utm** : attribution source/campagne (UTM).

Aucune dépendance optionnelle déclarée dans le manifest ; les intégrations Website et Helpdesk sont activées par configuration (paramètres Website, équipes Helpdesk) ou présence des apps.

---

## 2. Intégrations Détaillées

### 2.1 Mail / Discuss

**Flux :**
```
Live Chat → discuss.channel (channel_type=livechat) → mail.message → bus
```

**Mécanismes :**
- Héritage et extension de `discuss.channel` (im_livechat ajoute champs livechat_*, chatbot_*, anonymous_*).
- Sessions live chat = canaux de type "livechat" ; opérateur et visiteur comme membres.
- Messages : mail.message (res_id = channel id, model = 'discuss.channel') ; sous-types comment/notification.
- Notifications : bus pour temps réel (nouveau message, typing, etc.).
- Chatter non utilisé sur le canal lui-même pour le flux visiteur ; côté opérateur, interface Discuss unifiée.

**Champs liés :**
- discuss.channel : message_ids, channel_member_ids, livechat_operator_id, livechat_channel_id.
- mail.message : author_id, body, date, subtype.

**Recommandations Miyukini :**
- Réutiliser ou aligner avec un Opérateur de messagerie (Discuss-like) pour les conversations ; Live Chat comme type de canal gouverné.
- Bus/WebSocket en façade uniquement ; pas de gouvernance partagée avec l’extérieur.

### 2.2 Rating

**Flux :**
```
discuss.channel (livechat) → rating.rating (parent = livechat_channel_id) → im_livechat.channel (rating.parent.mixin)
```

**Mécanismes :**
- rating.parent.mixin sur im_livechat.channel ; _rating_satisfaction_days = 14.
- Clôture session → proposition de notation (3 niveaux : satisfait / ok / insatisfait) ; commentaire si insatisfait.
- Rapport satisfaction par canal ; affichage sur site possible (page avec slug livechat).

**Champs liés :**
- rating.rating : res_id, res_model, parent (im_livechat.channel), rated_partner_id, rating.

**Recommandations Miyukini :**
- Opérateur Rating existant ou dédié ; parent = canal (équivalent im_livechat.channel) ; pas de logique métier dans le noyau.

### 2.3 Digest

**Flux :**
```
im_livechat (sessions, opérateurs) → digest (KPIs) → email digest
```

**Mécanismes :**
- digest_data.xml : indicateurs live chat (nombre de sessions, par opérateur, etc.).
- digest_views : configuration des digests (fréquence, contenus).
- Envoi périodique par email aux utilisateurs abonnés.

**Recommandations Miyukini :**
- Intégration avec MiyuNotify ou service digest existant ; KPIs live chat comme données agrégées gouvernées.

### 2.4 UTM

**Flux :**
```
Session / visiteur → utm_source, utm_medium, utm_campaign (attribution)
```

**Mécanismes :**
- Utilisation des champs UTM pour l’attribution des sessions (source, campagne) si exposés côté frontend/backend.
- Pas de modèle dédié dans im_livechat ; dépendance déclarée pour cohérence avec marketing/website.

**Recommandations Miyukini :**
- Si besoin d’attribution : lier à un module UTM ou champs équivalents sur la session/visiteur en façade.

### 2.5 Website

**Flux :**
```
Website (paramètres) → canal Live Chat sélectionné → widget sur les pages du site
```

**Mécanismes :**
- Configuration : Website > Configuration > Paramètres > Email & Marketing > Livechat > cocher et choisir le canal.
- Widget chargé sur les pages du site Odoo selon règles du canal (URL, pays).
- Script externe (onglet Widget du canal) pour sites hors Odoo : copier le script dans le `<head>` ; CORS si domaine différent.

**Recommandations Miyukini :**
- Widget = Façade Publique Gouvernée ; site Miyukini (MiyuWeb) ou site tiers : même principe (canal configuré, script/lien en façade).

### 2.6 Helpdesk

**Flux :**
```
Helpdesk Teams → Configuration → Live Chat (checkbox) → canal(s) lié(s)
```

**Mécanismes :**
- Depuis l’app Helpdesk : Configuration > Helpdesk Teams > sélectionner une équipe > section Channels > cocher Live Chat.
- Permet d’activer Live Chat sans passer par l’app Live Chat en premier ; création/utilisation d’un canal.
- Commandes opérateur (/ticket, etc.) peuvent créer ou lier un ticket Helpdesk.

**Recommandations Miyukini :**
- Si équivalent Helpdesk existe : contrat d’équipe entre LiveChatService et HelpdeskService ; Mandat pour création/liaison de tickets depuis une session.

### 2.7 Bus (Temps réel)

**Flux :**
```
Frontend (widget / Discuss) ↔ bus (WebSocket) ↔ discuss.channel / mail.message
```

**Mécanismes :**
- bus : notifications temps réel (nouveaux messages, typing, statut opérateur).
- Assets im_livechat : inclusion bus, websocket_worker ; embed (frontend visiteur) et backend (Discuss) utilisent le bus.
- Version websocket_worker exposée dans get_livechat_info pour compatibilité client.

**Recommandations Miyukini :**
- Bus/WebSocket en bordure uniquement ; pas de décision métier ni gouvernance exposée sur le bus.

---

## 3. APIs et Hooks

- **get_livechat_info(username)** : appelé par le frontend pour savoir si le canal est disponible (opérateurs ou chatbot), options (couleurs, textes), server_url, websocket_worker_version.
- **_get_livechat_discuss_channel_vals(...)** : création d’une session discuss.channel (membres, livechat_operator_id, livechat_channel_id, chatbot_current_step_id, etc.).
- **match_rule(channel_id, url, country_id)** : im_livechat.channel.rule ; utilisé côté backend lors de l’ouverture d’une session pour déterminer l’action (bouton, auto-popup, chatbot).
- **Discuss Store (_to_store)** : discuss.channel étend le store pour exposer anonymous_name, anonymous_country, operator, livechatChannel, livechat_active, chatbot (steps, currentStep).

---

## 4. Synthèse des Dépendances

| Module   | Type        | Rôle principal                                      |
|----------|-------------|------------------------------------------------------|
| mail     | Requis      | discuss.channel, mail.message, notifications, bus    |
| rating   | Requis      | Notation satisfaction canal                         |
| digest   | Requis      | KPIs et digests email                               |
| utm      | Requis      | Attribution source/campagne                         |
| website  | Config      | Paramètre canal, widget sur site Odoo               |
| helpdesk | Config      | Activation Live Chat depuis équipes, /ticket        |

---

## 5. Recommandations Miyukini

- **Frontière claire** : Visiteur = Utilisateur Externe ; widget = Façade Publique Gouvernée ; pas d’entrée dans le COG.
- **Discuss** : Réutiliser ou aligner un Opérateur de messagerie (type canal livechat) ; Mandats pour opérateurs.
- **Rating / Digest** : Réutiliser Opérateurs existants ; parent = canal.
- **Website / Helpdesk** : Contrats d’équipe et Mandats pour configuration et création de tickets.
- **Bus** : Canal technique en bordure ; pas de gouvernance sur le bus.

---

**Document** : Odoo Live Chat — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
