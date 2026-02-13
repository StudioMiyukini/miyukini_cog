# MWS — Audit de la Documentation et du Code Origin

**Date :** 13 février 2026  
**Version :** 1.0  
**Statut :** Corrigé

---

## Résumé

Cet audit vérifie la cohérence entre le code source de `miyukini-origin` et la documentation/scripts de déploiement.

---

## 1. Résultat de compilation

| Élément | Statut | Détail |
|---------|--------|--------|
| `cargo check -p miyukini-origin` | ✅ OK | Compile sans erreurs |
| Warnings | ⚠️ 85 | Code mort, imports inutilisés (normal en dev) |
| Dépendances | ✅ OK | Toutes résolues |

---

## 2. Problèmes identifiés et corrigés

### 2.1 Incompatibilité `setup-origin.sh` ↔ Code source

**Avant correction :**

| Attendu par le code (`config.rs`) | Généré par le script |
|-----------------------------------|----------------------|
| `[relay]` | `[network]` |
| `relay.port` | `network.relay_port` |
| `[tls].cert_path` | `[tls].cert_file` |
| `[tls].key_path` | `[tls].key_file` |
| `[tracker].web_port` | Absent |
| `[auth].token_file` | Absent |
| `[registry].data_dir` | `[registry].path` |
| `[cores].keys_dir` | Absent |
| `[policies].data_dir` | Absent |
| `[rate_limits]` | `[security]` (incompatible) |
| `[limits]` | Absent |

**Correction appliquée :** Le script `setup-origin.sh` a été réécrit pour générer une configuration conforme au schéma `OriginConfig` défini dans `apps/origin/src/config.rs`.

### 2.2 Répertoires manquants

**Avant :**
```bash
/var/lib/miyukini/data
/var/lib/miyukini/registry
/var/lib/miyukini/backup
```

**Après (ajoutés) :**
```bash
/var/lib/miyukini/keys      # [cores].keys_dir
/var/lib/miyukini/policies  # [policies].data_dir
/opt/scripts                # scripts de maintenance
```

### 2.3 Fichier tokens.json manquant

Le code (`config.rs` ligne 237) attend un fichier `token_file` mais le script ne le créait pas.

**Correction :** Ajout de la génération automatique de `/etc/miyukini/tokens.json` avec un token bootstrap.

### 2.4 Nom de fichier JWT incohérent

| Script original | Documentation | Code |
|-----------------|---------------|------|
| `/etc/miyukini/jwt.secret` | `/etc/miyukini/admin_jwt.key` | N/A (config) |

**Correction :** Alignement sur la documentation → `/etc/miyukini/admin_jwt.key`

---

## 3. Validation TLS

Le code valide l'existence des certificats TLS au démarrage (`config.rs` lignes 102-113) :

```rust
if !Path::new(&self.tls.cert_path).exists() {
    return Err(ConfigError::InvalidValue { ... });
}
```

**Impact :** Si les certificats n'existent pas, Origin refuse de démarrer.

**Comportement de secours :** Si TLS échoue à l'initialisation du Relay (`main.rs` ligne 171-188), le Relay est désactivé mais le Tracker et Admin continuent de fonctionner.

---

## 4. Structure de configuration validée

### Schéma `OriginConfig` (config.rs)

