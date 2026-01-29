# MiyuSQL — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un developpeur pour implementer MiyuSQL conformement aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pedagogique :** Aider a traduire les contrats MiyuSQL en logique d'implementation (Tools, gouvernance, KindMother, bornage).

**Avertissement :** Ce document ne cree aucune nouvelle regle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implementer le kit MiyuSQL (Tools query, transaction, cache, schema) de maniere conforme aux contrats : Documentation Fondatrice, KindMother Integration, Tool Governance Compliance, Security and States, Runtime Boundary, Dependencies.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne definit pas de nouvelles regles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprehension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuSQL - Documentation Fondatrice** : Identite, ToolkitId, liste des Tools, gouvernance
- **MiyuSQL - KindMother Integration Contract** : WriteIntent obligatoire, execution sous autorite KindMother
- **MiyuSQL - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities
- **MiyuSQL - Security and States Contract** : Niveau 2, etats autorises/interdits
- **MiyuSQL - Runtime Boundary Contract** : Bornage, interdictions (BOUND-*), invariants INV-BOUND-*
- **MiyuSQL - Dependencies Contract** : Liste fermee des dependances, ordre d'invocation
- **Master Butler - Tool Governance Contract** et **Toolkit Composition Contract** : Format ToolId, structure Toolkit

---

## 2. Principes a respecter absolument

### 2.1 Pas de logique metier (BOUND-1)

**Principe contractuel :** MiyuSQL n'interprete pas les donnees, ne choisit pas de schema, n'applique pas de regles metier.

**Traduction en implementation :**

- Les Tools reçoivent une requete (ou parametres) deja formee ; ils n'interpretent pas le contenu metier.
- Aucune regle applicative (validation metier, choix de table, regle de calcul) ne doit etre codee dans MiyuSQL.
- Les requetes sont executees telles que mandatees par KindMother ; pas de transformation metier.

### 2.2 Pas de decision (BOUND-2)

**Principe contractuel :** MiyuSQL ne decide pas si une action doit etre faite ; StrongFather decide ALLOW/DENY.

**Traduction en implementation :**

- MiyuSQL est invoque uniquement apres decision ALLOW de la gouvernance. L'implementation ne doit pas re-evaluer les permissions.
- En cas d'appel hors gouvernance (anomalie), MiyuSQL doit refuser l'execution et signaler (pas de decision de contournement).

### 2.3 Pas d'acces DB hors gouvernance (BOUND-3)

**Principe contractuel :** MiyuSQL n'accede a la base que dans le cadre d'un appel mandate par la gouvernance (KindMother).

**Traduction en implementation :**

- Toute connexion ou execution SQL doit etre declenchee par un flux venant de KindMother (ou du canal gouverné), pas par un appel direct externe.
- Pas de connexion « sauvage » ; pool ou connexion dediee sous controle du flux gouverné.
- Les drivers SQL (PostgreSQL, etc.) sont des dependances techniques ; l'acces reste toujours via le canal gouverné.

### 2.4 WriteIntent obligatoire pour ecritures (INV-KM-2)

**Principe contractuel :** Toute ecriture passe par une WriteIntent validee par KindMother.

**Traduction en implementation :**

