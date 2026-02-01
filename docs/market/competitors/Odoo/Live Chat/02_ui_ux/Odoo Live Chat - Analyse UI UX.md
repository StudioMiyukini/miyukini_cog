# Odoo Live Chat — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Live Chat** d'Odoo (module im_livechat), identifiant les composants d'interface, patterns de navigation, formulaires, widget visiteur et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Code source Odoo 18.0 (im_livechat), documentation Odoo

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues backend (Kanban canaux, formulaire canal, règles, chatbot, ratings, rapports)
- Widget visiteur (bouton, fenêtre de chat, couleurs, textes)
- Intégration Discuss (conversations live chat)
- Patterns de navigation et actions
- Rapports et digest

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Backend — Canaux Live Chat

### 1.1 Tableau de bord / Kanban — Canaux

- **Vue :** Kanban des canaux (`im_livechat_channel_views.xml`).
- **Contenu des cartes :** Nom du canal, indicateurs (nombre de conversations, opérateurs, etc.).
- **Actions :** Menu (ellipse verticale) > Configure Channel pour ouvrir le formulaire canal.
- **Navigation :** Fil d’Ariane pour revenir au tableau de bord des canaux.

### 1.2 Formulaire — Canal (im_livechat.channel)

**Onglets principaux :**

- **Opérateurs (Operators)**  
  - Liste des opérateurs (utilisateurs) du canal.  
  - Créateur du canal ajouté par défaut.  
  - Actions : Add (pop-up Select/Créer utilisateurs), édition/suppression par boîte (Open: Operators pop-up).  
  - Attention : créer un nouvel utilisateur impacte la facturation (nombre d’utilisateurs).

- **Options**  
  - **Livechat button :**  
    - Notification text (texte de la bulle à côté du bouton).  
    - Livechat Button Color / couleur du texte (bouton).  
  - **Livechat window :**  
    - Welcome Message (message de bienvenue, envoyé comme opérateur).  
    - Chat Input Placeholder.  
    - Channel Header Color (barre du haut de la fenêtre).  
  - Couleurs : sélecteur (slider, RGB, HSL, HEX selon OS).

- **Channel Rules**  
  - Lignes de règles : URL Regex, Live Chat Button (Show / Show with notification / Open automatically / Hide), Open automatically timer, Country (si GeoIP), Chatbot.  
  - Bouton "Add a line" > pop-up Create Rules.  
  - Ordre de matching : séquence.

- **Widget**  
  - Script pour site externe : code à copier dans le `<head>` du site.  
  - Lien à envoyer au client : ouvre une nouvelle fenêtre de chat.  
  - Pour site Odoo : Configuration dans Website > Paramètres > Email & Marketing > Channel.

---

## 2. Widget Visiteur (Frontend)

### 2.1 Bouton Live Chat

- **Position :** Coin inférieur droit du site.
- **Apparence :** Couleur fond et texte configurables (button_background_color, button_text_color).
- **Texte :** button_text (ex. "Have a Question? Chat with us.").
- **Variantes (selon règle) :**  
  - Show : bouton seul.  
  - Show with notification : bouton + bulle de texte flottante.  
  - Open automatically : bouton + ouverture automatique après X secondes.  
  - Hide : bouton masqué.

### 2.2 Fenêtre de chat

- **En-tête :** Barre colorée (header_background_color), titre (title_color).
- **Corps :**  
  - Message de bienvenue (default_message).  
  - Historique des messages (visiteur / opérateur ou chatbot).  
- **Saisie :** Placeholder (input_placeholder), ex. "Say Something...".
- **Fermeture :** Visiteur quitte > session se ferme côté backend (livechat_active = False), proposition de notation.

### 2.3 Lien direct (Web Page)

- **Champ calculé :** web_page = URL du type `/im_livechat/support/{channel_id}`.
- **Usage :** Envoyer le lien au client ; au clic, ouverture d’une fenêtre de chat dédiée.

---

## 3. Intégration Discuss (Opérateur)

- **Emplacement :** Conversations live chat visibles dans Discuss (app Discuss ou icône messagerie).
- **Comportement :** Fenêtre en bas à droite ; même expérience que les messages directs.
- **Contenu :** Nom de la conversation (visiteur + opérateur ou titre chatbot), messages, indicateur opérateur (avatar, livechat_username).
- **Fonctionnalités opérateur :**  
  - Réponses préenregistrées (:shortcut).  
  - Commandes (/ticket, /search_tickets, /help).  
  - Historique, transcript, envoi du transcript par email.  
- **Profil anonyme :** Affichage "anonymous" / anonymous_name et pays (anonymous_country) si disponible.

---

## 4. Chatbot (Backend)

- **Vues :** chatbot_script (liste/formulaire), chatbot_script_step, chatbot_script_answer (éditeur de scripts, étapes, réponses).
- **Scripts :** Lignes de dialogue, types d’étapes (texte, question, forward_operator, etc.).
- **Lien canal :** Règles du canal peuvent associer un chatbot_script_id (optionnel) et "Enabled only if no operator".

---

## 5. Ratings et Rapports

- **Ratings :** Vue rating_rating filtrée par canal (action "View Rating" depuis le canal) ; satisfaction sur 14 jours.
- **Rapports :**  
  - `im_livechat_report_channel_views.xml` : rapport par canal.  
  - `im_livechat_report_operator_views.xml` : rapport par opérateur.  
- **Digest :** Vue digest_views ; résumés périodiques activité live chat.

---

## 6. Patterns de Navigation

- **Canal :** Live Chat > Kanban > carte > Configure Channel > formulaire (onglets Opérateurs, Options, Channel Rules, Widget).
- **Conversation :** Discuss > conversation live chat (liste ou fenêtre).
- **Ratings :** Canal > Action "View Rating" ou menu rapport.
- **Widget :** Depuis n’importe quelle page du site (ou lien web_page) selon règles URL/pays.

---

## 7. Points d’attention pour Miyukini

- **Séparation front/back :** Widget visiteur = Façade Publique Gouvernée ; backend = Opérateurs gouvernés.
- **Couleurs et textes :** Données de configuration canal (KindMother, WriteIntent) ; pas de logique dans le noyau.
- **Règles :** Éditeur de règles (URL, pays, action) comme interface de configuration ; moteur de règles côté service.
- **Temps réel :** Conserver le pattern "bus / WebSocket en façade" sans exposer la gouvernance.

---

**Document** : Odoo Live Chat — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
