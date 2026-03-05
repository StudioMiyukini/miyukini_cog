# MiyuWeb â€” RÃ©fÃ©rence des outils

## 1. Contexte

Ce document dÃ©crit **chaque outil (Tool)** composant le kit MiyuWeb. Il constitue la rÃ©fÃ©rence technique des capacitÃ©s atomiques d'affichage de contenu web (rendu HTML, layout, thÃ¨me, scripts, assets, formulaires, Ã©vÃ©nements) sans dÃ©cision de contenu ni accÃ¨s direct Ã  la base. Les Tools sont gouvernÃ©s par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; l'autoritÃ© sur les donnÃ©es (templates, assets) appartient Ã  KindMother. MiyuWeb opÃ¨re sur des **donnÃ©es fournies dans le flux**.

**RÃ©fÃ©rence du kit :** [MiyuWeb - Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuWeb
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte Â« fait quoi Â»), **niveau de sÃ©curitÃ©** typique, **capability_id** si applicable

**Hors scope :**

- L'implÃ©mentation (moteur de rendu, sandbox JS, CSP)
- Toute dÃ©cision de contenu ou de logique mÃ©tier (OpÃ©rateurs, Cores)
- La lecture ou l'Ã©criture en base (KindMother, MiyuSQL)

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sÃ©curitÃ© | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.web.html.render` | Rendu HTML | Rend du HTML Ã  partir de donnÃ©es et de template fournis ; ne dÃ©cide pas du contenu. | 0â€“2 | `web.html.render` |
| `tool.web.layout.render` | Rendu layout | Rend un layout (structure de page) Ã  partir de donnÃ©es fournies. | 0â€“2 | `web.layout.render` |
| `tool.web.theme.resolve` | RÃ©solution thÃ¨me | RÃ©sout le thÃ¨me applicable (couleurs, styles) pour un contexte donnÃ©. | 0â€“1 | `web.theme.resolve` |
| `tool.web.script.execute` | ExÃ©cution script | ExÃ©cute un script (JS/TS) dans un contexte gouvernÃ© et sandboxÃ©. | 1â€“2 | `web.script.execute` |
| `tool.web.script.compile` | Compilation script | Compile ou valide un script sans l'exÃ©cuter. | 1â€“2 | `web.script.compile` |
| `tool.web.asset.serve` | Service asset | Sert un asset (image, CSS, etc.) Ã  partir de donnÃ©es fournies dans le flux. | 0â€“2 | `web.asset.serve` |
| `tool.web.form.validate` | Validation formulaire | Valide un formulaire (structure, champs) sans dÃ©cider des rÃ¨gles mÃ©tier. | 1â€“2 | `web.form.validate` |
| `tool.web.event.dispatch` | Dispatch Ã©vÃ©nement | Dispatche un Ã©vÃ©nement dans le flux gouvernÃ©. | 0â€“2 | `web.event.dispatch` |
| `tool.web.input.capture` | Capture entrÃ©e | Capture une entrÃ©e utilisateur (clic, saisie) pour le flux gouvernÃ©. | 0â€“2 | `web.input.capture` |

**Format ToolId :** `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` â€” conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. DÃ©tail par outil (rÃ©sumÃ©)

### 4.1 Rendu

- **tool.web.html.render** â€” Produit du HTML Ã  partir d'un template et de donnÃ©es fournis dans le flux. Ne choisit pas le contenu ; exÃ©cute uniquement le rendu. Les templates peuvent provenir d'une lecture KindMother/MiyuSQL en amont, fournis en entrÃ©e.
- **tool.web.layout.render** â€” Produit la structure de page (layout) Ã  partir de donnÃ©es fournies (zones, placeholders). Ne dÃ©cide pas du contenu des zones.

### 4.2 ThÃ¨me

- **tool.web.theme.resolve** â€” DÃ©termine le thÃ¨me applicable (couleurs, variables CSS, styles) pour un contexte donnÃ© (ex. mode clair/sombre, identifiant thÃ¨me). Retourne des donnÃ©es de thÃ¨me utilisables par les outils de rendu ; ne dÃ©cide pas de la politique de thÃ¨me.

### 4.3 Scripts

- **tool.web.script.execute** â€” ExÃ©cute un script (JavaScript, TypeScript compilÃ©) dans un contexte sandboxÃ© et gouvernÃ©. Les entrÃ©es (donnÃ©es, contexte) sont fournies dans le flux ; pas d'accÃ¨s direct Ã  la base ni de dÃ©cision mÃ©tier.
- **tool.web.script.compile** â€” Compile ou valide un script (syntaxe, types) sans l'exÃ©cuter. Utile pour vÃ©rification ou prÃ©paration.

### 4.4 Assets

- **tool.web.asset.serve** â€” Sert un asset (image, fichier CSS, binaire) Ã  partir de donnÃ©es fournies dans le flux (contenu ou mÃ©tadonnÃ©es). Ne lit pas la base ; l'asset peut avoir Ã©tÃ© rÃ©cupÃ©rÃ© en amont via MiyuSQL sous KindMother.

### 4.5 Formulaire et Ã©vÃ©nements

- **tool.web.form.validate** â€” Valide la structure et les champs d'un formulaire (prÃ©sence, types, contraintes techniques). Ne dÃ©finit pas les rÃ¨gles mÃ©tier ; exÃ©cute une validation gouvernÃ©e sur des rÃ¨gles fournies.
- **tool.web.event.dispatch** â€” Propage un Ã©vÃ©nement dans le flux gouvernÃ© (ex. clic, soumission). Ne dÃ©cide pas du traitement ; dispatche uniquement.
- **tool.web.input.capture** â€” Capture une entrÃ©e utilisateur (clic, saisie, touche) et la transmet au flux gouvernÃ©. Ne dÃ©cide pas de l'usage ; capture uniquement.

---

## 5. Alignement MIP

Chaque outil listÃ© ci-dessus est conÃ§u pour Ãªtre une **unitÃ© logique** pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation :

- **id** : identifiant du bloc (ex. dÃ©rivÃ© du ToolId)
- **do** : description fonctionnelle courte (ex. Â« rend du HTML Ã  partir de donnÃ©es et template fournis Â»)
- **role** : rÃ´le sÃ©mantique (ex. `web`)
- **layer** : couche (Strate 6 â€” outil / toolkit)

Ã€ l'implÃ©mentation, le code fournissant ces Tools devra Ãªtre balisÃ© MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) (domaine `web`).

---

## 6. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuWeb | [MiyuWeb - Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| KindMother | [KindMother - Index](..//..//cores//KindMother//_index.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence


