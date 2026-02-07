# Miyukini Conceptual References — Équivalents Suite Bureautique Collaborative Google (Google Workspace)

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités de la **suite bureautique interconnectée et collaborative de Google** (Google Workspace). Il vise à permettre la création d'**Outils**, **Opérateurs** et **Services** Miyukini pour proposer des **services bureautiques et collaboratifs gouvernés** équivalents :

- **Traitement de texte collaboratif** (Documents, édition temps réel, commentaires, suggestions, historique des versions)
- **Feuilles de calcul collaboratives** (données tabulaires, formules, graphiques, partage)
- **Présentations collaboratives** (diapositives, médias, animation, présentation en direct)
- **Stockage et gestion de fichiers** (dossiers, partage, synchronisation, recherche, versioning)
- **Communication** (messagerie professionnelle, chat d'équipe, visioconférence)
- **Planification** (agendas partagés, événements, créneaux, rappels)
- **Formulaires et enquêtes** (création, diffusion, collecte, analyse des réponses)
- **Notes et signets** (notes rapides, listes, étiquettes, partage)
- **Sites d'équipe** (pages web internes, contenu structuré, publication)
- **Fonctions avancées** (export/import, montage vidéo léger, applications sans code, assistance recherche)

Il **s'appuie sur** la documentation conceptuelle existante : [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md).

---

## Fondements conceptuels (alignement documentation existante)

Ce document applique les **définitions canoniques** et **règles** des références listées ci-dessus. Les équivalents suite bureautique collaborative respectent en particulier :

### Outils (Tools) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.
- **Règle :** *« Un Outil fait, mais ne décide jamais. »* Les Tools bureautiques (ex. `tool.doc.create`, `tool.sheet.cell.update`, `tool.drive.file.move`) exécutent des actions ; la décision (autoriser édition, partage, suppression) appartient à **StrongFather**.

### Kits d'Outils (Toolkits) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience et cohérence.
- **Règle :** *« Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants. »* Les Toolkits bureautiques (`toolkit.office.doc`, `toolkit.office.sheet`, `toolkit.office.drive`, etc.) agrègent des Tools existants sans logique métier propre.

### Opérateurs (Operators) — [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)

- **Définition :** Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.
- Les Opérateurs bureautiques (Documents collaboratifs, Feuilles de calcul, Stockage, Visioconférence, Agenda, Formulaires, etc.) sont des **Opérateurs de Domaine** ou **d'Interface** (Strate 7) ; ils n'ont pas d'autorité propre et passent par la gouvernance pour toute action.

### Service vs Opérateur — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- **Service** = capacité perçue par l'utilisateur. **Opérateur** = unité d'exécution gouvernée.
- **Règle :** *« Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs. »* Le service « suite bureautique collaborative » peut être livré par une **Équipe d'Opérateurs** sous **Contrat d'équipe** et **Mandat de Permission**.

### Données et écriture — [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

- **KindMother** : autorité sur toutes les données (documents, feuilles, présentations, fichiers, métadonnées, révisions, commentaires, partages). Toute écriture passe par **WriteIntent** sous autorité KindMother.
- **StrongFather** : décision ALLOW/DENY (création, édition, partage, suppression, accès). N'exécute jamais.

---

## Portée / Scope

**Ce document définit :**

- L'**analyse PR (Product Review)** de Google Workspace : périmètre fonctionnel et capacités transverses.
- La **cartographie** Google Workspace → Outils, Opérateurs, Services Miyukini.
- Les **Kits et Outils** déjà couverts ou à créer pour un service bureautique collaboratif gouverné.
- Les **Opérateurs** (Domaine, Interface) à déployer et les **Cores** impliqués.
- Les **Équipes d'Opérateurs** et **Contrats d'équipe** pour livrer la suite bureautique collaborative.

**Hors scope :**

- L'implémentation technique détaillée (protocoles temps réel, APIs tierces, codecs visio).
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools).
- La stratégie commerciale ou marketing des services bureautiques.

**Statut :** Document de référence normatif — source de vérité pour la conception des équivalents suite bureautique collaborative Miyukini.

---

## 1. Analyse PR — Google Workspace

### 1.1 Périmètre analysé

