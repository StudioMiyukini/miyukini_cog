# MiyuWeb — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuWeb. Il constitue la référence technique des capacités atomiques d'affichage de contenu web (rendu HTML, layout, thème, scripts, assets, formulaires, événements) sans décision de contenu ni accès direct à la base. Les Tools sont gouvernés par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; l'autorité sur les données (templates, assets) appartient à KindMother. MiyuWeb opère sur des **données fournies dans le flux**.

**Référence du kit :** [MiyuWeb - Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuWeb
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte « fait quoi »), **niveau de sécurité** typique, **capability_id** si applicable

**Hors scope :**

- L'implémentation (moteur de rendu, sandbox JS, CSP)
- Toute décision de contenu ou de logique métier (Opérateurs, Cores)
- La lecture ou l'écriture en base (KindMother, MiyuSQL)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.web.html.render` | Rendu HTML | Rend du HTML à partir de données et de template fournis ; ne décide pas du contenu. | 0–2 | `web.html.render` |
| `tool.web.layout.render` | Rendu layout | Rend un layout (structure de page) à partir de données fournies. | 0–2 | `web.layout.render` |
| `tool.web.theme.resolve` | Résolution thème | Résout le thème applicable (couleurs, styles) pour un contexte donné. | 0–1 | `web.theme.resolve` |
| `tool.web.script.execute` | Exécution script | Exécute un script (JS/TS) dans un contexte gouverné et sandboxé. | 1–2 | `web.script.execute` |
| `tool.web.script.compile` | Compilation script | Compile ou valide un script sans l'exécuter. | 1–2 | `web.script.compile` |
| `tool.web.asset.serve` | Service asset | Sert un asset (image, CSS, etc.) à partir de données fournies dans le flux. | 0–2 | `web.asset.serve` |
| `tool.web.form.validate` | Validation formulaire | Valide un formulaire (structure, champs) sans décider des règles métier. | 1–2 | `web.form.validate` |
| `tool.web.event.dispatch` | Dispatch événement | Dispatche un événement dans le flux gouverné. | 0–2 | `web.event.dispatch` |
| `tool.web.input.capture` | Capture entrée | Capture une entrée utilisateur (clic, saisie) pour le flux gouverné. | 0–2 | `web.input.capture` |

**Format ToolId :** `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` — conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. Détail par outil (résumé)

### 4.1 Rendu

- **tool.web.html.render** — Produit du HTML à partir d'un template et de données fournis dans le flux. Ne choisit pas le contenu ; exécute uniquement le rendu. Les templates peuvent provenir d'une lecture KindMother/MiyuSQL en amont, fournis en entrée.
- **tool.web.layout.render** — Produit la structure de page (layout) à partir de données fournies (zones, placeholders). Ne décide pas du contenu des zones.

### 4.2 Thème

- **tool.web.theme.resolve** — Détermine le thème applicable (couleurs, variables CSS, styles) pour un contexte donné (ex. mode clair/sombre, identifiant thème). Retourne des données de thème utilisables par les outils de rendu ; ne décide pas de la politique de thème.

### 4.3 Scripts

- **tool.web.script.execute** — Exécute un script (JavaScript, TypeScript compilé) dans un contexte sandboxé et gouverné. Les entrées (données, contexte) sont fournies dans le flux ; pas d'accès direct à la base ni de décision métier.
- **tool.web.script.compile** — Compile ou valide un script (syntaxe, types) sans l'exécuter. Utile pour vérification ou préparation.

### 4.4 Assets

- **tool.web.asset.serve** — Sert un asset (image, fichier CSS, binaire) à partir de données fournies dans le flux (contenu ou métadonnées). Ne lit pas la base ; l'asset peut avoir été récupéré en amont via MiyuSQL sous KindMother.

### 4.5 Formulaire et événements

- **tool.web.form.validate** — Valide la structure et les champs d'un formulaire (présence, types, contraintes techniques). Ne définit pas les règles métier ; exécute une validation gouvernée sur des règles fournies.
- **tool.web.event.dispatch** — Propage un événement dans le flux gouverné (ex. clic, soumission). Ne décide pas du traitement ; dispatche uniquement.
- **tool.web.input.capture** — Capture une entrée utilisateur (clic, saisie, touche) et la transmet au flux gouverné. Ne décide pas de l'usage ; capture uniquement.

---

## 5. Alignement MIP

Chaque outil listé ci-dessus est conçu pour être une **unité logique** pouvant devenir un **bloc MSCM** à l'implémentation :

- **id** : identifiant du bloc (ex. dérivé du ToolId)
- **do** : description fonctionnelle courte (ex. « rend du HTML à partir de données et template fournis »)
- **role** : rôle sémantique (ex. `web`)
- **layer** : couche (Strate 6 — outil / toolkit)

À l'implémentation, le code fournissant ces Tools devra être balisé MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) (domaine `web`).

---

## 6. Références croisées

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuWeb | [MiyuWeb - Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| KindMother | [KindMother - Index](../../core/KindMother/_index.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
