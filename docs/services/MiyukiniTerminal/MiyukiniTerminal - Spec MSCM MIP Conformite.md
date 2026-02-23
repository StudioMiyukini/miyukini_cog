# MiyukiniTerminal — Spécification Conformité MSCM et MIP

## Contexte

Ce document définit la **conformité MSCM (Miyukini Semantic Code Markup)** et **MIP (MSCM Index Protocol)** pour MiyukiniTerminal : balisage sémantique du code, structure de l'index, intégration à la Phase B de vérification MWS, et règles d'intégrité.

**Références :**

- [MIP v1 MSCM Index Protocol](../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)
- [MWS - Flux de Vérification](../../miyukini-webway-system/verification/MWS%20-%20Flux%20de%20Verification.md)
- [MSCM MIP Compliance Checklist](../../implementation/Miyukini%20COG%200.1%20-%20MSCM%20MIP%20Compliance%20Checklist.md)
- [Skill miyukini-mscm-mip](.cursor/skills/miyukini-mscm-mip/SKILL.md)

---

## Portée / Scope

- Balisage MSCM obligatoire pour le code Terminal
- Structure MIP et fichiers d'index
- Spécificité Phase B pour COG TERMINAL (services consultatifs vs code local)
- Liste des blocs à baliser avec IDs et rôles
- Règles d'intégrité et pipeline de génération

---

## 1. Principe fondamental

> La sémantique est dans le code (MSCM).  
> La structure est dans l'index (MIP).  
> La gouvernance est dans le graphe.

**Règle :** Tout code MiyukiniTerminal DOIT être balisé MSCM. L'index MIP est généré automatiquement et sert à la vérification de conformité MWS (Phase B).

---

## 2. Spécificité TERMINAL : Phase B adaptée

### 2.1 Différence COG STABLE vs TERMINAL

| Aspect | COG STABLE | COG TERMINAL |
|--------|------------|--------------|
| Services | Exécutés localement (JayKonta, JayKoa) | Consommés à distance (parent) |
| Phase B | Relay demande blocs des Services locaux | Relay demande blocs du **code Terminal** |
| Blocs MIP | Par Service (jaykonta.*, jaykoa.*) | Par module Terminal (liaison.*, sync.*, mws.*) |

### 2.2 Logique Phase B pour Terminal

Le TERMINAL n'héberge pas JayKonta ni JayKoa. Son `service_list` (svc_manifest) liste les services **consommés**, pas exécutés. La Phase B vérifie donc :

1. **Option A (recommandée) :** Le Relay demande des blocs de code du **client Terminal** lui-même — modules `terminal.liaison`, `terminal.sync`, `terminal.mws`, `terminal.storage` — pour attester que l'app n'a pas été modifiée.
2. **Option B (délégation) :** Phase B allégée ou déléguée au parent ; le parent atteste que le Terminal est légitime.

**Décision documentée :** Option A — le Terminal possède ses propres blocs MIP ; le Relay vérifie l'intégrité du code client Terminal comme preuve d'authenticité.

### 2.3 Mapping service_list → blocs MIP

Pour un TERMINAL, le `svc_manifest` peut inclure des **identifiants de modules MIP** au lieu des services distants :

```json
{
  "services": [
    {"id": "terminal.liaison.v1", "mode": "local"},
    {"id": "terminal.mws.v1", "mode": "local"},
    {"id": "terminal.sync.v1", "mode": "local"}
  ]
}
```

Ou convention : `service_id` = préfixe domaine MIP (ex. `terminal_*`). Le Relay interprète ces entrées comme des **modules de code local** à vérifier.

---

## 3. Champs MSCM obligatoires

### 3.1 Par type de bloc

| Champ | Obligatoire | Description |
|-------|-------------|-------------|
| `@id` | Oui | Identifiant unique global (format `terminal.{module}.v1.{kind}.{name}`) |
| `@do` | Oui | Description fonctionnelle (verbe + objet) |
| `@role` | Oui | Rôle sémantique (security, data, logic, api, infra, ui) |
| `@layer` | Oui | Couche (domain, infra, api, service, ui) |
| `@human` | Oui | Description humaine lisible |
| `@domain` | Recommandé | Domaine métier (terminal, liaison, mws, sync, storage) |

### 3.2 Conventions de nommage @id

```
terminal.{module}.v1.{kind}.{item}
```

| Composant | Exemples |
|-----------|----------|
| module | liaison, mws, sync, storage, ui |
| kind | fn, struct, component |
| item | validate_token, relay_connect, identity_table |

