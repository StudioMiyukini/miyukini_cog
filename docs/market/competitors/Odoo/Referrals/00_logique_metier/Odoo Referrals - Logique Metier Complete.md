# Odoo Referrals — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Referrals** (Parrainage) d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle. Il identifie les modèles de données, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 — Applications HR / Referrals

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données (référents, points, récompenses, niveaux, onboarding)
- Règles métier et contraintes (points par stage, échange contre récompenses)
- Workflow parrainage (partage postes → candidature → progression → embauche → points)
- Gamification (niveaux, avatars, tableau des référents)
- Configuration (récompenses, slides onboarding, niveaux, amis/avatars, alertes)

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Référence à hr.applicant (Candidature — app Recruitment)

**Rôle :** L'app Referrals s'appuie sur les **candidatures** de l'app Recruitment. Le champ `referrer_id` (ou équivalent) sur `hr.applicant` lie une candidature à l'**employé référent**.

**Champs clés pour Referrals :**
- `referrer_id` : Employé ayant référé le candidat (Many2one hr.employee ou res.users)
- Lorsqu'une candidature progresse dans les stages (hr.recruitment.stage), les **points** sont attribués au référent selon la configuration des stages (Show in Referrals, Points par stage).

**Règles métier :**
- Seules les candidatures avec un référent (`referrer_id` renseigné) sont prises en compte dans Referrals.
- Les points sont crédités au référent à chaque passage du candidat dans un stage configuré (points par stage).
- Total par défaut pour un candidat embauché : 85 points (1 + 20 + 9 + 5 + 50).

---

### 1.2 Points et Stages (réutilisation hr.recruitment.stage)

**Rôle :** Les **stages** du recrutement (Recruitment) définissent les **points** attribués au référent quand un candidat parrainé atteint ce stage.

**Structure par défaut (documentation) :**

| Stage                 | Points |
|-----------------------|--------|
| Initial Qualification | 1     |
| First Interview       | 20    |
| Second Interview      | 9     |
| Contract Proposal     | 5     |
| Contract Signed       | 50    |
| **Total (embauché)**  | **85**|

**Règles métier :**
- Les points sont identiques pour tous les postes (pas de variation par poste).
- La configuration des points par stage se fait dans l'app **Recruitment** (Configuration > Stages), pas dans Referrals.
- Les stages « Show in Referrals » déterminent quels stages sont visibles et comptabilisés dans Referrals.
- Les points déjà gagnés ne sont pas retirés si le candidat est refusé ultérieurement (points acquis par stage atteint).

---

### 1.3 Récompenses (Rewards)

**Rôle :** Les **récompenses** sont les seuls éléments à configurer obligatoirement après installation de Referrals. Les employés échangent leurs **points disponibles** contre des récompenses.

