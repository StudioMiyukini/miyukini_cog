# Déploiement MiyuCloud (`deploy/miyucloud/`)

La disposition et le durcissement de référence sont ceux de
[`deploy/cog-vm/installer.sh`](../cog-vm/installer.sh), validés en VM. **L'installateur reste la
source de vérité** : il génère l'unité `miyukini-miyucloud.service`. Les fichiers d'ici en sont la
copie lisible, pour un hôte installé à la main ; s'ils divergent, l'installateur l'emporte.

## Fichiers

| Fichier | Rôle |
|---|---|
| `miyucloud.service` | unité systemd, à installer sous `/etc/systemd/system/miyukini-miyucloud.service` ; identique à celle générée par l'installateur |
| `Caddyfile` | mandataire TLS (ACME) devant `127.0.0.1:8086` ; domaine et courriel = exemples |

## Disposition

| Chemin | Propriétaire, mode | Contenu |
|---|---|---|
| `/opt/miyukini/bin/miyucloud-server` | `root:root` 0755 | binaire |
| `/opt/miyukini/share/miyucloud/{static,static-dist}` | `root:root`, lecture seule | statiques (répertoire de travail) |
| `/var/lib/miyukini/miyucloud` | `miyukini-miyucloud` 0700 | données ; seul chemin en écriture (`ReadWritePaths`) |
| `/var/lib/miyukini/miyucloud/storage` | `miyukini-miyucloud` 0700 | blobs, vignettes, envois en cours, fichiers de travail du reclassement CDR |
| `/etc/miyukini/miyucloud.env` | `root:root` 0600 | secrets (voir ci-dessous) |

Utilisateur système `miyukini-miyucloud` : sans shell (`/usr/sbin/nologin`), sans home. Écoute sur
`127.0.0.1:8086` uniquement ; PostgreSQL local (rôle et base `miyucloud`). L'ancienne disposition
(`/opt/miyucloud/current`, `/etc/miyucloud/miyucloud.env`, utilisateur `miyucloud`, port 11442) est
abandonnée.

## Secrets (`/etc/miyukini/miyucloud.env`, jamais commité)

Générés une fois par l'installateur (`openssl rand`) et conservés ensuite :
`MIYUCLOUD_PG_PASSWORD`, `MIYUCLOUD_DB_CONNECTION_STRING`, `MIYUCLOUD_ENCRYPTION_KEY`.

## Variables d'identité Miyukini (facultatives, non secrètes)

À ajouter au même fichier d'environnement. Elles vont **par groupe** : un groupe incomplet ou invalide
fait **refuser le démarrage**.

| Groupe | Variables | Effet |
|---|---|---|
| Ancre du COG d'origine | `MIYUCLOUD_MIYU_COG_ID`, `MIYUCLOUD_MIYU_CLE_EMISSION` | active le schéma `MiyuVisa` (visa + preuve de possession) |
| Révocations (lot 2 §8.2, A-007) | `MIYUCLOUD_MIYU_REGISTRE` (64 hex), `MIYUCLOUD_MIYU_CLE_JOURNAL` (clé de journal du COG), `MIYUCLOUD_MIYU_REVOCATIONS_DEPOT` (répertoire) | révocations lues dans le registre vérifié du COG d'origine |

Comportement fail-closed :

- ancre sans groupe « révocations » : le serveur démarre, mais **tout visa est refusé** (liste
  absente) ; les autres modes d'accès (session, mot de passe d'application, clé d'appareil) restent ;
- groupe « révocations » sans ancre : refus de démarrer ;
- liste plus vieille que **15 min** (horodatage de la tête vérifiée) ou jamais relevée : visa refusé.

Le dépôt de révocations contient `tete.sceau` (dernière tête signée du registre) et `entrees.mrgj`
(entrées au format `MRGJ`). Il est déposé par la zone COG, relu toutes les 60 s et **jamais modifié**
par MiyuCloud (lecture seule suffit, par exemple `/var/lib/miyukini/revocations-cog`, `root` 0755,
fichiers 0644). Tout son contenu est revérifié (signature de tête, racine Merkle) : il n'a pas besoin
d'être de confiance.

## Reclassement CDR des fichiers hérités

Au démarrage puis toutes les 15 min, MiyuCloud repasse par le CDR les fichiers `inconnu` (antérieurs
au désarmement) : désarmable ⇒ `desarme`, sinon `quarantaine`. Un fichier `inconnu` n'est **jamais
servi**. `MIYUCLOUD_CDR_RECLASSEMENT=0` désactive la tâche : les fichiers `inconnu` restent alors non
servis (jamais l'inverse). La migration `20261025000002_cdr_reclassement.sql` (index partiel,
idempotente) est appliquée au démarrage avec les autres migrations.

## Mandataire

Derrière Caddy, régler `MIYUCLOUD_BASE_URL` sur l'URL publique (`https://…`) et garder
`MIYUCLOUD_SERVER_HOST=127.0.0.1`.
