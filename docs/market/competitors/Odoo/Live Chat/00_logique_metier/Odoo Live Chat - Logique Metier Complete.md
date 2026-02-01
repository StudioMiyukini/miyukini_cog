# Odoo Live Chat — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Live Chat** d'Odoo (module `im_livechat`, versions 18.0 / 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/18.0/addons/im_livechat`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (im_livechat.channel, im_livechat.channel.rule, discuss.channel livechat, chatbot.script, rating)
- Règles métier et contraintes (canal, sessions, opérateurs, règles URL/pays)
- Workflows (création session, attribution opérateur, clôture, notation)
- Chatbot (étapes, réponses, transfert opérateur)
- Intégration Discuss, Rating, UTM, Digest

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `im_livechat.channel` (Canal Live Chat)

**Rôle :** Définit un **canal de communication** accessible via script externe, script interne (site Odoo) ou lien web. Fournit outils de notation et règles d'accès pour visiteurs anonymes.

**Champs clés :**

#### Identification et apparence
- `name` : Nom du canal (obligatoire)
- `button_text` : Texte du bouton (défaut : "Have a Question? Chat with us.", traduisible)
- `default_message` : Message de bienvenue (défaut : "How may I help you?", traduisible)
- `input_placeholder` : Placeholder du champ de saisie (traduisible)
- `header_background_color` : Couleur fond en-tête (défaut #875A7B)
- `title_color` : Couleur titre (défaut #FFFFFF)
- `button_background_color` : Couleur fond bouton (défaut #875A7B)
- `button_text_color` : Couleur texte bouton (défaut #FFFFFF)
- `image_128` : Image du canal

#### Calculés
- `web_page` : URL page statique où le visiteur peut discuter (compute, non stocké)
- `are_you_inside` : Boolean "Êtes-vous opérateur de ce canal" (compute)
- `available_operator_ids` : Many2many opérateurs disponibles (filtrés par `_is_user_available()`)
- `script_external` : HTML du script d'intégration externe (compute)
- `nbr_channel` : Nombre de conversations du canal (compute)

#### Relationnels
- `user_ids` : Many2many vers `res.users` (Opérateurs, défaut : utilisateur créateur)
- `channel_ids` : One2many vers `discuss.channel` (Sessions)
- `rule_ids` : One2many vers `im_livechat.channel.rule` (Règles)
- `chatbot_script_count` : Nombre de chatbots distincts liés aux règles (compute)

**Héritage :** `rating.parent.mixin` (_rating_satisfaction_days = 14)

**Règles métier :**
- Un canal sans opérateur disponible et sans chatbot associé aux règles ne peut pas créer de session (`_get_livechat_discuss_channel_vals` retourne False).
- Attribution opérateur : priorité opérateur précédent si disponible, puis même langue, puis même pays, sinon opérateur le moins chargé (nombre de chats actifs, pas en appel).
- Chat "actif" : au moins un message dans les 30 dernières minutes.

**Méthodes clés :**
- `_get_operator(previous_operator_id, lang, country_id)` : retourne un opérateur ou False
- `_get_less_active_operator(operator_statuses, operators)` : opérateur le moins chargé (pas en appel prioritaire)
- `_get_livechat_discuss_channel_vals(...)` : valeurs pour créer une session discuss.channel
- `get_livechat_info(username)` : infos pour le widget (available, options, server_url)
- `action_join` / `action_quit` : ajout/retrait opérateur du canal

---

### 1.2 Modèle `im_livechat.channel.rule` (Règles de canal)

**Rôle :** Règles définissant l'accès au canal (URL, pays) et le comportement du bouton (affichage, notification, ouverture auto, masquage). Option chatbot.

**Champs :**
- `channel_id` : Many2one vers `im_livechat.channel`
- `regex_url` : Expression régulière sur l'URL de la page
- `action` : Selection — `display_button` (Show), `display_button_and_text` (Show with notification), `auto_popup` (Open automatically), `hide_button` (Hide)
- `auto_popup_timer` : Délai en secondes avant ouverture auto (si action = auto_popup)
- `chatbot_script_id` : Many2one vers `chatbot.script` (optionnel)
- `chatbot_only_if_no_operator` : Boolean — activer le bot seulement si aucun opérateur disponible
- `country_ids` : Many2many vers `res.country` (règle appliquée pour ces pays uniquement ; nécessite GeoIP)
- `sequence` : Ordre de matching (défaut 10)

**Règles métier :**
- `match_rule(channel_id, url, country_id)` : première règle qui matche (d'abord règles avec pays si country_id, puis règles sans pays). URL matche via `re.search(regex_url or "", url or "")`.
- Si `chatbot_script_id` présent : script doit être actif et avoir des étapes.
- Si `chatbot_only_if_no_operator` et opérateurs disponibles : la règle est ignorée.

---

### 1.3 Extension `discuss.channel` (Session Live Chat)

**Rôle :** Conversation entre visiteur(s) et opérateur. Hérite de `discuss.channel` (mail) et `rating.mixin`.

**Champs ajoutés (im_livechat) :**
- `anonymous_name` : Nom anonyme du visiteur
- `channel_type` : Selection add `('livechat', 'Livechat Conversation')` (ondelete cascade)
- `duration` : Float (durée session en heures, compute depuis premier/dernier message)
- `livechat_active` : Boolean — session active tant que le visiteur n'a pas quitté
- `livechat_channel_id` : Many2one vers `im_livechat.channel`
- `livechat_operator_id` : Many2one vers `res.partner` (opérateur, obligatoire si channel_type = livechat)
- `chatbot_current_step_id` : Many2one vers `chatbot.script.step`
- `chatbot_message_ids` : One2many vers `chatbot.message`
- `country_id` : Many2one vers `res.country` (pays du visiteur)

**Contrainte SQL :** `(channel_type = 'livechat' AND livechat_operator_id IS NOT NULL) OR (channel_type != 'livechat')`

**Règles métier :**
- Clôture session : `_close_livechat_session()` met `livechat_active = False`, notifie "Visitor left the conversation", fold_state = "closed", sortie appel RTC.
- Vacuum : `_gc_empty_livechat_sessions` supprime les sessions livechat sans aucun message, créées il y a plus d'1 heure.
- Rating : parent = `livechat_channel_id` (_rating_get_parent_field_name).
- Chatbot : création `chatbot.message` à chaque message si `chatbot_current_step_id` ; validation email, restart script, etc.

**Méthodes clés :**
- `_close_livechat_session()` : désactive livechat et notifie
- `_email_livechat_transcript(email)` : envoie transcript par email
- `_get_channel_history()` : historique en texte pour transcript
- `execute_command_history` : envoie commande bus pour historique
- Hooks chatbot : `_chatbot_post_message`, `_chatbot_validate_email`, `_chatbot_restart`, `_chatbot_find_customer_values_in_messages`

---

### 1.4 Modèles Chatbot (résumé)

- **chatbot.script** : Script de dialogue (titre, étapes, opérateur partenaire).
- **chatbot.script.step** : Étapes (texte, question, types, réponses possibles, forward_operator).
- **chatbot.script.answer** : Réponses proposées pour les étapes question.
- **chatbot.message** : Lien mail.message / discuss_channel_id / script_step_id, réponses utilisateur.

---

### 1.5 Rating

- **rating.rating** : Notation (satisfait / ok / insatisfait) liée au canal via `rating.parent.mixin` sur `im_livechat.channel`.
- Satisfaction canal : sur les 14 derniers jours (_rating_satisfaction_days).

---

## 2. Workflows

### 2.1 Création d'une session Live Chat

1. Visiteur ouvre le widget (site Odoo ou externe via script).
2. Frontend appelle `get_livechat_info` puis demande création session (anonymous_name, previous_operator_id, url, country_id, lang, chatbot_script selon règle).
3. Backend : `match_rule(channel_id, url, country_id)` → règle appliquée (action bouton, chatbot éventuel).
4. Si chatbot seul (règle avec chatbot, pas d'opérateur humain) : création session avec `chatbot_current_step_id` et `operator_partner_id` = chatbot.
5. Sinon : `_get_operator(previous_operator_id, lang, country_id)` → opérateur ou False. Si False, pas de session.
6. Création `discuss.channel` avec `channel_type='livechat'`, `livechat_channel_id`, `livechat_operator_id`, membres (opérateur + visiteur si user_id), `livechat_active=True`.
7. Conversation : messages via mail.message, bus pour temps réel.

### 2.2 Clôture et notation

1. Visiteur quitte → `_close_livechat_session()` : livechat_active = False, message "Visitor left", fold closed.
2. Proposition notation (rating) : 3 niveaux (satisfait / ok / insatisfait) ; commentaire possible si pas satisfait.
3. Rating enregistré sur `rating.rating`, parent = canal (im_livechat.channel).
4. Digest : statistiques live chat (rapports, digest email).

---

## 3. Intégrations Métier

- **Mail / Discuss** : discuss.channel, mail.message, notifications, bus.
- **Rating** : notation satisfaction canal, affichage sur site (page /livechat).
- **UTM** : source/campagne pour attribution.
- **Digest** : résumés périodiques activité live chat.
- **Website** : paramètre canal dans Configuration > Paramètres (Email & Marketing > Livechat).
- **Helpdesk** : activation Live Chat depuis équipe Helpdesk (Configuration > Helpdesk Teams > Live Chat).

---

## 4. Points d'attention pour Miyukini

- **Opérateurs** : notion d’"availability" (im_status, pas en appel, nombre de chats actifs) à reproduire côté gouvernance (Master Butler / capacités).
- **Règles par URL/pays** : moteur de règles (regex, GeoIP) comme outil ou Opérateur dédié.
- **Chatbot** : scripts et étapes = données gouvernées (KindMother, WriteIntent) ; pas d’exécution métier dans le noyau.
- **Sessions anonymes** : visiteur = Utilisateur Externe (Façade Publique Gouvernée, Mandat Public d'Accès).
- **Temps réel** : bus / WebSocket hors COG ; canal diplomatique ou façade exposée uniquement.

---

**Document** : Odoo Live Chat — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
