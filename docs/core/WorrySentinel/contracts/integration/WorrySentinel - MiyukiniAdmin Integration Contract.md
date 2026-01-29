# WorrySentinel — MiyukiniAdmin Integration Contract

## 1. Contexte

Ce document definit le **contrat d'integration entre WorrySentinel et MiyukiniAdmin**. Il specifie l'interface, le protocole, les regles de communication, et les garanties associees a l'integration avec MiyukiniAdmin en tant que console root d'administration (Operateur Souverain, Strate 9).

Ce document complete la Section 11 de la [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [MiyukiniAdmin - Documentation Fondatrice](../../../MiyukiniAdmin/foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md) pour la nature de MiyukiniAdmin
- [Miyukini Conceptual References - Security Levels](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) pour les niveaux de securite
- [Miyukini Framework - Integrity Degradation System](../../../../reference/Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) pour les etats de confiance

L'integration respecte les Lois d'Autonomie Systeme : MiyukiniAdmin peut fonctionner offline pour monitoring local (**LOI-1**).

## 2. Portee / Scope

Ce document couvre :
- L'interface contractuelle entre WorrySentinel et MiyukiniAdmin
- Le protocole de consultation (lecture des niveaux et etats)
- Le protocole de configuration (modification de gouvernance)
- Les capacites reservees a MiyukiniAdmin
- Les contraintes de tracabilite
- La validation obligatoire par StrongFather
- Les garanties de l'integration

Ce document **ne couvre pas** :
- Les details internes de WorrySentinel (voir documentation WorrySentinel)
- Les details internes de MiyukiniAdmin (voir documentation MiyukiniAdmin)
- L'integration MiyukiniAdmin-StrongFather (voir MiyukiniAdmin - StrongFather Integration Contract)
- L'integration MiyukiniAdmin-BondingBrother (voir MiyukiniAdmin - BondingBrother Integration Contract)

---

## 3. Principe fondamental

**WorrySentinel gouverne les niveaux de securite et les etats de confiance. MiyukiniAdmin est l'interface privilegiee de consultation et de configuration de cette gouvernance. Toute configuration par MiyukiniAdmin est soumise a validation par StrongFather.**

La relation est **d'interface administrative** : MiyukiniAdmin expose les capacites de gouvernance aux administrateurs humains, WorrySentinel maintient l'autorite sur les regles de gouvernance.

---

## 4. Nature de la relation WorrySentinel — MiyukiniAdmin

### 4.1 Relation d'interface administrative

**MiyukiniAdmin accede a WorrySentinel pour :**
- Consulter l'etat de securite actuel (niveaux, etats de confiance)
- Visualiser l'historique des transitions d'etat
- Configurer les niveaux de securite des produits (sous validation StrongFather)
- Declencher des modes de degradation (sous validation StrongFather)
- Visualiser les regles de gouvernance applicables
- Auditer les decisions de gouvernance

**WorrySentinel expose a MiyukiniAdmin :**
- L'etat de confiance global du systeme (T0-T4)
- Les niveaux de securite par produit et composant (0-4)
- L'historique des transitions avec justifications
- Les regles de gouvernance actives
- Les metriques de gouvernance

**Regle WS-MA-01 : Interface, pas autorite**

MiyukiniAdmin est une interface de consultation et de configuration. MiyukiniAdmin n'est pas une autorite de gouvernance. L'autorite reste exclusivement a WorrySentinel pour la gouvernance et a StrongFather pour la validation des changements.

**Regle WS-MA-02 : Validation obligatoire**

Toute modification de gouvernance initiee via MiyukiniAdmin est soumise a validation par StrongFather. MiyukiniAdmin ne peut pas bypasser cette validation.

**Regle WS-MA-03 : Mediation BondingBrother**

Toute interaction entre MiyukiniAdmin et WorrySentinel passe par BondingBrother. MiyukiniAdmin n'accede jamais directement a WorrySentinel.

### 4.2 Separation des responsabilites

| Responsabilite | WorrySentinel | MiyukiniAdmin |
|----------------|---------------|---------------|
| **Gouverner les etats de confiance** | ✅ Exclusif | ❌ Consulte |
| **Definir les niveaux de securite** | ✅ Exclusif | ❌ Configure (via validation) |
| **Orchestrer la degradation** | ✅ Exclusif | ❌ Declenche (via validation) |
| **Exposer l'interface admin** | ❌ Expose API | ✅ Exclusif |
| **Visualiser l'etat de gouvernance** | ❌ Source | ✅ Affiche |
| **Valider les changements** | ❌ Jamais | ❌ Soumet a StrongFather |
| **Tracer les actions** | ✅ Enregistre | ✅ Journalise |

**Regle WS-MA-04 : Aucun chevauchement d'autorite**

MiyukiniAdmin ne gouverne jamais. WorrySentinel n'expose jamais d'interface utilisateur. La separation est stricte.

---

## 5. Ce que MiyukiniAdmin ne fait JAMAIS vis-a-vis de WorrySentinel

### 5.1 Interdictions absolues

**INV-WS-MA-NEVER-1 : Ne gouverne jamais les etats de confiance**

MiyukiniAdmin ne gouverne **jamais** les etats de confiance (T0-T4). La definition et la transition des etats de confiance appartiennent exclusivement a WorrySentinel.

**INV-WS-MA-NEVER-2 : Ne bypass jamais la validation**

MiyukiniAdmin ne bypass **jamais** la validation StrongFather pour les modifications de gouvernance. Meme en situation d'urgence, la validation reste obligatoire.

**INV-WS-MA-NEVER-3 : N'accede jamais directement**

MiyukiniAdmin n'accede **jamais** directement a WorrySentinel. Toute interaction passe par BondingBrother.

**INV-WS-MA-NEVER-4 : Ne modifie jamais implicitement**

MiyukiniAdmin ne modifie **jamais** implicitement la gouvernance. Toute modification est explicite, tracee, et justifiee.

**INV-WS-MA-NEVER-5 : N'ignore jamais les contraintes**

MiyukiniAdmin n'ignore **jamais** les contraintes imposees par WorrySentinel. Si WorrySentinel indique que le systeme est en T4, MiyukiniAdmin affiche cet etat et ses consequences.

**INV-WS-MA-NEVER-6 : Ne cache jamais l'etat reel**

MiyukiniAdmin ne cache **jamais** l'etat reel de gouvernance. L'interface affiche toujours l'etat exact du systeme.

---

## 6. Capacites exposees a MiyukiniAdmin

### 6.1 Capacites de consultation (lecture seule)

| Capacite | Description | Validation SF |
|----------|-------------|---------------|
| `ws.state.trust.read` | Lecture de l'etat de confiance actuel | Non requise |
| `ws.state.trust.history` | Historique des transitions d'etat | Non requise |
| `ws.levels.security.read` | Lecture des niveaux de securite | Non requise |
| `ws.levels.security.history` | Historique des changements de niveaux | Non requise |
| `ws.governance.rules.read` | Lecture des regles de gouvernance | Non requise |
| `ws.metrics.read` | Metriques de gouvernance | Non requise |
| `ws.audit.read` | Audit des decisions de gouvernance | Non requise |

### 6.2 Capacites de configuration (necessitent validation)

| Capacite | Description | Validation SF | Criticite |
|----------|-------------|---------------|-----------|
| `ws.levels.security.write` | Attribution/modification niveau securite | **Requise** | HAUTE |
| `ws.degradation.activate` | Activation mode de degradation | **Requise** | CRITIQUE |
| `ws.degradation.deactivate` | Desactivation mode de degradation | **Requise** | HAUTE |
| `ws.governance.rules.modify` | Modification regles de transition | **Requise** | CRITIQUE |

### 6.3 Capacites speciales (conditions strictes)

| Capacite | Description | Conditions |
|----------|-------------|------------|
| `ws.emergency.escalate` | Escalade manuelle de l'etat de confiance | T2+ actif, role Recovery, validation SF |
| `ws.emergency.override` | Override temporaire (consultation TAMR) | T3-T4, TAMR confirme, tracabilite renforcee |

---

## 7. Interactions autorisees

### 7.1 INTERACTION-ADMIN-1 : Consultation des niveaux de securite

**Objectif :** Permettre aux administrateurs de visualiser les niveaux de securite.

**Donnees accessibles :**
- Niveaux de securite des produits et composants (0-4)
- Regles de gouvernance applicables par niveau
- Historique des changements de niveaux
- Justifications des attributions

**Format de consultation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `consultation_id` | Identifiant unique de la consultation | ✅ Oui |
| `scope` | Portee (all, product, component) | ✅ Oui |
| `entity_id` | Identifiant de l'entite (si scope != all) | ❌ Optionnel |
| `include_history` | Inclure l'historique | ❌ Optionnel |
| `timestamp` | Horodatage de la consultation | ✅ Oui |

**Format de reponse :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `response_id` | Identifiant de la reponse | ✅ Oui |
| `consultation_id` | Reference a la consultation | ✅ Oui |
| `security_levels` | Liste des niveaux par entite | ✅ Oui |
| `governance_rules` | Regles applicables | ✅ Oui |
| `history` | Historique (si demande) | ❌ Optionnel |
| `timestamp` | Horodatage de la reponse | ✅ Oui |

### 7.2 INTERACTION-ADMIN-2 : Consultation des etats de confiance

**Objectif :** Permettre aux administrateurs de visualiser l'etat de confiance du systeme.

**Donnees accessibles :**
- Etat de confiance courant (T0-T4)
- Description et implications de l'etat
- Historique des transitions
- Justifications des transitions

**Format de consultation :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `consultation_id` | Identifiant unique | ✅ Oui |
| `include_implications` | Inclure les implications | ❌ Optionnel |
| `include_history` | Inclure l'historique | ❌ Optionnel |
| `history_limit` | Nombre de transitions max | ❌ Optionnel |
| `timestamp` | Horodatage | ✅ Oui |

**Format de reponse :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `response_id` | Identifiant de la reponse | ✅ Oui |
| `current_state` | Etat de confiance actuel (T0-T4) | ✅ Oui |
| `state_description` | Description de l'etat | ✅ Oui |
| `implications` | Implications (capacites, restrictions) | ❌ Optionnel |
| `transitions_history` | Historique des transitions | ❌ Optionnel |
| `timestamp` | Horodatage de la reponse | ✅ Oui |

### 7.3 INTERACTION-ADMIN-3 : Configuration de la gouvernance

**Objectif :** Permettre aux administrateurs de configurer la gouvernance de securite.

**Actions possibles :**
- Attribution de niveaux de securite aux produits
- Modification des regles de transition
- Activation/desactivation de modes de degradation

**Contraintes strictes :**
- Toute configuration est soumise a validation StrongFather
- Toute configuration est tracee avec identite et justification
- Toute configuration est reversible (sauf T4)

**Format de demande de configuration :**

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `config_request_id` | Identifiant unique de la demande | ✅ Oui |
| `operator_id` | Identifiant de l'operateur | ✅ Oui |
| `config_type` | Type de configuration | ✅ Oui |
| `config_payload` | Donnees de configuration | ✅ Oui |
| `justification` | Justification detaillee | ✅ Oui |
| `timestamp` | Horodatage de la demande | ✅ Oui |

**Types de configuration :**

| Type | Description | Validation SF |
|------|-------------|---------------|
| `SECURITY_LEVEL_ASSIGN` | Attribution niveau securite | Requise |
| `SECURITY_LEVEL_MODIFY` | Modification niveau securite | Requise |
| `DEGRADATION_ACTIVATE` | Activation degradation | Requise |
| `DEGRADATION_DEACTIVATE` | Desactivation degradation | Requise |
| `TRANSITION_RULE_MODIFY` | Modification regle transition | Requise |

---

## 8. Flux d'integration typiques

### 8.1 Flux de consultation (lecture seule)

**Acteurs :** MiyukiniAdmin, BondingBrother, WorrySentinel

**Sequence :**

1. MiyukiniAdmin soumet une demande de consultation via BondingBrother
2. BondingBrother valide la demande (capacite de lecture)
3. BondingBrother transmet a WorrySentinel
4. WorrySentinel genere la reponse
5. BondingBrother retourne la reponse a MiyukiniAdmin
6. MiyukiniAdmin affiche les donnees dans l'interface

**Diagramme :**

```
MiyukiniAdmin           BondingBrother              WorrySentinel
     │                        │                          │
     │──ConsultationRequest───▶│                          │
     │  (ws.state.trust.read)  │                          │
     │                        │                          │
     │                        │──ForwardRequest──────────▶│
     │                        │                          │
     │                        │                          │  [Generate response]
     │                        │                          │
     │                        │◀─TrustStateResponse──────│
     │                        │                          │
     │◀─ConsultationResponse──│                          │
     │                        │                          │
     │  [Display in UI]       │                          │
```

### 8.2 Flux de configuration (avec validation SF)

**Acteurs :** MiyukiniAdmin, BondingBrother, StrongFather, WorrySentinel

**Sequence :**

1. MiyukiniAdmin soumet une demande de configuration via BondingBrother
2. BondingBrother valide la capacite de configuration
3. BondingBrother soumet la decision a StrongFather
4. StrongFather evalue et decide (APPROVED/DENIED)
5. Si APPROVED, BondingBrother transmet a WorrySentinel
6. WorrySentinel applique la configuration
7. WorrySentinel confirme l'application
8. BondingBrother retourne le resultat a MiyukiniAdmin
9. MiyukiniAdmin affiche le resultat

**Diagramme :**

```
MiyukiniAdmin    BondingBrother    StrongFather    WorrySentinel
     │                │                 │                │
     │──ConfigRequest─▶│                 │                │
     │  (level=3)      │                 │                │
     │                │──DecisionReq────▶│                │
     │                │                 │                │
     │                │                 │  [Evaluate]    │
     │                │                 │                │
     │                │◀─APPROVED───────│                │
     │                │                 │                │
     │                │──ApplyConfig────────────────────▶│
     │                │                 │                │
     │                │                 │  [Apply]       │
     │                │                 │                │
     │                │◀─ConfigApplied──────────────────│
     │                │                 │                │
     │◀─ConfigResult──│                 │                │
     │  (SUCCESS)      │                 │                │
```

### 8.3 Flux de configuration refusee

**Acteurs :** MiyukiniAdmin, BondingBrother, StrongFather

**Sequence :**

1. MiyukiniAdmin soumet une demande de configuration
2. BondingBrother soumet a StrongFather
3. StrongFather refuse (DENIED avec raison)
4. BondingBrother retourne le refus a MiyukiniAdmin
5. MiyukiniAdmin affiche le refus avec la raison

**Diagramme :**

```
MiyukiniAdmin    BondingBrother    StrongFather    WorrySentinel
     │                │                 │                │
     │──ConfigRequest─▶│                 │                │
     │                │──DecisionReq────▶│                │
     │                │                 │                │
     │                │                 │  [Evaluate]    │
     │                │                 │                │
     │                │◀─DENIED─────────│                │
     │                │  (reason: ...)   │                │
     │                │                 │                │
     │◀─ConfigResult──│                 │                │
     │  (DENIED)       │                 │                │
     │  (reason: ...)  │                 │                │
```

---

## 9. Regles d'integration

### 9.1 Regles de consultation

**Regle WS-MA-CONS-01 : Consultation non bloquante**

Les consultations sont non bloquantes. MiyukiniAdmin peut continuer a fonctionner meme si une consultation est en cours.

**Regle WS-MA-CONS-02 : Donnees fraiches**

Les donnees retournees sont fraiches. WorrySentinel ne met pas en cache les reponses. L'etat affiche est l'etat actuel.

**Regle WS-MA-CONS-03 : Consultation tracee**

Toute consultation est tracee avec l'identite de l'operateur et l'horodatage.

### 9.2 Regles de configuration

**Regle WS-MA-CONF-01 : Validation obligatoire**

Toute configuration est soumise a validation StrongFather. Pas d'exception.

**Regle WS-MA-CONF-02 : Justification obligatoire**

Toute configuration doit etre accompagnee d'une justification detaillee (minimum 50 caracteres).

**Regle WS-MA-CONF-03 : Atomicite**

Les configurations sont atomiques. Une configuration est entierement appliquee ou pas du tout.

**Regle WS-MA-CONF-04 : Reversibilite**

Les configurations sont reversibles (sauf etat T4 qui est terminal).

### 9.3 Regles de tracabilite

**Regle WS-MA-TRACE-01 : Tracabilite complete**

Toute interaction (consultation et configuration) est tracee avec :
- Identite de l'operateur
- Horodatage
- Type d'interaction
- Donnees echangees
- Resultat

**Regle WS-MA-TRACE-02 : Retention longue**

Les traces sont conservees selon la politique de retention :
- Consultations : 1 an
- Configurations : 2 ans
- Configurations critiques : Permanent

---

## 10. Affichage UI des etats WorrySentinel

### 10.1 Indicateur d'etat de confiance

MiyukiniAdmin affiche l'etat de confiance de maniere visible et permanente :

```
┌───────────────────────────────────────────────────────────────┐
│ Etat de confiance systeme                                      │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│   🟢 T0 — NOMINAL                                             │
│   Fonctionnement normal, toutes capacites disponibles         │
│                                                               │
│   Depuis: 2026-01-28 08:00:00                                │
│   Derniere transition: T1 → T0 (resolution anomalie)          │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

### 10.2 Indicateurs par etat

| Etat | Indicateur | Couleur | Message |
|------|------------|---------|---------|
| **T0 — Normal** | 🟢 | Vert | Fonctionnement normal |
| **T1 — Instable** | 🟡 | Jaune | Anomalie detectee, surveillance renforcee |
| **T2 — Degrade** | 🟠 | Orange | Capacites reduites, restrictions actives |
| **T3 — Restreint** | 🔴 | Rouge | Mode restreint, intervention requise |
| **T4 — Bloque** | ⛔ | Rouge fonce | Systeme bloque, recovery requis |

### 10.3 Dashboard de securite

```
┌───────────────────────────────────────────────────────────────┐
│ Dashboard Securite - WorrySentinel                             │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│ ETAT GLOBAL: 🟢 T0 — NOMINAL                                  │
│                                                               │
│ ┌─────────────────────────────────────────────────────────┐  │
│ │ Niveaux de securite par produit                          │  │
│ ├───────────────────────┬─────────────────────────────────┤  │
│ │ Produit               │ Niveau                          │  │
│ ├───────────────────────┼─────────────────────────────────┤  │
│ │ MediaService          │ 2 — SENSITIVE                   │  │
│ │ AuthService           │ 3 — CRITICAL                    │  │
│ │ PublicAPI             │ 1 — STANDARD                    │  │
│ │ InternalTools         │ 0 — PUBLIC                      │  │
│ └───────────────────────┴─────────────────────────────────┘  │
│                                                               │
│ ┌─────────────────────────────────────────────────────────┐  │
│ │ Historique recent                                        │  │
│ ├─────────────────────────────────────────────────────────┤  │
│ │ 2026-01-28 10:00 - Transition T1 → T0 (resolution)      │  │
│ │ 2026-01-28 08:30 - Transition T0 → T1 (anomalie)        │  │
│ │ 2026-01-27 15:00 - Niveau AuthService: 2 → 3            │  │
│ └─────────────────────────────────────────────────────────┘  │
│                                                               │
│ [Modifier niveau securite]  [Activer degradation]             │
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

### 10.4 Formulaire de changement de niveau

```
┌───────────────────────────────────────────────────────────────┐
│ Changement de niveau de securite                               │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│ Produit: AuthService                                          │
│                                                               │
│ Niveau actuel: 2 — SENSITIVE                                  │
│                                                               │
│ Nouveau niveau:                                               │
│ ○ 0 — PUBLIC                                                  │
│ ○ 1 — STANDARD                                                │
│ ○ 2 — SENSITIVE (actuel)                                      │
│ ● 3 — CRITICAL                                                │
│ ○ 4 — HIGHEST SECURITY                                        │
│                                                               │
│ Justification (obligatoire):                                  │
│ ┌───────────────────────────────────────────────────────────┐│
│ │Augmentation suite a audit de securite. Le service gere    ││
│ │des donnees de paiement et necessite protection renforcee. ││
│ └───────────────────────────────────────────────────────────┘│
│                                                               │
│ ⚠️ Cette action sera soumise a validation StrongFather       │
│                                                               │
│ [Annuler]                              [Soumettre pour validation]│
│                                                               │
└───────────────────────────────────────────────────────────────┘
```

---

## 11. Gestion des erreurs

### 11.1 Types d'erreurs

**Erreurs de consultation :**
- WorrySentinel indisponible
- Entite non trouvee
- Donnees corrompues

**Erreurs de configuration :**
- Validation StrongFather refusee
- Configuration invalide
- Conflit avec regles existantes

**Erreurs de protocole :**
- Format invalide
- Champ obligatoire manquant
- Timeout

### 11.2 Traitement des erreurs

**Regle WS-MA-ERR-01 : Affichage explicite**

Toutes les erreurs sont affichees explicitement a l'utilisateur avec :
- Code d'erreur
- Description claire
- Action recommandee

**Regle WS-MA-ERR-02 : Journalisation**

Toutes les erreurs sont journalisees pour diagnostic.

**Regle WS-MA-ERR-03 : Pas de fallback implicite**

En cas d'erreur, MiyukiniAdmin n'applique pas de fallback implicite. L'action echoue explicitement.

**Regle WS-MA-ERR-04 : Retry autorise**

L'utilisateur peut retenter une action apres correction du probleme.

### 11.3 Codes d'erreur

| Code | Description | Action recommandee |
|------|-------------|-------------------|
| `WS-MA-001` | WorrySentinel indisponible | Attendre et retenter |
| `WS-MA-002` | Entite non trouvee | Verifier l'identifiant |
| `WS-MA-003` | Validation SF refusee | Consulter la raison du refus |
| `WS-MA-004` | Configuration invalide | Corriger les parametres |
| `WS-MA-005` | Justification insuffisante | Fournir plus de details |
| `WS-MA-006` | Conflit de regles | Consulter les regles existantes |
| `WS-MA-007` | Timeout | Retenter |

---

## 12. Cas particuliers

### 12.1 Etat T4 (Bloque)

En etat T4, les capacites de MiyukiniAdmin sont reduites :

**Capacites maintenues :**
- Consultation de l'etat de confiance
- Consultation des niveaux de securite
- Lecture des traces

**Capacites suspendues :**
- Modification des niveaux de securite (sauf via override)
- Modification des regles de gouvernance

**Regle WS-MA-CASE-01 : Override T4**

En T4, seul un override explicite avec validation TAMR permet de modifier la gouvernance. L'override est soumis a conditions cumulatives :
- Protocole securite renforce actif
- Validation StrongFather
- Confirmation TAMR
- Tracabilite renforcee

### 12.2 Mode offline

En mode offline :

**Regle WS-MA-CASE-02 : Consultation locale**

MiyukiniAdmin peut consulter le dernier etat connu de WorrySentinel (cache local).

**Regle WS-MA-CASE-03 : Configuration impossible**

Les modifications de gouvernance sont impossibles en mode offline. La validation StrongFather requiert une connexion.

### 12.3 MiyukiniAdmin en mode recovery

En mode recovery (conditions cumulatives strictes) :

**Regle WS-MA-CASE-04 : Acces maintenu**

MiyukiniAdmin conserve l'acces en lecture a WorrySentinel.

**Regle WS-MA-CASE-05 : Configuration limitee**

Les modifications de gouvernance sont limitees aux actions de recovery (escalade d'etat, override).

---

## 13. Garanties de l'integration

### 13.1 Garantie de visibilite

**Engagement :** MiyukiniAdmin affiche toujours l'etat reel de gouvernance. Aucun masquage ou falsification de l'etat.

### 13.2 Garantie de validation

**Engagement :** Toute modification de gouvernance est validee par StrongFather. Pas de bypass possible.

### 13.3 Garantie de tracabilite

**Engagement :** Toute interaction est tracee de bout en bout. L'audit complet des consultations et configurations est possible.

### 13.4 Garantie de non-regression

**Engagement :** Une modification de gouvernance n'est pas annulee automatiquement. Seule une action explicite (validee) peut modifier la gouvernance.

### 13.5 Garantie de disponibilite

**Engagement :** L'integration ne bloque jamais MiyukiniAdmin. En cas de defaillance de WorrySentinel, MiyukiniAdmin affiche le dernier etat connu avec mention de l'indisponibilite.

---

## 14. Invariants de l'integration

### 14.1 Invariants de relation

**INV-WS-MA-1 : Interface uniquement**

MiyukiniAdmin est une interface. MiyukiniAdmin ne gouverne jamais.

**INV-WS-MA-2 : Validation obligatoire**

Toute configuration est validee par StrongFather. Pas d'exception.

**INV-WS-MA-3 : Mediation BondingBrother**

Toute interaction passe par BondingBrother. Pas d'acces direct.

### 14.2 Invariants de donnees

**INV-WS-MA-4 : Donnees reelles**

Les donnees affichees sont les donnees reelles. Pas de cache trompeur.

**INV-WS-MA-5 : Tracabilite complete**

Toute interaction est tracee avec contexte complet.

### 14.3 Invariants de protocole

**INV-WS-MA-6 : Format respecte**

Toutes les consultations et configurations respectent le format standardise.

**INV-WS-MA-7 : Explicite toujours**

Toute action est explicite. Jamais implicite, jamais silencieuse.

---

## 15. Conformite aux Lois d'Autonomie Systeme

### LOI-1 : Aucune dependance externe critique

**Conformite :** ✅ **Conforme**

L'integration respecte LOI-1 :
- MiyukiniAdmin peut fonctionner offline pour consultation locale
- La gouvernance locale de WorrySentinel ne depend pas d'un cloud
- Les configurations sont appliquees localement

### LOI-2 : Le systeme accepte l'isolement comme etat normal

**Conformite :** ✅ **Conforme**

L'integration respecte LOI-2 :
- En isolement, MiyukiniAdmin consulte le cache local
- Les configurations sont reportees jusqu'a reconnexion
- L'interface reste fonctionnelle en lecture

### LOI-3 : Auto-suffisance fonctionnelle

**Conformite :** ✅ **Conforme**

L'integration respecte LOI-3 :
- MiyukiniAdmin embarque toute sa logique UI
- WorrySentinel embarque toute sa logique de gouvernance
- Aucune dependance externe pour le fonctionnement de base

---

## 16. Exemples

### 16.1 Consultation de l'etat de confiance

**Demande MiyukiniAdmin :**
```json
{
  "consultation_id": "cons-ws-ma-001",
  "capability": "ws.state.trust.read",
  "include_implications": true,
  "include_history": true,
  "history_limit": 5,
  "timestamp": "2026-01-28T12:00:00Z"
}
```

**Reponse WorrySentinel :**
```json
{
  "response_id": "resp-ws-001",
  "consultation_id": "cons-ws-ma-001",
  "current_state": "T0",
  "state_description": "Nominal - Fonctionnement normal, aucune anomalie",
  "implications": {
    "available_capabilities": "ALL",
    "restrictions": [],
    "monitoring_level": "STANDARD"
  },
  "transitions_history": [
    {
      "from": "T1",
      "to": "T0",
      "timestamp": "2026-01-28T10:00:00Z",
      "reason": "Anomalie resolue - faux positif identifie"
    },
    {
      "from": "T0",
      "to": "T1",
      "timestamp": "2026-01-28T08:30:00Z",
      "reason": "Anomalie detectee - pattern de requetes suspect"
    }
  ],
  "timestamp": "2026-01-28T12:00:01Z"
}
```

### 16.2 Configuration de niveau de securite

**Demande MiyukiniAdmin :**
```json
{
  "config_request_id": "conf-ws-ma-001",
  "operator_id": "admin-user-001",
  "config_type": "SECURITY_LEVEL_MODIFY",
  "config_payload": {
    "entity_type": "product",
    "entity_id": "auth-service",
    "current_level": 2,
    "new_level": 3
  },
  "justification": "Augmentation suite a audit de securite. Le service gere des donnees de paiement et necessite une protection renforcee. Ref: AUDIT-2026-0128",
  "timestamp": "2026-01-28T12:05:00Z"
}
```

**Validation StrongFather (APPROVED) :**
```json
{
  "decision_id": "dec-sf-001",
  "config_request_id": "conf-ws-ma-001",
  "decision": "APPROVED",
  "reasoning": "Justification valide, role admin confirme, audit reference presente",
  "timestamp": "2026-01-28T12:05:02Z"
}
```

**Application WorrySentinel :**
```json
{
  "application_id": "app-ws-001",
  "config_request_id": "conf-ws-ma-001",
  "status": "APPLIED",
  "entity_id": "auth-service",
  "previous_level": 2,
  "new_level": 3,
  "effective_at": "2026-01-28T12:05:03Z",
  "timestamp": "2026-01-28T12:05:03Z"
}
```

**Resultat final MiyukiniAdmin :**
```json
{
  "result_id": "res-ma-001",
  "config_request_id": "conf-ws-ma-001",
  "status": "SUCCESS",
  "message": "Niveau de securite de auth-service modifie de 2 (SENSITIVE) a 3 (CRITICAL)",
  "decision_id": "dec-sf-001",
  "application_id": "app-ws-001",
  "timestamp": "2026-01-28T12:05:04Z"
}
```

### 16.3 Configuration refusee

**Demande MiyukiniAdmin :**
```json
{
  "config_request_id": "conf-ws-ma-002",
  "operator_id": "viewer-user-001",
  "config_type": "SECURITY_LEVEL_MODIFY",
  "config_payload": {
    "entity_type": "product",
    "entity_id": "public-api",
    "current_level": 1,
    "new_level": 0
  },
  "justification": "Reduction pour test",
  "timestamp": "2026-01-28T12:10:00Z"
}
```

**Validation StrongFather (DENIED) :**
```json
{
  "decision_id": "dec-sf-002",
  "config_request_id": "conf-ws-ma-002",
  "decision": "DENIED",
  "reasoning": "Role insuffisant (Viewer, Admin requis). Justification trop courte (15 chars, min 50)",
  "timestamp": "2026-01-28T12:10:01Z"
}
```

**Resultat MiyukiniAdmin :**
```json
{
  "result_id": "res-ma-002",
  "config_request_id": "conf-ws-ma-002",
  "status": "DENIED",
  "error_code": "WS-MA-003",
  "message": "Configuration refusee par StrongFather",
  "reason": "Role insuffisant (Viewer, Admin requis). Justification trop courte (15 chars, min 50)",
  "timestamp": "2026-01-28T12:10:02Z"
}
```

---

## 17. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il etablit l'interface et le protocole que MiyukiniAdmin doit respecter pour s'integrer avec WorrySentinel.

Toute implementation de l'integration avec WorrySentinel doit respecter ce contrat. Toute violation entraine un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dependances :**
- WorrySentinel - Documentation Fondatrice v1.2 (Section 11)
- MiyukiniAdmin - Documentation Fondatrice v1.0.0
- Miyukini Conceptual References - Security Levels
- Miyukini Framework - Integrity Degradation System

---

## 18. Mini log de generation

### Decision editoriale E1 : Direction de la relation

**Decision prise :** La relation est d'interface administrative : MiyukiniAdmin expose les capacites de gouvernance aux administrateurs humains, WorrySentinel maintient l'autorite. Cette direction respecte la Documentation Fondatrice de WorrySentinel Section 11 et le role de MiyukiniAdmin comme console root.

**Application :** Tout le document est structure autour de cette relation d'interface.

### Decision editoriale E2 : Validation obligatoire

**Decision prise :** Toute modification de gouvernance est soumise a validation StrongFather, conformement a REGLE-ADMIN-1 de la Documentation Fondatrice WorrySentinel. Pas d'exception ni de bypass.

**Application :** Regle WS-MA-02, INV-WS-MA-2, et flux de configuration etablissent cette obligation.

### Decision editoriale E3 : Separation consultation/configuration

**Decision prise :** Les capacites sont divisees en consultation (lecture seule, pas de validation) et configuration (validation obligatoire). Cette separation permet une interface reactive pour la lecture.

**Application :** Section 6 et Section 7 definissent clairement les deux types de capacites.

### Decision editoriale E4 : Affichage UI

**Decision prise :** Inclusion de maquettes UI pour guider l'implementation. L'affichage doit etre explicite, permanent, et reflechir l'etat reel. Conformement a INV-MA-10 (jamais silencieux, jamais implicite).

**Application :** Section 10 definit les indicateurs visuels et les formulaires.

### Warning W1 : Risque de confusion autorite

**Warning rencontre :** Risque que MiyukiniAdmin soit percu comme ayant autorite sur la gouvernance.

**Decision prise :** Les interdictions absolues (Section 5) clarifient que MiyukiniAdmin est une interface, pas une autorite. L'autorite reste a WorrySentinel et StrongFather.

**Correction effectuee :** Section 4 et Section 5 explicitent la separation. INV-WS-MA-1 etablit que MiyukiniAdmin ne gouverne jamais.

### Verification de coherence

**Verification effectuee :**
- ✅ Coherence avec WorrySentinel - Documentation Fondatrice : Confirmee (Section 11 respectee)
- ✅ Coherence avec MiyukiniAdmin - Documentation Fondatrice : Confirmee (INV-MA-4, INV-MA-10)
- ✅ Conformite LOI-1 : Confirmee (consultation locale possible)
- ✅ Conformite LOI-2 : Confirmee (isolement gere)
- ✅ Conformite LOI-3 : Confirmee (auto-suffisance)
- ✅ Validation StrongFather obligatoire : Confirmee (WS-MA-02, INV-WS-MA-2)
- ✅ Mediation BondingBrother : Confirmee (WS-MA-03, INV-WS-MA-3)
- ✅ Tracabilite complete : Confirmee (INV-WS-MA-5)

**Conclusion :** Aucune contradiction detectee. Le document est coherent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguite rencontree lors de la redaction de ce document.*
