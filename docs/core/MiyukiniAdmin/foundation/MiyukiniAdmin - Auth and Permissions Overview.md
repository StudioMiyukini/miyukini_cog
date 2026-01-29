# MiyukiniAdmin — Auth and Permissions Overview

## 1. Contexte

Ce document presente une **vue d'ensemble** du systeme d'authentification et d'autorisation integre a MiyukiniAdmin. Il sert de point d'entree pour les implementeurs et les architectes ; les specifications detaillees sont dans les contrats references.

**Principe :**

> **Aucun acces a la console sans authentification forte. Aucune action sensible sans permission explicite (role → capacite).**

---

## 2. Portee

| Domaine | Description | Contrat de reference |
|---------|-------------|------------------------|
| **Premier demarrage** | Environnement vierge, verrou StrongFather, Futur Admin, parcours installation, creation premier compte | [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) |
| **Detection etat environnement** | Distinction vierge / initialise / **compromis** (attaque, troncature, alteration) ; reponse securitaire si compromis | [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) (sections 3.2 à 3.5) |
| **Authentification** | Login, MFA, session, mot de passe, rate limiting, stockage secrets, audit auth | [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md) |
| **Autorisation** | Roles (Admin, Recovery, Audit), capacites, matrice role → capacites, verification avant action | [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md) |
| **Identite environnement** | Donnees d'identite COG chiffrees (EIP), produites par les Cores | [Environment Identity Protocol EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md) |

---

## 3. Flux résumés

### 3.1 Premier demarrage (environnement vierge)

1. MiyukiniAdmin demarre → detection : **presence** et **integrite** des artefacts (EIP, registre admin, schema bootstrap).
2. **Aucun artefact present** (jamais initialise) → **environnement vierge** → verrou StrongFather (MiyukiniAdmin + Cores uniquement).
3. Utilisateur = **Futur Admin** → parcours installation (EIP, config minimale, **creation premier compte admin**).
4. Compte cree (username, mot de passe fort, MFA) → environnement marque initialise → levée verrou → redirection dashboard.

*Detail :* [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md).

### 3.1bis Environnement compromis (attaque, troncature, alteration)

1. Detection : **artefacts presents mais invalides** (EIP tag/hash invalides, registre incoherent, schema tronque) ou **incoherence** entre artefacts.
2. Environnement classe **COMPROMIS** (pas vierge) → **reponse securitaire** : WorrySentinel T3/T4, page « Environnement compromis », pas de login, pas de parcours installation, alerte + audit.
3. **Si l'humain peut intervenir** : reprise uniquement via **procedure de recovery gouvernée** (decision humaine, auth forte, StrongFather, audit).
4. **Si l'interface humaine est compromise** (auth, donnees admin, MiyukiniAdmin) et **l'humain ne peut pas intervenir** : **recovery/rollback automatique** lancee. Si elle **echoue** : **si aucune sauvegarde locale antérieure** des donnees DB n'existe, une **sauvegarde compressee** est effectuee avant destruction ; puis donnees DB **totalement detruites** (jugees perdues), environnement **reinitialise en vierge** avec **memoire de sa corruption passee** (audit, pas de donnees sensibles). Parcours Futur Admin s'applique alors.

*Detail :* [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) sections 3.2 à 3.5 (notamment 3.5.4 et 3.5.5).

### 3.2 Connexion (environnement initialise)

1. Utilisateur ouvre l'UI → pas de session valide → page login.
2. Saisie identifiant + mot de passe → verification hash → si MFA actif, challenge TOTP ou cle.
3. Succes → creation session (binding IP/User-Agent, timeout) → acces dashboard selon **role**.
4. Echec → reponse generique « Identifiants invalides » ; rate limiting apres N echecs ; audit.

*Detail :* [Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md).

### 3.3 Action sensible (ex. changement niveau securite)

1. Utilisateur declenche l'action (ex. bouton « Changer niveau securite »).
2. **Verification permission** : le role du compte possede-t-il la capacite requise (ex. `admin.security.level.write`) ? Sinon → 403.
3. Si **capacite à validation StrongFather** : MiyukiniAdmin envoie une demande de decision à StrongFather (justification, contexte). Si DENIED → action non executee.
4. Si autorise : execution de l'action ; audit.

*Detail :* [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md), [StrongFather Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md).

---

## 4. Roles et capacites (résumé)

| Role | Périmètre |
|------|-----------|
| **Admin** | Dashboard, metriques, securite (lecture/ecriture niveau 0-4), DB via KindMother (read/write/migrate/repair/backup/restore), liste/isolation Operateurs, tests, gestion comptes (Admin, Audit). Pas d'acces DB recovery ni creation/revocation Recovery. |
| **Recovery** | Tout Admin + acces DB recovery (conditions T3/T4, MFA, StrongFather) + creation/revocation comptes Recovery. |
| **Audit** | Lecture seule : dashboard, metriques, logs, etat securite, liste Operateurs, resultats tests. Aucune modification. |

*Catalogue complet et matrice :* [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md).

---

## 5. Invariants cles

- **INV-AUTH-C-1** : Aucun acces console sans authentification reussie (identifiant + mot de passe + MFA si requis).
- **INV-AUTH-C-2** : Mots de passe et secrets MFA jamais stockes en clair.
- **INV-AUTH-6** : Environnement compromis → reponse securitaire ; parcours installation (Futur Admin) uniquement après procedure de recovery gouvernée ou après recovery automatique ayant conduit a un etat vierge avec memoire de corruption.
- **INV-AUTH-7** : Interface humaine compromise et humain ne peut pas intervenir → recovery/rollback automatique ; si echec → si aucune sauvegarde locale antérieure des donnees DB, sauvegarde compressee avant destruction ; puis donnees DB detruites (jugees perdues), environnement reinitialise en vierge avec memoire de corruption passee.
- **INV-PERM-1** : Un compte a exactement un role (Admin, Recovery ou Audit).
- **INV-PERM-2** : Chaque action sensible est precedee d'une verification de capacite ; absence → refus.
- **INV-PERM-4** : Seul Recovery possede `admin.db.recovery` et peut creer/revoker des comptes Recovery.

---

## 6. Documents associes

- [MiyukiniAdmin - Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Authentication Contract](../contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [MiyukiniAdmin - Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md)
- [MiyukiniAdmin - Threat Model Contract](../contracts/security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [MiyukiniAdmin - StrongFather Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference — Vue d'ensemble Auth & Permissions
