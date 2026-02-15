# Bot Miou — Intégration et Flux de Données

Document exhaustif décrivant l'intégration du Bot avec MiyukiniWatch, le profil utilisateur, le contexte applicatif, le format des agrégats, la construction du contexte et les schémas de données.

---

## 1. Vue d'ensemble des sources de données

Le Bot consomme des données provenant de **six sources** principales :

| Source | Type | Données fournies | Fréquence de mise à jour |
|--------|------|------------------|---------------------------|
| **MiyukiniWatch** | Service silencieux | Sessions, services, amis, clics | Temps réel (agrégats) |
| **Profil utilisateur** | Base auth Central | Pseudo, préférences, langue | Au chargement session |
| **Contexte applicatif** | Services (JayKoa, JayXpose, MWS, Jay1Tribu) | Événements, vitrine, connexion, présence | Au chargement + événements |
| **Paramètres Miou** | Préférences | Seuils, fréquence, LLM activé | Au chargement session |
| **Specs machine** | Système (sysinfo) | RAM, stockage, CPU, OS | Au démarrage + événement upgrade |
| **Réponses utilisateur Miou** | Base dédiée chiffrée | Réponses aux questions de curiosité | Enregistrement à la réponse |

**Invariant :** Les réponses utilisateur Miou sont les **seules données lues et enregistrées par Miou** pour son usage. Stockage local chiffré (SQLCipher). Voir [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md).

---

## 2. Intégration avec MiyukiniWatch

### 2.1 Rôle de MiyukiniWatch

MiyukiniWatch est le **fournisseur principal** des métriques d'usage. Il enregistre silencieusement :
- Début et fin de session
- Services ouverts (identifiant + horodatage)
- Amis contactés (identifiant + dernière interaction)
- Nombre de clics (agrégat)

**Invariant :** MiyukiniWatch ne lit jamais le contenu des messages, saisies ou fichiers. Le Bot ne reçoit que des agrégats.

### 2.2 Interface d'abstraction

Le Bot ne doit **pas** accéder directement à la base MiyukiniWatch. Une couche d'abstraction fournit des structures normalisées :

```rust
// Pseudo-structures — à adapter
struct SessionSummary {
    last_session_end: Option<DateTime>,
    days_since_last_visit: Option<u32>,
    current_session_start: Option<DateTime>,
    current_session_duration_minutes: u32,
}

struct ServiceUsage {
    service_id: String,
    service_name: String,  // Nom affiché (ex. "JayXpose")
    last_opened: Option<DateTime>,
    days_since_last_open: Option<u32>,
    minutes_today: u32,
}

struct FriendStatus {
    friend_id: String,
    friend_pseudo: String,
    last_contact: Option<DateTime>,
    days_since_last_contact: Option<u32>,
    total_chat_minutes: u32,  // Agrégat
}
```

### 2.3 Requêtes nécessaires

| Requête | Paramètres | Retour | Usage Bot |
|---------|------------|--------|-----------|
| `get_session_summary(profile_id)` | profile_id | SessionSummary | Jours absence, durée session |
| `get_services_usage(profile_id, period)` | profile_id, "day" ou "week" | Vec<ServiceUsage> | Service délaissé, service top |
| `get_friends_status(profile_id)` | profile_id | Vec<FriendStatus> | Ami le plus délaissé |
| `get_clicks_count(profile_id, period)` | profile_id, "session" | u32 | Optionnel, indicateur activité |

### 2.4 Dérivées calculées par le Constructeur de contexte

À partir des données brutes MiyukiniWatch, le Constructeur calcule :

| Dérivée | Calcul | Utilisation |
|---------|--------|-------------|
| `jours_depuis_derniere_visite` | `(now - last_session_end).days` | Condition retour_absence |
| `session_duration_minutes` | `(now - current_session_start).minutes` | Condition pause_sante |
| `service_delaisse` | `ServiceUsage` avec `days_since_last_open` max parmi services installés | Condition suggestion_service |
| `service_le_plus_utilise` | `ServiceUsage` avec `minutes_today` max | Variable {service_top}, résumé |
| `ami_plus_delaisse` | `FriendStatus` avec `days_since_last_contact` max | Condition rappel_ami |

