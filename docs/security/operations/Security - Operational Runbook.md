# Miyukini Security — Operational Runbook

## 1. Introduction

### Objet du document

Ce document constitue le **Security Operational Runbook** : un guide de procedures operationnelles pour les operateurs et administrateurs de l'ecosysteme Miyukini. Il fournit les actions concretes a executer selon l'etat du systeme, le niveau de confiance courant (T0-T4), et les incidents detectes.

Ce runbook traduit les concepts theoriques de la [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) et du [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) en procedures actionnables.

### Principe directeur

> **"Un operateur ne decide pas de la securite. Il observe l'etat du systeme et execute les procedures appropriees."**

L'operateur est guide par le systeme, pas l'inverse. Le runbook fournit les actions a executer, pas les decisions a prendre.

### Portee

Ce document definit :
- Les procedures par niveau de confiance (T0-T4)
- Les procedures par type d'incident
- Les procedures d'escalade vers TAMR
- Les checklists operateur
- Les actions de diagnostic et remediation
- Les procedures de retour a la normale

Ce document **ne couvre pas** :
- Les decisions architecturales (voir Architecture & Components)
- Les contraintes detaillees par niveau (voir Operational Constraints Contract)
- L'implementation technique des Security Engines

### Statut contractuel

Ce document est **contractuel, normatif, et de statut OPERATIONS**. Il etablit les procedures de reference que tout operateur doit suivre. Toute deviation non autorisee est une violation des procedures de securite.

### Destinataires

Ce runbook s'adresse a :
- **Operateurs systeme** : Surveillance quotidienne
- **Administrateurs** : Gestion des incidents
- **Equipes de support** : Escalade et intervention
- **Responsables securite** : Audit et conformite

---

## 2. Vue d'Ensemble des Etats Systeme

### 2.1 Les 5 Niveaux de Confiance (System Trust Levels)

Le systeme Miyukini fonctionne selon 5 niveaux de confiance, determines automatiquement par les Security Engines :

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    NIVEAUX DE CONFIANCE SYSTEME                              │
│                                                                             │
│   ┌─────────────────────────────────────────────────────────────────────┐   │
│   │  T0 - NORMAL     │ Systeme sain, toutes capacites disponibles       │   │
│   ├─────────────────────────────────────────────────────────────────────┤   │
│   │  T1 - INSTABLE   │ Anomalie detectee, log renforce                  │   │
│   ├─────────────────────────────────────────────────────────────────────┤   │
│   │  T2 - DEGRADE    │ Incoherence persistante, capacites reduites      │   │
│   ├─────────────────────────────────────────────────────────────────────┤   │
│   │  T3 - RESTREINT  │ Suspicion forte, gel partiel                     │   │
│   ├─────────────────────────────────────────────────────────────────────┤   │
│   │  T4 - BLOQUE     │ Integrite rompue, diagnostic uniquement          │   │
│   └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│   Direction : T0 ──▶ T1 ──▶ T2 ──▶ T3 ──▶ T4 (degradation)              │
│               T0 ◀── T1 ◀── T2 ◀── T3 ◀── T4 (restauration)             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Indicateurs Visuels MiyukiniAdmin

| Niveau | Couleur | Icone | Alerte |
|--------|---------|-------|--------|
| **T0** | Vert | ✅ | Aucune |
| **T1** | Jaune | ⚠️ | Information |
| **T2** | Orange | 🔶 | Avertissement |
| **T3** | Rouge | 🔴 | Critique |
| **T4** | Noir | ⛔ | Blocage |

### 2.3 Interactions avec les Niveaux de Securite

Les niveaux de confiance (T0-T4) sont **independants** des niveaux de securite declares (0-4) :

- **Niveaux de securite (0-4)** : Profil de risque de l'Operateur (fixe)
- **Niveaux de confiance (T0-T4)** : Etat d'integrite courant (dynamique)

**Combinaison** : La contrainte la plus restrictive s'applique toujours.

