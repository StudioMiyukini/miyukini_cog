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
| `pousser.sh` | hôte de développement | prépare un paquet hors dépôt (binaires de `target/release`, config, statiques, banc), écrit son `MANIFESTE` (SHA-256 lié au commit), le copie dans la VM, l'y vérifie, envoie à part l'artefact MMIA du mini modèle (s'il n'y est pas déjà) et lance l'installateur |
| `installer.sh` | VM jetable, root | vérifie le paquet contre son `MANIFESTE`, puis installation idempotente : utilisateurs, `/opt/miyukini`, `/etc/miyukini`, `/var/lib/miyukini/<svc>` (0700), PostgreSQL, profils AppArmor, zone COG et frontière externe de la Douane (débit `tc`), maia-llm et mini modèle, unités systemd durcies |
| `commun.sh` | sourcé | logique commune aux deux installateurs (VM et recette), dont le script de la frontière externe (`/opt/miyukini/libexec/douane-externe.sh`, écrit depuis `commun.sh`, donc couvert par le `MANIFESTE`) |
| `tester.sh` | VM | tests de fumée, sortie TAP ; `--banc` ajoute le banc d'intrusion lot 3 |
| `scenarios.sh` | hôte (`--distant`) ou VM | scénarios LIVE (voir plus bas) |

## Utilisation

```sh
deploy/cog-vm/pousser.sh [--modele X.mmia]   # paquet, pousse, (artefact MMIA), installe
# dans la VM :
/home/cog/cog-paquet/tester.sh          # tests de fumée (183 contrôles)
/home/cog/cog-paquet/tester.sh --banc   # + banc d'intrusion lot 3 (root, ~2 min ; 185 contrôles, dont le banc)
```

Les binaires doivent être construits au préalable (`cargo build --release -p <paquet>`) ; **maia-llm
sans CUDA** (la VM n'a pas de GPU ; `pousser.sh` refuse un binaire lié à CUDA) :
`cargo build --release -p maia-llm --no-default-features`. Outils d'essai : `--example portail_essai`
(miyuportail), `--example proposer_essai` (maia-memoire), et l'essai d'intégration d'Alicia
`cargo test --release -p miyualicia --test proposition_ia --no-run` (copié en `outils/alicia_proposition_ia`).
`pousser.sh --sans-installation` ne fait que déposer et vérifier le paquet.

### Mini modèle : paquet séparé (artefact MMIA)

Le modèle (Qwen2.5-1.5B-Instruct Q4_K_M, 1 117 320 736 octets, Apache-2.0, A-026) n'est **pas** dans le
paquet : l'artefact `MMIA` du canal (≈ 1,1 Go) est livré à part, pour quatre raisons : il est
incompressible et rendrait chaque paquet de code 60 fois plus lourd ; il change au rythme du canal,
pas des commits ; il s'authentifie lui-même (manifeste signé, ancre épinglée dans `commun.sh`, donc
couverte par le `MANIFESTE` et, en recette, par la signature du paquet ; SHA-256 du modèle consigné) ;
les sauvegardes de la recette l'excluent (reproductible depuis l'artefact).

```sh
# poste de signature (hors COG, jamais dans le dépôt) :
maia-llm emballer-modele --modele qwen2.5-1.5b-instruct-q4_k_m.gguf --ancre <ancre> --sortie qwen2.5-1.5b-instruct-q4_k_m.mmia
```

L'installateur (`--modele X.mmia`) le passe à `maia-llm installer-modele` (signature contre l'ancre,
nom imposé, taille, licence, empreinte vérifiée **avant** toute écriture), puis revérifie le SHA-256
installé contre `6a1a2eb6…407e`. Déjà installé à cette version : rien n'est refait. **Signature de
développement** : ancre `fb9d57a0…611a` (graine hors dépôt `/srv/app/models/cle-dev-maia.hex`), en
attendant la re-signature par la clé du canal officiel (A-026 « Reste »).

## Intégrité du paquet (manifeste)

`pousser.sh` écrit `MANIFESTE` à la racine du paquet : un en-tête (`# commit <sha>`,
`# arbre propre|modifié`, date) puis le SHA-256 de **chaque** fichier du paquet (format
`sha256sum`). Il affiche le commit, l'état de l'arbre, le SHA-256 de chaque binaire de `bin/` et
l'empreinte du manifeste lui-même. Un arbre « modifié » signale des binaires qui ne se
reconstruisent pas depuis le commit annoncé.

Dans la VM, avant toute installation :

