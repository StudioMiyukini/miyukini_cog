# Odoo Referrals — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Referrals** d'Odoo (18.0 / 19.0), à partir de la documentation officielle. Il identifie les vues, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0 — Applications HR / Referrals

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dashboard principal (points, niveau, anneau, boutons d’action)
- Onboarding (slides, Skip, Next, Start Now)
- Écran Hired (choix d’avatar pour parrainé embauché)
- View Jobs (cartes postes, partage email / SMS / WhatsApp / réseaux sociaux)
- My Referrals (cartes parrainages, badges, barre de progression, stages)
- Rewards (boutique récompenses, Buy)
- Level up (affichage et interaction)
- Configuration (Onboarding, Levels, Friends, Rewards, Alerts) et Reporting

**Hors scope :**
- Implémentation technique détaillée (guide d’implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Onboarding (Première ouverture)

**Rôle :** Présentation en 4 slides avant l’accès au dashboard.

**Caractéristiques :**
- Message fixe en haut : **GATHER YOUR TEAM! Job Referral Program** + image + texte.
- Chaque slide : image + message (texte configurable par l’admin).
- Boutons : **Next** (slide suivante), **Skip** (accès direct au dashboard), **Start Now** (à la fin, ferme définitivement l’onboarding pour cet utilisateur).
- Si l’utilisateur quitte sans cliquer **Start Now**, les slides réapparaissent à la prochaine ouverture.

**Configuration (Admin) :** Referrals > Configuration > Onboarding — édition du texte, de l’image, de la société (multi-company), réordonnancement par glisser-déposer.

---

### 1.2 Dashboard Principal

**Rôle :** Point d’entrée après onboarding (ou après Skip).

**Zone supérieure :**
- **Résumé des points** : à gauche **Total points earned**, à droite **Points to spend**.
- **Photo utilisateur** avec **anneau de progression** (cyan = points acquis vers le niveau suivant, blanc = restant). En dessous : **Level: #** (niveau actuel).
- Quand niveau atteignable : **LEVEL UP!** au-dessus de la photo, **CLICK TO LEVEL UP!** en dessous — clic sur la photo, le texte ou le bandeau pour level up.

**Zone statuts / actions :**
- Trois boutons / onglets sous l’avatar : **Referrals** (total), **Ongoing** (en cours), **Successful** (embauchés) — avec le nombre affiché au-dessus de chaque libellé.
- Boutons d’action : **View Jobs**, **Rewards**, **Email a friend** (en bas de l’écran).

**Règles d’affichage :**
- Level up ne coûte pas de points ; l’anneau se remet à zéro pour le prochain niveau.
- Au niveau max, l’anneau reste entièrement cyan, plus de « Level up ».

---

### 1.3 Écran Hired (Choix d’avatar)

**Rôle :** Affiché après **Start Now** si un ou plusieurs parrainés ont été embauchés depuis la dernière visite (ou avant toute visite).

**Caractéristiques :**
- Message : **(REFERRAL NAME) HAS BEEN HIRED! Choose an avatar for your new friend!**
- **Cinq avatars** en miniatures : ceux déjà assignés sont grisés, avec le nom du parrainé en dessous.
- Clic sur un avatar disponible → assignation. Si plusieurs embauchés, répétition pour chaque nom.
- À la fin des choix → chargement du **dashboard** ; les avatars « amis » apparaissent (survol = nom).

**Configuration (Admin) :** Referrals > Configuration > Friends — nom, image, miniature, position (Front/Back).

---

### 1.4 View Jobs (Postes ouverts)

**Accès :** Dashboard > **View Jobs**.

**Structure :**
- Une **carte par poste** (uniquement les postes **publiés**).
- Contenu carte : titre du poste, nombre d’**Open Positions** (Expected New Employees), **points** gagnés pour ce poste (même barème pour tous), description du poste (Job Summary).

**Actions par carte :**
- **Send Email** : popup template « Send Job Offer by Mail » — champ Email, Subject (défaut « Job for you »), Body avec lien de suivi vers la page du poste. **Send Mail** / Cancel.
- **Send SMS** : popup — Recipient (mobile), Body. **Send SMS** / Cancel (nécessite crédits IAP).
- **Send WhatsApp** : popup — Recipient, message. **Send WhatsApp** / Cancel (nécessite config WhatsApp).
- **Job Page** : ouvre la page web du poste dans un nouvel onglet (ce que voit le candidat). Sur la page : **Share Now** → Facebook / X (Twitter) / LinkedIn (ouverture nouvel onglet, lien de suivi pré-rempli ; l’utilisateur doit être connecté au réseau).

**Partage liste complète :** Dashboard > **Email a friend** — popup Send Job Offer by Mail, plusieurs destinataires (email séparés par virgule + espace), lien vers la liste de tous les postes ouverts.

---

### 1.5 My Referrals (Mes parrainages)

**Accès :** Dashboard > **Referrals** (ou Ongoing / Successful).

**Structure :**
- Une **carte par candidature** parrainée.
- **Badge** en haut à droite : **Hired** (blanc) ou **In Progress** (violet). Carte avec bande verte à gauche si Hired.
- Contenu carte : nom du candidat, sujet / titre de la candidature, poste, recruteur responsable, **points gagnés**.
- **Barre de progression** : points gagnés / total possible si embauché.
- **Liste des stages** du recrutement avec **points par stage** ; stage atteint = checkmark vert.

**Règles :**
- Referrals = tous (ongoing + successful).
- Ongoing = en pipeline (pas encore refusé ni embauché).
- Successful = embauchés.
- Pour les Hired, l’avatar choisi pour le « friend » peut apparaître sur la carte.

---

### 1.6 Rewards (Boutique récompenses)

**Accès :** Dashboard > **Rewards**.

**Structure :**
- Une **carte par récompense** (nom, description, photo, **coût en points** en haut à droite).
- Si l’utilisateur a assez de points : bouton **Buy** en bas de la carte.
- Si pas assez : texte **You need another (x) points to buy this** à la place du bouton.
- Clic **Buy** → popup de confirmation → **OK** / Cancel. Après OK : déduction des points, mise à jour de la liste.

**Configuration (Admin) :** Referrals > Configuration > Rewards — Product Name, Cost, Company (si multi), Gift Responsible, Photo, Description (obligatoire).

---

## 2. Configuration et Reporting (Administrator)

### 2.1 Configuration > Onboarding

- Liste des slides (texte, ordre). Clic sur une ligne → formulaire : **Text**, **Company** (optionnel, multi-company), **Image** (Edit / Clear). Réordonnancement par glisser-déposer (icône draggable).

### 2.2 Configuration > Levels

- Liste des niveaux (image, Level Name, points **Requirements** = total accumulé pour ce niveau). Formulaire : Level Name, Requirements, Image (Edit / Clear). Avertissement : image à fond transparent ; pas de retour arrière sans réinstaller le module.

### 2.3 Configuration > Friends

- Liste des avatars « amis » (Dashboard Image, Friend Name). Formulaire : Friend Name, Position (Front/Back), Thumbnail Image, Dashboard Image. Même avertissement sur les images.

### 2.4 Configuration > Rewards

- Création / édition des récompenses (Product Name, Cost, Company, Gift Responsible, Photo, onglet Description). Obligatoire : nom, description ; recommandé : coût et photo.

### 2.5 Configuration > Alerts

- Création d’alertes et personnalisation du dashboard (messages, fond d’écran).

### 2.6 Reporting

- **Referrals > Reporting** → rapport **Employees Referral Analysis**.
- Vue par défaut : **Bar Chart** empilée — axe y = nombre de parrainages, axe x = **Medium** (Email, LinkedIn, Facebook, etc.). Séries : Not Hired (refusés), In Progress, Hired. Filtre date par défaut : mois en cours.
- Survoler une barre = popover avec détail.
- **Vue Pivot** : changer filtres (ex. année), sélectionner/désélectionner les mesures (Earned Points, Employee Referral Refused, etc.) pour analyser par employé (nombre référés, nombre embauchés).
- **Insert in Spreadsheet** : insertion du tableau dans une feuille (Documents / Spreadsheet si installés).

---

## 3. Patterns de Navigation

- **Entrée** : App Referrals → Onboarding (si non terminé) ou Dashboard.
- **Dashboard** : hub vers View Jobs, Rewards, Email a friend, Referrals / Ongoing / Successful.
- **Sortie** : pas de tunnel obligatoire ; Skip et Start Now permettent de quitter l’onboarding.
- **Cohérence** : libellés et boutons alignés avec la doc (View Jobs, Rewards, Referrals, etc.) ; messages d’erreur ou d’info (pas assez de points, confirmation achat) en popup ou inline.

---

## 4. Points d’Attention pour Miyukini

- Garder une **hiérarchie visuelle** claire : points (total / à dépenser), niveau et anneau, puis actions (jobs, récompenses, parrainages).
- Prévoir **états vides** : aucun poste publié, aucun parrainage, aucune récompense configurée, pas assez de points pour aucune récompense.
- **Accessibilité** : contrastes, libellés des boutons et des liens (tracking), confirmation avant achat.
- **Responsive** : cartes et boutons utilisables sur mobile (partage, consultation des parrainages et récompenses).
- **Traçabilité** : lien de suivi dans tous les partages (email, SMS, WhatsApp, réseaux) pour associer la candidature au référent.

---

**Document** : Odoo Referrals — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
