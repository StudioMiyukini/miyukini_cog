#!/usr/bin/env bash
# Tests de fumée des services COG installés dans la VM jetable (sortie TAP).
#
# A exécuter DANS la VM, en utilisateur normal (sudo sans mot de passe requis) :
#   /home/cog/cog-paquet/tester.sh            tests de fumée (sans reconstruction)
#   /home/cog/cog-paquet/tester.sh --banc     ajoute le banc d'intrusion lot 3 (root, ~2 min)
#
# Codes de retour : 0 tous les contrôles passent · 1 au moins un « not ok ».
# Ne modifie pas la configuration des services ; les refus hors boucle locale sont
# vérifiés avec des instances jetables (répertoires temporaires, nettoyés en sortie).

set -uo pipefail

BANC=0
for arg in "$@"; do
	case $arg in
		--banc) BANC=1 ;;
		-h | --help) sed -n '2,9p' "$0"; exit 0 ;;
		*) printf 'option inconnue : %s\n' "$arg" >&2; exit 2 ;;
	esac
done

BIN=/opt/miyukini/bin
ETC=/etc/miyukini
VAR=/var/lib/miyukini
PAQUET=/home/cog/cog-paquet
TMP=$(mktemp -d /tmp/cog-tester.XXXXXX)
trap 'rm -rf "$TMP"' EXIT

N=0
ECHECS=0
SORTIE=()

# resultat <ok|nok> <description> [diagnostic]
resultat() {
	N=$((N + 1))
	if [[ $1 == ok ]]; then
		SORTIE+=("ok $N - $2")
	else
		ECHECS=$((ECHECS + 1))
		SORTIE+=("not ok $N - $2")
		[[ -n ${3:-} ]] && SORTIE+=("  # ${3}")
	fi
}

# controle <description> <commande...> : ok si la commande réussit.
controle() {
	local desc=$1 sortie
	shift
	if sortie=$("$@" 2>&1); then
		resultat ok "$desc"
	else
		resultat nok "$desc" "$(printf '%s' "$sortie" | head -n 2 | tr '\n' ' ' | cut -c1-200)"
	fi
}

code_http() { curl -s -o /dev/null -w '%{http_code}' --max-time 10 "$@"; }

# ------------------------------------------------------------------ services

for u in postgresql miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
	controle "$u actif" systemctl is-active --quiet "$u"
done

for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
	controle "$u : pas de redémarrage en boucle" \
		test "$(systemctl show -p NRestarts --value "$u")" = 0
done

# ------------------------------------------------------------------ écoute réseau

