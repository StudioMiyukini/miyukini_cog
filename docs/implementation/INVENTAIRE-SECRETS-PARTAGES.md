# Inventaire des authentifications par secret partagé (sortie du lot 2)

<!-- @id impl.secrets-partages -->
<!-- @role shared-secret-inventory -->
<!-- @layer 0-9+zones -->
<!-- @human Points exposés qui authentifient encore par secret partagé ; cible de sortie du lot 2 : aucun -->
<!-- @do track_shared_secret_endpoints -->

- **Date** : 2026-10-24 · **Base** : `miyauth` passeports, visas et preuve de possession (`miyukini/pop/v1`) disponibles depuis le lot 2 incrément 4
- **Sortie du lot 2** : plus aucune authentification par secret partagé sur les points exposés ([plan §4](PLAN-IMPLEMENTATION.md))
- « Visa + PoP » : visa scellé et signature de possession
- **Avancement** : points WAN d'Origin (#11–14, #17) traités (✅) ; assistant réutilisable `miyauth::http` (schémas `MiyuVisa` / `MiyuPasseport` + `X-Miyu-PoP`, requêtes signées machine à machine)

## Points à remplacer

<!-- @id impl.secrets-partages.liste -->
<!-- @role endpoint-list -->
<!-- @do list_shared_secret_endpoints -->

| # | Point (fichier) | Mécanisme | Exposition | Remplacement |
|---|---|---|---|---|
| 1 | central-remote `GET /api/bridge/validate` (`crates/central-remote/src/server.rs`, `pairing.rs`) | `device_id` porteur | LAN si `0.0.0.0` | passeport d'appareil + PoP |
| 2 | central-remote `validate-token`, `generate-token` | jeton API 30 j (QR) | LAN | passeport + visa + PoP ; retrait du mode clé API |
| 3 | central-remote `GET /ws` (`session.rs`) | jeton HMAC, Bearer ou `?token=` | LAN | session liée au visa + PoP ; retirer `?token=` |
| 4 | central-remote `POST /auth` | courriel + mot de passe → session | LAN (dans E2E) | session issue du passeport d'appareil |
| 5 | central-remote admin `/api/bridge/*` | secret `x-central-admin-secret` + loopback | local | basse priorité |
| 6 | miou-llm-bridge `/v1/*` (`security.rs`, `proxy.rs`) | Bearer statique | local, LAN si configuré | visa + PoP |
| 7–8 | miou-llm-bridge → hub TEAM ; hub TEAM (`apps/team/src/auth.rs`) | Bearer pré-partagé ; ouvert en loopback si absent | local / LAN | REGISTER signé + visa + PoP |
| 9 | maia-llm `/v1/*` (`apps/maia-llm/src/auth.rs`) | clé statique ; **laisse passer si aucune clé** | local / LAN | visa + PoP |
| 10 | miyustt / miyutts (`securite.rs`) | Bearer statique | local / LAN | visa + PoP ou Noise (lot 3) |
| 11–12 | Origin forum `validate`, `session/verify` (`apps/origin/src/web/forum_auth.rs`) | ~~mot de passe → jeton HMAC~~ | **WAN** | ✅ passeport + PoP → visa `forum` ; mot de passe seulement si `[forum] password_login` (défaut : non) |
| 13 | Origin `forum/sync` | ~~clé HMAC Central ⇄ Origin~~ | **WAN** | ✅ requête signée par la clé du COG (`[forum] sync_cles`), anti-rejeu |
| 14 | Origin Market publish (`web/market.rs`) | ~~`MIYUKINI_MARKET_DEV_TOKEN`~~ | **WAN** | ✅ requête signée + paquet scellé par un éditeur officiel (`[market] editeurs_officiels`) |
| 15 | Origin admin `/admin/*` | mot de passe → cookie `__Host-` | local derrière proxy | passeport admin + PoP (CSRF gardé) |
| 16 | Origin Tracker lobby | mot de passe de salon | WAN | facultatif : visas signés par l'hôte |
| 17 | Origin Relay « clé de Cores » (`relay/verification.rs`) | ~~clé statique par version~~ | **WAN** | ✅ retirée de toute décision ; Phase A = REGISTER signé (attestation de build : à spécifier) |
| 18 | MiyuCloud `/api/*`, DAV | **JWT HS256** + Basic + mot de passe d'application | local ; LAN/WAN en pratique | visa + PoP (clients natifs) |
| 19 | MiyuCloud Nextcloud/OCS | Basic + mot de passe d'application | idem | exception acceptée (clients DAV), TLS, portée limitée |
| 20 | MiyuCloud WOPI | `?access_token=` HS256 | idem | exception imposée par WOPI : durée courte |
| 21 | MiyuCloud device flow + admin | RFC 8628 → Bearer HS256 | idem | fin du flux = passeport d'appareil |
| 22 | miyukini-admin | mot de passe → cookie ; jeton d'installation | local | basse priorité |
| 23 | miyualicia-api `/auth/token` | client_id/secret → JWT HS256 | non branché | passeport de service + PoP |

- **Déjà signés** : REGISTER et enveloppes Tracker (`miyuwebway_participant`), Relay REGISTER, KindMother (défi Ed25519 mutuel), appairage v2 et poignée E2E de central-remote
- **À vérifier** : client forum-sync de `miyukinibb` apparemment non signé (#13 peut-être déjà cassé) ; synchronisation X25519 non authentifiée de `miyucloud/src/sync/transport.rs` (lot 3) ; clé CSRF de secours codée en dur dans `apps/cog-web-portal/src/csrf.rs`