### 2.5 Gestion de l'absence de MiyukiniWatch

Si MiyukiniWatch est désactivé ou indisponible :

| Donnée | Comportement |
|--------|--------------|
| `jours_depuis_derniere_visite` | None → condition retour_absence fausse |
| `session_duration_minutes` | 0 ou estimation par timer Central |
| `service_delaisse` | None → condition suggestion_service fausse |
| `service_le_plus_utilise` | None → pas de résumé ou valeur par défaut |
| `ami_plus_delaisse` | None → condition rappel_ami fausse |

Le Bot continue de fonctionner avec les données disponibles (profil, contexte applicatif).

---

## 3. Intégration avec le profil utilisateur

### 3.1 Données du profil

| Champ | Type | Usage Bot |
|-------|------|-----------|
| `pseudonyme` | String | Variable {pseudo} |
| `email` | String | Non utilisé (confidentialité) |
| `preferred_language` | String | Sélection du pack de templates |
| `miou_preferences` | Struct | Seuils, fréquence, etc. |

### 3.2 Structure MiouPreferences

```rust
struct MiouPreferences {
    bulles_actives: bool,
    frequence: FrequenceBulles,  // Discret, Normal, Bavard
    seuil_pause_minutes: u32,
    rappels_pause_actives: bool,
    son_bulles: bool,
    llm_actif: bool,
}
```

### 3.3 Valeurs par défaut

| Champ | Défaut |
|-------|--------|
| `bulles_actives` | true |
| `frequence` | Normal |
| `seuil_pause_minutes` | 120 |
| `rappels_pause_actives` | true |
| `son_bulles` | false |
| `llm_actif` | false |

### 3.4 Pseudo manquant

Si `pseudonyme` est vide ou null :
- Utiliser la partie avant `@` de l'email (si disponible).
- Sinon : "toi" ou "habitant".
- Ne jamais afficher l'email dans une bulle.

---

## 4. Intégration avec le contexte applicatif

### 4.1 JayKoa (calendrier)

**Données requises :**
- Prochain événement à venir (titre, date/heure de début).
- Événements dans les 24h (pour rappel).

**Interface :**
```rust
fn get_next_event(profile_id: &str) -> Option<Event> {
    // Retourne l'événement le plus proche dans le futur
}

struct Event {
    id: String,
    title: String,
    start: DateTime,
}
```

**Usage :** Condition rappel_événement si `event.start - now < 1h`.

### 4.2 JayXpose (exposant, vitrine)

**Données requises :**
- `has_exposant_profile(profile_id) -> bool`
- `is_vitrine_published(profile_id) -> bool`

**Usage :** Gamification (badges Exposant actif, Vitrine en ligne). Le Bot ne génère pas de bulles spécifiques JayXpose sauf via les badges.

### 4.3 MWS (Miyukini Webway System)

**Données requises :**
- `is_mws_connected() -> bool`

**Usage :** Badge Webway connecté. Pas de bulle directe MWS sauf via badge.

### 4.4 Jay1Tribu (messagerie, amis)

**Données requises :**
- Liste des amis (identifiant, pseudo) — métadonnées uniquement.
- Présence : qui est en ligne (événement ou polling).
- Dernière discussion par ami (horodatage) — fourni par MiyukiniWatch si intégré.

**Interface :**
```rust
fn get_online_friends(profile_id: &str) -> Vec<String>  // Pseudos
fn get_friends_list(profile_id: &str) -> Vec<Friend>    // Métadonnées
```

**Usage :** Notification ami connecté, rappel ami (via MiyukiniWatch pour le délai).

### 4.5 Indisponibilité d'un service

Si un service (JayKoa, Jay1Tribu, etc.) n'est pas installé ou ne répond pas :
- Retourner None ou valeur par défaut.
- Le Constructeur de contexte gère l'absence.
- Les conditions dépendant de ce service sont fausses.
- Pas d'erreur, pas de crash.

