# Tests JayFestival

Les tests de parcours (parcours_unc, parcours_org, parcours_exp, parcours_vis, global_router) ont été supprimés car ils référençaient les modules `app_state` et `screens` retirés lors de la migration UI vers Miyukini Central.

Les tests unitaires des modules `auth` (permissions, sign_in/sign_up) restent actifs dans `src/auth/`.

Pour exécuter les tests : `cargo test -p jayfestival`
