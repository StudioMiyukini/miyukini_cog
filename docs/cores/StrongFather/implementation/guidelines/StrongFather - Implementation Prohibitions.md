# StrongFather â€” Implementation Prohibitions

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document prÃ©sente les patterns **strictement interdits** pour StrongFather. Il complÃ¨te le document [Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md).

**Documents connexes :**
- [StrongFather - Implementation Overview](./StrongFather%20-%20Implementation%20Overview.md)
- [StrongFather - Implementation Patterns](./StrongFather%20-%20Implementation%20Patterns.md)

**Terminologie :** Voir [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Patterns strictement interdits

### 1.1. Cache dÃ©cisionnel (INTERD-PERS-3, INV-EXEC-3)

**Violation contractuelle :**

Un cache dÃ©cisionnel viole INTERD-PERS-3 (Ã©criture en cache) et INV-EXEC-3 (aucune persistance).

**Exemple d'implÃ©mentation invalide :**

```rust
// âŒ INCORRECT : Cache dÃ©cisionnel
pub struct StrongFather {
    decision_cache: HashMap<String, Decision>, // âŒ Cache = persistance interdite
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        // âŒ VÃ©rification de cache = persistance opÃ©rationnelle
        if let Some(cached) = self.decision_cache.get(&intent.intent_id) {
            return cached.clone(); // âŒ RÃ©utilisation de dÃ©cision = effet de bord
        }
        
        let decision = self.evaluate(intent);
        self.decision_cache.insert(intent.intent_id.clone(), decision.clone()); // âŒ Ã‰criture en cache
        decision
    }
}
```

**Pourquoi c'est interdit :**

- **Persistance opÃ©rationnelle :** Un cache est une forme de persistance qui affecte le comportement (INTERD-PERS-3).

- **Effet de bord :** Le cache modifie l'Ã©tat entre les Ã©valuations (INV-EXEC-2, INV-EXEC-3).

- **Non-dÃ©terminisme :** Un cache peut produire des rÃ©sultats diffÃ©rents selon l'historique (INV-POL-3).

**RÃ©fÃ©rence contrat :** Execution Prohibition Contract (INTERD-PERS-3), Invariants & Guarantees (INV-EXEC-2, INV-EXEC-3, INV-POL-3)

---

### 1.2. Ordonnancement ou planification (INTERD-TIME-1, INTERD-TIME-2, INV-DIFF-NOPLAN)

**Violation contractuelle :**

L'ordonnancement viole INTERD-TIME-1, la planification viole INTERD-TIME-2, et une dÃ©cision DIFFÃ‰RÃ‰E ne doit pas impliquer de planification (INV-DIFF-NOPLAN).

**Exemple d'implÃ©mentation invalide :**

```rust
// âŒ INCORRECT : Ordonnancement
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // âŒ KERN-INTERD-1 : Clock pour logique dÃ©cisionnelle
        
        // âŒ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tÃ´t, rÃ©essayer Ã  9h");
        }
        
        // âŒ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.schedule_revaluation(intent, now + Duration::hours(1)); // âŒ Planification interdite
            return Decision::deferred("Contexte futur requis");
        }
    }
    
    // âŒ INCORRECT : Planification pour dÃ©cision diffÃ©rÃ©e
    fn schedule_revaluation(&self, intent: &Intent, when: DateTime) {
        // âŒ INV-DIFF-NOPLAN : Pas de planification
        self.scheduler.schedule(intent, when); // âŒ Ordonnancement interdit
    }
}
```

**Pourquoi c'est interdit :**

- **Logique temporelle technique :** L'ordonnancement utilise le temps technique pour influencer les dÃ©cisions (INTERD-TIME-1, KERN-INTERD-1).

- **Planification interdite :** StrongFather ne planifie jamais d'exÃ©cutions futures (INTERD-TIME-2, INV-DIFF-NOPLAN).

- **ResponsabilitÃ© adaptateur :** Seul l'adaptateur dÃ©cide quand re-soumettre une intention diffÃ©rÃ©e (INV-DIFF-NOPLAN).

**RÃ©fÃ©rence contrat :** Execution Prohibition Contract (INTERD-TIME-1, INTERD-TIME-2), Boundary & Isolation Contract (KERN-INTERD-1), Invariants & Guarantees (INV-DIFF-NOPLAN)

---

### 1.3. Appel Ã  KindMother (INTERD-KM-1, INTERD-KM-2, INTERD-KM-3)

**Violation contractuelle :**

Tout appel Ã  KindMother viole INTERD-KM-1, INTERD-KM-2, INTERD-KM-3.

**Exemple d'implÃ©mentation invalide :**

```rust
// âŒ INCORRECT : Appel Ã  KindMother
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // âŒ INTERD-KM-2 : Lecture de donnÃ©es gÃ©rÃ©es par KindMother
        let existing_entity = self.kindmother.read_entity(&intent.subject)?;
        
        // âŒ INTERD-KM-1 : Appel Ã  KindMother
        if existing_entity.is_some() {
            return Decision::refused("EntitÃ© existe dÃ©jÃ ");
        }
        
        // âŒ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // âŒ Persistance interdite
        }
    }
}
```

**Pourquoi c'est interdit :**

- **IndÃ©pendance absolue :** StrongFather et KindMother sont totalement indÃ©pendants (INTERD-KM-4).

- **SÃ©paration des responsabilitÃ©s :** StrongFather dÃ©cide, KindMother persiste. Aucune communication directe (Boundary & Isolation Contract section 4.1).

- **Isolation garantie :** L'isolation garantit la puretÃ© fonctionnelle de StrongFather (INV-BOUND-5).

**RÃ©fÃ©rence contrat :** Boundary & Isolation Contract (section 4.1, INTERD-KM-*), Invariants & Guarantees (INV-BOUND-2, INV-BOUND-5)

---

### 1.4. Logique mÃ©tier spÃ©cifique (Execution Prohibition Contract)

**Violation contractuelle :**

La logique mÃ©tier spÃ©cifique viole l'interdiction d'exÃ©cution et la sÃ©paration des responsabilitÃ©s.

**Exemple d'implÃ©mentation invalide :**

```rust
// âŒ INCORRECT : Logique mÃ©tier spÃ©cifique
impl PolicyEngine {
    pub fn evaluate_permission(&self, policy: &Policy, intent: &Intent) -> PolicyResult {
        // âŒ Logique mÃ©tier spÃ©cifique (exemple : e-commerce)
        if intent.subject.starts_with("order_") {
            let order = self.parse_order(&intent.data)?; // âŒ Parsing mÃ©tier
            if order.total > 1000.0 {
                return PolicyResult::denied("Montant trop Ã©levÃ©"); // âŒ RÃ¨gle mÃ©tier
            }
        }
        
        // âŒ Validation technique
        if !self.validate_email(&intent.data.email) { // âŒ Validation technique interdite
            return PolicyResult::denied("Email invalide");
        }
    }
}
```

**Pourquoi c'est interdit :**

- **SÃ©paration des responsabilitÃ©s :** StrongFather Ã©value selon des politiques gÃ©nÃ©rales, pas des rÃ¨gles mÃ©tier spÃ©cifiques (Execution Prohibition Contract section 3.5).

- **RÃ©utilisabilitÃ© :** Les politiques doivent Ãªtre gÃ©nÃ©rales et rÃ©utilisables (Policy Engine Contract section 2.3).

- **Pas de validation technique :** StrongFather ne valide jamais la structure technique des donnÃ©es (Core Decision Contract section 2.3).

**RÃ©fÃ©rence contrat :** Execution Prohibition Contract (section 3.5), Policy Engine Contract (section 2.3), Core Decision Contract (section 2.3)

---

### 1.5. Mutation d'Ã©tat systÃ¨me (INTERD-STATE-*, INV-EXEC-2)

**Violation contractuelle :**

Toute mutation d'Ã©tat systÃ¨me viole INTERD-STATE-* et INV-EXEC-2.

**Exemple d'implÃ©mentation invalide :**

```rust
// âŒ INCORRECT : Mutation d'Ã©tat
pub struct StrongFather {
    evaluation_count: usize,        // âŒ Ã‰tat systÃ¨me
    last_evaluated_intent: Option<Intent>, // âŒ Ã‰tat systÃ¨me
    user_preferences: HashMap<String, String>, // âŒ Ã‰tat utilisateur
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        self.evaluation_count += 1; // âŒ INTERD-STATE-1 : Ã‰tat systÃ¨me
        self.last_evaluated_intent = Some(intent.clone()); // âŒ INTERD-STATE-1
        
        // âŒ INTERD-STATE-2 : Ã‰tat utilisateur
        self.user_preferences.insert("last_action".to_string(), intent.action_type.to_string());
        
        self.evaluate(intent)
    }
}
```

**Pourquoi c'est interdit :**

- **PuretÃ© fonctionnelle :** StrongFather ne modifie jamais d'Ã©tat (INV-EXEC-2, INV-BEHAV-1).

- **Isolation :** L'isolation garantit qu'aucun Ã©tat externe n'est modifiÃ© (INV-BOUND-5).

- **RÃ©versibilitÃ© :** Les Ã©valuations doivent Ãªtre rÃ©versibles conceptuellement (G-EXEC-3).

**RÃ©fÃ©rence contrat :** Execution Prohibition Contract (INTERD-STATE-*, INV-EXEC-2), Invariants & Guarantees (INV-BEHAV-1, INV-BOUND-5)

---

## 2. ImplÃ©mentations INVALIDES (exemples complets)

### 2.1. StrongFather avec cache

```rust
// âŒ IMPLÃ‰MENTATION INVALIDE : Cache dÃ©cisionnel
pub struct StrongFather {
    cache: HashMap<String, Decision>, // âŒ INTERD-PERS-3, INV-EXEC-3
}

impl StrongFather {
    pub fn evaluate_intent(&mut self, intent: &Intent) -> Decision {
        if let Some(cached) = self.cache.get(&intent.intent_id) {
            return cached.clone(); // âŒ RÃ©utilisation = effet de bord
        }
        let decision = self.evaluate(intent);
        self.cache.insert(intent.intent_id.clone(), decision.clone()); // âŒ Ã‰criture en cache
        decision
    }
}
```

**Violations :**
- INTERD-PERS-3 : Ã‰criture en cache
- INV-EXEC-3 : Aucune persistance
- INV-EXEC-2 : Modification d'Ã©tat
- INV-POL-3 : Non-dÃ©terminisme potentiel

---

### 2.2. StrongFather avec appel Ã  KindMother

```rust
// âŒ IMPLÃ‰MENTATION INVALIDE : Appel Ã  KindMother
pub struct StrongFather {
    kindmother: KindMotherClient, // âŒ INTERD-KM-1, INTERD-KM-4
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        // âŒ INTERD-KM-2 : Lecture de donnÃ©es gÃ©rÃ©es par KindMother
        let existing = self.kindmother.read_entity(&intent.subject)?;
        
        if existing.is_some() {
            return Decision::refused("Existe dÃ©jÃ "); // âŒ DÃ©pendance Ã  KindMother
        }
        
        // âŒ INTERD-KM-3 : Demande de persistance
        if intent.action_type == ActionType::Creation {
            self.kindmother.persist(intent)?; // âŒ Persistance interdite
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-KM-1 : Appel Ã  KindMother
- INTERD-KM-2 : Lecture de donnÃ©es KindMother
- INTERD-KM-3 : Demande de persistance
- INTERD-KM-4 : Connaissance de KindMother
- INV-BOUND-2 : IndÃ©pendance KindMother

---

### 2.3. StrongFather avec ordonnancement

```rust
// âŒ IMPLÃ‰MENTATION INVALIDE : Ordonnancement
pub struct StrongFather {
    scheduler: Scheduler, // âŒ INTERD-TIME-1, INTERD-TIME-2
    clock: Clock,
}

impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Decision {
        let now = self.clock.now(); // âŒ KERN-INTERD-1 : Clock pour logique dÃ©cisionnelle
        
        // âŒ INTERD-TIME-1 : Ordonnancement
        if now.hour() < 9 {
            return Decision::deferred("Trop tÃ´t");
        }
        
        // âŒ INTERD-TIME-2 : Planification
        if intent.requires_future_context {
            self.scheduler.schedule(intent, now + Duration::hours(1)); // âŒ Planification
            return Decision::deferred("Contexte futur requis");
        }
        
        Decision::accepted()
    }
}
```

**Violations :**
- INTERD-TIME-1 : Ordonnancement
- INTERD-TIME-2 : Planification
- KERN-INTERD-1 : Clock pour logique dÃ©cisionnelle
- INV-DIFF-NOPLAN : DÃ©cision diffÃ©rÃ©e sans planification

---

### 2.4. StrongFather avec callback exÃ©cutable

```rust
// âŒ IMPLÃ‰MENTATION INVALIDE : Callback exÃ©cutable
#[derive(Debug, Clone)]
pub struct Decision {
    pub intent_id: String,
    pub decision_type: DecisionType,
    pub execute: Box<dyn Fn() -> ()>, // âŒ Callback exÃ©cutable
}