---

### 4.6 Specs machine

**Données requises :**
- `ram_available_mb` (RAM disponible en Mo)
- `disk_free_gb` (espace disque libre en Go)
- `os_type` (Windows, Linux, macOS)
- `specs_previous` (dernière session) pour détecter upgrade

**Interface :**
```rust
struct MachineSpecs {
    ram_available_mb: u32,
    disk_free_gb: f32,
    os_type: String,  // "windows" | "linux" | "macos"
    cpu_cores: Option<u32>,
}

fn get_machine_specs() -> MachineSpecs
fn has_specs_upgraded_since_last_session() -> bool
```

**Usage :** Catégories `specs_ram_demande`, `specs_stockage_demande`, `specs_upgrade_commentaire`, `taquinerie_innocente` (variable {os}).

---

### 4.7 Réponses utilisateur Miou (stockage chiffré)

**Source :** Base dédiée `miou_user_responses` (SQLCipher). **Seule donnée lue et enregistrée par Miou.**

**Flux :**
- Miou affiche une bulle de type `curiosite_utilisateur` avec question et zone de saisie/boutons
- L'utilisateur répond (ou clique « Passer »)
- Si réponse : enregistrement chiffré dans la base
- Le Constructeur charge ces réponses pour personnaliser le contexte

**Interface :**
```rust
fn save_user_response(profile_id: &str, question_type: &str, response_text: &str)
fn get_user_responses(profile_id: &str) -> Vec<UserResponse>
fn clear_user_responses(profile_id: &str)  // Paramètres > « Réinitialiser »
fn get_relation_level(profile_id: &str) -> RelationLevel
fn set_relation_level(profile_id: &str, level: RelationLevel)
fn record_level_proposal_refused(profile_id: &str)
```

Voir [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) et [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md).

---

## 5. Format des agrégats

### 5.1 Schéma SessionSummary

```json
{
  "last_session_end": "2026-02-13T22:30:00Z",
  "days_since_last_visit": 1,
  "current_session_start": "2026-02-14T14:00:00Z",
  "current_session_duration_minutes": 45
}
```

### 5.2 Schéma ServiceUsage

```json
{
  "service_id": "jayxpose",
  "service_name": "JayXpose",
  "last_opened": "2026-02-10T09:00:00Z",
  "days_since_last_open": 4,
  "minutes_today": 25
}
```

### 5.3 Schéma FriendStatus

```json
{
  "friend_id": "cog-abc-123",
  "friend_pseudo": "Luna",
  "last_contact": "2026-02-01T18:00:00Z",
  "days_since_last_contact": 13,
  "total_chat_minutes": 120
}
```

### 5.4 Schéma BotContext (complet)

```json
{
  "session_start": "2026-02-14T14:00:00Z",
  "session_duration_minutes": 45,
  "is_first_connection_of_session": true,
  "pseudo": "Kaito",
  "langue": "fr",
  "jours_depuis_derniere_visite": 1,
  "service_le_plus_utilise": ["JayXpose", 25],
  "service_delaisse": ["JayKoa", 14],
  "services_ouverts_aujourd_hui": ["JayXpose", "Jay1Tribu"],
  "ami_plus_delaisse": ["Luna", 13],
  "amis_contactes_recemment": [],
  "evenement_prochain": ["Réunion équipe", "2026-02-14T15:00:00Z"],
  "evenement_dans_moins_d_une_heure": true,
  "badge_recent_non_annonce": null,
  "seuil_pause_minutes": 120,
  "max_bulles_par_session": 5,
  "bulles_deja_affichees": 0,
  "ami_connecte_recemment": null,
  "ram_available_mb": 2048,
  "disk_free_gb": 15.5,
  "os_type": "windows",
  "specs_upgraded_since_last": false,
  "user_responses": {"preference_rappel": "soir", "contexte_activite": "maison"},
  "relation_level": 2,
  "relation_level_name": "pote",
  "last_level_proposal_refused": false,
  "complicite_score": 12,
  "complicite_niveau": "modere",
  "score_qualite": 0.6,
  "reponses_par_palier": [0, 2, 2, 0, 0, 0, 0]
}
```

