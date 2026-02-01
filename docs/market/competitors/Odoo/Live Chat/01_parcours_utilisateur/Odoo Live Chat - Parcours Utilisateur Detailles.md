# Odoo Live Chat — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Live Chat** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Code source GitHub Odoo 18.0 / documentation Odoo

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs (visiteur, opérateur, administrateur canal)
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Visiteur (Website Visitor / Anonymous User)

**Profil :**
- Rôle externe : Consommateur du site web
- Responsabilités :
  - Ouvrir le widget Live Chat (bouton ou ouverture auto)
  - Saisir un nom (anonyme ou identifié si connecté)
  - Échanger en temps réel avec un opérateur ou un chatbot
  - Noter la conversation (satisfait / ok / insatisfait)
  - Optionnel : recevoir le transcript par email

**Besoins :**
- Bouton visible et clair (texte, couleur configurables)
- Message de bienvenue et placeholder de saisie
- Réponse rapide (opérateur ou chatbot)
- Fenêtre de chat simple (header, messages, saisie)
- Lien direct vers une session (widget tab ou URL partagée)

**Permissions :**
- Utilisateur externe (sans identité souveraine)
- Accès uniquement via Façade Publique Gouvernée (widget)
- Soumis aux règles du canal (URL, pays) et au Mandat Public d'Accès

### 1.2 Opérateur (Live Chat Operator / Agent)

**Profil :**
- Rôle interne : Répondre aux demandes des visiteurs
- Responsabilités :
  - Recevoir les conversations assignées au canal
  - Répondre en temps réel (fenêtre en bas à droite)
  - Utiliser les réponses préenregistrées (:shortcut)
  - Exécuter des commandes (/ticket, /search_tickets, /help)
  - Transférer vers un ticket Helpdesk si besoin
  - Consulter l'historique et le transcript

**Besoins :**
- Notifications de nouvelle conversation
- Vue Discuss unifiée (conversations live chat comme messages directs)
- Indication de disponibilité (im_status) et répartition de charge
- Réponses rapides (canned responses)
- Accès depuis n'importe où dans la base (pas bloqué sur une app)

**Permissions :**
- Utilisateur interne membre de `user_ids` du canal
- `_is_user_available()` détermine l’éligibilité à l’attribution
- Déconnexion après 30+ minutes d’inactivité (politique Odoo)

### 1.3 Administrateur Canal (Channel Administrator)

**Profil :**
- Rôle configuration : Créer et configurer les canaux Live Chat
- Responsabilités :
  - Créer des canaux (nom, opérateurs)
  - Configurer l’onglet Opérateurs (ajout/retrait)
  - Configurer l’onglet Options (bouton, fenêtre : textes, couleurs)
  - Configurer l’onglet Règles (URL regex, pays, action bouton, chatbot, timer)
  - Configurer l’onglet Widget (script externe, lien session)
  - Consulter les notations et rapports

**Besoins :**
- Formulaire canal complet (Opérateurs, Options, Règles, Widget)
- Règles par URL et pays (GeoIP si disponible)
- Intégration chatbot (scripts, étapes)
- Code d’intégration à copier (site Odoo vs site externe)
- Tableaux de bord et rapports (sessions, opérateurs, satisfaction)

**Permissions :**
- Droits sur `im_livechat.channel` et règles
- Accès aux rapports Live Chat et ratings

---

## 2. Parcours d'Onboarding

### 2.1 Activation Live Chat

- **Apps** : Recherche "Live Chat" > Installer.
- **Website** : Configuration > Paramètres > Email & Marketing > cocher Livechat > choisir canal > Enregistrer.
- **Helpdesk** : Configuration > Équipes Helpdesk > sélectionner une équipe > cocher Live Chat (Channels).

Après installation : un canal Live Chat est créé par défaut.

### 2.2 Premier canal

1. Aller dans l’app Live Chat.
2. Nouveau > Saisir le nom du canal.
3. Onglet Opérateurs : l’utilisateur créateur est ajouté par défaut ; ajouter d’autres opérateurs (Select / New).
4. Onglet Options : adapter texte du bouton, message de bienvenue, placeholder, couleurs (bouton, header).
5. Onglet Règles : ajouter une ligne > URL Regex (optionnel), Action (Show / Show with notification / Open automatically / Hide), Timer auto-popup, pays (optionnel), Chatbot (optionnel).
6. Onglet Widget : copier le script pour site externe ou configurer le canal dans Website > Paramètres pour le site Odoo.

