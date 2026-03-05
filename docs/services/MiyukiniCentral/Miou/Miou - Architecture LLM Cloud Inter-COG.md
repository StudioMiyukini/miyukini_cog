# Miou â€” Architecture LLM Cloud Inter-COG

## Contexte

Le Document Fondateur de Miou pose la rÃ¨gle : **"100% local. Pas de cloud, pas de tÃ©lÃ©mÃ©trie, pas d'API externe."** Cette rÃ¨gle protÃ¨ge la souverainetÃ© de l'utilisateur contre les clouds tiers (OpenAI, Anthropic, etc.).

Elle n'interdit pas qu'un utilisateur dÃ©ploie **son propre LLM** sur un COG qu'il possÃ¨de, et le rende accessible Ã  ses autres COGs via le MWS. C'est prÃ©cisÃ©ment ce que ce document dÃ©crit : l'Ã©volution de Miou vers un **Service Inter-COG (Type 3)**, oÃ¹ le LLM tourne sur un COG dÃ©diÃ© de l'utilisateur (ex. serveur maison avec GPU) et dessert l'ensemble de ses COGs via le Webway.

La souverainetÃ© est **intÃ©gralement maintenue** : l'utilisateur contrÃ´le tous les nÅ“uds. Aucune donnÃ©e ne quitte son rÃ©seau.

Cette architecture rÃ©sout en particulier un problÃ¨me structurel : les **COGs TERMINAL** (Android, iOS) sont physiquement incapables de faire tourner un LLM local. Sans le Service Inter-COG, Miou sur mobile serait condamnÃ© Ã  rester en Proto-IA pour toujours. Avec le COG MiouCloud, Miou est disponible dans sa **meilleure version sur tous les COGs de l'utilisateur**, y compris mobiles.

---

## PortÃ©e / Scope

Ce document spÃ©cifie :

- L'**architecture topologique** du COG Miou (Service Inter-COG)
- Le **protocole d'Ã©change** entre un COG client et le COG Miou via le MWS
- Le **flux complet** d'une interaction (de l'Ã©vÃ©nement COG Ã  la bulle affichÃ©e)
- Les **rÃ¨gles de dÃ©gradation gracieuse** (LOI-1 / LOI-2) quand le COG Miou n'est pas joignable
- L'**inventaire des composants** Ã  construire

Ce document **ne remplace pas** le moteur local Proto-IA + LLM local dÃ©crit dans [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md). Les deux architectures sont **complÃ©mentaires** : le LLM Inter-COG est la couche haute de qualitÃ© pour les utilisateurs qui disposent d'un COG serveur ; le LLM local reste la couche optionnelle pour les autres.

---

## 1. Positionnement architectural

### 1.1 Les trois couches d'intelligence de Miou

| Couche | Nature | DisponibilitÃ© | QualitÃ© | Ressources |
|--------|--------|---------------|---------|------------|
| **Proto-IA (Bot)** | Templates + rÃ¨gles | Toujours, sur chaque COG | Fonctionnelle | Aucune |
| **LLM local** | Petit modÃ¨le (SmolLM2-1.7B) sur le COG | Optionnel, consentement + specs | Bonne | CPU + 1.2 Go RAM |
| **LLM Inter-COG** | Grand modÃ¨le (ex. Mistral 7B) sur COG dÃ©diÃ© GPU | Optionnel, COG Miou joignable via MWS | TrÃ¨s bonne | GPU 8+ Go sur serveur dÃ©diÃ© |

**RÃ¨gle de prioritÃ© :** LLM Inter-COG > LLM local > Proto-IA. Si la couche haute n'est pas disponible, la couche infÃ©rieure prend le relais sans interruption visible.

### 1.2 Type de service

Miou-Cloud est un **Service Inter-COG (Type 3)** : il vit sur un COG dÃ©diÃ©, ses espaces sont Central (cÃ´tÃ© client) et les Protocoles Inter-COG (tunnel MWS).

---

