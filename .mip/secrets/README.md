# Secrets - Authentification outils externes

> Ce dossier est dans .gitignore. Ne jamais committer son contenu.

Stocke les informations necessaires pour que les agents puissent s'authentifier sur les outils externes :

- GitHub : token, SSH keys
- VPS : cles SSH, credentials
- Auth : tokens OAuth, API keys
- Autres : toute donnee sensible pour les services externes

Format recommande : un fichier par service (ex. `github.env`, `vps.env`) ou variables d'environnement chargees depuis `secrets/.env` (non versionne).

Configuration initiale : lors du SETUP MIP, ce dossier est cree et ajoute au `.gitignore` du projet.

## Politique minimale

- 1 fichier par service (`github.env`, `vps.env`, `provider.env`).
- Aucune valeur secrete dans les fichiers versionnes.
- Rotation immediate apres incident, partage, ou depart collaborateur.
- Tenir `inventory.md` (service, proprietaire, date creation, date rotation, sans secret).
- Permissions restreintes : Linux/macOS `chmod 600`, Windows `icacls` en acces utilisateur courant.

## Exemple de fichiers

Création d'exemples non secrets : `github.env`, `openai.env`, `anthropic.env`, `vps.env`, `aws.env`.

Chaque fichier contient des variables d'environnement avec des valeurs de remplacement (ex. `your_api_key_here`).

Fichiers de centralisation auth applicative:

- `kindmother.env`
- `miyucloud.env`
- `supabase.env`
- `auth-centralisation.md` (cartographie codebase)
