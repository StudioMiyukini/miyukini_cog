# MiyukiniWatch — Intégration Miou et Agrégats

## Contexte

**Miou** — l'avatar et mascotte des COGs — est le principal consommateur des données produites par **MiyukiniWatch**. Ce document décrit le contrat d'intégration entre les deux services : quels agrégats sont exposés, sous quelle forme, avec quelles garanties, et comment Miou les utilise pour générer des bulles, rappels et suggestions adaptés au bien-être de l'utilisateur.

## Portée / Scope

- **Applicable à :** Contrat d'intégration MiyukiniWatch → Miou, format des agrégats, règles de consommation, exemples de bulles.
- **Audience :** Développeurs Miou, développeurs MiyukiniWatch, équipes produit, architectes.
- **Statut :** Spécification fonctionnelle normative — contrat entre MiyukiniWatch et Miou.

---

## 1. Principe d'intégration

### 1.1 Séparation des responsabilités

| Service | Responsabilité |
|---------|---------------|
| **MiyukiniWatch** | Collecte, stocke, agrège les métriques. Expose des **agrégats** en lecture seule à Miou. Ne décide pas du contenu des bulles. |
| **Miou** | Consomme les agrégats. Génère les messages, bulles, rappels et suggestions. Décide du ton, du timing et de la priorisation. |

**Règle fondamentale :** MiyukiniWatch est un **fournisseur de données**. Miou est un **consommateur de contexte**. La responsabilité de la production des bulles appartient exclusivement à Miou.

### 1.2 Agrégats, pas données brutes

Miou ne reçoit **jamais** les données brutes de MiyukiniWatch. Il reçoit uniquement des **agrégats pré-calculés** par l'Opérateur MiyukiniWatchAggregator.

| Ce que Miou reçoit | Ce que Miou ne reçoit PAS |
|--------------------|--------------------------|
| « 3 jours depuis la dernière session » | Liste détaillée de toutes les sessions |
| « Top 3 services : JayXpose (12), JayKoa (5), Jay1Tribu (3) » | Horodatage de chaque ouverture de service |
| « Ami X non contacté depuis 10 jours » | Historique complet des interactions avec chaque ami |
| « Activité : 5 sessions cette semaine » | Compteurs de clics bruts par minute |

---

## 2. Catalogue des agrégats exposés

### 2.1 Agrégats de session

