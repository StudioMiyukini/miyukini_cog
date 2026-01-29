# Miyukini Conceptual References — Doctrine Securite Fondamentale

## 1. Contexte

Ce document définit la **Doctrine Fondamentale de Sécurité** de l'écosystème Miyukini : les principes, postulats et architectures qui font de la sécurité une propriété structurelle du système, intégrée dès la conception et obligatoire à l'implémentation.

**Principe directeur fondamental :**

**"La sécurité dans Miyukini n'est pas un module, ni une fonctionnalité, ni un service. Elle est une propriété structurelle du système."**

Ce document est le fondement philosophique et architectural de toute la sécurité Miyukini.

## 2. Portée / Scope

Ce document définit :
- Le principe fondateur et les postulats fondamentaux
- L'architecture des strates de sécurité
- Les porteurs de vérité (STA, OSV)
- La chaîne de confiance système
- Les niveaux d'intégrité conceptuels
- Les solutions actives de sécurité (Engines)
- Les supports de sécurité (structures)
- Les lois et contraintes systémiques
- Le modèle fédéral d'environnement
- Les principes de gouvernance humaine

Ce document **ne couvre pas** :
- Les niveaux de sécurité opérationnels (0-4) → voir [Security Levels](Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- Les protocoles de sécurité temps réel/asynchrone → voir [Security Protocols](Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)
- Le système de dégradation graduée (T0-T4) → voir [Integrity Degradation System](Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)
- Les implémentations cryptographiques spécifiques

---

## 3. Principe Fondateur

### 3.1 La Sécurité comme Propriété Structurelle

> La sécurité dans Miyukini n'est pas un module, ni une fonctionnalité, ni un service.
> Elle est une **propriété structurelle du système**.

Elle existe comme :
- **Loi d'architecture** : règle non négociable de conception
- **Contrainte de fonctionnement** : limite imposée à tout comportement
- **Règle de conception** : principe directeur de développement
- **Invariant système** : propriété toujours vraie
- **Principe de gouvernance** : cadre de décision

### 3.2 Ce que la Sécurité Miyukini Protège

> La sécurité Miyukini ne protège pas seulement le système.
> Elle protège la **vérité**, la **structure**, la **mémoire** et la **cognition**.

| Domaine | Protection |
|---------|------------|
| **Vérité** | État certifié, référence officielle |
| **Structure** | Architecture, graphes, relations |
| **Mémoire** | Historique, traçabilité, versioning |
| **Cognition** | Décisions IA, agents, anti-dérive |

---

## 4. Postulats Fondamentaux

### 4.1 Les 5 Postulats

Ces postulats sont les axiomes sur lesquels repose toute la doctrine de sécurité :

| # | Postulat |
|---|----------|
| **P1** | Un système ne tombe pas par ses fonctionnalités mais par ses **interfaces** et ses **frontières** |
| **P2** | La sécurité technique est insuffisante sans **sécurité structurelle** |
| **P3** | La sécurité du code est insuffisante sans **sécurité cognitive** |
| **P4** | La protection périmétrique est insuffisante sans **protection de la vérité** |
| **P5** | La sécurité est une **propriété émergente** du système |

### 4.2 Implications des Postulats

**P1 — Interfaces et Frontières**
- Les surfaces d'attaque sont aux jonctions, pas au cœur
- Border Guard protège les transitions inter-couches
- Chaque interface est un point de contrôle

**P2 — Sécurité Structurelle**
- Le code sécurisé ne suffit pas si l'architecture est faible
- MSCM garantit la sémantique structurelle
- MIP maintient la cohérence globale

**P3 — Sécurité Cognitive**
- Les agents IA sont des vecteurs potentiels de dérive
- Cognitive Guard surveille les décisions IA
- Multi-agents contradictoires évitent le consensus erroné

**P4 — Protection de la Vérité**
- Sans vérité de référence, toute validation est vaine
- STA est le porteur de vérité officiel
- OSV est la version certifiée restaurable

**P5 — Propriété Émergente**
- La sécurité n'est pas ajoutée, elle émerge de l'architecture
- Elle résulte de l'interaction des composants
- Elle est intrinsèque, pas extrinsèque

---

## 5. Architecture des Strates de Sécurité

### 5.1 Modèle Conceptuel des Strates

```
┌────────────────────────────────────┐
│            SERVICES                │  ← plus haut niveau
│  (apps, outils, plateformes, IA,   │
│   interfaces, produits finaux)     │
└────────────────────────────────────┘
              ↓
┌────────────────────────────────────┐
│             CORES                  │
│ (StrongFather, KindMother, COG,    │
│  cores métiers, orchestration)     │
└────────────────────────────────────┘
              ↓
┌────────────────────────────────────┐
│   INFRASTRUCTURE SYSTÉMIQUE        │
│ (Security Engines / Integrity /    │
│  Validation / Consensus / Audit)   │
└────────────────────────────────────┘
              ↓
┌────────────────────────────────────┐
│              KERNEL                │
│ (abstraction OS, hardware, runtime │
│  system services, bas niveau)      │
└────────────────────────────────────┘
              ↓
┌────────────────────────────────────┐
│             SUBSTRAT               │  ← physique / logique brut
│ (OS, drivers, hardware, runtime)   │
└────────────────────────────────────┘
```

### 5.2 Position des Security Engines

Les moteurs de sécurité constituent une **strate d'infrastructure systémique** :
- **Au-dessus du Kernel** (plus bas niveau système)
- **En dessous des Cores** (logique fonctionnelle)
- **Couche obligatoire de médiation**

**Règle de circulation :**
```
Services → Cores → Security Engines → Kernel → Substrat
```

Et inversement. **Aucun saut de strate autorisé, aucun bypass, pas de raccourci.**

### 5.3 Traversée des Strates

La sécurité **traverse toutes les strates** :

```
Humain (gouvernance)
   ↑
Agents / COG (cognition)
   ↑
MIP (mémoire structurelle)
   ↑
MSCM (sémantique du code)
   ↑
Code (substrat logique)
```

Chaque strate a ses propres mécanismes de protection, et tous sont interconnectés.

---

## 6. Vérité et Confiance

### 6.1 System Truth Anchor (STA)

**Définition :** Le STA est le **porteur de vérité officiel** du système.

**Contenu du STA :**
- Empreintes MSCM (sémantique du code)
- Empreintes MIP (mémoire structurelle)
- Signatures structurelles
- Graph checksums
- Versions certifiées
- Références d'intégrité

**Rôle :**
| Fonction | Description |
|----------|-------------|
| **Vérité officielle** | Référence de ce qui est "vrai" |
| **État certifié** | Point de comparaison fiable |
| **Base de comparaison** | Détection des déviations |
| **Source de confiance** | Ancrage de toute validation |

> Le STA est l'autorité de référence.

### 6.2 Official Secure Version (OSV)

**Définition :** Version officielle sûre du système.

**Caractéristiques :**
- ✅ **Validée** : a passé tous les contrôles
- ✅ **Auditée** : vérifiée par processus formel
- ✅ **Signée** : authentifiée cryptographiquement
- ✅ **Figée** : immuable une fois certifiée
- ✅ **Archivée** : stockée de manière durable
- ✅ **Restaurable** : peut être réactivée en cas de besoin

> Toute version non OSV est considérée comme non certifiée.

### 6.3 Relation STA / OSV

| Concept | Rôle | Nature |
|---------|------|--------|
| **STA** | Référence de vérité | État actuel certifié |
| **OSV** | Version de référence | Snapshot certifié restaurable |

Le STA contient l'état de vérité courant, l'OSV est une version figée de cet état.

---

## 7. Chaîne de Confiance

### 7.1 Flux de Confiance

```
CODE
 ↓
MSCM (sémantique locale)
 ↓
MIP (mémoire structurelle)
 ↓
GRAPH (modèle global)
 ↓
STA (Truth Anchor)
 ↓
OSV (Version officielle)
```

**Règle :** Toute rupture dans cette chaîne déclenche **alerte, blocage ou rollback**.

### 7.2 Validation de la Chaîne

Chaque maillon de la chaîne valide le précédent :
- Le code doit être conforme au MSCM
- Le MSCM doit être cohérent avec le MIP
- Le MIP doit correspondre au Graph
- Le Graph doit être ancré dans le STA
- Le STA doit correspondre à une OSV

**Si un maillon est rompu :**
1. Détection par les sondes d'intégrité
2. Consolidation par Caring Nanny
3. Évaluation par StrongFather
4. Dégradation progressive (T0 → T1 → T2 → T3 → T4)

---

## 8. Intégrité Multi-Niveaux

### 8.1 Les 5 Niveaux d'Intégrité Conceptuels

#### Niveau 1 — Intégrité Passive (Barrière Simple)

**Vérifications :**
- Hash fichiers
- Tailles
- Noms et extensions
- Structure dossiers
- Comptage blocs et IDs

**Rôle :** Filtrage des attaques faibles + erreurs humaines

#### Niveau 2 — Intégrité Structurelle

**Vérifications :**
- Unicité des IDs
- Hiérarchie cohérente
- Graph valide
- Layers cohérents
- Domains cohérents
- Dépendances valides

**Rôle :** Protection de la structure du système

#### Niveau 3 — Intégrité Sémantique

**Vérifications :**
- Cohérence DO / ROLE / LAYER
- Non-contradictions métier
- Unicité des responsabilités
- Absence de doublons sémantiques

**Rôle :** Protection du sens du système

#### Niveau 4 — Intégrité Cognitive

**Vérifications :**
- Cohérence des décisions IA
- Absence de dérives
- Contrôle des feedback loops
- Multi-agents contradictoires

**Rôle :** Protection de l'intelligence du système

#### Niveau 5 — Intégrité Historique

**Vérifications :**
- Versioning complet
- Traçabilité continue
- Continuité temporelle
- Archivage valide
- Snapshots cohérents

**Rôle :** Protection de la mémoire du système

### 8.2 Synthèse des Niveaux

| Niveau | Nom | Protège | Détecte |
|--------|-----|---------|---------|
| 1 | Passive | Fichiers | Corruption simple |
| 2 | Structurelle | Architecture | Incohérence structure |
| 3 | Sémantique | Sens | Contradiction métier |
| 4 | Cognitive | Intelligence | Dérive IA |
| 5 | Historique | Mémoire | Rupture continuité |

---

## 9. Solutions Actives de Sécurité (Engines)

### 9.1 Vue d'Ensemble

Les solutions actives sont des **mécanismes vivants, dynamiques, opérants** qui garantissent la protection en temps réel.

| Engine | Rôle |
|--------|------|
| Integrity Engine | Vérification permanente de l'intégrité |
| Validation Engine | Filtrage systémique |
| Policy Engine | Règles de fonctionnement |
| Consensus Engine | Éviter la décision unique |
| Audit Engine | Traçabilité active |
| Sandbox Engine | Isolement |
| Cognitive Guard | Sécurité IA |
| Recovery Engine | Résilience |

### 9.2 Integrity Engine

**Rôle :** Vérification permanente de l'intégrité

**Actions :**
- Hash checks
- Structure checks
- Graph validation
- MSCM validation
- MIP validation
- Diff structurel
- Checksum global
- Alerte / Blocage / Rollback

**👉 Agit en continu**

### 9.3 Validation Engine

**Rôle :** Filtrage systémique

**Actions :**
- Validation entrées
- Validation flux
- Validation formats
- Validation structures
- Validation transitions
- Validation décisions
- Validation index

**👉 Empêche l'entrée de corruption**

### 9.4 Policy Engine

**Rôle :** Règles de fonctionnement

**Actions :**
- Contrôle d'accès
- Scopes
- Permissions
- Règles système
- Contraintes d'exécution
- Autorisations dynamiques

**👉 Impose la loi du système**

### 9.5 Consensus Engine

**Rôle :** Éviter la décision unique

**Actions :**
- Multi-agents
- Validation croisée
- Vote structurel
- Contradictions
- Arbitrage
- Escalade humaine

**👉 Empêche sabotage et dérive**

### 9.6 Audit Engine

**Rôle :** Traçabilité active

**Actions :**
- Logs
- Historiques
- Traçabilité
- Journaux d'action
- Journaux de décision
- Journaux IA
- Journaux structurels

**👉 Mémoire de sécurité**

### 9.7 Sandbox Engine

**Rôle :** Isolement

**Actions :**
- Exécution isolée
- Test sécurisé
- Simulation
- Bac à sable agents
- Sandbox outils
- Sandbox décisions

**👉 Empêche propagation**

### 9.8 Cognitive Guard

**Rôle :** Sécurité IA

**Actions :**
- Détection dérive
- Détection biais
- Anti-feedback-loop
- Contradiction agents
- Surveillance cognition
- Seuils de confiance

**👉 Sécurité cognitive**

### 9.9 Recovery Engine

**Rôle :** Résilience

**Actions :**
- Rollback
- Restauration
- Snapshot
- Recovery
- Freeze
- Safe-mode
- Reboot logique

**👉 Sécurité par résilience**

---

## 10. Supports de Sécurité

### 10.1 Vue d'Ensemble

Les supports sont ce qui **rend la sécurité possible, stable, durable**. Ils forment le squelette sur lequel s'appuient les solutions actives.

| Support | Rôle |
|---------|------|
| STA | Support de vérité |
| OSV | Support de stabilité |
| MSCM | Support sémantique |
| MIP | Support structurel |
| Index global | Support cognitif |
| Abstraction Layers | Support d'isolation |
| Versioning System | Support temporel |
| Storage sécurisé | Support physique/logique |
| Gouvernance humaine | Support ultime |
| Architecture | Support fondamental |

### 10.2 STA — System Truth Anchor

- Référence officielle
- État certifié
- Point de comparaison
- Source de confiance

### 10.3 OSV — Official Secure Version

- Version figée
- Version validée
- Version signée
- Version restaurable

### 10.4 MSCM

- Structure du code
- Responsabilité explicite
- Lisibilité
- Traçabilité
- Indexabilité

### 10.5 MIP

- Mémoire globale
- Graph système
- Macro-structure
- Navigation IA
- Gouvernance

### 10.6 Index Global

- Vision système
- Relations
- Dépendances
- Flux
- Cartographie

### 10.7 Abstraction Layers

- OS abstraction
- Hardware abstraction
- Data abstraction
- API abstraction

### 10.8 Versioning System

- Historique
- Traçabilité
- Rollback
- Comparaison
- Audit

### 10.9 Storage Sécurisé

- Stockage immuable
- Snapshots
- Backups
- WORM storage
- Archivage

### 10.10 Gouvernance Humaine

- Supervision
- Arbitrage
- Validation
- Décision finale
- Responsabilité

### 10.11 Architecture

- Séparation des couches
- Séparation des rôles
- Séparation des responsabilités
- Séparation des flux
- Séparation des pouvoirs

### 10.12 Synthèse

**Les supports sont le squelette.**
**Les solutions actives sont le système immunitaire.**

---

## 11. Sécurité Systémique

### 11.1 Lois du Système

Ces règles sont **absolues et non négociables** :

| Loi | Description |
|-----|-------------|
| **L1** | Aucun accès direct hardware |
| **L2** | Aucune source de vérité multiple |
| **L3** | Aucun bypass des cores |
| **L4** | Aucune écriture sans traçabilité |
| **L5** | Aucune décision sans validation |
| **L6** | Aucune structure sans indexation |

### 11.2 Contraintes de Fonctionnement

Tout flux dans le système doit respecter ces contraintes :

- **Tout passe par abstraction** : pas d'accès direct
- **Tout passe par validation** : pas d'action non vérifiée
- **Tout passe par consensus** : pas de décision unilatérale critique
- **Tout passe par versioning** : pas de modification sans trace

### 11.3 Flux Système

```
Code → MSCM → MIP → Graph → Agents → Outils
```

Ce flux est **unidirectionnel et traçable**. Aucun retour en arrière sans validation.

### 11.4 Sécurité et Hardware

Le hardware est traité comme une **source d'état**, jamais comme une dépendance directe.

**Règle :** Toute interaction matérielle passe par une couche d'abstraction système (Kernel).

---

## 12. Modèle Fédéral d'Environnement

### 12.1 Analogie Pays / Fédération

| Concept Système | Analogie Politique |
|-----------------|-------------------|
| Fédération X | Union (version des cores) |
| Instance | Pays (environnement) |
| Agents | Institutions |
| Cores | Constitution |
| MSCM | Lois |
| MIP | Cadastre / Registre national |
| STA | Registre d'état civil |
| OSV | Constitution certifiée |

### 12.2 Identité des Environnements

Chaque instance possède :

```
ENV_ID        = identité unique
CORE_VERSION  = X
STA_ID        = identité de vérité
OSV_HASH      = constitution certifiée
GRAPH_HASH    = structure
STRUCT_HASH   = organisation
```

### 12.3 Maillage de Confiance (Mesh)

```
ENV_A ↔ ENV_B ↔ ENV_C ↔ ENV_D
```

Chaque environnement peut :
- S'identifier
- Se présenter
- Se décrire
- Se certifier
- Se comparer
- Se valider

### 12.4 Protocole de Certification Inter-Instances

Quand deux environnements se rencontrent :

1. Échange identités
2. Vérification CORE_VERSION (X)
3. Vérification STA
4. Vérification OSV
5. Vérification graph
6. Vérification structure
7. Vérification historique
8. Vérification compatibilité dogmatique

### 12.5 Dogme de Version

**Les versions de cores sont des dogmes incompatibles.**

```
CoreSet v5 ≠ CoreSet v6
```

Conséquences :
- ❌ Pas de confiance automatique
- ❌ Pas d'interop directe
- ✅ Nécessité de passerelles de traduction
- ✅ Certification conditionnelle

### 12.6 Certification Dynamique

```
ENV_A certifie ENV_B
ENV_C certifie ENV_B
ENV_D certifie ENV_B
```

Principes :
- **Pluralité** : plusieurs certificateurs
- **Consensus** : accord entre certificateurs
- **Preuve par diversité** : sources multiples indépendantes

### 12.7 Cas Non Certifiable

Si une instance ne peut pas être certifiée :
- Alerte
- Isolement
- Restriction
- Mode dégradé
- Sandbox
- Lecture seule
- Non-propagation
- Non-réplication

---

## 13. Environnements Isolés et Reconnexion

### 13.1 Systèmes Isolés

Dans un système isolé :
- La menace est **interne**
- La corruption est **silencieuse**
- La dérive est **progressive**

La sécurité repose sur :
- STA local
- OSV locale
- Validation manuelle
- Audits réguliers
- Versioning strict
- Stockage immuable

### 13.2 Mode Fermé (Air-Gapped)

L'instance sait qu'elle est isolée :
```
ENV_MODE = CLOSED
```

**Adaptation automatique :**
- Validation interne renforcée
- Pluralité interne
- Audits locaux
- Consensus interne
- Gouvernance humaine
- OSV locale
- STA locale

### 13.3 Reconnexion

Quand la connexion revient :
```
ENV_MODE = RECONNECTING
```

**Étapes :**
1. Auto-diagnostic
2. Intégrité locale
3. Reconstruction MIP
4. Comparaison STA
5. Comparaison OSV
6. Vérification historique
7. Validation fédérale
8. Recertification dynamique
9. Réintégration mesh

### 13.4 Système de Dégradation

```
NORMAL
  ↓
RESTRICTED
  ↓
ISOLATED
  ↓
SAFE MODE
  ↓
QUARANTINE
```

---

## 14. Preuves Utilisées (Sans PoW)

### 14.1 Types de Preuves

Miyukini utilise des preuves **sans Proof of Work** :

| Preuve | Vérifie |
|--------|---------|
| **Proof of Integrity** | État non corrompu |
| **Proof of Consistency** | Cohérence interne |
| **Proof of Diversity** | Pluralité des sources |
| **Proof of History** | Continuité temporelle |
| **Proof of Governance** | Supervision humaine |
| **Proof of Structure** | Architecture valide |
| **Proof of Cognition** | Décisions IA saines |

### 14.2 Utilisation

Ces preuves sont utilisées pour :
- Certification inter-instances
- Validation de reconnexion
- Audit de conformité
- Détection d'intrusion
- Rollback sécurisé

---

## 15. Gouvernance Humaine

### 15.1 Principe

**La sécurité est gouvernée par l'humain.**

L'humain est le dernier recours, l'arbitre final, la source ultime de légitimité.

### 15.2 Règles de Gouvernance

| Règle | Description |
|-------|-------------|
| **G1** | Supervision humaine obligatoire |
| **G2** | Validation humaine des versions OSV |
| **G3** | Arbitrage humain des conflits |
| **G4** | Contrôle des décisions critiques |

### 15.3 Surfaces d'Attaque Reconnues

La doctrine reconnaît explicitement les surfaces d'attaque :
- Interfaces
- Abstractions
- Transitions inter-couches
- MSCM (sémantique)
- MIP (mémoire)
- COG (cognition)
- Agents
- **Gouvernance humaine**

L'humain lui-même est une surface d'attaque potentielle (social engineering, erreur, malveillance interne).

---

## 16. Formulations Synthèse

### 16.1 Formulation Technique

> La sécurité Miyukini repose sur un ensemble de **supports structurels** (vérité, mémoire, sémantique, architecture, gouvernance) et de **solutions actives** (intégrité, validation, consensus, audit, cognition, résilience).
> Les supports garantissent la stabilité, les solutions actives garantissent la protection dynamique.

### 16.2 Formulation Architecturale

> Dans l'architecture stratifiée Miyukini, les moteurs de sécurité constituent une strate d'infrastructure systémique située entre le Kernel et les Cores.
> Ils forment une couche obligatoire de médiation, garantissant que tout flux, toute donnée, toute action, toute décision transitant entre le bas niveau système (Kernel) et la logique fonctionnelle (Cores) est validée, contrôlée et sécurisée.

### 16.3 Formulation Fédérale

> Chaque environnement Miyukini est un État souverain, doté de sa propre identité, de sa propre vérité, de sa propre constitution, de sa propre mémoire et de sa propre gouvernance.
> Les environnements forment une fédération, où la confiance n'est jamais automatique, mais toujours certifiée, vérifiée, pluraliste et dynamique.

### 16.4 Formule Finale

> **La sécurité n'est pas un composant du système Miyukini.**
> **Elle est sa condition d'existence.**

> **Miyukini n'est pas un système sécurisé.**
> **C'est un écosystème de confiance souveraine fédérée.**

---

## 17. Intégration avec les Cores

### 17.1 Rôles des Cores dans la Sécurité

| Core | Rôle Sécurité |
|------|---------------|
| **StrongFather** | Décisions finales, validation systématique |
| **Border Guard** | Classification sources, protection injection |
| **BondingBrother** | Médiation sécurisée, traçabilité |
| **Caring Nanny** | Détection anomalies, état système |
| **Master Butler** | Capacités et permissions |
| **TAMR** | Intervention humaine, traçabilité absolue |
| **Ever Buddy** | Compatibilité, versioning |
| **KindMother** | Persistance, synchronisation |

### 17.2 Intégration avec l'Implémentation

Toute implémentation doit :
- ✅ Respecter MSCM
- ✅ Générer MIP
- ✅ Maintenir STA
- ✅ Versionner OSV
- ✅ Appliquer validation multi-niveaux
- ✅ Intégrer contrôles d'intégrité
- ✅ Respecter les lois système

---

## 18. Conclusion

La Doctrine Fondamentale de Sécurité Miyukini garantit que :

- ✅ **La sécurité est structurelle** : Pas un module, une propriété émergente
- ✅ **La vérité est protégée** : STA et OSV comme ancres de confiance
- ✅ **L'intégrité est multi-niveaux** : Du passif au cognitif
- ✅ **Les solutions sont actives** : 8 engines de protection dynamique
- ✅ **Les supports sont solides** : 10 piliers structurels
- ✅ **L'architecture est stratifiée** : Aucun bypass possible
- ✅ **La fédération est souveraine** : Confiance construite, jamais automatique
- ✅ **L'humain gouverne** : Supervision, arbitrage, validation finale

**Principe fondamental :**

**"La sécurité dans Miyukini n'est pas un module, ni une fonctionnalité, ni un service. Elle est une propriété structurelle du système."**

---

**Date de création :** 2026-01-28  
**Version :** 1.0  
**Statut :** Document fondateur contractuel

**Documentation associée :**
- [Miyukini Conceptual References - Security Levels](Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) : Niveaux de sécurité opérationnels (0-4)
- [Miyukini Conceptual References - Security Protocols](Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) : Protocoles temps réel et asynchrone
- [Miyukini Conceptual References - Integrity Degradation System](Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) : Système de dégradation graduée (T0-T4)
- [Miyukini Conceptual References - Security Performance Impact](Miyukini%20Conceptual%20References%20-%20Security%20Performance%20Impact.md) : Impact sur les performances
- [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Renforcement de confiance externe
- [Miyukini Conceptual References - Souverainete Environnement](Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) : Souveraineté des environnements
- [StrongFather - Documentation Fondatrice](../core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) : Décisions finales
- [Border Guard - Documentation Fondatrice](../core/BorderGuard/foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) : Classification sources
- [Caring Nanny - Documentation Fondatrice](../core/CaringNanny/foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) : Détection anomalies
