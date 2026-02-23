# MiyukiniTerminal — Index de la Documentation

## Contexte

Ce document centralise l'**index de toute la documentation** du service MiyukiniTerminal (App Android Terminal COG). Il sert de table des matières et de point d'entrée pour les développeurs et architectes.

**Références :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Plan de Développement](../../implementation/Miyukini%20COG%20-%20Plan%20Developpement%20App%20Android%20Terminal.md)

---

## Portée / Scope

- Table des matières de tous les documents MiyukiniTerminal
- Arborescence des fichiers
- Liens croisés et statut de rédaction
- Guide de navigation

---

## 1. Arborescence des documents

```
docs/services/MiyukiniTerminal/
├── _index.md                              # Point d'entrée (ce fichier parent)
├── MiyukiniTerminal - Document Fondateur.md
├── MiyukiniTerminal - Spec MSCM MIP Conformite.md
├── MiyukiniTerminal - Index Documentation.md          # Ce document
│
├── MiyukiniTerminal - Architecture Technique.md
├── MiyukiniTerminal - Stack Dioxus Mobile Spec.md
├── MiyukiniTerminal - Environnement Dev Android.md
├── MiyukiniTerminal - Alignement Central Dioxus.md
│
├── MiyukiniTerminal - Spec MWS Passeport Permis.md
├── MiyukiniTerminal - Spec Canaux Connexion MWS Parent Enfant.md
├── MiyukiniTerminal - Spec Protocole Relay Terminal.md
├── MiyukiniTerminal - Spec MiyuWebwayParticipant Adapt.md
│
├── MiyukiniTerminal - Spec Flux Liaison Parent.md
├── MiyukiniTerminal - Spec Central Gestion Terminaux.md
├── MiyukiniTerminal - Spec Token Liaison Securite.md
│
├── MiyukiniTerminal - Spec Stockage Local.md
├── MiyukiniTerminal - Spec Queue Actions Offline.md
│
├── MiyukiniTerminal - Spec Synchronisation Parent.md
├── MiyukiniTerminal - Spec Mode Offline.md
│
├── MiyukiniTerminal - Spec Ecrans et Navigation.md
├── MiyukiniTerminal - Spec Design System Mobile.md
├── MiyukiniTerminal - Spec Parcours Utilisateur.md
│
├── MiyukiniTerminal - Spec Services Consultatifs.md
├── MiyukiniTerminal - Spec Actions Simples.md
├── MiyukiniTerminal - Spec Notifications.md
│
├── MiyukiniTerminal - Spec Securite.md
├── MiyukiniTerminal - Spec Conformite Cores.md
│
├── MiyukiniTerminal - Spec Build et Signature.md
├── MiyukiniTerminal - Spec CI CD.md
│
├── MiyukiniTerminal - Spec Strategy Tests.md
│
├── MiyukiniTerminal - Reference Technique Complete.md
└── reference/
    └── _index.md                         # Index des références externes
```

---

## 2. Table des matières par catégorie

### A. Documents fondateurs

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 1 | [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md) | ✅ Rédigé | Vision, objectifs, non-objectifs, conformité LOI |
| 2 | Index Documentation (ce doc) | ✅ Rédigé | Table des matières, arborescence, liens |

### B. Architecture & technique

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 3 | [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md) | ✅ Rédigé | Couches, flux, schémas Mermaid |
| 4 | [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md) | ✅ Rédigé | Dioxus 0.6+, features mobile, WebView/WGPU |
| 5 | [Environnement Dev Android](./MiyukiniTerminal%20-%20Environnement%20Dev%20Android.md) | ✅ Rédigé | Toolchain, SDK, NDK, variables |
| 6 | [Alignement Central Dioxus](./MiyukiniTerminal%20-%20Alignement%20Central%20Dioxus.md) | ✅ Rédigé | Patterns Central → Terminal |

### C. MWS & protocoles

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 7 | [Spec MWS Passeport Permis](./MiyukiniTerminal%20-%20Spec%20MWS%20Passeport%20Permis.md) | ✅ Rédigé | Passeport TERMINAL, Permis, champs |
| 7b | [Spec Canaux Connexion MWS Parent-Enfant](./MiyukiniTerminal%20-%20Spec%20Canaux%20Connexion%20MWS%20Parent%20Enfant.md) | ✅ Rédigé | Modes direct Relay vs via parent ; canal sync |
| 8 | [Spec Protocole Relay Terminal](./MiyukiniTerminal%20-%20Spec%20Protocole%20Relay%20Terminal.md) | ✅ Rédigé | REGISTER, trames, heartbeats |
| 9 | [Spec MiyuWebwayParticipant Adapt](./MiyukiniTerminal%20-%20Spec%20MiyuWebwayParticipant%20Adapt.md) | ✅ Rédigé | Réutilisation, adaptation client |
| 9b | [Spec MSCM MIP Conformite](./MiyukiniTerminal%20-%20Spec%20MSCM%20MIP%20Conformite.md) | ✅ Rédigé | Balisage MSCM, index MIP, Phase B Terminal |

