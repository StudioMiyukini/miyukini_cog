# Déploiement COG en VM jetable (`deploy/cog-vm/`)

Installation de test des services COG dans la VM jetable `cog-essai` (Ubuntu 26.04, QEMU/KVM).

> **INTERDIT SUR UN HÔTE RÉEL.** Ce dossier ne doit **jamais** être utilisé — ni `installer.sh`,
> ni `tester.sh`, ni `pousser.sh` vers une autre cible — sur un hôte réel : serveur de production
> (Cloudflare tunnel, Immich, Nextcloud, pm2, etc.), serveur personnel, poste de travail ou VM
> durable. Il crée des comptes système, réécrit des unités systemd, installe PostgreSQL, change le
> mot de passe d'un rôle et, avec le banc, manipule nftables et des espaces de noms réseau.
> Il n'est **pas** un déploiement de production et n'en est pas une ébauche.
>
> Garde-fous : `installer.sh` refuse de s'exécuter sans `--vm-jetable` et sur un hôte non
> virtualisé (`systemd-detect-virt --vm`) ; `pousser.sh` ne vise que la VM `cog-essai`
> (`127.0.0.1:2222`, clé `~/vm/cog-essai/cle_vm`). Ces garde-fous ne remplacent pas la règle.

## Fichiers

| Fichier | Où il s'exécute | Rôle |
|---|---|---|
| `pousser.sh` | hôte de développement | prépare un paquet hors dépôt (binaires de `target/release`, config, statiques, banc), écrit son `MANIFESTE` (SHA-256 lié au commit), le copie dans la VM, l'y vérifie et lance l'installateur |
| `installer.sh` | VM jetable, root | vérifie le paquet contre son `MANIFESTE`, puis installation idempotente : utilisateurs, `/opt/miyukini`, `/etc/miyukini`, `/var/lib/miyukini/<svc>` (0700), PostgreSQL, unités systemd durcies |
| `tester.sh` | VM | tests de fumée, sortie TAP ; `--banc` ajoute le banc d'intrusion lot 3 |

## Utilisation

```sh
deploy/cog-vm/pousser.sh          # construit le paquet, pousse, installe
# dans la VM :
/home/cog/cog-paquet/tester.sh          # tests de fumée (62 contrôles)
/home/cog/cog-paquet/tester.sh --banc   # + banc d'intrusion lot 3 (root, ~2 min)
```

Les binaires doivent être construits au préalable (`cargo build --release -p <paquet>`).
`pousser.sh --sans-installation` ne fait que déposer et vérifier le paquet.

## Intégrité du paquet (manifeste)

`pousser.sh` écrit `MANIFESTE` à la racine du paquet : un en-tête (`# commit <sha>`,
`# arbre propre|modifié`, date) puis le SHA-256 de **chaque** fichier du paquet (format
`sha256sum`). Il affiche le commit, l'état de l'arbre, le SHA-256 de chaque binaire de `bin/` et
l'empreinte du manifeste lui-même. Un arbre « modifié » signale des binaires qui ne se
reconstruisent pas depuis le commit annoncé.

Dans la VM, avant toute installation :

1. l'empreinte du `MANIFESTE` reçu est comparée à celle calculée sur l'hôte (`pousser.sh`, puis de
   nouveau par `installer.sh --manifeste-sha256 <hex>`) ;
2. `sha256sum --check --strict` sur tous les fichiers, et aucun fichier hors manifeste ;
3. après copie, les binaires de `/opt/miyukini/bin` sont revérifiés contre
   `/opt/miyukini/MANIFESTE` (conservé ; `tester.sh` le recontrôle).

## Services installés

| Unité | Utilisateur | Écoute | Données | Notes |
|---|---|---|---|---|
| `miyukini-kindmother` | `miyukini-kindmother` | 127.0.0.1:50051 | `/var/lib/miyukini/kindmother` | identité et clé de base générées au premier démarrage (0600 / 0700) |
| `miyukini-maia` | `miyukini-maia` | 127.0.0.1:11435 | `/var/lib/miyukini/maia` | sans modèle GGUF ni appelant épinglé : API protégée en 503 |
| `miyukini-miyucloud` | `miyukini-miyucloud` | 127.0.0.1:8086 | `/var/lib/miyukini/miyucloud` | PostgreSQL local (rôle et base `miyucloud`), chiffrement au repos activé |
| `miyukini-cog-web-portal` | `miyukini-portail` | 127.0.0.1:8090 | `/var/lib/miyukini/cog-web-portal` | aucun service PortalContract raccordé pour l'instant |
| `postgresql` (paquet) | `postgres` | 127.0.0.1:5432 | `/var/lib/postgresql` | dépendance de MiyuCloud |

