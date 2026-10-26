#!/usr/bin/env bash
# Scénarios LIVE du COG dans la VM jetable cog-essai — sortie TAP.
#
# INTERDIT SUR UN HÔTE RÉEL (voir README.md) : le script écrit dans la base PostgreSQL
# de MiyuCloud, crée un compte administrateur, redémarre miyukini-miyucloud et modifie
# la configuration systemd de MAIA (drop-in) de la VM. Il ne touche pas à l'hôte.
#
# Usage :
#   deploy/cog-vm/scenarios.sh --distant        (hôte) copie ce script et les migrations dans la VM
#                                               et l'y exécute ; code de retour = nombre de « not ok »
#   bash /home/cog/cog-scenarios/scenarios.sh   (VM) exécution directe
#
# Idempotent : le compte administrateur est créé au premier passage (si le système n'est pas
# initialisé) et ses identifiants sont conservés dans ~/.cog-scenarios/admin.env (0600).
# Chaque passage utilise des noms uniques (suffixe horodaté).
set -uo pipefail

# ─────────────────────────────────────────────────────────────── mode hôte (--distant)
if [[ ${1:-} == --distant ]]; then
	ICI=$(cd "$(dirname "$0")" && pwd)
	RACINE=$(cd "$ICI/../.." && pwd)
	VM=${COG_VM:-$HOME/vm/cog-essai}
	SSH_OPTS=(-q -o BatchMode=yes -o StrictHostKeyChecking=no -o "UserKnownHostsFile=$VM/known_hosts" -i "$VM/cle_vm" -P ${COG_SSH_PORT:-2222})
	DEST=/home/cog/cog-scenarios
	"$VM/ssh-vm.sh" "rm -rf '$DEST' && mkdir -p '$DEST/migrations'"
	scp "${SSH_OPTS[@]}" "$ICI/scenarios.sh" "cog@127.0.0.1:$DEST/scenarios.sh"
	scp "${SSH_OPTS[@]}" "$RACINE"/apps/miyucloud/migrations/*.sql "cog@127.0.0.1:$DEST/migrations/"
	if [[ -n ${COG_PROPOSER:-} ]]; then
		scp "${SSH_OPTS[@]}" "$COG_PROPOSER" "cog@127.0.0.1:$DEST/cog-proposer"
	fi
	"$VM/ssh-vm.sh" "bash '$DEST/scenarios.sh'"
	exit $?
fi

# ─────────────────────────────────────────────────────────────── exécution dans la VM
B=${COG_MIYUCLOUD:-http://127.0.0.1:8086}
MAIA=${COG_MAIA:-http://127.0.0.1:11435}
KM=${COG_KINDMOTHER:-127.0.0.1:50051}
HERE=$(cd "$(dirname "$0")" && pwd)
MIG=$HERE/migrations
W=$(mktemp -d "${TMPDIR:-/tmp}/cog-scen.XXXXXX")
trap 'rm -rf "$W"' EXIT
SUFFIXE=$(date +%s)$$
N=0
NOK=0
# Début du scénario, pour le journal noyau (refus AppArmor pendant le passage).
DEBUT_NOYAU=$(date '+%Y-%m-%d %H:%M:%S')

ok() { N=$((N + 1)); printf 'ok %d - %s\n' "$N" "$1"; }
nok() { N=$((N + 1)); NOK=$((NOK + 1)); printf 'not ok %d - %s%s\n' "$N" "$1" "${2:+ # $2}"; }
# verifie NOM COMMANDE… : ok si la commande réussit
verifie() { local nom=$1; shift; if "$@" >"$W/verifie.out" 2>&1; then ok "$nom"; else nok "$nom" "$(head -c 300 "$W/verifie.out" | tr '\n' ' ')"; fi; }
# egalite NOM ATTENDU OBTENU
egalite() { if [[ $2 == "$3" ]]; then ok "$1"; else nok "$1" "attendu « $2 », obtenu « $3 »"; fi; }

sql() { sudo -n -u postgres psql -d miyucloud -X -A -t -q -v ON_ERROR_STOP=1 -c "$1"; }
blob_chemin() { sudo -n find /var/lib/miyukini/miyucloud/storage -name "$1.blob" -print -quit; }
# req METHODE URL SORTIE [args curl…] → affiche le code HTTP ; en-têtes dans SORTIE.h
req() {
	local m=$1 u=$2 out=$3
	shift 3
	curl -s -X "$m" -o "$out" -D "$out.h" -w '%{http_code}' "$@" "$u"
}

# ─────────────────────────────────────────────────────────────── fabriques de fichiers
cat >"$W/gen.py" <<'PY'
# Génère un PNG 16×16 avec un chunk eXIf (TIFF : Make) et un tEXt portant un marqueur.
import struct, sys, zlib
def morceau(t, d):
    return struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d) & 0xFFFFFFFF)
def png(graine, marqueur):
    w = h = 16
    lignes = b"".join(b"\x00" + bytes(v for x in range(w) for v in ((x * 17 + graine) & 255, (y * 23 + graine) & 255, (x * y + graine) & 255)) for y in range(h))
    valeur = marqueur.encode() + b"\x00"
    entree = struct.pack("<HHII", 0x010F, 2, len(valeur), 8 + 2 + 12 + 4)
    tiff = b"II*\x00" + struct.pack("<I", 8) + struct.pack("<H", 1) + entree + b"\x00\x00\x00\x00" + valeur
    return (b"\x89PNG\r\n\x1a\n" + morceau(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
            + morceau(b"eXIf", tiff) + morceau(b"tEXt", b"Comment\x00" + marqueur.encode())
            + morceau(b"IDAT", zlib.compress(lignes)) + morceau(b"IEND", b""))
sys.stdout.buffer.write(png(int(sys.argv[1]), sys.argv[2]))
PY
MARQUEUR_EXIF="COGEXIF-$SUFFIXE"
python3 -I "$W/gen.py" 42 "$MARQUEUR_EXIF" >"$W/exif.png"
python3 -I "$W/gen.py" 77 "COGCOPIE-$SUFFIXE" >"$W/copie.png"
printf '%%PDF-1.4\n1 0 obj << /Type /Catalog >> endobj\n%%%%EOF\n%s\n' "$SUFFIXE" >"$W/doc.pdf"
# Polyglotte : en-tête PNG déclaré, contenu PE (MZ) — ne doit jamais être servi sans accord.
{ printf '\x89PNG\r\n\x1a\n'; printf 'MZ\x90\x00\x03\x00\x00\x00This program cannot be run in DOS mode.'; head -c 300 /dev/urandom; } >"$W/polyglotte.png"

# Outils de test (Ed25519 via openssl, requêtes signées miyauth, poignée de main KindMother).
cat >"$W/cog_test.py" <<'PY'
import base64, hashlib, os, socket, struct, subprocess, sys, tempfile, json, time
PKCS8 = bytes.fromhex("302e020100300506032b657004220420")
DOMAINE = b"miyukini/requete-signee/v1"
def b64u(o):
    return base64.urlsafe_b64encode(o).rstrip(b"=").decode()
def cle(graine):
    d = tempfile.mkdtemp(prefix="cog-cle-")
    der, pem = os.path.join(d, "k.der"), os.path.join(d, "k.pem")
    open(der, "wb").write(PKCS8 + graine)
    subprocess.run(["openssl", "pkey", "-inform", "DER", "-in", der, "-out", pem], check=True, capture_output=True)
    pub = subprocess.run(["openssl", "pkey", "-inform", "DER", "-in", der, "-pubout", "-outform", "DER"], check=True, capture_output=True).stdout[-32:]
    return d, pem, pub
def pub_de(graine):
    d, _pem, pub = cle(graine)
    subprocess.run(["rm", "-rf", d], check=False)
    return pub
def signe(graine, message):
    d, pem, pub = cle(graine)
    try:
        m = os.path.join(d, "m.bin")
        open(m, "wb").write(message)
        sig = subprocess.run(["openssl", "pkeyutl", "-sign", "-rawin", "-inkey", pem, "-in", m], check=True, capture_output=True).stdout
        return pub, sig
    finally:
        subprocess.run(["rm", "-rf", d], check=False)
def lp(o):
    return struct.pack(">H", min(len(o), 0xFFFF)) + o
def message(usage, methode, chemin, ts, nonce, corps):
    return (DOMAINE + b"\0" + lp(usage.encode()) + lp(methode.encode()) + lp(chemin.encode())
            + struct.pack(">Q", ts) + lp(nonce) + hashlib.sha256(corps).digest())
def entetes(graine_hex, usage, methode, chemin, corps, ts=None, nonce=None):
    ts = int(time.time()) if ts is None else ts
    nonce = os.urandom(16) if nonce is None else nonce
    pub, sig = signe(bytes.fromhex(graine_hex), message(usage, methode, chemin, ts, nonce, corps))
    return {"x-miyu-signataire": b64u(pub), "x-miyu-horodatage": str(ts),
            "x-miyu-nonce": b64u(nonce), "x-miyu-signature": b64u(sig)}
def trame_envoi(s, o):
    b = json.dumps(o).encode()
    s.sendall(struct.pack(">I", len(b)) + b)
def trame_lecture(s):
    h = b""
    while len(h) < 4:
        c = s.recv(4 - len(h))
        if not c:
            return None
        h += c
    n = struct.unpack(">I", h)[0]
    out = b""
    while len(out) < n:
        c = s.recv(n - len(out))
        if not c:
            break
        out += c
    return json.loads(out) if out else None
def km_poignee(adresse, operateur, graine_hex, epingle_hex):
    """Poignée de main kindmother-auth-v2 ; renvoie le type de la réponse finale."""
    hote, port = adresse.rsplit(":", 1)
    s = socket.create_connection((hote, int(port)), timeout=5)
    try:
        cn = os.urandom(32)
        trame_envoi(s, {"type": "ClientHello", "protocol": "kindmother-auth-v2", "client_nonce": cn.hex()})
        d = trame_lecture(s)
        if not d or d.get("type") != "Challenge":
            return "pas_de_defi:" + str(d and d.get("type"))
        if epingle_hex and d["server_pub"] != epingle_hex:
            return "cle_serveur_non_epinglee"
        sn, spub = bytes.fromhex(d["server_nonce"]), bytes.fromhex(d["server_pub"])
        transcription = b"kindmother-auth-v2\0" + b"client\0" + cn + sn + spub + operateur.encode()
        if graine_hex is None:
            sig = bytes(64)
        else:
            sig = signe(bytes.fromhex(graine_hex), transcription)[1]
        trame_envoi(s, {"type": "Authenticate", "operator_id": operateur, "signature": sig.hex()})
        r = trame_lecture(s)
        return "fin_de_connexion" if r is None else r.get("type", "?")
    except OSError as e:
        return "erreur_reseau:" + type(e).__name__
    finally:
        s.close()
if __name__ == "__main__":
    c = sys.argv[1]
    if c == "pub":
        print(pub_de(bytes.fromhex(sys.argv[2])).hex())
    elif c == "entetes":
        # entetes GRAINE USAGE METHODE CHEMIN CORPS_FICHIER|- [TS [NONCE_HEX]]
        corps = open(sys.argv[6], "rb").read() if sys.argv[6] != "-" else b""
        ts = int(sys.argv[7]) if len(sys.argv) > 7 and sys.argv[7] else None
        nonce = bytes.fromhex(sys.argv[8]) if len(sys.argv) > 8 and sys.argv[8] else None
        for k, v in entetes(sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5], corps, ts, nonce).items():
            print(f"{k}: {v}")
    elif c == "km":
        # km ADRESSE OPERATEUR GRAINE_HEX|- EPINGLE_HEX|-
        print(km_poignee(sys.argv[2], sys.argv[3], None if sys.argv[4] == "-" else sys.argv[4], None if sys.argv[5] == "-" else sys.argv[5]))
PY

# ─────────────────────────────────────────────────────────────── 1. migrations
echo "# 1. migrations MiyuCloud (PostgreSQL)"
verifie "migrations de l'instance : toutes réussies, dont 20261025000002_cdr_reclassement" bash -c "
	n=\$(sudo -n -u postgres psql -d miyucloud -X -A -t -c \"select count(*) from _sqlx_migrations where success\")
	t=\$(sudo -n -u postgres psql -d miyucloud -X -A -t -c \"select count(*) from _sqlx_migrations where not success\")
	r=\$(sudo -n -u postgres psql -d miyucloud -X -A -t -c \"select count(*) from _sqlx_migrations where version=20261025000002 and success\")
	[ \"\$n\" -ge 10 ] && [ \"\$t\" = 0 ] && [ \"\$r\" = 1 ]"

MIGTEST=miyucloud_migtest_$SUFFIXE
sudo -n -u postgres createdb "$MIGTEST" 2>/dev/null
mig_ok=1
for f in "$MIG"/*.sql; do
	# Lecture par l'entrée standard : le rôle postgres ne peut pas lire le répertoire personnel.
	if ! sudo -n -u postgres psql -d "$MIGTEST" -X -q -v ON_ERROR_STOP=1 --single-transaction <"$f" >"$W/mig.out" 2>&1; then
		mig_ok=0
		nok "migration réelle $(basename "$f")" "$(head -c 300 "$W/mig.out" | tr '\n' ' ')"
	fi
done
[[ $mig_ok == 1 ]] && ok "migrations rejouées sur une base vierge, en transaction, ON_ERROR_STOP ($(ls "$MIG"/*.sql | wc -l) fichiers)"
verifie "index partiel de reclassement CDR présent (20261025000002)" bash -c \
	"sudo -n -u postgres psql -d '$MIGTEST' -X -A -t -c \"select 1 from pg_indexes where indexname='idx_files_etat_cdr_inconnu'\" | grep -q 1"
sudo -n -u postgres dropdb "$MIGTEST" 2>/dev/null

# ─────────────────────────────────────────────────────────────── 2. premier administrateur
echo "# 2. premier administrateur et connexion par l'API"
STATUT=$(curl -s "$B/api/auth/status")
if [[ $(jq -r .initialized <<<"$STATUT") == false ]]; then
	ADM_USER=admin
	ADM_PW=$(openssl rand -base64 24)
	umask 077
	mkdir -p "$HOME/.cog-scenarios" && chmod 700 "$HOME/.cog-scenarios"
	printf 'COG_ADMIN_USER=%s\nCOG_ADMIN_PASSWORD=%s\n' "$ADM_USER" "$ADM_PW" >"$HOME/.cog-scenarios/admin.env"
	jq -n --arg u "$ADM_USER" --arg p "$ADM_PW" '{username:$u,email:"admin@cog.test",password:$p}' >"$W/setup.json"
	c=$(req POST "$B/api/setup" "$W/setup.out" -H 'content-type: application/json' --data-binary @"$W/setup.json")
	egalite "premier administrateur créé par POST /api/setup" 201 "$c"
fi
# shellcheck disable=SC1091
. "$HOME/.cog-scenarios/admin.env" || { nok "identifiants administrateur présents"; exit 1; }
egalite "système initialisé (GET /api/auth/status)" true "$(curl -s "$B/api/auth/status" | jq -r .initialized)"
jq -n --arg u "$COG_ADMIN_USER" --arg p "$COG_ADMIN_PASSWORD" '{username:$u,password:$p}' >"$W/login.json"
c=$(req POST "$B/api/auth/login" "$W/login.out" -H 'content-type: application/json' --data-binary @"$W/login.json")
egalite "connexion par l'API (POST /api/auth/login)" 200 "$c"
TOK=$(jq -r .access_token "$W/login.out")
[[ -n $TOK && $TOK != null ]] && ok "jeton d'accès reçu" || nok "jeton d'accès reçu"
jq -n '{username:"admin",password:"mauvais-mot-de-passe-COG"}' >"$W/mauvais.json"
c=$(req POST "$B/api/auth/login" "$W/mauvais.out" -H 'content-type: application/json' --data-binary @"$W/mauvais.json")
[[ $c == 401 || $c == 403 ]] && ok "mauvais mot de passe refusé ($c, « Invalid credentials »)" || nok "mauvais mot de passe refusé" "code $c"
c=$(req GET "$B/api/files" "$W/sans.out")
egalite "API protégée sans session refusée (401)" 401 "$c"
AUTH=(-H "Authorization: Bearer $TOK")
DOSSIER=$(curl -s "${AUTH[@]}" "$B/api/folders" | jq -r '.[0].id')
[[ -n $DOSSIER && $DOSSIER != null ]] && ok "dossier personnel accessible ($DOSSIER)" || nok "dossier personnel accessible"

# Téléversement helper : televerse CHEMIN NOM TYPE SORTIE → code HTTP ; id dans SORTIE.id
televerse() {
	local c
	c=$(req POST "$B/api/files/upload" "$4" "${AUTH[@]}" -F "folder_id=$DOSSIER" -F "file=@$1;filename=$2;type=$3")
	jq -r '.id // empty' "$4" >"$4.id" 2>/dev/null
	echo "$c"
}

# ─────────────────────────────────────────────────────────────── 3. EXIF : désarmement
echo "# 3. PNG avec métadonnées EXIF : désarmement, contenu chiffré au repos"
c=$(televerse "$W/exif.png" "exif-$SUFFIXE.png" image/png "$W/up-exif.out")
egalite "téléversement PNG EXIF (201)" 201 "$c"
FID_EXIF=$(cat "$W/up-exif.out.id")
ETAT=$(sql "select etat_cdr from storage.files where id='$FID_EXIF'" | tr -d ' ')
egalite "état CDR du PNG = desarme" desarme "$ETAT"
c=$(req GET "$B/api/files/$FID_EXIF" "$W/dl-exif.bin" "${AUTH[@]}")
egalite "téléchargement entier (200)" 200 "$c"
python3 -I - "$W/dl-exif.bin" "$MARQUEUR_EXIF" <<'PY' && ok "téléchargement : métadonnées absentes (ni eXIf, ni marqueur), PNG valide" || nok "téléchargement : métadonnées absentes, PNG valide"
import sys
d = open(sys.argv[1], "rb").read()
assert d[:8] == b"\x89PNG\r\n\x1a\n", "signature PNG"
assert b"eXIf" not in d and sys.argv[2].encode() not in d and b"tEXt" not in d, "métadonnées présentes"
PY
H=$(sql "select blob_hash from storage.files where id='$FID_EXIF'" | tr -d ' ')
P=$(blob_chemin "$H")
if [[ -n $P ]]; then
	sudo -n cat "$P" >"$W/blob-exif.bin"
	python3 -I - "$W/blob-exif.bin" "$MARQUEUR_EXIF" <<'PY' && ok "sur disque : blob commence par MIYUCENC, clair absent" || nok "sur disque : blob MIYUCENC, clair absent"
import sys
d = open(sys.argv[1], "rb").read()
assert d[:8] == b"MIYUCENC", "en-tête MIYUCENC absent"
assert d[8] == 1, "version de format != 1"
assert b"\x89PNG" not in d and sys.argv[2].encode() not in d and b"eXIf" not in d, "clair présent sur disque"
PY
else
	nok "sur disque : blob introuvable pour $H"
fi
TAILLE=$(stat -c %s "$W/dl-exif.bin")
# Plages (Range)
c=$(req GET "$B/api/files/$FID_EXIF" "$W/rg1.bin" "${AUTH[@]}" -H 'Range: bytes=0-15')
egalite "plage bytes=0-15 : 206" 206 "$c"
cmp -s <(head -c 16 "$W/dl-exif.bin") "$W/rg1.bin" && ok "plage bytes=0-15 : octets identiques au téléchargement entier" || nok "plage bytes=0-15 : octets identiques"
c=$(req GET "$B/api/files/$FID_EXIF" "$W/rg2.bin" "${AUTH[@]}" -H 'Range: bytes=10-29')
egalite "plage bytes=10-29 : 206" 206 "$c"
cmp -s <(dd if="$W/dl-exif.bin" bs=1 skip=10 count=20 2>/dev/null) "$W/rg2.bin" && ok "plage bytes=10-29 : octets identiques" || nok "plage bytes=10-29 : octets identiques"
grep -qi "content-range: bytes 10-29/$TAILLE" "$W/rg2.bin.h" && ok "plage : en-tête Content-Range exact" || nok "plage : en-tête Content-Range exact"

# ─────────────────────────────────────────────────────────────── 4. CDR : quarantaine et refus
echo "# 4. CDR : type non désarmable (quarantaine) et polyglotte (refus ou quarantaine)"
c=$(televerse "$W/doc.pdf" "doc-$SUFFIXE.pdf" application/pdf "$W/up-pdf.out")
egalite "téléversement PDF accepté en quarantaine (201)" 201 "$c"
FID_PDF=$(cat "$W/up-pdf.out.id")
egalite "état CDR du PDF = quarantaine" quarantaine "$(sql "select etat_cdr from storage.files where id='$FID_PDF'" | tr -d ' ')"
c=$(req GET "$B/api/files/$FID_PDF" "$W/pdf-sans.bin" "${AUTH[@]}")
egalite "PDF en quarantaine : téléchargement sans accord refusé (403)" 403 "$c"
c=$(req GET "$B/api/files/$FID_PDF?accepter_non_desarme=true" "$W/pdf-accord.bin" "${AUTH[@]}")
egalite "PDF en quarantaine : téléchargement avec accord explicite (200)" 200 "$c"
cmp -s "$W/doc.pdf" "$W/pdf-accord.bin" && ok "PDF : octets d'origine intacts (quarantaine)" || nok "PDF : octets d'origine intacts"

c=$(televerse "$W/polyglotte.png" "polyglotte-$SUFFIXE.png" image/png "$W/up-poly.out")
if [[ $c =~ ^4 ]]; then
	ok "polyglotte PNG/PE refusé à l'écriture ($c)"
	n=$(sql "select count(*) from storage.files where name='polyglotte-$SUFFIXE.png'" | tr -d ' ')
	egalite "polyglotte refusé : rien n'est stocké" 0 "$n"
elif [[ $c == 201 ]]; then
	FID_POLY=$(cat "$W/up-poly.out.id")
	egalite "polyglotte accepté seulement en quarantaine" quarantaine "$(sql "select etat_cdr from storage.files where id='$FID_POLY'" | tr -d ' ')"
	c2=$(req GET "$B/api/files/$FID_POLY" "$W/poly.bin" "${AUTH[@]}")
	egalite "polyglotte : jamais servi sans accord (403)" 403 "$c2"
else
	nok "polyglotte : refus ou quarantaine" "code $c"
fi

c=$(req POST "$B/api/files/upload" "$W/sans-dossier.out" "${AUTH[@]}" -F "file=@$W/copie.png;filename=sans-dossier-$SUFFIXE.png;type=image/png")
egalite "téléversement sans folder_id : refus de saisie (400)" 400 "$c"
egalite "téléversement sans folder_id : rien n'est stocké" 0 "$(sql "select count(*) from storage.files where name='sans-dossier-$SUFFIXE.png'" | tr -d ' ')"

# ─────────────────────────────────────────────────────────────── 5. copie puis suppression de l'original
echo "# 5. copie, suppression de l'original, cohérence ref_count"
c=$(televerse "$W/copie.png" "source-$SUFFIXE.png" image/png "$W/up-src.out")
egalite "téléversement de la source (201)" 201 "$c"
FID_SRC=$(cat "$W/up-src.out.id")
H_SRC=$(sql "select blob_hash from storage.files where id='$FID_SRC'" | tr -d ' ')
egalite "ref_count de la source = 1 après téléversement" 1 "$(sql "select ref_count from storage.blobs where hash='$H_SRC'" | tr -d ' ')"
c=$(req POST "$B/api/folders" "$W/dossier.out" "${AUTH[@]}" -H 'content-type: application/json' \
	--data-binary "{\"name\":\"cible-$SUFFIXE\",\"parent_id\":\"$DOSSIER\"}")
egalite "dossier cible créé (201)" 201 "$c"
CIBLE=$(jq -r .id "$W/dossier.out")
jq -n --arg f "$FID_SRC" --arg t "$CIBLE" '{file_ids:[$f],target_folder_id:$t}' >"$W/copie.json"
c=$(req POST "$B/api/batch/files/copy" "$W/copie.out" "${AUTH[@]}" -H 'content-type: application/json' --data-binary @"$W/copie.json")
egalite "copie par lot (POST /api/batch/files/copy)" 200 "$c"
egalite "ref_count = 2 après copie (deux lignes de fichiers)" 2 "$(sql "select ref_count from storage.blobs where hash='$H_SRC'" | tr -d ' ')"
FID_COPIE=$(sql "select id from storage.files where blob_hash='$H_SRC' and folder_id='$CIBLE'" | tr -d ' ')
[[ -n $FID_COPIE ]] && ok "copie présente dans le dossier cible" || nok "copie présente dans le dossier cible"
req GET "$B/api/files/$FID_SRC" "$W/src-avant.bin" "${AUTH[@]}" >/dev/null
c=$(req DELETE "$B/api/files/$FID_SRC" "$W/del-src.out" "${AUTH[@]}")
[[ $c == 2* ]] && ok "suppression de l'original ($c)" || nok "suppression de l'original" "code $c"
c=$(req GET "$B/api/files/$FID_COPIE" "$W/copie-apres.bin" "${AUTH[@]}")
egalite "copie lisible après suppression de l'original (200)" 200 "$c"
cmp -s "$W/copie-apres.bin" "$W/src-avant.bin" && ok "copie : octets identiques à l'original avant suppression" || nok "copie : octets identiques à l'original avant suppression"
egalite "cohérence ref_count (blob de la source) après suppression de l'original" \
	"$(sql "select count(*) from storage.files where blob_hash='$H_SRC'" | tr -d ' ')" \
	"$(sql "select ref_count from storage.blobs where hash='$H_SRC'" | tr -d ' ')"

# ─────────────────────────────────────────────────────────────── 6. WebDAV
echo "# 6. WebDAV (PUT / GET / PROPFIND)"
NOM_DAV="dav-$SUFFIXE.txt"
# Une PUT à la racine WebDAV range le fichier dans le dossier personnel : on lit donc ce dossier.
NOM_PERSO=$(curl -s "${AUTH[@]}" "$B/api/folders" | jq -r '.[0].name')
CHEMIN_PERSO=$(python3 -I -c "import sys,urllib.parse;print(urllib.parse.quote(sys.argv[1]))" "$NOM_PERSO")
c=$(req PUT "$B/webdav/$CHEMIN_PERSO/$NOM_DAV" "$W/dav-put.out" "${AUTH[@]}" -H 'content-type: text/plain' --data-binary 'contenu WebDAV COG')
[[ $c == 201 || $c == 204 || $c == 200 ]] && ok "WebDAV PUT ($c)" || nok "WebDAV PUT" "code $c"
c=$(req GET "$B/webdav/$CHEMIN_PERSO/$NOM_DAV" "$W/dav-get.bin" "${AUTH[@]}")
egalite "WebDAV GET (200)" 200 "$c"
[[ $(cat "$W/dav-get.bin") == "contenu WebDAV COG" ]] && ok "WebDAV GET : contenu identique" || nok "WebDAV GET : contenu identique"
c=$(req PROPFIND "$B/webdav/$CHEMIN_PERSO/" "$W/dav-pf.xml" "${AUTH[@]}" -H 'Depth: 1' -H 'content-type: application/xml' --data-binary '<?xml version="1.0"?><d:propfind xmlns:d="DAV:"><d:allprop/></d:propfind>')
egalite "WebDAV PROPFIND (207)" 207 "$c"
grep -q "$NOM_DAV" "$W/dav-pf.xml" && ok "WebDAV PROPFIND : fichier listé" || nok "WebDAV PROPFIND : fichier listé"
c=$(req PROPFIND "$B/webdav/$CHEMIN_PERSO/" "$W/dav-anon.xml" -H 'Depth: 1')
egalite "WebDAV PROPFIND sans authentification refusé (401)" 401 "$c"

# ─────────────────────────────────────────────────────────────── 7. WOPI sans jeton
echo "# 7. WOPI"
FID_WOPI=$FID_EXIF
c=$(req GET "$B/wopi/files/$FID_WOPI" "$W/wopi0.out")
[[ $c == 401 || $c == 403 || $c == 404 ]] && ok "WOPI CheckFileInfo sans jeton refusé ($c ; WOPI désactivé = 404)" || nok "WOPI sans jeton refusé" "code $c"
c=$(req GET "$B/wopi/files/$FID_WOPI?access_token=jeton-invente" "$W/wopi1.out")
[[ $c == 401 || $c == 403 || $c == 404 ]] && ok "WOPI avec jeton inventé refusé ($c)" || nok "WOPI avec jeton inventé refusé" "code $c"

# ─────────────────────────────────────────────────────────────── 8. suppression + ramasse-miettes
echo "# 8. suppression, corbeille, ramasse-miettes"
c=$(req DELETE "$B/api/files/$FID_COPIE" "$W/del-copie.out" "${AUTH[@]}")
[[ $c == 2* ]] && ok "suppression de la copie ($c)" || nok "suppression de la copie" "code $c"
# Vider la corbeille de la copie et de l'original (si elles y sont restées).
for id in "$FID_SRC" "$FID_COPIE"; do
	if [[ $(sql "select count(*) from storage.files where id='$id'" | tr -d ' ') == 1 ]]; then
		req DELETE "$B/api/trash/$id" "$W/tr.out" "${AUTH[@]}" >/dev/null
	fi
done
egalite "aucune ligne de fichier restante pour le blob de la source" 0 "$(sql "select count(*) from storage.files where blob_hash='$H_SRC'" | tr -d ' ')"
egalite "ref_count du blob de la source = 0 après suppression" 0 "$(sql "select ref_count from storage.blobs where hash='$H_SRC'" | tr -d ' ')"
# Le ramasse-miettes tourne au démarrage (passage immédiat) : redémarrage contrôlé.
P_SRC=$(blob_chemin "$H_SRC")
sudo -n systemctl restart miyukini-miyucloud
for _ in $(seq 1 60); do curl -s -o /dev/null "$B/api/auth/status" && break; sleep 1; done
for _ in $(seq 1 90); do
	[[ $(sql "select count(*) from storage.blobs where hash='$H_SRC'" | tr -d ' ') == 0 ]] && break
	sleep 1
done
egalite "ramasse-miettes : ligne du blob sans référence supprimée" 0 "$(sql "select count(*) from storage.blobs where hash='$H_SRC'" | tr -d ' ')"
[[ -z $P_SRC || ! -e $P_SRC ]] && ok "ramasse-miettes : fichier du blob supprimé du disque" || nok "ramasse-miettes : fichier du blob supprimé du disque"

# ─────────────────────────────────────────────────────────────── 8 bis. réconciliation des références
echo "# 8 bis. réconciliation : ref_count corrompu, réparé au démarrage"
H_EXIF=$(sql "select blob_hash from storage.files where id='$FID_EXIF'" | tr -d ' ')
# Le contenu désarmé est identique d'un passage à l'autre : le blob est partagé entre
# plusieurs lignes de fichiers. Le compte réel (lignes) sert de référence, pas une constante.
N_REEL=$(sql "select count(*) from storage.files where blob_hash='$H_EXIF'" | tr -d ' ')
sql "update storage.blobs set ref_count = $((N_REEL + 7)) where hash='$H_EXIF'" >/dev/null
egalite "corruption de test : ref_count ≠ nombre de fichiers ($N_REEL)" "$((N_REEL + 7))" "$(sql "select ref_count from storage.blobs where hash='$H_EXIF'" | tr -d ' ')"
sudo -n systemctl restart miyukini-miyucloud
for _ in $(seq 1 60); do curl -s -o /dev/null "$B/api/auth/status" && break; sleep 1; done
for _ in $(seq 1 90); do
	[[ $(sql "select ref_count from storage.blobs where hash='$H_EXIF'" | tr -d ' ') == "$N_REEL" ]] && break
	sleep 1
done
egalite "réconciliation au démarrage : ref_count recalculé depuis storage.files ($N_REEL)" "$N_REEL" "$(sql "select ref_count from storage.blobs where hash='$H_EXIF'" | tr -d ' ')"

# ─────────────────────────────────────────────────────────────── 9. cohérence globale
echo "# 9. cohérence ref_count / storage.files (ensemble de la base)"
egalite "aucun blob dont ref_count diffère du nombre de fichiers" 0 \
	"$(sql "select count(*) from storage.blobs b where b.ref_count <> (select count(*) from storage.files f where f.blob_hash=b.hash)" | tr -d ' ')"
egalite "aucun blob orphelin (ref_count 0 sans fichier)" 0 \
	"$(sql "select count(*) from storage.blobs b where b.ref_count = 0 and not exists (select 1 from storage.files f where f.blob_hash=b.hash)" | tr -d ' ')"
egalite "requête SQL ref_count : résultat cohérent pour les blobs du scénario" 0 \
	"$(sql "select count(*) from storage.blobs b where b.hash in ('$(sql "select blob_hash from storage.files where id='$FID_EXIF'" | tr -d ' ')','$(sql "select blob_hash from storage.files where id='$FID_PDF'" | tr -d ' ')') and b.ref_count <> (select count(*) from storage.files f where f.blob_hash=b.hash)" | tr -d ' ')"

# ─────────────────────────────────────────────────────────────── 10. MAIA : routes mémoire signées
echo "# 10. MAIA : routes mémoire signées (127.0.0.1:11435)"
MDIR=$HOME/.cog-scenarios/maia
DROPIN=/etc/systemd/system/miyukini-maia.service.d/memoire-essai.conf
KMDIR=/var/lib/miyukini/kindmother/credentials
MCREDS=/var/lib/miyukini/maia/kindmother-credentials
UTIL=alice
AUTRE=bob
OPER=maia-$UTIL
TEST_PY="$W/cog_test.py"
mkdir -p "$MDIR" && chmod 700 "$MDIR"
for nom in utilisateur approbation; do
	[[ -s $MDIR/$nom.seed ]] || (umask 077 && openssl rand -hex 32 >"$MDIR/$nom.seed")
done
SEED_U=$(cat "$MDIR/utilisateur.seed")
SEED_A=$(cat "$MDIR/approbation.seed")
PUB_U=$(python3 -I "$TEST_PY" pub "$SEED_U")
PUB_A=$(python3 -I "$TEST_PY" pub "$SEED_A")
[[ $PUB_U != "$PUB_A" ]] && ok "clés de l'essai distinctes (utilisateur ≠ approbation)" || nok "clés de l'essai distinctes"

if ! sudo -n test -e "$DROPIN"; then
	c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-avant.out")
	egalite "MAIA sans configuration : routes mémoire en 503" 503 "$c"
	# 1. Déclaration de l'utilisateur de mémoire : KindMother provisionne l'opérateur maia-<user>
	#    (justificatif généré par le service, base dédiée chiffrée). Pas de clé écrite à la main.
	sudo -n rm -f "$KMDIR/$OPER.pub" "$KMDIR/$OPER.key" "$MCREDS/$OPER.key"
	printf '%s\n' "$UTIL" | sudo -n tee "$KMDIR/../maia-utilisateurs" >/dev/null
	sudo -n chown miyukini-kindmother:miyukini-kindmother "$KMDIR/../maia-utilisateurs"
	sudo -n chmod 600 "$KMDIR/../maia-utilisateurs"
	sudo -n systemctl restart miyukini-kindmother
	for _ in $(seq 1 30); do ss -ltn | grep -q '127\.0\.0\.1:50051' && break; sleep 1; done
	# 2. Identifiants côté MAIA (copie du justificatif de l'opérateur et de la clé serveur épinglée).
	sudo -n install -d -m 0700 -o miyukini-maia -g miyukini-maia "$MCREDS"
	sudo -n cp "$KMDIR/$OPER.key" "$MCREDS/$OPER.key"
	sudo -n cp "$KMDIR/server.pub" "$MCREDS/server.pub"
	sudo -n chown miyukini-maia:miyukini-maia "$MCREDS/$OPER.key" "$MCREDS/server.pub"
	sudo -n chmod 600 "$MCREDS/$OPER.key" "$MCREDS/server.pub"
	# 3. Clés épinglées distinctes (utilisateur ≠ approbation) dans un drop-in systemd (VM uniquement).
	sudo -n install -d -m 0755 "$(dirname "$DROPIN")"
	{
		printf '# Généré par deploy/cog-vm/scenarios.sh (VM jetable uniquement) : routes mémoire de l'\''essai.\n'
		printf '[Service]\n'
		printf 'Environment=MAIA_MEMOIRE_UTILISATEUR=%s\n' "$UTIL"
		printf 'Environment=KINDMOTHER_ADDR=127.0.0.1:50051\n'
		printf 'Environment=KINDMOTHER_CREDENTIALS_DIR=%s\n' "$MCREDS"
		printf 'Environment=MAIA_MEMOIRE_CLES_UTILISATEUR=%s\n' "$PUB_U"
		printf 'Environment=MAIA_TAMR_CLES_APPROBATION=%s\n' "$PUB_A"
	} | sudo -n tee "$DROPIN" >/dev/null
	sudo -n systemctl daemon-reload
	# MAIA n'ouvre la mémoire qu'au démarrage : KindMother doit écouter avant.
	sudo -n systemctl restart miyukini-maia
	for _ in $(seq 1 60); do curl -s -o /dev/null "$MAIA/health" && break; sleep 1; done
fi

# MAIA ouvre KindMother une fois, au démarrage. Après une période d'inactivité, KindMother ferme
# la connexion et l'entrepôt de MAIA ne se reconnecte pas (« Broken pipe », 503) : défaut de MAIA,
# signalé. Le scénario redémarre MAIA pour partir d'une connexion neuve.
sudo -n systemctl restart miyukini-maia
for _ in $(seq 1 60); do curl -s -o /dev/null "$MAIA/health" && break; sleep 1; done

# Identifiant d'instance (L2) : en-tête rendu par toute réponse des routes mémoire.
req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-inst.out" >/dev/null
INST=$(grep -i '^x-maia-instance:' "$W/m-inst.out.h" | tr -d '\r' | awk '{print $2}')
[[ -n $INST ]] && ok "en-tête x-maia-instance présent sur les routes mémoire" || nok "en-tête x-maia-instance présent"
c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-nonsigne.out")
egalite "requête mémoire non signée refusée (401)" 401 "$c"

# Appel signé, signature fraîche à chaque fois : appel_signe GRAINE USAGE METHODE CHEMIN CORPS SORTIE [curl…] → code HTTP
appel_signe() {
	local g=$1 u=$2 m=$3 ch=$4 cf=$5 out=$6
	shift 6
	local -a h
	mapfile -t h < <(signee "$g" "$u" "$m" "$ch" "$cf")
	req "$m" "$MAIA$ch" "$out" "${h[@]}" "$@"
}
# Requête signée : entetes GRAINE USAGE METHODE CHEMIN CORPS [TS NONCE]
signee() { # signee GRAINE USAGE METHODE CHEMIN CORPS_FICHIER [TS NONCE] → en-têtes -H pour curl
	python3 -I "$TEST_PY" entetes "$1" "$2" "$3" "$4" "$5" "${6:-}" "${7:-}" | awk '{ print "-H"; print }'
}
: >"$W/vide"
mapfile -t H_CONSULTER < <(signee "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/consulter" "$W/vide")
c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-ok.out" "${H_CONSULTER[@]}")
egalite "consulter : requête signée par la clé de l'utilisateur (200)" 200 "$c"
c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-rejeu.out" "${H_CONSULTER[@]}")
egalite "rejeu de la même requête signée refusé (401)" 401 "$c"
mapfile -t H_AUTRE < <(signee "$SEED_U" "maia/memoire/$INST/$AUTRE" POST "/v1/memoire/$AUTRE/consulter" "$W/vide")
c=$(req POST "$MAIA/v1/memoire/$AUTRE/consulter" "$W/m-autre.out" "${H_AUTRE[@]}")
egalite "autre utilisateur (bob) sur la mémoire d'alice : refus (signature liée à bob refusée, 401)" 401 "$c"
mapfile -t H_CROISE < <(signee "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$AUTRE/consulter" "$W/vide")
c=$(req POST "$MAIA/v1/memoire/$AUTRE/consulter" "$W/m-croise.out" "${H_CROISE[@]}")
egalite "signature d'alice présentée pour le chemin de bob : refus (403)" 403 "$c"
c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/m-cookie.out" "${H_CONSULTER[@]}" -H 'Cookie: session=x')
egalite "requête signée avec un cookie ambiant : refus (400)" 400 "$c"

# Approbations TAMR : clé mémoire refusée, clé d'approbation seule acceptée sur sa route.
mapfile -t H_APPR_MEM < <(signee "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/inconnue" "$W/vide")
c=$(req POST "$MAIA/v1/memoire/$UTIL/approbations/00000000-0000-0000-0000-000000000000" "$W/appr-mem.out" "${H_APPR_MEM[@]}")
egalite "approbation signée par la clé mémoire : refusée (401)" 401 "$c"
c=$(req POST "$MAIA/v1/memoire/$UTIL/approbations/00000000-0000-0000-0000-000000000000" "$W/appr-non.out")
egalite "approbation non signée : refusée (401)" 401 "$c"
printf '%s' '{"empreinte":"sans-demande"}' >"$W/appr-vide.json"
mapfile -t H_APPR_OK < <(signee "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/00000000-0000-0000-0000-000000000000" "$W/appr-vide.json")
c=$(req POST "$MAIA/v1/memoire/$UTIL/approbations/00000000-0000-0000-0000-000000000000" "$W/appr-cle.out" -H 'content-type: application/json' --data-binary @"$W/appr-vide.json" "${H_APPR_OK[@]}")
[[ $c =~ ^4 && $c != 401 && $c != 403 ]] && ok "approbation signée par la clé d'approbation : passe la garde, puis refusée faute de demande ($c)" || nok "approbation signée par la clé d'approbation : passe la garde" "code $c"
mapfile -t H_CONS_APPR < <(signee "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/consulter" "$W/vide")
c=$(req POST "$MAIA/v1/memoire/$UTIL/consulter" "$W/cons-appr.out" "${H_CONS_APPR[@]}")
egalite "clé d'approbation refusée sur une route autre que l'approbation (401)" 401 "$c"

# Pose d'une règle (ordinaire, non sensible).
printf '%s' '{"branche":"preferences.style","texte":"Réponses brèves.","importance":3}' >"$W/regle.json"
mapfile -t H_REGLE < <(signee "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/regles" "$W/regle.json")
c=$(req POST "$MAIA/v1/memoire/$UTIL/regles" "$W/regle.out" -H 'content-type: application/json' --data-binary @"$W/regle.json" "${H_REGLE[@]}")
[[ $c == 2* ]] && ok "règle posée par l'utilisateur ($c)" || nok "règle posée par l'utilisateur" "code $c : $(head -c 200 "$W/regle.out")"

# ─── 10 bis. candidat sensible : proposition (API MAIA), validation 202, approbation TAMR
echo "# 10 bis. candidat sensible : proposition, validation, approbation TAMR"
PROP_SRC=$HERE/cog-proposer
if [[ ! -x $PROP_SRC ]]; then
	nok "outil d'essai cog-proposer présent (COG_PROPOSER, voir README.md)" "absent : $PROP_SRC"
else
	CP=/tmp/cog-proposer-$SUFFIXE
	install -m 0755 "$PROP_SRC" "$CP"
	# Le justificatif de l'opérateur maia-alice appartient à miyukini-maia : l'outil s'exécute sous ce compte.
	propose() { sudo -n -u miyukini-maia "$CP" "$UTIL" 127.0.0.1:50051 "$MCREDS" "$1" "$2" 2>"$W/prop.err"; }
	TXT1="Allergie de test COG $SUFFIXE"
	TXT2="Second candidat de test COG $SUFFIXE"
	ID1=$(propose sante.essai "$TXT1")
	ID2=$(propose sante.essai "$TXT2")
	rm -f "$CP"
	[[ $ID1 =~ ^[0-9a-f-]{36}$ && $ID2 =~ ^[0-9a-f-]{36}$ ]] && ok "candidats sensibles proposés via MemoireMaia::proposer (domaine sante)" || nok "candidats sensibles proposés" "$(head -c 200 "$W/prop.err")"

	# Nombre de notes (et non de candidats) portant le texte donné, dans une consultation.
	notes_avec() { jq --arg t "$1" '[.notes[]? | select(.texte | contains($t))] | length' "$2" 2>/dev/null || echo "?"; }
	appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/consulter" "$W/vide" "$W/cons1.out" >/dev/null
	egalite "consulter : les deux candidats sensibles sont listés" 2 "$(jq --arg a "$ID1" --arg b "$ID2" '[.candidats[]? | select(.id == $a or .id == $b)] | length' "$W/cons1.out" 2>/dev/null)"

	c=$(appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/candidats/$ID1/valider" "$W/vide" "$W/val1.out")
	egalite "validation d'un candidat sensible : 202 (demande d'approbation TAMR)" 202 "$c"
	IV1=$(jq -r '.approbation.intervention_id // empty' "$W/val1.out")
	EMP1=$(jq -r '.approbation.empreinte // empty' "$W/val1.out")
	[[ -n $IV1 && -n $EMP1 ]] && ok "demande d'approbation : intervention et empreinte de l'action exacte" || nok "demande d'approbation : intervention et empreinte" "$(head -c 200 "$W/val1.out")"
	c=$(appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/candidats/$ID2/valider" "$W/vide" "$W/val2.out")
	egalite "second candidat sensible : 202" 202 "$c"
	IV2=$(jq -r '.approbation.intervention_id // empty' "$W/val2.out")
	EMP2=$(jq -r '.approbation.empreinte // empty' "$W/val2.out")

	appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/consulter" "$W/vide" "$W/cons2.out" >/dev/null
	egalite "avant approbation : aucune note écrite à partir du candidat" 0 "$(notes_avec "$TXT1" "$W/cons2.out")"

	printf '%s' "{\"empreinte\":\"$EMP1\"}" >"$W/appr1.json"
	c=$(appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/$IV1" "$W/appr1.json" "$W/appr1-mem.out" -H 'content-type: application/json' --data-binary @"$W/appr1.json")
	egalite "approbation signée par la clé MÉMOIRE (au lieu de la clé d'approbation) : refusée (401)" 401 "$c"

	# Empreinte fausse sur la seconde demande : refusée, et la demande est consommée (R-APPR-2).
	printf '%s' "{\"empreinte\":\"$(printf '0%.0s' $(seq 1 64))\"}" >"$W/appr-faux.json"
	c=$(appel_signe "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/$IV2" "$W/appr-faux.json" "$W/appr-faux.out" -H 'content-type: application/json' --data-binary @"$W/appr-faux.json")
	[[ $c =~ ^4 ]] && ok "approbation avec une empreinte qui ne correspond pas à l'action : refusée ($c)" || nok "approbation avec une empreinte fausse : refusée" "code $c"
	printf '%s' "{\"empreinte\":\"$EMP2\"}" >"$W/appr2.json"
	c=$(appel_signe "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/$IV2" "$W/appr2.json" "$W/appr2-apres.out" -H 'content-type: application/json' --data-binary @"$W/appr2.json")
	[[ $c =~ ^4 ]] && ok "demande refusée puis reprise avec la bonne empreinte : non réutilisable ($c)" || nok "demande consommée après refus" "code $c"

	c=$(appel_signe "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/$IV1" "$W/appr1.json" "$W/appr1-ok.out" -H 'content-type: application/json' --data-binary @"$W/appr1.json")
	egalite "approbation par la clé d'approbation avec la bonne empreinte : 200 (StrongFather ALLOW)" 200 "$c"
	jq -e '.tamr == "approuve"' "$W/appr1-ok.out" >/dev/null && ok "réponse : approbation TAMR accordée, note écrite" || nok "réponse : approbation TAMR accordée" "$(head -c 200 "$W/appr1-ok.out")"

	appel_signe "$SEED_U" "maia/memoire/$INST/$UTIL" POST "/v1/memoire/$UTIL/consulter" "$W/vide" "$W/cons3.out" >/dev/null
	egalite "après approbation : une note issue du candidat est présente" 1 "$(notes_avec "$TXT1" "$W/cons3.out")"

	c=$(appel_signe "$SEED_A" "maia/tamr-approbation/$INST/$UTIL" POST "/v1/memoire/$UTIL/approbations/$IV1" "$W/appr1.json" "$W/appr1-rejeu.out" -H 'content-type: application/json' --data-binary @"$W/appr1.json")
	[[ $c =~ ^4 ]] && ok "approbation rejouée : refusée, une seule réponse par demande ($c)" || nok "approbation rejouée : refusée" "code $c"
fi

# ─────────────────────────────────────────────────────────────── 11. KindMother : client non autorisé
echo "# 11. KindMother (127.0.0.1:50051) : clients non autorisés"
PIN_KM=$(sudo -n cat "$KMDIR/server.pub" | tr -d '\n')
r=$(python3 -I "$TEST_PY" km "$KM" intrus-essai - "$PIN_KM")
[[ $r != Authenticated ]] && ok "opérateur inconnu refusé à la poignée de main ($r)" || nok "opérateur inconnu refusé"
r=$(python3 -I "$TEST_PY" km "$KM" "$OPER" "$(openssl rand -hex 32)" "$PIN_KM")
[[ $r != Authenticated ]] && ok "opérateur connu avec une mauvaise clé refusé ($r)" || nok "opérateur connu avec une mauvaise clé refusé"
r=$(python3 -I "$TEST_PY" km "$KM" "$OPER" - "$PIN_KM")
[[ $r != Authenticated ]] && ok "opérateur connu sans signature refusé ($r)" || nok "opérateur connu sans signature refusé"
SEED_KM=$(sudo -n cat "$KMDIR/$OPER.key" | tr -d '\n')
r=$(python3 -I "$TEST_PY" km "$KM" "$OPER" "$SEED_KM" "$PIN_KM")
egalite "témoin positif : l'opérateur de la mémoire s'authentifie avec sa clé" Authenticated "$r"

# ─────────────────────────────────────────────────────────────── 11 bis. Portail scindé (A-022 §3)
echo "# 11 bis. Portail scindé : entrée et sortie, files à sens unique, jeton à usage unique, pannes, AppArmor"
OUTIL=/home/cog/cog-paquet/outils/portail_essai
PF=/var/lib/miyukini/portail-files
VE=/etc/miyukini/portail-entree/voies
VS=/etc/miyukini/portail-sortie/voies
SP=essai.portail
# Dossier d'essai jetable (clés d'essai, jetons) lisible des comptes façade et moteur.
T=$(sudo -n mktemp -d /run/cog-portail.XXXXXX)
sudo -n install -m 0755 "$OUTIL" "$T/outil"
sudo -n chmod 1777 "$T"
facade() { sudo -n -u miyukini-facade "$T/outil" "$@"; }
moteur() { sudo -n -u miyukini-moteur "$T/outil" "$@"; }
# attendre COMMANDE… : réessaie 15 s au plus
attendre() { local i; for i in $(seq 1 30); do "$@" >"$W/attendre.out" 2>&1 && return 0; sleep 0.5; done; return 1; }
cores() { sudo -n "$T/outil" cores --cles "$T/cles" --service "$SP" --voies-entree "$VE" --voies-sortie "$VS" \
	--entree-uid "$(id -u miyukini-portail-entree)" --entree-gid "$(id -g miyukini-portail-entree)" "$@" >/dev/null; }
cle_publiee() { sudo -n -u miyukini-facade test -s "$PF/publication/$SP.cle"; }
nb_publications() { facade lire --publication "$PF/publication" --service "$SP" 2>/dev/null; }

if [[ ! -x $OUTIL ]]; then
	nok "outil d'essai du Portail présent dans le paquet" "$OUTIL absent"
else
	cores --etat active
	sudo -n chmod 0644 "$T/cles"
	egalite "Cores : descripteur de l'entrée (graine des jetons) en 0600 au compte de l'entrée" "600 miyukini-portail-entree" \
		"$(sudo -n stat -c '%a %U' "$VE/$SP.voie")"
	egalite "Cores : descripteur de la sortie (sans secret) en 0640, groupe de la sortie" "640 miyukini-portail-sortie" \
		"$(sudo -n stat -c '%a %G' "$VS/$SP.voie")"
	attendre cle_publiee && ok "sortie : clé de publication de la voie publiée pour la façade" || nok "sortie : clé de publication publiée"

	# Chemin nominal : façade → entrée → moteur → sortie → façade.
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 1 >/dev/null
	attendre moteur relever --admises "$PF/admises" --jeton "$T/jeton1" &&
		egalite "entrée : déclaration de la façade admise et remise au moteur (séquence)" 1 "$(cat "$W/attendre.out")" ||
		nok "entrée : déclaration admise et remise au moteur" "$(head -c 200 "$W/attendre.out")"
	moteur repondre --reponses "$PF/reponses" --jeton "$T/jeton1" --texte "créneau 10h" >/dev/null
	attendre bash -c "[ \"\$(sudo -n -u miyukini-facade '$T/outil' lire --publication '$PF/publication' --service '$SP' 2>/dev/null)\" = 1 ]" &&
		ok "sortie : réponse à jeton publiée, signée par la clé de la voie (vérifiée par la façade)" ||
		nok "sortie : réponse publiée et vérifiée"

	# Jeton à usage unique.
	moteur repondre --reponses "$PF/reponses" --jeton "$T/jeton1" --texte "rejeu" >/dev/null
	attendre bash -c "sudo -n journalctl -u miyukini-portail-sortie --since '-30s' --no-pager -o cat | grep -q 'jeton de réponse déjà utilisé'" &&
		ok "sortie : second usage du même jeton refusé (journal)" || nok "sortie : second usage du jeton refusé"
	egalite "sortie : aucune publication pour le jeton rejoué" 0 "$(nb_publications)"
	# Rejeu d'une trame déjà admise.
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 1 >/dev/null
	attendre bash -c "sudo -n journalctl -u miyukini-portail-entree --since '-30s' --no-pager -o cat | grep -q 'séquence rejouée'" &&
		ok "entrée : trame rejouée refusée (anti-rejeu par auteur)" || nok "entrée : trame rejouée refusée"

	# L'entrée refuse toute publication : une réponse glissée dans le dépôt est rejetée, rien ne sort.
	sudo -n -u miyukini-facade "$T/outil" repondre --reponses "$PF/depot" --jeton "$T/jeton1" --texte "par l'entrée" >/dev/null 2>&1
	attendre bash -c "sudo -n journalctl -u miyukini-portail-entree --since '-30s' --no-pager -o cat | grep -q 'élément non admis'" &&
		ok "entrée : réponse déposée dans sa file refusée (l'entrée ne publie jamais)" ||
		nok "entrée : réponse déposée refusée"
	egalite "entrée : rien publié par ce chemin" 0 "$(nb_publications)"

	# Pannes : kill -9 de l'entrée, la sortie répond encore ; kill -9 de la sortie, l'entrée admet encore.
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 2 >/dev/null
	attendre moteur relever --admises "$PF/admises" --jeton "$T/jeton2" >/dev/null
	sudo -n systemctl stop miyukini-portail-entree
	egalite "panne de l'entrée (arrêtée) : la sortie reste active" active "$(systemctl is-active miyukini-portail-sortie)"
	moteur repondre --reponses "$PF/reponses" --jeton "$T/jeton2" --texte "pendant la panne de l'entrée" >/dev/null
	attendre bash -c "[ \"\$(sudo -n -u miyukini-facade '$T/outil' lire --publication '$PF/publication' --service '$SP' 2>/dev/null)\" = 1 ]" &&
		ok "panne de l'entrée : la sortie publie la réponse" || nok "panne de l'entrée : la sortie publie"
	sudo -n systemctl start miyukini-portail-entree
	attendre systemctl is-active --quiet miyukini-portail-entree
	PID_S=$(systemctl show -p MainPID --value miyukini-portail-sortie)
	[[ $PID_S =~ ^[1-9][0-9]*$ ]] && sudo -n kill -9 "$PID_S"
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 3 >/dev/null
	attendre moteur relever --admises "$PF/admises" --jeton "$T/jeton3" &&
		egalite "kill -9 de la sortie : l'entrée admet encore (séquence)" 3 "$(cat "$W/attendre.out")" ||
		nok "kill -9 de la sortie : l'entrée admet encore"
	attendre bash -c "[ \"\$(systemctl show -p MainPID --value miyukini-portail-sortie)\" != '$PID_S' ] && systemctl is-active --quiet miyukini-portail-sortie" &&
		ok "kill -9 de la sortie : relancée par systemd (Restart=on-failure)" || nok "sortie relancée après kill -9"
	moteur repondre --reponses "$PF/reponses" --jeton "$T/jeton3" --texte "après relance" >/dev/null
	attendre bash -c "[ \"\$(sudo -n -u miyukini-facade '$T/outil' lire --publication '$PF/publication' --service '$SP' 2>/dev/null)\" = 1 ]" &&
		ok "sortie relancée : clé et jetons repris de son état, réponse publiée" || nok "sortie relancée : réponse publiée"
	moteur repondre --reponses "$PF/reponses" --jeton "$T/jeton1" --texte "rejeu après relance" >/dev/null
	sleep 2
	egalite "sortie relancée : jeton consommé avant la panne toujours refusé" 0 "$(nb_publications)"
	PID_E=$(systemctl show -p MainPID --value miyukini-portail-entree)
	[[ $PID_E =~ ^[1-9][0-9]*$ ]] && sudo -n kill -9 "$PID_E"
	egalite "kill -9 de l'entrée : la sortie reste active" active "$(systemctl is-active miyukini-portail-sortie)"
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 2 >/dev/null
	facade deposer --cles "$T/cles" --service "$SP" --depot "$PF/depot" --sequence 4 >/dev/null
	attendre moteur relever --admises "$PF/admises" --jeton "$T/jeton4" &&
		egalite "entrée relancée après kill -9 : marques reprises, rejeu refusé, trame neuve admise" 4 "$(cat "$W/attendre.out")" ||
		nok "entrée relancée : admission après kill -9"

	# AppArmor : chaque profil refuse l'écriture dans les files de l'autre sens. Le compte qui exécute
	# la sonde est le PROPRIÉTAIRE de la file visée (les droits l'y autoriseraient) : seul AppArmor refuse.
	# Témoin : la même sonde, binaire copié hors de son chemin de profil (non confiné), y écrit.
	sonde_e() { sudo -n -u miyukini-facade "$1" --voies "$VE" --etat /nonexistent --depot "$PF/depot" --admises "$PF/admises" \
		--sas /opt/miyukini/bin/portail-sas --interdit "$PF/publication" --verifier-cloisonnement >/dev/null 2>&1; }
	sonde_s() { sudo -n -u miyukini-portail-entree "$1" --voies "$VS" --etat /nonexistent --reponses "$PF/reponses" \
		--publication "$PF/publication" --interdit "$PF/depot" --verifier-cloisonnement >/dev/null 2>&1; }
	sudo -n install -m 0755 /opt/miyukini/bin/portail-entree "$T/entree-non-confinee"
	sudo -n install -m 0755 /opt/miyukini/bin/portail-sortie "$T/sortie-non-confinee"
	sonde_e "$T/entree-non-confinee"; egalite "témoin : sans profil, le propriétaire de la file de publication y écrit" 1 "$?"
	sonde_e /opt/miyukini/bin/portail-entree; egalite "AppArmor : l'entrée ne peut pas écrire dans la file de publication" 0 "$?"
	attendre bash -c "sudo -n journalctl -k --since '-60s' --no-pager | grep 'apparmor=\"DENIED\"' | grep 'profile=\"miyukini-portail-entree\"' | grep -q 'portail-files/publication/.sonde-cloisonnement'" &&
		ok "AppArmor : refus journalisé (profil miyukini-portail-entree, file publication)" || nok "AppArmor : refus de l'entrée journalisé"
	sonde_s "$T/sortie-non-confinee"; egalite "témoin : sans profil, le propriétaire de la file de dépôt y écrit" 1 "$?"
	sonde_s /opt/miyukini/bin/portail-sortie; egalite "AppArmor : la sortie ne peut pas écrire dans la file de dépôt" 0 "$?"
	attendre bash -c "sudo -n journalctl -k --since '-60s' --no-pager | grep 'apparmor=\"DENIED\"' | grep 'profile=\"miyukini-portail-sortie\"' | grep -q 'portail-files/depot/.sonde-cloisonnement'" &&
		ok "AppArmor : refus journalisé (profil miyukini-portail-sortie, file dépôt)" || nok "AppArmor : refus de la sortie journalisé"

	# Coupure puis rétablissement : clé retirée, puis nouvelle clé (rotation).
	CLE_AVANT=$(sudo -n cat "$PF/publication/$SP.cle")
	cores --etat coupee
	attendre bash -c "! sudo -n test -e '$PF/publication/$SP.cle'" && ok "coupure : clé de publication retirée par la sortie" || nok "coupure : clé retirée"
	cores --etat active --rotation 1
	attendre cle_publiee && [[ $(sudo -n cat "$PF/publication/$SP.cle") != "$CLE_AVANT" ]] &&
		ok "rétablissement : nouvelle clé de publication" || nok "rétablissement : nouvelle clé"

	# Nettoyage : voie supprimée (aucun résidu dans les files).
	sudo -n rm -f "$VE/$SP.voie" "$VS/$SP.voie"
	attendre bash -c "! sudo -n test -e '$PF/publication/$SP.cle' && ! sudo -n test -e /var/lib/miyukini/portail-sortie/$SP.cle" &&
		ok "voie supprimée : clé et état de la sortie effacés" || nok "voie supprimée : aucun résidu"
fi
sudo -n rm -rf "$T"

# ─────────────────────────────────────────────────────────────── 13. IA locale : maia-llm, MAIA, Alicia (A-026)
echo "# 13. IA locale : maia-llm (127.0.0.1:11436, modèle réel, CPU), MAIA par lui, mention IA, Alicia"
LLM=http://127.0.0.1:11436
h=$(curl -s --max-time 10 "$LLM/maia/llm/health")
grep -q '"model_loaded":true' <<<"$h" && grep -q '"licence":"Apache-2.0"' <<<"$h" && grep -q '"gpu_layers":0' <<<"$h" &&
	ok "maia-llm : modèle signé chargé (Apache-2.0), CPU" || nok "maia-llm : modèle chargé" "$h"
c=$(req POST "$LLM/v1/chat/completions" "$W/llm-ns.out" -H 'content-type: application/json' -d '{"messages":[]}')
egalite "maia-llm : inférence sans signature refusée" 401 "$c"
# Appelant d'essai de l'API protégée de MAIA (VM seulement) : drop-in retiré en fin de section.
for nom in appelant alicia; do
	[[ -s $MDIR/$nom.seed ]] || (umask 077 && openssl rand -hex 32 >"$MDIR/$nom.seed")
done
SEED_C=$(cat "$MDIR/appelant.seed")
PUB_C=$(python3 -I "$TEST_PY" pub "$SEED_C")
DROPIN_C=/etc/systemd/system/miyukini-maia.service.d/appelant-essai.conf
sudo -n install -d -m 0755 /etc/systemd/system/miyukini-maia.service.d
printf '[Service]\nEnvironment=MAIA_CLES_APPELANTS_SUPPLEMENTAIRES=%s\n' "$PUB_C" | sudo -n tee "$DROPIN_C" >/dev/null
sudo -n systemctl daemon-reload
sudo -n systemctl restart miyukini-maia
for _ in $(seq 1 60); do curl -s -o /dev/null "$MAIA/health" && break; sleep 1; done
printf '%s' '{"message":"Bonjour ! Qui es-tu, en une phrase ?","enable_tools":false}' >"$W/chat.json"
t0=$(date +%s%N)
c=$(appel_signe "$SEED_C" maia/api POST /v1/agents/miou/chat "$W/chat.json" "$W/chat.out" -H 'content-type: application/json' --data-binary @"$W/chat.json")
ms=$((($(date +%s%N) - t0) / 1000000))
egalite "MAIA : /v1/agents/miou/chat signé : 200" 200 "$c"
python3 -I - "$W/chat.out" <<'PY' && ok "MAIA répond par maia-llm (backend native, sans repli) en $ms ms, genere_par_ia et mention IA présents : « $(python3 -I -c 'import json,sys; print(json.load(open(sys.argv[1]))["content"][:80].replace("\n"," "))' "$W/chat.out" 2>/dev/null) »" || nok "MAIA répond par maia-llm, mention IA" "$(head -c 300 "$W/chat.out")"
import json, sys
v = json.load(open(sys.argv[1]))
assert v.get("backend") == "native", v
assert v.get("fallback") is False, v
assert v.get("genere_par_ia") is True, v
assert "IA" in (v.get("mention_ia") or ""), v
assert len((v.get("content") or "").strip()) > 3, v
PY
p=$(curl -s --max-time 10 "$MAIA/maia/presentation")
grep -q '"nature":"intelligence artificielle"' <<<"$p" && ok "MAIA : /maia/presentation, nature « intelligence artificielle » (AI Act art. 50)" || nok "MAIA : présentation IA" "$p"
sudo -n rm -f "$DROPIN_C"
sudo -n systemctl daemon-reload
sudo -n systemctl restart miyukini-maia
for _ in $(seq 1 60); do curl -s -o /dev/null "$MAIA/health" && break; sleep 1; done
c=$(req GET "$MAIA/v1/agents" "$W/ag.out")
egalite "MAIA : appelant d'essai retiré, API protégée de nouveau en 503" 503 "$c"

# Alicia : clé d'essai épinglée dans maia-llm (VM seulement, retirée en fin de section) ; essai
# d'intégration de miyualicia (binaire du paquet) contre le moteur INSTALLÉ ([moteur_ia] url de
# /etc/miyukini/alicia.toml) et la politique INSTALLÉE (/etc/miyukini/strongfather/alicia.toml),
# présentée au chemin compilé dans l'essai par un espace de montage privé (aucune trace hors de lui).
ESSAI_A=/home/cog/cog-paquet/outils/alicia_proposition_ia
URL_A=$(sed -n 's/^url = "\(.*\)"$/\1/p' /etc/miyukini/alicia.toml)
egalite "Alicia : [moteur_ia] url = maia-llm" "http://127.0.0.1:11436" "$URL_A"
if [[ -x $ESSAI_A ]]; then
	(umask 077 && python3 -I -c 'import sys; open(sys.argv[2], "wb").write(bytes.fromhex(open(sys.argv[1]).read().strip()))' "$MDIR/alicia.seed" "$W/alicia.cle")
	PUB_A=$(python3 -I "$TEST_PY" pub "$(cat "$MDIR/alicia.seed")")
	TOML=/etc/miyukini/maia-llm.toml
	sudo -n cp "$TOML" "$W/maia-llm.toml.avant"
	sudo -n sed -i "s|^appelants_inference = \[\(\"[^\"]*\"\)\]|appelants_inference = [\1, \"$PUB_A\"]|" "$TOML"
	grep -q "$PUB_A" "$TOML" && ok "Alicia : clé d'essai épinglée dans appelants_inference" || nok "Alicia : clé épinglée"
	sudo -n systemctl restart miyukini-maia-llm
	for _ in $(seq 1 120); do curl -s "$LLM/maia/llm/health" | grep -q '"model_loaded":true' && break; sleep 1; done
	POL=$(cat "$ESSAI_A.politique")
	RAC=${POL%%/crates/*}
	t0=$(date +%s)
	sudo -n unshare -m sh -c 'set -e; mount -t tmpfs -o mode=0755 tmpfs /srv
		mkdir -p "$1/crates/miyualicia" "$1/config/strongfather"
		cp /etc/miyukini/strongfather/alicia.toml "$1/config/strongfather/alicia.toml"
		exec setpriv --reuid=cog --regid=cog --init-groups env ALICIA_MOTEUR_URL="$3" ALICIA_CLE="$4" \
			"$2" --exact --include-ignored --test-threads=1 --nocapture \
			l_ia_ne_deverrouille_jamais_une_porte_sous_la_politique_de_base intentions_francaises_avec_le_vrai_modele' \
		_ "$RAC" "$ESSAI_A" "$URL_A" "$W/alicia.cle" >"$W/alicia.out" 2>&1
	rc=$?
	duree=$(($(date +%s) - t0))
	cp "$W/alicia.out" "$HOME/.cog-scenarios/alicia-essai.out" 2>/dev/null
	TAUX=$(sed -n 's/^TAUX \([0-9]*\/[0-9]*\).*/\1/p' "$W/alicia.out")
	[[ $rc -eq 0 ]] && grep -q 'test result: ok. 2 passed' "$W/alicia.out" &&
		ok "Alicia propose ses intentions par maia-llm (modèle réel, CPU) : $TAUX phrases conformes en ${duree} s" ||
		nok "Alicia : intentions avec le vrai modèle" "code $rc ; $(grep -E 'TAUX|panicked|FAILED' "$W/alicia.out" | head -3 | tr '\n' ' ')"
	grep -q '« déverrouille la porte » → {"intent":"control_device","device_type":"lock".*"action":"unlock"' "$W/alicia.out" &&
		ok "Alicia : « déverrouille la porte » proposé comme commande de serrure (lock/unlock) par l'IA" ||
		nok "Alicia : proposition de serrure" "$(grep 'porte' "$W/alicia.out" | head -2 | tr '\n' ' ')"
	grep -q 'test l_ia_ne_deverrouille_jamais_une_porte_sous_la_politique_de_base ... ok' "$W/alicia.out" &&
		ok "Alicia : commande de serrure proposée par l'IA refusée par les Cores sous la politique installée (confirmation exigée)" ||
		nok "Alicia : serrure refusée sans confirmation"
	sudo -n cp "$W/maia-llm.toml.avant" "$TOML"
	sudo -n chown root:root "$TOML"
	sudo -n chmod 0644 "$TOML"
	sudo -n systemctl restart miyukini-maia-llm
	for _ in $(seq 1 120); do curl -s "$LLM/maia/llm/health" | grep -q '"model_loaded":true' && break; sleep 1; done
	! grep -q "$PUB_A" "$TOML" && ok "Alicia : clé d'essai retirée de maia-llm" || nok "Alicia : clé d'essai retirée"
