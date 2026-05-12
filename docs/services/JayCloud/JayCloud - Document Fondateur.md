# JayCloud — Document fondateur

## Contexte

**JayCloud** est le **service Miyukini unifié du domaine cloud personnel** au sein
de l'écosystème COG. Il prend le relais de **MiyuCloud** (cloud privé chiffré
limité aux fichiers) et **élargit** son périmètre pour offrir l'équivalent
souverain et chiffré d'une suite cloud personnelle complète à la **Nextcloud**.

**Un seul service COG**, **plusieurs points d'entrée** standardisés :

| Point d'entrée | Protocole | Périmètre |
|----------------|-----------|-----------|
| **Drive web**   | HTTPS portail authentifié | Fichiers, dossiers, partage, versionning, corbeille. |
| **WebDAV**      | RFC 4918 | Montage natif depuis Windows / macOS / Linux ; clients tiers (Cyberduck, RaiDrive, etc.). |
| **CalDAV**      | RFC 4791 | Synchronisation agenda avec Thunderbird, Apple Calendar, DAVx⁵ Android. |
| **CardDAV**     | RFC 6352 | Synchronisation contacts avec carnets natifs OS et clients DAV. |
| **Webmail**     | HTTPS portail authentifié | Lecture, écriture, recherche d'emails de **JayMail** depuis n'importe quel navigateur. |

Les cinq points d'entrée s'appuient sur les **mêmes Opérateurs et Kits**
souverains du COG ; seul le protocole d'exposition diffère selon le client.

Ce document est le **document fondateur** du service : il en fixe la raison
d'être, les besoins stratégiques, le positionnement (notamment vs Nextcloud
et vs les services Miyukini existants), l'intégration avec les services COG
en place et les niveaux de sécurité associés à la sensibilité des données.
Il s'adresse aux équipes produit, technique, sécurité et aux parties
prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayCloud — besoins, positionnement
  stratégique, protocoles d'exposition (WebDAV, CalDAV, CardDAV, Webmail,
  Drive web), intégration multi-services (MiyuCloud → migration, JayKoa,
  JayContact, JayMail, JayDocs/Sheets/Slides), niveaux de sécurité et
  modèle de souveraineté.
- **Hors périmètre** : Spécifications techniques détaillées (API, schémas,
  contrats de bytes), implémentation des crates (référencés dans d'autres
  documents — *JayCloud - Spec MSCM MIP* et suivants).
- **Références** : Glossaire Miyukini, Document fondateur **MiyuCloud** (à
  archiver), Documents fondateurs **JayKoa / JayContact / JayMail / Jay
  Bureau**, RFC 4918 / 4791 / 6352, [Politique de résidence des données
  sensibles](../../miyukini-webway-system/reference/_index.md).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au Protocole d'écriture de la documentation conceptuelle
Miyukini :

| Élément | Description |
|---------|-------------|
| **Documentation autorisée (liste fermée)** | Glossaire Miyukini ; Politique de résidence des données sensibles ; Document fondateur MiyuCloud (référencé pour archivage) ; Documents fondateurs JayKoa / JayContact / JayMail / Jay Bureau ; RFC WebDAV / CalDAV / CardDAV. |
| **Ce document ne fusionne pas** | Avec les documents de spec technique (qui détailleront les API, schémas, contrats d'intégration) — ils restent distincts. |
| **Ce document n'anticipe pas** | Les spécifications d'Opérateurs / Kits ni l'implémentation. Les choix de bibliothèques Rust seront tranchés en Spec. |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Ne pas anticiper** | Les étapes suivantes (Spec MSCM/MIP, contrats d'API, implémentation) ne sont pas rédigées dans ce document. |
| ❌ **Ne pas fusionner** | Ce document ne fusionne pas avec les documents de spec ni avec les Docs Fondateurs des services consommateurs/consommés. |
| ❌ **Ne pas corriger hors périmètre** | Toute modification des services consommateurs (JayKoa, JayContact, JayMail) ou du Glossaire reste hors périmètre. |
| ✅ **Source de vérité** | Ce document est la **référence** pour la raison d'être, le positionnement et les règles de sécurité du service JayCloud. |

### Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | JayCloud **remplace** MiyuCloud (retrait après migration). | MiyuCloud était limité au stockage de fichiers ; JayCloud généralise sous une marque cohérente avec la famille Jay et étend aux protocoles standards DAV. |
| **DS-02** | Scope P0 = **Files + CalDAV + CardDAV + Webmail**. Pas d'office collaboratif (déjà couvert par Jay Bureau). | Cible Nextcloud sur les 4 piliers les plus consommés ; évite la duplication avec Jay Docs/Sheets/Slides. |
| **DS-03** | JayCloud est un **proxy souverain** des données existantes, pas un nouveau silo. | Les fichiers restent gérés par les Opérateurs cloud existants ; CalDAV expose JayKoa ; CardDAV expose JayContact ; Webmail expose JayMail. JayCloud orchestre, ne duplique pas. |
| **DS-04** | **Données souveraines au minimum niveau 2 (Sensitive)** ; niveau 3 pour les pièces jointes mail et fichiers tagués `sensible`. | Cohérent avec MiyuCloud (chiffrement libSQL au repos + KindMother pour les clés). |
| **DS-05** | **Portail web authentifié** = un seul portail JayCloud, qui rend l'UI Drive + Webmail + (vues read-only Agenda/Contacts) ; les clients DAV restent les apps natives (Thunderbird, etc.). | Évite de reconstruire des clients lourds que les utilisateurs ont déjà ; concentre l'UI sur ce qui n'existe que côté web. |
| **DS-06** | Authentification : **identité COG** (KindMother) + jetons applicatifs révocables par client DAV. | Pas de mot de passe maître séparé ; chaque appareil/client reçoit un app-password révocable, comme Nextcloud. |
| **Dépendance critique** | Le Document fondateur MiyuCloud doit être archivé en `docs/services/DEPRECATED.md` une fois la migration livrée ; les contrats d'intégration avec JayKoa / JayContact / JayMail seront formalisés dans *JayCloud - Integration Services*. | — |

---

## 1. Besoins stratégiques

### 1.1 Origine du besoin

Quatre constats ont fait émerger la nécessité d'unifier un service cloud
souverain pour le COG :

1. **MiyuCloud est utile mais étroit** : il fournit stockage chiffré et
   portail web pour les fichiers, mais ne couvre ni l'agenda, ni les
   contacts, ni la messagerie depuis l'extérieur du COG.
2. **L'utilisateur a déjà ses outils** : il utilise Thunderbird, Apple
   Calendar, DAVx⁵, des clients WebDAV. Le COG doit s'**intégrer** à
   ces outils, pas les remplacer.
3. **Le périmètre Nextcloud-like est une attente marché** : les
   utilisateurs cherchant la souveraineté numérique connaissent
   Nextcloud comme référence. JayCloud doit pouvoir se présenter comme
   « Nextcloud, mais en Rust, sans serveur Apache, sans PHP, sans cron,
   intégré nativement au COG ».
4. **Les services Miyukini existants ont déjà la donnée** : JayKoa a
   l'agenda, JayContact a les contacts, JayMail a les emails, MiyuCloud
   a les fichiers. **Il manque uniquement la couche d'exposition
   standardisée et le portail unifié**. JayCloud comble exactement
   cette couche.

### 1.2 Besoins fonctionnels identifiés

