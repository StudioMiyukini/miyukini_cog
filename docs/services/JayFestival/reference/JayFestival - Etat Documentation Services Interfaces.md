# JayFestival — État de la documentation des services interfacés

## Contexte

Ce document fait le point sur la **documentation** de tous les **services interfacés avec JayFestival** (services Jay, outils Miyu*, Cores), en vue d’une **implémentation complète incluant l’UI**. Pour chaque service, il indique ce qui existe, ce qui manque, et les **ambiguïtés ou choix humains** à trancher.

**Référence** : [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) pour les couplages côté JayFestival.

## Portée / Scope

- **Périmètre** : Services Jay (JayKoa, JayXpose, JayFaim, JayKonta), outils Miyu* (Miyauth, Miyuprofile, Miyunotify, Miyuinvoice, Miyubooking, Miyucms, Miyumedia, Miyufeeds, MiyuClock), Cores (StrongFather, KindMother, Master Butler, WorrySentinel), backend alpha (Supabase).
- **Critère « prêt pour implémentation complète UI incluse »** : Document fondateur ou équivalent ; Opérateurs / Kits ou contrats d’intégration ; parcours ou écrans décrits ; points d’entrée UI ou patterns d’intégration documentés lorsque l’UI est exposée ou intégrée dans JayFestival.
- **Hors périmètre** : Implémentation effective du code (ce document ne fait que l’audit de la doc).

---

## 1. Synthèse par service

| Service | Doc existante | Prêt implémentation complète (UI incluse) | Manques principaux |
|---------|----------------|-------------------------------------------|---------------------|
| **JayKoa** | Oui (fondateur, Opérateurs, Parcours, Ecrans et UI, Integration Consommateurs, Bornage, Maquettes) | **Oui** | — |
| **JayKonta** | Oui (fondateur, publics Account/Purse, Integration Services, Points d’entrée) | **Partiel** | Pas de doc « Ecrans et UI » dédiée ; UI à déduire des Parcours. |
| **JayXpose** | Document fondateur uniquement | **Non** | Opérateurs/Kits, Parcours, Ecrans/UI, Contrat d’intégration JayFestival. |
| **JayFaim** | Document fondateur uniquement | **Non** | Opérateurs/Kits, Parcours, Ecrans/UI, Contrat d’intégration JayFestival. |
| **Miyauth** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines, Audit | **Oui** (côté consommation) | Pas d’écrans propres ; JayFestival consomme auth → écrans Connexion/Inscription dans JayFestival. |
| **Miyuprofile** | _index + références éparses | **Oui** (alpha) | **P1 tranché** : Supabase uniquement pour le moment (source de vérité = tables Supabase). |
| **Miyunotify** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Oui** (côté consommation) | Pas d’écran propre ; JayFestival déclenche envois → pas d’UI dédiée côté JayFestival. |
| **Miyuinvoice** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Oui** (côté consommation) | UI devis/factures dans JayFestival selon spec UI Catakana ; couplage JayKonta à clarifier. |
| **Miyubooking** | Doc fondatrice, Reference Outils, contrats, Implementation Guidelines | **Partiel** | Parcours/écrans réservation dans JayFestival à aligner avec Miyubooking. |
| **Miyucms / Miyumedia** | Docs fondatrices, Reference Outils, contrats | **Partiel** | Contrat d’intégration « documents d’édition » depuis JayFestival à formaliser. |
| **Miyufeeds** | Doc fondatrice | **Partiel** | Usage « Actualités » public (phase 2) à borner. |
| **MiyuClock** | Doc fondatrice, Reference Outils, contrats (KindMother, Security, etc.) | **Oui** (côté consommation) | **P1 tranché** : MiyuClock **atteste l'horaire et la date IRL** ; JayKoa organise les données et fait l'interface utilisateur. |
| **KindMother / StrongFather / Master Butler / WorrySentinel** | Contrats COG (core) | **Oui** (côté gouvernance) | Pas d’UI propre ; intégration dans JayFestival via Mandats et persistance. |
| **Supabase** | Référence Base de Donnees et Migration | **Oui** (alpha) | Exception pré-COG ; pas d’écran « Supabase » — backend uniquement. |

---

## 2. Détail par service

### 2.1 JayKoa (agenda)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayKoa - Document Fondateur](../../JayKoa/JayKoa%20-%20Document%20Fondateur.md) |
| Opérateurs et Toolkits | Oui | [JayKoa - Operateurs et Toolkits](../../JayKoa/JayKoa%20-%20Operateurs%20et%20Toolkits.md) |
| Parcours utilisateurs | Oui | [JayKoa - Parcours Utilisateurs](../../JayKoa/JayKoa%20-%20Parcours%20Utilisateurs.md) |
| Ecrans et UI | Oui | [JayKoa - Ecrans et UI](../../JayKoa/JayKoa%20-%20Ecrans%20et%20UI.md) — composants (calendrier, alerte conflit, export) et intégration dans UIs consommatrices |
| Integration Services Consommateurs | Oui | [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) — types d’entrées JayFestival, responsabilités |
| Maquettes UI | Oui | [JayKoa - Maquettes UI Type Google Agenda](../../JayKoa/reference/JayKoa%20-%20Maquettes%20UI%20Type%20Google%20Agenda.md) |
| Bornage | Oui | [JayKoa - Bornage Implementation](../../JayKoa/JayKoa%20-%20Bornage%20Implementation.md) |

**Prêt pour implémentation complète UI incluse** : Oui. JayFestival doit intégrer les composants/patterns décrits dans « Ecrans et UI » (vue calendrier, alerte conflit, export) dans ses écrans Exposant/Visiteur/Organisateur. **Décision P1 (tranchée)** : **JayKoa organise les données et fait l’interface avec l’utilisateur** ; **MiyuClock atteste l’horaire et la date IRL** (référentiel temps réel). Voir [Interpolarité Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) et référence MiyuClock.

---

### 2.2 JayKonta (budget, devis, facturation)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayKonta - Document Fondateur](../../JayKonta/JayKonta%20-%20Document%20Fondateur.md) |
| Publics (Account, Purse) | Oui | Analyse des besoins, Opérateurs et Toolkits, Parcours par public |
| Integration Services | Oui | [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) — flux JayFestival (devis, factures, budget) |
| Points d’entrée | Oui | [JayKonta - Points Entree JayBudget et JayKonta](../../JayKonta/reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) |
| Ecrans et UI | Non | Aucun document « Ecrans et UI » dédié ; l’UI est décrite dans les Parcours (Account, Purse). |

**Prêt pour implémentation complète UI incluse** : Partiel. Les flux et opérateurs sont documentés ; l’UI budget/devis/factures dans JayFestival est couverte par la [Specification UI Conforme Catakana](../JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) (écrans ORG-E12, ORG-E13, EXP-E13, etc.). Pour une UI **JayKonta native** (hors JayFestival), il manque une spec écrans dédiée.

**Décision P0 (tranchée)** : **Miyuinvoice + JayKonta** — facturation exposants = Miyuinvoice en façade avec JayKonta en backend (devis, factures, encaissements).

---

### 2.3 JayXpose (profil exposant, vitrine)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayXpose - Document Fondateur](../../JayXpose/JayXpose%20-%20Document%20Fondateur.md) |
| Opérateurs et Toolkits | Non | Non documenté. |
| Parcours / publics | Non | Non documenté. |
| Ecrans et UI | Non | Non documenté. |
| Contrat d’intégration JayFestival | Non | L’interpolarité est décrite (fiche exposant, répertoire) mais pas le contrat (API, données exposées, champs requis). |

**Prêt pour implémentation complète UI incluse** : Non. Impossible d’implémenter l’intégration « fiche exposant / répertoire » côté JayFestival sans : (1) Opérateurs/Kits JayXpose, (2) Contrat d’intégration (quels champs JayFestival lit, comment lier exposant ↔ vitrine), (3) Optionnel : écrans/UI vitrine si JayFestival affiche un bloc « Vitrine » ou lien vers vitrine.

**Choix humain** : En alpha JayFestival, la fiche exposant peut rester **locale** (données Catakana/JayFestival uniquement) sans JayXpose ; documenter explicitement « JayXpose = phase 2 ou optionnel alpha ».

---

### 2.4 JayFaim (restauration sur événement)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Document fondateur | Oui | [JayFaim - Document Fondateur](../../JayFaim/JayFaim%20-%20Document%20Fondateur.md) |
| Opérateurs et Toolkits | Non | Non documenté. |
| Parcours / Ecrans et UI | Non | Non documenté. |
| Contrat d’intégration JayFestival | Non | Orchestration décrite (créneaux, commandes, paiement) mais pas le contrat (qui appelle qui, données partagées). |

**Prêt pour implémentation complète UI incluse** : Non. JayFaim est marqué **phase 2 ou optionnel** dans le Bornage JayFestival ; pour une implémentation complète ultérieure, il faudra : Opérateurs/Kits JayFaim, Parcours, Ecrans/UI, Contrat d’intégration avec JayFestival (et JayKonta).

**Choix humain** : Confirmer que JayFaim est **hors scope alpha** ; documenter la roadmap (phase 2) pour ne pas bloquer l’alpha.

---

### 2.5 Miyauth (authentification)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuAuth - Documentation Fondatrice](../../../tools/MiyuAuth/MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| Reference Outils | Oui | [MiyuAuth - Reference Outils](../../../tools/MiyuAuth/MiyuAuth%20-%20Reference%20Outils.md) |
| Contrats (governance, security, KindMother, etc.) | Oui | Plusieurs contrats sous `tools/MiyuAuth/contracts/` |
| Implementation Guidelines | Oui | [MiyuAuth - Reference Implementation Guidelines](../../../tools/MiyuAuth/implementation/MiyuAuth%20-%20Reference%20Implementation%20Guidelines.md) |
| Ecrans Connexion/Inscription | — | Portés par **JayFestival** (UNC-E12, UNC-E13, ORG-E02, ORG-E03, etc.) ; Miyauth fournit l’auth, pas l’UI. |

**Prêt pour implémentation complète UI incluse** : Oui pour la **consommation** depuis JayFestival. L’UI (formulaires Connexion, Inscription) est dans le périmètre JayFestival (Specification UI Conforme Catakana) ; Miyauth est appelé en backend. JayFestival a une Auth à lui, dérivée de l’Auth Catakana qui utilise Supabase Auth ; en alpha, cette Auth JayFestival s’appuie sur Supabase Auth (exception pré-COG).

**Ambiguïté** : En alpha, Supabase Auth est utilisé ; la **bascule vers Miyauth** (COG-native) n’est pas datée — à documenter dans la roadmap post-alpha.

---

### 2.6 Miyuprofile (profil utilisateur)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation | _index + références éparses (Document fondateur JayFestival, publics) | Pas de dossier dédié « Miyuprofile » avec Doc fondatrice au même niveau que MiyuAuth. |
| Profil organisateur / exposant / visiteur | Cité dans JayFestival (Bornage, Document fondateur) | Fiche organisateur, fiche exposant, profil visiteur — partie dans JayFestival, partie potentiellement Miyuprofile. |

**Prêt pour implémentation complète UI incluse** : Partiel. Les écrans « Mon compte », « Fiche entreprise » sont décrits dans les Écrans et cycle JayFestival ; la frontière **données profil Miyuprofile vs données locales JayFestival** n’est pas formalisée.

**Choix humain** : (1) Où se trouve la source de vérité du profil (Miyuprofile vs tables JayFestival/Supabase) ? (2) Créer ou non un document fondateur / Opérateurs Miyuprofile et un contrat d’intégration JayFestival ↔ Miyuprofile.

---

### 2.7 Miyunotify (annonces, notifications)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuNotify - Documentation Fondatrice](../../../tools/MiyuNotify/MiyuNotify%20-%20Documentation%20Fondatrice.md) |
| Reference Outils | Oui | [MiyuNotify - Reference Outils](../../../tools/MiyuNotify/MiyuNotify%20-%20Reference%20Outils.md) |
| Contrats, Implementation Guidelines | Oui | Sous `tools/MiyuNotify/` |
| UI dans JayFestival | — | JayFestival **déclenche** les envois (annonces, notifications candidature, etc.) ; pas d’écran « Miyunotify » propre dans JayFestival — les écrans sont « Annonces et notifications » (ORG-E23), « Notifications et préférences » (EXP-E19), etc., avec appels à Miyunotify. |