else
	nok "Alicia : essai d'intégration absent du paquet ($ESSAI_A)"
fi

# ─────────────────────────────────────────────────────────────── 14. Douane : frontière externe, tc
echo "# 14. Douane : frontière externe (règles strictes → tc → filet), coupure, rétablissement, échec de tc"
DX=/opt/miyukini/libexec/douane-externe.sh
NSX=mcog-f-externe
etat_x() { sudo -n "$DX" etat | cut -d' ' -f1; }
nft_x() { sudo -n ip netns exec "$NSX" nft list table inet miyudouane; }
classes_x() { sudo -n ip netns exec "$NSX" tc class show dev "$1"; }
filet_x() { nft_x | grep 'ct direction original limit rate over 640 kbytes/second burst 6843 bytes' >/dev/null; }
strict_x() { nft_x | grep 'ct mark 0x00000001 limit rate over 512 kbytes/second' >/dev/null && ! nft_x | grep 'ct direction' >/dev/null; }
egalite "frontière externe : état initial filet" filet "$(etat_x)"
sudo -n "$DX" couper voie-a >"$W/x.out" 2>&1
rc=$?
[[ $rc -eq 0 && $(etat_x) == filet ]] && ! classes_x ext0 | grep '4d44:1000' >/dev/null && ! nft_x | grep 'voie=voie-a' >/dev/null &&
	ok "coupure de voie-a : classe tc et règles de la voie retirées, ordre a → b → c refait" || nok "coupure de voie" "rc $rc ; $(tail -c 300 "$W/x.out")"
