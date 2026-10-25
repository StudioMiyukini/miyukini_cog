#!/usr/bin/env bash
# Essai réel du relais NFLOG (VM jetable, root via sudo) — miyurelais.
# Espaces de noms et table nftables PROPRES à l'essai (préfixe mrel-, table
# bridge mrelais_essai dans l'espace du pont) ; rien n'est touché dans
# l'espace réseau initial. Nettoyage en fin d'essai (et sur erreur).
# Usage : sudo ./essai_nflog.sh <chemin de essai_vm> <chemin de miyurelais>
set -u
ESSAI=$(readlink -f "$1"); RELAIS=$(readlink -f "$2")
G=111
OK=0; KO=0
verifier() { if grep -qE -- "$2" "$3"; then echo "OK  $1"; OK=$((OK+1)); else echo "KO  $1 (motif « $2 » absent de $3)"; KO=$((KO+1)); fi; }
absent()   { if grep -qE -- "$2" "$3"; then echo "KO  $1 (motif « $2 » présent dans $3)"; KO=$((KO+1)); else echo "OK  $1"; OK=$((OK+1)); fi; }
nettoyer() { for n in mrel-pont mrel-a mrel-b; do ip netns del "$n" 2>/dev/null; done; }
trap nettoyer EXIT
nettoyer
D=$(mktemp -d /tmp/mrel.XXXX)

# --- Topologie : pont de zone Franche, deux sandboxes (ports sv0, sv1).
for n in mrel-pont mrel-a mrel-b; do ip netns add "$n"; ip -n "$n" link set lo up; done
ip -n mrel-pont link add br0 type bridge
ip -n mrel-pont link set br0 up
ip link add mrel-x0 netns mrel-pont type veth peer name a0 netns mrel-a
ip link add mrel-x1 netns mrel-pont type veth peer name b0 netns mrel-b
ip -n mrel-pont link set mrel-x0 name sv0
ip -n mrel-pont link set mrel-x1 name sv1
for p in sv0 sv1; do ip -n mrel-pont link set "$p" master br0; ip -n mrel-pont link set "$p" up; done
ip -n mrel-a addr add 10.77.1.10/24 dev a0; ip -n mrel-a link set a0 up
ip -n mrel-b addr add 10.77.1.20/24 dev b0; ip -n mrel-b link set b0 up

# --- Règles de l'essai (convention MDOU, A-004) + témoin de vie.
ip netns exec mrel-pont nft -f - <<NFT
table bridge mrelais_essai {
  chain transit {
    type filter hook forward priority 0; policy accept;
    ether type ip ip saddr 10.77.1.10 ip daddr 10.77.1.20 tcp dport 8080 ct state new limit rate 20/second log group $G prefix "MDOU-TEMOIN|zone-franche|" accept
    ether type ip ip saddr 10.77.1.10 ip daddr 10.77.1.20 tcp dport 9999 log group $G prefix "MDOU|zone-franche|transit|NON-DECLARE|" drop
  }
}
NFT
ip netns exec mrel-pont nft list table bridge mrelais_essai > "$D/regles.txt"
ip -n mrel-a link show a0 >/dev/null && ip netns exec mrel-a ping -c1 -W1 10.77.1.20 >/dev/null && echo "OK  pont fonctionnel (ping)" || echo "KO  pont"

connexions() { # $1 = port, $2 = nombre
  for _ in $(seq "$2"); do ip netns exec mrel-a timeout 1 bash -c "echo > /dev/tcp/10.77.1.20/$1" 2>/dev/null; sleep 0.3; done; }

