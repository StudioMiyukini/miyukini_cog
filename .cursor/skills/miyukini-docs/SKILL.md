---
name: miyukini-docs
description: Nomenclature et regles de documentation du projet Miyukini COG. Format de nommage des fichiers, arborescence standard (docs/core, tools, services, reference, miyukini-webway-system), prefixes, qualite requise, structure MWS. Utiliser quand on cree ou modifie un fichier de documentation, quand on genere de la doc automatiquement, ou quand on organise les fichiers docs/.
---

# Documentation Miyukini — Nomenclature et regles

## Format de nommage des fichiers

```
<PREFIX> - <SUJET> <DETAIL_OPTIONNEL>.<ext>
```

**Conventions :** pas d'accents, majuscules permises.

## Prefixes (cadre du document)

Un SEUL prefixe obligatoire par document, selon le cadre :
- Framework Miyukini
- Application en cours de dev
- Didacticiel / mode d'emploi client

## Sujet

Precis et oriente module ou fonctionnalite :
- `Fonctionnalite Authentication`
- `Module Facturation`

## Detail optionnel

Facultatif mais recommande pour eviter les fichiers fourre-tout :
- `V1`, `Alpha`, `Beta`, `MVP`
- `Edge_cases`, `Supabase`, `Stripe`
- `Offline`, `PWA`, `Localstorage`

## Extensions autorisees

| Extension | Usage |
|-----------|-------|
| `.md` | Documentation principale (PRIORITAIRE) |
| `.txt` | Notes brutes |
| `.pdf` | Export fige |
| `.drawio` | Schemas |
| `.json` | Specifications machine |
| `.yaml` | Config / infra |
| `.csv` | Donnees de reference |

## Arborescence standard `docs/`

```
docs/
├── core/              # Un sous-dossier par Core (8 Cores Strate 4 + BondingBrother Strate 5)
│   ├── BondingBrother/   # Strate 5 — mediation
│   ├── BorderGuard/
│   ├── CaringNanny/
│   ├── EverBuddy/
│   ├── KindMother/
│   ├── MasterButler/
│   ├── StrongFather/
│   ├── TAMR/
│   └── WorrySentinel/
├── miyukini-webway-system/  # Racine MWS (systeme presence/decouverte/transport)
│   ├── MWS - Document Fondateur.md
│   ├── reference/_index.md   # Index des references MWS
│   ├── architecture/, acteurs/, verification/, securite/, lobbys/, protocole/, deploiement/
├── tools/             # Un sous-dossier par Toolkit
│   └── Miyu{Nom}/
├── services/          # Un sous-dossier par Service
│   └── Jay{Nom}/
├── reference/         # References conceptuelles (Glossaire, Relay, MWS, etc.)
├── protocols/         # Protocoles de dev (MIP, MSCM)
├── kernel/            # Architecture Kernel
├── security/          # Politiques de securite
├── ux_ui/             # Guidelines UI/UX
├── games/             # Documentation des jeux
├── implementation/    # Guides d'implementation
├── legal/             # Documents juridiques
├── market/            # Analyse concurrentielle
└── archive/           # Deprecie
```

## Regles de qualite

Tout document DOIT avoir :
1. Un titre clair en **H1**
2. Une section **"Contexte"**
3. Une section **"Portee / Scope"**
4. Etre oriente **action ou decision**

## Regles de rangement

1. Deduire le dossier depuis le contenu
2. Deduire le sous-dossier
3. Refuser la creation si le nom ne respecte pas la nomenclature
4. Ne JAMAIS ecraser un fichier existant sans versioning (`_vX`)

## Structure type d'un dossier Toolkit dans docs/tools/

```
docs/tools/Miyu{Nom}/
├── _index.md
├── Documentation Fondatrice.md
├── Reference Outils.md
├── contracts/
│   └── governance/
└── implementation/
```

## Documentation MWS (Miyukini Webway System)

- **Racine obligatoire :** `docs/miyukini-webway-system/` — tout document fondateur ou de positionnement MWS part de la. Les specs detaillees (relay, protocole, contrats) restent dans `docs/reference/`, `docs/tools/MiyuWebwayTracker/`, `docs/setup/` et sont **liees** depuis l'index MWS.
- **Prefixe des fichiers MWS :** `MWS - <Sujet>.md` (ex. `MWS - Document Fondateur.md`, `MWS - Origin.md`).
- **Index des references :** `docs/miyukini-webway-system/reference/_index.md` pointe vers tous les documents MWS et references conceptuelles.

## Rappel

- Une **page** sert a livrer
- Un **ecran** sert a concevoir
- Les documents `brain_` ne sont PAS contractuels

## References

*Chemins relatifs a la racine du workspace.*

- **Racine documentation** : `docs/`
- **References conceptuelles** : `docs/reference/` (Glossaire, MWS, Relay, etc.)
- **MWS** : `docs/miyukini-webway-system/` — fondateur, architecture, acteurs, index : `reference/_index.md`
- **Toolkits** : `docs/tools/Miyu{Nom}/` (ex. MiyuWebwayTracker, MiyuAuth) — Documentation Fondatrice, Reference Outils, contracts/, implementation/
- **Services** : `docs/services/` (Jay*, Miyukini*)
- **Template contrats** : `docs/contrats/` (Miyukini Protocol - Ecriture Enrichie Toolkits, etc.)