| Composant Google | Rôle | Capacités principales retenues |
|------------------|------|-------------------------------|
| **Google Docs** | Traitement de texte | Édition riche, collaboration temps réel, commentaires, suggestions, historique versions, modèles, import/export (DOCX, PDF, ODT) |
| **Google Sheets** | Feuilles de calcul | Cellules, formules, graphiques, tableaux croisés, collaboration temps réel, commentaires, historique, import/export (XLSX, CSV) |
| **Google Slides** | Présentations | Diapositives, thèmes, médias, animation, collaboration temps réel, présentation en direct, commentaires, export (PDF, PPTX) |
| **Google Drive** | Stockage et gestion fichiers | Dossiers, fichiers, partage, permissions, recherche, versioning, synchronisation, quota, liens partagés |
| **Gmail** | Messagerie professionnelle | Envoi/réception, dossiers, étiquettes, pièces jointes, recherche, filtres, signatures, calendrier intégré |
| **Google Meet** | Visioconférence | Audio/vidéo, partage d'écran, sous-titres, réactions, sondages, enregistrement, planification depuis Agenda/Chat |
| **Google Chat** | Messagerie d'équipe | Espaces, conversations 1-1 et groupe, tâches, intégration Docs/Sheets, démarrage Meet, bots |
| **Google Agenda** | Agendas partagés | Événements, créneaux, rappels, disponibilités, partage de calendriers, réunions récurrentes, salles/ressources |
| **Google Forms** | Formulaires et enquêtes | Questions (choix, texte, échelle, date), logique conditionnelle, collecte réponses, analyse, export |
| **Google Keep** | Notes numériques | Notes texte, listes, étiquettes, rappels, dessin, partage, intégration Docs |
| **Google Sites** | Sites d'équipe | Pages, sections, médias, publication, thèmes, partage par lien |
| **Google Vids** | Montage vidéo | Montage léger, sous-titres, partage (fonctionnalité récente Workspace) |
| **NotebookLM** | Assistant recherche | Recherche sur corpus, synthèse, citations (IA) |
| **AppSheet** | Applications sans code | Création d'apps à partir de données (Sheets, etc.) |

### 1.2 Synthèse des capacités transverses

Les capacités ci-dessous sont **transverses** à plusieurs composants Google Workspace ; elles constituent le périmètre fonctionnel à couvrir en Miyukini COG.

| Domaine fonctionnel | Capacités | Composants Google typiques |
|--------------------|-----------|----------------------------|
| **Document riche** | Création, édition, révisions, commentaires, suggestions, export (PDF, Office) | Docs, Sheets, Slides |
| **Collaboration temps réel** | Édition simultanée, curseurs, présence, verrous optiques, conflits | Docs, Sheets, Slides |
| **Stockage et hiérarchie** | Dossiers, fichiers, métadonnées, recherche full-text, versioning | Drive |
| **Partage et permissions** | Partager avec utilisateurs/groupes, lien public/restreint, rôles (lecture, commentaire, édition) | Drive, Docs, Sheets, Slides, Sites |
| **Messagerie** | Envoi/réception, conversations, pièces jointes, recherche | Gmail, Chat |
| **Visioconférence** | Audio/vidéo, partage d'écran, sous-titres, réactions, enregistrement | Meet |
| **Planification** | Événements, créneaux, rappels, disponibilités, ressources | Agenda |
| **Formulaires et collecte** | Questions, logique, réponses, analyse, export | Forms |
| **Notes et signets** | Notes, listes, étiquettes, rappels, partage | Keep |
| **Publication web** | Pages, sections, médias, thèmes, partage | Sites |
| **Calcul et formules** | Cellules, formules, graphiques, tableaux croisés | Sheets |
| **Export / import** | Export PDF, Office, CSV ; import depuis fichiers | Docs, Sheets, Slides, Drive, Forms |

---

## 2. Équivalents déjà couverts par le projet Miyukini

Les Kits et Tools suivants **existent déjà** dans [docs/tools/](../tools/_index.md) et couvrent une partie des besoins suite bureautique collaborative :

| Fonctionnalité Google Workspace | Équivalent Miyukini existant | Détail |
|--------------------------------|------------------------------|--------|
| **Traitement de texte (base)** | **MiyuText** | `tool.text.markdown.render`, `tool.text.template.apply`, `tool.text.sanitize`, recherche/remplacement ; pas d'édition riche collaborative native. |
| **Calcul et formules** | **MiyuCalc** | `tool.calc.expression.eval`, `tool.calc.format.number`, conversion unités ; pas de feuille de calcul structurée avec cellules. |
| **Export (PDF, XLSX, CSV)** | **MiyuExport** | Génération CSV, XLSX, PDF ; utilisable pour export de documents et feuilles. |
| **Contenu éditorial, révisions, commentaires** | **MiyuCMS** | `tool.content.create`, `tool.content.update`, `tool.content.publish`, `tool.content.revision.*`, `tool.content.comment.*` ; réutilisable pour documents et pages. |
| **Médias (upload, transformation)** | **MiyuMedia** | `tool.media.upload`, `tool.media.serve`, `tool.media.transform` ; pour pièces jointes et médias dans présentations/sites. |
| **Identité et rôles** | **MiyuAuth** | `tool.identity.resolve`, `tool.identity.attest`, `tool.identity.verify`, `tool.identity.role` ; base pour partage et permissions. |
| **Notifications** | **MiyuNotify** | Envoi, préférences, canaux ; pour rappels, commentaires, invitations. |
| **Recherche** | **MiyuSearch** | Indexation full-text, requête, suggestions ; pour recherche dans documents et Drive. |
| **Horloge** | **MiyuClock** | Instant présent, delta (LOI-4) ; pour planification et versioning. |
| **Réservation / créneaux** | **MiyuBooking** | Créneaux, réservations, ressources ; réutilisable pour Agenda et salles. |
| **Formulaires / sondages** | **MiyuPolls** | Création, vote, résultats ; complémentaire à Forms (sondages vs formulaires longs). |
| **Messagerie privée / équipe** | **MiyuPM** | Envoi, dossiers, brouillons, conversation, export ; base pour Chat d'équipe. |
| **Affichage web, layout, thèmes** | **MiyuWeb**, **MiyuWidgets** | Rendu HTML, layout, thème, formulaires ; pour UI Documents, Sheets, Slides, Sites. |
| **Données et requêtes** | **MiyuSQL** | Requêtes, transactions, cache ; persistance des données bureautiques. |
| **Planification (jobs)** | **MiyuJobs** | Schedule, cron, enqueue ; pour rappels, purge versions, synchronisation. |
| **Profil, contacts** | **MiyuProfile**, **MiyuContacts** | Profil étendu, carnet d'adresses ; pour partage et présence. |
| **Validation** | **MiyuValidate** | Schéma, sanitize ; pour formulaires et champs. |

