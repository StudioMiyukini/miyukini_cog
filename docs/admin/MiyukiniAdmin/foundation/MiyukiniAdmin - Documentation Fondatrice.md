# MiyukiniAdmin — Documentation Fondatrice

## 1. Contexte

Ce document definit l'identite, le role et les principes fondamentaux de **MiyukiniAdmin** : l'Operateur Souverain (Strate 9) de l'ecosysteme Miyukini Core System.

MiyukiniAdmin est une **exception volontaire** a la logique Operateur standard. Il constitue la **console root** de l'ecosysteme : un outil d'orchestration et de controle, pas un Operateur metier.

**Principe fondamental :**

> **"MiyukiniAdmin est une console root, pas un produit metier."**

Il observe, installe, arbitre, mais ne vit pas dans le flux normal.  
Il est out-of-band, comme un BIOS / hyperviseur / console root.

## 2. Portee / Scope

Ce document definit :
- L'identite et le role de MiyukiniAdmin
- Les invariants fondamentaux
- Les interdictions absolues
- Le perimetre fonctionnel exact
- Les relations avec les cores
- Les cas extremes (ecriture DB directe)

Ce document **ne couvre pas** :
- Les details d'implementation technique (voir Implementation Guidelines)
- Les specifications UI/UX detaillees (voir section UI)
- Les protocoles de communication detailles (voir Contracts)

---

## 3. Definition Canonique

### 3.1 Enonce Canonique

> **MiyukiniAdmin est un Operateur Souverain, autonome et non reutilisable.**

Il constitue :
- Une exception volontaire a la logique Operateur standard
- Un outil d'orchestration et de controle, pas un Operateur metier
- Une autorite quasi ultime, strictement encadree

### 3.2 Ce que MiyukiniAdmin EST

| Propriete | Description |
|-----------|-------------|
| **Console root** | Point d'administration central de l'ecosysteme |
| **Operateur Souverain** | Autorite administrative au-dessus de la pyramide ; unique Operateur Souverain (Strate 9) |
| **Auto-suffisant** | Backend + Frontend + Logique metier admin internes ; capacites internes propres (pas de consommation d'Outil ni Kit d'Outils) |
| **Auth dediee** | Systeme d'auth propre (compte admin, MFA, RBAC) independant des Operateurs metier ; empeche les intrusions |
| **Tracable** | Toute action est journalisee et auditable |
| **Explicite** | Jamais silencieux, jamais implicite |

### 3.3 Ce que MiyukiniAdmin N'EST PAS

| Propriete | Raison |
|-----------|--------|
| **Un Operateur metier** | Pas de logique business applicative |
| **Un Outil ou Kit d'Outils** | Il ne fournit pas de capacite recomposable |
| **Une API publique** | Aucune exposition externe |
| **Un composant embarquable** | Jamais embarque dans un autre Operateur |
| **B2B / B2C / B2B2C** | Hors modele de livraison standard |

---

## 4. Invariants Fondamentaux

### 4.1 Catalogue des Invariants

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-MA-1** | Independance inverse | Aucun Operateur ne peut dependre de MiyukiniAdmin |
| **INV-MA-2** | Non-consommation | MiyukiniAdmin ne consomme aucun Outil ou Kit d'Outils |
| **INV-MA-3** | Non-exposition | MiyukiniAdmin n'expose aucune API publique |
| **INV-MA-4** | Mediation obligatoire | Toujours via BondingBrother pour acceder aux cores |
| **INV-MA-5** | Tracabilite totale | Toute action est tracable, horodatee, justifiee, auditable |
| **INV-MA-6** | Recovery controle | Ecriture DB directe uniquement en mode recovery |
| **INV-MA-7** | UI isolee | UI propre, isolee, non reutilisable |
| **INV-MA-8** | Logique admin | Logique metier administrative uniquement |
| **INV-MA-9** | Autonomie complete | Backend et frontend internes complets |
| **INV-MA-10** | Explicite | Jamais silencieux, jamais implicite |

### 4.2 Regles Absolues (Non Negociables)

**Aucun autre Operateur ne peut dependre de MiyukiniAdmin**

**MiyukiniAdmin ne consomme aucun Outil ou Kit d'Outils**

