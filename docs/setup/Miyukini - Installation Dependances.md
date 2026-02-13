# Miyukini Core System — Installation des dépendances

## Contexte

Ce document décrit comment installer toutes les dépendances du projet **miyukini-core-system** (https://github.com/StudioMiyukini/miyukini-core-system) sur Windows.

## Portée / Scope

- **Applicable à :** Développement local, post-clone
- **OS :** Windows (PowerShell)

---

## Prérequis

À installer sur ta machine (dans l’ordre si besoin) :

| Outil | Rôle | Lien |
|-------|------|------|
| **Git** | Cloner le dépôt | https://git-scm.com/download/win |
| **Node.js** (LTS) | npm / pnpm / yarn | https://nodejs.org/ |
| **Python 3** (si le projet contient du Python) | pip, pyproject | https://www.python.org/downloads/ |

Vérification en PowerShell (hors Cursor) :

```powershell
git --version
node --version
npm --version
# optionnel :
python --version
pip --version
```

---

## Étapes

### 1. Cloner le dépôt (si pas déjà fait)

Depuis un dossier parent (ou vide) :

```powershell
cd "c:\Users\miyuk\Documents\Cursor\Miyukini_COG"
git clone https://github.com/StudioMiyukini/miyukini-core-system.git .
```

Si le dépôt est privé, utilise un token ou SSH :

```powershell
git clone https://<TOKEN>@github.com/StudioMiyukini/miyukini-core-system.git .
```

### 2. Lancer le script d’installation

Depuis la **racine du projet** (là où se trouve le dossier `scripts/`) :

```powershell
cd "c:\Users\miyuk\Documents\Cursor\Miyukini_COG"
.\scripts\install-dependencies.ps1
```

Le script :

- détecte `package.json` → lance **npm** (ou **pnpm** / **yarn** si présents) ;
- détecte `requirements.txt` → lance **pip install -r requirements.txt** ;
- détecte `pyproject.toml` → lance **pip install -e .** ;
- parcourt les sous-dossiers type `apps/`, `packages/`, `frontend/`, `backend/`, etc. (monorepo).

### 3. Si exécution de scripts est bloquée

Si PowerShell affiche une erreur de stratégie d’exécution :

```powershell
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
```

Puis relancer `.\scripts\install-dependencies.ps1`.

---

## Installation manuelle (sans script)

Si tu préfères tout faire à la main :

- **Node :** à la racine (et dans chaque app/package qui a un `package.json`) :
  ```powershell
  npm install
  # ou : pnpm install / yarn install
  ```
- **Python :** à la racine (ou dans le dossier concerné) :
  ```powershell
  pip install -r requirements.txt
  pip install -e .   # si pyproject.toml présent
  ```

---

## Dépannage

- **« git / npm / pip non reconnu »**  
  Installer l’outil concerné et **rouvrir** PowerShell (ou Cursor) pour que le PATH soit à jour.

- **« Aucun package.json / requirements.txt trouvé »**  
  Vérifier que le clone a bien rempli le dossier (présence de `package.json`, `requirements.txt` ou `pyproject.toml` à la racine ou dans les sous-dossiers).

- **Erreurs réseau / proxy**  
  Configurer `npm` ou `pip` selon ton environnement (proxy, firewall).

---

## Voir aussi

- [Miyukini - Hostinger VPS Origin Webway](Miyukini%20-%20Hostinger%20VPS%20Origin%20Webway.md) — VPS Hostinger (Debian 13) pour héberger Origin (relay, Tracker MWS, catalogue).

---

**Date :** 2026-02-05  
**Projet :** miyukini-core-system
