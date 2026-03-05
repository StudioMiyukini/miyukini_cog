# MiyukiniAdmin â€” Vocabulary & Glossary

## 1. Contexte

Ce document definit le **vocabulaire canonique** de MiyukiniAdmin. Il etablit les termes officiels, leurs definitions et leur usage correct dans le contexte de MiyukiniAdmin.

## 2. Portee / Scope

Ce document definit :
- Les termes specifiques a MiyukiniAdmin
- Les definitions canoniques
- Les relations entre concepts
- Les termes a eviter

---

## 3. Termes Fondamentaux

### 3.1 MiyukiniAdmin

**Definition :** Operateur Souverain de l'ecosysteme Miyukini, constituant la console root d'administration.

**Caracteristiques :**
- Strate 9 de la pyramide
- Auto-suffisant (backend + frontend internes)
- Non reutilisable par d'autres Operateurs
- Acces exclusif via BondingBrother

**Usage :**
- âœ“ "MiyukiniAdmin est la console root"
- âœ— "MiyukiniAdmin est un outil"
- âœ— "MiyukiniAdmin est une API"

---

### 3.2 Console Root

**Definition :** Interface d'administration de niveau systeme, comparable a un BIOS/UEFI pour un OS.

**Caracteristiques :**
- Hors du flux applicatif normal
- Autorite quasi-ultime
- Acces aux operations critiques

**Usage :**
- âœ“ "MiyukiniAdmin fonctionne comme une console root"
- âœ— "MiyukiniAdmin est un backoffice"

---

### 3.3 Operateur Souverain

**Definition :** Type special d'Operateur qui echappe aux regles standard de dependance et de composition.

**Caracteristiques :**
- Ne depend d'aucun Outil ou Kit d'Outils
- N'est dependance d'aucun autre Operateur
- Autorite administrative sur le systeme

**Usage :**
- âœ“ "MiyukiniAdmin est l'unique Operateur Souverain"
- âœ— "Les Operateurs Souverains sont..."

---

## 4. Termes d'Architecture

### 4.1 Admin Bridge

**Definition :** Composant interne de MiyukiniAdmin servant d'interface unique vers BondingBrother.

**Responsabilites :**
- Traduction des requetes admin
- Gestion des sessions
- Serialisation/deserialisation

**Usage :**
- âœ“ "L'Admin Bridge communique avec BondingBrother"
- âœ— "L'Admin Bridge accede directement aux cores"

---

### 4.2 Strate 9

**Definition :** Position de MiyukiniAdmin dans la pyramide Miyukini, au-dessus de la Strate 7 (Operateurs standard).

**Caracteristiques :**
- Exception a la pyramide standard
- Au-dessus, pas dedans
- Observe et administre

---

## 5. Termes de Securite

### 5.1 Security Level (Niveau de Securite)

**Definition :** Niveau de 0 a 4 definissant le profil de risque et les controles appliques.

| Niveau | Nom | Description |
|--------|-----|-------------|
| 0 | PUBLIC | Securite minimale |
| 1 | STANDARD | Securite basique |
| 2 | SENSITIVE | Donnees sensibles |
| 3 | CRITICAL | Systemes critiques |
| 4 | HARDENED | Isolation maximale |

---

### 5.2 Trust Level (Niveau de Confiance)

**Definition :** Etat d'integrite du systeme de T0 a T4.

| Niveau | Nom | Description |
|--------|-----|-------------|
| T0 | Normal | Fonctionnement optimal |
| T1 | Attention | Anomalies mineures |
| T2 | Degrade | Performance reduite |
| T3 | Critique | Intervention requise |
| T4 | Urgence | Mode survie |

---

### 5.3 Degradation Mode (Mode de Degradation)

**Definition :** Etat temporaire restreignant les fonctionnalites pour proteger le systeme.

| Mode | Description |
|------|-------------|
| WATCHFUL | Surveillance renforcee |
| RESTRICTED | Fonctions sensibles desactivees |
| LOCKDOWN | Lecture seule |
| ISOLATED | Isolation complete |

---

### 5.4 Futur Admin (Future Admin)

