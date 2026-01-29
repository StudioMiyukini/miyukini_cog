# WorrySentinel - Examples & Use Cases

## 1. Contexte

Ce document présente des **exemples concrets** et des **cas d'usage** de WorrySentinel. Il illustre comment la gouvernance de sécurité s'applique dans des scénarios réels, en respectant les contrats et les invariants.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Statut :** Ce document est **illustratif**. Les exemples servent à clarifier les concepts, pas à définir de nouvelles règles.

---

## 2. Exemples de niveaux de sécurité

### 2.1 Exemple : Site vitrine (Niveau 0)

**Contexte :**
Une entreprise déploie un site vitrine présentant ses services. Le site n'a pas de compte utilisateur, pas de données sensibles, uniquement du contenu public.

**Profil de sécurité :**

```
Product Security Profile:
  product_id: "site-vitrine-acme"
  required_level: 0
  justification: "Contenu public uniquement, aucune donnée sensible"
  offline_allowed: true
  degradation_allowed: true
```

**Gouvernance appliquée :**

| Core | Adaptation |
|------|------------|
| StrongFather | Décisions simplifiées |
| MasterButler | Permissions publiques |
| BorderGuard | Frontières assouplies |
| CaringNanny | Monitoring minimal |
| TAMR | Aucune intervention requise |

**Comportement en état dégradé :**

| État | Comportement |
|------|--------------|
| T0 | Normal |
| T1 | + traces (minimal) |
| T2 | Fonctions non essentielles bridées |
| T3 | Lecture seule |
| T4 | Bloqué |

**Principe appliqué :** *"Si ça casse, ce n'est pas grave."*

---

### 2.2 Exemple : CMS éditorial (Niveau 1)

**Contexte :**
Une équipe marketing utilise un CMS pour gérer du contenu éditorial. Le système a des comptes utilisateurs simples mais ne manipule pas de données personnelles sensibles.

**Profil de sécurité :**

```
Product Security Profile:
  product_id: "cms-editorial-v2"
  required_level: 1
  justification: "Gestion de contenu éditorial avec auth simple"
  offline_allowed: true
  degradation_allowed: true
```

**Gouvernance appliquée :**

| Core | Adaptation |
|------|------------|
| StrongFather | Décisions standard, validation normale |
| MasterButler | Permissions basiques |
| BorderGuard | Frontières standard |
| CaringNanny | Monitoring normal |
| TAMR | Intervention optionnelle |

**Scénario de dégradation :**

1. **T0 (Normal)** : Édition et publication normales
2. **T1 (Anomalie détectée)** : Log renforcé, édition normale
3. **T2 (Incohérence persistante)** : Publication désactivée, édition OK
4. **T3 (Suspicion forte)** : Lecture seule
5. **T4 (Compromis)** : Bloqué

---

### 2.3 Exemple : Gestion de profils utilisateurs (Niveau 2)

**Contexte :**
Une application SaaS gère des profils utilisateurs avec données personnelles : nom, email, préférences, historique d'utilisation.

**Profil de sécurité :**

```
Product Security Profile:
  product_id: "user-profile-service"
  required_level: 2
  justification: "Données personnelles utilisateurs (RGPD)"
  offline_allowed: true
  degradation_allowed: true
```

**Gouvernance appliquée :**

| Core | Adaptation |
|------|------------|
| StrongFather | Décisions renforcées, validation stricte |
| MasterButler | Permissions détaillées |
| BorderGuard | Frontières renforcées |
| CaringNanny | Monitoring actif, détection anomalies |
| TAMR | Intervention possible |
| BondingBrother | Traçabilité complète |

**Contraintes spécifiques :**
- Auth renforcée obligatoire
- Signatures d'intentions
- Traçabilité complète
- Contrôles de cohérence réguliers

**Interaction avec Tools :**

```
Requête Tool: "Exporter profil utilisateur"
Vérification WorrySentinel:
  - Niveau produit: 2 (Sensitive Data)
  - État confiance: T0 (Normal)
  - Résultat: Autorisé avec traçabilité complète
```

