# MWS â€” Chiffrement et TLS

## Contexte

Le **chiffrement** est un pilier fondamental de la sÃ©curitÃ© du MWS. Toutes les communications entre les acteurs (COGs, relays, trackers, Origin) sont protÃ©gÃ©es par **TLS** par dÃ©faut. Ce document dÃ©taille la politique de chiffrement, les exemptions possibles, et les exigences de sÃ©curitÃ©.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- TLS obligatoire : versions, cipher suites, certificats
- Canal de contrÃ´le vs canal de donnÃ©es
- Exemption temps rÃ©el : conditions et rÃ¨gles
- Authentification et replay protection
- Gestion des secrets et certificats

---

## 1. Principe fondamental

> **Le chiffrement n'est pas nÃ©gociable sur le canal de contrÃ´le. Sur le canal de donnÃ©es, il est obligatoire par dÃ©faut avec une exemption strictement encadrÃ©e pour les cas temps rÃ©el.**

| Canal | Chiffrement | Exemption possible |
|-------|-------------|-------------------|
| **ContrÃ´le** | TLS obligatoire | **Jamais** |
| **DonnÃ©es** | TLS par dÃ©faut | Oui, sous conditions strictes |

---

## 2. TLS obligatoire

### 2.1 Versions supportÃ©es

| Version | Statut |
|---------|--------|
| TLS 1.3 | **RecommandÃ©** |
| TLS 1.2 | AcceptÃ© (minimum) |
| TLS 1.1 et infÃ©rieures | **RefusÃ©** |

### 2.2 Cipher suites

Les cipher suites acceptÃ©es doivent garantir :

| Exigence | Description |
|----------|-------------|
| **Perfect Forward Secrecy (PFS)** | Obligatoire (ECDHE, DHE) |
| **Algorithmes sÃ»rs** | AES-GCM, ChaCha20-Poly1305 |
| **Taille de clÃ©** | Minimum 128 bits (256 recommandÃ©) |

**Cipher suites recommandÃ©es (TLS 1.3) :**
- `TLS_AES_256_GCM_SHA384`
- `TLS_CHACHA20_POLY1305_SHA256`
- `TLS_AES_128_GCM_SHA256`

**Cipher suites refusÃ©es :**
- RC4, 3DES, DES
- TLS_RSA_* (pas de PFS)
- MD5, SHA1 (pour les signatures)

### 2.3 Certificats