---

## 6. Construction du contexte (algorithme)

### 6.1 Flux du Constructeur

```
1. Initialiser BotContext avec valeurs par défaut
2. Charger profil utilisateur → pseudo, langue, MiouPreferences
3. Charger SessionSummary depuis MiyukiniWatch (ou défaut si indisponible)
4. Charger ServiceUsage[] depuis MiyukiniWatch
5. Charger FriendStatus[] depuis MiyukiniWatch
6. Charger prochain événement depuis JayKoa
7. Charger ami en ligne depuis Jay1Tribu (si événement)
8. Charger badge non annoncé depuis module gamification
9. Calculer dérivées (service_delaisse, ami_plus_delaisse, etc.)
10. Appliquer MiouPreferences (seuils, max_bulles)
11. Retourner BotContext
```

### 6.2 Gestion des erreurs

| Erreur | Comportement |
|--------|--------------|
| MiyukiniWatch timeout | Utiliser valeurs par défaut (None, 0). |
| Profil non trouvé | pseudo = "habitant", préférences = défaut. |
| JayKoa indisponible | evenement_prochain = None. |
| Jay1Tribu indisponible | ami_connecte_recemment = None, amis = []. |
| Exception inattendue | Log, retourner BotContext minimal (pseudo, heure). Ne jamais crasher. |

### 6.3 Cache et performance

| Donnée | Cache | Invalidation |
|--------|-------|--------------|
| Profil | Session | Au changement de profil |
| MiyukiniWatch agrégats | 0 (temps réel) ou 30s | À chaque génération ou timer |
| Événement JayKoa | 1 min | Timer ou événement |
| Amis en ligne | 30s | Polling ou WebSocket |
| Badge non annoncé | Session | À l'annonce |

---

## 7. Variables injectables (mapping)

### 7.1 Mapping Contexte → Variables

| Variable | Source dans BotContext |
|----------|------------------------|
| `{pseudo}` | context.pseudo |
| `{heure}` | now().format("%Hh%M") |
| `{jour_semaine}` | now().format("%A") (localisé) |
| `{jours}` | context.jours_depuis_derniere_visite ou ami_plus_delaisse.jours ou service_delaisse.jours |
| `{duree}` | format_duree(context.session_duration_minutes) |
| `{service}` | context.service_delaisse.name ou service_le_plus_utilise.name |
| `{ami}` | context.ami_plus_delaisse.pseudo ou ami_connecte_recemment |
| `{badge}` | context.badge_recent_non_annonce |
| `{evenement}` | context.evenement_prochain.title |
| `{temps_total}` | format_duree(minutes_today_total) |
| `{service_top}` | context.service_le_plus_utilise.name |

### 7.2 Formatage des durées

| Entrée | Sortie |
|--------|--------|
| 45 (minutes) | "45 min" |
| 90 | "1h30" |
| 120 | "2h" |
| 150 | "2h30" |

### 7.3 Formatage des jours

| Entrée | Sortie |
|--------|--------|
| 1 | "1" |
| 7 | "7" |
| 0 | "quelques" (ou ne pas afficher la bulle) |

---

## 8. Sécurité et validation

### 8.1 Sanitisation des variables

| Règle | Description |
|-------|-------------|
| **Longueur max** | Tronquer pseudo à 50 caractères. Tronquer titre événement à 80. |
| **Caractères spéciaux** | Pas de sanitisation HTML (affichage texte brut). Les caractères Unicode (accents, emojis) sont autorisés. |
| **Injection** | Les variables proviennent de sources internes (pas de saisie utilisateur directe dans le template). Risque faible. |

### 8.2 Validation du contexte