Voir : [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

---

## 3. Procedures par Niveau de Confiance

### 3.1 T0 — NORMAL

#### Etat du Systeme

```
┌─────────────────────────────────────────────────┐
│  T0 - NORMAL                          [✅]     │
│                                                 │
│  Statut : Systeme sain                         │
│  Integrite : Verifiee                          │
│  Capacites : Toutes disponibles                │
│  Monitoring : Standard                          │
│                                                 │
└─────────────────────────────────────────────────┘
```

#### Actions Operateur

**ACT-T0-1 : Surveillance Standard**

| Action | Frequence | Description |
|--------|-----------|-------------|
| Verification logs | Quotidienne | Examiner les logs Audit Engine |
| Verification integrite | Automatique | Confirmer le statut vert dans MiyukiniAdmin |
| Revue alertes | Continue | Aucune alerte attendue |

**ACT-T0-2 : Maintenance Preventive**

| Action | Frequence | Description |
|--------|-----------|-------------|
| Backup verification | Hebdomadaire | Verifier la validite des OSV |
| Capacity review | Mensuelle | Verifier les ressources systeme |
| Policy review | Trimestrielle | Auditer les politiques actives |

#### Checklist T0

```
□ Dashboard MiyukiniAdmin affiche T0 vert
□ Aucune alerte active
□ Logs Audit Engine sans erreur critique
□ Dernier backup OSV valide
□ Ressources systeme nominales
```

#### Actions Interdites en T0

Aucune action speciale requise. Operations normales autorisees.

---

### 3.2 T1 — INSTABLE

#### Etat du Systeme

```
┌─────────────────────────────────────────────────┐
│  T1 - INSTABLE                        [⚠️]     │
│                                                 │
│  Statut : Anomalie detectee                    │
│  Integrite : En verification                   │
│  Capacites : Toutes disponibles                │
│  Monitoring : Renforce                          │
│                                                 │
│  Log renforce : ACTIF                          │
│  Tracabilite : ETENDUE                         │
│                                                 │
└─────────────────────────────────────────────────┘
```

#### Declencheurs de T1

| Declencheur | Source | Description |
|-------------|--------|-------------|
| Anomalie unique | Integrity Engine | Hash mismatch isole |
| Erreur transitoire | Validation Engine | Validation echouee non reproductible |
| Signal externe suspect | Border Guard | Classification incertaine |
| Comportement inhabituel | Caring Nanny | Pattern non standard detecte |

#### Actions Operateur

**ACT-T1-1 : Surveillance Renforcee (Immediat)**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : SURVEILLANCE RENFORCEE T1                                      │
│                                                                            │
│  1. Ouvrir le dashboard MiyukiniAdmin                                      │
│     └── Section : Monitoring > System Trust Level                          │
│                                                                            │
│  2. Identifier la source de l'anomalie                                     │
│     └── Caring Nanny > Consolidated Signals                                │
│                                                                            │
│  3. Verifier les logs detailles                                            │
│     └── Audit Engine > Recent Events > Filter: anomaly                     │
│                                                                            │
│  4. Documenter l'observation                                               │
│     └── Operations Log > New Entry                                         │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

**ACT-T1-2 : Evaluation (< 15 minutes)**

| Etape | Action | Critere |
|-------|--------|---------|
| 1 | Identifier le type d'anomalie | Structure / Comportement / Environnement |
| 2 | Verifier la repetition | Unique / Multiple / Continu |
| 3 | Evaluer la correlation | Isole / Correle avec d'autres signaux |
| 4 | Determiner la persistance | Transitoire / Stable |

**ACT-T1-3 : Decision**

| Condition | Action | Resultat |
|-----------|--------|----------|
| Anomalie resolue spontanement | Documenter, continuer surveillance | Retour T0 |
| Anomalie isolee non reproductible | Documenter, monitoring 24h | Maintien T1 |
| Anomalie persistante ou reproductible | Escalader vers T2 | Passage T2 |
| Anomalie critique immediate | Escalader directement | Passage T2/T3 |

#### Checklist T1

```
□ Source de l'anomalie identifiee
□ Type d'anomalie classifie
□ Logs detailles examines
□ Observation documentee dans Operations Log
□ Decision prise et tracee
□ Surveillance renforcee confirmee active
```

#### Retour a T0

**Conditions de retour** :
- Anomalie resolue (manuellement ou spontanement)
- Aucune recurrence pendant 24h
- Verification d'integrite passee
- Validation par Caring Nanny

**Procedure** :

```
1. Confirmer resolution de l'anomalie
2. Executer verification integrite manuelle (Integrity Engine)
3. Attendre validation automatique
4. Documenter la resolution
```

---

### 3.3 T2 — DEGRADE

#### Etat du Systeme

```
┌─────────────────────────────────────────────────┐
│  T2 - DEGRADE                         [🔶]     │
│                                                 │
│  Statut : Incoherence persistante              │
│  Integrite : Partiellement compromise          │
│  Capacites : REDUITES                          │
│  Monitoring : Intensif                          │
│                                                 │
│  Extensions dynamiques : REFUSEES              │
│  Decisions : STRICTES                          │
│  MiyukiniAdmin : ALERTE VISIBLE                │
│                                                 │
└─────────────────────────────────────────────────┘
```

#### Declencheurs de T2

| Declencheur | Source | Description |
|-------------|--------|-------------|
| Anomalie persistante T1 | Caring Nanny | Non resolue apres delai |
| Violations multiples | Validation Engine | Echecs repetes de validation |
| Incoherence structurelle | Integrity Engine | Diff structurel confirme |
| Correlation d'anomalies | Caring Nanny | Plusieurs signaux correles |

#### Capacites Desactivees en T2

| Capacite | Statut | Justification |
|----------|--------|---------------|
| Extensions dynamiques | ❌ Refusees | Risque d'amplification |
| Nouveaux modules | ❌ Refuses | Surface d'attaque |
| Modifications en masse | ❌ Interdites | Integrite non garantie |
| Decisions automatiques critiques | ⚠️ Soumises a validation | Prudence accrue |

#### Actions Operateur

**ACT-T2-1 : Analyse Approfondie (Immediat)**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : ANALYSE APPROFONDIE T2                                         │
│                                                                            │
│  1. ISOLER la zone affectee                                                │
│     └── Identifier les Cores/Operateurs impactes                           │
│     └── Verifier l'etendue de l'incoherence                               │
│                                                                            │
│  2. DIAGNOSTIQUER la cause racine                                          │
│     └── Caring Nanny > Root Cause Approximation                            │
│     └── Audit Engine > Event Correlation                                   │
│                                                                            │
│  3. EVALUER l'impact                                                       │
│     └── Services affectes                                                  │
│     └── Donnees potentiellement compromises                                │
│     └── Utilisateurs impactes                                              │
│                                                                            │
│  4. DOCUMENTER completement                                                │
│     └── Operations Log > Incident Report                                   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

**ACT-T2-2 : Classification de Cause**

| Symptome | Interpretation Probable | Action Recommandee |
|----------|------------------------|---------------------|
| Anomalies aleatoires + memoire | Hardware defectueux | Diagnostic environnement |
| Invariant casse net | Modification de code | Audit code + Rollback |
| Comportement coherent mais interdit | Intrusion | Escalade T3 + Investigation |
| Erreurs transitoires | Bruit systeme | Monitoring renforce |

**ACT-T2-3 : Remediation**

| Type de Cause | Procedure | Delai Max |
|---------------|-----------|-----------|
| Hardware | ACT-T2-HW (voir section 6) | 4h |
| Code modifie | ACT-T2-CODE (voir section 6) | 2h |
| Erreurs transitoires | Surveillance continue | 24h |
| Cause inconnue | Escalade T3 | 1h |

#### Checklist T2

```
□ Zone affectee identifiee et isolee
□ Cause racine approximee
□ Impact evalue et documente
□ Incident Report cree
□ Remediation initiee ou escalade T3
□ Utilisateurs informes si necessaire
□ Monitoring intensif confirme actif
```

#### Communication

**Notifications obligatoires en T2** :

| Destinataire | Contenu | Delai |
|--------------|---------|-------|
| Equipe Operations | Alerte T2, zone affectee | Immediat |
| Responsable Securite | Resume incident | < 30 min |
| Utilisateurs affectes (si applicable) | Impact et mesures | < 1h |

#### Retour a T1/T0

**Conditions de retour a T1** :
- Cause racine identifiee
- Remediation en cours
- Pas de nouvelle anomalie
- Impact contenu

**Conditions de retour a T0** :
- Remediation complete
- Verification integrite passee
- Aucune anomalie pendant 48h
- Validation par Caring Nanny + StrongFather

---

### 3.4 T3 — RESTREINT

#### Etat du Systeme

```
┌─────────────────────────────────────────────────┐
│  T3 - RESTREINT                       [🔴]     │
│                                                 │
│  Statut : Suspicion forte                      │
│  Integrite : Potentiellement compromise        │
│  Capacites : MINIMALES                         │
│  Monitoring : Maximum                           │
│                                                 │
│  Operateurs non essentiels : GELES             │
│  Nouveaux modules : REFUSES                    │
│  Decisions critiques : DIFFEREES               │
│  TAMR : REQUIS pour override                   │
│                                                 │
└─────────────────────────────────────────────────┘
```

#### Declencheurs de T3

| Declencheur | Source | Description |
|-------------|--------|-------------|
| Suspicion d'intrusion | Border Guard + Caring Nanny | Comportement hostile detecte |
| Violation d'invariant | Integrity Engine | Invariant systeme viole |
| Echec remediation T2 | Operations | Cause non resolue |
| Correlation d'attaque | Cognitive Guard | Pattern d'attaque identifie |

#### Fonctionnement en T3

```
┌────────────────────────────────────────────────────────────────────────────┐
│                         MODE RESTREINT T3                                   │
│                                                                            │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  CAPACITES ACTIVES                                                   │   │
│  │  • Operations essentielles uniquement                               │   │
│  │  • Lecture autorisee                                                │   │
│  │  • Monitoring maximum                                               │   │
│  │  • Diagnostic complet                                               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                            │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  CAPACITES SUSPENDUES                                               │   │
│  │  • Ecritures non essentielles                                       │   │
│  │  • Extensions et nouveaux modules                                   │   │
│  │  • Decisions automatiques critiques                                 │   │
│  │  • Operateurs non essentiels                                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                            │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  INTERVENTION HUMAINE REQUISE                                        │   │
│  │  • TAMR disponible 24/7                                             │   │
│  │  • Override possible avec validation                                │   │
│  │  • Toute action tracee                                              │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

#### Actions Operateur

**ACT-T3-1 : Escalade Immediate (< 5 minutes)**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : ESCALADE T3                                                    │
│                                                                            │
│  ⚠️ PRIORITE MAXIMALE - AUCUNE ACTION SANS VALIDATION                      │
│                                                                            │
│  1. ALERTER l'equipe d'intervention                                        │
│     └── Appel immediat au responsable securite                             │
│     └── Notification TAMR activee                                          │
│                                                                            │
│  2. CONFIRMER l'etat T3                                                    │
│     └── Verifier le dashboard MiyukiniAdmin                                │
│     └── Confirmer les signaux declencheurs                                 │
│                                                                            │
│  3. ISOLER le systeme si necessaire                                        │
│     └── Couper les connexions externes non essentielles                    │
│     └── Activer le mode isolation Border Guard                             │
│                                                                            │
│  4. DOCUMENTER en temps reel                                               │
│     └── Incident Report en mode live                                       │
│     └── Chaque action tracee avec horodatage                               │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

**ACT-T3-2 : Investigation Securite**

| Etape | Action | Responsable | Validation |
|-------|--------|-------------|------------|
| 1 | Collecte logs complets | Operateur | Auto |
| 2 | Analyse forensique | Equipe Securite | TAMR |
| 3 | Identification de la menace | Equipe Securite | TAMR |
| 4 | Plan de remediation | Responsable Securite | TAMR |

**ACT-T3-3 : Decisions avec TAMR**

Toute decision critique en T3 **requiert validation TAMR** :

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : DECISION AVEC TAMR                                             │
│                                                                            │
│  1. Preparer la demande                                                    │
│     └── Description de l'action proposee                                   │
│     └── Justification                                                      │
│     └── Risques identifies                                                 │
│     └── Alternatives considerees                                           │
│                                                                            │
│  2. Soumettre a TAMR                                                       │
│     └── Via MiyukiniAdmin > TAMR > New Request                            │
│     └── Priorite : CRITIQUE                                                │
│                                                                            │
│  3. Attendre validation                                                    │
│     └── Delai max : 30 min                                                │
│     └── Si pas de reponse : Escalade superieure                           │
│                                                                            │
│  4. Executer si approuve                                                   │
│     └── Tracer l'autorisation TAMR                                         │
│     └── Executer l'action                                                  │
│     └── Documenter le resultat                                             │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

#### Checklist T3

```
□ Equipe d'intervention alertee
□ Responsable securite informe
□ TAMR active et disponible
□ Etat T3 confirme et documente
□ Systeme isole si necessaire
□ Logs complets collectes
□ Investigation en cours
□ Documentation en temps reel
□ Communication de crise preparee
```

#### Communication de Crise

**Notifications obligatoires en T3** :

| Destinataire | Contenu | Delai |
|--------------|---------|-------|
| Equipe d'intervention | Alerte T3, convocation | Immediat |
| Direction | Resume situation | < 15 min |
| Equipe juridique (si donnees compromises) | Briefing | < 30 min |
| Utilisateurs | Communication de crise | Selon decision TAMR |

#### Retour a T2/T1/T0

**Conditions de retour a T2** :
- Menace neutralisee ou non confirmee
- Remediation en cours avec plan valide
- Validation TAMR du retour

**Procedure de retour** :

```
1. Confirmer neutralisation/invalidation de la menace
2. Presenter rapport d'investigation a TAMR
3. Obtenir validation TAMR du retour
4. Executer verification integrite complete
5. Reactiver progressivement les capacites
6. Documenter la cloture de l'incident
```

---

### 3.5 T4 — BLOQUE

#### Etat du Systeme

```
┌─────────────────────────────────────────────────┐
│  T4 - BLOQUE                          [⛔]     │
│                                                 │
│  Statut : Integrite rompue                     │
│  Integrite : COMPROMISE                        │
│  Capacites : AUCUNE OPERATIONNELLE             │
│  Mode : DIAGNOSTIC UNIQUEMENT                   │
│                                                 │
│  ⛔ AUCUNE DECISION OPERATIONNELLE             │
│  ⛔ AUCUNE ECRITURE                            │
│  ⛔ AUCUNE EXECUTION NOUVELLE                  │
│                                                 │
│  ✅ Etat lisible                               │
│  ✅ Diagnostic autorise                        │
│  ✅ Sortie propre possible                     │
│                                                 │
└─────────────────────────────────────────────────┘
```

#### Declencheurs de T4

| Declencheur | Source | Description |
|-------------|--------|-------------|
| Confirmation de compromission | Investigation T3 | Intrusion confirmee |
| Violation critique d'integrite | Integrity Engine | Chaine de confiance rompue |
| Decision TAMR | Intervention humaine | Blocage preventif |
| Corruption detectee | Recovery Engine | Etat non recuperable |

#### Ce qui est Autorise en T4

| Action | Statut | Description |
|--------|--------|-------------|
| Lecture d'etat | ✅ Autorise | Observer l'etat du systeme |
| Collecte de logs | ✅ Autorise | Export pour forensique |
| Diagnostic | ✅ Autorise | Identifier l'etendue des degats |
| Sortie propre | ✅ Autorise | Shutdown ordonne |

#### Ce qui est Interdit en T4

| Action | Statut | Raison |
|--------|--------|--------|
| Toute ecriture | ❌ Interdit | Risque de propagation |
| Toute execution | ❌ Interdit | Code potentiellement compromis |
| Tentative de remediation | ❌ Interdit | Etat non fiable |
| Communication sortante | ❌ Interdit | Risque d'exfiltration |

#### Actions Operateur

**ACT-T4-1 : Constatation et Documentation (Immediat)**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : CONSTATATION T4                                                │
│                                                                            │
│  ⛔ SITUATION CRITIQUE - NE PAS TENTER DE REPARER                          │
│                                                                            │
│  1. CONFIRMER l'etat T4                                                    │
│     └── Dashboard MiyukiniAdmin (lecture seule)                            │
│     └── Verifier les signaux declencheurs                                  │
│                                                                            │
│  2. DOCUMENTER l'etat                                                      │
│     └── Screenshot/export de l'etat courant                                │
│     └── Export des logs disponibles                                        │
│     └── Horodatage precis                                                  │
│                                                                            │
│  3. ALERTER                                                                │
│     └── Appel immediat : Responsable Securite                              │
│     └── Appel immediat : Direction                                         │
│     └── Activation cellule de crise                                        │
│                                                                            │
│  4. ISOLER PHYSIQUEMENT si necessaire                                      │
│     └── Deconnexion reseau physique                                        │
│     └── Isolation du systeme                                               │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

**ACT-T4-2 : Cellule de Crise**

| Role | Responsabilite | Actions |
|------|----------------|---------|
| Responsable Securite | Direction des operations | Coordonne la reponse |
| Equipe Forensique | Investigation | Analyse de l'incident |
| TAMR | Decisions critiques | Valide toutes les actions |
| Communication | Relations externes | Gere la communication |
| Direction | Decisions business | Arbitrage final |

**ACT-T4-3 : Preparation Restauration**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : PREPARATION RESTAURATION                                       │
│                                                                            │
│  ⚠️ TOUTE RESTAURATION NECESSITE VALIDATION TAMR                           │
│                                                                            │
│  1. Identifier la derniere OSV valide                                      │
│     └── Verifier l'integrite de l'OSV                                      │
│     └── Confirmer la date et les donnees incluses                          │
│                                                                            │
│  2. Evaluer la perte de donnees                                            │
│     └── Donnees entre OSV et T4                                            │
│     └── Impact business                                                    │
│                                                                            │
│  3. Preparer l'environnement de restauration                               │
│     └── Environnement isole                                                │
│     └── Verification de l'integrite de l'environnement                     │
│                                                                            │
│  4. Presenter le plan a TAMR                                               │
│     └── OSV cible                                                          │
│     └── Perte de donnees acceptee                                          │
│     └── Procedure de restauration                                          │
│     └── Plan de verification post-restauration                             │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

#### Checklist T4

```
□ Etat T4 confirme et documente
□ Aucune tentative de remediation en cours
□ Logs exportes pour forensique
□ Systeme isole physiquement si necessaire
□ Cellule de crise activee
□ Direction informee
□ TAMR disponible et actif
□ OSV cible identifiee
□ Plan de restauration prepare
□ Perte de donnees evaluee et acceptee
```

#### Restauration depuis T4

**La restauration depuis T4 est une operation majeure** :

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : RESTAURATION DEPUIS T4                                         │
│                                                                            │
│  ⚠️ OPERATION IRREVERSIBLE - VALIDATION TAMR OBLIGATOIRE                   │
│                                                                            │
│  Phase 1 : Validation                                                      │
│  ─────────────────────                                                     │
│  □ Plan de restauration approuve par TAMR                                  │
│  □ OSV cible validee (integrite verifiee)                                  │
│  □ Perte de donnees acceptee formellement                                  │
│  □ Environnement de restauration pret                                      │
│                                                                            │
│  Phase 2 : Execution                                                       │
│  ───────────────────                                                       │
│  □ Shutdown propre du systeme T4                                           │
│  □ Restauration OSV sur environnement isole                                │
│  □ Verification integrite post-restauration                                │
│  □ Validation par Integrity Engine                                         │
│                                                                            │
│  Phase 3 : Reintegration                                                   │
│  ────────────────────────                                                  │
│  □ Tests fonctionnels complets                                             │
│  □ Verification securite complete                                          │
│  □ Validation TAMR de la mise en production                                │
│  □ Reactivation progressive des capacites                                  │
│                                                                            │
│  Phase 4 : Post-mortem                                                     │
│  ──────────────────────                                                    │
│  □ Rapport d'incident complet                                              │
│  □ Analyse des causes racines                                              │
│  □ Plan d'amelioration                                                     │
│  □ Cloture formelle de l'incident                                          │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Procedures par Type d'Incident

### 4.1 Incident : Violation d'Integrite

**Definition** : Ecart detecte entre l'etat actuel et l'etat certifie (STA/OSV).

#### Procedure

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : VIOLATION D'INTEGRITE                                          │
│                                                                            │
│  Niveau de confiance probable : T1 → T2 → T3                               │
│                                                                            │
│  1. IDENTIFIER l'ecart                                                     │
│     └── Integrity Engine > Diff Report                                     │
│     └── Quels fichiers/structures affectes ?                               │
│     └── Depuis quand ?                                                     │
│                                                                            │
│  2. CLASSIFIER l'ecart                                                     │
│     ┌─────────────────────────────────────────────────────────────────┐   │
│     │ Mineur : Fichier non critique, modification mineure             │   │
│     │ Modere : Fichier important, modification significative          │   │
│     │ Critique : Fichier systeme, invariant viole                     │   │
│     └─────────────────────────────────────────────────────────────────┘   │
│                                                                            │
│  3. AGIR selon classification                                              │
│     └── Mineur : Monitoring + Documentation                                │
│     └── Modere : Rollback fichier + Investigation                         │
│     └── Critique : Escalade T3 + Investigation securite                   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Incident : Comportement Suspect IA

**Definition** : Decision IA detectee comme potentiellement derivee ou biaisee.

#### Procedure

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : COMPORTEMENT SUSPECT IA                                        │
│                                                                            │
│  Niveau de confiance probable : T1 → T2                                    │
│  Source : Cognitive Guard                                                  │
│                                                                            │
│  1. IDENTIFIER l'agent et la decision                                      │
│     └── Cognitive Guard > Alert Details                                    │
│     └── Quel agent ? Quelle decision ? Quel contexte ?                    │
│                                                                            │
│  2. ANALYSER le pattern                                                    │
│     └── Decision isolee ou pattern recurrent ?                            │
│     └── Correlation avec d'autres agents ?                                │
│     └── Historique des decisions de l'agent                               │
│                                                                            │
│  3. AGIR selon analyse                                                     │
│     ┌─────────────────────────────────────────────────────────────────┐   │
│     │ Decision isolee : Monitoring renforce                           │   │
│     │ Pattern recurrent : Desactivation agent + Investigation        │   │
│     │ Correlation multi-agents : Escalade T2 + Audit complet         │   │
│     └─────────────────────────────────────────────────────────────────┘   │
│                                                                            │
│  4. DOCUMENTER                                                             │
│     └── Incident Report specifique IA                                      │
│     └── Decisions affectees                                                │
│     └── Actions correctives                                                │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 4.3 Incident : Intrusion Suspectee

**Definition** : Comportement coherent mais interdit, suggerant une intrusion.

#### Procedure

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : INTRUSION SUSPECTEE                                            │
│                                                                            │
│  Niveau de confiance probable : T2 → T3 → T4                               │
│  ⚠️ INCIDENT CRITIQUE - ESCALADE IMMEDIATE                                 │
│                                                                            │
│  1. ISOLER immediatement                                                   │
│     └── Couper les connexions externes                                     │
│     └── Activer Border Guard mode isolation                                │
│     └── Bloquer les sessions actives suspectes                             │
│                                                                            │
│  2. PRESERVER les preuves                                                  │
│     └── NE PAS MODIFIER l'etat                                             │
│     └── Export des logs complets                                           │
│     └── Snapshot de l'etat si possible                                     │
│                                                                            │
│  3. ESCALADER                                                              │
│     └── Notification TAMR immediate                                        │
│     └── Activation cellule de crise                                        │
│     └── Information direction                                              │
│                                                                            │
│  4. INVESTIGUER (sous controle TAMR)                                       │
│     └── Vecteur d'entree                                                   │
│     └── Etendue de la compromission                                        │
│     └── Donnees potentiellement exfiltrees                                 │
│     └── Actions de l'attaquant                                             │
│                                                                            │
│  5. DECIDER (TAMR)                                                         │
│     └── Remediation in-place vs Restauration OSV                           │
│     └── Communication externe                                              │
│     └── Obligations legales (notification CNIL si donnees personnelles)   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 4.4 Incident : Defaillance Hardware

**Definition** : Anomalies attribuees a un dysfonctionnement materiel.

#### Procedure

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : DEFAILLANCE HARDWARE                                           │
│                                                                            │
│  Niveau de confiance probable : T1 → T2                                    │
│  Source : Sondes Environnementales                                         │
│                                                                            │
│  1. CONFIRMER la nature hardware                                           │
│     └── Anomalies aleatoires (pas de pattern logiciel)                    │
│     └── Correlation avec metriques systeme (CPU, RAM, Disque)             │
│     └── Absence de pattern d'intrusion                                     │
│                                                                            │
│  2. IDENTIFIER le composant                                                │
│     └── Memoire : Erreurs aleatoires, corruption transitoire               │
│     └── Disque : Erreurs I/O, secteurs defectueux                         │
│     └── CPU : Instabilite calcul, temperature                              │
│                                                                            │
│  3. MITIGER                                                                │
│     └── Memoire : Redemarrage, test memoire                                │
│     └── Disque : Mode lecture seule, backup urgent                         │
│     └── CPU : Reduction charge, surveillance temperature                   │
│                                                                            │
│  4. PLANIFIER remplacement                                                 │
│     └── Evaluation urgence                                                 │
│     └── Planification maintenance                                          │
│     └── Communication utilisateurs                                         │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Procedures d'Escalade TAMR

### 5.1 Quand Escalader vers TAMR

| Situation | Niveau | Obligation TAMR |
|-----------|--------|-----------------|
| Passage en T3 | T3 | Obligatoire |
| Passage en T4 | T4 | Obligatoire |
| Intrusion suspectee | T2+ | Obligatoire |
| Decision critique en T2+ | T2+ | Requise |
| Override de securite | Tout | Obligatoire |
| Restauration OSV | Tout | Obligatoire |

### 5.2 Procedure d'Escalade Standard

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : ESCALADE TAMR STANDARD                                         │
│                                                                            │
│  1. PREPARER la demande                                                    │
│     ┌─────────────────────────────────────────────────────────────────┐   │
│     │ Contenu requis :                                                 │   │
│     │ • Contexte : Etat actuel du systeme                             │   │
│     │ • Declencheur : Evenement ayant mene a l'escalade               │   │
│     │ • Proposition : Action recommandee                               │   │
│     │ • Alternatives : Options considerees                             │   │
│     │ • Risques : Risques identifies pour chaque option               │   │
│     │ • Urgence : Niveau d'urgence (Critique/Haute/Normale)           │   │
│     └─────────────────────────────────────────────────────────────────┘   │
│                                                                            │
│  2. SOUMETTRE via MiyukiniAdmin                                            │
│     └── MiyukiniAdmin > TAMR > New Request                                │
│     └── Remplir le formulaire complet                                      │
│     └── Attacher les logs pertinents                                       │
│                                                                            │
│  3. ATTENDRE la decision                                                   │
│     └── Critique : Reponse attendue < 15 min                              │
│     └── Haute : Reponse attendue < 1h                                      │
│     └── Normale : Reponse attendue < 4h                                    │
│                                                                            │
│  4. EXECUTER si approuve                                                   │
│     └── Confirmer reception de l'autorisation                              │
│     └── Executer l'action autorisee                                        │
│     └── Documenter le resultat                                             │
│     └── Notifier TAMR de la completion                                     │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Escalade d'Urgence

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : ESCALADE TAMR URGENTE                                          │
│                                                                            │
│  ⚠️ A UTILISER UNIQUEMENT EN CAS DE DANGER IMMEDIAT                        │
│                                                                            │
│  1. APPEL TELEPHONIQUE immediat au responsable TAMR                        │
│     └── Numero d'astreinte : [Numero a completer]                          │
│     └── Identification : Nom, role, systeme                                │
│     └── Situation en une phrase                                            │
│                                                                            │
│  2. PENDANT l'appel                                                        │
│     └── Decrire la situation                                               │
│     └── Proposer l'action urgente                                          │
│     └── Obtenir autorisation verbale                                       │
│     └── Noter le nom de l'autorisateur                                     │
│                                                                            │
│  3. EXECUTER l'action urgente                                              │
│                                                                            │
│  4. DOCUMENTER immediatement apres                                         │
│     └── Creer la demande TAMR retroactive                                  │
│     └── Attacher l'autorisation verbale                                    │
│     └── Documenter l'action executee                                       │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 5.4 Si TAMR Ne Repond Pas

| Delai depasse | Action |
|---------------|--------|
| 15 min (Critique) | Escalade au niveau superieur |
| 1h (Haute) | Seconde notification + Escalade |
| 4h (Normale) | Rappel + Escalade si necessaire |

**Chaine d'escalade** :
1. Responsable TAMR principal
2. Responsable TAMR secondaire
3. Direction des operations
4. Direction generale

---

## 6. Procedures de Remediation

### 6.1 Rollback de Fichier

**ACT-T2-CODE : Rollback Code Modifie**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : ROLLBACK FICHIER                                               │
│                                                                            │
│  Prerequis : Violation d'integrite identifiee sur fichier(s) specifique(s)│
│                                                                            │
│  1. IDENTIFIER les fichiers affectes                                       │
│     └── Integrity Engine > Diff Report > Files                             │
│                                                                            │
│  2. VERIFIER la disponibilite de la version saine                         │
│     └── Recovery Engine > Available Snapshots                              │
│     └── Confirmer l'integrite de la version cible                          │
│                                                                            │
│  3. EXECUTER le rollback                                                   │
│     └── Recovery Engine > Rollback > Select Files                          │
│     └── Confirmer l'action                                                 │
│                                                                            │
│  4. VERIFIER le resultat                                                   │
│     └── Integrity Engine > Verify > Selected Files                         │
│     └── Confirmer le retour a l'etat sain                                  │
│                                                                            │
│  5. DOCUMENTER                                                             │
│     └── Fichiers restaures                                                 │
│     └── Version source                                                     │
│     └── Resultat de verification                                           │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Diagnostic Hardware

**ACT-T2-HW : Diagnostic Environnement**

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : DIAGNOSTIC HARDWARE                                            │
│                                                                            │
│  1. COLLECTER les metriques systeme                                        │
│     └── Caring Nanny > Environment Probes > Current State                  │
│     └── CPU, RAM, Disque, Temperature                                      │
│                                                                            │
│  2. ANALYSER les patterns d'erreur                                         │
│     └── Erreurs aleatoires = probable hardware                            │
│     └── Erreurs reproductibles = probable logiciel                        │
│                                                                            │
│  3. EXECUTER les tests diagnostiques                                       │
│     └── Test memoire                                                       │
│     └── Test disque                                                        │
│     └── Test CPU                                                           │
│                                                                            │
│  4. DETERMINER l'action                                                    │
│     └── Composant identifie : Planifier remplacement                       │
│     └── Aucun composant identifie : Surveillance renforcee                │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 6.3 Restauration OSV Complete

```
┌────────────────────────────────────────────────────────────────────────────┐
│  PROCEDURE : RESTAURATION OSV COMPLETE                                      │
│                                                                            │
│  ⚠️ OPERATION MAJEURE - VALIDATION TAMR OBLIGATOIRE                        │
│                                                                            │
│  Phase 1 : Preparation                                                     │
│  ─────────────────────────                                                 │
│  □ Confirmer l'impossibilite de remediation in-place                       │
│  □ Identifier la derniere OSV valide                                       │
│  □ Verifier l'integrite de l'OSV (checksums)                              │
│  □ Evaluer et documenter la perte de donnees                               │
│  □ Obtenir validation TAMR                                                 │
│                                                                            │
│  Phase 2 : Pre-restauration                                                │
│  ──────────────────────────                                                │
│  □ Backup de l'etat actuel (meme compromis) pour forensique               │
│  □ Export complet des logs                                                 │
│  □ Notification utilisateurs de l'indisponibilite                          │
│  □ Preparation de l'environnement de restauration                          │
│                                                                            │
│  Phase 3 : Restauration                                                    │
│  ──────────────────────                                                    │
│  □ Arret propre du systeme compromis                                       │
│  □ Restauration de l'OSV                                                   │
│  □ Verification integrite post-restauration                                │
│  □ Validation par Integrity Engine                                         │
│                                                                            │
│  Phase 4 : Post-restauration                                               │
│  ────────────────────────────                                              │
│  □ Tests fonctionnels complets                                             │
│  □ Verification securite                                                   │
│  □ Reactivation progressive des services                                   │
│  □ Notification utilisateurs du retour                                     │
│  □ Documentation complete de l'operation                                   │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Checklists Operateur

### 7.1 Checklist Quotidienne

```
┌────────────────────────────────────────────────────────────────────────────┐
│  CHECKLIST QUOTIDIENNE OPERATEUR                                            │
│                                                                            │
│  Heure recommandee : Debut de journee                                      │
│  Duree estimee : 15-30 minutes                                             │
│                                                                            │
│  VERIFICATION ETAT SYSTEME                                                 │
│  □ Dashboard MiyukiniAdmin affiche le niveau de confiance attendu          │
│  □ Aucune alerte non traitee                                               │
│  □ Tous les Cores rapportent un etat sain                                  │
│                                                                            │
│  VERIFICATION LOGS                                                         │
│  □ Audit Engine : Aucune erreur critique dans les 24h                      │
│  □ Integrity Engine : Aucune violation detectee                            │
│  □ Cognitive Guard : Aucune alerte IA                                      │
│                                                                            │
│  VERIFICATION RESSOURCES                                                   │
│  □ Espace disque suffisant                                                 │
│  □ Memoire disponible adequate                                             │
│  □ Performance CPU nominale                                                │
│                                                                            │
│  VERIFICATION BACKUPS                                                      │
│  □ Dernier backup complete avec succes                                     │
│  □ OSV disponible et valide                                                │
│                                                                            │
│  DOCUMENTATION                                                             │
│  □ Entree dans Operations Log : "Verification quotidienne OK" ou details  │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Checklist Hebdomadaire

```
┌────────────────────────────────────────────────────────────────────────────┐
│  CHECKLIST HEBDOMADAIRE OPERATEUR                                           │
│                                                                            │
│  Jour recommande : Lundi                                                   │
│  Duree estimee : 1-2 heures                                                │
│                                                                            │
│  REVUE DE LA SEMAINE PRECEDENTE                                            │
│  □ Revue des incidents de la semaine                                       │
│  □ Verification des actions correctives en cours                           │
│  □ Revue des alertes traitees                                              │
│                                                                            │
│  VERIFICATION APPROFONDIE                                                  │
│  □ Verification integrite complete (Integrity Engine > Full Scan)          │
│  □ Test de restauration OSV (environnement de test)                        │
│  □ Revue des permissions et capacites (Master Butler)                      │
│                                                                            │
│  MAINTENANCE                                                               │
│  □ Nettoyage des logs anciens (selon politique de retention)               │
│  □ Verification des certificats et signatures                              │
│  □ Mise a jour des listes de securite si necessaire                        │
│                                                                            │
│  RAPPORT                                                                   │
│  □ Rapport hebdomadaire operations                                         │
│  □ Metriques cles documentees                                              │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

### 7.3 Checklist Mensuelle

```
┌────────────────────────────────────────────────────────────────────────────┐
│  CHECKLIST MENSUELLE OPERATEUR                                              │
│                                                                            │
│  Jour recommande : Premier lundi du mois                                   │
│  Duree estimee : 4-8 heures                                                │
│                                                                            │
│  AUDIT COMPLET                                                             │
│  □ Audit securite complet (tous les Security Engines)                      │
│  □ Revue des politiques actives (Policy Engine)                            │
│  □ Verification des frontieres (Border Guard)                              │
│  □ Analyse des comportements IA du mois (Cognitive Guard)                  │
│                                                                            │
│  TEST DE RESILIENCE                                                        │
│  □ Test de restauration complete OSV (environnement de test)               │
│  □ Test de basculement (si applicable)                                     │
│  □ Verification des procedures d'escalade TAMR                             │
│                                                                            │
│  REVUE DES INCIDENTS                                                       │
│  □ Revue complete des incidents du mois                                    │
│  □ Analyse des causes racines                                              │
│  □ Verification des actions correctives                                    │
│  □ Identification des tendances                                            │
│                                                                            │
│  DOCUMENTATION ET REPORTING                                                │
│  □ Rapport mensuel securite                                                │
│  □ Mise a jour des procedures si necessaire                                │
│  □ Communication aux parties prenantes                                     │
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 8. Documentation Associee

### Documents de Reference (docs/reference)

| Document | Contenu |
|----------|---------|
| [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Principes fondateurs |
| [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux de securite (0-4) |
| [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | Niveaux de confiance (T0-T4) |
| [Security Protocols](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) | Protocoles de securite |

### Documents Securite (docs/security)

| Document | Contenu |
|----------|---------|
| [Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md) | Vision operationnelle |
| [Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md) | Les 8 Security Engines |
| [Security - Operational Constraints Contract](../contracts/operations/Security%20-%20Operational%20Constraints%20Contract.md) | Contraintes par niveau |
| [Security - Invariants & Guarantees](../contracts/governance/Security%20-%20Invariants%20&%20Guarantees.md) | Lois et garanties |

---

## 9. Conclusion

Ce runbook fournit les procedures operationnelles pour gerer la securite de l'ecosysteme Miyukini. Il garantit que :

- **Chaque niveau de confiance a ses procedures** : T0 a T4, actions claires
- **Chaque type d'incident a sa reponse** : Procedures standardisees
- **L'escalade TAMR est definie** : Quand, comment, vers qui
- **Les checklists guident les operateurs** : Quotidienne, hebdomadaire, mensuelle

**Principe final** :

> **"Un operateur ne decide pas de la securite. Il observe l'etat du systeme et execute les procedures appropriees. Le systeme guide, l'operateur agit."**

Ce runbook est de statut **OPERATIONS**. Toute deviation non documentee est une violation des procedures de securite.

---

**Date de creation :** 2026-01-28  
**Version :** 1.0  
**Statut :** OPERATIONS — Document procedural contractuel  
**Reference :** Miyukini Core System v2.4, [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

## 10. Mini Log de Generation

### Decisions structurantes

- Document oriente action avec procedures concretes
- Structure par niveau de confiance (T0-T4) pour navigation rapide
- Checklists actionnables pour les operateurs
- Integration des procedures d'escalade TAMR
- Diagrammes ASCII pour visualiser les etats et procedures

### Dependances critiques

- Integrity Degradation System : Definition des niveaux T0-T4
- Security Levels : Interaction niveaux de securite / niveaux de confiance
- Doctrine Securite Fondamentale : Principes directeurs
- Security - Operational Constraints Contract : Contraintes detaillees par niveau

### Verification de coherence

- ✅ Coherence avec les niveaux T0-T4 de l'Integrity Degradation System
- ✅ Coherence avec les contraintes de l'Operational Constraints Contract
- ✅ Coherence avec le role de TAMR dans la gouvernance
- ✅ Coherence avec les Security Engines et leurs responsabilites
- ✅ References correctes vers les documents sources

**Aucune contradiction detectee.**
