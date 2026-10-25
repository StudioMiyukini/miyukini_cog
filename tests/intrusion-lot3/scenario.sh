# shellcheck shell=bash
# Banc d'intrusion du lot 3 — constantes du scénario.
#
# Doit rester aligné sur :
#   - crates/miyudouane/examples/arretes_essai.rs (voies, ports, liste blanche) ;
#   - PlanReseau::reference (adresses, espaces de noms, groupes NFLOG).
# L'ordre de SANDBOXES fixe les adresses (.10, .11, …) et les ports de pont
# (sv0, sv1, …) : ne pas le modifier sans revoir les adresses ci-dessous.
# lancer.sh contrôle la cohérence avec le script généré avant toute action.

# shellcheck disable=SC2034  # variables utilisées par lancer.sh / lib.sh

SANDBOXES=(
	cog:voie-a      # sv0  10.77.0.10
	cog:voie-b      # sv1  10.77.0.11
	franche:voie-a  # sv2  10.77.1.10
	franche:voie-b  # sv3  10.77.1.11
	cog:capteur     # sv4  10.77.0.12
	locale:capteur  # sv5  10.77.2.10
)

# Espaces de noms du plan de référence (miyudouane).
NS_Z_COG=mcog-z-cog
NS_Z_FRANCHE=mcog-z-franche
NS_Z_LOCALE=mcog-z-locale
NS_F_EXT=mcog-f-externe
NS_F_PI=mcog-f-portail
NS_F_PL=mcog-f-local
NS_COG_A=mcog-s-cog-voie-a
NS_COG_B=mcog-s-cog-voie-b
NS_COG_CAPT=mcog-s-cog-capteur
NS_FR_A=mcog-s-franche-voie-a
NS_FR_B=mcog-s-franche-voie-b
NS_LOC_CAPT=mcog-s-locale-capteur
# Propre au banc : « Internet » simulé derrière la frontière externe.
NS_INET=mcog-t-inet
# Propre au banc : espace jetable du contrôle à blanc « nft -c ».
NS_VERIF=mcog-t-verif

NS_ZONES=("$NS_Z_COG" "$NS_Z_FRANCHE" "$NS_Z_LOCALE")
NS_FRONTIERES=("$NS_F_EXT" "$NS_F_PI" "$NS_F_PL")
NS_SANDBOXES=("$NS_COG_A" "$NS_COG_B" "$NS_FR_A" "$NS_FR_B" "$NS_COG_CAPT" "$NS_LOC_CAPT")
NS_PLAN=("${NS_ZONES[@]}" "${NS_FRONTIERES[@]}" "${NS_SANDBOXES[@]}")
NS_TOUS=("${NS_PLAN[@]}" "$NS_INET" "$NS_VERIF")

# Groupes NFLOG (convention miyudouane).
declare -A GROUPE_NFLOG=(
	["$NS_F_EXT"]=101 ["$NS_F_PI"]=102 ["$NS_F_PL"]=103
	["$NS_Z_COG"]=110 ["$NS_Z_FRANCHE"]=111 ["$NS_Z_LOCALE"]=112
)

# Adresses.
IP_COG_A=10.77.0.10
IP_COG_B=10.77.0.11
IP_COG_CAPT=10.77.0.12
IP_FR_A=10.77.1.10
IP_FR_B=10.77.1.11
IP_LOC_CAPT=10.77.2.10
IP_PI_COG=10.77.0.253   # Portail interne, côté zone COG
IP_PI_FR=10.77.1.253    # Portail interne, côté Franche
IP_PL_COG=10.77.0.252   # Portail local, côté zone COG
IP_PL_LOC=10.77.2.252   # Portail local, côté Franche locale
IP_EXT_FR=10.77.1.254   # Frontière externe, passerelle de la Franche
# « Internet » du banc (TEST-NET-3 en liste blanche, TEST-NET-1 hors liste).
IP_EXT0=203.0.113.254
IP_INET=203.0.113.1
IP_EXT0_HORS=192.0.2.254
IP_INET_HORS=192.0.2.1
# IPv6 forcé (sonde de défense en profondeur).
IP6_COG_A=fd77::a
IP6_COG_B=fd77::b
IP6_PI=fd77::fd

# Interfaces propres au banc : paire veth hôte → frontière externe (ext0).
IF_PHYSIQUE=mcogt-phy
IF_INET=inet0

# Ports.
PORT_A=7443       # voie-a, portail interne
PORT_B=7080       # voie-b, portail interne
PORT_CAPT=1883    # capteur, portail local
PORT_EXT=443      # voie-a, frontière externe (International → Franche)
PORT_SSH=22       # jamais déclaré
PORT_COG=9000     # écouteur dans les sandboxes COG (cible interdite)
PORT_WEB=80       # écouteur « Internet »

# Voies.
VOIE_A=voie-a
VOIE_B=voie-b
VOIE_CAPT=capteur

# Répertoire des règles lu par le script de mise en place (zones::REPERTOIRE_REGLES).
REPERTOIRE_REGLES=/etc/miyukini/douane
