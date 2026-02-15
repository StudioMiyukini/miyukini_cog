# Bot Miou — Document Fondateur et Architecture

Document exhaustif définissant la vision, la mission, les principes, l'architecture technique complète et les spécifications du Bot (Proto-IA) de Miou.

---

## 1. Vision et mission

### 1.1 Vision

Le Bot Miou est la **première couche d'intelligence** de l'avatar des COGs. Il incarne la voix de Miou à travers un moteur déterministe, fiable et léger. Il permet à Miou de s'adresser à l'utilisateur de manière personnalisée, contextuelle et bienveillante, sans dépendre d'un modèle de langage ni de ressources matérielles significatives.

La vision du Bot repose sur trois piliers :

| Pilier | Description |
|--------|-------------|
| **Fiabilité** | Le Bot fonctionne toujours. Aucune dépendance externe critique. Aucun risque de timeout, d'erreur réseau ou de modèle non chargé. |
| **Légèreté** | Consommation CPU et RAM négligeable. Compatible avec les machines les plus modestes (i3, 4 Go RAM). |
| **Personnalisation** | Les messages sont adaptés au contexte grâce aux agrégats MiyukiniWatch, au profil utilisateur et au contexte applicatif. |

### 1.2 Mission

La mission du Bot est triple :

1. **Générer** les bulles de Miou : sélectionner la catégorie de message appropriée, choisir une variante non répétitive, injecter les variables contextuelles, produire le texte final.
2. **Respecter** le ton et la personnalité de Miou : bienveillant, chaleureux, tutoiement, jamais culpabilisant, jamais intrusif.
3. **Servir** les quatre piliers de Miou : santé (pauses, hygiène d'usage), bien-être émotionnel (rappels amis, félicitations), bien-être physique (pauses), amusement (badges, célébrations).

### 1.3 Périmètre fonctionnel

| Inclus | Exclu |
|--------|-------|
| Génération de texte pour les bulles | Génération de voix (audio) — gérée par un module séparé |
| Sélection de la catégorie et variante | Affichage des bulles (UI) — géré par le composant bulle |
| Injection de variables | Téléchargement de modèles LLM |
| Anti-répétition | Scan des specs (géré par Proto-IA Scan) |
| Intégration avec MiyukiniWatch, profil, contexte | Consentement LLM (géré par Proto-IA Scan) |

---

## 2. Principes fondateurs

### 2.1 Principe de déterministe

Le Bot est **déterministe** : pour un même contexte en entrée, la sortie est reproductible. Cela facilite les tests, le débogage et la cohérence de l'expérience utilisateur. Les seules sources de variabilité sont :
- Le tirage aléatoire parmi les variantes non encore utilisées (seed possible pour tests).
- L'heure système (pour adapter matin/soir).

### 2.2 Principe de dégradation gracieuse

Si une source de données est indisponible (MiyukiniWatch désactivé, profil incomplet, service non connecté), le Bot **ne bloque jamais**. Il utilise des valeurs par défaut ou des templates génériques. Exemples :
- Pseudo inconnu → « toi » ou « habitant ».
- Jours d'absence inconnus → « un moment ».
- Service délaissé inconnu → ne pas afficher de bulle de suggestion service.

### 2.3 Principe d'invariance des contenus

Le Bot **ne lit jamais** le contenu des messages, des champs saisis, des fichiers ou des pages. Il ne reçoit que des **agrégats** et **métadonnées** :
- Durée de session, horodatage, fréquence.
- Identifiants de services, d'amis (pseudo technique).
- Compteurs (nombre de clics, nombre de jours).
- États binaires (badge débloqué oui/non, événement à venir oui/non).

### 2.4 Principe de non-intrusion

Le Bot génère des messages **courts** (1–3 phrases), **discrets** et **dismissibles**. Il ne produit jamais de texte long, de popup bloquante ou de message culpabilisant. Le ton reste toujours positif ou neutre-bienveillant.

### 2.5 Principe de cohérence narrative

Tous les templates respectent la **personnalité de Miou** : tutoiement, ton chaleureux, métaphore de la maison (emménagement, coins, salon), vocabulaire Miyukini (COG, Habitant, Webway, services). Aucun template ne doit rompre cette cohérence.

---

## 3. Architecture technique

### 3.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           BOT MIOU — ARCHITECTURE                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐  │
│  │ MiyukiniWatch│   │   Profil     │   │  Contexte    │   │  Paramètres  │  │
│  │  (agrégats)  │   │  utilisateur │   │  applicatif  │   │    Miou      │  │
│  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘   └──────┬───────┘  │
│         │                  │                  │                  │          │
│         └──────────────────┴──────────────────┴──────────────────┘          │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    CONSTRUCTEUR DE CONTEXTE                           │   │
│  │  Agrège les données, normalise, produit BotContext                     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    MOTEUR DE DÉCISION                                │   │
│  │  Évalue les conditions, applique les règles de priorité,              │   │
│  │  sélectionne la catégorie de bulle à afficher                         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    SÉLECTEUR DE VARIANTE                             │   │
│  │  Anti-répétition : choisit une variante non utilisée récemment        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    INJECTEUR DE VARIABLES                             │   │
│  │  Remplace {pseudo}, {jours}, etc. par les valeurs du contexte         │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    GÉNÉRATEUR DE BULLE                                │   │
│  │  Produit BulleOutput : texte, type, actions suggérées                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    HISTORIQUE DE SESSION                              │   │
│  │  Enregistre les bulles affichées pour anti-répétition                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Composants détaillés

#### 3.2.1 Constructeur de contexte

**Rôle :** Produire une structure `BotContext` unifiée à partir des sources hétérogènes (MiyukiniWatch, profil, services).

**Entrées :**
- Agrégats MiyukiniWatch (sessions, services, amis, clics).
- Profil utilisateur (pseudo, préférences, langue).
- Contexte applicatif (événements JayKoa, état vitrine JayXpose, connexion MWS, amis Jay1Tribu).
- Paramètres Miou (fréquence bulles, seuil pause, etc.).

**Sortie :** `BotContext` structuré, normalisé, prêt pour le moteur de décision.

**Responsabilités :**
- Gérer les valeurs manquantes (defaults).
- Convertir les durées (secondes → minutes/heures pour affichage).
- Détecter les plages horaires (matin 6h–12h, après-midi 12h–18h, soir 18h–6h).
- Calculer les dérivées (jours depuis dernière visite, service le plus utilisé, ami le plus délaissé).

#### 3.2.2 Moteur de décision

**Rôle :** Déterminer **quelle** bulle afficher (catégorie) en fonction du contexte.

**Algorithme :** Évaluation séquentielle des conditions par ordre de priorité. La première condition satisfaite détermine la catégorie. Si aucune condition n'est satisfaite → pas de bulle (silence).

**Priorités (ordre décroissant) :**
1. Pause santé (durée session > seuil).
2. Rappel événement (événement JayKoa dans < 1h).
3. Accueil (première connexion de la session).
4. Retour après absence (jours_absent > 3).
5. Félicitation badge (badge débloqué non annoncé).
6. Rappel ami (ami non contacté > 7 jours).
7. Suggestion service (service délaissé > 14 jours).
8. Sinon → silence.

**Détails :** Voir [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md).

#### 3.2.3 Sélecteur de variante

**Rôle :** Choisir **quelle** variante de la catégorie sélectionnée utiliser.

**Algorithme :**
1. Récupérer la liste des variantes de la catégorie.
2. Exclure les variantes utilisées dans la session courante.
3. Exclure les variantes utilisées dans les N dernières sessions (N=3 par défaut).
4. Si des variantes restent → choisir aléatoirement (ou round-robin).
5. Si aucune variante disponible → utiliser la première (ou déléguer au LLM si activé).

**Persistance :** L'historique des variantes utilisées est stocké en mémoire pour la session, et éventuellement persisté (dernières 3 sessions) pour éviter la répétition inter-sessions.

#### 3.2.4 Injecteur de variables

**Rôle :** Remplacer les placeholders `{variable}` dans le template par les valeurs du contexte.

**Variables supportées :** Voir [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md).

**Règles :**
- Variable inconnue → remplacer par chaîne vide ou valeur par défaut (ex. `{pseudo}` inconnu → « toi »).
- Variable mal formée → ignorer, ne pas crasher.
- Échappement : si une valeur utilisateur contient des caractères spéciaux, les garder (pas d'injection HTML/JS — le texte est affiché en texte brut).

#### 3.2.5 Générateur de bulle

**Rôle :** Produire la structure finale `BulleOutput` contenant :
- `texte` : le message à afficher.
- `type` : accueil, suggestion, rappel, félicitation, notification.
- `actions` : liste des boutons (label, action_id).
- `priorite` : pour la file d'attente des bulles (si plusieurs en attente).
- `auto_dismiss_sec` : délai avant fermeture automatique (optionnel).

---

## 4. Flux de données

### 4.1 Flux principal (génération d'une bulle)

```
1. Déclencheur (timer, événement, arrivée Salon)
       │
       ▼
2. Constructeur de contexte : récupère MiyukiniWatch, profil, contexte app
       │
       ▼
3. BotContext produit
       │
       ▼
4. Moteur de décision : évalue conditions, sélectionne catégorie
       │
       ├── Si aucune condition → FIN (pas de bulle)
       │
       ▼
5. Sélecteur de variante : choisit variante (anti-répétition)
       │
       ▼
6. Injecteur de variables : remplace {x} par valeurs
       │
       ▼
7. Générateur de bulle : produit BulleOutput
       │
       ▼
8. Historique : enregistre (catégorie, variante_id) pour session
       │
       ▼
9. Retour BulleOutput → composant UI bulle
```

### 4.2 Déclencheurs

| Déclencheur | Moment | Fréquence |
|-------------|--------|-----------|
| **Arrivée Salon** | 2–3 secondes après affichage du Salon | 1 fois par session |
| **Timer session** | Toutes les 30 minutes (vérifier pause santé) | Pendant la session |
| **Changement d'onglet** | Passage à un nouvel onglet | Limité (max 1 bulle / 30s) |
| **Événement externe** | Notification Jay1Tribu, rappel JayKoa | À la réception |
| **Badge débloqué** | Détection par gamification | Immédiat (si priorité OK) |

### 4.3 Contrôle de débit

Le Bot ne génère pas plus d'une bulle toutes les 30 secondes (configurable). Un compteur `last_bulle_timestamp` est maintenu. Si une nouvelle génération est demandée avant le délai, elle est mise en file d'attente (priorité selon type) ou ignorée selon la configuration « fréquence » (discrète, normale, bavarde).

---

## 5. Structures de données

### 5.1 BotContext (entrée du moteur)

```rust
// Structure conceptuelle — à adapter au langage d'implémentation
struct BotContext {
    // Session
    session_start: DateTime,
    session_duration_minutes: u32,
    is_first_connection_of_session: bool,
    
    // Utilisateur
    pseudo: String,
    langue: String,  // "fr", "en", etc.
    
    // Absence
    jours_depuis_derniere_visite: Option<u32>,
    
    // Services
    service_le_plus_utilise: Option<(String, u32)>,  // (id, minutes)
    service_delaisse: Option<(String, u32)>,        // (id, jours)
    services_ouverts_aujourd_hui: Vec<String>,
    
    // Amis (si Jay1Tribu)
    ami_plus_delaisse: Option<(String, u32)>,       // (pseudo, jours)
    amis_contactes_recemment: Vec<String>,
    
    // Événements (JayKoa)
    evenement_prochain: Option<(String, DateTime)>,
    evenement_dans_moins_d_une_heure: bool,
    
    // Badges
    badge_recent_non_annonce: Option<String>,
    
    // Paramètres
    seuil_pause_minutes: u32,
    max_bulles_par_session: u32,
    bulles_deja_affichees: u32,
}
```

### 5.2 BulleOutput (sortie du générateur)

```rust
struct BulleOutput {
    texte: String,
    type_bulle: TypeBulle,  // Accueil, Suggestion, Rappel, Felicitation, Notification
    actions: Vec<BulleAction>,
    priorite: u8,
    auto_dismiss_sec: Option<u32>,
}

struct BulleAction {
    label: String,
    action_id: String,  // "ouvrir_jaykoa", "voir_calendrier", "cest_noté", etc.
}
```

### 5.3 Historique de variantes

```rust
struct VarianteHistorique {
    categorie: String,
    variante_id: String,
    session_id: String,
    timestamp: DateTime,
}
```

---

## 6. Interfaces et points d'intégration

### 6.1 Interface avec MiyukiniWatch

Le Bot consomme des **agrégats** exposés par MiyukiniWatch. Pas d'accès direct à la base ; un module d'abstraction fournit :

- `get_session_summary(profile_id) -> SessionSummary`
- `get_services_usage(profile_id, period) -> Vec<ServiceUsage>`
- `get_friends_contact_status(profile_id) -> Vec<FriendStatus>`
- `get_last_visit_delta(profile_id) -> Option<Days>`

### 6.2 Interface avec le profil utilisateur

- `get_pseudo(profile_id) -> String`
- `get_preferred_language(profile_id) -> String`
- `get_miou_preferences(profile_id) -> MiouPreferences`

### 6.3 Interface avec le contexte applicatif

- **JayKoa :** `get_next_event(profile_id) -> Option<Event>`
- **JayXpose :** `is_vitrine_published(profile_id) -> bool`, `has_exposant_profile(profile_id) -> bool`
- **MWS :** `is_connected() -> bool`
- **Jay1Tribu :** `get_friends_list(profile_id) -> Vec<Friend>` (métadonnées uniquement)

### 6.4 Interface avec le composant bulle (UI)

Le composant bulle appelle :
- `generate_next_bulle(context) -> Option<BulleOutput>`

Et notifie :
- `on_bulle_displayed(bulle_id)` — pour l'historique d'anti-répétition.
- `on_bulle_dismissed(bulle_id)` — optionnel, pour stats.

---

## 7. Contraintes et limites

### 7.1 Contraintes techniques

| Contrainte | Valeur | Justification |
|------------|--------|---------------|
| Latence max génération | < 50 ms | L'utilisateur ne doit pas attendre. |
| RAM utilisée | < 10 Mo | Léger, pas d'impact sur machines modestes. |
| Pas de thread dédié | Synchrone | La génération est assez rapide pour être synchrone. |
| Pas de réseau | 100 % local | Conformité LOI-1. |

### 7.2 Limites fonctionnelles

| Limite | Description |
|--------|-------------|
| Pas de conversation | Le Bot ne répond pas à des questions de l'utilisateur. Il produit des messages proactifs uniquement. |
| Pas de personnalisation fine | Le Bot ne peut pas adapter le style à l'humeur (sauf via règles grossières : matin/soir, absence longue). Le LLM complète pour une personnalisation fine. |
| Templates fixes | Les phrases sont prédéfinies. Pas de génération créative. |
| Langue | Une langue par build (ou chargement dynamique de pack). Pas de traduction automatique. |

### 7.3 Edge cases gérés

| Cas | Comportement |
|-----|--------------|
| Contexte vide (première installation) | Bulle accueil générique : « Bienvenue dans ton COG. » |
| MiyukiniWatch désactivé | Utiliser uniquement profil (pseudo) et heure. Pas de rappels ami/service. |
| Toutes variantes épuisées | Réutiliser la première variante (ou déléguer au LLM si activé). |
| Variable manquante dans template | Remplacer par valeur par défaut ou chaîne vide. |
| Contexte partiel (service indisponible) | Ignorer la condition liée à ce service. Dégradation gracieuse. |

---

## 8. Tests et validation

### 8.1 Tests unitaires

| Module | Cas de test |
|--------|-------------|
| Injecteur de variables | Toutes les variables connues, variable inconnue, valeur vide, caractères spéciaux. |
| Sélecteur de variante | Variantes disponibles, toutes épuisées, une seule variante. |
| Moteur de décision | Chaque condition isolée, ordre de priorité, aucune condition. |
| Constructeur de contexte | Données complètes, données partielles, données vides. |

### 8.2 Tests d'intégration

| Scénario | Vérification |
|----------|--------------|
| Première connexion | Bulle accueil matin/soir selon heure. |
| Session > 2h | Bulle pause santé. |
| Retour après 5 jours | Bulle retour avec « 5 jours ». |
| Badge débloqué | Bulle félicitation avec nom du badge. |
| Anti-répétition | 5 connexions successives → 5 variantes différentes (si 5+ variantes). |

### 8.3 Tests de non-régression

- Snapshot des templates : toute modification de template doit être validée manuellement.
- Cohérence narrative : vérifier que tous les templates respectent le ton Miou (checklist).

---

## 9. Évolutions futures

### 9.1 Extensions prévues

| Extension | Description | Priorité |
|-----------|-------------|----------|
| Multi-langue | Packs de templates par langue (en, es). | P2 |
| Variantes saisonnières | Templates spécifiques Noël, été, etc. | P3 |
| Personnalisation fine | Règles utilisateur (ex. « ne jamais suggérer pause »). | P3 |
| Analytics internes | Comptage des bulles par type (pour améliorer les templates). | P3 |

### 9.2 Extensions exclues

| Exclusion | Raison |
|-----------|--------|
| Génération dynamique | Rôle du LLM. Le Bot reste déterministe. |
| Apprentissage | Pas de ML. Règles fixes. |
| Connexion cloud | Conformité souveraineté. |

---

## 10. Emplacement dans le code source (repères)

### 10.1 Structure de modules proposée

```
apps/central/src/
├── miou/
│   ├── mod.rs                 # Module principal Miou
│   ├── bot/
│   │   ├── mod.rs             # Module Bot
│   │   ├── context.rs         # Constructeur de contexte, BotContext
│   │   ├── decision.rs        # Moteur de décision
│   │   ├── variante.rs        # Sélecteur de variante, anti-répétition
│   │   ├── injector.rs        # Injecteur de variables
│   │   ├── generator.rs       # Générateur de BulleOutput
│   │   ├── templates.rs       # Banque de templates (ou chargement depuis fichier)
│   │   └── history.rs        # Historique de session
│   └── ...
```

### 10.2 Dépendances Cargo

Le Bot n'a pas besoin de dépendances externes supplémentaires. Il utilise :
- `chrono` (déjà présent) pour les dates et heures.
- `serde` (déjà présent) pour la sérialisation si besoin.
- `rand` ou `uuid` (optionnel) pour le tirage aléatoire des variantes.

### 10.3 Point d'entrée

Le composant bulle (ou le service Miou) appelle :

```rust
// Pseudo-code
let context = BotContext::build(&watch_agregats, &profile, &app_context)?;
if let Some(bulle) = bot::generate_next_bulle(&context) {
    state.write().miou_current_bulle = Some(bulle);
}
```

---

## 11. Conformité architecturale Miyukini

### 11.1 Lois d'Autonomie

| LOI | Conformité |
|-----|------------|
| **LOI-1** (pas de dépendance externe critique) | Le Bot ne fait aucun appel réseau. Pas d'API cloud. 100 % local. |
| **LOI-5** (souveraineté des données) | Toutes les données restent sur le COG. Aucune fuite. |
| **LOI-5** (souveraineté des données) | Le Bot ne lit que des agrégats. Pas de contenu. |

### 11.2 Relation avec les Cores

Le Bot n'interagit pas directement avec les Cores. Il consomme des données déjà agrégées par MiyukiniWatch (qui peut être gouverné par KindMother pour la persistance). La décision de « quoi afficher » est une logique applicative, pas une décision de gouvernance.

### 11.3 Relation avec BondingBrother

Le Bot n'est pas un Opérateur. C'est un composant interne de Miou. Il ne traduit pas d'intentions vers les Cores. Il produit du texte pour l'interface utilisateur.

---

## 12. Checklist de validation

Avant toute mise en production d'une modification du Bot :

- [ ] Tous les templates respectent le ton Miou (tutoiement, bienveillance).
- [ ] Aucun template ne contient de contenu culpabilisant.
- [ ] Les variables sont toutes documentées dans la Banque de Templates.
- [ ] Les tests unitaires passent.
- [ ] Les tests d'intégration (au moins 3 scénarios) passent.
- [ ] La dégradation gracieuse est vérifiée (MiyukiniWatch désactivé, profil incomplet).
- [ ] Aucune dépendance réseau ajoutée.
- [ ] La latence de génération reste < 50 ms sur machine de référence.

---

## 13. Glossaire technique

| Terme | Définition |
|-------|------------|
| **Bot** | Moteur déterministe de génération des bulles Miou. Synonyme : Proto-IA. |
| **Catégorie** | Type de message (accueil, pause, rappel ami, etc.). Détermine le « quoi » afficher. |
| **Variante** | Une des phrases possibles dans une catégorie. Détermine le « comment » afficher. |
| **Contexte** | Agrégat des données disponibles au moment de la génération. |
| **Template** | Phrase avec placeholders `{variable}`. |
| **Injecteur** | Module qui remplace les placeholders par les valeurs. |
| **Anti-répétition** | Mécanisme pour éviter d'afficher la même variante trop souvent. |
| **Bulle** | Message affiché en bas à droite. Structure : texte + actions optionnelles. |
| **Déclencheur** | Événement ou condition qui lance une tentative de génération. |
| **Débit** | Nombre max de bulles par unité de temps. Contrôle la fréquence. |

---

## 15. Historique des versions du Bot

| Version | Date | Modifications majeures |
|---------|------|------------------------|
| 0.1 | 2026-02 | Création initiale. Architecture en 5 composants. 12 catégories de templates. Intégration MiyukiniWatch, profil, contexte applicatif. |
| — | — | Évolutions futures : multi-langue, variantes saisonnières, personnalisation fine. |

---

## 16. Contributeurs et maintenance

La documentation du Bot est maintenue avec le reste de la documentation Miyukini. Toute modification des templates, des règles de priorité ou des seuils doit être documentée dans ce fichier ou les documents spécialisés (Banque de Templates, Moteur de Décision). Les tests de non-régression doivent être exécutés avant toute mise en production.

---

## 17. Références

- [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md)
- [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md)
- [Bot - Intégration et Flux de Données](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)
- [Miou - Document Fondateur](../Miou%20-%20Document%20Fondateur.md)
- [MiyukiniWatch — Document Fondateur](../../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Bot Miou : première couche d'intelligence, déterministe, fiable, légère. Au service de la relation entre Miou et l'utilisateur.*
