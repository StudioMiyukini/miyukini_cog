# Caring Nanny - Error & Rejection Model

## 1. Contexte

Ce document définit le **modèle d'erreur et de rejet** de Caring Nanny dans le Miyukini Core System. Caring Nanny étant un **observateur pur** (INV-CN-1), son modèle d'erreur est fondamentalement différent de celui des autres cores : Caring Nanny ne rejette jamais d'opérations, ne bloque jamais le système, et ne prend jamais de décision corrective.

**Question fondamentale :**

> *"Comment Caring Nanny gère-t-elle les erreurs tout en restant un observateur passif qui ne bloque jamais ?"*

Ce document répond à cette question en définissant :
- Les catégories d'erreurs que Caring Nanny peut rencontrer
- Les conditions d'invalidité dans le processus d'observation
- Les stratégies de dégradation gracieuse
- Les garanties de continuité même en cas d'erreur

**Références normatives :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) — Invariants fondateurs (Section 7)
- [Caring Nanny - Invariants et Garanties](./Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) — Garanties contractuelles
- [Caring Nanny - Violations & Anti-Patterns](./Caring%20Nanny%20-%20Violations%20%26%20Anti-Patterns.md) — Violations à éviter
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) — LOI-1 à LOI-6

## 2. Portée / Scope

Ce document couvre :
- Le modèle d'erreur interne au processus d'observation
- Les conditions d'invalidité des observations
- Les erreurs dans le flux de propagation
- Les stratégies de réponse qui préservent l'invariant non-bloquant (INV-CN-6)
- La classification des états d'erreur du point de vue de l'observateur

Ce document **ne couvre pas** :
- Les erreurs des composants observés (elles sont des faits à observer, pas des erreurs de Caring Nanny)
- Les violations d'invariants (voir Violations & Anti-Patterns)
- Les stratégies de correction (Caring Nanny ne corrige jamais — INV-NEG-CN-03)
- Les erreurs de décision (responsabilité de StrongFather)

---

## 3. Principes fondamentaux du modèle d'erreur

### 3.1 Caring Nanny ne bloque jamais

**Invariant absolu (INV-CN-6) :** Caring Nanny ne bloque **jamais** les opérations du système. Même en cas d'erreur interne, le système continue de fonctionner.

**Conséquences pour le modèle d'erreur :**
- Aucune erreur ne peut arrêter le flux d'observation
- Aucune erreur ne peut empêcher une réponse à une consultation
- Aucune erreur ne peut bloquer une propagation
- Les erreurs sont enregistrées, rapportées, mais jamais bloquantes

### 3.2 Caring Nanny ne rejette pas au sens traditionnel

Le terme "rejet" dans ce contexte est différent du rejet de StrongFather :

| Core | Nature du rejet |
|------|-----------------|
| **StrongFather** | Refuse une intention, bloque une action |
| **KindMother** | Refuse une écriture non conforme |
| **Caring Nanny** | **N'existe pas** — Caring Nanny accepte tout ce qu'elle peut observer |

Pour Caring Nanny, "rejet" signifie :
- Impossibilité de traiter une observation (mais observation enregistrée quand même)
- Détection d'une condition invalide (mais condition rapportée quand même)
- Incapacité à classifier un état (mais état "unknown" retourné quand même)

### 3.3 Dégradation gracieuse obligatoire

Tout scénario d'erreur doit aboutir à un état dégradé documenté, jamais à un blocage :

```
Erreur détectée → Stratégie de dégradation → Réponse dégradée
                                           → Enregistrement de l'erreur
                                           → Propagation si pertinente
```

---

## 4. Taxonomie des erreurs

### 4.1 Classification par origine

| Code | Origine | Description |
|------|---------|-------------|
| **ERR-OBS** | Observation | Erreur dans le processus de détection d'une condition |
| **ERR-CLS** | Classification | Erreur dans la catégorisation d'un état |
| **ERR-AGR** | Agrégation | Erreur dans l'agrégation des états partiels |
| **ERR-PRO** | Propagation | Erreur dans la transmission d'un changement d'état |
| **ERR-HIS** | Historisation | Erreur dans l'enregistrement de l'historique |
| **ERR-CON** | Consultation | Erreur dans la réponse à une demande d'état |
| **ERR-INT** | Interne | Erreur technique interne à Caring Nanny |

