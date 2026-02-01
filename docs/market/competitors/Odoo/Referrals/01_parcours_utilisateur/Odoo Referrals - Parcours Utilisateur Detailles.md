# Odoo Referrals — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Referrals d'Odoo, identifiant les personas, scénarios d'usage, onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Applications HR / Referrals

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (Référent, Officer, Administrator)
- Parcours d'onboarding (slides, première utilisation)
- Scénarios d'usage (partage postes, suivi parrainages, récompenses, level up)
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Référent (Employé — Referral User)

**Profil :**
- Employé qui recommande des candidats pour des postes ouverts
- Consulte le dashboard Referrals pour voir ses points, ses parrainages en cours et embauchés
- Partage des offres d'emploi (email, SMS, WhatsApp, réseaux sociaux) via des liens de suivi
- Échange des points contre des récompenses
- « Monte en niveau » (avatar) selon les points accumulés
- Choisit un avatar pour chaque parrainé embauché (écran « Hired »)

**Permissions :**
- Accès à l'app Referrals (dashboard, View Jobs, My Referrals, Rewards)
- Pas d'accès au Reporting ni aux menus Configuration (Onboarding, Levels, Friends, Rewards, Alerts)

### 1.2 Officer (Recruitment)

**Profil :**
- Utilisateur Recruitment avec droits Officer
- Peut accéder à Referrals comme un référent (dashboard, partage, récompenses)
- Pas d'accès au Reporting ni à la configuration Referrals

**Permissions :**
- Identiques au Referral User pour Referrals

### 1.3 Administrator (Recruitment)

**Profil :**
- Configure l'app Referrals : récompenses, slides onboarding, niveaux, amis (avatars), alertes
- Accède au **Reporting** (Employees Referral Analysis : sources, embauchés, refusés, en cours par canal)
- Modifie les slides d'onboarding, les niveaux, les avatars « amis », le fond et les alertes du dashboard

**Permissions :**
- Tout ce que Referral User / Officer peuvent faire
- Referrals > Configuration > Onboarding, Levels, Friends, Rewards, Alerts
- Referrals > Reporting

---

## 2. Parcours d'Onboarding

### 2.1 Première ouverture (Référent)

**Étapes :**

