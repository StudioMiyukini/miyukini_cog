# Security - Vocabulary & Glossary

## 1. Contexte

Ce document constitue le **glossaire officiel de la sécurité Miyukini** : un référentiel terminologique unique pour garantir une compréhension uniforme des concepts, acronymes et termes utilisés dans l'écosystème.

**Objectif :**

Fournir des définitions précises, non ambiguës, permettant à tout développeur, architecte ou opérateur de comprendre le vocabulaire sécurité Miyukini sans interprétation.

## 2. Portée / Scope

Ce document définit :
- Les acronymes officiels (STA, OSV, ECS, etc.)
- Les concepts fondamentaux de sécurité
- Les niveaux et états du système
- Les termes relatifs aux Engines de sécurité
- Les protocoles et leurs identifiants
- La terminologie structurelle et de gouvernance

Ce document **ne couvre pas** :
- Les détails d'implémentation technique
- Les spécifications cryptographiques détaillées
- Les procédures opérationnelles (voir Operational Runbook)

---

## 3. Acronymes Officiels

### 3.1 Acronymes Principaux

| Acronyme | Signification | Définition |
|----------|---------------|------------|
| **STA** | System Truth Anchor | Porteur de vérité officiel du système. Contient les empreintes MSCM, MIP, signatures structurelles, graph checksums et versions certifiées. |
| **OSV** | Official Secure Version | Version officielle sûre du système, validée, auditée, signée, figée, archivée et restaurable. |
| **ECS** | External Confidence Signal | Signal externe de confiance provenant d'Internet, traité comme information, jamais comme autorité. |
| **MSCM** | Miyukini Structured Code Model | Structure sémantique du code définissant responsabilités, lisibilité, traçabilité et indexabilité. |
| **MIP** | Miyukini Index Protocol | Mémoire structurelle globale du système, graphe système, macro-structure et gouvernance. |
| **COG** | Cognitive Orchestration Graph | Graphe d'orchestration cognitive gérant les agents IA et leurs interactions. |
| **TAMR** | Trusted Authority for Manual Review | Autorité de confiance pour revue manuelle et intervention humaine tracée. |

### 3.2 Acronymes des Cores

| Acronyme | Core | Rôle Sécurité |
|----------|------|---------------|
| **SF** | StrongFather | Décisions finales, validation systématique |
| **BG** | Border Guard | Classification sources, protection injection |
| **BB** | BondingBrother | Médiation sécurisée, traçabilité |
| **CN** | Caring Nanny | Détection anomalies, état système |
| **MB** | Master Butler | Capacités et permissions |
| **EB** | Ever Buddy | Compatibilité, versioning |
| **KM** | KindMother | Persistance, synchronisation |

### 3.3 Acronymes des Protocoles

| Acronyme | Signification | Type |
|----------|---------------|------|
| **RT-SEC** | Real-Time Security | Protocoles temps réel |
| **AS-SEC** | Asynchronous Security | Protocoles asynchrones |
| **NET-SEC** | Network Security | Protocoles retour Internet |
| **INV** | Invariant | Règles non négociables |

---

## 4. Concepts Fondamentaux

### 4.1 Principes Architecturaux

| Terme | Définition |
|-------|------------|
| **Propriété structurelle** | La sécurité comme caractéristique intrinsèque du système, non ajoutée mais émergente de l'architecture. |
| **Propriété émergente** | Sécurité résultant de l'interaction des composants, pas d'un module dédié. |
| **Sécurité structurelle** | Protection de l'architecture, des graphes et des relations du système. |
| **Sécurité cognitive** | Protection contre les dérives IA, biais et feedback loops erronés. |
| **Sécurité temporelle** | Protection de l'historique, versioning et continuité du système. |

### 4.2 Vérité et Confiance

| Terme | Définition |
|-------|------------|
| **Porteur de vérité** | Composant détenant la référence officielle de l'état du système (STA). |
| **Chaîne de confiance** | Séquence de validation : Code → MSCM → MIP → Graph → STA → OSV. |
| **Maillage de confiance** | Réseau de certification entre environnements Miyukini (Mesh). |
| **Confiance graduée** | Confiance exprimée en niveaux (T0-T4), jamais binaire. |
| **Zero-trust** | Principe où aucune entité n'est automatiquement fiable. |

### 4.3 Postulats de Sécurité (P1-P5)

