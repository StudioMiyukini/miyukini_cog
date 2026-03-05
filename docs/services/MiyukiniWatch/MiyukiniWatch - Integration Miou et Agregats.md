# MiyukiniWatch â€” IntÃ©gration Miou et AgrÃ©gats

## Contexte

**Miou** â€” l'avatar et mascotte des COGs â€” est le principal consommateur des donnÃ©es produites par **MiyukiniWatch**. Ce document dÃ©crit le contrat d'intÃ©gration entre les deux services : quels agrÃ©gats sont exposÃ©s, sous quelle forme, avec quelles garanties, et comment Miou les utilise pour gÃ©nÃ©rer des bulles, rappels et suggestions adaptÃ©s au bien-Ãªtre de l'utilisateur.

## PortÃ©e / Scope

- **Applicable Ã  :** Contrat d'intÃ©gration MiyukiniWatch â†’ Miou, format des agrÃ©gats, rÃ¨gles de consommation, exemples de bulles.
- **Audience :** DÃ©veloppeurs Miou, dÃ©veloppeurs MiyukiniWatch, Ã©quipes produit, architectes.
- **Statut :** SpÃ©cification fonctionnelle normative â€” contrat entre MiyukiniWatch et Miou.

---

## 1. Principe d'intÃ©gration

### 1.1 SÃ©paration des responsabilitÃ©s

| Service | ResponsabilitÃ© |
|---------|---------------|
| **MiyukiniWatch** | Collecte, stocke, agrÃ¨ge les mÃ©triques. Expose des **agrÃ©gats** en lecture seule Ã  Miou. Ne dÃ©cide pas du contenu des bulles. |
| **Miou** | Consomme les agrÃ©gats. GÃ©nÃ¨re les messages, bulles, rappels et suggestions. DÃ©cide du ton, du timing et de la priorisation. |

**RÃ¨gle fondamentale :** MiyukiniWatch est un **fournisseur de donnÃ©es**. Miou est un **consommateur de contexte**. La responsabilitÃ© de la production des bulles appartient exclusivement Ã  Miou.

### 1.2 AgrÃ©gats, pas donnÃ©es brutes

Miou ne reÃ§oit **jamais** les donnÃ©es brutes de MiyukiniWatch. Il reÃ§oit uniquement des **agrÃ©gats prÃ©-calculÃ©s** par l'OpÃ©rateur MiyukiniWatchAggregator.

| Ce que Miou reÃ§oit | Ce que Miou ne reÃ§oit PAS |
|--------------------|--------------------------|
| Â« 3 jours depuis la derniÃ¨re session Â» | Liste dÃ©taillÃ©e de toutes les sessions |
| Â« Top 3 services : JayXpose (12), JayKoa (5), Jay1Tribu (3) Â» | Horodatage de chaque ouverture de service |
| Â« Ami X non contactÃ© depuis 10 jours Â» | Historique complet des interactions avec chaque ami |
| Â« ActivitÃ© : 5 sessions cette semaine Â» | Compteurs de clics bruts par minute |

---

## 2. Catalogue des agrÃ©gats exposÃ©s

### 2.1 AgrÃ©gats de session