**Invariant :** Aucun Kit existant ne fournit la **structure dédiée** : document collaboratif temps réel (type Docs), feuille de calcul avec cellules et formules (type Sheets), présentation avec diapositives (type Slides), stockage hiérarchique avec partage et versioning (type Drive), visioconférence (type Meet), agenda partagé avec événements et ressources (type Agenda), formulaires longs avec logique conditionnelle (type Forms), notes rapides avec étiquettes (type Keep), sites d'équipe (type Sites). Ces capacités sont à modéliser en **Outils et Opérateurs bureautiques/collaboratifs** dédiés.

---

## 3. Cartographie Google Workspace → Miyukini COG

### 3.1 Documents collaboratifs (équivalent Google Docs)

| Fonctionnalité Google Docs | Équivalent Miyukini | Type | Détail |
|----------------------------|---------------------|------|--------|
| **Création document** | Tools office.doc | Tools | `tool.office.doc.create` ; titre, type (texte riche), modèle fournis ; persistance = KindMother ; autorisation = StrongFather. |
| **Édition contenu** | Tools office.doc | Tools | `tool.office.doc.content.update` (bloc, delta ou full) ; pas de décision métier dans le Tool. |
| **Collaboration temps réel** | Tools office.collab + flux | Tools + flux | `tool.office.collab.presence.update`, `tool.office.collab.cursor.update`, `tool.office.collab.lock.acquire` (optique) ; état présence/cursor = données KindMother ou flux temps réel gouverné. |
| **Révisions / historique** | MiyuCMS + extension | Tools | Réutilisation `tool.content.revision.list`, `tool.content.revision.restore`, `tool.content.revision.compare` avec type « document office » ou `tool.office.doc.revision.*`. |
| **Commentaires et suggestions** | MiyuCMS + extension | Tools | `tool.content.comment.create/list` ou `tool.office.doc.comment.*`, `tool.office.doc.suggestion.create/apply/reject` ; autorisation = StrongFather. |
| **Partage et permissions** | Tools office.share | Tools | `tool.office.share.add`, `tool.office.share.update`, `tool.office.share.remove`, `tool.office.share.list` ; décision = StrongFather. |
| **Export (DOCX, PDF, ODT)** | MiyuExport + MiyuText | Tools | `tool.export.document` (format fourni) ou extension MiyuExport ; contenu fourni dans le flux. |
| **Import** | Tools office.doc | Tools | `tool.office.doc.import` (format, fichier) ; persistance = KindMother. |

### 3.2 Feuilles de calcul (équivalent Google Sheets)

| Fonctionnalité Google Sheets | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Création feuille** | Tools office.sheet | Tools | `tool.office.sheet.create` ; titre, dimensions optionnelles ; persistance = KindMother. |
| **Lecture/écriture cellules** | Tools office.sheet | Tools | `tool.office.sheet.cell.get`, `tool.office.sheet.cell.update`, `tool.office.sheet.cell.batch.update` ; plage fournie ; pas de décision dans le Tool. |
| **Formules** | MiyuCalc + Tools sheet | Tools | `tool.calc.expression.eval` pour évaluation formule ; `tool.office.sheet.formula.set`, `tool.office.sheet.recalculate` ; dépendances = données KindMother. |
| **Graphiques** | Tools office.sheet | Tools | `tool.office.sheet.chart.create`, `tool.office.sheet.chart.update`, `tool.office.sheet.chart.list` ; données et config fournies. |
| **Collaboration temps réel** | Tools office.collab | Tools | Même principe que Docs : présence, curseurs, verrous par plage optionnels. |
| **Commentaires sur cellule** | Tools office.sheet | Tools | `tool.office.sheet.comment.create`, `tool.office.sheet.comment.list`, `tool.office.sheet.comment.resolve` ; autorisation = StrongFather. |
| **Révisions** | Tools office.sheet | Tools | `tool.office.sheet.revision.list`, `tool.office.sheet.revision.restore` ; persistance = KindMother. |
| **Export (XLSX, CSV)** | MiyuExport | Tools | Déjà couvert ; données feuille fournies dans le flux. |
| **Import** | Tools office.sheet | Tools | `tool.office.sheet.import` (format, fichier) ; persistance = KindMother. |

### 3.3 Présentations (équivalent Google Slides)

