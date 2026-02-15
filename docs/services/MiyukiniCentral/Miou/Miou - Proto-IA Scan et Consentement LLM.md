# Miou — Proto-IA, Scan des spécifications et Consentement LLM

La **Proto-IA** (ou **Bot**) est la **première couche d'intelligence** de Miou. Elle est toujours active. Le **LLM** est une couche complémentaire optionnelle : il ne s'active que si les spécifications de runtime le permettent et si l'utilisateur y consent. Ce document précise le scan des specs au premier lancement, le flux de consentement, le toggle manuel et la transparence non intrusive.

**Périmètre 0.1.x :** Aucune implémentation LLM. Le toggle « Mode LLM » est présent dans Paramètres > Miou > Intelligence — l'utilisateur peut l'activer/désactiver ; la valeur est persistée pour compatibilité future. Miou fonctionne à 100 % en Proto-IA.

---

## 1. Proto-IA (Bot) vs LLM : les deux couches

| Couche | Nature | Condition d'activation | Rôle |
|--------|--------|------------------------|------|
| **Proto-IA (Bot)** | Moteur à règles + templates. Logique déterministe, sélection par conditions, remplissage de variables. | **Toujours active** dès la première connexion. | Génère toutes les bulles de Miou. Accueil, rappels, suggestions, félicitations. Aucune ressource significative. |
| **LLM** | Petit modèle de langage local (ex. SmolLM2-1.7B). Inférence CPU, ~1.2 Go RAM. | **Optionnelle.** Nécessite : (1) specs suffisantes, (2) consentement utilisateur, (3) modèle téléchargé. | Complète la Proto-IA : varier le ton, enrichir certains messages, conversation future. |

**Règle fondamentale :** Miou fonctionne **entièrement** avec la Proto-IA seule. Le LLM est un **bonus** jamais obligatoire.

---

## 2. Scan des spécifications au premier lancement

### 2.1 Moment du scan

Le scan est effectué **à la première connexion** de l'utilisateur (juste après le Rite d'Entrée ou la première Connexion réussie). Miou « prend connaissance » de son environnement de runtime une seule fois par installation COG (résultat persisté).

### 2.2 Métriques collectées

| Métrique | Méthode (indicatif) | Seuil « suffisant » pour proposer le LLM |
|----------|---------------------|------------------------------------------|
| **RAM totale** | `total_memory_mb` (OS) | ≥ 6 Go (recommandé : ≥ 8 Go) |
| **RAM disponible** | `available_memory_mb` au démarrage | ≥ 2.5 Go libres pour le modèle |
| **CPU** | Cœurs logiques, fréquence base | ≥ 4 threads (ex. i3-1005G1 = 4 threads OK) |
| **Stockage** | Espace libre pour le modèle GGUF | ≥ 1.5 Go (SmolLM2-1.7B Q4) |
| **Architecture** | x86_64 / ARM64 | x86_64 pour les modèles GGUF courants |

Le scan est **silencieux** : aucune popup, aucun bandeau. Les résultats sont stockés localement (KindMother ou préférences Miou).

### 2.3 Décision

| Condition | Décision |
|-----------|----------|
| **Specs suffisantes** (RAM ≥ 6 Go, CPU ≥ 4 threads, stockage OK) | Miou peut proposer le LLM à l'utilisateur (voir section 3). |
| **Specs insuffisantes** | Aucune proposition. Miou reste en Proto-IA. Pas de message explicatif (éviter la frustration). L'utilisateur peut activer manuellement le LLM s'il le souhaite (à ses risques). |

---

## 3. Flux de consentement : Miou demande la permission

### 3.1 Déclenchement

Si le scan indique des **specs suffisantes** et que l'utilisateur **n'a jamais répondu** (ni accepté ni refusé) :

- Miou affiche une **bulle dédiée**, non intrusive, en bas à droite.
- Moment : 30 secondes à 1 minute après l'arrivée dans le Salon (laisser l'utilisateur s'installer).

### 3.2 Contenu de la bulle de proposition

**Exemple de texte :**

> « [pseudo], mon environnement me permet d’être un peu plus vivante. Si tu le souhaites, je peux utiliser une partie de la puissance de calcul de ton COG pour varier mes messages et mieux m’adapter à toi. C’est optionnel — je fonctionne très bien sans. Tu préfères que je reste légère ou que j’essaie ? »

**Actions proposées :**

| Bouton | Action |
|--------|--------|
| **« Oui, vas-y »** | Consentement enregistré. Téléchargement du modèle proposé (si pas encore présent) au prochain besoin. Miou remercie : « Merci. Je vais me préparer en douceur. » |
| **« Pas pour l’instant »** | Refus enregistré. Miou : « Très bien, je reste discrète. Tu peux changer d’avis dans Paramètres > Miou. » Aucune relance immédiate. |
| **« Plus d’infos »** | Ouvre un panneau **Conditions d’utilisation du LLM** (voir section 5) — transparence sur les données, la localité, les ressources. Puis retour à la bulle (l’utilisateur peut choisir Oui ou Non). |

### 3.3 Relance après refus

Si l'utilisateur a refusé (« Pas pour l'instant ») :

