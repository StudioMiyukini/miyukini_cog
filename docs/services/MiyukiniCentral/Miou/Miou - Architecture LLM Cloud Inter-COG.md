# Miou — Architecture LLM Cloud Inter-COG

## Contexte

Le Document Fondateur de Miou pose la règle : **"100% local. Pas de cloud, pas de télémétrie, pas d'API externe."** Cette règle protège la souveraineté de l'utilisateur contre les clouds tiers (OpenAI, Anthropic, etc.).

Elle n'interdit pas qu'un utilisateur déploie **son propre LLM** sur un COG qu'il possède, et le rende accessible à ses autres COGs via le MWS. C'est précisément ce que ce document décrit : l'évolution de Miou vers un **Service Inter-COG (Type 3)**, où le LLM tourne sur un COG dédié de l'utilisateur (ex. serveur maison avec GPU) et dessert l'ensemble de ses COGs via le Webway.

La souveraineté est **intégralement maintenue** : l'utilisateur contrôle tous les nœuds. Aucune donnée ne quitte son réseau.

Cette architecture résout en particulier un problème structurel : les **COGs TERMINAL** (Android, iOS) sont physiquement incapables de faire tourner un LLM local. Sans le Service Inter-COG, Miou sur mobile serait condamné à rester en Proto-IA pour toujours. Avec le COG MiouCloud, Miou est disponible dans sa **meilleure version sur tous les COGs de l'utilisateur**, y compris mobiles.

---

## Portée / Scope

Ce document spécifie :

- L'**architecture topologique** du COG Miou (Service Inter-COG)
- Le **protocole d'échange** entre un COG client et le COG Miou via le MWS
- Le **flux complet** d'une interaction (de l'événement COG à la bulle affichée)
- Les **règles de dégradation gracieuse** (LOI-1 / LOI-2) quand le COG Miou n'est pas joignable
- L'**inventaire des composants** à construire

Ce document **ne remplace pas** le moteur local Proto-IA + LLM local décrit dans [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md). Les deux architectures sont **complémentaires** : le LLM Inter-COG est la couche haute de qualité pour les utilisateurs qui disposent d'un COG serveur ; le LLM local reste la couche optionnelle pour les autres.

---

## 1. Positionnement architectural

### 1.1 Les trois couches d'intelligence de Miou

| Couche | Nature | Disponibilité | Qualité | Ressources |
|--------|--------|---------------|---------|------------|
| **Proto-IA (Bot)** | Templates + règles | Toujours, sur chaque COG | Fonctionnelle | Aucune |
| **LLM local** | Petit modèle (SmolLM2-1.7B) sur le COG | Optionnel, consentement + specs | Bonne | CPU + 1.2 Go RAM |
| **LLM Inter-COG** | Grand modèle (ex. Mistral 7B) sur COG dédié GPU | Optionnel, COG Miou joignable via MWS | Très bonne | GPU 8+ Go sur serveur dédié |

**Règle de priorité :** LLM Inter-COG > LLM local > Proto-IA. Si la couche haute n'est pas disponible, la couche inférieure prend le relais sans interruption visible.

### 1.2 Type de service

Miou-Cloud est un **Service Inter-COG (Type 3)** : il vit sur un COG dédié, ses espaces sont Central (côté client) et les Protocoles Inter-COG (tunnel MWS).

---

## 2. Topologie MWS

```
┌──────────────────────────────────────────────────────────────────┐
│                        MWS (Webway)                              │
│                                                                  │
│  ┌──────────────────┐    Origin          ┌────────────────────┐ │
│  │  COG STABLE      │  (Relay+Tracker)   │  COG SPECIAL       │ │
│  │  (PC principal)  │         │          │  "MiouCloud"       │ │
│  │                  │         │          │  (serveur maison)  │ │
│  │  Central         │◄────────┼─────────►│                    │ │
│  │   └─ Miou UI     │  Tunnel MWS        │  Ollama / vLLM     │ │
│  │   └─ MiouClient  │         │          │  + Mistral 7B      │ │
│  └──────────────────┘         │          │  + GPU 3060 Ti     │ │
│                               │          │  + MiyukiniMiou    │ │
│  ┌──────────────────┐         │          └────────────────────┘ │
│  │  COG TERMINAL    │◄────────┘                                  │
│  │  (téléphone)     │  Même tunnel, même service                │
│  │   └─ Miou UI     │                                            │
│  └──────────────────┘                                            │
└──────────────────────────────────────────────────────────────────┘
```

**Le COG MiouCloud :**
- Se connecte au Relay, passe la vérification 3 phases, obtient son **Permis de circulation**
- S'enregistre dans le **catalogue du Tracker** sous l'identifiant de service `miou_cloud`
- Maintient un état par COG client connecté (profil émotionnel, historique de session)
- Le tunnel MWS est l'unique canal de transport — pas de port réseau exposé directement

---

## 3. Protocole d'échange

### 3.1 Structures de données

```rust
/// Requête envoyée par un COG client vers le COG MiouCloud
#[derive(Serialize, Deserialize)]
pub struct MiouCloudRequest {
    /// Identifiant du COG demandeur (depuis le Permis de circulation)
    pub cog_id: String,
    /// Nature de la requête
    pub request_type: MiouCloudRequestType,
    /// Snapshot du contexte système au moment de la requête
    pub context: MiouContext,
}

#[derive(Serialize, Deserialize)]
pub enum MiouCloudRequestType {
    /// Miou commente spontanément l'état du système
    ContextReaction,
    /// L'utilisateur parle directement à Miou (Onglet Chatbot)
    UserChat { message: String },
    /// Vérification de disponibilité (heartbeat)
    Ping,
    /// Notification d'un événement système notable
    EventNotification { event: MiouSystemEvent },
}

/// Contexte système transmis avec chaque requête
/// INVARIANT : uniquement des agrégats et métadonnées. Jamais de contenu.
#[derive(Serialize, Deserialize)]
pub struct MiouContext {
    /// Services actifs et leur état
    pub active_services: Vec<ServiceStatus>,
    /// Durée de la session courante (minutes)
    pub session_duration_minutes: u32,
    /// Résumé des erreurs récentes (type, count — pas de détail)
    pub recent_errors: Vec<ErrorSummary>,
    /// Niveau de confiance MWS de ce COG
    pub trust_state: TrustState,
    /// Écran actif dans Central (ex. "Salon", "Webway", "JayKoa")
    pub current_screen: String,
    /// Dernière action utilisateur (type d'action — pas de contenu)
    pub last_user_action: String,
    /// Palier d'attachement actuel avec Miou
    pub relation_level: RelationLevel,
    /// Pseudo de l'utilisateur (sanitisé)
    pub user_pseudo: String,
}

#[derive(Serialize, Deserialize)]
pub enum MiouSystemEvent {
    TrackerDisconnected { duration_minutes: u32 },
    BackupCompleted { service: String },
    ServiceLaunched { service: String },
    BadgeUnlocked { badge_id: String },
    LongSession { duration_minutes: u32 },
    FriendInactive { friend_pseudo: String, days: u32 },
}

/// Réponse du COG MiouCloud vers le COG client
#[derive(Serialize, Deserialize)]
pub struct MiouCloudResponse {
    /// Texte de la bulle (1–2 phrases, 30–80 tokens)
    pub text: String,
    /// État émotionnel à refléter dans l'animation de l'avatar
    pub emotion: MiouEmotion,
    /// Identifiant du son à jouer côté client (optionnel)
    pub audio_cue: Option<String>,
    /// Priorité d'affichage de la bulle
    pub priority: MiouBubblePriority,
}

#[derive(Serialize, Deserialize)]
pub enum MiouEmotion {
    Happy, Curious, Worried, Sleepy, Playful, Alert, Comforting, Proud,
}

#[derive(Serialize, Deserialize)]
pub enum MiouBubblePriority {
    /// Bulle normale, entre dans la file d'attente
    Normal,
    /// Bulle douce, peut être différée
    Gentle,
    /// Rappel important (événement imminent, santé)
    Urgent,
}
```

### 3.2 Invariant de confidentialité

Identique à celui de la couche LLM locale :

> Le contexte ne contient que des **agrégats et métadonnées**. Jamais de messages, saisies, fichiers ou contenus personnels. L'invariant MiyukiniWatch s'applique intégralement au contexte Inter-COG.

---

## 4. Architecture du COG MiouCloud

