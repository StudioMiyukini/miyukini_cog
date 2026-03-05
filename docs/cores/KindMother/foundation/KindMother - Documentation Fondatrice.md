# Miyukini Core System â€” KindMother Documentation Fondatrice

## 1. Introduction

### RÃ´le de KindMother

KindMother (KM) est le moteur interne de donnÃ©es du Miyukini Core System (MCS) version 2.4. Il constitue la couche d'abstraction et d'orchestration de la persistance pour l'ensemble du systÃ¨me.

KindMother n'est pas un produit public. Il est conÃ§u avec une discipline de produit futur, mais reste strictement interne au systÃ¨me. Son existence est transparente pour les modules SPM CMS et les produits qui consomment ces modules.

### ProblÃ¨me que KindMother rÃ©sout

Dans l'architecture actuelle de MCS, les modules SPM CMS exposent des traits fonctionnels (ContentManager, MediaManager, etc.) que les produits implÃ©mentent via des adaptateurs. Ces adaptateurs gÃ¨rent directement la persistance selon les besoins du produit (PostgreSQL, MongoDB, fichiers, etc.).

Cette approche prÃ©sente plusieurs limitations :

1. **Absence de cohÃ©rence globale** : Chaque adaptateur gÃ¨re sa propre persistance sans garantie de cohÃ©rence entre modules ou instances.

2. **Pas de support offline-first** : Aucun mÃ©canisme pour fonctionner sans connexion rÃ©seau ou avec des instances locales dÃ©rivÃ©es.

3. **Synchronisation manuelle** : Les produits doivent implÃ©menter eux-mÃªmes la synchronisation entre instances, ce qui conduit Ã  de la duplication et des incohÃ©rences.

4. **Gestion d'identitÃ© dispersÃ©e** : Chaque adaptateur gÃ¨re ses propres identifiants d'instances, sans vision globale.

5. **Permissions conceptuelles non centralisÃ©es** : Les vÃ©rifications de permissions sont dispersÃ©es dans les adaptateurs sans cohÃ©rence systÃ©mique.

KindMother rÃ©sout ces problÃ¨mes en fournissant un moteur unifiÃ© qui :
- GÃ¨re l'identitÃ© des instances de base de donnÃ©es (mÃ¨re et filles)
- Garantit la cohÃ©rence des donnÃ©es Ã  travers les modules et les instances
- Supporte le mode offline-first avec synchronisation automatique
- Centralise la gestion des permissions conceptuelles
- Abstraction complÃ¨te de la persistance (SQLite interne, jamais exposÃ©)

### Positionnement

KindMother est un **moteur interne** :
- Il n'est pas exposÃ© comme API publique
- Il n'est pas un module SPM CMS
- Il n'est pas dans le kernel
- Il est utilisÃ© par les adaptateurs produits pour gÃ©rer la persistance de maniÃ¨re unifiÃ©e

KindMother est conÃ§u avec une **discipline de produit** :
- Architecture claire et documentÃ©e
- Contrats stables et Ã©volutifs
- PrÃªt pour une implÃ©mentation future en Rust
- Mais reste strictement interne au systÃ¨me

---

## 2. Positionnement dans Miyukini Core System

### Relation avec le Kernel

KindMother utilise les capacitÃ©s du kernel pour ses opÃ©rations fondamentales :

- **Id / IdGenerator** : GÃ©nÃ©ration et gestion des identifiants uniques pour les instances, les entitÃ©s, et les opÃ©rations de synchronisation
- **Clock** : Horodatage des opÃ©rations, dÃ©tection des conflits, gestion des deltas temporels
- **Logger** : Logging structurÃ© des opÃ©rations de persistance, synchronisation, et rÃ©solution de conflits

KindMother **ne modifie pas** le kernel. Il consomme uniquement les contrats existants (traits, types) sans introduire de dÃ©pendances inverses.

### Relation avec les Modules SPM

Les modules SPM CMS (Content, Hierarchy, Taxonomies, Media, Publication, Search) **ne connaissent pas** KindMother. Ils continuent d'exposer leurs traits fonctionnels (ContentManager, HierarchyManager, etc.) sans aucune rÃ©fÃ©rence Ã  la persistance ou Ã  la synchronisation.

Les **adaptateurs produits** qui implÃ©mentent ces traits utilisent KindMother pour gÃ©rer la persistance. L'adaptateur reÃ§oit une demande du module SPM, la traduit en opÃ©ration KindMother, puis retourne le rÃ©sultat au module.

**RÃ¨gle fondamentale :** Aucun module SPM ne parle directement Ã  une base de donnÃ©es. Toute interaction avec la persistance passe par KindMother via les adaptateurs produits.

### Relation avec l'Auth

KindMother gÃ¨re les **permissions conceptuelles**, pas l'authentification technique.

**Permissions conceptuelles** : VÃ©rifications au niveau des donnÃ©es (qui peut lire/Ã©crire quelles entitÃ©s selon les rÃ¨gles mÃ©tier). Ces permissions sont dÃ©finies par le produit et appliquÃ©es par KindMother lors des opÃ©rations de lecture/Ã©criture. KindMother ne dÃ©finit aucune rÃ¨gle de permission par dÃ©faut ; il exÃ©cute des rÃ¨gles fournies par le produit.

