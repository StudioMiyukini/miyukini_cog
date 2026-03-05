# WorrySentinel - FAQ & Common Questions

## 1. Contexte

Ce document rÃ©pond aux **questions frÃ©quemment posÃ©es** sur WorrySentinel. Il clarifie les points de confusion courants et fournit des rÃ©ponses faisant autoritÃ© basÃ©es sur les contrats et la documentation fondatrice.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

## 2. Questions fondamentales

### Q1 : Qu'est-ce que WorrySentinel exactement ?

**RÃ©ponse :**

WorrySentinel est le **core de gouvernance de sÃ©curitÃ© transversale** du Miyukini Core System. Il reprÃ©sente la "volontÃ© sÃ©curitaire" du systÃ¨me : il dÃ©finit quels niveaux de sÃ©curitÃ© s'appliquent, quels Ã©tats de confiance sont acceptables, et comment la dÃ©gradation doit progresser.

**Ce que WorrySentinel EST :**
- Un gouvernant de la sÃ©curitÃ© (pas un exÃ©cuteur)
- Une pression verticale sur tous les cores
- L'autoritÃ© sur les niveaux de sÃ©curitÃ© et Ã©tats de confiance
- Un observateur et corrÃ©lateur de signaux

**Ce que WorrySentinel N'EST PAS :**
- Un systÃ¨me de sÃ©curitÃ© technique
- Un exÃ©cuteur de contrÃ´les
- Un persisteur de donnÃ©es
- Un core fonctionnel

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 2](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#2-dÃ©finition-de-worrysentinel)

---

### Q2 : Quelle est la diffÃ©rence entre WorrySentinel et StrongFather ?

**RÃ©ponse :**

La distinction est fondamentale et porte sur la sÃ©paration entre **gouvernance** et **exÃ©cution** :

| Aspect | WorrySentinel | StrongFather |
|--------|---------------|--------------|
| **RÃ´le** | Gouverne les niveaux et Ã©tats | Prend des dÃ©cisions |
| **Nature** | Conceptuel, dÃ©claratif | OpÃ©rationnel, exÃ©cutif |
| **Action** | DÃ©finit des contraintes | Applique des politiques |
| **ExÃ©cution** | Jamais | Toujours |
| **Strate** | 4 (Gouvernance) | 5 (Core fonctionnel) |

**Relation :**
- WorrySentinel gouverne la sÃ©vÃ©ritÃ© des dÃ©cisions de StrongFather
- StrongFather adapte ses politiques selon les contraintes de WorrySentinel
- WorrySentinel ne connaÃ®t pas les dÃ©tails des dÃ©cisions de StrongFather

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 9](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#9-relations-avec-les-autres-cores)

---

### Q3 : Pourquoi WorrySentinel existe-t-il ? Quel problÃ¨me rÃ©sout-il ?

**RÃ©ponse :**

WorrySentinel rÃ©sout le problÃ¨me de la **dispersion de la gouvernance de sÃ©curitÃ©** :

**Sans WorrySentinel :**
- Chaque composant dÃ©finit ses propres niveaux de sÃ©curitÃ©
- Les rÃ¨gles de gouvernance sont dupliquÃ©es et incohÃ©rentes
- Pas de vision globale de l'Ã©tat de sÃ©curitÃ©
- DÃ©gradation non coordonnÃ©e

**Avec WorrySentinel :**
- Gouvernance centralisÃ©e et cohÃ©rente
- Niveaux de sÃ©curitÃ© uniformes (0-4)
- Ã‰tats de confiance globaux (T0-T4)
- DÃ©gradation progressive orchestrÃ©e
- TraÃ§abilitÃ© complÃ¨te

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 3](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#3-pourquoi-worrysentinel-existe)

---

### Q4 : Comment WorrySentinel interagit-il avec les autres cores ?

**RÃ©ponse :**

WorrySentinel interagit selon deux flux :

**Flux descendant (gouvernance) :**
WorrySentinel impose des contraintes aux cores :

| Core | Contrainte imposÃ©e |
|------|-------------------|
| StrongFather | SÃ©vÃ©ritÃ© des dÃ©cisions |
| MasterButler | Permissions actives |
| BorderGuard | Durcissement frontiÃ¨res |
| CaringNanny | IntensitÃ© monitoring |
| LogisticsSteward | Durcissement quotas |
| TAMR | Droits intervention humaine |
| Kernel | FrÃ©quence sondes |

**Flux montant (observation) :**
WorrySentinel reÃ§oit des signaux des cores :

| Core | Signaux remontÃ©s |
|------|-----------------|
| Kernel | Signaux clock, id, traces |
| StrongFather | DÃ©cisions refusÃ©es |
| BorderGuard | Anomalies I/O |
| CaringNanny | Anomalies monitoring |
| KindMother | IncohÃ©rences donnÃ©es |
| LogisticsSteward | DÃ©rives allocation |

**RÃ©fÃ©rence :** [Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md)

---

## 3. Questions sur les niveaux de sÃ©curitÃ©

### Q5 : Comment choisir le bon niveau de sÃ©curitÃ© pour mon produit ?

**RÃ©ponse :**

Le niveau de sÃ©curitÃ© est dÃ©terminÃ© par le **profil de risque** de votre produit :

| Si votre produit... | Niveau recommandÃ© |
|--------------------|-------------------|
| Affiche uniquement des donnÃ©es publiques | **0 â€” Public** |
| GÃ¨re du contenu Ã©ditorial simple | **1 â€” Standard** |
| Manipule des donnÃ©es personnelles | **2 â€” Sensitive** |
| GÃ¨re l'authentification ou les paiements | **3 â€” Critical** |
| Fonctionne en environnement hostile | **4 â€” Hardened** |

**RÃ¨gle :** Le niveau dÃ©clarÃ© doit correspondre au profil de risque rÃ©el. DÃ©clarer un niveau infÃ©rieur est une violation (INTERD-SEC-1).

**RÃ©fÃ©rence :** [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

### Q6 : Peut-on changer de niveau de sÃ©curitÃ© pendant l'exÃ©cution ?

**RÃ©ponse :**

**Non.** Le niveau de sÃ©curitÃ© est immuable pendant toute la durÃ©e d'une opÃ©ration (RÃˆGLE-SEC-4).

**Pourquoi ?**
- CohÃ©rence des contraintes pendant l'opÃ©ration
- PrÃ©vention des attaques par changement de niveau
- TraÃ§abilitÃ© fiable

**Quand peut-on changer ?**
- Entre deux opÃ©rations
- Avec justification explicite
- Avec validation par BorderGuard et StrongFather
- Avec traÃ§abilitÃ© complÃ¨te

**RÃ©fÃ©rence :** [Security Levels Governance Contract - Section 4.4](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#44-rÃ¨gle-sec-4--immuabilitÃ©-opÃ©rationnelle)

---

### Q7 : Que se passe-t-il si un composant de niveau 1 essaie d'accÃ©der Ã  un composant de niveau 3 ?

**RÃ©ponse :**

L'accÃ¨s direct est **interdit** selon INV-GOV-6 (CohÃ©rence inter-composants).

**Matrice d'accÃ¨s :**

| Source \ Cible | N0 | N1 | N2 | N3 | N4 |
|----------------|----|----|----|----|----| 
| **N1** | âœ… | âœ… | âŒ | âŒ | âŒ |

**Solution :** Une **mÃ©diation explicite** est requise, gouvernÃ©e par WorrySentinel et validÃ©e par StrongFather.

**RÃ©fÃ©rence :** [Security Levels Governance Contract - Section 4.3](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#43-rÃ¨gle-sec-3--cohÃ©rence-inter-composants)

---

## 4. Questions sur les Ã©tats de confiance

### Q8 : Comment le systÃ¨me passe-t-il d'un Ã©tat de confiance Ã  un autre ?

**RÃ©ponse :**

Les transitions sont gouvernÃ©es par des rÃ¨gles explicites et suivent une progression :

**Transitions autorisÃ©es :**

```
T0 â†â†’ T1 â†â†’ T2 â†â†’ T3 â†’ T4
```

**RÃ¨gles clÃ©s :**
- **Progression uniquement par Ã©tapes** : Pas de saut T0â†’T4
- **Justification obligatoire** : Chaque transition est tracÃ©e
- **T4 est terminal** : Aucune transition sortante de T4

**DÃ©clencheurs de transition :**
- T0â†’T1 : DÃ©tection d'anomalie
- T1â†’T2 : Persistance d'anomalie
- T2â†’T3 : Aggravation de l'Ã©tat
- T3â†’T4 : Confirmation de compromission

**RÃ©fÃ©rence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

### Q9 : Qui peut modifier l'Ã©tat de confiance du systÃ¨me ?

**RÃ©ponse :**

**Personne ne "modifie" directement** l'Ã©tat de confiance. WorrySentinel **dÃ©clare** l'Ã©tat basÃ© sur la corrÃ©lation des signaux.

**Processus :**
1. Les cores remontent des signaux d'intÃ©gritÃ©
2. WorrySentinel observe et corrÃ¨le les signaux
3. WorrySentinel Ã©value la nÃ©cessitÃ© d'une transition
4. WorrySentinel dÃ©clare le nouvel Ã©tat (si transition)
5. Les cores adaptent leur comportement

**Violations :**
- Modifier l'Ã©tat directement â†’ VIOL-GOV-1
- Transition sans justification â†’ Violation INV-GOV-3

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 7](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#7-Ã©tats-de-confiance-du-systÃ¨me)

---

### Q10 : Comment revenir de T4 (BloquÃ©) Ã  un Ã©tat normal ?

**RÃ©ponse :**

**T4 est un Ã©tat terminal.** Aucune transition sortante n'est autorisÃ©e.

**Pourquoi ?**
- T4 indique que l'intÃ©gritÃ© du systÃ¨me est rompue
- La confiance ne peut pas Ãªtre restaurÃ©e automatiquement
- Une intervention humaine majeure est requise

**Actions possibles en T4 :**
- Diagnostics uniquement
- Lecture de l'Ã©tat
- Sortie propre

**Restauration :**
- Intervention humaine (TAMR ou administrateur)
- Analyse forensique
- RÃ©initialisation du systÃ¨me
- Nouveau cycle de confiance

**RÃ©fÃ©rence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

## 5. Questions sur les invariants

### Q11 : Pourquoi WorrySentinel ne peut-il pas implÃ©menter de contrÃ´les de sÃ©curitÃ© ?

**RÃ©ponse :**

C'est l'invariant **INV-WS-1** : Aucune autoritÃ© sur l'implÃ©mentation.

**Raisons :**
1. **SÃ©paration des responsabilitÃ©s** : La gouvernance dÃ©finit QUOI, pas COMMENT
2. **Ã‰volutivitÃ©** : L'implÃ©mentation peut changer sans modifier la gouvernance
3. **ClartÃ© architecturale** : Pas de confusion entre gouvernant et exÃ©cuteur
4. **TestabilitÃ©** : Gouvernance et implÃ©mentation sont testables sÃ©parÃ©ment

**ConsÃ©quence :**
- WorrySentinel dÃ©finit les contraintes
- Les cores fonctionnels implÃ©mentent les contrÃ´les

**RÃ©fÃ©rence :** [Invariants & Guarantees - INV-WS-1](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#41-inv-ws-1--aucune-autoritÃ©-sur-limplÃ©mentation)

---

### Q12 : Qu'est-ce que le "zero-trust" de WorrySentinel ?

**RÃ©ponse :**

Le **zero-trust** (INV-WS-6) signifie que WorrySentinel ne fait confiance Ã  aucun appelant.

**ConcrÃ¨tement :**
- Chaque demande est Ã©valuÃ©e selon les rÃ¨gles
- Aucune confiance prÃ©supposÃ©e
- Aucun privilÃ¨ge par dÃ©faut
- Aucun contournement pour appelant "de confiance"

**Application :**
- Validation de toutes les entrÃ©es
- VÃ©rification du contexte Ã  chaque interaction
- Application des contraintes sans exception

**RÃ©fÃ©rence :** [Invariants & Guarantees - INV-WS-6](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#52-inv-ws-6--zero-trust)

---

### Q13 : Que signifie "aucune modification d'Ã©tat" (INV-WS-4) ?

**RÃ©ponse :**

WorrySentinel **gouverne** et **dÃ©clare**, mais ne **modifie** jamais l'Ã©tat du systÃ¨me.

**Distinction :**

| Action | AutorisÃ©e | Exemple |
|--------|-----------|---------|
| DÃ©clarer un Ã©tat cible | âœ… | "L'Ã©tat cible est T2" |
| DÃ©finir des rÃ¨gles de transition | âœ… | "T1â†’T2 si anomalie persiste" |
| Modifier directement un Ã©tat | âŒ | `self.state = T2` |
| CrÃ©er/supprimer un fait | âŒ | `create_fact(...)` |

**Pourquoi ?**
- WorrySentinel est un gouvernant conceptuel, pas un acteur opÃ©rationnel
- La modification d'Ã©tat est la responsabilitÃ© des cores fonctionnels

**RÃ©fÃ©rence :** [Invariants & Guarantees - INV-WS-4](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md#44-inv-ws-4--aucune-modification-dÃ©tat)

---

## 6. Questions pratiques

### Q14 : Comment configurer WorrySentinel via MiyukiniAdmin ?

**RÃ©ponse :**

MiyukiniAdmin peut :

**Consulter :**
- Niveaux de sÃ©curitÃ© des produits
- Ã‰tat de confiance courant
- Historique des transitions
- Contraintes applicables

**Configurer (via StrongFather) :**
- Attribution de niveaux de sÃ©curitÃ©
- RÃ¨gles de transition personnalisÃ©es
- RÃ¨gles de dÃ©gradation

**Restriction :** Toute configuration passe par StrongFather pour validation (RÃˆGLE-ADMIN-1).

**RÃ©fÃ©rence :** [MiyukiniAdmin Integration Contract](../contracts/integration/WorrySentinel%20-%20MiyukiniAdmin%20Integration%20Contract.md)

---

### Q15 : Comment WorrySentinel gÃ¨re-t-il le mode offline ?

**RÃ©ponse :**

WorrySentinel fonctionne de maniÃ¨re **autonome** en mode offline :

**Ce qui fonctionne :**
- Gouvernance des niveaux de sÃ©curitÃ© locaux
- Gestion des Ã©tats de confiance locaux
- DÃ©gradation progressive locale
- TraÃ§abilitÃ© locale

**Ce qui change :**
- Pas de synchronisation avec le cloud
- Pas de signaux externes
- RÃ©conciliation Ã  la reconnexion

**Principe :** WorrySentinel ne nÃ©cessite pas de connexion Internet permanente (LOI-1).

**RÃ©fÃ©rence :** [Documentation Fondatrice - Section 10](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md#10-ce-que-worrysentinel-permet-et-ne-change-pas)

---

### Q16 : WorrySentinel impacte-t-il les performances ?

**RÃ©ponse :**

**En Ã©tat nominal (T0) :** Impact quasi nul.

**Selon le niveau de sÃ©curitÃ© :**

| Niveau | Impact performance |
|--------|-------------------|
| 0 | ðŸŸ¢ Quasi nul |
| 1 | ðŸŸ¢ Faible |
| 2 | ðŸŸ¡ ModÃ©rÃ© |
| 3 | ðŸŸ  AcceptÃ© |
| 4 | ðŸ”´ Secondaire |

**Principe :** L'impact performance est proportionnel au profil de risque. Un produit de niveau 4 accepte que la performance soit secondaire par rapport Ã  la sÃ©curitÃ©.

**RÃ©fÃ©rence :** [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)

---

## 7. Questions avancÃ©es

### Q17 : Comment WorrySentinel distingue-t-il une panne hardware d'une intrusion ?

**RÃ©ponse :**

WorrySentinel utilise la **corrÃ©lation de signaux** et l'**heuristique de cause probable** :

| SymptÃ´me | InterprÃ©tation probable |
|----------|------------------------|
| Anomalies alÃ©atoires + mÃ©moire | Hardware dÃ©fectueux |
| Invariant cassÃ© net | Modification de code |
| Comportement cohÃ©rent mais interdit | Intrusion |
| Erreurs transitoires | Bruit systÃ¨me |

**Processus :**
1. Sondes dÃ©tectent des anomalies
2. CaringNanny consolide les signaux
3. WorrySentinel corrÃ¨le les patterns
4. ProbabilitÃ© dominante dÃ©terminÃ©e
5. DÃ©cision de dÃ©gradation adaptÃ©e

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Integrity Degradation System](..//..//..//miyukini-webway-system//reference//_index.md)

---

### Q18 : Quelle est la relation entre WorrySentinel et les Tools/Toolkits ?

**RÃ©ponse :**

WorrySentinel gouverne la sÃ©curitÃ© des Tools en dÃ©finissant :

| ResponsabilitÃ© | Description |
|----------------|-------------|
| Niveau de sÃ©curitÃ© par Tool | Chaque Tool a un niveau requis |
| Blocage en Ã©tat dÃ©gradÃ© | Certains Tools bloquÃ©s en T2+ |
| Audit | Tous les appels de Tools sont auditables |

**Question fondamentale :**
> "Le niveau de sÃ©curitÃ© actuel permet-il cet appel de Tool ?"

**RÃ©fÃ©rence :** [Security Levels Governance Contract - Section 6](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md#6-gouvernance-de-sÃ©curitÃ©-des-tools-et-toolkits)

---

### Q19 : Comment Ã©tendre WorrySentinel ?

**RÃ©ponse :**

WorrySentinel peut Ãªtre Ã©tendu **uniquement** aux points dÃ©finis :

**Extensible :**
- Nouveaux signaux d'intÃ©gritÃ©
- Nouvelles rÃ¨gles de corrÃ©lation
- Nouveaux types de contraintes
- Nouvelles mÃ©triques d'observation

**Non extensible (figÃ©) :**
- Nombre de niveaux de sÃ©curitÃ© (0-4)
- Nombre d'Ã©tats de confiance (T0-T4)
- Nature transversale
- SÃ©paration gouvernance/implÃ©mentation
- Flux descendant et montant

**RÃ©fÃ©rence :** [Architecture & Flows - Section 12](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md#12-points-dextension-et-non-extension)

---

## 8. Questions de dÃ©pannage

### Q20 : Mon produit est bloquÃ© en T3, que faire ?

**RÃ©ponse :**

**Ã‰tat T3 (Restreint) signifie :** Suspicion forte, gel des produits non essentiels.

**Actions :**
1. Consulter les signaux ayant dÃ©clenchÃ© T3
2. Analyser les anomalies dÃ©tectÃ©es
3. Demander intervention TAMR si nÃ©cessaire
4. RÃ©soudre les anomalies identifiÃ©es
5. Attendre la transition T3â†’T2 (confirmation de sÃ©curitÃ©)

**Si bloquÃ© :**
- Contacter l'administrateur (MiyukiniAdmin)
- Demander un override TAMR (avec justification)
- Analyser les logs de traÃ§abilitÃ©

**RÃ©fÃ©rence :** [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)

---

### Q21 : Comment diagnostiquer une violation d'invariant ?

**RÃ©ponse :**

**SymptÃ´mes de violation :**
- Comportement incohÃ©rent du systÃ¨me
- DÃ©cisions inexplicables
- Transitions d'Ã©tat non justifiÃ©es
- TraÃ§abilitÃ© incomplÃ¨te

**Diagnostic :**
1. Identifier l'invariant potentiellement violÃ©
2. VÃ©rifier les logs de traÃ§abilitÃ©
3. Analyser le flux de gouvernance
4. Identifier le composant fautif
5. Corriger immÃ©diatement

**Invariants les plus courants Ã  vÃ©rifier :**
- INV-WS-1 : Code d'implÃ©mentation dans WorrySentinel ?
- INV-WS-4 : Modification d'Ã©tat directe ?
- INV-GOV-4 : Transition brutale ?
- INV-GOV-6 : AccÃ¨s inter-niveaux non mÃ©diÃ© ?

**RÃ©fÃ©rence :** [Violations & Anti-Patterns](../contracts/governance/WorrySentinel%20-%20Violations%20&%20Anti-Patterns.md)

---

## 9. RÃ©fÃ©rences

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | RÃ©ponses officielles |
| [Invariants & Guarantees](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Questions sur les invariants |
| [Security Levels Governance Contract](../contracts/levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md) | Questions sur les niveaux |
| [Trust States Governance Contract](../contracts/levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md) | Questions sur les Ã©tats |
| [Architecture & Flows](../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md) | Questions sur l'architecture |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** RÃ©fÃ©rence â€” Questions frÃ©quentes  
**Type :** FAQ et clarifications

