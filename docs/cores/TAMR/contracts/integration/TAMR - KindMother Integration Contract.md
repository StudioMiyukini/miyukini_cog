# TAMR - KindMother Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre TAMR et KindMother** pour la persistance des traces d'intervention humaine. Il spécifie la répartition des responsabilités : TAMR définit la structure conceptuelle des traces d'intervention, KindMother assure leur persistance selon ses mécanismes.

Ce document complète la section « Relation avec KindMother » de la [Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md) pour les types d'intervention à tracer
- [TAMR - Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md) pour INV-TAMR-1 (traçabilité absolue)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) pour la terminologie TAMR
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) pour LOI-2, LOI-3 (traces locales, synchronisation différée)
- [Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) pour les principes de sécurité
- [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) pour les niveaux T0-T4
- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) pour les niveaux 0-4

L'intégration respecte les Lois d'Autonomie Système : les traces d'intervention sont persistées localement et peuvent être synchronisées à la reconnexion (**LOI-2**, **LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- La répartition des responsabilités TAMR (structure) / KindMother (persistance)
- La structure conceptuelle des traces d'intervention définie par TAMR
- Les types de traces (approbation, override, escalade, supervision)
- Les garanties de persistance et de traçabilité
- Le comportement en mode offline

Ce document **ne couvre pas** :
- Les détails internes de KindMother (voir documentation KindMother)
- La médiation des intentions d'intervention (voir [TAMR - BondingBrother Integration Contract](./TAMR%20-%20BondingBrother%20Integration%20Contract.md))
- La décision d'autoriser une intervention (voir [TAMR - StrongFather Integration Contract](./TAMR%20-%20StrongFather%20Integration%20Contract.md))
- L'implémentation technique des interfaces de persistance

---

## 3. Principe fondamental

**TAMR définit ce qui doit être tracé lors d'une intervention humaine. KindMother persiste ces traces. TAMR ne persiste rien ; KindMother ne définit pas la structure des traces d'intervention.**

La relation est unidirectionnelle et complémentaire :
- **TAMR** : définit la structure conceptuelle des traces (identité intervenant, type, moment, contexte, justification si requise, résultat)
- **KindMother** : gère la persistance, la cohérence, la rétention et la synchronisation des données de traces selon ses mécanismes

Les traces d'intervention sont des données comme les autres du point de vue de KindMother ; leur schéma conceptuel est imposé par TAMR.

---

## 4. Positionnement de KindMother

### 4.1 Autorité sur la persistance des traces

**KindMother est l'autorité pour :**
- La persistance des traces d'intervention
- La cohérence et l'intégrité des données de traces
- La rétention et l'archivage (selon politiques produit)
- La synchronisation des traces entre instances (offline-first)

**Règle TAMR-KM-01 : TAMR ne persiste pas**

TAMR ne peut jamais écrire, modifier ou supprimer de données. Toute persistance de trace d'intervention est déléguée à KindMother (via le produit ou BondingBrother).

**Règle TAMR-KM-02 : Structure définie par TAMR**