| Postulat | Énoncé |
|----------|--------|
| **P1** | Un système ne tombe pas par ses fonctionnalités mais par ses interfaces et ses frontières. |
| **P2** | La sécurité technique est insuffisante sans sécurité structurelle. |
| **P3** | La sécurité du code est insuffisante sans sécurité cognitive. |
| **P4** | La protection périmétrique est insuffisante sans protection de la vérité. |
| **P5** | La sécurité est une propriété émergente du système. |

---

## 5. Niveaux et États

### 5.1 Niveaux de Confiance (Trust Levels T0-T4)

| Niveau | État | Signification | Capacités |
|--------|------|---------------|-----------|
| **T0** | Normal | Système sain | Toutes capacités disponibles |
| **T1** | Instable | Anomalie détectée | Log renforcé, traçabilité étendue |
| **T2** | Dégradé | Incohérence persistante | Certaines capacités désactivées |
| **T3** | Restreint | Suspicion forte | Gel des Opérateurs non essentiels |
| **T4** | Bloqué | Intégrité rompue | Uniquement diagnostics |

### 5.2 Niveaux de Sécurité (Security Levels 0-4)

| Niveau | Nom | Cas d'Usage | Impact Performance |
|--------|-----|-------------|-------------------|
| **0** | PUBLIC / DISPLAY | Site vitrine, dashboards lecture seule | Quasi nul |
| **1** | STANDARD / CMS | CMS, backoffice simple | Faible |
| **2** | SENSITIVE DATA | Données personnelles, profils | Modéré |
| **3** | CRITICAL SYSTEM | Auth, paiement, décisions | Accepté |
| **4** | HARDENED / ISOLATED | Environnement isolé, mode survie | Secondaire |

### 5.3 États de Dégradation

| État | Comportement |
|------|--------------|
| **Nominal** | Fonctionnement normal |
| **Doute** | Vérifications renforcées |
| **Suspect** | Fonctions sensibles désactivées |
| **Critique** | Lecture seule |
| **Compromis** | Blocage progressif → total |

### 5.4 Modes d'Environnement

| Mode | Description |
|------|-------------|
| **NORMAL** | Fonctionnement standard connecté |
| **RESTRICTED** | Capacités limitées |
| **ISOLATED** | Fonctionnement hors ligne autonome |
| **CLOSED** | Mode air-gapped délibéré |
| **SAFE MODE** | Mode de sécurité minimal |
| **QUARANTINE** | Isolement total pour diagnostic |
| **RECONNECTING** | Phase de reconnexion après isolement |

---

## 6. Intégrité Multi-Niveaux

### 6.1 Les 5 Niveaux d'Intégrité

| Niveau | Nom | Protège | Vérifications |
|--------|-----|---------|---------------|
| **1** | Passive | Fichiers | Hash, tailles, noms, structure dossiers |
| **2** | Structurelle | Architecture | Unicité IDs, hiérarchie, graphes, dépendances |
| **3** | Sémantique | Sens | Cohérence DO/ROLE/LAYER, non-contradictions |
| **4** | Cognitive | Intelligence | Décisions IA, dérives, feedback loops |
| **5** | Historique | Mémoire | Versioning, traçabilité, continuité temporelle |

### 6.2 Types de Sondes (Integrity Probes)

| Type | Vérifications | Détecte |
|------|---------------|---------|
| **Structurelle** | Invariants, cohérence inter-cores, graphes | Modification code, violation invariants |
| **Comportementale** | Décisions, fréquence erreurs | Bug hardware/logiciel, comportement modifié |
| **Environnementale** | Mémoire, disque, CPU | Corruption, instabilité matérielle |
| **Identité locale** | System Identity, empreinte | Clonage, rollback frauduleux |

---

## 7. Security Engines

### 7.1 Moteurs de Sécurité (8 Engines)

| Engine | Rôle | Actions Principales |
|--------|------|---------------------|
| **Integrity Engine** | Vérification permanente de l'intégrité | Hash checks, structure checks, graph validation, diff structurel |
| **Validation Engine** | Filtrage systémique | Validation entrées, flux, formats, structures, transitions |
| **Policy Engine** | Règles de fonctionnement | Contrôle d'accès, scopes, permissions, contraintes d'exécution |
| **Consensus Engine** | Éviter la décision unique | Multi-agents, validation croisée, vote structurel, arbitrage |
| **Audit Engine** | Traçabilité active | Logs, historiques, journaux d'action/décision/IA/structurels |
| **Sandbox Engine** | Isolement | Exécution isolée, test sécurisé, simulation, bac à sable |
| **Cognitive Guard** | Sécurité IA | Détection dérive/biais, anti-feedback-loop, seuils de confiance |
| **Recovery Engine** | Résilience | Rollback, restauration, snapshot, freeze, safe-mode |

