# MiyuWeb â€” Tool Governance Compliance Contract

## 1. Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

Ce document dÃ©finit la **conformitÃ© de MiyuWeb** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuWeb est un **Kit d'Outils (Toolkit)** dÃ©clarÃ© et gouvernÃ© par l'environnement ; ce contrat Ã©tablit la dÃ©claration formelle du ToolkitId, des ToolIds composants, et des capabilities associÃ©es.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- La conformitÃ© au [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformitÃ© au [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La dÃ©claration formelle du ToolkitId et des ToolIds MiyuWeb
- Le catalogue des capabilities exposÃ©es

Ce document **ne couvre pas** :
- L'implÃ©mentation technique des Tools
- Les contrats MiyuWeb hors gouvernance (intÃ©gration KindMother, sÃ©curitÃ©, bornage)

---

## 3. ConformitÃ© au Tool Governance Contract

### 3.1 Principes respectÃ©s

| Principe Master Butler | Application MiyuWeb |
|------------------------|---------------------|
| Tout Tool possÃ¨de un ToolId unique et immuable | Chaque outil MiyuWeb a un ToolId au format `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` |
| Tout Tool est liÃ© Ã  exactement une Capability | Chaque ToolId est associÃ© Ã  un capability_id (voir section 5) |
| Un Tool ne prend jamais de dÃ©cision mÃ©tier | Les Tools MiyuWeb exÃ©cutent uniquement ; pas de dÃ©cision de contenu ni d'accÃ¨s direct Ã  la base |
| Un Tool ne connaÃ®t jamais l'OpÃ©rateur appelant | MiyuWeb reÃ§oit un contexte gouvernÃ© ; pas d'identitÃ© OpÃ©rateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.web.<action>` ou `tool.web.<sous-domaine>.<action>`

- **PrÃ©fixe :** `tool.`
- **Domaine MiyuWeb :** `web`
- **Segments :** minuscules, sans accents, sÃ©parÃ©s par des points (ex. `html.render`, `script.execute`, `asset.serve`)

---

## 4. ConformitÃ© au Toolkit Composition Contract

### 4.1 Principes respectÃ©s

| Principe Toolkit | Application MiyuWeb |
|------------------|---------------------|
| Un Toolkit agrÃ¨ge des Tools existants | MiyuWeb regroupe neuf Tools dÃ©clarÃ©s individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacitÃ© nouvelle | MiyuWeb n'expose que les capacitÃ©s de ses Tools composants |
| Un Toolkit est dÃ©clarÃ© et validÃ© par l'environnement | MiyuWeb est dÃ©clarÃ© dans Master Butler avec ToolkitId `toolkit.web.miyuweb` |
| Tout Toolkit contient au moins deux Tools | MiyuWeb contient neuf Tools |

### 4.2 Structure formelle du Toolkit MiyuWeb

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.web.miyuweb` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `web` |
| **Name** | `miyuweb` |
| **Tools** | Ensemble des neuf ToolIds (voir section 5) |
| **security_level** | 0, 1 ou 2 (selon politique d'exposition ; voir [MiyuWeb - Security and States Contract](../security/MiyuWeb%20-%20Security%20and%20States%20Contract.md)) |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. DÃ©claration ToolkitId et ToolIds

### 5.1 ToolkitId

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.web.miyuweb` |
| **Nom lisible** | MiyuWeb |
| **Description** | Kit d'outils d'affichage de contenu web (rendu HTML, layout, thÃ¨me, scripts, assets, formulaires, Ã©vÃ©nements) ; opÃ¨re sur des donnÃ©es fournies dans le flux. |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.web.html.render` | `web.html.render` | Rend du HTML Ã  partir de donnÃ©es et de template fournis ; ne dÃ©cide pas du contenu |
| `tool.web.layout.render` | `web.layout.render` | Rend un layout (structure de page) Ã  partir de donnÃ©es fournies |
| `tool.web.theme.resolve` | `web.theme.resolve` | RÃ©sout le thÃ¨me applicable (couleurs, styles) pour un contexte donnÃ© |
| `tool.web.script.execute` | `web.script.execute` | ExÃ©cute un script (JS/TS) dans un contexte gouvernÃ© et sandboxÃ© |
| `tool.web.script.compile` | `web.script.compile` | Compile ou valide un script sans l'exÃ©cuter |
| `tool.web.asset.serve` | `web.asset.serve` | Sert un asset (image, CSS, etc.) Ã  partir de donnÃ©es fournies dans le flux |
| `tool.web.form.validate` | `web.form.validate` | Valide un formulaire (structure, champs) sans dÃ©cider des rÃ¨gles mÃ©tier |
| `tool.web.event.dispatch` | `web.event.dispatch` | Dispatche un Ã©vÃ©nement dans le flux gouvernÃ© |
| `tool.web.input.capture` | `web.input.capture` | Capture une entrÃ©e utilisateur (clic, saisie) pour le flux gouvernÃ© |

### 5.3 Invariants de dÃ©claration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.web.miyuweb` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listÃ© est dÃ©clarÃ© individuellement dans le catalogue avant d'Ãªtre associÃ© au Toolkit |
| **INV-DECL-3** | Le niveau de sÃ©curitÃ© du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools (0, 1 ou 2 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine web n'est ajoutÃ© au Toolkit MiyuWeb sans rÃ©vision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler dÃ©clare le Toolkit MiyuWeb et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 RÃ©solution

- Un OpÃ©rateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vÃ©rifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sÃ©curitÃ©.
- En cas d'autorisation (StrongFather ALLOW), l'exÃ©cution est dÃ©lÃ©guÃ©e ; les templates et assets utilisÃ©s sont fournis dans le flux (Ã©ventuellement issus de KindMother/MiyuSQL en amont), MiyuWeb ne lit pas la base directement.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Reference Outils | [MiyuWeb - Reference Outils](../../MiyuWeb%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence



