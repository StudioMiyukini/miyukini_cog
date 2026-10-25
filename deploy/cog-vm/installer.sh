#!/usr/bin/env bash
# Installateur des services COG dans la VM jetable de test (idempotent).
#
# A exécuter DANS la VM, en root (sudo), depuis le paquet déposé par pousser.sh :
#   sudo ./installer.sh --vm-jetable --bundle <rép. paquet> [--avec-banc] [--manifeste-sha256 <hex>]
#
# INTERDIT sur un hôte réel (production, serveur personnel, poste de travail) : VM
# jetable cog-essai uniquement (voir README.md). Refus sans --vm-jetable et sur un
# hôte non virtualisé (systemd-detect-virt --vm), comme le banc lot 3.
# Il ne touche ni à /etc/miyukini/douane (banc d'intrusion), ni au réseau (pas de
# ufw/nftables/règles), ni à docker.
#
# Effets :
#   - utilisateurs système dédiés (un par service), pas de shell, pas de home ;
#   - /opt/miyukini/bin (binaires), /opt/miyukini/share (statiques MiyuCloud) ;
#   - paquet vérifié AVANT toute action : MANIFESTE (SHA-256 de chaque fichier, lié
#     au commit) contrôlé, empreinte du manifeste comparée à celle affichée par
#     pousser.sh (--manifeste-sha256) ; binaires installés revérifiés ensuite ;
#   - /etc/miyukini (politiques StrongFather, bridge.toml, fichiers d'environnement),
#     propriété root, lecture seule pour les services ; bridge.toml est présenté à
#     MAIA par un montage lié en lecture seule (BindReadOnlyPaths) à l'endroit où il
#     le lit ($MIYUKINI_DATA_DIR/bridge.toml) : MAIA ne peut ni le réécrire ni le
#     remplacer ;
#   - /var/lib/miyukini/<service> en 0700, propriété du service ;
#   - secrets générés ici (0600, jamais commités) : jeton CSRF du portail, clé de
#     chiffrement et mot de passe PostgreSQL de MiyuCloud. KindMother génère sa propre
#     clé d'identité et sa clé de base au premier démarrage, dans son répertoire 0700 ;
#   - PostgreSQL local (rôle + base `miyucloud`) pour MiyuCloud ; le mot de passe du
#     rôle est posé avec la journalisation des requêtes coupée dans la session ;
#   - unités systemd durcies, écoute sur 127.0.0.1 uniquement ;
#   - profils AppArmor (deploy/apparmor/, copiés depuis le paquet), chargés en mode
#     enforce, attachés aux unités par AppArmorProfile= ;
#   - zone COG (Douane) : arrêtés d'essai générés par arretes_essai, vérifiés par
#     miyudouane (regles, zone) ; table nftables propre « miyudouane_hote » (sortie
#     refusée aux comptes COG hors boucle locale), chargée au démarrage par
#     miyukini-douane-cog.service ; les services en dépendent.
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
MANIFESTE_ATTENDU=""
VM_JETABLE=0

log() { printf '[installer] %s\n' "$*"; }
die() { printf '[installer] ERREUR: %s\n' "$*" >&2; exit 1; }

usage() {
	cat <<'FIN'
Usage : sudo installer.sh --vm-jetable --bundle <répertoire> [--avec-banc] [--manifeste-sha256 <hex>]

  --vm-jetable               obligatoire : confirme la VM jetable cog-essai (jamais un hôte réel)
  --bundle <rép>             paquet déposé par pousser.sh (bin/, config/, static/, banc/, MANIFESTE)
  --avec-banc                installe aussi les outils du banc d'intrusion lot 3
  --manifeste-sha256 <hex>   empreinte du MANIFESTE affichée par pousser.sh (vérifiée)

VM jetable uniquement : refus sans --vm-jetable et sur un hôte non virtualisé.
FIN
}

