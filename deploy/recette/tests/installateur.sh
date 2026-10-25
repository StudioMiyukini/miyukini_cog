#!/usr/bin/env bash
# Essais de l'installateur de recette COG : refus des paquets non conformes et copie privée
# (sortie TAP, root).
#
#   sudo installateur.sh --paquet cog-<v>-<commit>.tar.zst --cle-publique-pem CLE.pem \
#        --confirme-machine-de-recette [--sans-reinstallation]
#
# Préalable : COG installé par installer.sh depuis ce même paquet (les refus ne doivent rien
# changer ; I7 le réinstalle). Durée : 2 à 4 minutes (I7 : réinstallation complète).
#
# Essais (paquet extrait dans /var/tmp, archive d'origine intacte) :
#   I1 lien symbolique : config/bridge.toml remplacé par un lien vers une copie non listée de même
#      contenu (même nombre de fichiers, SHA-256 égaux par le lien) : refus AVANT toute action
#      (pas de copie privée, pas d'authentification, rien d'installé) ;
#   I2 fichier parasite exploitation/MANIFESTE.sig : refus ;
#   I3 fichier listé remplacé par un autre, même nombre total : refus ;
#   I4 mauvaise clé publique, I5 mauvaise empreinte de clé, I6 fichier modifié : refus ;
#   I7 paquet dans un dossier modifiable par un autre compte (nobody) : dès la copie privée faite,
#      nobody réécrit installer.sh, commun.sh, bin/outil-sceau, exploitation/etat.sh et remplace
#      config/bridge.toml par un lien vers /etc/shadow. L'installation réussit, aucun code de
#      nobody n'est exécuté par root, les fichiers installés sont ceux du MANIFESTE signé.
# Pour chaque refus : code 1, motif affiché, « rien n'a été exécuté ni installé », /opt/miyukini
# inchangé, services actifs, aucune copie privée laissée dans /run.
set -uo pipefail
umask 0077

PAQUET=""
PEM=""
CONFIRME=0
REINSTALLER=1
while (($#)); do
	case $1 in
		--paquet) PAQUET=${2:?}; shift ;;
		--cle-publique-pem) PEM=${2:?}; shift ;;
		--confirme-machine-de-recette) CONFIRME=1 ;;
		--sans-reinstallation) REINSTALLER=0 ;;
		-h | --help) sed -n '2,23p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
		*) echo "option inconnue : $1" >&2; exit 2 ;;
	esac
	shift
done
((EUID == 0)) || { echo "root requis (sudo)" >&2; exit 2; }
((CONFIRME)) || { echo "refus : confirmez avec --confirme-machine-de-recette" >&2; exit 2; }
[[ -f $PAQUET && -f $PEM ]] || { echo "--paquet et --cle-publique-pem requis" >&2; exit 2; }
PAQUET=$(realpath "$PAQUET")
PEM=$(realpath "$PEM")

EXPL=/opt/miyukini/exploitation
SERVICES=(miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal)
TMP=$(mktemp -d /var/tmp/essai-installateur.XXXXXX)
chmod 0755 "$TMP"
trap 'rm -rf "$TMP"' EXIT

n=0
ko=0
ok() { n=$((n + 1)); echo "ok $n - $1"; }
nok() { n=$((n + 1)); ko=$((ko + 1)); echo "not ok $n - $1"; [[ -n ${2:-} ]] && echo "  # $2"; }
verifie() { local d=$1; shift; if "$@"; then ok "$d"; else nok "$d"; fi; }

empreinte_installee() { sha256sum /opt/miyukini/MANIFESTE | cut -d' ' -f1; }
# etat_code : empreinte de l'état installé (fichiers de /opt/miyukini, unités et profils miyukini-*).
etat_code() {
	{
		find /opt/miyukini -type f -printf '%p %s %T@\n'
		find /etc/systemd/system /etc/apparmor.d /etc/miyukini -maxdepth 1 -name 'miyukini-*' -printf '%p %s %T@\n'
	} 2>/dev/null | LC_ALL=C sort | sha256sum | cut -d' ' -f1
}
services_actifs() { local u; for u in "${SERVICES[@]}"; do systemctl is-active --quiet "$u" || return 1; done; }
zone() { nft list table inet miyudouane_hote >/dev/null 2>&1; }
copies_privees() { find /run -maxdepth 1 \( -name 'miyukini-paquet.*' -o -name 'miyukini-auth.*' \) | wc -l; }
sante() { (. "$EXPL/commun.sh" && COG_DELAI_SANTE=60 controle_sante) >/dev/null 2>&1; }

# extraire <nom> : paquet extrait dans $TMP/<nom>/ ; affiche la racine du paquet.
extraire() {
	mkdir -p "$TMP/$1"
	zstd -dc "$PAQUET" | tar -C "$TMP/$1" -xpf -
	find "$TMP/$1" -mindepth 1 -maxdepth 1
}