### 4.2 Classification par sévérité

| Niveau | Code | Description | Impact |
|--------|------|-------------|--------|
| **Critique** | SEV-CRIT | Erreur compromettant l'intégrité de l'observation | Dégradation majeure, alerte immédiate |
| **Majeure** | SEV-MAJ | Erreur affectant la qualité de l'observation | Dégradation partielle, alerte |
| **Mineure** | SEV-MIN | Erreur sans impact significatif | Enregistrement, pas d'alerte |
| **Info** | SEV-INFO | Condition anormale non bloquante | Enregistrement seul |

### 4.3 Classification par récupérabilité

| Type | Description | Stratégie |
|------|-------------|-----------|
| **Transitoire** | Erreur temporaire, résolution automatique probable | Retry avec backoff, fallback temporaire |
| **Permanente** | Erreur persistante jusqu'à intervention | Dégradation stable, alerte pour intervention |
| **Intermittente** | Erreur qui apparaît et disparaît | Historisation du pattern, dégradation adaptative |

---

## 5. Erreurs d'observation (ERR-OBS)

### 5.1 ERR-OBS-001 : Source d'observation indisponible

**Description :** Le composant à observer ne répond pas ou n'est pas accessible.

**Causes possibles :**
- Composant en cours de démarrage
- Composant défaillant
- Connexion locale rompue
- Ressource temporairement verrouillée

**Réponse NON-BLOQUANTE :**
1. Retourner l'état `unknown` pour ce composant
2. Enregistrer la tentative d'observation avec l'échec
3. Propager l'incertitude si pertinent
4. Planifier une nouvelle tentative (sans blocage)

**Ce que Caring Nanny NE fait PAS :**
- ❌ Bloquer en attente de la source
- ❌ Retry infini bloquant
- ❌ Déclencher une action corrective
- ❌ Ignorer silencieusement l'échec

### 5.2 ERR-OBS-002 : Format d'observation invalide

**Description :** Les données reçues de la source ne correspondent pas au format attendu.

**Causes possibles :**
- Version incompatible du composant observé
- Données corrompues
- Réponse partielle
- Erreur de sérialisation

**Réponse NON-BLOQUANTE :**
1. Enregistrer le format invalide reçu (pour diagnostic)
2. Tenter un parsing partiel si possible
3. Retourner `unknown` avec indication "format_invalid"
4. Journaliser avec sévérité SEV-MAJ

### 5.3 ERR-OBS-003 : Observation incomplète

**Description :** L'observation a réussi partiellement mais des informations manquent.

**Causes possibles :**
- Timeout partiel
- Composant en état transitoire
- Réponse tronquée

**Réponse NON-BLOQUANTE :**
1. Utiliser les informations disponibles
2. Marquer l'observation comme "partielle"
3. Compléter avec le dernier état connu si disponible
4. Signaler l'incomplétude dans le contexte

---

## 6. Erreurs de classification (ERR-CLS)

### 6.1 ERR-CLS-001 : Condition non classifiable

**Description :** Une condition observée ne correspond à aucun critère de classification connu.

**Causes possibles :**
- Nouveau type de condition non prévu
- Combinaison de conditions inédite
- Règles de classification incomplètes

**Réponse NON-BLOQUANTE :**
1. Classifier comme `unknown` avec sous-catégorie "unclassified"
2. Enregistrer la condition brute complète
3. Propager avec indication "requires_classification_update"
4. **Ne pas inventer** une classification

### 6.2 ERR-CLS-002 : Classification ambiguë

**Description :** Une condition correspond à plusieurs catégories mutuellement exclusives.

**Causes possibles :**
- Règles de classification avec chevauchement
- Condition à la frontière entre deux états
- Données contradictoires dans la condition

**Réponse NON-BLOQUANTE :**
1. Appliquer la règle de priorité : `error > degraded > syncing > offline > healthy`
2. Enregistrer l'ambiguïté avec toutes les catégories candidates
3. Propager l'état de priorité maximale
4. Journaliser pour revue des règles

### 6.3 ERR-CLS-003 : Critères de classification indisponibles

**Description :** Les règles de classification ne sont pas chargées ou sont corrompues.

