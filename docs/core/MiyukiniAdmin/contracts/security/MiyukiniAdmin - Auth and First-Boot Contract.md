# MiyukiniAdmin — Auth and First-Boot Contract

## 1. Contexte

Ce document definit le **contrat d'authentification, d'autorisation et de premier demarrage** de MiyukiniAdmin et de l'environnement COG. MiyukiniAdmin etant un service critique au-dessus de tous les autres, son auth et le bootstrap de l'environnement doivent etre robustes, isoles et gouvernes.

**Principe fondamental :**

> **L'environnement est soit vierge, soit deja initialise. Au premier demarrage, seul MiyukiniAdmin et les Cores peuvent agir ; StrongFather verrouille toute autre action.**

**References :**
- [MiyukiniAdmin - Documentation Fondatrice](../../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Threat Model Contract](./MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- La detection d'etat de l'environnement (vierge vs initialise vs **compromis** : attaque, troncature, alteration)
- La **distinction** entre environnement vierge et environnement attaque/tronque/altere (reponse securitaire)
- Le verrou StrongFather au demarrage (actions autorisees : MiyukiniAdmin + Cores uniquement)
- Le statut **Futur Admin** et le processus d'installation (uniquement si environnement vierge)
- La **reponse securitaire** en cas d'environnement compromis (pas de parcours installation)
- Le systeme d'auth propre a MiyukiniAdmin (compte admin, permissions, intrusion)
- L'articulation avec le protocole d'identite d'environnement (EIP)

Ce document **ne couvre pas** :
- L'implementation cryptographique detaillee de l'EIP (voir [Environment Identity Protocol (EIP)](../../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md))
- Les Mandats de Permission des Operateurs metier (hors MiyukiniAdmin)
- La visite inter-COG (Utilisateur Visiteur, Visa)

---

## 3. Etat de l'environnement au demarrage

### 3.1 Donnees critiques pour l'etat

MiyukiniAdmin determine l'etat de l'environnement en verifiant l'**existence** et l'**integrite** des donnees critiques suivantes :

| Donnee critique | Role | Emplacement | Present + integre = environnement initialise |
|-----------------|------|-------------|----------------------------------------------|
| **Blob identite environnement** | Identite souveraine du COG (EIP) | KindMother / stockage protege | Oui |
| **Registre premier admin** | Existence d'au moins un compte admin MiyukiniAdmin | Stockage MiyukiniAdmin | Oui |
| **Schema bootstrap** | Tables noyau (kernel_config, core_registry, etc.) | KindMother / DB | Oui |
| **Politique StrongFather bootstrap** | Verrou « seul MiyukiniAdmin + Cores » actif ou desactive | StrongFather | Desactive = environnement opere normalement |

**Regle :** L'etat depend de la **presence** et de l'**integrite** de ces donnees. Absence totale → **vierge**. Presence avec integrite invalide → **compromis** (attaque, troncature, alteration), pas vierge.

---

### 3.2 Etats de l'environnement (vierge, initialise, compromis)

| Etat | Definition | Exemple |
|------|------------|---------|
| **VIERGE** | Environnement **jamais initialise** : les donnees critiques n'ont **jamais ete creees** ou le stockage est vide (premier deploiement). Aucune trace d'un boot precedent reussi. | Pas de blob EIP ; pas de compte admin ; pas de schema bootstrap ou schema vide. |
| **INITIALISE** | Environnement **deja initialise** et **intact** : toutes les donnees critiques sont **presentes** et **integres** (hash EIP valide, registre admin coherent, schema complet). | Blob EIP present, tag/hash valides ; au moins un compte admin ; schema et tables noyau complets et coherents. |
| **COMPROMIS** | Environnement **attaque, tronque ou altere** : des donnees critiques sont **presentes mais invalides** (corruption, troncature, incohérence) ou **incoherentes** entre elles. Indique une intrusion, une panne grave ou une alteration malveillante. | Blob EIP present mais tag/hash invalides ; registre admin present mais corrompu ou incohérent ; schema present mais tronque ; incohérence entre EIP et etat reel (ex. EIP dit « initialise » mais aucun compte admin). |

**Regle fondamentale :** Un environnement **compromis** n'est **pas** un environnement vierge. Il ne doit **jamais** declencher le parcours d'installation (Futur Admin). Il releve d'une **reponse securitaire** (voir 3.5).

---

### 3.3 Criteres de distinction (vierge vs compromis)

La distinction repose sur l'**absence totale** vs la **presence defectueuse** des artefacts.

| Criteres | VIERGE | COMPROMIS |
|----------|--------|-----------|
| **Blob EIP** | Absent (jamais cree). | Present mais **invalide** : tag AEAD invalide, hash d'integrite incoherent avec l'etat courant, ou format corrompu. |
| **Registre admin** | Absent ou vide (aucun compte). | Present mais **incoherent** : structure corrompue, hash de mot de passe invalide, ou incohérence (ex. flag « environnement initialise » present alors qu'aucun compte admin valide). |
| **Schema bootstrap** | Absent ou tables vides (jamais migre). | Present mais **tronque** : tables manquantes, colonnes manquantes, contraintes rompues, ou checksum schema invalide. |
| **Cohérence globale** | N/A (rien a croiser). | **Incoherence** : EIP indique une iteration / version alors que le registre admin est vide ; ou schema present sans blob EIP ; ou blob EIP present sans schema complet. |

**Indicateurs de compromission (exemples) :**

- EIP : dechiffrement reussi mais `integrity_hash` ne correspond pas a l'etat actuel des Cores / Kernel (alteration post-creation).
- EIP : tag AEAD invalide (blob tronque ou modifie).
- Registre admin : presence d'entrees mais aucune ne permet un login valide (ex. tous revoques ou corrompus) alors que l'environnement est marque initialise.
- Schema : checksum des tables noyau different de la valeur attendue (migration partielle ou troncature).
- Fichier ou slot de stockage : taille nulle ou anormale alors qu'un blob EIP est attendu (troncature).

**Regle :** Dès qu'une **presence** est detectee avec **integrite invalide** ou **incoherence** entre artefacts, l'environnement est classe **COMPROMIS**. L'absence totale et coherente (rien n'a jamais ete cree) reste **VIERGE**.

---

### 3.4 Algorithme de detection (au demarrage)

```
1. Demarrer Kernel (Id, Logger, Clock, Config, Lifecycle).
2. MiyukiniAdmin demarre en mode « detection ».
3. Verifier presence des artefacts :
   - Blob EIP (existe ?)
   - Registre admin (existe ? au moins un compte ?)
   - Schema bootstrap (existe ? tables noyau presentes ?)
4. Si aucun artefact present (stockage vide, jamais initialise) → VIERGE.
   → Activer verrou StrongFather + flux Futur Admin (parcours installation).
5. Si artefacts presents : verifier integrite et coherence :
   - EIP : dechiffrement + verification tag ; verification integrity_hash vs etat courant.
   - Registre admin : structure valide ; au moins un compte non revoque et coherent.
   - Schema : checksum / structure complete ; coherence avec EIP (iteration, version).
6. Si tout present et valide → INITIALISE.
   → Pas de verrou bootstrap ; flux auth classique (login admin).
7. Si presence mais integrite invalide ou incoherence → COMPROMIS.
   → Reponse securitaire (voir 3.5) : pas de parcours installation, pas de login normal.
```

---

### 3.5 Reponse securitaire (environnement attaque, tronque, altere)

Lorsque l'environnement est classe **COMPROMIS** (attaque, troncature, alteration), MiyukiniAdmin applique une **reponse securitaire** — et **non** le parcours d'installation (Futur Admin).

#### 3.5.1 Principes

- **Ne jamais traiter un environnement compromis comme vierge.** Pas d'acces au parcours d'installation (Futur Admin) sans controle supplementaire.
- **Ne pas autoriser le login classique** tant que l'integrite n'est pas retablie ou qu'une procedure de recovery gouvernée n'a pas ete menee.
- **Proteger le systeme** : limiter la surface d'attaque, alerter, tracer, exiger une intervention humaine et une decision de gouvernance pour toute reprise.

#### 3.5.2 Mesures immediates

| Mesure | Description |
|--------|-------------|
| **Mode degrade / lockdown** | WorrySentinel passe en etat **T3 (Restreint)** ou **T4 (Bloque)** selon politique. Gel des Operateurs metier ; seuls MiyukiniAdmin et les Cores peuvent agir (verrou type bootstrap ou equivalent). |
| **Blocage acces normal** | Page dediee « Environnement compromis » affichee a tout acces a l'UI. Pas de formulaire de login classique. Pas de parcours installation (Futur Admin) propose. |
| **Alerte et audit** | Evenement « ENVIRONMENT_COMPROMISED » enregistre (timestamp, indicateurs detectes : EIP invalide, registre incoherent, schema tronque, etc.). Notification interne selon politique (log, alerte equipe). |
| **Pas de reinitialisation automatique (cas standard)** | Lorsque l'humain peut intervenir, aucune suppression ou reinitialisation automatique des donnees. Toute reprise passe par une **procedure de recovery** gouvernée (voir 3.5.3). |
| **Recovery automatique (interface compromise)** | Lorsque l'**interface humaine est compromise** (auth, donnees admin, MiyukiniAdmin) et que l'**humain ne peut pas intervenir**, une **recovery/rollback automatique** est lancee (voir 3.5.5). |

#### 3.5.3 Procedure de recovery (gouvernée)

La reprise après compromission releve d'une **procedure de recovery** explicite, **pas** du parcours d'installation :

| Etape | Description |
|-------|-------------|
| **1. Diagnostic** | MiyukiniAdmin (mode recovery ou protocole dedie) permet une lecture limitee des indicateurs (quel artefact est invalide, sans exposer de donnees sensibles). Option : export de rapports d'audit pour analyse hors ligne. |
| **2. Decision humaine** | Un responsable (humain) decide : reparation in-place (si possible), restauration depuis backup sain, ou reinitialisation complete (destruction des donnees et nouveau premier boot). La decision est tracee et justifiee. |
| **3. Authentification forte** | Toute action de recovery (reparation, restauration, reinitialisation) exige une authentification forte (MFA, voire protocole recovery dedie) et une **decision StrongFather** (justification, contexte). |
| **4. Execution gouvernée** | Les actions de recovery sont executees sous controle MiyukiniAdmin + Cores ; StrongFather valide chaque etape critique. Toute action est auditee. |
| **5. Retour a l'etat initialise** | Une fois l'integrite retablie (reparation ou nouveau premier boot après reinitialisation), l'environnement est re-marque INITIALISE. Le verrou / mode degrade est leve. Le login classique redevient possible. |

**Regle :** La reinitialisation complete (destruction des donnees + nouveau premier boot) n'est **pas** proposee automatiquement lorsque l'humain peut intervenir. Elle n'est disponible que dans le cadre de la procedure de recovery, après decision humaine et validation StrongFather. Lorsque l'interface humaine est compromise et l'humain ne peut pas intervenir, la recovery automatique puis, en cas d'echec, la destruction et la reinitialisation en environnement vierge avec memoire de corruption s'appliquent (voir 3.5.5).

#### 3.5.4 Recovery automatique (interface humaine compromise, humain ne peut pas intervenir)

Lorsque la **compromission concerne l'interface humaine** (auth, donnees admin, MiyukiniAdmin) et que l'**humain ne peut pas intervenir** (ex. plus d'acces au login, plus de compte admin valide, MiyukiniAdmin inutilisable), le systeme applique une **recovery/rollback automatique**, puis, en cas d'echec, une **destruction des donnees DB** et une **reinitialisation en environnement vierge avec memoire de sa corruption passee**.

##### 3.5.4.1 Declenchement

| Condition | Description |
|-----------|-------------|
| **Interface compromise** | Auth compromise (ex. registre admin corrompu, plus de login possible), donnees admin compromisees, ou MiyukiniAdmin lui-meme compromis (ex. code ou config alteres). |
| **Humain ne peut pas intervenir** | Aucun acces au parcours de recovery gouvernée (pas de login, pas de compte admin valide, pas d'acces physique ou protocole recovery utilisable). Le delai ou les indicateurs (ex. detection repetee de compromission) declenchent le passage en mode recovery automatique. |

**Regle :** Le passage en recovery automatique est declenche selon la politique (ex. apres un delai sans acces humain valide, ou des que l'impossibilite d'intervention humaine est constatee).

##### 3.5.4.2 Recovery/rollback automatique

1. **Lancement** : Une **recovery/rollback automatique** est lancee (ex. restauration depuis un point de coherence connu — backup, snapshot — ou tentative de reparation des artefacts critiques).
2. **Criteres de succes** : Integrite EIP retablie, au moins un compte admin valide ou parcours Futur Admin de nouveau accessible, schema bootstrap coherent.
3. **Si succes** : L'environnement repasse en etat **INITIALISE** (ou **VIERGE** si rollback revient a un etat pre-initialisation). Le verrou / mode degrade est leve. L'humain peut a nouveau intervenir (login ou parcours installation).
4. **Si echec** : La recovery automatique a echoue (ex. backup absent ou corrompu, reparation impossible). Le systeme applique alors la **destruction des donnees DB** et la **reinitialisation en environnement vierge avec memoire de corruption** (voir 3.5.4.3).

##### 3.5.4.3 Echec de la recovery automatique : sauvegarde pre-destruction (si besoin), destruction des donnees DB et reinitialisation

Lorsque la recovery/rollback automatique **echoue** :

| Etape | Description |
|-------|-------------|
| **0. Sauvegarde pre-destruction (si aucune sauvegarde locale antérieure)** | **Si il n'existe pas de sauvegarde locale antérieure** des donnees de la DB : une **sauvegarde** des donnees DB est effectuee **avant** la destruction, puis **compressee** (ex. archive au format defini par politique — zip, tar.gz, etc.) et stockee dans un emplacement dedie (ex. repertoire local ou slot protege hors DB). Cette sauvegarde « dernier recours » permet un examen forensique ou une tentative de recuperation ultérieure par un humain. **Si une sauvegarde locale antérieure existe deja** : la destruction peut etre effectuee sans nouvelle sauvegarde (les donnees sont neanmoins jugees perdues pour l'environnement courant). |
| **1. Donnees DB jugees perdues** | Les donnees de la base (KindMother, donnees metier, registre admin, etc.) sont **totalement detruites** et **jugees perdues** pour l'environnement. Apres destruction, l'environnement considere que ces donnees ne sont plus recuperables depuis la DB. |
| **2. Destruction** | Suppression ou purge complete des donnees DB (et des artefacts critiques associes) selon un protocole defini (ex. wipe des tables, suppression des fichiers de stockage). |
| **3. Reinitialisation** | L'environnement **se reinitialise** pour redevenir **vierge** : plus de blob EIP (ou EIP invalide efface), plus de registre admin, schema bootstrap vide ou recree vide. L'etat cible est **VIERGE**. |
| **4. Memoire de la corruption passee** | L'environnement reste **vierge** (parcours Futur Admin, premier boot) mais conserve une **memoire de sa corruption passee** : une trace persistante (ex. flag, audit immuable, ou champ dedie hors DB detruite) indique que cet environnement a deja ete compromis et reinitialise apres echec de recovery automatique. Cette memoire n'expose pas de donnees sensibles ; elle sert a l'audit, au diagnostic et eventuellement a des politiques renforcees (ex. alerte, niveau de vigilance). |

**Regle :** En cas de reponse securitaire (corruption MiyukiniAdmin / interface compromise) conduisant a la destruction des donnees DB, une **sauvegarde compressee** est effectuee **avant** destruction **si et seulement si** il n'existe pas de sauvegarde locale antérieure des donnees de la DB. La memoire de corruption passee **survit** a la destruction des donnees DB (stockage dedie, ex. fichier ou slot protege non efface lors du wipe, ou re-ecrit avant le passage en vierge). Elle ne contient que des metadonnees d'audit (ex. timestamp de la reinitialisation, raison « recovery automatique echouee », pas de donnees utilisateur ni de secrets).

##### 3.5.4.4 Etat resultant : vierge avec memoire de corruption

| Propriete | Description |
|-----------|-------------|
| **Etat** | **VIERGE** : parcours d'installation (Futur Admin) s'applique ; pas de login existant ; EIP et compte admin a recreer. |
| **Mémoire** | Une **memoire de corruption passee** est presente : l'environnement « sait » qu'il a ete compromis et reinitialise apres echec de recovery automatique. Cette memoire peut etre consultee (ex. par un futur admin après installation) pour audit ou vigilance. |
| **Donnees DB** | **Perdues** : toutes les donnees DB ont ete detruites et sont jugees perdues. Aucune restauration possible depuis cet environnement. |

**Regle :** Un environnement **vierge avec memoire de corruption** est traite comme **vierge** pour le flux (parcours installation, Futur Admin). La memoire de corruption n'empêche pas le parcours ; elle l'accompagne (ex. message informatif ou alerte pour le Futur Admin).

#### 3.5.5 Invariants (reponse securitaire et recovery automatique)

| Code | Invariant |
|------|-----------|
| **INV-AUTH-6** | Un environnement detecte comme **compromis** (attaque, troncature, alteration) declenche une **reponse securitaire** (mode degrade, blocage login normal, alerte, audit). Il ne declenche **jamais** le parcours d'installation (Futur Admin) sans procedure de recovery gouvernée, sauf après recovery automatique ayant conduit a un etat vierge avec memoire de corruption (voir INV-AUTH-7). |
| **INV-AUTH-7** | Lorsque l'interface humaine est compromise (auth, donnees admin, MiyukiniAdmin) et que l'humain ne peut pas intervenir, une **recovery/rollback automatique** est lancee. Si elle echoue : **si aucune sauvegarde locale antérieure** des donnees DB n'existe, une **sauvegarde compressee** est effectuee avant destruction ; puis les **donnees DB sont totalement detruites** et **jugees perdues** ; l'environnement **se reinitialise en vierge** mais conserve une **memoire de sa corruption passee** (audit, pas de donnees sensibles). |

### 4.1 Principe

Lorsque l'environnement est **vierge**, StrongFather applique un **verrou bootstrap** :

- **Toute action** emise par un acteur autre que **MiyukiniAdmin** ou les **Cores** est **refusee**.
- Les Cores peuvent repondre aux requetes de MiyukiniAdmin (generation EIP, persistance, etc.) mais aucun Operateur metier (Strate 7), aucun Outil/Kit (Strate 6) consomme par un tiers ne peut executer d'action metier.
- BondingBrother n'accepte que les requetes dont la source est **MiyukiniAdmin** ou un Core (pour les reponses internes).

### 4.2 Actions autorisees sous verrou

| Acteur | Autorisé sous verrou |
|--------|------------------------|
| **MiyukiniAdmin** | Oui — processus d'installation, creation compte admin, appels aux Cores |
| **StrongFather** | Oui — decisions, verrou, politique bootstrap |
| **KindMother** | Oui — persistance EIP, schema, registre admin |
| **CaringNanny** | Oui — observation etat |
| **WorrySentinel** | Oui — niveau securite bootstrap |
| **Kernel** | Oui — Id, Config, Lifecycle, etc. |
| **Autres Cores** | Oui — selon besoins bootstrap |
| **Operateurs (Strate 7)** | Non |
| **Outils / Kits (Strate 6) hors MiyukiniAdmin** | Non (MiyukiniAdmin ne consomme pas d'Outils ; ses capacites sont internes) |

### 4.3 Levée du verrou

Le verrou bootstrap est leve **uniquement** lorsque :

1. Le processus d'installation a ete mene a son terme (EIP genere, compte admin cree, configuration minimale validee).
2. MiyukiniAdmin enregistre explicitement la fin du premier boot (flag « environnement initialise »).
3. StrongFather desactive la politique « bootstrap lock » et applique les politiques normales (Mandats, Operateurs, etc.).

---

## 5. Futur Admin et processus d'installation

### 5.1 Futur Admin (Future Admin)

En environnement **vierge**, l'utilisateur qui accede a MiyukiniAdmin (premier contact) n'a pas encore de compte. Il est traite comme **Futur Admin** :

- **Definition :** Utilisateur considere comme le futur administrateur du COG, le temps du processus d'installation.
- **Droits :** Uniquement le parcours d'installation (voir 5.2). Aucun droit sur les donnees metier ni sur les Operateurs tant que le verrou est actif.
- **Identification :** Aucune auth forte requise avant la creation du compte admin (contexte local, premier acces). En revanche, la **creation du compte admin** lie une identite forte (credentials + MFA) a ce premier admin.

### 5.2 Parcours d'installation (First-Boot)

Pendant le processus d'installation, l'utilisateur (Futur Admin) est dirige vers un flux dedie :

1. **Accueil installation**  
   - Message clair : environnement vierge, configuration requise.  
   - Pas d'acces au dashboard metier ni aux Operateurs.

2. **Generation identite environnement (EIP)**  
   - Les Cores produisent les donnees d'identite du COG de facon **chiffree** (protocole EIP).  
   - KindMother persiste le blob EIP.  
   - Pas de modification manuelle ; tout passe par les Cores.

3. **Configuration minimale**  
   - Parametres obligatoires (nom environnement, niveau securite initial, etc.) selon contrats Cores.  
   - Validation StrongFather pour les decisions de config.

4. **Creation du compte admin MiyukiniAdmin**  
   - Saisie identifiant, mot de passe fort, MFA (TOTP ou cle materielle).  
   - Stockage securise (hash mot de passe, secrets MFA).  
   - Ce compte devient le **premier admin** ; il pourra creer d'autres comptes admin ensuite (selon politique).

5. **Finalisation**  
   - MiyukiniAdmin enregistre que l'environnement est initialise.  
   - StrongFather leve le verrou bootstrap.  
   - Redirection vers le dashboard ; l'utilisateur est desormais authentifie comme admin.

### 5.3 Robustesse

- Le parcours d'installation est **atomique** dans la mesure du possible : si une etape critique echoue, l'environnement reste vierge et le verrou reste actif.
- Aucun Operateur metier ni service externe ne peut etre invoque pendant l'installation.
- Toutes les actions (EIP, creation compte, config) sont tracees et auditees.

---

## 6. Systeme d'auth MiyukiniAdmin

### 6.1 Principe

MiyukiniAdmin dispose de son **propre systeme d'authentification et d'autorisation**, independant des Operateurs metier et des Mandats de Permission (StrongFather) qui gouvernent les Operateurs. Ce systeme a pour but d'**empecher les intrusions** et de garantir que seuls les comptes admin autorises accedent a la console.

**Documentation detaillee :**
- **Authentification** (login, MFA, session, mot de passe, rate limiting, stockage secrets, audit) : [MiyukiniAdmin - Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md).
- **Autorisation** (roles, capacites, matrice role → capacites) : [MiyukiniAdmin - Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md).

### 6.2 Composants

| Composant | Role |
|-----------|------|
| **Registre des comptes admin** | Liste des identites autorisees a utiliser MiyukiniAdmin (stockage securise, hors portee Operateurs). |
| **Auth locale** | Login / mot de passe + MFA (TOTP ou cle materielle). |
| **Session** | Session liee (IP, User-Agent, timeout court). Pas de cookie non securise. |
| **RBAC MiyukiniAdmin** | Roles internes (Admin, Recovery, Audit) pour les capacites admin (voir Permission Contract). |

### 6.3 Flux d'authentification (environnement initialise)

1. Utilisateur accede a l'UI MiyukiniAdmin.
2. Si pas de session valide → page de login.
3. Saisie identifiant + mot de passe → verification hash.
4. Si MFA requis → challenge TOTP ou cle.
5. Si succes → creation session (binding IP + User-Agent, expiration).
6. Acces au dashboard et aux capacites selon role (Admin, Recovery, Audit).

*Detail complet :* [Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md).

### 6.4 Permissions (RBAC MiyukiniAdmin)

Les **permissions** sont attachees aux **roles** MiyukiniAdmin (pas aux Mandats de Permission StrongFather des Operateurs). Roles et capacites :

| Role | Capacites typiques |
|------|---------------------|
| **Admin** | Dashboard, metriques, securite (niveau 0-4), liste Operateurs, DB via KindMother, tests, config, gestion comptes (Admin, Audit). |
| **Recovery** | Tout ce qu'Admin peut + acces DB recovery (sous conditions cumulatives : T3/T4, MFA, StrongFather) + creation/revocation comptes Recovery. |
| **Audit** | Lecture seule : logs, metriques, etat securite, liste Operateurs. Pas de modification. |

Regle : un compte admin a **un seul role** ; les capacites sont explicites et minimales (moindre privilege). *Catalogue complet et matrice :* [Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md).

### 6.5 Protection contre les intrusions

- **Rate limiting** : apres N echecs de login, blocage temporaire + alerte (voir Authentication Contract).
- **MFA obligatoire** pour les comptes Admin et Recovery.
- **Pas d'API publique** : MiyukiniAdmin n'expose aucune API vers l'exterieur (INV-MA-3).
- **Chiffrement** : TLS pour l'UI ; secrets et blobs EIP chiffres au repos (voir EIP).
- **Audit** : chaque tentative de login (succes/echec) et chaque action sensible sont loguees (voir Authentication Contract).

---

## 7. Articulation avec EIP (Environment Identity Protocol)

La **generation des donnees d'identite de l'environnement** pendant le premier boot est regie par le **Environment Identity Protocol (EIP)** :

- Les **Cores** produisent ces donnees (pas MiyukiniAdmin seul).
- Les donnees sont **chiffrees** et stockees via KindMother.
- Le contenu, le format et la cryptographie sont definis dans le document [MiyukiniAdmin - Environment Identity Protocol EIP](../../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md).

MiyukiniAdmin orchestre le flux (affichage du parcours installation, appels a BondingBrother) mais ne genere pas lui-meme les cles ni le blob EIP ; il delegue aux Cores selon le protocole EIP.

---

## 8. Invariants et garanties

| Code | Invariant |
|------|-----------|
| **INV-AUTH-1** | En environnement vierge, seuls MiyukiniAdmin et les Cores peuvent effectuer des actions gouvernées. |
| **INV-AUTH-2** | Le verrou StrongFather bootstrap est actif tant que l'environnement n'est pas marque initialise. |
| **INV-AUTH-3** | Le premier compte admin est cree uniquement pendant le processus d'installation, sous verrou. |
| **INV-AUTH-4** | MiyukiniAdmin utilise son propre systeme d'auth (registre admin, MFA, session) independant des Operateurs. |
| **INV-AUTH-5** | Les donnees d'identite d'environnement (EIP) sont produites par les Cores et stockees chiffrees. |
| **INV-AUTH-6** | Un environnement compromis declenche une reponse securitaire (mode degrade, blocage login normal, alerte, audit). Il ne declenche jamais le parcours d'installation (Futur Admin) sans procedure de recovery gouvernée, sauf après recovery automatique ayant conduit a un etat vierge avec memoire de corruption (INV-AUTH-7). |
| **INV-AUTH-7** | Lorsque l'interface humaine est compromise (auth, donnees admin, MiyukiniAdmin) et l'humain ne peut pas intervenir : recovery/rollback automatique lancee ; si echec, donnees DB totalement detruites (jugees perdues), environnement reinitialise en vierge avec memoire de sa corruption passee. |

---

## 9. Résumé des flux

### 9.1 Premier demarrage (environnement vierge)

```
Demarrage → Detection (aucun artefact present) → Environnement VIERGE
→ Verrou StrongFather (MiyukiniAdmin + Cores uniquement)
→ Utilisateur = Futur Admin → Parcours installation
→ EIP genere (Cores) → Config minimale → Creation compte admin
→ Environnement marque initialise → Levée verrou → Admin connecte → Dashboard
```

### 9.2 Demarrage suivant (environnement deja initialise)

```
Demarrage → Detection (artefacts presents et integres) → Environnement INITIALISE
→ Pas de verrou bootstrap → Flux auth classique
→ Utilisateur doit se connecter (login + MFA) → Session → Dashboard selon role
```

### 9.3 Demarrage avec environnement compromis (attaque, troncature, alteration)

```
Demarrage → Detection (artefacts presents mais invalides ou incoherents) → Environnement COMPROMIS
→ Reponse securitaire :
   - WorrySentinel T3/T4 (mode degrade / lockdown)
   - Page « Environnement compromis » (pas de login, pas de parcours installation)
   - Alerte + audit (ENVIRONMENT_COMPROMISED)
   - Si humain peut intervenir : procedure de recovery gouvernée (decision humaine, auth forte, StrongFather, audit)
   - Si interface humaine compromise et humain ne peut pas intervenir : recovery/rollback automatique (voir 9.4)
→ Apres recovery gouvernée : environnement re-marque INITIALISE → login classique de nouveau possible
```

### 9.4 Recovery automatique (interface compromise, humain ne peut pas intervenir)

```
Interface compromise (auth, donnees admin, MiyukiniAdmin) + humain ne peut pas intervenir
→ Lancement recovery/rollback automatique (restauration backup, reparation artefacts, etc.)
→ Si succes : environnement repasse INITIALISE ou VIERGE → humain peut a nouveau intervenir
→ Si echec : si aucune sauvegarde locale antérieure des donnees DB → sauvegarde compressee effectuee avant destruction
           → Donnees DB totalement detruites (jugees perdues)
           → Environnement se reinitialise en VIERGE
           → Memoire de la corruption passee conservee (audit, pas de donnees sensibles)
           → Parcours Futur Admin s'applique ; nouvel EIP et compte admin a recreer
```

---

## 10. Documents associes

- [MiyukiniAdmin - Documentation Fondatrice](../../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Installation & Bootstrap Guide](../../foundation/MiyukiniAdmin%20-%20Installation%20&%20Bootstrap%20Guide.md)
- [MiyukiniAdmin - Authentication Contract](./MiyukiniAdmin%20-%20Authentication%20Contract.md)
- [MiyukiniAdmin - Permission Contract](./MiyukiniAdmin%20-%20Permission%20Contract.md)
- [MiyukiniAdmin - Threat Model Contract](./MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)
- [MiyukiniAdmin - StrongFather Integration Contract](../integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md)
- [MiyukiniAdmin - Environment Identity Protocol EIP](../../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Contrat normatif — Auth et First-Boot