**Definition :** En environnement **vierge**, utilisateur considere comme le futur administrateur du COG pendant le processus d'installation. Il n'a pas encore de compte admin ; il est dirige vers le parcours d'installation (creation compte admin, generation EIP, config minimale). Aucun droit sur les donnees metier ni sur les Operateurs tant que le verrou StrongFather bootstrap est actif.

**Usage :** "L'utilisateur actuel est traite comme Futur Admin jusqu'a la creation du premier compte admin."

**Voir :** [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)

---

### 5.5 Environnement compromis (attaque, troncature, alteration)

**Definition :** Etat de l'environnement ou des donnees critiques sont **presentes mais invalides** (EIP corrompu ou tag/hash invalides, registre admin incoherent, schema tronque) ou **incoherentes** entre elles. Indique une intrusion, une panne grave ou une alteration malveillante. Un environnement compromis **n'est pas** un environnement vierge ; il declenche une **reponse securitaire** (mode degrade, blocage login, alerte, procedure de recovery gouvernÃ©e), pas le parcours d'installation (Futur Admin).

**Usage :** "L'environnement a ete detecte comme compromis ; la reponse securitaire est activee."

**Voir :** [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) sections 3.2 Ã  3.5

---

### 5.6 Reponse securitaire (environnement compromis)

**Definition :** Ensemble de mesures appliquees lorsque l'environnement est classe **compromis** (attaque, troncature, alteration) : passage WorrySentinel en T3/T4 (mode degrade / lockdown), affichage page dediee Â« Environnement compromis Â» (pas de login, pas de parcours installation), alerte et audit (ENVIRONMENT_COMPROMISED). Si l'humain peut intervenir : reprise uniquement via **procedure de recovery gouvernÃ©e**. Si l'**interface humaine est compromise** (auth, donnees admin, MiyukiniAdmin) et l'**humain ne peut pas intervenir** : **recovery/rollback automatique** lancee ; si echec, donnees DB detruites (jugees perdues), environnement reinitialise en **vierge avec memoire de corruption passee**.

**Usage :** "La reponse securitaire est en vigueur ; la procedure de recovery doit etre suivie (ou la recovery automatique s'applique)."

**Voir :** [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) sections 3.5 et 3.5.4

---

### 5.6bis Vierge avec memoire de corruption (Virgin with memory of past corruption)

**Definition :** Etat d'un environnement **reinitialise en vierge** apres echec de la **recovery automatique** (interface humaine compromise, humain ne pouvant pas intervenir). **Avant destruction** : si aucune sauvegarde locale antÃ©rieure des donnees DB n'existait, une **sauvegarde compressee** a ete effectuee (dernier recours pour examen forensique ou recuperation ultÃ©rieure). Les donnees DB ont ensuite ete **totalement detruites** et **jugees perdues**. L'environnement est **vierge** (parcours Futur Admin, premier boot) mais conserve une **memoire de sa corruption passee** : une trace persistante (audit, pas de donnees sensibles) indique qu'il a deja ete compromis et reinitialise apres echec de recovery automatique. Cette memoire sert a l'audit, au diagnostic et eventuellement a des politiques renforcees (vigilance, alerte).

**Usage :** "L'environnement est vierge avec memoire de corruption ; le parcours d'installation s'applique, mais une trace de la compromission passee est conservee."

**Voir :** [Auth and First-Boot Contract](../contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) section 3.5.4.3 et 3.5.4.4

---

### 5.7 EIP (Environment Identity Protocol)

**Definition :** Protocole de production et de stockage **chiffre** des donnees d'identite du COG. Les Cores produisent ces donnees (environment_id, core_versions, integrity_hash, etc.) ; le blob est chiffre (AEAD) et persiste via KindMother. Utilise au premier demarrage pour attester l'identite de l'environnement.

**Usage :** "La generation de l'identite environnement suit le protocole EIP."

