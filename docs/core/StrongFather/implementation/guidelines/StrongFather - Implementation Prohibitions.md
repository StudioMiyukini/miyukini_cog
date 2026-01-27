# StrongFather — Implementation Prohibitions

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document présente les patterns **strictement interdits** pour StrongFather. Il complète le document [Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md).

**Documents connexes :**
- [StrongFather - Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md)
- [StrongFather - Implementation Patterns](./StrongFather%20-%20Implementation%20Patterns.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 1. Patterns strictement interdits

### 1.1. Cache décisionnel (INTERD-PERS-3, INV-EXEC-3)

**Violation contractuelle :**

Un cache décisionnel viole INTERD-PERS-3 (écriture en cache) et INV-EXEC-3 (aucune persistance).

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Cache décisionnel
pub struct StrongFather {
    decision_cache: HashMap<String, Decision>, // ❌ Cache = persistance interdite
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        // ❌ Vérification de cache = persistance opérationnelle
        if let Some(cached) = self.decision_cache.get(&intent.intent_id) {
            return cached.clone(); // ❌ Réutilisation de décision = effet de bord
        }
        
        let decision = self.evaluate(intent);
        self.decision_cache.insert(intent.intent_id.clone(), decision.clone()); // ❌ Écriture en cache
        decision
    }
}
```

**Pourquoi c'est interdit :**

- **Persistance opérationnelle :** Un cache est une forme de persistance qui affecte le comportement (INTERD-PERS-3).

- **Effet de bord :** Le cache modifie l'état entre les évaluations (INV-EXEC-2, INV-EXEC-3).

- **Non-déterminisme :** Un cache peut produire des résultats différents selon l'historique (INV-POL-3).

**Référence contrat :** Execution Prohibition Contract (INTERD-PERS-3), Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-POL-3)

---

### 1.2. Ordonnancement ou planification (INTERD-TIME-1, INTERD-TIME-2, INV-DIFF-NOPLAN)

**Violation contractuelle :**

L'ordonnancement viole INTERD-TIME-1, la planification viole INTERD-TIME-2, et une décision DIFFÉRÉE ne doit pas impliquer de planification (INV-DIFF-NOPLAN).

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Ordonnancement
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // ❌ KERN-INTERD-1 : Clock pour logique décisionnelle
        
        // ❌ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tôt, réessayer à 9h");
        }
        
        // ❌ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.schedule_revaluation(intent, now + Duration::hours(1)); // ❌ Planification interdite
            return Decision::deferred("Contexte futur requis");
        }
    }
    
    // ❌ INCORRECT : Planification pour décision différée
    fn schedule_revaluation(&self, intent: &Intent, when: DateTime) {
        // ❌ INV-DIFF-NOPLAN : Pas de planification
        self.scheduler.schedule(intent, when); // ❌ Ordonnancement interdit
    }
}
```

**Pourquoi c'est interdit :**

- **Logique temporelle technique :** L'ordonnancement utilise le temps technique pour influencer les décisions (INTERD-TIME-1, KERN-INTERD-1).

- **Planification interdite :** StrongFather ne planifie jamais d'exécutions futures (INTERD-TIME-2, INV-DIFF-NOPLAN).

- **Responsabilité adaptateur :** Seul l'adaptateur décide quand re-soumettre une intention différée (INV-DIFF-NOPLAN).

**Référence contrat :** Execution Prohibition Contract (INTERD-TIME-1, INTERD-TIME-2), Boundary & Isolation Contract (KERN-INTERD-1), Invariants & Guarantees (INV-DIFF-NOPLAN)

---

### 1.3. Appel à KindMother (INTERD-KM-1, INTERD-KM-2, INTERD-KM-3)

**Violation contractuelle :**

Tout appel à KindMother viole INTERD-KM-1, INTERD-KM-2, INTERD-KM-3.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Appel à KindMother
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // ❌ INTERD-KM-2 : Lecture de données gérées par KindMother
        let existing_entity = self.kindmother.read_entity(&intent.subject)?;
        
        // ❌ INTERD-KM-1 : Appel à KindMother
        if existing_entity.is_some() {
            return Decision::refused("Entité existe déjà");
        }
        
        // ❌ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // ❌ Persistance interdite
        }
    }
}
```

**Pourquoi c'est interdit :**

- **Indépendance absolue :** StrongFather et KindMother sont totalement indépendants (INTERD-KM-4).

- **Séparation des responsabilités :** StrongFather décide, KindMother persiste. Aucune communication directe (Boundary & Isolation Contract section 4.1).

- **Isolation garantie :** L'isolation garantit la pureté fonctionnelle de StrongFather (INV-BOUND-5).

**Référence contrat :** Boundary & Isolation Contract (section 4.1, INTERD-KM-*), Invariants & Guarantees (INV-BOUND-2, INV-BOUND-5)

---

### 1.4. Logique métier spécifique (Execution Prohibition Contract)

**Violation contractuelle :**

La logique métier spécifique viole l'interdiction d'exécution et la séparation des responsabilités.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Logique métier spécifique
impl PolicyEngine {
    pub fn evaluate_permission(&self, policy: &Policy, intent: &Intent) -> PolicyResult {
        // ❌ Logique métier spécifique (exemple : e-commerce)
        if intent.subject.starts_with("order_") {
            let order = self.parse_order(&intent.data)?; // ❌ Parsing métier
            if order.total > 1000.0 {
                return PolicyResult::denied("Montant trop élevé"); // ❌ Règle métier
            }
        }
        
        // ❌ Validation technique
        if !self.validate_email(&intent.data.email) { // ❌ Validation technique interdite
            return PolicyResult::denied("Email invalide");
        }
    }
}
```