**Authentification technique** : Gestion des tokens, sessions, OAuth, JWT, etc. Cela reste du ressort du produit ou d'un module auth dÃ©diÃ©, en dehors de KindMother.

KindMother reÃ§oit un contexte d'autorisation (utilisateur, rÃ´les, permissions) du produit via l'adaptateur, puis applique les rÃ¨gles de permissions conceptuelles lors des opÃ©rations.

### Architecture de dÃ©pendances

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           PRODUIT                        â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  Adaptateurs SPM                    â”‚  â”‚
â”‚  â”‚  (implÃ©mentent les traits)         â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â”‚           â”‚                               â”‚
â”‚           â–¼                               â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  â”‚
â”‚  â”‚  KindMother                        â”‚  â”‚
â”‚  â”‚  (moteur de donnÃ©es)               â”‚  â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           â”‚
           â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚         MODULES SPM CMS                  â”‚
â”‚  (traits fonctionnels, pas de DB)       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           â”‚
           â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚           KERNEL                         â”‚
â”‚  (Id, Clock, Logger)                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Flux de dÃ©pendance :** Produit â†’ KindMother â†’ Modules SPM â†’ Kernel

**RÃ¨gle :** Les dÃ©pendances sont strictement unidirectionnelles. KindMother ne dÃ©pend pas des modules SPM, et les modules SPM ne dÃ©pendent pas de KindMother.

---

## 3. Concepts fondamentaux

### DB MÃ¨re

La **DB MÃ¨re** est la source de vÃ©ritÃ© unique et l'autoritÃ© centrale pour toutes les donnÃ©es du systÃ¨me. Elle dÃ©tient l'autoritÃ© finale pour valider et appliquer les changements. Toutes les instances filles synchronisent leurs donnÃ©es avec la DB MÃ¨re.

**CaractÃ©ristiques :**
- Source de vÃ©ritÃ© unique
- AutoritÃ© finale pour toutes les opÃ©rations d'Ã©criture
- Point de rÃ©fÃ©rence pour la synchronisation
- Une seule DB MÃ¨re par systÃ¨me MCS

### DB Fille

Une **DB Fille** est une instance locale dÃ©rivÃ©e de la DB MÃ¨re. Elle peut fonctionner de maniÃ¨re autonome (offline-first) et synchronise pÃ©riodiquement ses donnÃ©es avec la DB MÃ¨re.

**CaractÃ©ristiques :**
- Instance locale dÃ©rivÃ©e
- Fonctionne en mode offline
- Synchronise avec la DB MÃ¨re
- Peut avoir plusieurs DB Filles par systÃ¨me
- AutoritÃ© limitÃ©e (Ã©critures locales, validation par la MÃ¨re)

### Instance Identity

L'**Instance Identity** est l'identitÃ© unique d'une instance de base de donnÃ©es (mÃ¨re ou fille). Cette identitÃ© permet de distinguer les instances, de tracer l'origine des donnÃ©es, et de gÃ©rer la synchronisation.

**CaractÃ©ristiques :**
- Identifiant unique et immuable
- GÃ©nÃ©rÃ© par le kernel (Id)
- AssociÃ© Ã  chaque instance au moment de sa crÃ©ation
- UtilisÃ© pour la traÃ§abilitÃ© et la synchronisation

### WriteIntent

Un **WriteIntent** est une intention d'Ã©criture avant validation et synchronisation. Il reprÃ©sente une demande de modification qui doit Ãªtre validÃ©e selon les rÃ¨gles de permissions et de cohÃ©rence avant d'Ãªtre appliquÃ©e.

**CaractÃ©ristiques :**
- ReprÃ©sente une intention, pas une modification immÃ©diate
- Contient les donnÃ©es Ã  modifier et le contexte (utilisateur, permissions)
- Doit Ãªtre validÃ© avant application
- Peut Ãªtre rejetÃ© si les permissions ou la cohÃ©rence ne sont pas respectÃ©es
- En mode offline, les WriteIntent sont stockÃ©s localement et synchronisÃ©s plus tard

### Delta

Un **Delta** est la diffÃ©rence entre deux Ã©tats de donnÃ©es pour la synchronisation. Il reprÃ©sente les changements qui doivent Ãªtre propagÃ©s d'une instance Ã  une autre (MÃ¨re â†’ Fille ou Fille â†’ MÃ¨re).

**CaractÃ©ristiques :**
- ReprÃ©sente uniquement les diffÃ©rences, pas l'Ã©tat complet
- Contient les opÃ©rations (crÃ©ation, modification, suppression) avec leurs donnÃ©es
- UtilisÃ© pour optimiser la synchronisation (transfÃ©rer seulement les changements)
- Peut Ãªtre calculÃ© entre deux points dans le temps ou entre deux instances

### AutoritÃ©

L'**AutoritÃ©** est la capacitÃ© d'une instance Ã  valider et appliquer des changements. La DB MÃ¨re a l'autoritÃ© finale, tandis que les DB Filles ont une autoritÃ© limitÃ©e (Ã©critures locales, validation diffÃ©rÃ©e par la MÃ¨re).

