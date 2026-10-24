# Contraintes d'implémentation COG

<!-- @id impl.contraintes -->
<!-- @role implementation-constraints -->
<!-- @layer 0-9+zones -->
<!-- @human Règles obligatoires pour tout code COG : build, erreurs, sécurité, architecture, données, IA, tests, traçabilité -->
<!-- @do constrain_implementation -->

- **Statut** : v1.0, 2026-10-24 — s'applique à tout code nouveau ou modifié
- **Base** : [doc conceptuelle gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [menaces EX-01..63](../securite/MODELE-MENACES-COG.md)
- **Règle** : une contrainte non tenue bloque la fusion ; une dérogation est écrite, motivée et datée dans la PR
- **Niveaux** : **[B]** bloquant · **[R]** requis avant la sortie du lot · **[V]** visé

## 1. Build et outillage

<!-- @id impl.contraintes.build -->
<!-- @role build-rules -->
<!-- @do constrain_build -->

- **C-01 [B]** Toute crate hérite des lints du workspace (`[lints] workspace = true`) — aujourd'hui 37 / 149
- **C-02 [B]** `unsafe_code = "forbid"` sans exception (0 bloc `unsafe` mesuré au 2026-10-24 : à conserver)
- **C-03 [B]** `cargo clippy -D warnings` propre sur les crates touchées
- **C-04 [B]** `Cargo.lock` versionné ; `cargo audit` et `cargo deny` (licences, doublons, sources) en CI (EX-15)
- **C-05 [R]** Chaîne Rust épinglée (`rust-toolchain.toml`)
- **C-06 [V]** Compilations reproductibles pour les Cores et services officiels (EX-04)

## 2. Erreurs et robustesse

<!-- @id impl.contraintes.erreurs -->
<!-- @role error-rules -->
<!-- @do constrain_errors -->

- **C-10 [B]** Aucun `unwrap`, `expect`, `panic!`, `todo!` hors tests dans : Kernel, Cores, toolkits sécurité, code exposé au réseau
- **C-11 [B]** Erreurs typées (`thiserror`) ; une variante par cause de refus
- **C-12 [B]** **Fail-closed** : toute erreur de contrôle = refus, jamais d'autorisation par défaut
- **C-13 [B]** Tailles bornées sur toute entrée (octets, éléments, profondeur)
- **C-14 [R]** Pas de `todo!` / `unimplemented!` en chemin d'exécution ; un stub refuse explicitement

## 3. Sécurité

<!-- @id impl.contraintes.securite -->
<!-- @role security-rules -->
<!-- @do constrain_security -->

- **C-20 [B]** Identité dérivée d'un justificatif vérifié, **jamais** d'un champ de la requête (persona, user_id, rôle)
- **C-21 [B]** Exposition désactivée par défaut ; écoute sur `127.0.0.1` sauf contrat signé (EX-48)
- **C-22 [B]** Authentification sur tout point d'accès non local ; comparaisons de secrets en temps constant
- **C-23 [B]** Signature vérifiée **avant** tout décodage de contenu (EX-09)
- **C-24 [B]** Cryptographie uniquement via `miyusceau`, `ed25519-dalek`, `sha2`, `rand_core::OsRng` ; aucune primitive maison
- **C-25 [B]** Secrets : `zeroize`, jamais journalisés, jamais dans le dépôt
- **C-26 [R]** Tout décodeur d'entrée externe a une cible de fuzzing (EX-10)
- **C-27 [B]** Moteur de service sans parseur de protocole externe (EX-41) ; décodage dans le Portail
- **C-28 [B]** Contenu non fiable jamais rendu tel quel (EX-23)

## 4. Architecture

<!-- @id impl.contraintes.architecture -->
<!-- @role architecture-rules -->
<!-- @do constrain_architecture -->

- **C-30 [B]** Sens des dépendances : S → C → K ; un Core ne dépend d'aucun service
- **C-31 [B]** Les Cores gouvernent, n'exécutent pas (D1) ; exécution par Douane, Vigies, Kernel
- **C-32 [B]** Services : intentions vers BondingBrother ; décision StrongFather ; persistance KindMother seule
- **C-33 [B]** Zone COG sans route sortante ; flux uniquement par le Portail (voies à sens unique, EX-44)
- **C-34 [R]** Chaque service déclare son contrat (ressources, flux, classe H0–H3) dans un manifeste signé (EX-16, EX-40)
- **C-35 [B]** MiyukiniAdmin en zone COG uniquement (D7)

## 5. Données

<!-- @id impl.contraintes.donnees -->
<!-- @role data-rules -->
<!-- @do constrain_data -->

- **C-40 [B]** Une clé de base par service (EX-21) ; étiquettes de sensibilité KindMother (EX-47)
- **C-41 [R]** Migrations réversibles, base principale versionnée (EX-08)
- **C-42 [B]** Aucun langage de requête exposé par une base secondaire (EX-07)

## 6. IA

<!-- @id impl.contraintes.ia -->
<!-- @role ai-rules -->
<!-- @do constrain_ai -->

- **C-50 [B]** Outils de MAIA appelés par intentions, jamais par commande libre
- **C-51 [B]** Marquage `etranger` propagé ; aucune action sur contenu étranger sans validation (EX-60)
- **C-52 [B]** Aucun apprentissage automatique sur données étrangères (EX-63)
- **C-53 [R]** Modèles reçus par le canal officiel, signés (EX-26)

## 7. Tests

<!-- @id impl.contraintes.tests -->
<!-- @role test-rules -->
<!-- @do constrain_tests -->

- **C-60 [B]** Tout changement est testé ; tout refus de sécurité a son test négatif
- **C-61 [B]** Formats binaires : vecteurs de référence figés (modèle : `miyusceau/tests/vecteurs.rs`)
- **C-62 [R]** Couverture mesurée des crates sécurité ; objectif ≥ 80 % des lignes
- **C-63 [V]** Tests de bout en bout entre deux COG (fédération, Portail)

## 8. Dépendances et licences

<!-- @id impl.contraintes.dependances -->
<!-- @role dependency-rules -->
<!-- @do constrain_dependencies -->

- **C-70 [B]** Nouvelle dépendance justifiée dans la PR ; fonctionnalités minimales
- **C-71 [B]** Licences Apache-2.0, MIT, BSD, ISC, Zlib ; aucun code GPL (IDS/IPS maison en salle blanche)
- **C-72 [R]** Modèles IA : Apache-2.0 ou MIT uniquement

## 9. Traçabilité

<!-- @id impl.contraintes.tracabilite -->
<!-- @role traceability-rules -->
<!-- @do constrain_traceability -->

- **C-80 [B]** Chaque PR cite ses exigences (EX-xx), décisions (Dx) et contraintes (C-xx)
- **C-81 [B]** Bloc MSCM `@id` sur tout nouveau module
- **C-82 [B]** Écart avec la doc gelée = amendement écrit avant le code ([gel](../architecture/GEL-DOC-CONCEPTUELLE-v1.md))
