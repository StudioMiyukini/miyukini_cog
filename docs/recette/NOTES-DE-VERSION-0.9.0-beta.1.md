# Notes de version — COG 0.9.0-beta.1

<!-- @id recette.notes-version -->
<!-- @role release-notes -->
<!-- @layer 0-9+zones -->
<!-- @human Notes de version de la beta 0.9.0-beta.1 : livraison, installation, mise à jour, limites, vérifications du testeur -->
<!-- @do describe_beta_release -->

- **Statut** : v1.0, 2026-10-25 · **Version** : `0.9.0-beta.1` (workspace) · **Journal** : [CHANGELOG](../../CHANGELOG.md)
- **Cible** : machine de recette **dédiée**, Ubuntu 26.04, physique ou VM, chez un testeur. **Jamais** un hôte de production, un poste de travail ni une VM durable partagée
- **Liés** : [plan](PLAN-RECETTE-BETA.md) · [cas de test](CAS-DE-TEST-BETA.md) · [limites](LIMITES-CONNUES-BETA.md) · [modèle de PV](PV-RECETTE-MODELE.md)

## 1. Clé d'éditeur (à comparer hors bande)

<!-- @id recette.notes-version.cle -->
<!-- @role trust-anchor -->
<!-- @do publish_publisher_key -->

- Clé publique Ed25519 (hex) : `d802a28c1319e0d178ba371c401ec1d1d307fc85c0f0256ca9fe8fc5138da938`
- SHA-256 du fichier `cle-publique-editeur.pem` : `012ea776e61a3d9f6879e901e46b558c189183cff0214fdefcc267bd0229af83`
- Le testeur reçoit la clé par un canal distinct de l'archive (courriel signé, remise en main propre…) ; la clé contenue dans le paquet ne fait foi **que** si son empreinte correspond

## 2. Ce qui est livré

<!-- @id recette.notes-version.livre -->
<!-- @role deliverables -->
<!-- @do list_deliverables -->

- Archive `cog-0.9.0-beta.1-<commit12>.tar.zst` et son `.sha256`
- Binaires serveur (release, `--locked`) : `kindmother-server`, `maia`, `miyucloud-server`, `cog-web-portal`, `portail-sas`, `miyudouane`, `miyurelais`, `miyuregistre-publier`, `outil-sceau`, exemple `arretes_essai`
  - `miyuracine` (racine hors ligne) n'est **jamais** dans le paquet ; l'installateur refuse un paquet qui le contient
