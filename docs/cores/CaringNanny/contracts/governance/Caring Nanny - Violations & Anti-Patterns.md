# Caring Nanny - Violations & Anti-Patterns

## 1. Contexte

Ce document catalogue les **violations** des invariants de Caring Nanny et les **anti-patterns** Ã  Ã©viter lors de l'implÃ©mentation ou de l'intÃ©gration. Il constitue un guide de conformitÃ© permettant d'identifier les Ã©carts par rapport aux contrats normatifs et de comprendre leurs consÃ©quences.

**RÃ©fÃ©rences normatives :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) â€” Invariants fondateurs (Section 7)
- [Caring Nanny - Invariants et Garanties](./Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) â€” Invariants dÃ©taillÃ©s
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) â€” LOI-1 Ã  LOI-6

## 2. PortÃ©e / Scope

Ce document couvre :
- Les violations des invariants fondamentaux (INV-CN-*)
- Les violations des invariants de non-action (INV-NEG-CN-*)
- Les violations des invariants de flux (INV-FLUX-CN-*)
- Les anti-patterns architecturaux, d'implÃ©mentation, et d'intÃ©gration
- Les mÃ©canismes de dÃ©tection et de prÃ©vention

Ce document **ne couvre pas** :
- Le modÃ¨le d'erreur opÃ©rationnel (voir Error & Rejection Model)
- Les dÃ©tails d'implÃ©mentation spÃ©cifiques
- Les stratÃ©gies de correction (Caring Nanny ne corrige jamais)

---

## 3. Taxonomie des violations

### 3.1 Niveaux de sÃ©vÃ©ritÃ©

| Niveau | Code | Description | ConsÃ©quence |
|--------|------|-------------|-------------|
| **Critique** | CRIT | Viole un invariant fondamental, compromet la nature de Caring Nanny | Rejet immÃ©diat, refonte requise |
| **Majeur** | MAJ | Viole une garantie ou un invariant de flux | Correction urgente avant mise en production |
| **Mineur** | MIN | DÃ©gradation de qualitÃ© sans violation d'invariant | Correction recommandÃ©e |
| **Avertissement** | WARN | Pratique dÃ©conseillÃ©e, risque potentiel | Revue et justification requises |

### 3.2 CatÃ©gories de violations

