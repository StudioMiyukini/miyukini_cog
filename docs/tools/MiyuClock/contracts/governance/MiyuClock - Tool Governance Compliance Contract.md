# MiyuClock â€” Tool Governance Compliance Contract

## 1. Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

Ce document dÃ©finit la **conformitÃ© de MiyuClock** aux contrats de gouvernance des Outils et Kits d'Outils de Master Butler. MiyuClock est un **Kit d'Outils (Toolkit)** dÃ©clarÃ© et gouvernÃ© par l'environnement ; ce contrat Ã©tablit la dÃ©claration formelle du ToolkitId, des ToolIds composants, et des capabilities associÃ©es.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- La conformitÃ© au [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)
- La conformitÃ© au [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)
- La dÃ©claration formelle du ToolkitId et des ToolIds MiyuClock
- Le catalogue des capabilities exposÃ©es

Ce document **ne couvre pas** :
- L'implÃ©mentation technique des Tools
- Les contrats MiyuClock hors gouvernance (sÃ©curitÃ©, bornage, intÃ©gration KindMother)

---

## 3. ConformitÃ© au Tool Governance Contract

### 3.1 Principes respectÃ©s

| Principe Master Butler | Application MiyuClock |
|------------------------|---------------------|
| Tout Tool possÃ¨de un ToolId unique et immuable | Chaque outil MiyuClock a un ToolId au format `tool.<domain>.<action>` |
| Tout Tool est liÃ© Ã  exactement une Capability | Chaque ToolId est associÃ© Ã  un capability_id (voir section 5) |
| Un Tool ne prend jamais de dÃ©cision mÃ©tier | Les Tools MiyuClock exÃ©cutent uniquement ; pas de dÃ©cision, pas de persistance |
| Un Tool ne connaÃ®t jamais l'OpÃ©rateur appelant | MiyuClock reÃ§oit un contexte gouvernÃ© ; pas d'identitÃ© OpÃ©rateur dans la logique Tool |

### 3.2 Format ToolId

Format canonique : `tool.<domain>.<action>`

- **PrÃ©fixe :** `tool.`
- **Domaine MiyuClock :** `time`
- **Segments :** minuscules, sans accents, sÃ©parÃ©s par des points

---

## 4. ConformitÃ© au Toolkit Composition Contract

### 4.1 Principes respectÃ©s

| Principe Toolkit | Application MiyuClock |
|------------------|---------------------|
| Un Toolkit agrÃ¨ge des Tools existants | MiyuClock regroupe deux Tools dÃ©clarÃ©s individuellement dans le catalogue |
| Un Toolkit n'ajoute aucune capacitÃ© nouvelle | MiyuClock n'expose que les capacitÃ©s de ses Tools composants |
| Un Toolkit est dÃ©clarÃ© et validÃ© par l'environnement | MiyuClock est dÃ©clarÃ© dans Master Butler avec ToolkitId `toolkit.time.miyuclock` |
| Tout Toolkit contient au moins deux Tools | MiyuClock contient deux Tools |

### 4.2 Structure formelle du Toolkit MiyuClock

| Champ | Valeur |
|-------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Format ToolkitId** | `toolkit.<domain>.<name>` |
| **Domaine** | `time` |
| **Name** | `miyuclock` |
| **Tools** | Ensemble des deux ToolIds (voir section 5) |
| **security_level** | 0 ou 1 (selon politique ; voir [MiyuClock - Security and States Contract](../security/MiyuClock%20-%20Security%20and%20States%20Contract.md)) |
| **allowed_states** | HEALTHY, DEGRADED |
| **disallowed_states** | SECURITY_LOCKDOWN, MAINTENANCE |
| **status** | Active |

---

## 5. DÃ©claration ToolkitId et ToolIds

### 5.1 ToolkitId

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **ToolkitId** | `toolkit.time.miyuclock` |
| **Nom lisible** | MiyuClock |
| **Description** | Kit d'outils de mesure du temps (instant prÃ©sent, delta entre instants) ; horloge locale, pas de temps global (LOI-4) |

### 5.2 Liste des ToolIds composants

| ToolId | capability_id (ex.) | Description courte |
|--------|---------------------|---------------------|
| `tool.time.now` | `time.now` | Retourne l'instant prÃ©sent (rÃ©fÃ©rence locale ; pas de timezone imposÃ©e) |
| `tool.time.delta` | `time.delta` | Retourne la durÃ©e Ã©coulÃ©e entre deux instants fournis dans le flux (t_prev, t_now) |

### 5.3 Invariants de dÃ©claration

| Code | Invariant |
|------|-----------|
| **INV-DECL-1** | Le ToolkitId `toolkit.time.miyuclock` est unique dans le catalogue Master Butler |
| **INV-DECL-2** | Chaque ToolId listÃ© est dÃ©clarÃ© individuellement dans le catalogue avant d'Ãªtre associÃ© au Toolkit |
| **INV-DECL-3** | Le niveau de sÃ©curitÃ© du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools (0 ou 1 selon politique) |
| **INV-DECL-4** | Aucun Tool hors domaine time n'est ajoutÃ© au Toolkit MiyuClock sans rÃ©vision contractuelle |

---

## 6. Catalogue et utilisation

### 6.1 Enregistrement

- Master Butler dÃ©clare le Toolkit MiyuClock et la liste des ToolIds composants.
- Toute utilisation du Toolkit ou d'un de ses Tools passe par le catalogue et la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather).

### 6.2 RÃ©solution

- Un OpÃ©rateur (ou adaptateur) demande l'utilisation d'un Tool ou du Toolkit via BondingBrother.
- Master Butler vÃ©rifie l'existence du Tool/Toolkit, les permissions requises, et le niveau de sÃ©curitÃ©.
- En cas d'autorisation (StrongFather ALLOW), l'exÃ©cution est dÃ©lÃ©guÃ©e ; la mesure du temps s'appuie sur le Kernel (Clock) ; MiyuClock ne persiste pas.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Reference Outils | [MiyuClock - Reference Outils](../../MiyuClock%20-%20Reference%20Outils.md) |
| Master Butler - Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Master Butler - Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence



