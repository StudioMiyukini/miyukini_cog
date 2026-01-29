# MiyukiniAdmin — Implementation Security and Controls

## 1. Contexte

Ce document definit l'**implementation** des fonctionnalites de **controles et de securite** evoquees dans les contrats MiyukiniAdmin : detection d'etat de l'environnement (vierge, initialise, compromis), verrou StrongFather, reponse securitaire, recovery automatique, sauvegarde pre-destruction, authentification et autorisation (RBAC).

**References contractuelles :**
- [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md)
- [Environment Identity Protocol EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)

---

## 2. Portee / Scope

Ce document definit :
- Les **modules et services** a implementer (environment_state, auth, permission, recovery, backup_pre_destruction)
- Les **structures de donnees** et **algorithmes** (detection etat, verification integrite, recovery, destruction)
- Les **points d'integration** (main, routes, middleware, BondingBrother, KindMother, StrongFather, WorrySentinel)
- La **configuration** et les **politiques** (seuils, delais, formats)
- Les **criteres de validation** (tests, conformite aux contrats)

Ce document **ne couvre pas** :
- Le code source complet (voir Reference Implementation Guidelines)
- Les specifications de tests detaillees (voir contrats testing)

---

## 3. Modules et services a implementer

### 3.1 Vue d'ensemble

| Module / Service | Role | Contrat de reference |
|------------------|------|------------------------|
| **EnvironmentStateService** | Detection etat (VIERGE, INITIALISE, COMPROMIS), verification integrite EIP/registre/schema | Auth and First-Boot 3.1 à 3.5 |
| **BootstrapLockService** | Application / levée du verrou StrongFather (bootstrap lock) | Auth and First-Boot 4 |
| **InstallationFlowService** | Parcours Futur Admin, creation premier compte, EIP | Auth and First-Boot 5 |
| **SecurityResponseService** | Reponse securitaire (mode degrade, page « Environnement compromis ») | Auth and First-Boot 3.5 |
| **AutoRecoveryService** | Recovery/rollback automatique, declenchement, succes/echec | Auth and First-Boot 3.5.4 |
| **PreDestructionBackupService** | Sauvegarde compressee avant destruction (si pas de sauvegarde locale antérieure) | Auth and First-Boot 3.5.4.3 |
| **DestructionAndReinitService** | Destruction DB, reinitialisation vierge, memoire de corruption | Auth and First-Boot 3.5.4.3, 3.5.4.4 |
| **AuthService** | Login, MFA, session, rate limiting, registre comptes admin | Authentication Contract |
| **PermissionService** | RBAC, roles (Admin, Recovery, Audit), verification capacites | Permission Contract |

### 3.2 Structure backend recommandee

```
miyukini_admin/
├── src/
│   ├── main.rs                    # Demarrage + detection etat + routage (login / installation / compromis / dashboard)
│   ├── config.rs
│   ├── api_routes.rs
│   ├── api_handlers.rs
│   ├── services/
│   │   ├── environment_state.rs   # EnvironmentStateService
│   │   ├── bootstrap_lock.rs      # BootstrapLockService (ou integre StrongFather via bridge)
│   │   ├── installation_flow.rs   # InstallationFlowService
│   │   ├── security_response.rs   # SecurityResponseService
│   │   ├── auto_recovery.rs       # AutoRecoveryService
│   │   ├── pre_destruction_backup.rs # PreDestructionBackupService
│   │   ├── destruction_reinit.rs  # DestructionAndReinitService
│   │   ├── auth.rs                # AuthService (login, MFA, session, rate limit)
│   │   ├── permission.rs           # PermissionService (RBAC, check capability)
│   │   ├── backup_service.rs      # Deja existant ; etendre pour backup pre-destruction
│   │   ├── security_service.rs    # Deja existant ; etendre pour niveaux + reponse securitaire
│   │   └── ...
│   ├── models/
│   │   ├── environment_state.rs   # Enum VIERGE | INITIALISE | COMPROMIS
│   │   ├── admin_account.rs       # Compte admin (account_id, username, password_hash, role, mfa_*)
│   │   ├── session.rs             # Session (session_id, account_id, role, expires_at, ip, user_agent)
│   │   └── corruption_memory.rs   # Memoire de corruption passee (timestamp, reason)
│   ├── middleware/
│   │   ├── auth_middleware.rs     # Verification session, redirection login si invalide
│   │   ├── permission_middleware.rs # Verification capacite avant action
│   │   └── rate_limit_middleware.rs  # Rate limiting login / IP
│   └── audit_logger.rs
```

---

## 4. Detection d'etat de l'environnement

### 4.1 Algorithme (EnvironmentStateService)

**Entree :** Acces KindMother (ou stockage local) pour blob EIP, registre admin, schema bootstrap.

**Sortie :** `EnvironmentState { Vierge | Initialise | Compromis }` + indicateurs optionnels (pour audit).

**Etapes :**

1. **Presence des artefacts**
   - Verifier existence blob EIP (KindMother ou slot dedie).
   - Verifier existence registre admin (au moins une entree ou fichier/table presente).
   - Verifier existence schema bootstrap (tables noyau presentes).

2. **Si aucun artefact present** (tout absent ou vide coherent) → **VIERGE**.

3. **Si au moins un artefact present** : verification **integrite** et **coherence**.
   - **EIP** : recuperer blob ; avec cle derivee (secret bootstrap), dechiffrer ; verifier tag AEAD ; verifier `integrity_hash` vs etat courant (ordre chargement, contrats) ; verifier `protocol_version` supporte. Si tag invalide ou hash incoherent → **COMPROMIS**.
   - **Registre admin** : structure valide ; au moins un compte non revoque et coherent (hash lisible). Si structure corrompue ou aucun compte valide alors que flag « environnement initialise » present → **COMPROMIS**.
   - **Schema** : checksum ou structure complete (tables attendues, colonnes). Si tronque ou incoherent → **COMPROMIS**.
   - **Coherence globale** : EIP indique iteration/version alors que registre vide ; ou schema present sans blob EIP ; etc. → **COMPROMIS**.

4. **Si tout present et valide** → **INITIALISE**.

5. **Si presence mais invalide ou incoherent** → **COMPROMIS**.

**Implementation (pseudo-code) :**

```rust
// environment_state.rs
pub enum EnvironmentState { Vierge, Initialise, Compromis }

pub struct EnvironmentStateService { ... }

impl EnvironmentStateService {
    pub async fn detect(&self) -> Result<EnvironmentState, Error> {
        let has_eip = self.storage.has_eip_blob().await?;
        let has_admin_registry = self.storage.has_admin_registry().await?;
        let has_bootstrap_schema = self.storage.has_bootstrap_schema().await?;

        if !has_eip && !has_admin_registry && !has_bootstrap_schema {
            return Ok(EnvironmentState::Vierge);
        }

        if has_eip {
            let valid = self.verify_eip_integrity().await?;
            if !valid { return Ok(EnvironmentState::Compromis); }
        }
        if has_admin_registry {
            let valid = self.verify_admin_registry().await?;
            if !valid { return Ok(EnvironmentState::Compromis); }
        }
        if has_bootstrap_schema {
            let valid = self.verify_schema_integrity().await?;
            if !valid { return Ok(EnvironmentState::Compromis); }
        }

        self.verify_global_consistency().await?;
        Ok(EnvironmentState::Initialise)
    }
}
```

### 4.2 Point d'appel

- **Au demarrage** de MiyukiniAdmin (dans `main` ou `server::run`) : appeler `EnvironmentStateService::detect()` une fois ; stocker le resultat dans l'etat applicatif (ex. `AppState::environment_state`).
- **Routage** : selon l'etat, servir la route appropriee (voir section 8).

---

## 5. Verrou StrongFather (bootstrap lock)

### 5.1 Principe

Lorsque l'etat est **VIERGE**, StrongFather applique un **verrou bootstrap** : seuls MiyukiniAdmin et les Cores peuvent effectuer des actions. Les Operateurs (Strate 7) et Outils/Kits (Strate 6) sont bloques.

**Implementation :**

- **Cote MiyukiniAdmin** : ne pas exposer d'actions metier (dashboard metier, liste Operateurs metier) tant que l'etat est VIERGE ; exposer uniquement le **parcours d'installation** (routes `/setup`, `/setup/eip`, `/setup/config`, `/setup/create-admin`, etc.).
- **Cote StrongFather (via BondingBrother)** : la politique « bootstrap lock » est activee lorsque l'environnement est vierge ; toute requete dont la source n'est pas MiyukiniAdmin ou un Core est refusee. L'implementation peut etre un flag dans la config StrongFather ou une decision conditionnelle (si `environment_state == Vierge` alors refuser les requetes Operateurs).

**Levée du verrou :** Lors de la finalisation du parcours d'installation (compte admin cree, EIP genere, config minimale validee), MiyukiniAdmin enregistre « environnement initialise » et demande a StrongFather (via BondingBrother) de desactiver la politique bootstrap lock.

---

## 6. Parcours Futur Admin (installation)

### 6.1 Routes et UI

- **Route** : `/setup` (ou `/installation`) lorsque `environment_state == Vierge`.
- **Redirection** : Si l'utilisateur accede a `/` ou `/dashboard` et que l'etat est VIERGE, rediriger vers `/setup`.
- **Etapes UI** : Accueil installation → Generation EIP (appel BondingBrother/Cores) → Configuration minimale → Creation compte admin (formulaire username, mot de passe, MFA) → Finalisation (enregistrement « environnement initialise », levée verrou) → Redirection vers `/dashboard`.

### 6.2 Creation compte admin

- **Formulaire** : username (unique), mot de passe (politique : longueur ≥ 12, complexite), MFA (TOTP ou WebAuthn).
- **Stockage** : hash mot de passe (Argon2id), role = Admin par defaut, secret MFA chiffre (AES-256-GCM) si TOTP.
- **Audit** : evenement `FIRST_ADMIN_CREATED` avec timestamp, account_id (pas de mot de passe ni secret).

### 6.3 Generation EIP

- MiyukiniAdmin envoie une requete « generer identite environnement » via BondingBrother.
- Les Cores (Kernel + Cores) produisent le payload EIP ; chiffrement (AEAD) ; KindMother persiste le blob.
- Succes → etape suivante du parcours. Echec → message explicite, pas de destruction.

---

## 7. Reponse securitaire (environnement compromis)

### 7.1 Mesures immediates

- **WorrySentinel** : demander (via BondingBrother) le passage en etat **T3** ou **T4** (mode degrade / lockdown).
- **Page dediee** : pour toute requete UI lorsque `environment_state == Compromis`, servir une page « Environnement compromis » (pas de formulaire login, pas de lien vers parcours installation). Message : environnement en etat de securite, procedure de recovery requise (ou recovery automatique en cours selon politique).
- **Audit** : enregistrer `ENVIRONMENT_COMPROMISED` avec timestamp et indicateurs (EIP invalide, registre incoherent, etc.).
- **Alerte** : selon politique (log, notification).

### 7.2 Routage

- Si `environment_state == Compromis` : toutes les routes (sauf peut-etre `/health`) renvoient la page « Environnement compromis » ou un JSON `{ "status": "compromised" }` pour l'API.

---

## 8. Recovery automatique (interface compromise, humain ne peut pas intervenir)

### 8.1 Declenchement

- **Condition** : `environment_state == Compromis` **et** (option politique) impossibilite d'intervention humaine : ex. delai sans login reussi, ou detection explicite « interface compromise » (auth/registre admin/MiyukiniAdmin alteres).
- **Implementation** : au demarrage, apres avoir detecte COMPROMIS, verifier si une intervention humaine est possible (ex. au moins un compte admin valide et login possible). Si non (ou apres timeout), lancer **AutoRecoveryService::run()**.

### 8.2 Algorithme AutoRecoveryService

1. **Recovery/rollback** : tenter restauration depuis backup local (si existe et valide) ou reparation des artefacts (EIP, registre, schema). Criteres de succes : integrite EIP retablie, au moins un compte admin valide ou parcours Futur Admin accessible, schema coherent.
2. **Si succes** : mettre a jour `environment_state` vers INITIALISE (ou VIERGE si rollback pre-initialisation) ; lever le mode degrade ; arreter.
3. **Si echec** : appeler **PreDestructionBackupService** puis **DestructionAndReinitService** (voir sections 9 et 10).