sudo -n "$DX" retablir voie-a >"$W/x.out" 2>&1
rc=$?
[[ $rc -eq 0 && $(etat_x) == filet ]] && classes_x ext0 | grep '4d44:1000' >/dev/null && classes_x ext-franche | grep 'class htb 4d44:1 ' >/dev/null && filet_x &&
	ok "rétablissement de voie-a : classes et filet de retour" || nok "rétablissement de voie" "rc $rc ; $(tail -c 300 "$W/x.out")"
# Échec de (b) : une racine tc étrangère sur wg-fed (rien n'est touché) ⇒ règles strictes, aucun filet.
sudo -n ip netns exec "$NSX" tc qdisc del dev wg-fed root handle 4d44: 2>/dev/null
sudo -n ip netns exec "$NSX" tc qdisc add dev wg-fed root handle 1: prio
sudo -n "$DX" appliquer >"$W/x.out" 2>&1
rc=$?
[[ $rc -eq 3 && $(etat_x) == strict ]] && strict_x && grep -q 'étrangère' "$W/x.out" &&
	ok "tc refusé (racine étrangère sur wg-fed) : code 3, règle DEBIT stricte maintenue, AUCUN filet" || nok "échec de tc : stricte sans filet" "rc $rc ; état $(etat_x) ; $(tail -c 300 "$W/x.out")"
