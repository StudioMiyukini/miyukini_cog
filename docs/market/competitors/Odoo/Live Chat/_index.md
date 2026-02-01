# Odoo Live Chat — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Live Chat** d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Code source Odoo 18.0 (module im_livechat)

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Live Chat - Logique Métier Complète](./00_logique_metier/Odoo%20Live%20Chat%20-%20Logique%20Metier%20Complete.md)**
  - Modèles im_livechat.channel, im_livechat.channel.rule, discuss.channel (livechat), chatbot, rating
  - Règles métier et workflows (création session, attribution opérateur, clôture, notation)
  - Chatbot (étapes, réponses, transfert opérateur)
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Live Chat - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Live%20Chat%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas (visiteur, opérateur, administrateur canal)
  - Parcours d'onboarding et activation Live Chat
  - Scénarios d'usage (visiteur, opérateur, admin)
  - Points de friction et recommandations Miyukini

### 02_ui_ux/
- **[Odoo Live Chat - Analyse UI/UX](./02_ui_ux/Odoo%20Live%20Chat%20-%20Analyse%20UI%20UX.md)**
  - Vues backend (Kanban canaux, formulaire canal, règles, chatbot, ratings, rapports)
  - Widget visiteur (bouton, fenêtre de chat, couleurs, textes)
  - Intégration Discuss (conversations live chat)
  - Patterns de navigation

### 03_integrations/
- **[Odoo Live Chat - Intégrations Cross-App](./03_integrations/Odoo%20Live%20Chat%20-%20Integrations%20Cross%20App.md)**
  - Dépendances (mail, rating, digest, utm)
  - Intégration Discuss / Mail, Rating, Digest, Website, Helpdesk
  - Bus et temps réel
  - Recommandations Miyukini

### 04_specifications_miyukini/
- **[Odoo Live Chat - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Live%20Chat%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (LiveChatChannel, Session, Message, Chatbot, Rating, UI, Widget)
  - Équipe d'Opérateurs LiveChatService
  - Contrat d'Équipe et Mandats de Permission
  - Niveaux de sécurité (1–2) et Façade Publique Gouvernée
  - Correspondance Miyukini : MiyuLiveChat / MiyukiniLiveChat (LiveChatService)

### 05_integration_cog/
- **[Odoo Live Chat - Guide Intégration COG](./05_integration_cog/Odoo%20Live%20Chat%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG (LiveChatService)
  - Patterns WriteIntent et Mandats (canal, session, message, rating)
  - Exemples de code pseudo-Rust
  - Façade widget (Utilisateur Externe, Mandat Public d'Accès)

### 06_guides_implementation/
- **[Odoo Live Chat - Guide Implémentation](./06_guides_implementation/Odoo%20Live%20Chat%20-%20Guide%20Implementation.md)**
  - Architecture technique (crates Rust : channel, session, message, chatbot, rating, UI)
  - Schémas de données (canal, règle, session, message, rating)
  - API et contrats
  - Plan de développement par phases (MVP → Essentiel → Complet)
  - Bornage fonctionnel et critères d'acceptation
  - Risques et mitigation

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Canaux Live Chat**
   - Création, configuration (opérateurs, options, règles, widget)
   - Règles par URL (regex) et pays (GeoIP) : afficher/masquer bouton, ouverture auto
   - Script externe et lien web_page pour sites tiers

2. **Sessions et messages**
   - Création session (visiteur anonyme ou identifié), attribution opérateur (priorité précédent, langue, pays, charge)
   - Messages temps réel (Discuss / bus)
   - Clôture session, transcript, envoi transcript par email

3. **Chatbot**
   - Scripts et étapes (texte, question, forward_operator)
   - Réponses préenregistrées, validation email
   - Règles canal : chatbot_script_id, chatbot_only_if_no_operator

4. **Notation et rapports**
   - Rating (satisfait / ok / insatisfait), commentaire si insatisfait
   - Satisfaction canal sur 14 jours
   - Rapports canal/opérateur, digest

5. **Intégrations**
   - Mail/Discuss (canaux livechat, messages, notifications)
   - Rating, Digest, UTM
   - Website (paramètre canal, widget sur site Odoo)
   - Helpdesk (activation Live Chat depuis équipes, commandes /ticket)

### Architecture Miyukini Proposée

**Équipe d'Opérateurs :** LiveChatService

**Opérateurs :**
- LiveChatChannelOperator (canaux, règles, get_livechat_info, match_rule)
- LiveChatSessionOperator (sessions, attribution, clôture)
- LiveChatMessageOperator (messages, transcript, email)
- LiveChatChatbotOperator (scripts, étapes, réponses)
- LiveChatRatingOperator (notation satisfaction canal)
- LiveChatUI (interface backend)
- LiveChatWidget (Façade Publique Gouvernée — Utilisateur Externe)

**Réutilisation :** MiyuNotify, MiyuContacts, KindMother, StrongFather, Master Butler, WorrySentinel, Ever Buddy, module Discuss-like

**Niveaux de sécurité :** 1–2 (Standard à Sensitive) ; widget = Façade (Mandat Public d'Accès)

**Correspondance Miyukini :** MiyuLiveChat / MiyukiniLiveChat (LiveChatService)

---

## Statut de l'Analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

## Prochaines Étapes

1. **Valider les spécifications** : Revue avec l'équipe technique
2. **Démarrer l'implémentation** : Phase 1 (MVP) selon le guide
3. **Itérer** : Selon retours et besoins utilisateurs
4. **Intégration** : Avec Miyukini Website, Helpdesk, Discuss

---

**Document** : Odoo Live Chat — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100 % — référence pour implémentation Miyukini