Avant de passer le contexte au Moteur de décision :
- `session_duration_minutes` >= 0
- `jours_depuis_derniere_visite` <= 365 (si > 365, considérer 365)
- `seuil_pause_minutes` dans [30, 480]
- `max_bulles_par_session` dans [1, 15]
- `pseudo` non vide (sinon "habitant")

### 8.3 Confidentialité

| Donnée | Exposition |
|--------|------------|
| Email | Jamais dans une bulle. |
| Contenu des messages | Jamais (MiyukiniWatch ne les lit pas). |
| Pseudo | Oui, dans les bulles. |
| Identifiants techniques (service_id, friend_id) | Non exposés ; seuls les noms affichés (service_name, friend_pseudo) sont utilisés. |

---

## 9. API internes (spécification)

### 9.1 Constructeur de contexte

```rust
pub fn build_context(
    profile_id: &str,
    watch: &MiyukiniWatchAggregator,
    profile_store: &ProfileStore,
    app_context: &AppContext,
) -> Result<BotContext, BotError>
```

### 9.2 Générateur principal

```rust
pub fn generate_next_bulle(
    context: &BotContext,
    templates: &TemplateStore,
    history: &mut SessionHistory,
) -> Option<BulleOutput>
```

### 9.3 Sélecteur de variante

```rust
pub fn select_variante(
    categorie: &str,
    templates: &TemplateStore,
    history: &SessionHistory,
) -> Option<String>  // variante_id
```

### 9.4 Injecteur

```rust
pub fn inject_variables(
    template: &str,
    context: &BotContext,
    categorie: &str,
) -> String
```

---

## 10. Schémas de persistance

### 10.1 Historique anti-répétition

**Table ou collection :** `miou_variante_history`

| Champ | Type |
|-------|------|
| id | UUID |
| profile_id | String |
| session_id | String |
| categorie | String |
| variante_id | String |
| created_at | DateTime |

**Rétention :** 3 sessions ou 7 jours. Purge automatique.

### 10.2 État badge annoncé

**Table ou collection :** `miou_badge_announced`

| Champ | Type |
|-------|------|
| profile_id | String |
| badge_id | String |
| announced_at | DateTime |

**Usage :** Éviter de réafficher la même félicitation.

### 10.3 Préférences Miou

**Stockage :** Dans le profil utilisateur (JSON ou colonnes dédiées).

---

## 11. Diagramme de flux de données

```
┌─────────────────┐
│ MiyukiniWatch   │
│ (sessions,      │
│  services,      │
│  amis, clics)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌─────────────────┐
│ Profil          │     │ Contexte app     │
│ (pseudo,        │     │ (JayKoa,         │
│  préférences)   │     │  JayXpose,       │
└────────┬────────┘     │  Jay1Tribu, MWS) │
         │              └────────┬─────────┘
         │                       │
         └───────────┬────────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │ CONSTRUCTEUR        │
         │ DE CONTEXTE         │
         │                     │
         │ - Agrège            │
         │ - Normalise         │
         │ - Calcule dérivées  │
         └──────────┬──────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │ BotContext          │
         └──────────┬──────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │ MOTEUR DÉCISION      │
         │ + SÉLECTEUR VARIANTE │
         │ + INJECTEUR          │
         └──────────┬──────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │ BulleOutput         │
         │ (texte, actions)    │
         └──────────┬──────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │ Composant UI Bulle  │
         └─────────────────────┘
```

---

## 12. Tests d'intégration des données

### 12.1 Scénarios de test

| Scénario | Données mock | Résultat attendu |
|----------|--------------|------------------|
| Contexte vide | Toutes sources vides | Accueil générique, pseudo "habitant" |
| MiyukiniWatch désactivé | SessionSummary = None | Accueil selon heure, pas de retour/rappels |
| Retour 5 jours | days_since = 5 | Bulle retour avec "5 jours" |
| Service JayKoa délaissé 20 jours | ServiceUsage JayKoa, days = 20 | Bulle suggestion JayKoa |
| Ami Luna 12 jours | FriendStatus Luna, days = 12 | Bulle rappel ami Luna |
| Événement dans 30 min | Event start = now + 30min | Bulle rappel événement |
| Session 2h30 | session_duration = 150 | Bulle pause |

