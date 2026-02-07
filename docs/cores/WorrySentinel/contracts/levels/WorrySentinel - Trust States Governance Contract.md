# WorrySentinel — Trust States Governance Contract

## 1. Introduction

### Objet du contrat

Ce document définit le **WorrySentinel — Trust States Governance Contract** : un contrat normatif, non négociable, et de statut FONDATION qui établit les règles absolues de gouvernance des états de confiance système (T0-T4), leurs définitions, leurs transitions, et leur impact sur l'écosystème Miyukini Core System v2.4.

Ce contrat précise la nature conceptuelle des états de confiance, les règles de transition, les capacités associées à chaque état, et les garanties de gouvernance, sans jamais introduire de détail d'implémentation technique, de mécanisme de détection concret, ou de contrôle algorithmique.

### Portée

Ce contrat s'applique à **toutes les opérations impliquant des états de confiance** dans WorrySentinel et définit de manière absolue :
- la définition formelle de chaque état de confiance (T0-T4),
- les règles de transition entre états,
- les capacités et restrictions associées à chaque état,
- les invariants de gouvernance des états de confiance,
- les garanties offertes aux composants et produits,
- la distinction entre états de confiance et niveaux de sécurité.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il établit des règles absolues qui ne peuvent être contournées, négociées, ou modifiées. Le contrat prime sur toute considération pratique.

### Relation avec les autres contrats

Ce contrat complète et respecte les documents contractuels existants :
- **WorrySentinel — Documentation Fondatrice** : Source des définitions conceptuelles des états T0-T4
- **WorrySentinel — Security Levels Governance Contract** : Contrat jumeau pour les niveaux de sécurité (0-4)
- **WorrySentinel — Invariants & Guarantees** : Catalogue consolidé des invariants WorrySentinel
- **[Miyukini Conceptual References - Integrity Degradation System](../../../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)** : Référence conceptuelle du système de dégradation
- **CaringNanny — Documentation Fondatrice** : Core responsable de la consolidation des signaux d'intégrité
- **StrongFather — Documentation Fondatrice** : Core responsable des décisions selon les états de confiance
- **TAMR — Documentation Fondatrice** : Mécanisme d'intervention humaine en états dégradés

Il n'introduit aucune contradiction et constitue la définition formelle de la gouvernance des états de confiance dans WorrySentinel.

---

## 2. Distinction états de confiance et niveaux de sécurité

### 2.1. Deux dimensions indépendantes

WorrySentinel gouverne deux dimensions indépendantes mais complémentaires :

| Dimension | Objet | Valeurs | Portée |
|-----------|-------|---------|--------|
| **États de confiance** | Intégrité du système | T0-T4 | Globale (écosystème) |
| **Niveaux de sécurité** | Profil de risque | 0-4 | Locale (produit/composant) |

**RÈGLE-DIST-1 : Indépendance conceptuelle**

Les états de confiance et les niveaux de sécurité sont **conceptuellement indépendants**. Un système peut être en état T0 (normal) tout en gérant des données de niveau 4 (sécurité maximale).

**RÈGLE-DIST-2 : Cumul des restrictions**

Les restrictions sont **cumulatives** : un produit de niveau de sécurité élevé en état de confiance dégradé cumule les restrictions des deux dimensions.

### 2.2. Questions auxquelles chaque dimension répond

**États de confiance (ce contrat) :**
> *"Quel est l'état d'intégrité du système ? Le système est-il sain ?"*

**Niveaux de sécurité (Security Levels Governance Contract) :**
> *"Quel est le profil de risque de ce produit/composant ? Quelle sensibilité des données ?"*

---

## 3. Définition des états de confiance

### 3.1. Principe fondamental

Les états de confiance (System Trust Levels) caractérisent l'**intégrité globale du système**. Ils sont :
- **Globaux** : Applicables à tout l'écosystème, pas à un composant isolé
- **Exclusifs** : Le système est dans un et un seul état à tout instant
- **Progressifs** : La dégradation est progressive, jamais brutale
- **Gouvernés** : WorrySentinel définit les règles, mais n'évalue pas directement

### 3.2. Échelle des états de confiance

| État | Nom | Signification | Correspondance globale |
|------|-----|---------------|------------------------|
| **T0** | Normal | Système sain, aucune anomalie | 🟢 Nominal |
| **T1** | Instable | Anomalie détectée, non confirmée | 🟡 Doute |
| **T2** | Dégradé | Incohérence persistante | 🟠 Suspect |
| **T3** | Restreint | Suspicion forte, intégrité menacée | 🔴 Critique |
| **T4** | Bloqué | Intégrité rompue, système compromis | ⛔ Compromis |