impl Decision {
    pub fn call(&self) {
        (self.execute)(); // âŒ ExÃ©cution interdite
    }
}
```

**Violations :**
- INV-EXEC-1 : Aucune exÃ©cution
- G-NOEXEC-1 : Aucune exÃ©cution
- Core Decision Contract section 2.3 : DÃ©cision non-exÃ©cutable

---

### 2.5. StrongFather avec mÃ©lange erreur/rejet

```rust
// âŒ IMPLÃ‰MENTATION INVALIDE : MÃ©lange erreur/rejet
impl StrongFather {
    pub fn evaluate_intent(&self, intent: &Intent) -> Result<Decision, SFError> {
        // âŒ INCORRECT : Retourner une erreur pour un rejet structurel
        if intent.intent_id.is_empty() {
            return Err(SFError::StructuralError {
                reason: "Identifiant vide".to_string(),
                location: "IntentValidator".to_string(),
            }); // âŒ Rejet structurel â‰  Erreur
        }
        
        // âŒ INCORRECT : Retourner un rejet pour une erreur
        if self.policy_engine.is_corrupted() {
            return Ok(Decision {
                decision_type: DecisionType::Refused {
                    reason: RefusalReason::InternalError, // âŒ Erreur â‰  Rejet
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

## 3. PiÃ¨ges classiques et erreurs frÃ©quentes

### 3.1. PiÃ¨ge : Cache "pour performance"

**Erreur frÃ©quente :**

ImplÃ©menter un cache pour amÃ©liorer les performances, violant ainsi INTERD-PERS-3 et INV-EXEC-3.

**Pourquoi c'est un piÃ¨ge :**

- **Violation contractuelle :** Un cache est une forme de persistance opÃ©rationnelle (INTERD-PERS-3).

- **Non-dÃ©terminisme :** Un cache peut produire des rÃ©sultats diffÃ©rents selon l'historique (INV-POL-3).

- **Effet de bord :** Un cache modifie l'Ã©tat entre les Ã©valuations (INV-EXEC-2).

**Solution :**

- **Pas de cache :** Accepter que chaque Ã©valuation soit indÃ©pendante.

- **Optimisation autorisÃ©e :** Optimiser l'algorithme d'Ã©valuation, pas ajouter un cache.

---

### 3.2. PiÃ¨ge : Utilisation de Clock pour logique dÃ©cisionnelle

**Erreur frÃ©quente :**

Utiliser `Clock` pour dÃ©terminer si une intention est valide selon l'heure, violant KERN-INTERD-1.

**Pourquoi c'est un piÃ¨ge :**

- **Violation contractuelle :** Clock est autorisÃ© uniquement pour l'horodatage de traces (KERN-AUTH-3, KERN-INTERD-1).

- **Logique temporelle interdite :** StrongFather ne gÃ¨re jamais le temps technique (INTERD-TIME-*).

**Solution :**

- **Clock uniquement pour traces :** Utiliser Clock uniquement aprÃ¨s production de dÃ©cision pour horodater la trace.

- **Pas de logique temporelle :** Ne jamais utiliser Clock pour influencer une Ã©valuation.

---

### 3.3. PiÃ¨ge : MÃ©langer erreur et rejet

**Erreur frÃ©quente :**

Retourner une erreur (`Err(SFError)`) pour un rejet structurel, violant INV-ERR-1.

**Pourquoi c'est un piÃ¨ge :**

- **Violation contractuelle :** Un rejet est un rÃ©sultat normal, pas un dysfonctionnement (INV-ERR-1).

- **Confusion sÃ©mantique :** Erreur = interne, Rejet = externe (Error & Rejection Model section 2).

**Solution :**

- **Rejet = Decision :** Un rejet structurel produit une `Decision` avec `DecisionType::Refused`.

- **Erreur = SFError :** Un dysfonctionnement interne retourne `Err(SFError)`.

---

### 3.4. PiÃ¨ge : Planification pour dÃ©cision DIFFÃ‰RÃ‰E

**Erreur frÃ©quente :**

ImplÃ©menter un scheduler pour les dÃ©cisions DIFFÃ‰RÃ‰ES, violant INV-DIFF-NOPLAN et INTERD-TIME-2.

**Pourquoi c'est un piÃ¨ge :**

- **Violation contractuelle :** Une dÃ©cision DIFFÃ‰RÃ‰E n'implique aucune planification (INV-DIFF-NOPLAN).

- **Ordonnancement interdit :** StrongFather ne planifie jamais d'exÃ©cutions futures (INTERD-TIME-2).

**Solution :**

- **Pas de scheduler :** Ne jamais implÃ©menter de scheduler ou de planification.

- **DÃ©cision pure :** Une dÃ©cision DIFFÃ‰RÃ‰E indique uniquement que le contexte futur est requis.

---

### 3.5. PiÃ¨ge : Logique mÃ©tier dans les politiques

**Erreur frÃ©quente :**

ImplÃ©menter de la logique mÃ©tier spÃ©cifique dans les politiques, violant Policy Engine Contract section 2.3.

**Pourquoi c'est un piÃ¨ge :**

- **Violation contractuelle :** Les politiques ne contiennent jamais de logique mÃ©tier spÃ©cifique (Policy Engine Contract section 2.3).

- **RÃ©utilisabilitÃ© :** Les politiques doivent Ãªtre gÃ©nÃ©rales et rÃ©utilisables.

**Solution :**

- **Politiques gÃ©nÃ©rales :** Les politiques expriment des rÃ¨gles gÃ©nÃ©rales (permission, contrainte, prioritÃ©).

- **Pas de parsing mÃ©tier :** Ne jamais parser des structures mÃ©tier spÃ©cifiques dans les politiques.

---

## 4. Erreurs d'interprÃ©tation corrigÃ©es

### E1 : Cache "pour performance"

**Erreur d'interprÃ©tation :** Un dÃ©veloppeur pourrait penser qu'un cache en mÃ©moire est acceptable car "ce n'est pas de la persistance sur disque".

**Correction :** Clarification que toute forme de persistance opÃ©rationnelle (cache, Ã©tat mutable) est interdite, mÃªme en mÃ©moire. RÃ©fÃ©rence : INTERD-PERS-3, INV-EXEC-3.

---

### E2 : Clock pour "validation temporelle"

**Erreur d'interprÃ©tation :** Un dÃ©veloppeur pourrait penser qu'utiliser Clock pour valider si une intention est "trop ancienne" est acceptable.

**Correction :** Clarification que Clock est autorisÃ© uniquement pour l'horodatage de traces aprÃ¨s production de dÃ©cision, jamais pour la logique dÃ©cisionnelle. RÃ©fÃ©rence : KERN-AUTH-3, KERN-INTERD-1.

---

### E3 : Rejet = Erreur

**Erreur d'interprÃ©tation :** Un dÃ©veloppeur pourrait penser qu'un rejet structurel doit retourner une erreur (`Err(SFError)`).

**Correction :** Clarification que les rejets sont des rÃ©sultats normaux d'Ã©valuation (dÃ©cisions REFUSÃ‰ES), pas des dysfonctionnements. RÃ©fÃ©rence : Error & Rejection Model section 2, INV-ERR-1.

---

### E4 : Planification pour DIFFÃ‰RÃ‰E

**Erreur d'interprÃ©tation :** Un dÃ©veloppeur pourrait penser qu'une dÃ©cision DIFFÃ‰RÃ‰E doit Ãªtre "planifiÃ©e" pour rÃ©Ã©valuation automatique.

**Correction :** Clarification que INV-DIFF-NOPLAN interdit toute planification. Seul l'adaptateur dÃ©cide quand re-soumettre. RÃ©fÃ©rence : INV-DIFF-NOPLAN, INTERD-TIME-2.

---

## 5. AmbiguÃ¯tÃ©s clarifiÃ©es

### A1 : "PuretÃ© fonctionnelle" vs "Ã‰tat interne"

**AmbiguÃ¯tÃ© :** Un dÃ©veloppeur pourrait se demander si un Ã©tat interne (comme le Policy Engine avec ses politiques chargÃ©es) viole la puretÃ© fonctionnelle.

**Clarification :** La puretÃ© fonctionnelle concerne l'absence d'effet de bord sur le systÃ¨me externe. Un Ã©tat interne immuable (politiques chargÃ©es) est acceptable. Ce qui est interdit : mutation d'Ã©tat entre Ã©valuations, cache, compteurs, etc.

**RÃ©fÃ©rence :** INV-EXEC-5, INV-BEHAV-3, G-EXEC-1

---

### A2 : "TraÃ§abilitÃ©" vs "Persistance opÃ©rationnelle"

**AmbiguÃ¯tÃ© :** Un dÃ©veloppeur pourrait se demander si la traÃ§abilitÃ© (via Logger) viole l'interdiction de persistance.

**Clarification :** La traÃ§abilitÃ© est autorisÃ©e via le kernel (KERN-AUTH-2) car elle est passive et n'affecte pas le comportement. La persistance opÃ©rationnelle (cache, Ã©tat mutable) est interdite car elle affecte le comportement.

**RÃ©fÃ©rence :** Audit & Trace Contract, Boundary & Isolation Contract (KERN-AUTH-2), Execution Prohibition Contract (INTERD-PERS-*)

---

### A3 : "DÃ©terminisme" vs "Performance"

**AmbiguÃ¯tÃ© :** Un dÃ©veloppeur pourrait se demander si le dÃ©terminisme empÃªche toute optimisation.

**Clarification :** Le dÃ©terminisme (INV-POL-3) garantit que pour une entrÃ©e donnÃ©e, la sortie est toujours la mÃªme. Les optimisations d'algorithme sont autorisÃ©es tant qu'elles prÃ©servent le dÃ©terminisme. Ce qui est interdit : cache, Ã©tat mutable entre Ã©valuations, sources de non-dÃ©terminisme.

**RÃ©fÃ©rence :** INV-POL-3, Policy Engine Contract section 7

---

## 6. ConformitÃ© MSCM/MIP

### 6.1 Obligation de balisage MSCM

Tout code implÃ©mentÃ© pour StrongFather DOIT Ãªtre balisÃ© selon le protocole MSCM v1.

**RÃ©fÃ©rence :** [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**Obligations minimales :**
- Chaque bloc fonctionnel DOIT avoir un identifiant unique (`@id`)
- Le rÃ´le sÃ©mantique DOIT Ãªtre explicite (`@role`)
- La couche architecturale DOIT Ãªtre dÃ©clarÃ©e (`@layer`)
- Une description humaine DOIT accompagner chaque bloc (`@human`)

### 6.2 IntÃ©gration MIP

AprÃ¨s implÃ©mentation, l'index MIP DOIT Ãªtre rÃ©gÃ©nÃ©rÃ© pour :
- Valider l'intÃ©gritÃ© des blocs MSCM
- Mettre Ã  jour le graphe de dÃ©pendances
- VÃ©rifier la cohÃ©rence hiÃ©rarchique

### 6.3 Check-list MSCM

Avant toute livraison, vÃ©rifier :
- [ ] Tous les blocs critiques sont balisÃ©s MSCM
- [ ] Les identifiants sont uniques globalement
- [ ] Les couches (layer) sont cohÃ©rentes avec l'architecture
- [ ] L'index MIP peut Ãªtre rÃ©gÃ©nÃ©rÃ© sans erreur

---

**Conclusion :** Ce document guide l'implÃ©mentation de StrongFather en respectant strictement tous les contrats FONDATION v1.1. Toute interprÃ©tation qui contredit un contrat FONDATION est invalide. Les contrats FONDATION priment toujours sur ce guide.

---

**Document crÃ©Ã© le :** 2026-01-27  
**Version :** 1.1 (rÃ©organisation)  
**Statut :** POST-FONDATION / NON NORMATIF / INFORMATIF  
**RÃ©fÃ©rence :** StrongFather Contrats FONDATION v1.1 (gelÃ©s, non modifiables)  
**Type :** Guide d'implÃ©mentation non contractuel

