# Miou — Intégration TTS eSpeak

Ce document spécifie l'intégration du **TTS eSpeak NG** pour Miou : moteur léger, configuration utilisateur (activation/désactivation), stratégie hybride avec les fichiers MP3 pré-enregistrés, et périmètre 0.1.x.

---

## 1. Contexte et objectifs

| Objectif | Description |
|----------|-------------|
| **Voix dynamique** | Vocaliser du texte généré (bulles, chatbot) sans pré-enregistrement |
| **Légèreté** | Compatible configs modestes (RAM < 512 Mo, machines sans GPU) |
| **Contrôle utilisateur** | Le TTS peut être activé/désactivé dans Paramètres > Miou |
| **Cohérence** | Stratégie hybride : MP3 prioritaire quand disponibles, TTS en fallback |

**Choix eSpeak NG :**
- Empreinte ~2 Mo (programme + données), RAM ~5–10 Mo
- Français natif, 100 % offline
- Synthèse par formants : voix synthétique mais claire et compréhensible

---

## 2. Stratégie hybride : MP3 vs TTS

### 2.1 Priorité

| Priorité | Source | Condition |
|----------|--------|-----------|
| 1 | **Fichiers MP3** pré-enregistrés | Si le fichier existe pour le contexte (Rite d'Entrée, Connexion, Salon) |
| 2 | **TTS eSpeak** | Si TTS activé ET (pas de MP3 OU texte dynamique) |

### 2.2 Où le TTS s'applique

| Contexte | MP3 | TTS (si activé) |
|----------|-----|-----------------|
| Rite d'Entrée (étapes) | `login_new_ask_*.mp3` | Fallback si fichier absent |
| Connexion (retour) | `login_retour_*.mp3` | Fallback si fichier absent |
| Salon — accueil | `salon_bonjour.mp3` etc. | Fallback + texte avec `{pseudo}` dynamique |
| Bulles (texte dynamique) | Non applicable | **Principal usage** : texte généré par Proto-IA |
| Chatbot (futur) | Non applicable | Réponses vocales si activé |

**Règle :** Pour les bulles avec variables (`{pseudo}`, `{jours}`, etc.), le TTS est la seule option — pas de pré-enregistrement possible.

---

## 3. Configuration utilisateur

### 3.1 Paramètres Miyukini > Miou

| Option | Clé | Défaut | Description |
|--------|-----|--------|-------------|
| **Voix Miou** | `miou_voice_enabled` | Activé | Master switch : désactive tout son (MP3 + TTS). |
| **TTS eSpeak** | `miou_tts_enabled` | Désactivé | Active la synthèse vocale pour les textes dynamiques. Inactif si Voix Miou = non. |
| **Voix Salon** | `miou_voice_salon_enabled` | Désactivé | Son d'accueil sur le Salon (MP3 ou TTS). |
| **Son des bulles** | `miou_bubble_sound_enabled` | Désactivé | Petit « pop » à l'apparition d'une bulle (fichier fixe). |
| **Mode LLM** | `miou_llm_enabled` | Désactivé | Active le LLM si disponible. **Non implémenté en 0.1.x** — toggle présent pour préparation. |

### 3.2 Logique d'activation

```
Jouer son pour texte T ?
├── miou_voice_enabled = false → NE PAS JOUER
├── Contexte a un MP3 ET pas de texte dynamique ?
│   └── OUI → Jouer MP3 (rodio)
└── TTS requis (texte dynamique OU MP3 absent)
    ├── miou_tts_enabled = false → NE PAS JOUER (texte reste visible)
    └── miou_tts_enabled = true → Jouer via eSpeak (TTS)
```

### 3.3 LLM : périmètre 0.1.x

| Aspect | 0.1.x |
|--------|-------|
| **Implémentation LLM** | Aucune — Miou fonctionne à 100 % en Proto-IA |
| **Toggle LLM** | Présent dans Paramètres > Miou > Intelligence — désactivé, non fonctionnel |
| **Objectif** | Préparer l'UI et la persistence pour les versions futures |

**Règle :** Le toggle LLM est affiché mais ne déclenche aucune logique d'inférence en 0.1.x. L'utilisateur peut l'activer/désactiver ; la valeur est persistée pour compatibilité future.

---

## 4. Intégration technique eSpeak

### 4.1 Dépendance système

| Plateforme | Installation |
|------------|--------------|
| **Windows** | `espeak-ng` via [releases](https://github.com/espeak-ng/espeak-ng/releases) ou Chocolatey : `choco install espeak-ng` |
| **Linux** | `sudo apt install espeak-ng` / `sudo dnf install espeak-ng` |
| **macOS** | `brew install espeak-ng` |

**Détection :** Au démarrage, Central vérifie la présence du binaire `espeak-ng` dans le PATH. Si absent → TTS désactivé automatiquement, pas d'erreur utilisateur (option grisée ou message discret dans Paramètres).

### 4.2 Invocation

```bash
# Synthèse fichier WAV temporaire — texte passé en argument
espeak-ng -v fr -w output.wav "Bonjour, comment vas-tu ?"

# -v fr   : voix française
# -w PATH : fichier WAV de sortie
# Texte  : dernier argument (échapper les guillemets si besoin)
```

**Flux :**
1. Texte à vocaliser (ex. « Bonjour {pseudo} ! »)
2. `std::process::Command::new("espeak-ng")` avec args `["-v", "fr", "-w", temp_path, "--", &text]`
3. Fichier WAV temporaire créé
4. Lecture via `rodio` (déjà utilisé pour MP3) — rodio supporte WAV
5. Suppression du fichier temporaire après lecture

### 4.3 Exécution non-bloquante

Comme pour les MP3 actuels (`play_voice_background`), le TTS s'exécute dans un **thread dédié** :
- Pas de blocage UI
- Lecture asynchrone
- Gestion des erreurs silencieuse (si eSpeak échoue → pas de son, pas de popup)

### 4.4 Module Rust (structure proposée)

```
apps/central/src/audio.rs
├── resolve_voice_path()      # Existant
├── play_voice_background()   # Existant — MP3
├── play_tts_background()     # Nouveau — texte → eSpeak → WAV → rodio
└── is_espeak_available()     # Nouveau — détection binaire
```

### 4.5 Paramètres eSpeak (optionnel)

| Paramètre | Valeur par défaut | Description |
|-----------|-------------------|-------------|
| Langue | `fr` | Français |
| Vitesse | 150–170 mots/min | Ajustable si besoin (option future dans Paramètres) |
| Volume | 100 | Pas de modification côté eSpeak (volume système) |

---

## 5. Schéma de configuration persistée

```rust
// Exemple de structure (KindMother ou fichier préférences)
struct MiouUserPreferences {
    voice_enabled: bool,        // Voix Miou (master)
    tts_enabled: bool,          // TTS eSpeak
    voice_salon_enabled: bool,
    bubble_sound_enabled: bool,
    llm_enabled: bool,          // Toggle présent, non utilisé en 0.1.x
    // ...
}
```

---

## 6. Références

- [Miou - Voix et Audio](./Miou%20-%20Voix%20et%20Audio.md)
- [Miou - Proto-IA Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md)
- [Miou - Guide UI UX](./Miou%20-%20Guide%20UI%20UX.md) — section Paramètres
- eSpeak NG : https://github.com/espeak-ng/espeak-ng

---

**Version :** 1.0  
**Statut :** Spécification intégration TTS eSpeak  
**Périmètre 0.1.x :** TTS eSpeak intégrable ; LLM non implémenté, toggle présent
