#!/usr/bin/env bash
# Essais de mise à jour de la machine de recette COG (sortie TAP, root).
#
#   sudo mise-a-jour.sh --paquet-maj cog-<v>-<commit>-maj.tar.zst --cle-publique-pem CLE.pem \
#        --confirme-machine-de-recette
#
# Préalable : COG installé par installer.sh (paquet A). Le paquet de mise à jour (B) est signé par
# la même clé d'éditeur et a un MANIFESTE différent de celui installé. Durée : 5 à 10 minutes.
#
# Essais, dans cet ordre :
#   U1 paquet altéré (un octet d'un fichier, archive reconstruite) : refus, code 1, AVANT toute
#      sauvegarde préalable, version installée et services inchangés ;
#   U2 mauvaise clé publique (clé Ed25519 jetable) : idem ;
#   U3 mauvaise empreinte de clé (--empreinte-cle-pem) : idem ;
#   U6 lien symbolique dans le paquet (même nombre de fichiers, SHA-256 égaux par le lien) : idem ;
#   U7 fichier parasite exploitation/MANIFESTE.sig : idem ;
#   U8 fichier listé remplacé par un autre, même nombre total : idem ;
#   U9 lien physique dans l'archive : idem ;
#   U4 échec de santé forcé (--essai-echec-sante) : retour arrière automatique, code 3, ancienne
#      version restaurée, données conservées, zone COG présente, services actifs ;
#   U5 mise à jour réussie : code 0, MANIFESTE installé = celui de B, services sains.
set -uo pipefail
umask 0077

PAQUET=""
PEM=""
CONFIRME=0
while (($#)); do
	case $1 in
		--paquet-maj) PAQUET=${2:?}; shift ;;
		--cle-publique-pem) PEM=${2:?}; shift ;;
		--confirme-machine-de-recette) CONFIRME=1 ;;
		-h | --help) sed -n '2,21p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
		*) echo "option inconnue : $1" >&2; exit 2 ;;
	esac
	shift
done
((EUID == 0)) || { echo "root requis (sudo)" >&2; exit 2; }
((CONFIRME)) || { echo "refus : confirmez avec --confirme-machine-de-recette" >&2; exit 2; }
[[ -f $PAQUET && -f $PEM ]] || { echo "--paquet-maj et --cle-publique-pem requis" >&2; exit 2; }
PAQUET=$(realpath "$PAQUET")
PEM=$(realpath "$PEM")

EXPL=/opt/miyukini/exploitation
MAJ=$EXPL/mettre-a-jour.sh
PREALABLES=/var/backups/miyukini/prealables
SERVICES=(miyukini-kindmother miyukini-maia miyukini-miyucloud miyukini-cog-web-portal)
MARQUEUR=/var/lib/miyukini/cog-web-portal/essai-mise-a-jour.marqueur
TMP=$(mktemp -d /var/tmp/essai-maj.XXXXXX)
trap 'rm -rf "$TMP"' EXIT

n=0
ko=0
ok() { n=$((n + 1)); echo "ok $n - $1"; }
nok() { n=$((n + 1)); ko=$((ko + 1)); echo "not ok $n - $1"; [[ -n ${2:-} ]] && echo "  # $2"; }
verifie() { local d=$1; shift; if "$@"; then ok "$d"; else nok "$d"; fi; }

nb_prealables() { find "$PREALABLES" -maxdepth 1 -name '*.tar.zst.enc' 2>/dev/null | wc -l; }
empreinte_installee() { sha256sum /opt/miyukini/MANIFESTE | cut -d' ' -f1; }
services_actifs() { local u; for u in "${SERVICES[@]}"; do systemctl is-active --quiet "$u" || return 1; done; }
zone() { nft list table inet miyudouane_hote >/dev/null 2>&1; }
binaires_conformes() {
	(cd /opt/miyukini && grep -v '^#' MANIFESTE | grep '  \./bin/' | grep -v 'bin/arretes_essai$' | sha256sum --check --strict --quiet -)
}
sante() { (. "$EXPL/commun.sh" && COG_DELAI_SANTE=60 controle_sante) >/dev/null 2>&1; }

# lancer <journal> <args...> : exécute mettre-a-jour.sh, renvoie son code.
lancer() {
	local j=$1
	shift
	"$MAJ" "$@" --confirme-machine-de-recette >"$j" 2>&1
}

