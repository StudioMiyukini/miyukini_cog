# Master Butler - LogisticsSteward Integration Contract

## 1. Contexte

Ce document definit le **contrat d'integration entre Master Butler et LogisticsSteward**. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a l'integration avec LogisticsSteward en tant qu'arbitre de l'allocation et de la priorisation des ressources.

Ce document complete la Section 3 de la [Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [Master Butler - Capability API Contract](../api/Master%20Butler%20-%20Capability%20API%20Contract.md) pour l'API des capacites
- [Master Butler - Discovery API Contract](../api/Master%20Butler%20-%20Discovery%20API%20Contract.md) pour l'API de decouverte
- [LogisticsSteward - Documentation Fondatrice](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) pour la nature de LogisticsSteward

L'integration respecte les [Lois d'Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : toutes les interrogations sont locales et ne requierent aucune dependance externe (**LOI-1**).

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre Master Butler et LogisticsSteward
- Le protocole de communication (interrogations et reponses)
- Les types d'interrogations acceptees par Master Butler pour LogisticsSteward
- La separation stricte entre exposition et gouvernance
- La gestion des erreurs et des reponses
- Les garanties de l'integration

Ce document **ne couvre pas** :
- Les details internes de LogisticsSteward (voir documentation LogisticsSteward)
- Les details internes des registres (voir Capability Registry Contract, Permission Registry Contract)
- Les regles d'arbitrage de LogisticsSteward (voir Quota Definition Contract, Priority Management Contract)
- L'integration avec StrongFather (voir StrongFather Integration Contract)

---

## 3. Principe fondamental

**Master Butler expose les capacites existantes dans le systeme. LogisticsSteward gouverne l'usage de ces capacites sans jamais modifier leur existence. La separation est absolue : Master Butler dit "quoi existe", LogisticsSteward dit "qui peut utiliser quoi, quand, et a quelle priorite".**

La relation est de consultation : LogisticsSteward interroge Master Butler pour connaitre les capacites disponibles avant d'arbitrer leur usage. Master Butler repond avec les informations demandees sans jamais participer a l'arbitrage.

---

## 4. Nature de la relation Master Butler — LogisticsSteward

### 4.1 Relation de consultation

**Master Butler est consulte par LogisticsSteward :**
- Pour connaitre les capacites existantes dans le systeme
- Pour obtenir les metadonnees des capacites (ressources consommees, caracteristiques)
- Pour decouvrir les Tools et Toolkits disponibles
- Pour calculer l'impact potentiel d'une operation sur les ressources

**Regle MB-LS-01 : Exposition sans gouvernance**

Master Butler ne participe jamais aux decisions d'arbitrage de LogisticsSteward. Il fournit des informations factuelles sur les capacites existantes, sans recommandation d'allocation, sans interpretation de priorite, sans jugement sur l'usage.

**Regle MB-LS-02 : Exhaustivite des reponses**

Les reponses de Master Butler a LogisticsSteward sont exhaustives. Aucune capacite pertinente n'est omise ou filtree.

**Regle MB-LS-03 : Metadonnees de ressources**

Master Butler fournit les metadonnees de ressources associees aux capacites (cout estimatif, categorie de ressource, caracteristiques de consommation) sans jamais mesurer les ressources reelles.

### 4.2 Separation des responsabilites

| Responsabilite | Master Butler | LogisticsSteward |
|----------------|---------------|------------------|
| **Connaitre les capacites** | ✅ Exclusif | ❌ Interroge |
| **Exposer les metadonnees** | ✅ Exclusif | ❌ Consomme |
| **Decider de l'allocation** | ❌ Jamais | ✅ Exclusif |
| **Definir les priorites** | ❌ Jamais | ✅ Exclusif |
| **Appliquer les quotas** | ❌ Jamais | ✅ Exclusif |
| **Cataloguer les Tools** | ✅ Exclusif | ❌ Consulte |
| **Limiter l'usage des Tools** | ❌ Jamais | ✅ Exclusif |

**Regle MB-LS-04 : Aucun chevauchement**

Aucun chevauchement de responsabilites n'est autorise. Master Butler ne prend jamais de decision d'allocation, LogisticsSteward ne maintient jamais de registre de capacites.

**Regle MB-LS-05 : Existence vs Usage**

Master Butler gere l'existence des capacites. LogisticsSteward gere l'usage de ces capacites. Une capacite peut exister (registre Master Butler) mais etre interdite d'usage (decision LogisticsSteward).

---

## 5. Types d'interrogations

### 5.1 Interrogation des capacites disponibles

**AVAILABLE_CAPABILITIES**
- **Objectif :** Obtenir la liste des capacites existantes dans le systeme
- **Payload :** Filtres optionnels (module, categorie, type de ressource)
- **Reponse :** Liste des capacites avec leurs metadonnees de ressources

**Regle MB-LS-QUERY-01 : Liste complete**

La liste retournee inclut toutes les capacites correspondant aux criteres, sans filtrage base sur des considerations d'allocation.

### 5.2 Interrogation des metadonnees de ressources

**CAPABILITY_RESOURCE_METADATA**
- **Objectif :** Obtenir les metadonnees de ressources d'une capacite specifique
- **Payload :** Identifiant de la capacite
- **Reponse :** Metadonnees de ressources (cout estimatif, categorie, caracteristiques)

**Regle MB-LS-QUERY-02 : Metadonnees declaratives**

Les metadonnees de ressources sont declaratives et estimatives. Elles ne refletent pas la consommation reelle (responsabilite du Kernel) mais les caracteristiques declarees de la capacite.

**Structure des metadonnees de ressources :**

| Champ | Description | Exemple |
|-------|-------------|---------|
| `resource_category` | Categorie de ressource | `compute`, `storage`, `network`, `memory` |
| `estimated_cost` | Cout estimatif relatif | `low`, `medium`, `high`, `critical` |
| `burst_capable` | Capacite de pic | `true`, `false` |
| `concurrent_limit` | Limite de concurrence declaree | `1`, `10`, `unlimited` |
| `duration_profile` | Profil de duree | `instant`, `short`, `long`, `persistent` |

### 5.3 Interrogation des Tools et Toolkits

**TOOL_RESOURCE_PROFILE**
- **Objectif :** Obtenir le profil de ressources d'un Tool
- **Payload :** Identifiant du Tool
- **Reponse :** Profil de ressources (capacites utilisees, cout, caracteristiques)

**TOOLKIT_RESOURCE_PROFILE**
- **Objectif :** Obtenir le profil de ressources agrege d'un Toolkit
- **Payload :** Identifiant du Toolkit
- **Reponse :** Profil agrege (somme des Tools, caracteristiques globales)

**Regle MB-LS-QUERY-03 : Agregation sans interpretation**

L'agregation des profils de ressources est une somme factuelle. Master Butler n'interprete pas l'impact global sur les ressources systeme.

### 5.4 Interrogation de l'impact potentiel

**OPERATION_IMPACT_ESTIMATE**
- **Objectif :** Estimer l'impact d'une operation sur les ressources
- **Payload :** Identifiant de la capacite, parametres de l'operation
- **Reponse :** Estimation d'impact (ressources concernees, cout estimatif)

**Regle MB-LS-QUERY-04 : Estimation sans garantie**

L'estimation d'impact est basee sur les metadonnees declaratives. Elle ne constitue pas une prediction precise de la consommation reelle.

### 5.5 Interrogation des capacites par entite

**ENTITY_CAPABILITIES**
- **Objectif :** Obtenir les capacites declarees par une entite (Operateur, Service)
- **Payload :** Identifiant de l'entite
- **Reponse :** Liste des capacites declarees avec leurs metadonnees

**Regle MB-LS-QUERY-05 : Entite connue uniquement**

Master Butler connait les capacites declarees par les entites enregistrees. Il ne connait pas les allocations ou priorites accordees par LogisticsSteward.

### 5.6 Regles generales d'interrogation

**Regle MB-LS-QUERY-06 : Toute interrogation est sans etat**

Les interrogations de LogisticsSteward ne modifient jamais l'etat de Master Butler. Ce sont des lectures pures.

**Regle MB-LS-QUERY-07 : Pas d'effet de bord**

Aucune interrogation ne produit d'effet de bord sur le registre, les metadonnees, ou les associations.

**Regle MB-LS-QUERY-08 : Reponse immediate**

Les reponses sont fournies immediatement. Aucune interrogation n'est mise en attente ou differee.

---

## 6. Protocole de communication

### 6.1 Format des interrogations

Les interrogations de LogisticsSteward suivent un format standardise.

**Structure de base :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `interrogation_id` | Identifiant unique de l'interrogation | ✅ Oui |
| `arbitrage_id` | Reference a l'arbitrage en cours | ✅ Oui |
| `type` | Type d'interrogation | ✅ Oui |
| `payload` | Donnees specifiques a l'interrogation | ✅ Oui |
| `contexte_appelant` | Contexte de LogisticsSteward | ✅ Oui |
| `timestamp` | Horodatage de l'interrogation | ✅ Oui |

**Regle MB-LS-PROT-01 : Format standardise**

Toutes les interrogations respectent le format standardise. Aucune interrogation ad-hoc n'est acceptee.

**Regle MB-LS-PROT-02 : Tracabilite par arbitrage**

Chaque interrogation reference l'arbitrage en cours d'evaluation pour assurer la tracabilite bout-en-bout.

### 6.2 Format des reponses

Les reponses de Master Butler suivent un format standardise.

**Structure de base :**

| Element | Description | Obligatoire |
|---------|-------------|-------------|
| `reponse_id` | Identifiant unique de la reponse | ✅ Oui |
| `interrogation_id` | Reference a l'interrogation | ✅ Oui |
| `statut` | Statut de la reponse (SUCCESS, NOT_FOUND, ERROR) | ✅ Oui |
| `donnees` | Donnees de la reponse | Si SUCCESS |
| `erreur` | Details de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la reponse | ✅ Oui |

**Regle MB-LS-PROT-03 : Reponse toujours structuree**

Master Butler retourne toujours une reponse structuree, meme en cas d'erreur ou de non-existence.

**Regle MB-LS-PROT-04 : Pas d'interpretation**

Les reponses sont des informations brutes. Master Butler n'interprete pas les donnees pour LogisticsSteward.

### 6.3 Statuts de reponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | L'interrogation a abouti, les donnees sont fournies |
| `NOT_FOUND` | L'element recherche n'existe pas dans le registre |
| `INVALID_QUERY` | L'interrogation est mal formee ou incomplete |
| `ERROR` | Une erreur interne s'est produite |

**Regle MB-LS-PROT-05 : NOT_FOUND n'est pas une erreur**

Le statut `NOT_FOUND` est une reponse valide, pas une erreur. Il indique que l'element recherche n'existe pas dans le registre.

---

## 7. Flux d'interrogation typique

### 7.1 Flux d'arbitrage d'allocation

**Acteurs :** Entite demandeur, LogisticsSteward, Master Butler, StrongFather

**Sequence :**

1. Une entite demande l'acces a une ressource conceptuelle
2. LogisticsSteward recoit la demande d'arbitrage
3. LogisticsSteward interroge Master Butler : `CAPABILITY_EXISTS`
4. Master Butler repond avec l'existence et les metadonnees
5. LogisticsSteward interroge Master Butler : `CAPABILITY_RESOURCE_METADATA`
6. Master Butler repond avec les metadonnees de ressources
7. LogisticsSteward evalue les regles de quota et priorite
8. LogisticsSteward soumet sa decision a StrongFather pour validation
9. StrongFather valide ou invalide la decision
10. La decision validee est executee par le Kernel

**Regle MB-LS-FLOW-01 : Interrogations multiples possibles**

LogisticsSteward peut effectuer plusieurs interrogations pour un meme arbitrage. Master Butler repond a chacune independamment.

### 7.2 Flux de decouverte des capacites

**Acteurs :** LogisticsSteward, Master Butler

**Sequence :**

1. LogisticsSteward a besoin de connaitre les capacites disponibles
2. LogisticsSteward interroge Master Butler : `AVAILABLE_CAPABILITIES`
3. Master Butler retourne la liste des capacites avec metadonnees
4. LogisticsSteward utilise ces informations pour ses regles d'arbitrage

### 7.3 Diagramme de sequence

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│     Entite      │    │ LogisticsSteward │    │  Master Butler  │
└────────┬────────┘    └────────┬─────────┘    └────────┬────────┘
         │                      │                       │
         ├── Demande acces ────►│                       │
         │                      │                       │
         │                      ├── CAPABILITY_EXISTS ─►│
         │                      │                       │
         │                      │◄── Existence + Meta ──┤
         │                      │                       │
         │                      ├── RESOURCE_METADATA ─►│
         │                      │                       │
         │                      │◄── Metadonnees ───────┤
         │                      │                       │
         │                      ├── Evaluation regles ──┤
         │                      │   (interne)           │
         │                      │                       │
         │                      ├── Decision ──────────►│
         │                      │   (vers StrongFather) │
         │                      │                       │
         │◄── Resultat ─────────┤                       │
         │                      │                       │
```

---

## 8. Regles d'integration

### 8.1 Regles de communication

**Regle MB-LS-INT-01 : LogisticsSteward initie toujours**

LogisticsSteward initie toujours les interrogations. Master Butler ne contacte jamais LogisticsSteward spontanement.

**Regle MB-LS-INT-02 : Pas de notification de quotas**

Master Butler ne connait pas les quotas ou limites definis par LogisticsSteward. Il ne peut pas notifier de depassement.

**Regle MB-LS-INT-03 : Synchronisme des reponses**

Les reponses de Master Butler sont synchrones. LogisticsSteward attend la reponse avant de poursuivre l'arbitrage.

### 8.2 Regles de donnees

**Regle MB-LS-INT-04 : Donnees fraiches**

Les donnees retournees par Master Butler refletent l'etat actuel du registre au moment de l'interrogation.

**Regle MB-LS-INT-05 : Pas de cache cote LogisticsSteward**

LogisticsSteward ne met pas en cache les reponses de Master Butler pour les decisions d'arbitrage. Chaque arbitrage necessite de nouvelles interrogations pour garantir la fraicheur.

**Regle MB-LS-INT-06 : Coherence garantie**

Master Butler garantit la coherence des donnees retournees. Les informations sur une capacite et ses metadonnees sont coherentes entre elles.

### 8.3 Regles de tracabilite

**Regle MB-LS-INT-07 : Tracabilite des interrogations**

Toutes les interrogations de LogisticsSteward sont tracees par Master Butler avec le contexte complet.

**Regle MB-LS-INT-08 : Correlation arbitrage-interrogation**

Chaque interrogation est correlee a l'arbitrage en cours d'evaluation pour permettre l'audit bout-en-bout.

---

## 9. Gestion des erreurs

### 9.1 Types d'erreurs

**Erreurs de format :**
- Interrogation mal formee
- Champ obligatoire manquant
- Type d'interrogation inconnu

**Erreurs de donnees :**
- Capacite inexistante (NOT_FOUND, pas une erreur)
- Tool inexistant (NOT_FOUND, pas une erreur)
- Entite inconnue

**Erreurs internes :**
- Erreur de registre
- Erreur de calcul d'agregation

### 9.2 Traitement des erreurs

**Regle MB-LS-ERR-01 : Reponse structuree toujours**

Master Butler retourne toujours une reponse structuree, meme en cas d'erreur. LogisticsSteward peut toujours interpreter la reponse.

**Regle MB-LS-ERR-02 : NOT_FOUND est informatif**

Le statut `NOT_FOUND` est une information, pas une erreur. LogisticsSteward peut utiliser cette information dans son arbitrage (capacite inexistante = pas d'allocation possible).

**Regle MB-LS-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisees par Master Butler pour audit et diagnostic.

**Regle MB-LS-ERR-04 : Pas de retry automatique**

En cas d'erreur, LogisticsSteward decide de la strategie (retry, echec de l'arbitrage). Master Butler ne retry jamais automatiquement.

---

## 10. Garanties de l'integration

### 10.1 Garantie d'exhaustivite

**Engagement :** Les reponses de Master Butler sont exhaustives. Toutes les capacites pertinentes sont fournies sans omission.

### 10.2 Garantie d'exactitude

**Engagement :** Les informations fournies par Master Butler sont exactes et refletent l'etat actuel du registre.

### 10.3 Garantie de neutralite

**Engagement :** Master Butler fournit des informations sans interpretation d'allocation, sans recommandation de priorite, sans jugement sur l'usage. L'arbitrage appartient exclusivement a LogisticsSteward.

### 10.4 Garantie de tracabilite

**Engagement :** Toute interaction entre LogisticsSteward et Master Butler est tracable de bout en bout. L'audit complet des interrogations et reponses est possible.

### 10.5 Garantie de disponibilite

**Engagement :** Master Butler est disponible pour repondre aux interrogations de LogisticsSteward sans dependance externe (conformite LOI-1).

### 10.6 Garantie de separation

**Engagement :** Master Butler ne participe jamais aux decisions d'allocation ou de priorite. La separation entre exposition (Master Butler) et gouvernance (LogisticsSteward) est absolue.

---

## 11. Invariants de l'integration

### 11.1 Invariants de relation

**INV-MB-LS-1 : Consultation unidirectionnelle**

LogisticsSteward interroge Master Butler. Master Butler ne sollicite jamais LogisticsSteward.

**INV-MB-LS-2 : Information sans arbitrage**

Master Butler fournit des informations. Il ne participe jamais aux decisions d'allocation de LogisticsSteward.

**INV-MB-LS-3 : Separation existence/usage**

Master Butler gere l'existence des capacites. LogisticsSteward gere leur usage. Aucun chevauchement n'est permis.

### 11.2 Invariants de donnees

**INV-MB-LS-4 : Lecture pure**

Les interrogations sont des lectures pures. Aucune modification du registre n'est causee par une interrogation.

**INV-MB-LS-5 : Donnees factuelles**

Les donnees retournees sont factuelles (existe/n'existe pas, metadonnees declaratives). Aucune donnee interpretee n'est retournee.

**INV-MB-LS-6 : Metadonnees declaratives**

Les metadonnees de ressources sont declaratives et estimatives. Elles ne refletent pas la consommation reelle (responsabilite du Kernel).

### 11.3 Invariants de protocole

**INV-MB-LS-7 : Format respecte**

Toutes les interrogations et reponses respectent le format standardise.

**INV-MB-LS-8 : Tracabilite complete**

Toute interaction est tracable avec son contexte complet.

---

## 12. Exemples

### 12.1 Interrogation des metadonnees de ressources

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-001",
  "arbitrage_id": "arb-300",
  "type": "CAPABILITY_RESOURCE_METADATA",
  "payload": {
    "capability_id": "content.render.heavy"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "entite_demandeur": "operator-cms-001"
  },
  "timestamp": "2026-01-28T10:00:00Z"
}
```

**Reponse Master Butler :**
```
{
  "reponse_id": "resp-mb-ls-001",
  "interrogation_id": "int-ls-001",
  "statut": "SUCCESS",
  "donnees": {
    "capability_id": "content.render.heavy",
    "resource_metadata": {
      "resource_category": "compute",
      "estimated_cost": "high",
      "burst_capable": true,
      "concurrent_limit": 5,
      "duration_profile": "long"
    }
  },
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 12.2 Interrogation des capacites disponibles

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-002",
  "arbitrage_id": "arb-301",
  "type": "AVAILABLE_CAPABILITIES",
  "payload": {
    "filters": {
      "resource_category": "compute",
      "estimated_cost": ["medium", "high"]
    }
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "raison": "degradation_d2"
  },
  "timestamp": "2026-01-28T10:05:00Z"
}
```

**Reponse Master Butler :**
```
{
  "reponse_id": "resp-mb-ls-002",
  "interrogation_id": "int-ls-002",
  "statut": "SUCCESS",
  "donnees": {
    "capabilities": [
      {
        "id": "content.render.heavy",
        "resource_category": "compute",
        "estimated_cost": "high"
      },
      {
        "id": "media.transcode",
        "resource_category": "compute",
        "estimated_cost": "high"
      },
      {
        "id": "search.index.full",
        "resource_category": "compute",
        "estimated_cost": "medium"
      }
    ],
    "total_count": 3
  },
  "timestamp": "2026-01-28T10:05:02Z"
}
```

### 12.3 Interrogation du profil de ressources d'un Toolkit

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-003",
  "arbitrage_id": "arb-302",
  "type": "TOOLKIT_RESOURCE_PROFILE",
  "payload": {
    "toolkit_id": "cms-publishing-toolkit"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "entite_demandeur": "operator-cms-001"
  },
  "timestamp": "2026-01-28T10:10:00Z"
}
```

**Reponse Master Butler :**
```
{
  "reponse_id": "resp-mb-ls-003",
  "interrogation_id": "int-ls-003",
  "statut": "SUCCESS",
  "donnees": {
    "toolkit_id": "cms-publishing-toolkit",
    "tools": [
      {
        "id": "content.validate",
        "estimated_cost": "low"
      },
      {
        "id": "content.render.preview",
        "estimated_cost": "medium"
      },
      {
        "id": "content.publish",
        "estimated_cost": "low"
      }
    ],
    "aggregated_profile": {
      "total_estimated_cost": "medium",
      "resource_categories": ["compute", "storage"],
      "max_concurrent": 10
    }
  },
  "timestamp": "2026-01-28T10:10:03Z"
}
```

### 12.4 Capacite inexistante

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-004",
  "arbitrage_id": "arb-303",
  "type": "CAPABILITY_RESOURCE_METADATA",
  "payload": {
    "capability_id": "nonexistent.capability"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "entite_demandeur": "operator-unknown"
  },
  "timestamp": "2026-01-28T10:15:00Z"
}
```