**Pourquoi c'est interdit :**

- **Séparation des responsabilités :** StrongFather évalue selon des politiques générales, pas des règles métier spécifiques (Execution Prohibition Contract section 3.5).

- **Réutilisabilité :** Les politiques doivent être générales et réutilisables (Policy Engine Contract section 2.3).

- **Pas de validation technique :** StrongFather ne valide jamais la structure technique des données (Core Decision Contract section 2.3).

**Référence contrat :** Execution Prohibition Contract (section 3.5), Policy Engine Contract (section 2.3), Core Decision Contract (section 2.3)

---

### 1.5. Mutation d'état système (INTERD-STATE-*, INV-EXEC-2)

**Violation contractuelle :**

Toute mutation d'état système viole INTERD-STATE-* et INV-EXEC-2.

**Exemple d'implémentation invalide :**

```rust
// ❌ INCORRECT : Mutation d'état
pub struct StrongFather {
    evaluation_count: usize,        // ❌ État système
    last_evaluated_intent: Option<Intent>, // ❌ État système
    user_preferences: HashMap<String, String>, // ❌ État utilisateur
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        self.evaluation_count += 1; // ❌ INTERD-STATE-1 : État système
        self.last_evaluated_intent = Some(intent.clone()); // ❌ INTERD-STATE-1
        
        // ❌ INTERD-STATE-2 : État utilisateur
        self.user_preferences.insert("last_action".to_string(), intent.action_type.to_string());
        
        self.evaluate(intent)
    }
}
```

**Pourquoi c'est interdit :**

- **Pureté fonctionnelle :** StrongFather ne modifie jamais d'état (INV-EXEC-2, INV-BEHAV-1).

- **Isolation :** L'isolation garantit qu'aucun état externe n'est modifié (INV-BOUND-5).

- **Réversibilité :** Les évaluations doivent être réversibles conceptuellement (G-EXEC-3).

**Référence contrat :** Execution Prohibition Contract (INTERD-STATE-*, INV-EXEC-2), Invariants & Guarantees (INV-BEHAV-1, INV-BOUND-5)

---

## 2. Implémentations INVALIDES (exemples complets)

### 2.1. StrongFather avec cache

```rust
// ❌ IMPLÉMENTATION INVALIDE : Cache décisionnel
pub struct StrongFather {
    cache: HashMap<String, Decision>, // ❌ INTERD-PERS-3, INV-EXEC-3
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        if let Some(cached) = self.cache.get(&intent.intent_id) {
            return cached.clone(); // ❌ Réutilisation = effet de bord
        }
        let decision = self.evaluate(intent);
        self.cache.insert(intent.intent_id.clone(), decision.clone()); // ❌ Écriture en cache
        decision
    }
}
```

**Violations :**
- INTERD-PERS-3 : Écriture en cache
- INV-EXEC-3 : Aucune persistance
- INV-EXEC-2 : Modification d'état
- INV-POL-3 : Non-déterminisme potentiel

---

### 2.2. StrongFather avec appel à KindMother

```rust
// ❌ IMPLÉMENTATION INVALIDE : Appel à KindMother
pub struct StrongFather {
    kindmother: KindMotherClient, // ❌ INTERD-KM-1, INTERD-KM-4
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // ❌ INTERD-KM-2 : Lecture de données gérées par KindMother
        let existing = self.kindmother.read_entity(&intent.subject)?;
        
        if existing.is_some() {
            return Decision::refused("Existe déjà"); // ❌ Dépendance à KindMother
        }
        
        // ❌ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // ❌ Persistance interdite
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-KM-1 : Appel à KindMother
- INTERD-KM-2 : Lecture de données KindMother
- INTERD-KM-3 : Demande de persistance
- INTERD-KM-4 : Connaissance de KindMother
- INV-BOUND-2 : Indépendance KindMother

---

### 2.3. StrongFather avec ordonnancement

```rust
// ❌ IMPLÉMENTATION INVALIDE : Ordonnancement
pub struct StrongFather {
    scheduler: Scheduler, // ❌ INTERD-TIME-1, INTERD-TIME-2
    clock: Clock,
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // ❌ KERN-INTERD-1 : Clock pour logique décisionnelle
        
        // ❌ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tôt");
        }
        
        // ❌ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.scheduler.schedule(intent, now + Duration::hours(1)); // ❌ Planification
            return Decision::deferred("Contexte futur requis");
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-TIME-1 : Ordonnancement
- INTERD-TIME-2 : Planification
- KERN-INTERD-1 : Clock pour logique décisionnelle
- INV-DIFF-NOPLAN : Décision différée sans planification

---

### 2.4. StrongFather avec callback exécutable

```rust
// ❌ IMPLÉMENTATION INVALIDE : Callback exécutable
#[derive(Debug, Clone)]
pub struct Decision {
    pub intent_id: String,
    pub decision_type: DecisionType,
    pub execute: Box<dyn Fn() -> ()>, // ❌ Callback exécutable
}

impl Decision {
    pub fn call(&self) {
        (self.execute)(); // ❌ Exécution interdite
    }
}
```

**Violations :**
- INV-EXEC-1 : Aucune exécution
- G-NOEXEC-1 : Aucune exécution
- Core Decision Contract section 2.3 : Décision non-exécutable

---

### 2.5. StrongFather avec mélange erreur/rejet

```rust
// ❌ IMPLÉMENTATION INVALIDE : Mélange erreur/rejet
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Result<Decision, SFError> {
        // ❌ INCORRECT : Retourner une erreur pour un rejet structurel
        if intent.intent_id.is_empty() {
            return Err(SFError::StructuralError {
                reason: "Identifiant vide".to_string(),
                location: "IntentValidator".to_string(),
            }); // ❌ Rejet structurel ≠ Erreur
        }
        
        // ❌ INCORRECT : Retourner un rejet pour une erreur
        if self.policy_engine.is_corrupted() {
            return Ok(Decision {
                decision_type: DecisionType::Refused {
                    reason: RefusalReason::InternalError, // ❌ Erreur ≠ Rejet
                },
                // ...
            });
        }
    }
}
```

**Violations :**
- INV-ERR-1 : Distinction erreur/rejet
- Error & Rejection Model section 2 : Distinction fondamentale

---

## 3. Pièges classiques et erreurs fréquentes

### 3.1. Piège : Cache "pour performance"

**Erreur fréquente :**

Implémenter un cache pour améliorer les performances, violant ainsi INTERD-PERS-3 et INV-EXEC-3.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Un cache est une forme de persistance opérationnelle (INTERD-PERS-3).

- **Non-déterminisme :** Un cache peut produire des résultats différents selon l'historique (INV-POL-3).

- **Effet de bord :** Un cache modifie l'état entre les évaluations (INV-EXEC-2).

**Solution :**

- **Pas de cache :** Accepter que chaque évaluation soit indépendante.

- **Optimisation autorisée :** Optimiser l'algorithme d'évaluation, pas ajouter un cache.

---

### 3.2. Piège : Utilisation de Clock pour logique décisionnelle

**Erreur fréquente :**

Utiliser `Clock` pour déterminer si une intention est valide selon l'heure, violant KERN-INTERD-1.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Clock est autorisé uniquement pour l'horodatage de traces (KERN-AUTH-3, KERN-INTERD-1).

- **Logique temporelle interdite :** StrongFather ne gère jamais le temps technique (INTERD-TIME-*).

**Solution :**

- **Clock uniquement pour traces :** Utiliser Clock uniquement après production de décision pour horodater la trace.

- **Pas de logique temporelle :** Ne jamais utiliser Clock pour influencer une évaluation.

---

### 3.3. Piège : Mélanger erreur et rejet

**Erreur fréquente :**

Retourner une erreur (`Err(SFError)`) pour un rejet structurel, violant INV-ERR-1.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Un rejet est un résultat normal, pas un dysfonctionnement (INV-ERR-1).

- **Confusion sémantique :** Erreur = interne, Rejet = externe (Error & Rejection Model section 2).

**Solution :**

- **Rejet = Decision :** Un rejet structurel produit une `Decision` avec `DecisionType::Refused`.

- **Erreur = SFError :** Un dysfonctionnement interne retourne `Err(SFError)`.

---

### 3.4. Piège : Planification pour décision DIFFÉRÉE

**Erreur fréquente :**

Implémenter un scheduler pour les décisions DIFFÉRÉES, violant INV-DIFF-NOPLAN et INTERD-TIME-2.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Une décision DIFFÉRÉE n'implique aucune planification (INV-DIFF-NOPLAN).

