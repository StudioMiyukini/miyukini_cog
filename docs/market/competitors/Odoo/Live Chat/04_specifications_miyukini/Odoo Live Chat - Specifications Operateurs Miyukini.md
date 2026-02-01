# Odoo Live Chat — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Live Chat** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Live Chat
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **LiveChatChannelOperator** | Gestion des canaux et règles | Opérateur de Service |
| **LiveChatSessionOperator** | Gestion des sessions (création, clôture, attribution) | Opérateur de Service |
| **LiveChatMessageOperator** | Envoi/réception messages, transcript | Opérateur de Service |
| **LiveChatChatbotOperator** | Scripts, étapes, réponses chatbot | Opérateur de Service |
| **LiveChatRatingOperator** | Notation satisfaction canal | Opérateur de Service |
| **LiveChatUI** | Interface backend (canaux, Discuss) | Opérateur d'Interface |
| **LiveChatWidget** | Façade publique (widget visiteur) | Façade Publique Gouvernée |

### 1.2 Équipe d'Opérateurs : LiveChatService

**Définition :**
> **LiveChatService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de chat en direct avec les visiteurs du site.**

**Composition :**
- LiveChatChannelOperator (niveau sécurité 2)
- LiveChatSessionOperator (niveau sécurité 2)
- LiveChatMessageOperator (niveau sécurité 2)
- LiveChatChatbotOperator (niveau sécurité 1–2)
- LiveChatRatingOperator (niveau sécurité 1)
- LiveChatUI (niveau sécurité 1–2)
- LiveChatWidget : Façade Publique Gouvernée (Utilisateur Externe, Mandat Public d'Accès)

---

## 2. Opérateurs Détaillés

### 2.1 LiveChatChannelOperator

**Rôle :** Gestion des canaux Live Chat (création, configuration, opérateurs, règles, options, widget).

**Capacités :**
- Création/modification de canaux (nom, opérateurs, couleurs, textes)
- Gestion des règles (URL regex, pays, action bouton, timer, chatbot)
- Génération script externe et lien web_page
- Calcul opérateurs disponibles (availability)
- Réponse get_livechat_info (available, options, server_url)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification canal, ajout/retrait opérateurs
- **KindMother** : Persistance canaux et règles (WriteIntent)
- **Master Butler** : Permissions canal (création, configuration)
- **WorrySentinel** : Niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Cycle de vie canal

**Contrat d'équipe :**
- Consomme : LiveChatSessionOperator (sessions), LiveChatChatbotOperator (scripts), MiyuNotify (digest), MiyuContacts (pays si GeoIP)
- Expose : `channel.create`, `channel.update`, `channel.get_livechat_info`, `channel.match_rule`

**Mandat de Permission requis :**
- Création canal : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification canal : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Ajout opérateur : Mandat avec StrongFather (décision) + Master Butler (permissions)

### 2.2 LiveChatSessionOperator

**Rôle :** Gestion des sessions (création, attribution opérateur, clôture, vacuum).

**Capacités :**
- Création session (anonymous_name, previous_operator_id, url, country_id, lang, chatbot_script)
- Attribution opérateur (priorité précédent, langue, pays, charge)
- Clôture session (livechat_active = False, message "Visitor left")
- Vacuum sessions vides (> 1 h sans message)
- Gestion membres canal (opérateur + visiteur)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création session (règles, disponibilité)
- **KindMother** : Persistance sessions (WriteIntent)
- **Master Butler** : Permissions session (création, clôture)
- **WorrySentinel** : Niveau sécurité, isolation
- **Ever Buddy** : Cycle de vie session

**Contrat d'équipe :**
- Consommé par : LiveChatChannelOperator (get_livechat_info appelle _get_operator)
- Consomme : LiveChatChannelOperator (canal, règles, opérateurs), LiveChatChatbotOperator (chatbot), LiveChatMessageOperator (messages)
- Expose : `session.create`, `session.close`, `session.assign_operator`

**Mandat de Permission requis :**
- Création session : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + LiveChatChannelOperator (match_rule, _get_operator)
- Clôture session : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.3 LiveChatMessageOperator

**Rôle :** Envoi/réception messages, transcript, envoi email transcript.

**Capacités :**
- Post message (visiteur, opérateur, chatbot)
- Historique messages (tri par id)
- Transcript (texte pour email)
- Envoi transcript par email
- Intégration bus (temps réel) en façade

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision envoi message (selon Mandat)
- **KindMother** : Persistance messages (WriteIntent)
- **Master Butler** : Permissions envoi/lecture
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consommé par : LiveChatSessionOperator
- Consomme : MiyuNotify (notifications), bus (façade)
- Expose : `message.post`, `message.history`, `message.transcript`, `message.send_transcript_email`

**Mandat de Permission requis :**
- Post message : Mandat avec KindMother (WriteIntent) + StrongFather (décision) + session active
- Envoi transcript : Mandat avec StrongFather (décision) + MiyuNotify (email)

### 2.4 LiveChatChatbotOperator

**Rôle :** Scripts chatbot (étapes, réponses, validation email, forward_operator).

**Capacités :**
- Création/modification scripts et étapes
- Réponses préenregistrées (texte, question, sélection)
- Validation email visiteur
- Transfert vers opérateur (forward_operator)
- Restart script
- Création chatbot.message à chaque message si step actif

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données collectées)

