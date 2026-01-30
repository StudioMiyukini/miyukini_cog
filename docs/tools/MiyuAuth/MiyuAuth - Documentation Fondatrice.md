# MiyuAuth — Documentation Fondatrice

## 1. Contexte

**MiyuAuth** est le **kit d'outils (Toolkit)** d'identité utilisateur de l'écosystème Miyukini. Il intègre les outils de résolution de rôle (citoyen / visiteur / externe), d'attestation, de vérification Passeport Utilisateur et Visa de Connexion, alignés sur la Connexion Inter-COG et sur KindMother Identity & Cross-Domain Trust.

L'autorité sur la validation de la confiance inter-domaines appartient à **KindMother** (Core de données, Strate 4). MiyuAuth expose des capacités d'exécution gouvernée (résolution, attestation, vérification, rôle) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuAuth
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother et la Connexion Inter-COG
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (stockage identité, signatures)
- Toute décision ALLOW/DENY ou autorisation métier — celle-ci reste du ressort de StrongFather et des Cores

---

## 3. Définition canonique

> **MiyuAuth est une composition officielle d'outils d'identité utilisateur (résolution de rôle, attestation, vérification Passeport/Visa), déclarée et gouvernée par l'environnement.**

- MiyuAuth **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuAuth **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (résoudre un contexte d'identité, attester, vérifier Passeport/Visa, exposer le rôle) sans décider de la confiance ni de l'autorisation.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute confiance utilisée pour l'identité est validée par KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.identity.miyauth` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `identity` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuAuth est composé des Tools suivants (format canonique `tool.<domain>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) est décrit dans [MiyuAuth - Reference Outils](./MiyuAuth%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.identity.resolve` | Résout un contexte d'identité (citoyen, visiteur, externe) à partir des données fournies ; ne décide pas de la confiance |
| `tool.identity.attest` | Produit une attestation d'identité pour un contexte validé par KindMother |
| `tool.identity.verify` | Vérifie un Passeport Utilisateur ou un Visa de Connexion (structure, signature) ; ne valide pas la confiance |
| `tool.identity.role` | Retourne le rôle résolu (citoyen, visiteur, externe) pour un contexte d'identité gouverné |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuAuth en contient quatre.

---

## 6. Gouvernance

Tout appel à un outil du kit MiyuAuth (ou au kit lui-même) suit le flux de gouvernance suivant :

1. **Opérateur** (Strate 7) — demande d'utilisation d'un Tool ou du Toolkit
2. **BondingBrother** — médiation, traduction de l'intention, préparation du contexte
3. **Master Butler** — vérification de l'existence du Tool/Toolkit, permissions requises, niveau de sécurité
4. **WorrySentinel** — vérification que le niveau de sécurité actuel permet cet appel
5. **Caring Nanny** — vérification que l'état système (HEALTHY, DEGRADED, etc.) permet cet appel
6. **StrongFather** — décision finale ALLOW ou DENY
7. **Exécution** — si autorisé, le Tool (ou les Tools du Toolkit) exécute l'action ; toute confiance utilisée pour l'identité est **validée par KindMother** (Identity & Cross-Domain Trust).

Le Toolkit MiyuAuth est **déclaré** dans Master Butler et **compatibilisé** par Ever Buddy (cycle de vie, versions des Outils) selon le [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md).

**Référence :** [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (schéma de flux complet).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 ou 3** selon politique identité (détail dans [MiyuAuth - Security and States Contract](./contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md)) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother et Connexion Inter-COG

- **KindMother** est l'unique validateur de la confiance inter-domaines ([KindMother - Identity & Cross-Domain Trust Contract](../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md)). MiyuAuth exécute des capacités (resolve, attest, verify, role) **sans décider** de la confiance ; toute confiance utilisée pour l'identité est validée par KindMother.
- **Connexion Inter-COG** : MiyuAuth s'aligne sur les concepts Passeport Utilisateur, Visa de Connexion, COG Hébergeur, COG Origine, Utilisateur Visiteur, Utilisateur Externe, citoyen. Les Tools `verify` et `role` opèrent sur ces concepts ; la décision d'autorisation reste à StrongFather et au COG Hébergeur.

**Références :** [Miyukini Conceptual References - Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md), [KindMother - Identity & Cross-Domain Trust Contract](../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md).

---

## 8bis. Relation avec MiyuSQL — Données d'identification, Passeport, Visa

MiyuAuth et **MiyuSQL** sont deux Toolkits distincts (Strate 6) ; leurs rôles sont complémentaires et ne se recouvrent pas.

### 8bis.1 Qui manipule les données d'identification ?

| Responsabilité | Acteur | Toolkit / Core |
|----------------|--------|-----------------|
| **Autorité sur les données** (dont identité, Passeport, Visa) | KindMother | Core Strate 4 |
| **Persistance : lecture / écriture** (requêtes, transactions, cache) | KindMother mandate, **MiyuSQL** exécute | MiyuSQL (`tool.query.execute`, `tool.transaction.*`, `tool.cache.*`, `tool.schema.read`) |
| **Résolution, attestation, vérification, rôle** (sans persister ni lire en base) | **MiyuAuth** exécute sur des données fournies | MiyuAuth (`tool.identity.resolve`, `tool.identity.attest`, `tool.identity.verify`, `tool.identity.role`) |

- **KindMother** est l'autorité sur toutes les données, y compris les données d'identification, les Passeports Utilisateurs et les Visas de Connexion. Toute lecture ou écriture en base (insert, update, select, transaction) est **sous autorité KindMother** et **exécutée via MiyuSQL** lorsque KindMother mandate l'opération (WriteIntent pour les écritures, mandat d'exécution pour les lectures).
- **MiyuAuth** ne persiste pas et ne lit pas les données d'identification en base. Il opère sur des **données (contexte, artefacts Passeport/Visa) qui lui sont fournies** dans le flux gouverné — par exemple après qu'elles aient été lues via MiyuSQL sous autorité KindMother, ou transmises dans la requête (session, token, etc.). MiyuAuth exécute uniquement : résolution de contexte, attestation, vérification de structure/signature, détermination du rôle.

### 8bis.2 Passeport Utilisateur et Visa de Connexion

| Opération | Qui décide / qui exécute | Toolkit impliqué |
|-----------|---------------------------|-------------------|
| **Stockage** (création, mise à jour, révocation) d'un Passeport ou d'un Visa | KindMother (autorité) ; exécution en base via **MiyuSQL** (sous WriteIntent / mandat) | MiyuSQL |
| **Lecture** d'un Passeport ou Visa depuis la persistance | KindMother (autorité) ; exécution en base via **MiyuSQL** (mandat d'exécution) | MiyuSQL |
| **Vérification** (structure, signature, validité) d'un artefact Passeport/Visa déjà fourni | **MiyuAuth** (`tool.identity.verify`) — opère sur l'artefact reçu, pas sur la base | MiyuAuth |
| **Résolution** du rôle (citoyen / visiteur / externe) à partir d'un contexte fourni | **MiyuAuth** (`tool.identity.resolve`, `tool.identity.role`) | MiyuAuth |