### 3.3. Définition détaillée de T0 — Normal

**État conceptuel :** Système sain, aucune anomalie détectée.

**Caractéristiques :**
- ✅ Toutes les capacités disponibles
- ✅ Décisions normales
- ✅ Extensions dynamiques autorisées
- ✅ Monitoring standard

**Capacités autorisées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ✅ Autorisées |
| Extensions dynamiques | ✅ Autorisées |
| Nouveaux modules | ✅ Autorisés |
| Décisions critiques | ✅ Normales |
| Fonctions sensibles | ✅ Disponibles |

**Contraintes :**
- Aucune contrainte additionnelle

**Indicateur de sortie :**
- Détection d'une anomalie → Transition vers T1

### 3.4. Définition détaillée de T1 — Instable

**État conceptuel :** Anomalie détectée, mais pas encore confirmée.

**Caractéristiques :**
- ✅ Log renforcé
- ✅ Traçabilité étendue
- ✅ Aucun blocage opérationnel
- ✅ Surveillance accrue

**Capacités autorisées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ✅ Autorisées |
| Extensions dynamiques | ✅ Autorisées avec traçabilité |
| Nouveaux modules | ✅ Autorisés avec traçabilité |
| Décisions critiques | ✅ Normales avec log renforcé |
| Fonctions sensibles | ✅ Disponibles avec surveillance |

**Contraintes :**
- **C-T1-1** : Toutes les opérations sont tracées de manière étendue
- **C-T1-2** : Les logs sont renforcés (niveau de détail accru)
- **C-T1-3** : Surveillance accrue des patterns comportementaux

**Indicateurs de sortie :**
- Anomalie résolue → Transition vers T0
- Anomalie persistante → Transition vers T2

### 3.5. Définition détaillée de T2 — Dégradé

**État conceptuel :** Incohérence persistante, suspicion modérée.

**Caractéristiques :**
- ✅ Certaines capacités désactivées
- ✅ Décisions plus strictes
- ❌ Refus des extensions dynamiques
- ✅ Monitoring visible (MiyukiniAdmin)

**Capacités autorisées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ✅ Autorisées (fonctions non essentielles) |
| Extensions dynamiques | ❌ Refusées |
| Nouveaux modules | ❌ Refusés |
| Décisions critiques | ⚠️ Strictes (seuils abaissés) |
| Fonctions sensibles | ⚠️ Bridées |

**Contraintes :**
- **C-T2-1** : Extensions dynamiques bloquées
- **C-T2-2** : Nouveaux modules refusés
- **C-T2-3** : Seuils de décision abaissés (plus de refus)
- **C-T2-4** : Monitoring visible dans MiyukiniAdmin
- **C-T2-5** : Fonctions non essentielles potentiellement désactivées

**Indicateurs de sortie :**
- Amélioration de l'état → Transition vers T1
- Aggravation de l'état → Transition vers T3

### 3.6. Définition détaillée de T3 — Restreint

**État conceptuel :** Suspicion forte, intégrité potentiellement compromise.

**Caractéristiques :**
- ✅ Gel des produits non essentiels
- ❌ Refus de nouveaux modules
- ⚠️ Décisions critiques → AMBIGUË / DIFFÉRÉE
- ✅ TAMR requis pour override

**Capacités autorisées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ⚠️ Mode minimal uniquement |
| Extensions dynamiques | ❌ Refusées |
| Nouveaux modules | ❌ Refusés |
| Décisions critiques | ⚠️ AMBIGUË ou DIFFÉRÉE (TAMR requis) |
| Fonctions sensibles | ❌ Bloquées |
| Produits non essentiels | ❌ Gelés |

**Contraintes :**
- **C-T3-1** : Gel des produits non essentiels
- **C-T3-2** : Mode minimal uniquement pour les produits essentiels
- **C-T3-3** : Décisions critiques nécessitent validation TAMR
- **C-T3-4** : Fonctions sensibles bloquées
- **C-T3-5** : Aucune nouvelle intégration
- **C-T3-6** : Audit continu obligatoire

**Indicateurs de sortie :**
- Confirmation de sécurité (via TAMR) → Transition vers T2
- Confirmation de compromission → Transition vers T4

### 3.7. Définition détaillée de T4 — Bloqué

