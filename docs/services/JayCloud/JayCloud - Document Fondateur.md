# JayCloud — Document fondateur

## Contexte

**JayCloud** est le **service Miyukini de sauvegarde cloud souverain** du COG.
Il prend le relais de **MiyuCloud** (stockage chiffré limité, sans logique
de sauvegarde structurée) en se concentrant **exclusivement** sur les
fonctions de sauvegarde et de stockage durable :

| Fonction | Périmètre |
|----------|-----------|
| **Backup de fichiers** | Stockage chiffré au repos de fichiers et arborescences. |
| **Snapshots / versionning** | Snapshots datés (complets et incrémentaux), récupération point-in-time. |
| **Restore** | Restauration sélective (un fichier, un dossier, un snapshot complet) vers l'emplacement d'origine ou ailleurs. |
| **Partage de liens** | Liens publics signés (mot de passe / expiration) vers un fichier ou un snapshot. |
| **WebDAV (RFC 4918)** | Protocole standard pour outils de sauvegarde tiers (rclone, Cyberduck, RaiDrive) et montage natif OS si voulu. |
| **Migration depuis MiyuCloud** | Rapatriement transparent de l'arborescence + redirection des liens publics existants. |

> **JayCloud ne fait QUE de la sauvegarde cloud.** Aucune fonction de
> messagerie, agenda, contacts, bureautique, lecture média, ou
> synchronisation live de données métier n'est dans le périmètre.
> Ces domaines appartiennent aux services Jay dédiés correspondants.

Ce document est le **document fondateur** du service. Il fixe la raison
d'être, le périmètre étroit, le positionnement vis-à-vis de
Nextcloud et des autres services Miyukini, et les règles de sécurité.

## Portée / Scope

- **Périmètre** : Service de sauvegarde cloud souverain — backup chiffré
  de fichiers, snapshots, restore, partage de liens, exposition WebDAV
  pour outils tiers, migration MiyuCloud. **Une seule responsabilité :
  la sauvegarde**.
- **Hors périmètre** : Toute fonctionnalité qui appartient déjà à un
  service Jay :
  - CalDAV / synchronisation agenda → reste avec **JayKoa**.
  - CardDAV / synchronisation contacts → reste avec **JayContact**.
  - Mail (UI ou API web) → reste avec **JayMail**.
  - Messagerie temps réel → reste avec **JayMessage / Jay1Tribu**.
  - Office collaboratif → reste avec **Jay Bureau** (Docs / Sheets / Slides).
  - Lecture média → services médias dédiés à venir.
- **Hors périmètre aussi** : tout portail web généraliste de type
  Nextcloud (apps, dashboard, multi-fonctionnel). JayCloud expose
  WebDAV + une interface web **minimale** dédiée backup/restore.

### Cadre de travail

| Élément | Description |
|---------|-------------|
| **Documentation autorisée** | Glossaire Miyukini ; Document fondateur **MiyuCloud** (archivé) ; RFC 4918 (WebDAV) ; Politique de résidence des données sensibles. |
| **Ce document ne fusionne pas** | Avec les Documents fondateurs des autres services Jay — JayCloud ne consomme pas leurs API. |
| **Ce document n'anticipe pas** | Les spécifications techniques (renvoyées au document P1 *JayCloud - Spec MSCM MIP Conformite*). |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Une seule responsabilité** | JayCloud ne fait QUE de la sauvegarde cloud. Aucun ajout fonctionnel hors backup ne sera accepté en P0/P1. |
| ❌ **Aucun doublon avec un service Jay** | Si une fonctionnalité Nextcloud appartient à un service Jay existant (mail, calendar, contacts, office), elle reste là où elle est. JayCloud ne la duplique pas. |
| ❌ **Pas de UI généraliste type Nextcloud** | Une interface web **minimaliste** dédiée à voir / déclencher / restaurer une sauvegarde, c'est tout. |
| ✅ **Source de vérité** | Ce document est la **référence** pour le périmètre et les règles du service JayCloud. |

### Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | JayCloud **remplace** MiyuCloud, en élargissant uniquement vers les fonctions de **backup structurées** (snapshots, restore, scheduling) que MiyuCloud n'avait pas. | MiyuCloud n'était qu'un dossier chiffré ; JayCloud apporte la logique de sauvegarde. Cohérent avec la marque Jay (services métiers). |
| **DS-02** | Scope = **backup files + snapshots + restore + partage + WebDAV + migration**. Rien d'autre. | Garantit que JayCloud reste un service **étroit et maîtrisé**, sans dérive vers le hub multi-fonction Nextcloud-like. |
| **DS-03** | JayCloud ne **consomme** aucun autre service Jay. | Un service de sauvegarde n'a pas besoin de connaître les contenus métier ; il opère sur des fichiers / blobs. Élimine tout couplage et tout risque de doublon. |
| **DS-04** | **Données souveraines au minimum niveau 2 (Sensitive)** ; niveau 3 pour les snapshots tagués `sensible`. | Cohérent avec MiyuCloud et la Politique de résidence Miyukini. |
| **DS-05** | **WebDAV (RFC 4918) comme seul protocole standard exposé.** Pas de CalDAV, pas de CardDAV, pas de IMAP/SMTP. | Garde JayCloud strictement dans son périmètre backup. CalDAV / CardDAV restent l'affaire de futures expositions par JayKoa / JayContact eux-mêmes si nécessaire. |
| **DS-06** | **Portail web minimaliste** = liste des snapshots, déclencher backup manuel, lancer un restore, gérer les liens partagés. Pas de Drive UI généraliste. | Évite de réinventer un explorateur de fichiers à la Nextcloud ; concentre l'UI sur les opérations backup. |
| **DS-07** | Authentification : **identité COG (KindMother)** + jetons applicatifs WebDAV révocables par appareil / outil. | Cohérent avec le reste du COG. Chaque outil de backup tiers reçoit son token scopé. |
| **DS-08** | **Aucune duplication avec les services Jay existants.** JayMail / JayKoa / JayContact / JayMessage / Jay1Tribu / Jay Bureau / JayManga restent les sources de vérité de leur domaine. | Cette spec ne crée pas de nouveau silo de fonctionnalités déjà présentes ailleurs. |

---

## 1. Besoins stratégiques

### 1.1 Origine du besoin

Trois constats motivent la création d'un service de sauvegarde cloud
dédié :

1. **MiyuCloud est un dossier chiffré, pas une sauvegarde.** Il stocke
   des fichiers, mais ne fournit ni snapshots datés, ni stratégie de
   rétention, ni restore structuré, ni indicateur d'intégrité.
2. **La souveraineté exige une sauvegarde locale + une copie distante.**
   La règle 3-2-1 (3 copies, 2 médias, 1 hors site) doit être tenable
   dans l'écosystème COG, sans dépendre d'un fournisseur cloud externe.
3. **Les autres services Jay sont autonomes mais ne backupent pas leur
   propre donnée.** JayKonta, JayKoa, JayContact, JayMail, etc.,
   produisent des données. Une couche de sauvegarde **transverse**
   (au-dessus du filesystem ou via export structuré) leur évite à chacun
   de réimplémenter une politique de sauvegarde.

### 1.2 Besoins fonctionnels identifiés

| Besoin | Description |
|--------|-------------|
| **Backup ad-hoc** | Sauvegarde manuelle d'un fichier ou d'un dossier à la demande. |
| **Backup planifié** | Sauvegarde automatique selon un planning (quotidien, hebdo, mensuel) configurable par cible. |
| **Snapshots** | Snapshot daté complet + snapshots incrémentaux. Politique de rétention configurable (ex: 7 quotidiens, 4 hebdo, 12 mensuels). |
| **Restore sélectif** | Récupération d'un fichier précis depuis un snapshot, ou restauration complète d'un snapshot vers un emplacement choisi. |
| **Partage de liens** | Liens publics ou semi-privés (mot de passe / expiration) vers un fichier sauvegardé. |
| **Exposition WebDAV** | Outils tiers (rclone, Cyberduck, ou simple montage OS) accèdent aux fichiers via WebDAV standard. |
| **Migration MiyuCloud** | Rapatriement automatique de l'arborescence MiyuCloud, redirection 308 des liens publics existants. |
| **Auth + app-passwords** | Délégation à KindMother pour l'identité, jetons révocables par outil de backup. |
| **Intégrité** | Vérification de checksums (SHA-256) à l'écriture et à la lecture ; alerte sur corruption. |

### 1.3 Besoin stratégique de fond

JayCloud répond à un besoin stratégique simple : **rendre la sauvegarde
du COG fiable, vérifiable et souveraine**, sans dépendre d'un service
cloud externe ni d'un script fait maison par chaque utilisateur.

---

## 2. Positionnement