**Exemples :**
- `terminal.liaison.v1.fn.validate_token`
- `terminal.mws.v1.fn.relay_register`
- `terminal.storage.v1.struct.identity`

---

## 4. Liste des blocs à baliser (inventaire)

### 4.1 Module liaison

| Bloc | @id | @role | @layer | @do |
|------|-----|-------|--------|-----|
| Validation token | terminal.liaison.v1.fn.validate_token | security | domain | Valide et décode le token de liaison |
| Stockage identité | terminal.liaison.v1.fn.store_identity | data | domain | Persiste cog_id et parent_cog_id de façon sécurisée |
| Décodage QR | terminal.liaison.v1.fn.decode_qr | logic | domain | Extrait le token depuis les données QR |
| Vérification signature | terminal.liaison.v1.fn.verify_signature | security | domain | Vérifie la signature Ed25519 du token |

### 4.2 Module MWS

| Bloc | @id | @role | @layer | @do |
|------|-----|-------|--------|-----|
| Connexion Relay | terminal.mws.v1.fn.relay_connect | infra | infra | Établit la connexion TCP/TLS au Relay |
| Construction Passeport | terminal.mws.v1.fn.build_passport | logic | domain | Construit le payload REGISTER avec parent_cog_id |
| Envoi REGISTER | terminal.mws.v1.fn.send_register | api | infra | Envoie le message REGISTER au Relay |
| Parse REGISTER_OK | terminal.mws.v1.fn.parse_register_ok | logic | domain | Extrait session_id et permis_id de la réponse |
| Heartbeat | terminal.mws.v1.fn.send_heartbeat | api | infra | Envoie HEARTBEAT pour maintenir la session |

### 4.3 Module sync

| Bloc | @id | @role | @layer | @do |
|------|-----|-------|--------|-----|
| Sync initiale | terminal.sync.v1.fn.sync_initial | api | service | Demande et stocke les données initiales du parent |
| Sync incrémentale | terminal.sync.v1.fn.sync_delta | api | service | Récupère les delta depuis last_sync |
| Mise à jour cache | terminal.sync.v1.fn.update_cache | data | domain | Met à jour le cache local depuis la réponse sync |
| Détection conflit | terminal.sync.v1.fn.detect_conflict | logic | domain | Compare versions et détecte les conflits |

### 4.4 Module storage

| Bloc | @id | @role | @layer | @do |
|------|-----|-------|--------|-----|
| Lecture identité | terminal.storage.v1.fn.get_identity | data | infra | Lit cog_id et parent_cog_id depuis le stockage |
| Écriture queue | terminal.storage.v1.fn.queue_push | data | domain | Ajoute une action à la queue des actions différées |
| Rejeu queue | terminal.storage.v1.fn.queue_replay | logic | domain | Rejoue les actions pending à la reconnexion |
| Lecture cache service | terminal.storage.v1.fn.get_cache | data | infra | Lit les données en cache pour un service donné |

### 4.5 Module UI (Dioxus)

| Bloc | @id | @role | @layer | @do |
|------|-----|-------|--------|-----|
| Écran Liaison | terminal.ui.v1.component.liaison_screen | ui | ui | Affiche l'écran de liaison (QR, saisie) |
| Écran Salon | terminal.ui.v1.component.salon_screen | ui | ui | Affiche la liste des services |
| Indicateur connexion | terminal.ui.v1.component.connection_status | ui | ui | Affiche l'état online/offline/degrading |

---

## 5. Structure MIP (fichiers d'index)

### 5.1 Emplacement

Pour un projet `apps/terminal/` autonome ou intégré :

```
apps/terminal/
├── src/
├── mscm_index/          # Ou à la racine du workspace (mscm_index/)
│   ├── blocks.json
│   ├── domains.json
│   ├── layers.json
│   └── files.json
```

Si Terminal est dans le workspace Miyukini-COG, ses blocs sont **fusionnés** dans l'index global `mscm_index/` à la racine.

### 5.2 Format blocks.json (extrait Terminal)

```json
{
  "id": "terminal.liaison.v1.fn.validate_token",
  "file": "apps/terminal/src/services/liaison.rs",
  "start_line": 42,
  "end_line": 89,
  "role": "security",
  "layer": "domain",
  "domain": "terminal",
  "do": "validate_and_decode_link_token",
  "human": "Valide la signature et l'expiration du token de liaison, extrait cog_id et parent_cog_id"
}
```

### 5.3 Domaines (domains.json)