```
COG SPECIAL "MiouCloud"
├── Cores/ (identiques aux autres COGs — gouvernance standard)
├── Services/
│   └── MiyukiniMiou/              ← Service Inter-COG (Type 3)
│       ├── data/
│       │   ├── types.rs           # MiouCloudRequest, Response, Context
│       │   └── cog_profiles.rs    # Profils émotionnels par COG client (KindMother)
│       ├── domain/
│       │   ├── personality.rs     # System prompt, règles, moteur émotionnel
│       │   ├── context_builder.rs # Enrichissement du contexte reçu
│       │   └── llm_client.rs      # Interface vers Ollama/vLLM local
│       └── services/
│           └── mws_endpoint.rs    # Exposition du service sur le tunnel MWS
└── Infra/
    └── Ollama (Mistral 7B, GPU 3060 Ti)
```

### 4.1 Modèles compatibles

| Modèle | RAM GPU | Latence estimée (3060 Ti) | Qualité | Recommandation |
|--------|---------|---------------------------|---------|----------------|
| **Mistral 7B Q4_K_M** | ~5 Go | ~1.5s | Très bonne | **Recommandé** — excellent rapport qualité/ressources |
| **Llama3.2 3B Q4** | ~2.5 Go | < 1s | Bonne | Alternative légère, GPU partiellement libéré |
| **Mistral 7B Q8** | ~8 Go | ~2s | Excellente | Qualité maximale, limite mémoire 3060 Ti (8 Go) |
| **Qwen2.5 7B Q4** | ~5 Go | ~1.5s | Très bonne | Alternative à Mistral, fort en raisonnement |

**Moteur d'inférence :** Ollama (HTTP local sur le COG MiouCloud). Interface propre, modèles interchangeables sans recompilation.

### 4.2 Construction du prompt (COG MiouCloud)

```
Tu es Miou, l'avatar bienveillant du COG Miyukini de {pseudo}.
Ton rôle : veiller sur la santé, le bien-être et l'amusement de {pseudo}.
Ton ton : chaleureux, tutoiement, jamais culpabilisant.
Réponds en 1-2 phrases maximum. Pas de markdown, pas d'URL.
Palier de relation actuel : {relation_level}.

Contexte système :
- Heure locale : {heure}
- Session en cours : {duree_session} minutes
- Écran actif : {current_screen}
- Événement déclencheur : {event_description}
- Services actifs : {services_summary}
- Erreurs récentes : {errors_summary}

Objectif de ce message : {objectif}
```

---

## 5. Flux complet d'une interaction

```
1.  COG STABLE — Central détecte un événement
    ex. : TrackerDisconnected depuis 5 min

2.  MiouClient construit un MiouCloudRequest :
    {
      cog_id: "cog-abc123",
      request_type: EventNotification {
        event: TrackerDisconnected { duration_minutes: 5 }
      },
      context: { session: 180, screen: "Webway", relation: Amie, ... }
    }

3.  Le message transite via le tunnel MWS vers le COG MiouCloud
    (Permis de circulation validé, tunnel chiffré)

4.  COG MiouCloud — traitement :
    a. context_builder.rs enrichit avec le profil COG client
       (historique de session, profil émotionnel stocké via KindMother)
    b. personality.rs construit le prompt complet
    c. llm_client.rs → Ollama → Mistral 7B (~1.5s)
    d. Parse la sortie, détermine l'émotion et la priorité

5.  MiouCloudResponse retourne via le tunnel :
    {
      text: "Mrrr... le Tracker fait la sieste. Tes données sont
             en sécurité avec KindMother, t'inquiète pas~ 🐱",
      emotion: Worried,
      audio_cue: "miou_worried_soft",
      priority: Normal
    }

6.  COG STABLE — Central :
    - Affiche la bulle (Système de Bulles)
    - Joue le son audio_cue
    - Anime l'avatar selon emotion
```

**Latence totale estimée :** ~2–3s (1.5s LLM + ~0.5s tunnel MWS aller-retour local).

---

## 6. Modèle push vs pull

### 6.1 Recommandation : modèle hybride

| Mode | Description | Usage |
|------|-------------|-------|
| **Push événements** | Le COG client envoie les événements système au COG MiouCloud en temps réel | Déclencheurs automatiques (Tracker down, badge, pause santé) |
| **Pull dialogue** | Le COG client interroge le COG MiouCloud quand l'utilisateur parle directement à Miou | Onglet Chatbot, requête explicite |