```
OriginConfig
├── identity: IdentityConfig
│   ├── role: String
│   ├── ip: String
│   └── domain: Option<String>
├── relay: RelayConfig
│   ├── host: String (default "0.0.0.0")
│   └── port: u16 (default 7000)
├── tracker: TrackerConfig
│   ├── host: String (default "0.0.0.0")
│   ├── port: u16 (default 21000)
│   ├── web_port: u16 (default 8080)
│   ├── pools: PoolsConfig
│   └── lobbys: LobbysConfig
├── tls: TlsConfig
│   ├── cert_path: String
│   ├── key_path: String
│   └── min_version: String (default "1.2")
├── auth: AuthConfig
│   ├── token_file: String
│   └── token_rotation_days: u32 (default 7)
├── registry: RegistryConfig
│   └── data_dir: String
├── cores: CoresConfig
│   └── keys_dir: String
├── policies: PoliciesConfig
│   ├── data_dir: String
│   ├── quarantine_escalation: Vec<u64>
│   └── timestamp_window_seconds: u64
├── rate_limits: RateLimitsConfig
│   ├── register_per_minute_per_ip: u32
│   ├── connections_per_token: u32
│   ├── requests_per_hour_per_cog: u32
│   └── tcp_connections_per_ip: u32
├── pow: PowConfig
│   ├── enabled: bool
│   ├── difficulty_normal: u8
│   ├── difficulty_attack: u8
│   └── challenge_ttl_seconds: u32
├── admin: AdminConfig
│   ├── host: String (default "127.0.0.1")
│   ├── port: u16 (default 8081)
│   └── config_file: Option<String>
├── logging: LoggingConfig
│   ├── level: String
│   ├── relay_log: Option<String>
│   ├── tracker_log: Option<String>
│   ├── audit_log: Option<String>
│   └── admin_log: Option<String>
└── limits: LimitsConfig
    ├── max_connections: u32 (default 10000)
    ├── heartbeat_interval_seconds: u32 (default 30)
    └── tunnel_timeout_seconds: u32 (default 300)
```

---

## 5. Ports et services

| Service | Port | Protocole | Exposé | Processus |
|---------|------|-----------|--------|-----------|
| SSH | 22 | TCP | Oui | sshd |
| HTTP | 80 | TCP | Oui | nginx (redirect) |
| HTTPS | 443 | TCP | Oui | nginx (TLS termination) |
| Relay | 7000 | TCP+TLS | Oui | miyukini-origin |
| Tracker | 21000 | TCP | Oui | miyukini-origin |
| Web (interne) | 8080 | HTTP | Non | miyukini-origin |
| Admin (interne) | 8081 | HTTP | Non | miyukini-origin |

---

## 6. Checklist pré-déploiement

### Fichiers requis

- [ ] `/etc/miyukini/origin.toml` — Configuration principale
- [ ] `/etc/miyukini/admin.toml` — Configuration MiyukiniAdmin
- [ ] `/etc/miyukini/tokens.json` — Tokens d'authentification
- [ ] `/etc/miyukini/admin_jwt.key` — Clé secrète JWT
- [ ] `/etc/miyukini/tls/origin.crt` — Certificat TLS
- [ ] `/etc/miyukini/tls/origin.key` — Clé privée TLS

### Répertoires requis

- [ ] `/var/lib/miyukini/registry`
- [ ] `/var/lib/miyukini/keys`
- [ ] `/var/lib/miyukini/policies`
- [ ] `/var/log/miyukini`

### Services

- [ ] nginx — reverse proxy HTTPS
- [ ] miyukini-origin — service systemd
- [ ] firewalld — ports 80, 443, 7000, 21000
- [ ] chronyd — synchronisation NTP
- [ ] SELinux — `httpd_can_network_connect=on`

---

## 7. Fichiers modifiés

| Fichier | Action | Description |
|---------|--------|-------------|
| `setup-origin.sh` | Modifié | Configuration TOML alignée avec le code |
| `setup-origin.sh` | Modifié | Ajout répertoires keys/policies |
| `setup-origin.sh` | Modifié | Génération tokens.json |
| `setup-origin.sh` | Modifié | Correction nom fichier JWT |

---

## 8. Recommandations

### Priorité haute

1. **Supprimer le code mort** — 85 warnings de code inutilisé
2. **Tests unitaires** — Ajouter des tests pour la validation de config
3. **Mode dégradé** — Documenter le comportement sans TLS

### Priorité moyenne

4. **Unifier les scripts** — Un seul script de référence aligné avec la doc
5. **Variables d'environnement** — Permettre override via env vars
6. **Health check** — Endpoint `/health` pour monitoring

### Priorité basse

7. **Métriques Prometheus** — Exporter les métriques relay/tracker
8. **Graceful shutdown** — Signal handling propre

---

## Références

- [MWS - Implementation Origin Hostinger](./deploiement/MWS%20-%20Implementation%20Origin%20Hostinger.md)
- [MWS - Guide de Déploiement](./deploiement/MWS%20-%20Guide%20de%20Deploiement.md)
- Code source : `apps/origin/src/config.rs`

---

**Classification :** Documentation MWS — Audit  
**Auteur :** Agent IA Cursor  
**Validé par :** —
