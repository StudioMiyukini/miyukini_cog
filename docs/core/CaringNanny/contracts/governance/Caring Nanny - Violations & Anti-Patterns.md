# Caring Nanny - Violations & Anti-Patterns

## 1. Contexte

Ce document catalogue les **violations** des invariants de Caring Nanny et les **anti-patterns** à éviter lors de l'implémentation ou de l'intégration. Il constitue un guide de conformité permettant d'identifier les écarts par rapport aux contrats normatifs et de comprendre leurs conséquences.

**Références normatives :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) — Invariants fondateurs (Section 7)
- [Caring Nanny - Invariants et Garanties](./Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) — Invariants détaillés
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) — LOI-1 à LOI-6

## 2. Portée / Scope

Ce document couvre :
- Les violations des invariants fondamentaux (INV-CN-*)
- Les violations des invariants de non-action (INV-NEG-CN-*)
- Les violations des invariants de flux (INV-FLUX-CN-*)
- Les anti-patterns architecturaux, d'implémentation, et d'intégration
- Les mécanismes de détection et de prévention

Ce document **ne couvre pas** :
- Le modèle d'erreur opérationnel (voir Error & Rejection Model)
- Les détails d'implémentation spécifiques
- Les stratégies de correction (Caring Nanny ne corrige jamais)

---

## 3. Taxonomie des violations

### 3.1 Niveaux de sévérité

| Niveau | Code | Description | Conséquence |
|--------|------|-------------|-------------|
| **Critique** | CRIT | Viole un invariant fondamental, compromet la nature de Caring Nanny | Rejet immédiat, refonte requise |
| **Majeur** | MAJ | Viole une garantie ou un invariant de flux | Correction urgente avant mise en production |
| **Mineur** | MIN | Dégradation de qualité sans violation d'invariant | Correction recommandée |
| **Avertissement** | WARN | Pratique déconseillée, risque potentiel | Revue et justification requises |

### 3.2 Catégories de violations

| Catégorie | Description |
|-----------|-------------|
| **VIO-NAT** | Violation de nature (ce que Caring Nanny EST) |
| **VIO-NEG** | Violation de non-action (ce que Caring Nanny NE FAIT JAMAIS) |
| **VIO-FLUX** | Violation de flux (comment l'information transite) |
| **VIO-GAR** | Violation de garantie envers les consommateurs ou autorités |
| **VIO-LOI** | Violation des Lois d'Autonomie Système |

---

## 4. Violations des invariants fondamentaux (INV-CN-*)

### 4.1 VIO-CN-001 : Modification de données observées

**Invariant violé :** INV-CN-1 (Observateur pur)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NAT

**Description :** Caring Nanny modifie des données dans le système qu'elle observe, que ce soit directement ou via un effet de bord.

**Exemples de violations :**
```
❌ Mettre à jour un flag "last_observed_at" dans les données métier
❌ Marquer une entité comme "checked" après observation
❌ Créer des entrées dans les tables métier de KindMother
❌ Modifier l'état d'un composant pour "forcer" une cohérence
```

**Conséquences :**
- Corruption de la séparation des responsabilités
- Impossibilité de distinguer les modifications métier des modifications d'observation
- Violation de l'autorité exclusive de KindMother sur les données

**Prévention :**
- Aucune méthode `write()`, `update()`, `delete()` sur les données métier
- Audit automatisé des appels API sortants
- Historique d'observation séparé des données observées

---

### 4.2 VIO-CN-002 : Capacité d'exécution masquée

**Invariant violé :** INV-CN-2 (Aucune capacité d'exécution)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NAT

**Description :** Caring Nanny déclenche une action, directement ou indirectement, en réponse à une observation.

**Exemples de violations :**
```
❌ Appeler une fonction de redémarrage suite à la détection d'une erreur
❌ Déclencher une synchronisation automatique en réponse à un état "degraded"
❌ Envoyer un email/notification directement (sans passer par BondingBrother)
❌ Invalider un cache en réponse à une détection d'incohérence
```

**Conséquences :**
- Caring Nanny devient un acteur, pas un observateur
- Effets de bord imprévisibles
- Violation de la chaîne de responsabilité (décision = StrongFather, exécution = composants dédiés)

**Prévention :**
- Aucune méthode `execute()`, `trigger()`, `invoke()`, `send()` directe
- Propagation d'informations uniquement via BondingBrother
- Revue architecturale systématique

---

### 4.3 VIO-CN-003 : Autorité implicite

**Invariant violé :** INV-CN-3 (Non-autoritaire)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NAT

**Description :** Caring Nanny prend des décisions d'autorisation, de validation, ou de blocage basées sur l'état observé.

**Exemples de violations :**
```
❌ Bloquer une opération parce que l'état est "degraded"
❌ Refuser de rapporter un état car il est "invalide"
❌ Valider qu'une entité est "prête" avant de permettre une action
❌ Définir des seuils qui bloquent automatiquement certaines opérations
```

**Conséquences :**
- Caring Nanny devient une autorité de fait
- Court-circuitage de StrongFather dans la chaîne de décision
- Comportement imprévisible et non traçable

**Prévention :**
- Aucune méthode `validate()`, `authorize()`, `approve()`, `reject()`
- L'état est toujours informatif, jamais prescriptif
- Les décisions basées sur l'état sont prises par StrongFather

---

### 4.4 VIO-CN-004 : État incohérent rapporté

**Invariant violé :** INV-CN-4 (État cohérent)  
**Sévérité :** MAJ  
**Catégorie :** VIO-NAT

**Description :** Caring Nanny rapporte un état contenant des contradictions (un composant simultanément dans deux états mutuellement exclusifs).

**Exemples de violations :**
```
❌ Composant X rapporté comme "healthy" et "error" simultanément
❌ État global "offline" avec sous-composants "syncing"
❌ Transition de "healthy" vers "healthy" enregistrée
❌ Deux états différents retournés pour la même requête à des moments proches
```

**Conséquences :**
- Perte de confiance dans les informations d'état
- Décisions incorrectes basées sur des états contradictoires
- Impossibilité de diagnostic fiable

**Prévention :**
- Règles de cohérence explicites dans l'agrégateur d'état
- Tests automatisés de cohérence sur toutes les réponses
- Priorité des états en cas de contradiction (error > degraded > syncing > offline > healthy)

---

### 4.5 VIO-CN-005 : Perte de traçabilité

**Invariant violé :** INV-CN-5 (Traçabilité complète)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** Une observation, transition, ou propagation n'est pas enregistrée dans l'historique.

**Exemples de violations :**
```
❌ Transition d'état sans entrée dans l'historique
❌ Observation détectée mais non journalisée
❌ Propagation effectuée sans trace
❌ Consultation d'état non enregistrée
```

**Conséquences :**
- Impossibilité d'audit a posteriori
- Diagnostic incomplet en cas de problème
- Violation de LOI-3 (souveraineté de l'état local)

**Prévention :**
- Journalisation immédiate avant traitement
- Réconciliation périodique observations/historique
- Alertes en cas de divergence

---

### 4.6 VIO-CN-006 : Observation bloquante

**Invariant violé :** INV-CN-6 (Non-bloquant)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** L'observation de Caring Nanny bloque ou ralentit significativement les opérations du système.

**Exemples de violations :**
```
❌ Consultation d'état synchrone avec timeout élevé
❌ Observation qui verrouille des ressources partagées
❌ Buffer d'observations sans limite causant une saturation mémoire
❌ Propagation synchrone attendant une confirmation
```

**Conséquences :**
- Dégradation des performances du système
- Latence perceptible pour les utilisateurs
- Potentiel blocage complet en cas de surcharge

**Prévention :**
- Consultations asynchrones ou à faible latence
- Propagations non bloquantes (fire-and-forget)
- Monitoring des temps de réponse
- Limites sur les buffers d'observations

---

### 4.7 VIO-CN-007 : Propagation altérée

**Invariant violé :** INV-CN-7 (Propagation fidèle)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** L'information propagée diffère de l'information observée (filtrage, transformation, interprétation).

**Exemples de violations :**
```
❌ Filtrer certains états avant propagation
❌ Transformer "error" en "degraded" pour "simplifier"
❌ Ajouter une interprétation à l'état propagé
❌ Agréger plusieurs transitions en une seule notification
```

**Conséquences :**
- Perte d'information pour les destinataires
- Décisions basées sur des informations incomplètes
- Divergence entre l'historique local et les notifications

**Prévention :**
- Comparaison automatisée observation/propagation
- Propagation de l'état brut sans transformation
- Interprétation laissée aux destinataires

---

## 5. Violations des invariants de non-action (INV-NEG-CN-*)

### 5.1 VIO-NEG-001 : Écriture dans la persistance métier

**Invariant violé :** INV-NEG-CN-01 (Jamais de modification de données)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny écrit, modifie, ou supprime des données dans KindMother ou tout autre stockage métier.

**Exemples de violations :**
```
❌ INSERT dans une table métier
❌ UPDATE d'un champ métier suite à une observation
❌ DELETE d'une entité considérée comme "obsolète"
❌ Création d'un WriteIntent depuis Caring Nanny
```

**Conséquences :**
- Violation de l'autorité de KindMother
- Données métier polluées par des modifications non métier
- Traçabilité des modifications compromise

**Prévention :**
- Aucune dépendance vers les API d'écriture de KindMother
- Audit des appels SQL/API
- Historique d'observation dans un stockage séparé

---

### 5.2 VIO-NEG-002 : Logique décisionnelle

**Invariant violé :** INV-NEG-CN-02 (Jamais de décision)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny contient une logique conditionnelle qui aboutit à une action ou un comportement différencié basé sur des critères métier.

**Exemples de violations :**
```
❌ if (state == "error") { activateBackupMode(); }
❌ if (component.health < threshold) { notifyAdmin(); }
❌ switch (state) { case "degraded": limitOperations(); }
❌ Déterminer dynamiquement si une opération doit être autorisée
```

**Ce qui est autorisé :**
```
✅ Classifier l'état selon les catégories définies
✅ Appliquer des règles d'agrégation prédéfinies (error > degraded > ...)
✅ Déterminer les destinataires d'une propagation selon des règles établies
```

**Prévention :**
- Revue de code ciblée sur les structures conditionnelles
- Les règles métier sont externes à Caring Nanny
- Actions décidées par StrongFather, jamais par Caring Nanny

---

### 5.3 VIO-NEG-003 : Action corrective automatique

**Invariant violé :** INV-NEG-CN-03 (Jamais d'action corrective)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny tente de corriger une anomalie détectée au lieu de simplement la rapporter.

**Exemples de violations :**
```
❌ Redémarrer un service défaillant
❌ Réinitialiser un composant bloqué
❌ Forcer une synchronisation pour résoudre une incohérence
❌ Vider un cache pour résoudre un problème de performance
❌ Basculer vers un mode de secours
```

**Flux correct :**
```
Anomalie détectée → Observation enregistrée → Propagation via BondingBrother
                    → StrongFather décide de l'action (ou pas)
                    → Composant concerné exécute l'action (ou pas)
```

**Prévention :**
- Aucune capacité d'action dans Caring Nanny
- Rôle limité à : détecter, classifier, propager, historiser
- Actions correctives implémentées ailleurs

---

### 5.4 VIO-NEG-004 : Médiation d'intentions

**Invariant violé :** INV-NEG-CN-04 (Jamais de médiation d'intentions)  
**Sévérité :** CRIT  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny reçoit, traduit, ou route des intentions de produits vers les autorités.

**Exemples de violations :**
```
❌ Exposer une API pour recevoir des demandes de produits
❌ Traduire une demande utilisateur en action système
❌ Router une intention vers KindMother ou StrongFather
❌ Filtrer les réponses des autorités avant de les transmettre aux produits
```

**Distinction claire :**
| Rôle | Responsable | Caring Nanny |
|------|-------------|--------------|
| Recevoir des intentions | BondingBrother | ❌ Interdit |
| Observer des états | Caring Nanny | ✅ Autorisé |
| Propager des états | Caring Nanny (via BondingBrother) | ✅ Autorisé |

**Prévention :**
- Aucune interface d'intention exposée par Caring Nanny
- Toute médiation passe par BondingBrother

---

### 5.5 VIO-NEG-005 : Définition de règles dynamiques

**Invariant violé :** INV-NEG-CN-05 (Jamais de définition de règles)  
**Sévérité :** MAJ  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny définit ou modifie dynamiquement les règles de classification des états ou de détection des anomalies.

**Exemples de violations :**
```
❌ Ajuster dynamiquement les seuils de dégradation
❌ Apprendre de nouveaux patterns d'anomalie
❌ Modifier les critères de classification en fonction du contexte
❌ Créer de nouvelles catégories d'état à la volée
```

**Ce qui est autorisé :**
```
✅ Charger des règles depuis une configuration externe
✅ Appliquer des règles définies par le produit
✅ Classifier selon des critères établis au déploiement
```

**Prévention :**
- Règles chargées depuis une source externe (configuration)
- Aucune logique d'apprentissage ou d'adaptation dans Caring Nanny
- Les règles sont définies par le produit ou l'écosystème

---

### 5.6 VIO-NEG-006 : Persistance autonome externe

**Invariant violé :** INV-NEG-CN-06 (Jamais de gestion de persistance)  
**Sévérité :** MAJ  
**Catégorie :** VIO-NEG

**Description :** Caring Nanny gère directement la persistance de ses observations dans un système externe.

**Exemples de violations :**
```
❌ Connexion directe à une base de données externe pour persister l'historique
❌ Gestion autonome de transactions de persistance
❌ Définition de stratégies de rétention dans Caring Nanny
❌ Écriture vers un système de logs externe sans délégation
```

**Ce qui est autorisé :**
```
✅ Maintenir un historique en mémoire
✅ Déléguer la persistance à KindMother si nécessaire
✅ Exposer l'historique pour consultation
```

**Prévention :**
- Aucune connexion directe à un système de persistance externe
- Si persistance nécessaire, délégation à KindMother via les canaux appropriés

---

## 6. Violations des invariants de flux (INV-FLUX-CN-*)

### 6.1 VIO-FLUX-001 : Séquence d'observation incomplète

**Invariant violé :** INV-FLUX-CN-01 (Séquence d'observation cohérente)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** Une observation ne suit pas la séquence définie, avec des étapes sautées ou dans le désordre.

**Séquence obligatoire :**
1. Détection de condition
2. Évaluation selon les critères de classification
3. Traduction en état partiel
4. Agrégation en état global (si applicable)
5. Détection de transition (si changement)
6. Enregistrement dans l'historique

**Exemples de violations :**
```
❌ Enregistrer une observation sans l'avoir classifiée
❌ Agréger sans avoir traduit en état partiel
❌ Détecter une transition sans enregistrer dans l'historique
❌ Sauter l'évaluation pour "optimiser"
```

**Prévention :**
- Chaque étape est tracée individuellement
- Alertes sur séquences incomplètes
- Pipeline d'observation explicite et testable

---

### 6.2 VIO-FLUX-002 : Séquence de propagation incomplète

**Invariant violé :** INV-FLUX-CN-02 (Séquence de propagation cohérente)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** Une propagation ne suit pas la séquence définie.

**Séquence obligatoire :**
1. Identification des destinataires
2. Formulation du message (état précédent, état actuel, cause)
3. Délégation à BondingBrother
4. Enregistrement de la propagation

**Exemples de violations :**
```
❌ Propager sans identifier les destinataires
❌ Envoyer un message sans état précédent/actuel/cause
❌ Contourner BondingBrother pour une propagation directe
❌ Oublier d'enregistrer la propagation
```

**Prévention :**
- Validation du message avant propagation
- Aucun canal direct vers les destinataires
- Comparaison transitions/propagations

---

### 6.3 VIO-FLUX-003 : Perte d'observations

**Invariant violé :** INV-FLUX-CN-03 (Pas de perte d'observation)  
**Sévérité :** MAJ  
**Catégorie :** VIO-FLUX

**Description :** Des observations sont perdues en raison de la charge, de conditions anormales, ou de défauts d'implémentation.

**Exemples de violations :**
```
❌ Buffer plein sans traitement des observations en attente
❌ Exception non gérée qui fait perdre une observation
❌ Prioritisation trop agressive qui supprime des observations non critiques
❌ Crash pendant le traitement sans récupération
```

**Prévention :**
- Buffer d'observations avec limite et stratégie de débordement
- Journalisation immédiate avant traitement complet
- Priorité aux observations critiques (error > degraded > autres)
- Réconciliation périodique conditions détectées/observations enregistrées

---

## 7. Violations des garanties (GAR-*)

### 7.1 VIO-GAR-001 : État indisponible

**Garantie violée :** GAR-CONS-01 (État toujours disponible)  
**Sévérité :** MAJ  
**Catégorie :** VIO-GAR

**Description :** Une demande d'état n'obtient pas de réponse.

**Exemples de violations :**
```
❌ Timeout sans réponse sur une consultation d'état
❌ Erreur non gérée retournant une exception au lieu d'un état
❌ Blocage infini sur une ressource
```

**Comportement attendu :**
- Toujours retourner une réponse
- En cas d'incertitude, retourner "unknown" ou le dernier état connu
- Inclure le timestamp de l'observation

---

### 7.2 VIO-GAR-002 : Notification non fiable

**Garantie violée :** GAR-CONS-04 (Notifications fiables)  
**Sévérité :** MAJ  
**Catégorie :** VIO-GAR

**Description :** Les notifications de changement d'état sont manquantes, dupliquées, ou désordonnées.

**Exemples de violations :**
```
❌ Transition sans notification correspondante
❌ Même notification émise plusieurs fois
❌ Notifications reçues dans un ordre différent de l'ordre des transitions
```

**Prévention :**
- Comparaison transitions enregistrées/notifications émises
- Mécanisme d'idempotence sur les notifications
- Numérotation séquentielle des notifications

---

### 7.3 VIO-GAR-003 : Contexte incomplet

**Garantie violée :** GAR-CONS-05 (Contexte complet)  
**Sévérité :** MIN  
**Catégorie :** VIO-GAR

**Description :** Une réponse d'état ne contient pas toutes les informations de contexte requises.

**Informations requises :**
- État courant
- Timestamp de l'observation
- Durée dans l'état actuel
- Cause de la dernière transition (si disponible)

**Exemples de violations :**
```
❌ État retourné sans timestamp
❌ Durée dans l'état non calculée
❌ Cause de transition omise
```

---

### 7.4 VIO-GAR-004 : Observation intrusive

**Garantie violée :** GAR-AUTH-01 (Observation non intrusive)  
**Sévérité :** MAJ  
**Catégorie :** VIO-GAR

**Description :** L'observation de Caring Nanny impacte les performances ou le fonctionnement des autorités observées.

**Exemples de violations :**
```
❌ Requêtes d'observation causant une charge significative sur KindMother
❌ Verrouillage de ressources pendant l'observation
❌ Polling agressif dégradant les performances
```

**Conformité LOI-2 :** L'observation intrusive peut empêcher le système de fonctionner normalement en isolation.

**Prévention :**
- Observation passive (événements push plutôt que polling)
- Aucun verrouillage de ressources
- Tests de charge avec et sans Caring Nanny

---

## 8. Violations des Lois d'Autonomie (VIO-LOI-*)

### 8.1 VIO-LOI-001 : Dépendance externe critique

**Loi violée :** LOI-1 (Aucune dépendance externe critique à l'exécution)  
**Sévérité :** CRIT  
**Catégorie :** VIO-LOI

**Description :** Caring Nanny nécessite un appel externe pour fonctionner.

**Exemples de violations :**
```
❌ Charger les règles de classification depuis un serveur distant
❌ Valider l'état via un service externe
❌ Persister l'historique uniquement sur un cloud
❌ Consulter une API externe pour classifier un état
```

**Prévention :**
- Règles de classification embarquées localement
- Historique local autonome
- Services externes optionnels, jamais obligatoires

---

### 8.2 VIO-LOI-002 : Isolement traité comme erreur

**Loi violée :** LOI-2 (Le système accepte l'isolement comme état normal)  
**Sévérité :** MAJ  
**Catégorie :** VIO-LOI

**Description :** Caring Nanny traite l'état "offline" comme une erreur plutôt qu'un état normal.

**Exemples de violations :**
```
❌ Classifier "offline" avec la même sévérité que "error"
❌ Retry infini pour reconnecter en mode offline
❌ Alertes de type "erreur" pour l'état isolé
❌ Blocage de l'observation en l'absence de connexion
```

**Distinction requise :**
| État | Nature | Traitement |
|------|--------|------------|
| offline | Normal | Information, pas d'alerte |
| error | Anormal | Alerte, investigation requise |

---

### 8.3 VIO-LOI-003 : État local non souverain

**Loi violée :** LOI-3 (L'état local est souverain)  
**Sévérité :** MAJ  
**Catégorie :** VIO-LOI

**Description :** Caring Nanny invalide ou ignore l'état local au profit d'un état distant.

**Exemples de violations :**
```
❌ Écraser l'historique local avec un historique distant
❌ Ignorer les observations locales si elles contredisent un état distant
❌ Synchronisation qui efface des transitions locales
```

**Prévention :**
- L'historique local est la source de vérité
- La réconciliation est explicite et traçable
- Les données locales ne sont jamais invalidées implicitement

---

### 8.4 VIO-LOI-004 : Dépendance au temps global

**Loi violée :** LOI-4 (Pas de temps global requis)  
**Sévérité :** MIN  
**Catégorie :** VIO-LOI

**Description :** Caring Nanny dépend d'une horloge synchronisée entre nœuds.

**Exemples de violations :**
```
❌ Comparaison directe de timestamps entre nœuds distants
❌ Résolution de conflits par "le plus récent gagne" (timestamps absolus)
❌ Validation d'observations basée sur l'heure réseau
```

**Prévention :**
- Timestamps locaux uniquement
- Horloges logiques ou vectorielles pour l'ordonnancement inter-nœuds
- Comparaison temporelle explicitement encadrée

---

## 9. Anti-patterns architecturaux

### 9.1 ANTI-ARCH-001 : Observateur omniscient

**Description :** Caring Nanny est conçue pour tout savoir sur tous les composants à tout moment.

**Problème :** Charge excessive, couplage fort, violation de LOI-5 (ressources proportionnelles au hardware).

**Pattern correct :**
- Observation ciblée sur les composants critiques
- Pull on-demand plutôt que push constant
- Granularité configurable

---

### 9.2 ANTI-ARCH-002 : Caring Nanny comme bus d'événements

**Description :** Caring Nanny est utilisée comme bus d'événements général pour tous les événements système.

**Problème :** Détournement de la responsabilité (observation d'état vs distribution d'événements), surcharge.

**Pattern correct :**
- Caring Nanny observe les états, pas tous les événements
- La propagation d'état passe par BondingBrother
- Les événements métier ont leur propre canal

---

### 9.3 ANTI-ARCH-003 : État distribué synchrone

**Description :** Caring Nanny tente de maintenir un état synchrone entre plusieurs nœuds.

**Problème :** Violation de LOI-1, LOI-2, LOI-4. Impossible sans dépendance externe et temps global.

**Pattern correct :**
- Chaque nœud a son propre Caring Nanny
- L'état est local et souverain
- La réconciliation est explicite et asynchrone

---

### 9.4 ANTI-ARCH-004 : Circuit de feedback automatique

**Description :** Caring Nanny déclenche automatiquement des actions en réponse aux états détectés (circuit fermé observation → action).

**Problème :** Violation de INV-CN-2, INV-CN-3. Caring Nanny devient un acteur décisionnel.

**Pattern correct :**
```
Observation → Propagation → Décision (StrongFather) → Action (Composant)
             ↑
        Caring Nanny s'arrête ici
```

---

## 10. Anti-patterns d'implémentation

### 10.1 ANTI-IMPL-001 : Cache d'état mutable partagé

**Description :** L'état observé est stocké dans un cache mutable accessible par plusieurs composants.

**Problème :** Race conditions, états incohérents, violations de INV-CN-4.

**Pattern correct :**
- État immutable
- Copies locales pour les consommateurs
- Versioning des états

---

### 10.2 ANTI-IMPL-002 : Polling agressif

**Description :** Caring Nanny interroge les composants à haute fréquence pour détecter les changements.

**Problème :** Violation de GAR-AUTH-01 (observation intrusive), LOI-5 (ressources proportionnelles).

**Pattern correct :**
- Événements push des composants vers Caring Nanny
- Polling à basse fréquence comme fallback
- Fréquence adaptative selon l'activité

---

### 10.3 ANTI-IMPL-003 : Historique illimité

**Description :** L'historique d'observations croît indéfiniment sans limite ni rétention.

**Problème :** Violation de LOI-5, saturation mémoire/disque.

**Pattern correct :**
- Rétention configurable
- Archivage périodique
- Agrégation des observations anciennes

---

### 10.4 ANTI-IMPL-004 : Exception comme contrôle de flux

**Description :** Les conditions d'erreur d'observation sont gérées via des exceptions qui remontent et bloquent.

**Problème :** Violation de INV-CN-6 (non-bloquant), comportement imprévisible.

**Pattern correct :**
- Gestion explicite des erreurs d'observation
- Fallback sur état "unknown" si nécessaire
- Logging de l'erreur sans blocage

---

## 11. Anti-patterns d'intégration

### 11.1 ANTI-INT-001 : Accès direct aux autorités

**Description :** Un produit consulte Caring Nanny puis accède directement à KindMother ou StrongFather sans passer par BondingBrother.

**Problème :** Contournement de l'architecture de médiation, traçabilité perdue.

**Pattern correct :**
```
Produit → BondingBrother → (Caring Nanny pour l'état)
                        → (KindMother pour les données)
                        → (StrongFather pour les décisions)
```

---

### 11.2 ANTI-INT-002 : État comme prérequis bloquant

**Description :** Un composant bloque une opération en attendant un état spécifique de Caring Nanny.

**Problème :** Violation de INV-CN-6, LOI-2. L'état est informatif, pas prescriptif.

**Pattern correct :**
- Consulter l'état de manière non bloquante
- Prendre une décision basée sur l'état (via StrongFather si nécessaire)
- Procéder ou dégradé selon la décision, pas selon l'état seul

---

### 11.3 ANTI-INT-003 : Couplage état/action

**Description :** Le code client couple directement un état observé à une action sans passer par le circuit de décision.

**Problème :** Court-circuitage de StrongFather, logique décisionnelle dispersée.

**Exemple de violation :**
```pseudocode
// ❌ INTERDIT
state = caringNanny.getState()
if state == "healthy":
    executeOperation()
else:
    abort()
```

**Pattern correct :**
```pseudocode
// ✅ CORRECT
context = { state: caringNanny.getState() }
decision = strongFather.evaluate(intent, context)
if decision.approved:
    executeOperation()
```

---

## 12. Détection et prévention

### 12.1 Mécanismes de détection

| Violation | Mécanisme | Moment |
|-----------|-----------|--------|
| VIO-CN-001 à VIO-CN-003 | Revue architecturale, analyse statique | CI, PR |
| VIO-CN-004 | Tests de cohérence automatisés | CI, Runtime |
| VIO-CN-005, VIO-FLUX-001/002/003 | Réconciliation traces/observations | Runtime, Batch |
| VIO-CN-006 | Monitoring latence | Runtime |
| VIO-CN-007 | Comparaison observation/propagation | Runtime |
| VIO-NEG-* | Analyse des dépendances, revue de code | CI, PR |
| VIO-GAR-* | Tests contractuels | CI, Release |
| VIO-LOI-* | Tests d'isolation (mode offline) | CI, Release |
| ANTI-* | Revue de design, tests de charge | PR, Release |

### 12.2 Checklist de conformité

Avant toute mise en production, vérifier :

**Nature (INV-CN-1, 2, 3) :**
- [ ] Aucune méthode d'écriture vers les données métier
- [ ] Aucune méthode d'exécution d'action
- [ ] Aucune méthode de validation/autorisation

**Cohérence (INV-CN-4) :**
- [ ] Tests de cohérence sur toutes les réponses d'état
- [ ] Règles de priorité documentées et implémentées

**Flux (INV-CN-5, 6, 7, INV-FLUX-*) :**
- [ ] Traçabilité complète vérifiable
- [ ] Tests de performance (latence < seuil)
- [ ] Comparaison observation/propagation automatisée

**Autonomie (LOI-1 à LOI-6) :**
- [ ] Fonctionnement vérifié en mode offline
- [ ] État "offline" traité comme état normal
- [ ] Aucune dépendance externe obligatoire
- [ ] Ressources prévisibles et maîtrisées

### 12.3 Actions en cas de violation détectée

| Sévérité | Action immédiate | Délai de correction |
|----------|------------------|---------------------|
| CRIT | Blocage du déploiement, escalade | Avant mise en production |
| MAJ | Alerte, correction prioritaire | Avant release |
| MIN | Ticket de correction | Selon planning |
| WARN | Documentation, justification | Revue périodique |

---

## 13. Matrice de correspondance violations/invariants

| Code violation | Invariant(s) concerné(s) | Sévérité |
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

Ce document est **contractuel, normatif, et de statut GOUVERNANCE**. Il catalogue les violations et anti-patterns à éviter lors de l'implémentation ou de l'intégration de Caring Nanny.

Toute implémentation de Caring Nanny doit être vérifiée contre ce catalogue. Toute violation de sévérité CRIT bloque la mise en production. Les violations MAJ doivent être corrigées avant release.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** GOUVERNANCE — Catalogue normatif  
**Dépendances :**
- Caring Nanny - Documentation Fondatrice v1.6
- Caring Nanny - Invariants et Garanties v1.0
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1
