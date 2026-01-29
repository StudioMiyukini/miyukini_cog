# TAMR - Authority Limits Contract

## 1. Introduction

### Objet du contrat

Ce document definit le **TAMR - Authority Limits Contract** : un contrat normatif, non negociable, et de statut FONDATION qui etablit les **limites d'autorite humaine** et les **restrictions contextuelles** applicables aux interventions humaines dans le Miyukini Core System.

Les limites d'autorite definissent ce que l'humain peut et ne peut pas faire lors d'une intervention, selon le type d'intervention, le contexte, et les regles en vigueur. Elles se distinguent des **limites infranchissables** (definies dans le [TAMR - Inviolable Limits Contract](./TAMR%20-%20Inviolable%20Limits%20Contract.md)), qui ne peuvent jamais etre depassees, meme par un override.

### Contexte

TAMR (The Authority Must Rest) est le Human Interaction Core du Miyukini Core System. Il definit ou, quand, et comment l'humain intervient. Les limites d'autorite precisent **jusqu'ou** l'autorite humaine s'etend dans un contexte donne : quelles actions sont permises, quelles restrictions s'appliquent, et comment le contexte (niveau de securite, niveau de confiance, role, point d'intervention) modifie ces limites.

Sans limites d'autorite explicites, les interventions humaines pourraient etre incoherentes, excessives, ou insuffisantes selon le contexte. Ce contrat fixe le cadre conceptuel des restrictions contextuelles.

### Portee / Scope

Ce contrat s'applique a **toutes les interventions humaines dans le systeme Miyukini** et definit de maniere absolue :

- la nature conceptuelle des limites d'autorite (distinctes des limites infranchissables),
- les limites par type d'intervention (Approval, Override, Escalation, Supervision),
- les restrictions contextuelles (niveau de securite 0-4, niveau de confiance T0-T4, role, point d'intervention),
- les regles d'evaluation des limites (qui evalue, sur quels criteres),
- les invariants associes aux limites d'autorite,
- la relation avec StrongFather (evaluation) et avec le contrat Inviolable Limits (frontiere absolue).

Ce contrat **ne couvre pas** :

- les limites infranchissables (voir [TAMR - Inviolable Limits Contract](./TAMR%20-%20Inviolable%20Limits%20Contract.md)),
- les types d'intervention (voir [TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)),
- les points d'intervention (voir [TAMR - Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)),
- la decision d'autoriser ou refuser une intervention (responsabilite StrongFather).

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il etablit des regles absolues qui ne peuvent etre contournees, negociees, ou modifiees. Le contrat prime sur toute consideration pratique.

### Relation avec les autres contrats

Ce contrat complete et respecte les documents contractuels existants :

- **[TAMR - Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Definition philosophique des limites d'autorite (section 4, 5, 7)
- **[TAMR - Inviolable Limits Contract](./TAMR%20-%20Inviolable%20Limits%20Contract.md)** : Limites que meme un override ne peut franchir ; les limites d'autorite s'appliquent **en deca** de ces frontieres
- **[TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Types d'intervention auxquels s'appliquent les limites
- **[TAMR - Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)** : Points ou les limites s'appliquent
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie officielle (limite d'autorite, intervenant, contexte)
- **[Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)** : Principes de securite
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformite LOI-1 a LOI-6
- **[Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Niveaux T0-T4 (contexte de confiance)
- **[Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux 0-4 (contexte de securite)

---

## 2. Definition des limites d'autorite

### 2.1 Definition canonique

> **Une limite d'autorite est une restriction conceptuelle sur ce que l'humain peut faire lors d'une intervention, definie explicitement, evaluee selon le contexte, et applicable sans depasser les limites infranchissables.**

Les limites d'autorite sont :

- **Explicites** : Definies clairement, jamais implicites.
- **Contextuelles** : Leur application depend du contexte (type d'intervention, niveau de securite, niveau de confiance, role, point d'intervention).
- **Evaluables** : StrongFather peut evaluer si une intervention respecte ou non ces limites.
- **Protectrices** : Elles protegent le systeme et l'humain contre les interventions inappropriées.
- **Non absolues (pour ce contrat)** : Elles peuvent etre assouplies ou renforcees par le contexte ; seules les limites infranchissables sont absolues.

### 2.2 Distinction : limites d'autorite vs limites infranchissables

| Critere | Limites d'autorite (ce contrat) | Limites infranchissables |
|---------|----------------------------------|---------------------------|
| **Nature** | Restrictions contextuelles | Frontieres absolues |
| **Modifiable par contexte** | Oui (niveau securite, confiance, role) | Non |
| **Franchissable par override** | Non par defaut, mais le contexte peut autoriser des derogations controlees | Jamais |
| **Qui definit** | TAMR (conceptuel) | TAMR (conceptuel) |
| **Qui evalue** | StrongFather | StrongFather |
| **Exemples** | "En niveau 2, l'override necessite une double validation" | "Aucune suppression de donnees d'audit" |

**INV-AL-1 : Hierarchie des limites**

Les limites d'autorite s'appliquent **strictement a l'interieur** du domaine defini par les limites infranchissables. Une intervention qui franchirait une limite infranchissable est invalide quelle que soit l'autorite contextuelle.

---

## 3. Limites par type d'intervention

Les limites d'autorite varient selon le type d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION). Les regles ci-dessous sont conceptuelles ; StrongFather les traduit en politiques concretes.

### 3.1 APPROVAL (Approbation)

| Limite | Description | Contextuelle |
|--------|-------------|--------------|
| **AL-APPR-1** | L'approbateur doit etre designe ou autorise pour le point d'intervention concerne | Oui (role, point) |
| **AL-APPR-2** | L'approbation ne peut porter que sur l'action declaree au point d'intervention | Oui (point) |
| **AL-APPR-3** | Une seule reponse valide par demande d'approbation | Non |
| **AL-APPR-4** | Le delai d'approbation peut etre restreint selon le niveau de securite ou de confiance | Oui (niveau 0-4, T0-T4) |
| **AL-APPR-5** | En niveau 3-4 ou T3-T4, l'approbation peut etre obligatoire pour certaines actions | Oui (niveau, T) |

### 3.2 OVERRIDE (Derogation)

| Limite | Description | Contextuelle |
|--------|-------------|--------------|
| **AL-OVER-1** | L'override ne peut jamais franchir une limite infranchissable | Non (absolu) |
| **AL-OVER-2** | Justification obligatoire ; longueur ou format minimal selon le contexte | Oui (niveau, politique) |
| **AL-OVER-3** | Seuls les roles autorises peuvent overrider ; la liste depend du point et du niveau de securite | Oui (role, point, niveau) |
| **AL-OVER-4** | En niveau 3-4 ou T3-T4, l'override peut exiger une double validation ou une escalade prealable | Oui (niveau, T) |
| **AL-OVER-5** | L'override ne peut porter que sur la decision automatique prealable identifiee | Non |

### 3.3 ESCALATION (Escalade)

| Limite | Description | Contextuelle |
|--------|-------------|--------------|
| **AL-ESC-1** | La chaine d'escalade doit etre prealablement definie ; l'escalade ne peut aller qu'au niveau superieur designe | Oui (produit, processus) |
| **AL-ESC-2** | L'initiateur doit etre autorise a escalader pour ce point et ce sujet | Oui (role, point) |
| **AL-ESC-3** | Le motif d'escalade est obligatoire ; contenu minimal selon le contexte | Oui (niveau) |
| **AL-ESC-4** | La duree maximale d'attente de resolution peut etre restreinte selon le niveau de confiance (T2-T4) | Oui (T0-T4) |
| **AL-ESC-5** | En T4, l'escalade peut etre le seul canal d'action autorise | Oui (T4) |

### 3.4 SUPERVISION (Supervision)

| Limite | Description | Contextuelle |
|--------|-------------|--------------|
| **AL-SUP-1** | Le superviseur doit etre designe et autorise pour le perimetre de supervision | Oui (role, perimetre) |
| **AL-SUP-2** | La duree de la supervision est bornee (explicite ou timeout) | Oui (produit, niveau) |
| **AL-SUP-3** | En mode passif, la supervision ne doit pas modifier le comportement du systeme | Non |
| **AL-SUP-4** | Toute intervention declenchee depuis une supervision reste soumise aux limites du type concerne (APPROVAL, OVERRIDE, ESCALATION) | Oui (type) |
| **AL-SUP-5** | En niveau 2-4, le perimetre de supervision peut etre restreint (donnees sensibles, processus critiques) | Oui (niveau 0-4) |

---

## 4. Restrictions contextuelles

Les limites d'autorite sont **restrictions contextuelles** : le contexte determine quelles limites s'appliquent et avec quelle severite.

### 4.1 Contexte : niveau de securite (0-4)

Le niveau de securite est defini par l'Operateur (voir [Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)).

| Niveau | Profil | Effet sur les limites d'autorite |
|--------|--------|-----------------------------------|
| **0** | PUBLIC / DISPLAY | Limites minimales ; peu ou pas d'intervention requise |
| **1** | STANDARD / CMS | Limites de base ; approbations optionnelles selon configuration |
| **2** | SENSITIVE DATA | Limites renforcees ; override et approbation soumis a validation renforcee pour donnees sensibles |
| **3** | CRITICAL SYSTEM | Limites strictes ; approbation obligatoire pour operations critiques, override soumis a justification renforcee |
| **4** | HARDENED / ISOLATED | Limites maximales ; toute intervention tracée et restreinte aux roles et points declares |

**INV-AL-2 : Monotonie par niveau de securite**

Un niveau de securite plus eleve (0 -> 4) ne peut jamais **reduire** les restrictions d'autorite. Les limites sont monotones croissantes avec le niveau de securite.

### 4.2 Contexte : niveau de confiance (T0-T4)

Le niveau de confiance reflete l'etat du systeme (voir [Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)).

| Niveau | Etat | Effet sur les limites d'autorite |
|--------|------|-----------------------------------|
| **T0** | Normal | Limites definies par la configuration et le niveau de securite |
| **T1** | Instable | Supervision recommandee ; logging renforce sur les interventions |
| **T2** | Degrade | Points ANOMALY_RESPONSE actives ; supervision obligatoire pour certains processus |
| **T3** | Restreint | Override et approbation soumis a TAMR ; escalade facilitee |
| **T4** | Bloque | Intervention humaine obligatoire ; seuls les points de diagnostic et d'escalade actifs |

**INV-AL-3 : Monotonie par niveau de confiance**

Un niveau de confiance plus eleve (T0 -> T4) ne peut jamais **reduire** les restrictions d'autorite. La degradation du systeme renforce les limites, jamais l'inverse.

### 4.3 Contexte : role et point d'intervention

- **Role** : Seuls les roles autorises pour un type d'intervention et un point donne peuvent exercer cette intervention. La liste des roles est definie par le produit et validee par StrongFather.
- **Point d'intervention** : Les limites s'appliquent au point d'intervention declare. Une intervention hors point declare est invalide (voir [TAMR - Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)).

**INV-AL-4 : Declaration prealable**

Aucune limite d'autorite ne peut etre evaluee pour une intervention qui ne s'inscrit pas dans un point d'intervention declare et pour lequel l'intervenant a un role reconnu.

---

## 5. Regles d'evaluation des limites

### 5.1 Responsable de l'evaluation

**StrongFather** est exclusivement responsable d'evaluer si une intervention respecte les limites d'autorite. TAMR definit les limites (conceptuelles) ; StrongFather applique les politiques et rend la decision (autorisée / refusée).

- TAMR dit : "En niveau 3, l'override necessite une justification renforcee."
- StrongFather dit : "Cette intervention respecte-t-elle la politique de justification niveau 3 ?"

### 5.2 Critères d'evaluation

L'evaluation prend en compte :

1. **Type d'intervention** : Les limites AL-xxx correspondant au type sont appliquees.
2. **Contexte** : Niveau de securite (0-4), niveau de confiance (T0-T4), role de l'intervenant, point d'intervention.
3. **Limites infranchissables** : Aucune evaluation positive si une limite infranchissable serait franchise.
4. **Donnees de l'intervention** : Identite, sujet, justification (si requise), moment, etc.

### 5.3 Ordre d'evaluation

1. Verifier que l'intervention s'inscrit dans un point d'intervention declare et que l'intervenant a un role autorise (INV-AL-4).
2. Verifier qu'aucune limite infranchissable n'est franchise (contrat Inviolable Limits).
3. Appliquer les limites d'autorite selon le type d'intervention (section 3).
4. Appliquer les restrictions contextuelles (niveau securite, niveau confiance) (section 4).
5. Rendre la decision (autorisée / refusée) selon les politiques StrongFather.

**INV-AL-5 : Non-decision par TAMR**

TAMR ne prend jamais de decision. Il definit les limites ; StrongFather evalue et decide.

---

## 6. Invariants des limites d'autorite

### 6.1 Liste des invariants

| Id | Invariant |
|----|-----------|
| **INV-AL-1** | Les limites d'autorite s'appliquent strictement a l'interieur du domaine defini par les limites infranchissables. |
| **INV-AL-2** | Un niveau de securite plus eleve (0 -> 4) ne peut jamais reduire les restrictions d'autorite. |
| **INV-AL-3** | Un niveau de confiance plus eleve (T0 -> T4) ne peut jamais reduire les restrictions d'autorite. |
| **INV-AL-4** | Aucune limite d'autorite ne peut etre evaluee pour une intervention hors point declare ou sans role reconnu. |
| **INV-AL-5** | TAMR definit les limites ; StrongFather evalue et decide. TAMR ne prend jamais de decision. |

### 6.2 Traçabilité

Toute evaluation des limites d'autorite (effectuee par StrongFather) doit pouvoir etre auditee : contexte utilise, limites appliquees, decision. La structure des traces est definie par TAMR (voir contrats Audit) et persistee par KindMother.

---

## 7. Conformite aux Lois d'Autonomie Systeme

Ce contrat respecte les [Lois d'Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dependance externe critique a l'execution

**Conformite :** Conforme

Les limites d'autorite sont definies conceptuellement et evaluees localement par StrongFather. Aucun appel externe n'est requis pour appliquer les limites.

### LOI-2 : Le systeme accepte l'isolement comme etat normal

**Conformite :** Conforme

En mode isole, les limites d'autorite s'appliquent de la meme maniere : l'evaluation est locale, les contextes (niveau securite, niveau confiance) sont connus localement.

### LOI-3 a LOI-6

Les limites d'autorite n'introduisent pas de dependance au temps global, a l'etat distant, ou a la federation. La conformite est preservee.

---

## 8. Integration avec les cores

| Core | Role par rapport aux limites d'autorite |
|------|------------------------------------------|
| **StrongFather** | Evalue le respect des limites et decide d'autoriser ou refuser l'intervention |
| **KindMother** | Persiste les traces d'intervention (incluant le contexte et la decision) |
| **BondingBrother** | Medie l'intention d'intervention ; ne modifie pas les limites |
| **Caring Nanny** | Fournit le niveau de confiance (T0-T4) utilise comme contexte |
| **BorderGuard** | Contribue a l'identification de l'intervenant (confiance) |
| **MasterButler** | Expose les capacites d'intervention selon les limites en vigueur |

---

## 9. Conclusion contractuelle

Ce contrat etablit de maniere definitive et non negociable les **limites d'autorite humaine** et les **restrictions contextuelles** applicables aux interventions dans TAMR.

Il garantit que :

- les limites d'autorite sont explicites, contextuelles, et distinctes des limites infranchissables ;
- les limites par type d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) sont definies ;
- les restrictions contextuelles (niveau securite 0-4, niveau confiance T0-T4, role, point d'intervention) sont precisees ;
- l'evaluation est du ressort de StrongFather, jamais de TAMR ;
- les invariants INV-AL-1 a INV-AL-5 sont respectes ;
- la conformite aux Lois d'Autonomie Systeme est assuree.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisee.

---

**Document cree le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif  
**Reference :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4, [TAMR - Inviolable Limits Contract](./TAMR%20-%20Inviolable%20Limits%20Contract.md)
