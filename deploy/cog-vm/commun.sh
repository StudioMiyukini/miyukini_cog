#!/usr/bin/env bash
# Fonctions communes des installateurs COG : deploy/cog-vm/installer.sh (VM jetable cog-essai)
# et deploy/recette/installer.sh (machine de recette). FICHIER SOURCÉ : aucun effet de bord à
# l'inclusion, seulement des définitions.
#
# Le script appelant : règle `set -euo pipefail`, définit LOG_TAG (libellé des journaux), vérifie
# ses propres garde-fous (root, cible autorisée), copie le paquet dans un dossier privé (root
# 0700, sans suivre les liens), puis source ce fichier depuis CETTE copie. Il renseigne BUNDLE
# (la copie privée) AVANT d'appeler les fonctions de paquet, et ne les appelle qu'après
# verifier_paquet (et, en recette, l'authentification). Les fonctions de paquet n'installent que
# des fichiers listés au MANIFESTE (installer_fichier), jamais un glob.
#
# Layout de paquet attendu (produit par pousser.sh ou deploy/recette/construire-paquet.sh) :
#   MANIFESTE  bin/  config/{bridge.toml,strongfather/,maia/maia-llm.toml}  static/{static,static-dist}
#   apparmor/miyukini-*  banc/cible/examples/arretes_essai | bin/arretes_essai (optionnel)
#   systemd/*.service (optionnel : sinon générées ici)  exploitation/ (recette)  commun.sh
# Le mini modèle de MAIA et d'Alicia (artefact MMIA, ≈ 1,1 Go) n'est PAS dans ce paquet : il est
# livré à part (paquet modèle) et installé par modele_ia (MODELE_ARTEFACT).

LOG_TAG=${LOG_TAG:-cog}
log() { printf '[%s] %s\n' "$LOG_TAG" "$*"; }
die() { printf '[%s] ERREUR: %s\n' "$LOG_TAG" "$*" >&2; exit 1; }

# Opérations de root dans les dossiers des comptes de service (revue de sécurité) : un compte de
# service maître de son dossier peut y poser un lien (p.ex. vers /etc/shadow). Root n'y crée, ne
# modifie, ne lit ni ne change jamais les droits d'un fichier À TRAVERS un lien :
#   - création de fichiers et de sous-dossiers dans un dossier de service : EN TANT QUE ce compte
#     (en_tant_que, setpriv sans aucune capacité) ;
#   - fichier root à poser dans un dossier de service (points de montage) : préparé dans un dossier
#     root de $VAR puis renommé (rename(2) remplace un lien, ne le suit jamais : poser_fichier_root) ;
#   - chown/chmod de root : seulement sur des chemins dont le parent appartient à root, liens
#     refusés d'abord (refuser_liens), chown -h.
refuser_liens() {
	local c
	for c in "$@"; do
		[[ ! -L $c ]] || die "lien symbolique refusé : $c (aucune opération de root à travers un lien)"
	done
}
# en_tant_que <compte> <commande…> : commande sous l'identité du compte (uid, gid, groupes ; root
# perd toute capacité en changeant d'uid).
en_tant_que() {
	local u=$1
	shift
	setpriv --reuid="$u" --regid="$(id -g "$u")" --init-groups -- "$@"
}
# dossier_service <compte> <chemin> <mode> : crée (ou remet au mode) un sous-dossier d'un dossier de
# service, EN TANT QUE le compte ; un lien à sa place fait échouer.
dossier_service() {
	en_tant_que "$1" sh -c 'set -e; [ ! -L "$1" ] || { echo "lien symbolique refusé : $1" >&2; exit 1; }
		mkdir -p -m "$2" "$1"; chmod "$2" "$1"' _ "$2" "$3" || die "dossier $2 (compte $1) refusé"
}
# poser_fichier_root <destination> <mode> : remplace <destination> (dans un dossier de service) par
# un fichier root neuf au contenu de stdin ; le renommage remplace un lien éventuel sans le suivre,
# et échoue si un dossier occupe la place.
poser_fichier_root() {
	local dest=$1 mode=$2 prep
	refuser_liens "$(dirname "$dest")"
	prep=$(mktemp -d "$VAR/.preparation.XXXXXX")
	cat >"$prep/f"
	chown root:root "$prep/f"
	chmod "$mode" "$prep/f"
	if ! mv -f -T "$prep/f" "$dest"; then
		rm -rf "$prep"
		die "$dest : remplacement refusé"
	fi
	rmdir "$prep"
}

PREFIXE=/opt/miyukini
BIN=$PREFIXE/bin
PARTAGE=$PREFIXE/share
EXPLOITATION=$PREFIXE/exploitation
ETC=/etc/miyukini
VAR=/var/lib/miyukini
UNITES_DIR=${UNITES_DIR:-/etc/systemd/system}

BUNDLE=${BUNDLE:-}
MANIFESTE_ATTENDU=${MANIFESTE_ATTENDU:-}
AVEC_BANC=${AVEC_BANC:-0}

# Binaires des services, puis outils installés sans service (présents dans le paquet seulement).
BINAIRES=(kindmother-server maia maia-llm miyucloud-server cog-web-portal miyudouane portail-sas portail-entree portail-sortie)
BINAIRES_OUTILS=(miyurelais miyuregistre-publier outil-sceau)

# Un utilisateur système par service ; aucun shell, aucun home. Portail scindé (A-022 §3) : un
# compte par moitié (entrée, sortie) et un compte par extrémité de file (façade, moteur), sans service
# installé pour ces deux derniers (ils identifient l'écrivain ou le lecteur d'une file).
UTILISATEURS=(miyukini-kindmother miyukini-maia miyukini-maia-llm miyukini-miyucloud miyukini-portail
	miyukini-portail-entree miyukini-portail-sortie miyukini-facade miyukini-moteur)
# Ordre de démarrage ; l'arrêt se fait dans l'ordre inverse. maia-llm (moteur llama.cpp, A-026)
# démarre avant MAIA, qui l'appelle.
SERVICES=(miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal
	miyukini-portail-entree miyukini-portail-sortie)

# Portail scindé : files à sens unique. Chaque file : répertoire du LECTEUR, mode 3730 (collant,
# setgid, groupe -wx, rien pour les autres), groupe PROPRE à la file réunissant son seul écrivain et
# son seul lecteur. L'entrée et la sortie n'ont aucun groupe en commun.
PORTAIL_FILES=$VAR/portail-files
#  <file> <lecteur (propriétaire)> <écrivain>
FILES_PORTAIL=(
	"depot miyukini-portail-entree miyukini-facade"
	"admises miyukini-moteur miyukini-portail-entree"
	"reponses miyukini-portail-sortie miyukini-moteur"
	"publication miyukini-facade miyukini-portail-sortie"
)

DOUANE_VAR=$VAR/douane
DOUANE_NFT=$ETC/douane-cog.nft
DOUANE_UNITE=miyukini-douane-cog.service
ATTENTE_UNITE=miyukini-attente-kindmother
# Frontière externe (espace mcog-f-externe) : règles strictes → tc → filet (douane-externe.sh).
EXTERNE_UNITE=miyukini-douane-externe.service
LIBEXEC=$PREFIXE/libexec
DESCRIPTION_TC=$ETC/douane-tc.desc
# Capacités des interfaces de la frontière externe (Kio/s ; description miyudouane-tc 1) :
#   ext0   carte physique 1 Gbit/s       = 10^9 / 8 / 1024  ≈ 122 070 Kio/s
#   wg-fed WireGuard fédéral 100 Mbit/s  = 10^8 / 8 / 1024  ≈  12 207 Kio/s (lien montant domestique)
#   ext-franche (veth vers la Franche) : défaut de la Douane (1 250 000 Kio/s ≈ 10 Gbit/s)
CAPACITES_TC=("ext0 122070" "wg-fed 12207")
# Ports d'écoute des services COG (loopback seulement) : refus de toute entrée non locale.
# 11436 : maia-llm (MAIA garde 11435).
PORTS_COG_TCP=(50051 11435 11436 8086 8090 5432)

# Mini modèle (A-026) : Qwen2.5-1.5B-Instruct Q4_K_M, livré par le canal (artefact MMIA).
MODELES=$VAR/modeles
MODELE_COMPOSANT=qwen2.5-1.5b-instruct-q4_k_m
MODELE_VERSION=1
MODELE_SHA256=6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e
MODELE_TAILLE=1117320736
# Ancre des manifestes de modèles : SIGNATURE DE DÉVELOPPEMENT (clé hors dépôt,
# /srv/app/models/cle-dev-maia.hex) en attendant la re-signature par la clé du canal officiel
# (A-026 « Reste »). Épinglée ici, donc couverte par le MANIFESTE (et sa signature en recette).
ANCRE_MODELES=${ANCRE_MODELES:-fb9d57a05cf97bb2cdce1fa58d17e5a8d4ac1ecc38e67c906f92853e7474611a}
# Artefact MMIA à installer (optionnel ; chemin hors paquet, paquet modèle séparé).
MODELE_ARTEFACT=${MODELE_ARTEFACT:-}
# Clé publique d'Alicia (cle_voix, base64 URL ou hex) à épingler dans maia-llm, fournie par
# l'exploitant (Alicia tourne dans Central, hors de ces installateurs). Absente : seule MAIA.
CLE_ALICIA_FICHIER=$ETC/alicia-cle-voix.pub

# ------------------------------------------------------------------ manifeste et signature

# Contrôle de forme du paquet (avant tout `sha256sum --check`) :
#   - aucune entrée qui ne soit ni fichier régulier ni dossier (lien symbolique, tube, périphérique,
#     socket) : `sha256sum` et `install` suivraient un lien que `find -type f` ne compte pas ;
#   - la LISTE EXACTE triée des fichiers présents (hors ./MANIFESTE, ./MANIFESTE.sceau et
#     ./MANIFESTE.sig, à la racine seulement) est égale à celle du MANIFESTE : aucun fichier ajouté,
#     retiré ou substitué, même à nombre égal ;
#   - chaque ligne du MANIFESTE est « <sha256>  ./<chemin> » (aucun nom échappé, aucun doublon).
# L'installation ne lit ensuite QUE des fichiers listés (fichiers_du_manifeste, installer_fichier),
# jamais un glob du paquet. Fonctions sans effet de bord : réutilisées par authentifier_paquet.

# chemins_du_manifeste <MANIFESTE> : chemins listés (forme « ./x »), triés.
chemins_du_manifeste() {
	grep -v '^#' "$1" | sed -n 's/^[0-9a-f]\{64\}  //p' | LC_ALL=C sort
}

# controle_forme_paquet <répertoire> <MANIFESTE> : code 1, motif sur stderr, au moindre écart.
controle_forme_paquet() {
	local dir=$1 manifeste=$2 speciaux mal listes presents
	[[ -d $dir && ! -L $dir ]] || { echo "paquet refusé : $dir n'est pas un dossier" >&2; return 1; }
	speciaux=$(cd "$dir" && find . ! -type f ! -type d -printf '%y %p\n' | head -n 5)
	if [[ -n $speciaux ]]; then
		echo "paquet refusé : entrée(s) ni fichier régulier ni dossier (lien symbolique, tube, périphérique) : $(tr '\n' ' ' <<<"$speciaux")" >&2
		return 1
	fi
	[[ -f $manifeste && ! -L $manifeste ]] || { echo "paquet refusé : MANIFESTE absent" >&2; return 1; }
	mal=$(grep -v '^#' "$manifeste" | grep -cvE '^[0-9a-f]{64}  \./[^[:cntrl:]]+$' || true)
	((mal == 0)) || { echo "paquet refusé : $mal ligne(s) mal formée(s) dans le MANIFESTE" >&2; return 1; }
	listes=$(chemins_du_manifeste "$manifeste")
	[[ -n $listes ]] || { echo "paquet refusé : MANIFESTE vide" >&2; return 1; }
	[[ $listes == "$(LC_ALL=C sort -u <<<"$listes")" ]] || { echo "paquet refusé : chemin en double dans le MANIFESTE" >&2; return 1; }
	presents=$(cd "$dir" && find . -type f ! -path ./MANIFESTE ! -path ./MANIFESTE.sceau ! -path ./MANIFESTE.sig | LC_ALL=C sort)
	if [[ $listes != "$presents" ]]; then
		echo "paquet refusé : liste des fichiers différente du MANIFESTE ($(grep -c . <<<"$presents") présents, $(grep -c . <<<"$listes") listés) : $(diff <(printf '%s\n' "$listes") <(printf '%s\n' "$presents") | sed -n 's/^< /absent /p; s/^> /hors manifeste /p' | head -n 5 | tr '\n' ' ')" >&2
		return 1
	fi
}

# verifier_paquet : forme du paquet (ci-dessus), puis SHA-256 de chaque fichier listé.
# MANIFESTE_ATTENDU (optionnel) : empreinte du MANIFESTE affichée à part, comparée ici.
# BUNDLE doit être la copie privée (root, 0700) faite par l'installateur (copier_paquet_prive).
verifier_paquet() {
	local manifeste=$BUNDLE/MANIFESTE empreinte
	[[ -f $manifeste && ! -L $manifeste ]] || die "MANIFESTE absent du paquet"
	empreinte=$(sha256sum "$manifeste" | cut -d' ' -f1)
	log "manifeste : sha256 $empreinte"
	sed -n 's/^# /['"$LOG_TAG"']   /p' "$manifeste"
	if [[ -n $MANIFESTE_ATTENDU ]]; then
		[[ $empreinte == "$MANIFESTE_ATTENDU" ]] ||
			die "empreinte du MANIFESTE différente de celle attendue ($MANIFESTE_ATTENDU)"
	fi
	controle_forme_paquet "$BUNDLE" "$manifeste" || die "paquet non conforme au MANIFESTE (voir ci-dessus)"
	(cd "$BUNDLE" && grep -v '^#' MANIFESTE | sha256sum --check --strict --quiet -) ||
		die "paquet altéré : SHA-256 différent du MANIFESTE"
	log "paquet conforme au manifeste ($(grep -vc '^#' "$manifeste") fichiers, liste exacte, aucun lien)"
}

# fichiers_du_manifeste <regex ERE> : chemins relatifs (sans « ./ ») listés au MANIFESTE du
# paquet, filtrés par la regex. Seule source des fichiers installés.
fichiers_du_manifeste() {
	chemins_du_manifeste "$BUNDLE/MANIFESTE" | sed 's|^\./||' | grep -E "$1" || true
}

# au_manifeste <chemin relatif> : le fichier est listé au MANIFESTE du paquet.
au_manifeste() {
	# Sans -q : grep lit toute l'entrée (un arrêt anticipé ferait échouer « sort » par SIGPIPE, et
	# le tube entier sous pipefail).
	chemins_du_manifeste "$BUNDLE/MANIFESTE" | grep -xF "./$1" >/dev/null
}

# installer_fichier <mode> <chemin relatif au paquet> <destination> : installe un fichier LISTÉ,
# contrôlé régulier et non lien juste avant la copie.
installer_fichier() {
	local mode=$1 rel=$2 dest=$3
	au_manifeste "$rel" || die "$rel absent du MANIFESTE : non installé"
	[[ -f $BUNDLE/$rel && ! -L $BUNDLE/$rel ]] || die "$rel : pas un fichier régulier (lien ?) : non installé"
	install -o root -g root -m "$mode" "$BUNDLE/$rel" "$dest"
}

# verifier_signature <clé publique hex ou fichier> : le MANIFESTE est scellé par l'éditeur
# (Sceau v1.1, type ManifestePaquet). Seule cette clé épinglée est acceptée.
verifier_signature() {
	local cle=$1 outil=$BUNDLE/bin/outil-sceau
	[[ -x $outil && ! -L $outil ]] && au_manifeste bin/outil-sceau || die "outil-sceau absent du paquet (bin/outil-sceau)"
	[[ -f $BUNDLE/MANIFESTE.sceau ]] || die "MANIFESTE.sceau absent : paquet non signé"
	"$outil" verifier --cle-publique "$cle" --manifeste "$BUNDLE/MANIFESTE" --sceau "$BUNDLE/MANIFESTE.sceau" ||
		die "signature du paquet refusée (clé d'éditeur attendue non reconnue, ou paquet altéré)"
	log "signature du MANIFESTE vérifiée"
}

# verifier_binaires_installes : copie le MANIFESTE dans /opt/miyukini et revérifie les binaires.
verifier_binaires_installes() {
	install -o root -g root -m 0644 "$BUNDLE/MANIFESTE" "$PREFIXE/MANIFESTE"
	(cd "$PREFIXE" && grep -v '^#' MANIFESTE | grep '  \./bin/' | grep -v 'bin/arretes_essai$' | sha256sum --check --strict --quiet -) ||
		die "binaires installés différents du MANIFESTE"
	log "binaires installés conformes au manifeste ($PREFIXE/MANIFESTE)"
}

# ------------------------------------------------------------------ paquets et comptes

paquets() {
	log "paquets apt"
	export DEBIAN_FRONTEND=noninteractive
	apt-get update -qq
	apt-get install -y -qq --no-install-recommends postgresql openssl ca-certificates python3 \
		apparmor apparmor-utils nftables zstd curl iproute2 kmod libgomp1
	if ((AVEC_BANC)); then
		apt-get install -y -qq --no-install-recommends \
			iproute2 nftables kmod procps socat netcat-openbsd tcpdump iputils-ping nmap
	fi
}