while (($#)); do
	case $1 in
		--bundle) BUNDLE=${2:?--bundle : valeur attendue}; shift ;;
		--avec-banc) AVEC_BANC=1 ;;
		--vm-jetable) VM_JETABLE=1 ;;
		--manifeste-sha256) MANIFESTE_ATTENDU=${2:?--manifeste-sha256 : valeur attendue}; shift ;;
		-h | --help) usage; exit 0 ;;
		*) usage >&2; die "option inconnue : $1" ;;
	esac
	shift
done

((EUID == 0)) || die "root requis (sudo)"
# Garde-fou B7 : jamais sur un hôte réel (voir README.md).
((VM_JETABLE)) || die "refus : VM jetable cog-essai uniquement ; confirmez avec --vm-jetable (jamais sur un hôte réel)"
if ! command -v systemd-detect-virt >/dev/null 2>&1 || ! systemd-detect-virt --vm -q; then
	die "refus : hôte non virtualisé (systemd-detect-virt --vm) ; cet installateur ne s'exécute que dans la VM jetable"
fi
[[ -n $BUNDLE ]] || BUNDLE=$(cd "$(dirname "$0")" && pwd)
BUNDLE=$(cd "$BUNDLE" && pwd)

# ------------------------------------------------------------------ manifeste (B7)

# Vérifie le paquet AVANT toute action : empreinte du manifeste (si fournie), puis
# SHA-256 de chaque fichier, et aucun fichier hors manifeste.
verifier_paquet() {
	local manifeste=$BUNDLE/MANIFESTE empreinte listes presents
	[[ -f $manifeste ]] || die "MANIFESTE absent du paquet (pousser.sh le génère)"
	empreinte=$(sha256sum "$manifeste" | cut -d' ' -f1)
	log "manifeste : sha256 $empreinte"
	sed -n 's/^# /[installer]   /p' "$manifeste"
	if [[ -n $MANIFESTE_ATTENDU ]]; then
		[[ $empreinte == "$MANIFESTE_ATTENDU" ]] ||
			die "empreinte du MANIFESTE différente de celle affichée par pousser.sh ($MANIFESTE_ATTENDU)"
	else
		log "ATTENTION : --manifeste-sha256 non fourni ; comparer l'empreinte ci-dessus à celle affichée par pousser.sh"
	fi
	(cd "$BUNDLE" && grep -v '^#' MANIFESTE | sha256sum --check --strict --quiet -) ||
		die "paquet altéré : SHA-256 différent du MANIFESTE"
	listes=$(grep -vc '^#' "$manifeste")
	presents=$(cd "$BUNDLE" && find . -type f ! -path ./MANIFESTE | wc -l)
	[[ $listes == "$presents" ]] || die "paquet : $presents fichier(s) présents, $listes au manifeste"
	log "paquet conforme au manifeste ($listes fichiers)"
}
verifier_paquet

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
	apt-get install -y -qq --no-install-recommends postgresql openssl ca-certificates python3 \
		apparmor apparmor-utils nftables
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
	# bridge.toml (B2) : source de vérité = dépôt ; installé dans /etc/miyukini,
	# propriété root, lecture seule. MAIA le lit à $MIYUKINI_DATA_DIR/bridge.toml :
	# ce chemin n'est qu'un point de montage (fichier vide root) recouvert, dans
	# l'espace de noms du service, par un montage lié en lecture seule
	# (BindReadOnlyPaths, voir unites). L'ancienne copie modifiable est retirée.
	install -o root -g root -m 0444 "$BUNDLE/config/bridge.toml" "$ETC/bridge.toml"
	rm -f "$VAR/maia/bridge.toml"
	install -o root -g root -m 0444 /dev/null "$VAR/maia/bridge.toml"
}

