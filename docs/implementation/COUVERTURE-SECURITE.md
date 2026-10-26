# Couverture de tests des crates de sécurité

<!-- @id impl.couverture.securite -->
<!-- @role test-coverage -->
<!-- @layer 0-9+zones -->
<!-- @human Méthode, commande et mesures de la couverture des crates de sécurité (C-62, C-26) -->
<!-- @do constrain_tests -->

- **Statut** : v1.1, 2026-10-26 — point L4 de [COG-POSTE-100](COG-POSTE-100.md), « couverture ≥ 80 % mesurée sur les crates de sécurité »
- **Contraintes** : C-62 et C-26 ([contraintes](CONTRAINTES-IMPLEMENTATION.md))
- **Outil** : `scripts/couverture-securite.sh` · **CI** : `.github/workflows/couverture.yml` (manuel et lundi 03:17 UTC)

## Méthode

- Mesure : `cargo-llvm-cov` 0.9.1 (MIT/Apache-2.0), composant `llvm-tools-preview`, chaîne 1.99.0.
- Indicateur du seuil : couverture des **lignes hors tests**, par crate, seuil 80 %.
- Lignes exclues : items `#[cfg(test)]` / `#[test]`, modules de test, `tests/`, `examples/`, `fuzz/`, `build.rs`.
- L'exclusion réutilise l'analyse lexicale de `scripts/compter_unwrap.py` (C-10).
- Indicateur complémentaire : **régions** llvm par crate, informatif (sans seuil). Mesure brute du résumé JSON de `cargo llvm-cov`, sur les mêmes fichiers. Les modules `#[cfg(test)]` internes y restent comptés.
- Les binaires lancés par les tests (ex. `miyuracine`, `miyurelais`) sont mesurés.
- Target dédié `/tmp/mcog-cov-target` (`COUV_TARGET_DIR`), supprimé en fin de run : le target principal n'est pas touché.
- Quatre tests de `miyuportail` sont sautés pendant la mesure : ils lancent le sas réel. Le binaire instrumenté ne peut pas écrire son profil sous seccomp et `RLIMIT_FSIZE = 0` : il est tué (`FinAnormale`), c'est le confinement qui agit. Ces tests restent lancés par `cog-ci`.
  - `chemin_nominal_par_le_sas_reel`, `sas_refuse_entree_malformee_ou_surdimensionnee`, `deuxieme_validation_divergente_refusee` ;
  - `descripteur_sans_cloexec_refuse_le_lancement` (ajouté le 2026-10-26 ; passe sans instrumentation).

## Commande

```
scripts/couverture-securite.sh                  # 23 crates, seuil 80 %
scripts/couverture-securite.sh --seuil 85       # ou SEUIL_COUVERTURE=85
scripts/couverture-securite.sh --crate miyauth  # restreint (répétable)
scripts/couverture-securite.sh --detail         # + couverture par fichier
scripts/couverture-securite.sh --garder         # conserve le target, le LCOV et le résumé JSON
scripts/couverture-securite.sh --lcov F         # tableaux depuis un LCOV existant (régions si F.json voisin)
```

- Codes de sortie : `0` au seuil, `1` crate sous le seuil, `2` crate introuvable, `3` build ou tests en échec, `4` outil absent.
- Durée locale : environ 10 min à froid ; 3 Go de disque.

## Mesures (2026-10-26, commit `05ce4d0d`)

Arbre de travail : commit `05ce4d0d` plus des modifications non commitées d'autres chantiers en cours (`maia-federee`, `miyauth`, `miyuportail`). Leurs chiffres peuvent donc bouger d'une mesure à l'autre.

