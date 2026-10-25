# shellcheck shell=bash
# Banc d'intrusion du lot 3 — fonctions communes (exécution, TAP, écouteurs,
# sondes, compteurs nftables, journaux NFLOG).
#
# Variables attendues (posées par lancer.sh) : DRY_RUN (0/1), TRAVAIL, DELAI,
# OUTIL_ECOUTE, NFLOG_DISPO (0/1), PING_DISPO (0/1).

# ----------------------------------------------------------------- messages

info() { printf '# %s\n' "$*"; }
avertir() { printf '# ATTENTION : %s\n' "$*" >&2; }
die() {
	printf 'Bail out! %s\n' "$*"
	exit 2
}

# ---------------------------------------------------------------- exécution

# Imprime une commande telle qu'elle serait exécutée (citations shell). Le
# préfixe « # + » garde la sortie TAP valide.
montrer() {
	local a
	printf '# +'
	for a in "$@"; do printf ' %q' "$a"; done
	printf '\n'
}

# Exécute (ou imprime en --dry-run) une commande privilégiée.
run() {
	if ((DRY_RUN)); then
		montrer "$@"
		return 0
	fi
	"$@"
}

# Comme run, mais l'échec arrête le banc (le retrait passe par le piège EXIT).
run_ou_die() {
	run "$@" || die "échec : $*"
}

# Exécution dans un espace de noms.
nsx() {
	local ns=$1
	shift
	run ip netns exec "$ns" "$@"
}

# Processus d'arrière-plan suivis (tués au retrait, avec tout processus
# restant dans les espaces de noms du banc).
PIDS_FOND=()
run_fond() {
	local journal=$1
	shift
	if ((DRY_RUN)); then
		montrer "$@"
		printf '#   (arrière-plan, sortie → %s)\n' "$journal"
		return 0
	fi
	"$@" >>"$journal" 2>&1 &
	PIDS_FOND+=("$!")
}

# --------------------------------------------------------------------- TAP

TAP_N=0
TAP_ECHECS=0
TAP_IGNORES=0
TAP_ECHECS_LISTE=()

tap_ok() {
	TAP_N=$((TAP_N + 1))
	printf 'ok %d - %s\n' "$TAP_N" "$1"
}

tap_pas_ok() {
	TAP_N=$((TAP_N + 1))
	TAP_ECHECS=$((TAP_ECHECS + 1))
	TAP_ECHECS_LISTE+=("$TAP_N - $1")
	printf 'not ok %d - %s\n' "$TAP_N" "$1"
	shift
	local l
	for l in "$@"; do printf '#   %s\n' "$l"; done
}

tap_ignore() {
	TAP_N=$((TAP_N + 1))
	TAP_IGNORES=$((TAP_IGNORES + 1))
	printf 'ok %d - %s # SKIP %s\n' "$TAP_N" "$1" "$2"
}

# tap_si <description> <commande…> : ok si la commande réussit.
tap_si() {
	local desc=$1
	shift
	if ((DRY_RUN)); then
		montrer "$@"
		tap_ignore "$desc" "dry-run"
		return 0
	fi
	local sortie
	if sortie=$("$@" 2>&1); then
		tap_ok "$desc"
	else
		tap_pas_ok "$desc" "commande : $*" "sortie : ${sortie:0:300}"
	fi
}

section() {
	printf '#\n# ==== %s\n' "$*"
}

# ---------------------------------------------------------------- écouteurs

# Script d'écoute Python : accepte en boucle, ajoute les octets reçus au fichier.
PY_ECOUTE='import socket, sys
port, fichier = int(sys.argv[1]), sys.argv[2]
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(("0.0.0.0", port))
s.listen(16)
while True:
    c, _ = s.accept()
    c.settimeout(3)
    d = b""
    try:
        while True:
            x = c.recv(4096)
            if not x:
                break
            d += x
    except OSError:
        pass
    c.close()
    with open(fichier, "ab") as o:
        o.write(d)
'

fichier_recu() { printf '%s/recu/%s_%s.txt' "$TRAVAIL" "$1" "$2"; }

