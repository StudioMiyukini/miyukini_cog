# Border Guard - Threat Model Contract

## 1. Contexte

Ce document définit le **modèle de menaces** gouverné par Border Guard dans l'écosystème Miyukini. Il spécifie formellement les catégories de menaces, les vecteurs d'attaque, les réponses conceptuelles, et les règles de détection que Border Guard applique pour protéger les frontières du système.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Références principales :**
- [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)
- [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](../../../../reference/Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non négociable**. Il dérive directement de la Documentation Fondatrice et des protocoles de sécurité.

---

## 2. Portée / Scope

- **Applicable à :** Toutes les frontières définies par Border Guard
- **Responsable :** Border Guard (définition des menaces et réponses conceptuelles)
- **Consommateurs :** BondingBrother (application), StrongFather (décision), Caring Nanny (observation)
- **Ne couvre pas :** L'implémentation technique des mécanismes de protection

---

## 3. Philosophie de sécurité

### 3.1 Principe fondamental

**"La sécurité n'est pas un mur. C'est un système nerveux. Il ressent, évalue, s'adapte, se dégrade, se protège."**

### 3.2 Posture de Border Guard

Border Guard adopte une posture de **défense en profondeur conceptuelle** :

1. **Définition** — Border Guard définit ce qui est une menace
2. **Classification** — Border Guard classifie les sources selon leur niveau de confiance
3. **Conseil** — Border Guard informe les autres cores sur les menaces
4. **Jamais d'exécution** — Border Guard ne bloque pas lui-même

### 3.3 Internet comme source de signaux

**Principe fondamental :**

> **"Internet n'a jamais raison. Il peut seulement confirmer ou infirmer ce que le système croit déjà."**

- Internet est un **capteur, pas un cerveau**
- Les signaux externes sont des **informations, pas des autorités**
- L'état local **prime toujours** sur les signaux externes

---

## 4. Catégories de menaces

### 4.1 Menaces aux frontières externes

Les menaces ciblant la frontière entre l'écosystème Miyukini et le monde extérieur.

#### THREAT-EXT-1 : Injection de données malveillantes

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative d'injecter des données non valides ou malveillantes via les frontières externes |
| **Vecteurs** | API publiques, webhooks, formulaires, imports |
| **Indicateurs** | Données hors format, patterns d'injection connus, séquences suspectes |
| **Impact potentiel** | Corruption de données, compromission système |
| **Réponse Border Guard** | Classification UNKNOWN → HOSTILE si pattern confirmé |

#### THREAT-EXT-2 : Usurpation d'identité

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative de se faire passer pour une source légitime |
| **Vecteurs** | Tokens volés, sessions hijackées, replay d'authentification |
| **Indicateurs** | Contexte incohérent, device inconnu, localisation suspecte |
| **Impact potentiel** | Accès non autorisé, actions frauduleuses |
| **Réponse Border Guard** | Dégradation confiance VERIFIED → UNKNOWN, notification StrongFather |

#### THREAT-EXT-3 : Attaque par déni de service

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative de saturer les frontières pour bloquer le système |
| **Vecteurs** | Requêtes massives, patterns répétitifs, amplification |
| **Indicateurs** | Volume anormal, rythme anormal, sources multiples coordonnées |
| **Impact potentiel** | Indisponibilité du système |
| **Réponse Border Guard** | Classification HOSTILE, isolation de la frontière ciblée |

#### THREAT-EXT-4 : Exfiltration de données

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative d'extraire des données au-delà des frontières autorisées |
| **Vecteurs** | Requêtes massives, scraping, tunneling |
| **Indicateurs** | Volume de sortie anormal, patterns d'extraction |
| **Impact potentiel** | Fuite de données sensibles |
| **Réponse Border Guard** | Resserrement des règles de sortie, notification Caring Nanny |

### 4.2 Menaces aux frontières internes

Les menaces ciblant les frontières entre zones de confiance internes.

#### THREAT-INT-1 : Escalade de privilèges

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative d'accéder à une zone de confiance supérieure sans autorisation |
| **Vecteurs** | Exploitation de failles, contournement de règles |
| **Indicateurs** | Tentatives de franchissement non autorisées, patterns d'exploration |
| **Impact potentiel** | Accès à des zones sensibles |
| **Réponse Border Guard** | Renforcement de la frontière, classification HOSTILE si répété |

#### THREAT-INT-2 : Mouvement latéral

| Aspect | Définition |
|--------|------------|
| **Description** | Tentative de se propager entre zones internes |
| **Vecteurs** | Exploitation de relations de confiance, rebond |
| **Indicateurs** | Accès inhabituels entre zones, patterns de propagation |
| **Impact potentiel** | Compromission étendue |
| **Réponse Border Guard** | Isolation des zones, resserrement des franchissements |

#### THREAT-INT-3 : Corruption de données internes

| Aspect | Définition |
|--------|------------|
| **Description** | Modification non autorisée de données dans une zone de confiance |
| **Vecteurs** | Injection interne, race conditions, manipulation d'état |
| **Indicateurs** | Incohérences de données, signatures invalides |
| **Impact potentiel** | Perte d'intégrité |
| **Réponse Border Guard** | Notification Caring Nanny, gel de la zone concernée |

### 4.3 Menaces aux frontières d'intégration

Les menaces ciblant les relations avec les systèmes externes.

#### THREAT-INTEG-1 : Compromission d'intégration

| Aspect | Définition |
|--------|------------|
| **Description** | Un système externe intégré devient malveillant ou compromis |
| **Vecteurs** | Intégration légitime devenue hostile, piratage du partenaire |
| **Indicateurs** | Comportement anormal de l'intégration, signaux externes de compromission |
| **Impact potentiel** | Canal de confiance devenu canal d'attaque |
| **Réponse Border Guard** | Suspension de l'intégration, classification HOSTILE |

#### THREAT-INTEG-2 : Abus d'intégration

| Aspect | Définition |
|--------|------------|
| **Description** | Un système intégré dépasse les limites de son contrat |
| **Vecteurs** | Requêtes hors scope, accès non autorisés, volume excessif |
| **Indicateurs** | Écarts par rapport au contrat d'intégration |
| **Impact potentiel** | Surcharge, accès non autorisé |
| **Réponse Border Guard** | Dégradation confiance, restriction des accès |

#### THREAT-INTEG-3 : Injection via intégration

| Aspect | Définition |
|--------|------------|
| **Description** | Utilisation d'une intégration légitime pour injecter des données malveillantes |
| **Vecteurs** | Webhooks manipulés, réponses API altérées |
| **Indicateurs** | Données incohérentes, signatures invalides |
| **Impact potentiel** | Corruption via canal de confiance |
| **Réponse Border Guard** | Validation renforcée, suspension si répété |

### 4.4 Menaces réseau et signaux externes

Les menaces liées aux communications réseau et aux signaux Internet.

#### THREAT-NET-1 : Signal externe malveillant

| Aspect | Définition |
|--------|------------|
| **Description** | Signal Internet tentant d'imposer un état ou une action |
| **Vecteurs** | Update signals falsifiés, compliance signals manipulés |
| **Indicateurs** | Signal non vérifiable, contradiction avec état local |
| **Impact potentiel** | Manipulation de l'état local |
| **Réponse Border Guard** | Rejet du signal, marquage AMBIGU |

#### THREAT-NET-2 : Réseau compromis

| Aspect | Définition |
|--------|------------|
| **Description** | Le réseau lui-même est devenu hostile (MITM, DNS poisoning) |
| **Vecteurs** | Interception, modification en transit, redirection |
| **Indicateurs** | Certificats invalides, réponses incohérentes |
| **Impact potentiel** | Toute communication compromise |
| **Réponse Border Guard** | Isolation automatique, fonctionnement local |

#### THREAT-NET-3 : Dépendance externe exploitée

| Aspect | Définition |
|--------|------------|
| **Description** | Exploitation d'une dépendance à un service externe |
| **Vecteurs** | Service tiers compromis, indisponibilité forcée |
| **Indicateurs** | Comportement anormal du service externe |
| **Impact potentiel** | Perte d'autonomie |
| **Réponse Border Guard** | Activation du mode dégradé, aucune dépendance critique |

---

## 5. Vecteurs d'attaque et réponses

### 5.1 Matrice vecteur / réponse

| Vecteur | Classification résultante | Action Border Guard | Core notifié |
|---------|---------------------------|---------------------|--------------|
| Pattern d'injection | UNKNOWN → HOSTILE | Blocage définition | StrongFather |
| Contexte incohérent | VERIFIED → UNKNOWN | Réévaluation | StrongFather |
| Volume anormal | UNKNOWN / HOSTILE | Isolation frontière | Caring Nanny |
| Signature invalide | HOSTILE | Blocage définition | StrongFather, TAMR |
| Signal externe contradictoire | AMBIGU | Marquage, pas d'action | Caring Nanny |
| Certificat invalide | HOSTILE (réseau) | Isolation réseau | Caring Nanny |

### 5.2 Réponses graduées

| Niveau de menace | Réponse Border Guard |
|------------------|---------------------|
| **Suspicion** | Surveillance accrue, pas de changement de classification |
| **Anomalie confirmée** | Dégradation de confiance (ex: VERIFIED → UNKNOWN) |
| **Pattern hostile** | Classification HOSTILE, notification cores |
| **Compromission** | Isolation frontière, gel zone, notification TAMR |

---

## 6. External Confidence Signals (ECS)

### 6.1 Traitement des signaux externes

Border Guard traite les signaux Internet selon le contrat External Signal Trust :

| Type de signal | Traitement Border Guard |
|----------------|------------------------|
| **Update signal** | Validation format, passage à Ever Buddy |
| **Compliance signal** | Comparaison état local, passage à Caring Nanny |
| **Alert signal** | Évaluation gravité, notification StrongFather |
| **Metadata signal** | Validation structure, stockage si conforme |

### 6.2 Règles de traitement ECS

1. **Isolation** — Tout signal est isolé avant traitement
2. **Validation format** — Rejet si format invalide
3. **Comparaison locale** — Comparaison avec l'état connu
4. **Classification** — Attribution d'un niveau de confiance au signal
5. **Transmission** — Passage aux cores concernés avec classification

### 6.3 Matrice signal / état local

| Signal externe | État local | Effet Border Guard |
|----------------|------------|-------------------|
| Conforme | Sain | Aucun changement |
| Conforme | Dégradé | Peut aider à remonter (via Caring Nanny) |
| Non conforme | Sain | Suspicion légère, surveillance |
| Non conforme | Dégradé | Renforce dégradation |
| Contradictoire | Tout état | Marqué AMBIGU, pas d'action automatique |

---

## 7. Bootstrap sécurisé (premier contact réseau)

### 7.1 Règles absolues

Le premier contact avec Internet après démarrage suit des règles strictes :

| Règle | Exigence |
|-------|----------|
| ❌ Aucune clé privée transmise | Les secrets restent locaux |
| ❌ Aucun état interne exposé | Pas de fuite d'information |
| ❌ Aucun module activé | Pas d'activation à distance |
| ❌ Aucune décision modifiée | Le réseau n'impose rien |

### 7.2 Informations autorisées

| Information | Autorisée | Raison |
|-------------|-----------|--------|
| Hash public du système | ✅ | Vérification d'intégrité |
| Version déclarative | ✅ | Compatibilité |
| Capacités exposées | ✅ | Via Master Butler |
| État de confiance (T0-T4) | ✅ | Anonymisé |
| Clés privées | ❌ | Secret absolu |
| État interne détaillé | ❌ | Information sensible |
| Logs complets | ❌ | Information sensible |

---

## 8. Handshake de conformité

### 8.1 Processus de reconnexion

Lors du retour Internet après une période hors ligne :

```
1. Border Guard : isolation du canal réseau
2. Échange de conformité :
   - Version du noyau
   - Version des cores
   - Intégrité locale
   - État de confiance
3. Border Guard : validation de conformité
4. Si conforme : levée progressive de l'isolation
5. Si non conforme : maintien de l'isolation
```

### 8.2 Résultats possibles

| Résultat | Action Border Guard |
|----------|---------------------|
| **Conforme** | Frontières normales selon niveau sécurité |
| **Partiellement conforme** | Frontières restreintes, surveillance |
| **Non conforme** | Isolation maintenue, notification TAMR |
| **Signal suspect** | Isolation renforcée, enquête |

---

## 9. Comportements en dégradation réseau

### 9.1 Situations et réponses

| Situation | Comportement Border Guard |
|-----------|---------------------------|
| **Pas d'Internet** | Fonctionnement normal, frontières locales actives |
| **Réseau instable** | Aucune panique, file d'attente des signaux |
| **Réseau compromis** | Isolation automatique, frontières fermées vers réseau |
| **Signaux incohérents** | Marqués AMBIGU, pas d'action automatique |
| **Tentative d'injection** | Classification HOSTILE, isolation immédiate |

### 9.2 Principe d'autonomie

**Même sans Internet, le système :**
- ✔️ Fonctionne (frontières locales actives)
- ✔️ Décide (StrongFather opérationnel)
- ✔️ Se protège (Border Guard définit les menaces locales)
- ✔️ Se dégrade (graduellement si nécessaire)

**Internet améliore la confiance, jamais la capacité.**

---

## 10. Détection et indicateurs

### 10.1 Indicateurs de menace par catégorie

| Catégorie | Indicateurs surveillés |
|-----------|----------------------|
| **Injection** | Patterns connus, données hors format, séquences suspectes |
| **Usurpation** | Contexte incohérent, device inconnu, replay détecté |
| **DoS** | Volume anormal, rythme anormal, patterns répétitifs |
| **Escalade** | Tentatives non autorisées, exploration de frontières |
| **Compromission** | Comportement anormal, signaux externes de compromission |

### 10.2 Niveaux de détection

| Niveau | Description | Seuil par défaut |
|--------|-------------|------------------|
| **INFO** | Événement normal à tracer | Toujours |
| **WARNING** | Événement inhabituel | Configuration dépendante |
| **ALERT** | Menace potentielle | Notification StrongFather |
| **CRITICAL** | Menace confirmée | Notification StrongFather + TAMR |
| **EMERGENCY** | Compromission active | Action immédiate + notification tous cores |

### 10.3 Adaptation des seuils selon niveau de sécurité

Les seuils de détection s'adaptent au niveau de sécurité déclaré :

| Niveau de sécurité | Sensibilité | Comportement |
|--------------------|-------------|--------------|
| **0 - PUBLIC** | Basse | WARNING rarement, ALERT sur patterns évidents |
| **1 - STANDARD** | Standard | Seuils par défaut |
| **2 - SENSITIVE** | Haute | WARNING fréquent, ALERT sur anomalies |
| **3 - CRITICAL** | Très haute | ALERT sur suspicions, CRITICAL rapidement |
| **4 - HARDENED** | Maximale | Toute anomalie = CRITICAL minimum |

---

## 11. Invariants de ce contrat

### INV-TMC-1 : Border Guard ne bloque jamais

Border Guard **définit** les menaces et les réponses conceptuelles. Il ne **bloque jamais** lui-même. L'application est déléguée à BondingBrother.

### INV-TMC-2 : Internet n'est pas une autorité

Aucun signal Internet ne peut **imposer** une action au système. Les signaux sont des **informations** traitées selon l'état local.

### INV-TMC-3 : Classification exhaustive des menaces

Toute menace détectée **doit** conduire à une classification de la source (UNKNOWN, HOSTILE, ou maintien avec surveillance).

### INV-TMC-4 : Dégradation graduée

La réponse à une menace suit toujours une **gradation** (suspicion → anomalie → menace → compromission), sauf compromission flagrante en niveau de sécurité 4.

### INV-TMC-5 : Autonomie préservée

Le système **reste fonctionnel** même sans Internet. Les frontières locales restent actives et Border Guard continue de définir les menaces.

### INV-TMC-6 : Traçabilité des détections

Toute détection de menace est **traçable** avec l'indicateur, le niveau de détection, et la réponse appliquée.

---

## 12. Interaction avec les autres cores

### 12.1 Flux vers StrongFather

| Événement | Information transmise |
|-----------|----------------------|
| Menace détectée | Type, indicateurs, classification source |
| Signal externe | Classification, comparaison état local |
| Anomalie de franchissement | Frontière concernée, source, indicateurs |

**StrongFather décide.** Border Guard informe.

### 12.2 Flux vers BondingBrother

| Événement | Information transmise |
|-----------|----------------------|
| Classification HOSTILE | Source, raison, règles de blocage à appliquer |
| Règles de franchissement modifiées | Nouvelles règles pour la frontière |
| Isolation de frontière | Frontière concernée, niveau d'isolation |

**BondingBrother applique.** Border Guard définit.

### 12.3 Flux vers Caring Nanny

| Événement | Information transmise |
|-----------|----------------------|
| État des frontières | Saines, sous pression, compromises |
| Signaux de conformité | Résultat de comparaison avec état local |
| Anomalies réseau | Type, gravité, impact potentiel |

**Caring Nanny observe.** Border Guard signale.

### 12.4 Flux vers TAMR

| Événement | Information transmise |
|-----------|----------------------|
| Menace CRITICAL ou EMERGENCY | Détails complets pour l'humain |
| Réhabilitation requise | Source HOSTILE à réhabiliter |
| Décision humaine requise | Cas ambigu nécessitant intervention |

**TAMR implique l'humain.** Border Guard fournit le contexte.

---

## 13. Références croisées

### Invariants associés (Documentation Fondatrice - Section 7)

| Invariant | Énoncé | Relation |
|-----------|--------|----------|
| INV-BG-1 | Aucune capacité d'exécution | Border Guard définit, ne bloque pas |
| INV-BG-3 | Aucune décision autonome | Border Guard informe, StrongFather décide |
| INV-BG-4 | Classification exhaustive | Toute source détectée est classifiée |
| INV-BG-8 | Traçabilité complète | Toute détection est traçable |

### Documents associés

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Miyukini Conceptual References - Security Protocols](../../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) | Protocoles temps réel et asynchrones |
| [Miyukini Conceptual References - External Signal Trust](../../../../reference/Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) | Traitement des signaux Internet |
| [Border Guard - Security Levels Adaptation Contract](./Border%20Guard%20-%20Security%20Levels%20Adaptation%20Contract.md) | Adaptation des seuils |
| [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Classification des sources |
| [Border Guard - StrongFather Integration Contract](../integration/Border%20Guard%20-%20StrongFather%20Integration%20Contract.md) | Flux de décision |
| [Border Guard - CaringNanny Integration Contract](../integration/Border%20Guard%20-%20CaringNanny%20Integration%20Contract.md) | Flux d'observation |

---

## 14. Synthèse contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Menaces catégorisées** — 13 types de menaces formellement définies
2. **Réponses graduées** — De la suspicion à la compromission
3. **Internet non autoritaire** — Signaux traités comme informations
4. **Autonomie préservée** — Fonctionnement sans Internet
5. **Détection adaptative** — Seuils selon niveau de sécurité
6. **Traçabilité complète** — Toute détection documentée

### Phrase de synthèse

> **Border Guard définit le modèle de menaces en catégorisant les attaques aux frontières externes, internes, d'intégration et réseau, en établissant des réponses graduées de la suspicion à la compromission, tout en garantissant que les signaux Internet restent des informations et jamais des autorités, préservant l'autonomie du système.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat — Normatif  
**Référence :** Border Guard v1.5, Security Protocols v1.0, External Signal Trust v1.0  
**Type :** Contrat de modèle de menaces
