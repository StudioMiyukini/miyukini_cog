# Miou — Voix et Audio

Système audio de Miou : voix existante (Rite d'Entrée, Connexion), cohérence sonore, roadmap pour le Salon et au-delà.

---

## 1. Système existant

Miou dispose déjà d'une voix sur deux écrans de Central :

### 1.1 Rite d'Entrée (COG vierge)

Miou guide le nouvel habitant à travers trois étapes (Nom, Email, Clé). Chaque étape déclenche un fichier voix :

| Étape | Fichier | Phrase associée (texte) |
|-------|---------|------------------------|
| Nom | `login_new_ask_name.mp3` | « Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ? » |
| Email | `login_new_ask_email.mp3` | « Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ? » |
| Clé | `login_new_ask_password.mp3` | « Pour finir, peux-tu me donner une clé pour protéger l'entrée ? » |

### 1.2 Connexion (retour d'un habitant connu)

Miou accueille l'habitant avec une phrase aléatoire parmi trois, chacune associée à un fichier audio :

| Variante | Fichier | Phrase |
|----------|---------|--------|
| a | `login_retour_a.mp3` | « Quelle bonne surprise. Entre donc avec ta clé et rejoins moi à l'intérieur. » |
| b | `login_retour_b.mp3` | « Te voilà de retour. Entre donc avec ta clé et rejoins moi à l'intérieur. » |
| c | `login_retour_c.mp3` | « J'étais si impatiente de te revoir. Entre donc avec ta clé et rejoins moi à l'intérieur. » |

### 1.3 Infrastructure technique

| Élément | Détail |
|---------|--------|
| **Module** | `apps/central/src/audio.rs` |
| **Bibliothèque** | `rodio` (crate Rust, feature `mp3`) |
| **Format** | MP3 |
| **Chemin** | `{base}/voices/fr/{filename}` — résolution multi-chemins (base, parent, exe dir) via `resolve_voice_path()` |
| **Exécution** | Thread dédié (`std::thread::spawn`), non-bloquant pour l'UI |
| **Fallback Windows** | Si rodio échoue : `cmd /C start /min {path}` |
| **Localisation** | Sous-dossier `fr` — structure prête pour d'autres langues (`en`, `es`, etc.) |

---

## 2. Principes sonores de Miou

| Principe | Description |
|----------|-------------|
| **Voix = identité** | La voix de Miou est un marqueur d'identité fort. Elle doit rester cohérente (même timbre, même ton) à travers tous les écrans. |
| **Optionnelle** | La voix est toujours désactivable par l'utilisateur (Paramètres > Miou). Le texte reste visible dans tous les cas. |
| **Non intrusive** | Pas de son imprévu. Le son ne se déclenche que sur des moments attendus (connexion, bulle importante). |
| **Courte** | Chaque fichier audio dure 3–8 secondes max. Miou ne fait pas de discours. |

---

## 3. Roadmap : voix au-delà de Connexion

### 3.1 Voix sur le Salon (P2)

Ajouter un son ponctuel à l'arrivée dans le Salon (première bulle de la session) :

| Moment | Son | Phrase texte associée |
|--------|-----|----------------------|
| Arrivée matin | `salon_bonjour.mp3` | « Bonjour [pseudo] ! » (voix Miou, ~2s) |
| Arrivée soir | `salon_bonsoir.mp3` | « Bonsoir [pseudo]. » (voix Miou, ~2s) |
| Retour après absence | `salon_retour.mp3` | « Te voilà de retour. » (voix Miou, ~2s) |

**Règle :** Un seul son par session (la première bulle). Pas de voix sur les bulles suivantes sauf opt-in explicite.

### 3.2 Sons d'ambiance des bulles (P3)

Son très court (~0.5s) accompagnant l'apparition d'une bulle :

| Type de bulle | Son | Description |
|---------------|-----|-------------|
| Toutes | `bubble_appear.mp3` | « Pop » discret, léger, plaisant. |
| Félicitation | `bubble_congrats.mp3` | Petit carillon joyeux (~1s). |

**Règle :** Désactivé par défaut. Activable dans Paramètres > Miou > « Son des bulles ».

### 3.3 TTS eSpeak (voix dynamique)

Pour les textes **dynamiques** (bulles avec variables, messages générés), Miou utilise **eSpeak NG** comme moteur TTS léger :

- **Empreinte** : ~2 Mo, RAM ~5–10 Mo — compatible configs modestes
- **Français** : support natif, 100 % offline
- **Configuration** : activable/désactivable dans Paramètres > Miou > TTS eSpeak
- **Stratégie hybride** : MP3 prioritaire quand disponibles, TTS en fallback pour texte dynamique

Voir [Miou - Intégration TTS eSpeak](./Miou%20-%20Integration%20TTS%20eSpeak.md) pour les détails d'intégration.

### 3.4 Voix LLM (futur, P4+)

À très long terme, si Miou dispose d'un LLM conversationnel : TTS pour vocaliser les réponses générées. eSpeak couvre déjà le besoin ; Piper (meilleure qualité) possible si ressources suffisantes. **LLM non implémenté en 0.1.x** — le toggle est présent dans Paramètres pour préparation.

---

## 4. Organisation des fichiers audio

```
voices/
└── fr/
    ├── login_new_ask_name.mp3          # Rite d'Entrée — étape Nom
    ├── login_new_ask_email.mp3         # Rite d'Entrée — étape Email
    ├── login_new_ask_password.mp3      # Rite d'Entrée — étape Clé
    ├── login_retour_a.mp3              # Connexion — variante a
    ├── login_retour_b.mp3              # Connexion — variante b
    ├── login_retour_c.mp3              # Connexion — variante c
    ├── salon_bonjour.mp3               # (futur) Salon — matin
    ├── salon_bonsoir.mp3               # (futur) Salon — soir
    ├── salon_retour.mp3                # (futur) Salon — retour après absence
    ├── bubble_appear.mp3               # (futur) Son apparition bulle
    └── bubble_congrats.mp3             # (futur) Son félicitation
```

**Convention de nommage :** `{ecran_ou_contexte}_{variante_ou_action}.mp3`

---

## 5. Configuration utilisateur

Dans **Paramètres Miyukini > Miou** :

| Option | Clé | Défaut | Description |
|--------|-----|--------|-------------|
| **Voix Miou** | `miou_voice_enabled` | Activée | Master switch : désactiver = aucun son (MP3 + TTS). |
| **TTS eSpeak** | `miou_tts_enabled` | Désactivé | Synthèse vocale pour textes dynamiques (bulles, etc.). |
| **Voix Salon** | `miou_voice_salon_enabled` | Désactivée | Son d'accueil sur le Salon (MP3 ou TTS). |
| **Son des bulles** | `miou_bubble_sound_enabled` | Désactivé | Petit « pop » à l'apparition d'une bulle. |
| **Mode LLM** | `miou_llm_enabled` | Désactivé | Active le LLM si disponible. **Non implémenté en 0.1.x** — toggle présent. |
| **Volume Miou** | (futur) | Système | Contrôle de volume dédié. |

**Règle :** TTS et LLM sont indépendants. Le TTS vocalise le texte (Proto-IA ou LLM). Le LLM génère le texte — sans LLM en 0.1.x, Miou reste en Proto-IA.

---

## 6. Références

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Système de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md)
- [Miou - Intégration TTS eSpeak](./Miou%20-%20Integration%20TTS%20eSpeak.md)
- Code existant : `apps/central/src/audio.rs` (résolution chemin, lecture rodio, fallback Windows)
- Code existant : `apps/central/src/screens/connexion.rs` (phrases Miou, sélection aléatoire, déclenchement audio)
- Code existant : `apps/central/src/screens/rite_entree.rs` (voix par étape)

---

*La voix de Miou : courte, chaleureuse, optionnelle. MP3 pour les écrans fixes, eSpeak pour les bulles dynamiques. Un marqueur d'identité qui prolonge la présence de l'avatar au-delà du texte.*