**Prêt pour implémentation complète UI incluse** : Oui pour la consommation. Les écrans côté JayFestival sont documentés (Specification UI, Écrans et cycle) ; Miyunotify est un outil appelé en backend. Contrat d’intégration (payload, événements) à vérifier dans Miyunotify Reference Outils / contrats.

---

### 2.8 Miyuinvoice (devis, factures)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuInvoice - Documentation Fondatrice](../../../tools/MiyuInvoice/MiyuInvoice%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats, Implementation Guidelines | Oui | Sous `tools/MiyuInvoice/` |
| Couplage JayKonta | Cité (Bornage : « Miyuinvoice / JayKonta ») | Rôle respectif non tranché : JayKonta = service COG budget/facturation ; Miyuinvoice = outil ? façade ? |

**Prêt pour implémentation complète UI incluse** : Oui pour la consommation, sous réserve de clarifier Miyuinvoice vs JayKonta. L’UI devis/factures dans JayFestival (ORG-E12, ORG-E13, EXP-E13) est dans la Specification UI Conforme Catakana.

**Choix humain** : Décision : facturation exposants JayFestival = **JayKonta uniquement**, ou **Miyuinvoice** (qui s’appuie sur JayKonta) ? Si Miyuinvoice est la façade, documenter le flux JayFestival → Miyuinvoice → JayKonta.

---

### 2.9 Miyubooking (réservations, créneaux, billets, pass)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuBooking - Documentation Fondatrice](../../../tools/MiyuBooking/MiyuBooking%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats, Implementation Guidelines | Oui | Sous `tools/MiyuBooking/` |
| Écrans JayFestival (réservations, billets, pass) | Oui (Écrans et cycle Visiteurs, Organisateurs) | VIS-E06 à VIS-E09, ORG-E24 (services visiteur) — parcours décrits. |

**Prêt pour implémentation complète UI incluse** : Partiel. Les parcours et écrans sont dans JayFestival ; le **contrat d’intégration** (quels Kits Miyubooking sont appelés, pour quelles réservations/billets/pass) n’est pas formalisé dans un document unique « JayFestival ↔ Miyubooking ». À préciser : création de créneaux, réservation atelier, émission billet, pass VIP.

**Recommandation** : Rédiger une section « JayFestival ↔ Miyubooking » dans un doc d’intégration (ou dans Interpolarité) : capacités consommées, données échangées, écrans concernés.

---

### 2.10 Miyucms / Miyumedia (documents, médias, actualités)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation | MiyuMedia Doc fondatrice, Reference Outils ; MiyuCMS idem | Sous `tools/MiyuMedia/`, `tools/MiyuCMS/` |
| Usage JayFestival | Documents d’édition (contrats, règlements), galeries (phase 2), actualités (Miyufeeds/Miyucms) | Cité dans Bornage, Document fondateur. |

**Prêt pour implémentation complète UI incluse** : Partiel. Pas de **contrat d’intégration** explicite « JayFestival ↔ Miyucms/Miyumedia » (upload, stockage, lien document ↔ édition, affichage dans écrans Documents ORG-E22, EXP-E12). Pour phase 2 (galeries, actualités), borner le périmètre.

**Choix humain** : En alpha, les documents d’édition peuvent rester en **stockage local** (Supabase Storage ou tables) ; migration vers Miyucms/Miyumedia en phase 2 à documenter.

---

### 2.11 Miyufeeds (flux actualités)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuFeeds - Documentation Fondatrice](../../../tools/MiyuFeeds/MiyuFeeds%20-%20Documentation%20Fondatrice.md) |
| Usage JayFestival | Phase 2 — module Actualités (News) public | Bornage : « Miyucms/Miyufeeds » pour actualités ; annonces organisateur = Miyunotify en alpha. |

**Prêt pour implémentation complète UI incluse** : Partiel (phase 2). Décision : actualités éditoriales public = Miyufeeds en phase 2 ; pas bloquant pour alpha.

---