Le push est plus naturel pour un compagnon : Miou reçoit le flux d'événements et **décide elle-même** d'intervenir ou non, selon la pertinence et le profil émotionnel du COG client.

### 6.2 État maintenu par le COG MiouCloud

Le COG MiouCloud maintient, pour chaque COG client connecté :

| Donnée | Description | Persistance |
|--------|-------------|-------------|
| `last_event_seen` | Dernier événement traité | Session |
| `last_bubble_sent_at` | Horodatage de la dernière bulle | Session |
| `emotional_profile` | Ton et relation actuels (palier, humeur déduite) | KindMother (long terme) |
| `session_event_log` | Événements de la session courante | Session |

---

## 7. Dégradation gracieuse (conformité LOI-1 / LOI-2)

**Règle fondamentale :** Le COG MiouCloud est un **enrichissement**, jamais une dépendance critique.

```rust
impl MiouClient {
    pub async fn request(&self, req: MiouCloudRequest) -> MiouResponse {
        // 1. Tenter le COG MiouCloud via MWS
        if let Ok(response) = self.send_via_mws(&req).await {
            return response;
        }

        // 2. Fallback : LLM local (si installé et actif)
        if let Ok(response) = self.local_llm.generate(&req).await {
            return response;
        }

        // 3. Fallback final : Proto-IA (toujours disponible)
        self.proto_ia.react(&req.context)
    }
}
```

| Scénario | Comportement |
|----------|-------------|
| COG MiouCloud joignable | Réponse LLM haute qualité (Mistral 7B) |
| COG MiouCloud offline | Fallback LLM local si disponible, sinon Proto-IA |
| MWS dégradé (timeout) | Timeout court (2s max) → fallback immédiat |
| COG MiouCloud surchargé | File d'attente côté serveur, réponse différée ou fallback |

**Conformité :**

| Loi | Conformité |
|-----|------------|
| **LOI-1** (pas de dépendance critique) | Fallback complet : LLM local → Proto-IA. Miou parle toujours. |
| **LOI-2** (isolement accepté) | Si le serveur est down, le COG fonctionne à 100% en local. |
| **LOI-5** (souveraineté) | Le COG MiouCloud est possédé par l'utilisateur. Aucune donnée ne quitte son réseau. |
| **LOI-6** (fédération) | N'importe quel COG de l'utilisateur (STABLE, TERMINAL) accède au service via le Webway. |

---

## 8. Inventaire des composants à construire

| Composant | Localisation | Nature |
|-----------|-------------|--------|
| **MiyukiniMiou** (service serveur) | `services/MiyukiniMiou/` sur le COG MiouCloud | Nouveau crate — endpoint MWS + interface Ollama |
| **MiouClient** (couche client) | Dans chaque COG | Extension du client Miou existant — appels MWS |
| **MiouContext** (struct partagée) | Crate partagée ou crate MWS | Types sérialisables communs |
| **System prompt Miou** | Fichier de configuration sur le COG MiouCloud | Personnalité + templates contextuels |
| **Modelfile Ollama** | Serveur COG MiouCloud | Configuration Mistral 7B |
| **Enregistrement MWS** | Origin / Tracker | Référencer `miou_cloud` dans le catalogue |

---

## 9. Différence avec le LLM local existant (synthèse)

| Critère | LLM local (existant) | LLM Inter-COG (ce document) |
|---------|---------------------|------------------------------|
| **Localisation du modèle** | Sur le COG client | Sur le COG MiouCloud (serveur dédié GPU) |
| **Modèle** | SmolLM2-1.7B (~1.2 Go) | Mistral 7B (~5 Go) |
| **Qualité** | Bonne | Très bonne |
| **Disponibilité** | Si specs suffisantes + consentement | Si COG MiouCloud joignable via MWS |
| **Transport** | Intra-processus | Tunnel MWS chiffré |
| **Ressources COG client** | CPU + RAM du COG | Aucune (inférence déportée) |
| **Cas d'usage principal** | COGs sans GPU, machines modestes | Utilisateurs avec serveur dédié GPU |

Les deux peuvent coexister : si le COG MiouCloud est indisponible, le LLM local prend le relais. Si ni l'un ni l'autre n'est disponible, la Proto-IA assure.

