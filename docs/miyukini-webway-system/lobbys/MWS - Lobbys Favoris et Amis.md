# MWS â€” Lobbys, Favoris et Amis

## Contexte

Les **Lobbys**, **favoris** et **amis** sont les mÃ©canismes de dÃ©couverte et de connexion sociale du MWS. Ils permettent aux COGs d'exposer leurs services, de trouver d'autres COGs, et d'Ã©tablir des relations de confiance facilitant les connexions futures.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Lobbys : dÃ©finition, crÃ©ation, visibilitÃ©, accÃ¨s
- Lobbys privÃ©s : mot de passe, bans, dÃ©-ban
- Surfaces de connexion : dÃ©claration, limites
- Flow client â†’ hÃ´te : dÃ©couverte, connexion, accord d'hÃ´te
- Favoris : marquage, retrouver rapidement un hÃ´te
- Amis : relation de confiance, contrÃ´les allÃ©gÃ©s

---

## 1. Lobbys

### 1.1 DÃ©finition

Un **Lobby** est une entrÃ©e dans le **catalogue de Lobbys** tenu par les trackers, reprÃ©sentant les services qu'un COG hÃ´te expose pour des connexions entrantes. Ce catalogue **n'est pas affichÃ© sur le portail web des trackers** (qui ne montre que les services WEB publics) : les Lobbys sont **visibles et joignables depuis les services COG** concernÃ©s (ex. client jeu, client SaaS). C'est le point de rendez-vous entre les COGs qui offrent des services et ceux qui souhaitent les consommer.

### 1.2 CrÃ©ation d'un Lobby

Quand un COG se prÃ©sente Ã  un Tracker (aprÃ¨s validation du Permis de circulation), il peut dÃ©clarer ses **surfaces de connexion** :

```mermaid
sequenceDiagram
    participant COG as COG HÃ´te
    participant T as Tracker

    COG->>T: DÃ©claration de surfaces
    Note over COG,T: services, ports, visibilitÃ©
    T->>T: CrÃ©er Lobby dans le catalogue de Lobbys
    T->>COG: Lobby crÃ©Ã© (lobby_id)
    T->>T: Diffuser (visible depuis les services COG, pas depuis le portail web)
```

### 1.3 Structure d'un Lobby

| Champ | Type | Description |
|-------|------|-------------|
| `lobby_id` | string | Identifiant unique du Lobby |
| `host_cog_id` | string | COG hÃ©bergeur |
| `host_username` | string | Nom/pseudo de l'utilisateur hÃ´te |
| `services` | array | Services exposÃ©s (service_id, description) |
| `ports` | array | Ports concernÃ©s |
| `visibility` | enum | `public` ou `private` |
| `password_protected` | bool | Si privÃ©, protÃ©gÃ© par mot de passe |
| `core_version` | string | Version des Cores du hÃ´te |
| `created_at` | datetime | Date de crÃ©ation |
| `max_connections` | int | Nombre max de connexions (si limitÃ©) |
| `current_connections` | int | Nombre de connexions actives |
| `verified` | bool | **Lobby vÃ©rifiÃ©** (contremesure R-012) : badge visuel pour les Lobbys dont l'hÃ´te a Ã©tÃ© vÃ©rifiÃ© par Origin ou le tracker. Afficher clairement le `host_cog_id` avant connexion pour limiter le phishing. |

### 1.4 VisibilitÃ© des Lobbys

Les Lobbys sont visibles **depuis les services COG** qui les consomment (ex. liste de serveurs dans un jeu, liste de Lobbys dans un client SaaS), **pas** depuis le portail web des trackers (rÃ©servÃ© aux services WEB publics).

| VisibilitÃ© | Description |
|------------|-------------|
| **Public** | Visible dans le catalogue de Lobbys (affichÃ© par les services), accessible Ã  tous les COGs du mÃªme pool |
| **PrivÃ©** | Visible dans le catalogue mais nÃ©cessite un mot de passe pour rejoindre |
| **CachÃ©** | Non listÃ© dans le catalogue, accessible uniquement par ID direct (fonctionnalitÃ© avancÃ©e) |