ecoute_sur() { ss -ltnH | awk -v a="$1" '$4 == a { f = 1 } END { exit !f }'; }
for pair in "KindMother:127.0.0.1:50051" "MAIA:127.0.0.1:11435" "MiyuCloud:127.0.0.1:8086" \
	"portail:127.0.0.1:8090" "PostgreSQL:127.0.0.1:5432"; do
	nom=${pair%%:*}
	adresse=${pair#*:}
	controle "$nom écoute sur $adresse" ecoute_sur "$adresse"
done

controle "aucune écoute hors boucle locale (hors SSH et résolveur systemd)" \
	bash -c '! ss -ltnH | awk "{ print \$4 }" | grep -vE "^(127\.|\[::1\]|\*:22\$|0\.0\.0\.0:22\$|\[::\]:22\$|0\.0\.0\.0:53\$)" | grep -q .'

# ------------------------------------------------------------------ KindMother

cat >"$TMP/km.py" <<'PY'
import json, os, socket, struct, sys

def trame(s, obj):
    b = json.dumps(obj).encode()
    s.sendall(struct.pack(">I", len(b)) + b)

def lire(s):
    try:
        h = s.recv(4)
        if len(h) < 4:
            return b""
        n = struct.unpack(">I", h)[0]
        out = b""
        while len(out) < n:
            c = s.recv(n - len(out))
            if not c:
                break
            out += c
        return out
    except OSError:
        return b""

s = socket.create_connection(("127.0.0.1", 50051), timeout=5)
if sys.argv[1] == "hello":
    # Protocole : étiquette interne "type" (serde tag = "type").
    trame(s, {"type": "ClientHello", "protocol": "kindmother-auth-v2", "client_nonce": os.urandom(32).hex()})
    sys.exit(0 if b'"type":"Challenge"' in lire(s) else 1)
# Requête de service sans poignée de main : jamais de HealthCheckResult.
trame(s, {"type": "HealthCheck", "database": None})
r = lire(s)
sys.exit(0 if b"HealthCheckResult" not in r else 1)
PY

controle "KindMother répond au ClientHello par un défi signé" python3 -I "$TMP/km.py" hello
controle "KindMother refuse HealthCheck sans authentification" python3 -I "$TMP/km.py" sans-auth

controle "clé d'identité KindMother 0600 dans un répertoire 0700" bash -c \
	"test \"\$(sudo -n stat -c %a $VAR/kindmother/credentials/server.key)\" = 600 && \
	 test \"\$(sudo -n stat -c %a $VAR/kindmother)\" = 700"

refus_km() {
	local d="$TMP/km-hors-boucle" rc
	mkdir -p "$d"
	KINDMOTHER_DATA_DIR="$d" KINDMOTHER_LISTEN_ADDR=0.0.0.0:19877 timeout 90 "$BIN/kindmother-server" \
		>"$TMP/km.log" 2>&1
	rc=$?
	[[ $rc -ne 0 ]] && grep -q 'Refusing non-loopback' "$TMP/km.log"
}
controle "KindMother refuse une écoute hors boucle locale (démarrage jetable)" refus_km

# ------------------------------------------------------------------ MAIA

controle "MAIA /health répond 200 avec status ok" bash -c \
	"curl -fsS --max-time 10 http://127.0.0.1:11435/health | grep -q '\"status\":\"ok\"'"

for chemin in /v1/agents /v1/skills /v1/security/status /v1/models/local; do
	controle "MAIA GET $chemin sans signature refusé (503, aucun appelant épinglé)" \
		test "$(code_http "http://127.0.0.1:11435$chemin")" = 503
done
controle "MAIA POST /v1/llm/chat sans signature refusé (503)" \
	test "$(code_http -X POST -H 'content-type: application/json' -d '{}' http://127.0.0.1:11435/v1/llm/chat)" = 503
controle "MAIA routes mémoire désactivées par défaut (503)" \
	test "$(code_http -X POST -H 'content-type: application/json' -d '{}' http://127.0.0.1:11435/v1/memoire/tests/consulter)" = 503

refus_maia() {
	local d="$TMP/maia-hors-boucle" rc
	mkdir -p "$d"
	printf 'bind_addr = "0.0.0.0:19876"\n' >"$d/bridge.toml"
	MIYUKINI_DATA_DIR="$d" timeout 30 "$BIN/maia" >"$TMP/maia.log" 2>&1
	rc=$?
	[[ $rc -ne 0 ]] && grep -q 'Refus de démarrer' "$TMP/maia.log"
}
controle "MAIA refuse de démarrer hors boucle locale sans appelant épinglé (démarrage jetable)" refus_maia

# ------------------------------------------------------------------ MiyuCloud et PostgreSQL

controle "MiyuCloud /api/auth/status répond 200" bash -c \
	"curl -fsS --max-time 10 http://127.0.0.1:8086/api/auth/status | grep -q initialized"
controle "MiyuCloud page d'accueil répond 200" test "$(code_http http://127.0.0.1:8086/)" = 200
controle "MiyuCloud API protégée refuse sans session (401)" \
	test "$(code_http http://127.0.0.1:8086/api/files)" = 401
controle "PostgreSQL : schéma MiyuCloud appliqué (tables publiques présentes)" bash -c \
	"n=\$(sudo -n runuser -u postgres -- psql -d miyucloud -tAc \"SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public'\" | tr -d ' '); test \"\${n:-0}\" -gt 0"

# ------------------------------------------------------------------ portail web

controle "portail : page d'accueil 200" test "$(code_http http://127.0.0.1:8090/)" = 200
controle "portail : en-têtes CSP et anti-cadrage présents" bash -c \
	"h=\$(curl -sI --max-time 10 http://127.0.0.1:8090/); echo \"\$h\" | grep -qi '^content-security-policy: default-src .self' && echo \"\$h\" | grep -qi '^x-frame-options: DENY'"
controle "portail : chemin inconnu sans erreur serveur (< 500)" bash -c \
	"c=\$(curl -s -o /dev/null -w '%{http_code}' --max-time 10 http://127.0.0.1:8090/inconnu/page); test \"\$c\" -lt 500"

# ------------------------------------------------------------------ durcissement et permissions

for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-portail; do
	controle "compte de service $u : UID système, shell nologin" bash -c \
		"e=\$(getent passwd $u) && uid=\$(echo \"\$e\" | cut -d: -f3) && sh=\$(echo \"\$e\" | cut -d: -f7) && test \"\$uid\" -lt 1000 && test \"\$sh\" = /usr/sbin/nologin"
done

for svc in kindmother:miyukini-kindmother maia:miyukini-maia miyucloud:miyukini-miyucloud cog-web-portal:miyukini-portail; do
	dir=$VAR/${svc%%:*}
	usr=${svc#*:}
	controle "$dir : 0700, propriété de $usr" bash -c \
		"test \"\$(stat -c '%a %U' $dir)\" = '700 $usr'"
done

controle "secrets (portail, MiyuCloud) en 0600" bash -c \
	"test \"\$(stat -c %a $ETC/cog-web-portal.env)\" = 600 && test \"\$(stat -c %a $ETC/miyucloud.env)\" = 600"

# B2 : bridge.toml de MAIA dans /etc/miyukini (root, lecture seule), présenté à MAIA par
# un montage lié en lecture seule à l'endroit où il le lit.
controle "bridge.toml : $ETC/bridge.toml, propriété root, 0444" bash -c \
	"test \"\$(stat -c '%a %U' $ETC/bridge.toml)\" = '444 root'"
controle "MAIA : bridge.toml lié en lecture seule (BindReadOnlyPaths)" bash -c \
	"systemctl show -p BindReadOnlyPaths --value miyukini-maia | grep -qF '$ETC/bridge.toml:$VAR/maia/bridge.toml'"
bridge_monte_ro() {
	local pid
	pid=$(systemctl show -p MainPID --value miyukini-maia)
	[[ -n $pid && $pid != 0 ]] || return 1
	sudo -n awk -v p="$VAR/maia/bridge.toml" '$5 == p && $6 ~ /^ro(,|$)/ { f = 1 } END { exit !f }' "/proc/$pid/mountinfo"
}
controle "MAIA : bridge.toml monté en lecture seule dans l'espace du service" bridge_monte_ro

# B7 : binaires installés conformes au manifeste du paquet (lié au commit).
controle "binaires installés conformes à /opt/miyukini/MANIFESTE" bash -c \
	"cd /opt/miyukini && grep -v '^#' MANIFESTE | grep '  \\./bin/' | sha256sum --check --strict --quiet -"

propriete_egale() { [[ "$(systemctl show -p "$2" --value "$1")" == "$3" ]]; }
for u in miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
	controle "$u : NoNewPrivileges=yes" propriete_egale "$u" NoNewPrivileges yes
	controle "$u : ProtectSystem=strict" propriete_egale "$u" ProtectSystem strict
	controle "$u : PrivateTmp=yes" propriete_egale "$u" PrivateTmp yes
done

# ------------------------------------------------------------------ périmètre (Douane, sas, racine)

controle "Douane : binaire présent, aucune configuration /etc/miyukini/douane (hors banc)" \
	bash -c "test -x $BIN/miyudouane && test ! -e $ETC/douane"
controle "portail-sas : sans argument, refuse (code non nul)" bash -c \
	"! $BIN/portail-sas </dev/null >/dev/null 2>&1"
controle "racine hors ligne (miyuracine) absente de la VM réseau" test ! -e "$BIN/miyuracine"

# ------------------------------------------------------------------ banc d'intrusion lot 3 (option)

if ((BANC)); then
	banc_sortie=$TMP/banc.tap
	# Répertoire de travail du banc : créé en root par le banc (mktemp), conservé pour
	# inspection ; le testeur ne le supprime pas (fichiers root).
	sudo -n "$PAQUET/banc/tests/intrusion-lot3/lancer.sh" --vm-jetable --sans-construction \
		--cible "$PAQUET/banc/cible" >"$banc_sortie" 2>&1
	rc_banc=$?
	ok_banc=$(grep -c '^ok ' "$banc_sortie")
	nok_banc=$(grep -c '^not ok ' "$banc_sortie")
	if ((rc_banc == 0 && nok_banc == 0 && ok_banc > 0)); then
		resultat ok "banc d'intrusion lot 3 : $ok_banc contrôles ok, $nok_banc not ok"
	else
		resultat nok "banc d'intrusion lot 3 : $ok_banc ok, $nok_banc not ok (code $rc_banc)" \
			"voir $banc_sortie (journal conservé avant nettoyage)"
		cp "$banc_sortie" "/home/cog/banc-echec.tap" 2>/dev/null || true
	fi
fi

# ------------------------------------------------------------------ sortie TAP

printf '1..%d\n' "$N"
printf '%s\n' "${SORTIE[@]}"
if ((ECHECS > 0)); then
	printf '# %d échec(s) sur %d contrôle(s)\n' "$ECHECS" "$N"
	exit 1
fi
printf '# %d contrôle(s) réussi(s)\n' "$N"
exit 0
