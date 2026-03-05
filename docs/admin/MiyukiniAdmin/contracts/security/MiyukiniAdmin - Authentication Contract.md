# MiyukiniAdmin â€” Authentication Contract

## 1. Contexte

Ce document definit le **contrat d'authentification** integre a MiyukiniAdmin. L'authentification est le mecanisme qui assure que seul un **compte admin autorise** peut acceder a la console. Elle est independante des Operateurs metier et des Mandats de Permission (StrongFather).

**Principe fondamental :**

> **Aucun acces a la console MiyukiniAdmin sans authentification forte (identifiant + mot de passe + MFA). Aucune exception en environnement initialise.**

**References :**
- [MiyukiniAdmin - Auth and First-Boot Contract](./MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md)
- [MiyukiniAdmin - Threat Model Contract](./MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)

---

## 2. Portee / Scope

Ce document definit :
- Le **registre des comptes admin** (schema, stockage, cycle de vie)
- Le **flux de login** (identifiant, mot de passe, verification, MFA)
- La **gestion des sessions** (creation, binding, timeout, revocation)
- La **politique de mot de passe** (complexite, rotation, reset)
- Le **MFA** (TOTP, cle materielle, enrollment, challenge)
- Le **rate limiting** et la protection contre les intrusions
- Le **stockage des secrets** (hash, MFA, jamais en clair)
- L'**audit des evenements d'auth** (login, MFA, session)

Ce document **ne couvre pas** :
- L'autorisation (roles, permissions) â€” voir [Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md)
- Le premier demarrage (Futur Admin, parcours installation) â€” voir [Auth and First-Boot Contract](./MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- L'identite environnement (EIP) â€” voir [Environment Identity Protocol EIP](..//..//..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)

---

## 3. Registre des comptes admin

### 3.1 Principe

MiyukiniAdmin maintient un **registre des comptes admin** : liste des identites autorisees a utiliser la console. Ce registre est stocke de maniere securise, **hors portee des Operateurs metier** (KindMother peut persister les donnees sous autorite MiyukiniAdmin, mais le schema et l'acces sont controles par MiyukiniAdmin).

### 3.2 Donnees par compte

| Champ | Type | Obligatoire | Description | Stockage |
|-------|------|-------------|-------------|----------|
| **account_id** | UUID | Oui | Identifiant unique du compte (genere par Kernel Id). | Clair |
| **username** | String | Oui | Identifiant de connexion (unique, immutable apres creation). | Clair |
| **password_hash** | String | Oui | Hash du mot de passe (Argon2id ou bcrypt). | Protege |
| **role** | Enum | Oui | Role MiyukiniAdmin (Admin, Recovery, Audit). | Clair |
| **mfa_enabled** | Boolean | Oui | MFA active ou non. | Clair |
| **mfa_secret_encrypted** | Blob | Conditionnel | Secret TOTP chiffre (si MFA TOTP). | Protege |
| **mfa_backup_codes_encrypted** | Blob | Optionnel | Codes de secours chiffres. | Protege |
| **created_at** | Timestamp | Oui | Date de creation du compte. | Clair |
| **updated_at** | Timestamp | Oui | Derniere mise a jour. | Clair |
| **last_login_at** | Timestamp | Optionnel | Derniere connexion reussie. | Clair |
| **locked_until** | Timestamp | Optionnel | Fin du blocage (rate limiting). | Clair |
| **failed_attempts** | Integer | Oui | Nombre d'echecs consecutifs (remis a zero au succes). | Clair |

**Regle :** Le mot de passe et les secrets MFA ne sont **jamais** stockes en clair. Uniquement des hashes ou blobs chiffres.

### 3.3 Cycle de vie

| Evenement | Action |
|-----------|--------|
| **Creation** | Premier compte lors du parcours d'installation (Futur Admin) ; comptes suivants par un admin ayant la capacite `admin.accounts.create`. |
| **Modification** | Changement mot de passe, activation/desactivation MFA, changement role â€” par l'admin lui-meme (mot de passe, MFA) ou par un admin avec `admin.accounts.write`. |
| **Verrouillage** | Automatique apres N echecs de login (rate limiting) ; manuel par un admin avec `admin.accounts.lock`. |
| **Deverrouillage** | Automatique apres delai ; manuel par un admin avec `admin.accounts.unlock`. |
| **Revocation** | Desactivation du compte (plus de login) â€” par un admin avec `admin.accounts.revoke`. Pas de suppression physique immediate (audit). |

---

## 4. Flux de login

### 4.1 PrÃ©requis

- Environnement **initialise** (pas en mode Futur Admin).
- Compte admin existant, non revoque, non verrouille (ou verrou expire).

### 4.2 Ã‰tapes

1. **Acces UI** : Utilisateur ouvre l'UI MiyukiniAdmin (ex. `/login`).
2. **Saisie identifiant + mot de passe** : Formulaire ; pas de pre-remplissage.
3. **Verification rate limit** : Si l'adresse IP ou le compte est bloque (trop d'echecs), reponse generique Â« Compte temporairement indisponible Â» sans indiquer si le compte existe.
4. **Verification compte** : Recherche par `username` ; si absent ou revoque â†’ echec generique Â« Identifiants invalides Â» (pas de fuite d'information).
5. **Verification mot de passe** : Comparaison avec `password_hash` (Argon2id/bcrypt verify). Si echec â†’ increment `failed_attempts`, potentiel verrouillage, audit, reponse generique.
6. **Challenge MFA** (si `mfa_enabled`) : Affichage formulaire TOTP ou demande cle materielle. Verification du code/cle. Si echec â†’ audit, reponse generique.
7. **Succes** : Remise a zero `failed_attempts`, mise a jour `last_login_at`, **creation de session** (voir section 5), redirection vers le dashboard. Audit Â« login success Â».

### 4.3 Reponses generiques (anti-fuite)

| Cas | Reponse utilisateur | Audit |
|-----|----------------------|-------|
| Compte inexistant | Â« Identifiants invalides Â» | Tentative login (username tente, pas de compte) |
| Mot de passe incorrect | Â« Identifiants invalides Â» | Tentative login (account_id, echec mot de passe) |
| MFA incorrect | Â« Code invalide Â» ou Â« Identifiants invalides Â» | Tentative login (account_id, echec MFA) |
| Compte verrouille | Â« Compte temporairement indisponible Â» | Tentative login (account_id, compte verrouille) |
| Compte revoque | Â« Identifiants invalides Â» | Tentative login (account_id revoque) |

**Regle :** Ne jamais reveler si un username existe ou non ; ne jamais exposer de detail technique dans l'UI.

---

## 5. Gestion des sessions

### 5.1 Creation de session

Apres login reussi, MiyukiniAdmin cree une **session** :

| Champ | Description |
|-------|-------------|
| **session_id** | Identifiant unique (UUID, aleatoire). |
| **account_id** | Compte associe. |
| **role** | Role du compte (snapshot au moment de la creation). |
| **created_at** | Horodatage creation. |
| **expires_at** | Horodatage expiration (voir 5.3). |
| **ip_address** | Adresse IP du client (binding). |
| **user_agent** | User-Agent du client (binding). |

Le **token de session** (ex. cookie HttpOnly Secure SameSite, ou token dans header) contient ou reference `session_id`. Le mot de passe et les secrets MFA ne sont jamais inclus.

### 5.2 Binding et validation

A chaque requete authentifiee :

1. **Lecture du token** : Recuperation du `session_id` (cookie ou header).
2. **Recherche session** : Session existante, non expiree, non revoquee.
3. **Binding** : Verification optionnelle que l'IP et/ou User-Agent n'ont pas change (politique configurable : strict = refus si changement ; souple = alerte mais accepte).
4. **Renouvellement** : Si Â« sliding Â» activÃ©, mise a jour de `expires_at` dans la fenetre autorisee.

Si une etape echoue â†’ session invalide, redirection vers `/login`, audit Â« session invalid Â».

### 5.3 Timeout et expiration

| Parametre | Valeur recommandee | Description |
|-----------|--------------------|-------------|
| **Timeout absolu** | 8 h | La session expire au plus tard 8 h apres creation. |
| **Timeout inactivite** | 15 min | Si aucune action pendant 15 min, session expire (sliding ou fixe selon config). |
| **Sliding** | Optionnel | Chaque requete authentifiee recule `expires_at` dans la limite du timeout absolu. |

**Regle :** Un timeout court reduit le risque de vol de session ; un timeout trop court degrade l'UX. Les valeurs ci-dessus sont des recommandations contractuelles ; l'implementation peut les rendre configurables.

### 5.4 Revocation

Une session peut etre revoquee :

- **Manuellement** : L'admin se deconnecte (bouton Â« Deconnexion Â») â†’ session supprimee ou marquee invalide.
- **Par un autre admin** : Un admin avec la capacite `admin.accounts.revoke_session` peut invalider une session cible (ex. pour bloquer un compte compromis).
- **Automatiquement** : Lors du changement de mot de passe ou de la revocation du compte â†’ toutes les sessions de ce compte sont invalidees.

---

## 6. Politique de mot de passe

### 6.1 Exigences minimales

| Contrainte | Valeur minimale |
|------------|-----------------|
| **Longueur** | 12 caracteres |
| **Complexite** | Au moins une majuscule, une minuscule, un chiffre, un caractere special (liste definie : ex. `!@#$%^&*()`) |
| **Pas dans dictionnaire** | Mot de passe non present dans une liste de mots courants (optionnel mais recommande) |
| **Pas de reuse recent** | Les N derniers mots de passe (ex. 5) ne peuvent pas etre reutilises (optionnel) |

### 6.2 Hash

- **Algorithme** : Argon2id (recommandÃ©) ou bcrypt.
- **Parametres** : CoÃ»t suffisant (ex. Argon2id : memory 64 MiB, iterations 3 ; bcrypt : cost 12). Aucun sel en clair dans les logs.

### 6.3 Rotation

- **Rotation obligatoire** : Optionnel par politique (ex. tous les 90 jours). Si active, au-dela du delai le compte exige un nouveau mot de passe au prochain login.
- **Reset** : Un admin avec `admin.accounts.reset_password` peut forcer un reset pour un compte cible ; l'utilisateur doit definir un nouveau mot de passe au prochain login (lien ou flux dedie avec token temporaire).

### 6.4 Premier compte (parcours installation)

Lors de la creation du **premier** compte admin (Futur Admin), les memes exigences de complexite et de hash s'appliquent. Le mot de passe n'est jamais stocke en clair.

---

## 7. MFA (Multi-Factor Authentication)

### 7.1 Obligation

- **Admin** et **Recovery** : MFA **obligatoire** (pas de compte actif sans MFA apres enrollment).
- **Audit** : MFA **recommandÃ©** (peut etre impose par politique).

### 7.2 Methodes supportees

| Methode | Description | Stockage secret |
|---------|-------------|-----------------|
| **TOTP** | Application authentificator (ex. Google Authenticator, Authy). Secret partage au moment de l'enrollment. | Secret chiffre (AES-256-GCM) avec cle derivee du secret bootstrap ou cle dediee. |
| **Cle materielle** | WebAuthn / FIDO2 (cle USB, etc.). Pas de secret stocke cote serveur. | Credential ID + public key ; pas de secret. |

### 7.3 Enrollment TOTP

1. Admin (ou premier admin) initie Â« Activer MFA Â».
2. Serveur genere un secret TOTP (aleatoire), affiche QR code + code manuel.
3. Utilisateur scanne avec l'app et saisit un premier code pour prouver la possession.
4. Si valide : stockage du secret **chiffre** ; `mfa_enabled = true`. Optionnel : generation de codes de secours (chiffres).
5. Audit Â« MFA enabled Â».

### 7.4 Enrollment WebAuthn

1. Admin initie Â« Ajouter cle de securite Â».
2. Navigateur declenche `navigator.credentials.create()` ; l'utilisateur enregistre la cle.
3. Serveur recoit la public key et le credential ID ; les stocke (pas de secret). Optionnel : plusieurs cles par compte.
4. Audit Â« WebAuthn credential registered Â».

### 7.5 Challenge au login

- **TOTP** : Utilisateur saisit le code a 6 chiffres ; serveur verifie avec le secret TOTP (fenetre de tolerance raisonnable, ex. Â±1 periode).
- **WebAuthn** : Serveur envoie challenge ; navigateur `navigator.credentials.get()` ; signature verifiee cote serveur.

Si echec : audit ; pas de fuite sur le fait que le compte existe.

---

## 8. Rate limiting et protection

### 8.1 Limites

| Seuil | Action |
|-------|--------|
| **N echecs par compte** (ex. 5) en fenetre (ex. 5 min) | Verrouillage du compte pendant D (ex. 15 min). Increment `failed_attempts` ; si atteint N, `locked_until = now + D`. |
| **N echecs par IP** (ex. 10) en fenetre (ex. 5 min) | Blocage temporaire de l'IP (ex. 15 min). Reponse HTTP 429 ou page Â« Trop de tentatives Â». |
| **Apres verrouillage** | Toute tentative de login pour ce compte ou cette IP retourne Â« Compte temporairement indisponible Â» ou 429 sans traiter les credentials. |

### 8.2 Alerte

- Lorsqu'un compte est verrouille (rate limit) : **audit** + optionnel **alerte** (log, notification interne) pour detection d'attaque.
- Lorsqu'une IP est bloquee : audit pour analyse.

### 8.3 Deverrouillage

- **Automatique** : Apres expiration de `locked_until`, le compte peut tenter Ã  nouveau (sous reserve que l'IP ne soit pas encore bloquee).
- **Manuel** : Un admin avec `admin.accounts.unlock` peut lever le verrou d'un compte.

---

## 9. Stockage des secrets

### 9.1 Mot de passe

- **Hash uniquement** (Argon2id ou bcrypt). Jamais de mot de passe en clair en base, en log, ou en memoire apres traitement.
- **Transport** : Formulaire en HTTPS (TLS 1.2+). Pas d'envoi en clair.

### 9.2 MFA

- **TOTP** : Secret chiffre (AES-256-GCM) avant persistance. Cle de chiffrement derivee (KDF) depuis un secret maÃ®tre (ex. stocke dans config securisee ou HSM). Pas de secret TOTP en clair.
- **WebAuthn** : Pas de secret cote serveur ; uniquement public key et credential ID.

### 9.3 Sessions

- **Token** : Valeur aleatoire (session_id) non previsible. Stockage cote serveur : session_id â†’ account_id, role, expires_at, etc. Pas de mot de passe ni de secret MFA dans le token.
- **Cookie** : Si cookie utilisÃ©, flags HttpOnly, Secure, SameSite=Strict (ou Lax selon contraintes). Pas de script accessible.

---

## 10. Audit des evenements d'authentification

### 10.1 Evenements traces

| Evenement | Champs traces (minimaux) |
|-----------|--------------------------|
| **Login success** | timestamp, account_id, session_id, ip, user_agent |
| **Login failure** | timestamp, username_tente (ou hash), ip, raison (password / MFA / locked / revoked) |
| **MFA enrollment** | timestamp, account_id, methode (TOTP / WebAuthn) |
| **MFA challenge failure** | timestamp, account_id, ip |
| **Session created** | timestamp, account_id, session_id |
| **Session revoked** | timestamp, session_id, raison (logout / admin / password change) |
| **Password change** | timestamp, account_id, initie_par (self / admin_id) |
| **Account locked (rate limit)** | timestamp, account_id, ip |
| **Account unlocked** | timestamp, account_id, initie_par (auto / admin_id) |

### 10.2 Retention

- **Succes** : Retention selon politique (ex. 1 an).
- **Echecs et verrouillages** : Retention plus longue (ex. 2 ans) pour analyse de securite.
- **Pas de stockage** des mots de passe ni des codes MFA dans les logs.

---

## 11. Invariants et garanties

| Code | Invariant |
|------|-----------|
| **INV-AUTH-C-1** | Aucun acces a la console sans authentification reussie (identifiant + mot de passe + MFA si requis). |
| **INV-AUTH-C-2** | Les mots de passe et secrets MFA ne sont jamais stockes en clair. |
| **INV-AUTH-C-3** | Toute tentative de login (succes ou echec) et toute revocation de session sont auditees. |
| **INV-AUTH-C-4** | Les reponses au login ne revelent pas l'existence ou l'etat d'un compte (reponse generique en cas d'echec). |
| **INV-AUTH-C-5** | Rate limiting actif : verrouillage compte et/ou IP apres N echecs. |
| **INV-AUTH-C-6** | Session liee (IP/User-Agent) et timeout court ; revocation possible. |

---

## 12. Documents associes

- [MiyukiniAdmin - Auth and First-Boot Contract](./MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md)
- [MiyukiniAdmin - Threat Model Contract](./MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Contrat normatif â€” Authentification MiyukiniAdmin