| Besoin | Description | Protocole / Service consommé |
|--------|-------------|------------------------------|
| **Drive web** | Navigation, upload, download, partage, recherche de fichiers depuis un navigateur. | HTTPS portail ; consomme l'Opérateur Files (héritier de MiyuCloud). |
| **WebDAV** | Montage natif du Drive comme lecteur réseau sur Windows / macOS / Linux. | RFC 4918 ; mappe sur le même Opérateur Files. |
| **Partage de liens** | Liens publics ou semi-privés (mot de passe, expiration) vers un fichier / dossier. | HTTPS portail ; signature des liens via KindMother. |
| **Sync agenda externe** | Synchronisation bidirectionnelle des calendriers JayKoa avec Thunderbird, Apple Calendar, Android. | RFC 4791 (CalDAV) ; wrap JayKoa. |
| **Sync contacts externe** | Synchronisation bidirectionnelle du carnet JayContact avec les contacts OS et clients DAV. | RFC 6352 (CardDAV) ; wrap JayContact. |
| **Webmail** | Lecture, écriture, recherche d'emails JayMail depuis un navigateur. | HTTPS portail ; consomme l'Opérateur Mail (JayMail backend). |
| **Authentification unifiée** | Identité COG (KindMother) + jetons applicatifs révocables par appareil / client DAV. | OAuth-like interne au COG. |
| **Activité / notifications** | Flux d'activité unifié (fichier modifié, événement créé, mail reçu) consommable par Miou / Alicia. | Bus interne COG ; intégration MiyukiniNotify. |

### 1.3 Besoin stratégique de fond

JayCloud répond à un besoin stratégique structurant : **rendre le COG
visible depuis l'écosystème logiciel existant de l'utilisateur**, sans
sacrifier la souveraineté ni introduire de dépendance cloud externe.

Sans JayCloud :
- Les utilisateurs doivent installer Central pour accéder à leur agenda
  / contacts depuis un autre appareil.
- L'intégration aux clients DAV de l'OS est impossible → friction quotidienne.
- Le COG reste un **îlot fonctionnel**, non un **hub** des outils
  numériques de la personne.

Avec JayCloud :
- Le COG **émet des protocoles standards** ; tout client DAV, WebDAV,
  IMAP du marché peut s'y connecter.
- L'utilisateur garde **ses outils habituels** ; le COG devient le serveur
  invisible derrière.
- La promesse « Nextcloud souverain en Rust » devient tangible et démontrable.

---

## 2. Positionnement

### 2.1 Vis-à-vis de Nextcloud

| Axe | Nextcloud | JayCloud |
|-----|-----------|----------|
| **Stack** | PHP + Apache/Nginx + MariaDB/PostgreSQL + Redis + cron | Rust monolithique async (Tokio + Axum + libSQL chiffré). |
| **Déploiement** | Manuel (LAMP) ou Docker / snap | Inclus dans Central. Lancement géré par MasterButler. |
| **Maintenance** | Mises à jour manuelles, occ commands, jobs cron | Auto-géré par le COG, mises à jour signées via le Service Market. |
| **Chiffrement** | Optionnel, complexe à configurer | Par défaut, dérivé de l'identité KindMother. |
| **Modularité** | Apps Nextcloud (PHP) | Services Miyukini (Rust) consommés via Opérateurs / Kits. |
| **Identité** | Comptes Nextcloud + LDAP | Identité COG souveraine, jetons applicatifs révocables. |

**Promesse** : la même surface fonctionnelle (Files + Calendar + Contacts
+ Mail) mais en consommation mémoire et CPU divisée par un ordre de
grandeur, sans dépendances système, et sans aucun composant cloud
externe imposé.

### 2.2 Vis-à-vis des autres services Miyukini

JayCloud n'introduit **aucune nouvelle source de vérité**. Il consomme :