**Reponse Master Butler :**
```
{
  "reponse_id": "resp-mb-ls-004",
  "interrogation_id": "int-ls-004",
  "statut": "NOT_FOUND",
  "donnees": {
    "exists": false,
    "capability_id": "nonexistent.capability"
  },
  "timestamp": "2026-01-28T10:15:01Z"
}
```

**Note :** LogisticsSteward peut utiliser cette information pour refuser l'arbitrage (capacite inexistante = pas d'allocation possible).

---

## 13. Cas particuliers

### 13.1 Mode degradation

Lorsque LogisticsSteward est en mode degradation (D1 a D4), il peut interroger Master Butler pour :
- Identifier les capacites a cout eleve a restreindre
- Determiner les capacites critiques a preserver
- Calculer l'impact de la desactivation d'un Toolkit

Master Butler fournit ces informations sans connaitre le niveau de degradation actuel.

### 13.2 Nouvelles capacites

Lorsqu'une nouvelle capacite est declaree dans Master Butler :
- LogisticsSteward n'est pas notifie automatiquement
- LogisticsSteward decouvre la nouvelle capacite lors de ses interrogations
- Les quotas par defaut s'appliquent jusqu'a definition explicite

### 13.3 Capacites supprimees

Lorsqu'une capacite est supprimee de Master Butler :
- LogisticsSteward recoit `NOT_FOUND` lors de l'interrogation
- Les allocations existantes deviennent invalides
- LogisticsSteward doit gerer la transition gracieusement

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole que Master Butler doit respecter pour s'integrer avec LogisticsSteward.

