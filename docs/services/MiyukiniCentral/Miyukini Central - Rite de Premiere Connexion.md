# Miyukini Central — Rite de première connexion

Documentation du parcours de création du premier compte (COG vierge). Miou accompagne l'utilisateur à la voix et au texte.

---

## 1. Vue d'ensemble du parcours

Le Rite d'Entrée se compose de **quatre phases** :

| Phase | Nom | Description |
|-------|-----|-------------|
| 1 | **Nom** | Miou demande le nom ou pseudonyme |
| 2 | **Email** | Adresse e-mail pour le compte |
| 3 | **Clé** | Mot de passe (avec règles de complexité) |
| 4 | **Infos complémentaires** | Miou pose quelques questions pour mieux connaître l'utilisateur |

Chaque question ou catégorie de l'étape 4 dispose d'une **option « Passer »** — l'utilisateur n'est jamais obligé de répondre.

---

## 2. Étapes 1 à 3 (existant)

### 2.1 Étape Nom

- **Voix Miou :** `login_new_ask_name.mp3`
- **Texte :** « Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ? »
- **Saisie :** Nom ou pseudo (texte libre)
- **Validation :** Non bloquante — on peut continuer avec une chaîne vide (le pseudonyme sera utilisé)

### 2.2 Étape Email

- **Voix Miou :** `login_new_ask_email.mp3`
- **Texte :** « Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ? »
- **Saisie :** Adresse e-mail (requise pour la création du compte)

### 2.3 Étape Clé (mot de passe)

- **Voix Miou :** `login_new_ask_password.mp3`
- **Texte :** « Pour finir, peux-tu me donner une clé pour protéger l'entrée ? Suis les instructions ci-dessous. »
- **Règles :** Longueur min 8 caractères, une majuscule, une minuscule, un chiffre, un symbole
- **Action :** Création du compte (`sign_up`). Si succès → passage à l'étape 4.

---

## 3. Étape 4 — Infos complémentaires (Miou te connaît)

Après la création du compte, Miou propose **en douceur** de remplir quelques informations pour personnaliser son accompagnement. **Tout est facultatif** : l'utilisateur peut passer une catégorie entière ou chaque question individuellement.

### 3.1 Organisation par catégorie

Les questions sont regroupées en **catégories**. Chaque catégorie peut être :
- **Répondue** (au moins partiellement)
- **Passée** (bouton « Passer » ou « Ne pas répondre »)

### 3.2 Catégories du Rite (écran première connexion)

#### Catégorie A — Identité

| Question | Type | Stockage | Obligatoire |
|----------|------|----------|-------------|
| Comment te sens-tu le mieux défini ? | Masculin / Féminin / Neutre / Passer | `genre` | Non |
| Quelle est ta date de naissance ? | Date (JJ/MM/AAAA) ou Passer | `date_naissance` | Non |

#### Catégorie B — Localisation

| Question | Type | Stockage | Obligatoire |
|----------|------|----------|-------------|
| Où habites-tu ? (ville) | Texte libre ou Passer | `ville` | Non |
| Adresse postale complète | Numéro + rue, code postal, ville (champs séparés) ou Passer | `numero_voie`, `rue`, `code_postal` | Non |

#### Catégorie C — Situation personnelle

| Question | Type | Stockage | Obligatoire |
|----------|------|----------|-------------|
| Quel est ton statut relationnel ? | Célibataire / En couple / Marié(e) / Passer | `statut_marital` | Non |
| Si non célibataire : genre et prénom de ton/sa partenaire ? | Texte libre (genre + prénom) ou Passer | `partenaire_genre`, `partenaire_nom` | Non |
| As-tu des enfants ? | Oui / Non / Passer | `enfants_present` | Non |
| Si oui : combien ? | Nombre (1–20) ou Passer | `enfants_nombre` | Non |
| Si oui : leurs prénoms ? | Texte libre ou Passer | `enfants_noms` | Non |

#### Catégorie D — Contexte de vie (enrichissement)

| Question | Type | Stockage | Obligatoire |
|----------|------|----------|-------------|
| Que fais-tu dans la vie ? (profession, études) | Texte libre ou Passer | `profession` | Non |
| Quelle est ta langue maternelle ? | Liste (Français, Anglais, etc.) + Autre ou Passer | `langue_maternelle` | Non |
| Tu préfères le matin ou le soir pour mes rappels ? | Matin / Soir / Peu importe / Passer | `preference_rappel` (Miou) | Non |

### 3.3 Interface utilisateur

- Une **catégorie à la fois** (affichage progressif, scroll si besoin)
- Pour chaque question : champ de saisie adapté + **bouton « Passer »**
- En bas de catégorie : **« Passer la catégorie »** — saute toutes les questions restantes de la catégorie
- En bas de l'écran : **« Terminer et entrer »** — valide les réponses saisies et entre dans Central

### 3.4 Ton de Miou

« Pour que je te connaisse un peu mieux, peux-tu me répondre à quelques questions ? Tu peux passer celles que tu ne veux pas partager. »

---

## 4. Questions réservées (hors Rite — Miou demandera plus tard)

Certaines questions sont **trop intimes** ou **trop détaillées** pour l'écran du Rite. Miou les posera **progressivement** selon le **palier d'attachement** (Connaissance → Pote → Amie → etc.). Voir [Bot - Registre Questions et Paliers d'Attachement](./Miou/Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md).

### 4.1 Exemples de questions « plus tard »

| Palier | Exemple |
|--------|---------|
| Connaissance (1) | Préférence ton (discrète / bavarde), contexte activité |
| Pote (2) | Hobbies, chronotype, activité déconnexion |
| Amie (3) | Ce qui fait du bien, besoin de présence |
| Amie proche (4) | Projet cœur, style d'accompagnement |
| Meilleure amie (5) | Rêves, peurs légères, soutien préféré |
| Grande sœur (6) | Moments où être plus présente, réassurance |

### 4.2 Sujets exclus du Rite (trop intimes)

- Détails sur la relation actuelle (depuis quand, comment ça va)
- Santé (physique ou mentale)
- Situations familiales complexes (divorce, garde, etc.)
- Préférences affectives détaillées
- Revenus, situation financière
- Convictions politiques ou religieuses

**Principe :** Le Rite collecte des **infos de base** pour un premier contact personnalisé. Le reste se construit dans la durée, par la relation.

---

## 5. Stockage des données

| Champ | Table / base | Chiffrement |
|-------|--------------|-------------|
| `genre`, `date_naissance`, `ville`, `numero_voie`, `rue`, `code_postal` | `central_profiles` | Selon config DB (SQLCipher si activé) |
| `statut_marital`, `partenaire_genre`, `partenaire_nom` | `central_profiles` | Idem |
| `enfants_nombre`, `enfants_noms` | `central_profiles` | Idem |
| `profession`, `langue_maternelle` | `central_profiles` | Idem |
| `preference_rappel` | Base Miou (réponses utilisateur) | Chiffré |

---

## 6. Références

- [Miou - Document Fondateur](./Miou/Miou%20-%20Document%20Fondateur.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Miou/Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Miou/Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
