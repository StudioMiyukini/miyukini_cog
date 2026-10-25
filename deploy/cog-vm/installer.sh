#!/usr/bin/env bash
# Installateur des services COG dans la VM jetable de test (idempotent).
#
# A exécuter DANS la VM, en root (sudo), depuis le paquet déposé par pousser.sh :
#   sudo ./installer.sh --bundle <rép. paquet> [--avec-banc]
#
# Ce script n'est PAS destiné à un hôte de production (voir README.md).
# Il ne touche ni à /etc/miyukini/douane (banc d'intrusion), ni au réseau (pas de
# ufw/nftables/règles), ni à docker.
#
# Effets :
#   - utilisateurs système dédiés (un par service), pas de shell, pas de home ;
#   - /opt/miyukini/bin (binaires), /opt/miyukini/share (statiques MiyuCloud) ;
#   - /etc/miyukini (politiques StrongFather, bridge.toml, fichiers d'environnement) ;
#   - /var/lib/miyukini/<service> en 0700, propriété du service ;
#   - secrets générés ici (0600, jamais commités) : jeton CSRF du portail, clé de
#     chiffrement et mot de passe PostgreSQL de MiyuCloud. KindMother génère sa propre
#     clé d'identité et sa clé de base au premier démarrage, dans son répertoire 0700 ;
#   - PostgreSQL local (rôle + base `miyucloud`) pour MiyuCloud ;
#   - unités systemd durcies, écoute sur 127.0.0.1 uniquement.
#
# Ré-exécuter ce script met à jour binaires, unités et configuration non secrète,
# et conserve les secrets existants.

set -euo pipefail
umask 0022

PREFIXE=/opt/miyukini
BIN=$PREFIXE/bin
PARTAGE=$PREFIXE/share
ETC=/etc/miyukini
VAR=/var/lib/miyukini

BUNDLE=""
AVEC_BANC=0

log() { printf '[installer] %s\n' "$*"; }
die() { printf '[installer] ERREUR: %s\n' "$*" >&2; exit 1; }

usage() {
	cat <<'FIN'
Usage : sudo installer.sh --bundle <répertoire> [--avec-banc]

  --bundle <rép>   paquet déposé par pousser.sh (bin/, config/, static/, banc/)
  --avec-banc      installe aussi les outils du banc d'intrusion lot 3
FIN
}

while (($#)); do
	case $1 in
		--bundle) BUNDLE=${2:?--bundle : valeur attendue}; shift ;;
		--avec-banc) AVEC_BANC=1 ;;
		-h | --help) usage; exit 0 ;;
		*) usage >&2; die "option inconnue : $1" ;;
	esac
	shift
done

((EUID == 0)) || die "root requis (sudo)"
[[ -n $BUNDLE ]] || BUNDLE=$(cd "$(dirname "$0")" && pwd)
BUNDLE=$(cd "$BUNDLE" && pwd)

BINAIRES=(kindmother-server maia miyucloud-server cog-web-portal miyudouane portail-sas)
for b in "${BINAIRES[@]}"; do
	[[ -x $BUNDLE/bin/$b ]] || die "binaire absent du paquet : bin/$b"
done
[[ -f $BUNDLE/config/bridge.toml ]] || die "config/bridge.toml absent du paquet"
[[ -d $BUNDLE/config/strongfather ]] || die "config/strongfather absent du paquet"
[[ -d $BUNDLE/static/static-dist ]] || die "static/static-dist absent du paquet"

# ------------------------------------------------------------------ paquets

paquets() {
	log "paquets apt"
	export DEBIAN_FRONTEND=noninteractive
	apt-get update -qq
	apt-get install -y -qq --no-install-recommends postgresql openssl ca-certificates python3
	if ((AVEC_BANC)); then
		apt-get install -y -qq --no-install-recommends \
			iproute2 nftables kmod procps socat netcat-openbsd tcpdump iputils-ping nmap
	fi
}

# ------------------------------------------------------------------ utilisateurs

# Un utilisateur système par service ; aucun shell, aucun home.
UTILISATEURS=(miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-portail)

utilisateurs() {
	local u
	for u in "${UTILISATEURS[@]}"; do
		if ! getent passwd "$u" >/dev/null; then
			log "création de l'utilisateur $u"
			useradd --system --no-create-home --home-dir /nonexistent --shell /usr/sbin/nologin "$u"
		fi
	done
}

# ------------------------------------------------------------------ répertoires et binaires

repertoires() {
	install -d -o root -g root -m 0755 "$PREFIXE" "$BIN" "$PARTAGE" "$PARTAGE/miyucloud"
	install -d -o root -g root -m 0755 "$ETC" "$ETC/strongfather"
	install -d -o root -g root -m 0755 "$VAR"

	local svc
	for svc in kindmother maia miyucloud cog-web-portal; do
		install -d -o root -g root -m 0700 "$VAR/$svc"
	done
	chown miyukini-kindmother "$VAR/kindmother"
	chown miyukini-maia "$VAR/maia"
	chown miyukini-miyucloud "$VAR/miyucloud"
	chown miyukini-portail "$VAR/cog-web-portal"
	install -d -o miyukini-miyucloud -g miyukini-miyucloud -m 0700 "$VAR/miyucloud/storage"
}