1. tout lien symbolique, tube ou périphérique dans le paquet est refusé, puis le paquet est copié
   sans suivre les liens dans `/run/miyukini-paquet.XXXXXX` (root, 0700) ; le dossier déposé n'est
   plus relu, seule la copie est vérifiée, sourcée et installée (effacée à la sortie) ;
2. l'empreinte du `MANIFESTE` reçu est comparée à celle calculée sur l'hôte (`pousser.sh`, puis de
   nouveau par `installer.sh --manifeste-sha256 <hex>`, avant de sourcer `commun.sh`) ;
3. la **liste exacte** triée des fichiers présents doit égaler celle du `MANIFESTE` (aucun fichier
   ajouté, retiré ou substitué, même à nombre égal), puis `sha256sum --check --strict` sur chacun ;
4. seuls les fichiers listés au `MANIFESTE` sont installés (aucun glob), chacun recontrôlé fichier
   régulier non lien juste avant `install` ;
5. après copie, les binaires de `/opt/miyukini/bin` sont revérifiés contre
   `/opt/miyukini/MANIFESTE` (conservé ; `tester.sh` le recontrôle).

## Services installés

| Unité | Utilisateur | Écoute | Données | Notes |
|---|---|---|---|---|
| `miyukini-kindmother` | `miyukini-kindmother` | 127.0.0.1:50051 | `/var/lib/miyukini/kindmother` | identité et clé de base générées au premier démarrage (0600 / 0700) |
| `miyukini-maia-llm` | `miyukini-maia-llm` | 127.0.0.1:11436 | `/var/lib/miyukini/maia-llm` (config. liée en lecture seule) ; modèles `/var/lib/miyukini/modeles` (root, lecture seule) | moteur llama.cpp de MAIA et d'Alicia, CPU (`gpu_layers = 0`), `MemoryMax=3G`, aucun périphérique ; appelants épinglés : MAIA (et Alicia si `/etc/miyukini/alicia-cle-voix.pub`) |
| `miyukini-maia` | `miyukini-maia` | 127.0.0.1:11435 | `/var/lib/miyukini/maia` | `MAIA_LLM_URL=http://127.0.0.1:11436` ; clé de service `keys/maia.seed` (0600) créée par l'installateur ; sans appelant épinglé : API protégée en 503 |
| `miyukini-douane-externe` | root (oneshot) | aucune | `/var/lib/miyukini/douane/externe` (0700) | frontière externe : espace `mcog-f-externe`, règles strictes → `tc` → filet |
| `miyukini-miyucloud` | `miyukini-miyucloud` | 127.0.0.1:8086 | `/var/lib/miyukini/miyucloud` | PostgreSQL local (rôle et base `miyucloud`), chiffrement au repos activé |
| `miyukini-cog-web-portal` | `miyukini-portail` | 127.0.0.1:8090 | `/var/lib/miyukini/cog-web-portal` | aucun service PortalContract raccordé pour l'instant |
| `miyukini-portail-entree` | `miyukini-portail-entree` | aucune (`PrivateNetwork`) | `/var/lib/miyukini/portail-entree` | Portail, sens Franche → zone COG : relève seule (A-022 §3) |
| `miyukini-portail-sortie` | `miyukini-portail-sortie` | aucune (`PrivateNetwork`) | `/var/lib/miyukini/portail-sortie` | Portail, sens zone COG → Franche : réponses à jeton et H1 seules |
| `postgresql` (paquet) | `postgres` | 127.0.0.1:5432 | `/var/lib/postgresql` | dépendance de MiyuCloud |

Arborescence de configuration :

- `/etc/miyukini/strongfather/*.toml` : politiques StrongFather (lues par MAIA via `COG_POLITIQUES_DIR`) ;
- `/etc/miyukini/bridge.toml` : copie du `bridge.toml` du dépôt, propriété root, `0444`. MAIA le
  lit à `$MIYUKINI_DATA_DIR/bridge.toml` (`/var/lib/miyukini/maia/bridge.toml`) : ce chemin n'est
  qu'un point de montage recouvert, dans l'espace de noms du service, par un montage lié en lecture
  seule (`BindReadOnlyPaths`). MAIA, propriétaire de son répertoire de données, ne peut donc ni
  réécrire sa configuration ni la remplacer ;