sudo -n ip netns exec "$NSX" tc qdisc show dev wg-fed | grep '^qdisc prio 1: root' >/dev/null &&
	ok "racine étrangère laissée intacte (jamais remplacée par la Douane)" || nok "racine étrangère intacte"
sudo -n ip netns exec "$NSX" tc qdisc del dev wg-fed root handle 1:
sudo -n "$DX" appliquer >"$W/x.out" 2>&1
rc=$?
[[ $rc -eq 0 && $(etat_x) == filet ]] && filet_x && sudo -n ip netns exec "$NSX" tc qdisc show dev wg-fed | grep '^qdisc htb 4d44: root' >/dev/null &&
	ok "racine étrangère retirée : (a) → (b) → (c) réussis, filet seulement après tc" || nok "réapplication après retrait" "rc $rc ; $(tail -c 300 "$W/x.out")"
# Nouvel arrêté externe altéré : refusé avant d'être retenu, l'arrêté en place reste appliqué.
sudo -n cat /var/lib/miyukini/douane/arretes/externe.arrete >"$W/altere.arrete"
python3 -I -c 'import sys; p=sys.argv[1]; b=bytearray(open(p,"rb").read()); b[40]^=0xff; open(p,"wb").write(b)' "$W/altere.arrete"
sudo -n "$DX" arrete "$W/altere.arrete" >"$W/x.out" 2>&1
rc=$?
[[ $rc -ne 0 && $(etat_x) == filet ]] && filet_x && ! sudo -n cmp -s "$W/altere.arrete" /var/lib/miyukini/douane/arretes/externe.arrete &&
	ok "arrêté externe altéré refusé : arrêté en place conservé, filet maintenu" || nok "arrêté altéré refusé" "rc $rc"
