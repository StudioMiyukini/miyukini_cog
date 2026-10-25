# Journal des versions — Miyukini COG

<!-- @id changelog -->
<!-- @role changelog -->
<!-- @layer 0-9+zones -->
<!-- @human Journal des versions du COG : ce qui change d'une version à l'autre -->
<!-- @do list_releases -->

- Format : une section par version, la plus récente en tête ; détails dans les notes de version
- Version du workspace : `Cargo.toml` racine (`[workspace.package]`)

## 0.9.0-beta.1 — 2026-10-25 (beta de recette)

<!-- @id changelog.0-9-0-beta-1 -->
<!-- @role release -->
<!-- @do describe_release_0_9_0_beta_1 -->

- **Statut** : beta pour une **machine de recette dédiée** (Ubuntu 26.04, physique ou VM) ; jamais un hôte de production
- **Notes de version** : [docs/recette/NOTES-DE-VERSION-0.9.0-beta.1.md](docs/recette/NOTES-DE-VERSION-0.9.0-beta.1.md)

### Ajouté

- Paquet de recette signé : `deploy/recette/construire-paquet.sh` (binaires serveur en release `--locked`, `MANIFESTE` SHA-256 lié au commit, signature détachée Ed25519 vérifiable par `openssl`, sceau `ManifestePaquet` par `outil-sceau`)
- Installateur de recette `deploy/recette/installer.sh` : refus d'une machine qui ressemble à une production, authentification du paquet **avant toute exécution** de son contenu, profils AppArmor en enforce, zone COG de la Douane
- Exploitation (`/opt/miyukini/exploitation/`) : `etat.sh`, `sauvegarder.sh` (chiffrée), `restaurer.sh`, `mettre-a-jour.sh`, `desinstaller.sh`
- Mise à jour : paquet altéré ou mauvaise clé refusés avant toute sauvegarde ; sauvegarde préalable automatique ; retour arrière automatique si le contrôle de santé échoue
- Essais : `deploy/recette/tests/mise-a-jour.sh` (refus, retour arrière forcé, réussite) et `deploy/recette/tests/zone.sh` (repli de la zone COG)
- Douane : `miyudouane hote` génère la table hôte `inet miyudouane_hote` depuis une description déclarative (format `miyudouane-hote 2`) ; boucle locale **par compte** (`boucle`), résolveur 127.0.0.53 refusé et journalisé
- Zone COG rattachée à `nftables.service` (`PartOf=`), rechargeable, contrôlée chaque minute (minuteur fail-closed : rechargement, sinon arrêt des services COG)
- Relais NFLOG `miyurelais`, éditeur du registre de révocation, étiquette `publiable` (A-015) : voir les notes de version pour leur périmètre

### Modifié

- `deploy/cog-vm/installer.sh` : logique commune extraite dans `deploy/cog-vm/commun.sh`, partagée avec l'installateur de recette
- La table hôte n'est plus écrite à la main par les installateurs : elle est rendue et auditée par `miyudouane hote`
- `tester.sh` : contrôles de zone par compte (flux déclarés, refus comptés et journalisés), rattachement systemd

### Corrigé

- MAIA : reconnexion à KindMother sans écriture dupliquée (`maia-memoire`)

### Limites connues

- Voir [LIMITES-CONNUES-BETA](docs/recette/LIMITES-CONNUES-BETA.md) et la section « Limites » des notes de version