Toute implementation de l'integration avec LogisticsSteward doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dependances :**
- Master Butler - Documentation Fondatrice v1.4 (Section 3)
- Master Butler - Capability API Contract v1.0
- Master Butler - Discovery API Contract v1.0
- LogisticsSteward - Documentation Fondatrice v1.0.0

---

## 15. Mini log de generation

### Decision editoriale E1 : Direction de la relation

**Decision prise :** La relation est de consultation : LogisticsSteward interroge, Master Butler repond. Cette direction est similaire a la relation StrongFather → Master Butler.

**Application :** Tout le document est structure autour de cette direction unidirectionnelle.

### Decision editoriale E2 : Metadonnees de ressources

**Decision prise :** Master Butler fournit des metadonnees de ressources declaratives (cout estimatif, categorie, caracteristiques) sans jamais mesurer les ressources reelles. Cette separation preserve les responsabilites du Kernel.

**Application :** Section 5.2 definit la structure des metadonnees de ressources avec des champs estimatifs.

### Decision editoriale E3 : Separation existence/usage

**Decision prise :** La separation entre existence (Master Butler) et usage (LogisticsSteward) est explicitement definie comme absolue. Une capacite peut exister mais etre interdite d'usage.

**Application :** Regle MB-LS-05 et INV-MB-LS-3 formalisent cette separation.