| Domaine | Blocs associés |
|---------|----------------|
| terminal | Tous les blocs terminal.* |
| liaison | terminal.liaison.* |
| mws | terminal.mws.* |
| sync | terminal.sync.* |
| storage | terminal.storage.* |

### 5.4 Couches (layers.json)

| Layer | Blocs |
|-------|-------|
| domain | Logique métier (validation, détection conflits) |
| infra | Transport, stockage |
| api | Appels réseau, sync |
| service | Orchestration |
| ui | Composants Dioxus |

---

## 6. Intégration Phase B (Relay)

### 6.1 Séquence vérification Terminal

```mermaid
sequenceDiagram
    participant R as Relay
    participant T as Terminal

    R->>R: Cog type = TERMINAL ; service_list = modules locaux
    R->>R: Sélectionner bloc aléatoire (ex. terminal.liaison.v1.fn.validate_token)
    R->>T: Demande bloc SERVICE_BLOCK (service_id=terminal.liaison, block_index=X)
    T->>T: Extraire bloc depuis code source (lignes start_line..end_line)
    T->>T: Chiffrer bloc avec clé de vérification
    T->>R: Paquet chiffré
    R->>R: Déchiffrer et comparer avec référence Origin
    alt Bloc correct
        R->>R: Module terminal.liaison OK
    else Bloc incorrect
        R->>R: Terminal suspect → Quarantaine
    end
```

### 6.2 Référence Origin

Origin (ou le Relay) doit posséder les **blocs de référence** de l'app MiyukiniTerminal pour chaque version publiée. Ces références sont construites à partir de l'index MIP du build release.

### 6.3 Clé de conformité (Phase A pour Terminal)

Le Terminal n'a pas de Cores complets. La Phase A peut être adaptée :

| Option | Description |
|--------|-------------|
| **Clé dérivée** | Une clé de conformité est dérivée du hash des blocs MIP du Terminal (attestation de build) |
| **Délégation parent** | La clé du parent est utilisée ; le Terminal prouve la liaison valide |
| **Clé dédiée** | Une clé spécifique "terminal" est intégrée à la compilation |

**Recommandation :** Clé dérivée du hash des blocs critiques (liaison, mws) — garantit que le client n'a pas été modifié.

---

## 7. Règles d'intégrité

### 7.1 Obligations

| Règle | Description |
|-------|-------------|
| **ID unique** | Aucun @id en doublon dans le codebase |
| **Pas d'orphelin** | Tout bloc avec @id doit être référençable dans hierarchy ou graph |
| **Couverture** | Toutes les fonctions publiques et structs principaux sont balisés |
| **Cohérence layer** | Un bloc ne peut être que dans une seule couche |

### 7.2 Pipeline de génération

```
1. Scan apps/terminal/src/**/*.rs
2. Parse commentaires MSCM (//! @id, @do, ...)
3. Extraction des blocs (file, start_line, end_line)
4. Validation (IDs uniques, champs obligatoires)
5. Construction domains.json, layers.json
6. Fusion dans mscm_index/ global (si workspace partagé)
7. Export références pour Origin (build release)
```

### 7.3 Outil

Utiliser `tools/mip-generator/` ou équivalent ; étendre le scan pour inclure `apps/terminal/`.

---

## 8. Exemple de balisage (extrait code)

```rust
//! @id terminal.liaison.v1.fn.validate_token
//! @role security
//! @layer domain
//! @domain terminal
//! @do validate_and_decode_link_token
//! @human Valide la signature et l'expiration du token de liaison, extrait cog_id et parent_cog_id

pub fn validate_token(token_b64: &str) -> Result<TokenPayload, ValidationError> {
    let decoded = base64::decode(token_b64)?;
    let signature_valid = verify_ed25519(&decoded, &PUBLIC_KEY)?;
    if !signature_valid {
        return Err(ValidationError::InvalidSignature);
    }
    let payload = extract_payload(&decoded)?;
    if payload.exp < now_epoch() {
        return Err(ValidationError::Expired);
    }
    Ok(payload)
}
```

---

## 9. Références

- [MIP v1 Protocol](../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)
- [MWS - Flux de Vérification](../../miyukini-webway-system/verification/MWS%20-%20Flux%20de%20Verification.md)
- [MSCM MIP Compliance Checklist](../../implementation/Miyukini%20COG%200.1%20-%20MSCM%20MIP%20Compliance%20Checklist.md)
- [Spec MWS Passeport Permis](./MiyukiniTerminal%20-%20Spec%20MWS%20Passeport%20Permis.md)
