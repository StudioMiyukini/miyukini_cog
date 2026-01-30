# MiyuClock — Runtime Boundary Contract

## 1. Contexte

Ce document définit le **bornage (frontières d'exécution)** du kit MiyuClock. Il établit ce que MiyuClock ne fait jamais, les frontières avec le Kernel (Clock) et les Cores, et les invariants de limite. MiyuClock est un Kit d'Outils qui orchestre des capacités atomiques de mesure du temps (instant présent, delta entre instants) sans décision métier, sans persistance et sans temps global (conformité **LOI-4**).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Ce que MiyuClock ne fait jamais (pas de persistance, pas de décision métier, pas de timezone imposée, pas de dépendance temps global, pas de capacité hors Tools composants)
- Les frontières avec le Kernel (Clock) et les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather, BondingBrother)
- Les invariants de limite (bornage)

Ce document **ne couvre pas** :
- L'implémentation technique du trait Clock du Kernel
- La persistance des timestamps (voir MiyuClock - KindMother Integration Contract)

---

## 3. Principe fondamental

### 3.1 Bornage

> **MiyuClock exécute des capacités gouvernées de mesure du temps (now, delta). Il ne persiste jamais, ne décide jamais de logique métier, n'impose jamais de timezone, et ne dépend jamais d'un temps global (LOI-4).**

### 3.2 Ce que MiyuClock ne fait jamais

| Code | Interdiction |
|------|--------------|
| **BOUND-1** | **Pas de persistance** — MiyuClock ne lit ni n'écrit en base. Toute utilisation de timestamps pour la persistance relève de l'Opérateur et de KindMother/MiyuSQL. |
| **BOUND-2** | **Pas de décision métier** — MiyuClock ne décide pas si une action doit être faite (ALLOW/DENY = StrongFather). Il exécute uniquement la mesure du temps mandatee. |
| **BOUND-3** | **Pas de timezone imposée** — MiyuClock ne impose aucune timezone ; l'instant fourni par `tool.time.now` est une référence locale. L'interprétation timezone reste à l'Opérateur ou au flux. |
| **BOUND-4** | **Pas de temps global** — MiyuClock ne dépend d'aucun serveur de temps externe (NTP, etc.). Conformité **LOI-4** : horloge locale uniquement. |
| **BOUND-5** | **Pas de modification d'état métier** — MiyuClock ne modifie pas les permissions, ne crée pas de mandat, ne révoque rien. Il fournit des valeurs de temps dans le flux. |
| **BOUND-6** | **Pas de capacité nouvelle** — MiyuClock n'ajoute aucune capacité qui n'existe pas dans ses Tools composants (now, delta). Il orchestre, n'invente pas. |

---

## 4. Frontière avec le Kernel (Clock)

### 4.1 Rôle du Kernel

| Frontière | Description |
|-----------|-------------|
| **Source de temps** | Le Kernel fournit le trait **Clock** (trace / horodatage local). MiyuClock s'appuie sur ce trait pour fournir `tool.time.now` et les instants nécessaires à `tool.time.delta`. |
| **Limite** | MiyuClock ne remplace pas le Clock du Kernel ; il expose la mesure du temps aux Opérateurs via la gouvernance. MiyuClock ne définit pas la source d'horloge (Kernel). |

### 4.2 Invariant LOI-4

Aucune dépendance à un temps global. L'horloge est locale ; le Kernel (Clock) est la seule source de temps pour MiyuClock. Aucun appel à NTP, serveur de temps externe ou temps universel requis.

---

## 5. Frontières avec les Cores

### 5.1 StrongFather

| Frontière | Description |
|-----------|-------------|
| **Décision** | StrongFather décide ALLOW ou DENY. MiyuClock n'est invoqué qu'en cas d'ALLOW. |
| **Limite** | MiyuClock ne prend aucune décision stratégique. Il n'émet pas de mandat, ne révoque rien. |

### 5.2 Master Butler

| Frontière | Description |
|-----------|-------------|
| **Catalogue** | Master Butler déclare le Toolkit et les Tools. MiyuClock n'enregistre pas lui-même les Tools ; il est déclaré par l'environnement. |
| **Limite** | MiyuClock ne gère pas les permissions ni le catalogue. Il est invoqué après vérification Master Butler. |

### 5.3 WorrySentinel et Caring Nanny

| Frontière | Description |
|-----------|-------------|
| **Sécurité et état** | WorrySentinel (niveau de sécurité) et Caring Nanny (état système) sont vérifiés avant l'appel à MiyuClock. |
| **Limite** | MiyuClock ne modifie pas le niveau de sécurité ni l'état système. Il n'est invoqué que si les pré-conditions sont remplies. |

### 5.4 BondingBrother

| Frontière | Description |
|-----------|-------------|
| **Médiation** | BondingBrother traduit l'intention et prépare le contexte. MiyuClock reçoit une demande déjà médiée. |
| **Limite** | MiyuClock ne médie pas les intentions ; il exécute la capacité (now ou delta) fournie dans le contexte gouverné. |

---

## 6. Invariants de limite

Les invariants MiyuClock utilisent des préfixes catégoriels (BOUND = bornage). Pour le format canonique des invariants des Cores, voir [Miyukini Conceptual References - Standardisation Numeration Invariants](../../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md).

| Code | Invariant |
|------|-----------|
| **INV-BOUND-1** | Aucune persistance ; MiyuClock ne lit ni n'écrit en base. Toute persistance de timestamps = Opérateur + KindMother/MiyuSQL. |
| **INV-BOUND-2** | Aucune exécution sans passage par la gouvernance (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) |
| **INV-BOUND-3** | Aucune décision métier dans MiyuClock ; exécution de la mesure du temps uniquement |
| **INV-BOUND-4** | Aucune dépendance à un temps global (LOI-4) ; horloge locale (Kernel Clock) uniquement |
| **INV-BOUND-5** | Le Toolkit n'expose que les capacités de ses Tools composants (now, delta) ; pas de capacité nouvelle |

---

## 7. Réponses aux violations

### 7.1 Comportement attendu

Si une condition de bornage est violée (ex. appel sans gouvernance, tentative de persistance par MiyuClock, dépendance temps global), MiyuClock ne doit pas exécuter. La réponse (rejet, erreur explicite) est gérée par la couche gouvernance (BondingBrother / StrongFather) ou par le contrat d'intégration (KindMother pour la persistance), pas par MiyuClock lui-même.

### 7.2 Traçabilité

Toute tentative d'appel hors bornage doit être tracée (observability, audit) selon les contrats Caring Nanny et KindMother ; MiyuClock ne décide pas du contenu du trace, il peut fournir un signal d'échec au flux gouverné.

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| MiyuClock - Tool Governance Compliance Contract | [MiyuClock - Tool Governance Compliance Contract](../governance/MiyuClock%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