1. Ouverture de l'app **Referrals** pour la première fois.
2. Affichage du **message principal** en haut : « GATHER YOUR TEAM! Job Referral Program » + image + texte.
3. **4 slides** d'onboarding (texte par défaut sur les superhéros, postes, points, récompenses, compétition).
4. Boutons **Next** (slide suivante), **Skip** (accès direct au dashboard), **Start Now** (à la fin des 4 slides).
5. Si l'utilisateur ne clique pas sur **Start Now** et quitte, les slides réapparaissent à la prochaine ouverture.
6. Après **Start Now** : si un ou plusieurs parrainés ont été embauchés avant toute visite Referrals → écran **Hired** (choix d'avatar pour chaque embauché) → puis dashboard. Sinon → dashboard principal.

**Durée estimée :** 1 à 2 minutes.

**Points de friction identifiés :**
- Slides obligatoires jusqu'à « Start Now » peuvent sembler longues pour les utilisateurs pressés (Skip permet de contourner).
- Pas de personnalisation du contenu des slides sans droits Admin.

### 2.2 Configuration initiale (Administrator)

**Étapes :**

1. Vérifier que **Employees**, **Recruitment** et **Website** sont installés.
2. Installer l'app **Referrals**.
3. **Referrals > Configuration > Rewards** : créer au moins une récompense (nom, coût en points, description, photo, responsable livraison). C'est la seule configuration obligatoire documentée.
4. Optionnel : **Onboarding** — modifier textes/images/séquences des slides, société si multi-company.
5. Optionnel : **Levels** — modifier noms, points requis, images des niveaux.
6. Optionnel : **Friends** — modifier noms, images, position (Front/Back) des avatars pour les parrainés embauchés.
7. Optionnel : **Alerts** — créer des alertes et personnaliser le dashboard.

**Durée estimée :** 30 min à 1 h pour une configuration de base (récompenses + optionnel).

---

## 3. Scénarios d'Usage Principaux

### 3.1 Partager des postes (Référent)

1. Ouvrir **Referrals** → Dashboard.
2. Cliquer **View Jobs** → liste de cartes (postes publiés uniquement).
3. Pour un poste : **Send Email** / **Send SMS** / **Send WhatsApp** (saisir destinataire, lien de suivi pré-rempli) ou **Job Page** (ouvrir la page web du poste) puis **Share Now** (Facebook, X, LinkedIn).
4. Ou depuis le dashboard : **Email a friend** → envoyer la liste complète des postes avec lien de suivi (plusieurs destinataires possibles, séparés par virgule + espace).
5. Le candidat qui postule via le lien est enregistré avec le référent courant ; les points seront crédités au référent à chaque progression de stage.

**Succès :** lien envoyé, candidature future liée au référent.

### 3.2 Consulter ses parrainages et ses points (Référent)

1. Dashboard : en haut, **Total points earned** et **Points to spend**.
2. Sous l'avatar : **Referrals**, **Ongoing**, **Successful** (nombre de parrainages total, en cours, embauchés).
3. Cliquer **Referrals** (ou Ongoing / Successful) → **My Referrals** : cartes par candidature (nom, sujet, poste, recruteur, points gagnés, barre de progression, stages avec checkmarks). Badge **Hired** (blanc) ou **In Progress** (violet).
4. Vérifier la progression des points par stage sur chaque carte.

**Succès :** visibilité claire sur l’état des parrainages et les points.

### 3.3 Échanger des points contre une récompense (Référent)

1. Dashboard → **Rewards**.
2. Parcourir les cartes récompenses (nom, coût, description, photo).
3. Si assez de points : bouton **Buy** ; sinon message « You need another (x) points to buy this ».
4. Cliquer **Buy** → fenêtre de confirmation → **OK** → points déduits, responsable récompense alerté.

**Succès :** achat enregistré, points mis à jour, traçabilité.

### 3.4 Level up (Référent)

1. Quand le total de points accumulés atteint le seuil du niveau suivant : anneau autour de la photo entièrement cyan, affichage **LEVEL UP!** et **CLICK TO LEVEL UP!**.
2. Cliquer sur la photo, le texte ou le bandeau → niveau mis à jour, nouvel avatar, anneau réinitialisé pour le prochain niveau.
3. Aucun point n’est déduit.

**Succès :** sentiment de progression et gamification.

### 3.5 Choisir un avatar pour un parrainé embauché (Référent)

1. Après embauche d’un parrainé, à la prochaine ouverture de Referrals (ou après Start Now) → écran **Hired** : « (REFERRAL NAME) HAS BEEN HIRED! Choose an avatar for your new friend! ».
2. Cinq avatars proposés ; ceux déjà utilisés sont grisés avec le nom en dessous.
3. Cliquer sur un avatar disponible → si plusieurs embauchés, répéter pour le suivant → puis chargement du dashboard avec les avatars visibles (survol = nom).

**Succès :** personnification de l’équipe « superhéros » sur le dashboard.

### 3.6 Consulter le reporting (Administrator)

1. **Referrals > Reporting** → rapport **Employees Referral Analysis** (vue Bar Chart empilée par défaut).
2. Axes : nombre de parrainages (y), canal / medium (x) — Email, LinkedIn, Facebook, etc. Étapes : Not Hired (refusés), In Progress, Hired. Filtre par défaut : mois en cours.
3. Survoler une barre pour détails.
4. Option **Pivot** : retirer le filtre date, sélectionner l’année, masquer certaines mesures (ex. Earned Points, Refused) → voir par employé : total candidats référés et nombre embauchés (ex. « qui sont les meilleurs référents cette année »).
5. Option **Insert in Spreadsheet** (si app Documents/Spreadsheet) : insérer le tableau dans une feuille.

**Succès :** analyse des canaux et des référents les plus performants.

---

## 4. Points de Friction Identifiés

- **Onboarding** : obligation de cliquer « Start Now » pour ne plus revoir les slides ; sortie sans clic = slides à revoir.
- **Images** : modification des images (niveaux, amis) nécessite fond transparent ; pas de retour à l’original sans réinstallation du module.
- **SMS / WhatsApp** : envoi SMS soumis aux crédits IAP ; WhatsApp nécessite configuration Odoo.
- **Reporting** : réservé aux Administrators ; pas de vue synthétique pour les référents (ex. classement anonyme).
- **Multi-société** : récompenses à dupliquer par société pour une même offre logique.

---

## 5. Recommandations pour Miyukini

- Proposer un **onboarding** court et optionnel (skip ou « ne plus afficher » explicite).
- Séparer clairement **parcours référent** (dashboard, partage, récompenses, level up) et **parcours administrateur** (configuration, reporting).
- Garder la **traçabilité** des points (crédits, dépenses) et des achats de récompenses pour audit.
- Intégrer avec l’équivalent **Recruitment** (référent sur candidature, synchronisation des stages et points) sans dupliquer la logique de recrutement.
- Prévoir des **récompenses** configurables (coût, responsable, description) et un flux de notification au responsable à l’achat.
- En reporting : permettre filtres par période, par canal, par référent (admin) et, si besoin, vues dérivées pour tableaux de bord ou export.

---

**Document** : Odoo Referrals — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