### 2.3 Première conversation (opérateur)

1. S’assurer d’être dans la liste des opérateurs et d’être "disponible" (im_status).
2. Ouvrir le site avec le widget (ou le lien web_page du canal).
3. Lancer une conversation côté visiteur.
4. Réception de la conversation dans Discuss (fenêtre en bas à droite).
5. Répondre ; à la fin, le visiteur note la conversation.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Visiteur : obtenir de l’aide sur une page

1. Visiteur sur une page couverte par une règle (URL/pays).
2. Bouton Live Chat affiché (ou avec notification, ou ouverture auto après X secondes).
3. Clic sur le bouton > fenêtre de chat avec message de bienvenue.
4. Saisie du message > attribution à un opérateur (ou réponse chatbot).
5. Échange en temps réel.
6. Visiteur quitte > proposition de notation > (optionnel) commentaire si insatisfait.

### 3.2 Opérateur : répondre avec une réponse préenregistrée

1. Nouvelle conversation reçue dans Discuss.
2. Clic sur la conversation > fenêtre de chat.
3. Saisie `:shortcut` dans le champ > remplacement par le texte configuré.
4. Envoi du message.

### 3.3 Opérateur : créer un ticket depuis le chat

1. Conversation ouverte.
2. Saisie `/ticket` (ou commande équivalente selon config).
3. Création d’un ticket Helpdesk lié (si module Helpdesk installé) ou action configurée.

### 3.4 Admin : afficher le bouton uniquement sur certaines pages

1. Ouvrir le canal > onglet Règles.
2. Nouvelle règle : URL Regex (ex. page contact), Action "Show" ; une autre règle : URL Regex (ex. page accueil), Action "Hide".
3. Ordre de matching par séquence ; pays optionnel (GeoIP).

### 3.5 Admin : ouvrir le chat automatiquement après 10 secondes

1. Canal > Règles > nouvelle règle.
2. Action : "Open automatically".
3. Open automatically timer : 10 (secondes).
4. URL Regex : vide ou ciblé selon besoin.

---

## 4. Points de Friction Identifiés

- **Disponibilité opérateurs** : si aucun opérateur disponible et pas de chatbot, le visiteur ne peut pas démarrer de session ; message ou fallback à définir.
- **GeoIP** : règles par pays nécessitent GeoIP (installé par défaut en Odoo Online, configuration manuelle on-premise).
- **Limite 30 minutes** : opérateur considéré "inactif" après 30 min sans message ; comportement à aligner avec la politique RH.
- **Script externe** : intégration sur site tiers (CORS, domaine) peut nécessiter configuration serveur.
- **Rating** : affichage des notes sur le site nécessite une page avec slug `livechat` pour être reconnue par la base.
- **Création d’utilisateurs** : ajouter un nouvel opérateur peut impacter la facturation (nombre d’utilisateurs) ; privilégier l’ajout d’utilisateurs existants.

---

## 5. Recommandations pour Miyukini

- **Visiteur** : traiter comme Utilisateur Externe ; Façade Publique Gouvernée + Mandat Public d’Accès ; pas d’entrée dans le COG.
- **Opérateur** : Opérateur de Service (LiveChatOperator) avec capacités (répondre, canned responses, commandes) ; Mandat de Permission pour accès aux sessions du canal.
- **Canal et règles** : Opérateur de configuration (LiveChatChannelOperator) ou partie d’un Opérateur LiveChat ; règles URL/pays comme données gouvernées (KindMother, WriteIntent).
- **Chatbot** : scripts et étapes en données ; exécution dans un cadre gouverné (pas de logique métier dans le noyau).
- **Temps réel** : bus/WebSocket en façade uniquement ; pas de gouvernance partagée avec l’extérieur.
- **Notation** : réutiliser ou aligner avec un Opérateur Rating existant ; parent = canal (équivalent im_livechat.channel).

---

**Document** : Odoo Live Chat — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