### 7.2 Supports de Sécurité (10 Piliers)

| Support | Rôle |
|---------|------|
| **STA** | Support de vérité |
| **OSV** | Support de stabilité |
| **MSCM** | Support sémantique |
| **MIP** | Support structurel |
| **Index global** | Support cognitif |
| **Abstraction Layers** | Support d'isolation |
| **Versioning System** | Support temporel |
| **Storage sécurisé** | Support physique/logique |
| **Gouvernance humaine** | Support ultime |
| **Architecture** | Support fondamental |

---

## 8. Protocoles de Sécurité

### 8.1 Protocoles Temps Réel (RT-SEC)

| Protocole | Nom | Objectif |
|-----------|-----|----------|
| **RT-SEC-1** | Session Éphémère Forte | Réduire la surface d'attaque via sessions courtes |
| **RT-SEC-2** | Authentification en Couches | Validation multi-niveaux (identité, capacité, contexte, état) |
| **RT-SEC-3** | Validation Systématique | Aucun bypass de validation même en temps réel |
| **RT-SEC-4** | Détection Active d'Anomalie | Détection rythme anormal, incohérences, tentatives répétées |
| **RT-SEC-5** | Traçabilité Immédiate | Toute décision traçable, horodatée, signée |

### 8.2 Protocoles Asynchrones (AS-SEC)

| Protocole | Nom | Objectif |
|-----------|-----|----------|
| **AS-SEC-1** | Actions Non Engagées | Intentions préparées, jamais exécutées avant revalidation |
| **AS-SEC-2** | Signature Locale Faible | Signature non autoritaire pour détection falsification |
| **AS-SEC-3** | Revalidation Complète | Vérification totale à la reconnexion |
| **AS-SEC-4** | Anti-Replay & Anti-Ordre | ID unique, horodatage, dépendances explicites |
| **AS-SEC-5** | Dégradation Graduée | Progression contrôlée : avertissement → blocage |

### 8.3 Protocoles Retour Internet (NET-SEC)

| Protocole | Nom | Objectif |
|-----------|-----|----------|
| **NET-SEC-1** | Handshake de Conformité | Échange version/intégrité/confiance avant synchronisation |
| **NET-SEC-2** | Mise à Jour Sécurisée | Téléchargement signé, vérification locale, rollback possible |
| **NET-SEC-3** | Renforcement/Affaiblissement Local | Auto-limitation selon verdict serveur |

---

## 9. Lois et Invariants

### 9.1 Lois du Système (L1-L6)

| Loi | Énoncé |
|-----|--------|
| **L1** | Aucun accès direct hardware |
| **L2** | Aucune source de vérité multiple |
| **L3** | Aucun bypass des cores |
| **L4** | Aucune écriture sans traçabilité |
| **L5** | Aucune décision sans validation |
| **L6** | Aucune structure sans indexation |

### 9.2 Règles de Gouvernance (G1-G4)

| Règle | Énoncé |
|-------|--------|
| **G1** | Supervision humaine obligatoire |
| **G2** | Validation humaine des versions OSV |
| **G3** | Arbitrage humain des conflits |
| **G4** | Contrôle des décisions critiques |

### 9.3 Contraintes de Fonctionnement

| Contrainte | Description |
|------------|-------------|
| **Abstraction** | Tout passe par abstraction, pas d'accès direct |
| **Validation** | Tout passe par validation, pas d'action non vérifiée |
| **Consensus** | Tout passe par consensus, pas de décision unilatérale critique |
| **Versioning** | Tout passe par versioning, pas de modification sans trace |

---

## 10. Preuves et Attestations

### 10.1 Types de Preuves (Sans PoW)

| Preuve | Vérifie |
|--------|---------|
| **Proof of Integrity** | État non corrompu |
| **Proof of Consistency** | Cohérence interne |
| **Proof of Diversity** | Pluralité des sources |
| **Proof of History** | Continuité temporelle |
| **Proof of Governance** | Supervision humaine |
| **Proof of Structure** | Architecture valide |
| **Proof of Cognition** | Décisions IA saines |

### 10.2 Termes de Certification

| Terme | Définition |
|-------|------------|
| **Certification dynamique** | Processus de certification mutuelle entre environnements |
| **Certificateur** | Environnement validant un autre environnement |
| **Pluralité** | Exigence de plusieurs certificateurs |
| **Preuve par diversité** | Validation par sources multiples indépendantes |

