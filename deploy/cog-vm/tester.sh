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

for u in postgresql miyukini-douane-cog miyukini-douane-externe miyukini-attente-kindmother miyukini-kindmother miyukini-maia-llm \
	miyukini-maia miyukini-miyucloud miyukini-cog-web-portal miyukini-portail-entree miyukini-portail-sortie; do
	controle "$u actif" systemctl is-active --quiet "$u"
done

for u in miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal; do
	controle "$u : pas de redémarrage en boucle" \
		test "$(systemctl show -p NRestarts --value "$u")" = 0
done

# ------------------------------------------------------------------ écoute réseau

ecoute_sur() { ss -ltnH | awk -v a="$1" '$4 == a { f = 1 } END { exit !f }'; }
for pair in "KindMother:127.0.0.1:50051" "MAIA:127.0.0.1:11435" "maia-llm:127.0.0.1:11436" "MiyuCloud:127.0.0.1:8086" \
	"portail:127.0.0.1:8090" "PostgreSQL:127.0.0.1:5432"; do
	nom=${pair%%:*}
	adresse=${pair#*:}
	controle "$nom écoute sur $adresse" ecoute_sur "$adresse"
done

controle "aucune écoute hors boucle locale (hors SSH et résolveur systemd)" \
	bash -c '! ss -ltnH | awk "{ print \$4 }" | grep -vE "^(127\.|\[::1\]|\*:22\$|0\.0\.0\.0:22\$|\[::\]:22\$|0\.0\.0\.0:53\$)" | grep -q .'

# ------------------------------------------------------------------ Douane : zone COG

controle "Douane : unité de zone COG activée au démarrage" systemctl is-enabled --quiet miyukini-douane-cog
controle "Douane : table nftables miyudouane_hote chargée, sortie des comptes COG refusée (par compte)" \
	bash -c "sudo -n nft list table inet miyudouane_hote | grep -qE 'meta skuid .*drop'"
controle "Douane : table nftables de l'hôte chargée sans réécriture des autres tables (table inet miyudouane_hote seule)" \
	bash -c "sudo -n nft list tables | grep -q 'table inet miyudouane_hote'"
# M2 : la zone suit nftables (PartOf), les services COG suivent la zone (BindsTo), contrôle périodique.
controle "Douane : unité de zone rattachée à nftables.service (PartOf=)" \
	bash -c "systemctl show -p PartOf --value miyukini-douane-cog | grep -qw nftables.service"
controle "Douane : unité de zone rechargeable (ExecReload = nft -f du rendu miyudouane hote)" \
	bash -c "systemctl show -p ExecReload --value miyukini-douane-cog | grep -q '/etc/miyukini/douane-cog.nft'"
controle "Douane : minuteur de contrôle périodique de la zone actif (fail-closed)" \
	systemctl is-active --quiet miyukini-douane-cog-controle.timer
for u in miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal miyukini-portail-entree miyukini-portail-sortie; do
	controle "COG $u : lié à la zone (BindsTo=miyukini-douane-cog.service)" \
		bash -c "systemctl show -p BindsTo --value $u | grep -qw miyukini-douane-cog.service"
done
controle "Douane : description de zone générée (miyudouane-hote 2, flux boucle par compte : 3, dont maia -> maia-llm)" \
	bash -c "sudo -n head -c 4096 /etc/miyukini/douane-hote.desc | grep -q '^miyudouane-hote 2' && sudo -n grep -c '^boucle ' /etc/miyukini/douane-hote.desc | grep -qx 3 && sudo -n grep -qx 'boucle miyukini-maia tcp 127.0.0.1 11436' /etc/miyukini/douane-hote.desc && sudo -n grep -qx 'compte miyukini-maia-llm' /etc/miyukini/douane-hote.desc"

# Contrôle positif : le compte cog (non COG) atteint Internet ; sans cela l'isolement ne prouve rien.
controle "contrôle : un compte non COG atteint Internet (NAT de la VM)" \
	test "$(curl -s -m 10 -o /dev/null -w '%{http_code}' http://1.1.1.1/)" != 000

for u in miyukini-kindmother miyukini-maia miyukini-maia-llm miyukini-miyucloud miyukini-portail; do
	controle "COG $u : sortie vers Internet impossible (1.1.1.1:80 injoignable)" \
		test "$(sudo -n -u "$u" curl -s -m 5 -o /dev/null -w '%{http_code}' http://1.1.1.1/)" = 000
done
controle "MAIA /health 200 en boucle locale (lu par root : un compte COG n'a pas de flux vers son propre port)" \
	test "$(curl -s -m 5 -o /dev/null -w '%{http_code}' http://127.0.0.1:11435/health)" = 200
# M3 : un compte COG n'atteint en boucle locale que ses flux déclarés (directive « boucle » de
# miyudouane hote). Refus prouvé par le compteur nommé « rejets » de la table Douane.
controle "Douane : flux de boucle déclarés par compte (maia -> 50051, miyucloud -> 5432), aucun lo sans compte" bash -c '
	t=$(sudo -n nft list chain inet miyudouane_hote sortie)
	m="(\"?miyukini-maia\"?|$(id -u miyukini-maia))"; c="(\"?miyukini-miyucloud\"?|$(id -u miyukini-miyucloud))"
	grep -qE "meta skuid $m oifname \"lo\" ip daddr 127.0.0.1 tcp dport 50051 accept" <<<"$t" &&
	grep -qE "meta skuid $c oifname \"lo\" ip daddr 127.0.0.1 tcp dport 5432 accept" <<<"$t" &&
	grep -qE "meta skuid $m oifname \"lo\" ip daddr 127.0.0.1 tcp dport 11436 accept" <<<"$t" &&
	! grep -qE "^[[:space:]]*oifname \"lo\" accept" <<<"$t"'
controle "Douane : rejet des comptes COG journalisé (MDOU|zone-cog|sortie|SORTIE-COG|, NFLOG 110) puis compté" bash -c '
	t=$(sudo -n nft list chain inet miyudouane_hote sortie)
	grep -q "log prefix \"MDOU|zone-cog|sortie|SORTIE-COG|\" group 110" <<<"$t" &&
	grep -q "meta skuid @comptes_cog counter name \"rejets\" drop" <<<"$t"'
# Journal effectif : capture NFLOG (groupe 110) pendant une tentative 127.0.0.53:53 du compte MAIA.
if command -v tcpdump >/dev/null 2>&1; then
	controle "COG miyukini-maia : tentative 127.0.0.53:53 journalisée sur NFLOG 110 (préfixe SORTIE-COG)" bash -c '
		d=$(sudo -n mktemp -d /run/cog-nflog.XXXXXX); f=$d/capture.pcap
		sudo -n timeout 8 tcpdump -Z root -i nflog:110 -c 1 -U -w "$f" >/dev/null 2>&1 & p=$!
		sleep 2
		sudo -n -u miyukini-maia timeout 3 bash -c "exec 3<>/dev/tcp/127.0.0.53/53" 2>/dev/null
		wait $p
		r=1; sudo -n grep -aq "MDOU|zone-cog|sortie|SORTIE-COG|" "$f" && r=0
		sudo -n rm -rf "$d"; exit $r'
fi
rejets_zone() { sudo -n nft list counter inet miyudouane_hote rejets | sed -n 's/.*packets \([0-9]*\).*/\1/p'; }
export -f rejets_zone
controle "COG miyukini-maia : résolveur loopback 127.0.0.53:53 refusé (compteur rejets incrémenté)" bash -c '
	avant=$(rejets_zone)
	sudo -n -u miyukini-maia timeout 4 bash -c "exec 3<>/dev/tcp/127.0.0.53/53" 2>/dev/null
	apres=$(rejets_zone)
	[ -n "$avant" ] && [ "$apres" -gt "$avant" ]'
controle "COG miyukini-miyucloud : 127.0.0.1:8086 (son propre port, non déclaré) refusé (compteur rejets)" bash -c '
	avant=$(rejets_zone)
	sudo -n -u miyukini-miyucloud timeout 4 bash -c "exec 3<>/dev/tcp/127.0.0.1/8086" 2>/dev/null
	apres=$(rejets_zone)
	[ -n "$avant" ] && [ "$apres" -gt "$avant" ]'
controle "COG miyukini-portail : KindMother 127.0.0.1:50051 (flux d'un autre compte) refusé (compteur rejets)" bash -c '
	avant=$(rejets_zone)
	sudo -n -u miyukini-portail timeout 4 bash -c "exec 3<>/dev/tcp/127.0.0.1/50051" 2>/dev/null
	apres=$(rejets_zone)
	[ -n "$avant" ] && [ "$apres" -gt "$avant" ]'
controle "COG miyukini-miyucloud : PostgreSQL 127.0.0.1:5432 joignable (flux déclaré)" \
	sudo -n -u miyukini-miyucloud bash -c 'exec 3<>/dev/tcp/127.0.0.1/5432'
controle "COG miyukini-maia : KindMother joignable en boucle locale depuis son compte" \
	sudo -n -u miyukini-maia bash -c 'exec 3<>/dev/tcp/127.0.0.1/50051'
controle "COG miyukini-maia : maia-llm 127.0.0.1:11436 joignable (flux déclaré, A-026)" \
	sudo -n -u miyukini-maia bash -c 'exec 3<>/dev/tcp/127.0.0.1/11436'
controle "COG miyukini-maia-llm : aucun flux local (KindMother 127.0.0.1:50051 refusé, compteur rejets)" bash -c '
	avant=$(rejets_zone)
	sudo -n -u miyukini-maia-llm timeout 4 bash -c "exec 3<>/dev/tcp/127.0.0.1/50051" 2>/dev/null
	apres=$(rejets_zone)
	[ -n "$avant" ] && [ "$apres" -gt "$avant" ]'

# ------------------------------------------------------------------ Douane : frontière externe, débit tc (A-023, A-025)

NSX=mcog-f-externe
DX=$VAR/douane/externe
SBX=(--sandbox cog:voie-a --sandbox cog:voie-b --sandbox franche:voie-a --sandbox franche:voie-b --sandbox cog:capteur --sandbox locale:capteur)
controle "frontière externe : unité miyukini-douane-externe activée au démarrage" systemctl is-enabled --quiet miyukini-douane-externe
controle "frontière externe : état « filet » (règles strictes, puis tc, puis filet)" \
	bash -c "sudo -n /opt/miyukini/libexec/douane-externe.sh etat | grep -q '^filet '"
controle "frontière externe : espace $NSX avec ext-franche, ext0, wg-fed" bash -c \
	"for i in ext-franche ext0 wg-fed; do sudo -n ip -n $NSX link show \$i >/dev/null || exit 1; done"
controle "tc : modules sch_htb, sch_fq_codel, cls_fw chargés et déclarés au démarrage (modules-load.d)" bash -c \
	"for m in sch_htb sch_fq_codel cls_fw; do grep -q \"^\$m \" /proc/modules || exit 1; grep -qx \$m /etc/modules-load.d/miyukini-douane.conf || exit 1; done"
controle "tc : description miyudouane-tc 1, capacités ext0 122070 et wg-fed 12207 Kio/s" bash -c \
	"grep -qx 'miyudouane-tc 1' $ETC/douane-tc.desc && grep -qx 'capacite ext0 122070' $ETC/douane-tc.desc && grep -qx 'capacite wg-fed 12207' $ETC/douane-tc.desc"
for i in ext-franche ext0 wg-fed; do
	controle "tc : $i porte la racine htb 4d44: de la Douane (défaut ffff, file 4d45: fq_codel)" bash -c \
		"q=\$(sudo -n ip netns exec $NSX tc qdisc show dev $i) && grep -q '^qdisc htb 4d44: root .*default 0xffff' <<<\"\$q\" && grep -q '^qdisc fq_codel 4d45: parent 4d44:ffff' <<<\"\$q\""
done
for i in ext-franche ext0; do
	controle "tc : $i classe de voie 4d44:1000, feuille du flux 4d44:1 (512 Kio/s), filtre fw 0x1 -> 4d44:1" bash -c \
		"c=\$(sudo -n ip netns exec $NSX tc class show dev $i) && f=\$(sudo -n ip netns exec $NSX tc filter show dev $i) && grep -q 'class htb 4d44:1000 parent 4d44:f000' <<<\"\$c\" && grep -q 'class htb 4d44:1 parent 4d44:1000 .*rate 4194Kbit ceil 4194Kbit' <<<\"\$c\" && grep -q 'fw.*handle 0x1 classid 4d44:1' <<<\"\$f\""
done
# Audit : le texte appliqué est exactement le rendu audité de la Douane (arrêté signé en vigueur,
# description), et le noyau porte autant de classes, files et filtres que ce rendu, par interface.
controle "tc : texte appliqué = rendu audité de miyudouane tc (arrêté signé, description)" bash -c "
	a=\$(sudo -n cat $VAR/douane/arretes/ancre.hex | tr -d '\n')
	c=(); while read -r v; do c+=(--couper \"\$v\"); done < <(sudo -n cat $DX/coupees 2>/dev/null)
	diff <(sudo -n $BIN/miyudouane tc --description $ETC/douane-tc.desc --arrete $VAR/douane/arretes/externe.arrete --ancre \"\$a\" ${SBX[*]} \"\${c[@]}\") <(sudo -n cat $DX/$NSX.tc)"
controle "tc : état du noyau conforme au rendu (nombre de classes, files et filtres par interface)" bash -c "
	t=\$(sudo -n cat $DX/$NSX.tc)
	for i in ext-franche ext0 wg-fed; do
		[ \"\$(grep -c \"^class add dev \$i \" <<<\"\$t\")\" = \"\$(sudo -n ip netns exec $NSX tc class show dev \$i | grep -c '^class htb 4d44:')\" ] || exit 1
		[ \"\$(grep -c \"^qdisc add dev \$i \" <<<\"\$t\")\" = \"\$(sudo -n ip netns exec $NSX tc qdisc show dev \$i | grep -cE '^qdisc (htb|fq_codel) ')\" ] || exit 1
		[ \"\$(grep -c \"^filter add dev \$i \" <<<\"\$t\")\" = \"\$(sudo -n ip netns exec $NSX tc filter show dev \$i | grep -c 'fw.*classid')\" ] || exit 1
	done"
controle "tc : rendu sans del, replace, change ni action (qdisc add, class add, filter add seulement)" bash -c \
	"! sudo -n grep -vE '^(#|$|qdisc add dev |class add dev |filter add dev )' $DX/$NSX.tc | grep -q ."
# Filet seulement après tc : règle DEBIT par sens (5/4, rafale) en place, plus aucune règle stricte ;
# journal de l'unité : (a) puis (b) puis (c), dans cet ordre, au dernier démarrage de l'espace.
controle "nftables $NSX : filet 5/4 par sens (640 Kio/s, rafale 6843 o), aucune règle DEBIT stricte" bash -c "
	r=\$(sudo -n ip netns exec $NSX nft list table inet miyudouane)
	grep -q 'ct direction original limit rate over 640 kbytes/second burst 6843 bytes' <<<\"\$r\" &&
	grep -q 'ct direction reply limit rate over 640 kbytes/second burst 6843 bytes' <<<\"\$r\" &&
	! grep -E 'limit rate over 512 kbytes/second' <<<\"\$r\" | grep -q ."
controle "ordre au démarrage de l'espace : (a) règles strictes, puis (b) tc, puis (c) filet" bash -c "
	j=\$(sudo -n journalctl -o cat _SYSTEMD_INVOCATION_ID=\$(systemctl show -p InvocationID --value miyukini-douane-externe) | grep -oE '^\[douane-externe\] \((a|b|c)\) [^ ]+ [^ ]+' | cut -d' ' -f2 | tr -d '()' | tr -d '\n')
	[ \"\$j\" = abc ]"
controle "frontière externe : fichiers de la Douane hors /etc/miyukini/douane (réservé au banc), 0600" bash -c \
	"test ! -e $ETC/douane && test \"\$(sudo -n stat -c %a $DX/$NSX.tc)\" = 600 && test \"\$(sudo -n stat -c %a $DX/$NSX.nft)\" = 600"

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

# Les refus hors boucle locale se testent avec une COPIE du binaire, hors de son chemin : le profil
# AppArmor s'attache au chemin /opt/miyukini/bin/<service>, la copie n'est pas confinée. Le test
# porte sur le code de refus, pas sur le confinement (couvert par le contrôle « enforce » ci-dessus).
refus_km() {
	local d="$TMP/km-hors-boucle" rc
	mkdir -p "$d"
	install -m 0755 "$BIN/kindmother-server" "$TMP/kindmother-essai"
	KINDMOTHER_DATA_DIR="$d" KINDMOTHER_LISTEN_ADDR=0.0.0.0:19877 timeout 90 "$TMP/kindmother-essai" \
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
# Routes mémoire : 503 tant que la mémoire n'est pas configurée ; une fois la configuration
# d'essai posée par scenarios.sh (drop-in), elles sont actives et refusent sans signature (401).
if [[ -e /etc/systemd/system/miyukini-maia.service.d/memoire-essai.conf ]]; then
	controle "MAIA routes mémoire actives (configuration d'essai) : sans signature refusé (401)" \
		test "$(code_http -X POST -H 'content-type: application/json' -d '{}' http://127.0.0.1:11435/v1/memoire/tests/consulter)" = 401
else
	controle "MAIA routes mémoire désactivées par défaut (503)" \
		test "$(code_http -X POST -H 'content-type: application/json' -d '{}' http://127.0.0.1:11435/v1/memoire/tests/consulter)" = 503
fi
# MAIA n'ouvre la mémoire qu'une fois, au démarrage : elle doit démarrer après KindMother et
# attendre que son port écoute (sinon, au redémarrage de la VM, la mémoire reste désactivée).
controle "MAIA démarre après KindMother et après l'attente de son port (ordre au démarrage)" bash -c \
	"systemctl show miyukini-maia -p After --value | grep -q 'miyukini-kindmother.service' && systemctl show miyukini-maia -p After --value | grep -q 'miyukini-attente-kindmother.service' && systemctl cat miyukini-attente-kindmother | grep -q '127.0.0.1/50051'"

refus_maia() {
	local d="$TMP/maia-hors-boucle" rc
	mkdir -p "$d"
	printf 'bind_addr = "0.0.0.0:19876"\n' >"$d/bridge.toml"
	install -m 0755 "$BIN/maia" "$TMP/maia-essai"
	MIYUKINI_DATA_DIR="$d" timeout 30 "$TMP/maia-essai" >"$TMP/maia.log" 2>&1
	rc=$?
	[[ $rc -ne 0 ]] && grep -q 'Refus de démarrer' "$TMP/maia.log"
}
controle "MAIA refuse de démarrer hors boucle locale sans appelant épinglé (démarrage jetable)" refus_maia

# ------------------------------------------------------------------ maia-llm et mini modèle (A-026)

# Requêtes signées (miyauth : Ed25519 par openssl) avec une graine brute de 32 octets.
cat >"$TMP/signe.py" <<'PY'
import base64, hashlib, os, struct, subprocess, sys, tempfile, time
PKCS8 = bytes.fromhex("302e020100300506032b657004220420")
def b64u(o): return base64.urlsafe_b64encode(o).rstrip(b"=").decode()
def lp(o): return struct.pack(">H", len(o)) + o
graine = bytes.fromhex(open(sys.argv[1]).read().strip())
usage, methode, chemin, corps = sys.argv[2], sys.argv[3], sys.argv[4], open(sys.argv[5], "rb").read()
ts, nonce = int(time.time()), os.urandom(16)
m = (b"miyukini/requete-signee/v1\0" + lp(usage.encode()) + lp(methode.encode()) + lp(chemin.encode())
     + struct.pack(">Q", ts) + lp(nonce) + hashlib.sha256(corps).digest())
d = tempfile.mkdtemp()
try:
    open(d + "/k.der", "wb").write(PKCS8 + graine)
    subprocess.run(["openssl", "pkey", "-inform", "DER", "-in", d + "/k.der", "-out", d + "/k.pem"], check=True, capture_output=True)
    pub = subprocess.run(["openssl", "pkey", "-inform", "DER", "-in", d + "/k.der", "-pubout", "-outform", "DER"], check=True, capture_output=True).stdout[-32:]
    open(d + "/m", "wb").write(m)
    sig = subprocess.run(["openssl", "pkeyutl", "-sign", "-rawin", "-inkey", d + "/k.pem", "-in", d + "/m"], check=True, capture_output=True).stdout
finally:
    subprocess.run(["rm", "-rf", d])
for k, v in (("x-miyu-signataire", b64u(pub)), ("x-miyu-horodatage", str(ts)), ("x-miyu-nonce", b64u(nonce)), ("x-miyu-signature", b64u(sig))):
    print("-H"); print(f"{k}: {v}")
PY
LLM=http://127.0.0.1:11436
controle "maia-llm : modèle Qwen2.5-1.5B chargé, licence Apache-2.0, CPU (gpu_layers 0)" bash -c \
	"h=\$(curl -fsS --max-time 10 $LLM/maia/llm/health) && grep -q '\"model_loaded\":true' <<<\"\$h\" && grep -q '\"licence\":\"Apache-2.0\"' <<<\"\$h\" && grep -q '\"gpu_layers\":0' <<<\"\$h\" && grep -qi 'qwen2.5-1.5b-instruct' <<<\"\$h\""
controle "maia-llm : modèle du canal conforme (root 0644, SHA-256 consigné A-026, manifeste signé présent)" bash -c \
	"f=$VAR/modeles/qwen2.5-1.5b-instruct-q4_k_m.gguf && test \"\$(stat -c '%U %a %s' \$f)\" = 'root 644 1117320736' && test -f \$f.sceau && test \"\$(stat -c '%U %a' $VAR/modeles)\" = 'root 755' && sha256sum \$f | grep -q '^6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e '"
controle "maia-llm : /v1/chat/completions sans signature refusé (401)" \
	test "$(code_http -X POST -H 'content-type: application/json' -d '{}' $LLM/v1/chat/completions)" = 401
controle "maia-llm : /maia/llm/models sans signature refusé (401)" test "$(code_http $LLM/maia/llm/models)" = 401
controle "maia-llm : configuration root 0644 (/etc/miyukini/maia-llm.toml), liée en lecture seule, port 11436, ancre de développement" bash -c \
	"test \"\$(stat -c '%a %U' $ETC/maia-llm.toml)\" = '644 root' && systemctl show -p BindReadOnlyPaths --value miyukini-maia-llm | grep -qF '$ETC/maia-llm.toml:$VAR/maia-llm/maia-llm.toml' && grep -qx 'bind_addr = \"127.0.0.1:11436\"' $ETC/maia-llm.toml && grep -q '^model_trust_anchors = \[\"fb9d57a05cf97bb2cdce1fa58d17e5a8d4ac1ecc38e67c906f92853e7474611a\"\]' $ETC/maia-llm.toml && grep -qx 'gpu_layers = 0' $ETC/maia-llm.toml"
controle "maia-llm : modèles en lecture seule dans le service (ReadOnlyPaths), aucun périphérique, MemoryMax 3G" bash -c \
	"systemctl show -p ReadOnlyPaths --value miyukini-maia-llm | grep -qw $VAR/modeles && test \"\$(systemctl show -p PrivateDevices --value miyukini-maia-llm)\" = yes && test \"\$(systemctl show -p MemoryMax --value miyukini-maia-llm)\" = 3221225472"
controle "MAIA : MAIA_LLM_URL vers maia-llm (11436), après et avec miyukini-maia-llm" bash -c \
	"systemctl show -p Environment --value miyukini-maia | grep -q 'MAIA_LLM_URL=http://127.0.0.1:11436' && systemctl show -p After --value miyukini-maia | grep -qw miyukini-maia-llm.service && systemctl show -p Wants --value miyukini-maia | grep -qw miyukini-maia-llm.service"
controle "MAIA : clé de service 0600 au compte miyukini-maia, épinglée par maia-llm (inférence et modèles)" bash -c \
	"f=$VAR/maia/keys/maia.seed && test \"\$(sudo -n stat -c '%a %U %s' \$f)\" = '600 miyukini-maia 32' && p=\$(sudo -n cat \$f | { printf '\x30\x2e\x02\x01\x00\x30\x05\x06\x03\x2b\x65\x70\x04\x22\x04\x20'; cat; } | openssl pkey -inform DER -pubout -outform DER | tail -c 32 | od -An -tx1 | tr -d ' \n') && grep -q \"^appelants_inference = \[\\\"\$p\\\"\" $ETC/maia-llm.toml && grep -qx \"appelants_modeles = \[\\\"\$p\\\"\]\" $ETC/maia-llm.toml"
# Réponse réelle du modèle en CPU, à une requête signée par la clé de MAIA (lue par root) : texte
# non vide, marqué généré par IA.
llm_repond() {
	local g=$TMP/maia.hex corps=$TMP/llm.json h r
	sudo -n od -An -tx1 "$VAR/maia/keys/maia.seed" | tr -d ' \n' >"$g"
	printf '%s' '{"messages":[{"role":"user","content":"Dis bonjour en un mot."}],"max_tokens":12,"temperature":0,"seed":7}' >"$corps"
	mapfile -t h < <(python3 -I "$TMP/signe.py" "$g" maia-llm/inference POST /v1/chat/completions "$corps")
	rm -f "$g"
	r=$(curl -sS --max-time 120 -X POST -H 'content-type: application/json' "${h[@]}" --data-binary @"$corps" $LLM/v1/chat/completions) || return 1
	printf '%s\n' "$r" >"$TMP/llm.out"
	grep -q '"genere_par_ia":true' <<<"$r" && grep -q '"moteur":"maia-llm"' <<<"$r" &&
		python3 -I -c 'import json,sys; c=json.load(sys.stdin)["choices"][0]["message"]["content"]; sys.exit(0 if len(c.strip())>0 else 1)' <<<"$r"
}
controle "maia-llm : réponse réelle du modèle (CPU) à une requête signée par MAIA, marquée IA (x_miyukini)" llm_repond
controle "MAIA : /maia/presentation (mention IA, AI Act art. 50) : « intelligence artificielle »" bash -c \
	"curl -fsS --max-time 10 http://127.0.0.1:11435/maia/presentation | grep -q '\"nature\":\"intelligence artificielle\"'"
controle "Alicia : [moteur_ia] url = maia-llm (http://127.0.0.1:11436) dans $ETC/alicia.toml" bash -c \
	"grep -qx 'url = \"http://127.0.0.1:11436\"' $ETC/alicia.toml && grep -qx '\[moteur_ia\]' $ETC/alicia.toml"

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

# ------------------------------------------------------------------ AppArmor (enforce)

# Les cinq profils sont chargés en mode enforce (aucun en complain).
for p in miyukini-kindmother miyukini-maia miyukini-maia-llm miyukini-miyucloud miyukini-cog-web-portal miyukini-portail-sas \
	miyukini-portail-entree miyukini-portail-sortie; do
	controle "AppArmor : profil $p chargé en enforce" \
		sudo -n grep -q "^$p (enforce)\$" /sys/kernel/security/apparmor/profiles
done
controle "AppArmor : aucun profil miyukini-* en complain" \
	bash -c "! sudo -n grep -q '^miyukini-.* (complain)\$' /sys/kernel/security/apparmor/profiles"
for u in miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal miyukini-portail-entree miyukini-portail-sortie; do
	controle "$u : processus principal confiné par son profil (enforce)" bash -c \
		"pid=\$(systemctl show -p MainPID --value $u) && sudo -n cat /proc/\$pid/attr/current | grep -qE '^$u(//&unconfined)? \\(enforce\\)\$'"
done

# ------------------------------------------------------------------ durcissement et permissions

for u in miyukini-kindmother miyukini-maia miyukini-maia-llm miyukini-miyucloud miyukini-portail miyukini-portail-entree \
	miyukini-portail-sortie miyukini-facade miyukini-moteur; do
	controle "compte de service $u : UID système, shell nologin" bash -c \
		"e=\$(getent passwd $u) && uid=\$(echo \"\$e\" | cut -d: -f3) && sh=\$(echo \"\$e\" | cut -d: -f7) && test \"\$uid\" -lt 1000 && test \"\$sh\" = /usr/sbin/nologin"
done

for svc in kindmother:miyukini-kindmother maia:miyukini-maia maia-llm:miyukini-maia-llm miyucloud:miyukini-miyucloud cog-web-portal:miyukini-portail; do
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
	"cd /opt/miyukini && grep -v '^#' MANIFESTE | grep '  \\./bin/' | grep -v 'bin/arretes_essai$' | sha256sum --check --strict --quiet -"

propriete_egale() { [[ "$(systemctl show -p "$2" --value "$1")" == "$3" ]]; }
for u in miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal miyukini-portail-entree miyukini-portail-sortie; do
	controle "$u : NoNewPrivileges=yes" propriete_egale "$u" NoNewPrivileges yes
	controle "$u : ProtectSystem=strict" propriete_egale "$u" ProtectSystem strict
	controle "$u : PrivateTmp=yes" propriete_egale "$u" PrivateTmp yes
done

# ------------------------------------------------------------------ Portail scindé (A-022 §3)

PF=$VAR/portail-files
# Processus sous leurs comptes, l'un sans l'autre.
for m in entree sortie; do
	controle "portail-$m : processus principal sous le compte miyukini-portail-$m" bash -c \
		"pid=\$(systemctl show -p MainPID --value miyukini-portail-$m) && test \"\$(ps -o user= -p \$pid)\" = miyukini-portail-$m"
	controle "portail-$m : sans réseau (PrivateNetwork=yes)" propriete_egale "miyukini-portail-$m" PrivateNetwork yes
	controle "portail-$m : UMask=0027" propriete_egale "miyukini-portail-$m" UMask 0027
done
controle "portail : entrée et sortie indépendantes (aucune ne requiert, ne lie ni n'attend l'autre)" bash -c '
	for a in entree sortie; do b=entree; [ $a = entree ] && b=sortie
		systemctl show -p Requires -p BindsTo -p Requisite -p After -p PartOf --value miyukini-portail-$a | grep -q "miyukini-portail-$b" && exit 1
	done; exit 0'
# Groupes de file : chaque file réunit son seul écrivain et son seul lecteur ; aucun groupe commun.
controle "portail : groupes de file (q-depot, q-admises) pour l'entrée seulement" bash -c \
	"g=\$(id -nG miyukini-portail-entree) && [[ \$g == *miyukini-q-depot* && \$g == *miyukini-q-admises* && \$g != *miyukini-q-reponses* && \$g != *miyukini-q-publication* ]]"
controle "portail : groupes de file (q-reponses, q-publication) pour la sortie seulement" bash -c \
	"g=\$(id -nG miyukini-portail-sortie) && [[ \$g == *miyukini-q-reponses* && \$g == *miyukini-q-publication* && \$g != *miyukini-q-depot* && \$g != *miyukini-q-admises* ]]"
controle "portail : entrée et sortie sans aucun groupe commun" bash -c \
	"! comm -12 <(id -G miyukini-portail-entree | tr ' ' '\n' | sort) <(id -G miyukini-portail-sortie | tr ' ' '\n' | sort) | grep -q ."
for ligne in "depot miyukini-portail-entree" "admises miyukini-moteur" "reponses miyukini-portail-sortie" "publication miyukini-facade"; do
	read -r f lecteur <<<"$ligne"
	controle "portail : file $f à son lecteur $lecteur, groupe miyukini-q-$f, 3730 (rien pour les autres)" bash -c \
		"test \"\$(stat -c '%a %U %G' $PF/$f)\" = '3730 $lecteur miyukini-q-$f'"
done
for m in entree sortie; do
	controle "portail-$m : état propre $VAR/portail-$m 0700 à son compte" bash -c \
		"test \"\$(sudo -n stat -c '%a %U' $VAR/portail-$m)\" = '700 miyukini-portail-$m'"
	controle "portail-$m : descripteurs $ETC/portail-$m/voies root, groupe du compte, 2750" bash -c \
		"test \"\$(sudo -n stat -c '%a %U %G' $ETC/portail-$m/voies)\" = '2750 root miyukini-portail-$m'"
done
controle "portail : aucun état commun (états, configurations et files distincts, aucun lien)" bash -c \
	"test \"\$(sudo -n stat -c %i $VAR/portail-entree)\" != \"\$(sudo -n stat -c %i $VAR/portail-sortie)\" && ! sudo -n find $VAR/portail-entree $VAR/portail-sortie $PF -type l | grep -q ."
controle "portail-entree : files de la sortie inaccessibles dans son espace (InaccessiblePaths)" bash -c \
	"systemctl show -p InaccessiblePaths --value miyukini-portail-entree | grep -q '$PF/publication'"
controle "portail-sortie : files de l'entrée inaccessibles dans son espace (InaccessiblePaths)" bash -c \
	"systemctl show -p InaccessiblePaths --value miyukini-portail-sortie | grep -q '$PF/depot'"
controle "portail-entree : démarrage annoncé « relève seule, aucune capacité de publier »" bash -c \
	"sudo -n journalctl -u miyukini-portail-entree --no-pager -o cat | grep -q 'aucune capacité de publier'"
controle "portail-sortie : démarrage annoncé « aucune capacité de relever »" bash -c \
	"sudo -n journalctl -u miyukini-portail-sortie --no-pager -o cat | grep -q 'aucune capacité de relever'"

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
	# Le banc exige qu'aucun espace mcog-* n'existe : la frontière externe installée est arrêtée
	# (ExecStop retire ses espaces) pendant le banc, puis redémarrée (règles strictes → tc → filet).
	sudo -n systemctl stop miyukini-douane-externe
	sudo -n "$PAQUET/banc/tests/intrusion-lot3/lancer.sh" --vm-jetable --sans-construction \
		--cible "$PAQUET/banc/cible" >"$banc_sortie" 2>&1
	rc_banc=$?
	sudo -n systemctl start miyukini-douane-externe
	controle "frontière externe redémarrée après le banc (état filet)" bash -c \
		"systemctl is-active --quiet miyukini-douane-externe && sudo -n /opt/miyukini/libexec/douane-externe.sh etat | grep -q '^filet '"
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
