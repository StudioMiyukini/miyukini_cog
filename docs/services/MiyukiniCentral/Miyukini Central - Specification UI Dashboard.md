# Miyukini Central â€” SpÃ©cification UI Dashboard

## Contexte

Ce document dÃ©crit de faÃ§on **exhaustive** lâ€™interface utilisateur du Hub Miyukini Central aprÃ¨s lâ€™Ã©cran de chargement : structure du header, onglets, sidebar et body (dashboard). Il sert de rÃ©fÃ©rence unique pour lâ€™implÃ©mentation et les Ã©volutions UI.

**RÃ©fÃ©rence conceptuelle :** [Miyukini Conceptual References - Miyukini Central Hub Services](..//..//_index.md).

## PortÃ©e / Scope

- **Applicable Ã  :** ImplÃ©mentation UI du Central (egui/eframe), design, recette.
- **Audience :** DÃ©veloppeurs, designers, QA.
- **Statut :** SpÃ©cification UI normative â€” source de vÃ©ritÃ© pour le rendu du Hub.

---

## 1. Flux gÃ©nÃ©ral

1. **Chargement** : Ã©cran de chargement (titre, barre de progression, phrases alternÃ©es).
2. **AprÃ¨s chargement** : **Dashboard** avec **header sticky** en haut.
3. Le **header** reste collÃ© en haut ; les **Services** nâ€™utilisent que le **body**, sauf mode plein Ã©cran forcÃ© dâ€™un Service.

---

## 2. Header (sticky, 2 lignes, sans sÃ©paration visuelle)

Le header est composÃ© de **2 lignes**, sans trait de sÃ©paration entre elles.

### 2.1 Ligne 1

| Zone | Contenu | Alignement |
|------|---------|-------------|
| **Gauche** | Bloc **titre + version** : Â« MIYUKINI COG Â» + Â« vers. 0.1.0 Â» (ou version courante). | Gauche de lâ€™affichage. |
| **Droite** | Deux blocs cÃ´te Ã  cÃ´te : | Droite de lâ€™affichage. |

**Bloc de connexion :**

- **Connexion / DÃ©connexion** : selon lâ€™Ã©tat (connectÃ© / non connectÃ©).
- **Profil** : accÃ¨s au profil utilisateur (overlay ou onglet).

**Bloc de configuration :**

- **Bouton avec une roue** (icÃ´ne engrenage) : Â« Configuration Â» â€” ouvre les paramÃ¨tres (overlay ou onglet).

Aucune ligne ou bordure entre la ligne 1 et la ligne 2.

### 2.2 Ligne 2 â€” Onglets

- **RÃ´le :** navigation de **service en service** (HUB + onglets des Services ouverts).
- **Position :** collÃ©s **en bas** au body : pas dâ€™espace entre la barre dâ€™onglets et le contenu du body (transition visuelle nulle).
- **Forme des onglets :**
  - **En bas :** carrÃ©s (bord droit angle vif).
  - **En haut :** coins arrondis, rayon **5 px**.
- **Espacement :** **12 px** entre chaque onglet.
- **Couleurs :**
  - **Onglet actif :** mÃªme couleur de fond que le **body** (fusion visuelle avec le contenu).
  - **Onglets non actifs :** gris (niveau Â« normal Â» inactif), **flottants** au-dessus de la barre d'onglets (lÃ©ger dÃ©calage en bas, ligne de dÃ©marcation).
  - **Survol (hover) :** gris lÃ©gÃ¨rement diffÃ©rent (effet de survol) â€” donc **2 niveaux de gris** pour les inactifs (normal + hover).
- **Effet flottant (onglets inactifs) :** les onglets inactifs ne touchent pas le bas de la barre ; un petit espace (2 px) laisse voir le fond de la barre ; une fine ligne horizontale sous l'onglet marque la dÃ©marcation. Aucune bordure verticale entre onglets.
- **Comportement :** clic = activation de lâ€™onglet ; fermeture possible pour les onglets Services (pas pour HUB).

Le header est **collÃ© en haut** de lâ€™Ã©cran ; le body occupe tout lâ€™espace restant sous la ligne 2.

---

## 3. Body (zone principale)

- **Usage par dÃ©faut :** **Dashboard** (vue HUB).
- **Usage avec onglet Service actif :** contenu du Service affichÃ© dans le body (pas de deuxiÃ¨me fenÃªtre, sauf mode plein Ã©cran forcÃ© par le Service).

---

## 4. Dashboard (vue HUB)

Le Dashboard est la vue affichÃ©e lorsque lâ€™onglet **HUB** est actif. Il comporte une **sidebar** et une **zone body** cÃ´te Ã  cÃ´te.

### 4.1 Sidebar (gauche)

Contenu, de haut en bas :

1. **Barre de recherche**  
   - Champ de recherche (nom ou description des Services).

2. **Filtrage**  
   - Filtres par catÃ©gorie (ou type) pour restreindre la liste des Services.

3. **Services favoris**  
   - Liste (ou section) des Services marquÃ©s comme favoris par lâ€™utilisateur.

4. **Liste des Services**  
   - Tous les Services (ou ceux passant le filtre), affichÃ©s par **ordre alphabÃ©tique**.

La sidebar est fixe en largeur ; le contenu peut dÃ©filer si la liste est longue.

### 4.2 Body (zone droite du Dashboard)

Contenu affichÃ© pour le **Service sÃ©lectionnÃ©** dans la sidebar :

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **Logo** | Logo ou icÃ´ne du Service. |
| **Titre** | Nom du Service. |
| **Description complÃ¨te** | Texte de description du Service (pas de troncature courte). |
| **Screenshots (mock)** | Zone dâ€™images type Â« captures dâ€™Ã©cran Â» (mock / placeholder si pas de vraies captures). |
| **Bouton Lancer** | PlacÃ© sous le titre (ou sous la description). Au clic : ouverture du Service (nouvel onglet ou remplacement selon rÃ¨gles de navigation). |

Si **aucun Service nâ€™est sÃ©lectionnÃ©** : affichage dâ€™un Ã©tat vide (message du type Â« SÃ©lectionnez un Service dans la liste Â» ou Ã©quivalent).

---

## 5. RÃ©sumÃ© des constantes UI

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| Rayon des coins (haut des onglets) | 5 px |
| Espacement entre onglets | 12 px |
| Onglet actif | Fond = couleur du body |
| Onglets inactifs | 2 niveaux de gris (normal + hover) |
| Header | 2 lignes, sticky, sans sÃ©paration |

---

## 6. RÃ©fÃ©rences

- [Miyukini Conceptual References - Miyukini Central Hub Services](..//..//_index.md)
- [Miyukini Conceptual References - Glossaire](..//..//_index.md)

---

**Date de crÃ©ation :** 2026-02-02  
**Version :** 1.0  
**Statut :** SpÃ©cification UI normative â€” Miyukini Central Dashboard