En résumé : **la manipulation des données** (CRUD, persistance) des Passeports et Visas est du ressort de **KindMother + MiyuSQL**. **L'utilisation de ces données** (vérifier, résoudre, attester, rôle) est du ressort de **MiyuAuth** sur des entrées déjà fournies dans le flux.

### 8bis.3 Flux typique (lecture puis vérification)

1. Un Opérateur ou un COG a besoin d'un Passeport ou Visa (ex. pour une visite inter-COG).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. récupérer un Visa par identifiant).
3. **MiyuSQL** exécute la requête (ex. `tool.query.execute` SELECT) sous autorité KindMother et retourne les données à KindMother / au flux.
4. Le flux fournit l'artefact (Passeport/Visa) à **MiyuAuth** pour **vérification** (`tool.identity.verify`) ou **résolution de rôle** (`tool.identity.resolve`, `tool.identity.role`).
5. MiyuAuth retourne le résultat (valide/invalide, rôle, etc.) sans accéder lui-même à la base.

MiyuAuth **ne dépend pas** de MiyuSQL (pas d'appel direct) ; la relation est **indirecte** via KindMother et le flux gouverné : les données persistées ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuAuth peut être invoqué ensuite quand elles sont fournies en entrée.

**Référence :** [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md), [MiyuSQL - KindMother Integration Contract](../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuAuth sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `identity` — cohérent avec la projection domains.json (blocs du domaine « identity »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuAuth est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

À l'implémentation, le code fournissant les Tools MiyuAuth devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md). La documentation ne génère pas les fichiers `mscm_index/*` ; elle définit les concepts pour une indexation future.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |
| KindMother - Identity & Cross-Domain Trust | [KindMother - Identity & Cross-Domain Trust Contract](../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Souveraineté Environnement | [Miyukini Conceptual References - Souveraineté Environnement](../../reference/Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