---

## 2. Lobbys privÃ©s

### 2.1 Protection par mot de passe

Un COG hÃ´te peut protÃ©ger son Lobby avec un **mot de passe** :

| Aspect | Description |
|--------|-------------|
| **DÃ©finition** | Le hÃ´te dÃ©finit un mot de passe lors de la crÃ©ation du Lobby |
| **Transmission** | Le mot de passe est partagÃ© hors-bande (message, invitation) |
| **VÃ©rification** | Le Tracker vÃ©rifie le mot de passe avant d'autoriser la connexion |

### 2.2 Limite d'Ã©checs et ban

```mermaid
stateDiagram-v2
    [*] --> Tentative: COG client essaie de rejoindre
    Tentative --> Ã‰chec: Mot de passe incorrect
    Ã‰chec --> Compteur: IncrÃ©menter compteur
    Compteur --> Tentative: < 3 Ã©checs
    Compteur --> Ban: 3 Ã©checs atteints
    Ban --> NotificationHÃ´te: Notifier l'utilisateur hÃ´te
    Tentative --> SuccÃ¨s: Mot de passe correct
    SuccÃ¨s --> [*]
```

| RÃ¨gle | Valeur | Description |
|-------|--------|-------------|
| **Limite d'Ã©checs** | **3** | AprÃ¨s 3 mots de passe incorrects (contremesure R-011) |
| **DÃ©lai exponentiel** | 1 s, 2 s, 4 sâ€¦ | DÃ©lai croissant entre chaque essai pour limiter le brute force |
| **Action** | Ban | Le COG client est banni de ce Lobby |
| **Notification** | Utilisateur hÃ´te | L'hÃ´te est notifiÃ© du ban |
| **DÃ©-ban** | Manuel uniquement | Seul l'utilisateur hÃ´te peut dÃ©-bannir |
| **Mot de passe** | RecommandÃ© â‰¥ 12 caractÃ¨res | Pour rÃ©duire le risque de devinette |

### 2.3 DÃ©-ban manuel

| Acteur | Action |
|--------|--------|
| **Utilisateur hÃ´te** | Peut voir la liste des COGs bannis de ses Lobbys |
| **Utilisateur hÃ´te** | Peut dÃ©-bannir manuellement un COG |
| **Tracker** | ExÃ©cute le dÃ©-ban aprÃ¨s demande du hÃ´te |
| **SystÃ¨me** | Aucun dÃ©-ban automatique |

---

## 3. Surfaces de connexion

### 3.1 DÃ©claration des surfaces

Quand un COG se prÃ©sente au Tracker, il dÃ©clare ses **surfaces de connexion** :

| DÃ©claration | Description |
|-------------|-------------|
| **Services exposÃ©s** | Quels services sont accessibles (service_id) |
| **Ports** | Sur quels ports ces services Ã©coutent |
| **Acceptation** | Si le COG accepte des connexions entrantes |
| **Attentes** | Ce que le COG propose (ex. jeu, SaaS, portail) |
| **DÃ©sirs** | Ce que le COG cherche Ã  joindre (optionnel) |

### 3.2 Surface stricte

| Principe | Description |
|----------|-------------|
| **Surface explicite** | Seuls les services et ports dÃ©clarÃ©s acceptent des connexions |
| **Rejet hors surface** | Toute connexion sur un port/service non dÃ©clarÃ© est rejetÃ©e |
| **IntÃ©gritÃ© prioritaire** | Les Cores et la DB ne sont jamais exposÃ©s directement |

### 3.3 Limite de connexions

| Type de COG | Limite | Exclusions |
|-------------|--------|------------|
| **COG classique** | 100 connexions simultanÃ©es | Ports 80 et 8080 exclus |
| **COG spÃ©cial** | Configurable (supÃ©rieur) | Selon Passeport spÃ©cial |