| Fonctionnalité Google Slides | Équivalent Miyukini | Type | Détail |
|------------------------------|---------------------|------|--------|
| **Création présentation** | Tools office.slides | Tools | `tool.office.slides.create` ; titre, thème fournis ; persistance = KindMother. |
| **Diapositives** | Tools office.slides | Tools | `tool.office.slides.slide.add`, `tool.office.slides.slide.update`, `tool.office.slides.slide.remove`, `tool.office.slides.slide.reorder` ; contenu (texte, médias, forme) fourni. |
| **Médias et formes** | MiyuMedia + Tools slides | Tools | `tool.media.upload` pour médias ; `tool.office.slides.shape.add`, `tool.office.slides.media.add` ; persistance = KindMother. |
| **Thèmes et mise en page** | Tools office.slides | Tools | `tool.office.slides.theme.apply`, `tool.office.slides.layout.apply` ; données thème = KindMother. |
| **Collaboration temps réel** | Tools office.collab | Tools | Même principe Docs/Sheets. |
| **Commentaires** | Tools office.slides | Tools | `tool.office.slides.comment.create/list/resolve` ; autorisation = StrongFather. |
| **Export (PDF, PPTX)** | MiyuExport | Tools | Extension pour format présentation ; contenu fourni. |
| **Mode présentation** | Données + flux | Flux | État « slide courante », avance/recule = lecture seule ou événements ; pas de Tool métier dédié au « mode présentation » (affichage côté Interface). |

### 3.4 Stockage et gestion de fichiers (équivalent Google Drive)

| Fonctionnalité Google Drive | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Création dossier / fichier** | Tools office.drive | Tools | `tool.office.drive.folder.create`, `tool.office.drive.file.create` (métadonnées + référence contenu ou upload) ; persistance = KindMother. |
| **Arborescence** | Tools office.drive | Tools | `tool.office.drive.tree.get`, `tool.office.drive.node.move`, `tool.office.drive.node.copy`, `tool.office.drive.node.delete` ; décision suppression = StrongFather. |
| **Partage et permissions** | Tools office.share | Tools | `tool.office.share.add/update/remove/list` (cible = fichier ou dossier) ; héritage optionnel ; décision = StrongFather. |
| **Versioning** | Tools office.drive | Tools | `tool.office.drive.version.list`, `tool.office.drive.version.restore`, `tool.office.drive.version.pin` ; persistance = KindMother. |
| **Recherche** | MiyuSearch | Tools | `tool.search.fulltext` (scope = drive) ou `tool.office.drive.search` (filtres type, date, partagé) ; pas de décision dans le Tool. |
| **Lien partagé** | Tools office.drive | Tools | `tool.office.drive.link.create` (visibilité, expiration fournies) ; décision = StrongFather. |
| **Quota** | Politique + Caring Nanny | Core | Politique stockage = StrongFather / Border Guard ; observation usage = Caring Nanny ; pas de Tool dédié « quota » (vérification côté gouvernance). |

### 3.5 Messagerie (équivalent Gmail, Google Chat)

| Fonctionnalité Gmail / Chat | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Envoi / réception email** | MiyuNotify + extension | Tools | Canal email déjà dans MiyuNotify ; `tool.notify.email.send`, `tool.notify.inbox.list` (si inbox documenté) ; ou Opérateur Messagerie email dédié avec Tools email.send/receive/list. |
| **Chat espaces et conversations** | MiyuPM + extension | Tools | MiyuPM : conversations, brouillons ; extension pour « espaces » (groupes nommés, canaux) : `tool.office.chat.space.create`, `tool.office.chat.message.send`, `tool.office.chat.message.list` ; autorisation = StrongFather. |
| **Tâches dans espaces** | Tools office.chat | Tools | `tool.office.chat.task.create`, `tool.office.chat.task.list`, `tool.office.chat.task.update` ; persistance = KindMother. |
| **Pièces jointes** | MiyuMedia + flux | Tools | `tool.media.upload` ; référence attachée au message ; déjà couvert. |
| **Démarrage réunion (Meet)** | Tools office.meet | Tools | `tool.office.meet.session.create` (lié à événement Agenda ou ad-hoc) ; décision = StrongFather. |

### 3.6 Visioconférence (équivalent Google Meet)

| Fonctionnalité Google Meet | Équivalent Miyukini | Type | Détail |
|---------------------------|---------------------|------|--------|
| **Création / rejoindre session** | Tools office.meet | Tools | `tool.office.meet.session.create`, `tool.office.meet.session.join`, `tool.office.meet.session.leave` ; décision = StrongFather ; binding transport (WebRTC ou autre) hors scope COG. |
| **Partage d'écran** | Tools office.meet | Tools | `tool.office.meet.screen.share.start`, `tool.office.meet.screen.share.stop` ; capacité technique, pas de décision métier. |
| **Réactions, sous-titres** | Données + flux | Flux | Réactions = données ou événements ; sous-titres = flux texte (binding). Pas de Tool métier dédié. |
| **Enregistrement** | Tools office.meet | Tools | `tool.office.meet.recording.start`, `tool.office.meet.recording.stop` ; stockage = KindMother (référence fichier Drive ou média). |
| **Sondages / Q&A** | MiyuPolls + intégration | Tools | Réutilisation MiyuPolls dans contexte réunion ; ou `tool.office.meet.poll.create`, `tool.office.meet.poll.vote`. |

### 3.7 Agenda (équivalent Google Agenda)