### 2.1 Vis-à-vis de Nextcloud

JayCloud **n'est pas un fork de Nextcloud**. JayCloud ne reprend que
la couche **Files** de Nextcloud (sauvegarde + stockage), et l'épure à
l'essentiel.

| Axe | Nextcloud (Files seul) | JayCloud |
|-----|------------------------|----------|
| **Périmètre** | Drive + sharing + apps optionnelles (Calendar, Contacts, Mail, Office…) | **Sauvegarde** + sharing + WebDAV. Une seule responsabilité. |
| **Stack** | PHP + Apache/Nginx + MariaDB/PostgreSQL | Rust async (Tokio + Axum + libSQL chiffré). |
| **Apps** | Catalogue d'apps Nextcloud (calendar, contacts, mail, deck, talk…) | **Aucune.** Les autres domaines = services Jay dédiés (JayKoa, JayContact, JayMail, JayMessage…). |
| **Maintenance** | Manuelle (cron, occ commands) | Auto-géré par MasterButler. |
| **Chiffrement** | Optionnel, complexe à configurer | Par défaut, dérivé de l'identité KindMother. |

### 2.2 Vis-à-vis des autres services Miyukini

JayCloud **ne consomme** aucun autre service Jay (DS-03). Il ne stocke
que ce qu'on lui demande de sauvegarder. Les autres services restent
**propriétaires de leur donnée live** ; JayCloud opère uniquement sur
des fichiers et blobs qu'on lui confie.

| Service Jay | Rapport avec JayCloud |
|-------------|------------------------|
| **JayKonta** | Aucun couplage. Si l'utilisateur veut sauvegarder son fichier `jaykonta.db`, il configure une cible JayCloud — sans dépendance API. |
| **JayKoa** | Aucun couplage. JayKoa peut exporter un fichier `.ics` que JayCloud sauvegardera comme n'importe quel fichier. |
| **JayContact** | Aucun couplage. Idem (export `.vcf` si besoin). |
| **JayMail** | Aucun couplage. JayMail garde ses propres archives mail. |
| **JayMessage / Jay1Tribu** | Aucun couplage. |
| **Jay Bureau (Docs / Sheets / Slides)** | Aucun couplage. Les docs Jay Bureau sont sauvegardés comme fichiers ordinaires. |
| **JayManga** | Aucun couplage. |
| **MiyukiniWatch** | Aucun couplage. |

### 2.3 Vis-à-vis de Central

Central reste le hub natif (UI Dioxus desktop / mobile). JayCloud est
un service backend qui expose :

- **Une UI web minimaliste** (lister snapshots, déclencher backup,
  lancer restore, gérer partages) accessible depuis n'importe quel
  navigateur.
- **WebDAV** pour outils tiers.
- **API interne** consommée par Central pour configurer les sauvegardes.

---

## 3. Architecture cible (vue conceptuelle)

> *La spécification technique détaillée est hors périmètre du document
> fondateur — référencée dans le futur document* JayCloud - Spec MSCM MIP.

```
┌─────────────────────────────────────────────────────────────┐
│                  JayCloud (service COG)                     │
│                                                             │
│        ┌──────────────────┐    ┌────────────────┐           │
│        │ UI web backup    │    │ Portail conn.  │           │
│        │ (HTML+HTMX min.) │    │ Identité COG   │           │
│        └────────┬─────────┘    └────────┬───────┘           │
│                 │                       │                   │
│  ┌──────────────┴───────────────────────┴─────────────────┐ │
│  │           Axum HTTPS portail (auth COG)                │ │
│  └──────────────┬───────────────────────┬─────────────────┘ │
│                 │                       │                   │
│        ┌────────┴────────┐    ┌─────────┴───────┐           │
│        │  WebDAV         │    │  API backup     │           │
│        │  (RFC 4918)     │    │  (REST interne) │           │
│        └────────┬────────┘    └─────────┬───────┘           │
│                 │                       │                   │
│  ┌──────────────┴───────────────────────┴─────────────────┐ │
│  │  Opérateurs : files / snapshots / restore / share /    │ │
│  │               auth                                     │ │
│  └──────────────────────┬─────────────────────────────────┘ │
│                         │                                   │
│        ┌────────────────┴──────────────────────┐            │
│        │  Storage local chiffré (libSQL +      │            │
│        │  filesystem chiffré au repos)         │            │
│        └────────────────┬──────────────────────┘            │
│                         │                                   │
│        ┌────────────────┴──────────────────────┐            │
│        │  KindMother (identité, clés)          │            │
│        └───────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### 3.1 Couches

| Couche | Rôle |
|--------|------|
| **Portail** | Authentification COG, sessions, UI backup. |
| **Protocole** | Adaptateur WebDAV (RFC 4918) pour les outils de sauvegarde tiers. |
| **Opérateurs** | files (stockage), snapshots (politique), restore, share, auth. |
| **Storage** | Filesystem chiffré + libSQL pour métadonnées. |
| **Sécurité** | KindMother pour l'identité et les clés ; libSQL chiffré ; jetons applicatifs révocables. |

### 3.2 Dépendances

| Dépendance | Type | Justification |
|------------|------|---------------|
| **kindmother** | obligatoire | Identité COG, dérivation des clés. |
| **borderguard** | obligatoire | Gate Cores pour toute écriture (cohérent gouvernance Alicia). |
| **miyukininotify** | optionnelle | Notifications (backup terminé, échec, restore en cours). |

**JayCloud ne dépend d'aucun autre service Jay** (DS-03).

---

## 4. Protocole WebDAV (seul protocole standard exposé)

- **Référence** : RFC 4918.
- **Cible** : outils tiers de sauvegarde (rclone, restic via WebDAV
  backend, Duplicati, Cyberduck) + montage natif optionnel sur OS.
- **Méthodes** : `OPTIONS`, `PROPFIND`, `GET`, `HEAD`, `PUT`, `DELETE`,
  `MKCOL`, `COPY`, `MOVE`, `LOCK`, `UNLOCK`.
- **Pas de CalDAV ni CardDAV** (DS-05).

---

## 5. Sécurité & souveraineté

### 5.1 Niveaux de sécurité

| Donnée | Niveau (Politique résidence) |
|--------|------------------------------|
| Fichiers sauvegardés | Niveau 2 par défaut ; tag `sensible` → Niveau 3. |
| Snapshots | Niveau hérité du contenu sauvegardé. |
| Métadonnées de partage public | Niveau 2. |
| Jetons applicatifs WebDAV | Niveau 3 (révocables, scope par appareil). |
| Identifiants COG | Délégués à KindMother (Niveau 3). |

### 5.2 Modèle de menace

| Menace | Mitigation |
|--------|------------|
| Vol physique du COG | Chiffrement au repos (filesystem + libSQL), clés dérivées KindMother. |
| Compromission d'un outil de backup tiers | Jetons applicatifs scopés et révocables. |
| Interception réseau | TLS 1.3 obligatoire ; HSTS forcé. |
| Compromission du portail | Aucun mot de passe stocké ; sessions courtes ; CSRF sur toute écriture. |
| Corruption silencieuse du storage | Checksums SHA-256 systématiques ; vérification d'intégrité périodique configurable. |

### 5.3 Contrats de souveraineté

JayCloud garantit :

- **Aucune donnée ne quitte le COG** sauf à la demande explicite de
  l'utilisateur (partage de lien public, copie distante vers un
  autre COG via MWS — futur P6).
- **Aucun fournisseur cloud externe** n'est en chemin critique.
- **Les clés ne quittent jamais KindMother**.

---

## 6. Migration depuis MiyuCloud

### 6.1 Stratégie

| Étape | Description | Côté utilisateur |
|-------|-------------|------------------|
| **M-0** | JayCloud livré avec un module `migrate-miyucloud` qui détecte une instance MiyuCloud présente. | Aucune action. |
| **M-1** | Au premier lancement, proposition de migration : import de l'arborescence MiyuCloud comme premier snapshot JayCloud. | Acceptation 1-clic dans Central. |
| **M-2** | MiyuCloud passe en lecture seule. Les liens publics existants restent servis pendant la fenêtre de transition. | Aucune action. |
| **M-3** | Fenêtre de transition (90 jours par défaut) : redirection 308 des liens MiyuCloud vers JayCloud. | Aucune action ; rétrocompatibilité totale. |
| **M-4** | Retrait de MiyuCloud : entrée dans `docs/services/DEPRECATED.md`, désinstallation propre. | Notification fin de migration. |

### 6.2 Catalogue services

Lors de la livraison de JayCloud :

- `apps/origin/src/web/content.rs` : retirer **MiyuCloud**, ajouter
  **JayCloud** (catégorie *Outils*).
- `docs/services/DEPRECATED.md` : section MiyuCloud.

---

## 7. Roadmap (étapes de livraison)

> *Estimations conceptuelles uniquement.*

| Phase | Périmètre | Livrable | État |
|-------|-----------|----------|------|
| **P0** | Document fondateur | Ce document | ✅ en cours |
| **P1** | Spec MSCM/MIP | *JayCloud - Spec MSCM MIP Conformite.md* — opérateurs, contrats internes, choix bibliothèques. | à venir |
| **P2** | Skeleton crates | `crates/jaycloud/` + `crates/jaycloud-client/` + `crates/jaycloud-migrate/` + `service.manifest.json` + workspace registration. | à venir |
| **P3** | Backup core + WebDAV | files_op héritier MiyuCloud, snapshots_op, restore_op, share_op, WebDAV adapter (Litmus pass). | à venir |
| **P4** | UI web backup + scheduling | Portail HTTPS minimaliste (liste snapshots, déclencher backup, lancer restore, gérer partages). Backups planifiés. | à venir |
| **P5** | Migration MiyuCloud | Module `migrate-miyucloud`, redirection liens, catalogue services MAJ. | à venir |
| **P6** | Intégration Alicia | Capacités (`trigger_backup`, `list_snapshots`, `restore_file`, `share_file`) dans le manifeste Alicia. Sauvegarde MWS inter-COG. | à venir |
| **P7** | Polish | Vérification intégrité périodique, RFC 3253 versioning WebDAV, sharing extensions. | à venir |

---

## Annexes

### A. Mapping Nextcloud → JayCloud

| Module Nextcloud | Statut JayCloud |
|------------------|-----------------|
| Files (Drive + WebDAV) | ✅ Périmètre principal de JayCloud. |
| Sharing | ✅ Liens publics signés KindMother. |
| Versioning Nextcloud | ✅ Implémenté via snapshots. |
| Trash bin | ✅ Snapshots permettent la récupération. |
| Calendar | ❌ Hors scope — couvert par **JayKoa**. |
| Contacts | ❌ Hors scope — couvert par **JayContact**. |
| Mail | ❌ Hors scope — couvert par **JayMail**. |
| Talk | ❌ Hors scope — couvert par **JayMessage + Jay1Tribu**. |
| Office (Collabora) | ❌ Hors scope — couvert par **Jay Bureau**. |
| Photos / Music / Video | ❌ Hors scope — services médias dédiés à venir. |
| Notes | ❌ Hors scope — futur service Miyukini dédié. |
| Tasks | ❌ Hors scope — couvert par JayKoa. |
| Federation | ❌ P7 (OIDC inter-COG via MWS). |
| Activity feed | ⚠ minimal — notifications de backup uniquement, via MiyukiniNotify. |

### B. Glossaire local

| Terme | Définition |
|-------|------------|
| **Backup** | Copie chiffrée d'un fichier / dossier à un instant T, stockée hors de la source. |
| **Snapshot** | Backup daté nommé (complet ou incrémental) avec politique de rétention associée. |
| **Restore** | Récupération d'un ou plusieurs fichiers depuis un snapshot vers un emplacement choisi. |
| **Cible de backup** | Configuration nommée pour une sauvegarde planifiée (source, planning, rétention). |
| **Politique de rétention** | Règles définissant combien de snapshots garder et lesquels (ex: 7 quotidiens + 4 hebdo + 12 mensuels). |
| **Fenêtre de transition** | Période (90j par défaut) pendant laquelle MiyuCloud reste accessible en lecture seule pour servir les liens existants. |

### C. Risques identifiés

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Volume MiyuCloud >100 Go à migrer | moyenne | moyen | Migration par rename si même filesystem ; sinon copie streamée avec reprise. |
| Corruption silencieuse de snapshots anciens | faible | élevé | Vérification d'intégrité périodique (SHA-256), notification utilisateur. |
| Rétention mal configurée → perte de données | moyenne | élevé | Valeurs par défaut conservatrices ; confirmation explicite avant suppression de snapshot. |
| Outils WebDAV exotiques mal supportés | faible | faible | Matrice d'outils officiellement testés (rclone, restic, Cyberduck, Duplicati). |

---

> **Prochaine étape MIP** : Spec MSCM/MIP P1 — détaille les opérateurs
> (files / snapshots / restore / share / auth), les contrats internes,
> les bibliothèques Rust retenues, et le plan d'implémentation.