### 8.3 Point d'appel

- Dans `main` ou un task dedie : apres `detect()` si `Compromis`, evaluer « humain peut intervenir » ; si non, lancer `AutoRecoveryService::run().await`.

---

## 9. Sauvegarde pre-destruction (si pas de sauvegarde locale antérieure)

### 9.1 Principe

Avant de detruire les donnees DB (echec de la recovery automatique), **si aucune sauvegarde locale antérieure** des donnees DB n'existe : effectuer une **sauvegarde** des donnees DB, la **compresser** (zip, tar.gz, etc.), la stocker dans un emplacement dedie (ex. `data/backups/pre_destruction_<timestamp>.tar.gz`).

### 9.2 Algorithme PreDestructionBackupService

1. **Verifier** : lister les sauvegardes locales existantes (ex. repertoire `data/backups/` ou table des backups). Si au moins une sauvegarde locale **anterieure** (avant ce cycle de recovery) existe → **ne pas** creer de nouvelle sauvegarde ; passer a la destruction.
2. **Si aucune sauvegarde locale antérieure** :
   - Exporter les donnees DB (dump SQL ou export KindMother selon protocole).
   - Compresser (ex. `flate2` + tar, ou `zip`).
   - Stocker dans un fichier dedie (ex. `data/backups/pre_destruction_<timestamp>.tar.gz`).
   - Audit : `PRE_DESTRUCTION_BACKUP_CREATED` avec chemin et taille (pas de contenu).
3. **Poursuivre** : appeler DestructionAndReinitService.

### 9.3 Implementation (pseudo-code)

```rust
// pre_destruction_backup.rs
impl PreDestructionBackupService {
    pub async fn run_if_needed(&self) -> Result<(), Error> {
        let has_prior_local = self.backup_service.has_prior_local_backup().await?;
        if has_prior_local {
            return Ok(());
        }
        let dump = self.database_service.dump_all().await?;
        let compressed = self.compress(dump)?;
        let path = self.storage.store_pre_destruction_backup(compressed).await?;
        self.audit.log(PRE_DESTRUCTION_BACKUP_CREATED { path, ... }).await?;
        Ok(())
    }
}
```

---

## 10. Destruction DB et reinitialisation (vierge avec memoire de corruption)

### 10.1 Algorithme DestructionAndReinitService

1. **Destruction** : purge complete des donnees DB (wipe tables, suppression fichiers de stockage DB). Ne pas toucher au stockage de la **memoire de corruption** (fichier ou slot dedie hors DB).
2. **Memoire de corruption** : ecrire (ou mettre a jour) une structure persistante (ex. fichier `data/corruption_memory.json` ou slot KindMother dedie non efface) : `{ "reinitialised_at": "<timestamp>", "reason": "recovery_automatic_failed", "previous_state": "compromised" }`. Pas de donnees utilisateur ni de secrets.
3. **Reinitialisation** : supprimer ou invalider blob EIP, registre admin, schema bootstrap (ou recree vide). L'etat applicatif passe a **VIERGE**.
4. **Flag** : l'environnement est desormais **vierge avec memoire de corruption** ; au prochain demarrage, `detect()` retournera VIERGE ; la memoire de corruption peut etre lue pour afficher un message ou alerte au Futur Admin.

### 10.2 Implementation (pseudo-code)

```rust
// destruction_reinit.rs
impl DestructionAndReinitService {
    pub async fn run(&self) -> Result<(), Error> {
        self.pre_destruction_backup.run_if_needed().await?;
        self.database_service.wipe_all().await?;
        self.corruption_memory.write(CorruptionMemory {
            reinitialised_at: Utc::now(),
            reason: "recovery_automatic_failed",
        }).await?;
        self.storage.remove_eip_blob().await?;
        self.storage.remove_admin_registry().await?;
        self.storage.recreate_bootstrap_schema_empty().await?;
        self.audit.log(ENVIRONMENT_REINITIALISED_VIERGE_WITH_MEMORY).await?;
        Ok(())
    }
}
```

---