**Gouvernance :**
- **StrongFather** : Décision exécution étape (pas d’exécution métier dans le noyau)
- **KindMother** : Persistance scripts, étapes, réponses, chatbot.message (WriteIntent)
- **Master Butler** : Permissions script/étape
- **WorrySentinel** : Niveau sécurité (données collectées)

**Contrat d'équipe :**
- Consommé par : LiveChatChannelOperator (règles), LiveChatSessionOperator (session avec chatbot_current_step_id)
- Consomme : LiveChatMessageOperator (post message chatbot)
- Expose : `chatbot.script.create`, `chatbot.step.execute`, `chatbot.validate_email`, `chatbot.restart`

**Mandat de Permission requis :**
- Exécution étape : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Collecte email : Mandat avec WorrySentinel (niveau données sensibles)

### 2.5 LiveChatRatingOperator

**Rôle :** Notation satisfaction canal (satisfait / ok / insatisfait), commentaire, rapport satisfaction.

**Capacités :**
- Enregistrement rating (parent = canal)
- Satisfaction sur N jours (ex. 14)
- Affichage ratings sur site (page slug livechat) si configuré
- Action "View Rating" depuis canal

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **StrongFather** : Décision publication sur site (optionnel)
- **KindMother** : Persistance ratings (WriteIntent)
- **Master Butler** : Permissions notation/lecture
- **WorrySentinel** : Niveau données (anonymisées ou non)

**Contrat d'équipe :**
- Consommé par : LiveChatChannelOperator (action_view_rating)
- Consomme : KindMother (WriteIntent)
- Expose : `rating.create`, `rating.list_by_channel`, `rating.satisfaction_stats`

**Mandat de Permission requis :**
- Création rating : Mandat avec KindMother (WriteIntent) + StrongFather (décision) (session clôturée)
- Consultation : Mandat avec Master Butler (permissions canal)

### 2.6 LiveChatUI

**Rôle :** Interface backend (Kanban canaux, formulaire canal, Discuss, rapports, digest).

**Capacités :**
- Vue Kanban canaux, formulaire canal (onglets Opérateurs, Options, Règles, Widget)
- Intégration Discuss (conversations live chat en fenêtre)
- Vues chatbot (scripts, étapes, réponses)
- Rapports canal/opérateur, digest
- Actions : Configure Channel, View Rating, View Chatbots

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données affichées)

**Gouvernance :**
- **StrongFather** : Décision affichage (selon Mandat)
- **Master Butler** : Permissions vues
- **WorrySentinel** : Niveau sécurité affichage

**Contrat d'équipe :**
- Consomme : LiveChatChannelOperator, LiveChatSessionOperator, LiveChatMessageOperator, LiveChatChatbotOperator, LiveChatRatingOperator, Discuss (équivalent)
- Expose : UI uniquement (pas d’exécution métier directe)

**Mandat de Permission requis :**
- Accès canal : Mandat avec Master Butler (permissions canal)
- Accès session : Mandat avec Master Butler (permissions session/opérateur)

### 2.7 LiveChatWidget (Façade Publique Gouvernée)

**Rôle :** Façade d’exposition pour Utilisateur Externe (visiteur du site).

