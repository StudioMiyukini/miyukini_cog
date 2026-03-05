# MiyuSQL â€” Dependencies Contract

## 1. Contexte

Ce document definit le contrat des **dependances** du kit MiyuSQL. Il etablit la liste fermee des dependances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), l'absence de dependance metier, et l'ordre ou les contraintes d'utilisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Portee / Scope

Ce document definit :
- La liste fermee des dependances de MiyuSQL (Cores et Kernel)
- L'absence de dependance metier (Operateurs, produits, regles metier)
- L'ordre et les contraintes (flux d'appel, pre-conditions)
- Les invariants de dependance

Ce document **ne couvre pas** :
- Les dependances d'implementation (driver SQL, librairies techniques) â€” hors scope documentaire fondateur
- Les dependances des Cores eux-memes

---

## 3. Principe Fondamental

### 3.1 Liste Fermee

> **MiyuSQL ne depend que des Cores et du Kernel definis dans ce contrat. Aucune dependance metier (Operateur, produit, regle metier) n'est autorisee.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-DEP-1** | MiyuSQL ne connait pas les Operateurs ; il reÃ§oit un contexte gouverne (BondingBrother) |
| **INV-DEP-2** | MiyuSQL ne depend d'aucun produit ni regle metier applicative |
| **INV-DEP-3** | Toute invocation de MiyuSQL passe par la mediation BondingBrother et la gouvernance (Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-DEP-4** | Les operations de donnees sont executees sous autorite KindMother uniquement |

---

## 4. Liste Fermee des Dependances

### 4.1 Cores (Strate 4)

| DÃ©pendance | RÃ´le pour MiyuSQL | Contrainte |
|------------|-------------------|------------|
| **KindMother** | Autorite sur les donnees ; validation WriteIntent ; application des ecritures ; execution mandatee des requetes/transactions | MiyuSQL n'accede a la base que via KindMother ; aucune ecriture sans WriteIntent acceptee |
| **Master Butler** | Catalogue des Tools et Toolkits ; permissions ; declaration de MiyuSQL et des ToolIds | MiyuSQL est invoque apres verification Master Butler |
| **StrongFather** | Decision ALLOW/DENY pour l'utilisation des Tools | MiyuSQL n'est invoque qu'en cas d'ALLOW |
| **WorrySentinel** | Niveau de securite ; verification que le niveau actuel permet l'appel | Pre-condition a l'invocation |
| **Caring Nanny** | Etat systeme ; verification que l'etat (HEALTHY, DEGRADED, etc.) permet l'appel | Pre-condition a l'invocation |

### 4.2 Interface & Mediation (Strate 5)

| DÃ©pendance | RÃ´le pour MiyuSQL | Contrainte |
|------------|-------------------|------------|
| **BondingBrother** | Mediation ; traduction de l'intention ; preparation du contexte ; passage des requetes vers les Cores | MiyuSQL reÃ§oit les demandes via BondingBrother (ou via le flux gouvernÃ© initie par BondingBrother) |

### 4.3 Kernel (Strate K)

| DÃ©pendance | RÃ´le pour MiyuSQL | Contrainte |
|------------|-------------------|------------|
| **Kernel** | Id (identifiants), Logger (traÃ§abilite), Clock (horodatage), Config (configuration locale), Lifecycle | Usage minimal et neutre ; pas de logique metier ; conformite aux invariants Kernel |

---

## 5. Ordre et Contraintes

### 5.1 Flux d'Invocation (Ordre)

L'ordre d'implication des dependances lors d'un appel a un Tool MiyuSQL est :

1. **Operateur** (hors dependance MiyuSQL) emet une intention.
2. **BondingBrother** â€” mediation, traduction, contexte.
3. **Master Butler** â€” verification Tool/Toolkit, permissions.
4. **WorrySentinel** â€” niveau de securite.
5. **Caring Nanny** â€” etat systeme.
6. **StrongFather** â€” decision ALLOW/DENY.
7. Si ALLOW : **KindMother** â€” validation WriteIntent (si ecriture), mandat d'execution.
8. **MiyuSQL** â€” execution du Tool mandate.
9. **KindMother** â€” application effective (persistance) si ecriture.

### 5.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Pas d'invocation directe** | MiyuSQL n'est jamais invoque directement par un Operateur ; toujours via BondingBrother et la chaine de gouvernance |
| **Pas de bypass** | Aucune dependance ne peut etre contournee (pas d'acces direct a la base sans KindMother, pas d'execution sans StrongFather ALLOW) |
| **Pas de dependance inverse metier** | Aucun Core ni Kernel ne depend de MiyuSQL pour sa logique metier ; MiyuSQL est un outil consomme par le flux |

---

## 6. Absence de DÃ©pendance Metier

### 6.1 Ce dont MiyuSQL ne depend pas

| Type | Exemples | Raison |
|------|----------|--------|
| **Operateurs** | MiyukiniAdmin, tout Operateur de domaine | MiyuSQL est un Toolkit ; les Operateurs utilisent MiyuSQL via la gouvernance |
| **Produits / Regles metier** | Schemas applicatifs, regles metier | MiyuSQL n'interprete pas le metier ; il execute des requetes mandatees |
| **Autres Toolkits** | Kits d'outils metier | MiyuSQL est independant des autres Toolkits ; pas de couplage fonctionnel |
| **Services externes** | APIs externes, reseau metier | Conformite LOI-1 ; pas de dependance externe critique |

### 6.2 DÃ©pendances Techniques (Hors Scope Contractuel)

Les dependances techniques (driver SQL, pool de connexions, librairies) sont hors scope de ce contrat fondateur. Elles seront definies dans le guide d'implementation (MiyuSQL - Reference Implementation Guidelines) et doivent rester neutres (pas de logique metier).

---

## 7. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| MiyuSQL - Tool Governance Compliance Contract | [MiyuSQL - Tool Governance Compliance Contract](../contracts/governance/MiyuSQL%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| MiyuSQL - Runtime Boundary Contract | [MiyuSQL - Runtime Boundary Contract](../contracts/boundaries/MiyuSQL%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference

