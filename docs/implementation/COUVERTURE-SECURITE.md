# Couverture de tests des crates de sécurité

<!-- @id impl.couverture.securite -->
<!-- @role test-coverage -->
<!-- @layer 0-9+zones -->
<!-- @human Méthode, commande et mesures de la couverture des crates de sécurité (C-62, C-26) -->
<!-- @do constrain_tests -->

- **Statut** : v1.0, 2026-10-25 — lot 8, « couverture ≥ 80 % des crates sécurité »
- **Contraintes** : C-62 et C-26 ([contraintes](CONTRAINTES-IMPLEMENTATION.md))
- **Outil** : `scripts/couverture-securite.sh` · **CI** : `.github/workflows/couverture.yml` (manuel et lundi 03:17 UTC)

## Méthode

- Mesure : `cargo-llvm-cov` 0.9.1 (MIT/Apache-2.0), composant `llvm-tools-preview`, chaîne 1.99.0.
- Indicateur : couverture des **lignes hors tests**, par crate, seuil 80 %.
- Lignes exclues : items `#[cfg(test)]` / `#[test]`, modules de test, `tests/`, `examples/`, `fuzz/`, `build.rs`.
- L'exclusion réutilise l'analyse lexicale de `scripts/compter_unwrap.py` (C-10).
- Les binaires lancés par les tests (ex. `miyuracine`) sont mesurés.
- Target dédié `/tmp/mcog-cov-target` (`COUV_TARGET_DIR`), supprimé en fin de run : le target principal n'est pas touché.
- Trois tests du sas réel de `miyuportail` sont sautés pendant la mesure. Le binaire instrumenté ne peut pas écrire son profil sous seccomp et `RLIMIT_FSIZE = 0` : c'est le confinement qui agit. Ces tests restent lancés par `cog-ci`.

## Commande

```
scripts/couverture-securite.sh                  # 21 crates, seuil 80 %
scripts/couverture-securite.sh --seuil 85       # ou SEUIL_COUVERTURE=85
scripts/couverture-securite.sh --crate miyauth  # restreint (répétable)
scripts/couverture-securite.sh --detail         # + couverture par fichier
scripts/couverture-securite.sh --garder         # conserve le target et le LCOV
scripts/couverture-securite.sh --lcov F         # tableau depuis un LCOV existant
```

- Codes de sortie : `0` au seuil, `1` crate sous le seuil, `2` crate introuvable, `3` build ou tests en échec, `4` outil absent.
- Durée locale : environ 10 min à froid ; 3 Go de disque.

## Mesures (2026-10-25)

| Crate | Avant | Après |
|---|---|---|
| miyauth | 81,6 % | 81,6 % |
| miyusceau | 94,2 % | 94,3 % |
| **miyuracine** | **65,4 %** | **96,0 %** |
| miyuportail | 83,6 % | 83,6 % |
| miyucdr | 91,7 % | 91,7 % |
| miyudouane | 89,6 % | 89,6 % |
| miyuvigie | 89,9 % | 89,9 % |
| miyufrontiere | 81,2 % | 81,2 % |
| miyuregistre | 91,8 % | 91,8 % |
| miyucanal | 88,5 % | 88,5 % |
| miyufederation | 93,3 % | 93,3 % |
| miyuhybride | 87,7 % | 87,7 % |
| maia-federee | 93,1 % | 93,1 % |
| maia-memoire | 90,9 % | 90,9 % |
| strongfather | 89,9 % | 89,9 % |
| borderguard | 92,3 % | 92,3 % |
| bondingbrother | 83,0 % | 83,0 % |
| worrysentinel | 82,4 % | 82,4 % |
| tamr | 100 % | 100 % |
| **kindmother** | **72,2 %** | **99,8 %** |
| kindmother-service | 86,8 % | 86,8 % |
| **Total** | **86,6 %** | **89,9 %** |

## Tests ajoutés

- `miyuracine/tests/cli.rs` : l'outil de cérémonie, lancé en sous-processus.
  - Arguments refusés : option inattendue, répétée, sans valeur, hors bornes.
  - Fichiers : jamais d'écrasement (parts, graine, passation, fichier de clé), liens pendants refusés, droits 0600.
  - Sources : une part, trois parts, part altérée, parts de deux racines, fichier trop grand ou de clé invalide.
  - Rotation : sceau vérifié avec l'ancienne racine épinglée, refusé avec la nouvelle.
  - Fichier de clé : export Argon2id réel puis relecture, droits trop larges refusés.
  - Sans `--sans-mlock`, `RLIMIT_MEMLOCK = 0` : refus avant toute sortie de part.
- `miyuracine/tests/erreurs.rs` : un message distinct par motif de refus ; `FichierCleInvalide` ne distingue pas les causes.
- `kindmother/tests/refus.rs` :
  - Forme des intentions : bornes de colonnes et d'empreinte, opérations mal formées.
  - Lecture typée : colonnes invalides, tables réservées.
  - Cycle de vie : sauts et retours refusés, état final figé.
  - Pont MiyuSQL : tout mandat non signé est refusé, l'exécuteur n'est jamais atteint.
  - Stockage : cloisonnement par instance.

## Points d'attention

- Marges faibles (< 85 %) : miyauth, miyufrontiere, worrysentinel, bondingbrother, miyuportail.
- `miyuportail` : le binaire `portail-sas` n'est pas mesuré (tué par son propre confinement sous instrumentation).
- Aucun bug de production trouvé pendant ce lot.