### 2.12 MiyuClock (horloge, agenda)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Documentation fondatrice | Oui | [MiyuClock - Documentation Fondatrice](../../../tools/MiyuClock/MiyuClock%20-%20Documentation%20Fondatrice.md) |
| Reference Outils, contrats (KindMother, Security, etc.) | Oui | Sous `tools/MiyuClock/` |
| Rôle vs JayKoa | Cité (Document fondateur JayFestival : « MiyuClock, Miyubooking, données d’édition » pour agenda cross-événements) | JayKoa = intégrateur des **dates** (entrées agenda, conflits, vues). MiyuClock = outil **horloge** / temps ? |

**Prêt pour implémentation complète UI incluse** : Oui pour la consommation, si MiyuClock est clairement positionné (fuseaux, référentiel temps). **Ambiguïté** : MiyuClock vs JayKoa — partage des rôles (qui gère quoi) à clarifier dans un document (ex. « Agenda cross-événements : JayKoa + MiyuClock »). Les écrans agenda dans JayFestival s’appuient sur JayKoa (Ecrans et UI) ; MiyuClock peut être utilisé pour fuseaux / affichage temps.

---

### 2.13 Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Contrats COG | Oui | Sous `docs/core/` pour chaque Core. |
| UI | N/A | Pas d’écran « Core » ; intégration dans JayFestival via Mandats, persistance, permissions, niveaux de sécurité. |

**Prêt pour implémentation complète UI incluse** : Oui (côté gouvernance). L’UI JayFestival respecte les décisions des Cores ; pas de doc UI spécifique « Core » nécessaire pour JayFestival.

---

### 2.14 Supabase (backend alpha)

| Élément | Existant | Commentaire |
|---------|----------|-------------|
| Reference Base de Donnees et Migration | Oui | [JayFestival - Reference Base de Donnees et Migration](./JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) — tables, RLS, mapping services. |
| UI | N/A | Backend uniquement ; pas d’écran Supabase. |

**Prêt pour implémentation complète UI incluse** : Oui pour l’alpha (backend documenté). Migration post-alpha vers SQLite + KindMother documentée dans le même doc.

---

## 3. Ambiguïtés et choix humains à trancher

Les points suivants **nécessitent une décision ou un arbitrage humain** pour finaliser la documentation et permettre une implémentation sans ambiguïté.

### 3.1 Facturation exposants : JayKonta vs Miyuinvoice

- **Constat** : Le Bornage et les docs citent « Miyuinvoice / JayKonta » pour devis et factures exposants.
- **Question** : La facturation des exposants dans JayFestival passe-t-elle par **JayKonta uniquement** (opérateurs `quote.create`, `invoice.emit`), ou par **Miyuinvoice** qui lui-même s’appuie sur JayKonta ?
- **Impact** : Documentation des flux, contrats d’intégration, implémentation des écrans ORG-E12, ORG-E13, EXP-E13.
- **Recommandation** : Tracer explicitement dans un document (Interpolarité ou Reference Base de Donnees) : « Facturation exposants JayFestival : [JayKonta seul | Miyuinvoice → JayKonta]. »

### 3.2 JayXpose : scope alpha

- **Constat** : JayXpose n’a que le Document fondateur ; pas d’Opérateurs, Parcours, UI, ni contrat d’intégration.
- **Question** : En **alpha**, la fiche exposant et le répertoire JayFestival utilisent-ils **uniquement les données locales** (Supabase/tables JayFestival), ou doit-on prévoir l’appel à JayXpose dès l’alpha ?
- **Impact** : Si JayXpose hors alpha, documenter « Fiche exposant alpha = données locales ; JayXpose = phase 2 ou optionnel » dans le Bornage et l’Interpolarité. Si JayXpose en alpha, il faut au minimum un contrat d’intégration (champs, API ou façade).
- **Recommandation** : Conserver JayXpose **optionnel / phase 2** pour l’alpha et le formaliser dans le Bornage.

### 3.3 JayFaim : scope et roadmap

- **Constat** : JayFaim n’a que le Document fondateur ; restauration sur événement = phase 2 ou optionnel.
- **Question** : Confirmer que JayFaim est **hors scope alpha** et documenter la **roadmap** (phase 2) pour ne pas bloquer l’alpha.
- **Recommandation** : Ajouter dans le Bornage une ligne explicite « JayFaim : phase 2 ; pas de doc Opérateurs/UI requise pour alpha. »