# ecouter <netns> <port> : écouteur TCP IPv4 (0.0.0.0) qui consigne ce qu'il reçoit.
ecouter() {
	local ns=$1 port=$2 f
	f=$(fichier_recu "$ns" "$port")
	((DRY_RUN)) || : >"$f"
	local journal="$TRAVAIL/journaux/ecouteur_${ns}_${port}.log"
	case $OUTIL_ECOUTE in
		socat)
			run_fond "$journal" ip netns exec "$ns" socat -u "TCP4-LISTEN:$port,reuseaddr,fork" "OPEN:$f,creat,append"
			;;
		python3)
			run_fond "$journal" ip netns exec "$ns" python3 -c "$PY_ECOUTE" "$port" "$f"
			;;
		nc-openbsd)
			# shellcheck disable=SC2016  # développé par le bash de l'espace de noms
			run_fond "$journal" ip netns exec "$ns" bash -c 'while :; do nc -l "$1" >>"$2" || sleep 0.2; done' _ "$port" "$f"
			;;
		nc-busybox)
			# shellcheck disable=SC2016
			run_fond "$journal" ip netns exec "$ns" bash -c 'while :; do busybox nc -l -p "$1" >>"$2" || sleep 0.2; done' _ "$port" "$f"
			;;
		nc-traditionnel)
			# shellcheck disable=SC2016
			run_fond "$journal" ip netns exec "$ns" bash -c 'while :; do nc -l -p "$1" >>"$2" || sleep 0.2; done' _ "$port" "$f"
			;;
		*) die "outil d'écoute inconnu : $OUTIL_ECOUTE" ;;
	esac
}