| Identifiant | Nom | Contenu | Fraîcheur |
|-------------|-----|---------|-----------|
| `AGG_SESSION_SUMMARY` | Résumé de session | `days_since_last_session`, `avg_duration_seconds`, `usual_time_slot`, `total_sessions`, `consecutive_active_days` | Mis à jour à chaque nouvelle session |
| `AGG_SESSION_RETURN` | Indicateur de retour | `is_returning` (true si > 1 jour d'absence), `days_away` | Calculé à la connexion |
| `AGG_SESSION_TIME` | Tranche horaire actuelle | `current_time_slot` (MORNING / AFTERNOON / EVENING / NIGHT) | Temps réel |

### 2.2 Agrégats de services

| Identifiant | Nom | Contenu | Fraîcheur |
|-------------|-----|---------|-----------|
| `AGG_TOP_SERVICES` | Services les plus utilisés | Liste ordonnée `[(service_id, open_count, total_duration)]` — top 5, période 7 jours | Mis à jour à chaque session |
| `AGG_NEGLECTED_SERVICES` | Services délaissés | Liste `[(service_id, days_since_last_open)]` — services non ouverts depuis > 14 jours | Mis à jour quotidiennement |
| `AGG_FAVORITE_SERVICE` | Service favori | `service_id`, `open_count_week`, `total_duration_week` — service le plus fréquenté sur 7 jours | Mis à jour à chaque session |
| `AGG_FAVORITE_TAB` | Onglet favori | `tab_id` (Salon / Bibliothèque / Webway), `usage_ratio` | Mis à jour à chaque session |

### 2.3 Agrégats sociaux (amis)

| Identifiant | Nom | Contenu | Fraîcheur |
|-------------|-----|---------|-----------|
| `AGG_FRIEND_REMINDERS` | Rappels d'amis | Liste `[(friend_cog_id, days_since_last_interaction)]` — amis non contactés depuis > 7 jours | Mis à jour quotidiennement |
| `AGG_TOP_FRIENDS` | Amis les plus contactés | Liste ordonnée `[(friend_cog_id, total_duration_minutes)]` — top 3, période 30 jours | Mis à jour hebdomadairement |
| `AGG_SOCIAL_ACTIVITY` | Activité sociale | `distinct_friends_contacted_week`, `total_social_time_week` | Mis à jour à chaque session |

### 2.4 Agrégats d'activité

| Identifiant | Nom | Contenu | Fraîcheur |
|-------------|-----|---------|-----------|
| `AGG_ACTIVITY_LEVEL` | Niveau d'activité | `level` (inactive / low / moderate / active / very_active), `sessions_week`, `total_duration_week` | Mis à jour à chaque session |
| `AGG_CURRENT_SESSION` | Session en cours | `duration_current_session`, `services_opened_count` | Temps réel |

### 2.5 Agrégats de jalons (milestones)

| Identifiant | Nom | Contenu | Fraîcheur |
|-------------|-----|---------|-----------|
| `AGG_MILESTONES` | Jalons et badges | Liste `[(milestone_type, value, timestamp_achieved)]` — streaks, badges, premières fois | Mis à jour à la détection |
| `AGG_NEW_MILESTONE` | Nouveau jalon | `milestone_type`, `value` — non-null seulement si un jalon vient d'être atteint dans la session en cours | Session courante |

---

## 3. Contrat de consommation

### 3.1 API conceptuelle

Miou accède aux agrégats via une interface de lecture simple, gouvernée par les Cores :

```
MiouContext::get_aggregate(aggregate_id: &str) -> Option<Aggregate>
MiouContext::get_all_aggregates() -> Vec<Aggregate>
MiouContext::has_data() -> bool  // true si MiyukiniWatch a des données
MiouContext::is_collecting() -> bool  // true si la collecte est active
```

### 3.2 Règles de consommation

| Règle | Description |
|-------|-------------|
| **Lecture seule** | Miou ne peut pas modifier, effacer ou écrire dans MiyukiniWatch. |
| **Graceful degradation** | Si un agrégat n'est pas disponible (données effacées, collecte désactivée, catégorie désactivée), Miou utilise un message générique. Pas d'erreur, pas de crash. |
| **Pas de mise en cache longue** | Miou ne met pas en cache les agrégats au-delà de la session. À chaque nouvelle session, les agrégats sont rechargés depuis MiyukiniWatch. |
| **Résolution des pseudos** | L'identifiant technique `friend_cog_id` est résolu en pseudo lisible par le service de contacts (Jay1Tribu), pas par MiyukiniWatch ni par Miou directement. |
| **Pas de dépendance forte** | Si MiyukiniWatch est indisponible (hypothèse théorique), Miou continue de fonctionner avec des messages génériques. |

### 3.3 Gouvernance de l'accès

L'accès aux agrégats passe par BondingBrother et les Cores :

```
Miou demande un agrégat
  → BondingBrother
    → Master Butler : "Miou a-t-il le droit de lire cet agrégat ?"
    → KindMother : ReadIntent pour l'agrégat
  → Agrégat retourné à Miou
```

**Master Butler** maintient une entrée de permission permanente pour Miou en lecture sur les agrégats MiyukiniWatch. Cette permission est standard et ne nécessite pas d'intervention utilisateur.

---

## 4. Utilisation par Miou — Exemples de bulles

### 4.1 Matrice agrégat → bulle

| Agrégat consommé | Condition | Exemple de bulle Miou |
|-----------------|-----------|----------------------|
| `AGG_SESSION_RETURN` | `days_away >= 3` | « Tu n'es pas passé depuis 3 jours — tu m'as manqué ! » |
| `AGG_SESSION_RETURN` | `days_away == 0` | « Content de te revoir aujourd'hui ! » |
| `AGG_SESSION_TIME` | `current_time_slot == MORNING` | « Bonjour [pseudo] ! Bien dormi ? » |
| `AGG_SESSION_TIME` | `current_time_slot == EVENING` | « Bonsoir [pseudo], une petite visite ce soir ? » |
| `AGG_CURRENT_SESSION` | `duration > 90 min` | « Tu es là depuis un moment — une petite pause ? » |
| `AGG_FAVORITE_SERVICE` | — | « Tu reviens souvent sur JayXpose — ta vitrine est à jour ? » |
| `AGG_NEGLECTED_SERVICES` | `days_since > 21` | « Tu n'as pas ouvert JayKoa depuis 3 semaines — un événement à rappeler ? » |
| `AGG_FRIEND_REMINDERS` | `days_since > 14` | « Pense à reprendre contact avec [pseudo], ça fait un moment ! » |
| `AGG_TOP_FRIENDS` | — | « Tu passes beaucoup de temps avec [pseudo] — une belle amitié ! » |
| `AGG_ACTIVITY_LEVEL` | `level == "inactive"` | « On ne se voit plus trop... reviens quand tu veux ! » (après plusieurs jours) |
| `AGG_NEW_MILESTONE` | `type == "streak", value == 7` | « 7 jours d'affilée avec ton COG — bravo ! » |
| `AGG_NEW_MILESTONE` | `type == "first_service"` | « Premier service installé — bienvenue dans l'aventure ! » |
| `AGG_FAVORITE_TAB` | `tab == "Bibliothèque"` | « Tu passes souvent par la Bibliothèque — un service à découvrir ? » |
| (aucun agrégat) | `has_data() == false` | « Bienvenue dans Miyukini Central ! » (message générique) |
| (collecte off) | `is_collecting() == false` | Messages génériques (bienvenue, heure de la journée). |

### 4.2 Priorisation

Lorsque plusieurs bulles sont possibles, Miou applique une **file de priorité** :

| Priorité | Type de bulle | Exemples |
|----------|---------------|----------|
| **P0** | Accueil / retour | Bonjour/bonsoir, « Tu m'as manqué ». |
| **P1** | Jalons et badges | Streak, premier service, réseau connecté. |
| **P2** | Rappels utiles | Ami non contacté, service délaissé. |
| **P3** | Suggestions légères | Service favori, onglet habituel. |
| **P4** | Contexte ambiant | Durée de session, activité. |

En une session, Miou affiche **1 à 3 bulles maximum** (configurable). Les bulles de priorité supérieure passent en premier.

### 4.3 Cooldowns et non-répétition

| Règle | Description |
|-------|-------------|
| **Pas de répétition immédiate** | Un même type de bulle n'est pas affiché deux fois dans la même session. |
| **Cooldown par agrégat** | Chaque type d'agrégat a un cooldown (ex. `AGG_FRIEND_REMINDERS` : 3 jours entre deux rappels pour le même ami). |
| **Cooldown session** | Après le message d'accueil, un délai minimum avant la bulle suivante (ex. 5 minutes). |
| **Respecter le dismiss** | Si l'utilisateur ferme une bulle, ne pas reproposer le même contenu dans la session. |

---

## 5. Comportement en l'absence de données

### 5.1 Dégradation gracieuse

| Scénario | Comportement MiyukiniWatch | Comportement Miou |
|----------|---------------------------|-------------------|
| **Collecte active, données fraîches** | Agrégats complets et à jour. | Bulles personnalisées, rappels, suggestions. |
| **Collecte active, peu de données** (début) | Agrégats partiels ou vides. | Messages d'accueil génériques + « Reviens dans quelques jours pour que je te connaisse mieux. » |
| **Collecte active, données effacées** | Agrégats vides ; nouvelles données commencent à s'accumuler. | Messages génériques ; personnalisation progressive. |
| **Collecte désactivée, données présentes** | Agrégats existants (stables, non mis à jour). | Bulles basées sur les anciens agrégats (qui vieillissent). |
| **Collecte désactivée, données effacées** | Rien. | Messages entièrement génériques (bienvenue, heure). |

### 5.2 Messages génériques de repli

Miou dispose d'un ensemble de messages génériques (non basés sur MiyukiniWatch) pour chaque créneau :

| Créneau | Exemple de message générique |
|---------|------------------------------|
| Matin | « Bonjour [pseudo] ! Qu'est-ce qu'on fait aujourd'hui ? » |
| Après-midi | « Bon après-midi ! Un tour dans les services ? » |
| Soir | « Bonsoir [pseudo], content de te voir ! » |
| Nuit | « Encore debout ? N'oublie pas de te reposer. » |
| Retour (sans donnée de jours) | « Bienvenue dans Miyukini Central ! » |

---

## 6. Cycle de vie des agrégats

```
┌─────────────────┐     ┌───────────────────┐     ┌─────────────┐
│  MiyukiniWatch   │     │  Aggregator        │     │    Miou      │
│  Collector       │────▶│  (agrège)          │────▶│  (consomme)  │
│  (métriques)     │     │                     │     │  (bulles)    │
└─────────────────┘     └───────────────────┘     └─────────────┘
       │                          │                       │
       │  WriteIntent             │  Stockage agrégats    │  ReadIntent
       │  → KindMother            │  → KindMother         │  → KindMother
       ▼                          ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    KindMother (persistance)                      │
│  miyukiniwatch_metrics │ miyukiniwatch_daily │ miyukiniwatch_agg│
└─────────────────────────────────────────────────────────────────┘
```

### 6.1 Fraîcheur et mise à jour

| Moment | Action Aggregator | Agrégats mis à jour |
|--------|-------------------|---------------------|
| **Connexion (début de session)** | Calcul `AGG_SESSION_RETURN`, `AGG_SESSION_TIME` | Session, retour, heure |
| **Pendant la session** | Mise à jour `AGG_CURRENT_SESSION` (périodique) | Durée session en cours |
| **Ouverture d'un service** | Recalcul `AGG_TOP_SERVICES`, `AGG_FAVORITE_SERVICE` | Services |
| **Interaction sociale** | Recalcul `AGG_FRIEND_REMINDERS`, `AGG_TOP_FRIENDS` | Amis |
| **Détection de jalon** | Ajout à `AGG_MILESTONES`, publication `AGG_NEW_MILESTONE` | Jalons |
| **Fin de session** | Agrégation complète de la session | Tous les agrégats de session |
| **Quotidien** | Agrégation quotidienne | Agrégats quotidiens |
| **Hebdomadaire** | Agrégation hebdomadaire | Agrégats hebdomadaires |

---

## 7. Invariants d'intégration

| # | Invariant |
|---|-----------|
| **INT-1** | Miou ne reçoit que des agrégats, jamais des données brutes. |
| **INT-2** | Miou ne peut pas modifier les données MiyukiniWatch. |
| **INT-3** | Si MiyukiniWatch n'a pas de données, Miou fonctionne en mode générique. |
| **INT-4** | Miou ne stocke pas les agrégats MiyukiniWatch au-delà de la session. |
| **INT-5** | L'effacement des données par l'utilisateur est reflété immédiatement dans les agrégats. |
| **INT-6** | La désactivation de la collecte par catégorie rend les agrégats correspondants indisponibles. |
| **INT-7** | Le contrat d'agrégats est versionné avec l'environnement COG (LOI-7). |

---

## 8. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Lien fondateur MiyukiniWatch → Miou (section 5). |
| [MiyukiniWatch — Spécification Fonctionnelle : Métriques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue des métriques sources, format des agrégats. |
| [Miyukini Central — Miou, avatar, bulles et rôle](../MiyukiniCentral/Miyukini%20Central%20-%20Miou%20avatar%20bulles%20et%20role.md) | Rôle de Miou, canal de communication (bulles). |
| [Miou — Index du sous-service](../MiyukiniCentral/Miou/_index.md) | Architecture Miou, documents associés. |
| [Miou — Moteur de Génération Templates et LLM](../MiyukiniCentral/Miou/Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) | Architecture hybride de génération des bulles. |
| [Miyukini Central — Salon propositions](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md) | Utilisation des métriques dans le Salon. |

---

**Document** : MiyukiniWatch — Intégration Miou et Agrégats  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Spécification fonctionnelle normative — contrat entre MiyukiniWatch et Miou
