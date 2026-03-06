# Brief de Cadrage P0 -- MiyuCloud Integration Fix

<!-- @id: mip.brief.miyucloud-integration-fix -->
<!-- @do: brief_p0_miyucloud_integration_from_scratch -->
<!-- @role: Maria (Chef de Projet) -->
<!-- @layer: S7-Operator -->

**Classification MIP** : T3 (Integration & fix)
**Date** : 2026-03-06
**Responsable P0** : Maria
**Statut** : EN ATTENTE APPROBATION
**Mode autonomie** : BIG_STEPS
**Sequence precedente** : `2026-03-05-miyucloud-v2-reprise` (bloquee V6/V8)

---

## TL;DR

MiyuCloud a un back-end solide (37 tests serveur, 14 tests integ, 0 warning) mais ne fonctionne pas
en conditions reelles. Le serveur compile, l'UI compile (isolement), mais l'integration Central <-> Server
est cassee au runtime. Cette sequence chirurgicale vise a debloquer la compilation workspace,
connecter Central au serveur, et valider le cycle E2E complet.

---

## 1. Contexte et etat des lieux

### 1.1 Pourquoi cette sequence

La sequence precedente `miyucloud-v2-reprise` a livre V0-V5 et V7 (securite, TOTP, onboarding, API,
hardening, infra) mais s'est bloquee sur :
- **V6 bloquee** : compile front echoue sur assets `lord_of_the_castle` hors perimetre
- **V8-V9 non demarrees** : doc MSCM, integration finale, audit
- **P5/P6 jamais executees** : pas de test humain reel

Le serveur passe tous ses tests mais **n'a jamais ete teste en conditions reelles** avec Central.

### 1.2 Diagnostic (3 axes)

**AXE 1 -- Le serveur ne demarre pas en conditions reelles**
- `MIYUCLOUD_PASSPHRASE` obligatoire mais pas de guidance utilisateur
- `MIYUCLOUD_COG_TOKEN` genere aleatoirement si absent -> Central ne peut pas le connaitre
- `connect_auth` requiert `central_db_path` non configure par defaut

**AXE 2 -- L'UI Central ne compile pas dans le workspace complet**
- `miyukini-central-native` tire `lord_of_the_castle` qui requiert des assets supprimes
- Les 14 fichiers UI miyucloud (~8200 lignes) n'ont jamais ete valides en compilation reelle
- Types dupliques entre `state.rs` (Central) et `crates/miyucloud/data/types.rs`

**AXE 3 -- Fonctionnalites non testees end-to-end**
- 2FA TOTP : back OK (15 tests), jamais teste via l'UI
- Onboarding : back OK (13 tests), jamais connecte au front
- Surface web HTTPS : TLS auto-signe OK en test, jamais teste en vrai
- Health dashboard : `check_disk_space()` retourne `(0, 0)` -- stub
- Sync P2P : tests integ OK, jamais teste entre 2 instances reelles

### 1.3 Ce qui fonctionne (a conserver)

| Composant | Lignes | Tests | Qualite |
|-----------|--------|-------|---------|
| Crypto (ChaCha20, Argon2, X25519, HKDF) | ~2000 | OK | Solide |
| Stockage chiffre chunke + integrite SHA-256 | ~800 | OK | Solide |
| API REST (46 routes, auth X-COG-Token) | ~3000 | 37 pass | Solide |
| Sync P2P (vector clock, conflit, transport E2E) | ~2500 | 13 pass | Solide |
| DB KindMother/SQLite (12 tables) | ~1500 | OK | Solide |
| Auth sessions + TOTP back | ~1200 | 29 pass | Solide |
| Security headers + rate limiter + sanitize | ~600 | OK | Solide |
| TLS auto-signe + dual server pattern | ~400 | OK | Bon |

**Conclusion : le back-end est mature et bien teste. Le probleme est l'integration.**

---

## 2. Perimetre

### 2.1 Inclus (IN)

1. Debloquer la compilation Central (dep `lord_of_the_castle` / assets manquants)
2. Aligner les types Central UI <-> crate miyucloud (supprimer doublons `state.rs`)
3. Configurer le lancement automatique MiyuCloud depuis Central (ServiceManager)
4. Passer le COG token + passphrase au serveur depuis Central
5. Implementer `check_disk_space()` pour Windows et Unix
6. Valider le cycle E2E : upload -> stockage chiffre -> partage -> surface web
7. Tester TOTP + onboarding via l'UI
8. Mettre a jour la memoire projet et MSCM

### 2.2 Exclu (OUT)

