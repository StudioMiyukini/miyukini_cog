# MGE — Intégration COG

Contrat CogService, lancement depuis Central, stop, communication IPC et isolation. Le jeu MGE comme Service Inter-COG.

## Contexte

Dans l'écosystème Miyukini COG, un jeu (ex. Allumina) est un **Service Inter-COG (Type 3)**. Il est lancé depuis Miyukini Central ou en standalone. Le MGE fournit un bridge (mge-cog-bridge) pour implémenter le contrat CogService et l'isolation.

## Portée / Scope

- **Applicable à :** Intégration des jeux MGE avec Central, déploiement, cycle de vie.
- **Audience :** Architectes, développeurs de jeux, développeurs Central.
- **Statut :** Spécification normative.

---

## 1. Trait CogService

### 1.1 Rôle

Le trait `CogService` définit le contrat minimal qu'un service exécutable doit respecter pour être géré par Central (launcher). Un jeu MGE expose ce contrat via `GameRuntime`.

### 1.2 Signature (conceptuelle)

```rust
/// Contrat Service COG pour exécutables lançables depuis Central
pub trait CogService {
    /// Identifiant du service (ex. "allumina")
    fn service_id(&self) -> &str;

    /// Démarrage du service
    fn start(&mut self, config: ServiceConfig) -> Result<(), ServiceError>;

    /// Arrêt gracieux
    fn stop(&mut self) -> Result<(), ServiceError>;

    /// Statut actuel (Running, Stopped, Error)
    fn status(&self) -> ServiceStatus;

    /// Configuration lue (sans modification)
    fn config(&self) -> &ServiceConfig;
}
```

### 1.3 GameRuntime impl CogService

- `GameRuntime` encapsule l'Engine MGE + les plugins + la logique jeu.
- `start()` : initialise l'Engine, charge les plugins, lance la boucle (ou démarre le thread principal).
- `stop()` : envoie le signal d'arrêt, attend la fin de la boucle, sauvegarde si nécessaire.
- `status()` : Running tant que la boucle tourne, Stopped une fois terminé.

---

## 2. Lancement

### 2.1 Depuis Central

- Central affiche le catalogue des jeux (installés ou non).
- Pour un jeu installé : bouton « Jouer ».
- Central exécute le binaire du jeu : `Command::new(path_to_exe).spawn()`.
- Le jeu est un **processus fils** ; Central ne partage pas la mémoire.

### 2.2 Emplacement

- Jeux installés dans un répertoire dédié (ex. `%LOCALAPPDATA%/Miyukini-COG/games/Allumina/`).
- Chaque jeu a son propre `allumina.exe` (ou équivalent).
- Central connaît le chemin via configuration ou registre d'installation.

### 2.3 Téléchargement

- Si le jeu n'est pas installé : bouton « Télécharger ».
- Central récupère l'archive/binaire depuis Origin MWS, dépôt GitHub ou CDN.
- Extraction dans le répertoire des jeux ; enregistrement du chemin.

---

## 3. Stop

### 3.1 Signal gracieux

- Central peut envoyer un signal d'arrêt au processus (ex. fermeture de la fenêtre, ou message IPC).
- Le jeu reçoit le signal et appelle `GameRuntime::stop()`.
- La boucle s'arrête ; sauvegarde via BondingBrother → KindMother si nécessaire.
- Le processus se termine proprement.

### 3.2 Timeout

- Si le jeu ne répond pas dans un délai (ex. 5 s), Central peut forcer la terminaison (kill).
- Comportement dépendant de l'OS.

---

## 4. Communication

### 4.1 IPC (optionnel)

- Canal de communication entre Central et le jeu (ex. socket local, pipe, ou fichier).
- Utilisation : statut (en cours de chargement, en jeu, en pause), métriques.
- Pas obligatoire pour un MVP ; le lancement par exe suffit.

### 4.2 Événements

- Le jeu peut émettre des événements vers Central (ex. « partie terminée », « erreur »).
- Central peut envoyer des commandes (ex. « arrêt demandé »).
- Format à définir (JSON, binaire, etc.).

---

## 5. Isolation

### 5.1 Processus séparé

- Le jeu tourne dans un processus distinct de Central.
- Pas de shared memory directe entre Central et le jeu.
- En cas de crash du jeu, Central reste actif.

### 5.2 Données

- Les sauvegardes du jeu passent par KindMother (Core Strate 4) via BondingBrother.
- Le jeu ne parle jamais directement à KindMother ; il fait une demande à BondingBrother qui traduit vers KindMother.
- Isolation des données : chaque COG a son état local souverain (LOI-3).

### 5.3 Réseau

- Pour le multijoueur, le jeu utilise le MWS (MiyuWebwayParticipant, etc.).
- Le MWS est consommé par le COG ; le jeu en bénéficie via le processus dans lequel il tourne.

---

## 6. Dépendances Cores

| Core | Usage par le jeu |
|------|------------------|
| **KindMother** | Sauvegardes, persistance des données joueur. Via BondingBrother. |
| **StrongFather** | Autorisation (si le jeu vérifie des permissions). Via BondingBrother. |
| **MWS (Origin, Relay, Tracker)** | Découverte Lobbys, accord d'hôte, transport. Via MiyuWebwayParticipant. |
| **WorrySentinel** | Politique de sécurité (niveau, restrictions). Via Cores. |
| **Border Guard** | Frontières Inter-COG, confiance. Via Cores. |

---

## 7. Flux de lancement

```
1. Utilisateur clique « Jouer Allumina » dans Central
2. Central vérifie : Allumina installé ? Chemin connu ?
3. Central : Command::new(".../allumina.exe").spawn()
4. Processus Allumina démarre
5. GameRuntime::start() → Engine::new() → add_plugins() → build() ; puis loop { input(); engine.tick(); render(); }
6. Boucle jeu tourne
7. Utilisateur ferme la fenêtre ou clique « Quitter »
8. Signal stop → GameRuntime::stop() → sauvegarde → Engine::stop()
9. Processus se termine
10. Central détecte la fin du processus (optionnel : mise à jour statut)
```

---

## 8. Références

| Document | Rôle |
|----------|------|
| [Miyukini - Moteur Jeux et Central Launcher](./reference/Miyukini%20-%20Moteur%20Jeux%20et%20Central%20Launcher.md) | Architecture jeux + Central, structure workspace. |
| [MGE - Architecture Générale](./MGE%20-%20Architecture%20Generale.md) | mge-cog-bridge, couches. |
| [MGE - Mode Multijoueur](./MGE%20-%20Mode%20Multijoueur.md) | Host authoritative, MWS. |
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Type de Service, Lobbys. |

---

**Document** : MGE — Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