| Fonctionnalité Google Agenda | Équivalent Miyukini | Type | Détail |
|------------------------------|---------------------|------|--------|
| **Calendriers** | Tools office.calendar | Tools | `tool.office.calendar.create`, `tool.office.calendar.list`, `tool.office.calendar.share` ; persistance = KindMother. |
| **Événements** | MiyuBooking + extension | Tools | MiyuBooking pour créneaux/ressources ; extension pour événements génériques : `tool.office.calendar.event.create`, `tool.office.calendar.event.update`, `tool.office.calendar.event.delete`, `tool.office.calendar.event.list` ; rappels = MiyuNotify ou champ événement. |
| **Disponibilités** | Tools office.calendar | Tools | `tool.office.calendar.availability.get` (plage, calendriers) ; agrégation créneaux libres. |
| **Ressources / salles** | MiyuBooking | Tools | Déjà couvert (ressources, réservation) ; liaison événement ↔ ressource. |
| **Réunions récurrentes** | Tools office.calendar | Tools | `tool.office.calendar.event.create` (rrule ou pattern fourni) ; exécution seule. |

### 3.8 Formulaires (équivalent Google Forms)

| Fonctionnalité Google Forms | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Création formulaire** | Tools office.forms | Tools | `tool.office.forms.create` ; titre, sections fournis ; persistance = KindMother. |
| **Questions** | Tools office.forms | Tools | `tool.office.forms.question.add`, `tool.office.forms.question.update`, types (choix, texte, échelle, date, grille) ; logique conditionnelle = données fournies. |
| **Collecte réponses** | Tools office.forms | Tools | `tool.office.forms.response.submit`, `tool.office.forms.response.list` ; validation = MiyuValidate ; autorisation = StrongFather. |
| **Analyse et export** | MiyuExport + Tools | Tools | `tool.office.forms.response.export` (CSV, XLSX) ou agrégation + MiyuExport ; pas de décision dans le Tool. |

### 3.9 Notes (équivalent Google Keep)

| Fonctionnalité Google Keep | Équivalent Miyukini | Type | Détail |
|----------------------------|---------------------|------|--------|
| **Création note** | Tools office.notes | Tools | `tool.office.notes.create` ; contenu (texte, liste, dessin ref), étiquettes fournis ; persistance = KindMother. |
| **Liste, recherche** | Tools office.notes | Tools | `tool.office.notes.list`, `tool.office.notes.search` (étiquettes, texte) ; pas de décision dans le Tool. |
| **Étiquettes et rappels** | Tools office.notes | Tools | `tool.office.notes.label.add`, `tool.office.notes.reminder.set` ; rappels = MiyuJobs + MiyuNotify. |
| **Partage** | Tools office.share | Tools | `tool.office.share.add/remove/list` (cible = note). |

### 3.10 Sites d'équipe (équivalent Google Sites)

| Fonctionnalité Google Sites | Équivalent Miyukini | Type | Détail |
|-----------------------------|---------------------|------|--------|
| **Création site** | MiyuCMS + extension | Tools | Site = conteneur de pages ; `tool.content.create` (type site) ou `tool.office.sites.create` ; persistance = KindMother. |
| **Pages et sections** | MiyuWeb + MiyuCMS | Tools | Pages = contenu ; sections = blocs layout ; `tool.web.layout.render`, `tool.content.*` ; thèmes = MiyuWeb. |
| **Publication et partage** | Tools office.share | Tools | `tool.office.share.add/list` (cible = site ou page) ; visibilité publique/restreinte = StrongFather. |

### 3.11 Export / import et fonctions avancées

| Fonctionnalité | Équivalent Miyukini | Type | Détail |
|----------------|---------------------|------|--------|
| **Export multi-format** | MiyuExport | Tools | Déjà couvert ; extension par type (doc, sheet, slides) avec paramètres format. |
| **Montage vidéo (Vids)** | Tools office.media | Tools | `tool.office.video.sequence.create`, `tool.office.video.clip.add`, `tool.office.video.export` ; capacités légères ; médias = MiyuMedia. |
| **Assistant recherche (NotebookLM)** | MiyuSearch + politique | Tools + Core | Recherche sur corpus = MiyuSearch ; synthèse/IA = politique (TAMR, pas de décision automatique dans le Tool). |
| **Applications sans code (AppSheet)** | Opérateur ou Équipe | Opérateur | Création d'apps à partir de données = Opérateur de Domaine « Apps low-code » utilisant MiyuWeb, MiyuSQL, formulaires ; hors détail dans ce document. |

---

## 4. Synthèse — Outils et Kits à créer ou étendre

### 4.1 Kits d'outils (Toolkits) proposés

