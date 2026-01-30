# MiyuWeb — Tool Governance Compliance Contract

## 1. Contexte

Ce document définit la **conformité de MiyuWeb** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuWeb est un **Kit d'Outils (Toolkit)** déclaré et gouverné par l'environnement ; ce contrat établit la déclaration formelle du ToolkitId, des ToolIds composants, et des capabilities associées.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- La conformité au [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformité au [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La déclaration formelle du ToolkitId et des ToolIds MiyuWeb
- Le catalogue des capabilities exposées

Ce document **ne couvre pas** :
- L'implémentation technique des Tools
- Les contrats MiyuWeb hors gouvernance (intégration KindMother, sécurité, bornage)

---

## 3. Conformité au Tool Governance Contract

### 3.1 Principes respectés

| Principe Master Butler | Application MiyuWeb |
|------------------------|---------------------|
| Tout Tool possède un ToolId unique et immuable | Chaque outil MiyuWeb a un ToolId au format `tool.web.<sous-domaine>.<action>` ou `tool.web.<action>` |
| Tout Tool est lié à exactement une Capability | Chaque ToolId est associé à un capability_id (voir section 5) |
| Un Tool ne prend jamais de décision métier | Les Tools MiyuWeb exécutent uniquement ; pas de décision de contenu ni d'accès direct à la base |
| Un Tool ne connaît jamais l'Opérateur appelant | MiyuWeb reçoit un contexte gouverné ; pas d'identité Opérateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.web.<action>` ou `tool.web.<sous-domaine>.<action>`

- **Préfixe :** `tool.`
- **Domaine MiyuWeb :** `web`
- **Segments :** minuscules, sans accents, séparés par des points (ex. `html.render`, `script.execute`, `asset.serve`)

---

## 4. Conformité au Toolkit Composition Contract

### 4.1 Principes respectés

| Principe Toolkit | Application MiyuWeb |
|------------------|---------------------|
| Un Toolkit agrège des Tools existants | MiyuWeb regroupe neuf Tools déclarés individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacité nouvelle | MiyuWeb n'expose que les capacités de ses Tools composants |
| Un Toolkit est déclaré et validé par l'environnement | MiyuWeb est déclaré dans Master Butler avec ToolkitId `toolkit.web.miyuweb` |
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

## 5. Déclaration ToolkitId et ToolIds

### 5.1 ToolkitId

| Élément | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.web.miyuweb` |
| **Nom lisible** | MiyuWeb |
| **Description** | Kit d'outils d'affichage de contenu web (rendu HTML, layout, thème, scripts, assets, formulaires, événements) ; opère sur des données fournies dans le flux. |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.web.html.render` | `web.html.render` | Rend du HTML à partir de données et de template fournis ; ne décide pas du contenu |
| `tool.web.layout.render` | `web.layout.render` | Rend un layout (structure de page) à partir de données fournies |
| `tool.web.theme.resolve` | `web.theme.resolve` | Résout le thème applicable (couleurs, styles) pour un contexte donné |
| `tool.web.script.execute` | `web.script.execute` | Exécute un script (JS/TS) dans un contexte gouverné et sandboxé |
| `tool.web.script.compile` | `web.script.compile` | Compile ou valide un script sans l'exécuter |
| `tool.web.asset.serve` | `web.asset.serve` | Sert un asset (image, CSS, etc.) à partir de données fournies dans le flux |
| `tool.web.form.validate` | `web.form.validate` | Valide un formulaire (structure, champs) sans décider des règles métier |
| `tool.web.event.dispatch` | `web.event.dispatch` | Dispatche un événement dans le flux gouverné |
| `tool.web.input.capture` | `web.input.capture` | Capture une entrée utilisateur (clic, saisie) pour le flux gouverné |

### 5.3 Invariants de déclaration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.web.miyuweb` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listé est déclaré individuellement dans le catalogue avant d'être associé au Toolkit |
| **INV-DECL-3** | Le niveau de sécurité du Toolkit est au moins égal au maximum des niveaux de ses Tools (0, 1 ou 2 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine web n'est ajouté au Toolkit MiyuWeb sans révision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler déclare le Toolkit MiyuWeb et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 Résolution

- Un Opérateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vérifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sécurité.
- En cas d'autorisation (StrongFather ALLOW), l'exécution est déléguée ; les templates et assets utilisés sont fournis dans le flux (éventuellement issus de KindMother/MiyuSQL en amont), MiyuWeb ne lit pas la base directement.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Reference Outils | [MiyuWeb - Reference Outils](../../MiyuWeb%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