# sha_manifeste <racine> <chemin relatif> : SHA-256 attendu d'un fichier, selon le MANIFESTE signé.
sha_manifeste() { awk -v p="./$2" '$2 == p {print $1}' "$1/MANIFESTE"; }

# refus <libellé> <racine> <motif> <avant_copie 0|1> <args...>
refus() {
	local lib=$1 racine=$2 motif=$3 avant_copie=$4 j avant_m avant_c rc
	shift 4
	j=$TMP/$(tr -c 'a-zA-Z0-9' '_' <<<"$lib").log
	avant_m=$(empreinte_installee)
	avant_c=$(etat_code)
	"$racine/installer.sh" --confirme-machine-de-recette "$@" >"$j" 2>&1
	rc=$?
	if ((rc == 1)); then ok "$lib : refus (code 1)"; else nok "$lib : refus (code 1)" "code $rc"; fi
	if grep -q -- "$motif" "$j"; then ok "$lib : motif du refus affiché"; else nok "$lib : motif du refus affiché" "$(grep ERREUR "$j" | tail -n 2 | tr '\n' ' ')"; fi
	if grep -q "rien n'a été exécuté ni installé" "$j" && ! grep -q 'installation dans\|sceau de l.éditeur vérifié' "$j"; then
		ok "$lib : rien exécuté du paquet ni installé"
	else
		nok "$lib : rien exécuté du paquet ni installé" "$(tail -n 2 "$j" | tr '\n' ' ')"
	fi
	if ((avant_copie)); then
		if ! grep -q 'paquet copié dans\|signature Ed25519' "$j"; then ok "$lib : refus avant toute action (ni copie ni authentification)"; else nok "$lib : refus avant toute action (ni copie ni authentification)"; fi
	fi
	if [[ $(empreinte_installee) == "$avant_m" && $(etat_code) == "$avant_c" ]] && services_actifs && zone; then
		ok "$lib : /opt/miyukini, unités, services et zone inchangés"
	else
		nok "$lib : /opt/miyukini, unités, services et zone inchangés"
	fi
	(($(copies_privees) == 0)) && ok "$lib : aucune copie privée laissée dans /run" || nok "$lib : aucune copie privée laissée dans /run"
}

echo "# installateur de recette : refus des paquets non conformes, copie privée"
services_actifs && zone && ok "préalable : services COG actifs, zone COG présente" || nok "préalable : services COG actifs, zone COG présente"

# I1 : lien symbolique (même nombre de fichiers réguliers, SHA-256 égaux en suivant le lien).
r=$(extraire i1)
mv "$r/config/bridge.toml" "$r/config/.bridge.toml"
ln -s .bridge.toml "$r/config/bridge.toml"
refus "I1 lien symbolique dans le paquet" "$r" "ni fichier régulier ni dossier" 1 --cle-publique-pem "$PEM"

# I1b : dossier entier remplacé par un lien symbolique (config/strongfather -> copie non listée).
r=$(extraire i1b)
mv "$r/config/strongfather" "$r/config/.strongfather"
ln -s .strongfather "$r/config/strongfather"
refus "I1b dossier remplacé par un lien symbolique" "$r" "ni fichier régulier ni dossier" 1 --cle-publique-pem "$PEM"

# I2 : fichier parasite sous/MANIFESTE.sig (l'ancien contrôle « -name MANIFESTE.sig » l'excluait).
r=$(extraire i2)
printf '#!/bin/sh\nexit 0\n' >"$r/exploitation/MANIFESTE.sig"
refus "I2 fichier parasite exploitation/MANIFESTE.sig" "$r" "hors manifeste ./exploitation/MANIFESTE.sig" 0 --cle-publique-pem "$PEM"

# I3 : fichier listé remplacé par un autre, même nombre total.
r=$(extraire i3)
mv "$r/exploitation/etat.sh" "$r/exploitation/porte.sh"
refus "I3 fichier listé remplacé, même nombre total" "$r" "liste des fichiers différente" 0 --cle-publique-pem "$PEM"

# I4 : mauvaise clé ; I5 : mauvaise empreinte ; I6 : fichier modifié.
r=$(extraire i4)
openssl genpkey -algorithm ed25519 -out "$TMP/autre.key" 2>/dev/null
openssl pkey -in "$TMP/autre.key" -pubout -out "$TMP/autre.pem" 2>/dev/null
refus "I4 mauvaise clé publique" "$r" "signature du MANIFESTE refusée" 0 --cle-publique-pem "$TMP/autre.pem"
refus "I5 mauvaise empreinte de clé" "$r" "empreinte de la clé du paquet" 0 --empreinte-cle-pem "$(sha256sum "$TMP/autre.pem" | cut -d' ' -f1)"
printf '\n# altération\n' >>"$r/config/bridge.toml"
refus "I6 fichier du paquet modifié" "$r" "SHA-256" 0 --cle-publique-pem "$PEM"

# I7 : dossier modifiable par un autre compte ; modification après la copie privée.
if ((REINSTALLER)); then
	W=$TMP/i7
	r=$(extraire i7)
	chmod -R a+rwX "$W"
	MARQ=$TMP/i7-code-nobody-execute-par-root
	JOURNAL=$TMP/i7.log
	install -m 0644 /dev/null "$JOURNAL"
	attendu_commun=$(sha_manifeste "$r" commun.sh)
	attendu_etat=$(sha_manifeste "$r" exploitation/etat.sh)
	attendu_bridge=$(sha_manifeste "$r" config/bridge.toml)
	attendu_outil=$(sha_manifeste "$r" bin/outil-sceau)
	# Attaquant (nobody) : attend la copie privée, puis réécrit en place (même inode) les fichiers
	# que l'installateur exécute, source ou installe, et remplace bridge.toml par un lien.
	runuser -u nobody -- bash -c '
		r=$1 j=$2 marq=$3
		for _ in $(seq 1 1200); do grep -q "n.est plus relu" "$j" && break; sleep 0.05; done
		grep -q "installation dans" "$j" && avant=non || avant=oui
		charge="#!/bin/bash
touch $marq"
		printf "%s\n" "$charge" >"$r/installer.sh"
		printf "%s\n" "$charge" >>"$r/commun.sh"
		printf "%s\n" "$charge" >"$r/bin/outil-sceau"
		printf "%s\n" "$charge" >"$r/exploitation/etat.sh"
		rm -f "$r/config/bridge.toml" && ln -s /etc/shadow "$r/config/bridge.toml"
		echo "attaque avant_installation=$avant" >"$r/../attaque-faite"
	' attaquant "$r" "$JOURNAL" "$MARQ" &
	ATT=$!
	"$r/installer.sh" --confirme-machine-de-recette --cle-publique-pem "$PEM" >>"$JOURNAL" 2>&1
	rc=$?
	wait "$ATT"
	if ((rc == 0)); then ok "I7 dossier modifiable par nobody : installation réussie (code 0)"; else nok "I7 dossier modifiable par nobody : installation réussie (code 0)" "code $rc ; $(grep ERREUR "$JOURNAL" | tail -n 2 | tr '\n' ' ')"; fi
	grep -q 'avant_installation=oui' "$W/attaque-faite" 2>/dev/null &&
		ok "I7 : la source a été modifiée après la copie privée, avant l'installation" ||
		nok "I7 : la source a été modifiée après la copie privée, avant l'installation" "$(cat "$W/attaque-faite" 2>/dev/null)"
	grep -q 'modifiable par un autre compte' "$JOURNAL" && ok "I7 : avertissement sur le dossier source affiché" || nok "I7 : avertissement sur le dossier source affiché"
	[[ ! -e $MARQ ]] && ok "I7 : aucun code de nobody exécuté par root (installer.sh, commun.sh, outil-sceau)" || nok "I7 : aucun code de nobody exécuté par root (installer.sh, commun.sh, outil-sceau)"
	[[ $(sha256sum "$EXPL/commun.sh" | cut -d' ' -f1) == "$attendu_commun" ]] && ok "I7 : commun.sh installé = MANIFESTE signé" || nok "I7 : commun.sh installé = MANIFESTE signé"
	[[ $(sha256sum "$EXPL/etat.sh" | cut -d' ' -f1) == "$attendu_etat" ]] && ok "I7 : exploitation/etat.sh installé = MANIFESTE signé" || nok "I7 : exploitation/etat.sh installé = MANIFESTE signé"
	[[ -f /etc/miyukini/bridge.toml && ! -L /etc/miyukini/bridge.toml && $(sha256sum /etc/miyukini/bridge.toml | cut -d' ' -f1) == "$attendu_bridge" ]] &&
		ok "I7 : bridge.toml installé = MANIFESTE signé (lien vers /etc/shadow ignoré)" || nok "I7 : bridge.toml installé = MANIFESTE signé (lien vers /etc/shadow ignoré)"
	[[ $(sha256sum /opt/miyukini/bin/outil-sceau | cut -d' ' -f1) == "$attendu_outil" ]] && ok "I7 : outil-sceau installé = MANIFESTE signé" || nok "I7 : outil-sceau installé = MANIFESTE signé"
	verifie "I7 : services COG actifs et sains" sante
	(($(copies_privees) == 0)) && ok "I7 : copie privée effacée à la sortie" || nok "I7 : copie privée effacée à la sortie"
fi

echo "1..$n ko=$ko"
((ko == 0))