---

## 11. Termes Structurels

### 11.1 Architecture des Strates

| Terme | Définition |
|-------|------------|
| **Strate** | Couche architecturale du système avec responsabilités définies |
| **Substrat** | Couche physique/logique brute (OS, drivers, hardware, runtime) |
| **Kernel** | Abstraction OS, hardware, runtime, services bas niveau |
| **Infrastructure systémique** | Couche des Security Engines entre Kernel et Cores |
| **Cores** | Logique fonctionnelle (StrongFather, KindMother, etc.) |
| **Services** | Applications, outils, plateformes, interfaces, produits finaux |

### 11.2 Modèle Fédéral

| Terme | Définition |
|-------|------------|
| **Fédération** | Ensemble d'instances Miyukini liées par confiance certifiée |
| **Instance** | Environnement Miyukini autonome avec identité propre |
| **Mesh** | Maillage de confiance entre instances |
| **Dogme de version** | Incompatibilité structurelle entre versions de CoreSet |
| **Souveraineté** | Autonomie décisionnelle d'une instance |

### 11.3 Identité Système

| Terme | Définition |
|-------|------------|
| **ENV_ID** | Identité unique de l'environnement |
| **CORE_VERSION** | Version des cores de l'environnement |
| **STA_ID** | Identité de vérité de l'environnement |
| **OSV_HASH** | Hash de la constitution certifiée |
| **GRAPH_HASH** | Hash de la structure |
| **STRUCT_HASH** | Hash de l'organisation |

---

## 12. Termes de Décision

### 12.1 Résultats de Décision StrongFather

| Résultat | Signification |
|----------|---------------|
| **ACCEPTÉE** | Décision validée et exécutable |
| **REFUSÉE** | Décision rejetée définitivement |
| **DIFFÉRÉE** | Décision reportée pour réévaluation |
| **AMBIGUË** | Décision nécessitant clarification ou intervention |

### 12.2 Termes d'Évaluation

| Terme | Définition |
|-------|------------|
| **Probabilité dominante** | Interprétation la plus probable d'une anomalie |
| **Root Cause Approximation** | Attribution approximative de cause sans vérité absolue |
| **Consolidation** | Agrégation des signaux par Caring Nanny |
| **Escalade** | Transfert d'une décision à un niveau supérieur (TAMR) |

---

## 13. Termes de Sécurité Réseau

### 13.1 External Confidence Signal (ECS)

| Champ | Valeurs | Description |
|-------|---------|-------------|
| **origin** | internet | Source du signal |
| **type** | update, compliance, alert, metadata | Type de signal |
| **confidence** | low, medium, high | Niveau de confiance |
| **verifiability** | cryptographic, structural, declarative | Méthode de vérification |
| **impact_scope** | none, advisory, restrictive | Impact sur le système |

### 13.2 Termes de Connexion

| Terme | Définition |
|-------|------------|
| **Bootstrap sécurisé** | Premier contact avec Internet (observer sans exposer) |
| **Handshake de conformité** | Échange initial de version/intégrité/confiance |
| **Air-gapped** | Système délibérément isolé du réseau |
| **Reconnexion** | Phase de retour au réseau après isolement |

---

## 14. Termes de Menace

### 14.1 Surfaces d'Attaque Reconnues

| Surface | Description |
|---------|-------------|
| **Interfaces** | Points de communication entre composants |
| **Abstractions** | Couches d'abstraction système |
| **Transitions inter-couches** | Passages entre strates |
| **MSCM** | Sémantique du code |
| **MIP** | Mémoire structurelle |
| **COG** | Cognition et agents IA |
| **Gouvernance humaine** | Social engineering, erreur, malveillance interne |

### 14.2 Types d'Anomalies

| Anomalie | Interprétation Probable |
|----------|------------------------|
| **Anomalies aléatoires + mémoire** | Hardware défectueux |
| **Invariant cassé net** | Modification de code |
| **Comportement cohérent mais interdit** | Intrusion |
| **Erreurs transitoires** | Bruit système |

---

## 15. Glossaire Alphabétique Complet

