# Miou — Guide d'Implémentation Complet

Guide exhaustif pour implémenter Miou dans Miyukini Central : périmètre, architecture, composants, données, ordre de livraison et critères d'acceptation. Ce document est **informatif, non contractuel**. Les documents fondateurs priment.

---

## 1. Statut et bornage

### 1.1 Nature du document

| Attribut | Valeur |
|----------|--------|
| **Type** | Guide d'implémentation — informatif |
| **Contractuel** | Non. Les contrats sont dans les documents fondateurs. |
| **Objectif** | Aider le développeur à traduire la documentation Miou en code fonctionnel. |

### 1.2 Périmètre 0.1.x (bornage)

| Inclus | Exclu |
|--------|-------|
| Proto-IA (Bot) : Moteur de décision, templates, injecteur | LLM (aucune implémentation) |
| Bulles : UI, file d'attente, triggers | Onglet Chatbot Miou |
| Voix : MP3 (Rite d'Entrée, Connexion) | MIP-Miou, Miou LLM Memory |
| TTS eSpeak (optionnel, activable) | Base culture pop (usage dans templates) |
| Paramètres : toggles Voix, TTS, LLM (présent mais inactif) | Paliers d'attachement (simplifié : relation_level = 0) |
| Intégration MiyukiniWatch, JayKoa, profil | Questions curiosité, confirmation relation |
| Réponses utilisateur (stockage chiffré) | Gamification badges (hors scope initial) |
| Specs machine (RAM, stockage, upgrade) | Mode DND, templates saisonniers |

**Règle :** En 0.1.x, Miou fournit des bulles fonctionnelles (accueil, pause, rappel événement, retour, suggestions). La sophistication (paliers, curiosité, gamification) est progressive.

---

## 2. Architecture globale

### 2.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        MIYUKINI CENTRAL (app.rs)                         │
├─────────────────────────────────────────────────────────────────────────┤
│  Écrans : Rite d'Entrée │ Connexion │ Salon │ Services │ Paramètres     │
│  Overlay : Bulle Miou (position absolute, bas-droite)                   │
│  Audio : play_voice_background (MP3), play_tts_background (eSpeak)       │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           MODULE MIOU (apps/central/src/miou/)           │
├─────────────────────────────────────────────────────────────────────────┤
│  context.rs    │ Constructeur BotContext (MiyukiniWatch, profil, etc.)  │
│  engine.rs     │ Moteur de décision (priorités, conditions)              │
│  templates.rs  │ Banque de templates, sélection variante, injecteur      │
│  bubble.rs     │ Logique file d'attente, délai, max/session               │
│  state.rs      │ AppState Miou (current, queue, last_shown, count)       │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        SOURCES DE DONNÉES                                │
├─────────────────────────────────────────────────────────────────────────┤
│  MiyukiniWatch  │ SessionSummary, ServiceUsage, FriendStatus            │
│  Profil Central │ pseudo, MiouPreferences (seuils, fréquence, TTS, LLM) │
│  JayKoa         │ get_next_event()                                       │
│  Jay1Tribu      │ get_online_friends(), get_friends_list() (optionnel)   │
│  Specs machine  │ ram_available_mb, disk_free_gb, os_type                │
│  Réponses Miou  │ miou_user_responses (chiffré, optionnel 0.1.x)         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Flux de génération d'une bulle

```
1. TRIGGER (timer 2–3s, ou événement, ou périodique 30 min)
      │
2. build_context(profile_id, watch, profile_store, app_context) → BotContext
      │
3. engine::decide(&context, &prefs) → Option<Categorie>
      │  └─ Si None ou bulles désactivées → STOP
      │
4. templates::select_variante(categorie, &history) → Option<variante_id>
      │  └─ Si None (toutes utilisées) → fallback variante par défaut
      │
5. templates::inject_variables(template, &context) → String (texte final)
      │
6. bubble::enqueue(BulleOutput { texte, actions, type })
      │
7. UI : afficher bulle, play TTS si activé
      │
8. On dismiss : history.record(categorie, variante_id), count++
```

---

## 3. Arborescence code

### 3.1 Fichiers à créer / modifier

```
apps/central/src/
├── app.rs                      # [MODIFIER] Intégration overlay bulle, état Miou
├── audio.rs                   # [EXISTANT] + play_tts_background(), is_espeak_available()
├── data.rs                    # [MODIFIER] Référence base miou_user_responses si implémentée
├── screens/
│   ├── connexion.rs           # [EXISTANT] Voix Miou (déjà fait)
│   ├── rite_entree.rs         # [EXISTANT] Voix Miou (déjà fait)
│   └── home.rs                # [MODIFIER] Paramètres Miou (section)
└── miou/                      # [CRÉER] Module Miou
    ├── mod.rs                 # Exports publics
    ├── context.rs             # BotContext, build_context()
    ├── engine.rs              # decide(), conditions prioritaires
    ├── templates.rs           # Banque, select_variante(), inject_variables()
    ├── bubble.rs              # BulleOutput, enqueue(), état file
    ├── state.rs               # MiouState (current, queue, last_shown, count)
    └── triggers.rs            # Timer, périodique, événements → appels engine
```

### 3.2 Dépendances Cargo

```toml
# apps/central/Cargo.toml — existant
rodio = { version = "0.21", default-features = false, features = ["playback", "mp3"] }
# Pour WAV (TTS eSpeak) : rodio supporte WAV via le decoder — vérifier feature si besoin
```

---

## 4. Structures de données

### 4.1 BotContext (complet 0.1.x)

```rust
/// Contexte passé au moteur de décision et à l'injecteur.
/// Toutes les sources sont agrégées ici.
pub struct BotContext {
    // Session
    pub session_start: Option<DateTime<Utc>>,
    pub session_duration_minutes: u32,
    pub is_first_connection_of_session: bool,

    // Profil
    pub pseudo: String,
    pub langue: String,

    // MiyukiniWatch (ou défaut)
    pub jours_depuis_derniere_visite: Option<u32>,
    pub service_le_plus_utilise: Option<(String, u32)>,  // (nom, minutes)
    pub service_delaisse: Option<(String, u32)>,       // (nom, jours)
    pub ami_plus_delaisse: Option<(String, u32)>,       // (pseudo, jours)
    pub ami_connecte_recemment: Option<String>,

    // JayKoa
    pub evenement_prochain: Option<(String, DateTime<Utc>)>,
    pub evenement_dans_moins_d_une_heure: bool,

    // Paramètres
    pub seuil_pause_minutes: u32,
    pub max_bulles_par_session: u32,
    pub bulles_deja_affichees: u32,
    pub delai_min_entre_bulles_secs: u32,
    pub bulles_actives: bool,
    pub dnd_actif: bool,
    pub rappels_pause_actives: bool,

    // Specs machine
    pub ram_available_mb: u32,
    pub disk_free_gb: f32,
    pub os_type: String,  // "windows" | "linux" | "macos"
    pub specs_upgraded_since_last: bool,

    // Réponses utilisateur (0.1.x : optionnel, HashMap vide si non implémenté)
    pub user_responses: HashMap<String, String>,

    // Relation (0.1.x : simplifié, toujours 0 "inconnue")
    pub relation_level: u8,
}
```

### 4.2 MiouPreferences (persisté)

```rust
pub struct MiouPreferences {
    pub bulles_actives: bool,
    pub dnd_actif: bool,
    pub frequence: FrequenceBulles,  // Discret | Normal | Bavard
    pub seuil_pause_minutes: u32,
    pub rappels_pause_actives: bool,
    pub voix_enabled: bool,          // Master : MP3 + TTS
    pub tts_enabled: bool,           // TTS eSpeak
    pub voix_salon_enabled: bool,
    pub son_bulles_enabled: bool,
    pub llm_enabled: bool,           // Inactif en 0.1.x, persisté
}

pub enum FrequenceBulles {
    Discret,   // max 2, delai 120s
    Normal,    // max 5, delai 30s
    Bavard,    // max 10, delai 15s
}
```

### 4.3 BulleOutput

```rust
pub struct BulleOutput {
    pub texte: String,
    pub categorie: String,        // "accueil_matin", "pause_sante", etc.
    pub actions: Vec<BulleAction>,
}

pub struct BulleAction {
    pub label: String,
    pub action_type: ActionType,  // OuvrirService, Pause, Dismiss, etc.
    pub payload: Option<String>, // service_id, etc.
}
```

### 4.4 MiouState (état UI)

```rust
pub struct MiouState {
    pub current_bulle: Option<BulleOutput>,
    pub queue: VecDeque<BulleOutput>,
    pub last_shown_at: Option<DateTime<Utc>>,
    pub bulles_count_this_session: u32,
    pub session_id: String,
}
```

---

## 5. Moteur de décision (engine.rs)

### 5.1 Ordre de priorité (0.1.x minimal)

| P | Catégorie | Condition | Référence |
|---|-----------|-----------|-----------|
| 0 | — | `!bulles_actives` ou `dnd_actif` → silence | Catalogue Triggers C-10, C-21 |
| 1 | pause_sante | `session_duration >= seuil` ET `rappels_pause_actives` | C-01 |
| 2 | rappel_evenement | `evenement_prochain` ET `start - now < 1h` | E-30 |
| 3 | accueil_* | `is_first_connection` ET `bulles_count == 0` | E-01 |
| 4 | retour_absence | `jours_absent >= 3` ET `is_first_connection` | C-02 |
| 5 | rappel_ami | `ami_plus_delaisse.jours >= 7` | C-03 |
| 6 | suggestion_service | `service_delaisse.jours >= 14` | C-04 |
| 7 | specs_* | RAM < 512, disk < 1 Go, upgrade | C-09, C-17, C-14 |
| — | silence | Aucune condition | — |

### 5.2 Exclusions (avant décision)

- `bulles_deja_affichees >= max_bulles_par_session` → silence (sauf P0 pause/événement si config)
- `now - last_shown_at < delai_min_entre_bulles_secs` → silence
- Pause déjà affichée et dismissée < 2h → ne pas répéter
- Événement déjà rappelé cette session → exclure

### 5.3 Signature

```rust
pub fn decide(context: &BotContext) -> Option<&'static str> {
    // Retourne la catégorie (ex. "accueil_matin") ou None
}
```

---

## 6. Templates et injecteur

### 6.1 Banque minimale 0.1.x

| Catégorie | Variantes (exemples) |
|-----------|----------------------|
| accueil_matin | "Bonjour {pseudo} !", "Salut {pseudo}, une nouvelle journée commence." |
| accueil_apres_midi | "Hey {pseudo}, contente de te voir.", "Bonne après-midi {pseudo}." |
| accueil_soir | "Bonsoir {pseudo}.", "Hey {pseudo}, bonne soirée sur ton COG." |
| pause_sante | "Ça fait {duree} — accorde-toi une pause.", "Pause ? Tu en as bien besoin après {duree}." |
| rappel_evenement | "{evenement} dans moins d'une heure.", "Rappel : {evenement} bientôt." |
| retour_absence | "Te revoilà {pseudo} ! Ça fait {jours} jours.", "Contente de te revoir après {jours} jours." |
| rappel_ami | "Tu n'as pas donné de nouvelles à {ami} depuis {jours} jours." |
| suggestion_service | "Tu n'as pas ouvert {service} depuis un moment." |
| specs_ram_demande | "J'aimerais un peu plus de RAM pour mieux te servir." |
| specs_stockage_demande | "Mon disque s'essouffle — un peu de ménage ?" |
| specs_upgrade_commentaire | "Tu as amélioré la machine — merci !" |

### 6.2 Variables injectables

| Variable | Source BotContext | Exemple |
|----------|-------------------|---------|
| {pseudo} | context.pseudo | "Kaito" |
| {jours} | jours_depuis_derniere_visite ou ami/service | "5" |
| {duree} | format_duree(session_duration_minutes) | "2h15" |
| {service} | service_delaisse.0 | "JayKoa" |
| {ami} | ami_plus_delaisse.0 | "Luna" |
| {evenement} | evenement_prochain.0 | "Réunion équipe" |

### 6.3 Sélection variante

- Stocker les variantes utilisées cette session (et optionnellement 2–3 sessions).
- Anti-répétition : choisir une variante non utilisée récemment.
- Fallback : si toutes utilisées, reprendre la première.

---

## 7. Constructeur de contexte (context.rs)

### 7.1 Algorithme

```
1. Initialiser BotContext avec valeurs par défaut
2. Charger profil → pseudo, MiouPreferences
3. SessionSummary depuis MiyukiniWatch (ou 0, None)
4. ServiceUsage[], FriendStatus[] depuis MiyukiniWatch
5. Prochain événement depuis JayKoa (si service disponible)
6. ami_connecte_recemment depuis Jay1Tribu (si dispo)
7. Specs machine (sysinfo ou équivalent)
8. Appliquer prefs (seuil, max_bulles, delai)
9. Retourner context
```

### 7.2 Gestion des absences

| Source indisponible | Comportement |
|---------------------|--------------|
| MiyukiniWatch | session_duration=0, jours=None, service_delaisse=None, etc. |
| Profil | pseudo="habitant", prefs=défaut |
| JayKoa | evenement_prochain=None |
| Jay1Tribu | ami_connecte_recemment=None |
| Sysinfo | ram=0, disk=0, os="unknown" |

Aucun crash. Contexte minimal exploitable.

---

## 8. Triggers et orchestration

### 8.1 Triggers 0.1.x

| Trigger | Moment | Action |
|---------|--------|--------|
| T-SALON | 2–3 s après affichage Salon | build_context → decide → afficher si catégorie |
| T-PERIODIQUE | Toutes les 30 min | Idem (vérifier pause, événement) |
| T-EVENT | JayKoa event < 1h, ou ami connecté | Événement → rebuild → decide |
| T-DISMISS | Utilisateur ferme bulle | last_shown = now, count++, history.record |

### 8.2 Intégration dans app.rs

- Au montage du Salon : `spawn` timer 2–3 s → `miou_triggers::maybe_show_bulle()`
- Pendant session : timer 30 min ou événement Jay1Tribu
- État `MiouState` dans `AppState` ou `use_context`

---

## 9. Composant UI bulle

### 9.1 Spécifications (Système de Bulles)

| Attribut | Valeur |
|----------|--------|
| Position | Bas-droite, 16px marge |
| Largeur max | 360px (280px si fenêtre 480–800px) |
| Z-index | Au-dessus contenu, sous modales |
| Animation | Fade-in + slide-up 200ms |
| Contenu | Avatar 🌸, "Miou", texte, boutons actions, ✕ |

### 9.2 Structure Dioxus

```rust
#[component]
fn MiouBubbleOverlay(
    bubble: Signal<Option<BulleOutput>>,
    on_dismiss: EventHandler<()>,
    on_action: EventHandler<BulleAction>,
) -> Element {
    let bubble = bubble.read();
    let Some(b) = bubble else { return rsx! { }; };
    rsx! {
        div { class: "miou-bubble-overlay",
            div { class: "miou-bubble",
                div { "🌸 Miou" }
                button { onclick: move |_| on_dismiss.call(()), "✕" }
                p { "{b.texte}" }
                for action in b.actions {
                    button { onclick: move |_| on_action.call(action.clone()), "{action.label}" }
                }
            }
        }
    }
}
```

### 9.3 CSS (indicatif)

```
.miou-bubble-overlay { position: fixed; bottom: 16px; right: 16px; z-index: 1000; }
.miou-bubble { max-width: 360px; padding: 16px; border-radius: 12px; box-shadow: ...; }
```

---

## 10. Voix et TTS

### 10.1 Existant (Rite d'Entrée, Connexion)

- `audio::play_voice_background(base, filename)` — MP3
- Fichiers dans `voices/fr/` : `login_new_ask_*.mp3`, `login_retour_*.mp3`
- Résolution chemin : `resolve_voice_path(base, subpath)`

### 10.2 TTS eSpeak (extension)

```rust
pub fn is_espeak_available() -> bool {
    std::process::Command::new("espeak-ng").arg("--version").output().is_ok()
}

pub fn play_tts_background(text: &str) {
    if !is_espeak_available() { return; }
    std::thread::spawn(move || {
        let path = std::env::temp_dir().join(format!("miou_tts_{}.wav", uuid::Uuid::new_v4()));
        let status = std::process::Command::new("espeak-ng")
            .args(["-v", "fr", "-w", path.to_str().unwrap(), "--", text])
            .status();
        if status.is_ok() && path.exists() {
            play_wav_background(path);  // rodio decode WAV
            let _ = std::fs::remove_file(path);
        }
    });
}
```

### 10.3 Logique de lecture

- Si `voix_enabled` = false → ne jamais jouer.
- Si bulle avec texte dynamique ET `tts_enabled` → `play_tts_background(texte)`.
- Si contexte a un MP3 pré-enregistré (ex. accueil fixe) → `play_voice_background` (fallback TTS si MP3 absent).

---

## 11. Paramètres Miou

### 11.1 Emplacement

Paramètres Miyukini > Section "Miou" (carte ou sous-section).

### 11.2 Options 0.1.x

| Option | Type | Défaut | Clé |
|--------|------|--------|-----|
| Bulles activées | Toggle | true | bulles_actives |
| Fréquence | Discret/Normal/Bavard | Normal | frequence |
| Rappels de pause | 1h/2h/3h/Désactivé | 2h | seuil_pause_minutes |
| Voix Miou | Toggle | true | voix_enabled |
| TTS eSpeak | Toggle | false | tts_enabled |
| Mode LLM | Toggle | false | llm_enabled (inactif, grisé ou info) |

### 11.3 Persistance

- Stockage : profil utilisateur Central (KindMother ou fichier prefs).
- Structure : `MiouPreferences` sérialisée (JSON ou colonnes).

---

## 12. Données et persistance

### 12.1 MiyukiniWatch — Interface requise

Le Bot a besoin d'agrégats. Interface proposée (à adapter au crat miyukiniwatch) :

```rust
pub trait MiouWatchAggregator {
    fn session_summary(&self, profile_id: &str) -> SessionSummary;
    fn services_usage(&self, profile_id: &str, period: &str) -> Vec<ServiceUsage>;
    fn friends_status(&self, profile_id: &str) -> Vec<FriendStatus>;
}
```

### 12.2 JayKoa — Événement prochain

```rust
fn get_next_event(profile_id: &str) -> Option<(String, DateTime<Utc>)>;
```

### 12.3 Réponses utilisateur (optionnel 0.1.x)

- Table `miou_user_responses` : profile_id, question_id, response_text (chiffré), created_at.
- Chiffrement : SQLCipher ou équivalent (voir Connaissance Utilisateur).
- En 0.1.x : peut être reporté ; `user_responses` reste vide.

---

## 13. Ordre de livraison recommandé

| Phase | Livrable | Dépendances |
|-------|----------|-------------|
| **P1** | Constructeur contexte, BotContext | MiyukiniWatch, profil, JayKoa (stubs OK) |
| **P2** | Moteur décision, conditions P0–P4 | P1 |
| **P3** | Banque templates, injecteur | P2 |
| **P4** | Composant bulle UI, état MiouState | P3 |
| **P5** | Triggers (timer Salon, périodique) | P4 |
| **P6** | Paramètres Miou (section, persistance) | — |
| **P7** | TTS eSpeak (détection, play_tts_background) | P4, Paramètres |
| **P8** | Intégration finition (animations, responsive) | P4 |

---

## 14. Critères d'acceptation 0.1.x

### 14.1 Fonctionnels

- [ ] À l'arrivée sur le Salon (première fois) : bulle d'accueil affichée après 2–3 s.
- [ ] Après 2h de session : bulle pause santé (si seuil 2h, rappels actifs).
- [ ] Événement JayKoa < 1h : bulle rappel événement.
- [ ] Retour après 3+ jours : bulle retour absence.
- [ ] Dismiss : bulle se ferme, pas de répétition immédiate.
- [ ] Max 5 bulles/session (configurable), délai 30 s entre deux.
- [ ] Paramètres : désactiver bulles = plus aucune bulle.
- [ ] Voix Rite/Connexion : inchangée, fonctionnelle.
- [ ] TTS (si activé + eSpeak installé) : bulle vocale.

### 14.2 Non fonctionnels

- [ ] Pas de crash si MiyukiniWatch indisponible.
- [ ] Pas de crash si JayKoa/Jay1Tribu indisponibles.
- [ ] Latence build_context < 50 ms.
- [ ] Bulle visible au-dessus du contenu, fermeture fluide.

### 14.3 Exclus 0.1.x

- LLM : pas d'implémentation.
- Paliers, questions curiosité, confirmation relation.
- Mode DND (toggle peut exister, non fonctionnel ou basique).
- Gamification badges (hors scope).

---

## 15. Références

| Document | Lien |
|----------|------|
| Miou - Document Fondateur | [Document Fondateur](./Miou%20-%20Document%20Fondateur.md) |
| Miou - Système de Bulles et UI | [Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) |
| Miou - Moteur de Génération Templates et LLM | [Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) |
| Miou - Intégration TTS eSpeak | [TTS eSpeak](./Miou%20-%20Integration%20TTS%20eSpeak.md) |
| Miou - Voix et Audio | [Voix et Audio](./Miou%20-%20Voix%20et%20Audio.md) |
| Bot - Moteur de Décision et Règles | [Moteur Décision](./Bot/Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md) |
| Bot - Intégration et Flux de Données | [Intégration Bot](./Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) |
| Bot - Catalogue Complet des Triggers | [Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md) |
| Bot - Banque de Templates Volume 2 | [Templates Vol 2](./Bot/Bot%20-%20Banque%20de%20Templates%20Volume%202.md) |
| Bot - Connaissance Utilisateur et Specs Machine | [Connaissance](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) |
| MiyukiniWatch — Document Fondateur | [MiyukiniWatch](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md) |

---

**Version :** 1.0  
**Statut :** Guide d'implémentation — informatif  
**Périmètre :** Miou 0.1.x (Proto-IA, bulles, voix, TTS, paramètres)
