# MiyukiniTerminal — Spécification Central Gestion Terminaux

## Contexte

Ce document décrit l'écran **"Gérer mes Terminaux"** dans Miyukini Central : liste des terminaux liés, bouton "Ajouter Terminal", génération token/QR, révocation, limite 5, wireframes UX et intégration BondingBrother / stockage.

**Références :**

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Token Liaison Securite](./MiyukiniTerminal%20-%20Spec%20Token%20Liaison%20Securite.md)
- Code Central : `apps/central/`

---

## Portée / Scope

- Écran "Gérer mes Terminaux"
- Liste terminaux, ajout, révocation
- Génération token/QR
- Limite 5 terminaux
- Stockage côté STABLE
- Wireframes (texte)

---

## 1. Accès à l'écran

| Chemin | Description |
|--------|-------------|
| Miyukini (onglet) > Paramètres > **Terminaux** | Ou lien direct depuis Paramètres |
| Raccourci | Carte "Terminaux" dans Paramètres avec compteur (ex. "2/5 liés") |

---

## 2. Structure de l'écran

### 2.1 Layout

```
+------------------------------------------+
|  Gérer mes Terminaux                     |
|  [Retour]                                |
+------------------------------------------+
|                                          |
|  Terminaux liés (2/5)                    |
|  +------------------------------------+  |
|  | Terminal 1                         |  |
|  | Android • Lié le 15/01/2026        |  |
|  |                    [Révoquer]       |  |
|  +------------------------------------+  |
|  +------------------------------------+  |
|  | Terminal 2                         |  |
|  | Android • Lié le 10/02/2026       |  |
|  |                    [Révoquer]       |  |
|  +------------------------------------+  |
|                                          |
|  [ + Ajouter un Terminal ]               |
|  (désactivé si 5/5)                      |
|                                          |
+------------------------------------------+
```

### 2.2 Wireframe "Ajouter Terminal" (modal ou nouvel écran)

```
+------------------------------------------+
|  Ajouter un Terminal                    |
|  [X Fermer]                              |
+------------------------------------------+
|                                          |
|  Scannez ce QR avec l'app Terminal      |
|  ou partagez le lien ci-dessous.        |
|                                          |
|  +------------------------------------+  |
|  |                                    |  |
|  |        [QR CODE 200x200]           |  |
|  |                                    |  |
|  +------------------------------------+  |
|                                          |
|  Lien : [https://...] [Copier]          |
|                                          |
|  Code manuel : XXXX-XXXX  [Copier]      |
|                                          |
|  ⚠ Valide 15 minutes                    |
|                                          |
|  [Générer un nouveau lien]              |
|                                          |
+------------------------------------------+
```

---

## 3. Fonctionnalités

### 3.1 Liste des terminaux

| Champ affiché | Source |
|---------------|--------|
| Nom/Identifiant | cog_id (tronqué) ou label utilisateur |
| Type | "Android" (os_type) |
| Date liaison | created_at |
| Statut | Lié / Dernière connexion |
| Action | [Révoquer] |

### 3.2 Ajouter un Terminal

| Action | Comportement |
|--------|--------------|
| Clic "Ajouter" | Vérifier limite 5 ; si OK, générer token et afficher QR/lien |
| QR | Encodage des données token (voir Spec Flux) |
| Lien | URL deep link ou web avec token |
| Code manuel | 8 caractères ; mapping vers token en DB |
| Générer nouveau | Invalider ancien token ; créer nouveau |

### 3.3 Révoquer un Terminal

| Action | Comportement |
|--------|--------------|
| Clic "Révoquer" | Confirmation modal "Êtes-vous sûr ?" |
| Après confirmation | Supprimer/marquer révoqué en DB ; notifier Origin (optionnel, blacklist locale) |
| Côté Terminal | Au prochain heartbeat ou REGISTER : Permis révoqué ou CLOSE |

---

## 4. Limite 5 terminaux

| Règle | Description |
|-------|-------------|
| Comptage | Terminaux avec status=linked |
| Affichage | "X/5" dans en-tête |
| Bouton Ajouter | Désactivé si 5/5 |
| Message | "Vous avez atteint la limite de 5 terminaux. Révoquez-en un pour en ajouter." |

---

## 5. Intégration BondingBrother / Cores

| Composant | Rôle |
|-----------|------|
| **StrongFather** | Autorisation "ajouter terminal" (capacité MasterButler) |
| **KindMother** | Persistance : table `terminals` (cog_id, parent_cog_id, user_id, status, created_at) |
| **MasterButler** | Capability "terminal.manage" |
| **BondingBrother** | Pas de médiation spécifique ; accès direct aux données via service |

---

## 6. Schéma de données (côté STABLE)

### 6.1 Table terminals

| Colonne | Type | Description |
|---------|------|-------------|
| id | INTEGER PK | Auto |
| cog_id | TEXT | Identifiant Terminal (UUID) |
| parent_cog_id | TEXT | = cog_id du STABLE |
| user_id | TEXT | Utilisateur propriétaire |
| status | TEXT | pending, linked, revoked |
| token_hash | TEXT | Hash du token (pour invalidation) |
| created_at | INTEGER | Epoch |
| linked_at | INTEGER | Epoch (quand REGISTER_OK reçu ou sync) |
| label | TEXT | Optionnel (nom donné par l'utilisateur) |

### 6.2 Table terminal_tokens (optionnel)

Pour le code manuel : mapping code -> token (temporaire, TTL 15 min).

| Colonne | Type |
|---------|------|
| code | TEXT PK |
| token | TEXT |
| expires_at | INTEGER |
| used | BOOLEAN |

---

## 7. API (optionnel)

Si le Terminal récupère le token via API (code manuel) :

```
GET /api/terminal/link/{code}
Response: { "token": "...", "expires_at": ... }
```

Après utilisation : marquer `used=true` ; refuser les appels suivants.

---

## 8. Références

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Token Liaison Securite](./MiyukiniTerminal%20-%20Spec%20Token%20Liaison%20Securite.md)
- Central : `apps/central/src/`