---

### 2.4 Exemple : Module d'authentification (Niveau 3)

**Contexte :**
Le module d'authentification gère les identifiants, tokens, et sessions utilisateurs. Il est critique pour la sécurité de l'ensemble du système.

**Profil de sécurité :**

```
Product Security Profile:
  product_id: "auth-module-core"
  required_level: 3
  justification: "Module critique d'authentification"
  offline_allowed: false
  degradation_allowed: true
```

**Gouvernance appliquée :**

| Core | Adaptation |
|------|------------|
| StrongFather | Décisions strictes, vérifications croisées |
| MasterButler | Permissions critiques, vérification systématique |
| BorderGuard | Frontières strictes, classification renforcée |
| CaringNanny | Monitoring intensif, sondes actives |
| TAMR | Intervention requise en cas de doute |
| BondingBrother | Traçabilité absolue, signatures obligatoires |
| Kernel | Sondes fréquentes |

**Scénario : Anomalie détectée sur le module auth**

```
Séquence temporelle:

1. T0 (Normal) — Fonctionnement nominal
   - Auth fonctionne normalement
   - Toutes les fonctionnalités actives

2. Signal reçu: "Pattern de login suspect détecté"
   - Source: CaringNanny
   - Sévérité: Moyenne

3. WorrySentinel corrèle:
   - Signal CaringNanny + logs StrongFather
   - Décision: Transition T0 → T1

4. T1 (Instable) — Surveillance renforcée
   - Log renforcé activé
   - Traçabilité étendue
   - Auth fonctionne avec traces supplémentaires

5. Signaux persistants après observation
   - WorrySentinel: Transition T1 → T2

6. T2 (Dégradé) — Restrictions activées
   - Nouvelles inscriptions désactivées
   - Login existant avec vérification renforcée
   - StrongFather: Décisions strictes

7. Anomalie résolue (faux positif identifié)
   - WorrySentinel: Transition T2 → T1 → T0

8. T0 (Normal) — Retour nominal
```

---

### 2.5 Exemple : Infrastructure critique (Niveau 4)

**Contexte :**
Un système de contrôle industriel fonctionne sur du hardware potentiellement non fiable dans un environnement isolé.

**Profil de sécurité :**

```
Product Security Profile:
  product_id: "industrial-control-system"
  required_level: 4
  justification: "Infrastructure critique, environnement hostile"
  offline_allowed: true
  degradation_allowed: true
```

**Gouvernance appliquée :**

| Core | Adaptation |
|------|------------|
| StrongFather | Décisions ultra-strictes, aucune tolérance |
| MasterButler | Permissions minimales, vérification constante |
| BorderGuard | Frontières maximales, isolement strict |
| CaringNanny | Monitoring continu, sondes très fréquentes |
| TAMR | Intervention humaine systématique |
| BondingBrother | Traçabilité absolue, signatures cryptographiques |
| Kernel | Sondes très fréquentes, attestations régulières |

**Contraintes extrêmes :**
- Contrôles continus obligatoires
- Attestations régulières
- Très peu de fonctionnalités actives
- Blocage progressif dès T1
- Aucune tolérance aux anomalies

**Principe appliqué :** *"On protège l'intégrité coûte que coûte."*

---

## 3. Exemples de transitions d'état

### 3.1 Exemple : Transition T0 → T1 (Détection d'anomalie)

**Scénario :**
Le système fonctionne normalement (T0). Une anomalie est détectée par le Kernel : incohérence dans les traces d'exécution.

**Flux de gouvernance :**