| ToolkitId proposé | Domaine | Composition (résumé) | Usage principal |
|-------------------|---------|----------------------|------------------|
| `toolkit.office.doc` | office | tool.office.doc.*, tool.office.collab.*, tool.content.revision.*, tool.content.comment.*, tool.office.share.* | Documents collaboratifs (type Docs) |
| `toolkit.office.sheet` | office | tool.office.sheet.*, tool.office.collab.*, tool.calc.*, tool.office.share.* | Feuilles de calcul (type Sheets) |
| `toolkit.office.slides` | office | tool.office.slides.*, tool.media.*, tool.office.collab.*, tool.office.share.* | Présentations (type Slides) |
| `toolkit.office.drive` | office | tool.office.drive.*, tool.office.share.*, tool.search.* (scope drive) | Stockage et fichiers (type Drive) |
| `toolkit.office.collab` | office | tool.office.collab.presence.*, tool.office.collab.cursor.*, tool.office.collab.lock.* | Collaboration temps réel (partagé Docs/Sheets/Slides) |
| `toolkit.office.chat` | office | tool.office.chat.space.*, tool.office.chat.message.*, tool.office.chat.task.* (ou MiyuPM étendu) | Chat d'équipe (type Chat) |
| `toolkit.office.meet` | office | tool.office.meet.session.*, tool.office.meet.screen.*, tool.office.meet.recording.*, tool.office.meet.poll.* | Visioconférence (type Meet) |
| `toolkit.office.calendar` | office | tool.office.calendar.*, MiyuBooking (ressources) | Agenda partagé (type Agenda) |
| `toolkit.office.forms` | office | tool.office.forms.*, MiyuValidate | Formulaires et enquêtes (type Forms) |
| `toolkit.office.notes` | office | tool.office.notes.*, tool.office.share.* | Notes (type Keep) |
| `toolkit.office.share` | office | tool.office.share.add, tool.office.share.update, tool.office.share.remove, tool.office.share.list | Partage et permissions (transverse Drive, Docs, Sheets, Slides, Notes, Sites) |

**Invariant :** Chaque Toolkit contient au moins deux Tools. Les Toolkits sont validés par Ever Buddy et déclarés au Master Butler.

### 4.2 Outils (Tools) proposés — liste canonique orientée

#### 4.2.1 Documents (doc)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.doc.create` | Crée un document (titre, type, modèle) |
| `tool.office.doc.content.update` | Met à jour le contenu (bloc/delta/full) |
| `tool.office.doc.revision.list` | Liste les révisions |
| `tool.office.doc.revision.restore` | Restaure une révision |
| `tool.office.doc.comment.create` | Crée un commentaire |
| `tool.office.doc.comment.list` | Liste les commentaires |
| `tool.office.doc.suggestion.create` | Crée une suggestion |
| `tool.office.doc.suggestion.apply` | Applique une suggestion |
| `tool.office.doc.suggestion.reject` | Rejette une suggestion |
| `tool.office.doc.import` | Importe un fichier (DOCX, ODT, etc.) |
| `tool.office.doc.resolve` | Retourne un document par identifiant |

#### 4.2.2 Feuilles de calcul (sheet)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.sheet.create` | Crée une feuille |
| `tool.office.sheet.cell.get` | Lit cellule(s) |
| `tool.office.sheet.cell.update` | Met à jour cellule(s) |
| `tool.office.sheet.cell.batch.update` | Met à jour une plage |
| `tool.office.sheet.formula.set` | Définit une formule |
| `tool.office.sheet.recalculate` | Recalcule les formules |
| `tool.office.sheet.chart.create` | Crée un graphique |
| `tool.office.sheet.chart.update` | Met à jour un graphique |
| `tool.office.sheet.chart.list` | Liste les graphiques |
| `tool.office.sheet.comment.create/list/resolve` | Commentaires sur cellule |
| `tool.office.sheet.revision.list/restore` | Révisions |
| `tool.office.sheet.import` | Importe (XLSX, CSV) |
| `tool.office.sheet.resolve` | Retourne une feuille par identifiant |

#### 4.2.3 Présentations (slides)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.slides.create` | Crée une présentation |
| `tool.office.slides.slide.add/update/remove/reorder` | Gestion diapositives |
| `tool.office.slides.shape.add` | Ajoute une forme |
| `tool.office.slides.media.add` | Ajoute un média |
| `tool.office.slides.theme.apply` | Applique un thème |
| `tool.office.slides.layout.apply` | Applique une mise en page |
| `tool.office.slides.comment.create/list/resolve` | Commentaires |
| `tool.office.slides.resolve` | Retourne une présentation par identifiant |

#### 4.2.4 Stockage (drive)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.drive.folder.create` | Crée un dossier |
| `tool.office.drive.file.create` | Crée un fichier (métadonnées + contenu ref) |
| `tool.office.drive.tree.get` | Retourne l'arborescence (nœud ou racine) |
| `tool.office.drive.node.move` | Déplace un nœud |
| `tool.office.drive.node.copy` | Copie un nœud |
| `tool.office.drive.node.delete` | Supprime un nœud |
| `tool.office.drive.version.list` | Liste les versions |
| `tool.office.drive.version.restore` | Restaure une version |
| `tool.office.drive.version.pin` | Épingle une version |
| `tool.office.drive.link.create` | Crée un lien partagé |
| `tool.office.drive.search` | Recherche (filtres fournis) |
| `tool.office.drive.node.resolve` | Retourne un nœud par identifiant |

#### 4.2.5 Collaboration temps réel (collab)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.collab.presence.update` | Met à jour la présence (utilisateur, document, position) |
| `tool.office.collab.presence.list` | Liste les présences actives |
| `tool.office.collab.cursor.update` | Met à jour le curseur (utilisateur, document, position) |
| `tool.office.collab.cursor.list` | Liste les curseurs |
| `tool.office.collab.lock.acquire` | Acquiert un verrou optique (plage ou bloc) |
| `tool.office.collab.lock.release` | Libère un verrou |