binaires() {
	local b
	for b in "${BINAIRES[@]}"; do
		install -o root -g root -m 0755 "$BUNDLE/bin/$b" "$BIN/$b"
	done
}

politiques_et_statiques() {
	# Politiques StrongFather (lues par MAIA via COG_POLITIQUES_DIR) : lecture seule.
	install -o root -g root -m 0644 "$BUNDLE"/config/strongfather/*.toml "$ETC/strongfather/"
	# Statiques MiyuCloud (version de production et version source).
	rm -rf "$PARTAGE/miyucloud/static" "$PARTAGE/miyucloud/static-dist"
	cp -r "$BUNDLE/static/static" "$PARTAGE/miyucloud/static"
	cp -r "$BUNDLE/static/static-dist" "$PARTAGE/miyucloud/static-dist"
	chown -R root:root "$PARTAGE/miyucloud"
	find "$PARTAGE/miyucloud" -type d -exec chmod 0755 {} +
	find "$PARTAGE/miyucloud" -type f -exec chmod 0644 {} +
	# bridge.toml : source de vérité = dépôt. Lecture seule pour MAIA.
	install -o root -g root -m 0644 "$BUNDLE/config/bridge.toml" "$VAR/maia/bridge.toml"
}

# ------------------------------------------------------------------ secrets et environnements

# valeur_env <fichier> <clé> : valeur existante d'une clé (vide si absente).
valeur_env() {
	[[ -f $1 ]] || return 0
	sed -n "s/^$2=//p" "$1" | tail -n1
}

# ecrire_fichier <chemin> <mode> : écrit stdin de façon atomique, propriété root.
ecrire_fichier() {
	local chemin=$1 mode=$2 tmp
	tmp=$(mktemp "$chemin.XXXXXX")
	cat >"$tmp"
	chown root:root "$tmp"
	chmod "$mode" "$tmp"
	mv -f "$tmp" "$chemin"
}

environnements() {
	local jeton pg_pw cle_chiffrement

	# Portail : jeton CSRF propre à l'installation (le code exige >= 32 caractères).
	jeton=$(valeur_env "$ETC/cog-web-portal.env" PORTAL_CSRF_SECRET)
	[[ -n $jeton ]] || jeton=$(openssl rand -hex 32)
	ecrire_fichier "$ETC/cog-web-portal.env" 0600 <<FIN
PORTAL_CSRF_SECRET=$jeton
FIN

	# MiyuCloud : mot de passe PostgreSQL et clé de chiffrement au repos (AES-256-GCM).
	pg_pw=$(valeur_env "$ETC/miyucloud.env" MIYUCLOUD_PG_PASSWORD)
	[[ -n $pg_pw ]] || pg_pw=$(openssl rand -hex 24)
	cle_chiffrement=$(valeur_env "$ETC/miyucloud.env" MIYUCLOUD_ENCRYPTION_KEY)
	[[ -n $cle_chiffrement ]] || cle_chiffrement=$(openssl rand -base64 32)
	ecrire_fichier "$ETC/miyucloud.env" 0600 <<FIN
MIYUCLOUD_PG_PASSWORD=$pg_pw
MIYUCLOUD_DB_CONNECTION_STRING=postgres://miyucloud:$pg_pw@127.0.0.1:5432/miyucloud
MIYUCLOUD_ENCRYPTION_KEY=$cle_chiffrement
FIN

	# MAIA : aucun secret ; appelants non épinglés => API protégée refusée (503).
	ecrire_fichier "$ETC/maia.env" 0640 <<FIN
MIYUKINI_DATA_DIR=$VAR/maia
COG_POLITIQUES_DIR=$ETC/strongfather
RUST_LOG=maia=info,tower_http=info
FIN
	chown root:miyukini-maia "$ETC/maia.env"
}

# ------------------------------------------------------------------ PostgreSQL (MiyuCloud)

postgres() {
	log "PostgreSQL : rôle et base miyucloud"
	local pg_pw i
	pg_pw=$(valeur_env "$ETC/miyucloud.env" MIYUCLOUD_PG_PASSWORD)
	[[ -n $pg_pw ]] || die "mot de passe PostgreSQL absent de $ETC/miyucloud.env"
	systemctl enable --now postgresql
	for i in $(seq 1 30); do
		if runuser -u postgres -- psql -tAc 'SELECT 1' >/dev/null 2>&1; then break; fi
		sleep 1
	done
	runuser -u postgres -- psql -q -v ON_ERROR_STOP=1 <<FIN
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'miyucloud') THEN
    CREATE ROLE miyucloud LOGIN;
  END IF;
END
\$\$;
ALTER ROLE miyucloud WITH LOGIN PASSWORD '$pg_pw';
FIN
	if [[ $(runuser -u postgres -- psql -tAc "SELECT 1 FROM pg_database WHERE datname = 'miyucloud'") != 1 ]]; then
		runuser -u postgres -- createdb -O miyucloud miyucloud
	fi
}

# ------------------------------------------------------------------ unités systemd

# Durcissement commun. Chaque service ne reçoit que ce dont il a besoin.
DURCISSEMENT='NoNewPrivileges=true
PrivateTmp=true
PrivateDevices=true
ProtectSystem=strict
ProtectHome=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectKernelLogs=true
ProtectControlGroups=true
ProtectClock=true
ProtectHostname=true
LockPersonality=true
RestrictRealtime=true
RestrictSUIDSGID=true
RestrictNamespaces=true
RestrictAddressFamilies=AF_UNIX AF_INET AF_INET6
SystemCallArchitectures=native
SystemCallFilter=@system-service
CapabilityBoundingSet=
AmbientCapabilities=
MemoryDenyWriteExecute=true
UMask=0077
LimitNOFILE=65535'

# ecrire_unite <nom> <description> <utilisateur> <répertoire> <commande> <chemins-rw> <après> <env-fichier> <env-lignes>
ecrire_unite() {
	local nom=$1 description=$2 utilisateur=$3 repertoire=$4 commande=$5 rw=$6 apres=$7 fichier_env=$8 env_lignes=$9
	local envfile=""
	[[ -n $fichier_env ]] && envfile="EnvironmentFile=-$fichier_env"
	local rwligne=""
	[[ -n $rw ]] && rwligne="ReadWritePaths=$rw"
	cat >"/etc/systemd/system/$nom.service" <<FIN
# Généré par deploy/cog-vm/installer.sh : ne pas éditer à la main.
[Unit]
Description=$description
After=network-online.target $apres
Wants=network-online.target

[Service]
Type=simple
User=$utilisateur
Group=$utilisateur
WorkingDirectory=$repertoire
$envfile
$env_lignes
ExecStart=$commande
Restart=on-failure
RestartSec=3
TimeoutStartSec=30
TimeoutStopSec=30
$DURCISSEMENT
$rwligne

[Install]
WantedBy=multi-user.target
FIN
	chmod 0644 "/etc/systemd/system/$nom.service"
}

unites() {
	log "unités systemd"
	ecrire_unite miyukini-kindmother "KindMother — service libSQL isolé (COG)" \
		miyukini-kindmother "$VAR/kindmother" "$BIN/kindmother-server" "$VAR/kindmother" "" "" \
		"Environment=KINDMOTHER_DATA_DIR=$VAR/kindmother
Environment=KINDMOTHER_LISTEN_ADDR=127.0.0.1:50051"

	ecrire_unite miyukini-maia "MAIA — orchestrateur d'agents IA (COG)" \
		miyukini-maia "$VAR/maia" "$BIN/maia" "$VAR/maia" "" "$ETC/maia.env" \
		"Environment=MAIA_LLM_URL=http://127.0.0.1:11435"

	ecrire_unite miyukini-miyucloud "MiyuCloud — stockage et calendriers (COG)" \
		miyukini-miyucloud "$PARTAGE/miyucloud" "$BIN/miyucloud-server" "$VAR/miyucloud" \
		"postgresql.service" "$ETC/miyucloud.env" \
		"Environment=MIYUCLOUD_STORAGE_PATH=$VAR/miyucloud/storage
Environment=MIYUCLOUD_STATIC_PATH=$PARTAGE/miyucloud/static
Environment=MIYUCLOUD_SERVER_HOST=127.0.0.1
Environment=MIYUCLOUD_SERVER_PORT=8086
Environment=MIYUCLOUD_BASE_URL=http://127.0.0.1:8086
Environment=MIYUCLOUD_ENCRYPTION_ENABLED=true
Environment=MIYUCLOUD_OIDC_ENABLED=false
Environment=MIYUCLOUD_WOPI_ENABLED=false
Environment=MIYUCLOUD_COOKIE_SECURE=true
Environment=RUST_LOG=info"

	ecrire_unite miyukini-cog-web-portal "COG Web Portal — portail HTTP (COG)" \
		miyukini-portail "$VAR/cog-web-portal" "$BIN/cog-web-portal" "" "" "$ETC/cog-web-portal.env" \
		"Environment=PORTAL_LISTEN=127.0.0.1:8090"

	systemctl daemon-reload
}

demarrage() {
	log "démarrage des services"
	local u
	for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
		systemctl enable "$u.service" >/dev/null
		systemctl restart "$u.service"
	done
	# Ne reste pas silencieux en cas d'échec : état affiché.
	sleep 3
	for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
		printf '[installer] %-28s %s\n' "$u" "$(systemctl is-active "$u.service" || true)"
	done
}

# ------------------------------------------------------------------ exécution

log "bundle : $BUNDLE"
paquets
utilisateurs
repertoires
binaires
politiques_et_statiques
environnements
postgres
unites
demarrage

log "installation terminée (Douane non installée comme service : voir README.md)"