```
1. OBSERVATION
   Kernel → WorrySentinel
   Signal: IntegritySignal {
     source: CoreId::Kernel,
     signal_type: SignalType::TraceInconsistency,
     severity: Severity::Low,
     details: "Hash de trace incohérent"
   }

2. CORRELATION
   WorrySentinel analyse:
   - Signal unique
   - Sévérité faible
   - Pas de pattern connu
   - Première occurrence

3. EVALUATION
   WorrySentinel évalue:
   - Critères de transition T0→T1 remplis
   - Anomalie détectée mais non confirmée

4. DECLARATION
   WorrySentinel déclare:
   StateTransitionDeclaration {
     from: TrustState::Normal,
     to: TrustState::Unstable,
     justification: "Anomalie trace détectée, observation renforcée"
   }

5. GOUVERNANCE
   WorrySentinel impose aux cores:
   - CaringNanny: Monitoring renforcé
   - BondingBrother: Traçabilité étendue
   - Aucun blocage

6. TRACABILITE
   Trace complète:
   - Contexte: "Incohérence trace Kernel"
   - Règles: ["RÈGLE-TRANS-1: Détection anomalie"]
   - Justification: "Première anomalie, observation"
```

---

### 3.2 Exemple : Transition T2 → T3 (Aggravation)

**Scénario :**
Le système est en T2 (Dégradé) suite à des incohérences persistantes. De nouveaux signaux indiquent une aggravation.

**Signaux reçus :**

```
Signal 1 (StrongFather):
  - Type: Décisions refusées répétées
  - Pattern: Tentatives d'accès non autorisées

Signal 2 (BorderGuard):
  - Type: Violations de frontières
  - Pattern: Tentatives de contournement

Signal 3 (KindMother):
  - Type: Incohérences données
  - Pattern: Modifications non tracées
```

**Corrélation WorrySentinel :**

```
Analyse:
- 3 signaux de sources différentes
- Pattern coordonné suggérant intrusion
- Corrélation temporelle confirmée
- État actuel: T2 (Dégradé)

Évaluation:
- Critères T2→T3 remplis: aggravation confirmée
- Suspicion forte d'intégrité compromise

Décision:
- Transition T2 → T3
```

**Gouvernance T3 appliquée :**

| Core | Contrainte T3 |
|------|---------------|
| StrongFather | Décisions critiques → AMBIGUË / DIFFÉRÉE |
| MasterButler | Permissions minimales |
| BorderGuard | Isolement renforcé |
| CaringNanny | Monitoring maximal |
| TAMR | Override requis pour toute action sensible |
| Produits | Gel des produits non essentiels |

---

### 3.3 Exemple : Retour T3 → T2 (Amélioration)

**Scénario :**
Le système est en T3 (Restreint). L'analyse a confirmé un faux positif et les anomalies ont été résolues.

**Processus de retour :**

```
1. ANALYSE
   - TAMR confirme: faux positif
   - Anomalies résolues
   - Pas de compromission confirmée

2. INTERVENTION TAMR
   TamrDecision {
     decision_type: TamrDecisionType::ConfirmSecurity,
     justification: "Faux positif confirmé, anomalies résolues"
     authorized_by: "Admin-0042"
   }

3. ÉVALUATION WORRYSENTINEL
   - TAMR confirme sécurité
   - Signaux normaux depuis 24h
   - Critères T3→T2 remplis

4. DECLARATION
   StateTransitionDeclaration {
     from: TrustState::Restricted,
     to: TrustState::Degraded,
     justification: "Sécurité confirmée par TAMR, retour progressif"
   }

5. DEGRADATION INVERSE
   - Produits non essentiels: dégel progressif
   - Monitoring: réduction vers niveau T2
   - TAMR: intervention plus requise par défaut
```

**Note :** Le retour est progressif (T3→T2→T1→T0), jamais direct.

---

## 4. Exemples d'interaction inter-niveaux

### 4.1 Exemple : Accès médié (N1 → N3)

**Scénario :**
Un CMS éditorial (Niveau 1) doit vérifier les permissions d'un utilisateur auprès du module d'authentification (Niveau 3).

**Problème :**
- CMS Niveau 1 ne peut pas accéder directement au module auth Niveau 3
- Matrice d'accès : N1 → N3 = ❌

**Solution : Médiation**

