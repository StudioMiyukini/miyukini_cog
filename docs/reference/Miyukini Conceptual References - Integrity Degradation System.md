# Miyukini Conceptual References — Integrity & Degradation System

## 1. Contexte

Ce document définit le **système d'intégrité et de dégradation graduée** de l'écosystème Miyukini : un mécanisme de protection progressive qui évite les blocages brutaux et garantit une explicabilité interne.

**Principe directeur fondamental :**

**"Un système autonome ne bloque jamais brutalement. Il observe, interprète, dégrade, puis bloque seulement quand il est sûr."**

👉 La clé n'est pas la détection parfaite, mais la progression contrôlée + explicabilité interne.

## 2. Portée / Scope

Ce document définit :
- Les niveaux de confiance système (System Trust Levels) T0 à T4
- Les sondes d'intégrité (Integrity Probes) et leurs types
- L'attribution de cause (Root Cause Approximation)
- La dégradation progressive par niveau
- Le système de chaîne de confiance locale (System Trust Chain)
- Le rôle de BondingBrother dans la médiation observable

Ce document **ne couvre pas** :
- Les détails d'implémentation cryptographique
- Les protocoles de communication réseau (voir External Signal & Trust Reinforcement Contract)
- Les décisions spécifiques de StrongFather

---

## 3. Niveaux de Confiance Système (System Trust Levels)

### 3.1 Échelle de Confiance

Chaque core et chaque Opérateur opère avec un niveau de confiance courant :

| Niveau | État | Signification | Capacités |
|--------|------|---------------|-----------|
| **T0** | Normal | Système sain | Toutes les capacités disponibles |
| **T1** | Instable | Anomalie détectée | Log renforcé, traçabilité étendue |
| **T2** | Dégradé | Incohérence persistante | Certaines capacités désactivées |
| **T3** | Restreint | Suspicion forte | Gel des Opérateurs non essentiels |
| **T4** | Bloqué | Intégrité rompue | Uniquement diagnostics |

**📌 Ce n'est pas binaire**  
**📌 Chaque niveau réduit des capacités, jamais l'inverse**

### 3.2 Caractéristiques des Niveaux

#### T0 — Normal

**État :** Système sain, aucune anomalie détectée.

**Comportement :**
- ✅ Toutes les capacités disponibles
- ✅ Décisions normales
- ✅ Extensions dynamiques autorisées
- ✅ Monitoring standard

#### T1 — Instable

**État :** Anomalie détectée, mais pas encore confirmée.

**Comportement :**
- ✅ Log renforcé
- ✅ Traçabilité étendue
- ✅ Aucun blocage
- ✅ Surveillance accrue

**Transition :** Vers T0 si anomalie résolue, vers T2 si persistance.

#### T2 — Dégradé

**État :** Incohérence persistante, suspicion modérée.

**Comportement :**
- ✅ Certaines capacités désactivées
- ✅ Décisions plus strictes
- ✅ Refus des extensions dynamiques
- ✅ Monitoring visible (MiyukiniAdmin)

**Transition :** Vers T1 si amélioration, vers T3 si aggravation.

#### T3 — Restreint

**État :** Suspicion forte, intégrité potentiellement compromise.

**Comportement :**
- ✅ Gel des produits non essentiels
- ✅ Refus de nouveaux modules
- ✅ Décisions critiques → AMBIGUË / DIFFÉRÉE
- ✅ TAMR requis pour override

**Transition :** Vers T2 si confirmation de sécurité, vers T4 si confirmation de compromission.

#### T4 — Bloqué

**État :** Intégrité rompue, système compromis.

**Comportement :**
- ❌ Plus aucune décision opérationnelle
- ✅ Uniquement diagnostics
- ✅ État lisible
- ✅ Sortie propre possible

**📌 Jamais de corruption**  
**📌 Jamais d'exécution sauvage**

---

## 4. Sondes d'Intégrité (Integrity Probes)

### 4.1 Principe

Les sondes sont **conceptuelles, pas intrusives**. Elles s'exécutent :
- À chaque cycle critique
- À chaque décision importante
- À chaque transition d'état
- À chaque montée de charge

**📌 Pas de cron, pas de logique temporelle interdite**  
**📌 Conforme à StrongFather (INV-SF-4)**

### 4.2 Types de Sondes