**CaractÃ©ristiques :**
- DB MÃ¨re : autoritÃ© finale, toutes les Ã©critures sont validÃ©es immÃ©diatement
- DB Fille : autoritÃ© limitÃ©e, Ã©critures locales validÃ©es localement, validation finale par la MÃ¨re lors de la synchronisation
- Les conflits sont rÃ©solus selon l'autoritÃ© (prioritÃ© Ã  la MÃ¨re, ou rÃ©solution selon les rÃ¨gles du produit)

### Offline-first

L'**Offline-first** est la capacitÃ© Ã  fonctionner sans connexion Ã  la DB MÃ¨re. Une DB Fille peut continuer Ã  fonctionner normalement (lectures et Ã©critures locales) mÃªme si la connexion Ã  la MÃ¨re est indisponible, puis synchroniser les changements une fois la connexion rÃ©tablie.

**CaractÃ©ristiques :**
- Fonctionnement autonome sans connexion rÃ©seau
- Ã‰critures locales stockÃ©es et synchronisÃ©es plus tard
- Lectures depuis la copie locale
- DÃ©tection et rÃ©solution de conflits lors de la synchronisation
- Garantie de cohÃ©rence locale mÃªme en mode offline

---

## 4. Architecture logique (conceptuelle)

### Couches du moteur

KindMother est organisÃ© en couches logiques distinctes :

**1. Couche d'abstraction**
- Interface unifiÃ©e pour les opÃ©rations de donnÃ©es
- Masque les dÃ©tails de persistance aux adaptateurs
- DÃ©finit les contrats d'opÃ©rations (lecture, Ã©criture, synchronisation)

**2. Couche d'orchestration**
- Coordonne les opÃ©rations entre les diffÃ©rentes parties du moteur
- GÃ¨re les WriteIntent et leur validation
- Orchestre la synchronisation entre instances
- Applique les rÃ¨gles de permissions conceptuelles

**3. Couche de persistance**
- GÃ¨re le stockage physique des donnÃ©es (SQLite interne)
- Abstraction complÃ¨te : SQLite n'est jamais exposÃ© aux adaptateurs
- GÃ¨re les transactions et la cohÃ©rence locale
- Optimise les accÃ¨s et les requÃªtes

**4. Couche de synchronisation**
- DÃ©tecte les deltas entre instances
- GÃ¨re la propagation des changements (MÃ¨re â†’ Fille, Fille â†’ MÃ¨re)
- RÃ©sout les conflits selon les rÃ¨gles dÃ©finies
- Assure la cohÃ©rence globale aprÃ¨s synchronisation

### Flux de lecture

**1. Demande de lecture**
- L'adaptateur produit reÃ§oit une demande du module SPM
- L'adaptateur traduit la demande en opÃ©ration KindMother (lecture d'entitÃ©)