- **Ordonnancement interdit :** StrongFather ne planifie jamais d'exécutions futures (INTERD-TIME-2).

**Solution :**

- **Pas de scheduler :** Ne jamais implémenter de scheduler ou de planification.

- **Décision pure :** Une décision DIFFÉRÉE indique uniquement que le contexte futur est requis.

---

### 3.5. Piège : Logique métier dans les politiques

**Erreur fréquente :**

Implémenter de la logique métier spécifique dans les politiques, violant Policy Engine Contract section 2.3.

**Pourquoi c'est un piège :**

- **Violation contractuelle :** Les politiques ne contiennent jamais de logique métier spécifique (Policy Engine Contract section 2.3).

- **Réutilisabilité :** Les politiques doivent être générales et réutilisables.

**Solution :**

- **Politiques générales :** Les politiques expriment des règles générales (permission, contrainte, priorité).

- **Pas de parsing métier :** Ne jamais parser des structures métier spécifiques dans les politiques.

---

## 4. Erreurs d'interprétation corrigées

### E1 : Cache "pour performance"

**Erreur d'interprétation :** Un développeur pourrait penser qu'un cache en mémoire est acceptable car "ce n'est pas de la persistance sur disque".

**Correction :** Clarification que toute forme de persistance opérationnelle (cache, état mutable) est interdite, même en mémoire. Référence : INTERD-PERS-3, INV-EXEC-3.

---

### E2 : Clock pour "validation temporelle"

**Erreur d'interprétation :** Un développeur pourrait penser qu'utiliser Clock pour valider si une intention est "trop ancienne" est acceptable.

**Correction :** Clarification que Clock est autorisé uniquement pour l'horodatage de traces après production de décision, jamais pour la logique décisionnelle. Référence : KERN-AUTH-3, KERN-INTERD-1.

---

### E3 : Rejet = Erreur

**Erreur d'interprétation :** Un développeur pourrait penser qu'un rejet structurel doit retourner une erreur (`Err(SFError)`).

**Correction :** Clarification que les rejets sont des résultats normaux d'évaluation (décisions REFUSÉES), pas des dysfonctionnements. Référence : Error & Rejection Model section 2, INV-ERR-1.

---

### E4 : Planification pour DIFFÉRÉE

**Erreur d'interprétation :** Un développeur pourrait penser qu'une décision DIFFÉRÉE doit être "planifiée" pour réévaluation automatique.

**Correction :** Clarification que INV-DIFF-NOPLAN interdit toute planification. Seul l'adaptateur décide quand re-soumettre. Référence : INV-DIFF-NOPLAN, INTERD-TIME-2.

---

## 5. Ambiguïtés clarifiées

### A1 : "Pureté fonctionnelle" vs "État interne"

**Ambiguïté :** Un développeur pourrait se demander si un état interne (comme le Policy Engine avec ses politiques chargées) viole la pureté fonctionnelle.

**Clarification :** La pureté fonctionnelle concerne l'absence d'effet de bord sur le système externe. Un état interne immuable (politiques chargées) est acceptable. Ce qui est interdit : mutation d'état entre évaluations, cache, compteurs, etc.

**Référence :** INV-EXEC-5, INV-BEHAV-3, G-EXEC-1

---

### A2 : "Traçabilité" vs "Persistance opérationnelle"

**Ambiguïté :** Un développeur pourrait se demander si la traçabilité (via Logger) viole l'interdiction de persistance.

**Clarification :** La traçabilité est autorisée via le kernel (KERN-AUTH-2) car elle est passive et n'affecte pas le comportement. La persistance opérationnelle (cache, état mutable) est interdite car elle affecte le comportement.

**Référence :** Audit & Trace Contract, Boundary & Isolation Contract (KERN-AUTH-2), Execution Prohibition Contract (INTERD-PERS-*)

---

### A3 : "Déterminisme" vs "Performance"

**Ambiguïté :** Un développeur pourrait se demander si le déterminisme empêche toute optimisation.

**Clarification :** Le déterminisme (INV-POL-3) garantit que pour une entrée donnée, la sortie est toujours la même. Les optimisations d'algorithme sont autorisées tant qu'elles préservent le déterminisme. Ce qui est interdit : cache, état mutable entre évaluations, sources de non-déterminisme.

**Référence :** INV-POL-3, Policy Engine Contract section 7

---

**Conclusion :** Ce document guide l'implémentation de StrongFather en respectant strictement tous les contrats FONDATION v1.1. Toute interprétation qui contredit un contrat FONDATION est invalide. Les contrats FONDATION priment toujours sur ce guide.

---

**Document créé le :** 2026-01-27  
**Version :** 1.1 (réorganisation)  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**Référence :** StrongFather Contrats FONDATION v1.1 (gelés, non modifiables)  
**Type :** Guide d'implémentation non contractuel