## 11. Authentification (AuthService)

### 11.1 Registre comptes admin

- **Stockage** : table ou fichier protege (hors DB metier si besoin) avec champs : `account_id`, `username`, `password_hash`, `role`, `mfa_enabled`, `mfa_secret_encrypted`, `created_at`, `updated_at`, `last_login_at`, `locked_until`, `failed_attempts`.
- **Hash mot de passe** : Argon2id (ou bcrypt) avec parametres de cout suffisants.

### 11.2 Login

- **Route** : `POST /api/auth/login` (body : username, password).
- **Verification** : rate limit (par IP et par compte) ; recherche compte par username ; verification hash mot de passe ; si MFA actif, reponse `{ "require_mfa": true }` et enregistrement d'un token temporaire ; sinon creation session, retour token/session_id.
- **Reponses generiques** : en cas d'echec, toujours « Identifiants invalides » (pas de fuite d'information).

### 11.3 MFA

- **TOTP** : enrollment (generation secret, affichage QR) ; stockage secret chiffre (AES-256-GCM). Challenge : `POST /api/auth/mfa/verify` avec code 6 chiffres.
- **WebAuthn** : enrollment et challenge selon standard WebAuthn ; pas de secret stocke cote serveur.

### 11.4 Session

- **Creation** : apres login reussi (et MFA si requis), creer une session (session_id, account_id, role, expires_at, ip, user_agent). Stocker en base ou cache (ex. Redis) avec TTL.
- **Token** : cookie HttpOnly Secure SameSite (ou header Authorization) contenant ou reference session_id.
- **Validation** : a chaque requete protegee, verifier session existante, non expiree, optionnellement binding IP/User-Agent.
- **Timeout** : ex. 15 min inactivite, 8 h absolu.

### 11.5 Rate limiting

- **Par compte** : apres N echecs (ex. 5) en fenetre (ex. 5 min), verrouiller le compte (`locked_until = now + 15 min`).
- **Par IP** : apres N echecs (ex. 10) en fenetre, bloquer l'IP (retourner 429 ou page « Trop de tentatives »).
- **Implementation** : middleware ou compteur en memoire/fichier par IP et par account_id.

---

## 12. Autorisation (PermissionService, RBAC)

### 12.1 Roles et capacites

- **Roles** : enum `Admin | Recovery | Audit`.
- **Capacites** : liste explicite (ex. `admin.dashboard.read`, `admin.security.level.write`, `admin.db.recovery`, etc.) selon [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md).
- **Matrice** : fonction ou table `role_has_capability(role, capability) -> bool` implementant la matrice du contrat (Admin = toutes sauf recovery et creation Recovery ; Recovery = Admin + recovery + gestion Recovery ; Audit = lecture seule).

### 12.2 Verification avant action

- **Middleware** : pour chaque route protegee, associer une **capacite requise** (ex. `GET /api/security/level` → `admin.security.level.read`). Dans le middleware, recuperer le role de la session ; appeler `PermissionService::has_capability(role, capability)` ; si non, retourner 403.
- **Cote handler** : pour les actions a validation StrongFather, apres verification permission, envoyer une demande de decision a StrongFather via BondingBrother ; si DENIED, retourner 403 ou message « Action refusee par la gouvernance ».

### 12.3 UI

- **Navigation** : filtrer les liens du menu selon les capacites du role (ex. masquer « Recovery » pour Admin, masquer « Changement niveau securite » pour Audit).
- **Routes** : les routes cote serveur doivent refuser (403) si la capacite est absente, meme en cas d'acces direct par URL.

---

## 13. Integration (main, routes, middleware)

### 13.1 Sequence au demarrage (main)

1. Charger la configuration.
2. Initialiser le Kernel (Id, Logger, Clock, Config, Lifecycle).
3. **Detecter l'etat** : `EnvironmentStateService::detect()` → `state`.
4. Si **VIERGE** : configurer les routes pour `/setup` uniquement (et health) ; activer le verrou StrongFather (via bridge ou config).
5. Si **INITIALISE** : configurer les routes normales (dashboard, API) ; middleware auth sur les routes protegees ; pas de verrou.
6. Si **COMPROMIS** : configurer la page « Environnement compromis » ; demander a WorrySentinel T3/T4 ; evaluer « humain peut intervenir » ; si non, lancer en arriere-plan `AutoRecoveryService::run()` (ou apres un delai selon politique).
7. Démarrer le serveur HTTP.

