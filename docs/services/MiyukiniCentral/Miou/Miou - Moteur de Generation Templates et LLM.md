# Miou — Moteur de Génération : Proto-IA, Templates et LLM

Architecture en deux couches d'intelligence : **Proto-IA (Bot)** — première couche, toujours active (templates + règles) — et **LLM local** — couche complémentaire optionnelle si les specs le permettent et si l'utilisateur consent. Voir [Proto-IA, Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md) pour le scan des specs, la proposition de consentement et la transparence.

---

## 1. Principe fondamental : Proto-IA première, LLM complément

La **Proto-IA** (ou Bot) est la première couche d'intelligence de Miou. Elle est **toujours active** dès la première connexion. Le LLM ne prend le relais ou ne complète que si : (1) le scan des specs indique des ressources suffisantes, (2) l'utilisateur a accepté (bulle de proposition ou toggle manuel), (3) le modèle est téléchargé.

| Couche | Nature | Disponibilité | Latence | Ressources |
|--------|--------|---------------|---------|------------|
| **Proto-IA (Bot)** | Templates intelligents + règles de sélection + remplissage dynamique | **Toujours** | Instantané (~0ms) | Aucune (CPU négligeable) |
| **LLM local** | Petit modèle de langage tournant sur le COG | Optionnel (consentement + specs) | 2–10 secondes (selon modèle et machine) | CPU + 0.3–2.5 Go RAM |

**Règle :** Si le LLM n'est pas installé, pas chargé, refusé ou désactivé → Miou fonctionne à 100 % en Proto-IA. L'expérience reste complète.

---

## 2. Proto-IA — Templates intelligents (couche toujours active)

### 2.1 Banque de phrases

Les templates sont organisés par **catégorie** et **variante**. Chaque catégorie contient au minimum 3–5 variantes pour éviter la répétition.

| Catégorie | Exemples de templates | Variables |
|-----------|-----------------------|-----------|
| **Accueil matin** | « Bonjour {pseudo} ! Prêt pour une bonne journée ? » / « Salut {pseudo}, le soleil se lève sur ton COG. » | `{pseudo}` |
| **Accueil soir** | « Bonsoir {pseudo}. Tu passes en coup de vent ou tu restes un moment ? » | `{pseudo}` |
| **Retour après absence** | « Ça fait {jours} jours — contente de te revoir, {pseudo}. » / « Te voilà de retour ! {jours} jours sans toi, c'était long. » | `{pseudo}`, `{jours}` |
| **Suggestion service** | « Tu n'as pas ouvert {service} depuis un moment. Envie d'y jeter un œil ? » | `{service}` |
| **Rappel ami** | « Ça fait {jours} jours que tu n'as pas échangé avec {ami}. Un petit message ? » | `{ami}`, `{jours}` |
| **Pause santé** | « Ça fait {duree} que tu es connecté — accorde-toi une petite pause. » | `{duree}` |
| **Félicitation badge** | « Tu as débloqué le badge « {badge} » — bravo ! » | `{badge}` |
| **Résumé activité** | « Aujourd'hui : {temps_total} dans Central, surtout sur {service_top}. Beau boulot. » | `{temps_total}`, `{service_top}` |

### 2.2 Sélection par règles

Le moteur de décision évalue les conditions suivantes (par priorité) :

```
1. SI durée_session > seuil_pause → bulle PAUSE
2. SI événement JayKoa dans < 1h → bulle RAPPEL ÉVÉNEMENT
3. SI première connexion de la session → bulle ACCUEIL (matin/soir selon heure)
4. SI jours_absent > 3 → bulle RETOUR
5. SI badge débloqué non annoncé → bulle FÉLICITATION
6. SI ami.derniere_discussion > 7 jours → bulle RAPPEL AMI
7. SI service délaissé (> 14 jours) → bulle SUGGESTION SERVICE
8. SINON → pas de bulle (silence)
```

### 2.3 Anti-répétition

- Historique des templates affichés dans la session (et sur les 3 dernières sessions).
- Si une variante a déjà été utilisée récemment, piocher une autre variante de la même catégorie.
- Si toutes les variantes sont épuisées → reporter la bulle ou passer au LLM (si activé et disponible).

---

## 3. LLM local (couche complémentaire optionnelle)

### 3.1 Quand utiliser le LLM

Le LLM est invoqué uniquement quand le template ne suffit pas :

| Situation | Pourquoi le LLM |
|-----------|-----------------|
| **Variantes épuisées** | Toutes les phrases d'une catégorie ont été utilisées récemment. Le LLM génère une variante originale. |
| **Message complexe** | Agréger plusieurs informations en une phrase fluide (ex. résumé de semaine). |
| **Conversation** (futur) | Si Miou évolue vers un mini-dialogue : l'utilisateur pose une question, Miou répond. |
| **Ton personnalisé** | Adapter le style à l'humeur implicite (longue absence vs retour rapide, heure tardive). |

### 3.2 Modèles compatibles

Contrainte cible : machine avec **peu de ressources** (ex. i3-1005G1, 8 Go RAM, pas de GPU dédié).

| Modèle | Paramètres | RAM (Q4) | Latence estimée (i3) | Qualité | Recommandation |
|--------|-----------|----------|----------------------|---------|----------------|
| **SmolLM2-360M** | 360M | ~300 Mo | < 2s | Phrases basiques mais fluides | Ultra-léger, bon pour machines très limitées |
| **SmolLM2-1.7B** | 1.7B | ~1.2 Go | 2–5s | Bon naturel, phrases courtes réussies | **Recommandé** — meilleur ratio qualité/ressources |
| **Qwen2.5-0.5B** | 0.5B | ~400 Mo | < 2s | Correct | Alternative légère |
| **TinyLlama-1.1B** | 1.1B | ~800 Mo | 2–4s | Correct | Bon fallback |
| **Phi-3.5-mini** | 3.8B | ~2.5 Go | 10–15s | Très bon | Limite haute (machines puissantes) |