---

## 10. Cas d'usage prioritaires par type de COG

L'architecture Inter-COG n'apporte pas la même valeur à tous les COGs. Le tableau suivant précise l'impact selon le type.

| Type de COG | OS | LLM local possible ? | Bénéfice du Service Inter-COG |
|-------------|-----|---------------------|-------------------------------|
| **STABLE** (PC principal) | Windows / Linux / macOS | Oui, si specs suffisantes | Qualité supérieure (Mistral 7B vs SmolLM2-1.7B), CPU du COG non consommé |
| **SPECIAL** (serveur dédié) | Linux | Oui (machine puissante) | Peut lui-même être le COG MiouCloud |
| **TERMINAL** (téléphone) | Android / iOS | **Non — impossible** | **Seule voie pour un Miou LLM de qualité sur mobile** |
| **TERMINAL** (laptop léger) | Windows / Linux | Marginal (i3, 8 Go RAM) | Évite la dégradation thermique et RAM du LLM local |

### 10.1 Le cas TERMINAL Android / iOS

Un COG TERMINAL mobile n'a pas les ressources pour charger un modèle GGUF :

- **RAM** : 3–6 Go partagée entre OS, apps système et Central — charger 1.2 Go pour SmolLM2 est trop coûteux
- **Thermique** : une inférence prolongée chaufferait l'appareil et viderait la batterie
- **Stockage** : télécharger et stocker un fichier GGUF de 1–5 Go est problématique
- **CPU ARM** : l'inférence quantifiée GGUF est supportée, mais lente sur CPU mobile sans NPU dédié

Avec le Service Inter-COG, le terminal envoie un `MiouCloudRequest` de quelques centaines d'octets via le tunnel MWS. L'inférence Mistral 7B tourne sur le GPU du COG MiouCloud (~1.5s). Le terminal reçoit une `MiouCloudResponse` et affiche la bulle. **La charge computationnelle est entièrement déportée.**

```
COG TERMINAL Android
  └─ MiouClient
       └─ send_via_mws(request)   # ~200 octets envoyés
            │ tunnel MWS
       └─ recv MiouCloudResponse  # ~200 octets reçus
  └─ Affiche la bulle + joue le son
```

### 10.2 Comportement hors réseau local (TERMINAL mobile)

Un téléphone peut se retrouver hors du réseau local de l'utilisateur (déplacement, données mobiles). Le MWS est conçu pour les connexions distantes, mais si le COG MiouCloud n'est pas joignable :

| Scénario | Comportement |
|----------|-------------|
| Réseau local disponible | Service Inter-COG normal (~1.5s) |
| MWS joignable mais COG MiouCloud offline | Fallback Proto-IA immédiat (timeout 2s) |
| Hors réseau (avion, zone blanche) | Fallback Proto-IA immédiat |

**Règle :** Sur TERMINAL, la couche LLM local n'existe pas. La cascade est simplement : MiouCloud → Proto-IA. L'expérience reste complète en Proto-IA — Miou parle toujours.

---

## 12. Clause de souveraineté

> **Le COG MiouCloud n'est pas un service cloud tiers.** Il s'agit d'un COG appartenant à l'utilisateur, déployé sur une machine physique qu'il contrôle (serveur domestique, NAS avec GPU, machine dédiée). Le principe "100% local" du Document Fondateur reste valide dans son intention : **aucune donnée de l'utilisateur ne quitte son périmètre de contrôle**. Le Webway est le réseau interne de l'utilisateur, pas Internet ouvert.
>
> Cette architecture est une **extension du principe de souveraineté** à l'échelle d'un réseau de COGs personnel, pas une violation de ce principe.

---

## 11. Références

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md) — Identité, mission, principes (dont la clause de souveraineté)
- [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) — Architecture Proto-IA + LLM local
- [Miou - Systeme de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) — Affichage côté client
- [MWS - Document Fondateur](../../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) — Architecture Webway
- [Miyukini Conceptual References - Types de Services et Espaces](../../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) — Service Inter-COG (Type 3)
- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md) — Invariant de données (agrégats uniquement)

---

*Miou Inter-COG : souveraineté étendue au réseau personnel. Le LLM tourne chez toi, sur ta machine, pour tous tes COGs — y compris ton téléphone.*

*Créé : 2026-02-24*