| Identifiant | Nom | Contenu | FraÃ®cheur |
|-------------|-----|---------|-----------|
| `AGG_SESSION_SUMMARY` | RÃ©sumÃ© de session | `days_since_last_session`, `avg_duration_seconds`, `usual_time_slot`, `total_sessions`, `consecutive_active_days` | Mis Ã  jour Ã  chaque nouvelle session |
| `AGG_SESSION_RETURN` | Indicateur de retour | `is_returning` (true si > 1 jour d'absence), `days_away` | CalculÃ© Ã  la connexion |
| `AGG_SESSION_TIME` | Tranche horaire actuelle | `current_time_slot` (MORNING / AFTERNOON / EVENING / NIGHT) | Temps rÃ©el |

### 2.2 AgrÃ©gats de services

| Identifiant | Nom | Contenu | FraÃ®cheur |
|-------------|-----|---------|-----------|
| `AGG_TOP_SERVICES` | Services les plus utilisÃ©s | Liste ordonnÃ©e `[(service_id, open_count, total_duration)]` â€” top 5, pÃ©riode 7 jours | Mis Ã  jour Ã  chaque session |
| `AGG_NEGLECTED_SERVICES` | Services dÃ©laissÃ©s | Liste `[(service_id, days_since_last_open)]` â€” services non ouverts depuis > 14 jours | Mis Ã  jour quotidiennement |
| `AGG_FAVORITE_SERVICE` | Service favori | `service_id`, `open_count_week`, `total_duration_week` â€” service le plus frÃ©quentÃ© sur 7 jours | Mis Ã  jour Ã  chaque session |
| `AGG_FAVORITE_TAB` | Onglet favori | `tab_id` (Salon / BibliothÃ¨que / Webway), `usage_ratio` | Mis Ã  jour Ã  chaque session |

### 2.3 AgrÃ©gats sociaux (amis)

| Identifiant | Nom | Contenu | FraÃ®cheur |
|-------------|-----|---------|-----------|
| `AGG_FRIEND_REMINDERS` | Rappels d'amis | Liste `[(friend_cog_id, days_since_last_interaction)]` â€” amis non contactÃ©s depuis > 7 jours | Mis Ã  jour quotidiennement |
| `AGG_TOP_FRIENDS` | Amis les plus contactÃ©s | Liste ordonnÃ©e `[(friend_cog_id, total_duration_minutes)]` â€” top 3, pÃ©riode 30 jours | Mis Ã  jour hebdomadairement |
| `AGG_SOCIAL_ACTIVITY` | ActivitÃ© sociale | `distinct_friends_contacted_week`, `total_social_time_week` | Mis Ã  jour Ã  chaque session |

### 2.4 AgrÃ©gats d'activitÃ©

| Identifiant | Nom | Contenu | FraÃ®cheur |
|-------------|-----|---------|-----------|
| `AGG_ACTIVITY_LEVEL` | Niveau d'activitÃ© | `level` (inactive / low / moderate / active / very_active), `sessions_week`, `total_duration_week` | Mis Ã  jour Ã  chaque session |
| `AGG_CURRENT_SESSION` | Session en cours | `duration_current_session`, `services_opened_count` | Temps rÃ©el |

### 2.5 AgrÃ©gats de jalons (milestones)

| Identifiant | Nom | Contenu | FraÃ®cheur |
|-------------|-----|---------|-----------|
| `AGG_MILESTONES` | Jalons et badges | Liste `[(milestone_type, value, timestamp_achieved)]` â€” streaks, badges, premiÃ¨res fois | Mis Ã  jour Ã  la dÃ©tection |
| `AGG_NEW_MILESTONE` | Nouveau jalon | `milestone_type`, `value` â€” non-null seulement si un jalon vient d'Ãªtre atteint dans la session en cours | Session courante |

---

## 3. Contrat de consommation

### 3.1 API conceptuelle

Miou accÃ¨de aux agrÃ©gats via une interface de lecture simple, gouvernÃ©e par les Cores :

```
MiouContext::get_aggregate(aggregate_id: &str) -> Option<Aggregate>
MiouContext::get_all_aggregates() -> Vec<Aggregate>
MiouContext::has_data() -> bool  // true si MiyukiniWatch a des donnÃ©es
MiouContext::is_collecting() -> bool  // true si la collecte est active
```

### 3.2 RÃ¨gles de consommation

| RÃ¨gle | Description |
|-------|-------------|
| **Lecture seule** | Miou ne peut pas modifier, effacer ou Ã©crire dans MiyukiniWatch. |
| **Graceful degradation** | Si un agrÃ©gat n'est pas disponible (donnÃ©es effacÃ©es, collecte dÃ©sactivÃ©e, catÃ©gorie dÃ©sactivÃ©e), Miou utilise un message gÃ©nÃ©rique. Pas d'erreur, pas de crash. |
| **Pas de mise en cache longue** | Miou ne met pas en cache les agrÃ©gats au-delÃ  de la session. Ã€ chaque nouvelle session, les agrÃ©gats sont rechargÃ©s depuis MiyukiniWatch. |
| **RÃ©solution des pseudos** | L'identifiant technique `friend_cog_id` est rÃ©solu en pseudo lisible par le service de contacts (Jay1Tribu), pas par MiyukiniWatch ni par Miou directement. |
| **Pas de dÃ©pendance forte** | Si MiyukiniWatch est indisponible (hypothÃ¨se thÃ©orique), Miou continue de fonctionner avec des messages gÃ©nÃ©riques. |

### 3.3 Gouvernance de l'accÃ¨s

L'accÃ¨s aux agrÃ©gats passe par BondingBrother et les Cores :

```
Miou demande un agrÃ©gat
  â†’ BondingBrother
    â†’ Master Butler : "Miou a-t-il le droit de lire cet agrÃ©gat ?"
    â†’ KindMother : ReadIntent pour l'agrÃ©gat
  â†’ AgrÃ©gat retournÃ© Ã  Miou
```

**Master Butler** maintient une entrÃ©e de permission permanente pour Miou en lecture sur les agrÃ©gats MiyukiniWatch. Cette permission est standard et ne nÃ©cessite pas d'intervention utilisateur.

---

## 4. Utilisation par Miou â€” Exemples de bulles

### 4.1 Matrice agrÃ©gat â†’ bulle

| AgrÃ©gat consommÃ© | Condition | Exemple de bulle Miou |
|-----------------|-----------|----------------------|
| `AGG_SESSION_RETURN` | `days_away >= 3` | Â« Tu n'es pas passÃ© depuis 3 jours â€” tu m'as manquÃ© ! Â» |
| `AGG_SESSION_RETURN` | `days_away == 0` | Â« Content de te revoir aujourd'hui ! Â» |
| `AGG_SESSION_TIME` | `current_time_slot == MORNING` | Â« Bonjour [pseudo] ! Bien dormi ? Â» |
| `AGG_SESSION_TIME` | `current_time_slot == EVENING` | Â« Bonsoir [pseudo], une petite visite ce soir ? Â» |
| `AGG_CURRENT_SESSION` | `duration > 90 min` | Â« Tu es lÃ  depuis un moment â€” une petite pause ? Â» |
| `AGG_FAVORITE_SERVICE` | â€” | Â« Tu reviens souvent sur JayXpose â€” ta vitrine est Ã  jour ? Â» |
| `AGG_NEGLECTED_SERVICES` | `days_since > 21` | Â« Tu n'as pas ouvert JayKoa depuis 3 semaines â€” un Ã©vÃ©nement Ã  rappeler ? Â» |
| `AGG_FRIEND_REMINDERS` | `days_since > 14` | Â« Pense Ã  reprendre contact avec [pseudo], Ã§a fait un moment ! Â» |
| `AGG_TOP_FRIENDS` | â€” | Â« Tu passes beaucoup de temps avec [pseudo] â€” une belle amitiÃ© ! Â» |
| `AGG_ACTIVITY_LEVEL` | `level == "inactive"` | Â« On ne se voit plus trop... reviens quand tu veux ! Â» (aprÃ¨s plusieurs jours) |
| `AGG_NEW_MILESTONE` | `type == "streak", value == 7` | Â« 7 jours d'affilÃ©e avec ton COG â€” bravo ! Â» |
| `AGG_NEW_MILESTONE` | `type == "first_service"` | Â« Premier service installÃ© â€” bienvenue dans l'aventure ! Â» |
| `AGG_FAVORITE_TAB` | `tab == "BibliothÃ¨que"` | Â« Tu passes souvent par la BibliothÃ¨que â€” un service Ã  dÃ©couvrir ? Â» |
| (aucun agrÃ©gat) | `has_data() == false` | Â« Bienvenue dans Miyukini Central ! Â» (message gÃ©nÃ©rique) |
| (collecte off) | `is_collecting() == false` | Messages gÃ©nÃ©riques (bienvenue, heure de la journÃ©e). |

### 4.2 Priorisation

Lorsque plusieurs bulles sont possibles, Miou applique une **file de prioritÃ©** :

| PrioritÃ© | Type de bulle | Exemples |
|----------|---------------|----------|
| **P0** | Accueil / retour | Bonjour/bonsoir, Â« Tu m'as manquÃ© Â». |
| **P1** | Jalons et badges | Streak, premier service, rÃ©seau connectÃ©. |
| **P2** | Rappels utiles | Ami non contactÃ©, service dÃ©laissÃ©. |
| **P3** | Suggestions lÃ©gÃ¨res | Service favori, onglet habituel. |
| **P4** | Contexte ambiant | DurÃ©e de session, activitÃ©. |

En une session, Miou affiche **1 Ã  3 bulles maximum** (configurable). Les bulles de prioritÃ© supÃ©rieure passent en premier.

### 4.3 Cooldowns et non-rÃ©pÃ©tition

| RÃ¨gle | Description |
|-------|-------------|
| **Pas de rÃ©pÃ©tition immÃ©diate** | Un mÃªme type de bulle n'est pas affichÃ© deux fois dans la mÃªme session. |
| **Cooldown par agrÃ©gat** | Chaque type d'agrÃ©gat a un cooldown (ex. `AGG_FRIEND_REMINDERS` : 3 jours entre deux rappels pour le mÃªme ami). |
| **Cooldown session** | AprÃ¨s le message d'accueil, un dÃ©lai minimum avant la bulle suivante (ex. 5 minutes). |
| **Respecter le dismiss** | Si l'utilisateur ferme une bulle, ne pas reproposer le mÃªme contenu dans la session. |

---

## 5. Comportement en l'absence de donnÃ©es

### 5.1 DÃ©gradation gracieuse

| ScÃ©nario | Comportement MiyukiniWatch | Comportement Miou |
|----------|---------------------------|-------------------|
| **Collecte active, donnÃ©es fraÃ®ches** | AgrÃ©gats complets et Ã  jour. | Bulles personnalisÃ©es, rappels, suggestions. |
| **Collecte active, peu de donnÃ©es** (dÃ©but) | AgrÃ©gats partiels ou vides. | Messages d'accueil gÃ©nÃ©riques + Â« Reviens dans quelques jours pour que je te connaisse mieux. Â» |
| **Collecte active, donnÃ©es effacÃ©es** | AgrÃ©gats vides ; nouvelles donnÃ©es commencent Ã  s'accumuler. | Messages gÃ©nÃ©riques ; personnalisation progressive. |
| **Collecte dÃ©sactivÃ©e, donnÃ©es prÃ©sentes** | AgrÃ©gats existants (stables, non mis Ã  jour). | Bulles basÃ©es sur les anciens agrÃ©gats (qui vieillissent). |
| **Collecte dÃ©sactivÃ©e, donnÃ©es effacÃ©es** | Rien. | Messages entiÃ¨rement gÃ©nÃ©riques (bienvenue, heure). |

### 5.2 Messages gÃ©nÃ©riques de repli

Miou dispose d'un ensemble de messages gÃ©nÃ©riques (non basÃ©s sur MiyukiniWatch) pour chaque crÃ©neau :

| CrÃ©neau | Exemple de message gÃ©nÃ©rique |
|---------|------------------------------|
| Matin | Â« Bonjour [pseudo] ! Qu'est-ce qu'on fait aujourd'hui ? Â» |
| AprÃ¨s-midi | Â« Bon aprÃ¨s-midi ! Un tour dans les services ? Â» |
| Soir | Â« Bonsoir [pseudo], content de te voir ! Â» |
| Nuit | Â« Encore debout ? N'oublie pas de te reposer. Â» |
| Retour (sans donnÃ©e de jours) | Â« Bienvenue dans Miyukini Central ! Â» |

---

## 6. Cycle de vie des agrÃ©gats

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”     â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  MiyukiniWatch   â”‚     â”‚  Aggregator        â”‚     â”‚    Miou      â”‚
â”‚  Collector       â”‚â”€â”€â”€â”€â–¶â”‚  (agrÃ¨ge)          â”‚â”€â”€â”€â”€â–¶â”‚  (consomme)  â”‚
â”‚  (mÃ©triques)     â”‚     â”‚                     â”‚     â”‚  (bulles)    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜     â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜     â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
       â”‚                          â”‚                       â”‚
       â”‚  WriteIntent             â”‚  Stockage agrÃ©gats    â”‚  ReadIntent
       â”‚  â†’ KindMother            â”‚  â†’ KindMother         â”‚  â†’ KindMother
       â–¼                          â–¼                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    KindMother (persistance)                      â”‚
â”‚  miyukiniwatch_metrics â”‚ miyukiniwatch_daily â”‚ miyukiniwatch_aggâ”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 6.1 FraÃ®cheur et mise Ã  jour

| Moment | Action Aggregator | AgrÃ©gats mis Ã  jour |
|--------|-------------------|---------------------|
| **Connexion (dÃ©but de session)** | Calcul `AGG_SESSION_RETURN`, `AGG_SESSION_TIME` | Session, retour, heure |
| **Pendant la session** | Mise Ã  jour `AGG_CURRENT_SESSION` (pÃ©riodique) | DurÃ©e session en cours |
| **Ouverture d'un service** | Recalcul `AGG_TOP_SERVICES`, `AGG_FAVORITE_SERVICE` | Services |
| **Interaction sociale** | Recalcul `AGG_FRIEND_REMINDERS`, `AGG_TOP_FRIENDS` | Amis |
| **DÃ©tection de jalon** | Ajout Ã  `AGG_MILESTONES`, publication `AGG_NEW_MILESTONE` | Jalons |
| **Fin de session** | AgrÃ©gation complÃ¨te de la session | Tous les agrÃ©gats de session |
| **Quotidien** | AgrÃ©gation quotidienne | AgrÃ©gats quotidiens |
| **Hebdomadaire** | AgrÃ©gation hebdomadaire | AgrÃ©gats hebdomadaires |

---

## 7. Invariants d'intÃ©gration

| # | Invariant |
|---|-----------|
| **INT-1** | Miou ne reÃ§oit que des agrÃ©gats, jamais des donnÃ©es brutes. |
| **INT-2** | Miou ne peut pas modifier les donnÃ©es MiyukiniWatch. |
| **INT-3** | Si MiyukiniWatch n'a pas de donnÃ©es, Miou fonctionne en mode gÃ©nÃ©rique. |
| **INT-4** | Miou ne stocke pas les agrÃ©gats MiyukiniWatch au-delÃ  de la session. |
| **INT-5** | L'effacement des donnÃ©es par l'utilisateur est reflÃ©tÃ© immÃ©diatement dans les agrÃ©gats. |
| **INT-6** | La dÃ©sactivation de la collecte par catÃ©gorie rend les agrÃ©gats correspondants indisponibles. |
| **INT-7** | Le contrat d'agrÃ©gats est versionnÃ© avec l'environnement COG (LOI-7). |

---

## 8. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyukiniWatch â€” Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Lien fondateur MiyukiniWatch â†’ Miou (section 5). |
| [MiyukiniWatch â€” SpÃ©cification Fonctionnelle : MÃ©triques et Collecte](./MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Catalogue des mÃ©triques sources, format des agrÃ©gats. |
| [Miyukini Central â€” Miou, avatar, bulles et rÃ´le](..//..//_index.md) | RÃ´le de Miou, canal de communication (bulles). |
| [Miou â€” Index du sous-service](../MiyukiniCentral/Miou/_index.md) | Architecture Miou, documents associÃ©s. |
| [Miou â€” Moteur de GÃ©nÃ©ration Templates et LLM](../MiyukiniCentral/Miou/Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) | Architecture hybride de gÃ©nÃ©ration des bulles. |
| [Miyukini Central â€” Salon propositions](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md) | Utilisation des mÃ©triques dans le Salon. |

---

**Document** : MiyukiniWatch â€” IntÃ©gration Miou et AgrÃ©gats  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : SpÃ©cification fonctionnelle normative â€” contrat entre MiyukiniWatch et Miou