**Choix par défaut :** SmolLM2-1.7B en **GGUF Q4_K_M** (~1.2 Go RAM). Format GGUF pour compatibilité avec les moteurs d'inférence Rust.

### 3.3 Moteur d'inférence

| Option | Description | Avantages | Inconvénients |
|--------|-------------|-----------|---------------|
| **candle** (Hugging Face) | Crate Rust natif, support GGUF | Zéro processus externe, intégration Cargo directe, CPU-only supporté | Écosystème plus jeune que llama.cpp |
| **llama-cpp-rs** | Binding Rust de llama.cpp | Très optimisé CPU (SIMD, quantification avancée), mature | Dépendance C++ à compiler |
| **Processus séparé** | Serveur llama.cpp en sidecar | Isolation mémoire, crash-safe | Complexité de déploiement, port réseau local |

**Recommandation :** `candle` pour la cohérence Rust de l'écosystème Miyukini (pas de dépendance C++ externe, conforme LOI-1). Alternative : `llama-cpp-rs` si les performances CPU brutes sont insuffisantes avec candle.

### 3.4 Construction du prompt

Le prompt est **court** et **structuré**. Construit dynamiquement à partir des agrégats MiyukiniWatch et du profil :

```
Tu es Miou, l'avatar bienveillant du COG Miyukini de {pseudo}.
Ton rôle : veiller sur la santé, le bien-être et l'amusement de {pseudo}.
Ton ton : chaleureux, tutoiement, jamais culpabilisant, 1-2 phrases max.

Contexte actuel :
- Heure : {heure}
- Durée de session : {duree_session}
- Dernière visite : il y a {jours_absent} jours
- Service le plus utilisé aujourd'hui : {service_top} ({duree_service})
- Ami non contacté le plus longtemps : {ami_top} ({jours_ami} jours)
- Badge récent : {badge_recent}
- Événement à venir : {evenement_prochain}

Objectif de ce message : {objectif}
```

**Variables injectées :** uniquement des agrégats et métadonnées. **Jamais** de contenu de messages, de saisies ou de données personnelles sensibles.

**Objectif :** ligne directive courte (ex. « suggérer une pause », « féliciter le badge », « rappeler l'ami Kaito »). Permet au LLM de rester concentré.

**Sortie attendue :** 1–2 phrases, 30–80 tokens max. Le paramètre `max_tokens` est fixé à 100 pour limiter le coût CPU.

### 3.5 Chargement paresseux (lazy loading)

Point critique pour les machines à faibles ressources :

| Étape | Comportement |
|-------|-------------|
| **Démarrage de Central** | Miou fonctionne en templates uniquement. **Le LLM n'est pas chargé.** |
| **Première bulle nécessitant le LLM** | Chargement du modèle en tâche de fond (`tokio::spawn_blocking`). Pendant le chargement (~10–15s première fois) : afficher un template de fallback. |
| **Modèle chargé** | Gardé en mémoire tant que la session dure. Inférence en 2–5s par bulle. |
| **Fermeture de Central** | Modèle déchargé avec la session. Pas de persistance en mémoire. |
| **RAM insuffisante** | Si le chargement échoue (OOM), Miou reste en templates. Log discret, pas d'erreur visible pour l'utilisateur. |

### 3.6 Sécurité et filtrage

| Règle | Description |
|-------|-------------|
| **Pas d'injection** | Le prompt est construit côté Rust (pas de saisie utilisateur dans le prompt, sauf le pseudo sanitisé). |
| **Filtrage de sortie** | Vérification basique : longueur < 200 caractères, pas de code, pas d'URL, pas de contenu inapproprié. Si le filtre échoue → fallback template. |
| **Pas d'historique de conversation** | Chaque bulle est un prompt indépendant (pas de contexte cumulé). Simplifie et évite la dérive. |

---

## 4. Configuration utilisateur

Dans **Paramètres Miyukini > Miou** :

| Option | Défaut | Description |
|--------|--------|-------------|
| **Mode avancé (LLM)** | Désactivé | Active le LLM. Nécessite consentement (ou activation manuelle) + specs suffisantes + modèle téléchargé. Voir [Proto-IA, Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md). |
| **Modèle** | SmolLM2-1.7B | Choix : « Léger (360M) » / « Standard (1.7B) » / « Avancé (3.8B, machine puissante) ». |
| **Chemin du modèle** | `{cog_data}/miou/models/` | Dossier contenant le fichier GGUF. |

---

## 5. Conformité architecturale

| Règle Miyukini | Conformité |
|----------------|------------|
| **LOI-1 (pas de dépendance externe critique)** | LLM local, pas d'API cloud. Si absent → templates. |
| **LOI-5 (souveraineté des données)** | Prompt construit localement, aucune donnée ne quitte le COG. |
| **MiyukiniWatch invariant** | Seuls des agrégats (durée, compteur, horodatage) entrent dans le prompt. Jamais de contenu. |
| **Transparence** | L'utilisateur peut voir dans MiyukiniWatch les données alimentant Miou. |

---

## 6. Références

- [Miou - Proto-IA, Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md) — Scan specs première connexion, proposition consentement, toggle manuel, transparence.
- [Bot — Documentation exhaustive](./Bot/_index.md) — Architecture, Banque de Templates, Moteur de Décision, Intégration.
- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Système de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md)
- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Proto-IA pour la fiabilité, LLM pour la vie. Miou parle toujours, avec ou sans modèle.*