Arborescence de configuration :

- `/etc/miyukini/strongfather/*.toml` : politiques StrongFather (lues par MAIA via `COG_POLITIQUES_DIR`) ;
- `/etc/miyukini/bridge.toml` : copie du `bridge.toml` du dépôt, propriété root, `0444`. MAIA le
  lit à `$MIYUKINI_DATA_DIR/bridge.toml` (`/var/lib/miyukini/maia/bridge.toml`) : ce chemin n'est
  qu'un point de montage recouvert, dans l'espace de noms du service, par un montage lié en lecture
  seule (`BindReadOnlyPaths`). MAIA, propriétaire de son répertoire de données, ne peut donc ni
  réécrire sa configuration ni la remplacer ;
- `/etc/miyukini/{cog-web-portal,miyucloud}.env` : secrets (0600), générés une seule fois ;
- `/etc/miyukini/maia.env` : variables non secrètes de MAIA.

Aucun secret n'est commité : le jeton CSRF du portail, la clé de chiffrement MiyuCloud et le mot de passe
PostgreSQL sont tirés au hasard à la première installation et conservés aux exécutions suivantes.
Le mot de passe du rôle `miyucloud` est posé par `psql` via l'entrée standard, après
`SET log_statement = 'none'` et `SET log_min_duration_statement = -1` dans la même session : il
n'apparaît ni dans `argv` ni dans les journaux PostgreSQL.

## Non installés (et pourquoi)

- **Douane (`miyudouane`)** : pas de démon. C'est un outil de génération et de mise en place de règles
  nftables, appliqué par script sur les interfaces. Il est exercé par le banc `tests/intrusion-lot3/`
  dans des espaces de noms jetables, qui exige en outre que `/etc/miyukini/douane` soit absent.
- **Portail (`portail-sas`)** : processus de décodage lancé par lot par le Portail interne (pas encore
  de processus réseau). Le binaire est installé ; le testeur vérifie seulement son refus sans argument.
- **Stub Central (`miyukini-central-stub`)** : binaire qui attend une entrée standard puis quitte ; sans
  intérêt comme service.
- **Racine hors ligne (`miyuracine`)** : outil de cérémonie hors ligne (SLIP-39) ; il ne doit pas
  résider sur un serveur connecté. Non installé, le testeur vérifie son absence.

## Ce que `tester.sh` vérifie

- services actifs, sans redémarrage en boucle ; écoutes uniquement sur 127.0.0.1 (hors SSH et résolveur) ;
- refus fail-closed : KindMother sans poignée de main, MAIA sans signature (503) et routes mémoire
  désactivées (503), MiyuCloud sans session (401) ;
- démarrages jetables refusés : KindMother et MAIA hors boucle locale sans appelant épinglé ;
- permissions (0700 des données, 0600 des secrets, comptes `nologin`) et durcissement systemd ;
- `bridge.toml` de MAIA : `/etc/miyukini`, root, `0444`, monté en lecture seule dans le service ;
- binaires installés conformes au `MANIFESTE` du paquet ;
- banc d'intrusion lot 3 (option `--banc`).

## Points à connaître

- Le compte MiyuCloud n'est pas initialisé : le premier administrateur doit être créé via l'interface
  (`/api/auth/status` indique `initialized: false`).
- `deploy/miyucloud/miyucloud.service` (existant) décrit une autre disposition (`/opt/miyucloud/current`,
  `/etc/miyucloud/miyucloud.env`). Il n'est pas utilisé par cet installateur ; à aligner ou à retirer.
- Le banc lot 3 crée des espaces `mcog-*` et écrit `/etc/miyukini/douane` pendant son exécution : ne le
  lancer que sur la VM jetable (il refuse de tourner ailleurs par défaut).