### Warning W1 : Confusion metadonnees/mesures

**Warning rencontre :** Risque de confusion entre metadonnees declaratives de Master Butler et mesures reelles du Kernel.

**Decision prise :** Les metadonnees sont explicitement definies comme declaratives et estimatives. La mesure reelle est exclusivement la responsabilite du Kernel.

**Correction effectuee :** Regle MB-LS-QUERY-02 et INV-MB-LS-6 clarifient cette distinction.

### Warning W2 : Notification de nouvelles capacites

**Warning rencontre :** LogisticsSteward pourrait avoir besoin d'etre notifie lors de l'ajout de nouvelles capacites.

**Decision prise :** Pas de notification automatique. LogisticsSteward decouvre les nouvelles capacites lors de ses interrogations. Les quotas par defaut s'appliquent.

**Correction effectuee :** Section 13.2 documente ce comportement.

### Verification de coherence

**Verification effectuee :**
- ✅ Coherence avec Master Butler - Documentation Fondatrice : Confirmee (role de registre, pas de decision)
- ✅ Coherence avec LogisticsSteward - Documentation Fondatrice : Confirmee (separation Kernel, arbitrage sans execution)
- ✅ Coherence avec StrongFather Integration Contract : Confirmee (format similaire, protocole coherent)
- ✅ Conformite LOI-1 : Confirmee (aucune dependance externe pour les interrogations)

**Conclusion :** Aucune contradiction detectee. Le document est coherent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*