**MiyukiniAdmin n'expose aucune API publique**

**MiyukiniAdmin n'est jamais embarque dans un Operateur client**

Il n'est ni B2B, ni B2C, ni B2B2C.  
Il est out-of-band, comme un BIOS / hyperviseur / console root.

---

## 5. Interdictions

| Code | Interdiction | Raison |
|------|--------------|--------|
| **INTERD-MA-1** | Import par un autre Operateur | Maintenir l'independance inverse |
| **INTERD-MA-2** | Consommation d'Outils | Maintenir l'auto-suffisance |
| **INTERD-MA-3** | Exposition API publique | Securite et souverainete |
| **INTERD-MA-4** | Embarquement client | Maintenir l'isolation |
| **INTERD-MA-5** | Logique metier applicative | Separation admin/metier |
| **INTERD-MA-6** | Partage composants UI | Maintenir l'isolation UI |
| **INTERD-MA-7** | Bypass BondingBrother | Maintenir la gouvernance |
| **INTERD-MA-8** | Actions implicites | Maintenir la tracabilite |

---

## 6. Perimetre Fonctionnel

### 6.1 Installation & Bootstrap

**Fonctions :**
- **Premier demarrage** : detection environnement vierge vs initialise ; verrou StrongFather (seuls MiyukiniAdmin et les Cores peuvent agir) ; parcours Futur Admin et creation du premier compte admin (voir [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)).
- **Identite environnement** : generation des donnees d'identite du COG de facon chiffree par les Cores (protocole [EIP](../../../protocols/MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)).
- Installation complete de l'environnement Miyukini
- Verification hardware / OS / permissions
- Initialisation du kernel
- Generation des identites systeme (EIP)
- Deploiement et enregistrement des cores
- Validation de conformite post-installation

**Caracteristiques :**
- Script d'installation obligatoire
- Peut fonctionner offline
- Environnement isole et propre a lui-meme ; robuste car MiyukiniAdmin est un service critique au-dessus de tous les autres

### 6.2 Monitoring & Metriques

**Fonctions :**
- Lecture passive de metriques systeme (CPU, RAM, disque, reseau)
- Acces aux traces kernel
- Statistiques de decision (StrongFather)
- Etats Operateurs (CaringNanny)
- Sante DB / SQL engine
- Charge, latence, files internes

**Caracteristiques :**
- Zero modification implicite
- Lecture seule par defaut

### 6.3 Tests Techniques

**Fonctions :**
- Tests de performance (requetes communes)
- Tests de latence decisionnelle
- Tests de montee en charge controlee
- Tests de coherence DB
- Tests de conformite contractuelle

**Caracteristiques :**
- Environnement de diagnostic, pas de prod cachee

### 6.4 Securite & Arbitrage

**Fonctions :**
- Lecture de l'etat WorrySentinel
- Changement manuel et explicite du niveau de securite (0-4)
- Activation de modes de degradation
- Isolation de modules
- Desactivation temporaire de capacites

**Caracteristiques :**
- Toute action est tracable, horodatee, justifiee, auditable

### 6.5 Acces aux Donnees (Cas Normal)

**Fonctions :**
- Acces aux donnees via KindMother
- Toujours sous autorite StrongFather et contraintes WorrySentinel
- Operations : Lecture, Validation, Migration, Reparation controlee

**Caracteristiques :**
- Jamais de logique metier applicative

### 6.6 Recovery Exceptionnel (Cas Extreme)

**MiyukiniAdmin peut ecrire directement en DB, MAIS sous conditions cumulatives strictes :**

| Condition | Description |
|-----------|-------------|
| **Etat systeme >= Critique** | Niveau de confiance T3 ou T4 |
| **Protocole securite renforce** | Active explicitement |
| **Intervention humaine authentifiee** | Validation manuelle obligatoire |
| **Fenetre temporelle limitee** | Duree maximale definie |
| **Journalisation complete** | Toute operation tracee |
| **Revalidation obligatoire** | Apres intervention |

**Caracteristiques :**
- Ecriture temporaire
- Mode maintenance
- Blocage des Operateurs pendant l'operation
- Retour obligatoire via KindMother apres

