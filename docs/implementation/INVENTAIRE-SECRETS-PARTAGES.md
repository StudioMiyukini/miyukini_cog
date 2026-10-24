# Inventaire des authentifications par secret partagé (sortie du lot 2)

<!-- @id impl.secrets-partages -->
<!-- @role shared-secret-inventory -->
<!-- @layer 0-9+zones -->
<!-- @human Points exposés qui authentifient encore par secret partagé ; cible de sortie du lot 2 : aucun -->
<!-- @do track_shared_secret_endpoints -->

- **Date** : 2026-10-24 · **Base** : `miyauth` passeports, visas et preuve de possession (`miyukini/pop/v1`) disponibles depuis le lot 2 incrément 4
- **Sortie du lot 2** : plus aucune authentification par secret partagé sur les points exposés ([plan §4](PLAN-IMPLEMENTATION.md))
- « Visa + PoP » : visa scellé et signature de possession
- **Avancement** : points WAN d'Origin (#11–14, #17) et CentralRemote (#1–5) traités (✅) ; services locaux #6–10 et #23 traités (✅, `miyauth::http::service` : clé de service en graine 0600, garde des clés épinglées) ; assistant réutilisable `miyauth::http` (schémas `MiyuVisa` / `MiyuPasseport` + `X-Miyu-PoP`, requêtes signées machine à machine)

## Points à remplacer

<!-- @id impl.secrets-partages.liste -->
<!-- @role endpoint-list -->
<!-- @do list_shared_secret_endpoints -->

| # | Point (fichier) | Mécanisme | Exposition | Remplacement |
|---|---|---|---|---|
| 1 | central-remote `POST /api/bridge/validate` (`crates/central-remote/src/server.rs`, `identite.rs`) | ~~`device_id` porteur~~ | LAN si `0.0.0.0` | ✅ visa + PoP ; passeport d'appareil (24 h) émis par Central sur requête signée par la clé épinglée à l'appairage v2 |
| 2 | central-remote `validate-token`, `generate-token` | ~~jeton API 30 j (QR)~~ | LAN | ✅ supprimés (404) ; mode clé API retiré du bridge et de Central Mobile |
| 3 | central-remote `GET /ws` | ~~jeton HMAC, Bearer ou `?token=`~~ | LAN | ✅ visa + PoP vérifiés avant l'upgrade ; toute chaîne de requête refusée ; session fermée à l'expiration du visa ou à la révocation |
| 4 | central-remote `POST /auth` | ~~courriel + mot de passe → session~~ | LAN (dans E2E) | ✅ supprimé : le passeport d'appareil porte l'identité (titulaire = profil qui a approuvé l'appareil) |
| 5 | central-remote admin `/api/bridge/*` | ~~secret `x-central-admin-secret` + loopback~~ | local | ✅ routes HTTP supprimées : Central appelle `PairingManager` dans le processus |
| 6 | miou-llm-bridge `/v1/*`, `/status`, proxy (`signature.rs`, `proxy.rs`) | ~~Bearer statique ; loopback ouvert (K-09)~~ | local, LAN si configuré | ✅ requête signée par un appelant épinglé (`[security] cles_appelants`, usage `maia/api`) ; hub TEAM seulement sur `POST /v1/agents/{id}/chat` (`cles_hub_team`, `maia/team`) ; aucune clé ⇒ 503, même en loopback |
| 7–8 | miou-llm-bridge → hub TEAM ; hub TEAM (`apps/team/src/auth.rs`) | ~~Bearer pré-partagé ; ouvert en loopback si absent~~ | local / LAN | ✅ REGISTER / heartbeat / retrait signés par la clé du nœud (`noeuds_autorises`, `team/noeud`), nœud lié à sa clé ; clients `team/api` (`clients_autorises`) ; hub → nœud signé (`maia/team`) ; `TEAM_HMAC_SECRET` ignoré |
| 9 | maia-llm `/v1/*`, `/maia/llm/models*` (`apps/maia-llm/src/auth.rs`) | ~~clé statique ; laisse passer si aucune clé~~ | local / LAN | ✅ requête signée par un appelant épinglé (`appelants_inference` / `appelants_modeles`) ; aucune clé ⇒ 503 ; `auth_key`, `X-MAIA-Key`, `--auth-key` retirés |
| 10 | miyustt / miyutts (`securite.rs`) ; clients alicia-mvp, `miyualicia::NluBridge` | ~~Bearer statique (`ALICIA_VOICE_TOKEN`)~~ | local / LAN | ✅ requête signée par un appelant épinglé (`miyustt/transcription`, `miyutts/synthese`) ; aucune clé ⇒ 503 ; Noise au lot 3 |
| 11–12 | Origin forum `validate`, `session/verify` (`apps/origin/src/web/forum_auth.rs`) | ~~mot de passe → jeton HMAC~~ | **WAN** | ✅ passeport + PoP → visa `forum` ; mot de passe seulement si `[forum] password_login` (défaut : non) |
| 13 | Origin `forum/sync` | ~~clé HMAC Central ⇄ Origin~~ | **WAN** | ✅ requête signée par la clé du COG (`[forum] sync_cles`), anti-rejeu |
| 14 | Origin Market publish (`web/market.rs`) | ~~`MIYUKINI_MARKET_DEV_TOKEN`~~ | **WAN** | ✅ requête signée + paquet scellé par un éditeur officiel (`[market] editeurs_officiels`) |
| 15 | Origin admin `/admin/*` | mot de passe → cookie `__Host-` | local derrière proxy | passeport admin + PoP (CSRF gardé) |
| 16 | Origin Tracker lobby | mot de passe de salon | WAN | facultatif : visas signés par l'hôte |
| 17 | Origin Relay « clé de Cores » (`relay/verification.rs`) | ~~clé statique par version~~ | **WAN** | ✅ retirée de toute décision ; Phase A = REGISTER signé (attestation de build : à spécifier) |
| 18 | MiyuCloud `/api/*`, DAV | ~~JWT HS256~~ ✅ sessions serveur + visa/PoP (2cf67520) | local ; LAN/WAN en pratique | visa + PoP (clients natifs) |
| 19 | MiyuCloud Nextcloud/OCS | Basic + mot de passe d'application (exception ✅ : TLS ou loopback, limite par IP) | idem | exception acceptée (clients DAV), TLS, portée limitée |
| 20 | MiyuCloud WOPI | `?access_token=` HS256 (exception ✅ : clé par processus, ≤ 15 min) | idem | exception imposée par WOPI : durée courte |
| 21 | MiyuCloud device flow + admin | ~~Bearer HS256~~ ✅ clé d'appareil + requêtes signées ; admin = session | idem | fin du flux = passeport d'appareil |
| 22 | miyukini-admin | mot de passe → cookie ; jeton d'installation | local | basse priorité |
| 23 | miyualicia-api `/auth/token` | client_id/secret → JWT HS256 | non branché | ✅ route retirée par défaut (feature `jeton-secret-client`, désactivée) ; cible : passeport de service + PoP |

- **Déjà signés** : REGISTER et enveloppes Tracker (`miyuwebway_participant`), Relay REGISTER, KindMother (défi Ed25519 mutuel), appairage v2 et poignée E2E de central-remote
- **À vérifier** : client forum-sync de `miyukinibb` apparemment non signé (#13 peut-être déjà cassé) ; synchronisation X25519 non authentifiée de `miyucloud/src/sync/transport.rs` (lot 3) ; clé CSRF de secours codée en dur dans `apps/cog-web-portal/src/csrf.rs`