**Causes possibles :**
- Erreur au démarrage
- Configuration corrompue
- Règles non trouvées

**Réponse NON-BLOQUANTE (Critique) :**
1. Activer le mode dégradé : classification minimale (healthy/error/unknown)
2. Journaliser avec sévérité SEV-CRIT
3. Retourner des états avec indication "degraded_classification"
4. Tenter de recharger les règles périodiquement (sans blocage)

---

## 7. Erreurs d'agrégation (ERR-AGR)

### 7.1 ERR-AGR-001 : États partiels incohérents

**Description :** Les états partiels des composants contiennent des contradictions lors de l'agrégation.

**Exemple :**
- Composant A rapporte que Composant B est `healthy`
- Observation directe de Composant B indique `error`

**Réponse NON-BLOQUANTE :**
1. Priorité à l'observation directe sur les observations indirectes
2. Enregistrer l'incohérence avec les deux sources
3. Agréger avec l'état de priorité maximale (error > autres)
4. Propager l'état avec indication "inconsistency_detected"

### 7.2 ERR-AGR-002 : État partiel manquant

**Description :** Un composant attendu n'a pas d'état observable.

**Réponse NON-BLOQUANTE :**
1. Considérer le composant comme `unknown`
2. Agréger les autres états normalement
3. L'état global reflète l'incertitude si le composant est critique
4. Enregistrer le composant manquant

### 7.3 ERR-AGR-003 : Cycle de dépendance d'état

**Description :** Détection d'une dépendance circulaire dans les états des composants.

**Exemple :**
- État de A dépend de B, état de B dépend de C, état de C dépend de A