**État conceptuel :** Intégrité rompue, système compromis.

**Caractéristiques :**
- ❌ Plus aucune décision opérationnelle
- ✅ Uniquement diagnostics
- ✅ État lisible
- ✅ Sortie propre possible

**Capacités autorisées :**
| Capacité | Statut |
|----------|--------|
| Opérations normales | ❌ Bloquées |
| Extensions dynamiques | ❌ Bloquées |
| Nouveaux modules | ❌ Bloqués |
| Décisions critiques | ❌ Bloquées |
| Fonctions sensibles | ❌ Bloquées |
| Diagnostics | ✅ Autorisés |
| Lecture d'état | ✅ Autorisée |
| Sortie propre | ✅ Autorisée |

**Contraintes :**
- **C-T4-1** : Aucune opération métier autorisée
- **C-T4-2** : Uniquement diagnostics et lecture d'état
- **C-T4-3** : Sortie propre (shutdown graceful) autorisée
- **C-T4-4** : Aucune corruption autorisée (invariant préservé)
- **C-T4-5** : Aucune exécution sauvage (invariant préservé)

**Indicateurs de sortie :**
- **État terminal** : Aucune transition sortante automatique
- Intervention humaine requise pour résolution

**📌 Garantie absolue :** Jamais de corruption. Jamais d'exécution sauvage.

---

## 4. Règles de transition entre états

### 4.1. Matrice des transitions autorisées

| De → Vers | T0 | T1 | T2 | T3 | T4 |
|-----------|----|----|----|----|----| 
| **T0** | — | ✅ | ❌ | ❌ | ❌ |
| **T1** | ✅ | — | ✅ | ❌ | ❌ |
| **T2** | ❌ | ✅ | — | ✅ | ❌ |
| **T3** | ❌ | ❌ | ✅ | — | ✅ |
| **T4** | ❌ | ❌ | ❌ | ❌ | — |

### 4.2. Règles de transition

**RÈGLE-TRANS-1 : Progression séquentielle**

Les transitions vers un état plus dégradé sont **séquentielles**. Le système ne peut jamais sauter d'états :
- T0 → T1 → T2 → T3 → T4 (dégradation)
- T4 → T3 → T2 → T1 → T0 (amélioration, avec intervention)

**RÈGLE-TRANS-2 : Irréversibilité relative**

Les transitions vers un état plus dégradé sont **irréversibles sans intervention explicite**. Une fois en T2, le système ne peut pas revenir directement en T0.

**RÈGLE-TRANS-3 : Justification obligatoire**

Toute transition entre états DOIT être justifiée avec :
- La raison de la transition
- Les signaux ayant déclenché la transition
- Le contexte de la décision
- L'horodatage de la transition

**RÈGLE-TRANS-4 : Traçabilité complète**

Toute transition DOIT être tracée de manière complète et immutable.

**RÈGLE-TRANS-5 : T4 terminal**

L'état T4 est **terminal**. Aucune transition sortante n'est autorisée sans intervention humaine explicite hors du système.

### 4.3. Conditions de transition

**TRANS-T0-T1 : Détection d'anomalie**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Anomalie détectée par les sondes d'intégrité |
| Confirmation | Aucune confirmation requise (observation directe) |
| Réversibilité | Immédiate si anomalie résolue |

**TRANS-T1-T0 : Résolution d'anomalie**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Anomalie résolue, signaux revenus à la normale |
| Confirmation | Période d'observation sans nouvelle anomalie |
| Réversibilité | Directe |

**TRANS-T1-T2 : Persistance d'anomalie**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Anomalie persistante, incohérence confirmée |
| Confirmation | Consolidation par CaringNanny |
| Réversibilité | Via amélioration vers T1 |

**TRANS-T2-T1 : Amélioration de l'état**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Amélioration des indicateurs, incohérences réduites |
| Confirmation | Consolidation par CaringNanny |
| Réversibilité | Directe |

**TRANS-T2-T3 : Aggravation de l'état**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Aggravation significative, suspicion forte |
| Confirmation | Consolidation par CaringNanny, évaluation StrongFather |
| Réversibilité | Via amélioration vers T2 avec validation TAMR |

**TRANS-T3-T2 : Confirmation de sécurité**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Suspicion infirmée, confirmation de sécurité |
| Confirmation | Validation explicite via TAMR |
| Réversibilité | Directe (après validation TAMR) |

**TRANS-T3-T4 : Confirmation de compromission**

