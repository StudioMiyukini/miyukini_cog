---
name: miyukini-architecture
description: Architecture pyramidale Miyukini COG, strates 0-9, 8 Cores systeme, BondingBrother (Strate 5), Lois d'Autonomie LOI-1 a LOI-8, MWS (Webway) comme systeme complet subordonne aux Cores, environnements souverains et gouvernance. Utiliser quand on prend des decisions architecturales, quand on cree ou modifie un crate, quand on travaille sur les Cores ou le MWS, ou quand on verifie la conformite architecturale.
---

# Architecture Miyukini COG

## Definition COG

**C**ore-**O**rchestrated **G**overnance Environment

Miyukini n'est PAS un OS. C'est un environnement de gouvernance orchestre par des Cores.

## Pyramide des strates

| Strate | Nom | Contenu | Dossier crate |
|--------|-----|---------|---------------|
| **9** | MiyukiniAdmin | Operateur Souverain (exception) | `miyukini-admin` |
| **7** | Operateurs | Entites fonctionnelles gouvernees | `jayfestival`, `jayxpose`, etc. |
| **6** | Outils & Kits d'Outils | Capacites executables | `miyu*` (49 toolkits) |
| **5** | Interfaces & Adaptation | BondingBrother | `bondingbrother` |
| **4** | Cores Systeme | 8 Cores de gouvernance | `strongfather`, `kindmother`, etc. |
| **3** | Invariants & Contrats | Principes architecturaux | (dans le code des Cores) |
| **K** | Kernel | Substrat technique neutre | `miyukini-kernel` |
| **0** | Hardware & OS | Realite physique | (hors code) |

## 8 Lois d'Autonomie (NON NEGOCIABLES)

| Loi | Enonce |
|-----|--------|
| **LOI-1** | Aucune dependance externe critique a l'execution |
| **LOI-2** | Le systeme accepte l'isolement comme etat normal |
| **LOI-3** | L'etat local est souverain |
| **LOI-4** | Pas de temps global requis |
| **LOI-5** | Le cout doit etre proportionnel au hardware |
| **LOI-6** | L'autonomie n'empeche pas la federation |
| **LOI-7** | La strate Cores est immuable — evolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements |

## Regles architecturales fondamentales

1. **Les Cores decident ou gouvernent, jamais n'executent**
2. **Les Outils font, mais ne decident jamais**
3. **Les Operateurs sont gouvernes, jamais autonomes**
4. **La strate Cores est immuable** — toute evolution = nouvel environnement
5. **Un Operateur est lie a un environnement unique**
6. **Migration = processus formel, jamais copie brute**
7. **Le Kernel ne contient aucune logique metier**
8. **`unsafe_code = "forbid"` dans tous les crates**

## Flux standard

```
Utilisateur → Service → Operateur(s) → BondingBrother → Cores → Outils → Execution
```

L'utilisateur voit des Services. Le systeme execute via des Operateurs gouvernes.

## MWS (Miyukini Webway System)

Le **MWS** est un **systeme complet** (pas une strate) : presence, decouverte et transport des COGs sur le reseau. Il est **uniquement subordonne aux Cores** et **consomme par toutes les strates** (Cores, Outils, Operateurs, Services).

- **Acteurs :** Origin (source de verite, relay + tracker), relays (duplication, verification, Permis de circulation), trackers (douaniers, controle tracker, pools par version), COG participant, COG Tracker.
- **Racine documentaire :** `docs/miyukini-webway-system/` — index des references : `reference/_index.md`.
- **Compatibilite :** LOI-2 (optionnel, pas de dependance critique), LOI-6 (federation), LOI-7 / LOI-8 (versioning, migration).

## Environnement (COG)

Un COG est une entite **souveraine, versionnee, isolee et identifiee** :
- Version complete des Cores (figee)
- ID unique (genere par le Kernel)
- Operateurs assujettis
- Frontieres strictes

## Niveaux d'identite

| Niveau | Nom | Confiance |
|--------|-----|-----------|
| LSI | Local Sovereign ID | Auto-declaree |
| VID | Verified ID | Attestee par un tiers |
| WID | Witnessed ID | Temoignee par d'autres COG |

## Securite

**Etats de confiance (T0-T4) :**
- T0 Normal → T1 Instable → T2 Degrade → T3 Restreint → T4 Bloque

**Niveaux de securite (0-4) :**
- 0 Public → 1 Standard → 2 Sensitive → 3 Critical → 4 Highest

## References detaillees

*Chemins `docs/` relatifs a la racine du workspace. Liens relatifs au skill pour references/.*

- **Pyramide (skill)** : [references/pyramid.md](references/pyramid.md)
- **Cores (skill)** : [references/cores.md](references/cores.md)
- **References conceptuelles** : `docs/reference/` — Glossaire, Definition COG, Lois Autonomie, Pyramide Architecture Complete, Souverainete Environnement, etc.
- **MWS (Webway)** : `docs/miyukini-webway-system/` — `MWS - Document Fondateur.md`, `architecture/`, `acteurs/` (Origin, Relays, Trackers), index : `reference/_index.md`
- **Relay / Protocole** : `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md`, `Miyukini Conceptual References - Miyukini Webway Relay Protocol.md`