- L'implementation des Tools d'ecriture (tool.query.execute pour INSERT/UPDATE/DELETE/DDL) ne doit pas executer sans avoir reçu un mandat d'execution lie a une WriteIntent acceptee.
- L'interface entre KindMother et MiyuSQL doit garantir que l'appel a MiyuSQL n'a lieu qu'apres validation de la WriteIntent (etat ACCEPTEE puis mandat d'execution).

### 2.5 Liste fermee des dependances (INV-DEP-*)

**Principe contractuel :** MiyuSQL ne depend que des Cores et du Kernel definis dans le Dependencies Contract.

**Traduction en implementation :**

- Aucune dependance vers un Operateur, un produit, ou une regle metier.
- Les appels entrants passent par BondingBrother et la chaine de gouvernance ; MiyuSQL n'expose pas d'API publique directe aux Operateurs.
- Usage du Kernel (Id, Logger, Clock, Config, Lifecycle) pour identifiants, logs, horodatage, configuration locale, cycle de vie — sans logique metier.

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implementation |
|------|--------------|----------------|
| **BOUND-1** | Pas de logique metier | Pas de code qui interprete le contenu des requetes ou des donnees |
| **BOUND-2** | Pas de decision | Pas de code qui evalue ALLOW/DENY ; execution uniquement sur mandat |
| **BOUND-3** | Pas d'acces DB hors gouvernance | Connexion/execution uniquement depuis le canal gouverné (KindMother) |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte ; pas de revocation, pas de creation de mandat |
| **BOUND-5** | Pas de connaissance de l'Operateur appelant | Pas d'identite Operateur dans la logique Tool ; contexte anonymise (permissions, niveau) |
| **BOUND-6** | Pas de capacite nouvelle | Chaque Tool correspond exactement a un ToolId declare ; pas d'extension non declaree |

---

## 4. Patterns recommandes

### 4.1 Structure des Tools

- Chaque ToolId est implemente comme une unite d'execution atomique : entree (contexte gouverné, parametres), sortie (resultat ou erreur contractuelle).
- Pas d'etat metier partage entre appels ; etat technique (pool, cache) sous controle du flux gouverné.

### 4.2 Interface avec KindMother

- L'implementation MiyuSQL expose une surface d'appel consommee par KindMother (ou par le composant qui execute sous autorite KindMother). Les parametres incluent : type d'operation (query, transaction, cache, schema), parametres de la requete ou de l'operation, contexte (sans identite Operateur metier).
- La reponse inclut : succes/echec, resultat (lignes, count, metadonnees), ou erreur explicite (sans fuite d'information sensible).

### 4.3 Gestion des erreurs

- Les erreurs techniques (timeout, connexion, syntaxe SQL) sont remontees de maniere explicite sans exposer de donnees metier.
- En cas de violation de bornage (ex. appel sans mandat), refus d'execution et signal (observability) ; pas d'execution partielle.

### 4.4 Traçabilite

- Utiliser le Logger du Kernel pour tracer les executions (sans contenu metier sensible). Conformite aux contrats KindMother Observability et MiyuSQL Runtime Boundary.

---

## 5. Alignement MIP / MSCM

### 5.1 MIP v1

A l'implementation, le code fournissant les Tools MiyuSQL doit etre balise MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit genere selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : `data` (coherent avec domains.json).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool MiyuSQL est une unite logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json.

### 5.2 MSCM

Les blocs de code correspondant aux Tools doivent etre balises selon le standard MSCM (Miyukini Semantic Code Markup) pour permettre l'indexation et la gouvernance structurelle.

---

## 6. Tests (rappel)

- **Tests unitaires** : Conformement au [MiyuSQL - Unit Tests Contract](../contracts/testing/MiyuSQL%20-%20Unit%20Tests%20Contract.md) — pas de modification de donnees metier ; sandbox ou table MiyukiniSQLtest avec nettoyage.
- **Test de cycle MiyukiniSQLtest** : Conformement au [MiyuSQL - Cycle Tests Contract](../contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md) — scenario E2E (WriteIntent → validations Cores → table → colonne → donnee → lecture → affichage → suppression). Executable par MiyukiniAdmin.

---

## 7. References croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| MiyuSQL - Runtime Boundary Contract | [MiyuSQL - Runtime Boundary Contract](../contracts/boundaries/MiyuSQL%20-%20Runtime%20Boundary%20Contract.md) |
| MiyuSQL - Dependencies Contract | [MiyuSQL - Dependencies Contract](../dependencies/MiyuSQL%20-%20Dependencies%20Contract.md) |
| MiyuSQL - Unit Tests Contract | [MiyuSQL - Unit Tests Contract](../contracts/testing/MiyuSQL%20-%20Unit%20Tests%20Contract.md) |
| MiyuSQL - Cycle Tests Contract | [MiyuSQL - Cycle Tests Contract](../contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