| CatÃ©gorie | Description |
|-----------|-------------|
| **VIO-NAT** | Violation de nature (ce que Caring Nanny EST) |
| **VIO-NEG** | Violation de non-action (ce que Caring Nanny NE FAIT JAMAIS) |
| **VIO-FLUX** | Violation de flux (comment l'information transite) |
| **VIO-GAR** | Violation de garantie envers les consommateurs ou autoritÃ©s |
| **VIO-LOI** | Violation des Lois d'Autonomie SystÃ¨me |

---

## 4. Violations des invariants fondamentaux (INV-CN-*)

### 4.1 VIO-CN-001 : Modification de donnÃ©es observÃ©es

**Invariant violÃ© :** INV-CN-1 (Observateur pur)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NAT

**Description :** Caring Nanny modifie des donnÃ©es dans le systÃ¨me qu'elle observe, que ce soit directement ou via un effet de bord.

**Exemples de violations :**
```
âŒ Mettre Ã  jour un flag "last_observed_at" dans les donnÃ©es mÃ©tier
âŒ Marquer une entitÃ© comme "checked" aprÃ¨s observation
âŒ CrÃ©er des entrÃ©es dans les tables mÃ©tier de KindMother
âŒ Modifier l'Ã©tat d'un composant pour "forcer" une cohÃ©rence
```

**ConsÃ©quences :**
- Corruption de la sÃ©paration des responsabilitÃ©s
- ImpossibilitÃ© de distinguer les modifications mÃ©tier des modifications d'observation
- Violation de l'autoritÃ© exclusive de KindMother sur les donnÃ©es

**PrÃ©vention :**
- Aucune mÃ©thode `write()`, `update()`, `delete()` sur les donnÃ©es mÃ©tier
- Audit automatisÃ© des appels API sortants
- Historique d'observation sÃ©parÃ© des donnÃ©es observÃ©es

---

### 4.2 VIO-CN-002 : CapacitÃ© d'exÃ©cution masquÃ©e

**Invariant violÃ© :** INV-CN-2 (Aucune capacitÃ© d'exÃ©cution)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NAT

**Description :** Caring Nanny dÃ©clenche une action, directement ou indirectement, en rÃ©ponse Ã  une observation.

**Exemples de violations :**
```
âŒ Appeler une fonction de redÃ©marrage suite Ã  la dÃ©tection d'une erreur
âŒ DÃ©clencher une synchronisation automatique en rÃ©ponse Ã  un Ã©tat "degraded"
âŒ Envoyer un email/notification directement (sans passer par BondingBrother)
âŒ Invalider un cache en rÃ©ponse Ã  une dÃ©tection d'incohÃ©rence
```

**ConsÃ©quences :**
- Caring Nanny devient un acteur, pas un observateur
- Effets de bord imprÃ©visibles
- Violation de la chaÃ®ne de responsabilitÃ© (dÃ©cision = StrongFather, exÃ©cution = composants dÃ©diÃ©s)

**PrÃ©vention :**
- Aucune mÃ©thode `execute()`, `trigger()`, `invoke()`, `send()` directe
- Propagation d'informations uniquement via BondingBrother
- Revue architecturale systÃ©matique

---

### 4.3 VIO-CN-003 : AutoritÃ© implicite

**Invariant violÃ© :** INV-CN-3 (Non-autoritaire)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NAT

**Description :** Caring Nanny prend des dÃ©cisions d'autorisation, de validation, ou de blocage basÃ©es sur l'Ã©tat observÃ©.

**Exemples de violations :**
```
âŒ Bloquer une opÃ©ration parce que l'Ã©tat est "degraded"
âŒ Refuser de rapporter un Ã©tat car il est "invalide"
âŒ Valider qu'une entitÃ© est "prÃªte" avant de permettre une action
âŒ DÃ©finir des seuils qui bloquent automatiquement certaines opÃ©rations
```

**ConsÃ©quences :**
- Caring Nanny devient une autoritÃ© de fait
- Court-circuitage de StrongFather dans la chaÃ®ne de dÃ©cision
- Comportement imprÃ©visible et non traÃ§able

**PrÃ©vention :**
- Aucune mÃ©thode `validate()`, `authorize()`, `approve()`, `reject()`
- L'Ã©tat est toujours informatif, jamais prescriptif
- Les dÃ©cisions basÃ©es sur l'Ã©tat sont prises par StrongFather

---

### 4.4 VIO-CN-004 : Ã‰tat incohÃ©rent rapportÃ©

**Invariant violÃ© :** INV-CN-4 (Ã‰tat cohÃ©rent)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-NAT

**Description :** Caring Nanny rapporte un Ã©tat contenant des contradictions (un composant simultanÃ©ment dans deux Ã©tats mutuellement exclusifs).

**Exemples de violations :**
```
âŒ Composant X rapportÃ© comme "healthy" et "error" simultanÃ©ment
âŒ Ã‰tat global "offline" avec sous-composants "syncing"
âŒ Transition de "healthy" vers "healthy" enregistrÃ©e
âŒ Deux Ã©tats diffÃ©rents retournÃ©s pour la mÃªme requÃªte Ã  des moments proches
```

**ConsÃ©quences :**
- Perte de confiance dans les informations d'Ã©tat
- DÃ©cisions incorrectes basÃ©es sur des Ã©tats contradictoires
- ImpossibilitÃ© de diagnostic fiable

**PrÃ©vention :**
- RÃ¨gles de cohÃ©rence explicites dans l'agrÃ©gateur d'Ã©tat
- Tests automatisÃ©s de cohÃ©rence sur toutes les rÃ©ponses
- PrioritÃ© des Ã©tats en cas de contradiction (error > degraded > syncing > offline > healthy)

---

### 4.5 VIO-CN-005 : Perte de traÃ§abilitÃ©

**Invariant violÃ© :** INV-CN-5 (TraÃ§abilitÃ© complÃ¨te)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** Une observation, transition, ou propagation n'est pas enregistrÃ©e dans l'historique.

**Exemples de violations :**
```
âŒ Transition d'Ã©tat sans entrÃ©e dans l'historique
âŒ Observation dÃ©tectÃ©e mais non journalisÃ©e
âŒ Propagation effectuÃ©e sans trace
âŒ Consultation d'Ã©tat non enregistrÃ©e
```

**ConsÃ©quences :**
- ImpossibilitÃ© d'audit a posteriori
- Diagnostic incomplet en cas de problÃ¨me
- Violation de LOI-3 (souverainetÃ© de l'Ã©tat local)

**PrÃ©vention :**
- Journalisation immÃ©diate avant traitement
- RÃ©conciliation pÃ©riodique observations/historique
- Alertes en cas de divergence

---

### 4.6 VIO-CN-006 : Observation bloquante

**Invariant violÃ© :** INV-CN-6 (Non-bloquant)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** L'observation de Caring Nanny bloque ou ralentit significativement les opÃ©rations du systÃ¨me.

**Exemples de violations :**
```
âŒ Consultation d'Ã©tat synchrone avec timeout Ã©levÃ©
âŒ Observation qui verrouille des ressources partagÃ©es
âŒ Buffer d'observations sans limite causant une saturation mÃ©moire
âŒ Propagation synchrone attendant une confirmation
```

**ConsÃ©quences :**
- DÃ©gradation des performances du systÃ¨me
- Latence perceptible pour les utilisateurs
- Potentiel blocage complet en cas de surcharge

**PrÃ©vention :**
- Consultations asynchrones ou Ã  faible latence
- Propagations non bloquantes (fire-and-forget)
- Monitoring des temps de rÃ©ponse
- Limites sur les buffers d'observations

---

### 4.7 VIO-CN-007 : Propagation altÃ©rÃ©e

**Invariant violÃ© :** INV-CN-7 (Propagation fidÃ¨le)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** L'information propagÃ©e diffÃ¨re de l'information observÃ©e (filtrage, transformation, interprÃ©tation).

**Exemples de violations :**
```
âŒ Filtrer certains Ã©tats avant propagation
âŒ Transformer "error" en "degraded" pour "simplifier"
âŒ Ajouter une interprÃ©tation Ã  l'Ã©tat propagÃ©
âŒ AgrÃ©ger plusieurs transitions en une seule notification
```

**ConsÃ©quences :**
- Perte d'information pour les destinataires
- DÃ©cisions basÃ©es sur des informations incomplÃ¨tes
- Divergence entre l'historique local et les notifications

**PrÃ©vention :**
- Comparaison automatisÃ©e observation/propagation
- Propagation de l'Ã©tat brut sans transformation
- InterprÃ©tation laissÃ©e aux destinataires

---

## 5. Violations des invariants de non-action (INV-NEG-CN-*)

### 5.1 VIO-NEG-001 : Ã‰criture dans la persistance mÃ©tier

**Invariant violÃ© :** INV-NEG-CN-01 (Jamais de modification de donnÃ©es)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny Ã©crit, modifie, ou supprime des donnÃ©es dans KindMother ou tout autre stockage mÃ©tier.

**Exemples de violations :**
```
âŒ INSERT dans une table mÃ©tier
âŒ UPDATE d'un champ mÃ©tier suite Ã  une observation
âŒ DELETE d'une entitÃ© considÃ©rÃ©e comme "obsolÃ¨te"
âŒ CrÃ©ation d'un WriteIntent depuis Caring Nanny
```

**ConsÃ©quences :**
- Violation de l'autoritÃ© de KindMother
- DonnÃ©es mÃ©tier polluÃ©es par des modifications non mÃ©tier
- TraÃ§abilitÃ© des modifications compromise

**PrÃ©vention :**
- Aucune dÃ©pendance vers les API d'Ã©criture de KindMother
- Audit des appels SQL/API
- Historique d'observation dans un stockage sÃ©parÃ©

---

### 5.2 VIO-NEG-002 : Logique dÃ©cisionnelle

**Invariant violÃ© :** INV-NEG-CN-02 (Jamais de dÃ©cision)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny contient une logique conditionnelle qui aboutit Ã  une action ou un comportement diffÃ©renciÃ© basÃ© sur des critÃ¨res mÃ©tier.

**Exemples de violations :**
```
âŒ if (state == "error") { activateBackupMode(); }
âŒ if (component.health < threshold) { notifyAdmin(); }
âŒ switch (state) { case "degraded": limitOperations(); }
âŒ DÃ©terminer dynamiquement si une opÃ©ration doit Ãªtre autorisÃ©e
```

**Ce qui est autorisÃ© :**
```
âœ… Classifier l'Ã©tat selon les catÃ©gories dÃ©finies
âœ… Appliquer des rÃ¨gles d'agrÃ©gation prÃ©dÃ©finies (error > degraded > ...)
âœ… DÃ©terminer les destinataires d'une propagation selon des rÃ¨gles Ã©tablies
```

**PrÃ©vention :**
- Revue de code ciblÃ©e sur les structures conditionnelles
- Les rÃ¨gles mÃ©tier sont externes Ã  Caring Nanny
- Actions dÃ©cidÃ©es par StrongFather, jamais par Caring Nanny

---

### 5.3 VIO-NEG-003 : Action corrective automatique

**Invariant violÃ© :** INV-NEG-CN-03 (Jamais d'action corrective)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny tente de corriger une anomalie dÃ©tectÃ©e au lieu de simplement la rapporter.

**Exemples de violations :**
```
âŒ RedÃ©marrer un service dÃ©faillant
âŒ RÃ©initialiser un composant bloquÃ©
âŒ Forcer une synchronisation pour rÃ©soudre une incohÃ©rence
âŒ Vider un cache pour rÃ©soudre un problÃ¨me de performance
âŒ Basculer vers un mode de secours
```

**Flux correct :**
```
Anomalie dÃ©tectÃ©e â†’ Observation enregistrÃ©e â†’ Propagation via BondingBrother
                    â†’ StrongFather dÃ©cide de l'action (ou pas)
                    â†’ Composant concernÃ© exÃ©cute l'action (ou pas)
```

**PrÃ©vention :**
- Aucune capacitÃ© d'action dans Caring Nanny
- RÃ´le limitÃ© Ã  : dÃ©tecter, classifier, propager, historiser
- Actions correctives implÃ©mentÃ©es ailleurs

---

### 5.4 VIO-NEG-004 : MÃ©diation d'intentions

**Invariant violÃ© :** INV-NEG-CN-04 (Jamais de mÃ©diation d'intentions)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny reÃ§oit, traduit, ou route des intentions de produits vers les autoritÃ©s.

**Exemples de violations :**
```
âŒ Exposer une API pour recevoir des demandes de produits
âŒ Traduire une demande utilisateur en action systÃ¨me
âŒ Router une intention vers KindMother ou StrongFather
âŒ Filtrer les rÃ©ponses des autoritÃ©s avant de les transmettre aux produits
```

**Distinction claire :**
| RÃ´le | Responsable | Caring Nanny |
|------|-------------|--------------|
| Recevoir des intentions | BondingBrother | âŒ Interdit |
| Observer des Ã©tats | Caring Nanny | âœ… AutorisÃ© |
| Propager des Ã©tats | Caring Nanny (via BondingBrother) | âœ… AutorisÃ© |

**PrÃ©vention :**
- Aucune interface d'intention exposÃ©e par Caring Nanny
- Toute mÃ©diation passe par BondingBrother

---

### 5.5 VIO-NEG-005 : DÃ©finition de rÃ¨gles dynamiques

**Invariant violÃ© :** INV-NEG-CN-05 (Jamais de dÃ©finition de rÃ¨gles)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny dÃ©finit ou modifie dynamiquement les rÃ¨gles de classification des Ã©tats ou de dÃ©tection des anomalies.

**Exemples de violations :**
```
âŒ Ajuster dynamiquement les seuils de dÃ©gradation
âŒ Apprendre de nouveaux patterns d'anomalie
âŒ Modifier les critÃ¨res de classification en fonction du contexte
âŒ CrÃ©er de nouvelles catÃ©gories d'Ã©tat Ã  la volÃ©e
```

**Ce qui est autorisÃ© :**
```
âœ… Charger des rÃ¨gles depuis une configuration externe
âœ… Appliquer des rÃ¨gles dÃ©finies par le produit
âœ… Classifier selon des critÃ¨res Ã©tablis au dÃ©ploiement
```

**PrÃ©vention :**
- RÃ¨gles chargÃ©es depuis une source externe (configuration)
- Aucune logique d'apprentissage ou d'adaptation dans Caring Nanny
- Les rÃ¨gles sont dÃ©finies par le produit ou l'Ã©cosystÃ¨me

---

### 5.6 VIO-NEG-006 : Persistance autonome externe

**Invariant violÃ© :** INV-NEG-CN-06 (Jamais de gestion de persistance)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-NEG

**Description :** Caring Nanny gÃ¨re directement la persistance de ses observations dans un systÃ¨me externe.

**Exemples de violations :**
```
âŒ Connexion directe Ã  une base de donnÃ©es externe pour persister l'historique
âŒ Gestion autonome de transactions de persistance
âŒ DÃ©finition de stratÃ©gies de rÃ©tention dans Caring Nanny
âŒ Ã‰criture vers un systÃ¨me de logs externe sans dÃ©lÃ©gation
```

**Ce qui est autorisÃ© :**
```
âœ… Maintenir un historique en mÃ©moire
âœ… DÃ©lÃ©guer la persistance Ã  KindMother si nÃ©cessaire
âœ… Exposer l'historique pour consultation
```

**PrÃ©vention :**
- Aucune connexion directe Ã  un systÃ¨me de persistance externe
- Si persistance nÃ©cessaire, dÃ©lÃ©gation Ã  KindMother via les canaux appropriÃ©s

---

## 6. Violations des invariants de flux (INV-FLUX-CN-*)

### 6.1 VIO-FLUX-001 : SÃ©quence d'observation incomplÃ¨te

**Invariant violÃ© :** INV-FLUX-CN-01 (SÃ©quence d'observation cohÃ©rente)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** Une observation ne suit pas la sÃ©quence dÃ©finie, avec des Ã©tapes sautÃ©es ou dans le dÃ©sordre.

**SÃ©quence obligatoire :**
1. DÃ©tection de condition
2. Ã‰valuation selon les critÃ¨res de classification
3. Traduction en Ã©tat partiel
4. AgrÃ©gation en Ã©tat global (si applicable)
5. DÃ©tection de transition (si changement)
6. Enregistrement dans l'historique

**Exemples de violations :**
```
âŒ Enregistrer une observation sans l'avoir classifiÃ©e
âŒ AgrÃ©ger sans avoir traduit en Ã©tat partiel
âŒ DÃ©tecter une transition sans enregistrer dans l'historique
âŒ Sauter l'Ã©valuation pour "optimiser"
```

**PrÃ©vention :**
- Chaque Ã©tape est tracÃ©e individuellement
- Alertes sur sÃ©quences incomplÃ¨tes
- Pipeline d'observation explicite et testable

---

### 6.2 VIO-FLUX-002 : SÃ©quence de propagation incomplÃ¨te

**Invariant violÃ© :** INV-FLUX-CN-02 (SÃ©quence de propagation cohÃ©rente)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** Une propagation ne suit pas la sÃ©quence dÃ©finie.

**SÃ©quence obligatoire :**
1. Identification des destinataires
2. Formulation du message (Ã©tat prÃ©cÃ©dent, Ã©tat actuel, cause)
3. DÃ©lÃ©gation Ã  BondingBrother
4. Enregistrement de la propagation

**Exemples de violations :**
```
âŒ Propager sans identifier les destinataires
âŒ Envoyer un message sans Ã©tat prÃ©cÃ©dent/actuel/cause
âŒ Contourner BondingBrother pour une propagation directe
âŒ Oublier d'enregistrer la propagation
```

**PrÃ©vention :**
- Validation du message avant propagation
- Aucun canal direct vers les destinataires
- Comparaison transitions/propagations

---

### 6.3 VIO-FLUX-003 : Perte d'observations

**Invariant violÃ© :** INV-FLUX-CN-03 (Pas de perte d'observation)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-FLUX

**Description :** Des observations sont perdues en raison de la charge, de conditions anormales, ou de dÃ©fauts d'implÃ©mentation.

**Exemples de violations :**
```
âŒ Buffer plein sans traitement des observations en attente
âŒ Exception non gÃ©rÃ©e qui fait perdre une observation
âŒ Prioritisation trop agressive qui supprime des observations non critiques
âŒ Crash pendant le traitement sans rÃ©cupÃ©ration
```

**PrÃ©vention :**
- Buffer d'observations avec limite et stratÃ©gie de dÃ©bordement
- Journalisation immÃ©diate avant traitement complet
- PrioritÃ© aux observations critiques (error > degraded > autres)
- RÃ©conciliation pÃ©riodique conditions dÃ©tectÃ©es/observations enregistrÃ©es

---

## 7. Violations des garanties (GAR-*)

### 7.1 VIO-GAR-001 : Ã‰tat indisponible

**Garantie violÃ©e :** GAR-CONS-01 (Ã‰tat toujours disponible)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-GAR

**Description :** Une demande d'Ã©tat n'obtient pas de rÃ©ponse.

**Exemples de violations :**
```
âŒ Timeout sans rÃ©ponse sur une consultation d'Ã©tat
âŒ Erreur non gÃ©rÃ©e retournant une exception au lieu d'un Ã©tat
âŒ Blocage infini sur une ressource
```

**Comportement attendu :**
- Toujours retourner une rÃ©ponse
- En cas d'incertitude, retourner "unknown" ou le dernier Ã©tat connu
- Inclure le timestamp de l'observation

---

### 7.2 VIO-GAR-002 : Notification non fiable

**Garantie violÃ©e :** GAR-CONS-04 (Notifications fiables)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-GAR

**Description :** Les notifications de changement d'Ã©tat sont manquantes, dupliquÃ©es, ou dÃ©sordonnÃ©es.

**Exemples de violations :**
```
âŒ Transition sans notification correspondante
âŒ MÃªme notification Ã©mise plusieurs fois
âŒ Notifications reÃ§ues dans un ordre diffÃ©rent de l'ordre des transitions
```

**PrÃ©vention :**
- Comparaison transitions enregistrÃ©es/notifications Ã©mises
- MÃ©canisme d'idempotence sur les notifications
- NumÃ©rotation sÃ©quentielle des notifications

---

### 7.3 VIO-GAR-003 : Contexte incomplet

**Garantie violÃ©e :** GAR-CONS-05 (Contexte complet)  
**SÃ©vÃ©ritÃ© :** MIN  
**CatÃ©gorie :** VIO-GAR

**Description :** Une rÃ©ponse d'Ã©tat ne contient pas toutes les informations de contexte requises.

**Informations requises :**
- Ã‰tat courant
- Timestamp de l'observation
- DurÃ©e dans l'Ã©tat actuel
- Cause de la derniÃ¨re transition (si disponible)

**Exemples de violations :**
```
âŒ Ã‰tat retournÃ© sans timestamp
âŒ DurÃ©e dans l'Ã©tat non calculÃ©e
âŒ Cause de transition omise
```

---

### 7.4 VIO-GAR-004 : Observation intrusive

**Garantie violÃ©e :** GAR-AUTH-01 (Observation non intrusive)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-GAR

**Description :** L'observation de Caring Nanny impacte les performances ou le fonctionnement des autoritÃ©s observÃ©es.

**Exemples de violations :**
```
âŒ RequÃªtes d'observation causant une charge significative sur KindMother
âŒ Verrouillage de ressources pendant l'observation
âŒ Polling agressif dÃ©gradant les performances
```

**ConformitÃ© LOI-2 :** L'observation intrusive peut empÃªcher le systÃ¨me de fonctionner normalement en isolation.

**PrÃ©vention :**
- Observation passive (Ã©vÃ©nements push plutÃ´t que polling)
- Aucun verrouillage de ressources
- Tests de charge avec et sans Caring Nanny

---

## 8. Violations des Lois d'Autonomie (VIO-LOI-*)

### 8.1 VIO-LOI-001 : DÃ©pendance externe critique

**Loi violÃ©e :** LOI-1 (Aucune dÃ©pendance externe critique Ã  l'exÃ©cution)  
**SÃ©vÃ©ritÃ© :** CRIT  
**CatÃ©gorie :** VIO-LOI

**Description :** Caring Nanny nÃ©cessite un appel externe pour fonctionner.

**Exemples de violations :**
```
âŒ Charger les rÃ¨gles de classification depuis un serveur distant
âŒ Valider l'Ã©tat via un service externe
âŒ Persister l'historique uniquement sur un cloud
âŒ Consulter une API externe pour classifier un Ã©tat
```

**PrÃ©vention :**
- RÃ¨gles de classification embarquÃ©es localement
- Historique local autonome
- Services externes optionnels, jamais obligatoires

---

### 8.2 VIO-LOI-002 : Isolement traitÃ© comme erreur

**Loi violÃ©e :** LOI-2 (Le systÃ¨me accepte l'isolement comme Ã©tat normal)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-LOI

**Description :** Caring Nanny traite l'Ã©tat "offline" comme une erreur plutÃ´t qu'un Ã©tat normal.

**Exemples de violations :**
```
âŒ Classifier "offline" avec la mÃªme sÃ©vÃ©ritÃ© que "error"
âŒ Retry infini pour reconnecter en mode offline
âŒ Alertes de type "erreur" pour l'Ã©tat isolÃ©
âŒ Blocage de l'observation en l'absence de connexion
```

**Distinction requise :**
| Ã‰tat | Nature | Traitement |
|------|--------|------------|
| offline | Normal | Information, pas d'alerte |
| error | Anormal | Alerte, investigation requise |

---

### 8.3 VIO-LOI-003 : Ã‰tat local non souverain

**Loi violÃ©e :** LOI-3 (L'Ã©tat local est souverain)  
**SÃ©vÃ©ritÃ© :** MAJ  
**CatÃ©gorie :** VIO-LOI

**Description :** Caring Nanny invalide ou ignore l'Ã©tat local au profit d'un Ã©tat distant.

**Exemples de violations :**
```
âŒ Ã‰craser l'historique local avec un historique distant
âŒ Ignorer les observations locales si elles contredisent un Ã©tat distant
âŒ Synchronisation qui efface des transitions locales
```

**PrÃ©vention :**
- L'historique local est la source de vÃ©ritÃ©
- La rÃ©conciliation est explicite et traÃ§able
- Les donnÃ©es locales ne sont jamais invalidÃ©es implicitement

---

### 8.4 VIO-LOI-004 : DÃ©pendance au temps global

**Loi violÃ©e :** LOI-4 (Pas de temps global requis)  
**SÃ©vÃ©ritÃ© :** MIN  
**CatÃ©gorie :** VIO-LOI

**Description :** Caring Nanny dÃ©pend d'une horloge synchronisÃ©e entre nÅ“uds.

**Exemples de violations :**
```
âŒ Comparaison directe de timestamps entre nÅ“uds distants
âŒ RÃ©solution de conflits par "le plus rÃ©cent gagne" (timestamps absolus)
âŒ Validation d'observations basÃ©e sur l'heure rÃ©seau
```

**PrÃ©vention :**
- Timestamps locaux uniquement
- Horloges logiques ou vectorielles pour l'ordonnancement inter-nÅ“uds
- Comparaison temporelle explicitement encadrÃ©e

---

## 9. Anti-patterns architecturaux

### 9.1 ANTI-ARCH-001 : Observateur omniscient

**Description :** Caring Nanny est conÃ§ue pour tout savoir sur tous les composants Ã  tout moment.

**ProblÃ¨me :** Charge excessive, couplage fort, violation de LOI-5 (ressources proportionnelles au hardware).

**Pattern correct :**
- Observation ciblÃ©e sur les composants critiques
- Pull on-demand plutÃ´t que push constant
- GranularitÃ© configurable

---

### 9.2 ANTI-ARCH-002 : Caring Nanny comme bus d'Ã©vÃ©nements

**Description :** Caring Nanny est utilisÃ©e comme bus d'Ã©vÃ©nements gÃ©nÃ©ral pour tous les Ã©vÃ©nements systÃ¨me.

**ProblÃ¨me :** DÃ©tournement de la responsabilitÃ© (observation d'Ã©tat vs distribution d'Ã©vÃ©nements), surcharge.

**Pattern correct :**
- Caring Nanny observe les Ã©tats, pas tous les Ã©vÃ©nements
- La propagation d'Ã©tat passe par BondingBrother
- Les Ã©vÃ©nements mÃ©tier ont leur propre canal

---

### 9.3 ANTI-ARCH-003 : Ã‰tat distribuÃ© synchrone

**Description :** Caring Nanny tente de maintenir un Ã©tat synchrone entre plusieurs nÅ“uds.

**ProblÃ¨me :** Violation de LOI-1, LOI-2, LOI-4. Impossible sans dÃ©pendance externe et temps global.

**Pattern correct :**
- Chaque nÅ“ud a son propre Caring Nanny
- L'Ã©tat est local et souverain
- La rÃ©conciliation est explicite et asynchrone

---

### 9.4 ANTI-ARCH-004 : Circuit de feedback automatique

**Description :** Caring Nanny dÃ©clenche automatiquement des actions en rÃ©ponse aux Ã©tats dÃ©tectÃ©s (circuit fermÃ© observation â†’ action).

**ProblÃ¨me :** Violation de INV-CN-2, INV-CN-3. Caring Nanny devient un acteur dÃ©cisionnel.

**Pattern correct :**
```
Observation â†’ Propagation â†’ DÃ©cision (StrongFather) â†’ Action (Composant)
             â†‘
        Caring Nanny s'arrÃªte ici
```

---

## 10. Anti-patterns d'implÃ©mentation

### 10.1 ANTI-IMPL-001 : Cache d'Ã©tat mutable partagÃ©

**Description :** L'Ã©tat observÃ© est stockÃ© dans un cache mutable accessible par plusieurs composants.

**ProblÃ¨me :** Race conditions, Ã©tats incohÃ©rents, violations de INV-CN-4.

**Pattern correct :**
- Ã‰tat immutable
- Copies locales pour les consommateurs
- Versioning des Ã©tats

---

### 10.2 ANTI-IMPL-002 : Polling agressif

**Description :** Caring Nanny interroge les composants Ã  haute frÃ©quence pour dÃ©tecter les changements.

**ProblÃ¨me :** Violation de GAR-AUTH-01 (observation intrusive), LOI-5 (ressources proportionnelles).

**Pattern correct :**
- Ã‰vÃ©nements push des composants vers Caring Nanny
- Polling Ã  basse frÃ©quence comme fallback
- FrÃ©quence adaptative selon l'activitÃ©

---

### 10.3 ANTI-IMPL-003 : Historique illimitÃ©

**Description :** L'historique d'observations croÃ®t indÃ©finiment sans limite ni rÃ©tention.

**ProblÃ¨me :** Violation de LOI-5, saturation mÃ©moire/disque.

**Pattern correct :**
- RÃ©tention configurable
- Archivage pÃ©riodique
- AgrÃ©gation des observations anciennes

---

### 10.4 ANTI-IMPL-004 : Exception comme contrÃ´le de flux

**Description :** Les conditions d'erreur d'observation sont gÃ©rÃ©es via des exceptions qui remontent et bloquent.

**ProblÃ¨me :** Violation de INV-CN-6 (non-bloquant), comportement imprÃ©visible.

**Pattern correct :**
- Gestion explicite des erreurs d'observation
- Fallback sur Ã©tat "unknown" si nÃ©cessaire
- Logging de l'erreur sans blocage

---

## 11. Anti-patterns d'intÃ©gration

### 11.1 ANTI-INT-001 : AccÃ¨s direct aux autoritÃ©s

**Description :** Un produit consulte Caring Nanny puis accÃ¨de directement Ã  KindMother ou StrongFather sans passer par BondingBrother.

**ProblÃ¨me :** Contournement de l'architecture de mÃ©diation, traÃ§abilitÃ© perdue.

**Pattern correct :**
```
Produit â†’ BondingBrother â†’ (Caring Nanny pour l'Ã©tat)
                        â†’ (KindMother pour les donnÃ©es)
                        â†’ (StrongFather pour les dÃ©cisions)
```

---

### 11.2 ANTI-INT-002 : Ã‰tat comme prÃ©requis bloquant

**Description :** Un composant bloque une opÃ©ration en attendant un Ã©tat spÃ©cifique de Caring Nanny.

**ProblÃ¨me :** Violation de INV-CN-6, LOI-2. L'Ã©tat est informatif, pas prescriptif.

**Pattern correct :**
- Consulter l'Ã©tat de maniÃ¨re non bloquante
- Prendre une dÃ©cision basÃ©e sur l'Ã©tat (via StrongFather si nÃ©cessaire)
- ProcÃ©der ou dÃ©gradÃ© selon la dÃ©cision, pas selon l'Ã©tat seul

---

### 11.3 ANTI-INT-003 : Couplage Ã©tat/action

**Description :** Le code client couple directement un Ã©tat observÃ© Ã  une action sans passer par le circuit de dÃ©cision.

**ProblÃ¨me :** Court-circuitage de StrongFather, logique dÃ©cisionnelle dispersÃ©e.

**Exemple de violation :**
```pseudocode
// âŒ INTERDIT
state = caringNanny.getState()
if state == "healthy":
    executeOperation()
else:
    abort()
```

**Pattern correct :**
```pseudocode
// âœ… CORRECT
context = { state: caringNanny.getState() }
decision = strongFather.evaluate(intent, context)
if decision.approved:
    executeOperation()
```

---

## 12. DÃ©tection et prÃ©vention

### 12.1 MÃ©canismes de dÃ©tection

| Violation | MÃ©canisme | Moment |
|-----------|-----------|--------|
| VIO-CN-001 Ã  VIO-CN-003 | Revue architecturale, analyse statique | CI, PR |
| VIO-CN-004 | Tests de cohÃ©rence automatisÃ©s | CI, Runtime |
| VIO-CN-005, VIO-FLUX-001/002/003 | RÃ©conciliation traces/observations | Runtime, Batch |
| VIO-CN-006 | Monitoring latence | Runtime |
| VIO-CN-007 | Comparaison observation/propagation | Runtime |
| VIO-NEG-* | Analyse des dÃ©pendances, revue de code | CI, PR |
| VIO-GAR-* | Tests contractuels | CI, Release |
| VIO-LOI-* | Tests d'isolation (mode offline) | CI, Release |
| ANTI-* | Revue de design, tests de charge | PR, Release |

### 12.2 Checklist de conformitÃ©

Avant toute mise en production, vÃ©rifier :

**Nature (INV-CN-1, 2, 3) :**
- [ ] Aucune mÃ©thode d'Ã©criture vers les donnÃ©es mÃ©tier
- [ ] Aucune mÃ©thode d'exÃ©cution d'action
- [ ] Aucune mÃ©thode de validation/autorisation

**CohÃ©rence (INV-CN-4) :**
- [ ] Tests de cohÃ©rence sur toutes les rÃ©ponses d'Ã©tat
- [ ] RÃ¨gles de prioritÃ© documentÃ©es et implÃ©mentÃ©es

**Flux (INV-CN-5, 6, 7, INV-FLUX-*) :**
- [ ] TraÃ§abilitÃ© complÃ¨te vÃ©rifiable
- [ ] Tests de performance (latence < seuil)
- [ ] Comparaison observation/propagation automatisÃ©e

**Autonomie (LOI-1 Ã  LOI-6) :**
- [ ] Fonctionnement vÃ©rifiÃ© en mode offline
- [ ] Ã‰tat "offline" traitÃ© comme Ã©tat normal
- [ ] Aucune dÃ©pendance externe obligatoire
- [ ] Ressources prÃ©visibles et maÃ®trisÃ©es

### 12.3 Actions en cas de violation dÃ©tectÃ©e

| SÃ©vÃ©ritÃ© | Action immÃ©diate | DÃ©lai de correction |
|----------|------------------|---------------------|
| CRIT | Blocage du dÃ©ploiement, escalade | Avant mise en production |
| MAJ | Alerte, correction prioritaire | Avant release |
| MIN | Ticket de correction | Selon planning |
| WARN | Documentation, justification | Revue pÃ©riodique |

---

## 13. Matrice de correspondance violations/invariants

| Code violation | Invariant(s) concernÃ©(s) | SÃ©vÃ©ritÃ© |
|----------------|--------------------------|----------|
| VIO-CN-001 | INV-CN-1 | CRIT |
| VIO-CN-002 | INV-CN-2 | CRIT |
| VIO-CN-003 | INV-CN-3 | CRIT |
| VIO-CN-004 | INV-CN-4 | MAJ |
| VIO-CN-005 | INV-CN-5 | MAJ |
| VIO-CN-006 | INV-CN-6 | MAJ |
| VIO-CN-007 | INV-CN-7 | MAJ |
| VIO-NEG-001 | INV-NEG-CN-01 | CRIT |
| VIO-NEG-002 | INV-NEG-CN-02 | CRIT |
| VIO-NEG-003 | INV-NEG-CN-03 | CRIT |
| VIO-NEG-004 | INV-NEG-CN-04 | CRIT |
| VIO-NEG-005 | INV-NEG-CN-05 | MAJ |
| VIO-NEG-006 | INV-NEG-CN-06 | MAJ |
| VIO-FLUX-001 | INV-FLUX-CN-01 | MAJ |
| VIO-FLUX-002 | INV-FLUX-CN-02 | MAJ |
| VIO-FLUX-003 | INV-FLUX-CN-03 | MAJ |
| VIO-GAR-001 | GAR-CONS-01 | MAJ |
| VIO-GAR-002 | GAR-CONS-04 | MAJ |
| VIO-GAR-003 | GAR-CONS-05 | MIN |
| VIO-GAR-004 | GAR-AUTH-01 | MAJ |
| VIO-LOI-001 | LOI-1 | CRIT |
| VIO-LOI-002 | LOI-2 | MAJ |
| VIO-LOI-003 | LOI-3 | MAJ |
| VIO-LOI-004 | LOI-4 | MIN |

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut GOUVERNANCE**. Il catalogue les violations et anti-patterns Ã  Ã©viter lors de l'implÃ©mentation ou de l'intÃ©gration de Caring Nanny.

Toute implÃ©mentation de Caring Nanny doit Ãªtre vÃ©rifiÃ©e contre ce catalogue. Toute violation de sÃ©vÃ©ritÃ© CRIT bloque la mise en production. Les violations MAJ doivent Ãªtre corrigÃ©es avant release.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** GOUVERNANCE â€” Catalogue normatif  
**DÃ©pendances :**
- Caring Nanny - Documentation Fondatrice v1.6
- Caring Nanny - Invariants et Garanties v1.0
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

