# Chiffrement des bases de données (SQLCipher)

## Vue d'ensemble

Les bases SQLite du projet Miyukini COG sont chiffrées au repos via **SQLCipher** (AES-256). La clé est dérivée de manière déterministe à partir de :

- **Secret d'installation** : stocké dans `data_dir/.kindmother_secret` (généré à la première exécution)
- **Identifiant machine** : MachineGuid (Windows) ou `/etc/machine-id` (Linux/macOS)
- **Nom de la base** : salt unique par fichier `.db`

## Compilation sur Windows

SQLCipher requiert **OpenSSL**. Deux options :

### Option 1 : OpenSSL pré-compilé (recommandé)

1. Télécharger OpenSSL depuis [slproweb.com/products/Win32OpenSSL.html](https://slproweb.com/products/Win32OpenSSL.html)
2. Installer la version "Win64 OpenSSL" (fichier complet, pas "Light")
3. Définir la variable d'environnement avant de compiler :
   ```powershell
   $env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"
   cargo build
   ```
4. Adapter le chemin selon votre installation

### Option 2 : OpenSSL vendored (nécessite Perl)

Modifier les `Cargo.toml` pour utiliser `bundled-sqlcipher-vendored-openssl` au lieu de `bundled-sqlcipher`. Nécessite Perl (ex. Strawberry Perl) et les outils de compilation Visual Studio.

## Migration depuis des bases non chiffrées

**Important** : Les bases existantes créées avant cette implémentation ne sont pas chiffrées. Une base non chiffrée ouverte avec une clé SQLCipher échouera.

**Pour les nouvelles installations** : Rien à faire, les bases seront créées chiffrées.

**Pour migrer des données existantes** :
1. Sauvegarder les fichiers `.db` actuels
2. Exporter les données (ou les conserver en backup)
3. Supprimer les anciens fichiers `.db`
4. Relancer l'application : de nouvelles bases chiffrées seront créées
5. Ré-importer les données si nécessaire

Le secret d'installation (`.kindmother_secret`) est créé automatiquement au premier lancement. **Ne pas le perdre** : sans lui, les données chiffrées seront inaccessibles.
