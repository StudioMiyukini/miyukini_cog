# MiyukiniAdmin — Permission Contract (RBAC)

## 1. Contexte

Ce document definit le **contrat d'autorisation (permissions)** integre a MiyukiniAdmin. L'autorisation determine **ce qu'un compte admin authentifie peut faire** dans la console. Elle repose sur un modele **RBAC** (Role-Based Access Control) : chaque compte a **un seul role** ; chaque role possede un ensemble **explicite** de **capacites** (permissions).

**Principe fondamental :**

> **Un compte admin a un seul role. Les capacites sont explicites et minimales (moindre privilege). Aucune action sensible sans verification de permission.**

**References :**
- [MiyukiniAdmin - Auth and First-Boot Contract](./MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [MiyukiniAdmin - StrongFather Integration Contract](../integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)

---

## 2. Portee / Scope

Ce document definit :
- Les **roles** MiyukiniAdmin (Admin, Recovery, Audit)
- Le **catalogue des capacites** (permissions) associees aux actions de la console
- La **matrice role → capacites** (qui peut faire quoi)
- Les **regles de verification** (avant chaque action sensible)
- L'**articulation avec StrongFather** (certaines capacites requierent en plus une decision StrongFather)
- Les **invariants** (pas d'escalade, un role par compte)

Ce document **ne couvre pas** :
- L'authentification (login, MFA, session) — voir [Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md)
- Les Mandats de Permission des Operateurs metier (StrongFather pour Strate 7) — voir [StrongFather Integration Contract](../integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)
- La definition des niveaux de securite 0-4 — voir [Security Level Management Contract](./MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md)

---

## 3. Roles MiyukiniAdmin

### 3.1 Definition

Un **role** est un identifiant attribue a un compte admin. Il determine l'ensemble des **capacites** dont dispose ce compte. Un compte a **exactement un role** (pas de cumul, pas de role « super » implicite).

| Role | Description | Niveau de privilege |
|------|-------------|----------------------|
| **Admin** | Administrateur standard : dashboard, metriques, securite (lecture/ecriture niveau 0-4), liste Operateurs, DB via KindMother, tests, config non critique, gestion des comptes admin (selon capacites). | Eleve |
| **Recovery** | Administrateur recovery : tout ce qu'Admin peut + acces DB recovery (sous conditions cumulatives : T3/T4, MFA, StrongFather, etc.). | Maximum |
| **Audit** | Auditeur : lecture seule — logs, metriques, etat securite, liste Operateurs (lecture). Aucune modification. | Minimal |

### 3.2 Hierarchie

Il n'y a **pas de hierarchie** entre roles au sens « Admin herite de Audit ». Chaque role a une **liste explicite** de capacites. Recovery inclut toutes les capacites Admin **plus** les capacites recovery ; cela est modelise par la matrice (voir section 5), pas par un heritage de role.

### 3.3 Attribution

- **Premier compte** : Cree lors du parcours d'installation (Futur Admin) ; le role est **Admin** par defaut (ou configurable au moment de la creation).
- **Comptes suivants** : Crees par un admin ayant la capacite `admin.accounts.create` ; le role est choisi parmi Admin, Recovery, Audit (sous reserve que le createur puisse attribuer ce role — voir 5.2).

**Regle :** Seul un compte **Recovery** (ou Admin si politique le permet) peut creer un compte **Recovery**. Un compte **Audit** ne peut creer aucun autre compte.

---

## 4. Catalogue des capacites

### 4.1 Convention de nommage

Les capacites sont nommees selon le schema : `admin.<domaine>.<action>` ou `admin.<domaine>.<sujet>.<action>`.

- **admin** : prefixe MiyukiniAdmin (distinct des capacites Operateurs metier).
- **domaine** : dashboard, metrics, security, db, operators, tests, accounts, etc.
- **action** : read, write, create, delete, activate, revoke, etc.

### 4.2 Capacites par domaine

#### Dashboard et navigation

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.dashboard.read` | Acces au dashboard (etat systeme, liens). | Non |
| `admin.navigation.read` | Acces a la navigation (menu selon permissions). | Non |

#### Metriques et monitoring

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.metrics.read` | Lecture metriques systeme (CPU, RAM, disque, reseau). | Non |
| `admin.metrics.db.read` | Lecture metriques DB (requetes, latence, pool). | Non |
| `admin.logs.read` | Consultation logs d'audit et operationnels. | Non |

#### Securite (niveaux et degradation)

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.security.level.read` | Lecture niveau securite actuel (0-4). | Non |
| `admin.security.level.write` | Changement niveau securite. | **Oui** |
| `admin.security.degradation.activate` | Activation mode degradation. | **Oui** |
| `admin.security.trust.read` | Lecture etat WorrySentinel (T0-T4). | Non |

#### Base de donnees (KindMother)

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.db.read` | Lecture tables, lignes (CRUD read, liste, export). | Non |
| `admin.db.write` | Ecriture via KindMother (create, update, delete) — hors recovery. | **Oui** (selon politique) |
| `admin.db.migrate` | Execution migrations (schema, donnees). | **Oui** |
| `admin.db.repair` | Reparation DB (operations maintenances). | **Oui** |
| `admin.db.recovery` | Acces DB direct (mode recovery, conditions cumulatives). | **Oui** (CRITIQUE) |
| `admin.db.backup.read` | Liste backups, telechargement. | Non |
| `admin.db.backup.trigger` | Declencher backup. | **Oui** (selon politique) |
| `admin.db.restore` | Restauration depuis backup. | **Oui** |

#### Operateurs

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.operators.list` | Liste des Operateurs, etats. | Non |
| `admin.operators.isolate` | Isolation / restriction d'un Operateur. | **Oui** |

#### Tests

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.tests.coherence` | Lancement tests coherence DB, conformite. | Non |
| `admin.tests.load` | Lancement tests de charge. | **Oui** (MOYENNE) |
| `admin.tests.flux.read` | Consultation resultats tests flux cores. | Non |

#### Comptes admin (MiyukiniAdmin)

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.accounts.read` | Liste des comptes admin (identifiants, roles, etat). | Non |
| `admin.accounts.create` | Creation d'un nouveau compte admin. | Non (ou oui selon politique) |
| `admin.accounts.write` | Modification compte (role, MFA, etc.) — hors mot de passe. | Non |
| `admin.accounts.reset_password` | Forcer reset mot de passe pour un compte. | Non |
| `admin.accounts.lock` | Verrouiller un compte (manuellement). | Non |
| `admin.accounts.unlock` | Deverrouiller un compte (apres rate limit). | Non |
| `admin.accounts.revoke` | Revocation d'un compte (plus de login). | Non |
| `admin.accounts.revoke_session` | Revocation d'une session cible. | Non |

#### Configuration et systeme

| Capacite | Description | Validation StrongFather |
|----------|-------------|--------------------------|
| `admin.config.read` | Lecture configuration systeme (non sensible). | Non |
| `admin.config.write` | Modification configuration (parametres non critiques). | **Oui** (selon politique) |

### 4.3 Capacites et StrongFather

Pour les capacites marquees **Validation StrongFather : Oui**, l'execution de l'**action** (pas l'acces a l'UI) requiert en plus une **decision StrongFather** (demande via BondingBrother). Voir [StrongFather Integration Contract](../integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md).

**Regle :** La **permission** (capacite accordee au role) autorise a **demander** l'action ; la **decision StrongFather** autorise a **executer** l'action. Les deux sont necessaires pour les actions critiques.

---

## 5. Matrice role → capacites

### 5.1 Admin

Le role **Admin** possede les capacites suivantes (aucune capacite recovery ni creation de compte Recovery).

| Capacite | Admin |
|----------|-------|
| `admin.dashboard.read` | Oui |
| `admin.navigation.read` | Oui |
| `admin.metrics.read` | Oui |
| `admin.metrics.db.read` | Oui |
| `admin.logs.read` | Oui |
| `admin.security.level.read` | Oui |
| `admin.security.level.write` | Oui |
| `admin.security.degradation.activate` | Oui |
| `admin.security.trust.read` | Oui |
| `admin.db.read` | Oui |
| `admin.db.write` | Oui |
| `admin.db.migrate` | Oui |
| `admin.db.repair` | Oui |
| `admin.db.backup.read` | Oui |
| `admin.db.backup.trigger` | Oui |
| `admin.db.restore` | Oui (sans mode recovery direct) |
| `admin.operators.list` | Oui |
| `admin.operators.isolate` | Oui |
| `admin.tests.coherence` | Oui |
| `admin.tests.load` | Oui |
| `admin.tests.flux.read` | Oui |
| `admin.accounts.read` | Oui |
| `admin.accounts.create` | Oui (roles Admin, Audit uniquement) |
| `admin.accounts.write` | Oui |
| `admin.accounts.reset_password` | Oui |
| `admin.accounts.lock` | Oui |
| `admin.accounts.unlock` | Oui |
| `admin.accounts.revoke` | Oui (sauf compte Recovery) |
| `admin.accounts.revoke_session` | Oui |
| `admin.config.read` | Oui |
| `admin.config.write` | Oui |
| **admin.db.recovery** | **Non** |

### 5.2 Recovery

Le role **Recovery** possede **toutes** les capacites Admin **plus** :

| Capacite | Recovery |
|----------|----------|
| Toutes les capacites Admin | Oui |
| `admin.db.recovery` | Oui |
| `admin.accounts.create` (y compris role Recovery) | Oui |
| `admin.accounts.revoke` (y compris compte Recovery) | Oui |

**Regle :** Seul Recovery peut acceder au mode DB recovery et creer/révoquer des comptes Recovery.

### 5.3 Audit

Le role **Audit** possede **uniquement** les capacites en lecture :

| Capacite | Audit |
|----------|-------|
| `admin.dashboard.read` | Oui |
| `admin.navigation.read` | Oui (menu restreint) |
| `admin.metrics.read` | Oui |
| `admin.metrics.db.read` | Oui |
| `admin.logs.read` | Oui |
| `admin.security.level.read` | Oui |
| `admin.security.trust.read` | Oui |
| `admin.operators.list` | Oui (lecture seule) |
| `admin.tests.flux.read` | Oui (lecture resultats) |
| `admin.accounts.read` | Oui (liste, pas de modification) |
| `admin.config.read` | Oui |
| **Toute autre capacite** | **Non** |

**Regle :** Audit ne peut modifier aucune donnee ni declencher d'action critique.

### 5.4 Tableau récapitulatif (extrait)

| Capacite | Admin | Recovery | Audit |
|----------|-------|----------|-------|
| `admin.dashboard.read` | Oui | Oui | Oui |
| `admin.security.level.write` | Oui | Oui | Non |
| `admin.db.read` | Oui | Oui | Non (pas d'acces CRUD DB) |
| `admin.db.recovery` | Non | Oui | Non |
| `admin.accounts.create` | Oui (Admin, Audit) | Oui (tous) | Non |
| `admin.accounts.revoke` | Oui (sauf Recovery) | Oui (tous) | Non |

*(La liste complete est deduite des sections 5.1 à 5.3.)*

---

## 6. Verification des permissions

### 6.1 Moment de la verification

Avant **chaque action sensible** (requete HTTP modifiant des donnees, appel a une capacite critique, affichage d'une section restreinte), MiyukiniAdmin doit :

1. **Identifier** le compte authentifie (session → account_id → role).
2. **Resoudre** la capacite requise pour cette action (ex. afficher la page « Changement niveau securite » → `admin.security.level.write`).
3. **Verifier** que le role du compte possede cette capacite (matrice role → capacites).
4. Si **oui** : poursuivre (et, si capacite à validation StrongFather, enchaîner avec la demande StrongFather avant execution).
5. Si **non** : refuser l'action (HTTP 403 ou page « Acces refuse »), audit « permission denied ».

### 6.2 UI et navigation

- Les **liens et boutons** des sections restreintes ne sont affiches que si le compte possede la capacite correspondante (ex. pas de lien « Recovery » pour un role Admin).
- La **navigation** (menu) est filtree selon les capacites (voir `admin.navigation.read` et liste des capacites par page).
- Une tentative d'acces direct (URL) a une section non autorisee doit etre rejetee cote serveur (verification permission), pas seulement masquee en UI.

### 6.3 API internes

MiyukiniAdmin n'expose **pas d'API publique** (INV-MA-3). Les appels internes (backend → BondingBrother, etc.) sont effectues dans le contexte d'une session authentifiee. Chaque endpoint interne qui declenche une action sensible doit **verifier la capacite** associee avant execution.

### 6.4 Pas d'escalade

- Un compte ne peut **pas** s'auto-attribuer un role superieur (ex. Admin → Recovery). Seul un compte Recovery (ou Admin si politique autorise) peut modifier le role d'un autre compte (via `admin.accounts.write`), et seul Recovery peut attribuer le role Recovery.
- Un compte ne peut **pas** acquerir une capacite non incluse dans son role (pas de « permission dynamique » ni de contournement).

**Regle :** Les permissions sont **fixees** par le role ; la seule façon d'obtenir plus de droits est qu'un autre admin (avec les droits adequats) modifie le role du compte.

---

## 7. Articulation avec StrongFather

### 7.1 Double barriere

Pour les actions **critiques** (changement niveau securite, DB recovery, migration, etc.) :

1. **Permission** : Le compte doit avoir la **capacite** correspondante (ex. `admin.security.level.write`). Sinon → 403.
2. **Decision StrongFather** : MiyukiniAdmin envoie une **demande de decision** à StrongFather via BondingBrother (avec justification, contexte). StrongFather repond APPROVED / DENIED. Si DENIED → action non executee, reponse utilisateur « Action refusee par la gouvernance ».

Les deux barrieres sont **obligatoires** pour les capacites marquees « Validation StrongFather : Oui ».

### 7.2 Capacites sans validation StrongFather

Pour les capacites en **lecture seule** ou **non critiques** (ex. `admin.metrics.read`, `admin.logs.read`, `admin.accounts.read`), la seule verification est la **permission** (role → capacite). Pas d'appel StrongFather.

---

## 8. Invariants et garanties

| Code | Invariant |
|------|-----------|
| **INV-PERM-1** | Un compte admin a exactement un role (Admin, Recovery ou Audit). |
| **INV-PERM-2** | Chaque action sensible est precedee d'une verification de capacite ; absence de capacite → refus. |
| **INV-PERM-3** | Les capacites sont explicites (matrice) ; pas de capacite implicite ni heritee par defaut. |
| **INV-PERM-4** | Seul le role Recovery possede la capacite `admin.db.recovery` et peut creer/révoquer des comptes Recovery. |
| **INV-PERM-5** | Le role Audit ne possede que des capacites en lecture ; aucune modification. |
| **INV-PERM-6** | Pour les capacites à validation StrongFather, l'execution requiert en plus une decision StrongFather favorable. |

---

## 9. Résumé

- **Roles** : Admin, Recovery, Audit — un seul role par compte.
- **Capacites** : Catalogue explicite `admin.<domaine>.<action>` ; certaines requierent une decision StrongFather en plus de la permission.
- **Matrice** : Admin = capacites standard (sans recovery) ; Recovery = Admin + recovery + gestion comptes Recovery ; Audit = lecture seule.
- **Verification** : Avant chaque action sensible, resolution de la capacite requise et verification role → capacite ; refus si absent.
- **Pas d'escalade** : Permissions fixees par le role ; modification du role uniquement par un admin autorise.

---

## 10. Documents associes

- [MiyukiniAdmin - Auth and First-Boot Contract](./MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [MiyukiniAdmin - StrongFather Integration Contract](../integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)
- [MiyukiniAdmin - Security Level Management Contract](./MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Contrat normatif — Permissions (RBAC) MiyukiniAdmin