| Terme | Définition |
|-------|------------|
| **Abstraction Layer** | Couche d'isolation entre niveaux système |
| **Air-gapped** | Système volontairement déconnecté du réseau |
| **Anti-feedback-loop** | Mécanisme empêchant les boucles de rétroaction IA erronées |
| **Anti-replay** | Protection contre la relecture de commandes |
| **Attestation** | Preuve signée de conformité d'un composant |
| **Audit Engine** | Moteur de traçabilité active |
| **Bootstrap** | Premier démarrage ou premier contact réseau sécurisé |
| **Bypass** | Contournement interdit d'une couche de sécurité |
| **Certification dynamique** | Validation mutuelle entre environnements |
| **Chaîne de confiance** | Séquence de validation Code → OSV |
| **Checksum** | Somme de contrôle pour vérification d'intégrité |
| **Cognitive Guard** | Moteur de sécurité cognitive contre les dérives IA |
| **Consensus Engine** | Moteur empêchant les décisions unilatérales |
| **Core** | Composant fondamental de l'architecture Miyukini |
| **Dégradation graduée** | Réduction progressive des capacités sans blocage brutal |
| **Dérive IA** | Écart progressif du comportement IA attendu |
| **Dogme de version** | Incompatibilité entre versions de CoreSet |
| **ECS** | Signal externe de confiance (External Confidence Signal) |
| **Empreinte** | Hash ou signature identifiant un état |
| **Engine** | Moteur de sécurité actif |
| **Environnement** | Instance Miyukini autonome |
| **Explicabilité** | Capacité à justifier toute décision |
| **Fédération** | Ensemble d'instances liées par confiance |
| **Gel** | Arrêt temporaire de fonctionnalités |
| **Gouvernance humaine** | Supervision et arbitrage par l'humain |
| **Graph** | Représentation structurelle des relations |
| **Handshake** | Échange initial de validation |
| **Hash** | Empreinte cryptographique |
| **Intégrité** | État non corrompu du système |
| **Intention préparée** | Action en attente de validation |
| **Invariant** | Règle toujours vraie, non négociable |
| **Isolation** | Séparation stricte entre composants |
| **Kernel** | Couche d'abstraction bas niveau |
| **Mesh** | Maillage de confiance entre instances |
| **MIP** | Protocole d'index mémoire structurelle |
| **MSCM** | Modèle de code structuré sémantique |
| **Multi-agents** | Utilisation de plusieurs agents pour éviter le consensus erroné |
| **Opérateur** | Produit ou application utilisant l'écosystème |
| **OSV** | Version officielle sûre |
| **Pluralité** | Exigence de sources multiples |
| **Policy Engine** | Moteur de règles de fonctionnement |
| **Preuve** | Attestation vérifiable d'un état |
| **Quarantaine** | Isolement total pour diagnostic |
| **Recovery Engine** | Moteur de résilience et restauration |
| **Rollback** | Retour à un état antérieur sûr |
| **Root Cause** | Cause racine d'une anomalie |
| **Safe-mode** | Mode de fonctionnement minimal sécurisé |
| **Sandbox** | Environnement d'exécution isolé |
| **Session éphémère** | Session à durée de vie courte |
| **Signal** | Information provenant d'une source externe |
| **Signature** | Preuve cryptographique d'authenticité |
| **Snapshot** | Capture d'état à un instant T |
| **Sonde** | Mécanisme de vérification d'intégrité |
| **Souveraineté** | Autonomie décisionnelle d'une instance |
| **STA** | Ancre de vérité système |
| **Strate** | Couche architecturale |
| **Substrat** | Couche physique/logique brute |
| **Support** | Pilier structurel de la sécurité |
| **TAMR** | Autorité de revue manuelle de confiance |
| **Traçabilité** | Capacité à suivre l'historique des actions |
| **Trust Level** | Niveau de confiance (T0-T4) |
| **Validation Engine** | Moteur de filtrage systémique |
| **Versioning** | Gestion des versions du système |
| **Zero-trust** | Principe de méfiance par défaut |

---

## 16. Références Documentaires

Ce glossaire s'appuie sur les documents de référence suivants :

- [Miyukini Conceptual References - Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) : Document fondateur philosophique
- [Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) : Niveaux de sécurité (0-4)
- [Miyukini Conceptual References - Security Protocols](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) : Protocoles temps réel et asynchrone
- [Miyukini Conceptual References - Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) : Système de dégradation (T0-T4)
- [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](../../reference/Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md) : Intégration signaux externes

---

**Date de création :** 2026-01-28  
**Version :** 1.0  
**Statut :** Document de référence

**Documents associés :**
- [Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md)
- [Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md)
- [Security - FAQ & Common Questions](Security%20-%20FAQ%20&%20Common%20Questions.md)
