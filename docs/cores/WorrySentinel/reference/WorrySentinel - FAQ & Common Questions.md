# WorrySentinel - FAQ & Common Questions

## 1. Contexte

Ce document répond aux **questions fréquemment posées** sur WorrySentinel. Il clarifie les points de confusion courants et fournit des réponses faisant autorité basées sur les contrats et la documentation fondatrice.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

## 2. Questions fondamentales

### Q1 : Qu'est-ce que WorrySentinel exactement ?

**Réponse :**

WorrySentinel est le **core de gouvernance de sécurité transversale** du Miyukini Core System. Il représente la "volonté sécuritaire" du système : il définit quels niveaux de sécurité s'appliquent, quels états de confiance sont acceptables, et comment la dégradation doit progresser.

**Ce que WorrySentinel EST :**
- Un gouvernant de la sécurité (pas un exécuteur)
- Une pression verticale sur tous les cores
- L'autorité sur les niveaux de sécurité et états de confiance
- Un observateur et corrélateur de signaux

**Ce que WorrySentinel N'EST PAS :**
- Un système de sécurité technique
- Un exécuteur de contrôles
- Un persisteur de données
- Un core fonctionnel

**Référence :** [Documentation Fondatrice - Section 2](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#2-définition-de-worrysentinel)

---

### Q2 : Quelle est la différence entre WorrySentinel et StrongFather ?

**Réponse :**

La distinction est fondamentale et porte sur la séparation entre **gouvernance** et **exécution** :

| Aspect | WorrySentinel | StrongFather |
|--------|---------------|--------------|
| **Rôle** | Gouverne les niveaux et états | Prend des décisions |
| **Nature** | Conceptuel, déclaratif | Opérationnel, exécutif |
| **Action** | Définit des contraintes | Applique des politiques |
| **Exécution** | Jamais | Toujours |
| **Strate** | 4 (Gouvernance) | 5 (Core fonctionnel) |

**Relation :**
- WorrySentinel gouverne la sévérité des décisions de StrongFather
- StrongFather adapte ses politiques selon les contraintes de WorrySentinel
- WorrySentinel ne connaît pas les détails des décisions de StrongFather

**Référence :** [Documentation Fondatrice - Section 9](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#9-relations-avec-les-autres-cores)

---

### Q3 : Pourquoi WorrySentinel existe-t-il ? Quel problème résout-il ?

**Réponse :**

WorrySentinel résout le problème de la **dispersion de la gouvernance de sécurité** :

**Sans WorrySentinel :**
- Chaque composant définit ses propres niveaux de sécurité
- Les règles de gouvernance sont dupliquées et incohérentes
- Pas de vision globale de l'état de sécurité
- Dégradation non coordonnée

**Avec WorrySentinel :**
- Gouvernance centralisée et cohérente
- Niveaux de sécurité uniformes (0-4)
- États de confiance globaux (T0-T4)
- Dégradation progressive orchestrée
- Traçabilité complète

**Référence :** [Documentation Fondatrice - Section 3](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#3-pourquoi-worrysentinel-existe)

---

### Q4 : Comment WorrySentinel interagit-il avec les autres cores ?

**Réponse :**

WorrySentinel interagit selon deux flux :

**Flux descendant (gouvernance) :**
WorrySentinel impose des contraintes aux cores :

| Core | Contrainte imposée |
|------|-------------------|
| StrongFather | Sévérité des décisions |
| MasterButler | Permissions actives |
| BorderGuard | Durcissement frontières |
| CaringNanny | Intensité monitoring |
| LogisticsSteward | Durcissement quotas |
| TAMR | Droits intervention humaine |
| Kernel | Fréquence sondes |

**Flux montant (observation) :**
WorrySentinel reçoit des signaux des cores :

| Core | Signaux remontés |
|------|-----------------|
| Kernel | Signaux clock, id, traces |
| StrongFather | Décisions refusées |
| BorderGuard | Anomalies I/O |
| CaringNanny | Anomalies monitoring |
| KindMother | Incohérences données |
| LogisticsSteward | Dérives allocation |

**Référence :** [Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md)

---

## 3. Questions sur les niveaux de sécurité

### Q5 : Comment choisir le bon niveau de sécurité pour mon produit ?

**Réponse :**

Le niveau de sécurité est déterminé par le **profil de risque** de votre produit :

| Si votre produit... | Niveau recommandé |
|--------------------|-------------------|
| Affiche uniquement des données publiques | **0 — Public** |
| Gère du contenu éditorial simple | **1 — Standard** |
| Manipule des données personnelles | **2 — Sensitive** |
| Gère l'authentification ou les paiements | **3 — Critical** |
| Fonctionne en environnement hostile | **4 — Hardened** |

**Règle :** Le niveau déclaré doit correspondre au profil de risque réel. Déclarer un niveau inférieur est une violation (INTERD-SEC-1).

**Référence :** [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

### Q6 : Peut-on changer de niveau de sécurité pendant l'exécution ?

**Réponse :**

**Non.** Le niveau de sécurité est immuable pendant toute la durée d'une opération (RÈGLE-SEC-4).

**Pourquoi ?**
- Cohérence des contraintes pendant l'opération
- Prévention des attaques par changement de niveau
- Traçabilité fiable

**Quand peut-on changer ?**
- Entre deux opérations
- Avec justification explicite
- Avec validation par BorderGuard et StrongFather
- Avec traçabilité complète

**Référence :** [Security Levels Governance Contract - Section 4.4](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#44-règle-sec-4--immuabilité-opérationnelle)

---

### Q7 : Que se passe-t-il si un composant de niveau 1 essaie d'accéder à un composant de niveau 3 ?

**Réponse :**

L'accès direct est **interdit** selon INV-GOV-6 (Cohérence inter-composants).

**Matrice d'accès :**

| Source \ Cible | N0 | N1 | N2 | N3 | N4 |
|----------------|----|----|----|----|----| 
| **N1** | ✅ | ✅ | ❌ | ❌ | ❌ |

**Solution :** Une **médiation explicite** est requise, gouvernée par WorrySentinel et validée par StrongFather.

**Référence :** [Security Levels Governance Contract - Section 4.3](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#43-règle-sec-3--cohérence-inter-composants)

---

## 4. Questions sur les états de confiance

### Q8 : Comment le système passe-t-il d'un état de confiance à un autre ?

**Réponse :**

Les transitions sont gouvernées par des règles explicites et suivent une progression :

**Transitions autorisées :**

```
T0 ←→ T1 ←→ T2 ←→ T3 → T4
```

**Règles clés :**
- **Progression uniquement par étapes** : Pas de saut T0→T4
- **Justification obligatoire** : Chaque transition est tracée
- **T4 est terminal** : Aucune transition sortante de T4

**Déclencheurs de transition :**
- T0→T1 : Détection d'anomalie
- T1→T2 : Persistance d'anomalie
- T2→T3 : Aggravation de l'état
- T3→T4 : Confirmation de compromission

**Référence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

### Q9 : Qui peut modifier l'état de confiance du système ?

**Réponse :**

**Personne ne "modifie" directement** l'état de confiance. WorrySentinel **déclare** l'état basé sur la corrélation des signaux.

**Processus :**
1. Les cores remontent des signaux d'intégrité
2. WorrySentinel observe et corrèle les signaux
3. WorrySentinel évalue la nécessité d'une transition
4. WorrySentinel déclare le nouvel état (si transition)
5. Les cores adaptent leur comportement

**Violations :**
- Modifier l'état directement → VIOL-GOV-1
- Transition sans justification → Violation INV-GOV-3

**Référence :** [Documentation Fondatrice - Section 7](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#7-états-de-confiance-du-système)

---

### Q10 : Comment revenir de T4 (Bloqué) à un état normal ?

**Réponse :**

**T4 est un état terminal.** Aucune transition sortante n'est autorisée.

**Pourquoi ?**
- T4 indique que l'intégrité du système est rompue
- La confiance ne peut pas être restaurée automatiquement
- Une intervention humaine majeure est requise

**Actions possibles en T4 :**
- Diagnostics uniquement
- Lecture de l'état
- Sortie propre

**Restauration :**
- Intervention humaine (TAMR ou administrateur)
- Analyse forensique
- Réinitialisation du système
- Nouveau cycle de confiance

**Référence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

## 5. Questions sur les invariants

### Q11 : Pourquoi WorrySentinel ne peut-il pas implémenter de contrôles de sécurité ?

**Réponse :**

C'est l'invariant **INV-WS-1** : Aucune autorité sur l'implémentation.

**Raisons :**
1. **Séparation des responsabilités** : La gouvernance définit QUOI, pas COMMENT
2. **Évolutivité** : L'implémentation peut changer sans modifier la gouvernance
3. **Clarté architecturale** : Pas de confusion entre gouvernant et exécuteur
4. **Testabilité** : Gouvernance et implémentation sont testables séparément

**Conséquence :**
- WorrySentinel définit les contraintes
- Les cores fonctionnels implémentent les contrôles

**Référence :** [Invariants & Guarantees - INV-WS-1](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#41-inv-ws-1--aucune-autorité-sur-limplémentation)

---

### Q12 : Qu'est-ce que le "zero-trust" de WorrySentinel ?

**Réponse :**

Le **zero-trust** (INV-WS-6) signifie que WorrySentinel ne fait confiance à aucun appelant.

**Concrètement :**
- Chaque demande est évaluée selon les règles
- Aucune confiance présupposée
- Aucun privilège par défaut
- Aucun contournement pour appelant "de confiance"

**Application :**
- Validation de toutes les entrées
- Vérification du contexte à chaque interaction
- Application des contraintes sans exception

**Référence :** [Invariants & Guarantees - INV-WS-6](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#52-inv-ws-6--zero-trust)

---

### Q13 : Que signifie "aucune modification d'état" (INV-WS-4) ?

**Réponse :**

WorrySentinel **gouverne** et **déclare**, mais ne **modifie** jamais l'état du système.

**Distinction :**

| Action | Autorisée | Exemple |
|--------|-----------|---------|
| Déclarer un état cible | ✅ | "L'état cible est T2" |
| Définir des règles de transition | ✅ | "T1→T2 si anomalie persiste" |
| Modifier directement un état | ❌ | `self.state = T2` |
| Créer/supprimer un fait | ❌ | `create_fact(...)` |

**Pourquoi ?**
- WorrySentinel est un gouvernant conceptuel, pas un acteur opérationnel
- La modification d'état est la responsabilité des cores fonctionnels

**Référence :** [Invariants & Guarantees - INV-WS-4](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#44-inv-ws-4--aucune-modification-détat)

---

## 6. Questions pratiques

### Q14 : Comment configurer WorrySentinel via MiyukiniAdmin ?

**Réponse :**

MiyukiniAdmin peut :

**Consulter :**
- Niveaux de sécurité des produits
- État de confiance courant
- Historique des transitions
- Contraintes applicables

**Configurer (via StrongFather) :**
- Attribution de niveaux de sécurité
- Règles de transition personnalisées
- Règles de dégradation

**Restriction :** Toute configuration passe par StrongFather pour validation (RÈGLE-ADMIN-1).

**Référence :** [MiyukiniAdmin Integration Contract](../contracts/integration/WorrySentinel%20-%20MiyukiniAdmin%20Integration%20Contract.md)

---

### Q15 : Comment WorrySentinel gère-t-il le mode offline ?

**Réponse :**

WorrySentinel fonctionne de manière **autonome** en mode offline :

**Ce qui fonctionne :**
- Gouvernance des niveaux de sécurité locaux
- Gestion des états de confiance locaux
- Dégradation progressive locale
- Traçabilité locale

**Ce qui change :**
- Pas de synchronisation avec le cloud
- Pas de signaux externes
- Réconciliation à la reconnexion

**Principe :** WorrySentinel ne nécessite pas de connexion Internet permanente (LOI-1).

**Référence :** [Documentation Fondatrice - Section 10](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#10-ce-que-worrysentinel-permet-et-ne-change-pas)

---

### Q16 : WorrySentinel impacte-t-il les performances ?

**Réponse :**

**En état nominal (T0) :** Impact quasi nul.

**Selon le niveau de sécurité :**

| Niveau | Impact performance |
|--------|-------------------|
| 0 | 🟢 Quasi nul |
| 1 | 🟢 Faible |
| 2 | 🟡 Modéré |
| 3 | 🟠 Accepté |
| 4 | 🔴 Secondaire |

**Principe :** L'impact performance est proportionnel au profil de risque. Un produit de niveau 4 accepte que la performance soit secondaire par rapport à la sécurité.

**Référence :** [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

## 7. Questions avancées

### Q17 : Comment WorrySentinel distingue-t-il une panne hardware d'une intrusion ?

**Réponse :**

WorrySentinel utilise la **corrélation de signaux** et l'**heuristique de cause probable** :

| Symptôme | Interprétation probable |
|----------|------------------------|
| Anomalies aléatoires + mémoire | Hardware défectueux |
| Invariant cassé net | Modification de code |
| Comportement cohérent mais interdit | Intrusion |
| Erreurs transitoires | Bruit système |

**Processus :**
1. Sondes détectent des anomalies
2. CaringNanny consolide les signaux
3. WorrySentinel corrèle les patterns
4. Probabilité dominante déterminée
5. Décision de dégradation adaptée

**Référence :** [Miyukini Conceptual References - Integrity Degradation System](../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)

---

### Q18 : Quelle est la relation entre WorrySentinel et les Tools/Toolkits ?

**Réponse :**

WorrySentinel gouverne la sécurité des Tools en définissant :

| Responsabilité | Description |
|----------------|-------------|
| Niveau de sécurité par Tool | Chaque Tool a un niveau requis |
| Blocage en état dégradé | Certains Tools bloqués en T2+ |
| Audit | Tous les appels de Tools sont auditables |

**Question fondamentale :**
> "Le niveau de sécurité actuel permet-il cet appel de Tool ?"

**Référence :** [Security Levels Governance Contract - Section 6](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#6-gouvernance-de-sécurité-des-tools-et-toolkits)

---

### Q19 : Comment étendre WorrySentinel ?

**Réponse :**

WorrySentinel peut être étendu **uniquement** aux points définis :

**Extensible :**
- Nouveaux signaux d'intégrité
- Nouvelles règles de corrélation
- Nouveaux types de contraintes
- Nouvelles métriques d'observation

**Non extensible (figé) :**
- Nombre de niveaux de sécurité (0-4)
- Nombre d'états de confiance (T0-T4)
- Nature transversale
- Séparation gouvernance/implémentation
- Flux descendant et montant

**Référence :** [Architecture & Flows - Section 12](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md#12-points-dextension-et-non-extension)

---

## 8. Questions de dépannage

### Q20 : Mon produit est bloqué en T3, que faire ?

**Réponse :**

**État T3 (Restreint) signifie :** Suspicion forte, gel des produits non essentiels.

**Actions :**
1. Consulter les signaux ayant déclenché T3
2. Analyser les anomalies détectées
3. Demander intervention TAMR si nécessaire
4. Résoudre les anomalies identifiées
5. Attendre la transition T3→T2 (confirmation de sécurité)

**Si bloqué :**
- Contacter l'administrateur (MiyukiniAdmin)
- Demander un override TAMR (avec justification)
- Analyser les logs de traçabilité

**Référence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

### Q21 : Comment diagnostiquer une violation d'invariant ?

**Réponse :**

**Symptômes de violation :**
- Comportement incohérent du système
- Décisions inexplicables
- Transitions d'état non justifiées
- Traçabilité incomplète

**Diagnostic :**
1. Identifier l'invariant potentiellement violé
2. Vérifier les logs de traçabilité
3. Analyser le flux de gouvernance
4. Identifier le composant fautif
5. Corriger immédiatement

**Invariants les plus courants à vérifier :**
- INV-WS-1 : Code d'implémentation dans WorrySentinel ?
- INV-WS-4 : Modification d'état directe ?
- INV-GOV-4 : Transition brutale ?
- INV-GOV-6 : Accès inter-niveaux non médié ?

**Référence :** [Violations & Anti-Patterns](../contracts/governance/WorrySentinel%20-%20Violations%20&%20Anti-Patterns.md)

---

## 9. Références

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Réponses officielles |
| [Invariants & Guarantees](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Questions sur les invariants |
| [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md) | Questions sur les niveaux |
| [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md) | Questions sur les états |
| [Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md) | Questions sur l'architecture |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Référence — Questions fréquentes  
**Type :** FAQ et clarifications
