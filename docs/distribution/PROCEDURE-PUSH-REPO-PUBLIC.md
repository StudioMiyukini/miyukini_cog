# Procédure — Mettre à jour le dépôt public avec la distribution

Ce document décrit comment publier une **distribution fonctionnelle** (binaires + doc complète, sans code source) dans le dépôt public Miyukini COG (ex. `miyukini-COG_public_EN`).

---

## Prérequis

- Rust installé (pour compiler les binaires).
- Accès en écriture au dépôt public (token ou SSH).
- PowerShell (Windows).

---

## Étapes

### 1. Générer la distribution

À la racine du dépôt **Miyukini_COG** (dépôt principal, avec le code source) :

```powershell
cd "c:\Users\miyuk\Documents\Cursor\Miyukini_COG"
.\scripts\prepare-public-distribution.ps1
```

Cela :

- compile en release `miyukini-central` et `kindmother-server` ;
- crée le dossier **`public-dist/`** avec :
  - **bin/** — les exécutables ;
  - **docs/** — toute la documentation du projet ;
  - **README.md** — présentation de la distribution ;
  - **MODE_EMPLOI.md** — mode d’emploi utilisateur ;
  - **LICENSE** ;
  - **docs/legal/** — politique de licence et licence pro service-tier.

Aucun code source n’est inclus.

### 2. Cloner le dépôt public (si pas déjà fait)

Dans un dossier **séparé** (hors Miyukini_COG) :

```powershell
cd C:\Users\miyuk\Documents\Cursor
git clone https://github.com/StudioMiyukini/miyukini-COG_public_EN.git miyukini-COG-public
cd miyukini-COG-public
```

(Adapter l’URL si le dépôt public a un autre nom.)

### 3. Remplacer le contenu du clone par la distribution

- Supprimer **tout** le contenu actuel du clone (sauf `.git/`).
- Copier **tout** le contenu de **`public-dist/`** à la racine du clone.

Exemple PowerShell (à exécuter depuis le clone du dépôt public) :

```powershell
# Depuis miyukini-COG-public
$dist = "c:\Users\miyuk\Documents\Cursor\Miyukini_COG\public-dist"
Get-ChildItem -Force | Where-Object { $_.Name -ne ".git" } | Remove-Item -Recurse -Force
Copy-Item -Path "$dist\*" -Destination "." -Recurse -Force
```

### 4. Commit et push

```powershell
git add -A
git status
git commit -m "Distribution Miyukini COG : binaires + doc complète (sans source)"
git push origin main
```

(Si le dépôt public utilise une autre branche par défaut, adapter.)

Pour l’authentification : utiliser un token dans l’URL ou configurer le credential helper (voir la doc Git / GitHub).

---

## Résumé

| Dépôt | Contenu |
|-------|--------|
| **Miyukini_COG** (privé/source) | Code source, crates, apps, docs, script de préparation. |
| **miyukini-COG_public_EN** (public) | Uniquement `bin/`, `docs/`, `README.md`, `MODE_EMPLOI.md`, `LICENSE` — pas de code source. |

Répéter les étapes 1 à 4 à chaque nouvelle version que vous souhaitez publier.
