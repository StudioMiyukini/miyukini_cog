# Miyukini Central — Écrans et UI

## Contexte

Ce document décrit en détail tous les **écrans** (vues) du Hub Miyukini Central, leur structure, leur contenu et leurs interactions. Il complète les documents de parcours utilisateurs et de UI/UX Header en fournissant une vue exhaustive de l'interface.

## Portée / Scope

- **Périmètre :** Description détaillée de tous les écrans du Hub ; structure, composants, interactions.
- **Hors périmètre :** Implémentation technique détaillée (voir Stack UI egui/eframe) ; contenu des Services eux-mêmes.

---

## 1. Structure générale de l'interface

### 1.1 Layout principal

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ HEADER (toujours visible sauf plein écran)                                  │
│ [Logo] [Menu Services] [Onglets] [Profil] [Config]                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CONTENU PRINCIPAL (selon onglet actif)                                    │
│                                                                             │
│  - Onglet HUB : Accueil, Catalogue, Mes Services, Paramètres               │
│  - Onglet Service : Interface du Service ouvert                            │
│  - Onglet Profile : Profil utilisateur, paramètres                         │
│                                                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Zones de l'interface

| Zone | Description | Visibilité |
|------|-------------|------------|
| **Header** | Navigation principale, onglets, profil | Toujours visible (sauf plein écran) |
| **Contenu principal** | Contenu de l'onglet actif | Variable selon onglet |
| **Fenêtres modales** | Dialogs, confirmations, erreurs | Sur demande |

---

## 2. Onglet HUB

L'onglet **HUB** est l'onglet système toujours présent. Il contient plusieurs vues internes.

### 2.1 Vue Accueil

**Objectif :** Point d'entrée principal, vue d'ensemble.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet HUB - Vue Accueil)                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ Bienvenue sur Miyukini Central                                      │  │
│  │ Hub de gestion des Services — découvrez, activez et lancez vos     │  │
│  │ Services.                                                            │  │
│  │                                                                      │  │
│  │ [📦 Voir le catalogue]  [📌 Mes Services]                           │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Services récents                                                          │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐                    │
│  │ 🔢 Calc  │ │ 🎮 Jeu   │ │ 📝 Texte │ │ 📋 Notes│                    │
│  │          │ │          │ │          │ │          │                    │
│  │ [Ouvrir] │ │ [Ouvrir] │ │ [Ouvrir] │ │ [Ouvrir] │                    │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘                    │
│                                                                             │
│  État de l'environnement                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ 🟢 Environnement normal (T0)                                        │  │
│  │ Environnement connecté : COG-Local v1.2.3                          │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Bannière d'accueil** : Message de bienvenue, boutons d'action rapide
- **Services récents** : Grille de cartes Services (6 max) avec bouton "Ouvrir"
- **État environnement** : Badge état de confiance (T0-T4), identité COG

**Actions :**
- Clic "Voir le catalogue" → Navigation vers Vue Catalogue
- Clic "Mes Services" → Navigation vers Vue Mes Services
- Clic "Ouvrir" sur un Service → Lancement du Service (nouvel onglet)

---

### 2.2 Vue Catalogue

**Objectif :** Découvrir tous les Services disponibles.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet HUB - Vue Catalogue)                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Catalogue des Services                                                     │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ 🔍 [Rechercher un Service...]                                       │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Catégorie : [Toutes] [🛠️ Utilitaires] [🎯 Loisirs] [⚡ Productivité]    │
│                                                                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐     │
│  │ 🔢           │ │ 🎮           │ │ 📝           │ │ 📋           │     │
│  │              │ │              │ │              │ │              │     │
│  │ Calculatrice │ │ Jeu          │ │ Traitement   │ │ Notes        │     │
│  │              │ │              │ │ de texte     │ │              │     │
│  │ Calculs      │ │ Jeu de       │ │ Éditeur de   │ │ Notes        │     │
│  │ basiques :   │ │ démonstration│ │ texte simple │ │ rapides :    │     │
│  │ +, −, ×, /   │ │ : clics      │ │ pour rédiger │ │ listez vos   │     │
│  │              │ │ rapides      │ │ des documents │ │ idées        │     │
│  │              │ │              │ │              │ │              │     │
│  │ 🟢 Standard  │ │ 🟢 Standard  │ │ 🟢 Standard  │ │ 🟢 Standard  │     │
│  │              │ │              │ │              │ │              │     │
│  │ [Détails]    │ │ [Détails]    │ │ [Détails]    │ │ [Détails]    │     │
│  │ [Ouvrir]     │ │ [Ouvrir]     │ │ [Ouvrir]     │ │ [Ouvrir]     │     │
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘     │
│                                                                             │
│  [Scroll si plus de Services...]                                           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Barre de recherche** : Champ de recherche texte (filtre côté client)
- **Filtres catégories** : Pills de catégories (Toutes, Utilitaires, Loisirs, Productivité)
- **Grille de Services** : Cartes Services avec icône, nom, description, badge niveau sécurité, boutons

**Actions :**
- Recherche : Filtre la liste en temps réel
- Clic catégorie : Filtre par catégorie
- Clic "Détails" : Affiche Vue Fiche Service
- Clic "Ouvrir" : Lance le Service (nouvel onglet)

---

### 2.3 Vue Fiche Service

**Objectif :** Détails complets d'un Service.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet HUB - Vue Fiche Service)                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ 🔢                                                                   │  │
│  │                                                                      │  │
│  │ Calculatrice                                                        │  │
│  │                                                                      │  │
│  │ Calculs basiques : addition, soustraction, multiplication, division.│  │
│  │                                                                      │  │
│  │ 🛠️ Utilitaires · v1.0.0                                             │  │
│  │ 🟢 Niveau de sécurité : Standard                                    │  │
│  │ ✅ État : ACTIF                                                     │  │
│  │                                                                      │  │
│  │ Opérateur(s) : Opérateur Calculatrice                               │  │
│  │                                                                      │  │
│  │ Prérequis :                                                          │  │
│  │ - Environnement COG vers. 1.2.0+                                         │  │
│  │ - Niveau de sécurité : Standard (1)                                 │  │
│  │                                                                      │  │
│  │ [Ouvrir ce Service]  [← Catalogue]  [← Mes Services]                │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Icône grande** : Icône du Service (taille ~48x48px)
- **Nom et description** : Titre et description complète
- **Métadonnées** : Catégorie, version, niveau de sécurité, état de vie
- **Opérateur(s)** : Liste des Opérateurs qui portent le Service
- **Prérequis** : Conditions d'utilisation (environnement, sécurité)
- **Boutons d'action** : Ouvrir, retour Catalogue ou Mes Services

**Actions :**
- Clic "Ouvrir ce Service" : Lance le Service (nouvel onglet)
- Clic "← Catalogue" : Retour à Vue Catalogue
- Clic "← Mes Services" : Retour à Vue Mes Services

---

### 2.4 Vue Mes Services

**Objectif :** Liste des Services auxquels l'utilisateur a accès.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet HUB - Vue Mes Services)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Mes Services                                                               │
│                                                                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐     │
│  │ 🔢           │ │ 🎮           │ │ 📝           │ │ 📋           │     │
│  │              │ │              │ │              │ │              │     │
│  │ Calculatrice │ │ Jeu          │ │ Traitement   │ │ Notes        │     │
│  │              │ │              │ │ de texte     │ │              │     │
│  │              │ │              │ │              │ │              │     │
│  │ 🟢 Ouvert    │ │ 🟢 À jour    │ │ 🟢 À jour    │ │ 🟢 À jour    │     │
│  │              │ │              │ │              │ │              │     │
│  │ [Ouvrir]     │ │ [Ouvrir]     │ │ [Ouvrir]     │ │ [Ouvrir]     │     │
│  │ [Détails]    │ │ [Détails]    │ │ [Détails]    │ │ [Détails]    │     │
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘     │
│                                                                             │
│  Si aucun Service activé :                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ Aucun Service activé                                                │  │
│  │                                                                      │  │
│  │ Parcourez le Catalogue et cliquez sur « Ouvrir » pour en ajouter. │  │
│  │                                                                      │  │
│  │ [📦 Voir le catalogue]                                              │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Grille de Services** : Cartes Services avec état (Ouvert, À jour, Déprécié)
- **Message vide** : Si aucun Service activé, message avec lien vers Catalogue

**Actions :**
- Clic "Ouvrir" : Active l'onglet existant ou lance le Service
- Clic "Détails" : Affiche Vue Fiche Service

---

### 2.5 Vue Paramètres

**Objectif :** Préférences UI du Hub.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet HUB - Vue Paramètres)                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Paramètres                                                                 │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ Apparence                                                            │  │
│  │                                                                      │  │
│  │ ☑ Thème sombre                                                      │  │
│  │                                                                      │  │
│  │ Langue : [Français ▼]                                               │  │
│  │                                                                      │  │
│  │ Taille de fenêtre : [Restauration automatique]                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ À propos                                                            │  │
│  │                                                                      │  │
│  │ Miyukini Central — MVP Hub                                          │  │
│  │ Version démo — Services factices                                    │  │
│  │                                                                      │  │
│  │ Licence : Miyukini COG                                              │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Préférences UI** : Thème, langue, taille fenêtre
- **À propos** : Version, crédits, licence

