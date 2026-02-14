# Miyukini Security — Liste des mesures de sécurité COG et MWS

## Contexte

Ce document recense **toutes les mesures de sécurité** existantes dans le projet Miyukini COG et dans le **MWS** (Miyukini Webway System). Il sert de référence normative et d’index pour les audits et la conformité.

**Références fondatrices :**
- [Doctrine Sécurité Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md)
- [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md)
- [MWS - Contre-Mesures de Sécurité](../../miyukini-webway-system/securite/MWS%20-%20Contre-Mesures%20de%20Securite.md)

---

## 1. Principes et doctrine

| Mesure | Description | Référence |
|--------|-------------|-----------|
| **Sécurité comme propriété structurelle** | La sécurité n’est pas un module ni un service ; elle est une propriété structurelle du système (loi d’architecture, contrainte de fonctionnement, invariant). | Doctrine Sécurité Fondamentale |
| **Protection Vérité / Structure / Mémoire / Cognition** | La sécurité protège la vérité (état certifié), la structure (architecture, graphes), la mémoire (historique, traçabilité), la cognition (décisions IA, anti-dérive). | Doctrine Sécurité Fondamentale |
| **5 postulats (P1–P5)** | Interfaces et frontières (P1), sécurité structurelle (P2), sécurité cognitive (P3), protection de la vérité (P4), sécurité propriété émergente (P5). | Doctrine Sécurité Fondamentale |
| **Médiation obligatoire** | Tout flux passe par les Security Engines ; aucun bypass autorisé entre Cores et Kernel. | Security - Architecture & Components |
| **Gouvernance technique forcée** | « La gouvernance Core n’est valide que si elle est techniquement forcée » — isolation processus, chiffrement, API authentifiée. | Security - Gouvernance Cores Protection Données |

---

## 2. Niveaux et états

### 2.1 Niveaux de confiance (T0–T4)

| Niveau | État | Rôle sécurité |
|--------|------|----------------|
| **T0** | Normal | Toutes capacités, monitoring standard |
| **T1** | Instable | Log renforcé, traçabilité étendue, pas de blocage |
| **T2** | Dégradé | Certaines capacités désactivées, décisions strictes |
| **T3** | Restreint | Gel Opérateurs non essentiels |
| **T4** | Bloqué | Uniquement diagnostics, lecture seule |

Référence : [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)

### 2.2 Niveaux de sécurité (0–4)

| Niveau | Profil | Usage type |
|--------|--------|------------|
| **0** | Public / Display | Vitrine, données publiques, dashboards lecture seule |
| **1** | Standard | Données courantes, formulaires, workflows standards |
| **2** | Sensitive | Données sensibles, renforcement contrôles |
| **3** | Critical | Données critiques, chiffrement, audit strict |
| **4** | Highest | Données les plus sensibles, contrôles maximaux |