La structure conceptuelle des traces (champs obligatoires, sémantique, types d'intervention) est définie par TAMR. KindMother persiste des données conformes à cette structure.

**Règle TAMR-KM-03 : Pas de définition de contenu par KindMother**

KindMother ne définit pas ce qu'est une trace d'intervention ni quels champs sont requis. Elle applique le schéma dérivé des exigences TAMR.

---

## 5. Structure conceptuelle des traces (définie par TAMR)

### 5.1 Champs obligatoires

Toute trace d'intervention persistée par KindMother doit contenir au minimum les éléments suivants, définis par TAMR (voir Documentation Fondatrice, section « Définition des exigences de traçabilité ») :

| Élément | Description | Source TAMR |
|--------|-------------|-------------|
| **Identité de l'intervenant** | Qui a effectué l'intervention (référence fournie par le produit/auth) | INV-TAMR-1, INV-TAMR-2 |
| **Type d'intervention** | APPROVAL, OVERRIDE, ESCALATION, SUPERVISION | Types Contract |
| **Moment de l'intervention** | Horodatage (local) | INV-TAMR-1 |
| **Contexte de l'intervention** | Point d'intervention, processus, entité concernée | Documentation Fondatrice |
| **Résultat de l'intervention** | Décision prise (approuvé, refusé, overridé, escaladé, etc.) | INV-TAMR-1 |
| **Justification** | Obligatoire pour OVERRIDE ; optionnelle pour les autres types | INV-TAMR-7 |

### 5.2 Structure conceptuelle (schéma cible)

```text
TraceIntervention {
  trace_id          : Identifiant unique (géré par KindMother ou produit)
  intervenant_id    : Identité de l'humain intervenant
  type              : APPROVAL | OVERRIDE | ESCALATION | SUPERVISION
  moment            : Timestamp (local)
  contexte          : { point_id?, processus_id?, entité_id?, ... }
  résultat          : Résultat de l'intervention
  justification?    : Obligatoire si type = OVERRIDE
  métadonnées?      : Données additionnelles (sans modifier la sémantique TAMR)
}
```

**Règle TAMR-KM-04 : Exhaustivité des champs obligatoires**

Aucune trace ne peut être persistée sans les champs obligatoires définis par TAMR. KindMother (ou le produit qui alimente KindMother) doit rejeter ou refuser toute écriture incomplète.

**Règle TAMR-KM-05 : Justification pour override**

Si le type d'intervention est OVERRIDE, le champ justification doit être renseigné. C'est une exigence TAMR (INV-TAMR-7), respectée avant persistance.

---

## 6. Types de traces et sémantique

### 6.1 Par type d'intervention

| Type | Trace typique | Justification |
|------|----------------|---------------|
| **APPROVAL** | Approbation / refus d'une action proposée | Optionnelle |
| **OVERRIDE** | Décision de déroger à une décision automatique | **Obligatoire** |
| **ESCALATION** | Montée en niveau, délégation, résolution | Optionnelle (contexte souvent suffisant) |
| **SUPERVISION** | Début/fin de supervision, intervention déclenchée | Optionnelle |

### 6.2 Règles de cohérence

**Règle TAMR-KM-06 : Unicité de type par trace**

Une trace correspond à une et une seule intervention, donc un seul type parmi les quatre définis par TAMR.

**Règle TAMR-KM-07 : Pas de modification rétroactive du sens**

Une fois une trace persistée, sa sémantique (type, résultat, intervenant, moment) ne doit pas être altérée pour des raisons métier. Les corrections techniques (correction de faute, mise à jour technique) restent sous la responsabilité de KindMother et des politiques produit, sans contredire l'invariant de traçabilité (INV-TAMR-1).

---

## 7. Flux de persistance des traces

### 7.1 Responsabilité du flux

1. Une intervention humaine a lieu (approbation, override, escalade, supervision).
2. Le produit (ou BondingBrother) produit un enregistrement de trace conforme à la structure TAMR.
3. Ce enregistrement est transmis à KindMother pour persistance (selon les mécanismes du produit et de KindMother).
4. KindMother persiste la trace et assure cohérence, rétention, synchronisation.

**TAMR** intervient uniquement en amont : il définit la structure et les règles (champs obligatoires, justification pour override). **TAMR ne participe pas au flux d'écriture.**

### 7.2 Diagramme de responsabilités

```text
┌─────────────┐     structure      ┌─────────────┐     écriture      ┌─────────────┐
│    TAMR     │ ─────────────────► │  Produit /  │ ───────────────► │ KindMother  │
│ (conceptuel)│  (contrat, schéma) │ BondingBrother│  (données trace) │ (persistance)│
└─────────────┘                    └─────────────┘                  └─────────────┘
```

---

## 8. Garanties de l'intégration

### 8.1 Garantie de séparation des rôles

**Engagement :** TAMR ne persiste jamais ; KindMother ne définit jamais la structure des traces d'intervention. Les rôles restent strictement séparés.

### 8.2 Garantie de conformité structurelle

**Engagement :** Les données de traces persistées par KindMother sont conformes à la structure conceptuelle définie par TAMR (champs obligatoires, types, justification pour override).

### 8.3 Garantie de traçabilité

**Engagement :** Toute intervention humaine donnant lieu à une trace conforme à TAMR est persistée par KindMother sans perte des champs obligatoires. INV-TAMR-1 (traçabilité absolue) est respecté au niveau conceptuel ; KindMother en est le support de persistance.

### 8.4 Garantie de non-modification par TAMR

**Engagement :** TAMR ne modifie, ne lit ni ne supprime aucune donnée. Il ne fait que définir le contrat de structure et les règles de traçabilité.

---

## 9. Mode offline et synchronisation

### 9.1 Persistance locale

Conformément à **LOI-2** et **LOI-3**, les traces d'intervention peuvent être produites et persistées localement (DB Fille, stockage local). KindMother gère la persistance locale et la synchronisation ultérieure vers la DB Mère selon ses propres règles.

**Règle TAMR-KM-08 : Traces locales valides**

Les traces créées en mode isolé sont valides localement et doivent être conservées jusqu'à synchronisation. La structure TAMR s'applique identiquement en local et après synchronisation.

### 9.2 Pas d'exigence technique TAMR

TAMR ne définit pas de protocole de synchronisation, de timeout ni de stratégie de conflit. Ces aspects relèvent de KindMother et du produit.

---

## 10. Erreurs et rejets

### 10.1 Rejet de trace incomplète

Si une tentative de persistance de trace ne respecte pas la structure TAMR (champ obligatoire manquant, type invalide, override sans justification), KindMother (ou le composant qui valide avant écriture) doit rejeter l'écriture et signaler l'erreur.

**Règle TAMR-KM-09 : Validation avant persistance**

La conformité à la structure TAMR est vérifiée avant toute persistance. Une trace non conforme n'est pas persistée.

### 10.2 TAMR ne gère pas les erreurs techniques

Les erreurs de persistance (indisponibilité, timeout, conflit de sync) sont gérées par KindMother et le produit. TAMR ne définit pas de politique de retry ni de gestion d'erreur technique.

---

## 11. Invariants d'intégration

**INV-TAMR-KM-1 : TAMR ne persiste pas**

TAMR ne peut jamais effectuer d'opération de persistance, lecture ou suppression sur les traces. INTERD-TAMR-2.

**INV-TAMR-KM-2 : Structure TAMR, stockage KindMother**

La structure conceptuelle des traces est définie par TAMR ; le stockage, la cohérence et la synchronisation sont assurés par KindMother.

**INV-TAMR-KM-3 : Traces conformes**

Toute trace d'intervention persistée est conforme au schéma conceptuel TAMR (champs obligatoires, justification pour override).

---

## 12. Exemple conceptuel

### 12.1 Trace d'approbation

```text
TraceIntervention {
  trace_id       : "trace-001"
  intervenant_id : "user-456"
  type           : APPROVAL
  moment         : "2026-01-28T14:00:00Z"
  contexte       : { point_id: "approval-publication-1", entité_id: "article-789" }
  résultat       : APPROUVÉ
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
  contexte       : { décision_automatique: "REFUSÉ", entité_id: "demande-012" }
  résultat       : OVERRIDE_ACCEPTÉ
  justification  : "Exception client validée par le responsable, dossier documenté."
}
```

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit la répartition des responsabilités entre TAMR et KindMother pour la persistance des traces d'intervention.

Toute implémentation qui persiste des traces d'intervention humaine doit respecter ce contrat : structure définie par TAMR, persistance assurée par KindMother, sans persistance par TAMR.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- [TAMR - Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md) (Relation KindMother, exigences de traçabilité)
- [TAMR - Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)
- [TAMR - Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Miyukini Conceptual References - Doctrine Securite Fondamentale](../../../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)
- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- KindMother — Documentation Fondatrice (autorité des données)