- Nouvelles features (tout est deja implemente)
- Refactoring du back-end (deja mature)
- Migration Jay1Tribu
- Multi-utilisateurs
- CI/CD deploy (deja en V7 de la sequence precedente)

---

## 3. Plan retenu (5 vagues, 18 taches)

### 3.1 Vue d'ensemble

| Vague | Nom | Taches | Gate |
|-------|-----|--------|------|
| V0 | Debloquer compilation | 3 | `cargo check -p miyukini-central-native` OK |
| V1 | Integration Central <-> Server | 5 | MiyuCloud demarre depuis Central, API repond |
| V2 | Test E2E cycle complet | 4 | Upload/partage/TOTP/onboarding fonctionnels |
| V3 | Monitoring + polish | 3 | `check_disk_space()` + health dashboard OK |
| V4 | Documentation + cloture | 3 | MSCM + memoire + gate P4/P5/P6 |
| **Total** | | **18** | |

### 3.2 DAG de dependances

```text
V0 -> V1 -> V2 -> V3 -> V4
```

Sequence lineaire -- chaque vague depend de la precedente.

### 3.3 Detail des vagues

**V0 -- Debloquer compilation (CRITIQUE)**
- T01: Resoudre la dependance `lord_of_the_castle` qui bloque le build Central
- T02: Verifier que les 14 fichiers UI miyucloud compilent dans le workspace
- T03: Aligner les types `state.rs` <-> `crates/miyucloud/data/types.rs`

**V1 -- Integration Central <-> Server (CRITIQUE)**
- T04: ServiceManager passe `MIYUCLOUD_COG_TOKEN` et `MIYUCLOUD_PASSPHRASE` au lancement
- T05: Stocker/lire le COG token en DB Central pour le client HTTP
- T06: Configurer `central_db_path` automatiquement
- T07: Health check au demarrage du service dans Central (GET /health)
- T08: Flow complet : Central demarre MiyuCloud -> API disponible -> UI se connecte

**V2 -- Test E2E cycle complet (HAUTE)**
- T09: Upload fichier via UI -> verifier en DB + stockage chiffre
- T10: Creer partage -> acceder via surface web HTTPS
- T11: Tester TOTP setup via UI -> verifier en DB
- T12: Tester onboarding wizard -> verifier completion

**V3 -- Monitoring + polish (MOYENNE)**
- T13: Implementer `check_disk_space()` pour Windows et Unix
- T14: Connecter le health dashboard UI aux donnees reelles
- T15: Verifier la purge rate_limiter en conditions de charge

**V4 -- Documentation + cloture (STANDARD)**
- T16: Mettre a jour MSCM pour les nouveaux fichiers
- T17: Mettre a jour memoire projet `project-miyucloud.md`
- T18: Gate P4 (audit) + P5 (test humain) + P6 (cloture)

---

## 4. Risques et mitigations

| # | Risque | Probabilite | Impact | Mitigation |
|---|--------|-------------|--------|------------|
| R1 | Dep `lord_of_the_castle` imbriquee profond | Moyen | Eleve | Feature-gate ou stub |
| R2 | Types desalignes cassent la deserialisation | Moyen | Moyen | `serde(default)` + `serde(alias)` |
| R3 | Passphrase storage dans Central pose un probleme securite | Faible | Eleve | Derive depuis master key Central, jamais en clair |
| R4 | Surface web TLS echoue en prod | Faible | Moyen | Test avec cert auto-signe + fallback HTTP local |

---

## 5. Metriques cibles

| Metrique | Valeur actuelle | Cible |
|----------|----------------|-------|
| Compile workspace complet | ECHOUE | OK |
| Tests miyucloud-server | 37 pass | 37+ pass |
| Tests miyucloud lib | 14 pass | 14+ pass |
| Clippy warnings | 0 | 0 |
| Cycle E2E fonctionnel | NON | OUI |
| Health `check_disk_space` | stub (0,0) | Valeurs reelles |
| Score securite | 72/100 (dernier audit) | >85/100 |

---

## 6. Decision de cadrage

- **Classification** : T3 (integration & fix)
- **Approche** : Sprint unique, 5 vagues lineaires
- **Mode autonomie** : BIG_STEPS (gate apres V1, puis flow continu V2-V4)
- **Branche** : `feat/miyucloud-integration-fix` (a creer depuis la branche courante)

---

## 7. Prochaines etapes

1. Approbation utilisateur du brief
2. Creation branche `feat/miyucloud-integration-fix`
3. Demarrage V0 (deblocage compilation)

---

*Brief redige par Maria -- Chef de Projet, Miyukini AI Studio*
*2026-03-06 -- P0*