- `/etc/miyukini/{cog-web-portal,miyucloud}.env` : secrets (0600), générés une seule fois ;
- `/etc/miyukini/maia.env` : variables non secrètes de MAIA ; `bridge.toml` installé avec `upstream_url = ""` (aucun moteur hors COG) ;
- `/etc/miyukini/maia-llm.toml` (root, 0644) : rendu de `config/maia/maia-llm.toml` (clé publique de MAIA, ancre des
  modèles, `gpu_layers = 0`), lié en lecture seule sur `/var/lib/miyukini/maia-llm/maia-llm.toml` ;
- `/etc/miyukini/alicia.toml` : `[moteur_ia] url = "http://127.0.0.1:11436"`, `timeout_ms = 20000` (Alicia tourne
  dans Central, hors de ces installateurs : sa clé publique est à déposer dans `/etc/miyukini/alicia-cle-voix.pub`) ;
- `/etc/miyukini/douane-tc.desc` : description `miyudouane-tc 1` (capacités `ext0` 122 070 Kio/s ≈ 1 Gbit/s,
  `wg-fed` 12 207 Kio/s ≈ 100 Mbit/s ; `ext-franche` au défaut de la Douane).

Aucun secret n'est commité : le jeton CSRF du portail, la clé de chiffrement MiyuCloud et le mot de passe
PostgreSQL sont tirés au hasard à la première installation et conservés aux exécutions suivantes.
Le mot de passe du rôle `miyucloud` est posé par `psql` via l'entrée standard, après
`SET log_statement = 'none'` et `SET log_min_duration_statement = -1` dans la même session : il
n'apparaît ni dans `argv` ni dans les journaux PostgreSQL.

## Portail scindé (A-022 §3)

Deux processus **sans aucun état commun**, deux comptes, deux profils AppArmor, deux unités
**indépendantes** (aucune ne requiert ni n'attend l'autre) :

| | entrée (`portail-entree`) | sortie (`portail-sortie`) |
|---|---|---|
| compte | `miyukini-portail-entree` | `miyukini-portail-sortie` |
| lit | `portail-files/depot` (trames MSCE de la façade) | `portail-files/reponses` (réponses à jeton, publications H1 du moteur) |
| écrit | `portail-files/admises` (déclarations admises + jeton signé) | `portail-files/publication` (trames signées par la clé de la voie, `<service>.cle`) |
| état propre (0700) | `/var/lib/miyukini/portail-entree` : marques anti-rejeu, journal | `/var/lib/miyukini/portail-sortie` : clé de publication, jetons consommés, journal |
| configuration (Cores) | `/etc/miyukini/portail-entree/voies` (2750, graine des jetons en 0600) | `/etc/miyukini/portail-sortie/voies` (2750, ancre des jetons) |
| aveugle à | `reponses`, `publication`, état et configuration de la sortie | `depot`, `admises`, état et configuration de l'entrée |

Files à sens unique (`/var/lib/miyukini/portail-files/`) : répertoire du **lecteur**, mode `3730`
(collant, setgid, groupe `-wx`, rien pour les autres) ; groupe **propre à la file**, qui ne réunit
que son écrivain et son lecteur :

| file | lecteur (propriétaire) | écrivain | groupe |
|---|---|---|---|
| `depot` | `miyukini-portail-entree` | `miyukini-facade` | `miyukini-q-depot` |
| `admises` | `miyukini-moteur` | `miyukini-portail-entree` | `miyukini-q-admises` |
| `reponses` | `miyukini-portail-sortie` | `miyukini-moteur` | `miyukini-q-reponses` |
| `publication` | `miyukini-facade` | `miyukini-portail-sortie` | `miyukini-q-publication` |

`miyukini-facade` et `miyukini-moteur` identifient les extrémités des files (aucun service installé
pour eux dans la VM ; `scenarios.sh` les emploie via `outils/portail_essai`). Éléments créés en
`0640` (`UMask=0027`) ; chaque processus refuse de démarrer si un de ses répertoires est ouvert aux
« autres », n'appartient pas au compte attendu, ou s'il **peut** écrire dans une file de l'autre sens.

## Zone COG (Douane) et profils AppArmor

### Zone COG appliquée par la Douane

- `installer.sh` génère des arrêtés d'essai avec `arretes_essai` (`banc/cible/examples`, clé jetable), les
  vérifie avec `miyudouane regles` (signature, constitution, vigueur) pour `externe`, `portail-interne` et
  `portail-local`, puis compile la zone COG avec `miyudouane zone --zone cog`. Le plan des essais reprend les
  sandboxes du banc (`--sandbox cog:voie-a …`) : le Douane refuse un arrêté dont une voie n'a pas de sandbox.
