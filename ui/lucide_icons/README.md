# Lucide Icons — Pack UI central Miyukini

## Contexte

Pack d’icônes **Lucide** (SVG) pour l’UI générale du central : boutons de navigation, menus, actions courantes.

**Source :** [lucide-icons/lucide](https://github.com/lucide-icons/lucide)  
**Site :** [lucide.dev](https://lucide.dev)

## Contenu

- **1668 icônes** au format SVG (stroke 2px, style cohérent)
- **Rangement par catégories** (d’après les métadonnées Lucide) : chaque icône est dans un sous-dossier par catégorie.

### Catégories (dossiers)

`accessibility`, `account`, `animals`, `arrows`, `brands`, `buildings`, `charts`, `communication`, `connectivity`, `cursors`, `design`, `development`, `devices`, `emoji`, `files`, `finance`, `food-beverage`, `gaming`, `home`, `layout`, `mail`, `math`, `medical`, `multimedia`, `nature`, `navigation`, `notifications`, `people`, `photography`, `science`, `seasons`, `security`, `shapes`, `shopping`, `social`, `sports`, `sustainability`, `text`, `time`, `tools`, `transportation`, `travel`, `weather`

Exemple : `navigation/home.svg`, `time/clock.svg`, `arrows/chevron-left.svg`.

## Usage typique (central)

| Usage            | Icônes suggérées                    |
|------------------|-------------------------------------|
| Navigation menu  | `menu.svg`, `panel-left.svg`        |
| Accueil          | `home.svg`                         |
| Retour           | `arrow-left.svg`, `chevron-left.svg`|
| Paramètres       | `settings.svg`, `cog.svg`          |
| Fermer           | `x.svg`, `circle-x.svg`             |
| Valider          | `check.svg`, `circle-check.svg`    |
| Recherche        | `search.svg`                       |
| Catalogue / apps | `layout-grid.svg`, `apps.svg`       |

## Licence

ISC / MIT — usage libre (voir `LICENSE`). Attribution recommandée (Lucide Icons).

## Mise à jour

Pour récupérer une version plus récente des icônes :

1. Cloner le dépôt : `git clone --depth 1 https://github.com/lucide-icons/lucide.git temp_lucide`
2. Copier les SVG à la racine : `robocopy temp_lucide\icons ui\lucide_icons *.svg`
3. Ranger par catégories : exécuter `.\ui\lucide_icons\organize_by_category.ps1` (depuis la racine du repo ; le script lit les JSON dans `temp_lucide\icons` et déplace chaque SVG dans le dossier de sa première catégorie).
4. Supprimer le clone : `Remove-Item -Recurse -Force temp_lucide`