# refus_avant_sauvegarde <libellé> <journal> <motif attendu> <args...>
refus_avant_sauvegarde() {
	local lib=$1 j=$2 motif=$3 avant_p avant_m rc
	shift 3
	avant_p=$(nb_prealables)
	avant_m=$(empreinte_installee)
	lancer "$j" "$@"
	rc=$?
	if ((rc == 1)); then ok "$lib : refus (code 1)"; else nok "$lib : refus (code 1)" "code $rc"; fi
	if grep -q "$motif" "$j"; then ok "$lib : motif du refus affiché"; else nok "$lib : motif du refus affiché" "$(tail -n 2 "$j" | tr '\n' ' ')"; fi
	if ! grep -q 'sauvegarde préalable' "$j" && (($(nb_prealables) == avant_p)); then
		ok "$lib : aucune sauvegarde préalable créée (refus AVANT toute sauvegarde)"
	else
		nok "$lib : aucune sauvegarde préalable créée (refus AVANT toute sauvegarde)"
	fi
	if [[ $(empreinte_installee) == "$avant_m" ]] && services_actifs && zone; then
		ok "$lib : version installée, services et zone inchangés"
	else
		nok "$lib : version installée, services et zone inchangés"
	fi
}

echo "# mise à jour de la machine de recette : refus, retour arrière, réussite"
[[ -x $MAJ ]] || { nok "préalable : $MAJ installé"; echo "1..$n"; exit 1; }
services_actifs && zone && ok "préalable : services COG actifs, zone COG présente" || nok "préalable : services COG actifs, zone COG présente"
A=$(empreinte_installee)
B=$(zstd -dc "$PAQUET" | tar -xOf - --wildcards '*/MANIFESTE' | sha256sum | cut -d' ' -f1)
[[ $A != "$B" ]] && ok "préalable : paquet B différent de la version installée (MANIFESTE)" ||
	nok "préalable : paquet B différent de la version installée (MANIFESTE)"
echo "essai $(date -u +%FT%TZ)" >"$MARQUEUR"
chown miyukini-portail:miyukini-portail "$MARQUEUR"

# U1 : paquet altéré (un fichier de config modifié, archive reconstruite sans .sha256).
mkdir -p "$TMP/altere"
zstd -dc "$PAQUET" | tar -C "$TMP/altere" -xpf -
racine=$(find "$TMP/altere" -mindepth 1 -maxdepth 1)
printf '\n# altération\n' >>"$racine/config/bridge.toml"
tar -C "$TMP/altere" -cf - "$(basename "$racine")" | zstd -q -o "$TMP/altere.tar.zst"
refus_avant_sauvegarde "U1 paquet altéré" "$TMP/u1.log" "SHA-256" \
	--paquet "$TMP/altere.tar.zst" --cle-publique-pem "$PEM"

# U1b : MANIFESTE lui-même modifié (signature détachée invalide).
rm -rf "$TMP/altere"
mkdir -p "$TMP/altere"
zstd -dc "$PAQUET" | tar -C "$TMP/altere" -xpf -
racine=$(find "$TMP/altere" -mindepth 1 -maxdepth 1)
sed -i '1s/$/ altere/' "$racine/MANIFESTE"
tar -C "$TMP/altere" -cf - "$(basename "$racine")" | zstd -q -o "$TMP/altere2.tar.zst"
refus_avant_sauvegarde "U1b MANIFESTE modifié" "$TMP/u1b.log" "signature du MANIFESTE refusée" \
	--paquet "$TMP/altere2.tar.zst" --cle-publique-pem "$PEM"

# U2 : mauvaise clé (clé Ed25519 jetable).
openssl genpkey -algorithm ed25519 -out "$TMP/autre.key" 2>/dev/null
openssl pkey -in "$TMP/autre.key" -pubout -out "$TMP/autre.pem" 2>/dev/null
refus_avant_sauvegarde "U2 mauvaise clé publique" "$TMP/u2.log" "signature du MANIFESTE refusée" \
	--paquet "$PAQUET" --cle-publique-pem "$TMP/autre.pem"

# U3 : mauvaise empreinte de clé.
refus_avant_sauvegarde "U3 mauvaise empreinte de clé" "$TMP/u3.log" "empreinte de la clé" \
	--paquet "$PAQUET" --empreinte-cle-pem "$(sha256sum "$TMP/autre.pem" | cut -d' ' -f1)"

# archive_alteree <nom> <script bash exécuté dans la racine du paquet extrait> : archive dérivée de
# B (MANIFESTE, signature et sceau d'origine conservés), écrite dans $TMP/<nom>.tar.zst.
archive_alteree() {
	local nom=$1 script=$2 racine
	rm -rf "$TMP/$nom"
	mkdir -p "$TMP/$nom"
	zstd -dc "$PAQUET" | tar -C "$TMP/$nom" -xpf -
	racine=$(find "$TMP/$nom" -mindepth 1 -maxdepth 1)
	(cd "$racine" && bash -euc "$script")
	tar -C "$TMP/$nom" -cf - "$(basename "$racine")" | zstd -q -o "$TMP/$nom.tar.zst"
	rm -rf "${TMP:?}/$nom"
}