utilisateurs() {
	local u
	for u in "${UTILISATEURS[@]}"; do
		if ! getent passwd "$u" >/dev/null; then
			log "création de l'utilisateur $u"
			useradd --system --no-create-home --home-dir /nonexistent --shell /usr/sbin/nologin "$u"
		fi
	done
	groupes_files_portail
}

# Groupes des files du Portail : miyukini-q-<file>, membres = écrivain et lecteur de CETTE file
# seulement (toute autre appartenance est retirée : idempotent et correctif).
groupes_files_portail() {
	local ligne f lecteur ecrivain g m
	for ligne in "${FILES_PORTAIL[@]}"; do
		read -r f lecteur ecrivain <<<"$ligne"
		g=miyukini-q-$f
		getent group "$g" >/dev/null || groupadd --system "$g"
		for m in $(getent group "$g" | cut -d: -f4 | tr ',' ' '); do
			[[ $m == "$lecteur" || $m == "$ecrivain" ]] || gpasswd -d "$m" "$g" >/dev/null
		done
		usermod -a -G "$g" "$lecteur"
		usermod -a -G "$g" "$ecrivain"
	done
}

# ------------------------------------------------------------------ répertoires et binaires

repertoires() {
	install -d -o root -g root -m 0755 "$PREFIXE" "$BIN" "$PARTAGE" "$PARTAGE/miyucloud"
	install -d -o root -g root -m 0755 "$ETC" "$ETC/strongfather"
	install -d -o root -g root -m 0755 "$VAR"

	# Dossiers des services : enfants directs de $VAR (root, 0755), donc hors d'atteinte des comptes
	# de service ; liens refusés quand même, chown -h.
	local svc
	for svc in kindmother:miyukini-kindmother maia:miyukini-maia maia-llm:miyukini-maia-llm \
		miyucloud:miyukini-miyucloud cog-web-portal:miyukini-portail; do
		refuser_liens "$VAR/${svc%%:*}"
		install -d -o root -g root -m 0700 "$VAR/${svc%%:*}"
		chown -h "${svc#*:}" "$VAR/${svc%%:*}"
	done
	# Dossier du canal des modèles : root, 0755 ; écrit par l'installateur du canal seulement.
	refuser_liens "$MODELES"
	install -d -o root -g root -m 0755 "$MODELES"
	# Sous-dossiers des dossiers de service : créés EN TANT QUE le compte.
	dossier_service miyukini-miyucloud "$VAR/miyucloud/storage" 0700
	repertoires_portail
}

# Portail scindé : états propres (0700, un par moitié), descripteurs des Cores (root, groupe du
# compte destinataire, 2750 : jamais lisibles des autres ni de l'autre moitié), files (3730).
repertoires_portail() {
	local m ligne f lecteur ecrivain
	for m in entree sortie; do
		refuser_liens "$VAR/portail-$m" "$ETC/portail-$m" "$ETC/portail-$m/voies"
		install -d -o "miyukini-portail-$m" -g "miyukini-portail-$m" -m 0700 "$VAR/portail-$m"
		chown -h "miyukini-portail-$m:miyukini-portail-$m" "$VAR/portail-$m"
		chmod 0700 "$VAR/portail-$m"
		install -d -o root -g "miyukini-portail-$m" -m 0750 "$ETC/portail-$m" "$ETC/portail-$m/voies"
		chmod 2750 "$ETC/portail-$m/voies"
	done
	install -d -o root -g root -m 0755 "$PORTAIL_FILES"
	for ligne in "${FILES_PORTAIL[@]}"; do
		read -r f lecteur ecrivain <<<"$ligne"
		refuser_liens "$PORTAIL_FILES/$f"
		install -d "$PORTAIL_FILES/$f"
		chown -h "$lecteur:miyukini-q-$f" "$PORTAIL_FILES/$f"
		chmod 3730 "$PORTAIL_FILES/$f"
	done
}

binaires() {
	local b
	for b in "${BINAIRES[@]}"; do
		installer_fichier 0755 "bin/$b" "$BIN/$b"
	done
	for b in "${BINAIRES_OUTILS[@]}"; do
		if au_manifeste "bin/$b"; then
			installer_fichier 0755 "bin/$b" "$BIN/$b"
		fi
	done
}