### 12.2 Tests de robustesse

| Test | Action | Vérification |
|------|--------|--------------|
| Timeout MiyukiniWatch | Simuler timeout 5s | Contexte partiel, pas de crash |
| Profil corrompu | pseudo = null, prefs = null | Valeurs par défaut |
| Événement passé | Event start = now - 1h | evenement_prochain = None |
| Service désinstallé | JayKoa dans service_delaisse mais pas installé | Filtrer, ne pas suggérer |

---

## 13. Exemples de chaînes de traitement

### 13.1 Exemple complet : bulle retour après absence

1. **Déclencheur :** Utilisateur se connecte à 14h30.

2. **MiyukiniWatch :** `last_session_end = 2026-02-09 22h`, `session_start = 2026-02-14 14h30`.

3. **Constructeur :** `jours_depuis_derniere_visite = 5`, `is_first_connection_of_session = true`.

4. **Moteur :** Condition 4 vraie (jours >= 3, première connexion) → `retour_absence`.

5. **Sélecteur :** Variante `ra3` choisie (non utilisée récemment) : « {pseudo}, tu reviens après {jours} jours. Bienvenue. »

6. **Injecteur :** `pseudo = "Kaito"`, `jours = "5"` → « Kaito, tu reviens après 5 jours. Bienvenue. »

7. **Output :** `BulleOutput { texte: "...", type: Retour, actions: [] }`.

8. **Historique :** Enregistrement (retour_absence, ra3).

### 13.2 Exemple : bulle pause santé

1. **Déclencheur :** Timer 2h après session.

2. **Context :** `session_duration_minutes = 135`, `seuil_pause_minutes = 120`.

3. **Moteur :** Condition 1 vraie → `pause_sante`.

4. **Sélecteur :** Variante `ps5` : « Ça fait {duree} — une petite pause, ça ne fait pas de mal. »

5. **Injecteur :** `duree = "2h15"` → « Ça fait 2h15 — une petite pause, ça ne fait pas de mal. »

6. **Output :** `BulleOutput { texte: "...", type: Pause, actions: ["Pause", "Plus tard"] }`.

### 13.3 Exemple : dégradation (MiyukiniWatch off)

1. **Context :** MiyukiniWatch désactivé → `jours_depuis_derniere_visite = None`, `service_delaisse = None`, etc.

2. **Moteur :** Conditions 1, 2, 4, 6, 7 fausses. Condition 3 vraie (première connexion) → `accueil_apres_midi`.

3. **Sélecteur :** Variante `ap2` : « Hey {pseudo}, contente de te voir en ce milieu de journée. »

4. **Injecteur :** `pseudo = "Kaito"` (profil disponible) → « Hey Kaito, contente de te voir en ce milieu de journée. »

4. **Output :** Bulle accueil normale. Pas de crash, pas de données manquantes.

---

## 15. Checklist d'intégration

Avant de considérer l'intégration du Bot comme complète :

- [ ] MiyukiniWatch expose les agrégats nécessaires (SessionSummary, ServiceUsage, FriendStatus).
- [ ] Le profil utilisateur expose pseudo et MiouPreferences.
- [ ] JayKoa expose get_next_event (ou équivalent).
- [ ] Jay1Tribu expose get_online_friends et get_friends_list (ou équivalent).
- [ ] Le Constructeur de contexte gère toutes les absences de données (None, timeout).
- [ ] Les variables sont toutes mappées et ont des valeurs par défaut.
- [ ] Les tests d'intégration passent (contexte vide, Watch désactivé, cas nominaux).
- [ ] La latence de construction du contexte est < 20 ms.

---

## 16. Références

- [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md)
- [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md)
- [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)
- [MiyukiniWatch — Document Fondateur](../../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Intégration et flux de données : MiyukiniWatch, profil, contexte applicatif. Le Bot ne consomme que des agrégats, jamais de contenus. Construction robuste, dégradation gracieuse.*