**Réponse NON-BLOQUANTE :**
1. Briser le cycle en utilisant les derniers états connus
2. Enregistrer le cycle détecté
3. Journaliser avec SEV-MAJ (anomalie architecturale)
4. Ne pas tenter de résoudre le cycle (ce n'est pas une décision)

---

## 8. Erreurs de propagation (ERR-PRO)

### 8.1 ERR-PRO-001 : BondingBrother indisponible

**Description :** Impossible de déléguer la propagation à BondingBrother.

**Causes possibles :**
- BondingBrother non démarré
- Connexion locale rompue
- BondingBrother surchargé

**Réponse NON-BLOQUANTE :**
1. Buffer local de la notification (avec limite)
2. Enregistrer la propagation comme "pending"
3. Retry périodique (sans blocage)
4. Si buffer plein : prioritiser (error > degraded > autres), journaliser les propagations perdues

**Conformité LOI-1 :** Caring Nanny continue de fonctionner même si BondingBrother est indisponible.

### 8.2 ERR-PRO-002 : Destinataire inconnu

**Description :** Un destinataire de la propagation n'est pas identifiable.

**Réponse NON-BLOQUANTE :**
1. Propager aux destinataires connus
2. Enregistrer le destinataire inconnu
3. Ne pas bloquer la propagation pour les autres

### 8.3 ERR-PRO-003 : Propagation rejetée par BondingBrother

**Description :** BondingBrother refuse la propagation (format invalide, quota, etc.).

**Réponse NON-BLOQUANTE :**
1. Enregistrer le rejet avec la raison
2. Stocker temporairement pour analyse
3. Ne pas retry automatiquement sans correction
4. Journaliser avec SEV-MAJ

---

## 9. Erreurs d'historisation (ERR-HIS)

### 9.1 ERR-HIS-001 : Stockage d'historique saturé

**Description :** L'espace alloué à l'historique est plein.

**Réponse NON-BLOQUANTE :**
1. Appliquer la politique de rétention (supprimer les plus anciens)
2. Si politique échoue : mode dégradé (historique réduit)
3. Journaliser avec SEV-MAJ
4. Continuer à observer et à propager

**Conformité LOI-5 :** L'historique a des limites prévisibles.

### 9.2 ERR-HIS-002 : Corruption de l'historique

**Description :** Des entrées de l'historique sont corrompues ou incohérentes.

**Réponse NON-BLOQUANTE :**
1. Isoler les entrées corrompues
2. Continuer l'historisation des nouvelles observations
3. Marquer les entrées corrompues comme "corrupted"
4. Journaliser avec SEV-CRIT pour intervention

### 9.3 ERR-HIS-003 : Écriture d'historique échouée

**Description :** Impossible d'écrire une nouvelle entrée dans l'historique.

**Réponse NON-BLOQUANTE :**
1. Buffer temporaire en mémoire
2. Retry d'écriture périodique
3. Si buffer saturé : prioritiser les observations critiques
4. **Ne jamais** bloquer l'observation pour cause d'historisation

---

## 10. Erreurs de consultation (ERR-CON)

### 10.1 ERR-CON-001 : Requête d'état invalide

**Description :** Une demande d'état ne peut pas être interprétée.

**Exemples :**
- Identifiant de composant inconnu
- Format de requête invalide
- Paramètres manquants

**Réponse NON-BLOQUANTE :**
1. Retourner une réponse d'erreur structurée (pas une exception)
2. Inclure l'état global si le composant spécifique est inconnu
3. Enregistrer la requête invalide
4. **Toujours** retourner une réponse

**Format de réponse d'erreur :**
```
{
  state: "unknown",
  error: {
    code: "ERR-CON-001",
    message: "Composant non trouvé",
    requested_component: "xxx"
  },
  timestamp: <local_timestamp>,
  global_state: <état_global_si_disponible>
}
```

### 10.2 ERR-CON-002 : État non disponible temporairement

**Description :** L'état demandé n'est pas encore calculé ou est en cours de mise à jour.

**Réponse NON-BLOQUANTE :**
1. Retourner le dernier état connu
2. Inclure l'indication "stale" avec l'âge de l'état
3. **Ne jamais** bloquer en attente de l'état frais

**Format de réponse :**
```
{
  state: <dernier_état_connu>,
  staleness: {
    is_stale: true,
    age_ms: <durée_depuis_dernière_observation>,
    reason: "observation_in_progress"
  },
  timestamp: <timestamp_de_l'état_retourné>
}
```

---

## 11. Erreurs internes (ERR-INT)

### 11.1 ERR-INT-001 : Erreur technique non récupérable

**Description :** Erreur interne qui ne peut pas être gérée par les mécanismes standard.

**Exemples :**
- Out of memory
- Stack overflow
- Corruption de l'état interne

**Réponse :**
1. Mode de survie minimal : retourner `unknown` pour toutes les requêtes
2. Journaliser autant que possible
3. Alerter pour intervention urgente
4. **Ne pas tenter** de correction automatique (violation potentielle)

### 11.2 ERR-INT-002 : Configuration invalide

**Description :** La configuration de Caring Nanny est invalide ou incomplète.

**Réponse NON-BLOQUANTE :**
1. Utiliser les valeurs par défaut quand possible
2. Mode dégradé avec capacités réduites
3. Journaliser les éléments de configuration manquants
4. Continuer à fonctionner avec les capacités disponibles

---

## 12. États d'erreur observés vs erreurs de Caring Nanny

### 12.1 Distinction fondamentale

Il est **critique** de distinguer :

| Type | Description | Responsabilité de Caring Nanny |
|------|-------------|-------------------------------|
| **État d'erreur observé** | Un composant EST en état `error` | Observer, classifier, propager |
| **Erreur de Caring Nanny** | Caring Nanny a un problème interne | Gérer, dégrader gracieusement, continuer |

**Exemple :**
- KindMother est en état `error` → Caring Nanny **observe** et **rapporte** l'état `error`
- Caring Nanny ne peut pas contacter KindMother → Caring Nanny **gère** son erreur et retourne `unknown`

### 12.2 L'état `error` n'est pas une erreur de Caring Nanny

Quand Caring Nanny rapporte l'état `error` pour un composant :
- C'est une **observation correcte** d'un fait
- Ce n'est **pas** une erreur de Caring Nanny
- Caring Nanny a **réussi** son travail d'observation

### 12.3 L'état `unknown` signale une limite de Caring Nanny

Quand Caring Nanny retourne `unknown` :
- Caring Nanny reconnaît qu'elle **ne sait pas**
- C'est une réponse **honnête**, pas un échec
- Le consommateur peut prendre une décision en connaissance de cause

---

## 13. Conditions de rejet (au sens Caring Nanny)

### 13.1 Ce que "rejet" signifie pour Caring Nanny

Caring Nanny ne "rejette" pas au sens traditionnel. Les conditions de rejet sont des situations où Caring Nanny **ne peut pas fournir** l'information demandée dans le format attendu.

### 13.2 Condition REJ-001 : Observation impossible

**Condition :** Caring Nanny ne peut absolument pas observer un composant.

**Réponse :** État `unknown` avec contexte `observation_impossible`

**Ce qui n'est PAS un rejet :**
- Le composant existe mais est en erreur → observation réussie (état `error`)
- Le composant est lent à répondre → observation avec timeout puis `unknown`

### 13.3 Condition REJ-002 : Classification impossible

**Condition :** Une condition ne peut être classifiée même avec les règles de fallback.

**Réponse :** État `unknown` avec sous-catégorie `unclassifiable`

### 13.4 Condition REJ-003 : Propagation impossible

**Condition :** Impossible de propager même après les stratégies de retry et de buffer.

**Réponse :** Propagation enregistrée comme `failed`, observation conservée dans l'historique.

---

## 14. Dégradation gracieuse

### 14.1 Niveaux de dégradation

| Niveau | Description | Capacités |
|--------|-------------|-----------|
| **Nominal** | Toutes fonctions opérationnelles | Observation, classification, agrégation, propagation, historisation |
| **Dégradé léger** | Quelques fonctions limitées | Classification simplifiée, historique réduit |
| **Dégradé sévère** | Fonctions minimales uniquement | Observation basique, état `unknown` fréquent |
| **Survie** | Mode minimal | Retourne `unknown` pour tout, journalise ce qui est possible |

### 14.2 Transitions de dégradation

Les transitions entre niveaux sont :
- **Automatiques** (vers plus dégradé) : en réponse aux erreurs
- **Automatiques** (vers moins dégradé) : quand les conditions s'améliorent
- **Jamais bloquantes** : une transition ne peut pas bloquer le système

### 14.3 Signalement de la dégradation

Chaque réponse de Caring Nanny inclut son niveau de dégradation :

```
{
  state: <état_observé>,
  degradation_level: "nominal" | "light" | "severe" | "survival",
  degradation_reasons: [<liste_des_raisons>]
}
```

---

## 15. Conformité aux Lois d'Autonomie

### 15.1 LOI-1 : Aucune dépendance externe critique

**Conformité :** Toutes les stratégies de gestion d'erreur fonctionnent sans dépendance externe.
- Les règles de classification sont locales
- L'historique est local
- La dégradation est gérée localement

### 15.2 LOI-2 : Isolement accepté comme état normal

**Conformité :** Les erreurs de connexion ne sont pas traitées comme critiques.
- BondingBrother indisponible → buffer local
- Source d'observation indisponible → état `unknown`
- Pas de retry infini bloquant

### 15.3 LOI-3 : État local souverain

**Conformité :** L'historique local est la source de vérité.
- Les erreurs sont enregistrées localement
- Pas de validation externe de l'état

### 15.4 LOI-4 : Pas de temps global requis

**Conformité :** Toutes les timestamps sont locales.
- Les erreurs sont horodatées localement
- Pas de synchronisation temporelle requise

### 15.5 LOI-5 : Coût proportionnel au hardware

**Conformité :** Les stratégies de gestion d'erreur sont économes.
- Buffer avec limites
- Pas de retry exponentiel infini
- Historique avec rétention

### 15.6 LOI-6 : Autonomie préservée avec fédération

**Conformité :** Les erreurs de fédération ne bloquent pas.
- Propagation vers nœuds fédérés en best-effort
- Pas de dépendance au réseau de fédération

---

## 16. Modèle de réponse aux erreurs

### 16.1 Structure de réponse unifiée

Toute réponse de Caring Nanny, même en cas d'erreur, suit cette structure :

```
{
  // Toujours présent
  timestamp: <timestamp_local>,
  degradation_level: "nominal" | "light" | "severe" | "survival",
  
  // État (toujours présent, même si "unknown")
  state: "healthy" | "degraded" | "offline" | "syncing" | "error" | "unknown",
  
  // Contexte de l'état
  state_context: {
    is_stale: <boolean>,
    age_ms: <durée_si_stale>,
    confidence: "high" | "medium" | "low" | "none"
  },
  
  // Erreurs rencontrées (optionnel)
  errors: [
    {
      code: "ERR-xxx-xxx",
      severity: "CRIT" | "MAJ" | "MIN" | "INFO",
      message: <description>,
      recoverable: <boolean>
    }
  ],
  
  // Composant spécifique (si demandé)
  component: <identifiant_si_applicable>
}
```

### 16.2 Garantie de réponse

**Invariant absolu :** Caring Nanny retourne **toujours** une réponse conforme à cette structure, même en cas d'erreur critique.

Cas extrême (erreur non récupérable) :
```
{
  timestamp: <timestamp_local>,
  degradation_level: "survival",
  state: "unknown",
  state_context: {
    is_stale: true,
    age_ms: null,
    confidence: "none"
  },
  errors: [
    {
      code: "ERR-INT-001",
      severity: "CRIT",
      message: "Erreur interne non récupérable",
      recoverable: false
    }
  ]
}
```

---

## 17. Journalisation des erreurs

### 17.1 Format de journalisation

Toutes les erreurs sont journalisées avec :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `timestamp` | Horodatage local | Oui |
| `error_code` | Code de l'erreur (ERR-xxx-xxx) | Oui |
| `severity` | Sévérité (CRIT/MAJ/MIN/INFO) | Oui |
| `context` | Contexte de l'erreur | Oui |
| `attempted_action` | Action qui a échoué | Oui |
| `fallback_applied` | Stratégie de dégradation appliquée | Oui |
| `outcome` | Résultat après fallback | Oui |

### 17.2 Rétention des journaux d'erreur

Les journaux d'erreur suivent une politique de rétention configurable :
- Erreurs CRIT : rétention maximale
- Erreurs MAJ : rétention standard
- Erreurs MIN/INFO : rétention courte ou agrégation

---

## 18. Matrice de correspondance erreurs/réponses

| Code erreur | Réponse état | Dégradation | Action |
|-------------|--------------|-------------|--------|
| ERR-OBS-001 | `unknown` | light | Retry non-bloquant |
| ERR-OBS-002 | `unknown` | light | Journaliser |
| ERR-OBS-003 | `<partiel>` | light | Compléter si possible |
| ERR-CLS-001 | `unknown` | light | Journaliser |
| ERR-CLS-002 | `<priorité_max>` | nominal | Journaliser ambiguïté |
| ERR-CLS-003 | `<classification_minimale>` | severe | Retry chargement |
| ERR-AGR-001 | `<priorité_max>` | light | Journaliser incohérence |
| ERR-AGR-002 | `<agrégation_partielle>` | light | Signaler manquant |
| ERR-AGR-003 | `<dernier_connu>` | light | Journaliser cycle |
| ERR-PRO-001 | N/A (propagation) | light | Buffer local |
| ERR-PRO-002 | N/A (propagation) | nominal | Propager aux autres |
| ERR-PRO-003 | N/A (propagation) | light | Journaliser |
| ERR-HIS-001 | N/A (historique) | light | Appliquer rétention |
| ERR-HIS-002 | N/A (historique) | severe | Isoler entrées |
| ERR-HIS-003 | N/A (historique) | light | Buffer mémoire |
| ERR-CON-001 | `unknown` | nominal | Retourner erreur structurée |
| ERR-CON-002 | `<dernier_connu>` | nominal | Indiquer staleness |
| ERR-INT-001 | `unknown` | survival | Mode survie |
| ERR-INT-002 | `<selon_disponible>` | severe | Valeurs par défaut |

---

## 19. Statut contractuel

Ce document est **contractuel, normatif, et de statut GOUVERNANCE**. Il définit le modèle d'erreur et de rejet de Caring Nanny, garantissant que Caring Nanny reste un observateur non-bloquant même en cas d'erreur.

Toute implémentation de Caring Nanny doit :
- Implémenter toutes les stratégies de dégradation gracieuse
- Garantir une réponse pour toute consultation, même en cas d'erreur
- Ne jamais bloquer le système en raison d'une erreur interne
- Journaliser toutes les erreurs rencontrées

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** GOUVERNANCE — Modèle normatif  
**Dépendances :**
- Caring Nanny - Documentation Fondatrice v1.6
- Caring Nanny - Invariants et Garanties v1.0
- Caring Nanny - Violations & Anti-Patterns v1.0
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1