#### 🧠 A. Sondes Structurelles

**Vérifications :**
- Invariants des cores
- Cohérence inter-cores
- Conformité des graphes de dépendance

**Détecte :**
- Modification de code
- Désactivation de règles
- Core amputé
- Violation d'invariants

**Exemples :**
- Vérification que StrongFather ne persiste jamais (INV-SF-2)
- Vérification que KindMother ne décide jamais (INV-KM-1)
- Vérification de la cohérence des graphes d'autorité

#### ⚙️ B. Sondes Comportementales

**Vérifications :**
- Décisions incohérentes
- Décisions impossibles
- Fréquence anormale d'erreurs

**Distingue :**
- Bug hardware (erreurs aléatoires)
- Bug logiciel (erreurs reproductibles)
- Comportement artificiellement modifié (patterns suspects)

**Exemples :**
- Décisions StrongFather qui violent les politiques
- Erreurs KindMother qui ne correspondent pas aux patterns normaux
- Fréquence d'erreurs anormalement élevée

#### 🧬 C. Sondes Environnementales

**Vérifications :**
- Mémoire instable
- Corruption disque
- Reboot partiel
- CPU instable

**Clé pour différencier :** Erreur vs intrusion

**Exemples :**
- Corruption de données persistées
- Instabilité mémoire (RAM défectueuse)
- Anomalies CPU (overclocking, température)

#### 🔐 D. Sondes d'Identité Locale

**Vérifications :**
- Validité de la System Identity
- Cohérence avec l'empreinte structurelle
- Continuité d'exécution

**Détecte :**
- Clonage
- Rollback
- Snapshot frauduleux
- Modification de l'identité système

---

## 5. Attribution de Cause (Root Cause Approximation)

### 5.1 Principe

**⚠️ Important :** On ne cherche pas une vérité absolue, mais une probabilité dominante.

Chaque anomalie génère un signal avec :
- **Source** : structure | comportement | environnement
- **Répétition** : faible | moyenne | élevée
- **Persistence** : transitoire | stable
- **Corrélation inter-core** : oui / non

### 5.2 Heuristique Clé

| Symptôme | Interprétation Probable |
|----------|------------------------|
| Anomalies aléatoires + mémoire | Hardware défectueux |
| Invariant cassé net | Modification de code |
| Comportement cohérent mais interdit | Intrusion |
| Erreurs transitoires | Bruit système |

### 5.3 Consolidation

**👉 StrongFather ne tranche pas seul**  
**👉 Caring Nanny consolide**  
**👉 TAMR peut autoriser intervention humaine**

**Flux typique :**
1. Sondes détectent anomalie
2. Caring Nanny consolide les signaux
3. StrongFather évalue la probabilité
4. Décision de dégradation (T1 → T2 → T3 → T4)
5. TAMR peut intervenir si nécessaire

---

## 6. Dégradation Progressive

### 6.1 T1 — Instable

**Actions :**
- ✅ Log renforcé
- ✅ Traçabilité étendue
- ✅ Aucun blocage
- ✅ Surveillance accrue

**Impact :** Aucun sur les capacités opérationnelles.

### 6.2 T2 — Dégradé

**Actions :**
- ✅ Certaines capacités désactivées
- ✅ Décisions plus strictes
- ✅ Refus des extensions dynamiques
- ✅ Monitoring visible (MiyukiniAdmin)

**Impact :** Réduction des capacités non essentielles.

### 6.3 T3 — Restreint

**Actions :**
- ✅ Gel des produits non essentiels
- ✅ Refus de nouveaux modules
- ✅ Décisions critiques → AMBIGUË / DIFFÉRÉE
- ✅ TAMR requis pour override

**Impact :** Fonctionnement minimal, intervention humaine possible.

### 6.4 T4 — Bloqué

**Actions :**
- ❌ Plus aucune décision opérationnelle
- ✅ Uniquement diagnostics
- ✅ État lisible
- ✅ Sortie propre possible

**Impact :** Arrêt opérationnel, diagnostic uniquement.

**📌 Jamais de corruption**  
**📌 Jamais d'exécution sauvage**

---

## 7. System Trust Chain (Offline)

### 7.1 Principe Crypto Local

**Ce qu'on prend :**
- Hash immuable
- Chaîne de confiance
- Signatures locales
- Dérivation de clé

