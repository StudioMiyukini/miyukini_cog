# MiyukiniWatch — Gouvernance des Données et Rétention

## Contexte

**MiyukiniWatch** collecte et stocke des métriques d'usage localement dans le COG. Ce document définit les règles de gouvernance des données : souveraineté, rétention, purge automatique, effacement par l'utilisateur, désactivation de la collecte et audit.

## Portée / Scope

- **Applicable à :** Gouvernance des données, politique de rétention, droits de l'utilisateur, audibilité.
- **Audience :** Architectes, développeurs, équipes sécurité, équipes produit.
- **Statut :** Document normatif — référence de gouvernance des données MiyukiniWatch.

---

## 1. Principe fondamental : souveraineté locale

> **Toutes les données MiyukiniWatch restent sur le COG. Aucun envoi à un serveur tiers. Aucune télémétrie externe.**

Ce principe découle directement des Lois d'Autonomie :

| Loi | Application |
|-----|-------------|
| **LOI-1** | Aucune dépendance externe pour le stockage des métriques. |
| **LOI-2** | MiyukiniWatch fonctionne identiquement en mode isolé. |
| **LOI-3** | L'état local (métriques) est souverain — aucune autorité externe ne peut les modifier. |

### 1.1 Autorité de persistance : KindMother

**KindMother** est l'autorité exclusive de persistance. Toute opération sur les données MiyukiniWatch passe par KindMother :

| Opération | Mécanisme | Validation |
|-----------|-----------|------------|
| **Écriture de métrique** | WriteIntent soumis à KindMother | StrongFather autorise ; KindMother persiste. |
| **Lecture de métrique** | ReadIntent soumis à KindMother | Master Butler vérifie l'accès. |
| **Effacement** | DeleteIntent soumis à KindMother | StrongFather autorise ; KindMother supprime. |
| **Purge automatique** | PurgeIntent déclenché par l'Aggregator | Politique de rétention validée par StrongFather. |

---

## 2. Politique de rétention

### 2.1 Durées de rétention par défaut

| Type de données | Rétention par défaut | Après expiration |
|-----------------|---------------------|------------------|
| **Métriques brutes** (par événement) | 30 jours | Purgées automatiquement ; les agrégats quotidiens sont conservés. |
| **Agrégats quotidiens** | 90 jours | Purgés automatiquement ; les agrégats hebdomadaires sont conservés. |
| **Agrégats hebdomadaires** | 365 jours | Purgés automatiquement. |
| **Compteurs globaux** (total sessions, streaks) | Illimitée (tant que le COG existe) | Effaçables manuellement par l'utilisateur. |

### 2.2 Cascade de rétention

La rétention suit une cascade descendante qui préserve l'information agrégée tout en purgeant les détails :

```
Métriques brutes (30 j.)
    │
    ▼ agrégation quotidienne
Agrégats quotidiens (90 j.)
    │
    ▼ agrégation hebdomadaire
Agrégats hebdomadaires (365 j.)
    │
    ▼ purge complète
Données supprimées
```

**Règle :** Lorsque les données brutes sont purgées, les agrégats quotidiens correspondants existent déjà. Il n'y a pas de perte d'information agrégée lors de la purge d'un niveau.

### 2.3 Configuration de la rétention

L'utilisateur peut ajuster les durées de rétention via les paramètres de MiyukiniWatch :

| Paramètre | Plage autorisée | Défaut | Niveau de configuration |
|-----------|----------------|--------|------------------------|
| Rétention métriques brutes | 7 – 90 jours | 30 jours | Préférence utilisateur |
| Rétention agrégats quotidiens | 30 – 365 jours | 90 jours | Préférence utilisateur |
| Rétention agrégats hebdomadaires | 90 – 730 jours | 365 jours | Préférence utilisateur |
| Rétention compteurs globaux | Illimitée ou effacement manuel | Illimitée | Effacement manuel (TAMR) |

**Contrainte :** La rétention d'un niveau ne peut pas être inférieure à celle du niveau supérieur (ex. les agrégats quotidiens ne peuvent pas être conservés moins longtemps que les métriques brutes).

---

## 3. Purge automatique