### 13.2 Routes selon etat

| Etat | Routes accessibles |
|------|---------------------|
| **VIERGE** | `/setup`, `/setup/*`, `/health` |
| **INITIALISE** | `/`, `/dashboard`, `/database`, `/tests`, `/api/*` (avec auth + permission selon role), `/login` |
| **COMPROMIS** | `/` → page « Environnement compromis », `/health` (optionnel) |

### 13.3 Middleware chain

- **Rate limit** : sur `POST /api/auth/login` (et eventuellement autres endpoints sensibles).
- **Auth** : sur toutes les routes `/api/*` sauf `/api/auth/login` et `/api/status` (si public). Si session invalide ou absente → 401 ou redirection vers `/login`.
- **Permission** : sur chaque route protegee, verification de la capacite requise ; si absente → 403.

---

## 14. Configuration et politique

### 14.1 Parametres recommandes

| Parametre | Valeur | Description |
|-----------|--------|-------------|
| **auth.password_min_length** | 12 | Longueur minimale mot de passe |
| **auth.session_idle_timeout_secs** | 900 | 15 min |
| **auth.session_absolute_timeout_secs** | 28800 | 8 h |
| **auth.rate_limit_failures_per_account** | 5 | Verrouillage compte |
| **auth.rate_limit_failures_per_ip** | 10 | Blocage IP |
| **auth.lockout_duration_secs** | 900 | 15 min |
| **recovery.auto_recovery_delay_secs** | 300 | Delai avant recovery automatique (optionnel) |
| **backup.pre_destruction_compression** | "tar.gz" | Format compression |

### 14.2 Politique « humain ne peut pas intervenir »

- **Option A** : des que l'etat est COMPROMIS, considerer que l'humain ne peut pas intervenir (recovery automatique immediate).
- **Option B** : attendre un delai (ex. 5 min) sans aucun login reussi ; si aucun login, lancer la recovery automatique.
- **Option C** : verifier explicitement l'absence de compte admin valide (registre corrompu ou vide) ; si aucun compte valide, lancer la recovery automatique.

L'implementation doit documenter la politique choisie et la rendre configurable si possible.

---

## 15. Criteres de validation et tests

### 15.1 Conformite aux contrats

- **Auth and First-Boot** : detection VIERGE/INITIALISE/COMPROMIS conforme aux criteres ; verrou bootstrap applique en VIERGE ; parcours installation uniquement en VIERGE ; reponse securitaire en COMPROMIS ; recovery automatique lorsque interface compromise et humain ne peut pas intervenir ; sauvegarde pre-destruction si pas de sauvegarde locale antérieure ; destruction et reinit avec memoire de corruption.
- **Authentication** : registre admin, login, MFA, session, rate limiting, audit des evenements auth conformes au contrat.
- **Permission** : roles Admin/Recovery/Audit, matrice capacites, verification avant action, pas d'escalade conformes au contrat.

### 15.2 Tests recommandes

- **Unitaire** : `EnvironmentStateService::detect()` avec mocks (EIP absent/present valide/present invalide, registre vide/present valide/corrompu, schema vide/present tronque).
- **Unitaire** : `PreDestructionBackupService::run_if_needed()` avec et sans sauvegarde locale antérieure.
- **Unitaire** : `PermissionService::has_capability(role, capability)` pour toutes les paires (role, capacite) de la matrice.
- **Integration** : demarrage avec DB vide → etat VIERGE, route `/setup` accessible.
- **Integration** : demarrage avec EIP invalide (tag corrompu) → etat COMPROMIS, page « Environnement compromis » servie.
- **Integration** : login avec rate limit → verrouillage compte apres N echecs.

---

## 16. Documents associes

- [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md)
- [Reference Implementation Guidelines](./MiyukiniAdmin%20-%20Reference%20Implementation%20Guidelines.md)
- [Environment Identity Protocol EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document d'implementation — Controles et securite