# Démarrage de l'espace (unité) : espace recréé, ordre a → b → c.
sudo -n systemctl restart miyukini-douane-externe
j=$(sudo -n journalctl -o cat _SYSTEMD_INVOCATION_ID="$(systemctl show -p InvocationID --value miyukini-douane-externe)" | grep -oE '^\[douane-externe\] \((a|b|c)\)' | grep -oE '[abc]\)$' | tr -d ')\n')
[[ $j == abc && $(etat_x) == filet ]] && ok "redémarrage de l'espace : (a) puis (b) puis (c), état filet" || nok "redémarrage de l'espace" "ordre « $j », état $(etat_x)"

# ─────────────────────────────────────────────────────────────── 12. AppArmor : aucun refus COG
echo "# 12. AppArmor : aucun refus des profils COG pendant le scénario"
# Les sondes de cloisonnement de la section 11 bis sont des refus VOULUS (fichier .sonde-cloisonnement).
REFUS=$(sudo -n journalctl -k --since "$DEBUT_NOYAU" --no-pager 2>/dev/null | grep 'apparmor="DENIED".*profile="miyukini-' | grep -vc '\.sonde-cloisonnement-')
[[ ${REFUS:-0} -eq 0 ]] && ok "aucun refus AppArmor (profils miyukini-*) dans le journal noyau" || nok "refus AppArmor pendant le scénario" "$REFUS refus"

echo "# bilan : $((N - NOK)) ok, $NOK not ok"
echo "1..$N"
exit "$NOK"