### 3.1 Déclenchement

La purge automatique est exécutée par l'**Opérateur MiyukiniWatchAggregator** aux moments suivants :

| Moment | Action |
|--------|--------|
| **Première session du jour** | Purge des métriques brutes expirées (> rétention configurée). |
| **Première session de la semaine** | Purge des agrégats quotidiens expirés. |
| **Première session du mois** | Purge des agrégats hebdomadaires expirés. |

### 3.2 Processus de purge

```
1. Aggregator identifie les enregistrements expirés
2. Aggregator soumet un PurgeIntent à BondingBrother
3. BondingBrother → StrongFather : "La purge est-elle autorisée ?"
   → StrongFather vérifie la politique de rétention (TAMR)
4. StrongFather émet un Mandat de Permission
5. BondingBrother → KindMother : PurgeIntent validé
6. KindMother supprime les enregistrements expirés
7. Confirmation remontée ; aucune notification à l'utilisateur (purge silencieuse)
```

### 3.3 Garantie d'intégrité

- La purge ne supprime **que** les données expirées selon la politique en vigueur.
- Les agrégats de niveau inférieur sont **toujours créés avant** la purge du niveau supérieur.
- En cas d'échec de la purge (ex. KindMother indisponible), la purge est reportée à la prochaine session.

---

## 4. Droits de l'utilisateur

### 4.1 Droit de consultation

L'utilisateur peut ouvrir MiyukiniWatch comme n'importe quel service depuis Miyukini Central. Il y consulte :

| Élément visible | Description |
|-----------------|-------------|
| **Résumé des mesures** | Synthèse lisible : dernières sessions, services les plus utilisés, amis récemment contactés, agrégats de clics. |
| **Périmètre explicite** | Liste claire des types de données enregistrés (sessions, services, amis, clics) avec rappel : « MiyukiniWatch ne lit pas le contenu de tes messages ni de tes saisies. » |
| **Historique détaillé** | Possibilité de consulter par période (jour, semaine, mois) selon la rétention configurée. |
| **Politique de rétention** | Durées en cours et espace de stockage utilisé. |

### 4.2 Droit d'effacement

L'utilisateur dispose de plusieurs niveaux d'effacement :

| Action | Portée | Effet sur Miou |
|--------|--------|----------------|
| **Effacer une période** | Toutes les données (brutes + agrégats) pour une plage définie (ex. « dernière semaine », « dernier mois »). | Miou perd le contexte de cette période ; messages génériques pour les suggestions affectées. |
| **Effacer un type de métrique** | Toutes les données d'un type (ex. « Amis et interactions sociales »). | Miou ne peut plus faire de rappels d'amis ; le reste des suggestions continue normalement. |
| **Effacer tout l'historique** | Toutes les données MiyukiniWatch (brutes, agrégats, compteurs). Remise à zéro complète. | Miou revient à des messages entièrement génériques jusqu'à ce que de nouvelles métriques soient collectées. |

### 4.3 Processus d'effacement

```
1. Utilisateur clique "Effacer" dans l'interface MiyukiniWatch
2. Confirmation demandée (pas d'effacement accidentel)
3. Opérateur Presenter soumet un DeleteIntent à BondingBrother
4. StrongFather valide l'effacement (TAMR : intervention humaine confirmée)
5. KindMother exécute la suppression
6. Confirmation affichée à l'utilisateur
7. Les agrégats Miou sont recalculés à partir des données restantes
```

### 4.4 Droit de désactivation

| Action | Description |
|--------|-------------|
| **Désactiver la collecte** | L'Opérateur Collector cesse d'enregistrer de nouvelles métriques. Les données existantes restent consultables et effaçables. |
| **Réactiver la collecte** | L'Opérateur Collector reprend la collecte. Les données antérieures à la désactivation ne sont pas reconstituées. |

La désactivation est accessible depuis :
- L'interface MiyukiniWatch (bouton « Désactiver la collecte »).
- Les Paramètres Miyukini Central (section « Données et vie privée »).

### 4.5 Effet de la désactivation sur Miou