| Délai minimum avant nouvelle proposition | Condition |
|-------------------------------------------|-----------|
| **Long moment** | 30 jours (ou 10 sessions distinctes, selon la première atteinte). |
| **Contexte** | Miou peut reformuler légèrement : « Tu m'avais dit de rester légère. Avec le temps, si tu changes d'avis, tu peux activer le mode avancé dans Paramètres > Miou. » — ton informatif, pas de pression. |

Une seule relance automatique. Ensuite, l'utilisateur doit passer par le toggle manuel s'il change d'avis.

---

## 4. Toggle manuel : activer ou désactiver le LLM

L'utilisateur garde **toujours** le contrôle.

### 4.1 Emplacement

**Paramètres Miyukini > Miou > Intelligence**

| Option | Défaut | Description |
|--------|--------|-------------|
| **Mode avancé (LLM)** | Désactivé | Active ou désactive l'utilisation du LLM. Indépendant du consentement initial : si l'utilisateur active manuellement, le LLM se charge dès la prochaine bulle concernée. |

### 4.2 Comportement

| Action | Comportement |
|--------|--------------|
| **Activation manuelle** | Si le modèle n'est pas téléchargé : proposition de téléchargement (lien ou intégré). Si les specs étaient jugées insuffisantes au scan : avertissement discret « Ta machine peut être limitée — l'expérience peut être plus lente. » Pas de blocage. |
| **Désactivation** | Le LLM n'est plus invoqué. Modèle éventuellement déchargé en mémoire (au prochain redémarrage de Central). Miou revient à 100 % Proto-IA. |
| **Changement à tout moment** | Pas de confirmation obligatoire. Le paramètre prend effet immédiatement (ou au prochain chargement de bulle). |

---

## 5. Transparence : conditions d'utilisation et données

**Principe :** Grande transparence, **sans être intrusive**. L'information est **accessible** ; elle ne s'impose pas.

### 5.1 Accès à l'information

| Point d'accès | Contenu | Intrusion |
|---------------|---------|-----------|
| **Lien « Plus d'infos »** dans la bulle de proposition | Panneau Conditions d'utilisation du LLM | Sur action explicite de l'utilisateur |
| **Paramètres Miyukini > Miou > À propos du LLM** | Même contenu, consultable à tout moment | Sur navigation volontaire |
| **Première activation du LLM** | Récapitulatif court (1–2 phrases) : « Le LLM tourne localement. Aucune donnée ne quitte ton COG. » | Lien discret, pas de popup bloquante |

### 5.2 Contenu du panneau « Conditions d'utilisation du LLM »

**Structure proposée :**

1. **Titre :** « Comment Miou utilise le LLM »
2. **Bloc court (3–4 phrases) :**
   - Le LLM tourne **entièrement sur ton COG** — aucune donnée n'est envoyée à l'extérieur.
   - Les seules données utilisées sont des **agrégats** : durée de session, services utilisés, amis contactés (identifiants, pas les messages), heure. Voir MiyukiniWatch pour le détail.
   - Le LLM **ne lit jamais** le contenu de tes messages, de tes saisies ou de tes fichiers.
   - Tu peux **désactiver** à tout moment dans Paramètres > Miou > Intelligence.
3. **Lien :** « Voir MiyukiniWatch » (ouvre le service pour consulter et effacer les mesures).
4. **Bouton :** « Compris » — ferme le panneau.

### 5.3 Règles de présentation

| Règle | Description |
|-------|-------------|
| **Pas de popup obligatoire** | L'utilisateur n'est jamais bloqué par un écran « Accepter les conditions ». Le consentement est donné via la bulle « Oui, vas-y » ou le toggle. |
| **Texte court** | Le panneau tient sur un écran, pas de scroll infini. |
| **Pas de jargon** | Éviter « tokens », « inférence », « paramètres ». Utiliser « messages plus variés », « tourne sur ton ordinateur », « agrégats ». |
| **Ton Miou** | Même bienveillance : « Je te montre tout ça pour que tu saches exactement ce qui se passe. » |

---

## 6. Récapitulatif des états

| État | Specs | Consentement | Toggle | Comportement |
|------|-------|--------------|--------|--------------|
| **Proto-IA seule** | Quelconques | Refusé ou non demandé | Désactivé | Miou 100 % templates + règles. |
| **Proposition en attente** | Suffisantes | Jamais répondu | Désactivé | Miou propose le LLM (bulle). |
| **LLM activé** | Suffisantes | Accepté OU toggle manuel | Activé | Miou peut invoquer le LLM pour enrichir. |
| **LLM désactivé** | Quelconques | — | Désactivé | Retour à Proto-IA, même si consentement donné. |
| **Specs insuffisantes** | < seuils | — | — | Pas de proposition automatique. Toggle manuel possible (à risques). |

---

## 7. Données persistées

| Donnée | Stockage | Usage |
|--------|----------|-------|
| **Résultat du scan** | Préférences Miou (KindMother ou fichier local) | Éviter de rescanner à chaque lancement. Rescan possible si l'utilisateur le demande (Paramètres > Miou > « Revoir les spécifications »). |
| **Consentement LLM** | Préférences Miou | `pending` / `accepted` / `refused` |
| **Date du dernier refus** | Préférences Miou | Pour la relance après 30 jours |
| **Nombre de relances** | Préférences Miou | Maximum 1 relance automatique |

---

## 8. Références

- [Miou - Moteur de Génération Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md)
- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Proto-IA : toujours là. LLM : avec ton accord et si ta machine le permet. Transparence : accessible sans être intrusive.*