# Attend que les écouteurs soient prêts (ss), sinon une seconde.
attendre_ecouteurs() {
	((DRY_RUN)) && return 0
	if ! command -v ss >/dev/null 2>&1; then
		sleep 1
		return 0
	fi
	local spec ns port i
	for spec in "$@"; do
		ns=${spec%%:*}
		port=${spec##*:}
		for ((i = 0; i < 50; i++)); do
			if ip netns exec "$ns" ss -Hltn "( sport = :$port )" 2>/dev/null | grep -q .; then
				break
			fi
			sleep 0.1
		done
		((i < 50)) || die "écouteur $ns:$port non prêt"
	done
}

# ------------------------------------------------------------------ jetons

JETONS_INTERDITS=()
JETON_N=0
nouveau_jeton() {
	JETON_N=$((JETON_N + 1))
	printf 'MCOG-%s-%03d-%s' "$1" "$JETON_N" "$RANDOM$RANDOM"
}

jeton_recu_quelque_part() {
	grep -rqF -- "$1" "$TRAVAIL/recu" 2>/dev/null
}

# ----------------------------------------------------------------- client

# connecter <netns> <ip> <port> <jeton> : tente une connexion TCP (bash
# /dev/tcp, aucun outil requis) et envoie le jeton. Pose RESULTAT ∈
# connecte | expire | refuse | injoignable | interdit | erreur, et RESULTAT_DETAIL.
# « interdit » : EPERM, paquet local rejeté par la chaîne de sortie de
# l'espace client (la Douane d'une frontière qui ouvre la connexion).
connecter() {
	local ns=$1 ip=$2 port=$3 jeton=$4 sortie rc=0
	# shellcheck disable=SC2016
	sortie=$(LC_ALL=C ip netns exec "$ns" timeout "$DELAI" bash -c \
		'exec 3<>"/dev/tcp/$1/$2" && printf "%s\n" "$3" >&3 && exec 3>&-' _ "$ip" "$port" "$jeton" 2>&1) || rc=$?
	RESULTAT_DETAIL=${sortie//$'\n'/ }
	if ((rc == 0)); then
		RESULTAT=connecte
	elif ((rc == 124)); then
		RESULTAT=expire
	elif [[ $sortie == *"refused"* ]]; then
		RESULTAT=refuse
	elif [[ $sortie == *"unreachable"* || $sortie == *"No route"* ]]; then
		RESULTAT=injoignable
	elif [[ $sortie == *"Operation not permitted"* ]]; then
		RESULTAT=interdit
	else
		RESULTAT=erreur
	fi
}

# Forme imprimée en --dry-run (identique à la commande de connecter).
montrer_client() {
	# shellcheck disable=SC2016
	montrer ip netns exec "$1" timeout "$DELAI" bash -c \
		'exec 3<>"/dev/tcp/$1/$2" && printf "%s\n" "$3" >&3 && exec 3>&-' _ "$2" "$3" "$4"
}

# ---------------------------------------------- compteurs et journaux NFLOG

# Paquets rejetés par la Douane dans un espace de noms : somme des compteurs
# nommés « rejets » de ses tables (journalisation bornée, A-016 : la règle
# « log prefix "MDOU|…" » ne porte pas de compteur ; le rejet qui la suit
# incrémente toujours « rejets », journalisé ou non). Le préfixe exact est
# prouvé par la capture NFLOG (nflog_occurrences).
compteur_rejets() {
	local ns=$1
	{ ip netns exec "$ns" nft list counters 2>/dev/null || true; } |
		awk '/counter rejets \{/ { r = 1; next }
			r && /packets/ { for (i = 1; i < NF; i++) if ($i == "packets") s += $(i + 1); r = 0 }
			END { print s + 0 }'
}

# Vrai si l'espace de noms porte au moins une règle de journal de ce préfixe.
regle_journal_presente() {
	local ns=$1 prefixe=$2
	{ ip netns exec "$ns" nft list ruleset 2>/dev/null || true; } | grep -qF "log prefix \"$prefixe\""
}

# Occurrences du préfixe dans la capture NFLOG de l'espace de noms.
nflog_occurrences() {
	local ns=$1 prefixe=$2 f="$TRAVAIL/journaux/nflog_$1.pcap"
	[[ -f $f ]] || {
		echo 0
		return 0
	}
	{ grep -a -o -F -- "$prefixe" "$f" || true; } | wc -l | tr -d ' '
}

# Démarre une capture NFLOG (tcpdump -i nflog:<groupe>) dans un espace de noms.
capturer_nflog() {
	local ns=$1 g=${GROUPE_NFLOG[$1]}
	run_fond "$TRAVAIL/journaux/tcpdump_${ns}.log" \
		ip netns exec "$ns" tcpdump -i "nflog:$g" -n -U -s 0 -Z root -w "$TRAVAIL/journaux/nflog_${ns}.pcap"
}

# Instantané des journaux attendus : JOURNAL_C0[i], JOURNAL_N0[i] pour chaque
# « netns=préfixe » de JOURNAUX_ATTENDUS.
instantane_journaux() {
	JOURNAL_C0=()
	JOURNAL_N0=()
	local spec
	for spec in "${JOURNAUX_ATTENDUS[@]}"; do
		JOURNAL_C0+=("$(compteur_rejets "${spec%%=*}")")
		JOURNAL_N0+=("$(nflog_occurrences "${spec%%=*}" "${spec#*=}")")
	done
}

# Vérifie qu'au moins un des journaux attendus a progressé : le compteur
# « rejets » de l'espace a progressé (le paquet a été rejeté par la Douane) ET
# la capture NFLOG contient au moins une ligne de plus portant le préfixe exact
# (journalisation bornée : au moins une ligne par flux, pas une par paquet).
# Sans capture NFLOG : compteur « rejets » et présence de la règle de ce
# préfixe seulement. Émet une ligne TAP.
verifier_journaux() {
	local desc=$1 i spec c1 n1 trouve="" dc dn attente
	((${#JOURNAUX_ATTENDUS[@]})) || return 0
	for ((attente = 0; attente < 20; attente++)); do
		trouve=""
		for i in "${!JOURNAUX_ATTENDUS[@]}"; do
			spec=${JOURNAUX_ATTENDUS[$i]}
			c1=$(compteur_rejets "${spec%%=*}")
			n1=$(nflog_occurrences "${spec%%=*}" "${spec#*=}")
			dc=$((c1 - JOURNAL_C0[i]))
			dn=$((n1 - JOURNAL_N0[i]))
			if ((dc > 0)) && { ((NFLOG_DISPO)) && ((dn > 0)) ||
				{ ((!NFLOG_DISPO)) && regle_journal_presente "${spec%%=*}" "${spec#*=}"; }; }; then
				trouve="${spec#*=} dans ${spec%%=*} (rejets +$dc, nflog +$dn)"
				break
			fi
		done
		[[ -n $trouve ]] && break
		sleep 0.1
	done
	if [[ -n $trouve ]]; then
		if ((NFLOG_DISPO)); then
			tap_ok "[EX-12] journal Douane : $trouve"
		else
			tap_ok "[EX-12] journal Douane : $trouve # nflog non capturé (tcpdump sans nflog) : compteur rejets et règle du préfixe seuls"
		fi
	else
		tap_pas_ok "[EX-12] journal Douane absent pour : $desc" "attendu (l'un de) : ${JOURNAUX_ATTENDUS[*]}"
	fi
}

# ------------------------------------------------------------------ sondes

# sonde_passe <desc> <netns client> <ip> <port> <netns écouteur>
# Flux déclaré : la connexion aboutit ET le jeton arrive à l'écouteur.
sonde_passe() {
	local desc=$1 ns=$2 ip=$3 port=$4 ns_ecoute=$5 jeton f i
	jeton=$(nouveau_jeton P)
	if ((DRY_RUN)); then
		montrer_client "$ns" "$ip" "$port" "$jeton"
		tap_ignore "$desc — attendu : passe, reçu par $ns_ecoute:$port" "dry-run"
		return 0
	fi
	f=$(fichier_recu "$ns_ecoute" "$port")
	connecter "$ns" "$ip" "$port" "$jeton"
	for ((i = 0; i < 20; i++)); do
		grep -qF -- "$jeton" "$f" 2>/dev/null && break
		sleep 0.1
	done
	if [[ $RESULTAT == connecte ]] && grep -qF -- "$jeton" "$f" 2>/dev/null; then
		tap_ok "$desc"
	else
		tap_pas_ok "$desc" "résultat : $RESULTAT ($RESULTAT_DETAIL)" "jeton $jeton reçu par $ns_ecoute:$port : non"
	fi
}

# sonde_bloque <desc> <netns client> <ip> <port> [netns=préfixe MDOU…]
# Flux non déclaré : la connexion n'aboutit pas (expiration ou absence de
# route ; un refus TCP prouverait que le paquet a traversé), le jeton n'arrive
# nulle part, et l'un des journaux attendus progresse.
sonde_bloque() {
	local desc=$1 ns=$2 ip=$3 port=$4 jeton
	shift 4
	JOURNAUX_ATTENDUS=("$@")
	jeton=$(nouveau_jeton X)
	JETONS_INTERDITS+=("$jeton")
	if ((DRY_RUN)); then
		montrer_client "$ns" "$ip" "$port" "$jeton"
		tap_ignore "$desc — attendu : bloqué${*:+, journal : $*}" "dry-run"
		return 0
	fi
	instantane_journaux
	connecter "$ns" "$ip" "$port" "$jeton"
	sleep 0.3
	case $RESULTAT in
		expire | injoignable | interdit)
			if jeton_recu_quelque_part "$jeton"; then
				tap_pas_ok "$desc" "jeton $jeton reçu par un écouteur : le flux a traversé"
			else
				tap_ok "$desc ($RESULTAT)"
			fi
			;;
		*)
			tap_pas_ok "$desc" "résultat : $RESULTAT ($RESULTAT_DETAIL) — attendu : expiration ou absence de route"
			;;
	esac
	verifier_journaux "$desc"
}

# sonde_ping_bloque <desc> <netns> <ip> [netns=préfixe…]
sonde_ping_bloque() {
	local desc=$1 ns=$2 ip=$3
	shift 3
	JOURNAUX_ATTENDUS=("$@")
	if ((DRY_RUN)); then
		montrer ip netns exec "$ns" ping -n -q -c 2 -W 1 "$ip"
		tap_ignore "$desc — attendu : bloqué${*:+, journal : $*}" "dry-run"
		return 0
	fi
	if ((!PING_DISPO)); then
		tap_ignore "$desc" "ping absent"
		return 0
	fi
	instantane_journaux
	if LC_ALL=C ip netns exec "$ns" ping -n -q -c 2 -W 1 "$ip" >/dev/null 2>&1; then
		tap_pas_ok "$desc" "ping a reçu une réponse"
	else
		tap_ok "$desc"
	fi
	verifier_journaux "$desc"
}

# sonde_balayage <desc> <netns> <ip> <ports> <ouverts attendus (liste « p1,p2 » ou vide)>
sonde_balayage() {
	local desc=$1 ns=$2 ip=$3 ports=$4 attendus=$5 sortie ouverts
	local cmd=(ip netns exec "$ns" nmap -n -Pn -sT --max-retries 0 --initial-rtt-timeout 200ms
		--max-rtt-timeout 400ms -p "$ports" -oG - "$ip")
	if ((DRY_RUN)); then
		montrer "${cmd[@]}"
		tap_ignore "$desc — ports ouverts attendus : {${attendus}}" "dry-run"
		return 0
	fi
	if ! command -v nmap >/dev/null 2>&1; then
		tap_ignore "$desc" "nmap absent (facultatif)"
		return 0
	fi
	sortie=$("${cmd[@]}" 2>&1)
	ouverts=$(printf '%s\n' "$sortie" | grep -o '[0-9]*/open/' | cut -d/ -f1 | sort -n | paste -sd, -)
	if [[ $ouverts == "$attendus" ]]; then
		tap_ok "$desc (ouverts : {${ouverts}})"
	else
		tap_pas_ok "$desc" "ports ouverts : {${ouverts}} ; attendus : {${attendus}}"
	fi
}
