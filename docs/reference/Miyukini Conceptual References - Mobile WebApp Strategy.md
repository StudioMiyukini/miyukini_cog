# Miyukini Conceptual References — Mobile & WebApp Strategy

## 1. Contexte

Ce document définit la **stratégie mobile et WebApp** de l'écosystème Miyukini : comment les applications mobiles (Android/iOS) et les WebApps interagissent avec le système Miyukini tout en respectant les principes d'autonomie, de sécurité, et de gouvernance.

**Principe fondamental (à graver) :**

**"Le mobile n'est jamais une source de vérité. Il est un terminal intelligent, pas un nœud décisionnel."**

## 2. Portée / Scope

Ce document définit :
- L'architecture mobile cible (Android/iOS)
- L'optimisation de la passerelle Mobile ↔ Serveur
- Les 3 niveaux de fonctionnement mobile (dégradé)
- Le cache UX mobile (temporaire, pas persistance)
- La stratégie WebApp (filet de sécurité)
- Le positionnement des cores (toujours côté serveur)

Ce document **ne couvre pas** :
- Les détails d'implémentation technique (protocoles, APIs)
- Les spécifications UI/UX détaillées
- Les stratégies de synchronisation (voir KindMother)

---

## 3. Objectif Exact

Sur Android / iOS, Miyukini doit :

✅ **Optimiser la passerelle app locale → logique serveur**  
⚠️ **Fonctionner en mode dégradé si la connexion est mauvaise ou absente**  
❌ **Ne jamais exposer les cores**  
🔁 **Offrir une alternative WebApp sans casser la sécurité**  
🧠 **Rester cohérent avec StrongFather / BondingBrother / TAMR**

---

## 4. Principe Fondamental

### 4.1 Le Mobile n'est Jamais une Source de Vérité

**Le mobile est :**
- ✅ Un terminal intelligent
- ✅ Un client d'interface utilisateur
- ✅ Un cache UX temporaire

**Le mobile n'est PAS :**
- ❌ Un nœud décisionnel
- ❌ Une source de vérité
- ❌ Une autorité de persistance

### 4.2 Règles Absolues

**Aucune décision finale sur mobile :**
- Toutes les décisions passent par StrongFather (serveur)
- Le mobile peut préparer, mais jamais valider

**Aucune persistance critique :**
- Le mobile ne persiste jamais de données critiques
- Toute persistance passe par KindMother (serveur)

**Aucune logique métier forte :**
- Le mobile est une interface, pas une logique
- Toute logique métier est côté serveur

---

## 5. Architecture Cible Mobile Miyukini

### 5.1 Vue d'Ensemble

```
[MOBILE DEVICE]
┌───────────────────────────┐
│ App native (Android/iOS)  │
│ ├ UI / UX                 │
│ ├ Cache UX local          │
│ ├ Mode dégradé            │
│ └ Client BondingBrother   │
└─────────────▲─────────────┘
              │
      (Optimized Gateway)
              │
┌─────────────▼─────────────┐
│ Bonding Brother (Server)  │
│ ├ Auth                    │
│ ├ Session                 │
│ ├ Adaptation mobile       │
│ └ QoS / dégradation       │
└─────────────▲─────────────┘
              │
┌─────────────▼─────────────┐
│ Miyukini Core Runtime     │
│ StrongFather / etc.       │
└───────────────────────────┘
```

### 5.2 Principe d'Isolation

**👉 Le mobile ne parle jamais directement aux cores.**

Toute interaction passe par BondingBrother (serveur), qui :
- Adapte les requêtes mobiles
- Gère les sessions
- Applique la QoS et la dégradation
- Protège les cores

---

## 6. Optimisation de la Passerelle Mobile ↔ Serveur

### 6.1 Ce qu'on Optimise

- **Latence** : Réduction du temps de réponse
- **Nombre d'allers-retours** : Agrégation des requêtes
- **Taille des payloads** : Compression sémantique
- **Tolérance réseau** : Résilience aux pannes réseau

### 6.2 BondingBrother = Gateway Intelligent

BondingBrother joue 4 rôles pour le mobile :

#### 1. Agrégation (Batch de Requêtes)
- Combine plusieurs requêtes en une seule
- Réduit le nombre d'allers-retours
- Optimise la bande passante

#### 2. Compression Sémantique
- Pas juste gzip (compression technique)
- Compression sémantique (adaptation du niveau de détail)
- Réponses partielles autorisées

#### 3. Résilience Réseau
- Gestion des timeouts
- Retry intelligent
- Décisions DIFFÉRÉE si réseau instable

#### 4. Adaptation de Niveau de Détail
- GraphQL-like contractuel (mais contrôlé)
- Diffs d'état, pas états complets
- Réponses adaptées au contexte mobile

### 6.3 Stratégies Concrètes

**GraphQL-like contractuel :**
- Requêtes structurées et contrôlées
- Pas de requêtes arbitraires
- Contrats stricts définis par Master Butler

**Diffs d'état :**
- Envoi uniquement des changements
- Pas d'états complets à chaque requête
- Réduction de la bande passante

**Réponses partielles autorisées :**
- Le mobile peut accepter des réponses incomplètes
- Mode dégradé explicite
- Indicateur de confiance visible

**Timeouts explicites :**
- Timeouts définis et documentés
- Décision DIFFÉRÉE côté serveur si timeout
- Pas de blocage côté mobile

---

## 7. Mode Dégradé sur Mobile (Clé)

### 7.1 Les 3 Niveaux de Fonctionnement Mobile

#### 🟢 Niveau A — Connecté Normal

**État :** Connexion réseau stable et rapide.

**Comportement :**
- ✅ App native fonctionnelle
- ✅ Temps réel
- ✅ Données fraîches
- ✅ Interaction complète

**➡️ État nominal**

#### 🟡 Niveau B — Connecté Instable / Lent

**État :** Connexion réseau instable ou lente.

**Comportement :**
- ✅ UI locale maintenue
- ⚠️ Données potentiellement obsolètes
- ⚠️ Actions mises en attente
- ✅ Indicateur de confiance visible

**Règles :**
- ➡️ Décisions jamais finales
- ➡️ TAMR informe l'humain
- ➡️ Actions marquées "non engagées"

#### 🔴 Niveau C — Hors Ligne (Dégradé Maximal)

**État :** Pas de connexion réseau.

**Ce qui est autorisé :**
- ✅ Navigation UI
- ✅ Consultation cache UX
- ✅ Actions préparées mais non envoyées
- ✅ Simulation locale (non décisionnelle)

**Ce qui est interdit :**
- ❌ Validation finale
- ❌ Exécution
- ❌ Modification d'état réel

**➡️ Tout est marqué "non engagé"**

### 7.2 Transitions entre Niveaux

**A → B :** Détection de latence élevée ou instabilité  
**B → A :** Retour à la normale  
**B → C :** Perte de connexion  
**C → B :** Reconnexion instable  
**C → A :** Reconnexion stable

**Gouvernance :**
- Caring Nanny détecte l'état réseau
- StrongFather décide des restrictions
- BondingBrother adapte les réponses
- TAMR informe l'utilisateur

---

## 8. Le Cache Mobile

### 8.1 Nature du Cache Mobile

**Important : le cache mobile**

Le cache mobile est :
- ❌ **Pas une persistance** : Ne remplace pas KindMother
- ❌ **Pas une vérité** : Ne remplace pas les cores
- ✅ **Un cache UX temporaire** : Améliore l'expérience utilisateur

### 8.2 Caractéristiques du Cache

**Le cache mobile est :**
- **Invalidable à tout moment** : Le serveur peut invalider
- **Signé** : Vérification d'intégrité
- **Versionné** : Gestion des versions
- **Jetable** : Peut être supprimé sans impact

### 8.3 Règles d'Utilisation

**Le cache mobile peut contenir :**
- Données UI (affichage)
- Préférences utilisateur (non critiques)
- État de navigation
- Données de formulaire (non validées)

**Le cache mobile ne peut JAMAIS contenir :**
- Données critiques
- Décisions validées
- État système
- Clés privées

---

## 9. WebApp = Client de Secours Universel

### 9.1 Positionnement

**WebApp = client de secours universel**

**Caractéristiques :**
- ✅ Même protocole BondingBrother
- ✅ Mêmes droits que mobile
- ⚠️ Souvent moins performant
- ✅ Mais plus compatible

### 9.2 Sécurité WebApp

**Règles absolues :**
- ❌ Jamais de clé privée persistée
- ✅ Sessions courtes
- ✅ Permissions limitées
- ❌ Aucune capacité critique

**➡️ WebApp = UI + contrôle, rien de plus**

### 9.3 Comportement Dégradé WebApp

| Situation | Comportement |
|-----------|--------------|
| **Offline** | Quasi inutilisable |
| **Réseau lent** | Lecture seule |
| **Session expirée** | Blocage doux |
| **Désync** | Rafraîchissement forcé |

**👉 Contrairement à l'app native, le Web n'est pas autonome.**

### 9.4 Comparatif App Native vs WebApp

| Aspect | App native | WebApp |
|--------|------------|--------|
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐ |
| **Mode hors ligne** | ✅ | ❌ |
| **Sécurité** | ⭐⭐⭐⭐ | ⭐⭐ |
| **UX** | ⭐⭐⭐⭐ | ⭐⭐ |
| **Dépendance OS** | Oui | Non |
| **Dépendance réseau** | Partielle | Forte |

### 9.5 Règle Stratégique Miyukini

**L'app native est le client principal.**  
**La WebApp est le filet de sécurité universel.**

---

## 10. Rôle des Cores dans l'Architecture Mobile

### 10.1 BondingBrother

**Rôle :** Adaptation, session, QoS.

**Responsabilités :**
- Adaptation des requêtes mobiles
- Gestion des sessions
- Application de la QoS
- Gestion de la dégradation
- Agrégation et compression

### 10.2 StrongFather

**Rôle :** Décisions différées si instable.