- Règles appliquées sur l'hôte de la VM : table **`inet miyudouane_hote`**, **générée** par la Douane
  (`commun.sh`, fonction `douane_cog`) : l'installateur écrit la description `/etc/miyukini/douane-hote.desc`
  (format `miyudouane-hote 2`, en-tête de `crates/miyudouane/src/hote.rs`), la fait auditer
  (`miyudouane hote --description …`), puis l'applique (`--appliquer --fichier /etc/miyukini/douane-cog.nft`) ;
  le fichier rendu est rechargé par `miyukini-douane-cog.service`, dont dépendent les quatre services COG :
  - chaîne `sortie` : les comptes COG (`miyukini-kindmother`, `-maia`, `-miyucloud`, `-portail`) n'envoient rien
    hors boucle locale, et en boucle locale seulement leurs flux déclarés (`boucle`) :
    `miyukini-maia → 127.0.0.1:50051`, `miyukini-maia → 127.0.0.1:11436` (maia-llm), `miyukini-miyucloud → 127.0.0.1:5432`
    (et `miyukini-alicia → 127.0.0.1:11436` si ce compte existe) ; `miyukini-maia-llm` n'a aucun flux ; le reste (résolveur 127.0.0.53,
    autre port, autre compte) est journalisé `MDOU|zone-cog|sortie|SORTIE-COG|` (NFLOG 110) et rejeté (compteur `rejets`) ;
  - chaîne `entree` : entrée non locale refusée sur les ports COG (50051, 11435, 11436, 8086, 8090, 5432) ;
  - SSH d'administration et les autres comptes non touchés ; aucun `flush ruleset` (autres tables intactes).
- Rattachement : la zone suit `nftables.service` (`PartOf=`), les services la suivent (`BindsTo=`) ; minuteur
  `miyukini-douane-cog-controle.timer` (chaque minute) : table absente ⇒ rechargée, sinon services COG arrêtés.
- Contrôle positif dans `tester.sh` : un compte non COG atteint Internet, donc l'échec des comptes COG a un sens.
- **Limite** : les arrêtés sont des arrêtés d'essai valides un jour (`pas_apres`) : leur vérification a lieu à
  l'installation seulement.

### Frontière externe et débit `tc` (A-023, A-025)

- Unité `miyukini-douane-externe` (oneshot, au démarrage et à chaque installation, donc à chaque nouvel arrêté
  externe) : `douane-externe.sh demarrer` crée l'espace `mcog-f-externe` (et le pont `mcog-z-franche`) depuis le
  rendu `miyudouane mise-en-place`, filtré à ces deux espaces (aucune sandbox, aucune règle de `/etc/miyukini/douane`) ;
  `ext0` et `wg-fed`, sans carte ni WireGuard, sont des interfaces **factices** (dummy) qui portent la même mise en forme.
- Ordre sûr, chaque étape **seulement si la précédente a réussi** : (a) `miyudouane regles --frontiere externe
  --appliquer` (règle `DEBIT` stricte) ; (b) `miyudouane tc --description /etc/miyukini/douane-tc.desc --appliquer`
  (racine `htb 4d44:`, classe par voie, feuille `fq_codel` par flux, filtre `fw`, audité) ; (c) `regles
  --mise-en-forme-appliquee --appliquer` (filet 5/4 par sens). Échec de (a) : frontière fermée (`miyudouane fermeture`),
  code 1 ; de (b) : règle stricte maintenue, aucun filet, code 3 ; de (c) : code 4. État dans
  `/var/lib/miyukini/douane/externe/etat` (`strict`, `tc`, `filet`, `ferme`) ; journal `miyukini-douane-externe`.
- Exploitation : `douane-externe.sh couper|retablir <voie>` (EX-49), `arrete <fichier>` (arrêté vérifié avant
  d'être retenu ; refusé ⇒ l'ancien reste), `appliquer` (= `systemctl reload`), `etat`.
- Modules `sch_htb`, `sch_fq_codel`, `cls_fw` chargés au démarrage (`/etc/modules-load.d/miyukini-douane.conf`).
- Fichiers dans `/var/lib/miyukini/douane/externe` (0600), jamais `/etc/miyukini/douane` (réservé au banc). Le banc
  exige qu'aucun espace `mcog-*` n'existe : `tester.sh --banc` arrête l'unité pendant le banc puis la redémarre.
- **Limite** : arrêtés d'essai valides un jour (comme la zone COG) ; au-delà, (a) échoue et la frontière est fermée
  au démarrage suivant (fail-closed).

