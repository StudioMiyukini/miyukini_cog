# Miyukini Conceptual References — Security Protocols

## 1. Contexte

Ce document définit les **protocoles de sécurité** de l'écosystème Miyukini : les règles et mécanismes qui garantissent la sécurité du système dans deux régimes distincts (temps réel et asynchrone) tout en respectant les invariants communs.

**Principe fondamental :**

**"La sécurité n'est pas un mur. C'est un système nerveux. Il ressent, évalue, s'adapte, se dégrade, se protège."**

## 2. Portée / Scope

Ce document définit :
- Les invariants de sécurité communs (base)
- Les protocoles de sécurité temps réel (Online / Sync)
- Les protocoles de sécurité asynchrone (Offline / Async)
- La sécurité lors du retour Internet
- Les règles de dégradation graduée

Ce document **ne couvre pas** :
- Les détails d'implémentation cryptographique
- Les spécifications de chiffrement (voir les contrats spécifiques)
- Les stratégies de synchronisation (voir KindMother)

---

## 3. Vue d'Ensemble

### 3.1 Les 2 Grands Régimes de Sécurité

| Régime | Usage |
|--------|-------|
| **Sécurité temps réel (Online / Sync)** | Interaction immédiate, UI, monitoring, contrôle |
| **Sécurité asynchrone (Offline / Async)** | File d'attente, reprise, synchro différée, autonomie |

**👉 Ils ne partagent pas les mêmes règles, mais reposent sur les mêmes invariants.**

---

## 4. Invariants de Sécurité Communs (Base)

### 4.1 Règles Absolues

Ces règles sont **toujours vraies**, quel que soit le régime :

- ❌ **Aucun client n'est une source de vérité**
- ❌ **Aucune décision finale côté client**
- ✅ **Toute action est justifiée, signée, traçable**
- ✅ **Toute dégradation est visible**
- ❌ **Aucune clé critique persistée côté client**
- ✅ **Tout est révocable**

### 4.2 Porteurs des Invariants

Ces invariants sont portés par :
- **StrongFather** : Décisions finales, jamais côté client
- **Border Guard** : Classification des sources, protection injection
- **BondingBrother** : Médiation sécurisée, traçabilité
- **TAMR** : Intervention humaine, traçabilité absolue

---

## 5. Protocoles de Sécurité — Temps Réel (Online / Sync)

### 5.1 Cas d'Usage

- Monitoring live
- Commandes immédiates
- UI admin (MiyukiniAdmin)
- App mobile connectée
- WebApp active

### 5.2 Protocole RT-SEC-1 — Session Éphémère Forte

**Objectif :** Réduire la surface d'attaque.

**Règles :**
- ✅ Session courte (TTL strict)
- ✅ Renouvellement fréquent
- ✅ Invalidation immédiate possible
- ✅ Liée à :
  - Device
  - Contexte
  - Rôle

**👉 Une session ne survit pas à un changement de contexte.**

**Responsabilité :** BondingBrother (gestion sessions), Border Guard (classification)

### 5.3 Protocole RT-SEC-2 — Authentification en Couches

**Pas de "login unique".**

**Couches :**
1. **Identité** (qui ?)
2. **Capacité** (peut-il ?) → Master Butler
3. **Contexte** (maintenant ? ici ?)
4. **État système** (est-ce autorisé maintenant ?) → Caring Nanny + StrongFather

**👉 Master Butler + Caring Nanny + StrongFather**

**Flux :**
```
Requête
    ↓
Border Guard (classification source)
    ↓
Master Butler (capacités ?)
    ↓
Caring Nanny (état système ?)
    ↓
StrongFather (décision finale)
```

### 5.4 Protocole RT-SEC-3 — Validation Systématique

**Aucune optimisation ne peut court-circuiter :**
- ✅ Validation d'intention
- ✅ Validation de permission
- ✅ Validation de cohérence d'état

**Même en temps réel.**

**Responsabilité :** StrongFather (validation), Master Butler (permissions), Caring Nanny (cohérence)

### 5.5 Protocole RT-SEC-4 — Détection Active d'Anomalie

**En ligne, on peut détecter :**
- Rythme anormal
- Incohérence d'enchaînement
- Tentatives répétées
- Désynchronisation volontaire

**Réponses possibles :**
- Décision REFUSÉE
- Décision DIFFÉRÉE
- Dégradation de droits
- Suspension de session

**👉 Jamais d'auto-correction**

