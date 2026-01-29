# Border Guard - Trust Level Classification Contract

## 1. Contexte

Ce document définit les **niveaux de confiance** gouvernés par Border Guard dans l'écosystème Miyukini. Il spécifie formellement les quatre niveaux canoniques de confiance, leurs critères d'attribution, les règles de transition, et les obligations associées à chaque niveau.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : Niveau de confiance).

---

## 2. Portée / Scope

- **Applicable à :** Toute source, destination, ou interaction dans l'écosystème Miyukini
- **Responsable :** Border Guard (responsabilité exclusive de classification - INV-BG-4)
- **Consommateurs :** StrongFather (contexte de décision), BondingBrother (application), tous les cores
- **Ne couvre pas :** L'authentification technique (responsabilité des produits/modules auth)

---

## 3. Définition canonique du niveau de confiance

### 3.1 Qu'est-ce qu'un niveau de confiance ?

Un **niveau de confiance** est une classification qui indique le degré de fiabilité accordé à une source, une destination, ou une interaction. C'est une évaluation conceptuelle, pas une validation technique.

**Caractéristiques fondamentales :**

1. **Déclaratif** — Le niveau exprime un état de confiance, pas une action de validation
2. **Universel** — Tout élément interagissant avec le système possède un niveau de confiance
3. **Dynamique** — Le niveau peut évoluer selon les règles de transition
4. **Indépendant de la technologie** — Le niveau est conceptuel, l'implémentation est libre

**Ce qu'un niveau de confiance n'est PAS :**

- ❌ Un jeton d'authentification
- ❌ Une permission d'accès
- ❌ Un rôle utilisateur
- ❌ Une validation cryptographique

### 3.2 Responsabilité de Border Guard

Border Guard est **exclusivement responsable** de la classification des niveaux de confiance. Cette responsabilité inclut :

- Définir les critères de chaque niveau de confiance
- Classifier les sources et destinations selon ces niveaux
- Établir les règles de transition entre niveaux
- Maintenir la cohérence de la classification à travers le système

**Invariant associé :** INV-BG-4 — Toute source, destination, ou interaction **doit** être classifiée selon un niveau de confiance. Par défaut, tout ce qui n'est pas explicitement classifié est considéré comme "unknown".

---

## 4. Les quatre niveaux de confiance canoniques

Border Guard définit exactement quatre niveaux de confiance. Aucun autre niveau n'est autorisé.

### 4.1 Trusted (Confiance totale)

**Définition :** La source ou destination fait partie du cercle de confiance absolu. Aucune vérification supplémentaire n'est requise.

| Aspect | Spécification |
|--------|---------------|
| **Code** | `TRUSTED` |
| **Icône** | 🟢 |
| **Signification** | Confiance absolue, cercle de confiance interne |
| **Vérification** | Aucune vérification supplémentaire requise |
| **Restrictions** | Aucune restriction par défaut |
| **Révocabilité** | Révocable (mais rare) |

**Critères d'attribution :**

1. **Composant interne validé** — Cores du système, modules internes certifiés
2. **Autorité du système** — StrongFather, KindMother, autres cores
3. **Origine vérifiée et certifiée** — Passage par toutes les validations avec succès historique
4. **Aucun incident de confiance** — Jamais de violation ou compromission

**Exemples de sources "Trusted" :**

- StrongFather (core de décision)
- KindMother (core de persistance)
- Border Guard lui-même
- Bonding Brother
- Caring Nanny
- Modules internes certifiés du noyau

**Obligations :**

- Surveillance continue mais non intrusive
- Traçabilité des actions
- Réévaluation périodique (pas en temps réel)

### 4.2 Verified (Confiance vérifiée)

**Définition :** La source ou destination a été authentifiée et validée selon des critères stricts. Des vérifications ont été effectuées.

| Aspect | Spécification |
|--------|---------------|
| **Code** | `VERIFIED` |
| **Icône** | 🔵 |
| **Signification** | Confiance accordée après vérification |
| **Vérification** | Vérifications effectuées, résultat positif |
| **Restrictions** | Selon le contexte et les règles de franchissement |
| **Révocabilité** | Révocable à tout moment |

**Critères d'attribution :**

1. **Authentification réussie** — Identité vérifiée par un mécanisme d'auth
2. **Contexte validé** — Device, session, localisation cohérents
3. **Historique acceptable** — Pas d'incident majeur récent
4. **Intégration certifiée** — Pour les systèmes externes : contrat d'intégration respecté

**Exemples de sources "Verified" :**

- Utilisateur authentifié avec session valide
- API partenaire avec authentification valide
- Intégration Supabase avec credentials valides
- Module externe certifié
- Service tiers avec contrat actif

**Obligations :**

- Vérifications régulières (selon niveau de sécurité)
- Révocation possible à tout moment
- Traçabilité complète des actions
- Réévaluation en cas de changement de contexte

### 4.3 Unknown (Confiance inconnue)

**Définition :** La source ou destination n'a pas encore été classifiée ou son niveau de confiance ne peut être déterminé. Niveau par défaut pour tout ce qui arrive de l'extérieur.

| Aspect | Spécification |
|--------|---------------|
| **Code** | `UNKNOWN` |
| **Icône** | 🟡 |
| **Signification** | Confiance non établie, prudence requise |
| **Vérification** | Vérifications systématiques requises |
| **Restrictions** | Règles restrictives par défaut |
| **Évolution** | Peut évoluer vers Verified ou Hostile |

**Critères d'attribution :**

1. **Aucune classification explicite** — Niveau par défaut (INV-BG-4)
2. **Origine externe non authentifiée** — Requête sans identité vérifiée
3. **Première interaction** — Nouveau partenaire, nouveau device
4. **Classification expirée** — Niveau précédent expiré ou révoqué

**Exemples de sources "Unknown" :**

- Requête HTTP sans authentification
- Nouveau device d'un utilisateur
- Visiteur anonyme
- Intégration non encore classifiée
- Webhook sans signature vérifiée

**Obligations :**

- Traitement avec prudence
- Accès limité aux ressources publiques
- Vérifications systématiques avant toute élévation
- Surveillance renforcée des interactions

**Règle fondamentale :** "Unknown" n'est pas "hostile". C'est un état d'attente qui peut évoluer.

### 4.4 Hostile (Confiance nulle)

**Définition :** La source ou destination a été identifiée comme malveillante, compromise, ou violant les règles. Aucune interaction n'est autorisée.

| Aspect | Spécification |
|--------|---------------|
| **Code** | `HOSTILE` |
| **Icône** | 🔴 |
| **Signification** | Confiance nulle, menace identifiée |
| **Vérification** | Aucune vérification — blocage direct |
| **Restrictions** | Aucune interaction autorisée |
| **Révocabilité** | Révocable uniquement par processus formel |

**Critères d'attribution :**

1. **Source blacklistée** — Présente dans une liste de sources malveillantes
2. **Pattern d'attaque détecté** — Comportement identifié comme malveillant
3. **Compromission confirmée** — Compte ou intégration compromis
4. **Violation grave** — Violation des règles du système confirmée

**Exemples de sources "Hostile" :**

- IP blacklistée pour attaque DDoS
- Compte utilisateur compromis (avant réhabilitation)
- Intégration révoquée pour violation de contrat
- Token volé ou invalide
- Requête avec signature falsifiée

**Obligations :**

- Blocage systématique de toute interaction
- Journalisation de toutes les tentatives
- Alerte aux administrateurs (via TAMR)
- Processus formel pour réhabilitation

---

## 5. Règles de classification

### 5.1 Classification par défaut