Ce mode est exceptionnel, pas un fallback normal.  
Comparable a un mode recovery.

### 6.7 Tests des modules et cycle de vie

**Fonctions :**
- **Tests des modules** : Tests des Kits d'outils, Operateurs, Equipes d'operateurs et Services via le manifeste de test embarqué dans chaque module. Seul MiyukiniAdmin peut executer et interpreter ces tests (voir [Module Testing and Lifecycle Contract](../contracts/testing/MiyukiniAdmin%20-%20Module%20Testing%20and%20Lifecycle%20Contract.md)).
- **Cellule Admin** : Chaque module expose une cellule destinee uniquement a MiyukiniAdmin (identification, manifeste de test, metadonnees d'integrite). Seul MiyukiniAdmin peut la lire et l'utiliser.
- **Identification des modules** : Liste des modules presents obtenue via Master Butler (via BondingBrother), avec reference vers la cellule Admin de chaque module.
- **Verification d'integrite** : Collaboration avec les cores, notamment **TAMR** (champ d'action integrite et interventions humaines), pour verifier l'integrite des modules.
- **Cycle de vie des modules** : Ajout, verrouillage/deverrouillage et suppression d'un module — exclusivement via MiyukiniAdmin, sous validation StrongFather et enregistrement Master Butler / Ever Buddy selon le cas.

**Caracteristiques :**
- Exclusivite MiyukiniAdmin pour la cellule Admin et l'execution des tests embarqués
- Toute action lifecycle passe par BondingBrother ; aucun bypass des cores
- Tracabilite complete des tests et des actions de cycle de vie

---

## 7. Logique Metier & UI

### 7.1 Logique Metier Interne

**MiyukiniAdmin embarque en interne :**
- Toute sa logique metier propre
- Toute son interface utilisateur (UI/UX)
- Sans dependre d'aucun autre Operateur

Il est auto-suffisant fonctionnellement et visuellement.

**La logique metier de MiyukiniAdmin est strictement limitee a :**
- Installation de l'ecosysteme
- Configuration systeme
- Monitoring
- Tests techniques
- Arbitrage de securite
- Operations de maintenance
- Diagnostics
- Recovery

**Interdit :**
- Regles metier applicatives
- Workflows utilisateurs finaux
- Logique Operateur metier (B2B / B2C)
- Toute logique reutilisable ailleurs

Sa logique metier est administrative, technique, souveraine.  
Jamais fonctionnelle au sens "Operateur metier".

### 7.2 UI Propre, Isolee, Non Reutilisable

**MiyukiniAdmin :**
- Possede son propre design system
- Sa propre navigation
- Ses propres ecrans
- Ses propres etats UI
- Ses propres regles d'interaction

**Interdit :**
- Aucun composant UI partage
- Aucun theme herite
- Aucun framework UI "Operateur"

Meme s'il ressemble a PHPMyAdmin :
- Ce n'est pas une UI Operateur
- Ce n'est pas un frontend client
- C'est une console d'administration

### 7.3 Consequence Architecturale

**MiyukiniAdmin devient :**
- Un Operateur complet techniquement, mais ferme fonctionnellement

**Il est :**
- Un binaire / app / bundle autonome

**Avec :**
- Son backend interne
- Son frontend interne
- Ses regles internes

**Mais branche exclusivement sur l'ecosysteme via BondingBrother**

```
[MiyukiniAdmin]
 ├── UI propre
 ├── Logique metier admin
 ├── Securite maximale
 └── BondingBrother
        ↓
     Miyukini Core
```

---

## 8. Relations avec les Cores

### 8.1 BondingBrother

**Role :** Point d'acces exclusif pour MiyukiniAdmin.

**Responsabilites :**
- Mediation entre MiyukiniAdmin et les cores
- Exposition de capacites reservees
- Tracabilite complete des actions
- Validation des requetes administratives

### 8.2 StrongFather

**Role :** Autorite sur les decisions administratives.

**Responsabilites :**
- Validation des actions administratives
- Decisions sur les interventions
- Controle des changements de securite

### 8.3 KindMother

**Role :** Autorite sur l'acces aux donnees.

**Responsabilites :**
- Acces controle aux donnees
- Validation des operations de maintenance
- Reconciliation apres interventions

