#!/usr/bin/env bash
# Banc d'intrusion local du lot 3 (critère de sortie, PLAN-IMPLEMENTATION §5) :
# « aucun flux non déclaré ne traverse ; coupure d'une voie sans impact sur
# les autres (EX-49) ».
#
# À exécuter en root sur une machine virtuelle JETABLE dédiée. Jamais sur un
# hôte de production : le banc crée des espaces de noms, charge des modules
# noyau et écrit dans /etc/miyukini/douane. Voir README.md.
#
#   ./lancer.sh --dry-run                        # construit, génère, imprime les commandes
#   sudo ./lancer.sh --vm-jetable --sans-construction
#
# Sortie : TAP (ok / not ok), code de retour 0 si tout passe, 1 sinon,
# 2 si le banc n'a pas pu s'installer. Retrait automatique (piège EXIT).

# shellcheck source-path=SCRIPTDIR
set -uo pipefail

ICI=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
RACINE=$(cd "$ICI/../.." && pwd)
# shellcheck source=scenario.sh
. "$ICI/scenario.sh"
# shellcheck source=lib.sh
. "$ICI/lib.sh"

DRY_RUN=0
VM_JETABLE=0
HOTE_PHYSIQUE=0
CONSTRUIRE=1
DELAI=3
TRAVAIL=""
CIBLE="${CARGO_TARGET_DIR:-$RACINE/target}/release"

usage() {
	cat <<'FIN'
Usage : lancer.sh [options]

  --dry-run                 construit et génère (sans privilège), puis imprime
                            les commandes privilégiées sans les exécuter
  --vm-jetable              obligatoire hors --dry-run : confirme une VM jetable
  --accepter-hote-physique  autorise un hôte non virtualisé (déconseillé)
  --sans-construction       ne lance pas cargo (binaires déjà construits)
  --cible <répertoire>      répertoire des binaires (défaut : target/release)
  --travail <répertoire>    répertoire de travail (défaut : mktemp)
  --delai <secondes>        délai d'une tentative de connexion (défaut : 3)
  -h, --help                cette aide
FIN
}

while (($#)); do
	case $1 in
		--dry-run) DRY_RUN=1 ;;
		--vm-jetable) VM_JETABLE=1 ;;
		--accepter-hote-physique) HOTE_PHYSIQUE=1 ;;
		--sans-construction) CONSTRUIRE=0 ;;
		--cible) CIBLE=${2:?--cible : valeur attendue}; shift ;;
		--travail) TRAVAIL=${2:?--travail : valeur attendue}; shift ;;
		--delai) DELAI=${2:?--delai : valeur attendue}; shift ;;
		-h | --help) usage; exit 0 ;;
		*) printf 'option inconnue : %s\n' "$1" >&2; usage >&2; exit 2 ;;
	esac
	shift
done
[[ $DELAI =~ ^[1-9][0-9]*$ ]] || die "--delai : entier ≥ 1 attendu"

MIYUDOUANE="$CIBLE/miyudouane"
ARRETES_ESSAI="$CIBLE/examples/arretes_essai"

# ================================================================ pré-vol

OUTIL_ECOUTE=""
NFLOG_DISPO=0
PING_DISPO=0
MODULE_CHARGE_PAR_BANC=0
REPERTOIRE_CREE_PAR_BANC=""
BANC_INSTALLE=0

