# Miou — Catalogue Exhaustif des Connaissances

Ce document recense **toutes les connaissances** que Miou peut collecter sur l'utilisateur : clés de données, sources, format, usage et regroupement pour l'affichage dans « Ce que Miou sait de moi ».

---

## 1. Contexte

L'utilisateur peut **voir, modifier et supprimer** chaque donnée de la base de connaissance de Miou. La transparence est totale. Ce catalogue sert de référence pour :

- L'interface de gestion des connaissances (affichage par catégorie)
- L'enrichissement via chatbot et bulles
- Le mapping vers les adaptations Central (thème, fréquence, orientation)
- La maintenance et l'extension du registre

---

## 2. Sources des connaissances

| Source | Description | Exemple |
|--------|-------------|---------|
| **Bulles Miou** | Réponses aux questions de curiosité (Registre par palier) | `reconfort`, `hobby` |
| **Chatbot** | Discussion libre dans l'onglet Miou — extraction après confirmation | Toute donnée partagée et validée |
| **Rite d'Entrée** | Données du profil Central (pseudo, préférences globales) | `pseudo`, `theme_prefere`, `date_naissance` |
| **Paramètres Miou** | Réglages explicites (discret/bavard, rappels) | `preference_ton`, `preference_rappel` |

**Invariant :** Aucune donnée n'est collectée sans action explicite de l'utilisateur (réponse, saisie, réglage).

---

## 3. Catégories d'affichage

Les données sont regroupées dans l'interface « Ce que Miou sait de moi » par **catégorie** :

| Catégorie | Clé catégorie | Description |
|-----------|---------------|-------------|
| **Identité et profil** | `identite` | Pseudo, âge, contexte d'activité |
| **Préférences pratiques** | `preferences_pratiques` | Rappels, ton, moment préféré |
| **Loisirs et habitudes** | `loisirs` | Hobbies, lecture, déconnexion, chronotype |
| **Émotions et bien-être** | `emotions` | Reconfort, présence, lieux, motivation |
| **Projets et valeurs** | `projets` | Projet cœur, valeur, fierté, bonne journée |
| **Soutien et présence** | `soutien` | Rêve, peurs, bonheur quotidien, façon d'être soutenu |
| **Préférences d'accompagnement** | `accompagnement` | Style conseil, pousser ou laisser, réassurance |
| **Données Rite d'Entrée** | `rite_entree` | Profil Central, thème, date naissance (si partagé) |
| **Humeur et préférences d'ambiance** | `humeur` | Humeur préférée, thème ambiance (pour adaptation Central) |

---

## 4. Catalogue détaillé par clé

### 4.1 Identité et profil (`identite`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `pseudo` | string | Rite d'Entrée / Central | 0 | Nom d'affichage de l'utilisateur |
| `date_naissance` | date (optionnel) | Chatbot / Paramètres | 2 | Année de naissance ou date — pour références culturelles par génération |
| `contexte_activite` | enum | q1_3 (bulle) | 1 | Bureau / Maison / Les deux |
| `annee_generation` | int (dérivé) | date_naissance | 2 | Décennie (1990, 2000, 2010…) — calculé pour filtrage culture |

---

### 4.2 Préférences pratiques (`preferences_pratiques`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `preference_rappel` | enum | q1_1 (bulle) | 1 | Matin / Soir / Peu importe |
| `preference_ton` | enum | q1_2 (bulle) / Paramètres | 1 | Discrète / Bavarde / Comme maintenant |
| `moment_prefere` | enum | q1_4 (bulle) | 1 | Matin / Après-midi / Soir |
| `rythme_prefere` | enum | q2_3 (bulle) | 2 | Chargées / Tranquilles / Ça dépend |
| `preference_surprise` | enum | q3_5 (bulle) | 3 | Surprises / Prévenu / Les deux |
| `frequence_bulles` | enum | Paramètres / adaptation | 1 | Discret / Normal / Bavard — peut être dérivé de `preference_ton` |

---

### 4.3 Loisirs et habitudes (`loisirs`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `loisir_lecture` | texte | q2_1 (bulle) / Chatbot | 2 | Genres préférés, auteurs |
| `hobby` | texte | q2_2 (bulle) / Chatbot | 2 | Activité préférée |
| `chronotype` | enum | q2_4 (bulle) | 2 | Matin / Soir / Les deux |
| `activite_deconnexion` | texte | q2_5 (bulle) / Chatbot | 2 | Sport, musique, activité qui déconnecte |
| `lieu_ressource` | texte | q3_3 (bulle) / Chatbot | 3 | Endroit favori (réel ou virtuel) |

---

### 4.4 Émotions et bien-être (`emotions`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `reconfort` | texte | q3_1 (bulle) / Chatbot | 3 | Ce qui fait du bien après une journée difficile |
| `besoin_presence` | enum | q3_2 (bulle) | 3 | Espace / Présente / Les deux selon le jour |
| `motivation_actuelle` | texte | q3_4 (bulle) / Chatbot | 3 | Ce qui motive en ce moment |
| `bonheur_quotidien` | texte | q5_5 (bulle) / Chatbot | 5 | Petit bonheur du quotidien |
| `reassurance` | texte | q6_3 (bulle) / Chatbot | 6 | Ce qui rassure |