#### 4.2.6 Partage (share) — transverse

| ToolId | Action courte |
|--------|----------------|
| `tool.office.share.add` | Ajoute un partage (cible, destinataire, rôle) |
| `tool.office.share.update` | Met à jour un partage (rôle, expiration) |
| `tool.office.share.remove` | Supprime un partage |
| `tool.office.share.list` | Liste les partages d'une cible |

#### 4.2.7 Chat d'équipe (chat)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.chat.space.create` | Crée un espace (canal) |
| `tool.office.chat.space.list` | Liste les espaces |
| `tool.office.chat.message.send` | Envoie un message |
| `tool.office.chat.message.list` | Liste les messages |
| `tool.office.chat.task.create` | Crée une tâche |
| `tool.office.chat.task.list` | Liste les tâches |
| `tool.office.chat.task.update` | Met à jour une tâche |

#### 4.2.8 Visioconférence (meet)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.meet.session.create` | Crée une session |
| `tool.office.meet.session.join` | Rejoint une session |
| `tool.office.meet.session.leave` | Quitte une session |
| `tool.office.meet.screen.share.start` | Démarre le partage d'écran |
| `tool.office.meet.screen.share.stop` | Arrête le partage d'écran |
| `tool.office.meet.recording.start` | Démarre l'enregistrement |
| `tool.office.meet.recording.stop` | Arrête l'enregistrement |
| `tool.office.meet.poll.create` | Crée un sondage (réunion) |
| `tool.office.meet.poll.vote` | Vote à un sondage |

#### 4.2.9 Agenda (calendar)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.calendar.create` | Crée un calendrier |
| `tool.office.calendar.list` | Liste les calendriers |
| `tool.office.calendar.share` | Partage un calendrier |
| `tool.office.calendar.event.create` | Crée un événement |
| `tool.office.calendar.event.update` | Met à jour un événement |
| `tool.office.calendar.event.delete` | Supprime un événement |
| `tool.office.calendar.event.list` | Liste les événements (plage, calendriers) |
| `tool.office.calendar.availability.get` | Retourne les disponibilités (créneaux libres) |

#### 4.2.10 Formulaires (forms)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.forms.create` | Crée un formulaire |
| `tool.office.forms.question.add` | Ajoute une question |
| `tool.office.forms.question.update` | Met à jour une question |
| `tool.office.forms.response.submit` | Soumet une réponse |
| `tool.office.forms.response.list` | Liste les réponses |
| `tool.office.forms.response.export` | Exporte les réponses (CSV, XLSX) |
| `tool.office.forms.resolve` | Retourne un formulaire par identifiant |

#### 4.2.11 Notes (notes)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.notes.create` | Crée une note |
| `tool.office.notes.update` | Met à jour une note |
| `tool.office.notes.list` | Liste les notes (filtres) |
| `tool.office.notes.search` | Recherche (étiquettes, texte) |
| `tool.office.notes.label.add` | Ajoute une étiquette |
| `tool.office.notes.reminder.set` | Définit un rappel |
| `tool.office.notes.resolve` | Retourne une note par identifiant |

#### 4.2.12 Sites (sites)

| ToolId | Action courte |
|--------|----------------|
| `tool.office.sites.create` | Crée un site |
| `tool.office.sites.page.add` | Ajoute une page |
| `tool.office.sites.page.update` | Met à jour une page |
| `tool.office.sites.resolve` | Retourne un site par identifiant |