prevol() {
	section "Pré-vol"
	if ((!DRY_RUN)); then
		((EUID == 0)) || die "root requis (ou --dry-run)"
		((VM_JETABLE)) || die "refus : confirmez une VM jetable avec --vm-jetable (jamais sur un hôte de production)"
		if command -v systemd-detect-virt >/dev/null 2>&1 && ! systemd-detect-virt --vm -q; then
			((HOTE_PHYSIQUE)) || die "hôte non virtualisé (systemd-detect-virt --vm) : refus sans --accepter-hote-physique"
			avertir "hôte non virtualisé accepté explicitement"
		fi
	fi
	local c manquants=()
	for c in ip nft modprobe sysctl timeout install awk grep sed sort paste cut; do
		command -v "$c" >/dev/null 2>&1 || manquants+=("$c")
	done
	((${#manquants[@]} == 0)) || die "outils manquants : ${manquants[*]} (iproute2, nftables, kmod, procps, coreutils)"
	((BASH_VERSINFO[0] >= 4)) || die "bash ≥ 4 requis"

	local v
	v=$(nft --version 2>/dev/null | sed -n 's/.*v\([0-9][0-9]*\)\..*/\1/p')
	if [[ -z $v ]] || ((v < 1)); then
		die "nftables ≥ 1.0 requis (trouvé : $(nft --version 2>&1 | head -1))"
	fi

	# Écouteur : socat, sinon python3, sinon nc (OpenBSD, busybox, traditionnel).
	if command -v socat >/dev/null 2>&1; then
		OUTIL_ECOUTE=socat
	elif command -v python3 >/dev/null 2>&1; then
		OUTIL_ECOUTE=python3
	elif command -v nc >/dev/null 2>&1 && nc -h 2>&1 | grep -qi openbsd; then
		OUTIL_ECOUTE=nc-openbsd
	elif command -v busybox >/dev/null 2>&1 && busybox nc --help 2>&1 | grep -q -- '-l'; then
		OUTIL_ECOUTE=nc-busybox
	elif command -v nc >/dev/null 2>&1; then
		OUTIL_ECOUTE=nc-traditionnel
	else
		die "aucun outil d'écoute TCP : installez socat, python3 ou netcat (openbsd-netcat / busybox)"
	fi
	info "écouteur : $OUTIL_ECOUTE ; client : bash /dev/tcp"

	if command -v tcpdump >/dev/null 2>&1 && tcpdump -D 2>/dev/null | grep -q nflog; then
		NFLOG_DISPO=1
		info "journaux NFLOG : tcpdump -i nflog:<groupe> (préfixes MDOU| lus dans les captures)"
	else
		avertir "tcpdump avec prise en charge nflog absent : seuls le compteur rejets et la présence des règles MDOU| seront vérifiés"
	fi
	command -v ping >/dev/null 2>&1 && PING_DISPO=1
	command -v nmap >/dev/null 2>&1 || info "nmap absent : balayages ignorés (facultatif)"
	command -v ss >/dev/null 2>&1 || info "ss absent : attente fixe des écouteurs"

	if ((!DRY_RUN)); then
		if [[ -d /sys/module/br_netfilter ]]; then
			die "br_netfilter est chargé : il ferait passer les trames pontées par les crochets inet des zones (tout rejeté). Sur la VM jetable : modprobe -r br_netfilter (ou désinstallez Docker), puis relancez."
		fi
		if ip netns list 2>/dev/null | grep -q '^mcog-'; then
			die "des espaces de noms mcog-* existent déjà (déploiement réel ou banc précédent) : refus"
		fi
		if ip link show "$IF_PHYSIQUE" >/dev/null 2>&1; then
			die "l'interface $IF_PHYSIQUE existe déjà : refus"
		fi
		if [[ -d $REPERTOIRE_REGLES ]] && [[ -n $(ls -A "$REPERTOIRE_REGLES" 2>/dev/null) ]]; then
			die "$REPERTOIRE_REGLES n'est pas vide (configuration réelle ?) : refus"
		fi
		modprobe nf_tables || die "module nf_tables indisponible"
		if [[ ! -d /sys/module/nf_conntrack_bridge ]]; then
			modprobe nf_conntrack_bridge || die "module nf_conntrack_bridge indisponible (noyau ≥ 5.3 requis)"
			MODULE_CHARGE_PAR_BANC=1
		fi
	fi
}

# ============================================================ construction

construire() {
	section "Construction"
	if ((CONSTRUIRE)); then
		command -v cargo >/dev/null 2>&1 || die "cargo introuvable (en root ?) : construisez en utilisateur (./lancer.sh --dry-run) puis relancez avec --sans-construction"
		(cd "$RACINE" && cargo build --release -p miyudouane -p miyuportail) || die "cargo build"
		(cd "$RACINE" && cargo build --release -p miyudouane --example arretes_essai) || die "cargo build --example"
	else
		info "construction ignorée (--sans-construction)"
	fi
	[[ -x $MIYUDOUANE ]] || die "binaire absent : $MIYUDOUANE"
	[[ -x $ARRETES_ESSAI ]] || die "binaire absent : $ARRETES_ESSAI"
}

# ============================================================== génération

SB_ARGS=()
ANCRE=""

douane() { "$MIYUDOUANE" "$@" "${SB_ARGS[@]}"; }

generer() {
	section "Génération (miyudouane, sans privilège)"
	local s
	for s in "${SANDBOXES[@]}"; do SB_ARGS+=(--sandbox "$s"); done
	mkdir -p "$TRAVAIL"/{arretes,regles,recu,journaux} || die "répertoire de travail"

	ANCRE=$("$ARRETES_ESSAI" --sortie "$TRAVAIL/arretes") || die "arrêtés d'essai"
	info "arrêtés signés par une clé jetable ; ancre $ANCRE"
	local A="$TRAVAIL/arretes"

	douane regles --frontiere externe --arrete "$A/externe.arrete" --ancre "$ANCRE" >"$TRAVAIL/regles/$NS_F_EXT.nft" || die "règles externe"
	douane regles --frontiere portail-interne --arrete "$A/portail-interne.arrete" --ancre "$ANCRE" >"$TRAVAIL/regles/$NS_F_PI.nft" || die "règles portail interne"
	douane regles --frontiere portail-local --arrete "$A/portail-local.arrete" --ancre "$ANCRE" >"$TRAVAIL/regles/$NS_F_PL.nft" || die "règles portail local"
	douane zone --zone cog --arrete "portail-interne=$A/portail-interne.arrete" --arrete "portail-local=$A/portail-local.arrete" \
		--ancre "$ANCRE" >"$TRAVAIL/regles/$NS_Z_COG.nft" || die "règles zone COG"
	douane zone --zone franche --arrete "portail-interne=$A/portail-interne.arrete" --arrete "externe=$A/externe.arrete" \
		--ancre "$ANCRE" >"$TRAVAIL/regles/$NS_Z_FRANCHE.nft" || die "règles zone Franche"
	douane zone --zone locale --arrete "portail-local=$A/portail-local.arrete" \
		--ancre "$ANCRE" >"$TRAVAIL/regles/$NS_Z_LOCALE.nft" || die "règles zone Franche locale"
	# Pour mémoire (la coupure réelle passe par --appliquer) :
	douane regles --frontiere portail-interne --arrete "$A/portail-interne.arrete" --ancre "$ANCRE" --couper "$VOIE_A" \
		>"$TRAVAIL/regles/$NS_F_PI.coupe-$VOIE_A.nft" || die "règles portail interne, voie A coupée"
	douane mise-en-place >"$TRAVAIL/mise-en-place.sh" || die "script de mise en place"
	douane retrait >"$TRAVAIL/retrait.sh" || die "script de retrait"

	# Cohérence scénario ↔ plan généré (adresses et ports de pont).
	local attendu manque=0
	for attendu in \
		"ip -n $NS_COG_A addr add $IP_COG_A/24 dev eth0" \
		"ip -n $NS_COG_B addr add $IP_COG_B/24 dev eth0" \
		"ip -n $NS_COG_CAPT addr add $IP_COG_CAPT/24 dev eth0" \
		"ip -n $NS_FR_A addr add $IP_FR_A/24 dev eth0" \
		"ip -n $NS_FR_B addr add $IP_FR_B/24 dev eth0" \
		"ip -n $NS_LOC_CAPT addr add $IP_LOC_CAPT/24 dev eth0" \
		"ip -n $NS_F_PI addr add $IP_PI_COG/24 dev pi-cog" \
		"ip -n $NS_F_PI addr add $IP_PI_FR/24 dev pi-franche" \
		"ip -n $NS_F_PL addr add $IP_PL_COG/24 dev pl-cog" \
		"ip -n $NS_F_PL addr add $IP_PL_LOC/24 dev pl-locale" \
		"ip -n $NS_F_EXT addr add $IP_EXT_FR/24 dev ext-franche" \
		"# $NS_COG_A : aucune route par défaut"; do
		grep -qF -- "$attendu" "$TRAVAIL/mise-en-place.sh" || { avertir "absent du script généré : $attendu"; manque=1; }
	done
	grep -qF "voie=$VOIE_A " "$TRAVAIL/regles/$NS_F_PI.coupe-$VOIE_A.nft" && { avertir "voie A encore présente après coupure"; manque=1; }
	((manque == 0)) || die "le scénario (scenario.sh) ne correspond plus au plan miyudouane"
	info "fichiers générés dans $TRAVAIL"
}

# ============================================================ mise en place

mettre_en_place() {
	section "Mise en place (root)"
	BANC_INSTALLE=1
	if [[ ! -d $REPERTOIRE_REGLES ]]; then
		REPERTOIRE_CREE_PAR_BANC=$REPERTOIRE_REGLES
		# Créer aussi /etc/miyukini s'il n'existe pas, pour le retirer ensuite.
		[[ -d ${REPERTOIRE_REGLES%/*} ]] || REPERTOIRE_CREE_PAR_BANC=${REPERTOIRE_REGLES%/*}
	fi
	run_ou_die install -d -m 0700 "$REPERTOIRE_REGLES"
	# Contrôle à blanc (nft -c) dans un espace jetable : jamais dans celui de l'hôte.
	run_ou_die ip netns add "$NS_VERIF"
	local ns
	for ns in "${NS_ZONES[@]}" "${NS_FRONTIERES[@]}"; do
		nsx "$NS_VERIF" nft -c -f "$TRAVAIL/regles/$ns.nft" || die "nft -c refuse $ns.nft"
		run_ou_die install -m 0600 "$TRAVAIL/regles/$ns.nft" "$REPERTOIRE_REGLES/$ns.nft"
	done
	run_ou_die ip netns del "$NS_VERIF"

	# « Internet » du banc : une paire veth dont l'extrémité hôte devient ext0
	# de la frontière externe (mécanisme MCOG_IF_PHYSIQUE du script généré).
	run_ou_die ip netns add "$NS_INET"
	run_ou_die ip -n "$NS_INET" link set lo up
	nsx "$NS_INET" sysctl -qw net.ipv6.conf.all.disable_ipv6=1 || die "sysctl $NS_INET"
	run_ou_die ip link add "$IF_PHYSIQUE" type veth peer name "$IF_INET" netns "$NS_INET"

	if ((DRY_RUN)); then
		info "contenu de $TRAVAIL/mise-en-place.sh (exécuté avec MCOG_IF_PHYSIQUE=$IF_PHYSIQUE) :"
		montrer env "MCOG_IF_PHYSIQUE=$IF_PHYSIQUE" sh "$TRAVAIL/mise-en-place.sh"
		grep -v '^\s*#' "$TRAVAIL/mise-en-place.sh" | grep -v '^\s*$' | grep -v '^set ' | sed 's/^\s*/# +   /'
	else
		MCOG_IF_PHYSIQUE=$IF_PHYSIQUE sh "$TRAVAIL/mise-en-place.sh" || die "script de mise en place généré"
	fi

	run_ou_die ip -n "$NS_F_EXT" addr add "$IP_EXT0/24" dev ext0
	run_ou_die ip -n "$NS_F_EXT" addr add "$IP_EXT0_HORS/24" dev ext0
	run_ou_die ip -n "$NS_INET" addr add "$IP_INET/24" dev "$IF_INET"
	run_ou_die ip -n "$NS_INET" addr add "$IP_INET_HORS/24" dev "$IF_INET"
	run_ou_die ip -n "$NS_INET" link set "$IF_INET" up
	run_ou_die ip -n "$NS_INET" route add 10.77.0.0/16 via "$IP_EXT0" src "$IP_INET"
}

demarrer_ecouteurs() {
	section "Écouteurs et captures"
	local ns
	if ((NFLOG_DISPO)) || ((DRY_RUN)); then
		for ns in "${NS_ZONES[@]}" "${NS_FRONTIERES[@]}"; do capturer_nflog "$ns"; done
	fi
	ecouter "$NS_F_PI" "$PORT_A"
	ecouter "$NS_F_PI" "$PORT_B"
	ecouter "$NS_F_PI" "$PORT_SSH"
	ecouter "$NS_F_PL" "$PORT_CAPT"
	ecouter "$NS_F_PL" "$PORT_SSH"
	ecouter "$NS_FR_A" "$PORT_A"
	ecouter "$NS_FR_A" "$PORT_EXT"
	ecouter "$NS_FR_A" "$PORT_SSH"
	ecouter "$NS_FR_B" "$PORT_B"
	ecouter "$NS_COG_A" "$PORT_COG"
	ecouter "$NS_COG_B" "$PORT_B"
	ecouter "$NS_COG_B" "$PORT_COG"
	ecouter "$NS_LOC_CAPT" "$PORT_CAPT"
	ecouter "$NS_INET" "$PORT_WEB"
	attendre_ecouteurs "$NS_F_PI:$PORT_A" "$NS_F_PI:$PORT_B" "$NS_F_PI:$PORT_SSH" "$NS_F_PL:$PORT_CAPT" \
		"$NS_F_PL:$PORT_SSH" "$NS_FR_A:$PORT_A" "$NS_FR_A:$PORT_EXT" "$NS_FR_A:$PORT_SSH" "$NS_FR_B:$PORT_B" \
		"$NS_COG_A:$PORT_COG" "$NS_COG_B:$PORT_B" "$NS_COG_B:$PORT_COG" "$NS_LOC_CAPT:$PORT_CAPT" "$NS_INET:$PORT_WEB"
	((DRY_RUN)) && return 0
	sleep 1 # captures tcpdump prêtes
	if ((NFLOG_DISPO)); then
		for ns in "${NS_ZONES[@]}" "${NS_FRONTIERES[@]}"; do
			if [[ ! -f $TRAVAIL/journaux/nflog_$ns.pcap ]]; then
				avertir "capture NFLOG de $ns absente (voir $TRAVAIL/journaux/tcpdump_$ns.log ; AppArmor ?) : compteur rejets seul"
				NFLOG_DISPO=0
			fi
		done
	fi
}

# ================================================================= sondes

J_ZCOG() { printf '%s=MDOU|zone-cog|transit|%s|' "$NS_Z_COG" "$1"; }
J_ZFR() { printf '%s=MDOU|zone-franche|transit|%s|' "$NS_Z_FRANCHE" "$1"; }
J_ZLOC() { printf '%s=MDOU|zone-locale|transit|%s|' "$NS_Z_LOCALE" "$1"; }
J_EXT() { printf '%s=MDOU|externe|%s|%s|' "$NS_F_EXT" "$1" "$2"; }
J_PI() { printf '%s=MDOU|portail-interne|%s|%s|' "$NS_F_PI" "$1" "$2"; }
J_PL() { printf '%s=MDOU|portail-local|%s|%s|' "$NS_F_PL" "$1" "$2"; }

sondes_structure() {
	section "Structure : instances, routes, IPv6"
	local ns
	for ns in "${NS_FRONTIERES[@]}"; do
		# shellcheck disable=SC2016
		tap_si "[EX-19] instance Douane chargée dans $ns (3 chaînes en refus par défaut)" \
			bash -c '[ "$(ip netns exec "$1" nft list table inet miyudouane | grep -c "policy drop;")" = 3 ]' _ "$ns"
	done
	for ns in "${NS_ZONES[@]}"; do
		tap_si "[EX-21] pont de zone filtré chargé dans $ns (table bridge miyudouane_zone)" \
			ip netns exec "$ns" nft list table bridge miyudouane_zone
	done
	for ns in "$NS_COG_A" "$NS_COG_B" "$NS_COG_CAPT" "$NS_LOC_CAPT"; do
		# shellcheck disable=SC2016
		tap_si "[EX-20/INV-Z4] $ns : aucune route par défaut" \
			bash -c '[ -z "$(ip -n "$1" route show default)" ]' _ "$ns"
		# shellcheck disable=SC2016
		tap_si "[EX-20/INV-Z4] $ns : aucune route vers $IP_INET" \
			bash -c '! ip -n "$1" route get "$2" >/dev/null 2>&1' _ "$ns" "$IP_INET"
	done
	# shellcheck disable=SC2016
	tap_si "[EX-20] frontière externe : aucune route vers la zone COG ($IP_COG_A)" \
		bash -c '! ip -n "$1" route get "$2" >/dev/null 2>&1' _ "$NS_F_EXT" "$IP_COG_A"
	for ns in "$NS_F_PI" "$NS_F_PL"; do
		# shellcheck disable=SC2016
		tap_si "[EX-44] $ns : rupture protocolaire (ip_forward=0)" \
			bash -c '[ "$(ip netns exec "$1" sysctl -n net.ipv4.ip_forward)" = 0 ]' _ "$ns"
	done
	for ns in "${NS_PLAN[@]}"; do
		# shellcheck disable=SC2016
		tap_si "[IPv6] $ns : IPv6 désactivé" \
			bash -c '[ "$(ip netns exec "$1" sysctl -n net.ipv6.conf.all.disable_ipv6)" = 1 ]' _ "$ns"
	done
}

sondes_declarees() {
	section "Flux déclarés (doivent passer)"
	sonde_passe "[EX-44] voie A : COG A → Portail interne $IP_PI_COG:$PORT_A" "$NS_COG_A" "$IP_PI_COG" "$PORT_A" "$NS_F_PI"
	sonde_passe "[EX-44] voie B : COG B → Portail interne $IP_PI_COG:$PORT_B" "$NS_COG_B" "$IP_PI_COG" "$PORT_B" "$NS_F_PI"
	sonde_passe "[EX-44] voie A (sortie) : Portail interne → Franche A $IP_FR_A:$PORT_A" "$NS_F_PI" "$IP_FR_A" "$PORT_A" "$NS_FR_A"
	sonde_passe "[EX-44] voie B (sortie) : Portail interne → Franche B $IP_FR_B:$PORT_B" "$NS_F_PI" "$IP_FR_B" "$PORT_B" "$NS_FR_B"
	sonde_passe "[EX-19] capteur : COG capteur → Portail local $IP_PL_COG:$PORT_CAPT" "$NS_COG_CAPT" "$IP_PL_COG" "$PORT_CAPT" "$NS_F_PL"
	sonde_passe "[EX-19] capteur (sortie) : Portail local → Franche locale $IP_LOC_CAPT:$PORT_CAPT" "$NS_F_PL" "$IP_LOC_CAPT" "$PORT_CAPT" "$NS_LOC_CAPT"
	sonde_passe "[EX-19] externe : International $IP_INET → Franche A $IP_FR_A:$PORT_EXT (liste blanche)" "$NS_INET" "$IP_FR_A" "$PORT_EXT" "$NS_FR_A"
}

sondes_interdites() {
	section "Flux non déclarés (doivent échouer, journalisés)"
	# Zone COG : segmentation (EX-21) et sortie (EX-12, EX-20).
	sonde_bloque "[EX-21] latéral : COG A → COG B $IP_COG_B:$PORT_B" "$NS_COG_A" "$IP_COG_B" "$PORT_B" "$(J_ZCOG LATERAL)"
	sonde_bloque "[EX-21] COG A → Portail interne sur le port de la voie B ($PORT_B)" "$NS_COG_A" "$IP_PI_COG" "$PORT_B" "$(J_ZCOG SORTIE-COG)"
	sonde_bloque "[EX-17] COG A → Portail interne port non déclaré ($PORT_SSH)" "$NS_COG_A" "$IP_PI_COG" "$PORT_SSH" "$(J_ZCOG SORTIE-COG)"
	sonde_bloque "[EX-21] COG A → Portail local (voie du capteur, $PORT_CAPT)" "$NS_COG_A" "$IP_PL_COG" "$PORT_CAPT" "$(J_ZCOG SORTIE-COG)"
	sonde_bloque "[EX-20/INV-Z4] COG A → Internet $IP_INET:$PORT_WEB sans route" "$NS_COG_A" "$IP_INET" "$PORT_WEB"
	sonde_ping_bloque "[EX-12] ping COG A → Portail interne $IP_PI_COG" "$NS_COG_A" "$IP_PI_COG" "$(J_ZCOG SORTIE-COG)"
	sonde_ping_bloque "[EX-21] ping latéral COG A → COG B" "$NS_COG_A" "$IP_COG_B" "$(J_ZCOG LATERAL)"

	info "route par défaut FORCÉE dans $NS_COG_A via le Portail (défense en profondeur)"
	run_ou_die ip -n "$NS_COG_A" route add default via "$IP_PI_COG"
	sonde_bloque "[EX-12] COG A → Internet $IP_INET:$PORT_WEB, route forcée" "$NS_COG_A" "$IP_INET" "$PORT_WEB" "$(J_ZCOG SORTIE-COG)"
	sonde_bloque "[EX-12] COG A → Franche A $IP_FR_A:$PORT_A, route forcée" "$NS_COG_A" "$IP_FR_A" "$PORT_A" "$(J_ZCOG SORTIE-COG)"
	run_ou_die ip -n "$NS_COG_A" route del default via "$IP_PI_COG"

	# Franche (EX-29, INV-Z2, INV-Z7).
	sonde_bloque "[EX-29] latéral : Franche A → Franche B $IP_FR_B:$PORT_B" "$NS_FR_A" "$IP_FR_B" "$PORT_B" "$(J_ZFR LATERAL)"
	sonde_bloque "[INV-Z2] Franche A → COG A $IP_COG_A:$PORT_COG direct" "$NS_FR_A" "$IP_COG_A" "$PORT_COG" "$(J_ZFR NON-DECLARE)"
	sonde_bloque "[INV-Z7] Franche A ouvre vers le Portail interne $IP_PI_FR:$PORT_A" "$NS_FR_A" "$IP_PI_FR" "$PORT_A" "$(J_ZFR NON-DECLARE)"
	sonde_bloque "[EX-29] Franche A → Internet $IP_INET:$PORT_WEB non déclaré" "$NS_FR_A" "$IP_INET" "$PORT_WEB" "$(J_ZFR NON-DECLARE)"

	# Frontière externe (INV-Z6, EX-20).
	sonde_bloque "[EX-17] International → Franche A port non déclaré ($PORT_SSH)" "$NS_INET" "$IP_FR_A" "$PORT_SSH" "$(J_EXT transit NON-DECLARE)"
	sonde_bloque "[EX-20] International → COG A $IP_COG_A:$PORT_COG" "$NS_INET" "$IP_COG_A" "$PORT_COG"
	info "source hors liste blanche ($IP_INET_HORS)"
	run_ou_die ip -n "$NS_INET" route replace 10.77.0.0/16 via "$IP_EXT0_HORS" src "$IP_INET_HORS"
	sonde_bloque "[INV-Z6] International hors liste blanche $IP_INET_HORS → Franche A:$PORT_EXT" "$NS_INET" "$IP_FR_A" "$PORT_EXT" "$(J_EXT transit NON-DECLARE)"
	run_ou_die ip -n "$NS_INET" route replace 10.77.0.0/16 via "$IP_EXT0" src "$IP_INET"

	# Franche locale (EX-29).
	sonde_bloque "[EX-29] Franche locale → Internet $IP_INET:$PORT_WEB sans route" "$NS_LOC_CAPT" "$IP_INET" "$PORT_WEB"
	sonde_bloque "[EX-29] Franche locale ouvre vers le Portail local $IP_PL_LOC:$PORT_CAPT" "$NS_LOC_CAPT" "$IP_PL_LOC" "$PORT_CAPT" "$(J_ZLOC NON-DECLARE)"
	info "route par défaut FORCÉE dans $NS_LOC_CAPT via le Portail local"
	run_ou_die ip -n "$NS_LOC_CAPT" route add default via "$IP_PL_LOC"
	sonde_bloque "[EX-29] Franche locale → Internet $IP_INET:$PORT_WEB, route forcée" "$NS_LOC_CAPT" "$IP_INET" "$PORT_WEB" "$(J_ZLOC NON-DECLARE)"
	sonde_bloque "[EX-29] Franche locale → COG A $IP_COG_A:$PORT_COG, route forcée" "$NS_LOC_CAPT" "$IP_COG_A" "$PORT_COG" "$(J_ZLOC NON-DECLARE)"
	run_ou_die ip -n "$NS_LOC_CAPT" route del default via "$IP_PL_LOC"

	sondes_ipv6

	# Balayages (facultatifs, nmap) : seuls les ports déclarés sont ouverts.
	sonde_balayage "[EX-17] balayage COG A → Portail interne : seul $PORT_A ouvert" "$NS_COG_A" "$IP_PI_COG" \
		"1-1024,$PORT_CAPT,$PORT_B,$PORT_A,$PORT_COG" "$PORT_A"
	sonde_balayage "[INV-Z2] balayage Franche A → Portail interne : rien d'ouvert" "$NS_FR_A" "$IP_PI_FR" \
		"1-1024,$PORT_CAPT,$PORT_B,$PORT_A" ""
	sonde_balayage "[INV-Z6] balayage International → Franche A : seul $PORT_EXT ouvert" "$NS_INET" "$IP_FR_A" \
		"1-1024,$PORT_B,$PORT_A" "$PORT_EXT"
}

mac_de() {
	if ((DRY_RUN)); then
		printf '<mac-%s-%s>' "$1" "$2"
		return 0
	fi
	ip -n "$1" -o link show "$2" | sed -n 's/.*link\/ether \([0-9a-f:]*\).*/\1/p'
}

sondes_ipv6() {
	info "IPv6 FORCÉ (réactivé dans COG A, COG B et le Portail interne) : la Douane doit le rejeter"
	local ns dev ns_dev
	for ns_dev in "$NS_COG_A:eth0" "$NS_COG_B:eth0" "$NS_F_PI:pi-cog"; do
		ns=${ns_dev%%:*}
		dev=${ns_dev#*:}
		nsx "$ns" sysctl -qw net.ipv6.conf.all.disable_ipv6=0 "net.ipv6.conf.$dev.disable_ipv6=0" || die "sysctl IPv6 $ns"
	done
	run_ou_die ip -n "$NS_COG_A" addr add "$IP6_COG_A/64" dev eth0 nodad
	run_ou_die ip -n "$NS_COG_B" addr add "$IP6_COG_B/64" dev eth0 nodad
	run_ou_die ip -n "$NS_F_PI" addr add "$IP6_PI/64" dev pi-cog nodad
	# Voisins statiques : le SYN part sans dépendre de la découverte de voisins.
	run_ou_die ip -n "$NS_COG_A" neigh replace "$IP6_COG_B" lladdr "$(mac_de "$NS_COG_B" eth0)" dev eth0 nud permanent
	run_ou_die ip -n "$NS_COG_A" neigh replace "$IP6_PI" lladdr "$(mac_de "$NS_F_PI" pi-cog)" dev eth0 nud permanent
	sonde_bloque "[IPv6] latéral COG A → COG B [$IP6_COG_B]:$PORT_B" "$NS_COG_A" "$IP6_COG_B" "$PORT_B" \
		"$(J_ZCOG LATERAL)" "$(J_ZCOG NON-DECLARE)"
	sonde_bloque "[IPv6] COG A → Portail interne [$IP6_PI]:$PORT_A (voie A, mais en IPv6)" "$NS_COG_A" "$IP6_PI" "$PORT_A" \
		"$(J_ZCOG NON-DECLARE)" "$(J_PI entree NON-DECLARE)"
	for ns_dev in "$NS_COG_A:eth0" "$NS_COG_B:eth0" "$NS_F_PI:pi-cog"; do
		ns=${ns_dev%%:*}
		nsx "$ns" sysctl -qw net.ipv6.conf.all.disable_ipv6=1 || die "sysctl IPv6 $ns"
	done
}

sonde_aucun_jeton() {
	section "Bilan : aucun flux non déclaré n'a traversé"
	if ((DRY_RUN)); then
		tap_ignore "[EX-17] aucun des ${#JETONS_INTERDITS[@]} jetons interdits reçu par un écouteur" "dry-run"
		return 0
	fi
	local j recus=()
	for j in "${JETONS_INTERDITS[@]}"; do
		jeton_recu_quelque_part "$j" && recus+=("$j")
	done
	if ((${#recus[@]} == 0)); then
		tap_ok "[EX-17] aucun des ${#JETONS_INTERDITS[@]} jetons interdits reçu par un écouteur"
	else
		tap_pas_ok "[EX-17] jetons interdits reçus" "${recus[@]}"
	fi
}

# ================================================= coupure, fermeture, retour

appliquer_douane() {
	local desc=$1
	shift
	if ((DRY_RUN)); then
		montrer "$MIYUDOUANE" "$@" "${SB_ARGS[@]}"
		tap_ignore "$desc" "dry-run"
		return 0
	fi
	local sortie
	if sortie=$(douane "$@" 2>&1); then
		tap_ok "$desc"
	else
		tap_pas_ok "$desc" "${sortie:0:400}"
		die "application Douane impossible : la suite n'aurait pas de sens"
	fi
}

sondes_coupure() {
	local A="$TRAVAIL/arretes"
	section "Coupure de la voie A sur le Portail interne (EX-49)"
	appliquer_douane "[EX-49] coupure de $VOIE_A appliquée (miyudouane regles --couper $VOIE_A --appliquer)" \
		regles --frontiere portail-interne --arrete "$A/portail-interne.arrete" --ancre "$ANCRE" --couper "$VOIE_A" --appliquer
	sonde_bloque "[EX-49] voie A coupée : COG A → Portail interne :$PORT_A" "$NS_COG_A" "$IP_PI_COG" "$PORT_A" "$(J_PI entree SORTIE-COG)"
	sonde_bloque "[EX-49] voie A coupée (sortie) : Portail interne → Franche A :$PORT_A" "$NS_F_PI" "$IP_FR_A" "$PORT_A" "$(J_PI sortie NON-DECLARE)"
	sonde_passe "[EX-49] voie B intacte : COG B → Portail interne :$PORT_B" "$NS_COG_B" "$IP_PI_COG" "$PORT_B" "$NS_F_PI"
	sonde_passe "[EX-49] voie B intacte (sortie) : Portail interne → Franche B :$PORT_B" "$NS_F_PI" "$IP_FR_B" "$PORT_B" "$NS_FR_B"
	sonde_passe "[EX-19] autre frontière intacte : capteur → Portail local" "$NS_COG_CAPT" "$IP_PL_COG" "$PORT_CAPT" "$NS_F_PL"
	sonde_passe "[EX-19] autre instance intacte : International → Franche A :$PORT_EXT (voie-a de l'externe)" "$NS_INET" "$IP_FR_A" "$PORT_EXT" "$NS_FR_A"

	section "Fermeture du Portail local (aucun arrêté)"
	appliquer_douane "[EX-19] fermeture du Portail local appliquée (miyudouane fermeture --appliquer)" \
		fermeture --frontiere portail-local --appliquer
	sonde_bloque "[EX-19] Portail local fermé : capteur → Portail local" "$NS_COG_CAPT" "$IP_PL_COG" "$PORT_CAPT" "$(J_PL entree FERME)"
	sonde_passe "[EX-19] Portail interne non affecté : COG B → :$PORT_B" "$NS_COG_B" "$IP_PI_COG" "$PORT_B" "$NS_F_PI"

	section "Rétablissement"
	appliquer_douane "[EX-49] voie A rétablie (miyudouane regles --appliquer)" \
		regles --frontiere portail-interne --arrete "$A/portail-interne.arrete" --ancre "$ANCRE" --appliquer
	appliquer_douane "[EX-19] Portail local rouvert (miyudouane regles --appliquer)" \
		regles --frontiere portail-local --arrete "$A/portail-local.arrete" --ancre "$ANCRE" --appliquer
	sonde_passe "[EX-49] voie A rétablie : COG A → Portail interne :$PORT_A" "$NS_COG_A" "$IP_PI_COG" "$PORT_A" "$NS_F_PI"
	sonde_passe "[EX-49] voie A rétablie (sortie) : Portail interne → Franche A :$PORT_A" "$NS_F_PI" "$IP_FR_A" "$PORT_A" "$NS_FR_A"
	sonde_passe "[EX-19] capteur rétabli : COG capteur → Portail local" "$NS_COG_CAPT" "$IP_PL_COG" "$PORT_CAPT" "$NS_F_PL"
	sonde_passe "[EX-49] voie B toujours intacte : COG B → Portail interne :$PORT_B" "$NS_COG_B" "$IP_PI_COG" "$PORT_B" "$NS_F_PI"
}

# ================================================================= retrait

DEJA_RETIRE=0
# shellcheck disable=SC2329  # appelée par le piège EXIT
retirer() {
	local rc=$?
	((DEJA_RETIRE)) && return
	DEJA_RETIRE=1
	trap - INT TERM
	if ((BANC_INSTALLE)); then
		section "Retrait"
		local ns pid
		for pid in "${PIDS_FOND[@]}"; do run kill "$pid" 2>/dev/null; done
		# Tout processus restant dans un espace du banc le maintiendrait en vie.
		for ns in "${NS_TOUS[@]}"; do
			if ((DRY_RUN)); then
				montrer ip netns pids "$ns"
			else
				for pid in $(ip netns pids "$ns" 2>/dev/null); do kill "$pid" 2>/dev/null; done
			fi
		done
		((DRY_RUN)) || sleep 0.5
		if [[ -f $TRAVAIL/retrait.sh ]]; then
			run sh "$TRAVAIL/retrait.sh"
			if ((DRY_RUN)); then
				grep -v '^\s*#' "$TRAVAIL/retrait.sh" | grep -v '^\s*$' | grep -v '^set ' | sed 's/^\s*/# +   /'
			fi
		fi
		run ip netns del "$NS_INET" 2>/dev/null
		run ip netns del "$NS_VERIF" 2>/dev/null
		run ip link del "$IF_PHYSIQUE" 2>/dev/null
		for ns in "${NS_ZONES[@]}" "${NS_FRONTIERES[@]}"; do
			run rm -f "$REPERTOIRE_REGLES/$ns.nft"
		done
		case $REPERTOIRE_CREE_PAR_BANC in
			/etc/miyukini | /etc/miyukini/douane) run rm -rf --one-file-system "$REPERTOIRE_CREE_PAR_BANC" ;;
		esac
		if ((!DRY_RUN)); then
			if ip netns list 2>/dev/null | grep -q '^mcog-'; then
				printf '# ATTENTION : espaces mcog-* restants : %s\n' "$(ip netns list | grep '^mcog-' | tr '\n' ' ')"
				rc=1
			else
				info "machine restaurée (aucun espace mcog-* restant)"
			fi
		fi
	fi
	if ((MODULE_CHARGE_PAR_BANC)); then
		run modprobe -r nf_conntrack_bridge 2>/dev/null || true
	fi
	exit "$rc"
}

# ================================================================== main

trap retirer EXIT
trap 'exit 130' INT TERM

printf 'TAP version 13\n'
info "Banc d'intrusion du lot 3 — $(date -u +%Y-%m-%dT%H:%M:%SZ)$( ((DRY_RUN)) && printf ' — DRY-RUN')"
prevol
if [[ -z $TRAVAIL ]]; then
	TRAVAIL=$(mktemp -d "${TMPDIR:-/tmp}/mcog-intrusion.XXXXXX") || die "mktemp"
fi
TRAVAIL=$(cd "$TRAVAIL" 2>/dev/null && pwd || printf '%s' "$TRAVAIL")
info "répertoire de travail : $TRAVAIL"
construire
generer
mettre_en_place
demarrer_ecouteurs
sondes_structure
sondes_declarees
sondes_interdites
sondes_coupure
sonde_aucun_jeton

printf '1..%d\n' "$TAP_N"
if ((TAP_ECHECS)); then
	info "ÉCHEC : $TAP_ECHECS sur $TAP_N"
	for l in "${TAP_ECHECS_LISTE[@]}"; do info "  not ok $l"; done
	info "captures et journaux : $TRAVAIL/journaux"
	exit 1
fi
if ((DRY_RUN)); then
	info "DRY-RUN : $TAP_N contrôles listés, rien n'a été exécuté avec privilège"
else
	info "SUCCÈS : $TAP_N contrôles ($TAP_IGNORES ignorés) — critère de sortie du lot 3 satisfait sur ce banc"
fi
exit 0
