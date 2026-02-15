# MiyukiniWatch — Spécification Fonctionnelle : Métriques et Collecte

## Contexte

**MiyukiniWatch** collecte des métriques d'usage silencieusement en arrière-plan pour alimenter **Miou** (avatar/mascotte des COGs) et les suggestions du Salon. Ce document décrit exhaustivement les métriques collectées, les événements déclencheurs, les structures de données, les règles d'agrégation et les limites strictes de la collecte.

## Portée / Scope

- **Applicable à :** Spécification des métriques, événements, structures de données, agrégation.
- **Audience :** Développeurs, architectes, équipes produit.
- **Statut :** Spécification fonctionnelle normative.

### Hors périmètre

- Implémentation technique (code, API, schéma de base de données).
- Choix de chiffrement au repos (voir [Sécurité et Conformité](./MiyukiniWatch%20-%20Securite%20et%20Conformite.md)).
- Interface utilisateur (voir [Interface Utilisateur et Écrans](./MiyukiniWatch%20-%20Interface%20Utilisateur%20et%20Ecrans.md)).

---

## 1. Invariant fondamental : pas de lecture de contenus

Avant toute description de métrique, cet invariant est rappelé et s'applique sans exception :

> **MiyukiniWatch ne lit jamais le contenu des messages, des champs saisis, des fichiers ou des pages.**

La collecte se limite à quatre dimensions :

| Dimension | Ce qui est enregistré | Ce qui n'est JAMAIS enregistré |
|-----------|----------------------|-------------------------------|
| **Quand** | Horodatage, durée, fréquence | — |
| **Où** | Identifiant du service, onglet, écran | Contenu affiché, texte, images |
| **Qui** | Identifiant technique du contact (pour les amis) | Pseudo, messages échangés, contenu des discussions |
| **Combien** | Nombre de clics, durée de session, compteurs | Cible des clics (URL, bouton exact), texte saisi |

---

## 2. Catalogue des métriques

### 2.1 Sessions

| ID | Métrique | Événement déclencheur | Données enregistrées | Granularité | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **S-01** | Début de session | Connexion à Central (login réussi) | `session_id`, `timestamp_start` | Par session | Adaptation du message d'accueil. |
| **S-02** | Fin de session | Déconnexion ou fermeture de Central | `session_id`, `timestamp_end` | Par session | Calcul de la durée ; détection d'absence prolongée. |
| **S-03** | Durée de session | Calculée à partir de S-01 et S-02 | `session_id`, `duration_seconds` | Par session | Suggestions de pause après X minutes ; habitudes. |
| **S-04** | Heure de connexion | Dérivée de S-01 | `session_id`, `time_slot` (matin/après-midi/soir/nuit) | Par session | Adaptation du ton (bonjour/bonsoir) et des suggestions. |
| **S-05** | Jours depuis dernière session | Calculée à chaque nouvelle session | `days_since_last_session` | Global | « Tu n'es pas passé depuis X jours. » |
| **S-06** | Compteur de sessions | Incrémenté à chaque S-01 | `total_sessions_count` | Global | Badges de fidélité ; « 7 jours avec ton COG ». |
| **S-07** | Jours actifs consécutifs | Calculé à chaque session | `consecutive_active_days` | Global | Gamification : « Streak » de jours actifs. |

#### 2.1.1 Définition des tranches horaires

| Tranche | Plage | Identifiant |
|---------|-------|-------------|
| Matin | 06:00 – 11:59 | `MORNING` |
| Après-midi | 12:00 – 17:59 | `AFTERNOON` |
| Soir | 18:00 – 22:59 | `EVENING` |
| Nuit | 23:00 – 05:59 | `NIGHT` |

Les horodatages sont **locaux au COG** (pas de temps global requis — LOI-4).

---

### 2.2 Services utilisés

| ID | Métrique | Événement déclencheur | Données enregistrées | Granularité | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **SV-01** | Service ouvert | Navigation vers un service dans Central | `service_id`, `timestamp_open` | Par occurrence | « Tu reviens souvent sur [service]. » |
| **SV-02** | Service fermé / quitté | Navigation hors du service ou fermeture | `service_id`, `timestamp_close` | Par occurrence | Calcul du temps passé. |
| **SV-03** | Temps passé par service | Calculé à partir de SV-01 et SV-02 | `service_id`, `duration_seconds` | Par occurrence | Comprendre où l'utilisateur passe le plus de temps. |
| **SV-04** | Fréquence par service (jour) | Agrégat quotidien de SV-01 | `service_id`, `date`, `open_count` | Par jour par service | Classement des services les plus utilisés. |
| **SV-05** | Fréquence par service (semaine) | Agrégat hebdomadaire | `service_id`, `week`, `open_count` | Par semaine par service | Tendances sur la semaine. |
| **SV-06** | Dernier accès par service | Dérivé de SV-01 | `service_id`, `last_open_timestamp` | Par service | « Tu n'as pas ouvert [service] depuis un moment. » |
| **SV-07** | Onglet principal | Navigation entre onglets de Central (Salon, Bibliothèque, Webway) | `tab_id`, `timestamp`, `duration_seconds` | Par occurrence | « Tu passes souvent par la Bibliothèque. » |

---

### 2.3 Amis et interactions sociales

Ces métriques s'appliquent lorsque des services exposent des notions d'amis ou de contacts (ex. Jay1Tribu). MiyukiniWatch ne lit pas les messages ; il enregistre uniquement des métadonnées de timing et d'identifiant.

| ID | Métrique | Événement déclencheur | Données enregistrées | Granularité | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **A-01** | Ami contacté | Ouverture d'une discussion avec un contact (identifiant technique) | `friend_cog_id`, `timestamp_last_interaction` | Par occurrence | « Tu n'as pas échangé avec [pseudo] depuis X jours. » |
| **A-02** | Temps depuis dernière discussion | Calculé à chaque session à partir de A-01 | `friend_cog_id`, `days_since_last_interaction` | Par ami | Rappels bienveillants de reprise de contact. |
| **A-03** | Durée de discussion par ami | Temps de session où le service de discussion était actif avec ce contact | `friend_cog_id`, `total_duration_seconds` | Agrégat | Classement des relations les plus investies. |
| **A-04** | Classement amis par temps passé | Agrégat dérivé de A-03 | Liste ordonnée `[(friend_cog_id, total_duration)]` | Agrégat global | Miou peut mentionner les relations investies ou délaissées. |
| **A-05** | Nombre d'amis distincts contactés (période) | Agrégat de A-01 sur une période | `period`, `distinct_friends_count` | Par période | Indicateur de vie sociale. |

**Note sur les identifiants :** Seul l'identifiant technique du COG ami (`friend_cog_id`) est stocké. La résolution vers un pseudo lisible est effectuée par le service de contacts (Jay1Tribu ou autre) au moment de l'affichage, pas par MiyukiniWatch.

---

### 2.4 Interactions génériques

| ID | Métrique | Événement déclencheur | Données enregistrées | Granularité | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **I-01** | Nombre de clics (global) | Clic dans l'interface Central | `date`, `click_count` | Agrégat quotidien | Indicateur d'activité générale. |
| **I-02** | Nombre de clics par service | Clic dans un service spécifique | `service_id`, `date`, `click_count` | Agrégat quotidien par service | Indicateur d'activité par service (optionnel). |

**Limite stricte :** Aucun traçage de la cible du clic (pas de coordonnées, pas d'identifiant de bouton, pas d'URL). Le compteur est un agrégat brut.

---

### 2.5 Événements de cycle de vie

| ID | Métrique | Événement déclencheur | Données enregistrées | Granularité | Usage Miou |
|----|----------|----------------------|----------------------|-------------|------------|
| **L-01** | Rite d'Entrée effectué | Rite d'Entrée terminé (première fois) | `timestamp_rite`, `is_first` | Unique | « Bienvenue dans ton nouveau chez-toi. » |
| **L-02** | Type d'entrée (Rite vs Connexion) | Démarrage de session | `session_id`, `entry_type` (rite / connexion) | Par session | Adapter le message (premier lancement vs retour). |
| **L-03** | Premier service installé | Installation d'un service | `service_id`, `timestamp` | Unique | Badge « Premier service ». |
| **L-04** | Connexion MWS | Connexion au Webway réussie | `timestamp`, `connected` | Par événement | Badge « Réseau connecté » ; suggestions Webway. |