---

### 4.5 Projets et valeurs (`projets`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `valeur_actuelle` | texte | q4_1 (bulle) / Chatbot | 4 | Ce qui compte le plus en ce moment |
| `projet_coeur` | texte | q4_2 (bulle) / Chatbot | 4 | Projet qui tient à cœur |
| `source_fierte` | texte | q4_3 (bulle) / Chatbot | 4 | Ce qui rend fier/fière |
| `bonne_journee` | texte | q4_5 (bulle) / Chatbot | 4 | Définition d'une bonne journée |
| `reve` | texte | q5_1 (bulle) / Chatbot | 5 | Rêve à réaliser |
| `inspiration` | texte | q5_4 (bulle) / Chatbot | 5 | Personne ou chose qui inspire |

---

### 4.6 Soutien et présence (`soutien`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `soutien_prefere` | texte | q5_2 (bulle) / Chatbot | 5 | Façon préférée d'être soutenu |
| `peur_legere` | texte | q5_3 (bulle) / Chatbot | 5 | Petites peurs (non intime) |
| `moment_plus_presente` | texte | q6_1 (bulle) / Chatbot | 6 | Moments où Miou devrait être plus présente |
| `aide_soin` | texte | q6_4 (bulle) / Chatbot | 6 | Comment aider à prendre soin de soi |
| `rappel_personnalise` | texte | q6_5 (bulle) / Chatbot | 6 | Rappel spécifique demandé |

---

### 4.7 Préférences d'accompagnement (`accompagnement`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `style_accompagnement` | enum | q4_4 (bulle) | 4 | Pousser / Mon rythme / Les deux |
| `style_conseil` | enum | q6_2 (bulle) | 6 | Direct / Questions / Les deux |

---

### 4.8 Données Rite d'Entrée (`rite_entree`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `theme_central` | enum | Rite d'Entrée / Paramètres Central | 0 | Gaming, Minimal, etc. — thème visuel Central |
| `services_actifs` | liste | Central | 0 | Services installés (JayKoa, JayXpose…) |
| `frequence_notifications` | enum | Central | 0 | Préférence globale notifications |

**Note :** Ces données sont gérées par Central. Miou les lit en lecture seule pour adapter son contexte.

---

### 4.9 Humeur et préférences d'ambiance (`humeur`)

| Clé | Type | Source | Palier min | Description |
|-----|------|--------|------------|-------------|
| `humeur_actuelle` | enum/texte | Chatbot / Bulle courte | 3 | État actuel (optionnel, déclaré par l'utilisateur) |
| `humeur_preferee` | texte | Chatbot / q3_4 | 3 | Humeur préférée pour les échanges |
| `theme_ambiance` | enum | Chatbot / Paramètres | 2 | Gaming, Zen, Productif, Social — orientation des actions Miou |
| `orientation_actions` | enum | Chatbot / adaptation | 3 | Pauses / Rappels / Curiosité / Soutien — ce que Miou privilégie |

**Usage :** Ces clés servent à **adapter Central** (thème, fréquence, orientation). Voir [Miou - Préférences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md).

---

### 4.10 Données extraites par le chatbot (libres)

Le chatbot peut extraire des données **non prédéfinies** si l'utilisateur partage et confirme. Format :

| Clé | Convention | Exemple |
|-----|------------|---------|
| `chatbot_<theme>` | Préfixe `chatbot_` + thème court | `chatbot_serie_preferee`, `chatbot_plat_prefere` |
| Stockage | Texte libre, catégorie déduite | Miou propose une catégorie ou « Autre » |

**Affichage :** Ces entrées apparaissent dans « Ce que Miou sait de moi » sous une sous-catégorie « Infos partagées (chatbot) » ou rattachées à la catégorie la plus proche.

---

## 5. Schéma de stockage (résumé)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | UUID | Identifiant unique |
| `profile_id` | string | Profil COG |
| `key` | string | Clé (ex. `reconfort`, `hobby`) |
| `category` | string | Catégorie d'affichage |
| `value` | string (chiffré) | Valeur stockée |
| `source` | enum | `bulle`, `chatbot`, `parametres`, `rite_entree` |
| `question_id` | string (optionnel) | Ex. `q3_1` si source bulle |
| `created_at` | DateTime | Date d'enregistrement |
| `updated_at` | DateTime | Dernière modification |

---

## 6. Mapping vers les adaptations Central

| Clé(s) | Adaptation Central |
|--------|-------------------|
| `preference_ton`, `frequence_bulles` | Fréquence des bulles (Discret / Normal / Bavard) |
| `theme_ambiance`, `theme_central` | Thème visuel ou orientation (Gaming, Zen…) |
| `preference_rappel`, `moment_prefere` | Créneaux horaires des rappels |
| `orientation_actions`, `style_accompagnement` | Types d'actions privilégiées (pauses, curiosité, soutien) |
| `date_naissance` (dérivé) | Filtrage des références culture pop par génération |

---

## 7. Références

- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Miou - Préférences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)

---

**Version :** 1.0  
**Statut :** Catalogue exhaustif des connaissances Miou