> Les COGs ne sont pas des services type torrent. Cette limite garantit la qualitÃ© de suivi des organes de sÃ©curitÃ©.

### 3.4 Serveur web embarquÃ©

Un COG peut exposer des services web sur les ports 80 et/ou 8080 :

| Aspect | Description |
|--------|-------------|
| **Headless** | Le COG peut fonctionner sans interface utilisateur |
| **Permanence** | Services disponibles en continu |
| **Navigateur** | Accessible depuis un navigateur web |
| **Cas d'usage** | Sites web, SaaS, portails, applications web |

Ces ports **ne sont pas comptÃ©s** dans la limite des 100 connexions simultanÃ©es.

---

## 4. Flow client â†’ hÃ´te

### 4.1 DÃ©couverte

```mermaid
sequenceDiagram
    participant Client as COG Client
    participant T as Tracker
    participant Host as COG HÃ´te

    Client->>T: RequÃªte de dÃ©couverte
    T->>T: Filtrer par pool (core_version.MAJOR)
    T->>Client: Liste des Lobbys
    Client->>Client: Utilisateur parcourt la liste
    Client->>Client: SÃ©lection d'un Lobby
```

### 4.2 Connexion

```mermaid
sequenceDiagram
    participant Client as COG Client
    participant T as Tracker
    participant Host as COG HÃ´te

    Client->>T: Demande chemin vers Host
    T->>Client: Chemin (relay, tunnel, direct)
    Client->>Host: Connexion (Permis de circulation)
    Host->>Host: VÃ©rifier Permis client
    alt Permis valide
        Host->>Host: VÃ©rifier autorisation (Lobby, politique)
        alt AutorisÃ©
            Host->>Client: Accord d'hÃ´te
            Client->>Host: Consommation des services
        else Lobby privÃ©
            Host->>Client: Demande de mot de passe
            Client->>Host: Mot de passe
            alt Correct
                Host->>Client: Accord d'hÃ´te
            else Incorrect (< 5 fois)
                Host->>Client: Refus, rÃ©essayer
            else Incorrect (5 fois)
                Host->>T: Ban du client
                T->>Client: Banni de ce Lobby
            end
        end
    else Permis invalide
        Host->>Client: Refus
    end
```

### 4.3 Accord d'hÃ´te

L'**accord d'hÃ´te** est dÃ©livrÃ© par le COG hÃ´te au COG client :

| Champ | Description |
|-------|-------------|
| `accord_id` | Identifiant unique |
| `client_cog_id` | COG client autorisÃ© |
| `host_cog_id` | COG hÃ´te |
| `services_authorized` | Services accessibles |
| `lobby_id` | Lobby concernÃ© |
| `issued_at` | Date d'Ã©mission |
| `expires_at` | Date d'expiration |

L'accord d'hÃ´te est **distinct** du Permis de circulation (dÃ©livrÃ© par les relays, accord relay).

---

## 5. Favoris

### 5.1 DÃ©finition

Les **favoris** permettent Ã  un utilisateur de marquer des COGs hÃ´tes pour les retrouver rapidement.

### 5.2 Fonctionnement

| Aspect | Description |
|--------|-------------|
| **Marquage** | L'utilisateur ajoute un COG hÃ´te Ã  ses favoris |
| **Stockage** | Liste stockÃ©e localement dans le COG client |
| **Affichage** | Les favoris apparaissent en haut de la liste de dÃ©couverte |
| **Persistance** | Les favoris persistent entre les sessions |

### 5.3 Informations stockÃ©es

| Champ | Description |
|-------|-------------|
| `host_cog_id` | Identifiant du COG hÃ´te |
| `host_username` | Nom/pseudo de l'utilisateur hÃ´te |
| `services` | Services souvent utilisÃ©s |
| `added_at` | Date d'ajout aux favoris |
| `last_connected` | DerniÃ¨re connexion |
| `notes` | Notes personnelles (optionnel) |