## 2. Topologie MWS

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        MWS (Webway)                              â”‚
â”‚                                                                  â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”    Origin          â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”‚
â”‚  â”‚  COG STABLE      â”‚  (Relay+Tracker)   â”‚  COG SPECIAL       â”‚ â”‚
â”‚  â”‚  (PC principal)  â”‚         â”‚          â”‚  "MiouCloud"       â”‚ â”‚
â”‚  â”‚                  â”‚         â”‚          â”‚  (serveur maison)  â”‚ â”‚
â”‚  â”‚  Central         â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â–ºâ”‚                    â”‚ â”‚
â”‚  â”‚   â””â”€ Miou UI     â”‚  Tunnel MWS        â”‚  Ollama / vLLM     â”‚ â”‚
â”‚  â”‚   â””â”€ MiouClient  â”‚         â”‚          â”‚  + Mistral 7B      â”‚ â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜         â”‚          â”‚  + GPU 3060 Ti     â”‚ â”‚
â”‚                               â”‚          â”‚  + MiyukiniMiou    â”‚ â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”         â”‚          â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â”‚
â”‚  â”‚  COG TERMINAL    â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                  â”‚
â”‚  â”‚  (tÃ©lÃ©phone)     â”‚  MÃªme tunnel, mÃªme service                â”‚
â”‚  â”‚   â””â”€ Miou UI     â”‚                                            â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                            â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Le COG MiouCloud :**
- Se connecte au Relay, passe la vÃ©rification 3 phases, obtient son **Permis de circulation**
- S'enregistre dans le **catalogue du Tracker** sous l'identifiant de service `miou_cloud`
- Maintient un Ã©tat par COG client connectÃ© (profil Ã©motionnel, historique de session)
- Le tunnel MWS est l'unique canal de transport â€” pas de port rÃ©seau exposÃ© directement

---

## 3. Protocole d'Ã©change

### 3.1 Structures de donnÃ©es

```rust
/// RequÃªte envoyÃ©e par un COG client vers le COG MiouCloud
#[derive(Serialize, Deserialize)]
pub struct MiouCloudRequest {
    /// Identifiant du COG demandeur (depuis le Permis de circulation)
    pub cog_id: String,
    /// Nature de la requÃªte
    pub request_type: MiouCloudRequestType,
    /// Snapshot du contexte systÃ¨me au moment de la requÃªte
    pub context: MiouContext,
}

#[derive(Serialize, Deserialize)]
pub enum MiouCloudRequestType {
    /// Miou commente spontanÃ©ment l'Ã©tat du systÃ¨me
    ContextReaction,
    /// L'utilisateur parle directement Ã  Miou (Onglet Chatbot)
    UserChat { message: String },
    /// VÃ©rification de disponibilitÃ© (heartbeat)
    Ping,
    /// Notification d'un Ã©vÃ©nement systÃ¨me notable
    EventNotification { event: MiouSystemEvent },
}

/// Contexte systÃ¨me transmis avec chaque requÃªte
/// INVARIANT : uniquement des agrÃ©gats et mÃ©tadonnÃ©es. Jamais de contenu.
#[derive(Serialize, Deserialize)]
pub struct MiouContext {
    /// Services actifs et leur Ã©tat
    pub active_services: Vec<ServiceStatus>,
    /// DurÃ©e de la session courante (minutes)
    pub session_duration_minutes: u32,
    /// RÃ©sumÃ© des erreurs rÃ©centes (type, count â€” pas de dÃ©tail)
    pub recent_errors: Vec<ErrorSummary>,
    /// Niveau de confiance MWS de ce COG
    pub trust_state: TrustState,
    /// Ã‰cran actif dans Central (ex. "Salon", "Webway", "JayKoa")
    pub current_screen: String,
    /// DerniÃ¨re action utilisateur (type d'action â€” pas de contenu)
    pub last_user_action: String,
    /// Palier d'attachement actuel avec Miou
    pub relation_level: RelationLevel,
    /// Pseudo de l'utilisateur (sanitisÃ©)
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

/// RÃ©ponse du COG MiouCloud vers le COG client
#[derive(Serialize, Deserialize)]
pub struct MiouCloudResponse {
    /// Texte de la bulle (1â€“2 phrases, 30â€“80 tokens)
    pub text: String,
    /// Ã‰tat Ã©motionnel Ã  reflÃ©ter dans l'animation de l'avatar
    pub emotion: MiouEmotion,
    /// Identifiant du son Ã  jouer cÃ´tÃ© client (optionnel)
    pub audio_cue: Option<String>,
    /// PrioritÃ© d'affichage de la bulle
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
    /// Bulle douce, peut Ãªtre diffÃ©rÃ©e
    Gentle,
    /// Rappel important (Ã©vÃ©nement imminent, santÃ©)
    Urgent,
}
```

### 3.2 Invariant de confidentialitÃ©

Identique Ã  celui de la couche LLM locale :

> Le contexte ne contient que des **agrÃ©gats et mÃ©tadonnÃ©es**. Jamais de messages, saisies, fichiers ou contenus personnels. L'invariant MiyukiniWatch s'applique intÃ©gralement au contexte Inter-COG.

---

## 4. Architecture du COG MiouCloud

```
COG SPECIAL "MiouCloud"
â”œâ”€â”€ Cores/ (identiques aux autres COGs â€” gouvernance standard)
â”œâ”€â”€ Services/
â”‚   â””â”€â”€ MiyukiniMiou/              â† Service Inter-COG (Type 3)
â”‚       â”œâ”€â”€ data/
â”‚       â”‚   â”œâ”€â”€ types.rs           # MiouCloudRequest, Response, Context
â”‚       â”‚   â””â”€â”€ cog_profiles.rs    # Profils Ã©motionnels par COG client (KindMother)
â”‚       â”œâ”€â”€ domain/
â”‚       â”‚   â”œâ”€â”€ personality.rs     # System prompt, rÃ¨gles, moteur Ã©motionnel
â”‚       â”‚   â”œâ”€â”€ context_builder.rs # Enrichissement du contexte reÃ§u
â”‚       â”‚   â””â”€â”€ llm_client.rs      # Interface vers Ollama/vLLM local
â”‚       â””â”€â”€ services/
â”‚           â””â”€â”€ mws_endpoint.rs    # Exposition du service sur le tunnel MWS
â””â”€â”€ Infra/
    â””â”€â”€ Ollama (Mistral 7B, GPU 3060 Ti)
```

### 4.1 ModÃ¨les compatibles

| ModÃ¨le | RAM GPU | Latence estimÃ©e (3060 Ti) | QualitÃ© | Recommandation |
|--------|---------|---------------------------|---------|----------------|
| **Mistral 7B Q4_K_M** | ~5 Go | ~1.5s | TrÃ¨s bonne | **RecommandÃ©** â€” excellent rapport qualitÃ©/ressources |
| **Llama3.2 3B Q4** | ~2.5 Go | < 1s | Bonne | Alternative lÃ©gÃ¨re, GPU partiellement libÃ©rÃ© |
| **Mistral 7B Q8** | ~8 Go | ~2s | Excellente | QualitÃ© maximale, limite mÃ©moire 3060 Ti (8 Go) |
| **Qwen2.5 7B Q4** | ~5 Go | ~1.5s | TrÃ¨s bonne | Alternative Ã  Mistral, fort en raisonnement |

**Moteur d'infÃ©rence :** Ollama (HTTP local sur le COG MiouCloud). Interface propre, modÃ¨les interchangeables sans recompilation.

### 4.2 Construction du prompt (COG MiouCloud)

```
Tu es Miou, l'avatar bienveillant du COG Miyukini de {pseudo}.
Ton rÃ´le : veiller sur la santÃ©, le bien-Ãªtre et l'amusement de {pseudo}.
Ton ton : chaleureux, tutoiement, jamais culpabilisant.
RÃ©ponds en 1-2 phrases maximum. Pas de markdown, pas d'URL.
Palier de relation actuel : {relation_level}.

Contexte systÃ¨me :
- Heure locale : {heure}
- Session en cours : {duree_session} minutes
- Ã‰cran actif : {current_screen}
- Ã‰vÃ©nement dÃ©clencheur : {event_description}
- Services actifs : {services_summary}
- Erreurs rÃ©centes : {errors_summary}

Objectif de ce message : {objectif}
```

---

## 5. Flux complet d'une interaction

```
1.  COG STABLE â€” Central dÃ©tecte un Ã©vÃ©nement
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
    (Permis de circulation validÃ©, tunnel chiffrÃ©)

4.  COG MiouCloud â€” traitement :
    a. context_builder.rs enrichit avec le profil COG client
       (historique de session, profil Ã©motionnel stockÃ© via KindMother)
    b. personality.rs construit le prompt complet
    c. llm_client.rs â†’ Ollama â†’ Mistral 7B (~1.5s)
    d. Parse la sortie, dÃ©termine l'Ã©motion et la prioritÃ©

5.  MiouCloudResponse retourne via le tunnel :
    {
      text: "Mrrr... le Tracker fait la sieste. Tes donnÃ©es sont
             en sÃ©curitÃ© avec KindMother, t'inquiÃ¨te pas~ ðŸ±",
      emotion: Worried,
      audio_cue: "miou_worried_soft",
      priority: Normal
    }

6.  COG STABLE â€” Central :
    - Affiche la bulle (SystÃ¨me de Bulles)
    - Joue le son audio_cue
    - Anime l'avatar selon emotion
```

**Latence totale estimÃ©e :** ~2â€“3s (1.5s LLM + ~0.5s tunnel MWS aller-retour local).

---

## 6. ModÃ¨le push vs pull

### 6.1 Recommandation : modÃ¨le hybride

| Mode | Description | Usage |
|------|-------------|-------|
| **Push Ã©vÃ©nements** | Le COG client envoie les Ã©vÃ©nements systÃ¨me au COG MiouCloud en temps rÃ©el | DÃ©clencheurs automatiques (Tracker down, badge, pause santÃ©) |
| **Pull dialogue** | Le COG client interroge le COG MiouCloud quand l'utilisateur parle directement Ã  Miou | Onglet Chatbot, requÃªte explicite |

Le push est plus naturel pour un compagnon : Miou reÃ§oit le flux d'Ã©vÃ©nements et **dÃ©cide elle-mÃªme** d'intervenir ou non, selon la pertinence et le profil Ã©motionnel du COG client.

### 6.2 Ã‰tat maintenu par le COG MiouCloud

Le COG MiouCloud maintient, pour chaque COG client connectÃ© :

| DonnÃ©e | Description | Persistance |
|--------|-------------|-------------|
| `last_event_seen` | Dernier Ã©vÃ©nement traitÃ© | Session |
| `last_bubble_sent_at` | Horodatage de la derniÃ¨re bulle | Session |
| `emotional_profile` | Ton et relation actuels (palier, humeur dÃ©duite) | KindMother (long terme) |
| `session_event_log` | Ã‰vÃ©nements de la session courante | Session |

---

## 7. DÃ©gradation gracieuse (conformitÃ© LOI-1 / LOI-2)

**RÃ¨gle fondamentale :** Le COG MiouCloud est un **enrichissement**, jamais une dÃ©pendance critique.

```rust
impl MiouClient {
    pub async fn request(&self, req: MiouCloudRequest) -> MiouResponse {
        // 1. Tenter le COG MiouCloud via MWS
        if let Ok(response) = self.send_via_mws(&req).await {
            return response;
        }

        // 2. Fallback : LLM local (si installÃ© et actif)
        if let Ok(response) = self.local_llm.generate(&req).await {
            return response;
        }

        // 3. Fallback final : Proto-IA (toujours disponible)
        self.proto_ia.react(&req.context)
    }
}
```

| ScÃ©nario | Comportement |
|----------|-------------|
| COG MiouCloud joignable | RÃ©ponse LLM haute qualitÃ© (Mistral 7B) |
| COG MiouCloud offline | Fallback LLM local si disponible, sinon Proto-IA |
| MWS dÃ©gradÃ© (timeout) | Timeout court (2s max) â†’ fallback immÃ©diat |
| COG MiouCloud surchargÃ© | File d'attente cÃ´tÃ© serveur, rÃ©ponse diffÃ©rÃ©e ou fallback |

**ConformitÃ© :**

| Loi | ConformitÃ© |
|-----|------------|
| **LOI-1** (pas de dÃ©pendance critique) | Fallback complet : LLM local â†’ Proto-IA. Miou parle toujours. |
| **LOI-2** (isolement acceptÃ©) | Si le serveur est down, le COG fonctionne Ã  100% en local. |
| **LOI-5** (souverainetÃ©) | Le COG MiouCloud est possÃ©dÃ© par l'utilisateur. Aucune donnÃ©e ne quitte son rÃ©seau. |
| **LOI-6** (fÃ©dÃ©ration) | N'importe quel COG de l'utilisateur (STABLE, TERMINAL) accÃ¨de au service via le Webway. |

---

## 8. Inventaire des composants Ã  construire

| Composant | Localisation | Nature |
|-----------|-------------|--------|
| **MiyukiniMiou** (service serveur) | `services/MiyukiniMiou/` sur le COG MiouCloud | Nouveau crate â€” endpoint MWS + interface Ollama |
| **MiouClient** (couche client) | Dans chaque COG | Extension du client Miou existant â€” appels MWS |
| **MiouContext** (struct partagÃ©e) | Crate partagÃ©e ou crate MWS | Types sÃ©rialisables communs |
| **System prompt Miou** | Fichier de configuration sur le COG MiouCloud | PersonnalitÃ© + templates contextuels |
| **Modelfile Ollama** | Serveur COG MiouCloud | Configuration Mistral 7B |
| **Enregistrement MWS** | Origin / Tracker | RÃ©fÃ©rencer `miou_cloud` dans le catalogue |

---

## 9. DiffÃ©rence avec le LLM local existant (synthÃ¨se)

| CritÃ¨re | LLM local (existant) | LLM Inter-COG (ce document) |
|---------|---------------------|------------------------------|
| **Localisation du modÃ¨le** | Sur le COG client | Sur le COG MiouCloud (serveur dÃ©diÃ© GPU) |
| **ModÃ¨le** | SmolLM2-1.7B (~1.2 Go) | Mistral 7B (~5 Go) |
| **QualitÃ©** | Bonne | TrÃ¨s bonne |
| **DisponibilitÃ©** | Si specs suffisantes + consentement | Si COG MiouCloud joignable via MWS |
| **Transport** | Intra-processus | Tunnel MWS chiffrÃ© |
| **Ressources COG client** | CPU + RAM du COG | Aucune (infÃ©rence dÃ©portÃ©e) |
| **Cas d'usage principal** | COGs sans GPU, machines modestes | Utilisateurs avec serveur dÃ©diÃ© GPU |

Les deux peuvent coexister : si le COG MiouCloud est indisponible, le LLM local prend le relais. Si ni l'un ni l'autre n'est disponible, la Proto-IA assure.

---

## 10. Cas d'usage prioritaires par type de COG

L'architecture Inter-COG n'apporte pas la mÃªme valeur Ã  tous les COGs. Le tableau suivant prÃ©cise l'impact selon le type.

| Type de COG | OS | LLM local possible ? | BÃ©nÃ©fice du Service Inter-COG |
|-------------|-----|---------------------|-------------------------------|
| **STABLE** (PC principal) | Windows / Linux / macOS | Oui, si specs suffisantes | QualitÃ© supÃ©rieure (Mistral 7B vs SmolLM2-1.7B), CPU du COG non consommÃ© |
| **SPECIAL** (serveur dÃ©diÃ©) | Linux | Oui (machine puissante) | Peut lui-mÃªme Ãªtre le COG MiouCloud |
| **TERMINAL** (tÃ©lÃ©phone) | Android / iOS | **Non â€” impossible** | **Seule voie pour un Miou LLM de qualitÃ© sur mobile** |
| **TERMINAL** (laptop lÃ©ger) | Windows / Linux | Marginal (i3, 8 Go RAM) | Ã‰vite la dÃ©gradation thermique et RAM du LLM local |

### 10.1 Le cas TERMINAL Android / iOS

Un COG TERMINAL mobile n'a pas les ressources pour charger un modÃ¨le GGUF :

- **RAM** : 3â€“6 Go partagÃ©e entre OS, apps systÃ¨me et Central â€” charger 1.2 Go pour SmolLM2 est trop coÃ»teux
- **Thermique** : une infÃ©rence prolongÃ©e chaufferait l'appareil et viderait la batterie
- **Stockage** : tÃ©lÃ©charger et stocker un fichier GGUF de 1â€“5 Go est problÃ©matique
- **CPU ARM** : l'infÃ©rence quantifiÃ©e GGUF est supportÃ©e, mais lente sur CPU mobile sans NPU dÃ©diÃ©

Avec le Service Inter-COG, le terminal envoie un `MiouCloudRequest` de quelques centaines d'octets via le tunnel MWS. L'infÃ©rence Mistral 7B tourne sur le GPU du COG MiouCloud (~1.5s). Le terminal reÃ§oit une `MiouCloudResponse` et affiche la bulle. **La charge computationnelle est entiÃ¨rement dÃ©portÃ©e.**

```
COG TERMINAL Android
  â””â”€ MiouClient
       â””â”€ send_via_mws(request)   # ~200 octets envoyÃ©s
            â”‚ tunnel MWS
       â””â”€ recv MiouCloudResponse  # ~200 octets reÃ§us
  â””â”€ Affiche la bulle + joue le son
```

### 10.2 Comportement hors rÃ©seau local (TERMINAL mobile)

Un tÃ©lÃ©phone peut se retrouver hors du rÃ©seau local de l'utilisateur (dÃ©placement, donnÃ©es mobiles). Le MWS est conÃ§u pour les connexions distantes, mais si le COG MiouCloud n'est pas joignable :

| ScÃ©nario | Comportement |
|----------|-------------|
| RÃ©seau local disponible | Service Inter-COG normal (~1.5s) |
| MWS joignable mais COG MiouCloud offline | Fallback Proto-IA immÃ©diat (timeout 2s) |
| Hors rÃ©seau (avion, zone blanche) | Fallback Proto-IA immÃ©diat |

**RÃ¨gle :** Sur TERMINAL, la couche LLM local n'existe pas. La cascade est simplement : MiouCloud â†’ Proto-IA. L'expÃ©rience reste complÃ¨te en Proto-IA â€” Miou parle toujours.

---

## 12. Clause de souverainetÃ©

> **Le COG MiouCloud n'est pas un service cloud tiers.** Il s'agit d'un COG appartenant Ã  l'utilisateur, dÃ©ployÃ© sur une machine physique qu'il contrÃ´le (serveur domestique, NAS avec GPU, machine dÃ©diÃ©e). Le principe "100% local" du Document Fondateur reste valide dans son intention : **aucune donnÃ©e de l'utilisateur ne quitte son pÃ©rimÃ¨tre de contrÃ´le**. Le Webway est le rÃ©seau interne de l'utilisateur, pas Internet ouvert.
>
> Cette architecture est une **extension du principe de souverainetÃ©** Ã  l'Ã©chelle d'un rÃ©seau de COGs personnel, pas une violation de ce principe.

---

## 11. RÃ©fÃ©rences

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md) â€” IdentitÃ©, mission, principes (dont la clause de souverainetÃ©)
- [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) â€” Architecture Proto-IA + LLM local
- [Miou - Systeme de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) â€” Affichage cÃ´tÃ© client
- [MWS - Document Fondateur](../../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) â€” Architecture Webway
- [Miyukini Conceptual References - Types de Services et Espaces](..//..//..//miyukini-webway-system//reference//_index.md) â€” Service Inter-COG (Type 3)
- [MiyukiniWatch â€” Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md) â€” Invariant de donnÃ©es (agrÃ©gats uniquement)

---

*Miou Inter-COG : souverainetÃ© Ã©tendue au rÃ©seau personnel. Le LLM tourne chez toi, sur ta machine, pour tous tes COGs â€” y compris ton tÃ©lÃ©phone.*

*CrÃ©Ã© : 2026-02-24*