**Champs typiques (inférés) :**
- `name` / Product Name : Nom affiché de la récompense (obligatoire)
- `cost` : Coût en points pour acheter la récompense (obligatoire ; 0 = affiché comme gratuit)
- `company_id` : Société (multi-company ; obligatoire en multi-société)
- `gift_responsible_id` : Responsable de la procuration et livraison de la récompense (personne alertée à l'achat)
- `photo` / image : Photo de la récompense (affichée dans la boutique)
- `description` : Description (onglet Description, visible sur la carte récompense, obligatoire)

**Règles métier :**
- Seuls les utilisateurs avec droits **Administrator** sur Recruitment peuvent créer ou modifier les récompenses.
- Un employé ne peut acheter une récompense que s'il a assez de **points disponibles** (Total points earned − points déjà dépensés).
- Après achat : déduction des points disponibles ; alerte au « Gift Responsible » pour livraison.
- En multi-société : une récompense peut être configurée par société (une entrée par société pour la même récompense logique).

---

### 1.4 Niveaux (Levels)

**Rôle :** **Gamification** — les employés « montent de niveau » selon le total de points accumulés (sur la durée), avec un avatar associé à chaque niveau.

**Champs typiques (inférés) :**
- `name` / Level Name : Nom ou numéro du niveau (affiché sous la photo utilisateur sur le dashboard)
- `points_required` / Requirements : Nombre total de **points accumulés** (sur la durée) pour atteindre ce niveau (pas un delta depuis le niveau précédent)
- `image` : Avatar du niveau (ex. superhéros Odoo, capes, boucliers)

**Règles métier :**
- Les niveaux n'ont **aucun impact fonctionnel** ; ils servent uniquement à la gamification et à l'engagement.
- Le niveau actuel est affiché en haut du dashboard Referrals (Level: #).
- Un anneau coloré autour de la photo utilisateur indique la progression vers le niveau suivant (cyan = points acquis, blanc = points restants).
- **Level up :** quand le total accumulé atteint le seuil, l'utilisateur clique pour « level up » ; l'avatar change, aucun point n'est déduit.
- Au niveau maximum configuré, l'utilisateur continue d'accumuler des points (échangeables contre récompenses) mais ne peut plus monter de niveau ; l'anneau reste entièrement cyan.

---

### 1.5 Amis / Avatars embauchés (Friends)

**Rôle :** Quand un **candidat parrainé est embauché**, le référent peut choisir un **avatar** pour le représenter sur son dashboard Referrals (« équipe de superhéros »).

**Champs typiques (inférés) :**
- `name` / Friend Name : Nom (pour différencier en configuration ; pas affiché ailleurs dans l'app)
- `position` : Front ou Back — position de l'avatar par rapport à l'avatar du référent
- `image` / Thumbnail : Miniature
- `dashboard_image` : Image affichée sur le dashboard

**Règles métier :**
- Après embauche d'un parrainé, au prochain accès à Referrals, l'écran « Hired » s'affiche : choix d'un avatar parmi une liste prédéfinie (5 par défaut). Les avatars déjà assignés sont grisés.
- Si plusieurs parrainés ont été embauchés depuis le dernier passage, le référent choisit un avatar pour chacun successivement.
- Seuls les **Administrators** (Recruitment) peuvent modifier les amis (noms, images, position). Modification d'image nécessite un fond transparent ; pas de retour à l'image d'origine sans réinstallation du module.

---

### 1.6 Onboarding (Slides)

**Rôle :** Script d'**onboarding** en 4 slides affiché à la première ouverture de l'app Referrals (ou à chaque ouverture tant que « Start Now » n'a pas été cliqué).

**Champs typiques (inférés) :**
- `sequence` : Ordre des slides (glisser-déposer dans la liste)
- `text` : Message de la slide
- `company_id` : Société (optionnel ; si renseigné, la slide n'est affichée que pour cette société)
- `image` : Image de la slide

**Messages par défaut (documentation) :**
1. OH NO! VILLAINS ARE LURKING THE CITY! HELP US RECRUIT A TEAM OF SUPERHEROES TO SAVE THE DAY!
2. BROWSE THROUGH OPEN JOB POSITIONS, PROMOTE THEM ON SOCIAL MEDIA, OR REFER FRIENDS.
3. COLLECT POINTS AND EXCHANGE THEM FOR AWESOME GIFTS IN THE SHOP.
4. COMPETE AGAINST YOUR COLLEAGUES TO BUILD THE BEST JUSTICE LEAGUE!

**Règles métier :**
- Les slides s'affichent à chaque ouverture de Referrals jusqu'à ce que l'utilisateur ait vu toutes les slides et cliqué sur **Start Now**. Si l'utilisateur quitte ou ne clique pas sur Start Now, les slides reprennent à la prochaine ouverture.
- **Skip** : ouvre directement le dashboard principal.
- Seuls les **Administrators** (Recruitment) peuvent modifier les slides (Referrals > Configuration > Onboarding).
- Si des parrainés ont été embauchés avant toute ouverture de Referrals, après **Start Now** l'écran « Hired » (choix d'avatar) s'affiche avant le dashboard.

---

### 1.7 Alertes (Alerts)

**Rôle (documentation) :** Les **alertes** permettent d'afficher des messages personnalisés sur le dashboard (ex. bannières, annonces). Configuration du fond d'écran et des alertes par les administrateurs.

**Règles métier :**
- Création et gestion des alertes réservées aux **Administrators** (Recruitment).
- Notifier les utilisateurs (affichage sur le dashboard Referrals).

---

## 2. Workflow Métier

### 2.1 Parcours typique du référent

1. **Accès Referrals** → Onboarding (4 slides) ou Skip → Dashboard.
2. **Dashboard** : Total points earned, Points to spend, niveau actuel, anneau de progression, boutons Referrals / Ongoing / Successful, View Jobs, Rewards, Email a friend.
3. **Partage de postes** : View Jobs → cartes postes (titre, open positions, points gagnables, description) → Send Email / Send SMS / Send WhatsApp (lien de suivi) ou Job Page → partage Facebook / X / LinkedIn. Ou depuis le dashboard : Email a friend (liste complète de postes avec lien de suivi).
4. **Candidat postule** via le lien de suivi → candidature créée avec `referrer_id` = employé courant.
5. **Progression en recrutement** : le recruteur fait avancer la candidature dans les stages → à chaque stage configuré « Show in Referrals » + points, les points sont crédités au référent.
6. **Mes références** : Referrals > Referrals (ou Ongoing / Successful) → cartes par candidature (nom, sujet, poste, recruteur, points gagnés, barre de progression, liste des stages avec checkmarks).
7. **Embauche** : quand la candidature passe au stage « Hired » (Contract Signed) → au prochain accès Referrals du référent, écran « Hired » → choix d'avatar pour le nouveau « ami ».
8. **Récompenses** : Rewards → liste des récompenses (coût en points, description, photo) → Buy si assez de points → confirmation → déduction des points ; responsable récompense alerté.
9. **Level up** : quand le total accumulé atteint le seuil du niveau suivant → affichage « LEVEL UP! » / « CLICK TO LEVEL UP! » → clic → avatar mis à jour, aucun coût en points.

### 2.2 Règles de calcul des points

- **Total points earned** : somme de tous les points gagnés sur la durée (tous parrainages, tous stages).
- **Points to spend** : Total points earned − points déjà dépensés en récompenses.
- Les points sont attribués **à l’atteinte du stage** (pas retirés en cas de refus ultérieur).
- Configuration des points par stage : **Recruitment** > Configuration > Stages (pas dans Referrals).

---

## 3. Droits et Accès

- **Referral User, Officer, Administrator** (droits sur l’app **Recruitment**) peuvent accéder à l’app Referrals.
- **Reporting** et **menus de configuration** (Onboarding, Levels, Friends, Rewards, Alerts) : **Administrator** (Recruitment) uniquement.

---

## 4. Dépendances Fonctionnelles

- **Employees** : obligatoire (référent = employé).
- **Recruitment** : obligatoire (candidatures, stages, points, champ referrer_id).
- **Website** : obligatoire (publication des postes, liens de suivi vers les pages postes).

---

## 5. Points d'Attention pour Miyukini

- **KindMother** : persistance des entités Referrals (récompenses, niveaux, onboarding, affectation points, achats).
- **StrongFather** : aucune décision métier forte dans Referrals (lecture/affichage et échange points contre récompenses) ; la décision d’embauche reste dans Recruitment.
- **Master Butler** : permissions (Referral User / Officer / Administrator) et visibilité (reporting, configuration).
- **Ever Buddy** : évolution des niveaux et des récompenses (dépréciation, compatibilité).
- **WorrySentinel** : données personnelles (nom référent, candidats parrainés) et niveau de sécurité adapté.
- Traçabilité : qui a parrainé qui, quand les points ont été crédités, quand une récompense a été achetée (audit).
- Intégration étroite avec l’équivalent **Recruitment** (MiyuRecruitment) : champ référent sur candidature, synchronisation des stages et des points.

---

**Document** : Odoo Referrals — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
