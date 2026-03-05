# MIP UI (JSX sans compilation)

Interface setup/configuration MIP en React JSX avec Babel CDN, sans build.

## Lancer

1. Ouvrir `index.html` dans le navigateur.
2. Cliquer **Connecter dossier .mip**.
3. Selectionner le dossier `.mip` du projet.
4. Modifier les champs puis **Sauvegarder**.

## Menus deroulants

- Le profil actif est charge depuis `.mip/profiles/builtin` et `.mip/profiles/custom`.
- Les choix de securite et de deploiement sont derives de `.mip/modules/setup.md` (S2.8/S2.15).
- Les modeles IA proposent des suggestions a partir des profils.

## Garde-fou

- Ecriture restreinte a ces chemins :
  - `environment.md`
  - `profiles/active`
  - `config/subscriptions.md`
  - `config/mip-configurator.state.json`
- Toute tentative de sortie de `.mip` est rejetee.