```
1. DEMANDE
   CMS (N1) → WorrySentinel
   MediationRequest {
     source: "cms-editorial",
     source_level: 1,
     target: "auth-module",
     target_level: 3,
     operation: "verify_permissions"
   }

2. EVALUATION WORRYSENTINEL
   WorrySentinel vérifie:
   - Opération autorisée pour médiation
   - Contexte de sécurité acceptable
   - État de confiance compatible

3. VALIDATION STRONGFATHER
   StrongFather valide:
   - Politique de médiation respectée
   - Intention légitime

4. MEDIATION AUTORISEE
   MediationGrant {
     request_id: "med-0042",
     constraints: [
       "read_only",
       "specific_user_only",
       "trace_required"
     ],
     valid_for: "single_operation"
   }

5. OPERATION MEDIEE
   CMS → [Médiation] → Auth Module
   - Opération limitée aux contraintes
   - Traçabilité complète
   - Résultat retourné

6. FIN MEDIATION
   Médiation terminée, accès révoqué
```

---

### 4.2 Exemple : Blocage de Tool en T2

**Scénario :**
Un produit tente d'appeler un Tool UI pendant que le système est en T2 (Dégradé).

**Contexte :**
- Produit : Application de gestion (Niveau 2)
- Tool : UI Toolkit (Niveau 0)
- État système : T2 (Dégradé)

**Vérification WorrySentinel :**

```
Requête Tool:
  product: "app-gestion"
  product_level: 2
  tool: "ui-toolkit"
  tool_level: 0
  operation: "render_dashboard"

Vérification:
  - État confiance: T2 (Dégradé)
  - Règle RÈGLE-TOOL-SEC-3: "En T2+, certains Tools peuvent être bloqués"
  - Tool Niveau 0 en état T2: Potentiellement bloqué

Évaluation:
  - Tool UI non essentiel
  - État T2 requiert restrictions
  - Dashboard non critique

Résultat:
  ToolAccessDecision::Blocked {
    reason: "Tool non essentiel bloqué en état T2",
    alternative: "Utiliser mode texte minimal"
  }
```

**Message retourné :**

```
UI Toolkit indisponible car environnement en état SECURITY_DEGRADED (T2)
Alternative disponible: mode texte minimal
```

---

## 5. Exemples de cumul Niveau × État

### 5.1 Exemple : Produit N2 en T0 vs T2

**Produit :** Application de gestion de profils (Niveau 2)

**En T0 (Normal) :**

```
Comportement:
  - Fonctionnement normal
  - Toutes les fonctionnalités actives
  - Auth renforcée (N2)
  - Traçabilité complète (N2)

Contraintes N2 appliquées:
  - StrongFather: Décisions renforcées
  - MasterButler: Permissions détaillées
  - CaringNanny: Monitoring actif
```

**En T2 (Dégradé) :**

```
Comportement:
  - Fonctionnalités réduites
  - Restrictions modérées

Contraintes N2 + T2 cumulées:
  - StrongFather: Décisions strictes (N2) + plus strictes (T2)
  - MasterButler: Permissions limitées
  - CaringNanny: Monitoring intensif
  - Certaines fonctions désactivées

Fonctions désactivées:
  - Export massif de profils
  - Modification de paramètres sensibles
  - Intégrations tierces
```

**Principe :** Les restrictions sont cumulatives.

---

### 5.2 Exemple : Produit N4 en T1

**Produit :** Système de contrôle industriel (Niveau 4)

**En T0 (Normal) :**

```
Contraintes N4 (déjà strictes):
  - Contrôles continus
  - Attestations régulières
  - Aucune tolérance aux anomalies
  - Monitoring permanent
```

**En T1 (Instable) :**

```
Contraintes N4 + T1:
  - Tout ce qui précède PLUS:
  - Vérifications maximales supplémentaires
  - Log renforcé au maximum
  - Alerte immédiate TAMR
  - Préparation au gel

Différence avec N2 en T1:
  - N2 en T1: Surveillance renforcée, pas de blocage
  - N4 en T1: Surveillance maximale, préparation blocage imminent
```