# Revérifie les binaires installés contre le manifeste du paquet (conservé).
verifier_binaires_installes() {
	install -o root -g root -m 0644 "$BUNDLE/MANIFESTE" "$PREFIXE/MANIFESTE"
	(cd "$PREFIXE" && grep -v '^#' MANIFESTE | grep '  \./bin/' | sha256sum --check --strict --quiet -) ||
		die "binaires installés différents du MANIFESTE"
	log "binaires installés conformes au manifeste ($PREFIXE/MANIFESTE)"
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
	# Clé : base64 de exactement 32 octets (sinon le service refuse de démarrer),
	# conservée d'une réinstallation à l'autre ; la perdre rend les blobs chiffrés illisibles.
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
	# B3 : le mot de passe ne doit apparaître dans aucun journal PostgreSQL
	# (log_statement, requêtes lentes) : journalisation coupée dans CETTE session,
	# avant l'ALTER ROLE. Il transite par l'entrée standard, jamais par argv.
	runuser -u postgres -- psql -q -v ON_ERROR_STOP=1 <<FIN
SET log_statement = 'none';
SET log_min_duration_statement = -1;
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

# ------------------------------------------------------------------ AppArmor

# Profils du paquet (deploy/apparmor/) : copiés dans /etc/apparmor.d et chargés.
# Un profil qui ne se charge pas arrête l'installation (aucun service sans profil).
apparmor_profils() {
	log "AppArmor : profils"
	local f nom
	[[ -d $BUNDLE/apparmor ]] || die "apparmor/ absent du paquet (pousser.sh le copie depuis deploy/apparmor/)"
	for f in "$BUNDLE"/apparmor/miyukini-*; do
		nom=$(basename "$f")
		install -o root -g root -m 0644 "$f" "/etc/apparmor.d/$nom"
		apparmor_parser -r -W "/etc/apparmor.d/$nom" || die "profil AppArmor $nom refusé par apparmor_parser"
		log "  profil $nom chargé"
	done
}

# ------------------------------------------------------------------ Douane : zone COG

DOUANE_VAR=$VAR/douane
DOUANE_NFT=$ETC/douane-cog.nft
DOUANE_UNITE=miyukini-douane-cog.service
ATTENTE_UNITE=miyukini-attente-kindmother
# Ports d'écoute des services COG (loopback seulement) : refus de toute entrée non locale.
PORTS_COG="50051, 11435, 8086, 8090, 5432"

# Arrêtés d'essai : générés par arretes_essai (clé jetable, jamais conservée), vérifiés par
# miyudouane (signature, constitution, vigueur) ; la zone COG doit se compiler depuis eux.
# Le texte nftables de la zone est un rendu pour espaces de noms (miyudouane zone) : il ne
# s'applique pas à l'hôte. La table hôte ci-dessous est donc écrite ici, pour les seuls
# comptes des services COG. Limite : miyudouane ne produit pas de règles par UID (voir README).
douane_arretes() {
	log "Douane : arrêtés d'essai et vérification"
	local ex=$BUNDLE/banc/cible/examples/arretes_essai
	local dm=$BIN/miyudouane
	local maint ancre
	# Plan des essais : mêmes sandboxes que le banc (tests/intrusion-lot3/scenario.sh) ; le
	# Douane refuse un arrêté dont une voie n'a pas de sandbox dans le plan.
	local sb=(--sandbox cog:voie-a --sandbox cog:voie-b --sandbox franche:voie-a
		--sandbox franche:voie-b --sandbox cog:capteur --sandbox locale:capteur)
	[[ -x $ex ]] || die "arretes_essai absent du paquet (banc/cible/examples)"
	install -d -o root -g root -m 0700 "$DOUANE_VAR"
	rm -rf "$DOUANE_VAR/arretes"
	install -d -o root -g root -m 0700 "$DOUANE_VAR/arretes"
	maint=$(date +%s)
	"$ex" --sortie "$DOUANE_VAR/arretes" --maintenant "$maint" >/dev/null ||
		die "arretes_essai : échec de génération"
	ancre=$(tr -d '\n' <"$DOUANE_VAR/arretes/ancre.hex")
	[[ $ancre =~ ^[0-9a-f]{64}$ ]] || die "ancre d'essai invalide"
	local f
	for f in externe portail-interne portail-local; do
		"$dm" regles --frontiere "$f" --arrete "$DOUANE_VAR/arretes/$f.arrete" --ancre "$ancre" \
			--maintenant "$maint" "${sb[@]}" >/dev/null || die "miyudouane regles : arrêté $f refusé"
	done
	"$dm" zone --zone cog --arrete "portail-interne=$DOUANE_VAR/arretes/portail-interne.arrete" \
		--arrete "portail-local=$DOUANE_VAR/arretes/portail-local.arrete" \
		--ancre "$ancre" --maintenant "$maint" "${sb[@]}" >/dev/null || die "miyudouane zone cog : compilation refusée"
	log "  arrêtés externe, portail-interne, portail-local : signature et constitution vérifiées ; zone COG compilée"
}

# Table hôte « miyudouane_hote » : les comptes COG n'envoient rien hors boucle locale (aucune
# route sortante, Franche isolée) ; les services COG n'acceptent d'entrée que par loopback.
# Table propre : add + delete + table, aucun « flush ruleset » (autres tables intactes).
douane_cog() {
	log "Douane : zone COG (table nftables miyudouane_hote)"
	local uids="" u
	for u in "${UTILISATEURS[@]}"; do
		uids+="${uids:+, }$(id -u "$u")"
	done
	cat >"$DOUANE_NFT.tmp" <<FIN
# Généré par deploy/cog-vm/installer.sh (VM jetable) — Douane, zone COG.
# Table propre à la Douane : miyudouane_hote. Aucune autre table n'est touchée.
add table inet miyudouane_hote
delete table inet miyudouane_hote
table inet miyudouane_hote {
	chain sortie_cog {
		type filter hook output priority 0; policy accept;
		meta skuid { $uids } oifname != "lo" counter drop comment "zone COG : aucune route sortante"
	}
	chain entree_cog {
		type filter hook input priority 0; policy accept;
		iifname != "lo" tcp dport { $PORTS_COG } counter drop comment "services COG : loopback seulement"
	}
}
FIN
	chmod 0644 "$DOUANE_NFT.tmp"
	chown root:root "$DOUANE_NFT.tmp"
	mv -f "$DOUANE_NFT.tmp" "$DOUANE_NFT"
	nft -c -f "$DOUANE_NFT" || die "nft -c refuse $DOUANE_NFT"

	cat >"/etc/systemd/system/$DOUANE_UNITE" <<FIN
# Généré par deploy/cog-vm/installer.sh : ne pas éditer à la main.
[Unit]
Description=Douane — zone COG (table nftables miyudouane_hote)
After=nftables.service
Before=miyukini-kindmother.service miyukini-maia.service miyukini-miyucloud.service miyukini-cog-web-portal.service

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/sbin/nft -f $DOUANE_NFT
ExecStop=/usr/sbin/nft delete table inet miyudouane_hote

[Install]
WantedBy=multi-user.target
FIN
	chmod 0644 "/etc/systemd/system/$DOUANE_UNITE"
	nft -f "$DOUANE_NFT"
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

# ecrire_unite <nom> <description> <utilisateur> <répertoire> <commande> <chemins-rw> <après> <env-fichier> <env-lignes> <profil-apparmor>
# Toute unité dépend de la zone COG de la Douane (Requires= : pas de service sans zone).
ecrire_unite() {
	local nom=$1 description=$2 utilisateur=$3 repertoire=$4 commande=$5 rw=$6 apres=$7 fichier_env=$8 env_lignes=$9
	local profil=${10:-}
	local envfile=""
	[[ -n $fichier_env ]] && envfile="EnvironmentFile=-$fichier_env"
	local rwligne=""
	[[ -n $rw ]] && rwligne="ReadWritePaths=$rw"
	local profligne=""
	[[ -n $profil ]] && profligne="AppArmorProfile=$profil"
	cat >"/etc/systemd/system/$nom.service" <<FIN
# Généré par deploy/cog-vm/installer.sh : ne pas éditer à la main.
[Unit]
Description=$description
After=network-online.target $DOUANE_UNITE $apres
Wants=network-online.target
Requires=$DOUANE_UNITE

[Service]
Type=simple
User=$utilisateur
Group=$utilisateur
WorkingDirectory=$repertoire
$envfile
$env_lignes
$profligne
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
Environment=KINDMOTHER_LISTEN_ADDR=127.0.0.1:50051" "miyukini-kindmother"

	# bridge.toml (B2) : /etc/miyukini/bridge.toml (root, lecture seule) monté en
	# lecture seule à l'endroit où MAIA le lit ; ni réécriture ni remplacement.
	# MAIA ouvre la mémoire (KindMother) une seule fois, au démarrage, sans nouvel essai : elle
	# démarre après KindMother et attend (borné, sans échec fatal) que son port écoute.
	# Attente de KindMother : unité distincte, SANS profil AppArmor. Le profil de MAIA interdit
	# toute exécution (bash, timeout) ; AppArmorProfile= s'appliquerait à chaque commande de l'unité.
	cat >"/etc/systemd/system/$ATTENTE_UNITE.service" <<FIN
# Généré par deploy/cog-vm/installer.sh : ne pas éditer à la main.
[Unit]
Description=Attente de KindMother (127.0.0.1:50051) avant MAIA
After=miyukini-kindmother.service

[Service]
Type=oneshot
RemainAfterExit=yes
User=miyukini-maia
Group=miyukini-maia
ExecStart=-/usr/bin/timeout 30 /bin/bash -c 'until (exec 3<>/dev/tcp/127.0.0.1/50051) 2>/dev/null; do sleep 0.5; done'

[Install]
RequiredBy=miyukini-maia.service
FIN
	chmod 0644 "/etc/systemd/system/$ATTENTE_UNITE.service"

	ecrire_unite miyukini-maia "MAIA — orchestrateur d'agents IA (COG)" \
		miyukini-maia "$VAR/maia" "$BIN/maia" "$VAR/maia" "miyukini-kindmother.service $ATTENTE_UNITE.service" "$ETC/maia.env" \
		"Environment=MAIA_LLM_URL=http://127.0.0.1:11435
BindReadOnlyPaths=$ETC/bridge.toml:$VAR/maia/bridge.toml" "miyukini-maia"

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
Environment=RUST_LOG=info" "miyukini-miyucloud"

	ecrire_unite miyukini-cog-web-portal "COG Web Portal — portail HTTP (COG)" \
		miyukini-portail "$VAR/cog-web-portal" "$BIN/cog-web-portal" "" "" "$ETC/cog-web-portal.env" \
		"Environment=PORTAL_LISTEN=127.0.0.1:8090" "miyukini-cog-web-portal"

	systemctl daemon-reload
}

demarrage() {
	log "démarrage des services"
	local u
	systemctl enable "$DOUANE_UNITE" "$ATTENTE_UNITE.service" >/dev/null
	systemctl restart "$DOUANE_UNITE"
	for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
		systemctl enable "$u.service" >/dev/null
		systemctl restart "$u.service"
	done
	# Ne reste pas silencieux en cas d'échec : état affiché.
	sleep 3
	for u in "$DOUANE_UNITE" miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
		printf '[installer] %-28s %s\n' "$u" "$(systemctl is-active "$u" || true)"
	done
}

# ------------------------------------------------------------------ exécution

log "bundle : $BUNDLE"
paquets
utilisateurs
repertoires
binaires
verifier_binaires_installes
politiques_et_statiques
environnements
postgres
apparmor_profils
douane_arretes
douane_cog
unites
demarrage

log "installation terminée (Douane non installée comme service : voir README.md)"