- `MANIFESTE` (SHA-256 de chaque fichier, commit, état de l'arbre), `MANIFESTE.sig` (signature détachée Ed25519, vérifiée par `openssl`), `MANIFESTE.sceau` (sceau `ManifestePaquet`)
- Installateur `installer.sh`, logique commune `commun.sh`, unités systemd, profils AppArmor `miyukini-*` (enforce)
- Exploitation, installée dans `/opt/miyukini/exploitation/` : `etat.sh`, `sauvegarder.sh`, `restaurer.sh`, `mettre-a-jour.sh`, `desinstaller.sh`
- Essais : `tests/tester.sh` (fumée), `tests/scenarios.sh` (scénarios LIVE), `tests/zone.sh` (repli de la zone COG), `tests/mise-a-jour.sh` (mise à jour)

## 3. Sécurité de l'installation

<!-- @id recette.notes-version.securite -->
<!-- @role install-security -->
<!-- @do describe_install_security -->

- **Authentification avant toute exécution** : `MANIFESTE` et sa signature sont copiés dans un répertoire root 0700, vérifiés par `openssl` avec la clé fournie par le testeur, puis chaque fichier est contrôlé contre cette copie. Aucun fichier du paquet n'est exécuté avant
- **Refus d'une machine de production** : unité active `cloudflared`, `docker`, `nginx`, `apache2`, `php-fpm`, `pm2`, Immich, Nextcloud, ou répertoire `/etc/cloudflared`, `/opt/miyucloud`, `/var/www/nextcloud`
- **Zone COG** : table `inet miyudouane_hote` **générée** par `miyudouane hote --description /etc/miyukini/douane-hote.desc --appliquer --fichier /etc/miyukini/douane-cog.nft` (format `miyudouane-hote 2`)
  - comptes COG sans sortie hors boucle locale ; ports COG joignables par la seule boucle locale (IPv4 et IPv6)
  - boucle locale **par compte** : `miyukini-maia → 127.0.0.1:50051` (KindMother), `miyukini-miyucloud → 127.0.0.1:5432` (PostgreSQL) ; tout autre flux local (résolveur 127.0.0.53, autre port, autre compte) est rejeté, journalisé (`MDOU|zone-cog|sortie|SORTIE-COG|`, NFLOG 110) et compté
  - table remplacée seule, jamais de `flush ruleset`
- **Rattachement systemd** : la zone suit `nftables.service` (`PartOf=`) ; les services COG suivent la zone (`BindsTo=`) ; rechargement par `systemctl reload miyukini-douane-cog`
- **Contrôle périodique fail-closed** (minuteur, chaque minute) : table absente ⇒ rechargée ; non rétablie ⇒ services COG arrêtés
- **Secrets** générés sur place (0600), conservés d'une réinstallation à l'autre ; aucun secret dans le paquet ni affiché

## 4. Procédure du testeur

<!-- @id recette.notes-version.procedure -->
<!-- @role tester-procedure -->
<!-- @do run_tester_procedure -->

1. Machine dédiée Ubuntu 26.04, à jour, accès SSH ; instantané vierge si VM
2. `sha256sum -c cog-….tar.zst.sha256` puis `zstd -dc cog-….tar.zst | tar -xf -` ; `cd cog-…`
3. `sha256sum cle-publique-editeur.pem` : doit valoir l'empreinte du §1 (ou utiliser la clé reçue hors bande)
4. Installation : `sudo ./installer.sh --confirme-machine-de-recette --cle-publique-pem <clé.pem>`
   - ou `--empreinte-cle-pem <sha256 du §1>`
5. État : `sudo /opt/miyukini/exploitation/etat.sh` (code 0 attendu)
6. Fumée : `tests/tester.sh` (compte sudo)
7. Zone : `tests/zone.sh` (environ 4 min)
8. Amorçage (premier administrateur MiyuCloud, clés MAIA) : instructions affichées en fin d'installation ; scénarios LIVE : `bash tests/scenarios.sh`
9. Sauvegarde : `sudo /opt/miyukini/exploitation/sauvegarder.sh --phrase-fichier <fichier 0600, ≥ 16 caractères>` ; copier l'archive hors machine
10. Mise à jour (second paquet fourni par le mainteneur) : `sudo tests/mise-a-jour.sh --paquet-maj <paquet B> --cle-publique-pem <clé.pem> --confirme-machine-de-recette`
11. Désinstallation en fin de recette : `sudo /opt/miyukini/exploitation/desinstaller.sh --purger --confirme-purge`, ou retour à l'instantané

## 5. Mise à jour et retour arrière

<!-- @id recette.notes-version.mise-a-jour -->
<!-- @role update-rollback -->
<!-- @do describe_update -->

- Commande : `sudo /opt/miyukini/exploitation/mettre-a-jour.sh --paquet <archive> --cle-publique-pem <clé> --confirme-machine-de-recette`
- Ordre : extraction (rien n'est exécuté) → authentification → **sauvegarde préalable automatique** (clé locale, code compris) → remplacement → contrôle de santé → retour arrière automatique si échec
- Codes : `0` réussie · `1` refus avant toute modification (archive altérée, mauvaise clé, arguments) · `3` échec, retour arrière réussi · `4` échec du retour arrière (intervention manuelle, sauvegarde indiquée)
- Journal : `/var/backups/miyukini/mises-a-jour.journal`
- `--essai-echec-sante` force l'échec du contrôle de santé pour éprouver le retour arrière

## 6. Ce que le testeur doit vérifier

<!-- @id recette.notes-version.verifier -->
<!-- @role tester-checks -->
<!-- @do list_tester_checks -->

- Installation refusée avec une autre clé, une empreinte fausse, un fichier du paquet modifié : **rien** n'est installé
- `tester.sh` : tous les contrôles `ok` ; en particulier sortie Internet impossible pour les quatre comptes COG, 127.0.0.53:53 refusé et journalisé, flux locaux déclarés seuls joignables
- `zone.sh` : `restart nftables`, table supprimée, fichier de zone absent ⇒ jamais de service COG actif sans zone
- `mise-a-jour.sh` : refus avant sauvegarde (altération, mauvaise clé), retour arrière forcé (code 3, données et version précédente restaurées), mise à jour réussie (code 0)
- Aucune écoute hors boucle locale (`ss -ltn`) ; profils AppArmor en enforce ; aucun `DENIED` inexpliqué (`journalctl -k | grep DENIED`)
- Redémarrage de la machine : zone chargée avant les services, `tester.sh` de nouveau vert

## 7. Limites connues

<!-- @id recette.notes-version.limites -->
<!-- @role known-limits -->
<!-- @do summarize_limits -->

- Liste complète : [LIMITES-CONNUES-BETA](LIMITES-CONNUES-BETA.md) ; principales :
- Paquet construit depuis un arbre **modifié** tant que le mainteneur n'a pas commité : marqué `non-reproductible` (nom, `VERSION`, `MANIFESTE`) ; reproductibilité vérifiable par `--verifier-reproductible` sur kindmother-server, portail-sas et miyudouane seulement
- Arrêtés de la Douane : **arrêtés d'essai** (clé jetable, valides un jour, vérifiés à l'installation)
- MAIA sans modèle (API LLM en 503) ; le compte MAIA n'a pas de flux vers son propre port 11435
- Sauvegarde automatique chiffrée par une clé **locale** : ne restaure que sur la même machine
- Retour arrière des données = restauration de la sauvegarde préalable (migrations MiyuCloud montantes seulement)
- Relais NFLOG `miyurelais` livré mais non installé en service ; aucun audit externe ; beta à ne pas exposer à Internet
