# MiyuAuth â€” Documentation Fondatrice

## 1. Contexte

**MiyuAuth** est le **kit d'outils (Toolkit)** d'identitÃ© utilisateur de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de rÃ©solution de rÃ´le (citoyen / visiteur / externe), d'attestation, de vÃ©rification Passeport Utilisateur et Visa de Connexion, alignÃ©s sur la Connexion Inter-COG et sur KindMother Identity & Cross-Domain Trust.

L'autoritÃ© sur la validation de la confiance inter-domaines appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuAuth expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (rÃ©solution, attestation, vÃ©rification, rÃ´le) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuAuth
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother et la Connexion Inter-COG
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (stockage identitÃ©, signatures)
- Toute dÃ©cision ALLOW/DENY ou autorisation mÃ©tier â€” celle-ci reste du ressort de StrongFather et des Cores

---

## 3. DÃ©finition canonique

> **MiyuAuth est une composition officielle d'outils d'identitÃ© utilisateur (rÃ©solution de rÃ´le, attestation, vÃ©rification Passeport/Visa), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuAuth **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuAuth **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (rÃ©soudre un contexte d'identitÃ©, attester, vÃ©rifier Passeport/Visa, exposer le rÃ´le) sans dÃ©cider de la confiance ni de l'autorisation.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `identity` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuAuth est composÃ© des Tools suivants (format canonique `tool.<domain>.<action>`). Le dÃ©tail de chaque outil (action, niveau de sÃ©curitÃ©, capability_id) est dÃ©crit dans [MiyuAuth - Reference Outils](./MiyuAuth%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.identity.resolve` | RÃ©sout un contexte d'identitÃ© (citoyen, visiteur, externe) Ã  partir des donnÃ©es fournies ; ne dÃ©cide pas de la confiance |
| `tool.identity.attest` | Produit une attestation d'identitÃ© pour un contexte validÃ© par KindMother |
| `tool.identity.verify` | VÃ©rifie un Passeport Utilisateur ou un Visa de Connexion (structure, signature) ; ne valide pas la confiance |
| `tool.identity.role` | Retourne le rÃ´le rÃ©solu (citoyen, visiteur, externe) pour un contexte d'identitÃ© gouvernÃ© |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuAuth en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : toute confiance utilisÃ©e pour l'identitÃ© est **validÃ©e par KindMother** (Identity & Cross-Domain Trust) ; MiyuAuth opÃ¨re sur donnÃ©es fournies dans le flux (pas d'accÃ¨s direct Ã  la base). Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 ou 3** selon politique identitÃ© (dÃ©tail dans [MiyuAuth - Security and States Contract](./contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md)) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother et Connexion Inter-COG

- **KindMother** est l'unique validateur de la confiance inter-domaines ([KindMother - Identity & Cross-Domain Trust Contract](..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md)). MiyuAuth exÃ©cute des capacitÃ©s (resolve, attest, verify, role) **sans dÃ©cider** de la confiance ; toute confiance utilisÃ©e pour l'identitÃ© est validÃ©e par KindMother.
- **Connexion Inter-COG** : MiyuAuth s'aligne sur les concepts Passeport Utilisateur, Visa de Connexion, COG HÃ©bergeur, COG Origine, Utilisateur Visiteur, Utilisateur Externe, citoyen. Les Tools `verify` et `role` opÃ¨rent sur ces concepts ; la dÃ©cision d'autorisation reste Ã  StrongFather et au COG HÃ©bergeur.

**RÃ©fÃ©rences :** [Miyukini Conceptual References - Connexion Inter-COG](..//..//miyukini-webway-system//reference//_index.md), [KindMother - Identity & Cross-Domain Trust Contract](..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md).

---

## 8bis. Relation avec MiyuSQL â€” DonnÃ©es d'identification, Passeport, Visa

MiyuAuth et **MiyuSQL** sont deux Toolkits distincts (Strate 6) ; leurs rÃ´les sont complÃ©mentaires et ne se recouvrent pas.

### 8bis.1 Qui manipule les donnÃ©es d'identification ?

| ResponsabilitÃ© | Acteur | Toolkit / Core |
|----------------|--------|-----------------|
| **AutoritÃ© sur les donnÃ©es** (dont identitÃ©, Passeport, Visa) | KindMother | Core Strate 4 |
| **Persistance : lecture / Ã©criture** (requÃªtes, transactions, cache) | KindMother mandate, **MiyuSQL** exÃ©cute | MiyuSQL (`tool.query.execute`, `tool.transaction.*`, `tool.cache.*`, `tool.schema.read`) |
| **RÃ©solution, attestation, vÃ©rification, rÃ´le** (sans persister ni lire en base) | **MiyuAuth** exÃ©cute sur des donnÃ©es fournies | MiyuAuth (`tool.identity.resolve`, `tool.identity.attest`, `tool.identity.verify`, `tool.identity.role`) |

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es, y compris les donnÃ©es d'identification, les Passeports Utilisateurs et les Visas de Connexion. Toute lecture ou Ã©criture en base (insert, update, select, transaction) est **sous autoritÃ© KindMother** et **exÃ©cutÃ©e via MiyuSQL** lorsque KindMother mandate l'opÃ©ration (WriteIntent pour les Ã©critures, mandat d'exÃ©cution pour les lectures).
- **MiyuAuth** ne persiste pas et ne lit pas les donnÃ©es d'identification en base. Il opÃ¨re sur des **donnÃ©es (contexte, artefacts Passeport/Visa) qui lui sont fournies** dans le flux gouvernÃ© â€” par exemple aprÃ¨s qu'elles aient Ã©tÃ© lues via MiyuSQL sous autoritÃ© KindMother, ou transmises dans la requÃªte (session, token, etc.). MiyuAuth exÃ©cute uniquement : rÃ©solution de contexte, attestation, vÃ©rification de structure/signature, dÃ©termination du rÃ´le.

### 8bis.2 Passeport Utilisateur et Visa de Connexion

| OpÃ©ration | Qui dÃ©cide / qui exÃ©cute | Toolkit impliquÃ© |
|-----------|---------------------------|-------------------|
| **Stockage** (crÃ©ation, mise Ã  jour, rÃ©vocation) d'un Passeport ou d'un Visa | KindMother (autoritÃ©) ; exÃ©cution en base via **MiyuSQL** (sous WriteIntent / mandat) | MiyuSQL |
| **Lecture** d'un Passeport ou Visa depuis la persistance | KindMother (autoritÃ©) ; exÃ©cution en base via **MiyuSQL** (mandat d'exÃ©cution) | MiyuSQL |
| **VÃ©rification** (structure, signature, validitÃ©) d'un artefact Passeport/Visa dÃ©jÃ  fourni | **MiyuAuth** (`tool.identity.verify`) â€” opÃ¨re sur l'artefact reÃ§u, pas sur la base | MiyuAuth |
| **RÃ©solution** du rÃ´le (citoyen / visiteur / externe) Ã  partir d'un contexte fourni | **MiyuAuth** (`tool.identity.resolve`, `tool.identity.role`) | MiyuAuth |

En rÃ©sumÃ© : **la manipulation des donnÃ©es** (CRUD, persistance) des Passeports et Visas est du ressort de **KindMother + MiyuSQL**. **L'utilisation de ces donnÃ©es** (vÃ©rifier, rÃ©soudre, attester, rÃ´le) est du ressort de **MiyuAuth** sur des entrÃ©es dÃ©jÃ  fournies dans le flux.

### 8bis.3 Flux typique (lecture puis vÃ©rification)

1. Un OpÃ©rateur ou un COG a besoin d'un Passeport ou Visa (ex. pour une visite inter-COG).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. rÃ©cupÃ©rer un Visa par identifiant).
3. **MiyuSQL** exÃ©cute la requÃªte (ex. `tool.query.execute` SELECT) sous autoritÃ© KindMother et retourne les donnÃ©es Ã  KindMother / au flux.
4. Le flux fournit l'artefact (Passeport/Visa) Ã  **MiyuAuth** pour **vÃ©rification** (`tool.identity.verify`) ou **rÃ©solution de rÃ´le** (`tool.identity.resolve`, `tool.identity.role`).
5. MiyuAuth retourne le rÃ©sultat (valide/invalide, rÃ´le, etc.) sans accÃ©der lui-mÃªme Ã  la base.

MiyuAuth **ne dÃ©pend pas** de MiyuSQL (pas d'appel direct) ; la relation est **indirecte** via KindMother et le flux gouvernÃ© : les donnÃ©es persistÃ©es ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuAuth peut Ãªtre invoquÃ© ensuite quand elles sont fournies en entrÃ©e.

**RÃ©fÃ©rence :** [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md), [MiyuSQL - KindMother Integration Contract](../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuAuth - Tool Governance Compliance Contract](./contracts/governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuAuth sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `identity` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« identity Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuAuth est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuAuth devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md). La documentation ne gÃ©nÃ¨re pas les fichiers `mscm_index/*` ; elle dÃ©finit les concepts pour une indexation future.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](..//..//miyukini-webway-system//reference//_index.md) |
| KindMother - Identity & Cross-Domain Trust | [KindMother - Identity & Cross-Domain Trust Contract](..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| SouverainetÃ© Environnement | [Miyukini Conceptual References - SouverainetÃ© Environnement](..//..//miyukini-webway-system//reference//_index.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


