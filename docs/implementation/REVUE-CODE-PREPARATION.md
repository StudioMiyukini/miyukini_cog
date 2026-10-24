# Revue de code COG — préparation

<!-- @id impl.revue -->
<!-- @role code-review-plan -->
<!-- @layer 0-9+zones -->
<!-- @human Périmètre, priorités, méthode, grille et constats d'entrée de la revue de code du COG -->
<!-- @do prepare_code_review -->

- **Statut** : prête à lancer, 2026-10-24
- **Référentiels** : [contraintes C-xx](CONTRAINTES-IMPLEMENTATION.md) · [menaces EX-xx](../securite/MODELE-MENACES-COG.md) · [doc gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [audit 2026-10-23](../securite/AUDIT-2026-10-23-auth-wan.md)
- **Objectif** : mesurer l'écart code ↔ doc gelée, trouver les failles exploitables, chiffrer le reste à faire

## 1. Inventaire mesuré (2026-10-24)

<!-- @id impl.revue.inventaire -->
<!-- @role code-inventory -->
<!-- @do measure_codebase -->

- 149 paquets dans le workspace · ≈ 296 000 lignes Rust · 1 936 tests
- 2 075 `unwrap()` et 446 `expect()` (tests compris) · 0 bloc `unsafe` · 64 `todo!` / TODO
- Lints du workspace hérités par 37 paquets, lints propres pour 106 ; `unsafe` interdit partout depuis R-004
- Outils absents du serveur : `cargo-audit`, `cargo-deny`, `cargo-fuzz` ; une seule CI (`miyucloud-ci.yml`)
- **Déséquilibre** : les 10 Cores totalisent ≈ 7 000 lignes ; MiyuCloud seul ≈ 82 000

| Paquet | Lignes | Tests | unwrap | Remarque |
|---|---|---|---|---|
| miyucloud-server (`apps/miyucloud`) | 68 812 | 228 | 365 | exposé (WebDAV, Nextcloud) |
| miyukini-central-native | 41 619 | 31 | 16 | peu de tests pour la taille |
| miyukini-origin | 21 246 | 75 | 37 | exposé Internet |
| miyucloud | 13 164 | 303 | 492 | |
| maia (pont LLM) | 9 600 | 38 | 42 | outils appelés par l'IA |
| jaycloud | 4 540 | 128 | 257 | liens publics signés |
| strongfather | 2 588 | 25 | 5 | Core |
| borderguard · worrysentinel | 523 · 392 | 7 · 6 | 1 · 0 | Cores sécurité minces |
| miyuantispam · miyuvalidate · miyuwebway_tracker | 259 · 305 · 441 | **0** | 0 | stubs sans test |

## 2. Périmètre par priorité

<!-- @id impl.revue.priorites -->
<!-- @role review-scope -->
<!-- @do prioritize_review -->

- **P0 — exposé, authentification, cryptographie** (en premier)
  - `apps/team` : écoute `0.0.0.0:7800`, aucune authentification, CORS ouvert, SSRF par enregistrement de nœud, `is_allowed_destination` jamais appelé
  - `apps/origin`, `crates/miyukini-central` (remote, `shell_exec`), `central-remote*`, `miyauth`, `miyusceau`
  - `crates/miyualicia-api` : persona prise dans le corps de requête (C-20)
  - `miyuantispam` (limite toujours « autorisée »), `miyuvalidate`, `miyuwebway_tracker` / `participant`
  - `apps/miyucloud`, `miyucloud-dav`, `jaycloud` (liens publics) — surfaces HTTP les plus larges
  - `apps/miou-llm-bridge` (skills : outils déclenchés par l'IA)
- **P1 — gouvernance** : Kernel, 10 Cores, `kindmother*`, `miyukini-admin`, `miyumarket`
- **P2 — services métier** : Jay*, MiyuCloud (logique), Alicia, MAIA, Whisper
- **P3 — UI et outils** : `miyuki-ui-*`, `apps/*-app`, MIPOWER, `tools/`

## 3. Méthode

<!-- @id impl.revue.methode -->
<!-- @role review-method -->
<!-- @do define_review_method -->

- **Automatique** (avant lecture, par paquet)
  - `cargo clippy -D warnings` avec les lints du workspace forcés
  - `cargo audit`, `cargo deny check` (à installer)
  - comptage `unwrap` / `expect` / `panic` hors tests ; liaisons réseau (`0.0.0.0`, `bind`) ; routes sans authentification
- **Manuelle** : grille §4, un relecteur par paquet P0, double lecture pour l'authentification et la cryptographie
- **Dynamique** : cibles de fuzzing sur les décodeurs P0 ; tests d'intrusion locaux sur les routes HTTP
- **Conformité** : pour chaque exigence EX-xx, statut **fait / partiel / absent / contredit** avec preuve `fichier:ligne`

## 4. Grille de lecture

<!-- @id impl.revue.grille -->
<!-- @role review-checklist -->
<!-- @do check_code -->

- **Entrées** : tailles bornées, décodage strict, aucune confiance dans un champ client (C-13, C-20)
- **Authentification** : toute route non locale protégée ; identité issue du justificatif ; secrets comparés en temps constant
- **Autorisation** : refus par défaut ; décision par StrongFather, pas dans le service
- **Réseau** : écoute locale par défaut ; aucune sortie depuis la zone COG ; pas d'URL fournie par un tiers appelée sans liste blanche (SSRF)
- **Cryptographie** : primitives autorisées seulement ; signature vérifiée avant décodage ; nonces et aléa via `OsRng`
- **Erreurs** : fail-closed ; pas de panique atteignable depuis l'extérieur
- **Données** : clé par service, étiquettes, aucune requête libre exposée
- **Architecture** : sens S → C → K ; persistance par KindMother ; pas d'exécution dans un Core
- **Tests** : cas de refus testés ; vecteurs figés pour les formats

## 5. Format des constats

<!-- @id impl.revue.format -->
<!-- @role finding-format -->
<!-- @do record_findings -->

- Un constat = `R-NNN` · gravité (**critique / haute / moyenne / basse**) · `fichier:ligne` · scénario d'exploitation · exigence (EX / C) · correctif proposé
- Registre unique : `docs/implementation/REVUE-CONSTATS.md` (créé au lancement)
- Critique ou haute sur P0 → correctif avant toute nouvelle fonctionnalité

## 6. Constats d'entrée (déjà connus)

<!-- @id impl.revue.entree -->
<!-- @role known-findings -->
<!-- @do list_known_findings -->

| Id | Gravité | Constat | Réf. |
|---|---|---|---|
| R-001 | critique | ✅ lot 0 (f225741d, d14a04a0) — TEAM : API sans authentification sur `0.0.0.0`, CORS `*`, SSRF via URL de nœud | C-21, C-22 |
| R-002 | haute | ✅ lot 0 (ae59e034, a71adb22) — Alicia API : persona issue du corps de requête | C-20 |
| R-003 | haute | ✅ lot 0 (cee31a09) — `miyuantispam` autorise toujours | C-12, C-14 |
| R-004 | moyenne | ✅ 2026-10-24 — mesure corrigée : 106 paquets ont leurs propres lints ; 6 sans interdiction de `unsafe` (miyucloud-server, mipower, 4 outils) → `unsafe_code = "forbid"` ajouté ; 148 / 148 couverts | C-01, C-02 |
| R-005 | moyenne | ✅ 2026-10-24 — CI `cog-ci.yml` (build, tests, clippy sécurité, `cargo deny`, `cargo audit`) + `deny.toml` ; à valider au premier push | C-04, EX-15 |
| R-006 | moyenne | Douane, Vigies, Portail, canal officiel : absents du code | EX-12, EX-19, EX-20, EX-44 |
| R-007 | basse | TEAM : file de tâches jamais consommée, état non persistant | — |
| R-008 | haute | ✅ lot 0 (e92afebc) — Jay Bureau : WebSocket collaboratif sans jeton, `CollabRoom` sans contrôle d'ACL | C-22 |
| R-009 | haute | ✅ lot 0 (e92afebc) — Jay Bureau : échange X25519 non signé par l'identité (MITM) ; nonce de groupe fourni par l'appelant | C-24 |
| R-010 | haute | ✅ lot 0 (4d0a6bf1) — Whisper : jeton optionnel, comparaison non constante, origine validée par préfixe (`localhost.evil.com`) ; `alicia-mvp` sans auth | C-21, C-22 |
| R-011 | moyenne | ✅ lot 0 (e92afebc) — Jay Bureau : SMTP sans TLS possible, mot de passe dans `Debug`, `mark_notification_read` sans propriétaire, repli `PATH` au lancement | C-25 |
| R-012 | moyenne | maia-llm : prompt assemblé par concaténation (faux tour système), `max_tokens` non borné, verrou global, clé visible en argument | C-13, C-50 |
| R-013 | basse | Conflit de port 11435 entre maia-llm et `miou-llm-bridge` | — |
| R-014 | basse | Incohérences code : manifeste JayRDV vers un binaire inexistant ; restes du remplacement de MiyuCloud dans JayCloud (`miyucloud_transition_days`) | — |
| R-015 | moyenne | ✅ lot 0 (cee31a09) — `miyuwebway_tracker` et `miyuvalidate` : stubs sans test | C-14, C-60 |
| R-016 | haute | ✅ lot 0 (ae59e034) — Alicia API : `/dispatch/confirm` sans état exécute directement une capacité soumise à confirmation (contournement du garde-fou) — n'exécuter que l'enregistrement en attente | C-12 |
| R-017 | moyenne | ✅ lot 0 (ae59e034) — Alicia : confirmation en attente non réclamée atomiquement → double exécution possible (`capabilities_runtime.rs`) | C-12 |
| R-018 | moyenne | ✅ lot 0 (cee31a09, a71adb22) — `jaymail-client` : `send_mail` piloté par l'IA accepte `bcc` → exfiltration par injection ; afficher tous les destinataires à la confirmation, refuser `bcc` depuis l'IA, signaler les destinataires hors contacts | C-50, C-51 |
| R-019 | haute | ✅ (9f6f6794, bibliothèque ; chaîne produit → R-023) — `miyukini-cog-bridge/src/e2e.rs` (pont Android) : `sha256()` n'est pas SHA-256 (XOR + DefaultHasher), handshake non signé, compteurs de nonce partagés entre les deux sens → réutilisation de nonce | C-24 |
| R-020 | moyenne | ✅ (cde0cb2c) — jay-collab : `set_acl` retire le membre mais ne coupe pas son flux d'événements (révocation incomplète) — jeton d'annulation par connexion | C-12 |
| R-021 | moyenne | ✅ (cde0cb2c) — jay-collab : plafond anti-rejeu global → saturation par un seul utilisateur ; ACL avant enregistrement du nonce, plafond par identité | C-13 |
| R-022 | moyenne | ✅ (cde0cb2c) — jay-collab : jeton de jonction sans audience (serveur) → relais possible vers un autre serveur pendant 120 s ; ajouter l'identité du serveur ou un défi-réponse | C-23 |
| R-023 | haute | ✅ (3e34112f) — Mobile : si le handshake E2E échoue, la connexion continue en clair et envoie email + mot de passe ; route COG `/api/bridge/e2e-handshake` absente, identités non provisionnées au pairing | C-12, C-23 |
| R-024 | moyenne | Pairing mobile : Central (bureau) n'affiche pas encore le code de confirmation → confiance au premier usage sur HTTP ; liste blanche des appareils non persistée | C-22 |
| R-025 | moyenne | `/auth` en clair accepté depuis la machine locale : un proxy inverse local rendrait toute requête « locale » ; prévoir le refus total derrière proxy | C-12 |
| R-026 | haute | Pairing : code de confirmation à 6 chiffres dérivé des seules clés, sans engagement → MITM par recherche de collision (~10⁶ essais) ; engagement préalable (hash de clé + nonce) avec nonces des deux côtés dans le code, ou empreinte complète / QR affiché par Central | C-23, C-24 |
| R-027 | moyenne | Pairing : une demande en attente peut être écrasée par une autre clé ; l'approbation ne vérifie pas la clé ou le code vus par l'administrateur | C-12 |
| R-028 | moyenne | Pairing : file d'attente plafonnée sans durée de vie → saturation ; TTL (10 min) et limite par source | C-13 |

## 7. Livrables

<!-- @id impl.revue.livrables -->
<!-- @role review-deliverables -->
<!-- @do deliver_review -->

- Registre des constats `R-NNN`
- Matrice de conformité EX-01..63 (fait / partiel / absent / contredit)
- Liste des corrections P0, ordonnée, intégrée au [plan d'implémentation](PLAN-IMPLEMENTATION.md) lot 0