| Service consommé | Données exposées via JayCloud | Protocole d'exposition |
|------------------|-------------------------------|------------------------|
| **MiyuCloud → Files (intégré)** | Arborescence de fichiers, versionning, corbeille. | Drive web + WebDAV. |
| **JayKoa** | Calendriers, événements, récurrences. | CalDAV + Drive web (vue agenda read-only). |
| **JayContact** | Friends, foes, carnet d'adresses. | CardDAV + Drive web (vue contacts read-only). |
| **JayMail** | Boîtes mail, messages, recherche. | Webmail (HTTPS portail). |
| **MiyukiniNotify** | Flux d'activité du COG. | Bandeau de notifications du portail. |
| **KindMother** | Identité, clés, jetons applicatifs. | OAuth-like interne (jetons révocables). |

Les services consommés restent **autonomes** ; JayCloud est une **façade**.
Si JayCloud est désinstallé, JayKoa / JayContact / JayMail / l'Opérateur
Files continuent de fonctionner. L'inverse n'est pas vrai : JayCloud
sans ces services n'a rien à exposer.

### 2.3 Vis-à-vis de Central

Central reste **le hub natif** (UI Dioxus desktop / mobile). JayCloud est
**le hub web** (accessible depuis n'importe quel navigateur). Les deux
sont complémentaires :

- Central = chez soi / au travail / sur mobile avec installation préalable.
- JayCloud portail = depuis un cybercafé, un PC prêté, un appareil
  étranger sans installation.

---

## 3. Architecture cible (vue conceptuelle)

> *La spécification technique détaillée est hors périmètre du document
> fondateur — référencée dans le futur document* JayCloud - Spec MSCM MIP.

```
┌────────────────────────────────────────────────────────────────────────┐
│                          JayCloud (service COG)                        │
│                                                                        │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐            │
│  │ Drive web UI   │  │ Webmail UI     │  │ Portail conn.  │            │
│  │ (HTML+HTMX)    │  │ (HTML+HTMX)    │  │ Identité COG   │            │
│  └────────┬───────┘  └────────┬───────┘  └────────┬───────┘            │
│           │                   │                   │                    │
│  ┌────────┴───────────────────┴───────────────────┴───────┐            │
│  │            Axum HTTPS portail (auth COG)                │            │
│  └────────┬─────────┬─────────┬──────────┬─────────────────┘            │
│           │         │         │          │                              │
│  ┌────────┴───┐ ┌───┴────┐ ┌──┴──────┐ ┌─┴────────┐                     │
│  │ WebDAV     │ │ CalDAV │ │ CardDAV │ │ Webmail  │  Protocoles         │
│  │ (RFC 4918) │ │ 4791   │ │ 6352    │ │ HTTPS    │  exposés            │
│  └────────┬───┘ └───┬────┘ └────┬────┘ └────┬─────┘                     │
└───────────┼─────────┼───────────┼───────────┼───────────────────────────┘
            ↓         ↓           ↓           ↓
       ┌────────┐ ┌────────┐ ┌──────────┐ ┌────────┐
       │ Files  │ │ JayKoa │ │ JayContact│ │JayMail │   Services consommés
       │ Op.    │ │        │ │          │ │        │   (Opérateurs COG)
       └────────┘ └────────┘ └──────────┘ └────────┘
            ↓
       ┌─────────────────────────────────────────────┐
       │ KindMother (identité, clés, libSQL chiffré) │
       └─────────────────────────────────────────────┘
```

### 3.1 Couches

| Couche | Rôle |
|--------|------|
| **Portail** | Authentification COG, sessions, UI Drive et Webmail. |
| **Protocoles** | Adaptateurs WebDAV / CalDAV / CardDAV vers les Opérateurs internes. |
| **Opérateurs consommés** | Files (ex-MiyuCloud), JayKoa, JayContact, JayMail. |
| **Sécurité** | KindMother pour l'identité, libSQL chiffré pour les pièces sensibles, jetons applicatifs révocables. |

### 3.2 Dépendances

| Dépendance | Type | Justification |
|------------|------|---------------|
| **kindmother** | obligatoire | Identité COG et dérivation des clés de chiffrement. |
| **jaykoa** | optionnelle (active CalDAV) | Sans JayKoa, JayCloud désactive proprement CalDAV. |
| **jaycontact** | optionnelle (active CardDAV) | Idem CardDAV. |
| **jaymail** | optionnelle (active Webmail) | Idem Webmail. |
| **miyukininotify** | optionnelle | Flux d'activité dans le portail. |
| **borderguard** | obligatoire | Gate Cores pour toute écriture (cohérent gouvernance Alicia). |

---

## 4. Protocoles standards exposés

### 4.1 WebDAV (RFC 4918)

- Cible : Windows Explorer, macOS Finder, Linux GVFS, Cyberduck, RaiDrive.
- Méthodes minimales : `PROPFIND`, `GET`, `PUT`, `MKCOL`, `DELETE`, `COPY`,
  `MOVE`, `LOCK`, `UNLOCK`.
- Extensions : sharing (rfc-draft), versioning (RFC 3253) — repoussés en P1.

### 4.2 CalDAV (RFC 4791)

- Cible : Thunderbird, Apple Calendar, DAVx⁵ Android, iOS.
- Surface : un seul calendrier par utilisateur en P0 ; multi-calendriers en P1.
- Mapping events ↔ `jaykoa::EventSummary` ; types récurrents `iCalendar`
  → équivalents JayKoa.

### 4.3 CardDAV (RFC 6352)

- Cible : Apple Contacts, Thunderbird, DAVx⁵, iOS.
- Surface : un seul carnet par utilisateur en P0.
- Mapping cartes ↔ `jaycontact::ContactSummary` ; champs vCard 4.0 mappés
  sur les attributs de contact existants ; champs non couverts gardés
  dans `extra` JSON.

### 4.4 Webmail

- Pas de protocole standard sortant (HTTPS portail uniquement).
- Pas de remplacement de Roundcube / SOGo — interface dédiée Miyukini
  cohérente avec le reste du portail.
- Recherche, lecture, écriture, gestion des pièces jointes (avec
  ouverture WebDAV pour partager via Drive).

### 4.5 Pas dans P0

| Protocole | Pourquoi reporté |
|-----------|------------------|
| **IMAP / POP3 / SMTP sortant** | JayMail backend les gère déjà ; JayCloud n'a pas vocation à les exposer en plus. |
| **iCalendar publish** | Calendriers publics partagés — P1. |
| **Federated identity (OIDC)** | Authentification fédérée inter-COG — P2. |
| **Collabora / OnlyOffice** | Édition collaborative — déjà couvert par Jay Bureau, donc hors scope. |

---

## 5. Sécurité & souveraineté

### 5.1 Niveaux de sécurité

| Donnée | Niveau (Politique résidence) |
|--------|------------------------------|
| Fichiers personnels (Drive) | Niveau 2 (Sensitive) par défaut ; tag `sensible` → Niveau 3. |
| Métadonnées agenda (CalDAV) | Niveau 2. |
| Contacts (CardDAV) | Niveau 2. |
| Corps de messages mail | Niveau 2 ; pièces jointes Niveau 3. |
| Jetons applicatifs DAV | Niveau 3 (révocables, scope par appareil). |
| Identifiants / mots de passe COG | Délégués à KindMother (Niveau 3). |

### 5.2 Modèle de menace

| Menace | Mitigation |
|--------|------------|
| Vol physique du COG | Chiffrement libSQL au repos, clés dérivées par identité, partition booting protégée. |
| Compromission d'un client DAV | Jetons applicatifs scopés et révocables ; chaque appareil a son jeton ; rotation auto sur soupçon. |
| Interception réseau | TLS 1.3 obligatoire sur tous les protocoles ; HSTS forcé sur le portail. |
| Compromission du portail web | Aucun mot de passe stocké côté JayCloud (délégué KindMother) ; sessions courtes ; CSRF tokens sur toute écriture. |
| Service Market — service tiers malveillant | JayCloud reste **officiel** ; aucun service tiers ne peut s'enregistrer comme handler DAV. |

### 5.3 Contrats de souveraineté

JayCloud garantit :

- **Aucune donnée ne quitte le COG** sauf à la demande explicite de l'utilisateur (partage de lien public, sync MWS avec un autre COG, etc.).
- **Aucun fournisseur cloud externe** n'est en chemin critique. Le portail
  tourne sur le COG lui-même ; les liens publics sont servis par l'instance.
- **Les clés ne quittent jamais KindMother**. JayCloud demande des chiffrements
  / déchiffrements ; il n'a jamais les clés brutes.

---

## 6. Migration depuis MiyuCloud

### 6.1 Stratégie

| Étape | Description | Côté utilisateur |
|-------|-------------|------------------|
| **M-0** | JayCloud livré avec un module `migrate-miyucloud` qui détecte une instance MiyuCloud présente. | Aucune action. |
| **M-1** | Au premier lancement de JayCloud, proposition de migration : import des fichiers MiyuCloud sous l'arborescence Drive de JayCloud. | Acceptation 1-clic dans Central. |
| **M-2** | MiyuCloud passe en mode **lecture seule** une fois la migration confirmée — il continue de servir les liens publics existants pendant la fenêtre de transition. | Aucune action. |
| **M-3** | Fenêtre de transition (90 jours par défaut) : les liens publics MiyuCloud existants redirigent vers les nouveaux liens JayCloud. | Aucune action ; rétrocompatibilité totale des liens. |
| **M-4** | Retrait de MiyuCloud : entrée ajoutée à `docs/services/DEPRECATED.md`, le service est désinstallé proprement par MasterButler. | Notification de fin de migration. |

### 6.2 Compatibilité des liens publics

Les URL MiyuCloud (`/cloud/files/<token>`) sont gardées comme alias
permanents pendant la fenêtre de transition. Une redirection 308
HTTPS pointe vers la nouvelle URL JayCloud (`/jaycloud/share/<token>`).
Aucun lien externe partagé par l'utilisateur ne casse.

### 6.3 Catalogue services

Le catalogue public `apps/origin/src/web/content.rs` doit être mis à jour
lors de la livraison de JayCloud :

- Ajout de l'entrée **JayCloud** (catégorie *Outils*).
- Retrait de l'entrée **MiyuCloud** (devient redirection automatique côté
  Origin vers la page JayCloud, comme pour JayXpose / JayFestival).

