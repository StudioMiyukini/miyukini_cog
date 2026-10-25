# Compilations reproductibles des Cores et services officiels

<!-- @id impl.build.reproductible -->
<!-- @role build-reproducibility -->
<!-- @layer 0-9+zones -->
<!-- @human Méthode, commande et limites de la vérification de reproductibilité (EX-04, C-06) -->
<!-- @do constrain_build -->

- **Statut** : v1.0, 2026-10-25 — lot 8, exigence EX-04 « compilations reproductibles des Cores et services officiels »
- **Contrainte** : C-06 ([V] dans [contraintes](CONTRAINTES-IMPLEMENTATION.md))
- **Outil** : `scripts/build-reproductible.sh` · **CI** : `.github/workflows/reproductible.yml` (tag `v*` et déclenchement manuel)

## Méthode

- Le commit `HEAD` est exporté deux fois par `git archive` dans deux répertoires source distincts (`a/src`, `b/src`), chacun avec son propre `target`.
- Build `--release --locked` par paquet et binaire, `CARGO_INCREMENTAL=0`, `codegen-units=1`, debuginfo désactivée (variables d'environnement, `Cargo.toml` non modifié).
- `SOURCE_DATE_EPOCH` = date du commit. Environnement vidé (`env -i`), `TZ=UTC`, `LC_ALL=C`.
- `--remap-path-prefix` pour le source, le `target`, `CARGO_HOME` et le sysroot Rust.
- Comparaison des SHA-256 des deux binaires. Une différence fait échouer le script (code 1) et liste les binaires en cause.
- Manifeste écrit au format `nom  sha256` dans `target/reproductible/manifeste.txt` (ou `--manifeste`).

## Commande

```
scripts/build-reproductible.sh                              # liste par défaut
scripts/build-reproductible.sh --sous-ensemble portail-sas,miyudouane
scripts/build-reproductible.sh --liste                      # binaires et paquets
scripts/build-reproductible.sh --garder                     # conserve /tmp/mcog-repro-*
```

- Variables : `MCOG_REPRO_BASE` (répertoire de travail, défaut `/tmp`), `MCOG_REPRO_CODEGEN_UNITS` (défaut `1`).
- Codes de sortie : `0` identique, `1` différence, `2` erreur d'usage, autre valeur : erreur de build (`cargo`).

## Binaires couverts

- Liste par défaut : `miyudouane`, `portail-sas`, `miyuracine`, `kindmother-server`, `kindmother-import`.
- Hors défaut : `miyukini-central` (Dioxus, libs GTK requises sur Linux). À passer explicitement avec `--sous-ensemble`.
- Les Cores sans binaire (kernel, strongfather, borderguard, etc.) sont des bibliothèques compilées transitivement.

## Limites connues

- Le script compile le commit `HEAD` : les modifications non commitées sont ignorées (avertissement affiché).
- Deux builds sur la même machine et le même toolchain. La reproductibilité entre machines (toolchain, glibc, linker) n'est pas démontrée.
- La version du toolchain est épinglée en CI (1.99.0) ; un toolchain local différent peut produire d'autres SHA.
- `CARGO_HOME` (registre des dépendances) reste embarqué dans les chemins de panique et de débogage, remappé en `/build/cargo`. Un test avec un `CARGO_HOME` différent n'a pas été fait.
- `env!("CARGO_MANIFEST_DIR")` en code de production embarque le chemin source dans le binaire. `remap-path-prefix` ne le couvre pas. Relevé hors liste par défaut : `crates/miyukini-admin/src/main.rs` (lignes 229, 312, 630, 1170, 1185) et des usages dans `apps/central` à vérifier (certains sont en tests).
- Le temps d'exécution est doublé : deux compilations complètes de chaque binaire.

## Résultats

- Exécution du 2026-10-25 sur le commit `680112d8` (SOURCE_DATE_EPOCH 1792922704) : 5 binaires sur 5 identiques (liste par défaut, environ 4 min pour les deux passes).
- Causes de non-reproductibilité trouvées dans le périmètre : aucune. Aucun `build.rs` dans les crates de la liste ; pas de date ou de chemin source compilé.
- Contrôle : sans `remap-path-prefix`, deux builds dans deux répertoires source donnent aussi des SHA identiques pour `miyudouane`. Le chemin source n'apparaît pas dans ces binaires (chemins relatifs) ; `CARGO_HOME` y apparaît.
