# TAMR - Inviolable Limits Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **TAMR — Inviolable Limits Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit le catalogue des **limites infranchissables** dans le Miyukini Core System. Ces limites sont des restrictions absolues que **aucune intervention humaine** — y compris un override — ne peut franchir.

Ce contrat précise la nature conceptuelle des limites infranchissables, les domaines protégés, les règles de vérification, et les conséquences en cas de tentative de franchissement.

### Portée

Ce contrat s'applique à **toutes les interventions humaines dans le système Miyukini** et définit de manière absolue :

- la définition et les caractéristiques des limites infranchissables,
- le catalogue des domaines protégés par des limites infranchissables,
- les règles de non-franchissement,
- la relation avec les limites d'autorité contextuelles,
- les invariants et garanties associés,
- les conséquences et le rôle de vérification (StrongFather).

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :

- **[TAMR — Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Introduction de INV-TAMR-3 (limites infranchissables) et vocabulaire canonique
- **[TAMR — Authority Limits Contract](./TAMR%20-%20Authority%20Limits%20Contract.md)** : Limites d'autorité contextuelles (distinctes des limites infranchissables)
- **[TAMR — Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Règles d'override soumises aux limites infranchissables (R-OVER-2, INV-OVER-1)
- **[TAMR — Security Contract](../security/TAMR%20-%20Security%20Contract.md)** : Limites infranchissables de sécurité (section 9)
- **[Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)** : Terminologie officielle
- **[Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)** : Principes de sécurité
- **[Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)** : Conformité LOI-1 à LOI-6
- **[Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Niveaux T0-T4
- **[Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)** : Niveaux 0-4

Il n'introduit aucune contradiction et constitue la définition formelle des limites infranchissables dans TAMR.

---

## 2. Contexte

### Pourquoi des limites infranchissables

TAMR définit où, quand et comment l'humain intervient. L'override permet à l'humain de contredire une décision automatique — mais **certaines frontières ne doivent jamais être franchies**, même avec une justification humaine.

Les limites infranchissables existent pour :

- **Protéger l'intégrité du système** : empêcher toute action qui corromprait ou détruirait les garanties fondamentales du système
- **Protéger les données critiques** : empêcher la suppression, l'altération non traçable ou l'exfiltration de données dont la perte serait irréversible ou illégale
- **Préserver les règles fondamentales** : empêcher le contournement des lois d'autonomie (LOI-1 à LOI-6) et des contraintes de sécurité de base
- **Respecter les contraintes légales et réglementaires** : empêcher toute intervention qui ferait violer des obligations légales ou contractuelles

Sans limites infranchissables, un override malveillant ou erroné pourrait compromettre l'ensemble du système. Avec elles, l'humain reste l'arbitre dans son périmètre légitime, et le système reste protégé au-delà.

### Distinction : limites d'autorité vs limites infranchissables

| Aspect | Limites d'autorité (Authority Limits) | Limites infranchissables (Inviolable Limits) |
|--------|----------------------------------------|----------------------------------------------|
| **Définition** | Restrictions contextuelles sur ce que l'humain peut faire | Restrictions absolues que même un override ne peut dépasser |
| **Source** | Produit, politique, contexte | TAMR (ce contrat), sécurité, légal |
| **Modifiable par** | Configuration, politique (StrongFather) | Uniquement évolution formelle du contrat |
| **Override** | Peut être overridé si autorisé par StrongFather | **Jamais** overridable |
| **Exemple** | "Seul un manager peut approuver une dépense > X" | "Aucune intervention ne peut désactiver l'audit des accès" |

Les limites d'autorité sont définies dans le [TAMR — Authority Limits Contract](./TAMR%20-%20Authority%20Limits%20Contract.md). Les limites infranchissables sont définies **uniquement** dans le présent contrat.

---

## 3. Définition et caractéristiques

### 3.1 Définition canonique

**Limite infranchissable** : restriction absolue sur les effets possibles d'une intervention humaine telle qu'**aucune intervention** — approbation, override, escalade, supervision — **ne peut produire un effet qui franchit cette limite**.

Une limite infranchissable n'est pas une « règle métier » que l'humain peut contourner avec une justification : c'est une **frontière de non-régression** du système.

### 3.2 Caractéristiques obligatoires

| Caractéristique | Description |
|-----------------|-------------|
| **Absolue** | S'applique sans exception, quel que soit le contexte, l'identité de l'intervenant ou la justification |
| **Non négociable** | Aucune dérogation, délégation ou escalade ne peut autoriser son franchissement |
| **Explicite** | Énoncée clairement dans ce contrat ou dans un document référencé par ce contrat |
| **Vérifiable** | Le système (StrongFather) peut vérifier avant d'appliquer une intervention si l'effet franchirait la limite |
| **Protectrice** | Son objet est la protection de l'intégrité du système, des données critiques, des règles fondamentales ou du cadre légal |

### 3.3 Invariant fondateur : INV-TAMR-3

**INV-TAMR-3 : Limites infranchissables** (repris de la Documentation Fondatrice)

*Certaines limites d'autorité sont absolues et ne peuvent être dépassées par aucune intervention humaine.*

Il existe des limites que même un override ne peut franchir. Ces limites protègent :

- L'intégrité du système
- Les données critiques
- Les règles de sécurité fondamentales
- Les contraintes légales ou réglementaires

---

## 4. Catalogue des limites infranchissables

### 4.1 Domaine 1 : Intégrité du système

**LIM-INV-1 : Pas de corruption volontaire du système**

Aucune intervention humaine ne peut avoir pour effet :

- de désactiver ou contourner délibérément les mécanismes de traçabilité (audit, logs d'intervention),
- de modifier ou supprimer les règles qui garantissent INV-TAMR-1 à INV-TAMR-8,
- d'introduire une backdoor ou un canal non tracé permettant d'agir sans responsabilité,
- de corrompre les données ou métadonnées nécessaires au fonctionnement correct du système (ex. schémas, politiques de sécurité).

**Conséquence en cas de tentative :** Refus de l'intervention (override refusé), alerte, traçabilité de la tentative.

**Référence :** [Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

**LIM-INV-2 : Traçabilité des interventions non supprimable**

Aucune intervention humaine ne peut avoir pour effet de supprimer ou rendre illisible une trace d'intervention humaine déjà enregistrée, ou de modifier l'identité de l'intervenant ou le résultat enregistré.

Les traces d'intervention sont **immuables** pour ce qui concerne l'identité, le moment, le type, la justification (si présente) et le résultat. Les corrections (ex. annotation, rectification) doivent elles-mêmes être tracées et ne peuvent pas effacer l'enregistrement original.

**Conséquence en cas de tentative :** Refus de l'intervention, alerte.

**Référence :** INV-TAMR-1 (traçabilité absolue), [Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) (RT-SEC-5).

---

### 4.2 Domaine 2 : Données critiques

**LIM-INV-3 : Données de sécurité et d'audit ineffaçables**

Aucune intervention humaine ne peut avoir pour effet :

- de supprimer ou rendre inaccessibles les données nécessaires à l'audit de sécurité (logs d'accès, traces d'intervention, journaux d'événements critiques),
- de supprimer ou altérer les données permettant de prouver la conformité légale ou réglementaire,
- d'effacer les justifications associées aux overrides déjà enregistrés.

Les politiques de rétention et d'archivage peuvent être configurées par le produit, mais **aucune intervention humaine ne peut court-circuiter** ces politiques pour effacer des données encore sous rétention.

**Conséquence en cas de tentative :** Refus de l'intervention, escalade sécurité, traçabilité de la tentative.

**Référence :** [Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md), [Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md).

---

**LIM-INV-4 : Pas d'exfiltration non tracée**

Aucune intervention humaine ne peut avoir pour effet d'autoriser ou de réaliser une extraction massive de données sensibles ou critiques sans que cette extraction soit tracée (identité, moment, périmètre, justification).

Les exports, extractions ou copies de données sensibles doivent être soumis aux mêmes règles de traçabilité que les autres interventions. Une intervention ne peut pas « désactiver » cette traçabilité pour une opération d'extraction.

**Conséquence en cas de tentative :** Refus de l'intervention, alerte.

**Référence :** [Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) (niveaux 2 à 4).

---

### 4.3 Domaine 3 : Règles fondamentales

**LIM-INV-5 : Lois d'autonomie système (LOI-1 à LOI-6) non contournables**

Aucune intervention humaine ne peut avoir pour effet de modifier ou désactiver les garanties associées aux [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) de manière à les violer.

Exemples (non exhaustifs) :

- Introduire une dépendance externe critique à l'exécution (violation LOI-1),
- Rendre le système incapable d'accepter l'isolement comme état normal (violation LOI-2),
- Invalider l'état local souverain (violation LOI-3).

Les produits et politiques peuvent **renforcer** ces lois ; aucune intervention ne peut les **affaiblir** au point de les violer.

**Conséquence en cas de tentative :** Refus de l'intervention, audit, notification gouvernance.

**Référence :** [Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

---

**LIM-INV-6 : Niveaux d'intégrité (T0-T4) et de sécurité (0-4) non falsifiables**

Aucune intervention humaine ne peut avoir pour effet de falsifier ou masquer le niveau d'intégrité (T0-T4) ou le niveau de sécurité (0-4) effectivement appliqué au système ou à un sous-ensemble, de manière à tromper les mécanismes de dégradation ou de contrôle.

L'humain peut demander un changement de niveau selon les processus prévus ; il ne peut pas « forcer » un niveau incorrect pour contourner les contrôles.

**Conséquence en cas de tentative :** Refus de l'intervention, alerte, mise en conformité du niveau affiché.

**Référence :** [Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md), [Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md).

---

### 4.4 Domaine 4 : Contraintes légales et réglementaires

**LIM-INV-7 : Conformité légale et réglementaire**

Aucune intervention humaine ne peut avoir pour effet de faire accomplir au système une action qui violerait une contrainte légale ou réglementaire connue et déclarée (ex. RGPD, sectoriel, contrat).

Les contraintes applicables sont définies et maintenues en dehors du seul contrat TAMR (référentiel juridique, politique de conformité). TAMR garantit que **l'intervention humaine n'est pas un canal pour contourner** ces contraintes.

**Conséquence en cas de tentative :** Refus de l'intervention, notification conformité/légal, traçabilité de la tentative.

**Référence :** Doctrine sécurité, politiques produit, référentiel légal.

---

### 4.5 Tableau récapitulatif

| Identifiant | Domaine | Objet principal | Vérification |
|-------------|---------|-----------------|--------------|
| LIM-INV-1 | Intégrité système | Pas de corruption volontaire | StrongFather + audit |
| LIM-INV-2 | Intégrité système | Traces non supprimables | StrongFather + stockage immuable |
| LIM-INV-3 | Données critiques | Données sécurité/audit ineffaçables | StrongFather + politique rétention |
| LIM-INV-4 | Données critiques | Pas d'exfiltration non tracée | StrongFather + traçabilité export |
| LIM-INV-5 | Règles fondamentales | LOI-1 à LOI-6 non contournables | StrongFather + évaluation politique |
| LIM-INV-6 | Règles fondamentales | T0-T4 et 0-4 non falsifiables | StrongFather + CaringNanny |
| LIM-INV-7 | Légal / réglementaire | Conformité non contournable | StrongFather + référentiel conformité |

---

## 5. Règles de non-franchissement

### 5.1 Règle R-INV-1 : Vérification systématique

Toute intervention de type **OVERRIDE** (et toute intervention dont l'effet équivaut à un override) doit être vérifiée **avant** application contre l'ensemble des limites infranchissables (LIM-INV-1 à LIM-INV-7).

La vérification est de la responsabilité de **StrongFather** (ou du composant qui applique les politiques). TAMR ne décide pas ; TAMR **définit** les limites que StrongFather doit faire respecter.

### 5.2 Règle R-INV-2 : Refus sans exception

Si l'effet d'une intervention franchirait une limite infranchissable, l'intervention **doit être refusée**, sans exception.

Aucune justification, escalade, ou niveau hiérarchique ne peut autoriser le franchissement. Le refus est tracé (identité, moment, limite concernée, contexte).

### 5.3 Règle R-INV-3 : Traçabilité des tentatives

Toute tentative d'intervention qui serait refusée au motif d'une limite infranchissable doit être **tracée** (tentative, identité, moment, limite invoquée, contexte). Cette trace ne peut pas être supprimée (LIM-INV-2).

### 5.4 Règle R-INV-4 : Pas d'extension implicite

Le catalogue des limites infranchissables est **fermé** dans ce contrat. Aucune limite supplémentaire ne peut être considérée comme « inviolable » sans modification formelle de ce contrat ou référence explicite à un document contractuel annexe.

Les produits et politiques peuvent définir des **limites d'autorité** (contextuelles) supplémentaires ; ils ne peuvent pas déclarer de nouvelles « limites infranchissables » sans alignement contractuel.

---

## 6. Relation avec StrongFather et les autres cores

### 6.1 Rôle de StrongFather

StrongFather **décide** si une intervention est autorisée. Dans le cadre des limites infranchissables :

- StrongFather **vérifie** que l'effet de l'intervention ne franchit aucune limite LIM-INV-1 à LIM-INV-7.
- Si une limite serait franchie : StrongFather **refuse** l'intervention, indépendamment de la politique d'autorisation habituelle.
- StrongFather **trace** le refus et la limite invoquée.

TAMR ne fait pas la vérification ; TAMR **définit** ce qui doit être vérifié.

### 6.2 Relation avec le flux d'override

D'après la [Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) et le [TAMR — Security Contract](../security/TAMR%20-%20Security%20Contract.md) :

```
1. Décision automatique (acceptée ou refusée) émise
2. Un humain autorisé demande un override
3. L'intention d'override transite par BondingBrother
4. StrongFather évalue si l'override est autorisé
5. StrongFather vérifie que les limites infranchissables sont respectées  ← présent contrat
6. Si autorisé et limites OK : l'humain fournit une justification, override appliqué et tracé
7. Si limite infranchissable violée : override refusé, tentative tracée
```

### 6.3 Autres cores

| Core | Relation avec les limites infranchissables |
|------|-------------------------------------------|
| **KindMother** | Persiste les traces d'intervention et des tentatives ; ne doit pas permettre la suppression non tracée (alignement LIM-INV-2, LIM-INV-3) |
| **BondingBrother** | Transmet les intentions ; ne vérifie pas les limites (rôle StrongFather) |
| **CaringNanny** | Observe l'état (ex. T0-T4) ; peut alimenter la vérification LIM-INV-6 |
| **BorderGuard** | Confiance de l'intervenant ; ne dispense pas de la vérification des limites infranchissables |

---

## 7. Conformité aux Lois d'Autonomie Système

Ce contrat respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique à l'exécution

**Conformité :** ✅ **Conforme**

Les limites infranchissables sont définies conceptuellement et vérifiées localement par StrongFather. Aucune vérification de limite ne nécessite un appel externe bloquant.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ **Conforme**

Les vérifications des limites infranchissables peuvent être effectuées en mode isolé. Le refus d'une intervention qui franchirait une limite reste possible sans connexion externe.

### LOI-5 / LOI-6

Les limites LIM-INV-5 et LIM-INV-6 **protègent** explicitement le respect des lois d'autonomie et des niveaux d'intégrité/sécurité ; ce contrat ne les viole pas.

---

## 8. Évolution et fermeture du contrat

### 8.1 Catalogue fermé

Les limites infranchissables reconnues sont celles explicitement listées dans ce contrat (LIM-INV-1 à LIM-INV-7) ou dans un document contractuel référencé et approuvé selon le processus de gel TAMR.

Aucune « limite infranchissable » implicite ou dérivée uniquement d'une politique produit n'est reconnue sans mise à jour de ce contrat.

### 8.2 Conditions d'évolution

Toute ajout ou modification d'une limite infranchissable :

- doit être **explicite** et documentée dans ce contrat (ou annexe contractuelle),
- doit préserver la **rétrocompatibilité** avec les interventions déjà tracées,
- doit être **validée** selon les processus contractuels et de gel TAMR,
- doit préserver l'invariant **INV-TAMR-3** et la distinction avec les limites d'autorité contextuelles.

---

## 9. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable les **limites infranchissables** dans TAMR.

Il garantit que :

- les limites infranchissables sont définies, cataloguées (LIM-INV-1 à LIM-INV-7) et explicites ;
- aucune intervention humaine, y compris override, ne peut franchir ces limites ;
- la vérification est du ressort de StrongFather, sur la base des définitions TAMR ;
- les tentatives de franchissement sont refusées et tracées ;
- le contrat est fermé et ne peut être étendu qu par évolution formelle.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4, TAMR Security Contract, TAMR Intervention Types Contract

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Doctrine Sécurité | [Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) |
| Lois Autonomie | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| Integrity Degradation | [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) |