Ces deux modifications restent **hors périmètre du document fondateur** ;
elles seront pilotées par le ticket de livraison du service.

---

## 7. Roadmap (étapes de livraison)

> *Estimations conceptuelles uniquement — pas d'engagement de date dans
> ce document.*

| Phase | Périmètre | Livrable | État |
|-------|-----------|----------|------|
| **P0** | Document fondateur | Ce document | ✅ en cours |
| **P1** | Spec MSCM/MIP | *JayCloud - Spec MSCM MIP Conformite.md* — schémas Opérateurs/Kits, contrats d'intégration, choix des bibliothèques Rust. | à venir |
| **P2** | Skeleton crates | `crates/jaycloud/` (backend) + `crates/jaycloud-client/` (adapter Alicia) + `service.manifest.json` + workspace registration. | à venir |
| **P3** | Drive web + WebDAV | Portail HTTPS authentifié, navigation fichiers, upload/download, WebDAV monté depuis Windows/macOS. Tests CTS WebDAV. | à venir |
| **P4** | CalDAV + CardDAV | Adaptateurs CalDAV ↔ JayKoa, CardDAV ↔ JayContact. Tests DAVx⁵ Android + Thunderbird + Apple Calendar. | à venir |
| **P5** | Webmail | Interface webmail, lecture / écriture / pièces jointes via JayMail. | à venir |
| **P6** | Migration MiyuCloud | Module `migrate-miyucloud`, mode lecture seule, redirection des liens, retrait final. | à venir |
| **P7** | Intégration Alicia | Capacités exposées (`list_recent_uploads`, `share_link_for_file`, etc.) dans le manifeste Alicia. | à venir |
| **P8** | Polish & extensions | Versioning RFC 3253 ; iCalendar publish ; sharing extensions WebDAV. | à venir |

