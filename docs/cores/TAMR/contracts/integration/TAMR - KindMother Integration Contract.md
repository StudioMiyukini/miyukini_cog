# TAMR - KindMother Integration Contract

## 1. Contexte

Ce document dÃ©finit le **contrat d'intÃ©gration entre TAMR et KindMother** pour la persistance des traces d'intervention humaine. Il spÃ©cifie la rÃ©partition des responsabilitÃ©s : TAMR dÃ©finit la structure conceptuelle des traces d'intervention, KindMother assure leur persistance selon ses mÃ©canismes.

Ce document complÃ¨te la section Â« Relation avec KindMother Â» de la [Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md) pour les types d'intervention Ã  tracer
- [TAMR - Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md) pour INV-TAMR-1 (traÃ§abilitÃ© absolue)
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) pour la terminologie TAMR
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) pour LOI-2, LOI-3 (traces locales, synchronisation diffÃ©rÃ©e)
- [Miyukini Conceptual References - Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les principes de sÃ©curitÃ©
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les niveaux T0-T4
- [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) pour les niveaux 0-4

L'intÃ©gration respecte les Lois d'Autonomie SystÃ¨me : les traces d'intervention sont persistÃ©es localement et peuvent Ãªtre synchronisÃ©es Ã  la reconnexion (**LOI-2**, **LOI-3**).

## 2. PortÃ©e / Scope

Ce document couvre :
- La rÃ©partition des responsabilitÃ©s TAMR (structure) / KindMother (persistance)
- La structure conceptuelle des traces d'intervention dÃ©finie par TAMR
- Les types de traces (approbation, override, escalade, supervision)
- Les garanties de persistance et de traÃ§abilitÃ©
- Le comportement en mode offline

Ce document **ne couvre pas** :
- Les dÃ©tails internes de KindMother (voir documentation KindMother)
- La mÃ©diation des intentions d'intervention (voir [TAMR - BondingBrother Integration Contract](./TAMR%20-%20BondingBrother%20Integration%20Contract.md))
- La dÃ©cision d'autoriser une intervention (voir [TAMR - StrongFather Integration Contract](./TAMR%20-%20StrongFather%20Integration%20Contract.md))
- L'implÃ©mentation technique des interfaces de persistance

---

## 3. Principe fondamental

**TAMR dÃ©finit ce qui doit Ãªtre tracÃ© lors d'une intervention humaine. KindMother persiste ces traces. TAMR ne persiste rien ; KindMother ne dÃ©finit pas la structure des traces d'intervention.**

La relation est unidirectionnelle et complÃ©mentaire :
- **TAMR** : dÃ©finit la structure conceptuelle des traces (identitÃ© intervenant, type, moment, contexte, justification si requise, rÃ©sultat)
- **KindMother** : gÃ¨re la persistance, la cohÃ©rence, la rÃ©tention et la synchronisation des donnÃ©es de traces selon ses mÃ©canismes

Les traces d'intervention sont des donnÃ©es comme les autres du point de vue de KindMother ; leur schÃ©ma conceptuel est imposÃ© par TAMR.

---

## 4. Positionnement de KindMother

### 4.1 AutoritÃ© sur la persistance des traces

**KindMother est l'autoritÃ© pour :**
- La persistance des traces d'intervention
- La cohÃ©rence et l'intÃ©gritÃ© des donnÃ©es de traces
- La rÃ©tention et l'archivage (selon politiques produit)
- La synchronisation des traces entre instances (offline-first)

**RÃ¨gle TAMR-KM-01 : TAMR ne persiste pas**

TAMR ne peut jamais Ã©crire, modifier ou supprimer de donnÃ©es. Toute persistance de trace d'intervention est dÃ©lÃ©guÃ©e Ã  KindMother (via le produit ou BondingBrother).

**RÃ¨gle TAMR-KM-02 : Structure dÃ©finie par TAMR**

La structure conceptuelle des traces (champs obligatoires, sÃ©mantique, types d'intervention) est dÃ©finie par TAMR. KindMother persiste des donnÃ©es conformes Ã  cette structure.

**RÃ¨gle TAMR-KM-03 : Pas de dÃ©finition de contenu par KindMother**

KindMother ne dÃ©finit pas ce qu'est une trace d'intervention ni quels champs sont requis. Elle applique le schÃ©ma dÃ©rivÃ© des exigences TAMR.

---

## 5. Structure conceptuelle des traces (dÃ©finie par TAMR)

### 5.1 Champs obligatoires

Toute trace d'intervention persistÃ©e par KindMother doit contenir au minimum les Ã©lÃ©ments suivants, dÃ©finis par TAMR (voir Documentation Fondatrice, section Â« DÃ©finition des exigences de traÃ§abilitÃ© Â») :

| Ã‰lÃ©ment | Description | Source TAMR |
|--------|-------------|-------------|
| **IdentitÃ© de l'intervenant** | Qui a effectuÃ© l'intervention (rÃ©fÃ©rence fournie par le produit/auth) | INV-TAMR-1, INV-TAMR-2 |
| **Type d'intervention** | APPROVAL, OVERRIDE, ESCALATION, SUPERVISION | Types Contract |
| **Moment de l'intervention** | Horodatage (local) | INV-TAMR-1 |
| **Contexte de l'intervention** | Point d'intervention, processus, entitÃ© concernÃ©e | Documentation Fondatrice |
| **RÃ©sultat de l'intervention** | DÃ©cision prise (approuvÃ©, refusÃ©, overridÃ©, escaladÃ©, etc.) | INV-TAMR-1 |
| **Justification** | Obligatoire pour OVERRIDE ; optionnelle pour les autres types | INV-TAMR-7 |

### 5.2 Structure conceptuelle (schÃ©ma cible)

```text
TraceIntervention {
  trace_id          : Identifiant unique (gÃ©rÃ© par KindMother ou produit)
  intervenant_id    : IdentitÃ© de l'humain intervenant
  type              : APPROVAL | OVERRIDE | ESCALATION | SUPERVISION
  moment            : Timestamp (local)
  contexte          : { point_id?, processus_id?, entitÃ©_id?, ... }
  rÃ©sultat          : RÃ©sultat de l'intervention
  justification?    : Obligatoire si type = OVERRIDE
  mÃ©tadonnÃ©es?      : DonnÃ©es additionnelles (sans modifier la sÃ©mantique TAMR)
}
```

**RÃ¨gle TAMR-KM-04 : ExhaustivitÃ© des champs obligatoires**

Aucune trace ne peut Ãªtre persistÃ©e sans les champs obligatoires dÃ©finis par TAMR. KindMother (ou le produit qui alimente KindMother) doit rejeter ou refuser toute Ã©criture incomplÃ¨te.

**RÃ¨gle TAMR-KM-05 : Justification pour override**

Si le type d'intervention est OVERRIDE, le champ justification doit Ãªtre renseignÃ©. C'est une exigence TAMR (INV-TAMR-7), respectÃ©e avant persistance.

---

## 6. Types de traces et sÃ©mantique

### 6.1 Par type d'intervention

| Type | Trace typique | Justification |
|------|----------------|---------------|
| **APPROVAL** | Approbation / refus d'une action proposÃ©e | Optionnelle |
| **OVERRIDE** | DÃ©cision de dÃ©roger Ã  une dÃ©cision automatique | **Obligatoire** |
| **ESCALATION** | MontÃ©e en niveau, dÃ©lÃ©gation, rÃ©solution | Optionnelle (contexte souvent suffisant) |
| **SUPERVISION** | DÃ©but/fin de supervision, intervention dÃ©clenchÃ©e | Optionnelle |

### 6.2 RÃ¨gles de cohÃ©rence

**RÃ¨gle TAMR-KM-06 : UnicitÃ© de type par trace**

Une trace correspond Ã  une et une seule intervention, donc un seul type parmi les quatre dÃ©finis par TAMR.

**RÃ¨gle TAMR-KM-07 : Pas de modification rÃ©troactive du sens**

Une fois une trace persistÃ©e, sa sÃ©mantique (type, rÃ©sultat, intervenant, moment) ne doit pas Ãªtre altÃ©rÃ©e pour des raisons mÃ©tier. Les corrections techniques (correction de faute, mise Ã  jour technique) restent sous la responsabilitÃ© de KindMother et des politiques produit, sans contredire l'invariant de traÃ§abilitÃ© (INV-TAMR-1).

---

## 7. Flux de persistance des traces

### 7.1 ResponsabilitÃ© du flux

1. Une intervention humaine a lieu (approbation, override, escalade, supervision).
2. Le produit (ou BondingBrother) produit un enregistrement de trace conforme Ã  la structure TAMR.
3. Ce enregistrement est transmis Ã  KindMother pour persistance (selon les mÃ©canismes du produit et de KindMother).
4. KindMother persiste la trace et assure cohÃ©rence, rÃ©tention, synchronisation.

**TAMR** intervient uniquement en amont : il dÃ©finit la structure et les rÃ¨gles (champs obligatoires, justification pour override). **TAMR ne participe pas au flux d'Ã©criture.**

### 7.2 Diagramme de responsabilitÃ©s

```text
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     structure      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     Ã©criture      â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚    TAMR     â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚  Produit /  â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º â”‚ KindMother  â”‚
â”‚ (conceptuel)â”‚  (contrat, schÃ©ma) â”‚ BondingBrotherâ”‚  (donnÃ©es trace) â”‚ (persistance)â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 8. Garanties de l'intÃ©gration

### 8.1 Garantie de sÃ©paration des rÃ´les

**Engagement :** TAMR ne persiste jamais ; KindMother ne dÃ©finit jamais la structure des traces d'intervention. Les rÃ´les restent strictement sÃ©parÃ©s.

### 8.2 Garantie de conformitÃ© structurelle

**Engagement :** Les donnÃ©es de traces persistÃ©es par KindMother sont conformes Ã  la structure conceptuelle dÃ©finie par TAMR (champs obligatoires, types, justification pour override).

### 8.3 Garantie de traÃ§abilitÃ©

**Engagement :** Toute intervention humaine donnant lieu Ã  une trace conforme Ã  TAMR est persistÃ©e par KindMother sans perte des champs obligatoires. INV-TAMR-1 (traÃ§abilitÃ© absolue) est respectÃ© au niveau conceptuel ; KindMother en est le support de persistance.

### 8.4 Garantie de non-modification par TAMR

**Engagement :** TAMR ne modifie, ne lit ni ne supprime aucune donnÃ©e. Il ne fait que dÃ©finir le contrat de structure et les rÃ¨gles de traÃ§abilitÃ©.

---

## 9. Mode offline et synchronisation

### 9.1 Persistance locale

ConformÃ©ment Ã  **LOI-2** et **LOI-3**, les traces d'intervention peuvent Ãªtre produites et persistÃ©es localement (DB Fille, stockage local). KindMother gÃ¨re la persistance locale et la synchronisation ultÃ©rieure vers la DB MÃ¨re selon ses propres rÃ¨gles.

**RÃ¨gle TAMR-KM-08 : Traces locales valides**

Les traces crÃ©Ã©es en mode isolÃ© sont valides localement et doivent Ãªtre conservÃ©es jusqu'Ã  synchronisation. La structure TAMR s'applique identiquement en local et aprÃ¨s synchronisation.

### 9.2 Pas d'exigence technique TAMR

TAMR ne dÃ©finit pas de protocole de synchronisation, de timeout ni de stratÃ©gie de conflit. Ces aspects relÃ¨vent de KindMother et du produit.

---

## 10. Erreurs et rejets

### 10.1 Rejet de trace incomplÃ¨te

Si une tentative de persistance de trace ne respecte pas la structure TAMR (champ obligatoire manquant, type invalide, override sans justification), KindMother (ou le composant qui valide avant Ã©criture) doit rejeter l'Ã©criture et signaler l'erreur.

**RÃ¨gle TAMR-KM-09 : Validation avant persistance**

La conformitÃ© Ã  la structure TAMR est vÃ©rifiÃ©e avant toute persistance. Une trace non conforme n'est pas persistÃ©e.

### 10.2 TAMR ne gÃ¨re pas les erreurs techniques

Les erreurs de persistance (indisponibilitÃ©, timeout, conflit de sync) sont gÃ©rÃ©es par KindMother et le produit. TAMR ne dÃ©finit pas de politique de retry ni de gestion d'erreur technique.

---

## 11. Invariants d'intÃ©gration

**INV-TAMR-KM-1 : TAMR ne persiste pas**

TAMR ne peut jamais effectuer d'opÃ©ration de persistance, lecture ou suppression sur les traces. INTERD-TAMR-2.

**INV-TAMR-KM-2 : Structure TAMR, stockage KindMother**

La structure conceptuelle des traces est dÃ©finie par TAMR ; le stockage, la cohÃ©rence et la synchronisation sont assurÃ©s par KindMother.

**INV-TAMR-KM-3 : Traces conformes**

Toute trace d'intervention persistÃ©e est conforme au schÃ©ma conceptuel TAMR (champs obligatoires, justification pour override).

---

## 12. Exemple conceptuel

### 12.1 Trace d'approbation

```text
TraceIntervention {
  trace_id       : "trace-001"
  intervenant_id : "user-456"
  type           : APPROVAL
  moment         : "2026-01-28T14:00:00Z"
  contexte       : { point_id: "approval-publication-1", entitÃ©_id: "article-789" }
  rÃ©sultat       : APPROUVÃ‰
  justification  : (optionnel)
}
```

### 12.2 Trace d'override

```text
TraceIntervention {
  trace_id       : "trace-002"
  intervenant_id : "user-456"
  type           : OVERRIDE
  moment         : "2026-01-28T14:05:00Z"
  contexte       : { dÃ©cision_automatique: "REFUSÃ‰", entitÃ©_id: "demande-012" }
  rÃ©sultat       : OVERRIDE_ACCEPTÃ‰
  justification  : "Exception client validÃ©e par le responsable, dossier documentÃ©."
}
```

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit la rÃ©partition des responsabilitÃ©s entre TAMR et KindMother pour la persistance des traces d'intervention.

Toute implÃ©mentation qui persiste des traces d'intervention humaine doit respecter ce contrat : structure dÃ©finie par TAMR, persistance assurÃ©e par KindMother, sans persistance par TAMR.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :**
- [TAMR - Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) (Relation KindMother, exigences de traÃ§abilitÃ©)
- [TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)
- [TAMR - Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md)
- [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)
- KindMother â€” Documentation Fondatrice (autoritÃ© des donnÃ©es)

