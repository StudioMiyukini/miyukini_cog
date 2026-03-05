# MWS â€” Protocole Relay

## Contexte

Le **protocole relay** dÃ©finit le format des messages et les sÃ©quences d'Ã©change entre un COG et le relay. Il est **minimal**, orientÃ© **contrÃ´le de tunnel** et **routage par `cog_id`**. Ce document est un condensÃ© de la spÃ©cification complÃ¨te.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)

## PortÃ©e / Scope

- Version du protocole
- Format binaire des trames
- Types de messages
- SÃ©quences principales (enregistrement, vÃ©rification, donnÃ©es)
- Codes d'erreur

Pour la spÃ©cification complÃ¨te, voir [Miyukini Webway Relay Protocol](..//reference//_index.md).

---

## 1. Version du protocole

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Nom** | Miyukini Webway Relay Protocol |
| **Version actuelle** | **1** |
| **Identifiant** | `protocol_version = 1` (1 octet) |

Les Ã©volutions **compatibles** (champs optionnels) conservent la mÃªme version ; les changements **incompatibles** incrÃ©mentent la version majeure.

---

## 2. Format binaire des trames

### 2.1 Structure gÃ©nÃ©rale

```
+--------+--------+--------+------------------+
| Version|  Type  | Flags  | Payload length   |
+--------+--------+--------+------------------+
| Payload (variable)                           |
+----------------------------------------------+
```

| Champ | Taille | Description |
|-------|--------|-------------|
| **Version** | 1 octet | NumÃ©ro de version du protocole |
| **Type** | 1 octet | Type de message (voir section 3) |
| **Flags** | 1 octet | Bits optionnels (direction, fin de flux) |
| **Payload length** | 2 ou 4 octets | Longueur du payload (big-endian) |
| **Payload** | Variable | Contenu du message |

Les nombres multi-octets sont en **big-endian** (network byte order).

---

## 3. Types de messages

### 3.1 Messages principaux

| Code | Nom | Direction | Description |
|------|-----|-----------|-------------|
| 0x01 | **REGISTER** | COG â†’ Relay | Enregistrement du tunnel |
| 0x02 | **REGISTER_OK** | Relay â†’ COG | Enregistrement rÃ©ussi |
| 0x03 | **REGISTER_ERR** | Relay â†’ COG | Refus d'enregistrement |
| 0x04 | **CONNECT** | Client â†’ Relay | Demande de connexion vers un cog_id |
| 0x05 | **CONNECT_OK** | Relay â†’ Client | Tunnel Ã©tabli |
| 0x06 | **CONNECT_ERR** | Relay â†’ Client | Refus de connexion |
| 0x07 | **DATA** | Bidirectionnel | DonnÃ©es opaques |
| 0x08 | **HEARTBEAT** | Bidirectionnel | Garde la connexion vivante |
| 0x09 | **HEARTBEAT_ACK** | RÃ©ponse | AccusÃ© de HEARTBEAT |
| 0x0A | **CLOSE** | Bidirectionnel | Fermeture propre |
| 0x0B | **ERROR** | Bidirectionnel | Erreur protocolaire |

### 3.2 Messages du Registre

| Code | Nom | Direction | Description |
|------|-----|-----------|-------------|
| 0x0C | **REGISTRY_QUERY** | COG â†’ Relay | Interrogation du Registre de Services |
| 0x0D | **REGISTRY_RESPONSE** | Relay â†’ COG | RÃ©ponse du Registre |
| 0x0E | **UPDATE_AVAILABLE** | Relay â†’ COG | Notification de mise Ã  jour |

### 3.3 Messages de vÃ©rification

| Code | Nom | Direction | Description |
|------|-----|-----------|-------------|
| 0x10 | **CORE_KEY** | COG â†’ Relay | ClÃ© de conformitÃ© des Cores (Phase A) |
| 0x11 | **SERVICE_BLOCK** | COG â†’ Relay | Bloc de code MIP d'un Service (Phase B) |
| 0x12 | **VERIFY_RESULT** | Relay â†’ COG | RÃ©sultat d'une phase de vÃ©rification |
| 0x13 | **REDIRECT** | Relay/Origin â†’ COG | Redirection vers un autre relay |
| 0x30 | **PERMIT_REVOKE** | Relay/Origin â†’ Tracker | RÃ©vocation d'un Permis de circulation (contremesure R-009). Propagation en < 1 min ; le COG rÃ©voquÃ© reÃ§oit CLOSE avec raison `permit_revoked`. |

---

## 4. SÃ©quence d'enregistrement

### 4.1 Flow complet

```mermaid
sequenceDiagram
    participant COG as COG
    participant O as Origin/Relay

    COG->>O: TCP + TLS connect
    O->>COG: TLS handshake OK

    COG->>O: REGISTER (token, cog_id, Passeport)
    Note over O: Ã‰valuer capacitÃ©
    alt Origin saturÃ©
        O->>COG: REDIRECT (relay_host:port)
        COG->>O: (reconnexion au relay)
    end

    Note over O: Phase A : ClÃ© Cores
    O->>COG: Demande CORE_KEY
    COG->>O: CORE_KEY (clÃ©)
    O->>COG: VERIFY_RESULT (phase_a, OK/FAIL)

    Note over O: Phase B : Blocs Services
    loop Pour chaque Service
        O->>COG: Demande SERVICE_BLOCK (service_id, block_index)
        COG->>O: SERVICE_BLOCK (paquet chiffrÃ©)
        O->>COG: VERIFY_RESULT (phase_b, service_id, OK/FAIL)
    end

    Note over O: Phase C : SantÃ©
    O->>O: VÃ©rifier environment_health

    alt Tout conforme
        O->>COG: REGISTER_OK (permis_id, session_id)
    else Non-conforme
        O->>COG: REGISTER_ERR (code, raison)
    end
```

### 4.2 Format REGISTER

| Champ | Taille | Description |
|-------|--------|-------------|
| `token_len` | 2 octets | Longueur du token |
| `token` | Variable | Token d'authentification |
| `cog_id_len` | 2 octets | Longueur du cog_id |
| `cog_id` | Variable | Identifiant du COG |
| `cog_type` | 1 octet | Type de COG (voir ci-dessous) |
| `os_type` | 1 octet | Type d'OS (voir ci-dessous) |
| `core_version_len` | 1 octet | Longueur de core_version |
| `core_version` | Variable | Version des Cores |
| `svc_manifest_len` | 2 octets | Longueur du manifest |
| `svc_manifest` | Variable | Liste des Services (JSON) |
| `env_health_len` | 2 octets | Longueur du rapport de santÃ© |
| `environment_health` | Variable | Rapport de santÃ© |
| `permis_history_len` | 2 octets | Longueur de l'historique |
| `previous_permis` | Variable | Permis de circulation prÃ©cÃ©dents (JSON) |
| `passport_type` | 1 octet | 0 = STANDARD, 1 = SPECIAL |
| `special_key_len` | 2 octets | Longueur de la clÃ© spÃ©ciale |
| `special_key` | Variable | ClÃ© spÃ©ciale (si SPECIAL) |
| `parent_cog_id_len` | 2 octets | Longueur du parent_cog_id (0 si non applicable) |
| `parent_cog_id` | Variable | cog_id du parent (TERMINAL uniquement) |
| `nonce` | 16 octets | Protection anti-rejeu |
| `timestamp` | 8 octets | Horodatage (secondes depuis epoch) |

#### Valeurs de `cog_type`

| Code | Valeur | Description |
|------|--------|-------------|
| 0x00 | `ORIGIN` | Point central de vÃ©ritÃ© (un seul par rÃ©seau) |
| 0x01 | `RELAY` | COG de contrÃ´le d'intÃ©gritÃ© |
| 0x02 | `TRACKER` | Mapping et contrÃ´le |
| 0x03 | `STABLE` | COG d'utilisateur commun |
| 0x04 | `SPECIAL` | COG professionnel Ã  forte utilisation |
| 0x05 | `TERMINAL` | COG mobile enfant d'un STABLE |
| 0x06 | `LONE` | COG isolÃ© volontairement (ne devrait pas envoyer REGISTER) |

#### Valeurs de `os_type`

| Code | Valeur | Description |
|------|--------|-------------|
| 0x00 | `WINDOWS` | Microsoft Windows |
| 0x01 | `LINUX` | Distributions Linux |
| 0x02 | `MACOS` | Apple macOS |
| 0x03 | `ANDROID` | Google Android |
| 0x04 | `IOS` | Apple iOS |

### 4.3 Format REGISTER_OK

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `permis_id_len` | 2 octets | Longueur du permis_id |
| `permis_id` | Variable | Identifiant du Permis de circulation dÃ©livrÃ© (accord relay) |
| `permis_expires_at` | 8 octets | Date d'expiration du Permis |
| `permis_scope_len` | 2 octets | Longueur du scope |
| `permis_scope` | Variable | PortÃ©e du Permis (JSON) |
| `tracker_addresses_len` | 2 octets | Longueur de la liste des adresses de trackers officiels |
| `tracker_addresses` | Variable | Liste des adresses des trackers officiels/sÃ»rs (connus d'Origin) ; le COG ne doit se connecter qu'Ã  ces trackers. |
| `tracker_signature` | 64 octets | **Signature Ed25519** par Origin de la liste des trackers (contremesure R-004 â€” protection Eclipse). Le COG doit vÃ©rifier cette signature avant d'utiliser les trackers. |
| `status` | 1 octet | 0 = OK, 1 = UPDATE_RECOMMENDED |
| `min_core_version_len` | 1 octet | Longueur (optionnel) |
| `min_core_version` | Variable | Version minimale recommandÃ©e |

### 4.4 Codes d'erreur REGISTER_ERR

| Code | Nom | Description |
|------|-----|-------------|
| 1 | `invalid_token` | Token invalide |
| 2 | `cog_id_conflict` | cog_id dÃ©jÃ  enregistrÃ© |
| 3 | `unsupported_protocol_version` | Version de protocole non supportÃ©e |
| 4 | `auth_failed` | Ã‰chec d'authentification |
| 5 | `rate_limited` | Rate limiting |
| 6 | `internal_error` | Erreur interne |
| 7 | `incompatible_core_version` | Version des Cores incompatible |
| 8 | `unregistered_service` | Service non rÃ©pertoriÃ© |
| 9 | `core_key_mismatch` | ClÃ© de conformitÃ© Cores incorrecte |
| 10 | `service_block_mismatch` | Bloc de code Service incorrect |
| 11 | `environment_health_failed` | SantÃ© de l'environnement non conforme |
| 12 | `quarantine` | COG mis en quarantaine |
| 13 | `blacklisted` | COG blacklistÃ© |
| 14 | `redirect` | Redirection vers un autre relay |
| 15 | `special_key_invalid` | ClÃ© spÃ©ciale invalide |

---

## 5. Messages de donnÃ©es

### 5.1 Format DATA

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `sequence` | 4 octets | NumÃ©ro de sÃ©quence |
| `payload_len` | 4 octets | Longueur des donnÃ©es |
| `payload` | Variable | DonnÃ©es opaques |
| `mac` | 32 octets | **HMAC-SHA256** du message (session_key, header \|\| payload) â€” obligatoire mÃªme en mode temps rÃ©el non chiffrÃ© (contremesure R-003). Voir [MWS - Chiffrement et TLS](../securite/MWS%20-%20Chiffrement%20et%20TLS.md). |

### 5.2 Format HEARTBEAT

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `timestamp` | 8 octets | Horodatage |

### 5.3 Format CLOSE

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `reason` | 1 octet | Code de raison |
| `message_len` | 2 octets | Longueur du message |
| `message` | Variable | Message explicatif (optionnel) |

---

## 6. Messages du Registre

### 6.1 Format REGISTRY_QUERY

| Champ | Taille | Description |
|-------|--------|-------------|
| `query_type` | 1 octet | 1 = vÃ©rifier service, 2 = lister mises Ã  jour |
| `service_id_len` | 2 octets | Longueur du service_id |
| `service_id` | Variable | Identifiant du service (ou liste) |

### 6.2 Format REGISTRY_RESPONSE

| Champ | Taille | Description |
|-------|--------|-------------|
| `status` | 1 octet | 0 = FOUND, 1 = NOT_FOUND, 2 = SUSPENDED |
| `service_id_len` | 2 octets | Longueur |
| `service_id` | Variable | Service concernÃ© |
| `current_version_len` | 1 octet | Longueur |
| `current_version` | Variable | Version courante |
| `download_url_len` | 2 octets | Longueur |
| `download_url` | Variable | URL de tÃ©lÃ©chargement |
| `checksum` | 32 octets | SHA-256 |

### 6.3 Format UPDATE_AVAILABLE

| Champ | Taille | Description |
|-------|--------|-------------|
| `service_id_len` | 2 octets | Longueur |
| `service_id` | Variable | Service concernÃ© |
| `current_version_len` | 1 octet | Version installÃ©e |
| `current_version` | Variable | |
| `available_version_len` | 1 octet | Version disponible |
| `available_version` | Variable | |
| `severity` | 1 octet | 0 = optional, 1 = recommended, 2 = critical |
| `download_url_len` | 2 octets | |
| `download_url` | Variable | |
| `checksum` | 32 octets | SHA-256 |

---

## 7. Messages de vÃ©rification

### 7.1 Format CORE_KEY

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `key_len` | 2 octets | Longueur de la clÃ© |
| `key` | Variable | ClÃ© de conformitÃ© des Cores |

### 7.2 Format SERVICE_BLOCK

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `service_id_len` | 2 octets | Longueur |
| `service_id` | Variable | Service concernÃ© |
| `block_index` | 4 octets | Index du bloc MIP |
| `encrypted_block_len` | 4 octets | Longueur |
| `encrypted_block` | Variable | Bloc de code chiffrÃ© |

### 7.3 Format VERIFY_RESULT

| Champ | Taille | Description |
|-------|--------|-------------|
| `session_id` | 16 octets | Identifiant de session |
| `phase` | 1 octet | A = 1, B = 2, C = 3 |
| `result` | 1 octet | 0 = OK, 1 = FAIL, 2 = EXTENDED_REQUIRED |
| `service_id_len` | 2 octets | (Phase B uniquement) |
| `service_id` | Variable | (Phase B uniquement) |
| `message_len` | 2 octets | Message explicatif |
| `message` | Variable | |

### 7.4 Format REDIRECT

| Champ | Taille | Description |
|-------|--------|-------------|
| `relay_host_len` | 2 octets | Longueur |
| `relay_host` | Variable | Adresse du relay (ex. `relay.example.com`) |
| `relay_port` | 2 octets | Port (ex. 7000) |
| `reason` | 1 octet | 0 = saturated, 1 = maintenance, 2 = policy |

---

## 8. Limites et validation

### 8.1 Tailles maximales

| Champ | Maximum |
|-------|---------|
| `cog_id` | 256 octets |
| `token` | 512 octets |
| `svc_manifest` | 4096 octets |
| `payload` (DATA) | 64 Ko (configurable) |
| Trame de contrÃ´le | 64 Ko |

### 8.2 Validation

| RÃ¨gle | Description |
|-------|-------------|
| Longueur cohÃ©rente | Payload length doit correspondre aux champs |
| Encodage UTF-8 | Champs texte en UTF-8 valide |
| Version supportÃ©e | Rejet si version non supportÃ©e |
| Trames malformÃ©es | Fermeture + ERROR (`invalid_format`) |

### 8.3 Validation des payloads JSON (contremesure R-010)

Les payloads JSON (`svc_manifest`, `previous_permis`, `permis_scope`, etc.) doivent Ãªtre validÃ©s avant traitement :

| Exigence | Description |
|----------|-------------|
| **SchÃ©ma** | Chaque type de payload possÃ¨de un **JSON Schema** publiÃ© ; validation obligatoire cÃ´tÃ© relay/origin |
| **Profondeur max** | Imbrication limitÃ©e Ã  **5 niveaux** ; rejet si dÃ©passement |
| **Taille** | Respect des limites (ex. `svc_manifest` â‰¤ 4096 octets) |
| **Champs inconnus** | Politique dÃ©finie (rejet ou `additionalProperties: false`) |

Les schÃ©mas sont disponibles dans la documentation MWS et dans le dÃ©pÃ´t des spÃ©cifications.

---

## 9. RÃ©sumÃ© des flux

| Flux | Messages | Description |
|------|----------|-------------|
| **Enregistrement** | REGISTER â†’ (REDIRECT) â†’ CORE_KEY â†’ SERVICE_BLOCK â†’ REGISTER_OK/ERR | VÃ©rification et tunnel |
| **DonnÃ©es** | DATA (bidirectionnel) | Ã‰change de donnÃ©es |
| **Maintien** | HEARTBEAT â†” HEARTBEAT_ACK | Garder le tunnel actif |
| **Fermeture** | CLOSE | Fermeture propre |
| **Registre** | REGISTRY_QUERY â†’ REGISTRY_RESPONSE | Consultation |
| **Mise Ã  jour** | UPDATE_AVAILABLE (push) | Notification |

---

## RÃ©fÃ©rences

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Contre-Mesures de SÃ©curitÃ©](../securite/MWS%20-%20Contre-Mesures%20de%20Securite.md) â€” R-003, R-004, R-009, R-010
- [Miyukini Webway Relay Protocol](..//reference//_index.md) â€” SpÃ©cification complÃ¨te
- [MWS - Relays](../acteurs/MWS%20-%20Relays.md)

---

**Version :** 2.0  
**Mise Ã  jour :** MAC DATA (R-003), tracker_signature (R-004), PERMIT_REVOKE (R-009), validation JSON (R-010)  
**Classification :** Documentation MWS â€” Protocole