### D. Liaison Central ↔ Terminal

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 10 | [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md) | ✅ Rédigé | Flux complet, QR, token |
| 11 | [Spec Central Gestion Terminaux](./MiyukiniTerminal%20-%20Spec%20Central%20Gestion%20Terminaux.md) | ✅ Rédigé | Écran Central, limite 5 |
| 12 | [Spec Token Liaison Securite](./MiyukiniTerminal%20-%20Spec%20Token%20Liaison%20Securite.md) | ✅ Rédigé | Format token, stockage sécurisé |

### E. Stockage & persistance

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 13 | [Spec Stockage Local](./MiyukiniTerminal%20-%20Spec%20Stockage%20Local.md) | ✅ Rédigé | Tables, migrations, chemins Android |
| 14 | [Spec Queue Actions Offline](./MiyukiniTerminal%20-%20Spec%20Queue%20Actions%20Offline.md) | ✅ Rédigé | Structure queue, retry, conflits |

### F. Synchronisation

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 15 | [Spec Synchronisation Parent](./MiyukiniTerminal%20-%20Spec%20Synchronisation%20Parent.md) | ✅ Rédigé | Protocole sync, fréquence, conflits |
| 16 | [Spec Mode Offline](./MiyukiniTerminal%20-%20Spec%20Mode%20Offline.md) | ✅ Rédigé | États, cache, reconnexion |

### G. UI / UX

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 17 | [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md) | ✅ Rédigé | Écrans, flux, wireframes |
| 18 | [Spec Design System Mobile](./MiyukiniTerminal%20-%20Spec%20Design%20System%20Mobile.md) | ✅ Rédigé | Composants, thème, touch |
| 19 | [Spec Parcours Utilisateur](./MiyukiniTerminal%20-%20Spec%20Parcours%20Utilisateur.md) | ✅ Rédigé | Happy path, edge cases |

### H. Services & fonctionnalités

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 20 | [Spec Services Consultatifs](./MiyukiniTerminal%20-%20Spec%20Services%20Consultatifs.md) | ✅ Rédigé | JayKonta, JayKoa, format données |
| 21 | [Spec Actions Simples](./MiyukiniTerminal%20-%20Spec%20Actions%20Simples.md) | ✅ Rédigé | Dépenses, événements, délégation |
| 22 | [Spec Notifications](./MiyukiniTerminal%20-%20Spec%20Notifications.md) | ✅ Rédigé | Types, canal, permissions |

### I. Sécurité & conformité

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 23 | [Spec Securite](./MiyukiniTerminal%20-%20Spec%20Securite.md) | ✅ Rédigé | Keystore, TLS, verrouillage |
| 24 | [Spec Conformite Cores](./MiyukiniTerminal%20-%20Spec%20Conformite%20Cores.md) | ✅ Rédigé | Mapping Cores |

### J. Déploiement & CI

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 25 | [Spec Build et Signature](./MiyukiniTerminal%20-%20Spec%20Build%20et%20Signature.md) | ✅ Rédigé | dx bundle, APK, keystore |
| 26 | [Spec CI CD](./MiyukiniTerminal%20-%20Spec%20CI%20CD.md) | ✅ Rédigé | Pipeline, GitHub Actions |

### K. Tests

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 27 | [Spec Strategy Tests](./MiyukiniTerminal%20-%20Spec%20Strategy%20Tests.md) | ✅ Rédigé | Unitaires, intégration, E2E |

### L. Référence

| # | Document | Statut | Description |
|---|----------|--------|-------------|
| 28 | [Reference Technique Complete](./MiyukiniTerminal%20-%20Reference%20Technique%20Complete.md) | ✅ Rédigé | Tables, champs, constantes |
| 29 | [reference/_index.md](./reference/_index.md) | ✅ Rédigé | Références externes |

---

## 3. Ordre de lecture recommandé

1. **Document Fondateur** — Vision, bornage
2. **Architecture Technique** — Vue d'ensemble
3. **Stack Dioxus** + **Env Dev Android** — Setup
4. **Spec MWS** (7, 7b, 8, 9) — Connexion réseau, canaux parent-enfant
5. **Spec Liaison** (10, 11, 12) — Provisionnement
6. **Spec Stockage** + **Sync** (13–16) — Données
7. **Spec UI** (17–19) — Interface
8. **Spec Services** (20–22) — Fonctionnalités
9. **Spec Sécurité** (23–24) — Conformité
10. **Spec Build/CI/Tests** (25–27) — Déploiement

---

## 4. Liens vers références externes

| Document | Chemin |
|----------|--------|
| MWS - Passeport et Visa | `docs/miyukini-webway-system/verification/MWS - Passeport et Visa.md` |
| MWS - Protocole Relay | `docs/miyukini-webway-system/protocole/MWS - Protocole Relay.md` |
| Étude App Android Terminal | `docs/implementation/Miyukini COG - Etude App Android Terminal.md` |
| Plan Développement | `docs/implementation/Miyukini COG - Plan Developpement App Android Terminal.md` |
| Dioxus Mobile | https://dioxus.dev/learn/0.6/guides/mobile/ |

---

## 5. Mise à jour

Ce document doit être mis à jour à chaque création ou modification d'un document MiyukiniTerminal. Le statut (✅ Rédigé / ☐ À rédiger) reflète l'avancement de la Phase 1 du Plan de Développement.