| Condition | Description |
|-----------|-------------|
| Déclencheur | Compromission confirmée, intégrité rompue |
| Confirmation | Évaluation StrongFather avec preuves consolidées |
| Réversibilité | Intervention humaine hors système uniquement |

---

## 5. Gouvernance des états par WorrySentinel

### 5.1. Rôle de WorrySentinel

WorrySentinel **gouverne** les états de confiance mais ne les **évalue** pas directement :

| Responsabilité | WorrySentinel | Autres cores |
|----------------|---------------|--------------|
| Définition des états | ✅ | ❌ |
| Règles de transition | ✅ | ❌ |
| Capacités par état | ✅ | ❌ |
| Détection d'anomalies | ❌ | CaringNanny (consolidation) |
| Décision de transition | ❌ | StrongFather (évaluation) |
| Intervention humaine | ❌ | TAMR (override) |

### 5.2. Flux de gouvernance des états

```
Sondes d'intégrité (Kernel)
         │
         ▼
CaringNanny (consolidation des signaux)
         │
         ▼
StrongFather (évaluation, décision de transition)
         │
         ▼
WorrySentinel (gouvernance : règles, capacités, restrictions)
         │
         ▼
Tous les cores et produits (application des restrictions)
```

**Principe :** WorrySentinel définit les règles. Les autres cores les appliquent.

### 5.3. Règles de gouvernance

**GOV-TS-1 : État unique global**

Le système possède un et un seul état de confiance à tout moment. L'état est **global** à l'écosystème.

**GOV-TS-2 : Obligation de conformité**

Tous les cores fonctionnels et produits DOIVENT respecter les capacités et restrictions définies pour l'état courant.

**GOV-TS-3 : Propagation immédiate**

Tout changement d'état DOIT être propagé immédiatement à tous les composants concernés.

**GOV-TS-4 : Non-ignorabilité**

Aucun produit, aucun core ne peut ignorer l'état de confiance courant. L'adaptation au comportement selon l'état est **obligatoire**.

**GOV-TS-5 : Préservation des invariants**

Même en état T4 (Bloqué), les invariants FONDATION DOIVENT être préservés.

---

## 6. Interaction avec les autres cores

### 6.1. CaringNanny — Consolidation des signaux

**Rôle :** Consolider les signaux d'intégrité pour proposer des transitions d'état.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| CaringNanny → WorrySentinel | Propose des transitions basées sur les signaux consolidés |
| WorrySentinel → CaringNanny | Fournit les règles de seuils et de consolidation |

**Règle d'interaction :**
- CaringNanny consolide, WorrySentinel gouverne les règles de seuils
- CaringNanny propose, StrongFather décide

### 6.2. StrongFather — Décision de transition

**Rôle :** Décider des transitions d'état selon les signaux consolidés et les politiques.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| StrongFather ← CaringNanny | Reçoit les signaux consolidés |
| StrongFather → Système | Décide de la transition d'état |
| WorrySentinel → StrongFather | Fournit les règles de transition |

**Règle d'interaction :**
- StrongFather décide selon les règles définies par WorrySentinel
- StrongFather ne peut pas créer de nouvelles règles de transition

### 6.3. TAMR — Intervention humaine

**Rôle :** Permettre l'intervention humaine pour les transitions sensibles (T3 → T2, sortie de T4).

**Interactions :**
| Direction | Description |
|-----------|-------------|
| TAMR → StrongFather | Autorise les overrides validés |
| WorrySentinel → TAMR | Définit les conditions d'intervention |

**Règle d'interaction :**
- En T3+, TAMR est requis pour certaines décisions critiques
- TAMR trace toutes les interventions

### 6.4. BondingBrother — Médiateur observable

**Rôle :** Transporter les informations d'état vers les produits sans interpréter.

**Interactions :**
| Direction | Description |
|-----------|-------------|
| WorrySentinel → BondingBrother | Communique l'état courant |
| BondingBrother → Produits | Rend l'état visible aux produits |

**Règle d'interaction :**
- BondingBrother ne décide jamais
- BondingBrother transporte et rend visible

### 6.5. LogisticsSteward — Durcissement des quotas

**Rôle :** Adapter les règles d'arbitrage de ressources selon l'état de confiance.

**Interactions :**
| État | Impact sur LogisticsSteward |
|------|----------------------------|
| T0 | Quotas normaux |
| T1 | Quotas normaux avec monitoring |
| T2+ | Quotas restrictifs selon directives WorrySentinel |