---

## Annexes

### A. Mapping Nextcloud → JayCloud

| Module Nextcloud | Équivalent JayCloud / Miyukini | Statut |
|------------------|-------------------------------|--------|
| Files | JayCloud Drive (intègre MiyuCloud) | ✅ P3 |
| Calendar | CalDAV → JayKoa | ✅ P4 |
| Contacts | CardDAV → JayContact | ✅ P4 |
| Mail | Webmail → JayMail | ✅ P5 |
| Talk | **Hors scope** — couvert par Jay Message + Jay1Tribu | ❌ |
| Office (Collabora) | **Hors scope** — couvert par Jay Bureau (Docs / Sheets / Slides) | ❌ |
| Photos / Music / Video | **Hors scope** — couvert par MiyukiniWatch (introspection) et services médias dédiés à venir | ❌ |
| Notes | **Hors scope** — futur service Miyukini dédié | ❌ |
| Tasks | Couvert par JayKoa (champs de type tâche) | ✅ inclus P4 |
| Activity feed | Bandeau de notifications portail JayCloud (consomme MiyukiniNotify) | ✅ P3 |
| Sharing | Liens publics signés par KindMother (P3) ; sharing inter-COG via MWS (P8) | partiel |
| Federation | Hors scope P0 → repoussé P8+ (OIDC inter-COG via MWS) | ❌ P0 |