### Profils AppArmor (`deploy/apparmor/`, mode enforce)

| Profil | Binaire | Accès retenus (observés en journal `DENIED` pendant `scenarios.sh`) |
|---|---|---|
| `miyukini-kindmother` | `/opt/miyukini/bin/kindmother-server` | `/var/lib/miyukini/kindmother/` rw (chmod des droits 0700 par le service), TCP, `/etc/machine-id` r, `cpu.max` et `/proc/PID/cgroup` r |
| `miyukini-maia` | `/opt/miyukini/bin/maia` | `/var/lib/miyukini/maia/**` rwk (`bridge.toml` en lecture seule), `/etc/miyukini/strongfather/**` r, TCP, sondes `/proc/uptime`, `/proc/`, `/sys/class/drm/**` r (et `vendor` / `mem_info_vram_total` du périphérique PCI), `/proc/driver/nvidia/gpus/*/information` r ; `/proc/N/stat` et `/proc/N/task/` refusés en silence (`deny` : parcours des processus par `sysinfo`, inutile) ; aucun exec |
| `miyukini-miyucloud` | `/opt/miyukini/bin/miyucloud-server` | `/var/lib/miyukini/miyucloud/**` rwk, `/opt/miyukini/share/miyucloud/**` r, `/tmp` (fichiers temporaires), TCP (PostgreSQL et écoute), NSS `passwd`/`group`/`nsswitch.conf` r |
| `miyukini-cog-web-portal` | `/opt/miyukini/bin/cog-web-portal` | `/var/lib/miyukini/cog-web-portal/**` rwk, TCP, `cpu.max` et `/proc/PID/cgroup` r |
| `miyukini-portail-entree` | `/opt/miyukini/bin/portail-entree` | descripteurs de l'entrée r, état propre rw, `depot/` r + retrait, `admises/*` w ; lance `portail-sas` **sous ce profil** (`ix` : `NoNewPrivileges` interdit toute transition, même en pile — refus « no new privs » observé) ; refus **audités** sur `reponses/`, `publication/`, état et configuration de la sortie ; aucun réseau |
| `miyukini-portail-sortie` | `/opt/miyukini/bin/portail-sortie` | descripteurs de la sortie r, état propre rw, `reponses/` r + retrait, `publication/*` w ; refus **audités** sur `depot/`, `admises/`, état et configuration de l'entrée ; aucun réseau, aucun exec |
| `miyukini-maia-llm` | `/opt/miyukini/bin/maia-llm` | source `config/maia/apparmor-miyukini-maia-llm` : configuration et `/var/lib/miyukini/modeles/*.gguf` en lecture seule (écriture refusée), TCP, sondes CPU et mémoire ; règles GPU inertes pour le binaire CPU ; `nvidia-smi` interdit ; aucun exec |
| `miyukini-portail-sas` | `/opt/miyukini/bin/portail-sas` | aucun réseau (`deny network`), aucun fichier hors bibliothèques ; `/proc/PID/fd/` r (fermeture des descripteurs). Second niveau derrière Landlock et seccomp du sas |

- Chaque profil refuse explicitement l'écriture dans les répertoires des autres services et dans `/etc/miyukini`, `/opt/miyukini`.
- `installer.sh` copie `deploy/apparmor/miyukini-*` (paquet, manifeste compris) dans `/etc/apparmor.d/` et charge chaque
  profil avec `apparmor_parser -r -W` (échec = arrêt). Les unités portent `AppArmorProfile=`.
- L'attente de KindMother par MAIA est une unité distincte (`miyukini-attente-kindmother`, sans profil) : systemd
  applique `AppArmorProfile=` à chaque commande de l'unité, et MAIA ne doit pas pouvoir exécuter `bash`.
- `tester.sh` vérifie que les huit profils sont chargés en enforce (`/sys/kernel/security/apparmor/profiles`) et que
  le processus principal de chaque service est confiné. `scenarios.sh` échoue à la section 12 si le journal noyau
  contient un `DENIED` d'un profil `miyukini-*` depuis le début du passage.
- Les refus hors boucle locale (`tester.sh`) s'exécutent sur une copie du binaire placée hors de son chemin de profil :
  le test porte sur le code de refus, non sur le confinement.

## Non installés (et pourquoi)

- **Douane (`miyudouane`)** : pas de démon ; outil appliqué par l'installateur (zone COG sur l'hôte) et par
  l'unité de la frontière externe. Les autres frontières (portails) et les zones restent exercées par le banc
  `tests/intrusion-lot3/` dans des espaces de noms jetables (il exige que `/etc/miyukini/douane` soit absent).