**Responsabilités :**
- Décisions finales (jamais sur mobile)
- Décisions DIFFÉRÉE si réseau instable
- Évaluation des intentions mobiles
- Application des politiques

### 10.3 Caring Nanny

**Rôle :** État global + confiance.

**Responsabilités :**
- Observation de l'état réseau
- Détection des dégradations
- Calcul du niveau de confiance (T0-T4)
- Propagation de l'état aux Opérateurs

### 10.4 TAMR

**Rôle :** Informe l'humain + autorise override.

**Responsabilités :**
- Information de l'utilisateur sur l'état
- Autorisation d'override si nécessaire
- Traçabilité des interventions
- Points d'intervention humaine

### 10.5 Border Guard

**Rôle :** Protège contre injection mobile/web.

**Responsabilités :**
- Classification des sources (mobile, web)
- Niveaux de confiance des sources
- Règles de franchissement
- Protection contre injection

### 10.6 Master Butler

**Rôle :** Capacités exposées.

**Responsabilités :**
- Registre des capacités disponibles
- Permissions pour mobile/web
- Contrats d'API
- Limitations par plateforme

---

## 11. Flux Typiques Mobile

### 11.1 Flux Action Utilisateur → Décision

```
Mobile (Action utilisateur)
    ↓
BondingBrother (Agrégation, adaptation)
    ↓
StrongFather (Décision)
    ↓
KindMother (Exécution si acceptée)
    ↓
BondingBrother (Réponse adaptée)
    ↓
Mobile (Affichage résultat)
```

### 11.2 Flux Mode Dégradé

```
Mobile (Détection réseau instable)
    ↓
Caring Nanny (Observation état)
    ↓
StrongFather (Décision DIFFÉRÉE)
    ↓
TAMR (Information utilisateur)
    ↓
Mobile (Affichage indicateur, actions en attente)
```

### 11.3 Flux Hors Ligne

```
Mobile (Pas de connexion)
    ↓
Cache UX local (Navigation, préparation)
    ↓
Actions marquées "non engagées"
    ↓
Reconnexion
    ↓
BondingBrother (Envoi actions en attente)
    ↓
StrongFather (Décision)
    ↓
Mobile (Résultat)
```

---

## 12. Sécurité Mobile/WebApp

### 12.1 Principes de Sécurité

**Isolation stricte :**
- Aucun core exposé directement
- Toute interaction via BondingBrother
- Sessions authentifiées
- Permissions limitées

**Protection contre injection :**
- Border Guard classifie les sources
- Validation stricte des requêtes
- Signatures cryptographiques
- Vérification d'intégrité

**Gestion des sessions :**
- Sessions courtes (WebApp)
- Tokens renouvelables
- Révocation possible
- Traçabilité complète

### 12.2 Protection des Données

**Données critiques :**
- Jamais stockées sur mobile
- Toujours côté serveur (KindMother)
- Chiffrement en transit
- Vérification d'intégrité

**Cache UX :**
- Données non critiques uniquement
- Invalidable à tout moment
- Signé et versionné
- Jetable sans impact

---

## 13. Conclusion

La stratégie mobile et WebApp de Miyukini garantit que :

- ✅ **Le mobile est optimisé** : Passerelle intelligente, agrégation, compression
- ✅ **Le mode dégradé fonctionne** : 3 niveaux, cache UX, actions non engagées
- ✅ **Les cores ne sont jamais exposés** : Isolation via BondingBrother
- ✅ **La WebApp est un filet de sécurité** : Compatibilité universelle, sécurité limitée
- ✅ **La cohérence est maintenue** : Tous les cores respectent leurs rôles

**Règle stratégique finale :**

**L'app native est le client principal.**  
**La WebApp est le filet de sécurité universel.**  
**Les cores sont toujours côté serveur.**

---

**Date de création :** 2026-01-26  
**Version :** 1.0  
**Statut :** Document de référence stratégique

**Documentation associée :**
- [BondingBrother - Documentation Fondatrice](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md) : Gateway intelligent mobile
- [StrongFather - Documentation Fondatrice](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md) : Décisions différées
- [Caring Nanny - Documentation Fondatrice](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) : État réseau et dégradation
- [TAMR - Documentation Fondatrice](../core/TAMR/TAMR%20-%20Documentation%20Fondatrice.md) : Information utilisateur
- [Border Guard - Documentation Fondatrice](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md) : Protection injection mobile/web
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md) : Niveaux de confiance (T0-T4)
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Gestion réseau instable
- [Miyukini Conceptual References - Security Protocols](Miyukini%20Framework%20-%20Security%20Protocols.md) : Protocoles sécurité temps réel (RT-SEC) et asynchrone (AS-SEC)
- [Miyukini Conceptual References - Security Performance Impact](Miyukini%20Framework%20-%20Security%20Performance%20Impact.md) : Impact performance mobile/offline
- [Miyukini Conceptual References - Security Levels](Miyukini%20Framework%20-%20Security%20Levels.md) : Niveaux de sécurité selon plateforme (mobile ≥ 2, WebApp max 2)
