# Miyukini Central — Spécification UI Dashboard

## Contexte

Ce document décrit de façon **exhaustive** l’interface utilisateur du Hub Miyukini Central après l’écran de chargement : structure du header, onglets, sidebar et body (dashboard). Il sert de référence unique pour l’implémentation et les évolutions UI.

**Référence conceptuelle :** [Miyukini Conceptual References - Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md).

## Portée / Scope

- **Applicable à :** Implémentation UI du Central (egui/eframe), design, recette.
- **Audience :** Développeurs, designers, QA.
- **Statut :** Spécification UI normative — source de vérité pour le rendu du Hub.

---

## 1. Flux général

1. **Chargement** : écran de chargement (titre, barre de progression, phrases alternées).
2. **Après chargement** : **Dashboard** avec **header sticky** en haut.
3. Le **header** reste collé en haut ; les **Services** n’utilisent que le **body**, sauf mode plein écran forcé d’un Service.

---

## 2. Header (sticky, 2 lignes, sans séparation visuelle)

Le header est composé de **2 lignes**, sans trait de séparation entre elles.

### 2.1 Ligne 1

| Zone | Contenu | Alignement |
|------|---------|-------------|
| **Gauche** | Bloc **titre + version** : « MIYUKINI COG » + « vers. 0.1.0 » (ou version courante). | Gauche de l’affichage. |
| **Droite** | Deux blocs côte à côte : | Droite de l’affichage. |

**Bloc de connexion :**

- **Connexion / Déconnexion** : selon l’état (connecté / non connecté).
- **Profil** : accès au profil utilisateur (overlay ou onglet).

**Bloc de configuration :**

- **Bouton avec une roue** (icône engrenage) : « Configuration » — ouvre les paramètres (overlay ou onglet).

Aucune ligne ou bordure entre la ligne 1 et la ligne 2.

### 2.2 Ligne 2 — Onglets

- **Rôle :** navigation de **service en service** (HUB + onglets des Services ouverts).
- **Position :** collés **en bas** au body : pas d’espace entre la barre d’onglets et le contenu du body (transition visuelle nulle).
- **Forme des onglets :**
  - **En bas :** carrés (bord droit angle vif).
  - **En haut :** coins arrondis, rayon **5 px**.
- **Espacement :** **12 px** entre chaque onglet.
- **Couleurs :**
  - **Onglet actif :** même couleur de fond que le **body** (fusion visuelle avec le contenu).
  - **Onglets non actifs :** gris (niveau « normal » inactif), **flottants** au-dessus de la barre d'onglets (léger décalage en bas, ligne de démarcation).
  - **Survol (hover) :** gris légèrement différent (effet de survol) — donc **2 niveaux de gris** pour les inactifs (normal + hover).
- **Effet flottant (onglets inactifs) :** les onglets inactifs ne touchent pas le bas de la barre ; un petit espace (2 px) laisse voir le fond de la barre ; une fine ligne horizontale sous l'onglet marque la démarcation. Aucune bordure verticale entre onglets.
- **Comportement :** clic = activation de l’onglet ; fermeture possible pour les onglets Services (pas pour HUB).

Le header est **collé en haut** de l’écran ; le body occupe tout l’espace restant sous la ligne 2.

---

## 3. Body (zone principale)

- **Usage par défaut :** **Dashboard** (vue HUB).
- **Usage avec onglet Service actif :** contenu du Service affiché dans le body (pas de deuxième fenêtre, sauf mode plein écran forcé par le Service).

---

## 4. Dashboard (vue HUB)

Le Dashboard est la vue affichée lorsque l’onglet **HUB** est actif. Il comporte une **sidebar** et une **zone body** côte à côte.

### 4.1 Sidebar (gauche)

Contenu, de haut en bas :

1. **Barre de recherche**  
   - Champ de recherche (nom ou description des Services).

2. **Filtrage**  
   - Filtres par catégorie (ou type) pour restreindre la liste des Services.

3. **Services favoris**  
   - Liste (ou section) des Services marqués comme favoris par l’utilisateur.

4. **Liste des Services**  
   - Tous les Services (ou ceux passant le filtre), affichés par **ordre alphabétique**.

La sidebar est fixe en largeur ; le contenu peut défiler si la liste est longue.

### 4.2 Body (zone droite du Dashboard)

Contenu affiché pour le **Service sélectionné** dans la sidebar :

| Élément | Description |
|--------|-------------|
| **Logo** | Logo ou icône du Service. |
| **Titre** | Nom du Service. |
| **Description complète** | Texte de description du Service (pas de troncature courte). |
| **Screenshots (mock)** | Zone d’images type « captures d’écran » (mock / placeholder si pas de vraies captures). |
| **Bouton Lancer** | Placé sous le titre (ou sous la description). Au clic : ouverture du Service (nouvel onglet ou remplacement selon règles de navigation). |

Si **aucun Service n’est sélectionné** : affichage d’un état vide (message du type « Sélectionnez un Service dans la liste » ou équivalent).

---

## 5. Résumé des constantes UI

| Élément | Valeur |
|--------|--------|
| Rayon des coins (haut des onglets) | 5 px |
| Espacement entre onglets | 12 px |
| Onglet actif | Fond = couleur du body |
| Onglets inactifs | 2 niveaux de gris (normal + hover) |
| Header | 2 lignes, sticky, sans séparation |

---

## 6. Références

- [Miyukini Conceptual References - Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md)
- [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

**Date de création :** 2026-02-02  
**Version :** 1.0  
**Statut :** Spécification UI normative — Miyukini Central Dashboard