### 3.4 Miyuprofile : frontière avec JayFestival

- **Constat** : Pas de document fondateur Miyuprofile dédié au même niveau que MiyuAuth ; profils (organisateur, exposant, visiteur) sont décrits dans les écrans JayFestival.
- **Question** : Où est la **source de vérité** du profil (email, nom, structure, etc.) : Miyuprofile ou tables JayFestival/Supabase ? Faut-il un document « JayFestival ↔ Miyuprofile » (contrat d’intégration) ?
- **Impact** : Implémentation des écrans « Mon compte », « Fiche entreprise », synchronisation des données.
- **Recommandation** : Décider si l’alpha utilise des **profiles Supabase** uniquement, ou si Miyuprofile est consommé ; dans les deux cas, documenter la règle dans Reference Base de Donnees ou Bornage.

### 3.5 Agenda : JayKoa vs MiyuClock

- **Constat** : Document fondateur JayFestival cite « MiyuClock, Miyubooking » pour l’agenda cross-événements ; JayKoa est l’intégrateur des dates (Integration Services Consommateurs).
- **Question** : **Partage des rôles** : JayKoa = entrées agenda, conflits, vues agrégées ; MiyuClock = quoi exactement (fuseaux, horloge, référentiel temps) ? Éviter doublon ou flou pour l’implémentation.
- **Recommandation** : Rédiger un paragraphe court dans « JayFestival - Interpolarite Services Jay » ou dans « JayKoa - Integration Services Consommateurs » : « Pour l’agenda cross-événements, JayFestival publie vers JayKoa ; MiyuClock est utilisé pour [fuseaux / affichage temps / …]. »

### 3.6 JayKonta : document « Ecrans et UI »

- **Constat** : JayKonta n’a pas de document « Ecrans et UI » ; l’UI est décrite dans les Parcours (Account, Purse).
- **Question** : Souhaite-t-on un document **JayKonta - Ecrans et UI** (patterns, zones, composants) pour une UI JayKonta native (hors JayFestival) ? Pour JayFestival seul, la Specification UI Conforme Catakana suffit pour les écrans budget/devis/factures.
- **Recommandation** : Pour une **implémentation complète UI incluse** **dans JayFestival** : suffisant. Pour une **future app JayKonta standalone** : créer un doc Ecrans et UI JayKonta.

---

## 4. Actions recommandées (documentation)

| Priorité | Action | Responsable suggéré |
|----------|--------|----------------------|
| **P0** | Tranché : Miyuinvoice + JayKonta. JayXpose dans l'alpha (parcours demande stands, annuaire exposants). JayFaim hors scope alpha. | Product / Tech |
| **P0** | JayXpose = dans l'alpha (Bornage mis à jour). JayFaim = hors scope alpha. | Product |
| **P1** | Rédiger « JayFestival ↔ Miyubooking » : capacités consommées, données, écrans (section dans Interpolarité ou doc dédié). | Tech / Doc |
| **P1** | Tranché : Miyuprofile = Supabase uniquement pour le moment (source de vérité = tables Supabase). | Product / Tech |
| **P1** | Tranché : JayKoa organise données + interface ; MiyuClock atteste horaire/date IRL (Interpolarité et référence MiyuClock mises à jour). | Tech / Doc |
| **P2** | Si roadmap phase 2 : créer pour JayFaim les documents Opérateurs/Kits, Parcours, Contrat d’intégration JayFestival. | Product / Doc |
| **P2** | Si besoin UI JayKonta standalone : créer « JayKonta - Ecrans et UI ». | Product |

---

## 5. Références

| Document | Rôle |
|----------|------|
| [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayFestival avec services Jay. |
| [JayFestival - Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) | Périmètre alpha, phase 2, dépendances. |
| [JayFestival - Specification UI Conforme Catakana](../JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | Écrans et composants UI JayFestival. |
| [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Types d’entrées JayFestival, responsabilités. |
| [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) | Flux budget, devis, factures JayFestival. |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | Principe global interpolarité. |

---

**Document** : JayFestival — État de la documentation des services interfacés  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence — audit et décisions à trancher