# --- 1. NON-DECLARE : ligne, signal critique attribué, coupure de la voie émettrice.
ip netns exec mrel-pont "$ESSAI" --groupe $G --voie jayoffice --interface sv0 --duree-s 5 > "$D/1.txt" 2>&1 &
sleep 1; connexions 9999 2; wait
verifier "1 ligne canonique relayée" "^LIGNE [0-9]+\.[0-9]{6} vm-essai nflog-$G: MDOU\|zone-franche\|transit\|NON-DECLARE\| IN=sv0 OUT=sv1 SRC=10\.77\.1\.10 DST=10\.77\.1\.20 LEN=60 TTL=64 PROTO=TCP SPT=[0-9]+ DPT=9999 SYN$" "$D/1.txt"
verifier "1 signal R-A005-NON-DECLARE critique sur la voie" "^SIGNAL regle=R-A005-NON-DECLARE voie=jayoffice severite=Critique categorie=FluxHorsContrat" "$D/1.txt"
verifier "1 coupure de la voie fautive" "^COUPURE voie=jayoffice motif=IncidentCritique" "$D/1.txt"
verifier "1 métadonnée IPS (SYN, entrée sv0)" "^META src=10\.77\.1\.10 dst=10\.77\.1\.20 dport=9999 nouvelle=true entree=Some\(\"sv0\"\)" "$D/1.txt"
absent  "1 aucune perte ni refus" "pertes=[1-9]|refuses=[1-9]|^ECHEC" "$D/1.txt"

# --- 2. Capteur muet : voie active, aucun paquet ⇒ voie aveugle coupée.
ip netns exec mrel-pont "$ESSAI" --groupe $G --voie jayoffice --interface sv0 --voie-active --delai-muet-s 2 --duree-s 4 > "$D/2.txt" 2>&1
verifier "2 échec « capteur muet »" "^ECHEC capteur muet" "$D/2.txt"
verifier "2 lot en échec" "^LOT etat=EnEchec" "$D/2.txt"
verifier "2 voie aveugle coupée" "^COUPURE voie=jayoffice motif=VoieAveugle" "$D/2.txt"

# --- 3. Témoin de vie : voie active avec trafic accepté ⇒ jamais muette.
ip netns exec mrel-pont "$ESSAI" --groupe $G --voie jayoffice --interface sv0 --voie-active --delai-muet-s 2 --duree-s 6 > "$D/3.txt" 2>&1 &
sleep 0.5; connexions 8080 14; wait
verifier "3 témoins reçus" "^STATS .*temoins=[1-9]" "$D/3.txt"
absent  "3 jamais muette, aucune coupure" "^ECHEC|^COUPURE" "$D/3.txt"
absent  "3 aucun témoin relayé en ligne" "^LIGNE .*TEMOIN" "$D/3.txt"

# --- 4. Binaire miyurelais : lignes sur la sortie standard.
ip netns exec mrel-pont "$RELAIS" --groupe $G --hote vm-essai --duree-s 3 > "$D/4.out" 2> "$D/4.err" &
sleep 1; connexions 9999 1; wait
verifier "4 binaire : ligne canonique" "^[0-9]+\.[0-9]{6} vm-essai nflog-$G: MDOU\|zone-franche\|transit\|NON-DECLARE\| IN=sv0 OUT=sv1 .* DPT=9999 SYN$" "$D/4.out"
verifier "4 binaire : statistiques" "^STATS .*lignes=[1-9]" "$D/4.err"

# --- 5. Groupe déjà lié : l'ouverture échoue (EBUSY), jamais en silence.
ip netns exec mrel-pont "$RELAIS" --groupe $G --hote vm-essai --duree-s 3 > /dev/null 2> "$D/5a.err" &
sleep 0.5
ip netns exec mrel-pont "$RELAIS" --groupe $G --hote vm-essai --duree-s 1 > /dev/null 2> "$D/5b.err"; echo "code=$?" >> "$D/5b.err"
wait
verifier "5 second relais refusé (code 3)" "code=3" "$D/5b.err"

echo "--- sorties : $D"; for f in "$D"/1.txt "$D"/2.txt; do echo "== $f"; head -20 "$f"; done
echo "BILAN ok=$OK ko=$KO"
[ "$KO" -eq 0 ]