politiques_et_statiques() {
	# Politiques StrongFather (lues par MAIA via COG_POLITIQUES_DIR) : lecture seule.
	local f n=0
	while IFS= read -r f; do
		installer_fichier 0644 "$f" "$ETC/strongfather/${f##*/}"
		n=$((n + 1))
	done < <(fichiers_du_manifeste '^config/strongfather/[^/]+\.toml$')
	((n > 0)) || die "aucune politique config/strongfather/*.toml au MANIFESTE"
	# Statiques MiyuCloud (version de production et version source) : fichiers listés seulement.
	rm -rf "$PARTAGE/miyucloud/static" "$PARTAGE/miyucloud/static-dist"
	install -d -o root -g root -m 0755 "$PARTAGE/miyucloud/static" "$PARTAGE/miyucloud/static-dist"
	while IFS= read -r f; do
		[[ -f $BUNDLE/$f && ! -L $BUNDLE/$f ]] || die "$f : pas un fichier régulier (lien ?) : non installé"
		install -D -o root -g root -m 0644 "$BUNDLE/$f" "$PARTAGE/miyucloud/${f#static/}"
	done < <(fichiers_du_manifeste '^static/static(-dist)?/')
	chown -R root:root "$PARTAGE/miyucloud"
	find "$PARTAGE/miyucloud" -type d -exec chmod 0755 {} +
	find "$PARTAGE/miyucloud" -type f -exec chmod 0644 {} +
	# bridge.toml (B2) : installé dans /etc/miyukini, propriété root, lecture seule. MAIA le lit à
	# $MIYUKINI_DATA_DIR/bridge.toml : ce chemin n'est qu'un point de montage (fichier vide root)
	# recouvert, dans l'espace de noms du service, par un montage lié en lecture seule
	# (BindReadOnlyPaths, voir generer_unites).
	# upstream_url vidé (A-026) : aucun moteur d'IA hors COG, même proposé ; il serait de toute façon
	# refusé par la politique maia-inference-externe-refusee.
	au_manifeste config/bridge.toml || die "config/bridge.toml absent du MANIFESTE"
	sed 's|^upstream_url[[:space:]]*=.*$|upstream_url = ""|' "$BUNDLE/config/bridge.toml" | ecrire_fichier "$ETC/bridge.toml" 0444
	poser_fichier_root "$VAR/maia/bridge.toml" 0444 </dev/null
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
	# Clé : base64 de exactement 32 octets, conservée d'une réinstallation à l'autre ; la perdre
	# rend les blobs chiffrés illisibles.
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

# ------------------------------------------------------------------ maia-llm et mini modèle (A-026)

# cle_publique_graine : clé publique Ed25519 (hex) de la graine brute de 32 octets lue sur stdin
# (format miyauth), calculée par openssl dans un dossier privé de root.
cle_publique_graine() {
	local d pub
	d=$(mktemp -d /run/miyukini-cle.XXXXXX)
	chmod 0700 "$d"
	head -c 33 >"$d/graine"
	if [[ $(stat -c %s "$d/graine") != 32 ]]; then
		rm -rf "$d"
		die "graine de 32 octets attendue"
	fi
	{ printf '\x30\x2e\x02\x01\x00\x30\x05\x06\x03\x2b\x65\x70\x04\x22\x04\x20'; cat "$d/graine"; } >"$d/k.der"
	pub=$(openssl pkey -inform DER -in "$d/k.der" -pubout -outform DER | tail -c 32 | od -An -tx1 | tr -d ' \n')
	rm -rf "$d"
	[[ $pub =~ ^[0-9a-f]{64}$ ]] || die "clé publique non calculable"
	printf '%s' "$pub"
}

# cle_maia : graine de la clé de service de MAIA ($VAR/maia/keys/maia.seed, 32 octets, 0600,
# compte miyukini-maia), créée une fois pour que maia-llm puisse épingler sa clé publique avant le
# premier démarrage de MAIA. Jamais écrasée. Dossier et graine créés EN TANT QUE miyukini-maia (le
# dossier lui appartient) : un lien à la place du dossier ou de la graine fait échouer, root n'y
# écrit ni n'en change les droits ; contrôle final : fichier régulier de 32 octets, 0600, au compte.
cle_maia() {
	local d=$VAR/maia/keys f=$VAR/maia/keys/maia.seed
	refuser_liens "$VAR/maia"
	en_tant_que miyukini-maia sh -c 'set -e; umask 077; d=$1; f=$2
		[ ! -L "$d" ] || { echo "lien symbolique refusé : $d" >&2; exit 1; }
		mkdir -p -m 0700 "$d"; chmod 0700 "$d"
		[ ! -L "$f" ] || { echo "lien symbolique refusé : $f" >&2; exit 1; }
		if [ ! -e "$f" ]; then
			t=$(mktemp "$d/.graine.XXXXXX"); head -c 32 /dev/urandom >"$t"; mv -f "$t" "$f"; echo cree
		fi
		chmod 0600 "$f"' _ "$d" "$f" >"/run/miyukini-cle-maia.$$" ||
		{ rm -f "/run/miyukini-cle-maia.$$"; die "clé de service de MAIA : création refusée ($f)"; }
	grep -q cree "/run/miyukini-cle-maia.$$" && log "clé de service de MAIA créée ($f)"
	rm -f "/run/miyukini-cle-maia.$$"
	[[ ! -L $d && ! -L $f && $(LC_ALL=C stat -c '%s %F %U %a' "$f") == '32 regular file miyukini-maia 600' ]] ||
		die "$f : fichier régulier de 32 octets, 0600, au compte miyukini-maia attendu"
}

# configurer_maia_llm : /etc/miyukini/maia-llm.toml (root 0644) depuis config/maia/maia-llm.toml du
# paquet : clé de MAIA (inférence et modèles), clé d'Alicia si l'exploitant l'a fournie
# ($CLE_ALICIA_FICHIER), ancre des modèles, CPU imposé (binaire construit sans CUDA). Monté en
# lecture seule à l'endroit où maia-llm le lit ($VAR/maia-llm/maia-llm.toml, BindReadOnlyPaths).
# /etc/miyukini/alicia.toml : section [moteur_ia] d'Alicia (même moteur, boucle locale).
configurer_maia_llm() {
	local maia alicia="" inference
	au_manifeste config/maia/maia-llm.toml || die "config/maia/maia-llm.toml absent du MANIFESTE"
	# Graine lue EN TANT QUE miyukini-maia (jamais par root à travers un lien du compte).
	maia=$(en_tant_que miyukini-maia cat "$VAR/maia/keys/maia.seed" | cle_publique_graine) ||
		die "clé publique de MAIA non calculable"
	if [[ -f $CLE_ALICIA_FICHIER ]]; then
		alicia=$(tr -d ' \n' <"$CLE_ALICIA_FICHIER")
		[[ $alicia =~ ^([0-9a-f]{64}|[A-Za-z0-9_-]{43})$ ]] || die "$CLE_ALICIA_FICHIER : clé publique (hex ou base64 URL) attendue"
	fi
	inference="\"$maia\""
	[[ -n $alicia ]] && inference+=", \"$alicia\""
	[[ $ANCRE_MODELES =~ ^[0-9a-f]{64}$ ]] || die "ANCRE_MODELES invalide"
	sed -e "s|^appelants_inference = .*|appelants_inference = [$inference]|" \
		-e "s|^appelants_modeles = .*|appelants_modeles = [\"$maia\"]|" \
		-e "s|^model_trust_anchors = .*|model_trust_anchors = [\"$ANCRE_MODELES\"]|" \
		-e 's|^gpu_layers = .*|gpu_layers = 0|' \
		"$BUNDLE/config/maia/maia-llm.toml" | ecrire_fichier "$ETC/maia-llm.toml" 0644
	! grep -v '^[[:space:]]*#' "$ETC/maia-llm.toml" | grep 'À_REMPLACER' >/dev/null || die "maia-llm.toml : valeur à remplacer restante"
	poser_fichier_root "$VAR/maia-llm/maia-llm.toml" 0444 </dev/null
	ecrire_fichier "$ETC/alicia.toml" 0644 <<FIN
# Généré par l'installateur COG : moteur d'IA local d'Alicia (A-026), le même maia-llm que MAIA.
# Alicia signe ses requêtes avec sa clé (cle_voix) ; sa clé publique doit être épinglée par
# l'exploitant dans $CLE_ALICIA_FICHIER (puis réinstallation) pour être admise par maia-llm.
[moteur_ia]
url = "http://127.0.0.1:11436"
# CPU (VM, recette) : interprétation lente, délai large.
timeout_ms = 20000
FIN
	log "maia-llm : appelants épinglés (MAIA${alicia:+, Alicia}), ancre des modèles $ANCRE_MODELES (signature de développement), CPU"
}

# modele_installe : le composant est inscrit au canal et son fichier a l'empreinte attendue.
modele_installe() {
	local f=$MODELES/$MODELE_COMPOSANT.gguf
	[[ -f $MODELES/.canal-modeles.json && -f $f && -f $f.sceau && ! -L $f ]] || return 1
	[[ $(stat -c '%s %U %a' "$f") == "$MODELE_TAILLE root 644" ]] || return 1
	[[ $(sha256sum "$f" | cut -d' ' -f1) == "$MODELE_SHA256" ]]
}

# modele_ia : installe le mini modèle depuis l'artefact MMIA ($MODELE_ARTEFACT, hors paquet) par
# l'installateur du canal (maia-llm installer-modele : signature contre l'ancre épinglée, nom
# imposé, taille, licence, empreinte vérifiée AVANT toute écriture), puis revérifie l'empreinte
# du fichier installé contre celle consignée (A-026). Déjà installé à cette version : rien.
modele_ia() {
	if modele_installe; then
		log "modèle $MODELE_COMPOSANT déjà installé (SHA-256 $MODELE_SHA256 vérifié)"
		return 0
	fi
	if [[ -z $MODELE_ARTEFACT ]]; then
		log "ATTENTION : aucun modèle installé et aucun artefact MMIA fourni : maia-llm démarre sans modèle"
		return 0
	fi
	[[ -f $MODELE_ARTEFACT && ! -L $MODELE_ARTEFACT ]] || die "artefact MMIA introuvable : $MODELE_ARTEFACT"
	[[ $(head -c 4 "$MODELE_ARTEFACT") == MMIA ]] || die "$MODELE_ARTEFACT : pas un artefact MMIA"
	log "modèle : installation de l'artefact MMIA $(basename "$MODELE_ARTEFACT") par le canal (ancre $ANCRE_MODELES)"
	"$BIN/maia-llm" installer-modele --dossier "$MODELES" --ancre "$ANCRE_MODELES" \
		--composant "$MODELE_COMPOSANT" --version "$MODELE_VERSION" --artefact "$MODELE_ARTEFACT" ||
		die "artefact MMIA refusé par l'installateur du canal"
	chown -R -h root:root "$MODELES"
	chmod 0755 "$MODELES"
	modele_installe || die "modèle installé non conforme (SHA-256, taille ou droits)"
	log "modèle $MODELE_COMPOSANT installé : SHA-256 $MODELE_SHA256 conforme (A-026)"
}

# cle_sauvegarde_locale : clé aléatoire propre à la machine (0600, root), utilisée pour chiffrer
# les sauvegardes automatiques (préalables aux mises à jour, retour arrière). Jamais écrasée.
cle_sauvegarde_locale() {
	local f=$ETC/sauvegarde-locale.key
	if [[ ! -s $f ]]; then
		(umask 077 && openssl rand -hex 48 >"$f.tmp") && mv -f "$f.tmp" "$f"
	fi
	chown root:root "$f"
	chmod 0600 "$f"
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
	# B3 : le mot de passe ne doit apparaître dans aucun journal PostgreSQL : journalisation
	# coupée dans CETTE session, avant l'ALTER ROLE. Il transite par l'entrée standard, jamais argv.
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

# Profils du paquet : copiés dans /etc/apparmor.d et chargés en enforce. Un profil refusé arrête.
apparmor_profils() {
	log "AppArmor : profils"
	local f nom n=0
	while IFS= read -r f; do
		nom=${f##*/}
		installer_fichier 0644 "$f" "/etc/apparmor.d/$nom"
		apparmor_parser -r -W "/etc/apparmor.d/$nom" || die "profil AppArmor $nom refusé par apparmor_parser"
		log "  profil $nom chargé"
		n=$((n + 1))
	done < <(fichiers_du_manifeste '^apparmor/miyukini-[^/]+$')
	((n > 0)) || die "aucun profil apparmor/miyukini-* au MANIFESTE"
}

# ------------------------------------------------------------------ Douane : zone COG

# Plan des essais : mêmes sandboxes, même ordre, que le banc (tests/intrusion-lot3/scenario.sh) et
# douane-externe.sh (l'ordre fixe les adresses : identique pour toutes les commandes).
SANDBOXES_PLAN=(--sandbox cog:voie-a --sandbox cog:voie-b --sandbox franche:voie-a
	--sandbox franche:voie-b --sandbox cog:capteur --sandbox locale:capteur)

# arretes_essai : exemple du banc (clé jetable). Emplacement dans le paquet : banc/cible/examples
# (VM) ou bin/ (recette).
chemin_arretes_essai() {
	local rel
	for rel in banc/cible/examples/arretes_essai bin/arretes_essai; do
		if au_manifeste "$rel" && [[ -f $BUNDLE/$rel && ! -L $BUNDLE/$rel && -x $BUNDLE/$rel ]]; then
			printf '%s' "$BUNDLE/$rel"
			return 0
		fi
	done
	die "arretes_essai absent du paquet (ou non listé au MANIFESTE)"
}

# Arrêtés d'essai : générés par arretes_essai (clé jetable, jamais conservée), vérifiés par
# miyudouane (signature, constitution, vigueur) ; la zone COG doit se compiler depuis eux. Le
# texte nftables de `miyudouane zone` est un rendu pour espaces de noms : la table hôte est
# écrite par douane_cog. Les arrêtés sont valides un jour (pas_apres) : vérifiés à l'installation.
douane_arretes() {
	log "Douane : arrêtés d'essai et vérification"
	local ex dm=$BIN/miyudouane maint ancre f
	ex=$(chemin_arretes_essai)
	# Plan des essais : mêmes sandboxes que le banc (tests/intrusion-lot3/scenario.sh).
	local sb=("${SANDBOXES_PLAN[@]}")
	install -d -o root -g root -m 0700 "$DOUANE_VAR"
	rm -rf "$DOUANE_VAR/arretes"
	install -d -o root -g root -m 0700 "$DOUANE_VAR/arretes"
	maint=$(date +%s)
	"$ex" --sortie "$DOUANE_VAR/arretes" --maintenant "$maint" >/dev/null ||
		die "arretes_essai : échec de génération"
	ancre=$(tr -d '\n' <"$DOUANE_VAR/arretes/ancre.hex")
	[[ $ancre =~ ^[0-9a-f]{64}$ ]] || die "ancre d'essai invalide"
	for f in externe portail-interne portail-local; do
		"$dm" regles --frontiere "$f" --arrete "$DOUANE_VAR/arretes/$f.arrete" --ancre "$ancre" \
			--maintenant "$maint" "${sb[@]}" >/dev/null || die "miyudouane regles : arrêté $f refusé"
	done
	"$dm" zone --zone cog --arrete "portail-interne=$DOUANE_VAR/arretes/portail-interne.arrete" \
		--arrete "portail-local=$DOUANE_VAR/arretes/portail-local.arrete" \
		--ancre "$ancre" --maintenant "$maint" "${sb[@]}" >/dev/null || die "miyudouane zone cog : compilation refusée"
	log "  arrêtés externe, portail-interne, portail-local : signature et constitution vérifiées ; zone COG compilée"
}

# Table hôte « miyudouane_hote » : générée par la Douane (miyudouane hote), jamais écrite à la main.
# L'installateur écrit la DESCRIPTION (format miyudouane-hote 2, en-tête de crates/miyudouane/src/
# hote.rs : comptes de service, ports COG, flux de boucle locale PAR COMPTE), la fait auditer
# (rendu sans --appliquer), puis l'applique : remplacement atomique de la seule table
# inet miyudouane_hote, aucun « flush ruleset ». Le fichier rendu sert au rechargement (unité Douane).
# Boucle locale (M3) : un compte COG n'ouvre que les flux déclarés par « boucle » ; les réponses
# de ses propres ports passent par la règle « established » de miyudouane. Tout le reste (résolveur
# 127.0.0.53, autre port local, autre compte) est rejeté et compté (compteur « rejets »).
DESCRIPTION_HOTE=$ETC/douane-hote.desc
# <compte> <proto> <adresse> <port> : flux de boucle locale admis.
BOUCLES_COG=(
	"miyukini-maia tcp 127.0.0.1 50051"      # MAIA (et son unité d'attente) -> KindMother
	"miyukini-maia tcp 127.0.0.1 11436"      # MAIA -> maia-llm (A-026)
	"miyukini-miyucloud tcp 127.0.0.1 5432"  # MiyuCloud -> PostgreSQL
)
# Alicia (A-026) : si elle tourne sous un compte COG (miyukini-alicia) sur cette machine, ce compte
# est soumis à la zone et n'a qu'un flux local : maia-llm. Sinon (Alicia dans Central, hors de ces
# installateurs) : aucune directive.
COMPTE_ALICIA=miyukini-alicia
douane_cog() {
	log "Douane : zone COG (table nftables miyudouane_hote, générée par miyudouane hote)"
	local u p b comptes=("${UTILISATEURS[@]}") boucles=("${BOUCLES_COG[@]}")
	if getent passwd "$COMPTE_ALICIA" >/dev/null; then
		comptes+=("$COMPTE_ALICIA")
		boucles+=("$COMPTE_ALICIA tcp 127.0.0.1 11436")
		log "  compte $COMPTE_ALICIA présent : soumis à la zone, flux local vers maia-llm seulement"
	fi
	{
		echo "# Genere par l'installateur COG : description de la zone COG sur l'hote (miyudouane-hote 2)."
		echo "miyudouane-hote 2"
		echo "groupe 110"
		for u in "${comptes[@]}"; do
			echo "compte $u"
		done
		for p in "${PORTS_COG_TCP[@]}"; do
			echo "port tcp $p"
		done
		for b in "${boucles[@]}"; do
			echo "boucle $b"
		done
	} >"$DESCRIPTION_HOTE.tmp"
	chmod 0644 "$DESCRIPTION_HOTE.tmp"
	chown root:root "$DESCRIPTION_HOTE.tmp"
	mv -f "$DESCRIPTION_HOTE.tmp" "$DESCRIPTION_HOTE"
	"$BIN/miyudouane" hote --description "$DESCRIPTION_HOTE" >/dev/null || die "miyudouane hote : description refusée ou audit en échec"
	"$BIN/miyudouane" hote --description "$DESCRIPTION_HOTE" --appliquer --fichier "$DOUANE_NFT" ||
		die "miyudouane hote --appliquer : table non chargée"
	nft list table inet miyudouane_hote >/dev/null || die "table miyudouane_hote absente après application"
}

# ecrire_douane_externe : texte du script de la frontière externe (stdout). Il vit ici, dans
# commun.sh, pour être couvert par le MANIFESTE (et la signature du paquet en recette) sans fichier
# de paquet supplémentaire. Aucune expansion dans ce texte (délimiteur entre apostrophes).
ecrire_douane_externe() {
	cat <<'FIN_DOUANE_EXTERNE'
#!/usr/bin/env bash
# Frontière EXTERNE de la Douane : espace de noms mcog-f-externe, règles nftables, mise en forme tc.
# Écrit par commun.sh (ecrire_douane_externe) dans /opt/miyukini/libexec/douane-externe.sh (root,
# 0755), donc couvert par le MANIFESTE avec commun.sh ; lancé par
# l'unité miyukini-douane-externe.service (démarrage de l'espace) et par l'exploitant (arrêté
# externe, coupure ou rétablissement de voie). Root seulement.
#
#   douane-externe.sh demarrer          crée l'espace (rendu `miyudouane mise-en-place`, filtré à la
#                                       frontière externe et au pont Franche), puis appliquer
#   douane-externe.sh appliquer         (a) règles strictes → (b) tc → (c) filet
#   douane-externe.sh couper <voie>     voie omise (EX-49), puis appliquer
#   douane-externe.sh retablir <voie>   voie rétablie, puis appliquer
#   douane-externe.sh arrete <fichier>  nouvel arrêté externe (vérifié avant d'être retenu), puis appliquer
#   douane-externe.sh arreter           retire les espaces mcog-f-externe et mcog-z-franche
#   douane-externe.sh etat              état courant (strict | tc | filet | ferme)
#
# Ordre sûr (A-023, corrigé par A-025 ; en-tête de crates/miyudouane/src/application.rs), chaque
# étape SEULEMENT si la précédente a réussi :
#   (a) miyudouane regles --frontiere externe … --appliquer          règle DEBIT stricte
#   (b) miyudouane tc --description … --appliquer                    classes HTB, fq_codel, filtre fw
#   (c) miyudouane regles … --mise-en-forme-appliquee --appliquer    filet 5/4 par sens
# Échec de (a) : fermeture de la frontière (miyudouane fermeture), code 1. Échec de (b) : racine tc
# retirée par miyudouane, règle stricte maintenue, code 3. Échec de (c) : règle stricte et tc en
# place, code 4. Tout le texte appliqué est généré et audité par miyudouane, jamais écrit ici.
#
# Interfaces : ext-franche (lien veth vers le pont de la zone Franche) est créée par le rendu de la
# Douane. ext0 (carte physique) et wg-fed (WireGuard fédéral) : si l'opérateur fournit
# MCOG_IF_PHYSIQUE, le rendu y déplace la carte et la renomme ext0 ; sinon (VM, recette) ext0 et
# wg-fed sont des interfaces factices (dummy) qui portent la même mise en forme.
#
# Fichiers : arrêté et ancre dans /var/lib/miyukini/douane/arretes (root 0700), état et rendus
# dans /var/lib/miyukini/douane/externe (root 0700), description des capacités dans
# /etc/miyukini/douane-tc.desc (miyudouane-tc 1). Jamais /etc/miyukini/douane (réservé au banc).
set -euo pipefail
umask 0077
export LC_ALL=C

BIN=${COG_BIN:-/opt/miyukini/bin}
DM=$BIN/miyudouane
VAR=/var/lib/miyukini/douane
ARRETES=$VAR/arretes
ARRETE=$ARRETES/externe.arrete
ANCRE_F=$ARRETES/ancre.hex
DIR=$VAR/externe
COUPEES=$DIR/coupees
ETAT=$DIR/etat
DESC=/etc/miyukini/douane-tc.desc
NS=mcog-f-externe
NS_FRANCHE=mcog-z-franche
NFT=$DIR/$NS.nft
TC=$DIR/$NS.tc
# Plan des essais : mêmes sandboxes, dans le même ordre, que douane_arretes (commun.sh) et le banc.
SB=(--sandbox cog:voie-a --sandbox cog:voie-b --sandbox franche:voie-a
	--sandbox franche:voie-b --sandbox cog:capteur --sandbox locale:capteur)
MODULES=(sch_htb sch_fq_codel cls_fw nf_conntrack)

journal() {
	printf '[douane-externe] %s\n' "$*"
	logger -t miyukini-douane-externe -- "$*" 2>/dev/null || true
}
die() {
	journal "ERREUR: $*"
	exit "${2:-1}"
}
etat() { printf '%s %s\n' "$1" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >"$ETAT.tmp" && mv -f "$ETAT.tmp" "$ETAT"; }

((EUID == 0)) || die "root requis"
[[ -x $DM ]] || die "$DM absent"
install -d -o root -g root -m 0700 "$VAR" "$DIR"

ancre() {
	local a
	[[ -f $ANCRE_F && -f $ARRETE ]] || die "arrêté externe ou ancre absents ($ARRETES)"
	a=$(tr -d '\n' <"$ANCRE_F")
	[[ $a =~ ^[0-9a-f]{64}$ ]] || die "ancre invalide ($ANCRE_F)"
	printf '%s' "$a"
}

# Voies coupées (une par ligne, identifiants sûrs seulement).
couper_args() {
	local v
	[[ -f $COUPEES ]] || return 0
	while IFS= read -r v; do
		[[ $v =~ ^[a-z0-9][a-z0-9-]{0,31}$ ]] || die "voie coupée invalide dans $COUPEES : $v"
		printf -- '--couper\n%s\n' "$v"
	done <"$COUPEES"
}

espace_present() { ip netns list 2>/dev/null | awk '{print $1}' | grep -x "$1" >/dev/null; }

arreter() {
	local ns
	for ns in "$NS" "$NS_FRANCHE"; do
		if espace_present "$ns"; then ip netns del "$ns"; fi
	done
	rm -f "$NFT" "$TC"
	etat arrete
	journal "espaces $NS et $NS_FRANCHE retirés"
}

demarrer() {
	local m script
	for m in "${MODULES[@]}"; do
		modprobe "$m" || die "module noyau $m indisponible"
	done
	# Démarrage de l'espace : toujours à neuf (aucun état tc ou nftables hérité).
	arreter >/dev/null
	# Rendu de la Douane (texte seulement), filtré : lignes qui ne nomment QUE la frontière externe
	# et le pont de la zone Franche ; une interface n'est rattachée au pont que si elle vient d'un
	# lien retenu. Étapes 4 (sandboxes) et 6 (chargement depuis /etc/miyukini/douane) exclues : les
	# règles sont appliquées ci-dessous dans l'ordre sûr.
	script=$("$DM" mise-en-place "${SB[@]}" | awk -v a="$NS" -v b="$NS_FRANCHE" '
		/^# [0-9]+\./ { etape = $2 }
		(etape == "4." || etape == "6.") && !/^#/ { next }
		{
			l = $0; ok = 1
			while (match(l, /mcog-[a-z0-9-]+/)) {
				n = substr(l, RSTART, RLENGTH)
				if (n != a && n != b) { ok = 0; break }
				l = substr(l, RSTART + RLENGTH)
			}
			if (!ok) next
			if (match($0, / peer name [^ ]+ /)) { p = substr($0, RSTART + 11, RLENGTH - 12); pair[p] = 1 }
			if ($0 ~ / master br0 /) {
				for (i = 1; i < NF; i++) if ($i == "set") { if (!($(i + 1) in pair)) next; break }
			}
			print
		}')
	grep -q "ip netns add $NS\$" <<<"$script" || die "rendu de mise en place sans $NS"
	sh -c "$script" || die "mise en place de $NS refusée"
	# Interfaces extérieures absentes (ni carte physique ni WireGuard) : interfaces factices.
	for m in ext0 wg-fed; do
		if ! ip -n "$NS" link show "$m" >/dev/null 2>&1; then
			ip -n "$NS" link add "$m" type dummy
			ip -n "$NS" link set "$m" up
			journal "interface $m : factice (dummy) dans $NS"
		fi
	done
	journal "espace $NS prêt (ext-franche, ext0, wg-fed)"
	appliquer
}

fermer() {
	"$DM" fermeture --frontiere externe "${SB[@]}" --appliquer --fichier "$NFT" ||
		journal "ERREUR: fermeture de la frontière externe refusée"
	etat ferme
}

appliquer() {
	local a coupe=()
	espace_present "$NS" || die "espace $NS absent (démarrer d'abord)"
	[[ -f $DESC ]] || die "description tc absente ($DESC)"
	a=$(ancre)
	mapfile -t coupe < <(couper_args)
	# (a) règles strictes.
	if ! "$DM" regles --frontiere externe --arrete "$ARRETE" --ancre "$a" "${SB[@]}" "${coupe[@]}" \
		--appliquer --fichier "$NFT"; then
		journal "(a) règles strictes refusées : frontière externe fermée"
		fermer
		exit 1
	fi
	etat strict
	journal "(a) règles strictes appliquées (DEBIT strict)${coupe[*]:+ ; ${coupe[*]}}"
	# (b) mise en forme, seulement après (a).
	if ! "$DM" tc --description "$DESC" --arrete "$ARRETE" --ancre "$a" "${SB[@]}" "${coupe[@]}" \
		--appliquer --fichier "$TC"; then
		journal "(b) tc refusé : règles strictes maintenues, aucun filet"
		exit 3
	fi
	etat tc
	journal "(b) tc appliqué (racine htb 4d44:, audité)"
	# (c) filet, seulement après (b).
	if ! "$DM" regles --frontiere externe --arrete "$ARRETE" --ancre "$a" "${SB[@]}" "${coupe[@]}" \
		--mise-en-forme-appliquee --appliquer --fichier "$NFT"; then
		journal "(c) filet refusé : règles strictes et tc maintenus"
		exit 4
	fi
	etat filet
	journal "(c) filet appliqué (DEBIT 5/4 par sens, après tc)"
}

case ${1:-} in
	demarrer) demarrer ;;
	appliquer) appliquer ;;
	arreter) arreter ;;
	couper | retablir)
		v=${2:-}
		[[ $v =~ ^[a-z0-9][a-z0-9-]{0,31}$ ]] || die "voie attendue"
		touch "$COUPEES"
		if [[ $1 == couper ]]; then
			grep -qxF "$v" "$COUPEES" || printf '%s\n' "$v" >>"$COUPEES"
			journal "voie $v coupée"
		else
			grep -vxF "$v" "$COUPEES" >"$COUPEES.tmp" || true
			mv -f "$COUPEES.tmp" "$COUPEES"
			journal "voie $v rétablie"
		fi
		appliquer
		;;
	arrete)
		f=${2:-}
		[[ -f $f && ! -L $f ]] || die "arrêté attendu (fichier régulier)"
		a=$(ancre)
		# Vérifié (signature, constitution, vigueur, frontière) AVANT d'être retenu.
		"$DM" regles --frontiere externe --arrete "$f" --ancre "$a" "${SB[@]}" >/dev/null ||
			die "arrêté externe refusé : l'arrêté en place est conservé"
		install -o root -g root -m 0600 "$f" "$ARRETE.tmp"
		mv -f "$ARRETE.tmp" "$ARRETE"
		journal "nouvel arrêté externe retenu"
		appliquer
		;;
	etat) cat "$ETAT" 2>/dev/null || echo inconnu ;;
	*) sed -n '2,13p' "$0"; exit 2 ;;
