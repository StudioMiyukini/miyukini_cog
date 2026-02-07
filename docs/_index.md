# Miyukini COG - Index Documentation

## Contexte

Ce fichier est l'index global de la documentation du projet Miyukini COG.
La documentation est organisee selon les **strates de la Pyramide Miyukini** et par usage transverse.

## Portee / Scope

- **Applicable a :** Navigation et decouverte de la documentation
- **Audience :** Architectes, developpeurs, IA, contributeurs
- **Statut :** Index de reference

---

## Carte des Strates

| Strate | Dossier | Description | Contenu |
|--------|---------|-------------|---------|
| **K** | [kernel/](kernel/) | Substrat technique neutre | Id, Logger, Clock, Config, Lifecycle |
| **3** | [contrats/](contrats/) | Invariants et Contrats | Protocoles, principes architecturaux, templates |
| **4** | [cores/](cores/) | Cores Systeme | StrongFather, KindMother, CaringNanny, MasterButler, BorderGuard, EverBuddy, WorrySentinel, TAMR, LogisticsSteward |
| **5** | [interfaces/](interfaces/) | Interfaces et Adaptation | BondingBrother |
| **6** | [tools/](tools/) | Tools et Toolkits | 49+ Toolkits (MiyuAuth, MiyuCMS, MiyuBilling, etc.) |
| **7** | [services/](services/) | Services et Operateurs | JayKoa, JayFestival, JayKonta, JayRDV, JayXpose, JayFaim, MiyukiniClicker, MiyukiniCentral, MiyukiniSales, MiyukiniSurvivor |
| **9** | [admin/](admin/) | MiyukiniAdmin | Operateur Souverain (exception) |

---

## Documentation Transverse

| Dossier | Description |
|---------|-------------|
| [reference/](reference/) | References conceptuelles internes (Glossaire, Definition COG, Pyramide, Lois, etc.) |
| [reference/equivalents/](reference/equivalents/) | Analyses d'equivalence marche (PoS, Forum, Suite Bureautique, etc.) |
| [security/](security/) | Gouvernance securite transverse |
| [market/](market/) | Analyse de marche et concurrents (Odoo, etc.) |
| [implementation/](implementation/) | Documentation d'implementation versionnee (COG 0.1) |
| [ux_ui/](ux_ui/) | UI/UX, Design System, composants HyperUI |
| [spm-cms/](spm-cms/) | Framework SPM-CMS (adaptateurs, modules, contrats) |
| [qa/](qa/) | Qualite, audits, checklists |
| [legal/](legal/) | Documentation juridique |
| [setup/](setup/) | Installation et configuration des dependances |

---

## Documentation Publique

| Dossier | Description |
|---------|-------------|
| [public/](public/) | Vitrine documentation publique (copies des docs destinees au public) |

Contenu de `public/` :

- **README.md** - Presentation generale du projet
- **Miyukini - Glossaire.md** - Dictionnaire officiel de l'ecosysteme
- **Miyukini - Definition COG.md** - Definition du Core-Orchestrated Governance Environment
- **Miyukini - Pyramide Architecture Complete.md** - Architecture en strates
- **Miyukini - Lois Autonomie Systeme.md** - 8 lois d'autonomie non negociables
- **Miyukini - Objectif du projet.md** - Objectifs, strategie, produit, business, utilite, usage final
- **Miyukini - Souverainete Environnement.md** - Souverainete des environnements
- **Miyukini - Operators et Terminologie.md** - Operateurs et terminologie officielle
- **Miyukini - Tools et Toolkits.md** - Outils et Kits d'Outils
- **Miyukini - Mandats et Equipes Operators.md** - Mandats de Permission et Equipes
- **Miyukini - Connexion Inter-COG.md** - Architecture de visite gouvernee inter-COG
- **Miyukini - Kernel Maintenance Observability Contract.md** - Capacites de maintenance Kernel

> **Note :** Les fichiers dans `public/` sont des copies. Les originaux se trouvent dans `reference/`.

---

## Nomenclature

| Prefixe | Signification |
|---------|---------------|
| **MiyuXxx** | Toolkit concu par Miyukini |
| **MiyukiniOpsXxx** | Operateur concu par Miyukini |
| **MiyukiniXxx** | Service concu par Miyukini |
| **JayXxx** | Service officiel de la famille "Jay" |

---

## Strates de la Pyramide Miyukini (rappel)

```
Strate 9  │  MiyukiniAdmin          │  Operateur Souverain (exception)
Strate 7  │  Operateurs             │  Entites fonctionnelles gouvernees
Strate 6  │  Tools & Toolkits       │  Capacites executables
Strate 5  │  Interfaces & Adaptation│  BondingBrother
Strate 4  │  Cores Systeme          │  StrongFather, KindMother, etc.
Strate 3  │  Invariants & Contrats  │  Principes architecturaux
Strate K  │  Kernel                 │  Substrat technique neutre
Strate 0  │  Hardware & OS          │  Realite physique
```

---

**Date de creation :** 2026-02-07
**Derniere mise a jour :** 2026-02-07
