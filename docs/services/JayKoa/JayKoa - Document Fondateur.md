# JayKoa — Document fondateur

## Contexte

**JayKoa** est le **service Miyukini unifié du domaine agenda** au sein de l’écosystème COG. Il fournit une couche commune de modélisation, de conflits, de fuseaux et d’export pour tout ce qui relève du calendrier et des plages temporelles. Les services métier (JayRDV, JayFestival, et futurs services intervenant sur l’agenda) **s’appuient sur JayKoa** au lieu de dupliquer chacun sa propre logique agenda.

Ce document est le **document fondateur** du service : il en fixe la raison d’être, les besoins stratégiques, le positionnement, l’intégration avec les autres services et les niveaux de sécurité associés à la sensibilité des données. Il s’adresse aux équipes produit, technique, sécurité et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayKoa — besoins, positionnement stratégique, intégration multi-services, niveaux de sécurité et solutions de protection.
- **Hors périmètre** : Spécifications techniques détaillées (API, schémas), implémentation des crates (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md), [Niveaux de sécurité et protection](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

---

## 1. Besoins stratégiques

### 1.1 Origine du besoin

La définition des services **JayRDV** (prise de rendez-vous, créneaux, plannings) et **JayFestival** (agenda cross-événements, conflits de dates exposants/visiteurs) a fait émerger un **besoin transversal** :

- **JayRDV** : calendrier, créneaux, RDV, plannings, exceptions — consommation de Miyubooking et MiyuClock.
- **JayFestival** : agenda cross-événements, détection de conflits de dates (exposant ou visiteur inscrit à deux événements à la même date), vue calendrier, export/partage.

Sans service unifié, chaque service définit son propre « Kit Agenda » ou « Kit Calendrier », ce qui conduit à une **duplication de la sémantique agenda** et à l’impossibilité d’agréger, pour un même utilisateur, des entrées issues de plusieurs services (RDV + participations festivals + ateliers, etc.).

### 1.2 Besoins fonctionnels identifiés

| Besoin | Description | Consommateurs typiques |
|--------|-------------|-------------------------|
| **Entrées agenda unifiées** | Modélisation commune des « entrées » agenda : RDV, éditions/événements, participations, ateliers, créneaux réservés. | JayRDV, JayFestival, futurs services |
| **Détection de conflits** | Vérification de chevauchement de plages (dates, créneaux) avant validation d’une action (candidature, réservation). | JayFestival (exposants, visiteurs), JayRDV (double réservation) |
| **Vue calendrier agrégée** | Une vue utilisateur pouvant agréger des sources multiples (RDV + festivals + ateliers) selon Mandat et permissions. | Tous les services avec espace utilisateur |
| **Fuseaux et temps** | Référence temporelle cohérente (MiyuClock), fuseaux, pas de temps global requis (LOI-4). | Tous |
| **Export et partage** | Export (iCal, PDF), partage contrôlé, sans exposition de données sensibles au-delà du périmètre autorisé. | JayFestival, JayRDV, utilisateurs finaux |
| **Règles de visibilité** | Qui peut voir quelles entrées : selon service d’origine, rôle, Mandat, niveau de sécurité. | Tous |
| **Types d’événements et conflits** | Plusieurs types d’événements agenda ; les événements de type **présence physique** ne doivent pas se chevaucher — si l’utilisateur force, notification et indicateurs UI (alerte, rouge clignotant) pour pousser à la résolution, sans bloquer la réservation ni l’entrée dans l’agenda. | Tous |

### 1.3 Types d’événements agenda et règle « présence physique »

JayKoa distingue **plusieurs types d’événements** agenda (RDV, édition festival, atelier, etc.). Pour certains types, une **règle de non-chevauchement** s’applique :

| Type d’événement | Règle de conflit | Comportement si l’utilisateur force |
|------------------|------------------|-------------------------------------|
| **Présence physique** | Les événements de type **présence physique** **ne doivent pas se chevaucher** (une même personne ne peut pas être à deux endroits en même temps). | JayKoa **ne bloque pas** la réservation ni l’entrée de l’événement dans l’agenda. L’utilisateur est **notifié** ; la réservation ou l’inscription est enregistrée. JayKoa **pousse l’utilisateur à résoudre** le conflit horaire par des **alertes** et des **indicateurs UI** (ex. rouge clignotant sur les entrées en conflit) jusqu’à résolution. |
| **Autres types** | Règles définies par le service consommateur ou par contrat (blocage optionnel, simple alerte, etc.). | Selon règle métier du service. |

**Principe** : pour les événements **présence physique**, le conflit est **signalé et persistant** (alertes + indicateurs UI en rouge clignotant) tant que l’utilisateur n’a pas résolu le chevauchement (annulation, report, modification d’un des événements). La réservation ou l’entrée dans l’agenda reste possible pour ne pas bloquer le parcours utilisateur ; JayKoa incite à la résolution sans imposer de blocage technique.

### 1.4 Référentiel inspiré des agendas grand public (ex. Google Agenda)

Pour enrichir la documentation conceptuelle, JayKoa s’appuie sur un **référentiel fonctionnel** inspiré des capacités des agendas grand public (notamment **Google Agenda**) : vues (jour, semaine, mois, liste/agenda), rappels, partage (niveaux lecture/écriture, indicateur libre/occupé), calendriers multiples, export et synchronisation. La traduction de ces concepts en capacités et contraintes Miyukini (Mandats, WorrySentinel, services consommateurs) est détaillée dans [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md). Ce référentiel respecte le [Protocole d’écriture de la documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) (structure, portée, dépendances explicites).

### 1.5 Besoin stratégique de fond

> **Un service d’agenda unifié permet de coupler tous les services qui interviennent sur le domaine agenda, d’éviter la duplication et de garantir une expérience cohérente (conflits, fuseaux, export) pour l’utilisateur final.**

La création de **JayKoa** est **préalable ou parallèle structurant** au développement des couches agenda de JayRDV et JayFestival : sans lui, chaque service figerait un modèle propre et l’agrégation cross-service deviendrait coûteuse ou impossible.

---

## 2. Positionnement stratégique

### 2.1 Raison d’être

**JayKoa** a pour objectif de :

- **Centraliser le domaine « agenda »** : plages temporelles, types d’entrées (RDV, événement, atelier, etc.), règles de conflit, fuseaux. **JayKoa intègre tout ce qui manipule des dates** (JayRDV, JayFestival, et tout futur service).
- **Exposer des Opérateurs et Kits réutilisables** : les services métier (JayRDV, JayFestival, etc.) consomment JayKoa au lieu d’implémenter leur propre couche.
- **Permettre l’agenda multi-sources** : un même utilisateur peut disposer d’une vue agrégée (RDV + participations festivals + ateliers) lorsque les services concernés s’appuient sur JayKoa et que les Mandats et permissions le permettent.

### 2.2 Positionnement dans la pyramide Miyukini

| Élément | Rôle |
|--------|------|
| **MiyuClock** | Horloge locale, trace only — pas de logique métier agenda. |
| **Miyubooking** | Réservation de créneaux, booking (outil bas niveau). |
| **JayKoa** | **Service** de domaine « agenda » : orchestration, modélisation des entrées, conflits, visibilité, export. S’appuie sur MiyuClock, Miyubooking, KindMother, etc. |
| **JayRDV / JayFestival** | **Services métier** qui s’appuient sur JayKoa (et Miyubooking) pour tout ce qui relève du calendrier et des conflits. |

JayKoa ne remplace pas Miyubooking ni MiyuClock : il **s’appuie sur eux** et ajoute la couche **sémantique et gouvernance** du domaine agenda (entrées typées, conflits, agrégation, niveaux de sécurité).

### 2.3 Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather (décisions), KindMother (données), Master Butler (permissions), WorrySentinel (niveaux de sécurité, états de confiance). |
| **Réutilisabilité** | Un seul modèle agenda pour l’écosystème ; les services métier se couplent via Opérateurs et Kits, pas en dupliquant la logique. |
| **Couplage explicite** | Les services consommateurs (JayRDV, JayFestival, etc.) déclarent leur usage de JayKoa et le type d’entrées qu’ils publient (RDV, édition, atelier, etc.). |
| **Sécurité par niveau** | Les données agenda sont classées par niveau de sensibilité (WorrySentinel 0–4) ; les solutions de protection (résidence, chiffrement, audit) sont alignées sur ces niveaux. |

---

## 3. Intégration avec les autres services

### 3.1 Services consommateurs identifiés

| Service | Usage de JayKoa | Type d’entrées agenda |
|---------|--------------------------|-------------------------|
| **JayRDV** | Créneaux, plannings, RDV ; vérification conflits (double réservation) ; vue calendrier pro/client ; export. | RDV, créneaux réservés, exceptions |
| **JayFestival** | Agenda cross-événements ; conflits de dates (exposant/visiteur) ; vue calendrier éditions/participations ; export iCal/PDF. | Éditions, participations, candidatures, ateliers réservés |
| **Futurs services** | Tout service qui gère des plages temporelles, des réservations ou des événements (formations, interventions, etc.). | À définir par service |

### 3.2 Modèle d’intégration

- **JayKoa** expose des **Opérateurs** et **Kits d’outils** (ex. : `agenda.entries.list`, `agenda.conflict.check`, `agenda.export.ical`, `agenda.visualize`).
- Chaque **service consommateur** :
  - détient les **données métier** (qui a quel RDV, quelle candidature, etc.) ;
  - **publie** vers JayKoa les **entrées agenda** nécessaires (plage, type, identifiant opaque, niveau de sécurité) ;
  - **interroge** JayKoa pour conflits, vues agrégées, export, selon Mandat et permissions.
- **KindMother** : la résidence des données sensibles (qui appartient à quel utilisateur, quel RDV, etc.) reste définie par le **contrat de chaque service** (JayRDV, JayFestival) et la [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md). JayKoa peut détenir des **références** ou des **synthèses** (dates, types, conflits) sans être la seule copie des données personnelles ou métier.

Détail des schémas d’intégration : [JayKoa - Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md).

### 3.3 Bénéfice multi-services

- **Un utilisateur (ex. exposant)** peut avoir un **agenda unique** qui affiche ses RDV (JayRDV) et ses participations festivals (JayFestival), avec détection de conflits et export commun, si les deux services s’appuient sur JayKoa et que les Mandats autorisent l’agrégation.
- **Réduction de la duplication** : une seule implémentation des règles de conflit, fuseaux et export.
- **Cohérence** : mêmes règles de visibilité et de sécurité appliquées à toutes les entrées agenda, quel que soit le service d’origine.

---

## 4. Niveaux de sécurité et solutions de protection

### 4.1 Sensibilité des données agenda

Les données traitées par JayKoa peuvent être de **sensibilité variable** selon le type d’entrée et le contexte :

| Type de donnée | Exemple | Sensibilité | Niveau WorrySentinel typique |
|----------------|---------|-------------|------------------------------|
| **Métadonnées temporelles** | Date de début/fin, fuseau, type d’entrée (RDV, édition) | Faible | 0–1 |
| **Référence opaque** | Identifiant de l’entrée (sans nom, sans détail métier) | Faible à standard | 1 |
| **Contexte utilisateur** | « Mes » entrées, agrégation par utilisateur | Standard à sensible | 1–2 |
| **Données personnelles ou métier** | Nom du client, objet du RDV, nom de l’exposant, détail d’une candidature | Sensible à critique | 2–3 |

JayKoa **ne doit pas** être le seul détenteur des données personnelles ou métier : il travaille en **référence** (identifiants, plages, types) ; les données complètes restent sous la responsabilité des services métier et de leur COG de référence.

### 4.2 Niveaux de sécurité (rappel Glossaire)

| Niveau | Nom | Description |
|-------|-----|-------------|
| **0** | Public | Données publiques, aucune contrainte stricte |
| **1** | Standard | Données standard, contraintes de base |
| **2** | Sensitive | Données sensibles, contraintes renforcées |
| **3** | Critical | Données critiques, contraintes strictes |
| **4** | Highest | Sécurité maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sécurité et les états de confiance ; il ne décide pas des actions ni des permissions (Master Butler, StrongFather).

### 4.3 Solutions de protection par niveau

| Niveau | Mesures de protection |
|--------|----------------------|
| **0 – Public** | Aucune mesure spécifique ; pas de données personnelles. |
| **1 – Standard** | Contrôle d’accès (Mandat, Master Butler) ; traçabilité des accès ; pas de résidence centralisée obligatoire. |
| **2 – Sensitive** | Résidence centralisée sur COG de référence (selon [Politique de résidence](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)) ; accès via Visite gouvernée ou session ; audit des lectures/écritures ; pas d’exposition hors Mandat. |
| **3 – Critical** | Résidence centralisée obligatoire ; chiffrement au repos et en transit ; audit complet ; révocation immédiate possible (StrongFather, WorrySentinel). |
| **4 – Highest** | Contraintes maximales ; isolement renforcé ; procédures d’accès exceptionnel (TAMR, MiyukiniAdmin). |

Pour JayKoa :

- Les **entrées agenda** exposées à JayKoa (plage, type, id opaque) sont en général **niveau 0–1** tant qu’aucune donnée personnelle ou métier n’est incluse.
- Dès qu’une **agrégation** ou une **vue utilisateur** associe des entrées à une identité ou à un contexte métier, le **niveau du flux** est au moins **1**, et **2** si des données sensibles sont affichées ou exportées.
- Les **services consommateurs** (JayRDV, JayFestival) restent responsables du **niveau des données métier** qu’ils publient ou interrogent ; JayKoa applique les **règles de visibilité et d’export** selon le niveau déclaré et le Mandat.

Référence détaillée : [JayKoa - Niveaux Securite et Protection Donnees](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

### 4.4 Règles de sécurité spécifiques au service

| Règle | Description |
|-------|-------------|
| **AGD-SEC-1** | JayKoa ne persiste pas la copie canonique des données personnelles ou métier des services consommateurs ; il travaille sur références et synthèses. |
| **AGD-SEC-2** | Toute agrégation cross-service (vue utilisateur, export) est soumise à Mandat de Permission et au niveau de sécurité du contexte (WorrySentinel). |
| **AGD-SEC-3** | L’export (iCal, PDF) ne doit pas inclure de données au-delà du niveau autorisé pour le destinataire (ex. pas de noms de tiers en export public). |
| **AGD-SEC-4** | En état de confiance dégradé (T2–T4), les capacités d’agrégation ou d’export peuvent être restreintes (Caring Nanny, WorrySentinel). |
| **AGD-SEC-6** | Pour les événements de type **présence physique** en conflit : pas de blocage de la réservation ou de l’entrée dans l’agenda ; notification systématique ; indicateurs UI (alerte, rouge clignotant) pour pousser à la résolution du conflit jusqu’à ce que l’utilisateur le résolve. |

---

## 5. Prochaines étapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser (interne / partenaires).
2. **Spécifier** : Documenter les Opérateurs et Kits JayKoa (entrées, conflits, vue, export) et leurs Contrats d’équipe.
3. **Écrans et parcours** : S’appuyer sur [Écrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) et [Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) pour l’intégration dans les UIs des services consommateurs.
4. **Bornage** : Respecter le [Bornage pour l’implémentation](./JayKoa%20-%20Bornage%20Implementation.md) (MVP, phases, hors scope, dépendances).
5. **Sécurité** : Finaliser le document [Niveaux Sécurité et Protection](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) et l’alignement avec la Politique de résidence.
6. **Intégration** : Formaliser les contrats d’intégration avec JayRDV et JayFestival (voir [Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md)).
7. **Implémentation** : Développer les crates et Opérateurs en s’appuyant sur MiyuClock, Miyubooking, KindMother, WorrySentinel.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Opérateur, Mandat, COG, Niveaux de sécurité, WorrySentinel). |
| [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | Résidence centralisée, COG de référence, niveaux 2+. |
| [JayKoa - Niveaux Securite et Protection Donnees](./reference/JayKoa%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Détail des niveaux et mesures de protection pour le service. |
| [JayKoa - Ecrans et UI](./JayKoa%20-%20Ecrans%20et%20UI.md) | Besoins en écrans et UI (composants, patterns). |
| [JayKoa - Parcours Utilisateurs](./JayKoa%20-%20Parcours%20Utilisateurs.md) | Parcours utilisateurs et parcours côté service. |
| [JayKoa - Bornage Implementation](./JayKoa%20-%20Bornage%20Implementation.md) | Bornage pour l’implémentation (MVP, phases, hors scope). |
| [JayKoa - Integration Services Consommateurs](./reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) | Schémas d’intégration JayRDV, JayFestival ; séquence d’intégration, contrat conceptuel, filtres. |
| [JayKoa - Operateurs et Toolkits](./JayKoa%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs, Kits (Entrées, Conflits, Vue & Export, Événements publics), Équipe, filtres supportés. |
| [JayKoa - Audit Documentation et Manques](./JayKoa%20-%20Audit%20Documentation%20et%20Manques.md) | Audit de la documentation et manques pour un service complet. |
| [JayKoa - Referentiel Fonctionnel Inspire Google Agenda](./reference/JayKoa%20-%20Referentiel%20Fonctionnel%20Inspire%20Google%20Agenda.md) | Référentiel fonctionnel inspiré de Google Agenda (vues, rappels, partage, libre/occupé, calendriers multiples). |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (RDV, créneaux). |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (agenda cross-événements). |

---

**Document** : JayKoa — Document fondateur  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document fondateur — référence pour le service (besoins, positionnement, intégration multi-services, sécurité)