esac
FIN_DOUANE_EXTERNE
}

# douane_externe : frontière externe (A-023, A-025). Description des capacités (miyudouane-tc 1,
# ASCII, auditée par miyudouane tc), script douane-externe.sh (libexec), modules tc chargés au
# démarrage. L'unité miyukini-douane-externe crée l'espace mcog-f-externe et applique (a) règles
# strictes → (b) tc → (c) filet, chaque étape seulement si la précédente a réussi ; elle est
# (re)lancée par demarrage() à chaque installation, donc à chaque nouvel arrêté externe
# (douane_arretes en génère un à chaque installation).
douane_externe() {
	log "Douane : frontière externe (mcog-f-externe : règles strictes, tc, filet)"
	local c
	install -d -o root -g root -m 0755 "$LIBEXEC"
	ecrire_douane_externe | ecrire_fichier "$LIBEXEC/douane-externe.sh" 0755
	{
		echo "# Genere par l'installateur COG : capacites des interfaces de la frontiere externe (Kio/s)."
		echo "miyudouane-tc 1"
		for c in "${CAPACITES_TC[@]}"; do
			echo "capacite $c"
		done
	} | ecrire_fichier "$DESCRIPTION_TC" 0644
	printf '%s\n' sch_htb sch_fq_codel cls_fw | ecrire_fichier /etc/modules-load.d/miyukini-douane.conf 0644
	# Audit de la description et du rendu avant toute application (aucun effet).
	"$BIN/miyudouane" tc --description "$DESCRIPTION_TC" --arrete "$DOUANE_VAR/arretes/externe.arrete" \
		--ancre "$(tr -d '\n' <"$DOUANE_VAR/arretes/ancre.hex")" "${SANDBOXES_PLAN[@]}" >/dev/null ||
		die "miyudouane tc : description ou rendu refusé"
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

# ecrire_unite <nom> <description> <utilisateur> <répertoire> <commande> <chemins-rw> <après> <env-fichier> <env-lignes> <profil-apparmor> [<lignes-finales>] [<lignes-[Unit]>]
# Toute unité dépend de la zone COG de la Douane (Requires= : pas de service sans zone).
# <lignes-finales> : écrites après le durcissement commun (elles l'emportent, ex. UMask=).
ecrire_unite() {
	local nom=$1 description=$2 utilisateur=$3 repertoire=$4 commande=$5 rw=$6 apres=$7 fichier_env=$8 env_lignes=$9
	local profil=${10:-} finales=${11:-} unite=${12:-}
	local envfile="" rwligne="" profligne=""
	[[ -n $fichier_env ]] && envfile="EnvironmentFile=-$fichier_env"
	[[ -n $rw ]] && rwligne="ReadWritePaths=$rw"
	[[ -n $profil ]] && profligne="AppArmorProfile=$profil"
	cat >"$UNITES_DIR/$nom.service" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=$description
After=network-online.target $DOUANE_UNITE $apres
Wants=network-online.target
Requires=$DOUANE_UNITE
BindsTo=$DOUANE_UNITE
$unite

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
$finales

[Install]
WantedBy=multi-user.target
FIN
	chmod 0644 "$UNITES_DIR/$nom.service"
}

# generer_unites <répertoire> : écrit toutes les unités COG (utilisé aussi à la construction du
# paquet, pour qu'elles soient couvertes par le MANIFESTE).
generer_unites() {
	local UNITES_DIR=$1
	mkdir -p "$UNITES_DIR"

	cat >"$UNITES_DIR/$DOUANE_UNITE" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=Douane — zone COG (table nftables miyudouane_hote)
# nftables.service (Ubuntu) fait un « flush ruleset » au redémarrage : la Douane est liée à lui.
# Un redémarrage de nftables redémarre la Douane (la table est recréée) ; les services COG sont
# liés à la Douane (BindsTo=) et s'arrêtent avec elle : jamais de service sans zone.
After=nftables.service
PartOf=nftables.service
Before=miyukini-kindmother.service miyukini-maia-llm.service miyukini-maia.service miyukini-miyucloud.service miyukini-cog-web-portal.service miyukini-portail-entree.service miyukini-portail-sortie.service

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=/usr/sbin/nft -f $DOUANE_NFT
ExecReload=/usr/sbin/nft -f $DOUANE_NFT
ExecStop=/usr/sbin/nft delete table inet miyudouane_hote

[Install]
WantedBy=multi-user.target
FIN
	chmod 0644 "$UNITES_DIR/$DOUANE_UNITE"

	# Contrôle périodique de la zone (fail-closed) : si la table miyudouane_hote a disparu (flush
	# externe, nft à la main), elle est rechargée ; si elle ne revient pas, les services COG sont arrêtés.
	cat >"$UNITES_DIR/miyukini-douane-cog-controle.service" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=Contrôle de la zone COG (table miyudouane_hote, sinon rechargement ou arrêt des services COG)
After=$DOUANE_UNITE
Requires=$DOUANE_UNITE

[Service]
Type=oneshot
ExecStart=/bin/bash -c 'if nft list table inet miyudouane_hote >/dev/null 2>&1; then exit 0; fi; logger -t miyukini-douane "zone COG absente : rechargement"; if nft -f $DOUANE_NFT && nft list table inet miyudouane_hote >/dev/null 2>&1; then exit 0; fi; logger -t miyukini-douane "zone COG non rétablie : arrêt des services COG"; systemctl stop miyukini-portail-sortie miyukini-portail-entree miyukini-cog-web-portal miyukini-miyucloud miyukini-maia miyukini-maia-llm miyukini-kindmother; exit 1'
FIN
	chmod 0644 "$UNITES_DIR/miyukini-douane-cog-controle.service"
	cat >"$UNITES_DIR/miyukini-douane-cog-controle.timer" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=Minuteur du contrôle de la zone COG

[Timer]
OnBootSec=30s
OnUnitActiveSec=60s
AccuracySec=1s

[Install]
WantedBy=timers.target
FIN
	chmod 0644 "$UNITES_DIR/miyukini-douane-cog-controle.timer"

	# Frontière externe (A-023, A-025) : espace mcog-f-externe créé au démarrage, puis (a) règles
	# strictes → (b) tc → (c) filet. Reload = réapplication (nouvel arrêté, coupure, rétablissement :
	# voir douane-externe.sh). Indépendante des services de la zone COG.
	cat >"$UNITES_DIR/$EXTERNE_UNITE" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=Douane — frontière externe (mcog-f-externe : règles strictes, tc, filet)
After=network-online.target systemd-modules-load.service
Wants=network-online.target

[Service]
Type=oneshot
RemainAfterExit=yes
ExecStart=$LIBEXEC/douane-externe.sh demarrer
ExecReload=$LIBEXEC/douane-externe.sh appliquer
ExecStop=$LIBEXEC/douane-externe.sh arreter
TimeoutStartSec=60

[Install]
WantedBy=multi-user.target
FIN
	chmod 0644 "$UNITES_DIR/$EXTERNE_UNITE"

	ecrire_unite miyukini-kindmother "KindMother — service libSQL isolé (COG)" \
		miyukini-kindmother "$VAR/kindmother" "$BIN/kindmother-server" "$VAR/kindmother" "" "" \
		"Environment=KINDMOTHER_DATA_DIR=$VAR/kindmother
Environment=KINDMOTHER_LISTEN_ADDR=127.0.0.1:50051" "miyukini-kindmother"

	# MAIA ouvre la mémoire (KindMother) une seule fois, au démarrage, sans nouvel essai : elle
	# démarre après KindMother via une unité d'attente distincte, SANS profil AppArmor (le profil de
	# MAIA interdit toute exécution ; AppArmorProfile= s'appliquerait à chaque commande de l'unité).
	cat >"$UNITES_DIR/$ATTENTE_UNITE.service" <<FIN
# Généré par l'installateur COG : ne pas éditer à la main.
[Unit]
Description=Attente de KindMother (127.0.0.1:50051) avant MAIA
After=miyukini-kindmother.service
# Relancée avec KindMother : sans cela (RemainAfterExit), un redémarrage de KindMother laissait
# l'attente « active » et MAIA repartait avant l'écoute de 50051 (mémoire non ouverte).
PartOf=miyukini-kindmother.service

[Service]
Type=oneshot
RemainAfterExit=yes
User=miyukini-maia
Group=miyukini-maia
ExecStart=-/usr/bin/timeout 30 /bin/bash -c 'until (exec 3<>/dev/tcp/127.0.0.1/50051) 2>/dev/null; do sleep 0.5; done'

[Install]
RequiredBy=miyukini-maia.service
FIN
	chmod 0644 "$UNITES_DIR/$ATTENTE_UNITE.service"

	# bridge.toml (B2) : /etc/miyukini/bridge.toml (root, lecture seule) monté en lecture seule à
	# l'endroit où MAIA le lit ; ni réécriture ni remplacement.
	# maia-llm (A-026) : moteur llama.cpp de MAIA et d'Alicia, 127.0.0.1:11436, CPU (binaire sans
	# CUDA : aucun périphérique). Configuration et modèles en lecture seule ; aucun flux sortant
	# (compte de la zone COG sans directive « boucle »).
	ecrire_unite miyukini-maia-llm "maia-llm — moteur d'IA local llama.cpp (COG)" \
		miyukini-maia-llm "$VAR/maia-llm" "$BIN/maia-llm" "" "" "" \
		"Environment=MAIA_DATA_DIR=$VAR/maia-llm
Environment=RUST_LOG=maia_llm=info
BindReadOnlyPaths=$ETC/maia-llm.toml:$VAR/maia-llm/maia-llm.toml
ReadOnlyPaths=$MODELES
MemoryMax=3G" "miyukini-maia-llm" "TimeoutStartSec=120"

	# MAIA_LLM_URL : maia-llm (11436) ; 11435 est le port de MAIA elle-même.
	ecrire_unite miyukini-maia "MAIA — orchestrateur d'agents IA (COG)" \
		miyukini-maia "$VAR/maia" "$BIN/maia" "$VAR/maia" "miyukini-kindmother.service $ATTENTE_UNITE.service miyukini-maia-llm.service" "$ETC/maia.env" \
		"Environment=MAIA_LLM_URL=http://127.0.0.1:11436
BindReadOnlyPaths=$ETC/bridge.toml:$VAR/maia/bridge.toml" "miyukini-maia" "" "Wants=miyukini-maia-llm.service"

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

	# Portail scindé (A-022 §3) : deux unités INDÉPENDANTES (aucune ne requiert l'autre : la panne de
	# l'une n'arrête pas l'autre), sans réseau, chacune aveugle aux files, à l'état et à la
	# configuration de l'autre sens (InaccessiblePaths, en plus des droits et d'AppArmor).
	local pf=$PORTAIL_FILES
	ecrire_unite miyukini-portail-entree "Portail — entrée (Franche → zone COG ; relève, ne publie jamais)" \
		miyukini-portail-entree "$VAR/portail-entree" \
		"$BIN/portail-entree --voies $ETC/portail-entree/voies --etat $VAR/portail-entree --depot $pf/depot --admises $pf/admises --interdit $pf/reponses --interdit $pf/publication --sas $BIN/portail-sas" \
		"$VAR/portail-entree $pf/depot $pf/admises" "" "" \
		"PrivateNetwork=yes
IPAddressDeny=any
InaccessiblePaths=-$pf/reponses -$pf/publication -$VAR/portail-sortie -$ETC/portail-sortie
# Sas de décodage (lancé par lot) : Landlock et seccomp (groupe @sandbox).
SystemCallFilter=@sandbox" "miyukini-portail-entree" "UMask=0027"
	ecrire_unite miyukini-portail-sortie "Portail — sortie (zone COG → Franche ; réponses à jeton et H1, ne relève jamais)" \
		miyukini-portail-sortie "$VAR/portail-sortie" \
		"$BIN/portail-sortie --voies $ETC/portail-sortie/voies --etat $VAR/portail-sortie --reponses $pf/reponses --publication $pf/publication --interdit $pf/depot --interdit $pf/admises" \
		"$VAR/portail-sortie $pf/reponses $pf/publication" "" "" \
		"PrivateNetwork=yes
IPAddressDeny=any
InaccessiblePaths=-$pf/depot -$pf/admises -$VAR/portail-entree -$ETC/portail-entree" "miyukini-portail-sortie" "UMask=0027"
}

# unites : unités du paquet (systemd/, si présentes) sinon générées sur place.
unites() {
	local f liste
	liste=$(fichiers_du_manifeste '^systemd/[^/]+\.(service|timer)$')
	if [[ -n $liste ]]; then
		log "unités systemd (depuis le paquet, fichiers listés au MANIFESTE)"
		while IFS= read -r f; do
			installer_fichier 0644 "$f" "$UNITES_DIR/${f##*/}"
		done <<<"$liste"
	else
		log "unités systemd (générées)"
		generer_unites "$UNITES_DIR"
	fi
	systemctl daemon-reload
}

# ------------------------------------------------------------------ démarrage et contrôle de santé

demarrage() {
	log "démarrage des services"
	local u
	systemctl enable "$DOUANE_UNITE" "$ATTENTE_UNITE.service" >/dev/null
	systemctl restart "$DOUANE_UNITE"
	systemctl enable --now miyukini-douane-cog-controle.timer >/dev/null
	# Frontière externe : redémarrée à chaque installation (nouvel arrêté externe) ; un échec de (b)
	# ou (c) laisse la règle stricte, signalé ici sans arrêter l'installation des services.
	systemctl enable "$EXTERNE_UNITE" >/dev/null
	systemctl restart "$EXTERNE_UNITE" || log "ATTENTION : frontière externe : $("$LIBEXEC/douane-externe.sh" etat 2>/dev/null)"
	for u in "${SERVICES[@]}"; do
		systemctl enable "$u.service" >/dev/null
		systemctl restart "$u.service"
	done
	sleep 3
	for u in "$DOUANE_UNITE" "$EXTERNE_UNITE" "${SERVICES[@]}"; do
		printf '[%s] %-28s %s\n' "$LOG_TAG" "$u" "$(systemctl is-active "$u" || true)"
	done
}

arreter_services() {
	local i
	for ((i = ${#SERVICES[@]} - 1; i >= 0; i--)); do
		systemctl stop "${SERVICES[i]}" 2>/dev/null || true
	done
}

# sonde_port <port> : un service écoute sur 127.0.0.1:<port> (et seulement là).
sonde_port() {
	ss -ltnH | awk '{print $4}' | grep -x "127.0.0.1:$1" >/dev/null
}

# controle_sante : attend (borné par COG_DELAI_SANTE, 120 s par défaut) que les unités soient
# actives, que leurs ports écoutent sur loopback et que leurs sondes HTTP répondent. Code 1 si
# un contrôle échoue. COG_ECHEC_SANTE_INJECTE=1 force l'échec (essai de retour arrière seulement ;
# l'appelant ne le transmet qu'avec --confirme-machine-de-recette).
controle_sante() {
	local delai=${COG_DELAI_SANTE:-120} ecoule=0 echec u code
	if [[ -n ${COG_ECHEC_SANTE_INJECTE:-} ]]; then
		log "contrôle de santé : ÉCHEC INJECTÉ (essai de retour arrière)"
		return 1
	fi
	while :; do
		echec=""
		for u in "${SERVICES[@]}"; do
			systemctl is-active --quiet "$u" || echec+="$u inactif ; "
		done
		sonde_port 50051 || echec+="KindMother n'écoute pas 127.0.0.1:50051 ; "
		sonde_port 11435 || echec+="MAIA n'écoute pas 127.0.0.1:11435 ; "
		sonde_port 11436 || echec+="maia-llm n'écoute pas 127.0.0.1:11436 ; "
		sonde_port 8086 || echec+="MiyuCloud n'écoute pas 127.0.0.1:8086 ; "
		sonde_port 8090 || echec+="portail n'écoute pas 127.0.0.1:8090 ; "
		if [[ -z $echec ]]; then
			curl -fsS -o /dev/null --max-time 5 http://127.0.0.1:8086/api/auth/status || echec+="MiyuCloud /api/auth/status ; "
			curl -fsS -o /dev/null --max-time 5 http://127.0.0.1:11435/health || echec+="MAIA /health ; "
			curl -fsS -o /dev/null --max-time 5 http://127.0.0.1:11436/maia/llm/health || echec+="maia-llm /maia/llm/health ; "
			code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 5 http://127.0.0.1:8090/ || true)
			[[ $code =~ ^[1-4][0-9][0-9]$ ]] || echec+="portail HTTP $code ; "
		fi
		if [[ -z $echec ]]; then
			log "contrôle de santé : tous les services répondent"
			return 0
		fi
		if ((ecoule >= delai)); then
			log "contrôle de santé en échec : $echec"
			return 1
		fi
		sleep 3
		ecoule=$((ecoule + 3))
	done
}

# ------------------------------------------------------------------ exploitation

# installer_exploitation : scripts d'exploitation et commun.sh (depuis le paquet, vérifié).
installer_exploitation() {
	install -d -o root -g root -m 0755 "$EXPLOITATION"
	local f
	while IFS= read -r f; do
		installer_fichier 0755 "$f" "$EXPLOITATION/${f##*/}"
	done < <(fichiers_du_manifeste '^exploitation/[^/]+\.sh$')
	installer_fichier 0644 commun.sh "$EXPLOITATION/commun.sh"
	if au_manifeste VERSION; then
		installer_fichier 0644 VERSION "$PREFIXE/VERSION"
	fi
}