| Collecte | Données existantes | Comportement Miou |
|----------|-------------------|-------------------|
| **Active** | Présentes | Messages personnalisés selon les agrégats. |
| **Active** | Effacées | Messages génériques ; nouvelles métriques reconstruisent progressivement le contexte. |
| **Désactivée** | Présentes | Miou utilise les agrégats existants (qui vieillissent et finissent par être purgés). |
| **Désactivée** | Effacées | Messages entièrement génériques (bienvenue, accueil standard). |

---

## 5. Résidence des données

### 5.1 Emplacement de stockage

| Composant | Stockage | Responsable |
|-----------|----------|-------------|
| Métriques brutes | Base KindMother locale (table ou collection `miyukiniwatch_metrics`) | KindMother |
| Agrégats quotidiens | Base KindMother locale (table ou collection `miyukiniwatch_daily`) | KindMother |
| Agrégats hebdomadaires | Base KindMother locale (table ou collection `miyukiniwatch_weekly`) | KindMother |
| Compteurs globaux | Base KindMother locale (table ou collection `miyukiniwatch_globals`) | KindMother |
| Préférences de rétention | Paramètres utilisateur dans Central | Central / KindMother |

### 5.2 Isolation et accès

| Règle | Description |
|-------|-------------|
| **Profil unique** | Les données MiyukiniWatch sont liées au profil connecté. Aucun autre profil ne peut y accéder. |
| **Pas d'export** | Aucune fonctionnalité d'export n'est fournie par défaut. Les données ne quittent pas le COG. |
| **Pas de partage** | Aucun mécanisme de partage des métriques avec d'autres COGs. |
| **Pas de sauvegarde externe** | Les données MiyukiniWatch ne sont pas incluses dans d'éventuels backups réseau (uniquement backup local COG si configuré). |

---

## 6. Audit et transparence

### 6.1 Journal d'audit

MiyukiniWatch maintient un journal minimal d'audit accessible à l'utilisateur :

| Événement audité | Informations enregistrées |
|-----------------|--------------------------|
| **Collecte activée** | `timestamp`, `previous_state` |
| **Collecte désactivée** | `timestamp`, `previous_state` |
| **Effacement partiel** | `timestamp`, `scope` (période, type de métrique) |
| **Effacement total** | `timestamp` |
| **Purge automatique** | `timestamp`, `records_purged`, `level` (brut, quotidien, hebdomadaire) |
| **Modification de rétention** | `timestamp`, `parameter`, `old_value`, `new_value` |

### 6.2 Accès au journal

Le journal d'audit est consultable dans l'interface MiyukiniWatch, section « Historique des actions ». Il est lui-même soumis à la rétention (365 jours par défaut, non configurable en dessous de 90 jours).

---

## 7. Migration de COG

En cas de migration d'un COG vers un nouvel environnement (LOI-8) :

| Aspect | Règle |
|--------|-------|
| **Inclusion** | Les données MiyukiniWatch sont incluses dans le périmètre de migration si l'utilisateur le souhaite. |
| **Processus formel** | Pas de copie brute ; les données sont exportées via un format structuré validé par EverBuddy (compatibilité de version). |
| **Choix utilisateur** | L'utilisateur peut choisir de migrer les données MiyukiniWatch, de les laisser sur l'ancien COG, ou de les effacer avant migration. |
| **Pas de doublon** | Après migration réussie, les données sur l'ancien COG sont marquées « migrées » (pas de suppression automatique). |

---

## 8. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision et principes de gouvernance. |
| [MiyukiniWatch — Spécification Fonctionnelle : Métriques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue des métriques, niveaux d'agrégation. |
| [MiyukiniWatch — Sécurité et Conformité](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md) | Chiffrement au repos, classification, audit sécurité. |
| [MiyukiniWatch — Interface Utilisateur et Écrans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md) | Écrans de consultation, effacement, paramètres. |
| [Miyukini Central — Démarrage, dépendances et KindMother](../MiyukiniCentral/Miyukini%20Central%20-%20Demarrage%20dependances%20et%20KindMother.md) | Dépendance à KindMother pour la persistance. |

---

**Document** : MiyukiniWatch — Gouvernance des Données et Rétention  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif — référence de gouvernance des données MiyukiniWatch