| Crate | Lignes avant | Lignes après | Régions avant | Régions après |
|---|---|---|---|---|
| miyauth | 84,4 % | 83,3 % | 84,8 % | 84,0 % |
| miyusceau | 95,3 % | 95,3 % | 92,5 % | 92,5 % |
| miyuracine | 96,0 % | 96,0 % | 93,9 % | 93,9 % |
| miyuportail | 81,8 % | 83,1 % | 81,9 % | 82,7 % |
| miyucdr | 91,7 % | 91,7 % | 91,6 % | 91,6 % |
| miyudouane | 87,7 % | 87,7 % | 85,1 % | 85,1 % |
| miyuvigie | 90,6 % | 90,6 % | 90,7 % | 90,7 % |
| miyufrontiere | 81,2 % | 81,2 % | 91,6 % | 91,6 % |
| miyuregistre | 91,1 % | 91,1 % | 91,9 % | 91,9 % |
| miyucanal | 80,8 % | 80,8 % | 83,3 % | 83,3 % |
| miyufederation | 94,0 % | 94,0 % | 91,0 % | 91,0 % |
| miyuhybride | 87,9 % | 87,9 % | 86,0 % | 86,0 % |
| maia-federee | 94,3 % | 94,8 % | 94,0 % | 93,0 % |
| maia-memoire | 91,5 % | 91,5 % | 87,9 % | 87,9 % |
| maia-modeles | 87,2 % | 87,2 % | 88,5 % | 88,5 % |
| **miyurelais** | **69,7 %** | **85,4 %** | **75,8 %** | **86,1 %** |
| strongfather | 89,9 % | 89,9 % | 93,7 % | 93,7 % |
| borderguard | 92,3 % | 92,3 % | 87,8 % | 87,8 % |
| bondingbrother | 83,0 % | 83,0 % | 89,7 % | 89,7 % |
| worrysentinel | 84,2 % | 84,2 % | 87,2 % | 87,2 % |
| tamr | 100 % | 100 % | 100 % | 100 % |
| kindmother | 97,3 % | 97,3 % | 96,2 % | 96,2 % |
| kindmother-service | 88,4 % | 88,4 % | 86,8 % | 86,8 % |
| **Total** | **88,8 %** | **89,4 %** | **88,2 %** | **88,7 %** |

- Les 23 crates sont au seuil de 80 % des lignes.
- Les variations de `miyauth`, `miyuportail` et `maia-federee` viennent des chantiers en cours sur ces crates, pas de ce lot.

### Historique

| Date | Commit | Total lignes | Sous 80 % |
|---|---|---|---|
| 2026-10-25 | lot 8 | 89,9 % (21 crates) | aucune après ajout de tests (`miyuracine`, `kindmother`) |
| 2026-10-26 | `05ce4d0d` | 89,4 % (23 crates) | aucune après ajout de tests (`miyurelais`) |

## Tests ajoutés

### 2026-10-26 (`miyurelais`, 69,7 % → 85,4 %)

- `miyurelais/tests/refus.rs` : le relais NFLOG sur trames construites.
  - Configuration refusée : hôte vide, avec espace, non ASCII, trop long ; groupes vides ou contenant 0.
  - Paquet d'un groupe non configuré : refusé, jamais converti en ligne.
  - Erreur de socket : la relève s'arrête et échoue. Après des lignes lues, les lignes sont rendues **et** l'échec est porté (H1).
  - Datagramme plus long que le tampon : troncature comptée comme débordement, échec, la lecture continue.
  - Émetteur autre que le noyau : écarté, capteur dégradé.
  - Préfixe avec espace ou absent : refusé.
  - Interface non résolue ou nom invalide : `if<index>`, jamais le nom brut.
  - File des métadonnées IPS : bornée à `META_FLUX_MAX`, plus ancienne écartée et comptée ; UDP jamais « connexion nouvelle » ; ICMP sans métadonnée.
  - Voie désactivée : plus de « capteur muet » ; horodatage strictement croissant ; horloge système par défaut.
- `miyurelais/tests/binaire.rs` : le binaire en sous-processus, et la socket réelle.
  - Arguments refusés, code 2, rien sur la sortie standard : absents, veille non choisie ou choisie deux fois, valeur manquante ou invalide, argument inconnu.
  - Sans `CAP_NET_ADMIN` : ouverture NFLOG refusée par le noyau (`EPERM`), code 3, avec le groupe en cause.
  - `SocketNflog::ouvrir` : sans droit, le refus du noyau est rendu, jamais une socket à moitié liée. Avec le droit, seuls les groupes demandés sont liés.

### 2026-10-25 (lot 8)

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

- Marges faibles (< 85 % des lignes) : miyucanal (80,8 %), miyufrontiere (81,2 %), bondingbrother (83,0 %), miyuportail (83,1 %), miyauth (83,3 %), worrysentinel (84,2 %).
- Parmi elles, `miyauth` et `miyuportail` sont en cours de modification par d'autres chantiers : non touchées ici.
- `miyurelais` : la boucle de relève du binaire et la réception sur socket liée demandent `CAP_NET_ADMIN` ; elles sont couvertes par l'essai VM (`tests/vm/essai_nflog.sh`), pas par la mesure.
- `miyuportail` : le binaire `portail-sas` n'est pas mesuré (tué par son propre confinement sous instrumentation).
- Lors de la mesure « avant » du 2026-10-26, trois tests unitaires de `maia-federee` (`fiche::tests`) échouaient sur un chantier en cours ; ils passaient à la mesure « après ».
- Aucun bug de production trouvé pendant ce lot.