**2. VÃ©rification des permissions**
- KindMother vÃ©rifie les permissions conceptuelles (l'utilisateur peut-il lire cette entitÃ© ?)
- Si refusÃ©, retourne une erreur de permission

**3. RÃ©solution de l'instance**
- KindMother dÃ©termine quelle instance contient les donnÃ©es (MÃ¨re ou Fille locale)
- En mode offline, utilise uniquement la Fille locale

**4. Lecture depuis la persistance**
- KindMother lit les donnÃ©es depuis la couche de persistance (SQLite interne)
- Les donnÃ©es sont formatÃ©es selon le contrat du module SPM

**5. Retour du rÃ©sultat**
- Les donnÃ©es sont retournÃ©es Ã  l'adaptateur
- L'adaptateur les retourne au module SPM
- Le module SPM les retourne au produit

### Flux d'Ã©criture

**1. Demande d'Ã©criture**
- L'adaptateur produit reÃ§oit une demande du module SPM
- L'adaptateur traduit la demande en WriteIntent KindMother

**2. CrÃ©ation du WriteIntent**
- KindMother crÃ©e un WriteIntent avec les donnÃ©es Ã  modifier et le contexte (utilisateur, permissions, horodatage)

**3. Validation des permissions**
- KindMother vÃ©rifie les permissions conceptuelles (l'utilisateur peut-il Ã©crire cette entitÃ© ?)
- Si refusÃ©, le WriteIntent est rejetÃ© et une erreur est retournÃ©e

**4. Validation de la cohÃ©rence**
- KindMother vÃ©rifie les contraintes de cohÃ©rence (rÃ©fÃ©rences valides, rÃ¨gles mÃ©tier, etc.)
- Si invalide, le WriteIntent est rejetÃ©

**5. Application du WriteIntent**
- **DB MÃ¨re :** Le WriteIntent est appliquÃ© immÃ©diatement dans la persistance
- **DB Fille :** Le WriteIntent est appliquÃ© localement et marquÃ© pour synchronisation

**6. Retour du rÃ©sultat**
- Le rÃ©sultat (succÃ¨s ou erreur) est retournÃ© Ã  l'adaptateur
- L'adaptateur le retourne au module SPM
- Le module SPM le retourne au produit

### Flux de synchronisation

**1. DÃ©clenchement de la synchronisation**
- La synchronisation peut Ãªtre dÃ©clenchÃ©e automatiquement (pÃ©riodique) ou manuellement
- Peut Ãªtre MÃ¨re â†’ Fille (propagation) ou Fille â†’ MÃ¨re (remontÃ©e)

**2. DÃ©tection des deltas**
- KindMother compare l'Ã©tat de l'instance source avec l'instance cible
- Calcule les deltas (diffÃ©rences) depuis le dernier point de synchronisation

**3. Validation des deltas**
- Chaque delta est validÃ© selon les permissions et la cohÃ©rence
- Les deltas invalides sont rejetÃ©s ou mis en quarantaine

**4. DÃ©tection de conflits**
- Si un mÃªme Ã©lÃ©ment a Ã©tÃ© modifiÃ© dans les deux instances, un conflit est dÃ©tectÃ©
- Les conflits sont rÃ©solus selon les rÃ¨gles (prioritÃ© MÃ¨re, dernier gagnant, fusion, etc.)

**5. Application des deltas**
- Les deltas validÃ©s sont appliquÃ©s Ã  l'instance cible
- Les transactions garantissent la cohÃ©rence (tout ou rien)

**6. Mise Ã  jour du point de synchronisation**
- Le point de synchronisation est mis Ã  jour pour les prochaines synchronisations
- Les mÃ©tadonnÃ©es de synchronisation sont mises Ã  jour

---

## 5. ResponsabilitÃ©s de KindMother

### Gestion de l'identitÃ© des instances

KindMother gÃ©nÃ¨re et gÃ¨re l'identitÃ© unique de chaque instance de base de donnÃ©es (DB MÃ¨re et DB Filles). Cette identitÃ© permet de :
- Distinguer les instances lors de la synchronisation
- Tracer l'origine des donnÃ©es et des modifications
- GÃ©rer les relations entre instances (MÃ¨re â†” Filles)
- Assurer la traÃ§abilitÃ© des opÃ©rations

### Garantie de cohÃ©rence des donnÃ©es

KindMother garantit la cohÃ©rence des donnÃ©es Ã  plusieurs niveaux :

**CohÃ©rence locale :** Au sein d'une instance, toutes les opÃ©rations respectent les contraintes de cohÃ©rence (rÃ©fÃ©rences valides, intÃ©gritÃ© rÃ©fÃ©rentielle, rÃ¨gles mÃ©tier).

**CohÃ©rence globale :** Entre les instances (MÃ¨re et Filles), la synchronisation assure que les donnÃ©es convergent vers un Ã©tat cohÃ©rent.

**CohÃ©rence transactionnelle :** Les opÃ©rations sont atomiques (tout ou rien) pour Ã©viter les Ã©tats incohÃ©rents.

### Support offline-first

KindMother permet aux DB Filles de fonctionner de maniÃ¨re autonome sans connexion Ã  la DB MÃ¨re :
- Lectures depuis la copie locale
- Ã‰critures locales stockÃ©es et synchronisÃ©es plus tard
- DÃ©tection automatique de la disponibilitÃ© de la connexion
- Synchronisation automatique ou manuelle une fois la connexion rÃ©tablie

### Synchronisation mÃ¨re/fille

KindMother orchestre la synchronisation bidirectionnelle entre la DB MÃ¨re et les DB Filles :
- Propagation des changements de la MÃ¨re vers les Filles
- RemontÃ©e des changements des Filles vers la MÃ¨re
- DÃ©tection et rÃ©solution de conflits
- Optimisation des transferts (deltas uniquement, pas l'Ã©tat complet)

**RÃ¨gle de souverainetÃ© :** MÃªme en synchronisation bidirectionnelle, la DB MÃ¨re conserve l'autoritÃ© finale sur l'Ã©tat global. Cette souverainetÃ© Ã©vite toute interprÃ©tation CRDT ou peer-to-peer oÃ¹ les instances auraient une autoritÃ© Ã©quivalente.

### Gestion des permissions conceptuelles

KindMother applique les rÃ¨gles de permissions conceptuelles dÃ©finies par le produit :
- VÃ©rification des permissions avant chaque opÃ©ration de lecture/Ã©criture
- Support de contextes d'autorisation complexes (utilisateur, rÃ´les, ressources)
- Rejet des opÃ©rations non autorisÃ©es avec erreurs explicites
- TraÃ§abilitÃ© des vÃ©rifications de permissions

### Abstraction de la persistance

KindMother abstrait complÃ¨tement la persistance :
- Utilise SQLite en interne pour le stockage
- SQLite n'est jamais exposÃ© aux adaptateurs ou aux modules
- L'interface est purement conceptuelle (opÃ©rations, pas SQL)
- Permet un changement futur de moteur de persistance sans impact sur les adaptateurs

---

## 6. Ce que KindMother ne fait PAS

### N'est pas un ORM

KindMother n'est pas un Object-Relational Mapping. Il ne fournit pas de mapping automatique entre objets et tables de base de donnÃ©es. Les adaptateurs produits sont responsables de la traduction entre les types des modules SPM et les structures de donnÃ©es de KindMother.

### N'expose pas SQLite directement

SQLite est utilisÃ© en interne par KindMother, mais n'est jamais exposÃ© aux adaptateurs ou aux modules. Aucune requÃªte SQL, aucun schÃ©ma SQLite, aucune API SQLite n'est accessible depuis l'extÃ©rieur de KindMother.

### Ne gÃ¨re pas l'authentification technique

KindMother ne gÃ¨re pas l'authentification technique (tokens, sessions, OAuth, JWT, etc.). Il reÃ§oit un contexte d'autorisation du produit via l'adaptateur et applique les permissions conceptuelles, mais l'authentification reste du ressort du produit ou d'un module auth dÃ©diÃ©.

### N'est pas un framework applicatif

KindMother n'est pas un framework applicatif complet. Il ne fournit pas de routes HTTP, de middlewares, de validation de payloads, ou d'autres fonctionnalitÃ©s applicatives. Il se concentre uniquement sur la gestion des donnÃ©es.

### Ne contient pas de logique mÃ©tier

KindMother ne contient aucune logique mÃ©tier spÃ©cifique. Il applique les rÃ¨gles de permissions et de cohÃ©rence dÃ©finies par le produit, mais ne dÃ©finit pas ces rÃ¨gles. Toute logique mÃ©tier (validation, rÃ¨gles business, workflows) reste dans le produit.

### N'est pas un module SPM

KindMother n'est pas un module SPM CMS. Il ne fournit pas de capacitÃ©s fonctionnelles rÃ©utilisables comme Content ou Media. Il est un moteur interne de donnÃ©es, utilisÃ© par les adaptateurs produits pour gÃ©rer la persistance.

### Ne gÃ¨re pas le rendu ou l'UI

KindMother ne gÃ¨re aucun aspect de rendu ou d'interface utilisateur. Il se concentre uniquement sur la gestion des donnÃ©es et leur persistance.

### Ne fournit pas de recherche full-text

KindMother ne fournit pas de capacitÃ©s de recherche full-text. La recherche reste du ressort du module Search SPM CMS, qui peut utiliser KindMother pour la persistance mais gÃ¨re sa propre indexation et recherche.

---

## 7. Relations avec les autres Cores

### Vue d'ensemble

KindMother s'intÃ¨gre dans l'Ã©cosystÃ¨me Miyukini Core System en collaboration Ã©troite avec les autres Cores de la Strate 4 et les couches adjacentes. Cette section dÃ©finit les relations structurelles et les contrats inter-Cores.

### StrongFather â€” ComplÃ©mentaritÃ© DÃ©cision/Persistance

KindMother et StrongFather sont **complÃ©mentaires par conception** :

| ResponsabilitÃ© | StrongFather | KindMother |
|---------------|--------------|------------|
| DÃ©cision stratÃ©gique | âœ… | âŒ |
| Persistance des donnÃ©es | âŒ | âœ… |
| Validation des intentions | âœ… (PolicyEngine) | âŒ |
| ExÃ©cution des Ã©critures | âŒ | âœ… |

**Invariant INV-SF-2 :** StrongFather ne persiste jamais directement â€” la persistance appartient Ã  KindMother.

**Interdictions structurelles :**

| Code | Interdiction |
|------|--------------|
| **INTERD-KM-1** | KindMother ne peut pas prendre de dÃ©cisions stratÃ©giques |
| **INTERD-KM-2** | KindMother ne peut pas exposer SQLite ou ses schÃ©mas |
| **INTERD-KM-3** | KindMother ne peut pas bloquer le systÃ¨me en attente de rÃ©seau |
| **INTERD-KM-4** | KindMother ne peut pas contenir de logique mÃ©tier spÃ©cifique |

### BondingBrother â€” DÃ©lÃ©gation des intentions de donnÃ©es

BondingBrother (Strate 5 - Liaison gouvernÃ©e) dÃ©lÃ¨gue les opÃ©rations de donnÃ©es Ã  KindMother selon les contrats de dÃ©lÃ©gation :

| Code | Contrat de dÃ©lÃ©gation |
|------|----------------------|
| **KM-DELEG-01** | BondingBrother dÃ©lÃ¨gue les WriteIntent Ã  KindMother aprÃ¨s validation StrongFather |
| **KM-DELEG-02** | BondingBrother ne contourne jamais KindMother pour la persistance |
| **KM-DELEG-03** | BondingBrother transmet le contexte d'autorisation complet Ã  KindMother |

**Flux de dÃ©lÃ©gation :**

```
BondingBrother â†’ StrongFather (validation) â†’ KindMother (persistance)
```

### WorrySentinel â€” IntÃ©gration sÃ©curitÃ©

WorrySentinel (autoritÃ© sÃ©curitÃ©) peut interagir avec KindMother pour :

- **RÃ©vocation de mandats** : Invalider des autorisations stockÃ©es
- **Audit de sÃ©curitÃ©** : Consultation des traces d'opÃ©rations
- **Blocage d'urgence** : Suspension temporaire d'opÃ©rations (via StrongFather)

### Caring Nanny â€” Monitoring et dÃ©tection d'anomalies

Caring Nanny (Strate 3 - Supervision) surveille les patterns de KindMother pour :

- **DÃ©tection d'anomalies** : Patterns d'accÃ¨s inhabituels, volumes anormaux
- **SantÃ© du systÃ¨me** : MÃ©triques de synchronisation, latences
- **Alertes proactives** : DÃ©gradation de performance, conflits rÃ©currents

### Diagramme de relations

```mermaid
graph TB
    subgraph Strate4[Strate 4 - Cores SystÃ¨me]
        SF[StrongFather<br/>DÃ©cision]
        KM[KindMother<br/>Persistance]
        WS[WorrySentinel<br/>SÃ©curitÃ©]
    end

    subgraph Strate5[Strate 5 - Liaison]
        BB[BondingBrother<br/>MÃ©diation]
    end

    subgraph Strate3[Strate 3 - Supervision]
        CN[Caring Nanny<br/>Monitoring]
    end

    BB -->|"DÃ©lÃ¨gue donnÃ©es (KM-DELEG-*)"| KM
    BB -->|"DÃ©lÃ¨gue dÃ©cisions"| SF
    SF -.->|"ComplÃ©mentaire (INV-SF-2)"| KM
    KM -.->|"Monitoring patterns"| CN
    WS -.->|"RÃ©vocation mandats"| KM

    classDef coreData fill:#e1f5fe
    classDef coreDecision fill:#fff3e0
    classDef liaison fill:#f3e5f5
    classDef supervision fill:#e8f5e9

    class KM coreData
    class SF coreDecision
    class BB liaison
    class CN supervision
```

### RÃ©fÃ©rences croisÃ©es

- [StrongFather - Documentation Fondatrice](../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)
- [BondingBrother - Strate de Liaison GouvernÃ©e](..//..//BondingBrother//_index.md)
- [Connexion Inter-COG](..//..//..//miyukini-webway-system//reference//_index.md)
- [Ecosystem Dependency Contract](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 8. Profils d'usage

### Application locale

**Contexte :** Application desktop ou mobile qui fonctionne principalement en local, avec synchronisation occasionnelle.

**Configuration :** DB Fille seule, mode offline-first.

**Comportement :**
- Toutes les opÃ©rations (lecture et Ã©criture) se font localement
- Les donnÃ©es sont stockÃ©es dans la DB Fille locale
- Synchronisation pÃ©riodique ou manuelle avec la DB MÃ¨re
- Fonctionne mÃªme sans connexion rÃ©seau

**Exemples :** Application de prise de notes, gestionnaire de tÃ¢ches local, Ã©diteur de documents offline.

### Site web / CMS

**Contexte :** Site web ou CMS qui fonctionne principalement en ligne, avec accÃ¨s via KindMother en mode DB MÃ¨re.

**Configuration :** AccÃ¨s direct via KindMother en mode DB MÃ¨re.

**Comportement :**
- Toutes les opÃ©rations transitent par KindMother en mode DB MÃ¨re
- Pas de mode offline (le site nÃ©cessite une connexion serveur)
- Synchronisation en temps rÃ©el (pas de dÃ©lai)
- AutoritÃ© finale pour toutes les Ã©critures

**Exemples :** CMS web classique, site e-commerce, application SaaS.

### Jeu solo

**Contexte :** Jeu vidÃ©o solo qui fonctionne entiÃ¨rement en local, sans synchronisation avec un serveur.

**Configuration :** DB Fille locale, pas de synchronisation.

**Comportement :**
- Toutes les donnÃ©es sont stockÃ©es localement
- Pas de synchronisation avec une DB MÃ¨re
- Fonctionne entiÃ¨rement offline
- Pas de partage de donnÃ©es entre instances

**Exemples :** Jeu solo avec sauvegarde locale, simulateur local, application de crÃ©ation solo.

### Jeu asynchrone

**Contexte :** Jeu multijoueur asynchrone oÃ¹ les joueurs interagissent de maniÃ¨re dÃ©calÃ©e dans le temps.

**Configuration :** DB Fille par joueur, synchronisation pÃ©riodique avec DB MÃ¨re.

**Comportement :**
- Chaque joueur a sa propre DB Fille locale
- Les actions sont effectuÃ©es localement et synchronisÃ©es pÃ©riodiquement
- La DB MÃ¨re maintient l'Ã©tat global du jeu
- RÃ©solution de conflits lors de la synchronisation (ex. deux joueurs modifient la mÃªme ressource)

**Exemples :** Jeu de stratÃ©gie asynchrone, jeu de gestion multijoueur, application collaborative avec sync pÃ©riodique.

### Jeu temps rÃ©el (cache only)

**Contexte :** Jeu multijoueur temps rÃ©el oÃ¹ la latence est critique et la persistance est secondaire.

**Configuration :** Pas de persistance KindMother, cache uniquement.

**Comportement :**
- Les donnÃ©es sont en mÃ©moire uniquement (cache)
- Pas de persistance via KindMother (trop de latence)
- Persistance Ã©ventuelle via d'autres mÃ©canismes (sauvegarde pÃ©riodique, snapshots)
- KindMother n'est pas utilisÃ© pour ce profil d'usage

**Exemples :** Jeu d'action temps rÃ©el, jeu de combat multijoueur, application temps rÃ©el avec cache mÃ©moire.

---

## 9. DÃ©cisions fondatrices

### Principes non nÃ©gociables

**Offline-first :** KindMother doit supporter le mode offline-first pour les DB Filles. C'est un principe fondamental qui ne peut pas Ãªtre compromis. Toute implÃ©mentation doit garantir que les DB Filles fonctionnent de maniÃ¨re autonome.

**CohÃ©rence garantie :** KindMother doit garantir la cohÃ©rence des donnÃ©es Ã  tous les niveaux (local, global, transactionnel). Aucune opÃ©ration ne doit laisser le systÃ¨me dans un Ã©tat incohÃ©rent.

**Abstraction complÃ¨te :** SQLite (ou tout autre moteur de persistance) ne doit jamais Ãªtre exposÃ© aux adaptateurs ou aux modules. L'abstraction doit Ãªtre complÃ¨te et totale.

**Aucun module ne parle directement Ã  une DB :** RÃ¨gle fondamentale de l'architecture MCS. Toute interaction avec la persistance passe par KindMother via les adaptateurs produits.

**Permissions conceptuelles centralisÃ©es :** Les vÃ©rifications de permissions conceptuelles doivent Ãªtre centralisÃ©es dans KindMother, pas dispersÃ©es dans les adaptateurs.

### DÃ©cisions verrouillÃ©es

**DÃ©cision D1 â€” KindMother est le SEUL point d'entrÃ©e data :**

Toute opÃ©ration de lecture ou d'Ã©criture persistÃ©e dans MCS doit transiter par KindMother. Toute exception est considÃ©rÃ©e comme une violation architecturale. Cette dÃ©cision protÃ¨ge contre les contournements "juste pour tester" ou les optimisations prÃ©maturÃ©es qui bypasseraient KindMother.

**DÃ©cision D2 â€” SQLite est un dÃ©tail d'implÃ©mentation :**

SQLite est un dÃ©tail d'implÃ©mentation interne Ã  KindMother. Aucune hypothÃ¨se sur SQLite ne doit apparaÃ®tre hors de KindMother. Les adaptateurs et modules ne doivent jamais faire d'hypothÃ¨ses sur la structure, les schÃ©mas, ou les capacitÃ©s de SQLite. L'abstraction doit Ãªtre totale.

**DÃ©cision D3 â€” KindMother ne garantit PAS la compatibilitÃ© rÃ©troactive (v0.x) :**

KindMother ne garantit aucune compatibilitÃ© rÃ©troactive tant qu'il est en version interne (v0.x). Cette dÃ©cision libÃ¨re l'implÃ©mentation de contraintes de compatibilitÃ© prÃ©maturÃ©es et permet des Ã©volutions architecturales significatives sans impact sur les produits consommateurs.

### Contraintes assumÃ©es

**SQLite interne :** KindMother utilise SQLite comme moteur de persistance interne. Cette contrainte est assumÃ©e pour la v2.4, mais l'abstraction permet un changement futur sans impact sur les adaptateurs.

**Rust (futur) :** KindMother sera implÃ©mentÃ© en Rust, mais cette Ã©tape est strictement documentaire. Aucune implÃ©mentation technique n'est requise pour cette documentation.

**Discipline produit :** KindMother est conÃ§u avec une discipline de produit (architecture claire, contrats stables, documentation complÃ¨te) mÃªme s'il reste interne au systÃ¨me.

**Pas de dÃ©pendance au kernel :** KindMother utilise les capacitÃ©s du kernel (Id, Clock, Logger) mais ne modifie pas le kernel et n'introduit pas de dÃ©pendances inverses.

**Pas de logique mÃ©tier :** KindMother ne contient aucune logique mÃ©tier spÃ©cifique. Toute logique mÃ©tier reste dans le produit.

### LibertÃ©s laissÃ©es Ã  l'implÃ©mentation

**StratÃ©gies de synchronisation :** L'implÃ©mentation peut choisir la stratÃ©gie de synchronisation (push, pull, hybride, pÃ©riodique, Ã©vÃ©nementielle) selon les besoins et les contraintes.

**RÃ©solution de conflits :** L'implÃ©mentation peut choisir la stratÃ©gie de rÃ©solution de conflits (prioritÃ© MÃ¨re, dernier gagnant, fusion, rÃ©solution manuelle) selon les besoins du produit.

**Optimisations de persistance :** L'implÃ©mentation peut optimiser la persistance (indexation, cache, requÃªtes optimisÃ©es) tant que l'abstraction reste complÃ¨te.

**Gestion des transactions :** L'implÃ©mentation peut choisir le niveau d'isolation et la gestion des transactions selon les besoins de cohÃ©rence et de performance.

**MÃ©triques et observabilitÃ© :** L'implÃ©mentation peut ajouter des mÃ©triques et de l'observabilitÃ© (logs dÃ©taillÃ©s, mÃ©triques de performance, traces) tant que cela reste interne et n'expose pas SQLite.

**Ã‰volution du moteur de persistance :** L'implÃ©mentation peut changer le moteur de persistance (de SQLite Ã  autre chose) tant que l'abstraction reste complÃ¨te et que les adaptateurs ne sont pas impactÃ©s.

---

## 10. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce core respecte les **Lois d'Autonomie SystÃ¨me** dÃ©finies dans [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md). KindMother est **compatible** avec ces lois si les principes offline-first sont respectÃ©s.

### LOI-1 : Aucune dÃ©pendance externe critique Ã  l'exÃ©cution

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-1 :
- La **persistance locale est toujours disponible** (DB Fille en mode offline-first)
- Les opÃ©rations de lecture et d'Ã©criture fonctionnent sans connexion rÃ©seau
- SQLite interne garantit l'autonomie de la persistance
- L'absence de connexion ne bloque jamais le dÃ©marrage ni le fonctionnement de KindMother

**Architecture :** La DB Fille peut fonctionner de maniÃ¨re autonome sans connexion Ã  la DB MÃ¨re.

### LOI-2 : Le systÃ¨me accepte l'isolement comme Ã©tat normal

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-2 :
- Les **WriteIntent sont acceptÃ©s localement** et synchronisÃ©s plus tard
- Pas de blocage en attente de validation distante
- L'isolement active un mode offline explicite, pas une cascade d'erreurs
- Les opÃ©rations locales continuent normalement en mode isolÃ©

**Architecture :** Le mode offline-first est un principe fondamental de KindMother (Section 8, DÃ©cisions fondatrices).

### LOI-3 : L'Ã©tat local est souverain

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-3 :
- La **DB Fille dÃ©tient l'autoritÃ© locale** sur ses donnÃ©es
- Les dÃ©cisions prises localement sont valides localement
- La rÃ©conciliation avec la DB MÃ¨re est **explicite et traÃ§able** (voir Sync & Conflict Resolution Contract)
- Ã€ la reconnexion : rÃ©conciliation, comparaison, explication â€” jamais de "correction en douce"

**Architecture :** La souverainetÃ© de la DB MÃ¨re est prÃ©servÃ©e, mais la DB Fille est souveraine localement.

### LOI-4 : Pas de temps global requis

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-4 :
- La synchronisation utilise des **deltas et des points de synchronisation**, pas des timestamps absolus
- Les conflits ne se rÃ©solvent pas par "le plus rÃ©cent gagne" de maniÃ¨re implicite
- Le kernel Clock fournit un temps local, pas global
- Les comparaisons temporelles entre instances sont explicitement encadrÃ©es

**Architecture :** La synchronisation est basÃ©e sur des deltas et des points de synchronisation, pas sur des timestamps absolus.

### LOI-5 : Le coÃ»t doit Ãªtre proportionnel au hardware

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-5 :
- **SQLite interne**, optimisÃ© pour les ressources limitÃ©es
- MÃ©moire maÃ®trisÃ©e (pas de cache massif par dÃ©faut)
- CPU prÃ©visible (opÃ©rations transactionnelles, pas de workers inutiles)
- Pas de services fantÃ´mes consommant des ressources en arriÃ¨re-plan

**Architecture :** SQLite est un choix dÃ©libÃ©rÃ© pour la compatibilitÃ© avec hardware simple (Raspberry Pi, mini PC, etc.).

### LOI-6 : L'autonomie n'empÃªche pas la fÃ©dÃ©ration

**ConformitÃ© :** âœ… **Conforme**

KindMother respecte intÃ©gralement LOI-6 :
- La synchronisation MÃ¨re/Fille est **explicite et contrÃ´lÃ©e**
- Un nÅ“ud peut fonctionner sans synchronisation (DB Fille autonome)
- La synchronisation est **rÃ©versible** (un nÅ“ud peut se dÃ©connecter)
- Les Ã©changes de synchronisation sont **traÃ§ables** (deltas, journaux)

**Architecture :** La synchronisation est optionnelle et contrÃ´lÃ©e, jamais obligatoire pour le fonctionnement local.

### Points de vigilance

Pour maintenir la conformitÃ© aux lois d'autonomie :
- La synchronisation MÃ¨re/Fille doit rester **explicite et non-bloquante**
- La rÃ©solution de conflits ne doit pas prÃ©supposer de **temps global**
- Les WriteIntent doivent toujours Ãªtre acceptÃ©s localement, mÃªme en mode isolÃ©

---

## Conclusion

KindMother est le moteur interne de donnÃ©es du Miyukini Core System v2.4. Il rÃ©sout les problÃ¨mes de cohÃ©rence, de synchronisation, et d'offline-first en fournissant une abstraction complÃ¨te de la persistance et une orchestration unifiÃ©e des donnÃ©es.

Cette documentation fondatrice dÃ©finit les concepts, l'architecture, et les responsabilitÃ©s de KindMother sans entrer dans les dÃ©tails d'implÃ©mentation. Elle sert de rÃ©fÃ©rence pour une future implÃ©mentation en Rust et garantit que chaque concept est comprÃ©hensible indÃ©pendamment.

KindMother reste strictement interne au systÃ¨me, conÃ§u avec une discipline de produit mais sans Ãªtre un produit public. Il s'intÃ¨gre dans l'architecture MCS en respectant les principes fondamentaux : dÃ©pendances unidirectionnelles, sÃ©paration des responsabilitÃ©s, et abstraction complÃ¨te.

---

**Document crÃ©Ã© le :** 2026-01-24  
**Version :** 1.2  
**Statut :** Documentation fondatrice validÃ©e  
**DerniÃ¨re mise Ã  jour :** 2026-01-27 (ajout section Relations inter-Cores, correction liens)  
**RÃ©fÃ©rence :** Miyukini Core System v2.4, [Miyukini Conceptual References - Integrity Degradation System](..//..//..//miyukini-webway-system//reference//_index.md) (sondes environnementales, corruption disque)


