# TAMR - Inviolable Limits Contract

## 1. Introduction

### Objet du contrat

Ce document dÃ©finit le **TAMR â€” Inviolable Limits Contract** : un contrat normatif, non nÃ©gociable, et de statut FONDATION qui Ã©tablit le catalogue des **limites infranchissables** dans le Miyukini Core System. Ces limites sont des restrictions absolues que **aucune intervention humaine** â€” y compris un override â€” ne peut franchir.

Ce contrat prÃ©cise la nature conceptuelle des limites infranchissables, les domaines protÃ©gÃ©s, les rÃ¨gles de vÃ©rification, et les consÃ©quences en cas de tentative de franchissement.

### PortÃ©e

Ce contrat s'applique Ã  **toutes les interventions humaines dans le systÃ¨me Miyukini** et dÃ©finit de maniÃ¨re absolue :

- la dÃ©finition et les caractÃ©ristiques des limites infranchissables,
- le catalogue des domaines protÃ©gÃ©s par des limites infranchissables,
- les rÃ¨gles de non-franchissement,
- la relation avec les limites d'autoritÃ© contextuelles,
- les invariants et garanties associÃ©s,
- les consÃ©quences et le rÃ´le de vÃ©rification (StrongFather).

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il Ã©tablit des rÃ¨gles absolues qui ne peuvent Ãªtre contournÃ©es, nÃ©gociÃ©es, ou modifiÃ©es. Le contrat prime sur toute considÃ©ration pratique.

### Relation avec les autres contrats

Ce contrat complÃ¨te et respecte les documents contractuels existants :

- **[TAMR â€” Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Introduction de INV-TAMR-3 (limites infranchissables) et vocabulaire canonique
- **[TAMR â€” Authority Limits Contract](./TAMR%20-%20Authority%20Limits%20Contract.md)** : Limites d'autoritÃ© contextuelles (distinctes des limites infranchissables)
- **[TAMR â€” Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : RÃ¨gles d'override soumises aux limites infranchissables (R-OVER-2, INV-OVER-1)
- **[TAMR â€” Security Contract](../security/TAMR%20-%20Security%20Contract.md)** : Limites infranchissables de sÃ©curitÃ© (section 9)
- **[Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Terminologie officielle
- **[Miyukini Conceptual References - Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Principes de sÃ©curitÃ©
- **[Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : ConformitÃ© LOI-1 Ã  LOI-6
- **[Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Niveaux T0-T4
- **[Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Niveaux 0-4

Il n'introduit aucune contradiction et constitue la dÃ©finition formelle des limites infranchissables dans TAMR.

---

## 2. Contexte

### Pourquoi des limites infranchissables

TAMR dÃ©finit oÃ¹, quand et comment l'humain intervient. L'override permet Ã  l'humain de contredire une dÃ©cision automatique â€” mais **certaines frontiÃ¨res ne doivent jamais Ãªtre franchies**, mÃªme avec une justification humaine.

Les limites infranchissables existent pour :

- **ProtÃ©ger l'intÃ©gritÃ© du systÃ¨me** : empÃªcher toute action qui corromprait ou dÃ©truirait les garanties fondamentales du systÃ¨me
- **ProtÃ©ger les donnÃ©es critiques** : empÃªcher la suppression, l'altÃ©ration non traÃ§able ou l'exfiltration de donnÃ©es dont la perte serait irrÃ©versible ou illÃ©gale
- **PrÃ©server les rÃ¨gles fondamentales** : empÃªcher le contournement des lois d'autonomie (LOI-1 Ã  LOI-6) et des contraintes de sÃ©curitÃ© de base
- **Respecter les contraintes lÃ©gales et rÃ©glementaires** : empÃªcher toute intervention qui ferait violer des obligations lÃ©gales ou contractuelles

Sans limites infranchissables, un override malveillant ou erronÃ© pourrait compromettre l'ensemble du systÃ¨me. Avec elles, l'humain reste l'arbitre dans son pÃ©rimÃ¨tre lÃ©gitime, et le systÃ¨me reste protÃ©gÃ© au-delÃ .

### Distinction : limites d'autoritÃ© vs limites infranchissables

| Aspect | Limites d'autoritÃ© (Authority Limits) | Limites infranchissables (Inviolable Limits) |
|--------|----------------------------------------|----------------------------------------------|
| **DÃ©finition** | Restrictions contextuelles sur ce que l'humain peut faire | Restrictions absolues que mÃªme un override ne peut dÃ©passer |
| **Source** | Produit, politique, contexte | TAMR (ce contrat), sÃ©curitÃ©, lÃ©gal |
| **Modifiable par** | Configuration, politique (StrongFather) | Uniquement Ã©volution formelle du contrat |
| **Override** | Peut Ãªtre overridÃ© si autorisÃ© par StrongFather | **Jamais** overridable |
| **Exemple** | "Seul un manager peut approuver une dÃ©pense > X" | "Aucune intervention ne peut dÃ©sactiver l'audit des accÃ¨s" |

Les limites d'autoritÃ© sont dÃ©finies dans le [TAMR â€” Authority Limits Contract](./TAMR%20-%20Authority%20Limits%20Contract.md). Les limites infranchissables sont dÃ©finies **uniquement** dans le prÃ©sent contrat.

---

## 3. DÃ©finition et caractÃ©ristiques

### 3.1 DÃ©finition canonique

**Limite infranchissable** : restriction absolue sur les effets possibles d'une intervention humaine telle qu'**aucune intervention** â€” approbation, override, escalade, supervision â€” **ne peut produire un effet qui franchit cette limite**.

Une limite infranchissable n'est pas une Â« rÃ¨gle mÃ©tier Â» que l'humain peut contourner avec une justification : c'est une **frontiÃ¨re de non-rÃ©gression** du systÃ¨me.

### 3.2 CaractÃ©ristiques obligatoires

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Absolue** | S'applique sans exception, quel que soit le contexte, l'identitÃ© de l'intervenant ou la justification |
| **Non nÃ©gociable** | Aucune dÃ©rogation, dÃ©lÃ©gation ou escalade ne peut autoriser son franchissement |
| **Explicite** | Ã‰noncÃ©e clairement dans ce contrat ou dans un document rÃ©fÃ©rencÃ© par ce contrat |
| **VÃ©rifiable** | Le systÃ¨me (StrongFather) peut vÃ©rifier avant d'appliquer une intervention si l'effet franchirait la limite |
| **Protectrice** | Son objet est la protection de l'intÃ©gritÃ© du systÃ¨me, des donnÃ©es critiques, des rÃ¨gles fondamentales ou du cadre lÃ©gal |

### 3.3 Invariant fondateur : INV-TAMR-3

**INV-TAMR-3 : Limites infranchissables** (repris de la Documentation Fondatrice)

*Certaines limites d'autoritÃ© sont absolues et ne peuvent Ãªtre dÃ©passÃ©es par aucune intervention humaine.*

Il existe des limites que mÃªme un override ne peut franchir. Ces limites protÃ¨gent :

- L'intÃ©gritÃ© du systÃ¨me
- Les donnÃ©es critiques
- Les rÃ¨gles de sÃ©curitÃ© fondamentales
- Les contraintes lÃ©gales ou rÃ©glementaires

---

## 4. Catalogue des limites infranchissables

### 4.1 Domaine 1 : IntÃ©gritÃ© du systÃ¨me

**LIM-INV-1 : Pas de corruption volontaire du systÃ¨me**

Aucune intervention humaine ne peut avoir pour effet :

- de dÃ©sactiver ou contourner dÃ©libÃ©rÃ©ment les mÃ©canismes de traÃ§abilitÃ© (audit, logs d'intervention),
- de modifier ou supprimer les rÃ¨gles qui garantissent INV-TAMR-1 Ã  INV-TAMR-8,
- d'introduire une backdoor ou un canal non tracÃ© permettant d'agir sans responsabilitÃ©,
- de corrompre les donnÃ©es ou mÃ©tadonnÃ©es nÃ©cessaires au fonctionnement correct du systÃ¨me (ex. schÃ©mas, politiques de sÃ©curitÃ©).

**ConsÃ©quence en cas de tentative :** Refus de l'intervention (override refusÃ©), alerte, traÃ§abilitÃ© de la tentative.

**RÃ©fÃ©rence :** [Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**LIM-INV-2 : TraÃ§abilitÃ© des interventions non supprimable**

Aucune intervention humaine ne peut avoir pour effet de supprimer ou rendre illisible une trace d'intervention humaine dÃ©jÃ  enregistrÃ©e, ou de modifier l'identitÃ© de l'intervenant ou le rÃ©sultat enregistrÃ©.

Les traces d'intervention sont **immuables** pour ce qui concerne l'identitÃ©, le moment, le type, la justification (si prÃ©sente) et le rÃ©sultat. Les corrections (ex. annotation, rectification) doivent elles-mÃªmes Ãªtre tracÃ©es et ne peuvent pas effacer l'enregistrement original.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, alerte.

**RÃ©fÃ©rence :** INV-TAMR-1 (traÃ§abilitÃ© absolue), [Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md) (RT-SEC-5).

---

### 4.2 Domaine 2 : DonnÃ©es critiques

**LIM-INV-3 : DonnÃ©es de sÃ©curitÃ© et d'audit ineffaÃ§ables**

Aucune intervention humaine ne peut avoir pour effet :

- de supprimer ou rendre inaccessibles les donnÃ©es nÃ©cessaires Ã  l'audit de sÃ©curitÃ© (logs d'accÃ¨s, traces d'intervention, journaux d'Ã©vÃ©nements critiques),
- de supprimer ou altÃ©rer les donnÃ©es permettant de prouver la conformitÃ© lÃ©gale ou rÃ©glementaire,
- d'effacer les justifications associÃ©es aux overrides dÃ©jÃ  enregistrÃ©s.

Les politiques de rÃ©tention et d'archivage peuvent Ãªtre configurÃ©es par le produit, mais **aucune intervention humaine ne peut court-circuiter** ces politiques pour effacer des donnÃ©es encore sous rÃ©tention.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, escalade sÃ©curitÃ©, traÃ§abilitÃ© de la tentative.

**RÃ©fÃ©rence :** [Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md), [Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md).

---

**LIM-INV-4 : Pas d'exfiltration non tracÃ©e**

Aucune intervention humaine ne peut avoir pour effet d'autoriser ou de rÃ©aliser une extraction massive de donnÃ©es sensibles ou critiques sans que cette extraction soit tracÃ©e (identitÃ©, moment, pÃ©rimÃ¨tre, justification).

Les exports, extractions ou copies de donnÃ©es sensibles doivent Ãªtre soumis aux mÃªmes rÃ¨gles de traÃ§abilitÃ© que les autres interventions. Une intervention ne peut pas Â« dÃ©sactiver Â» cette traÃ§abilitÃ© pour une opÃ©ration d'extraction.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, alerte.

**RÃ©fÃ©rence :** [Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) (niveaux 2 Ã  4).

---

### 4.3 Domaine 3 : RÃ¨gles fondamentales

**LIM-INV-5 : Lois d'autonomie systÃ¨me (LOI-1 Ã  LOI-6) non contournables**

Aucune intervention humaine ne peut avoir pour effet de modifier ou dÃ©sactiver les garanties associÃ©es aux [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) de maniÃ¨re Ã  les violer.

Exemples (non exhaustifs) :

- Introduire une dÃ©pendance externe critique Ã  l'exÃ©cution (violation LOI-1),
- Rendre le systÃ¨me incapable d'accepter l'isolement comme Ã©tat normal (violation LOI-2),
- Invalider l'Ã©tat local souverain (violation LOI-3).

Les produits et politiques peuvent **renforcer** ces lois ; aucune intervention ne peut les **affaiblir** au point de les violer.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, audit, notification gouvernance.

**RÃ©fÃ©rence :** [Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md).

---

**LIM-INV-6 : Niveaux d'intÃ©gritÃ© (T0-T4) et de sÃ©curitÃ© (0-4) non falsifiables**

Aucune intervention humaine ne peut avoir pour effet de falsifier ou masquer le niveau d'intÃ©gritÃ© (T0-T4) ou le niveau de sÃ©curitÃ© (0-4) effectivement appliquÃ© au systÃ¨me ou Ã  un sous-ensemble, de maniÃ¨re Ã  tromper les mÃ©canismes de dÃ©gradation ou de contrÃ´le.

L'humain peut demander un changement de niveau selon les processus prÃ©vus ; il ne peut pas Â« forcer Â» un niveau incorrect pour contourner les contrÃ´les.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, alerte, mise en conformitÃ© du niveau affichÃ©.

**RÃ©fÃ©rence :** [Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md), [Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md).

---

### 4.4 Domaine 4 : Contraintes lÃ©gales et rÃ©glementaires

**LIM-INV-7 : ConformitÃ© lÃ©gale et rÃ©glementaire**

Aucune intervention humaine ne peut avoir pour effet de faire accomplir au systÃ¨me une action qui violerait une contrainte lÃ©gale ou rÃ©glementaire connue et dÃ©clarÃ©e (ex. RGPD, sectoriel, contrat).

Les contraintes applicables sont dÃ©finies et maintenues en dehors du seul contrat TAMR (rÃ©fÃ©rentiel juridique, politique de conformitÃ©). TAMR garantit que **l'intervention humaine n'est pas un canal pour contourner** ces contraintes.

**ConsÃ©quence en cas de tentative :** Refus de l'intervention, notification conformitÃ©/lÃ©gal, traÃ§abilitÃ© de la tentative.

**RÃ©fÃ©rence :** Doctrine sÃ©curitÃ©, politiques produit, rÃ©fÃ©rentiel lÃ©gal.

---

### 4.5 Tableau rÃ©capitulatif

| Identifiant | Domaine | Objet principal | VÃ©rification |
|-------------|---------|-----------------|--------------|
| LIM-INV-1 | IntÃ©gritÃ© systÃ¨me | Pas de corruption volontaire | StrongFather + audit |
| LIM-INV-2 | IntÃ©gritÃ© systÃ¨me | Traces non supprimables | StrongFather + stockage immuable |
| LIM-INV-3 | DonnÃ©es critiques | DonnÃ©es sÃ©curitÃ©/audit ineffaÃ§ables | StrongFather + politique rÃ©tention |
| LIM-INV-4 | DonnÃ©es critiques | Pas d'exfiltration non tracÃ©e | StrongFather + traÃ§abilitÃ© export |
| LIM-INV-5 | RÃ¨gles fondamentales | LOI-1 Ã  LOI-6 non contournables | StrongFather + Ã©valuation politique |
| LIM-INV-6 | RÃ¨gles fondamentales | T0-T4 et 0-4 non falsifiables | StrongFather + CaringNanny |
| LIM-INV-7 | LÃ©gal / rÃ©glementaire | ConformitÃ© non contournable | StrongFather + rÃ©fÃ©rentiel conformitÃ© |

---

## 5. RÃ¨gles de non-franchissement

### 5.1 RÃ¨gle R-INV-1 : VÃ©rification systÃ©matique

Toute intervention de type **OVERRIDE** (et toute intervention dont l'effet Ã©quivaut Ã  un override) doit Ãªtre vÃ©rifiÃ©e **avant** application contre l'ensemble des limites infranchissables (LIM-INV-1 Ã  LIM-INV-7).

La vÃ©rification est de la responsabilitÃ© de **StrongFather** (ou du composant qui applique les politiques). TAMR ne dÃ©cide pas ; TAMR **dÃ©finit** les limites que StrongFather doit faire respecter.

### 5.2 RÃ¨gle R-INV-2 : Refus sans exception

Si l'effet d'une intervention franchirait une limite infranchissable, l'intervention **doit Ãªtre refusÃ©e**, sans exception.

Aucune justification, escalade, ou niveau hiÃ©rarchique ne peut autoriser le franchissement. Le refus est tracÃ© (identitÃ©, moment, limite concernÃ©e, contexte).

### 5.3 RÃ¨gle R-INV-3 : TraÃ§abilitÃ© des tentatives

Toute tentative d'intervention qui serait refusÃ©e au motif d'une limite infranchissable doit Ãªtre **tracÃ©e** (tentative, identitÃ©, moment, limite invoquÃ©e, contexte). Cette trace ne peut pas Ãªtre supprimÃ©e (LIM-INV-2).

### 5.4 RÃ¨gle R-INV-4 : Pas d'extension implicite

Le catalogue des limites infranchissables est **fermÃ©** dans ce contrat. Aucune limite supplÃ©mentaire ne peut Ãªtre considÃ©rÃ©e comme Â« inviolable Â» sans modification formelle de ce contrat ou rÃ©fÃ©rence explicite Ã  un document contractuel annexe.

Les produits et politiques peuvent dÃ©finir des **limites d'autoritÃ©** (contextuelles) supplÃ©mentaires ; ils ne peuvent pas dÃ©clarer de nouvelles Â« limites infranchissables Â» sans alignement contractuel.

---

## 6. Relation avec StrongFather et les autres cores

### 6.1 RÃ´le de StrongFather

StrongFather **dÃ©cide** si une intervention est autorisÃ©e. Dans le cadre des limites infranchissables :

- StrongFather **vÃ©rifie** que l'effet de l'intervention ne franchit aucune limite LIM-INV-1 Ã  LIM-INV-7.
- Si une limite serait franchie : StrongFather **refuse** l'intervention, indÃ©pendamment de la politique d'autorisation habituelle.
- StrongFather **trace** le refus et la limite invoquÃ©e.

TAMR ne fait pas la vÃ©rification ; TAMR **dÃ©finit** ce qui doit Ãªtre vÃ©rifiÃ©.

### 6.2 Relation avec le flux d'override

D'aprÃ¨s la [Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) et le [TAMR â€” Security Contract](../security/TAMR%20-%20Security%20Contract.md) :

```
1. DÃ©cision automatique (acceptÃ©e ou refusÃ©e) Ã©mise
2. Un humain autorisÃ© demande un override
3. L'intention d'override transite par BondingBrother
4. StrongFather Ã©value si l'override est autorisÃ©
5. StrongFather vÃ©rifie que les limites infranchissables sont respectÃ©es  â† prÃ©sent contrat
6. Si autorisÃ© et limites OK : l'humain fournit une justification, override appliquÃ© et tracÃ©
7. Si limite infranchissable violÃ©e : override refusÃ©, tentative tracÃ©e
```

### 6.3 Autres cores

| Core | Relation avec les limites infranchissables |
|------|-------------------------------------------|
| **KindMother** | Persiste les traces d'intervention et des tentatives ; ne doit pas permettre la suppression non tracÃ©e (alignement LIM-INV-2, LIM-INV-3) |
| **BondingBrother** | Transmet les intentions ; ne vÃ©rifie pas les limites (rÃ´le StrongFather) |
| **CaringNanny** | Observe l'Ã©tat (ex. T0-T4) ; peut alimenter la vÃ©rification LIM-INV-6 |
| **BorderGuard** | Confiance de l'intervenant ; ne dispense pas de la vÃ©rification des limites infranchissables |

---

## 7. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md).

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** âœ… **Conforme**

Les limites infranchissables sont dÃ©finies conceptuellement et vÃ©rifiÃ©es localement par StrongFather. Aucune vÃ©rification de limite ne nÃ©cessite un appel externe bloquant.

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… **Conforme**

Les vÃ©rifications des limites infranchissables peuvent Ãªtre effectuÃ©es en mode isolÃ©. Le refus d'une intervention qui franchirait une limite reste possible sans connexion externe.

### LOI-5 / LOI-6

Les limites LIM-INV-5 et LIM-INV-6 **protÃ¨gent** explicitement le respect des lois d'autonomie et des niveaux d'intÃ©gritÃ©/sÃ©curitÃ© ; ce contrat ne les viole pas.

---

## 8. Ã‰volution et fermeture du contrat

### 8.1 Catalogue fermÃ©

Les limites infranchissables reconnues sont celles explicitement listÃ©es dans ce contrat (LIM-INV-1 Ã  LIM-INV-7) ou dans un document contractuel rÃ©fÃ©rencÃ© et approuvÃ© selon le processus de gel TAMR.

Aucune Â« limite infranchissable Â» implicite ou dÃ©rivÃ©e uniquement d'une politique produit n'est reconnue sans mise Ã  jour de ce contrat.

### 8.2 Conditions d'Ã©volution

Toute ajout ou modification d'une limite infranchissable :

- doit Ãªtre **explicite** et documentÃ©e dans ce contrat (ou annexe contractuelle),
- doit prÃ©server la **rÃ©trocompatibilitÃ©** avec les interventions dÃ©jÃ  tracÃ©es,
- doit Ãªtre **validÃ©e** selon les processus contractuels et de gel TAMR,
- doit prÃ©server l'invariant **INV-TAMR-3** et la distinction avec les limites d'autoritÃ© contextuelles.

---

## 9. Conclusion contractuelle

Ce contrat Ã©tablit de maniÃ¨re dÃ©finitive et non nÃ©gociable les **limites infranchissables** dans TAMR.

Il garantit que :

- les limites infranchissables sont dÃ©finies, cataloguÃ©es (LIM-INV-1 Ã  LIM-INV-7) et explicites ;
- aucune intervention humaine, y compris override, ne peut franchir ces limites ;
- la vÃ©rification est du ressort de StrongFather, sur la base des dÃ©finitions TAMR ;
- les tentatives de franchissement sont refusÃ©es et tracÃ©es ;
- le contrat est fermÃ© et ne peut Ãªtre Ã©tendu qu par Ã©volution formelle.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisÃ©e.

---

**Document crÃ©Ã© le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif validÃ©  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, TAMR Documentation Fondatrice v1.4, TAMR Security Contract, TAMR Intervention Types Contract

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Doctrine SÃ©curitÃ© | [Miyukini Conceptual References - Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Lois Autonomie | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Integrity Degradation | [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) |