**Règle d'interaction :**
- WorrySentinel impose des contraintes sécuritaires sur les décisions d'arbitrage
- LogisticsSteward adapte ses quotas selon l'état de confiance

---

## 7. Invariants de gouvernance des états de confiance

### 7.1. Invariants d'état

**INV-TS-1 : Unicité d'état**

Le système possède exactement un état de confiance à tout moment. Aucune superposition d'états n'est autorisée.

**INV-TS-2 : Complétude de l'échelle**

L'échelle T0-T4 est exhaustive. Aucun état intermédiaire ou additionnel n'existe.

**INV-TS-3 : Exclusivité des états**

Les cinq états sont mutuellement exclusifs. Le système ne peut pas être simultanément dans deux états différents.

### 7.2. Invariants de transition

**INV-TS-4 : Séquentialité des transitions**

Les transitions sont séquentielles. Aucun saut d'état n'est autorisé (T0 → T3 interdit).

**INV-TS-5 : Justification obligatoire**

Toute transition est justifiée et tracée. Aucune transition silencieuse n'est autorisée.

**INV-TS-6 : T4 terminal**

L'état T4 est terminal. Aucune transition sortante automatique n'est possible.

### 7.3. Invariants de gouvernance

**INV-TS-7 : Non-ignorabilité**

Aucun composant ne peut ignorer l'état de confiance courant.

**INV-TS-8 : Préservation des invariants FONDATION**

Même en T4, les invariants FONDATION sont préservés. Le système ne corrompt jamais.

**INV-TS-9 : WorrySentinel ne détecte pas**

WorrySentinel gouverne les règles mais ne détecte jamais directement. La détection est du ressort de CaringNanny et des sondes.

**INV-TS-10 : WorrySentinel ne décide pas des transitions**

WorrySentinel définit les règles de transition mais ne décide jamais des transitions. La décision appartient à StrongFather.

---

## 8. Garanties offertes

### 8.1. Garanties de gouvernance

**G-TS-1 : Cohérence d'état**

WorrySentinel garantit que l'état de confiance est cohérent à travers tout l'écosystème.

**G-TS-2 : Propagation immédiate**

WorrySentinel garantit que tout changement d'état est propagé immédiatement.

**G-TS-3 : Traçabilité complète**

WorrySentinel garantit que toute transition est tracée avec justification.

### 8.2. Garanties de dégradation

**G-TS-4 : Dégradation progressive**

WorrySentinel garantit que la dégradation est toujours progressive, jamais brutale.

**G-TS-5 : Capacités préservées en T0-T1**

En états T0 et T1, toutes les capacités opérationnelles sont préservées.

**G-TS-6 : Diagnostics toujours disponibles**

Même en T4, les capacités de diagnostic et de lecture d'état restent disponibles.

### 8.3. Garanties de protection

**G-TS-7 : Pas de corruption en T4**

En état T4, le système ne corrompt jamais les données.

**G-TS-8 : Pas d'exécution sauvage**

En état T4, aucune exécution non contrôlée ne se produit.

**G-TS-9 : Sortie propre toujours possible**

En tout état, une sortie propre (shutdown graceful) reste possible.

---

## 9. Violations et comportements interdits

### 9.1. Violations d'état

**VIOL-TS-1 : États multiples**

Un composant déclare ou gère plusieurs états simultanément.

*Violation :* INV-TS-1, INV-TS-3

**VIOL-TS-2 : Saut d'état**

Une transition saute un état intermédiaire (ex: T0 → T3).

*Violation :* INV-TS-4

**VIOL-TS-3 : Transition silencieuse**

Une transition se produit sans justification ni traçabilité.

*Violation :* INV-TS-5

### 9.2. Violations de gouvernance

**VIOL-TS-4 : Ignorance d'état**

Un composant ignore l'état de confiance courant et maintient un comportement nominal.

*Violation :* INV-TS-7

**VIOL-TS-5 : Détection par WorrySentinel**

WorrySentinel détecte directement une anomalie au lieu de gouverner les règles.

*Violation :* INV-TS-9

**VIOL-TS-6 : Décision de transition par WorrySentinel**

WorrySentinel décide directement d'une transition au lieu de définir les règles.

*Violation :* INV-TS-10

### 9.3. Comportements interdits

**INTERD-TS-1 : Création d'états**

Aucun composant ne peut créer de nouveaux états de confiance.

**INTERD-TS-2 : Modification de l'échelle**

