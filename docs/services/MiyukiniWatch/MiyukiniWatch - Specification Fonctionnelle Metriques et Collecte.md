# MiyukiniWatch â€” SpÃ©cification Fonctionnelle : MÃ©triques et Collecte

## Contexte

**MiyukiniWatch** collecte des mÃ©triques d'usage silencieusement en arriÃ¨re-plan pour alimenter **Miou** (avatar/mascotte des COGs) et les suggestions du Salon. Ce document dÃ©crit exhaustivement les mÃ©triques collectÃ©es, les Ã©vÃ©nements dÃ©clencheurs, les structures de donnÃ©es, les rÃ¨gles d'agrÃ©gation et les limites strictes de la collecte.

## PortÃ©e / Scope

- **Applicable Ã  :** SpÃ©cification des mÃ©triques, Ã©vÃ©nements, structures de donnÃ©es, agrÃ©gation.
- **Audience :** DÃ©veloppeurs, architectes, Ã©quipes produit.
- **Statut :** SpÃ©cification fonctionnelle normative.

### Hors pÃ©rimÃ¨tre

- ImplÃ©mentation technique (code, API, schÃ©ma de base de donnÃ©es).
- Choix de chiffrement au repos (voir [SÃ©curitÃ© et ConformitÃ©](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md)).
- Interface utilisateur (voir [Interface Utilisateur et Ã‰crans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md)).

---

## 1. Invariant fondamental : pas de lecture de contenus

Avant toute description de mÃ©trique, cet invariant est rappelÃ© et s'applique sans exception :

> **MiyukiniWatch ne lit jamais le contenu des messages, des champs saisis, des fichiers ou des pages.**

La collecte se limite Ã  quatre dimensions :

| Dimension | Ce qui est enregistrÃ© | Ce qui n'est JAMAIS enregistrÃ© |
|-----------|----------------------|-------------------------------|
| **Quand** | Horodatage, durÃ©e, frÃ©quence | â€” |
| **OÃ¹** | Identifiant du service, onglet, Ã©cran | Contenu affichÃ©, texte, images |
| **Qui** | Identifiant technique du contact (pour les amis) | Pseudo, messages Ã©changÃ©s, contenu des discussions |
| **Combien** | Nombre de clics, durÃ©e de session, compteurs | Cible des clics (URL, bouton exact), texte saisi |

---

## 2. Catalogue des mÃ©triques

### 2.1 Sessions

| ID | MÃ©trique | Ã‰vÃ©nement dÃ©clencheur | DonnÃ©es enregistrÃ©es | GranularitÃ© | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **S-01** | DÃ©but de session | Connexion Ã  Central (login rÃ©ussi) | `session_id`, `timestamp_start` | Par session | Adaptation du message d'accueil. |
| **S-02** | Fin de session | DÃ©connexion ou fermeture de Central | `session_id`, `timestamp_end` | Par session | Calcul de la durÃ©e ; dÃ©tection d'absence prolongÃ©e. |
| **S-03** | DurÃ©e de session | CalculÃ©e Ã  partir de S-01 et S-02 | `session_id`, `duration_seconds` | Par session | Suggestions de pause aprÃ¨s X minutes ; habitudes. |
| **S-04** | Heure de connexion | DÃ©rivÃ©e de S-01 | `session_id`, `time_slot` (matin/aprÃ¨s-midi/soir/nuit) | Par session | Adaptation du ton (bonjour/bonsoir) et des suggestions. |
| **S-05** | Jours depuis derniÃ¨re session | CalculÃ©e Ã  chaque nouvelle session | `days_since_last_session` | Global | Â« Tu n'es pas passÃ© depuis X jours. Â» |
| **S-06** | Compteur de sessions | IncrÃ©mentÃ© Ã  chaque S-01 | `total_sessions_count` | Global | Badges de fidÃ©litÃ© ; Â« 7 jours avec ton COG Â». |
| **S-07** | Jours actifs consÃ©cutifs | CalculÃ© Ã  chaque session | `consecutive_active_days` | Global | Gamification : Â« Streak Â» de jours actifs. |

#### 2.1.1 DÃ©finition des tranches horaires

| Tranche | Plage | Identifiant |
|---------|-------|-------------|
| Matin | 06:00 â€“ 11:59 | `MORNING` |
| AprÃ¨s-midi | 12:00 â€“ 17:59 | `AFTERNOON` |
| Soir | 18:00 â€“ 22:59 | `EVENING` |
| Nuit | 23:00 â€“ 05:59 | `NIGHT` |

Les horodatages sont **locaux au COG** (pas de temps global requis â€” LOI-4).

---

### 2.2 Services utilisÃ©s

| ID | MÃ©trique | Ã‰vÃ©nement dÃ©clencheur | DonnÃ©es enregistrÃ©es | GranularitÃ© | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **SV-01** | Service ouvert | Navigation vers un service dans Central | `service_id`, `timestamp_open` | Par occurrence | Â« Tu reviens souvent sur [service]. Â» |
| **SV-02** | Service fermÃ© / quittÃ© | Navigation hors du service ou fermeture | `service_id`, `timestamp_close` | Par occurrence | Calcul du temps passÃ©. |
| **SV-03** | Temps passÃ© par service | CalculÃ© Ã  partir de SV-01 et SV-02 | `service_id`, `duration_seconds` | Par occurrence | Comprendre oÃ¹ l'utilisateur passe le plus de temps. |
| **SV-04** | FrÃ©quence par service (jour) | AgrÃ©gat quotidien de SV-01 | `service_id`, `date`, `open_count` | Par jour par service | Classement des services les plus utilisÃ©s. |
| **SV-05** | FrÃ©quence par service (semaine) | AgrÃ©gat hebdomadaire | `service_id`, `week`, `open_count` | Par semaine par service | Tendances sur la semaine. |
| **SV-06** | Dernier accÃ¨s par service | DÃ©rivÃ© de SV-01 | `service_id`, `last_open_timestamp` | Par service | Â« Tu n'as pas ouvert [service] depuis un moment. Â» |
| **SV-07** | Onglet principal | Navigation entre onglets de Central (Salon, BibliothÃ¨que, Webway) | `tab_id`, `timestamp`, `duration_seconds` | Par occurrence | Â« Tu passes souvent par la BibliothÃ¨que. Â» |

---

### 2.3 Amis et interactions sociales

Ces mÃ©triques s'appliquent lorsque des services exposent des notions d'amis ou de contacts (ex. Jay1Tribu). MiyukiniWatch ne lit pas les messages ; il enregistre uniquement des mÃ©tadonnÃ©es de timing et d'identifiant.

| ID | MÃ©trique | Ã‰vÃ©nement dÃ©clencheur | DonnÃ©es enregistrÃ©es | GranularitÃ© | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **A-01** | Ami contactÃ© | Ouverture d'une discussion avec un contact (identifiant technique) | `friend_cog_id`, `timestamp_last_interaction` | Par occurrence | Â« Tu n'as pas Ã©changÃ© avec [pseudo] depuis X jours. Â» |
| **A-02** | Temps depuis derniÃ¨re discussion | CalculÃ© Ã  chaque session Ã  partir de A-01 | `friend_cog_id`, `days_since_last_interaction` | Par ami | Rappels bienveillants de reprise de contact. |
| **A-03** | DurÃ©e de discussion par ami | Temps de session oÃ¹ le service de discussion Ã©tait actif avec ce contact | `friend_cog_id`, `total_duration_seconds` | AgrÃ©gat | Classement des relations les plus investies. |
| **A-04** | Classement amis par temps passÃ© | AgrÃ©gat dÃ©rivÃ© de A-03 | Liste ordonnÃ©e `[(friend_cog_id, total_duration)]` | AgrÃ©gat global | Miou peut mentionner les relations investies ou dÃ©laissÃ©es. |
| **A-05** | Nombre d'amis distincts contactÃ©s (pÃ©riode) | AgrÃ©gat de A-01 sur une pÃ©riode | `period`, `distinct_friends_count` | Par pÃ©riode | Indicateur de vie sociale. |

**Note sur les identifiants :** Seul l'identifiant technique du COG ami (`friend_cog_id`) est stockÃ©. La rÃ©solution vers un pseudo lisible est effectuÃ©e par le service de contacts (Jay1Tribu ou autre) au moment de l'affichage, pas par MiyukiniWatch.

---

### 2.4 Interactions gÃ©nÃ©riques

| ID | MÃ©trique | Ã‰vÃ©nement dÃ©clencheur | DonnÃ©es enregistrÃ©es | GranularitÃ© | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **I-01** | Nombre de clics (global) | Clic dans l'interface Central | `date`, `click_count` | AgrÃ©gat quotidien | Indicateur d'activitÃ© gÃ©nÃ©rale. |
| **I-02** | Nombre de clics par service | Clic dans un service spÃ©cifique | `service_id`, `date`, `click_count` | AgrÃ©gat quotidien par service | Indicateur d'activitÃ© par service (optionnel). |

**Limite stricte :** Aucun traÃ§age de la cible du clic (pas de coordonnÃ©es, pas d'identifiant de bouton, pas d'URL). Le compteur est un agrÃ©gat brut.

---

### 2.5 Ã‰vÃ©nements de cycle de vie

| ID | MÃ©trique | Ã‰vÃ©nement dÃ©clencheur | DonnÃ©es enregistrÃ©es | GranularitÃ© | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **L-01** | Rite d'EntrÃ©e effectuÃ© | Rite d'EntrÃ©e terminÃ© (premiÃ¨re fois) | `timestamp_rite`, `is_first` | Unique | Â« Bienvenue dans ton nouveau chez-toi. Â» |
| **L-02** | Type d'entrÃ©e (Rite vs Connexion) | DÃ©marrage de session | `session_id`, `entry_type` (rite / connexion) | Par session | Adapter le message (premier lancement vs retour). |
| **L-03** | Premier service installÃ© | Installation d'un service | `service_id`, `timestamp` | Unique | Badge Â« Premier service Â». |
| **L-04** | Connexion MWS | Connexion au Webway rÃ©ussie | `timestamp`, `connected` | Par Ã©vÃ©nement | Badge Â« RÃ©seau connectÃ© Â» ; suggestions Webway. |

---

## 3. Ã‰vÃ©nements et mÃ©canisme de collecte

### 3.1 Bus d'Ã©vÃ©nements internes

MiyukiniWatch s'abonne aux Ã©vÃ©nements internes du COG via un **bus d'Ã©vÃ©nements local**. Il ne crÃ©e pas ses propres Ã©vÃ©nements ; il **consomme** les Ã©vÃ©nements Ã©mis par Central et les autres services.

| Source d'Ã©vÃ©nement | Ã‰vÃ©nements consommÃ©s | MÃ©triques produites |
|-------------------|---------------------|---------------------|
| Miyukini Central â€” Auth | `user_logged_in`, `user_logged_out` | S-01, S-02, S-03, S-04, S-05, S-06, S-07, L-02 |
| Miyukini Central â€” Navigation | `service_opened`, `service_closed`, `tab_changed` | SV-01, SV-02, SV-03, SV-07 |
| Miyukini Central â€” Rite | `rite_completed` | L-01 |
| Jay1Tribu / Services sociaux | `conversation_opened`, `conversation_closed` | A-01, A-02, A-03 |
| Miyukini Central â€” UI | `user_click` | I-01, I-02 |
| MWS Participant | `webway_connected`, `webway_disconnected` | L-04 |
| Central â€” Services | `service_installed` | L-03 |

### 3.2 RÃ¨gles de collecte

| RÃ¨gle | Description |
|-------|-------------|
| **PassivitÃ©** | MiyukiniWatch ne provoque jamais d'Ã©vÃ©nement. Il ne modifie pas le comportement des autres services. |
| **AtomicitÃ©** | Chaque mÃ©trique est un WriteIntent atomique vers KindMother. Un Ã©chec d'Ã©criture n'affecte pas les autres mÃ©triques. |
| **Idempotence** | Un mÃªme Ã©vÃ©nement reÃ§u deux fois ne produit pas de doublon (dÃ©duplication par `session_id` + `timestamp`). |
| **PrioritÃ© basse** | La collecte s'exÃ©cute avec une prioritÃ© basse ; en cas de contention (Caring Nanny T2+), elle est la premiÃ¨re Ã  Ãªtre rÃ©duite ou suspendue. |
| **Aucun blocage** | La collecte ne bloque jamais l'interface utilisateur ni les autres services. L'Ã©criture est asynchrone. |

---

## 4. AgrÃ©gation

### 4.1 Niveaux d'agrÃ©gation

Les mÃ©triques brutes sont agrÃ©gÃ©es Ã  trois niveaux temporels :

| Niveau | PÃ©riode | Exemples | RÃ©tention par dÃ©faut |
|--------|---------|----------|----------------------|
| **Brut** | Par Ã©vÃ©nement | Chaque ouverture de service avec son horodatage | 30 jours |
| **Quotidien** | Par jour | Nombre d'ouvertures de JayXpose le 14/02/2026 | 90 jours |
| **Hebdomadaire** | Par semaine | Temps total passÃ© dans Jay1Tribu semaine 7 | 365 jours |

### 4.2 Processus d'agrÃ©gation

L'**OpÃ©rateur MiyukiniWatchAggregator** agrÃ¨ge les mÃ©triques selon le calendrier suivant :

| DÃ©clencheur | Action |
|-------------|--------|
| **Fin de session** | AgrÃ©gation des mÃ©triques de la session (durÃ©e, services, clics). |
| **Quotidien** (premiÃ¨re session du jour) | AgrÃ©gation des donnÃ©es de la veille en rÃ©sumÃ©s quotidiens. Purge des donnÃ©es brutes expirÃ©es (> 30 jours par dÃ©faut). |
| **Hebdomadaire** (premiÃ¨re session de la semaine) | AgrÃ©gation des donnÃ©es quotidiennes de la semaine Ã©coulÃ©e en rÃ©sumÃ©s hebdomadaires. Purge des donnÃ©es quotidiennes expirÃ©es (> 90 jours par dÃ©faut). |
| **Ã€ la demande** | L'utilisateur ou Miou peut demander un agrÃ©gat spÃ©cifique (ex. Â« top 3 services cette semaine Â»). |

### 4.3 Format des agrÃ©gats pour Miou

Les agrÃ©gats exposÃ©s Ã  Miou sont des structures simplifiÃ©es ne contenant que les donnÃ©es nÃ©cessaires pour formuler des bulles et des suggestions :

| AgrÃ©gat | Contenu | Exemple |
|---------|---------|---------|
| `session_summary` | Jours depuis derniÃ¨re session, durÃ©e moyenne, tranche horaire habituelle | `{ days_away: 3, avg_duration: 45min, usual_slot: EVENING }` |
| `top_services` | Top 3 services par frÃ©quence (semaine en cours) | `[{ service: "JayXpose", count: 12 }, ...]` |
| `neglected_services` | Services non ouverts depuis > 14 jours | `[{ service: "JayKoa", days_since: 21 }]` |
| `friend_reminders` | Amis non contactÃ©s depuis > 7 jours | `[{ friend_id: "...", days_since: 10 }]` |
| `top_friends` | Top 3 amis par temps de discussion (30 jours) | `[{ friend_id: "...", total_min: 180 }]` |
| `activity_level` | Indicateur global d'activitÃ© (clics, sessions, durÃ©e) | `{ level: "active", sessions_week: 5 }` |
| `milestones` | Ã‰vÃ©nements remarquables (badges dÃ©bloquÃ©s, streaks) | `[{ type: "streak", value: 7 }]` |

**RÃ¨gle :** Miou ne reÃ§oit **que les agrÃ©gats**, jamais les donnÃ©es brutes. Si l'utilisateur efface ses donnÃ©es, les agrÃ©gats sont recalculÃ©s Ã  partir des donnÃ©es restantes (ou deviennent vides).

---

## 5. Structures de donnÃ©es conceptuelles

### 5.1 MÃ©trique brute

```
MetricRecord {
    record_id:    UUID          // Identifiant unique
    metric_id:    String        // Ex. "S-01", "SV-03", "A-01"
    session_id:   UUID          // Session associÃ©e
    timestamp:    LocalDateTime // Horodatage local au COG
    service_id:   Option<String>  // Service concernÃ© (si applicable)
    friend_id:    Option<String>  // Ami concernÃ© (si applicable)
    value_int:    Option<i64>   // Valeur numÃ©rique (durÃ©e, compteur)
    value_str:    Option<String> // Valeur textuelle (tranche horaire, type)
}
```

### 5.2 AgrÃ©gat quotidien

```
DailyAggregate {
    date:         LocalDate
    service_id:   Option<String>
    friend_id:    Option<String>
    metric_id:    String
    count:        i64           // Nombre d'occurrences
    total_value:  i64           // Somme (ex. durÃ©e totale en secondes)
    min_value:    Option<i64>
    max_value:    Option<i64>
}
```

### 5.3 AgrÃ©gat hebdomadaire

```
WeeklyAggregate {
    year:         i32
    week:         u8
    service_id:   Option<String>
    friend_id:    Option<String>
    metric_id:    String
    count:        i64
    total_value:  i64
    avg_value:    f64
}
```

---

## 6. Limites et exclusions

### 6.1 Ce que MiyukiniWatch NE collecte PAS

| CatÃ©gorie | Exclusion |
|-----------|-----------|
| **Contenu textuel** | Aucun message, champ de formulaire, texte saisi, requÃªte de recherche. |
| **Contenu mÃ©dias** | Aucune image, fichier, vidÃ©o, audio consultÃ© ou envoyÃ©. |
| **URL / liens** | Aucune URL visitÃ©e (interne ou externe). |
| **CoordonnÃ©es de clic** | Aucune position (x, y), aucun identifiant de bouton, aucune cible DOM. |
| **DonnÃ©es biomÃ©triques** | Aucune donnÃ©e de santÃ©, localisation GPS, capteur. |
| **DonnÃ©es de profil** | Aucune prÃ©fÃ©rence, paramÃ¨tre, configuration utilisateur. Le pseudo est rÃ©solu au moment de l'affichage par Central, pas stockÃ© par MiyukiniWatch. |
| **DonnÃ©es Inter-COG** | Aucune donnÃ©e reÃ§ue d'un autre COG (messages, fichiers). |
| **DonnÃ©es MWS** | Aucun log de connexion MWS au-delÃ  du simple boolÃ©en Â« connectÃ©/dÃ©connectÃ© Â». |

### 6.2 Limites de volumÃ©trie

| ParamÃ¨tre | Valeur par dÃ©faut | Configurable |
|-----------|-------------------|--------------|
| MÃ©triques brutes max par session | 10 000 | Oui (via TAMR) |
| Taille totale stockage MiyukiniWatch | 50 Mo | Oui (via TAMR) |
| AgrÃ©gats quotidiens max conservÃ©s | 90 jours | Oui (via prÃ©fÃ©rence utilisateur) |
| AgrÃ©gats hebdomadaires max conservÃ©s | 365 jours | Oui (via prÃ©fÃ©rence utilisateur) |

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyukiniWatch â€” Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision et principe, liste initiale des mÃ©triques. |
| [MiyukiniWatch â€” Gouvernance DonnÃ©es et RÃ©tention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | RÃ¨gles de rÃ©tention, purge, effacement. |
| [MiyukiniWatch â€” IntÃ©gration Miou et AgrÃ©gats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Format et contrat des agrÃ©gats consommÃ©s par Miou. |
| [MiyukiniWatch â€” Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | RÃ¨gles non nÃ©gociables. |
| [Miyukini Central â€” Miou, avatar, bulles et rÃ´le](..//..//_index.md) | Consommation des agrÃ©gats par Miou. |

---

**Document** : MiyukiniWatch â€” SpÃ©cification Fonctionnelle : MÃ©triques et Collecte  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : SpÃ©cification fonctionnelle normative