**Ce qu'on refuse :**
- Token externe obligatoire
- Licence distante
- DRM opaque
- Clé maître universelle

### 7.2 Dérivation de Clé

**Chaque core possède une clé locale dérivée :**
- Jamais stockée brute
- Dérivée de :
  - System Identity
  - Invariant set
  - Rôle du core

**Les cores :**
- Signent leurs attestations
- Vérifient celles des autres

**➡️ Un core falsifié ne peut plus signer correctement**

### 7.3 Vérification Périodique

**Pas de cron, mais :**
- À chaque cycle critique
- À chaque décision importante
- À chaque transition d'état
- À chaque montée de charge

**📌 Conforme à StrongFather (INV-SF-4)**

---

## 8. Rôle de BondingBrother

### 8.1 Médiateur Observable de la Confiance

BondingBrother devient :
- **Le médiateur observable de la confiance**

**Il :**
- Transporte les signaux d'intégrité
- N'interprète jamais
- Ne décide jamais
- Rend visible les dégradations aux Opérateurs

**➡️ Les Opérateurs ne peuvent pas ignorer l'état du système**

### 8.2 Visibilité Obligatoire

**Tout Opérateur :**
- Reçoit le niveau de confiance courant (T0-T4)
- Ne peut pas ignorer les dégradations
- Doit adapter son comportement selon le niveau

**Exemples :**
- Opérateur en T2 : Désactive certaines fonctionnalités
- Opérateur en T3 : Mode minimal uniquement
- Opérateur en T4 : Arrêt propre

---

## 9. Intégration avec les Cores

### 9.1 Caring Nanny

**Rôle :** Consolidation des signaux d'intégrité.

**Responsabilités :**
- Collecte les signaux des sondes
- Consolide les anomalies
- Calcule le niveau de confiance global
- Propage les changements d'état

### 9.2 StrongFather

**Rôle :** Évaluation des probabilités et décisions de dégradation.

**Responsabilités :**
- Évalue les signaux consolidés par Caring Nanny
- Détermine la probabilité dominante
- Décide des transitions de niveau (T0 → T1 → T2 → T3 → T4)
- Applique les restrictions selon le niveau

### 9.3 Border Guard

**Rôle :** Classification des sources d'anomalies.

**Responsabilités :**
- Classifie les sources (interne, externe, environnement)
- Définit les niveaux de confiance des sources
- Fournit le contexte à StrongFather

### 9.4 TAMR

**Rôle :** Intervention humaine si nécessaire.

**Responsabilités :**
- Autorise les overrides en T3
- Permet l'intervention humaine pour diagnostic
- Trace toutes les interventions

---

## 10. Bénéfices du Système

### 10.1 Résilience

**✔️ Système autonome**  
**✔️ Offline**  
**✔️ Résilient**  
**✔️ Explicable**

### 10.2 Protection

**✔️ Anti-triche par cohérence**  
**✔️ Pas fragile**  
**✔️ Pas autoritaire**

**👉 Le système se défend sans devenir paranoïaque.**

### 10.3 Explicabilité

**Tout est traçable :**
- Signaux détectés
- Consolidation par Caring Nanny
- Décisions de StrongFather
- Transitions de niveau
- Interventions TAMR

---

## 11. Conclusion

Le système d'intégrité et de dégradation graduée garantit que Miyukini :
- Ne bloque jamais brutalement
- Observe, interprète, dégrade progressivement
- Bloque seulement quand il est sûr
- Reste explicable et traçable
- Fonctionne de manière autonome même en dégradation

Ce système est la garantie que l'écosystème reste résilient, explicable, et protégé, sans devenir fragile ou autoritaire.

---

**Date de création :** 2026-01-26  
**Version :** 1.0  
**Statut :** Document de référence contractuel

**Documentation associée :**
- [Caring Nanny - Documentation Fondatrice](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md)
- [Border Guard - Documentation Fondatrice](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md)
- [TAMR - Documentation Fondatrice](../core/TAMR/TAMR%20-%20Documentation%20Fondatrice.md)
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20%26%20Trust%20Reinforcement%20Contract.md)
- [Miyukini Conceptual References - Security Levels](Miyukini%20Framework%20-%20Security%20Levels.md) : Distinction entre niveaux de confiance (T0-T4) et niveaux de sécurité (0-4)