# U6 : lien symbolique. config/bridge.toml devient un lien vers une copie non listée de même
# contenu : même nombre de fichiers réguliers, SHA-256 identiques en suivant le lien (l'ancien
# contrôle par nombre l'acceptait). Refus avant toute sauvegarde.
archive_alteree u6 'mv config/bridge.toml config/.bridge.toml && ln -s .bridge.toml config/bridge.toml'
refus_avant_sauvegarde "U6 lien symbolique dans le paquet" "$TMP/u6.log" "ni fichier régulier ni dossier" \
	--paquet "$TMP/u6.tar.zst" --cle-publique-pem "$PEM"

# U7 : fichier parasite sous/MANIFESTE.sig (l'ancien contrôle « -name MANIFESTE.sig » l'excluait).
archive_alteree u7 'printf "#!/bin/sh\nexit 0\n" >exploitation/MANIFESTE.sig'
refus_avant_sauvegarde "U7 fichier parasite exploitation/MANIFESTE.sig" "$TMP/u7.log" "hors manifeste ./exploitation/MANIFESTE.sig" \
	--paquet "$TMP/u7.tar.zst" --cle-publique-pem "$PEM"

# U8 : fichier listé remplacé par un autre, même nombre total (etat.sh retiré, porte.sh ajouté).
archive_alteree u8 'mv exploitation/etat.sh exploitation/porte.sh'
refus_avant_sauvegarde "U8 fichier listé remplacé, même nombre total" "$TMP/u8.log" "liste des fichiers différente" \
	--paquet "$TMP/u8.tar.zst" --cle-publique-pem "$PEM"

# U9 : lien physique dans l'archive (refusé dès le listage, avant extraction).
archive_alteree u9 'rm config/bridge.toml && ln config/strongfather/"$(ls config/strongfather | head -n1)" config/bridge.toml'
refus_avant_sauvegarde "U9 lien physique dans l'archive" "$TMP/u9.log" "ni fichier régulier ni dossier" \
	--paquet "$TMP/u9.tar.zst" --cle-publique-pem "$PEM"

# U4 : échec de santé forcé -> retour arrière automatique.
avant_p=$(nb_prealables)
lancer "$TMP/u4.log" --paquet "$PAQUET" --cle-publique-pem "$PEM" --essai-echec-sante
rc=$?
if ((rc == 3)); then ok "U4 échec de santé forcé : retour arrière automatique réussi (code 3)"; else nok "U4 échec de santé forcé : retour arrière automatique réussi (code 3)" "code $rc ; $(tail -n 3 "$TMP/u4.log" | tr '\n' ' ')"; fi
(($(nb_prealables) == avant_p + 1)) && ok "U4 : une sauvegarde préalable créée avant le remplacement" || nok "U4 : une sauvegarde préalable créée avant le remplacement"
grep -q 'ÉCHEC INJECTÉ' "$TMP/u4.log" && ok "U4 : l'échec injecté a bien été atteint après installation de B" || nok "U4 : l'échec injecté a bien été atteint après installation de B"
[[ $(empreinte_installee) == "$A" ]] && ok "U4 : version précédente restaurée (MANIFESTE installé = A)" || nok "U4 : version précédente restaurée (MANIFESTE installé = A)"
verifie "U4 : binaires restaurés conformes au MANIFESTE" binaires_conformes
[[ -f $MARQUEUR ]] && ok "U4 : données conservées (marqueur présent)" || nok "U4 : données conservées (marqueur présent)"
verifie "U4 : zone COG présente après retour arrière" zone
verifie "U4 : services COG actifs et sains après retour arrière" sante
grep -q 'retour arriere ok' /var/backups/miyukini/mises-a-jour.journal && ok "U4 : retour arrière tracé au journal" || nok "U4 : retour arrière tracé au journal"

# U5 : mise à jour réussie.
lancer "$TMP/u5.log" --paquet "$PAQUET" --cle-publique-pem "$PEM"
rc=$?
if ((rc == 0)); then ok "U5 mise à jour réussie (code 0)"; else nok "U5 mise à jour réussie (code 0)" "code $rc ; $(tail -n 3 "$TMP/u5.log" | tr '\n' ' ')"; fi
[[ $(empreinte_installee) == "$B" ]] && ok "U5 : MANIFESTE installé = celui du paquet B" || nok "U5 : MANIFESTE installé = celui du paquet B"
verifie "U5 : binaires conformes au MANIFESTE" binaires_conformes
[[ -f $MARQUEUR ]] && ok "U5 : données conservées (marqueur présent)" || nok "U5 : données conservées (marqueur présent)"
verifie "U5 : zone COG présente" zone
verifie "U5 : services COG actifs et sains" sante
grep -q "^.* ok " /var/backups/miyukini/mises-a-jour.journal && ok "U5 : mise à jour tracée au journal" || nok "U5 : mise à jour tracée au journal"

rm -f "$MARQUEUR"
echo "1..$n ko=$ko"
((ko == 0))