**Responsabilité :** Caring Nanny (détection), StrongFather (décision), Border Guard (classification)

### 5.6 Protocole RT-SEC-5 — Traçabilité Immédiate

**Toute décision :**
- ✅ Traçable
- ✅ Horodatée (trace seulement)
- ✅ Signée côté serveur
- ✅ Jamais bloquante

**Si la trace échoue :**
- ➡️ La décision continue (cf. R-TRACE-FAIL)

**Responsabilité :** Kernel (Logger), BondingBrother (journalisation), TAMR (traçabilité absolue)

---

## 6. Protocoles de Sécurité — Asynchrone / Dégradé

### 6.1 Cas d'Usage

- Mobile hors ligne
- File d'actions
- Synchronisation différée
- Environnements isolés
- Hardware faible

### 6.2 Protocole AS-SEC-1 — Actions Non Engagées

**Toute action asynchrone est :**
- ❌ Non exécutée
- ❌ Non persistée
- ❌ Non validée

**Statut :** "intention préparée"

**👉 Tant qu'elle n'a pas été réévaluée en ligne, elle n'existe pas.**

**Responsabilité :** BondingBrother (file d'attente), StrongFather (réévaluation)

### 6.3 Protocole AS-SEC-2 — Signature Locale Faible

**Côté client :**
- Signature non autoritaire
- Liée au device
- Liée à la session précédente
- Liée à la version du système

**Utilité :**
- Détecter falsification
- Détecter relecture
- Détecter injection

**👉 Pas pour décider. Juste pour classer le risque.**

**Responsabilité :** Border Guard (classification risque), BondingBrother (transport)

### 6.4 Protocole AS-SEC-3 — Revalidation Complète à la Reprise

**À la reconnexion :**

**Vérifications :**
1. ✅ Intégrité
2. ✅ Contexte
3. ✅ Version
4. ✅ Permissions
5. ✅ Évaluation StrongFather complète

**Résultat possible :**
- ACCEPTÉE
- REFUSÉE
- AMBIGUË
- DIFFÉRÉE

**Responsabilité :** Ever Buddy (compatibilité), StrongFather (réévaluation), Master Butler (permissions)

### 6.5 Protocole AS-SEC-4 — Anti-Replay & Anti-Ordre

**Chaque intention asynchrone :**
- ✅ ID unique global
- ✅ Horodatage conceptuel
- ✅ Dépendances explicites

**➡️ Impossible de :**
- Rejouer
- Réordonner
- Injecter hors séquence

**Responsabilité :** Kernel (Id), StrongFather (validation séquence), KindMother (persistance)

### 6.6 Protocole AS-SEC-5 — Dégradation Graduée

**Si problème détecté :**

| Niveau | Action |
|--------|--------|
| **1** | Avertissement |
| **2** | Lecture seule |
| **3** | Blocage actions |
| **4** | Reconnexion forcée |
| **5** | Quarantaine |

**👉 TAMR informe l'humain**  
**👉 Caring Nanny ajuste l'état global**

**Responsabilité :** Caring Nanny (détection), StrongFather (décision), TAMR (information)

---

## 7. Sécurité lors du Retour Internet

### 7.1 Protocole NET-SEC-1 — Handshake de Conformité

**Dès contact réseau :**

**Échange :**
- Version du noyau
- Version des cores
- Intégrité locale
- État de confiance

**➡️ Le système ne met rien à jour tant que la conformité n'est pas établie**

**Responsabilité :** Border Guard (isolation), Ever Buddy (compatibilité), Caring Nanny (état)

### 7.2 Protocole NET-SEC-2 — Mise à Jour Sécurisée

**Processus :**
1. ✅ Téléchargement signé
2. ✅ Vérification locale
3. ✅ Activation différée
4. ✅ Rollback possible

**👉 Ever Buddy gouverne tout ça**

**Responsabilité :** Ever Buddy (gouvernance), Border Guard (validation), StrongFather (décision)

### 7.3 Protocole NET-SEC-3 — Renforcement ou Affaiblissement Local

**Le serveur peut dire :**
- ✅ "Tu es sain"
- ⚠️ "Tu es dégradé"
- ❌ "Tu es compromis"

**➡️ Le système local s'auto-limite, jamais l'inverse**

**Responsabilité :** Caring Nanny (consolidation), StrongFather (décision), [External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md)

---

## 8. Résumé Ultra Clair

### 8.1 Temps Réel

- ✅ Sécurité stricte
- ✅ Sessions courtes
- ✅ Détection active
- ✅ Décisions immédiates

### 8.2 Asynchrone

- ✅ Aucune décision finale
- ✅ Intentions préparées
- ✅ Revalidation complète
- ✅ Dégradation graduée

### 8.3 Internet

- ✅ Contrôle de conformité
- ✅ Renforcement progressif
- ✅ Jamais de dépendance aveugle

---

## 9. Intégration avec les Cores

### 9.1 StrongFather

**Rôle :** Décisions finales, validation systématique.

**Protocoles concernés :**
- RT-SEC-2 (authentification en couches)
- RT-SEC-3 (validation systématique)
- RT-SEC-4 (détection anomalie)
- AS-SEC-3 (revalidation complète)
- NET-SEC-2 (mise à jour sécurisée)

### 9.2 Border Guard

**Rôle :** Classification des sources, protection injection.

**Protocoles concernés :**
- RT-SEC-1 (session éphémère)
- RT-SEC-2 (authentification en couches)
- RT-SEC-4 (détection anomalie)
- AS-SEC-2 (signature locale faible)
- NET-SEC-1 (handshake conformité)

### 9.3 BondingBrother

**Rôle :** Médiation sécurisée, traçabilité.

**Protocoles concernés :**
- RT-SEC-1 (session éphémère)
- RT-SEC-5 (traçabilité immédiate)
- AS-SEC-1 (actions non engagées)
- AS-SEC-2 (signature locale faible)

### 9.4 Caring Nanny

**Rôle :** Détection d'anomalies, état système.

**Protocoles concernés :**
- RT-SEC-2 (authentification en couches)
- RT-SEC-3 (validation cohérence)
- RT-SEC-4 (détection active)
- AS-SEC-5 (dégradation graduée)
- NET-SEC-1 (handshake conformité)
- NET-SEC-3 (renforcement local)

### 9.5 Master Butler

**Rôle :** Capacités et permissions.

**Protocoles concernés :**
- RT-SEC-2 (authentification en couches)
- RT-SEC-3 (validation permission)
- AS-SEC-3 (revalidation permissions)

### 9.6 TAMR

**Rôle :** Intervention humaine, traçabilité absolue.

**Protocoles concernés :**
- RT-SEC-5 (traçabilité immédiate)
- AS-SEC-5 (information utilisateur)

### 9.7 Ever Buddy

**Rôle :** Compatibilité, versioning.

**Protocoles concernés :**
- AS-SEC-3 (revalidation version)
- NET-SEC-1 (handshake conformité)
- NET-SEC-2 (mise à jour sécurisée)

---

## 10. Conclusion

Les protocoles de sécurité Miyukini garantissent que :

- ✅ **La sécurité est adaptative** : Différents protocoles selon le régime (temps réel vs asynchrone)
- ✅ **Les invariants sont respectés** : Règles communes à tous les régimes
- ✅ **La dégradation est graduée** : Pas de blocage brutal
- ✅ **La traçabilité est complète** : Toute action est traçable
- ✅ **L'autonomie est préservée** : Fonctionnement offline possible

**Philosophie Miyukini :**

**"La sécurité n'est pas un mur. C'est un système nerveux. Il ressent, évalue, s'adapte, se dégrade, se protège."**

---

**Date de création :** 2026-01-26  
**Version :** 1.0  
**Statut :** Document de référence contractuel

**Documentation associée :**
- [StrongFather - Documentation Fondatrice](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md) : Décisions finales
- [Border Guard - Documentation Fondatrice](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md) : Classification sources
- [BondingBrother - Documentation Fondatrice](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md) : Médiation sécurisée
- [Caring Nanny - Documentation Fondatrice](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) : Détection anomalies
- [TAMR - Documentation Fondatrice](../core/TAMR/TAMR%20-%20Documentation%20Fondatrice.md) : Traçabilité absolue
- [Master Butler - Documentation Fondatrice](../core/MasterButler/Master%20Butler%20-%20Documentation%20Fondatrice.md) : Permissions
- [Ever Buddy - Documentation Fondatrice](../core/EverBuddy/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) : Compatibilité
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Renforcement local
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) : Dégradation graduée (T0-T4)
- [Miyukini Conceptual References - Security Performance Impact](Miyukini%20Framework%20-%20Security%20Performance%20Impact.md) : Impact réel sur les performances
- [Miyukini Conceptual References - Security Levels](Miyukini%20Framework%20-%20Security%20Levels.md) : Niveaux de sécurité (0-4) et adaptation des protocoles