| Contexte | Niveau par défaut |
|----------|-------------------|
| Requête externe sans authentification | `UNKNOWN` |
| Requête externe avec authentification valide | `VERIFIED` (après vérification) |
| Composant interne du système | `TRUSTED` (si certifié) |
| Source blacklistée | `HOSTILE` |
| Classification expirée | `UNKNOWN` |

**Règle absolue :** En l'absence de classification explicite, le niveau est **toujours** `UNKNOWN`.

### 5.2 Critères d'évaluation

Pour classifier une source, Border Guard évalue (dans l'ordre) :

```
1. Est-ce une source blacklistée ?
   → OUI : HOSTILE
   → NON : continuer

2. Est-ce un composant interne certifié ?
   → OUI : TRUSTED
   → NON : continuer

3. L'authentification est-elle valide ?
   → NON : UNKNOWN
   → OUI : continuer

4. Le contexte est-il cohérent ?
   → NON : UNKNOWN
   → OUI : VERIFIED
```

### 5.3 Durée de validité

| Niveau | Durée de validité | Réévaluation |
|--------|-------------------|--------------|
| `TRUSTED` | Permanente (sauf révocation) | Périodique (mensuelle) |
| `VERIFIED` | Session ou TTL défini | À chaque changement de contexte |
| `UNKNOWN` | N/A (état par défaut) | À chaque tentative d'élévation |
| `HOSTILE` | Jusqu'à réhabilitation formelle | Sur demande explicite |

---

## 6. Transitions entre niveaux

### 6.1 Transitions autorisées

```
           ┌─────────────────────────────────────────┐
           │                                         │
           ▼                                         │
      ┌─────────┐     authentification      ┌─────────┐
      │ UNKNOWN │ ──────────────────────►   │VERIFIED │
      └─────────┘                           └─────────┘
           │                                     │
           │ pattern d'attaque                   │ certification
           │ ou violation                        │ complète
           ▼                                     ▼
      ┌─────────┐     compromission         ┌─────────┐
      │ HOSTILE │ ◄──────────────────────── │ TRUSTED │
      └─────────┘                           └─────────┘
           │                                     │
           │ réhabilitation                      │
           │ formelle                            │
           └──────────► UNKNOWN ◄────────────────┘
                       (via révocation)
```

### 6.2 Matrice de transition

| De \ Vers | UNKNOWN | VERIFIED | TRUSTED | HOSTILE |
|-----------|---------|----------|---------|---------|
| **UNKNOWN** | - | ✅ Auth réussie | ❌ Jamais direct | ✅ Pattern hostile |
| **VERIFIED** | ✅ Expiration/révocation | - | ✅ Certification | ✅ Compromission |
| **TRUSTED** | ✅ Révocation | ❌ Jamais | - | ✅ Violation grave |
| **HOSTILE** | ✅ Réhabilitation | ❌ Jamais direct | ❌ Jamais | - |

### 6.3 Règles de transition

| Règle | Description |
|-------|-------------|
| **TRANS-1** | Une transition vers TRUSTED est **toujours progressive** (UNKNOWN → VERIFIED → TRUSTED) |
| **TRANS-2** | Une transition vers HOSTILE peut être **immédiate** depuis n'importe quel niveau |
| **TRANS-3** | La réhabilitation depuis HOSTILE passe **obligatoirement** par UNKNOWN |
| **TRANS-4** | L'expiration d'un niveau VERIFIED ramène à UNKNOWN (pas à HOSTILE) |
| **TRANS-5** | Toute transition est **traçable** (INV-BG-8) |

### 6.4 Conditions de transition

#### UNKNOWN → VERIFIED

| Condition | Obligatoire |
|-----------|-------------|
| Authentification réussie | ✅ Oui |
| Contexte validé | ✅ Oui |
| Pas de pattern hostile | ✅ Oui |
| Accord de StrongFather | ✅ Oui |

#### VERIFIED → TRUSTED

| Condition | Obligatoire |
|-----------|-------------|
| Certification complète | ✅ Oui |
| Historique sans incident | ✅ Oui |
| Revue par autorité | ✅ Oui |
| Composant interne | ✅ Oui |

#### * → HOSTILE

| Condition | Obligatoire |
|-----------|-------------|
| Pattern d'attaque OU | ✅ Au moins un |
| Compromission confirmée OU | |
| Violation grave OU | |
| Blacklist explicite | |

#### HOSTILE → UNKNOWN

| Condition | Obligatoire |
|-----------|-------------|
| Processus formel de réhabilitation | ✅ Oui |
| Analyse de l'incident | ✅ Oui |
| Mesures correctives | ✅ Oui |
| Approbation TAMR | ✅ Oui |

---

## 7. Adaptation selon les niveaux de sécurité

La classification de confiance s'adapte selon le niveau de sécurité déclaré.

**Référence :** [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

### 7.1 Impact sur les critères

| Niveau de sécurité | Impact sur VERIFIED | Impact sur TRUSTED |
|--------------------|---------------------|-------------------|
| **0 - PUBLIC** | Critères assouplis | Largement distribué |
| **1 - STANDARD** | Critères standard | Distribution normale |
| **2 - SENSITIVE** | Critères renforcés | Distribution restreinte |
| **3 - CRITICAL** | Critères stricts | Distribution minimale |
| **4 - HARDENED** | Critères ultra-stricts | Quasi aucun (isolement) |

### 7.2 Impact sur les durées

| Niveau de sécurité | TTL VERIFIED | Réévaluation TRUSTED |
|--------------------|--------------|---------------------|
| **0 - PUBLIC** | Long (heures) | Rare |
| **1 - STANDARD** | Standard (minutes) | Mensuelle |
| **2 - SENSITIVE** | Court (minutes) | Hebdomadaire |
| **3 - CRITICAL** | Très court | Quotidienne |
| **4 - HARDENED** | Minimal | Constante |

### 7.3 Impact sur la détection hostile

| Niveau de sécurité | Seuil de détection | Réaction |
|--------------------|-------------------|----------|
| **0 - PUBLIC** | Haut (tolérant) | Dégradation douce |
| **1 - STANDARD** | Standard | Dégradation normale |
| **2 - SENSITIVE** | Bas (sensible) | Dégradation rapide |
| **3 - CRITICAL** | Très bas | Blocage rapide |
| **4 - HARDENED** | Minimal (zéro tolérance) | Blocage immédiat |

---

## 8. Relation avec l'authentification

### 8.1 Distinction fondamentale

| Concept | Responsable | Nature |
|---------|-------------|--------|
| **Niveau de confiance** | Border Guard | Classification conceptuelle |
| **Authentification** | Produit / Module Auth | Validation technique |

**Règle absolue :** Border Guard ne gère **jamais** l'authentification technique. Il utilise le résultat de l'authentification pour classifier.

### 8.2 Flux d'information

```
Produit/Module Auth                Border Guard
      │                                 │
      │  identité vérifiée              │
      │ ─────────────────────────────►  │
      │                                 │
      │  résultat authentification      │ classification
      │ (succès/échec + contexte)       │ ────────────►
      │                                 │
      │                                 │ niveau de confiance
      │                          ◄──────│ (VERIFIED, UNKNOWN, etc.)
```

### 8.3 Ce que Border Guard reçoit

| Information | Usage |
|-------------|-------|
| Identité vérifiée | Pour classification |
| Méthode d'authentification | Pour évaluation de la force |
| Contexte (device, session) | Pour cohérence |
| Historique d'authentification | Pour confiance historique |

### 8.4 Ce que Border Guard ne reçoit PAS

| Information | Pourquoi |
|-------------|----------|
| Mot de passe | Secret, responsabilité auth |
| Token brut | Secret, responsabilité auth |
| Clés cryptographiques | Secret, responsabilité auth |
| Détails de session | Non nécessaire pour classification |

---

## 9. Interaction avec les protocoles de sécurité

**Référence :** [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)

### 9.1 Classification des sources (Border Guard)

Border Guard participe aux protocoles de sécurité suivants :

| Protocole | Rôle de Border Guard |
|-----------|---------------------|
| **RT-SEC-1** (Session éphémère) | Classification de la source de session |
| **RT-SEC-2** (Auth en couches) | Fourniture du niveau de confiance |
| **RT-SEC-4** (Détection anomalie) | Classification résultante (HOSTILE si anomalie) |
| **AS-SEC-2** (Signature locale faible) | Classification du risque |
| **NET-SEC-1** (Handshake conformité) | Isolation si non conforme |

### 9.2 Flux de classification

```
Requête entrante
      │
      ▼
Border Guard : classification source
      │
      │ niveau de confiance
      ▼
Master Butler : capacités selon niveau
      │
      │ permissions
      ▼
Caring Nanny : état système
      │
      │ état global
      ▼
StrongFather : décision finale
```

---

## 10. Traçabilité des classifications

### 10.1 Éléments à tracer

| Élément | Obligatoire | Description |
|---------|-------------|-------------|
| Source classifiée | ✅ Oui | Identifiant de la source |
| Niveau attribué | ✅ Oui | TRUSTED, VERIFIED, UNKNOWN, HOSTILE |
| Date/heure | ✅ Oui | Horodatage de classification |
| Critères utilisés | ✅ Oui | Quels critères ont déterminé le niveau |
| Contexte | ✅ Oui | Informations contextuelles |
| Transition | Si applicable | Niveau précédent et raison |

### 10.2 Format de trace

```
Classification Trace:
- source_id: <identifiant>
- level: <TRUSTED|VERIFIED|UNKNOWN|HOSTILE>
- timestamp: <ISO 8601>
- criteria: [liste des critères appliqués]
- context: {device, session, location, etc.}
- previous_level: <si transition>
- transition_reason: <si transition>
```

**Invariant associé :** INV-BG-8 — Toute classification est **traçable** avec son origine, sa date, et sa justification.

---

## 11. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-BG-4 | Classification exhaustive | Fondement de ce contrat |
| INV-BG-8 | Traçabilité complète | Toute classification est traçable |
| INV-BG-9 | Cohérence globale | Pas de classification contradictoire |
| INV-BG-10 | Neutralité conceptuelle | Classification indépendante de la technologie |

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Border Guard - Boundary Definition Contract](./Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Zones de confiance |
| [Border Guard - Crossing Rules Contract](./Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Règles selon niveau |
| [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Adaptation selon niveau sécurité |
| [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) | Protocoles utilisant la classification |

### Références glossaire

| Terme | Définition |
|-------|------------|
| **Niveau de confiance** | Classification du degré de fiabilité accordé à une source |
| **Trusted** | Confiance totale — cercle de confiance absolu |
| **Verified** | Confiance vérifiée — authentifié et validé |
| **Unknown** | Confiance inconnue — niveau par défaut |
| **Hostile** | Confiance nulle — source malveillante identifiée |
| **Classification** | Attribution d'un niveau de confiance |

**Source :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 12. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Quatre niveaux et seulement quatre** — TRUSTED, VERIFIED, UNKNOWN, HOSTILE
2. **Classification exhaustive** — Tout élément a un niveau (UNKNOWN par défaut)
3. **Critères explicites** — Chaque niveau a des critères d'attribution documentés
4. **Transitions contrôlées** — Les changements de niveau suivent des règles strictes
5. **Traçabilité complète** — Toute classification est traçable
6. **Indépendance technique** — La classification est conceptuelle, pas technique

### Phrase de synthèse

> **Un niveau de confiance est une classification conceptuelle (TRUSTED, VERIFIED, UNKNOWN, HOSTILE) attribuée exclusivement par Border Guard à toute source, destination, ou interaction, selon des critères explicites et traçables, indépendamment de l'implémentation technique.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 4  
**Type :** Contrat de classification de confiance