**Actions :**
- Modification préférences : Sauvegarde automatique (eframe persistence)

---

## 3. Onglet Service

**Objectif :** Afficher l'interface d'un Service ouvert.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet Service - ex. Calculatrice)                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  [Contenu du Service - interface spécifique au Service]                    │
│                                                                             │
│  Exemple Calculatrice :                                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │                          [Affichage]                                │  │
│  │                                                                      │  │
│  │  [7] [8] [9] [/]                                                    │  │
│  │  [4] [5] [6] [×]                                                    │  │
│  │  [1] [2] [3] [-]                                                    │  │
│  │  [C] [0] [=] [+]                                                   │  │
│  │                                                                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Comportement :**
- Le Service s'affiche dans le contenu principal
- Le Header reste visible avec l'onglet du Service actif
- Fermeture de l'onglet → kill le Service

---

## 4. Onglet Profile

**Objectif :** Profil utilisateur et paramètres personnels.

**Structure :**
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ CONTENU PRINCIPAL (Onglet Profile)                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │                    [👤 Photo de profil]                             │  │
│  │                                                                      │  │
│  │                    Jean Dupont                                     │  │
│  │                    jean.dupont@example.com                          │  │
│  │                                                                      │  │
│  │  Environnement connecté : COG-Local v1.2.3                         │  │
│  │  Identité : LSI-ABC123...                                           │  │
│  │                                                                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  Préférences                                                                │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │ [Paramètres du Hub]  [Notifications]  [Sécurité]                   │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Composants :**
- **Photo et informations** : Photo de profil, nom, email
- **Environnement** : COG connecté, identité (LSI/VID/WID)
- **Préférences** : Onglets pour paramètres, notifications, sécurité

**Actions :**
- Modification préférences : Sauvegarde automatique
- Déconnexion : Option dans menu déroulant photo de profil

---

## 5. Fenêtres modales

### 5.1 Confirmation de lancement

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ FENÊTRE MODALE (Confirmation)                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │                    Ouvrir ce Service ?                             │  │
│  │                                                                      │  │
│  │                    🔢 Calculatrice                                 │  │
│  │                                                                      │  │
│  │                    [Annuler]  [Ouvrir]                             │  │
│  │                                                                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Message d'erreur gouverné

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ FENÊTRE MODALE (Erreur)                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                                                                      │  │
│  │                    ⚠️ Accès refusé                                 │  │
│  │                                                                      │  │
│  │                    Ce Service n'est pas disponible dans votre      │  │
│  │                    environnement.                                   │  │
│  │                                                                      │  │
│  │                    [Retour]  [Réessayer]                           │  │
│  │                                                                      │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 6. États et indicateurs visuels

### 6.1 États des Services

| État | Badge/Indicateur | Description |
|------|------------------|-------------|
| **ACTIF** | 🟢 "À jour" | Service utilisable normalement |
| **DÉPRÉCIÉ** | 🟡 "Déprécié" | Service utilisable mais migration recommandée |
| **BROUILLON** | ⚪ "Brouillon" | Service non disponible en production |
| **RETIRÉ** | 🔴 "Retiré" | Service non disponible |
| **Ouvert** | 🟢 "Ouvert" | Service actuellement ouvert (onglet actif) |

### 6.2 États de l'environnement

| État | Badge | Description |
|------|-------|-------------|
| **T0 (Normal)** | 🟢 "Normal" | Tous les Services accessibles |
| **T1 (Instable)** | 🟡 "Instable" | Surveillance accrue |
| **T2 (Dégradé)** | 🟠 "Dégradé" | Capacités réduites |
| **T3 (Restreint)** | 🔴 "Restreint" | Mode restreint |
| **T4 (Bloqué)** | 🔴 "Bloqué" | Uniquement diagnostics |

---

## 7. Références

| Document | Lien |
|----------|------|
| **UI/UX Header et Navigation** | [Miyukini Central - UI/UX Header et Navigation](./Miyukini%20Central%20-%20UI%20UX%20Header%20et%20Navigation.md) |
| **Parcours Utilisateurs** | [Miyukini Central - Parcours Utilisateurs](./Miyukini%20Central%20-%20Parcours%20Utilisateurs.md) |
| **Maquette Conceptuelle** | [Miyukini Central - Maquette Conceptuelle Header](./reference/Miyukini%20Central%20-%20Maquette%20Conceptuelle%20Header.md) |

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document de référence — Écrans et UI Miyukini Central
