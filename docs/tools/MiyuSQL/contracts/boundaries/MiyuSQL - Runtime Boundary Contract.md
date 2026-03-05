# MiyuSQL â€” Runtime Boundary Contract

## 1. Contexte

Ce document definit le **bornage (frontieres d'execution)** du kit MiyuSQL. Il etablit ce que MiyuSQL ne fait jamais, les frontieres avec les Cores, et les invariants de limite. MiyuSQL est un Kit d'Outils qui orchestre des capacites atomiques sans logique metier ni decision.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Portee / Scope

Ce document definit :
- Ce que MiyuSQL ne fait jamais (pas de logique metier, pas de decision, pas d'acces DB hors gouvernance)
- Les frontieres avec les Cores (KindMother, Master Butler, StrongFather, WorrySentinel, Caring Nanny)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- Les frontieres internes de KindMother (voir KindMother - Runtime Boundary & Enforcement Contract)
- L'implementation technique des Tools

---

## 3. Principe Fondamental

### 3.1 Bornage

> **MiyuSQL execute des capacites gouvernÃ©es. Il ne decide jamais, n'interprete pas le metier, et n'accede jamais a la base en dehors du flux gouvernÃ©.**

### 3.2 Ce que MiyuSQL ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de logique metier** â€” MiyuSQL n'interprete pas les donnees, ne choisit pas de schema, n'applique pas de regles metier. Il execute des requetes ou operations mandatees. |
| **BOUND-2** | **Pas de decision** â€” MiyuSQL ne decide pas si une action doit etre faite (ALLOW/DENY = StrongFather). Il execute uniquement ce qui a ete autorise. |
| **BOUND-3** | **Pas d'acces DB hors gouvernance** â€” MiyuSQL n'accede a la base que dans le cadre d'un appel mandate par la gouvernance (BondingBrother â†’ Cores â†’ KindMother). Aucun acces direct, bypass, ou connexion hors flux. |
| **BOUND-4** | **Pas de modification du contexte d'autorisation** â€” MiyuSQL ne modifie pas les permissions, ne cree pas de mandat, ne revoque rien. Il utilise le contexte fourni. |
| **BOUND-5** | **Pas de connaissance de l'Operateur appelant** â€” MiyuSQL ne connait pas l'identite metier de l'Operateur ; il reÃ§oit un contexte gouvernÃ© (permissions, niveau, instance). |
| **BOUND-6** | **Pas de capacite nouvelle** â€” MiyuSQL n'ajoute aucune capacite qui n'existe pas dans ses Tools composants. Il orchestre, n'invente pas. |

---

## 4. Frontieres avec les Cores

### 4.1 KindMother

| Frontier | Description |
|----------|-------------|
| **Autorite** | KindMother est l'autorite absolue sur les donnees. MiyuSQL execute sous autorite KindMother ; il n'applique une ecriture que si une WriteIntent a ete acceptee. |
| **Limite** | MiyuSQL ne valide pas les WriteIntent, ne decide pas de les accepter ou rejeter. Il execute l'execution mandatee par KindMother. |

### 4.2 Master Butler

| Frontier | Description |
|----------|-------------|
| **Catalogue** | Master Butler declare le Toolkit et les Tools. MiyuSQL n'enregistre pas lui-meme les Tools ; il est declare par l'environnement. |
| **Limite** | MiyuSQL ne gere pas les permissions ni le catalogue. Il est invoque apres verification Master Butler. |

### 4.3 StrongFather

| Frontier | Description |
|----------|-------------|
| **Decision** | StrongFather decide ALLOW ou DENY. MiyuSQL n'est invoque qu'en cas d'ALLOW. |
| **Limite** | MiyuSQL ne prend aucune decision strategique. Il n'emet pas de mandat, ne revoque rien. |

### 4.4 WorrySentinel et Caring Nanny

| Frontier | Description |
|----------|-------------|
| **Securite et etat** | WorrySentinel (niveau de securite) et Caring Nanny (etat systeme) sont verifies avant l'appel a MiyuSQL. |
| **Limite** | MiyuSQL ne modifie pas le niveau de securite ni l'etat systeme. Il n'est invoque que si les pre-conditions sont remplies. |

### 4.5 BondingBrother

| Frontier | Description |
|----------|-------------|
| **Mediation** | BondingBrother traduit l'intention et prepare le contexte. MiyuSQL reÃ§oit une demande deja mediee. |
| **Limite** | MiyuSQL ne medie pas les intentions ; il execute la requete ou l'operation fournie dans le contexte gouvernÃ©. |

---

## 5. Invariants de Limite

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucune execution d'ecriture sans WriteIntent acceptee par KindMother |
| **INV-BOUND-2** | Aucune execution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune connexion ou acces a la base en dehors du canal gouvernÃ© (KindMother / MiyuSQL mandate) |
| **INV-BOUND-4** | Aucune logique metier (choix de schema, regles, interpretation des donnees) dans les Tools MiyuSQL |
| **INV-BOUND-5** | Aucune decision ALLOW/DENY ou strategique dans MiyuSQL ; execution uniquement |
| **INV-BOUND-6** | Le Toolkit n'expose que les capacites de ses Tools composants ; pas de capacite nouvelle |

---

## 6. Reponses aux Violations

### 6.1 Comportement Attendu

Si une condition de bornage est violee (ex. appel sans gouvernance, ecriture sans WriteIntent), MiyuSQL ne doit pas executer. La reponse (rejet, erreur explicite) est geree par la couche gouvernance (BondingBrother / StrongFather / KindMother), pas par MiyuSQL lui-meme.

### 6.2 TraÃ§abilite

Toute tentative d'appel hors bornage doit etre tracÃ©e (observability, audit) selon les contrats KindMother et Caring Nanny ; MiyuSQL ne decide pas du contenu du trace, il peut fournir un signal d'echec au flux gouvernÃ©.

---

## 7. References Croisees

| Document | Lien |
|----------|------|
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../../MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| MiyuSQL - KindMother Integration Contract | [MiyuSQL - KindMother Integration Contract](../integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) |
| MiyuSQL - Tool Governance Compliance Contract | [MiyuSQL - Tool Governance Compliance Contract](../governance/MiyuSQL%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| KindMother - Runtime Boundary & Enforcement Contract | [KindMother - Runtime Boundary & Enforcement Contract](..//..//..//..//cores//KindMother//contracts//boundaries//KindMother%20-%20Runtime%20Boundary%20%26%20Enforcement%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de creation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Contrat de reference


