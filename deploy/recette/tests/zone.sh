#!/usr/bin/env bash
# Résistance de la zone COG (table inet miyudouane_hote) : sortie TAP.
# Exécuté sur la VM de recette ou la machine de recette, par un compte sudo.
#   zone.sh            (durée : environ 4 minutes)
#
# Invariant testé : jamais de service COG actif SANS table de zone. Trois cas :
#   Z1 systemctl restart nftables (flush ruleset d'Ubuntu) ;
#   Z2 table supprimée de l'extérieur, fichier de zone présent : rechargement par le minuteur ;
#   Z3 table supprimée, fichier de zone absent : le minuteur arrête les services COG (fail-closed).
# Le journal système (logger -t miyukini-douane) trace les rechargements et les arrêts.
set -u
SERVICES=(miyukini-kindmother miyukini-maia-llm miyukini-maia miyukini-miyucloud miyukini-cog-web-portal)
NOF=/etc/miyukini/douane-cog.nft
n=0
ko=0
ok() { n=$((n + 1)); echo "ok $n - $1"; }
nok() { n=$((n + 1)); ko=$((ko + 1)); echo "not ok $n - $1"; }
table() { sudo -n nft list table inet miyudouane_hote >/dev/null 2>&1; }
actifs() { local u; for u in "${SERVICES[@]}"; do systemctl is-active --quiet "$u" && echo "$u"; done; }
attendre() { # attendre <secondes> <commande...> : vrai si la commande réussit dans le délai
	local delai=$1 t=0
	shift
	while ! "$@"; do
		sleep 2
		t=$((t + 2))
		((t >= delai)) && return 1
	done
	return 0
}
demarrer_services() {
	local u
	for u in "${SERVICES[@]}"; do sudo -n systemctl start "$u"; done
	sleep 3
}

echo "# zone COG : restauration et repli"
demarrer_services
[[ $(actifs | wc -l) -eq ${#SERVICES[@]} ]] || { nok "préalable : services COG actifs"; echo "1..$n ko=$ko"; exit 1; }
table && ok "préalable : table miyudouane_hote présente, services actifs" || nok "préalable : table absente"

# Z1 : redémarrage de nftables.
sudo -n systemctl restart nftables
sleep 6
if table; then
	ok "Z1 restart nftables : table de zone rétablie par la Douane (PartOf=nftables)"
else
	nok "Z1 restart nftables : table de zone ABSENTE"
fi
if table || [[ $(actifs | wc -l) -eq 0 ]]; then
	ok "Z1 restart nftables : aucun service COG actif sans zone (actifs=$(actifs | wc -l))"
else
	nok "Z1 restart nftables : services COG actifs SANS zone"
fi
demarrer_services
[[ $(actifs | wc -l) -eq ${#SERVICES[@]} ]] && ok "Z1 : services COG redémarrés, zone présente" || nok "Z1 : services COG non redémarrés"

# Z2 : table supprimée, fichier de zone présent : le minuteur recharge la table.
sudo -n nft delete table inet miyudouane_hote
if attendre 130 table; then
	ok "Z2 table supprimée : rechargée par le minuteur (fichier de zone présent)"
else
	nok "Z2 table supprimée : non rechargée dans le délai"
fi
[[ $(actifs | wc -l) -eq ${#SERVICES[@]} ]] && ok "Z2 : services COG toujours actifs après rechargement" ||
	nok "Z2 : services COG arrêtés après un simple rechargement"

# Z3 : table supprimée, fichier de zone absent : repli fail-closed.
sudo -n mv "$NOF" "$NOF.hors-essai"
sudo -n nft delete table inet miyudouane_hote
if attendre 130 bash -c "[[ \$(systemctl is-active miyukini-maia) != active ]]"; then
	ok "Z3 zone non rétablissable : services COG arrêtés par le minuteur (fail-closed)"
else
	nok "Z3 zone non rétablissable : services COG toujours actifs"
fi
table && nok "Z3 : table réapparue sans fichier de zone" || ok "Z3 : aucune zone recréée sans fichier"
sudo -n mv "$NOF.hors-essai" "$NOF"
sudo -n systemctl restart miyukini-douane-cog
demarrer_services
table && [[ $(actifs | wc -l) -eq ${#SERVICES[@]} ]] &&
	ok "Z3 : fichier de zone rétabli, zone et services COG revenus" || nok "Z3 : retour à la normale incomplet"

echo "1..$n ko=$ko"
[[ $ko -eq 0 ]]