**Note :** Un produit N4 réagit plus fortement à T1 qu'un produit N2.

---

## 6. Exemple de flux de gouvernance complet

### 6.1 Scénario : Jour type d'un système e-commerce

**Système :**
- Site vitrine (N0)
- Catalogue produits (N1)
- Panier et profils (N2)
- Module paiement (N3)
- État initial : T0

**8h00 — Démarrage nominal**

```
WorrySentinel:
  - État: T0 (Normal)
  - Tous les produits actifs
  - Contraintes par niveau appliquées
```

**10h30 — Pic de trafic**

```
Signaux:
  - LogisticsSteward: Consommation ressources élevée
  - CaringNanny: Patterns normaux malgré charge

Évaluation WorrySentinel:
  - Charge élevée mais comportement normal
  - Pas de transition d'état

Résultat:
  - État maintenu: T0
  - Surveillance continue
```

**14h15 — Anomalie détectée**

```
Signal:
  - StrongFather: Pattern de décisions inhabituelles
  - Source: Module paiement (N3)

Évaluation WorrySentinel:
  - Anomalie sur composant critique (N3)
  - Première occurrence
  - Transition: T0 → T1

Gouvernance T1:
  - Log renforcé (tous produits)
  - Surveillance accrue module paiement
  - Pas de blocage
```

**14h45 — Anomalie persiste**

```
Signaux:
  - StrongFather: Pattern persiste
  - BorderGuard: Tentatives d'accès inhabituelles

Corrélation WorrySentinel:
  - Anomalie persistante
  - Corrélation StrongFather + BorderGuard
  - Transition: T1 → T2

Gouvernance T2:
  - Module paiement: Transactions limitées
  - Nouvelles inscriptions: Désactivées
  - Panier: Mode lecture seule
  - Site vitrine: Normal (N0)
  - Catalogue: Normal (N1)
```

**15h30 — Analyse confirme faux positif**

```
Intervention:
  - TAMR confirme: Bug logiciel, pas intrusion
  - Correction déployée

Évaluation WorrySentinel:
  - Anomalie résolue
  - TAMR confirme sécurité
  - Transition: T2 → T1

Gouvernance T1:
  - Retour progressif des fonctionnalités
  - Surveillance maintenue
```

**16h00 — Retour nominal**

```
Observation:
  - Aucune anomalie depuis 30 min
  - Signaux tous normaux

Évaluation WorrySentinel:
  - Critères T1 → T0 remplis
  - Transition: T1 → T0

Gouvernance T0:
  - Fonctionnement normal
  - Toutes fonctionnalités restaurées
```

---

## 7. Résumé des exemples

| Exemple | Type | Niveau | État | Point illustré |
|---------|------|--------|------|----------------|
| Site vitrine | Produit | 0 | T0-T4 | Niveau minimal |
| CMS éditorial | Produit | 1 | T0-T4 | Niveau standard |
| Profils utilisateurs | Produit | 2 | T0-T2 | Données sensibles |
| Module auth | Produit | 3 | T0-T3 | Composant critique |
| Contrôle industriel | Produit | 4 | T0-T1 | Sécurité maximale |
| T0→T1 | Transition | - | T0→T1 | Détection anomalie |
| T2→T3 | Transition | - | T2→T3 | Aggravation |
| T3→T2 | Transition | - | T3→T2 | Retour progressif |
| N1→N3 | Inter-niveaux | 1→3 | - | Médiation |
| Tool bloqué | Tools | 0/2 | T2 | Blocage en dégradé |
| N2 T0 vs T2 | Cumul | 2 | T0/T2 | Restrictions cumulées |
| E-commerce | Complet | 0-3 | T0-T2 | Journée type |

---

## 8. Documents associés

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Concepts illustrés |
| [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md) | Niveaux de sécurité |
| [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md) | États de confiance |
| [Progressive Degradation Contract](../contracts/degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md) | Dégradation progressive |
| [Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md) | Flux de gouvernance |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Référence — Exemples illustratifs  
**Type :** Exemples et cas d'usage