---

## 3. Événements et mécanisme de collecte

### 3.1 Bus d'événements internes

MiyukiniWatch s'abonne aux événements internes du COG via un **bus d'événements local**. Il ne crée pas ses propres événements ; il **consomme** les événements émis par Central et les autres services.

| Source d'événement | Événements consommés | Métriques produites |
|-------------------|---------------------|---------------------|
| Miyukini Central — Auth | `user_logged_in`, `user_logged_out` | S-01, S-02, S-03, S-04, S-05, S-06, S-07, L-02 |
| Miyukini Central — Navigation | `service_opened`, `service_closed`, `tab_changed` | SV-01, SV-02, SV-03, SV-07 |
| Miyukini Central — Rite | `rite_completed` | L-01 |
| Jay1Tribu / Services sociaux | `conversation_opened`, `conversation_closed` | A-01, A-02, A-03 |
| Miyukini Central — UI | `user_click` | I-01, I-02 |
| MWS Participant | `webway_connected`, `webway_disconnected` | L-04 |
| Central — Services | `service_installed` | L-03 |

### 3.2 Règles de collecte

| Règle | Description |
|-------|-------------|
| **Passivité** | MiyukiniWatch ne provoque jamais d'événement. Il ne modifie pas le comportement des autres services. |
| **Atomicité** | Chaque métrique est un WriteIntent atomique vers KindMother. Un échec d'écriture n'affecte pas les autres métriques. |
| **Idempotence** | Un même événement reçu deux fois ne produit pas de doublon (déduplication par `session_id` + `timestamp`). |
| **Priorité basse** | La collecte s'exécute avec une priorité basse ; en cas de contention (Caring Nanny T2+), elle est la première à être réduite ou suspendue. |
| **Aucun blocage** | La collecte ne bloque jamais l'interface utilisateur ni les autres services. L'écriture est asynchrone. |

---

## 4. Agrégation

### 4.1 Niveaux d'agrégation

Les métriques brutes sont agrégées à trois niveaux temporels :

| Niveau | Période | Exemples | Rétention par défaut |
|--------|---------|----------|----------------------|
| **Brut** | Par événement | Chaque ouverture de service avec son horodatage | 30 jours |
| **Quotidien** | Par jour | Nombre d'ouvertures de JayXpose le 14/02/2026 | 90 jours |
| **Hebdomadaire** | Par semaine | Temps total passé dans Jay1Tribu semaine 7 | 365 jours |

### 4.2 Processus d'agrégation

L'**Opérateur MiyukiniWatchAggregator** agrège les métriques selon le calendrier suivant :

| Déclencheur | Action |
|-------------|--------|
| **Fin de session** | Agrégation des métriques de la session (durée, services, clics). |
| **Quotidien** (première session du jour) | Agrégation des données de la veille en résumés quotidiens. Purge des données brutes expirées (> 30 jours par défaut). |
| **Hebdomadaire** (première session de la semaine) | Agrégation des données quotidiennes de la semaine écoulée en résumés hebdomadaires. Purge des données quotidiennes expirées (> 90 jours par défaut). |
| **À la demande** | L'utilisateur ou Miou peut demander un agrégat spécifique (ex. « top 3 services cette semaine »). |

### 4.3 Format des agrégats pour Miou

Les agrégats exposés à Miou sont des structures simplifiées ne contenant que les données nécessaires pour formuler des bulles et des suggestions :

| Agrégat | Contenu | Exemple |
|---------|---------|---------|
| `session_summary` | Jours depuis dernière session, durée moyenne, tranche horaire habituelle | `{ days_away: 3, avg_duration: 45min, usual_slot: EVENING }` |
| `top_services` | Top 3 services par fréquence (semaine en cours) | `[{ service: "JayXpose", count: 12 }, ...]` |
| `neglected_services` | Services non ouverts depuis > 14 jours | `[{ service: "JayKoa", days_since: 21 }]` |
| `friend_reminders` | Amis non contactés depuis > 7 jours | `[{ friend_id: "...", days_since: 10 }]` |
| `top_friends` | Top 3 amis par temps de discussion (30 jours) | `[{ friend_id: "...", total_min: 180 }]` |
| `activity_level` | Indicateur global d'activité (clics, sessions, durée) | `{ level: "active", sessions_week: 5 }` |
| `milestones` | Événements remarquables (badges débloqués, streaks) | `[{ type: "streak", value: 7 }]` |

**Règle :** Miou ne reçoit **que les agrégats**, jamais les données brutes. Si l'utilisateur efface ses données, les agrégats sont recalculés à partir des données restantes (ou deviennent vides).

---

## 5. Structures de données conceptuelles

### 5.1 Métrique brute

```
MetricRecord {
    record_id:    UUID          // Identifiant unique
    metric_id:    String        // Ex. "S-01", "SV-03", "A-01"
    session_id:   UUID          // Session associée
    timestamp:    LocalDateTime // Horodatage local au COG
    service_id:   Option<String>  // Service concerné (si applicable)
    friend_id:    Option<String>  // Ami concerné (si applicable)
    value_int:    Option<i64>   // Valeur numérique (durée, compteur)
    value_str:    Option<String> // Valeur textuelle (tranche horaire, type)
}
```

### 5.2 Agrégat quotidien

```
DailyAggregate {
    date:         LocalDate
    service_id:   Option<String>
    friend_id:    Option<String>
    metric_id:    String
    count:        i64           // Nombre d'occurrences
    total_value:  i64           // Somme (ex. durée totale en secondes)
    min_value:    Option<i64>
    max_value:    Option<i64>
}
```

### 5.3 Agrégat hebdomadaire

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

| Catégorie | Exclusion |
|-----------|-----------|
| **Contenu textuel** | Aucun message, champ de formulaire, texte saisi, requête de recherche. |
| **Contenu médias** | Aucune image, fichier, vidéo, audio consulté ou envoyé. |
| **URL / liens** | Aucune URL visitée (interne ou externe). |
| **Coordonnées de clic** | Aucune position (x, y), aucun identifiant de bouton, aucune cible DOM. |
| **Données biométriques** | Aucune donnée de santé, localisation GPS, capteur. |
| **Données de profil** | Aucune préférence, paramètre, configuration utilisateur. Le pseudo est résolu au moment de l'affichage par Central, pas stocké par MiyukiniWatch. |
| **Données Inter-COG** | Aucune donnée reçue d'un autre COG (messages, fichiers). |
| **Données MWS** | Aucun log de connexion MWS au-delà du simple booléen « connecté/déconnecté ». |

### 6.2 Limites de volumétrie

| Paramètre | Valeur par défaut | Configurable |
|-----------|-------------------|--------------|
| Métriques brutes max par session | 10 000 | Oui (via TAMR) |
| Taille totale stockage MiyukiniWatch | 50 Mo | Oui (via TAMR) |
| Agrégats quotidiens max conservés | 90 jours | Oui (via préférence utilisateur) |
| Agrégats hebdomadaires max conservés | 365 jours | Oui (via préférence utilisateur) |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Vision et principe, liste initiale des métriques. |
| [MiyukiniWatch — Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Règles de rétention, purge, effacement. |
| [MiyukiniWatch — Intégration Miou et Agrégats](./MiyukiniWatch%20-%20Integration%20Miou%20et%20Agregats.md) | Format et contrat des agrégats consommés par Miou. |
| [MiyukiniWatch — Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Règles non négociables. |
| [Miyukini Central — Miou, avatar, bulles et rôle](../MiyukiniCentral/Miyukini%20Central%20-%20Miou%20avatar%20bulles%20et%20role.md) | Consommation des agrégats par Miou. |

---

**Document** : MiyukiniWatch — Spécification Fonctionnelle : Métriques et Collecte  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Spécification fonctionnelle normative