### B. Glossaire local

| Terme | Définition |
|-------|------------|
| **Drive** | Espace de stockage de fichiers structuré en arborescence. |
| **Portail JayCloud** | Application web HTTPS authentifiée servie par le COG, point d'entrée web unifié. |
| **Jeton applicatif** | Token révocable scopé par appareil / client DAV, utilisé à la place du mot de passe COG. |
| **Opérateur Files** | Couche backend héritée de MiyuCloud, intégrée à JayCloud, qui gère le stockage chiffré des fichiers. |
| **Fenêtre de transition** | Période de coexistence MiyuCloud (lecture seule) + JayCloud (lecture / écriture) pendant la migration. |
| **CTS WebDAV** | Compatibility Test Suite — batterie de tests de conformité aux RFC DAV publiée par litmus / sabredav. |

### C. Risques identifiés

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Compatibilité partielle DAV avec certains clients exotiques | moyenne | moyen | Tests CTS WebDAV + matrice de clients officiellement supportés. |
| Performance du portail sous charge (Drive avec milliers de fichiers) | moyenne | élevé | Pagination, indexation libSQL FTS5, lazy-loading de l'arborescence. |
| Sync CalDAV/CardDAV bidirectionnelle complexe (résolution de conflits) | élevée | élevé | Stratégie « last-write-wins » en P4 ; CRDT en P7+. |
| Migration MiyuCloud sur très gros volumes (>100 Go) | moyenne | moyen | Migration en arrière-plan avec reprise sur erreur ; pas de copie, déplacement par renommage si filesystem identique. |
| Confusion utilisateur MiyuCloud → JayCloud pendant la transition | moyenne | faible | Notifications claires dans Central, page d'aide dédiée, redirection automatique des liens. |

---

> **Prochaine étape MIP** : production du document *JayCloud - Spec MSCM
> MIP Conformite.md* qui détaillera les Opérateurs / Kits, les contrats
> d'API entre JayCloud et les services consommés, et les choix de
> bibliothèques Rust (sabredav-rust ? rustical ? implémentation interne ?).
> Ce document ne peut commencer qu'après validation du présent document
> fondateur par les parties prenantes Miyukini.
