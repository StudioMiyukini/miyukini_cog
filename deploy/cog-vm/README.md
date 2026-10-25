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
| `installer.sh` | VM jetable, root | vérifie le paquet contre son `MANIFESTE`, puis installation idempotente : utilisateurs, `/opt/miyukini`, `/etc/miyukini`, `/var/lib/miyukini/<svc>` (0700), PostgreSQL, profils AppArmor, zone COG de la Douane, unités systemd durcies |
| `tester.sh` | VM | tests de fumée, sortie TAP ; `--banc` ajoute le banc d'intrusion lot 3 |

## Utilisation

```sh
deploy/cog-vm/pousser.sh          # construit le paquet, pousse, installe
# dans la VM :
/home/cog/cog-paquet/tester.sh          # tests de fumée (85 contrôles)
/home/cog/cog-paquet/tester.sh --banc   # + banc d'intrusion lot 3 (root, ~2 min ; 100 contrôles)
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

## Zone COG (Douane) et profils AppArmor

### Zone COG appliquée par la Douane

- `installer.sh` génère des arrêtés d'essai avec `arretes_essai` (`banc/cible/examples`, clé jetable), les
  vérifie avec `miyudouane regles` (signature, constitution, vigueur) pour `externe`, `portail-interne` et
  `portail-local`, puis compile la zone COG avec `miyudouane zone --zone cog`. Le plan des essais reprend les
  sandboxes du banc (`--sandbox cog:voie-a …`) : le Douane refuse un arrêté dont une voie n'a pas de sandbox.
- Règles appliquées sur l'hôte de la VM : table **`inet miyudouane_hote`** (fichier `/etc/miyukini/douane-cog.nft`,
  chargé par `miyukini-douane-cog.service`, dont dépendent les quatre services COG) :
  - `sortie_cog` : les comptes COG (`miyukini-kindmother`, `-maia`, `-miyucloud`, `-portail`) n'envoient rien hors
    boucle locale (`oifname != "lo"` refusé) : aucune route sortante ;
  - `entree_cog` : entrée non locale refusée sur les ports COG (50051, 11435, 8086, 8090, 5432) ;
  - SSH d'administration et les autres comptes non touchés ; aucun `flush ruleset` (autres tables intactes).
- Contrôle positif dans `tester.sh` : un compte non COG atteint Internet, donc l'échec des comptes COG a un sens.
- **Limite** : `miyudouane` ne produit pas de règles par UID (`meta skuid`) sur l'hôte ; `miyudouane zone` rend des
  règles pour espaces de noms (services en sandboxes). La table hôte est donc écrite par `installer.sh`, à partir
  de la politique « aucune sortie COG » que les arrêtés d'essai ne contredisent pas (flux COG déclarés vers Franche
  seulement, non routés dans la VM). Les arrêtés sont valides un jour (`pas_apres`) : leur vérification a lieu à
  l'installation seulement.

### Profils AppArmor (`deploy/apparmor/`, mode enforce)

| Profil | Binaire | Accès retenus (observés en journal `DENIED` pendant `scenarios.sh`) |
|---|---|---|
| `miyukini-kindmother` | `/opt/miyukini/bin/kindmother-server` | `/var/lib/miyukini/kindmother/` rw (chmod des droits 0700 par le service), TCP, `/etc/machine-id` r, `cpu.max` et `/proc/PID/cgroup` r |
| `miyukini-maia` | `/opt/miyukini/bin/maia` | `/var/lib/miyukini/maia/**` rwk (`bridge.toml` en lecture seule), `/etc/miyukini/strongfather/**` r, TCP, sondes `/proc/uptime`, `/proc/`, `/sys/class/drm/**` r ; aucun exec |
| `miyukini-miyucloud` | `/opt/miyukini/bin/miyucloud-server` | `/var/lib/miyukini/miyucloud/**` rwk, `/opt/miyukini/share/miyucloud/**` r, `/tmp` (fichiers temporaires), TCP (PostgreSQL et écoute), NSS `passwd`/`group`/`nsswitch.conf` r |
| `miyukini-cog-web-portal` | `/opt/miyukini/bin/cog-web-portal` | `/var/lib/miyukini/cog-web-portal/**` rwk, TCP, `cpu.max` et `/proc/PID/cgroup` r |
| `miyukini-portail-sas` | `/opt/miyukini/bin/portail-sas` | aucun réseau (`deny network`), aucun fichier hors bibliothèques ; `/proc/PID/fd/` r (fermeture des descripteurs). Second niveau derrière Landlock et seccomp du sas |

- Chaque profil refuse explicitement l'écriture dans les répertoires des autres services et dans `/etc/miyukini`, `/opt/miyukini`.
- `installer.sh` copie `deploy/apparmor/miyukini-*` (paquet, manifeste compris) dans `/etc/apparmor.d/` et charge chaque
  profil avec `apparmor_parser -r -W` (échec = arrêt). Les unités portent `AppArmorProfile=`.
- L'attente de KindMother par MAIA est une unité distincte (`miyukini-attente-kindmother`, sans profil) : systemd
  applique `AppArmorProfile=` à chaque commande de l'unité, et MAIA ne doit pas pouvoir exécuter `bash`.
- `tester.sh` vérifie que les cinq profils sont chargés en enforce (`/sys/kernel/security/apparmor/profiles`) et que
  le processus principal de chaque service est confiné. `scenarios.sh` échoue à la section 12 si le journal noyau
  contient un `DENIED` d'un profil `miyukini-*` depuis le début du passage.
- Les refus hors boucle locale (`tester.sh`) s'exécutent sur une copie du binaire placée hors de son chemin de profil :
  le test porte sur le code de refus, non sur le confinement.

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
- zone COG de la Douane : unité activée, table `miyudouane_hote` chargée, sortie refusée aux comptes COG (1.1.1.1
  injoignable), contrôle positif (un compte non COG sort), services joignables en boucle locale depuis leur compte ;
- profils AppArmor en enforce (cinq) et processus principaux confinés ;
- banc d'intrusion lot 3 (option `--banc`).

## Scénarios LIVE (`scenarios.sh`)

```sh
cargo build --release -p maia-memoire --example proposer_essai
COG_PROPOSER=target/release/examples/proposer_essai deploy/cog-vm/scenarios.sh --distant
```

`--distant` copie le script et les migrations (`apps/miyucloud/migrations`) dans la VM et l'y exécute ;
sortie TAP, code de retour = nombre de « not ok ». Le script :

- rejoue les migrations MiyuCloud sur une base vierge, en transaction (`ON_ERROR_STOP`), et vérifie celles
  de l'instance (dont `20261025000002_cdr_reclassement`) ;
- crée le premier administrateur au premier passage (identifiants dans `~/.cog-scenarios/admin.env`, 0600, dans
  la VM) puis teste EXIF / désarmement / chiffrement sur disque (`MIYUCENC`), plages, quarantaine, polyglotte,
  copie + suppression de l'original (`ref_count`), corbeille et ramasse-miettes, réconciliation après corruption,
  WebDAV, WOPI sans jeton, cohérence SQL de `storage.blobs` / `storage.files` ;
- contrôle final (section 12) : aucun refus AppArmor `DENIED` d'un profil `miyukini-*` pendant le passage ;
- configure les routes mémoire de MAIA **dans la VM seulement** : déclaration de `alice` dans
  `maia-utilisateurs` (KindMother provisionne `maia-alice`), drop-in systemd `memoire-essai.conf` avec deux clés
  épinglées distinctes (utilisateur ≠ approbation), redémarrage de KindMother puis MAIA ; puis teste les requêtes
  signées (200, 401, rejeu, autre utilisateur, cookie), les approbations TAMR (clé mémoire refusée, mauvaise
  empreinte, approbation par la clé d'approbation, rejeu) et le refus de KindMother pour un client non autorisé.

Limites connues de l'essai :

- **`proposer_essai`** (`crates/maia-memoire/examples/proposer_essai.rs`) : appelle `MemoireMaia::proposer` (le chemin
  du pipeline LLM, inaccessible sans modèle) ; sans `COG_PROPOSER`, le scénario 10 bis est « not ok ».
- **WOPI** : désactivé dans l'unité MiyuCloud (`MIYUCLOUD_WOPI_ENABLED=false`) ; le refus sans jeton est donc
  observé sur une réponse 404, non sur l'authentification WOPI.
- Les routes mémoire restent configurées dans la VM après l'essai ; `tester.sh` attend alors 401 (et non 503).

Ordre de démarrage : MAIA ouvre la mémoire une seule fois, au démarrage. L'unité MAIA démarre donc après
`miyukini-kindmother.service` et attend (30 s au plus, sans échec fatal) que `127.0.0.1:50051` écoute. Sans cet
ordre, un redémarrage de la VM laissait la mémoire désactivée (`Connection refused`) jusqu'au redémarrage manuel
de MAIA ; `tester.sh` contrôle désormais cet ordre.

## Points à connaître

- Le compte MiyuCloud n'est pas initialisé : le premier administrateur doit être créé via l'interface
  (`/api/auth/status` indique `initialized: false`).
- `deploy/miyucloud/miyucloud.service` (existant) décrit une autre disposition (`/opt/miyucloud/current`,
  `/etc/miyucloud/miyucloud.env`). Il n'est pas utilisé par cet installateur ; à aligner ou à retirer.
- Le banc lot 3 crée des espaces `mcog-*` et écrit `/etc/miyukini/douane` pendant son exécution : ne le
  lancer que sur la VM jetable (il refuse de tourner ailleurs par défaut).
- **Défaut de MAIA (à corriger côté code)** : l'entrepôt mémoire ne se reconnecte pas à KindMother après une
  coupure (« Broken pipe », 503 jusqu'au redémarrage de MAIA) ; KindMother ferme les connexions inactives. Le
  scénario redémarre MAIA avant la section 10 ; le défaut lui-même reste ouvert.
- **Débit (`tc`)** : non produit. `miyudouane` ne fait que du rejet nftables (`limit rate over N kbytes/second`
  par marque de connexion, `debit_max_kio_s`) et un contrôle de débit dans l'IPS. Aucune mise en forme (files,
  `tc qdisc`) n'est générée ; le format n'est pas défini dans le dépôt et n'a pas été inventé.
- Un profil AppArmor peut laisser un profil `null-` en mode complain après une session de mise au point ;
  il disparaît au redémarrage de la VM (`tester.sh` le signale).