La sécurité est un **paramètre de gouvernance** : l’Opérateur déclare son niveau, les Cores adaptent le comportement (StrongFather, MasterButler, BorderGuard, TAMR, Kernel).  
Référence : [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

---

## 3. Cores et gouvernance

### 3.1 Règles architecturales (sécurité)

| Mesure | Description |
|--------|-------------|
| **Cores décident, jamais n’exécutent** | Les Cores gouvernent ; les Outils exécutent. |
| **Strate Cores immuable** | Évolution = nouvel environnement (LOI-7). |
| **Pas de contournement KindMother** | Accès données uniquement via IPC authentifié vers processus KindMother isolé. |
| **Mandat obligatoire (GovernedContext)** | Les toolkits exigent `mandate_id` et `security_level` ; exécution refusée sans mandat. |
| **unsafe_code = "forbid"** | Tous les crates du workspace ont `unsafe_code = "forbid"` dans `[lints.rust]`. |

### 3.2 KindMother — Protection des données

| Mesure | Description | Référence |
|--------|-------------|-----------|
| **Isolation processus** | KindMother dans un processus séparé ; seul ce processus accède aux fichiers DB. | Security - Gouvernance Cores Protection Données |
| **Permissions fichier** | `chmod 600`, owner dédié (kindmother) sur les fichiers de base. | Id. |
| **Chiffrement au repos (libSQL)** | AES-256-GCM / AEGIS ; clé jamais sur disque. | Id. |
| **Dérivation de clé souveraine** | Machine ID + Install Secret + COG ID → Argon2id → Master Key (32 octets, RAM uniquement). | Id. |
| **API IPC authentifiée** | Requêtes IPC avec token (operator_id, request_id, timestamp, signature HMAC-SHA256) ; requête sans token valide rejetée. | Id. |
| **Matrice de permissions par Opérateur** | KindMother maintient quels Opérateurs accèdent à quelles tables (JayXpose, JayKonta, JayFestival, MiyukiniAdmin). | Id. |

### 3.3 StrongFather / BondingBrother

| Mesure | Description |
|--------|-------------|
| **Décision centrale** | StrongFather prend les décisions de gouvernance ; BondingBrother assure la médiation observable. |
| **Validation avant action sensible** | Pour les actions à validation StrongFather, le handler vérifie la permission puis demande la décision via BondingBrother ; si DENIED → 403. |
| **Contrats sécurité Cores** | StrongFather, BorderGuard, WorrySentinel, TAMR, CaringNanny, EverBuddy, BondingBrother ont des contrats sécurité (threat model, niveaux, implications). |

### 3.4 Kernel — Frontières de sécurité

| Mesure | Description |
|--------|-------------|
| **Aucune sécurité active dans le Kernel** | Authentification, autorisation, crypto, validation métier, audit sont dans les Security Engines / Cores. |
| **Primitives pour la sécurité** | Le Kernel fournit config, id, time, log, lifecycle pour horodatage, identifiants, journalisation. |
| **Invariants INV-K-1 à INV-K-10** | Chaque invariant a des implications sécurité (surface d’attaque minimale, pas de dépendance externe critique, déterminisme, pas de protocole applicatif, etc.). |
| **Sondes environnementales** | Horloge, configuration, lifecycle, ressources pour détection d’anomalies ; le Kernel observe, les Cores décident. |
| **Adaptation T0–T4 et niveaux 0–4** | Fréquence des sondes et comportement adaptés au niveau de confiance et au niveau de sécurité. |

Référence : [Kernel - Security Boundaries Contract](../../kernel/contracts/Kernel%20-%20Security%20Boundaries%20Contract.md)

---

## 4. Security Engines (strate entre Kernel et Cores)

| Engine | Rôle sécurité |
|--------|----------------|
| **Integrity Engine** | Vérification continue de l’intégrité (détection altérations). |
| **Validation Engine** | Filtrage systémique des entrées (injection, schémas). |
| **Policy Engine** | Règles du système, politiques d’accès. |
| **Consensus Engine** | Décisions pluralistes / multi-agents. |
| **Audit Engine** | Mémoire de sécurité, traçabilité, journalisation. |
| **Sandbox Engine** | Isolement d’exécution (code non fiable). |
| **Cognitive Guard** | Sécurité IA (anti-dérive, décisions contrôlées). |
| **Recovery Engine** | Résilience, rollback, restauration. |

Référence : [Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md)

---

## 5. Identité et contrôle d’accès

### 5.1 Niveaux d’identité (COG)

| Niveau | Nom | Confiance |
|--------|-----|-----------|
| **LSI** | Local Sovereign ID | Auto-déclarée |
| **VID** | Verified ID | Attestée par un tiers |
| **WID** | Witnessed ID | Témoignée par d’autres COG |

### 5.2 MiyukiniAdmin (Opérateur Souverain)

| Mesure | Description |
|--------|-------------|
| **Registre comptes admin** | account_id, username, password_hash (Argon2id), role, mfa_enabled, mfa_secret_encrypted, locked_until, failed_attempts. |
| **Login** | Rate limit par IP et par compte ; vérification hash ; réponse générique en cas d’échec (« Identifiants invalides »). |
| **MFA** | TOTP (secret chiffré AES-256-GCM) ; WebAuthn (pas de secret stocké serveur). |
| **Session** | session_id, TTL ; cookie HttpOnly Secure SameSite (ou header Authorization) ; timeout inactivité (ex. 15 min), absolu (ex. 8 h). |
| **Rate limiting** | Par compte : N échecs → verrouillage (ex. 15 min) ; par IP : N échecs → 429 / page « Trop de tentatives ». |
| **RBAC** | Rôles Admin / Recovery / Audit ; capacités explicites (admin.dashboard.read, admin.security.level.write, etc.) ; matrice role_has_capability ; middleware par route avec capacité requise ; 403 si capacité absente. |
| **États d’environnement** | VIERGE → routes /setup uniquement + verrou StrongFather ; INITIALISE → routes normales + auth ; COMPROMIS → page dédiée, WorrySentinel T3/T4, éventuel AutoRecovery. |

Références : [MiyukiniAdmin - Implementation Security and Controls](../../admin/MiyukiniAdmin/implementation/MiyukiniAdmin%20-%20Implementation%20Security%20and%20Controls.md), [MiyukiniAdmin - Permission Contract](../../admin/MiyukiniAdmin/contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md)

### 5.3 MiyukiniAdmin Origin (MWS)

| Mesure | Description |
|--------|-------------|
| **Accès /admin** | Authentification e-mail + mot de passe (Argon2id) ; JWT HMAC-SHA256 pour la session. |
| **Batterie de tests** | Connectivité, fonctionnel MWS, sécurité, réseau. |
| **Headers de sécurité** | Content-Security-Policy (default-src 'self', script-src 'self', style-src 'self' 'unsafe-inline'). |

Référence : [MWS - MiyukiniAdmin](../../miyukini-webway-system/administration/MWS%20-%20MiyukiniAdmin.md)

---

## 6. Chaîne de confiance et MIP

| Maillon | Rôle |
|---------|------|
| **CODE** | Source. |
| **MSCM** | Sémantique locale (balisage). |
| **MIP** | Mémoire structurelle, index ; cohérence CODE ↔ MSCM ↔ MIP. |
| **GRAPH** | Modèle global. |
| **STA** | System Truth Anchor. |
| **OSV** | Official Secure Version. |

Contrat MIP : invariants INV-MIP-1 à INV-MIP-6 ; pas de structure reconnue sans indexation (Loi L6).  
Référence : [Security - MIP Security Contract](../contracts/governance/Security%20-%20MIP%20Security%20Contract.md)

---

## 7. MWS — Mesures générales

### 7.1 Chiffrement et TLS

| Mesure | Description |
|--------|-------------|
| **TLS obligatoire canal de contrôle** | Aucune exception ; pas de plaintext sur le contrôle. |
| **TLS par défaut canal de données** | Obligatoire par défaut ; exemption strictement encadrée pour temps réel (durée max 4 h, renouvellement). |
| **Versions TLS** | TLS 1.3 recommandé ; TLS 1.2 minimum ; TLS 1.1 et inférieures refusées. |
| **Cipher suites** | PFS obligatoire ; AES-GCM, ChaCha20-Poly1305 ; clé min 128 bits (256 recommandé). |
| **Certificats** | CA reconnue (Let’s Encrypt) ; validité max 1 an (90 jours recommandé) ; certificate pinning pour clients Origin (R-014). |
| **Replay protection** | Fenêtre timestamp ±10 s (R-006) ; NTP recommandé. |
| **Token d’authentification** | Présence et validation des tokens sur les canaux MWS ; échec → rejet / déconnexion. |
| **Rotation des tokens** | Rotation 7 jours ; révocation ; notification (R-007). |

Référence : [MWS - Chiffrement et TLS](../../miyukini-webway-system/securite/MWS%20-%20Chiffrement%20et%20TLS.md)

### 7.2 Contre-mesures MWS (R-001 à R-015)

| ID | Priorité | Contre-mesure |
|----|----------|----------------|
| **R-001** | Critique | Haute disponibilité Origin ; procédure de failover (actif-passif, RTO/RPO). |
| **R-002** | Critique | Protection DDoS Origin (rate limiting, PoW, anti-DDoS frontal). |
| **R-003** | Élevée | Signature des paquets DATA (MAC 32 octets). |
| **R-004** | Élevée | Protection Eclipse (liste trackers signée dans REGISTER_OK). |
| **R-004bis** | Élevée | Adresse Origin non falsifiable (manifeste signé, certificate pinning). |
| **R-005** | Élevée | Signature des binaires (supply chain) ; Registre de Services (signature, signing_key). |
| **R-006** | Moyenne | Fenêtre timestamp ±10 s (replay). |
| **R-007** | Moyenne / Élevée | Rotation automatique des tokens ; certificate pinning Origin. |
| **R-008** | Moyenne | Durée max exemption temps réel (4 h). |
| **R-009** | Moyenne | Révocation de Permis en temps réel (PERMIT_REVOKE, propagation, cache). |
| **R-010** | Moyenne | Validation schéma JSON (manifest, payloads ; profondeur max 5). |
| **R-011** | Faible | Limite essais Lobby (3 essais + délai exponentiel). |
| **R-012** | Faible | Badge Lobby vérifié ; affichage cog_id hôte. |
| **R-013** | Faible | DNSSEC recommandé sur domaines MWS. |
| **R-014** | Faible / Élevée | Certificate pinning (clients Origin). |
| **R-015** | Faible | Fuzzing (parser binaire) ; plan de remédiation. |

Référence : [MWS - Contre-Mesures de Sécurité](../../miyukini-webway-system/securite/MWS%20-%20Contre-Mesures%20de%20Securite.md)

### 7.3 Relay

| Mesure | Description |
|--------|-------------|
| **Authentification** | Token et/ou secret ; enregistrement tunnel associé à cog_id. |
| **Clé de conformité Cores** | Vérification de la clé cachée dans le code pour authentifier les Cores. |
| **Phase B (blocs MIP)** | Origin demande des blocs de code MIP aléatoires et vérifie le déchiffrement (conformité Services). |
| **TLS** | Port 7000 (ou configuré) ; certificat CA ou auto-signé + pinning en test. |
| **Isolation / rate limiting** | Limites par COG ; surfaces web (80/8080) gérées par le COG (auth applicative). |

Références : [Miyukini Conceptual References - Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md), [MWS - Origin](../../miyukini-webway-system/acteurs/MWS%20-%20Origin.md)

### 7.4 Tracker — Systèmes passifs (MiyuWebwayTracker)

| Mesure | Description |
|--------|-------------|
| **Validation syntaxique** | Conformité schéma, champs obligatoires ; rejet des messages non conformes. |
| **Vérification de signature** | Intégrité et authenticité des déclarations (norme MWS). |
| **Journalisation** | Tracabilité annonces, requêtes, mises à jour de statuts (sans données métier ni secrets). |
| **Liste de statuts COG** | Trusted, Neutral, Under review, Distrusted, Rejected ; cohérence et traçabilité. |
| **Signalement** | Alertes vers Trackers / COGs / WorrySentinel sans bloquer le flux (passif). |
| **Vérification de version** | core_version, protocol_version ; enregistrement compatible / obsolète / incompatible. |
| **Registre de Services** | Vérification service_id vs Registre Origin ; non répertorié → journalisation + signalement. |
| **Contrôle tracker** | Validité Permis de circulation (non expiré, émis par relay reconnu). |
| **Pools par version** | Direction COG vers pool core_version.MAJOR ; pas de connexion inter-pools. |
| **Anti-énumération** | Pas d’exposition de la liste complète des cog_id à un demandeur non autorisé. |
| **Intégrité des logs** | Logs append-only, signés ou hashés pour détection d’altération. |

Référence : [MiyuWebwayTracker - Passive Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md)

### 7.5 Tracker — Systèmes actifs (MiyuWebwayTracker)

| Mesure | Description |
|--------|-------------|
| **Blocage** | Refus d’annonce, de requête de découverte, de connexion (statut Rejected/Distrusted, blacklist, rate limit, pattern d’attaque, permis invalide, incompatibilité de version, service non répertorié, etc.). |
| **Dégradation** | Throttling, réponses dégradées (Under review, rate limit). |
| **Blacklist** | Liste noire locale/partagée (IP, cog_id) avec expiration et audit. |
| **Isolation réseau (service non répertorié)** | Exclusion du maillage actif ; tunnel en surveillance ; notification utilisateur. |
| **Blocage inter-pool** | Refus des connexions entre COG de core_version.MAJOR différente. |
| **Politique par les Cores** | Les décisions de qui bloquer/dégrader viennent des Cores (Border Guard, WorrySentinel, StrongFather) ; le Tracker applique. |

Référence : [MiyuWebwayTracker - Active Systems Contract](../../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)

---

## 8. Toolkits et opérateurs

### 8.1 Contexte gouverné (toolkits)

| Mesure | Description |
|--------|-------------|
| **GovernedContext** | mandate_id + security_level ; has_mandate() ; exécution refusée si pas de mandat (NoMandate). |
| **Contrats Security and States** | MiyuAuth, MiyuClock, MiyuSQL, MiyuWeb : contrats sécurité et états dans docs/tools/Miyu{Nom}/contracts/security/. |

### 8.2 Services (exemples)

| Service | Mesure |
|---------|--------|
| **JayXpose** | Niveaux de sécurité et protection des données (reference) ; confidentialité_profil (public / authentifié) ; niveau sécurité lecture/formulaire. |
| **JayKonta / JayKoa** | Niveaux de sécurité et protection des données documentés (reference). |

Références : docs/services/JayXpose/reference/, docs/services/JayKonta/reference/, docs/services/JayKoa/reference/.

---

## 9. Protocoles et opérations

### 9.1 Protocoles de sécurité (référence)

| Protocole | Rôle |
|-----------|------|
| **RT-SEC-5** | Tracabilité immédiate (Kernel fournit time, id, log). |
| **AS-SEC-4** | Anti-replay et anti-ordre (time, id pour nonce/sequence). |

Référence : [Security Protocols](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)

### 9.2 Opérationnel

| Document | Contenu |
|----------|---------|
| **Security - Threat Model Summary** | Résumé du modèle de menaces. |
| **Security - Operational Runbook** | Procédures opérationnelles. |
| **Security - Versioning & Evolution** | Gel et évolution des versions (ex. Security - Gel et Versionnement v1.0.0). |
| **Security - Reference Implementation Guidelines** | Lignes directrices d’implémentation. |
| **Security - Operational Constraints Contract** | Contraintes opérationnelles. |

---

## 10. Synthèse par catégorie

| Catégorie | Exemples de mesures |
|-----------|---------------------|
| **Doctrine et architecture** | Sécurité propriété structurelle ; 5 postulats ; médiation obligatoire ; 8 Security Engines. |
| **Gouvernance** | Niveaux T0–T4 et 0–4 ; mandat ; Cores décident ; KindMother seul gardien des données. |
| **Données** | Isolation processus KindMother ; chiffrement libSQL ; dérivation clé souveraine ; IPC authentifié. |
| **Identité et accès** | LSI/VID/WID ; MiyukiniAdmin RBAC, MFA, sessions, rate limiting ; MiyukiniAdmin Origin (Argon2id, JWT). |
| **Rust / code** | unsafe_code = "forbid" sur tous les crates ; GovernedContext dans les toolkits. |
| **MWS transport** | TLS obligatoire contrôle ; replay ±10 s ; tokens ; certificate pinning Origin ; MAC DATA. |
| **MWS relay/tracker** | Auth token/secret ; clé Cores ; Phase B MIP ; liste trackers signée ; Permis ; pools par version ; systèmes passifs/actifs Tracker. |
| **MWS référentiel** | R-001 à R-015 (HA, DDoS, signatures, pinning, quarantaine, DNSSEC, fuzzing). |
| **Chaîne de confiance** | CODE → MSCM → MIP → GRAPH → STA → OSV ; contrat MIP. |
| **Kernel** | Pas de sécurité active ; primitives (config, id, time, log) ; invariants INV-K-1 à INV-K-10. |

---

## 11. Index des documents de sécurité

### Fondation et architecture
- [Security - Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md)
- [Security - Gouvernance Cores Protection Données](../foundation/Security%20-%20Gouvernance%20Cores%20Protection%20Donnees.md)
- [Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md)
- [Security - Core Integration Map](../architecture/Security%20-%20Core%20Integration%20Map.md)

### Référence conceptuelle
- [Doctrine Sécurité Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)
- [Security Protocols](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md)

### Contrats
- [Kernel - Security Boundaries Contract](../../kernel/contracts/Kernel%20-%20Security%20Boundaries%20Contract.md)
- [Security - MIP Security Contract](../contracts/governance/Security%20-%20MIP%20Security%20Contract.md)
- [Security - Invariants & Guarantees](../contracts/governance/Security%20-%20Invariants%20&%20Guarantees.md)
- [Security - Violations & Anti-Patterns](../contracts/governance/Security%20-%20Violations%20&%20Anti-Patterns.md)
- Contrats Cores : StrongFather, BorderGuard, WorrySentinel, TAMR, CaringNanny, EverBuddy, BondingBrother (docs/cores/*/contracts/security/)
- Contrats MiyukiniAdmin : Permission, Security Level Management (docs/admin/MiyukiniAdmin/contracts/security/)
- Contrats MWS Tracker : Passive Systems, Active Systems (docs/tools/MiyuWebwayTracker/contracts/security/)

### MWS
- [MWS - Contre-Mesures de Sécurité](../../miyukini-webway-system/securite/MWS%20-%20Contre-Mesures%20de%20Securite.md)
- [MWS - Chiffrement et TLS](../../miyukini-webway-system/securite/MWS%20-%20Chiffrement%20et%20TLS.md)
- [MWS - Audit de Sécurité Complet](../../miyukini-webway-system/securite/MWS%20-%20Audit%20de%20Securite%20Complet.md)
- [MWS - MiyukiniAdmin](../../miyukini-webway-system/administration/MWS%20-%20MiyukiniAdmin.md)
- Index MWS : [reference/_index.md](../../miyukini-webway-system/reference/_index.md)

---

**Version :** 1.0  
**Date :** 2026-02-14  
**Classification :** Documentation Security — Référence  
**Statut :** Liste normative des mesures de sécurité Miyukini COG et MWS