L'échelle T0-T4 ne peut pas être modifiée, étendue, ou réduite.

**INTERD-TS-3 : Sortie automatique de T4**

Aucune sortie automatique de l'état T4 n'est autorisée.

**INTERD-TS-4 : Contournement des capacités**

Aucun composant ne peut contourner les restrictions de capacités liées à un état.

---

## 10. Règles de fermeture du contrat

### 10.1. Contrat fermé

Ce contrat est **fermé**. Seules les définitions d'états, règles de transition, capacités, invariants, et garanties explicitement définis dans ce contrat sont autorisés.

### 10.2. Interdiction d'extension implicite

Aucune extension implicite de ce contrat n'est autorisée. Les règles suivantes s'appliquent :

- **INTERD-EXT-1** : Aucun état non défini dans ce contrat n'est reconnu
- **INTERD-EXT-2** : Aucune transition non définie dans ce contrat n'est autorisée
- **INTERD-EXT-3** : Aucune capacité non définie dans ce contrat n'est offerte

### 10.3. Primauté des invariants

**Règle absolue :**

Les invariants FONDATION priment toujours sur les considérations d'état. Aucune dégradation ne peut violer un invariant, même en état T4.

---

## 11. Conclusion contractuelle

Ce contrat établit de manière définitive et non négociable la gouvernance des états de confiance dans WorrySentinel.

Il garantit que :
- les cinq états de confiance (T0-T4) sont exhaustivement définis,
- les règles de transition sont explicites et séquentielles,
- les capacités par état sont clairement définies,
- la distinction avec les niveaux de sécurité est établie,
- WorrySentinel gouverne mais ne détecte ni ne décide,
- les invariants FONDATION sont préservés en tout état.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisée.

---

**Document créé le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION — Contrat normatif validé  
**Référence :** Miyukini Core System v2.4, WorrySentinel Documentation Fondatrice, Miyukini Conceptual References - Integrity Degradation System  
**Type :** Contrat de gouvernance des états de confiance

---

## 12. Mini log de génération

### Décision éditoriale E1 : Distinction états/niveaux

**Décision prise :** Ajout d'une section dédiée (Section 2) pour clarifier explicitement la distinction entre états de confiance (T0-T4, intégrité système) et niveaux de sécurité (0-4, profil de risque).

**Application :** Section 2 rédigée avec tableau comparatif et questions distinctives.

### Décision éditoriale E2 : Structure par état

**Décision prise :** Chaque état (T0-T4) est décrit de manière uniforme avec : état conceptuel, caractéristiques, capacités autorisées (tableau), contraintes, indicateurs de sortie.

**Application :** Section 3 rédigée avec format standardisé pour les 5 états.

### Décision éditoriale E3 : Matrice de transition

**Décision prise :** Inclusion d'une matrice visuelle des transitions autorisées (Section 4.1) pour clarifier les transitions permises et interdites.

**Application :** Matrice ajoutée avec transitions clairement identifiées.

### Ambiguïté A1 : Gouvernance vs détection

**Ambiguïté rencontrée :** Risque de confusion entre le rôle de gouvernance de WorrySentinel et le rôle de détection des anomalies.

**Décision prise :** Ajout des invariants INV-TS-9 et INV-TS-10 pour clarifier que WorrySentinel gouverne les règles mais ne détecte pas et ne décide pas des transitions.

**Correction effectuée :** Section 5.1 et Section 7 rédigées avec cette distinction explicite.

### Ambiguïté A2 : État T4 et sortie

**Ambiguïté rencontrée :** Comment sortir de l'état T4 si aucune transition sortante n'est autorisée ?

**Décision prise :** Clarification que T4 est terminal pour le système automatique. Seule une intervention humaine hors du système peut résoudre la situation. Ce n'est pas une transition automatique.

**Correction effectuée :** Section 3.7 et RÈGLE-TRANS-5 rédigées avec cette clarification.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Documentation Fondatrice WorrySentinel
- ✅ Cohérence avec Integrity Degradation System
- ✅ Cohérence avec les invariants INV-WS-1 à INV-WS-8
- ✅ Distinction états de confiance / niveaux de sécurité respectée
- ✅ Séparation gouvernance / détection / décision respectée
- ✅ Progressivité de la dégradation garantie
- ✅ Préservation des invariants FONDATION en T4 garantie

**Conclusion :** Contrat cohérent et complet, sans contradiction avec les documents existants.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce contrat.*