(Le détail des pages/sections peut réutiliser MiyuCMS et MiyuWeb ; les Tools ci-dessus définissent le contrat « site d'équipe ».)

---

## 5. Opérateurs à déployer

Les **Opérateurs** suivants exécutent les rôles nécessaires pour délivrer les services équivalents à Google Workspace. Chacun utilise les Tools et Kits listés ci-dessus et s'appuie sur les Cores (StrongFather, KindMother, Master Butler, BondingBrother, WorrySentinel, Caring Nanny, Ever Buddy).

| Opérateur proposé | Type | Rôle | Tools / Kits principaux | Service perçu |
|------------------|------|------|-------------------------|---------------|
| **Opérateur Documents collaboratifs** | Domaine | Gère la création, l'édition collaborative, les révisions et le partage de documents riches | toolkit.office.doc, toolkit.office.collab, toolkit.office.share, MiyuText, MiyuExport | « Documents type Google Docs » |
| **Opérateur Feuilles de calcul** | Domaine | Gère les feuilles, cellules, formules, graphiques et collaboration | toolkit.office.sheet, toolkit.office.collab, toolkit.office.share, MiyuCalc, MiyuExport | « Feuilles type Google Sheets » |
| **Opérateur Présentations** | Domaine | Gère les présentations, diapositives, médias et collaboration | toolkit.office.slides, toolkit.office.collab, toolkit.office.share, MiyuMedia, MiyuExport | « Présentations type Google Slides » |
| **Opérateur Stockage et fichiers** | Domaine | Gère dossiers, fichiers, versioning, partage et recherche | toolkit.office.drive, toolkit.office.share, MiyuSearch, MiyuMedia | « Stockage type Google Drive » |
| **Opérateur Messagerie professionnelle** | Domaine | Gère envoi/réception email, dossiers, pièces jointes | MiyuNotify (email), MiyuPM (optionnel) | « Messagerie type Gmail » |
| **Opérateur Chat d'équipe** | Domaine | Gère espaces, conversations, tâches, intégration Docs/Sheets | toolkit.office.chat, MiyuPM (extension) | « Chat type Google Chat » |
| **Opérateur Visioconférence** | Domaine | Gère sessions, partage d'écran, enregistrement, sondages | toolkit.office.meet, MiyuPolls | « Visio type Google Meet » |
| **Opérateur Agenda** | Domaine | Gère calendriers, événements, disponibilités, ressources | toolkit.office.calendar, MiyuBooking, MiyuNotify | « Agenda type Google Agenda » |
| **Opérateur Formulaires** | Domaine | Gère création formulaires, collecte et analyse des réponses | toolkit.office.forms, MiyuValidate, MiyuExport | « Formulaires type Google Forms » |
| **Opérateur Notes** | Domaine | Gère notes, listes, étiquettes, rappels, partage | toolkit.office.notes, toolkit.office.share, MiyuJobs, MiyuNotify | « Notes type Google Keep » |
| **Opérateur Sites d'équipe** | Domaine | Gère sites, pages, sections, publication | toolkit.office.sites, MiyuCMS, MiyuWeb, MiyuMedia | « Sites type Google Sites » |
| **Opérateur Interface Suite bureautique** | Interface | Expose l'ensemble des services (Documents, Sheets, Slides, Drive, Chat, Meet, Agenda, Forms, Notes, Sites) de façon utilisable | Tous les Toolkits office + MiyuWeb, MiyuWidgets | « Suite bureautique collaborative » (écran unique ou hub) |

---

## 6. Équipe d'Opérateurs et Service « Suite bureautique collaborative »

Le **Service** perçu par l'utilisateur — « suite bureautique interconnectée et collaborative » — peut être livré par une **Équipe d'Opérateurs** regroupant les Opérateurs listés en section 5, sous un **Contrat d'équipe** et un **Mandat de Permission** émis par StrongFather.

**Contrat d'équipe (orientation) :**

- **Opérateurs membres :** Documents collaboratifs, Feuilles de calcul, Présentations, Stockage et fichiers, Messagerie, Chat d'équipe, Visioconférence, Agenda, Formulaires, Notes, Sites d'équipe, Interface Suite bureautique.
- **Flux autorisés :** Les Opérateurs de Domaine fournissent données et capacités à l'Opérateur Interface ; Chat et Meet peuvent déclencher ou être liés à Agenda ; Drive fournit stockage pour pièces jointes et enregistrements ; partage (tool.office.share) est transverse.
- **Types d'échanges :** Données document, feuille, présentation, fichier, événement, message, réponse formulaire, note ; métadonnées partage et permissions.
- **Niveau de validation :** Conforme aux Security Levels (WorrySentinel) et aux politiques Border Guard ; écritures via WriteIntent vers KindMother ; décisions ALLOW/DENY par StrongFather.

**Règle clé :** Le contrat est validé UNE FOIS, pas à chaque appel. Les Mandats de Permission encadrent les sessions utilisateur et la collaboration entre Opérateurs.

---

## 7. Cores impliqués

| Core | Rôle dans la suite bureautique collaborative |
|------|----------------------------------------------|
| **StrongFather** | Décision ALLOW/DENY : création, édition, partage, suppression, accès aux documents, fichiers, événements, réunions ; émission des Mandats pour l'Équipe d'Opérateurs. |
| **KindMother** | Autorité sur toutes les données : documents, feuilles, présentations, fichiers, métadonnées, révisions, commentaires, partages, événements, réponses formulaires, notes ; toute écriture via WriteIntent. |
| **Master Butler** | Déclare les capacités (Tools et Toolkits office) ; définit les permissions d'accès aux Outils ; catalogue des Opérateurs. |
| **BondingBrother** | Traduction des intentions utilisateur en demandes vers les Opérateurs et Outils ; médiation entre Opérateurs (ex. Chat → Meet, Agenda → Meet). |
| **WorrySentinel** | Niveau de sécurité et état de confiance ; peut bloquer ou dégrader l'accès aux Tools si l'environnement est dégradé ; audit partage et accès. |
| **Caring Nanny** | Observation de l'état du système ; cohérence d'état (ex. quota stockage, santé des sessions Meet). |
| **Ever Buddy** | Cycle de vie des structures (révisions, dépréciation, compatibilité) ; versioning des documents et fichiers. |
| **Border Guard** | Règles de frontière (ex. partage externe, lien public) ; politique des accès inter-COG si applicable. |
| **TAMR** | Points d'intervention humaine (modération, validation manuelle, support). |

---

## 8. Références croisées

- [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) — Outil, Opérateur, Kit d'Outils, Service, WriteIntent, Cores
- [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)
- [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md)
- [Miyukini Conceptual References - Equivalents Reseaux Sociaux](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md)
- [docs/tools/ — Index de navigation](../tools/_index.md)

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — équivalents suite bureautique collaborative Google (Google Workspace).
