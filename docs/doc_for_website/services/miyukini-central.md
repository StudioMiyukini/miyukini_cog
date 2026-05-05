# Miyukini Central

## Hub Principal et Point d'Accès

**Miyukini Central** est le hub central du COG. C'est le point d'entrée principal pour l'utilisateur et la surface d'accès à tous les services.

## Rôle

> Central **rassemble** les services et **orchestre** l'expérience utilisateur.

Central n'est pas un simple launcher : il gère la navigation, l'authentification, les préférences et l'intégration de tous les services (JayKoa, JayKonta, Miou, etc.).

## Fonctionnalités Principales

| Fonction | Description |
|----------|-------------|
| **Tableau de bord** | Vue d'ensemble et accès rapide |
| **Catalogue de services** | Grille ou liste avec recherche et filtres (catégories, types) |
| **Cartes de services** | Nom, description, bouton d'ouverture pour chaque service |
| **Onglets** | Hub et services ouverts simultanément (multi-fenêtres) |
| **Sidebar** | Recherche et filtres pour parcourir le catalogue |
| **Profil et paramètres** | Overlays pour identité, préférences, thème clair/sombre (persistant) |
| **Rite de première connexion** | Onboarding et configuration initiale |
| **Intégration Miou** | Assistant IA intégré dans l'interface |

*Dans Miyukini, les utilisateurs n'installent pas d'applications : ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.*

## Architecture

```
┌─────────────────────────────────────────────────┐
│              MIYUKINI CENTRAL                   │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐          │
│  │Dashboard│ │ Services│ │  Miou   │          │
│  └─────────┘ └─────────┘ └─────────┘          │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐          │
│  │ Profil  │ │ Connexion│ │ Préférences│        │
│  └─────────┘ └─────────┘ └─────────┘          │
└─────────────────────────────────────────────────┘
```

## Services Intégrés

Central affiche et lance :

- JayKoa (calendrier universel)
- JayKonta (comptabilité)
- JayRDV (rendez-vous)
- JayShop (boutique)
- Jay Bureau (suite collaborative)
- Miou (assistant)
- MiyukiniWatch (métriques)
- Miyukini Cloud (cloud privé)
- MiyukiniClicker (jeu)
- Et les autres services actifs sur le COG

## Rite de Première Connexion

Lors de la première utilisation, Central guide l'utilisateur à travers :

1. **Accueil** — Présentation du COG
2. **Configuration** — Préférences de base
3. **Services** — Activation des services souhaités
4. **Miou** — Découverte de l'assistant (optionnel)
5. **Terminé** — Accès au tableau de bord

## Parcours utilisateur typique

1. Ouverture de **Miyukini Central** (application desktop).
2. Parcours du **catalogue de services** (grille ou liste, recherche, filtres).
3. Ouverture d'un service : l'Opérateur s'exécute sous gouvernance (StrongFather autorise, KindMother persiste).
4. L'utilisateur ne voit pas les Cores ni les Toolkits — il voit des **Services** et des interfaces cohérentes.

## Sécurité

- Authentification via TAMR / MiyuAuth
- Chaque service sous contrôle des Cores
- Données locales souveraines (LOI-3)