| Aspect | Exigence |
|--------|----------|
| **Certificat serveur** | SignÃ© par une CA reconnue (Let's Encrypt recommandÃ©) |
| **Validation cÃ´tÃ© client** | Obligatoire (chaÃ®ne de confiance, nom de domaine) |
| **Auto-signÃ©** | Uniquement en test, avec certificate pinning |
| **DurÃ©e de validitÃ©** | Maximum 1 an (90 jours recommandÃ© avec Let's Encrypt) |
| **Certificate pinning Origin** | Les clients se connectant Ã  **Origin** doivent implÃ©menter le **certificate pinning** (contremesure R-014) pour limiter les risques de DNS poisoning et MITM. |

### 2.4 Ports et endpoints

| Acteur | Port | Transport |
|--------|------|-----------|
| **Relay** | 7000 | TCP + TLS |
| **Tracker** | 21000 | TCP + TLS |
| **Catalogue web** | 80/443 | HTTP/HTTPS |
| **Origin** | Idem relay + tracker | TCP + TLS |

---

## 3. Canal de contrÃ´le

### 3.1 DÃ©finition

Le **canal de contrÃ´le** transporte les messages de gestion du MWS :

| Type de message | Description |
|-----------------|-------------|
| REGISTER | Enregistrement d'un tunnel |
| CONNECT | Demande de connexion |
| HEARTBEAT | Maintien de connexion |
| CLOSE | Fermeture de tunnel |
| ERROR | Erreurs protocolaires |
| CORE_KEY | ClÃ© de conformitÃ© des Cores |
| SERVICE_BLOCK | Bloc de code des Services |
| VERIFY_RESULT | RÃ©sultat de vÃ©rification |
| REDIRECT | Redirection vers un autre relay |
| REGISTRY_QUERY | Interrogation du Registre |
| UPDATE_AVAILABLE | Notification de mise Ã  jour |

### 3.2 Chiffrement obligatoire

| RÃ¨gle | Description |
|-------|-------------|
| **TLS toujours actif** | Aucune exception possible |
| **Pas de mode plaintext** | Aucun endpoint de contrÃ´le non chiffrÃ© |
| **Validation certificat** | Obligatoire des deux cÃ´tÃ©s |

---

## 4. Canal de donnÃ©es

### 4.1 DÃ©finition

Le **canal de donnÃ©es** transporte les donnÃ©es opaques Ã©changÃ©es entre COGs :

| Type de message | Description |
|-----------------|-------------|
| DATA | DonnÃ©es relayÃ©es (contenu opaque) |

### 4.2 Chiffrement par dÃ©faut

| RÃ¨gle | Description |
|-------|-------------|
| **TLS par dÃ©faut** | Les donnÃ©es sont chiffrÃ©es TLS |
| **Exemption possible** | Uniquement sous conditions strictes |

### 4.3 MAC sur paquets DATA (contremesure R-003)

MÃªme en **mode temps rÃ©el non chiffrÃ©**, l'intÃ©gritÃ© des paquets DATA doit Ãªtre garantie :

| Exigence | Description |
|----------|-------------|
| **MAC obligatoire** | Chaque trame DATA inclut un champ **MAC** de 32 octets (HMAC-SHA256) |
| **ClÃ© de session** | La clÃ© est dÃ©rivÃ©e lors de la nÃ©gociation TLS initiale : `session_key = HKDF(tls_master_secret, "MWS-DATA-MAC", session_id)` |
| **VÃ©rification** | Le rÃ©cepteur vÃ©rifie le MAC avant de traiter ; rejet si invalide |
| **Format** | Voir [MWS - Protocole Relay](../protocole/MWS%20-%20Protocole%20Relay.md) â€” format DATA avec champ `mac` |

---

## 5. Exemption temps rÃ©el

### 5.1 Cas d'usage

L'exemption temps rÃ©el est prÃ©vue pour les scÃ©narios nÃ©cessitant une **latence minimale** :

| Cas d'usage | Description |
|-------------|-------------|
| **Jeu multijoueur** | Ã‰changes rapides entre joueurs |
| **Streaming audio/vidÃ©o** | Diffusion en direct |
| **Interactions temps rÃ©el** | Latence critique |

### 5.2 Conditions strictes

L'exemption n'est possible que si **toutes** les conditions suivantes sont remplies :

| Condition | Description |
|-----------|-------------|
| **NÃ©gociation prÃ©alable** | Les deux COGs ont nÃ©gociÃ© l'exemption via le canal de contrÃ´le chiffrÃ© |
| **Permis valide** | Les deux COGs possÃ¨dent un Permis de circulation valide |
| **VÃ©rification prÃ©alable** | Les deux COGs ont Ã©tÃ© vÃ©rifiÃ©s par un relay |
| **Flux Ã©phÃ©mÃ¨re** | La session non chiffrÃ©e est limitÃ©e dans le temps |
| **DurÃ©e maximale** | **4 heures** â€” au-delÃ , renouvellement obligatoire (contremesure R-008) |
| **Notification utilisateur** | L'utilisateur est explicitement informÃ© du mode non chiffrÃ© |
| **Journalisation** | La session est journalisÃ©e (cog_ids, durÃ©e, volume) ; sessions > 1 h font l'objet d'une alerte si ratio non-chiffrÃ©/chiffrÃ© > 20 % |

### 5.3 NÃ©gociation

```mermaid
sequenceDiagram
    participant A as COG A
    participant B as COG B

    Note over A,B: Canal de contrÃ´le TLS
    A->>B: Demande exemption temps rÃ©el
    B->>B: VÃ©rifier conditions
    alt Conditions OK
        B->>A: Exemption accordÃ©e
        Note over A,B: Canal DATA sans TLS (Ã©phÃ©mÃ¨re)
    else Conditions non remplies
        B->>A: Exemption refusÃ©e
        Note over A,B: Canal DATA reste TLS
    end
```

### 5.4 Passeports spÃ©ciaux

Les COGs avec **Passeport spÃ©cial** peuvent nÃ©gocier l'exemption plus facilement :

| Aspect | Description |
|--------|-------------|
| **Risques assumÃ©s** | Le hÃ´te professionnel assume les risques |
| **ContrÃ´les allÃ©gÃ©s** | Moins de vÃ©rifications prÃ©alables |
| **Audit renforcÃ©** | Audits pÃ©riodiques plus stricts |

### 5.5 Journalisation obligatoire

Toute session en mode temps rÃ©el non chiffrÃ© est journalisÃ©e :

| Champ | Description |
|-------|-------------|
| `cog_id_source` | COG initiant l'exemption |
| `cog_id_destination` | COG acceptant l'exemption |
| `started_at` | DÃ©but de la session |
| `ended_at` | Fin de la session |
| `duration` | DurÃ©e totale |
| `volume_bytes` | Volume Ã©changÃ© |
| `reason` | Raison de l'exemption |

---

## 6. Authentification

### 6.1 Token d'authentification

| Exigence | Description |
|----------|-------------|
| **Entropie** | Minimum 256 bits d'entropie |
| **GÃ©nÃ©ration** | AlÃ©atoire cryptographiquement sÃ»r |
| **Transmission** | Une seule fois sur le canal TLS (dans REGISTER) |
| **Stockage** | Jamais en clair, droits restreints |

### 6.2 HMAC challenge-response (optionnel)

Au lieu d'envoyer le token brut, le relay peut utiliser un challenge-response :

```mermaid
sequenceDiagram
    participant COG as COG
    participant R as Relay

    COG->>R: Connexion TLS
    R->>COG: Challenge (nonce alÃ©atoire)
    COG->>COG: Calculer HMAC(secret, challenge)
    COG->>R: RÃ©ponse HMAC
    R->>R: VÃ©rifier HMAC
    alt HMAC valide
        R->>COG: Authentification rÃ©ussie
    else HMAC invalide
        R->>COG: Authentification Ã©chouÃ©e
        R->>R: Fermer connexion
    end
```

### 6.3 Ã‰chec d'authentification

| Action | Description |
|--------|-------------|
| **Fermeture immÃ©diate** | Connexion TLS fermÃ©e |
| **Journalisation** | Ã‰vÃ©nement journalisÃ© (sans exposer le token) |
| **Message gÃ©nÃ©rique** | Ne pas rÃ©vÃ©ler si c'est le token ou le cog_id qui est invalide |

---

## 7. Replay protection

### 7.1 MÃ©canismes

| MÃ©canisme | Description |
|-----------|-------------|
| **Nonce** | Chaque message critique inclut un nonce unique (min 16 octets) |
| **Timestamp** | Horodatage avec prÃ©cision seconde |
| **FenÃªtre d'acceptation** | **Â±10 secondes** (obligatoire) â€” contremesure R-006 |
| **Synchronisation NTP** | RecommandÃ©e pour tous les acteurs (drift max 5 s) |
| **Registre de nonces** | Cache bornÃ© des nonces vus rÃ©cemment |

### 7.2 VÃ©rification

```mermaid
flowchart TB
    A[Message reÃ§u] --> B{Timestamp dans fenÃªtre ?}
    B -->|Non| C[Rejeter : hors fenÃªtre]
    B -->|Oui| D{Nonce dÃ©jÃ  vu ?}
    D -->|Oui| E[Rejeter : replay dÃ©tectÃ©]
    D -->|Non| F[Enregistrer nonce]
    F --> G[Traiter message]
```

### 7.3 NumÃ©ro de sÃ©quence

Pour les messages de session active (HEARTBEAT, CLOSE) :

| Exigence | Description |
|----------|-------------|
| **Monotonie** | NumÃ©ro de sÃ©quence incrÃ©mentÃ© Ã  chaque message |
| **DÃ©tection** | DÃ©tecter les doublons et messages hors-ordre |
| **Rejet** | Rejeter les numÃ©ros de sÃ©quence invalides |

---

## 8. Gestion des secrets

### 8.1 Fichiers sensibles

| Exigence | Description |
|----------|-------------|
| **Droits restreints** | `chmod 600` sur les fichiers de secrets |
| **PropriÃ©taire** | Utilisateur du service uniquement |
| **Pas en clair** | Jamais de secrets en clair dans les logs |

### 8.2 Variables d'environnement

| Exigence | Description |
|----------|-------------|
| **Non visibles** | Pas visibles dans `/proc/*/environ` |
| **Pas dans les logs** | Pas dans les logs de dÃ©marrage |

### 8.3 Code source

| Exigence | Description |
|----------|-------------|
| **Pas de secrets** | Aucun token, clÃ© privÃ©e ou secret dans le code |
| **Git ignore** | Fichiers de secrets dans `.gitignore` |
| **Audit** | VÃ©rification pÃ©riodique de l'absence de secrets |

### 8.4 Rotation des tokens (contremesure R-007)

| Exigence | Description |
|----------|-------------|
| **RÃ©vocable** | Les tokens doivent pouvoir Ãªtre rÃ©voquÃ©s immÃ©diatement |
| **Renouvelable** | Renouvellement sans interruption de service |
| **Transition** | Plusieurs tokens valides simultanÃ©s pendant la transition |
| **Rotation automatique** | Tous les **7 jours** ; notification au COG 24 h avant expiration |
| **Alerte nouvelle IP** | Notifier le COG si son token est utilisÃ© depuis une nouvelle IP (optionnel) |

---

## 9. RÃ©sumÃ© des exigences

| Domaine | Exigence | Niveau |
|---------|----------|--------|
| **TLS** | TLS 1.2+ obligatoire | **Obligatoire** |
| **PFS** | Perfect Forward Secrecy | **Obligatoire** |
| **Cipher suites** | Suites sÃ»res uniquement | **Obligatoire** |
| **Certificats** | Validation cÃ´tÃ© client | **Obligatoire** |
| **Canal contrÃ´le** | TLS sans exception | **Obligatoire** |
| **Canal donnÃ©es** | TLS par dÃ©faut | **Obligatoire** |
| **Exemption temps rÃ©el** | Conditions strictes | **Optionnel** |
| **Token** | 256+ bits d'entropie | **Obligatoire** |
| **Replay protection** | Nonce + timestamp | **Obligatoire** |
| **Secrets** | Droits restreints, pas dans le code | **Obligatoire** |
| **Rotation** | Tokens rÃ©vocables et renouvelables | **Obligatoire** |

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)
- [MWS - Contre-Mesures de SÃ©curitÃ©](./MWS%20-%20Contre-Mesures%20de%20Securite.md) â€” R-003, R-006, R-007, R-008, R-014
- [Miyukini Webway Relay](..//reference//_index.md) â€” sections 3.3, 10
- [Miyukini Webway Relay Protocol](..//reference//_index.md) â€” section 2

---

**Version :** 2.0  
**Mise Ã  jour :** IntÃ©gration contremesures R-003, R-006, R-007, R-008, R-014  
**Classification :** Documentation MWS â€” SÃ©curitÃ©