**Voir :** [Environment Identity Protocol EIP](..//..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)

---

### 5.8 RBAC MiyukiniAdmin (Roles et capacites)

**Definition :** Modele d'autorisation par rÃ´les integre a MiyukiniAdmin. Chaque compte admin a **un seul role** (Admin, Recovery ou Audit). Chaque role possede un ensemble **explicite** de **capacites** (permissions). Une action sensible est autorisee si le role du compte possede la capacite requise.

**Roles :**
- **Admin** : capacites standard (dashboard, metriques, securite, DB via KindMother, tests, gestion comptes Admin/Audit).
- **Recovery** : tout Admin + acces DB recovery + creation/revocation comptes Recovery.
- **Audit** : lecture seule (logs, metriques, etat securite, liste Operateurs).

**Voir :** [Permission Contract](../contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md)

---

### 5.9 Recovery Mode (Mode Recovery)

**Definition :** Etat exceptionnel permettant l'acces direct a la base de donnees, sous conditions cumulatives strictes.

**Conditions requises :**
1. Trust Level >= T3
2. Protocole securite renforce
3. MFA verifie
4. Approbation StrongFather
5. Fenetre temporelle limitee
6. Journalisation complete

---

## 6. Termes de Monitoring

### 6.1 System Metrics (Metriques Systeme)

**Definition :** Indicateurs de consommation des ressources (CPU, RAM, disque, reseau).

**Source :** CaringNanny

---

### 6.2 DB Metrics (Metriques DB)

**Definition :** Indicateurs de performance de la base de donnees (queries, latence, pool).

**Source :** CaringNanny + KindMother

---

### 6.3 Health Score (Score de Sante)

**Definition :** Score global de 0 a 100 representant la sante du systeme.

| Score | Etat |
|-------|------|
| 80-100 | Bon |
| 60-79 | Acceptable |
| 40-59 | Degrade |
| 0-39 | Critique |

---

## 7. Termes de Testing

### 7.1 Cycle Test (Test de Cycle)

**Definition :** Test de performance, latence ou montee en charge evaluant le comportement systeme.

**Types :**
- Tests de performance
- Tests de latence
- Tests de charge
- Tests de resilience

---

### 7.2 Unit Test (Test Unitaire)

**Definition :** Test de coherence ou conformite verifiant l'integrite des donnees.

**Types :**
- Tests de coherence DB
- Tests de conformite contractuelle
- Tests de sante structurelle

---

## 8. Termes d'Operations

### 8.1 Operator Isolation (Isolation Operateur)

**Definition :** Restriction ou blocage d'un Operateur specifique.

| Niveau | Effet |
|--------|-------|
| MONITOR | Surveillance renforcee |
| THROTTLE | Limitation debit |
| RESTRICTED | Fonctions bloquees |
| FULL | Acces bloque |

---

### 8.2 Maintenance Operation (Operation de Maintenance)

**Definition :** Operation de maintenance sur la base de donnees (validation, optimisation, reparation, migration).

**Caracteristiques :**
- Passe par KindMother
- Validation StrongFather pour modifications
- Tracabilite complete

---

## 9. Termes d'Audit

### 9.1 Audit Log (Journal d'Audit)

**Definition :** Enregistrement chronologique de toutes les actions effectuees dans MiyukiniAdmin.

**Contenu :**
- Horodatage
- Identite operateur
- Action effectuee
- Resultat
- Justification (si applicable)

---

### 9.2 Audit Trail (Piste d'Audit)

**Definition :** Chaine complete de logs permettant de reconstituer l'historique d'une operation.

---

## 10. Acronymes et Abbreviations

| Acronyme | Signification |
|----------|---------------|
| **SF** | StrongFather |
| **KM** | KindMother |
| **BB** | BondingBrother |
| **CN** | CaringNanny |
| **WS** | WorrySentinel |
| **MFA** | Multi-Factor Authentication |
| **UI** | User Interface |
| **DB** | Database |

---

## 11. Termes a Eviter

| Terme incorrect | Terme correct | Raison |
|-----------------|---------------|--------|
| "Outil admin" | "MiyukiniAdmin" | Ce n'est pas un Outil |
| "API publique" | - | MiyukiniAdmin n'a pas d'API publique |
| "Backend admin" | "MiyukiniAdmin backend" | Precision |
| "Product admin" | "MiyukiniAdmin" | Ce n'est pas un produit metier |

---

## 12. Documents Associes

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference


