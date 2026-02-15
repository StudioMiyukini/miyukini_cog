# MiyukiniWatch — Contraintes et Invariants

## Contexte

Ce document rassemble l'ensemble des **contraintes non négociables** et des **invariants** qui régissent MiyukiniWatch. Ces règles s'appliquent à toute conception, implémentation, évolution et maintenance du service. Aucune exception ne peut être accordée sans remettre en cause les fondements de MiyukiniWatch.

## Portée / Scope

- **Applicable à :** Toutes les strates impliquées (Opérateurs, Outils, Cores), toute équipe travaillant sur MiyukiniWatch.
- **Audience :** Architectes, développeurs, équipes sécurité, équipes produit, auditeurs.
- **Statut :** Document normatif — contraintes non négociables.

---

## 1. Contraintes fondamentales

### 1.1 Non-lecture des contenus (Contrainte absolue)

| # | Contrainte | Portée | Justification |
|---|------------|--------|---------------|
| **C-01** | **MiyukiniWatch ne lit JAMAIS le contenu des messages.** | Tous les services de messagerie (Jay1Tribu, etc.) | Respect de la vie privée. Confiance utilisateur. |
| **C-02** | **MiyukiniWatch ne lit JAMAIS le contenu des champs saisis.** | Tous les formulaires et champs de texte. | Aucune analyse textuelle. |
| **C-03** | **MiyukiniWatch ne lit JAMAIS le contenu des fichiers.** | Fichiers envoyés, reçus, consultés. | Pas d'accès aux données métier. |
| **C-04** | **MiyukiniWatch ne lit JAMAIS le contenu des pages ou écrans affichés.** | Tous les services. | Pas de scraping, pas de capture d'écran, pas d'analyse DOM. |

**Formulation synthétique :**

> MiyukiniWatch enregistre uniquement **quand**, **où**, **qui** (identifiant technique) et **combien**. Jamais **quoi** (contenu).

### 1.2 Localité des données

| # | Contrainte | Description |
|---|------------|-------------|
| **C-05** | **Données locales uniquement.** | Toutes les données MiyukiniWatch restent sur le COG. Aucun envoi à un serveur tiers. Aucune télémétrie externe. |
| **C-06** | **Pas d'export par défaut.** | Aucune fonctionnalité d'export des données MiyukiniWatch hors du COG. |
| **C-07** | **Pas de partage Inter-COG.** | Les données MiyukiniWatch ne sont jamais partagées avec d'autres COGs. |
| **C-08** | **Indépendance réseau.** | MiyukiniWatch fonctionne identiquement que le COG soit connecté au MWS ou totalement isolé. |

### 1.3 Transparence et maîtrise utilisateur

| # | Contrainte | Description |
|---|------------|-------------|
| **C-09** | **Transparence totale.** | L'utilisateur peut consulter toutes les données collectées en ouvrant le service MiyukiniWatch. Aucune donnée cachée. |
| **C-10** | **Effacement possible.** | L'utilisateur peut effacer tout ou partie des données à tout moment. L'effacement est irréversible et effectif immédiatement. |
| **C-11** | **Désactivation possible.** | L'utilisateur peut désactiver toute nouvelle collecte. Les données existantes restent consultables et effaçables. |
| **C-12** | **Périmètre documenté et visible.** | La liste des métriques collectées est documentée (section 2 du Document Fondateur) et visible dans l'interface du service. |

### 1.4 Gouvernance par les Cores

| # | Contrainte | Description |
|---|------------|-------------|
| **C-13** | **Persistance via KindMother.** | Toute écriture, lecture ou suppression de données passe par KindMother (WriteIntent / ReadIntent / DeleteIntent). Pas de persistance directe. |
| **C-14** | **Autorisation via StrongFather.** | Toute opération de collecte ou d'effacement est soumise à l'autorisation de StrongFather via BondingBrother. |
| **C-15** | **Permissions via Master Butler.** | Les capacités (consulter, effacer, désactiver) sont déclarées et gouvernées par Master Butler. |
| **C-16** | **Sécurité via WorrySentinel.** | Le niveau de sécurité des données MiyukiniWatch est défini par WorrySentinel. |
| **C-17** | **Frontières via Border Guard.** | Border Guard garantit que les données ne franchissent jamais les frontières du COG. |

---

## 2. Invariants architecturaux

| # | Invariant | Description |
|---|-----------|-------------|
| **INV-01** | **Service interne COG (Type 1)** | MiyukiniWatch est et restera un service interne COG. Pas de surface web externe. Pas de protocole Inter-COG. |
| **INV-02** | **Silencieux** | MiyukiniWatch ne génère aucune notification, popup ou bandeau. Toute communication avec l'utilisateur passe par Miou ou par l'ouverture volontaire du service. |
| **INV-03** | **Passivité de la collecte** | Le Collector ne provoque jamais d'événement. Il ne modifie pas le comportement des autres services. Il écoute passivement les événements existants. |
| **INV-04** | **Agrégats pour Miou, pas données brutes** | Miou ne reçoit que des agrégats pré-calculés, jamais les métriques brutes. |
| **INV-05** | **Pas de blocage** | La collecte ne bloque jamais l'interface utilisateur ni les autres services. L'écriture est toujours asynchrone. |
| **INV-06** | **Priorité basse** | En cas de contention système (Caring Nanny T2+), MiyukiniWatch est le premier service à être réduit ou suspendu. |
| **INV-07** | **Horodatages locaux** | Tous les horodatages sont locaux au COG (LOI-4). Pas de synchronisation temporelle externe. |
| **INV-08** | **Gamification positive uniquement** | Les métriques ne sont jamais utilisées pour culpabiliser, punir ou créer de la pression. Miou utilise les agrégats pour encourager, féliciter et rappeler avec bienveillance. |

---

## 3. Invariants de données

| # | Invariant | Description |
|---|-----------|-------------|
| **DAT-01** | **Rétention bornée** | Toutes les données ont une durée de rétention finie (sauf les compteurs globaux, effaçables manuellement). Pas de stockage illimité. |
| **DAT-02** | **Cascade d'agrégation** | Les métriques brutes sont agrégées avant d'être purgées. Aucune perte d'information agrégée lors de la purge. |
| **DAT-03** | **Déduplication** | Un même événement reçu deux fois ne produit pas de doublon. |
| **DAT-04** | **Profil unique** | Les données sont liées au profil connecté. Aucun autre profil ne peut y accéder. |
| **DAT-05** | **Volumétrie contrôlée** | Des limites de volumétrie sont en place (métriques max par session, espace de stockage max). |

---

## 4. Invariants d'intégration (contrat Miou)

| # | Invariant | Description |
|---|-----------|-------------|
| **INT-01** | **Lecture seule pour Miou** | Miou ne peut pas modifier, effacer ou écrire dans MiyukiniWatch. |
| **INT-02** | **Dégradation gracieuse** | Si MiyukiniWatch n'a pas de données, Miou fonctionne avec des messages génériques. Pas d'erreur, pas de crash. |
| **INT-03** | **Résolution pseudo hors MiyukiniWatch** | Les identifiants techniques (`friend_cog_id`) sont résolus en pseudos par le service de contacts, pas par MiyukiniWatch. |
| **INT-04** | **Effacement immédiat** | L'effacement par l'utilisateur est reflété immédiatement dans les agrégats exposés à Miou. |
| **INT-05** | **Pas de cache longue** | Miou ne met pas en cache les agrégats au-delà de la session. |
| **INT-06** | **Versionnement** | Le contrat d'agrégats est versionné avec l'environnement COG (LOI-7). |

---

## 5. Conformité aux Lois d'Autonomie

| Loi | Conformité | Contraintes associées |
|-----|------------|----------------------|
| **LOI-1** | Aucune dépendance externe critique | C-05, C-08 |
| **LOI-2** | L'isolement est un état normal | C-08 |
| **LOI-3** | L'état local est souverain | C-05, C-06, C-07, C-10 |
| **LOI-4** | Pas de temps global requis | INV-07 |
| **LOI-5** | Coût proportionnel au hardware | INV-06, DAT-05 |
| **LOI-6** | L'autonomie n'empêche pas la fédération | C-08 (fonctionnement indépendant) |
| **LOI-7** | Strate Cores immuable | INT-06 |
| **LOI-8** | Migration = diplomatie | Migration formelle, pas de copie brute |

---

## 6. Matrice de vérification

Cette matrice peut être utilisée pour auditer la conformité d'une implémentation de MiyukiniWatch :

| Vérification | Critère | Résultat attendu |
|-------------|---------|-------------------|
| MiyukiniWatch accède-t-il au contenu des messages ? | C-01 | **Non** |
| MiyukiniWatch accède-t-il au contenu des champs de saisie ? | C-02 | **Non** |
| MiyukiniWatch accède-t-il aux fichiers ? | C-03 | **Non** |
| Des données quittent-elles le COG ? | C-05, C-06, C-07 | **Non** |
| L'utilisateur peut-il voir toutes les données ? | C-09 | **Oui** |
| L'utilisateur peut-il effacer les données ? | C-10 | **Oui** |
| L'utilisateur peut-il désactiver la collecte ? | C-11 | **Oui** |
| L'écriture passe-t-elle par KindMother ? | C-13 | **Oui** |
| L'autorisation passe-t-elle par StrongFather ? | C-14 | **Oui** |
| MiyukiniWatch génère-t-il des notifications ? | INV-02 | **Non** |
| La collecte bloque-t-elle l'UI ? | INV-05 | **Non** |
| Miou reçoit-il des données brutes ? | INV-04 | **Non** |
| Les données ont-elles une rétention bornée ? | DAT-01 | **Oui** |
| Les horodatages sont-ils locaux ? | INV-07 | **Oui** |
| Le service fonctionne-t-il hors réseau ? | C-08 | **Oui** |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Principes fondateurs, invariant de non-lecture. |
| [MiyukiniWatch — Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Interactions Cores, flux de gouvernance. |
| [MiyukiniWatch — Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Politique de rétention, droits utilisateur. |
| [MiyukiniWatch — Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Contrat d'intégration, invariants Miou. |
| Architecture Miyukini (skill miyukini-architecture) | Lois d'Autonomie, Pyramide, Cores. |

---

**Document** : MiyukiniWatch — Contraintes et Invariants  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif — contraintes non négociables du Service MiyukiniWatch