### 5.4 Synchronisation avec le Tracker

| Option | Description |
|--------|-------------|
| **Local uniquement** | Les favoris restent dans le COG client |
| **SignalÃ© au Tracker** | Le COG client peut signaler ses favoris au Tracker pour optimiser les suggestions |

---

## 6. Amis entre COGs

### 6.1 DÃ©finition

La relation **amis** entre deux COGs permet des connexions **plus rapides**, avec des **protocoles de contrÃ´le allÃ©gÃ©s** et une **pÃ©riodicitÃ© de re-vÃ©rification plus longue**.

### 6.2 CaractÃ©ristiques

| CaractÃ©ristique | Description |
|-----------------|-------------|
| **Demande humaine** | Les demandes d'amis sont initiÃ©es par les utilisateurs |
| **Confirmation humaine** | L'acceptation est manuelle, pas automatique |
| **ContrÃ´les allÃ©gÃ©s** | Les contrÃ´les douaniers (Tracker) et d'accÃ¨s (hÃ´te) sont simplifiÃ©s |
| **PÃ©riodicitÃ© longue** | La re-prÃ©sentation et le renouvellement de preuves sont moins frÃ©quents |
| **Noms/pseudos** | Les COGs exposent les noms de leurs utilisateurs pour la reconnaissance |

### 6.3 Flow de demande d'amis

```mermaid
sequenceDiagram
    participant UserA as Utilisateur A
    participant COGA as COG A
    participant T as Tracker
    participant COGB as COG B
    participant UserB as Utilisateur B

    UserA->>COGA: Demande d'ami vers COG B
    COGA->>T: Transmettre demande
    T->>COGB: Notifier demande d'ami
    COGB->>UserB: Afficher demande
    UserB->>COGB: Accepter/Refuser
    alt AcceptÃ©
        COGB->>T: Confirmation
        T->>COGA: AmitiÃ© Ã©tablie
        T->>COGB: AmitiÃ© Ã©tablie
        Note over COGA,COGB: ContrÃ´les allÃ©gÃ©s dÃ©sormais
    else RefusÃ©
        COGB->>T: Refus
        T->>COGA: Demande refusÃ©e
    end
```

### 6.4 Structure de la relation

| Champ | Description |
|-------|-------------|
| `friend_pair_id` | Identifiant unique de la paire |
| `cog_a_id` | Premier COG |
| `cog_a_username` | Nom de l'utilisateur A |
| `cog_b_id` | Second COG |
| `cog_b_username` | Nom de l'utilisateur B |
| `established_at` | Date d'Ã©tablissement |
| `control_level` | Niveau de contrÃ´le (allÃ©gÃ©) |
| `recheck_period` | PÃ©riode de re-vÃ©rification (ex. 7 jours) |

### 6.5 Avantages des amis

| Avantage | Description |
|----------|-------------|
| **Connexion rapide** | Pas besoin de parcourir le catalogue |
| **ContrÃ´les allÃ©gÃ©s** | VÃ©rifications simplifiÃ©es au Tracker |
| **Confiance mutuelle** | Les deux COGs se font confiance explicitement |
| **PÃ©riodicitÃ© longue** | Re-vÃ©rification moins frÃ©quente (ex. tous les 7 jours au lieu de 24h) |

### 6.6 Limites et sÃ©curitÃ©

| Limite | Description |
|--------|-------------|
| **Pas de contournement** | Les amis restent soumis aux rÃ¨gles de surface et de sÃ©curitÃ© |
| **RÃ©vocable** | Chaque utilisateur peut mettre fin Ã  la relation |
| **Surveillance** | Les Trackers peuvent surveiller les abus |
| **Passeport requis** | Les deux COGs doivent avoir un Permis de circulation valide |

---

## 7. Catalogue web des Trackers (services WEB publics uniquement)

### 7.1 Service web de portail (port 80)

Le service web des trackers (port 80) prÃ©sente le **catalogue des services WEB publics** des COGs connectÃ©s au rÃ©seau, Ã  la maniÃ¨re dâ€™un **moteur de recherche** ; il gÃ¨re aussi les **adresses URL**. Les **Lobbys des autres services COG** (jeu, APIs, etc.) **ne sont pas visibles** depuis ce portail. Le **catalogue de Lobbys** de chaque type de service est visible **depuis ces mÃªmes services** (ex. depuis le client jeu, le client SaaS).

| Fonction | Description |
|----------|-------------|
| **Services WEB publics** | Liste des COGs ayant une surface web active et publique (sites, SaaS, portails) |
| **Recherche / URLs** | Moteur de recherche et gestion des adresses URL |
| **Redirection** | Les utilisateurs web sont redirigÃ©s vers les COGs hÃ´tes (type No-IP) |
| **Pas de Lobbys** | Les Lobbys ne sont pas affichÃ©s ici ; ils sont visibles depuis les services COG concernÃ©s |

### 7.2 Fonctionnement "No-IP"

| Aspect | Description |
|--------|-------------|
| **Pas de domaine requis** | Les COGs n'ont pas besoin de nom de domaine |
| **Pas d'IP fixe** | Les COGs n'ont pas besoin d'IP fixe |
| **Facilitateur** | Le Tracker agit comme facilitateur et tunnel |
| **Redirection** | Le Tracker redirige le trafic vers le COG hÃ´te |

> Le catalogue web du Tracker n'a **pas de fonction de contrÃ´le** sur les connexions web ; il redirige uniquement.

### 7.3 Mise Ã  jour automatique

| Aspect | Description |
|--------|-------------|
| **Temps rÃ©el** | Le catalogue des services WEB publics est mis Ã  jour en temps rÃ©el |
| **Global** | Le catalogue est global et accessible par n'importe quel Tracker |
| **Diffusion** | Les mises Ã  jour sont diffusÃ©es automatiquement |

---

## 8. Exemples

### 8.1 Exemple de Lobby public

```json
{
  "lobby_id": "lobby-game-001",
  "host_cog_id": "550e8400-e29b-41d4-a716-446655440000",
  "host_username": "GameServer42",
  "services": [
    {"service_id": "game.server", "description": "Serveur de jeu multijoueur"}
  ],
  "ports": [25565],
  "visibility": "public",
  "password_protected": false,
  "core_version": "1.0",
  "max_connections": 50,
  "current_connections": 12
}
```

### 8.2 Exemple de Lobby privÃ©

```json
{
  "lobby_id": "lobby-private-001",
  "host_cog_id": "660f9500-f30c-52e5-b827-557766551111",
  "host_username": "PrivateClub",
  "services": [
    {"service_id": "chat.server", "description": "Salon de discussion privÃ©"}
  ],
  "ports": [8888],
  "visibility": "private",
  "password_protected": true,
  "core_version": "1.0",
  "max_connections": 20,
  "current_connections": 5
}
```

### 8.3 Exemple de relation amis

```json
{
  "friend_pair_id": "friends-001",
  "cog_a_id": "550e8400-e29b-41d4-a716-446655440000",
  "cog_a_username": "Alice",
  "cog_b_id": "660f9500-f30c-52e5-b827-557766551111",
  "cog_b_username": "Bob",
  "established_at": "2026-02-13T10:00:00Z",
  "control_level": "relaxed",
  "recheck_period": "P7D"
}
```

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Trackers](../acteurs/MWS%20-%20Trackers.md)
- [MWS - Passeport et Visa](../verification/MWS%20-%20Passeport%20et%20Visa.md)
- [Miyukini Webway Relay](..//reference//_index.md) â€” sections 8, 9

---

**Version :** 1.0  
**Classification :** Documentation MWS â€” Lobbys et Connexions