**Caractéristiques :**
- Strictement unidirectionnelle (le COG sort vers le visiteur, jamais l’inverse en termes de gouvernance)
- Sans identité persistante obligatoire (anonyme ou identifié selon site)
- Sans accès aux Cores
- Soumis à Mandat Public d’Accès (quotas, rate limits, services exposés)

**Capacités exposées :**
- get_livechat_info (available, options, server_url) — lecture seule
- Ouverture session (intention visiteur) → traduite en demande gouvernée côté COG
- Envoi message, réception messages (via bus en façade)
- Notation (session clôturée)

**Règle fondamentale :**
> Un utilisateur externe n’entre jamais dans un COG. Il interagit uniquement avec une façade d’exposition gouvernée.

---

## 3. Contrat d'Équipe LiveChatService

**Membres :** LiveChatChannelOperator, LiveChatSessionOperator, LiveChatMessageOperator, LiveChatChatbotOperator, LiveChatRatingOperator, LiveChatUI.

**Flux autorisés :**
- LiveChatUI → LiveChatChannelOperator (config, get_livechat_info, match_rule)
- LiveChatUI → LiveChatSessionOperator (liste sessions, clôture)
- LiveChatUI → LiveChatMessageOperator (messages, transcript)
- LiveChatUI → LiveChatChatbotOperator (scripts, étapes)
- LiveChatUI → LiveChatRatingOperator (ratings, rapports)
- LiveChatChannelOperator → LiveChatSessionOperator (création session via _get_livechat_discuss_channel_vals)
- LiveChatSessionOperator → LiveChatMessageOperator (post message, transcript)
- LiveChatSessionOperator → LiveChatChatbotOperator (étape courante, restart)
- LiveChatSessionOperator → LiveChatRatingOperator (création rating à clôture)
- LiveChatWidget (façade) → traduction intention → LiveChatChannelOperator / LiveChatSessionOperator (via BondingBrother)

**Types d’échanges :** Données canal, session, message, rating ; pas de données sensibles non nécessaires.

**Conditions préalables :** Mandat de Permission valide pour chaque flux métier.

**Niveau de validation :** StrongFather + KindMother (WriteIntent) pour toute écriture ; Master Butler pour permissions ; WorrySentinel pour niveau sécurité.

---

## 4. Niveaux de Sécurité

| Opérateur | Niveau | Justification |
|-----------|--------|----------------|
| LiveChatChannelOperator | 2 (Sensitive) | Données canal, opérateurs, règles |
| LiveChatSessionOperator | 2 (Sensitive) | Sessions, attribution, visiteur |
| LiveChatMessageOperator | 2 (Sensitive) | Contenu messages, transcript |
| LiveChatChatbotOperator | 1–2 | Scripts publics ; données collectées (email) sensibles |
| LiveChatRatingOperator | 1 (Standard) | Notes agrégées, commentaires optionnels |
| LiveChatUI | 1–2 | Affichage selon données |
| LiveChatWidget | Façade | Utilisateur Externe ; Mandat Public d’Accès |

---

## 5. Correspondance Miyukini

**Service proposé :** LiveChatService (Équipe d’Opérateurs).

**Opérateurs / composants existants à réutiliser :**
- MiyuNotify (notifications, digest, email transcript)
- MiyuContacts (pays si GeoIP)
- KindMother (WriteIntent)
- StrongFather (décisions)
- Master Butler (Mandats, permissions)
- WorrySentinel (niveaux sécurité)
- Ever Buddy (cycle de vie)
- Composant Discuss-like (messagerie) pour conversations opérateur

**Nouveaux crates / modules suggérés :**
- miyukini-livechat-channel (LiveChatChannelOperator)
- miyukini-livechat-session (LiveChatSessionOperator)
- miyukini-livechat-message (LiveChatMessageOperator) ou intégration dans module Discuss existant
- miyukini-livechat-chatbot (LiveChatChatbotOperator)
- miyukini-livechat-rating (LiveChatRatingOperator) ou réutilisation miyurating si existant
- miyukini-livechat-ui (LiveChatUI)
- Façade widget (LiveChatWidget) dans miyuweb ou module dédié façade

**Correspondance globale :** MiyuLiveChat ou MiyukiniLiveChat (LiveChatService) = équivalent Odoo Live Chat.

---

**Document** : Odoo Live Chat — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