### 8.4 CaringNanny

**Role :** Observation de l'etat systeme.

**Responsabilites :**
- Exposition des metriques systeme
- Etats des Operateurs
- Sante globale du systeme

### 8.5 WorrySentinel

**Role :** Controle de securite.

**Responsabilites :**
- Lecture de l'etat de securite
- Changement manuel des niveaux (0-4)
- Activation des modes de degradation

### 8.6 Master Butler

**Role :** Decouverte des modules (Kits d'outils, Operateurs, Equipes d'operateurs, Services).

**Responsabilites :**
- Exposition de la liste des modules presents via l'API de decouverte (via BondingBrother)
- Enregistrement des capacites et permissions lors de l'ajout d'un module
- Retrait ou mise a jour du registre lors du verrouillage ou de la suppression d'un module

MiyukiniAdmin interroge Master Butler exclusivement via BondingBrother pour identifier les modules et gerer leur cycle de vie.

### 8.7 TAMR

**Role :** Champ d'action integrite et interventions humaines.

**Responsabilites :**
- Verification d'integrite des modules en collaboration avec MiyukiniAdmin (limites infranchissables, integrite systeme)
- Cadre conceptuel pour les interventions humaines si une verification d'integrite le requiert

MiyukiniAdmin sollicite TAMR via BondingBrother pour la verification d'integrite des modules ; TAMR ne prend pas de decision mais definit le cadre (voir contrats TAMR).

---

## 9. Position dans la Pyramide

### 9.1 Positionnement

```
┌──────────────────────────────────────────┐
│ STRATE 9 — MiyukiniAdmin (EXCEPTION)     │
│ Operateur Souverain d'administration     │
└──────────────────────────────────────────┘
          ▲
          │ (hors pyramide)
          │
┌──────────────────────────────────────────┐
│ STRATE 7 — Operateurs                    │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 6 — Outils & Kits d'Outils        │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 5 — BondingBrother (Adaptateur)   │
└──────────────────────────────────────────┘
          ▲
┌──────────────────────────────────────────┐
│ STRATE 4 — Cores Systeme                 │
└──────────────────────────────────────────┘
```

**MiyukiniAdmin est au-dessus de la pyramide, pas dedans.**

Il observe, installe, arbitre, mais ne vit pas dans le flux normal.

### 9.2 Regles de Communication

- Passe par BondingBrother
- Respecte les contrats des cores
- Peut invoquer des capacites reservees
- N'expose rien en retour

---

## 10. Signature Conceptuelle

**MiyukiniAdmin est au Miyukini Core ce que le BIOS/UEFI est a un OS moderne :**

**Autonome, puissant, dangereux s'il est mal utilise — et absolument necessaire.**

---

## 11. Resume

### 11.1 Ce que MiyukiniAdmin EST

- Console root
- Operateur Souverain, autonome, non reutilisable
- Auto-suffisant fonctionnellement et visuellement
- Logique metier administrative, technique, souveraine
- UI propre, isolee, non reutilisable
- Aucun Operateur ne depend de lui
- Niveau de securite maximal
- Seule entite autorisee a : Installer Miyukini, Forcer la securite, Intervenir en recovery
- Toujours via BondingBrother
- Jamais silencieux
- Jamais implicite

### 11.2 Ce que MiyukiniAdmin N'EST PAS

- Un Operateur metier
- Un Outil ou Kit d'Outils
- Une API publique
- Un composant embarquable
- B2B / B2C / B2B2C
- Un outil reutilisable
- Un framework UI partage
- Une logique metier applicative

---

## 12. Documents Associes

- [MiyukiniAdmin - Index de Navigation](../_index.md)
- [MiyukiniAdmin - Module Testing and Lifecycle Contract](../contracts/testing/MiyukiniAdmin%20-%20Module%20Testing%20and%20Lifecycle%20Contract.md)
- [MiyukiniAdmin - Architecture & Flows](../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [Miyukini Conceptual References - MiyukiniAdmin Status](../../../reference/Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md)
- [Miyukini Conceptual References - Security Levels](../../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](../../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md)
- [BondingBrother - Documentation Fondatrice](../../BondingBrother/foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document fondateur de reference