- **Alicia** : pas de service ici (elle tourne dans Central) ; `scenarios.sh` exerce son code réel par l'essai
  d'intégration `proposition_ia` contre le maia-llm installé.
- **Sas (`portail-sas`)** : pas de service ; lancé par lot par `portail-entree` (confiné par
  le profil de l’entrée, `NoNewPrivileges` interdisant la transition ; son propre profil ne s’applique qu’à un lancement hors entrée). Le testeur vérifie aussi son refus sans argument.
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
  injoignable), contrôle positif (un compte non COG sort), services joignables en boucle locale depuis leur compte,
  `maia → maia-llm` admis, `maia-llm` sans aucun flux ;
- frontière externe : état `filet`, racine `htb 4d44:` sur `ext-franche`, `ext0`, `wg-fed`, classe de voie et feuille
  de flux (512 Kio/s), filtre `fw` ; texte appliqué **égal** au rendu audité de `miyudouane tc` (arrêté signé, description)
  et état du noyau conforme (nombre de classes, files, filtres) ; filet 5/4 par sens et plus aucune règle stricte ; ordre
  (a) → (b) → (c) lu dans le journal du dernier démarrage ; modules `tc` chargés ;
- maia-llm : modèle chargé (Apache-2.0, `gpu_layers 0`), fichier du canal conforme (root 0644, SHA-256 A-026), refus sans
  signature (401), réponse **réelle** du modèle à une requête signée par la clé de MAIA, marquée IA ; configuration liée en
  lecture seule ; `MAIA_LLM_URL` = 11436 ; mention IA de MAIA (`/maia/presentation`) ; `[moteur_ia]` d'Alicia ;
- profils AppArmor en enforce (huit) et processus principaux confinés ;
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
- Portail scindé (section 11 bis, outil `outils/portail_essai` du paquet, clés jetables) : descripteurs des Cores
  (0600 / 0640), chemin façade → entrée → moteur → sortie → façade (trame vérifiée avec la clé publiée), jeton
  rejoué refusé, trame rejouée refusée, réponse glissée dans le dépôt refusée, panne (arrêt, `kill -9`) de chaque
  processus sans arrêt de l'autre, reprise des marques et des jetons après relance, refus AppArmor de l'écriture
  croisée (sonde exécutée par le **propriétaire** de la file visée, témoin non confiné qui y écrit), coupure et
  rotation de clé, suppression sans résidu ;
- IA locale (section 13) : MAIA (appelant d'essai épinglé par un drop-in retiré en fin de section) répond par
  maia-llm (`backend native`, sans repli, `genere_par_ia`, `mention_ia`) ; Alicia (clé d'essai épinglée puis retirée) :
  essai `proposition_ia` avec le vrai modèle (18 phrases françaises, taux ≥ 75 %), « déverrouille la porte » proposé
  comme commande de serrure, et commande de serrure proposée par l'IA refusée par les Cores sous la politique
  **installée** (confirmation exigée), présentée au chemin compilé dans l'essai par un espace de montage privé ;
- frontière externe (section 14) : coupure et rétablissement de voie, échec de `tc` provoqué (racine étrangère sur
  `wg-fed` : code 3, règle stricte, **aucun** filet, racine étrangère intacte), reprise, arrêté altéré refusé,
  redémarrage de l'espace (ordre a → b → c) ;
- contrôle final (section 12) : aucun refus AppArmor `DENIED` d'un profil `miyukini-*` pendant le passage (hors
  sondes voulues `.sonde-cloisonnement-*` de la section 11 bis) ;
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
- **Débit (`tc`)** : produit et appliqué par la Douane sur la frontière externe (voir « Frontière externe et débit
  `tc` ») ; les interfaces `ext0` et `wg-fed` de la VM sont factices (aucun trafic réel n'y est mis en forme).
- **Opérations de root dans les dossiers des services** : un compte de service maître de son dossier pourrait y poser
  un lien. L'installateur n'y écrit, ne lit ni ne change de droits à travers un lien : création en tant que le compte
  (`setpriv`, graine de MAIA, `miyucloud/storage`), fichiers root posés par renommage depuis un dossier root
  (`bridge.toml`, `maia-llm.toml`), `chown -h` et refus des liens ailleurs (aussi dans `restaurer.sh`).
- Un profil AppArmor peut laisser un profil `null-` en mode complain après une session de mise au point ;
  il disparaît au redémarrage de la VM (`tester.sh` le signale).
