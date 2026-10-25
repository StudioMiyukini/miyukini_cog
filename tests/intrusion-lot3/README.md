# Banc d'intrusion local du lot 3

Critère de sortie du lot 3 ([PLAN-IMPLEMENTATION §5](../../docs/implementation/PLAN-IMPLEMENTATION.md)) :

> test d'intrusion local — aucun flux non déclaré ne traverse ; coupure d'une voie sans impact sur les autres (EX-49)

Ce banc monte le réseau des zones **exactement comme la Douane le rend** (`miyudouane mise-en-place`, règles
`miyudouane regles` / `miyudouane zone` compilées depuis des arrêtés signés), puis tente des connexions
déclarées et interdites avec des outils standard. Il produit un rapport TAP et sort en erreur au moindre écart.

## ⚠ Où le lancer

- **Uniquement sur une machine virtuelle jetable dédiée**, en root. **Jamais sur un hôte de production** ni sur
  le serveur miyukini : le banc crée des espaces de noms `mcog-*`, charge `nf_conntrack_bridge` et écrit dans
  `/etc/miyukini/douane`.
- Le banc refuse de démarrer si : pas root, `--vm-jetable` absent, hôte non virtualisé (`systemd-detect-virt --vm`,
  sauf `--accepter-hote-physique`), espaces `mcog-*` déjà présents, `/etc/miyukini/douane` non vide,
  `br_netfilter` chargé.
- Le retrait est automatique (piège `EXIT`, aussi sur échec ou Ctrl-C) : processus du banc tués, script
  `miyudouane retrait`, espaces et interfaces du banc supprimés, fichiers de règles et répertoires créés par le
  banc retirés, module `nf_conntrack_bridge` déchargé s'il a été chargé par le banc.

## Prérequis

| Élément | Détail |
|---|---|
| Noyau | Linux ≥ 5.3 avec `nf_tables`, `nf_conntrack`, `nf_conntrack_bridge` (suivi de connexion dans le pont), `nfnetlink_log` (NFLOG) |
| `br_netfilter` | **ne doit pas être chargé** : il ferait passer les trames pontées par les crochets `inet` des espaces de zone, qui rejettent tout (le filtrage des zones est fait par `table bridge` + `nf_conntrack_bridge`). Docker le charge : `modprobe -r br_netfilter` sur la VM |
| Outils | `iproute2` (`ip`, `ss`), `nftables` ≥ 1.0, `kmod`, `procps` (`sysctl`), `coreutils` (`timeout`), `bash` ≥ 4 |
| Écouteur TCP | `socat`, sinon `python3`, sinon `nc` (OpenBSD, busybox ou traditionnel) — détection automatique ; le client n'utilise que `bash` (`/dev/tcp`) |
| Journaux | `tcpdump` avec prise en charge `nflog` (recommandé). Sans lui, seuls le compteur `rejets` et la présence des règles `MDOU|…` sont vérifiés |
| Facultatif | `ping` (sondes ICMP), `nmap` (balayages de ports) |
| Construction | `cargo` (dans le compte utilisateur ; root n'en a généralement pas besoin) |

Paquets Debian/Ubuntu indicatifs : `iproute2 nftables kmod procps socat tcpdump iputils-ping nmap`.

## Lancer

```sh
# 1. En utilisateur : construit (cargo --release), génère les arrêtés et les règles,
#    imprime toutes les commandes privilégiées sans les exécuter.
tests/intrusion-lot3/lancer.sh --dry-run

# 2. En root sur la VM jetable : banc réel (sans reconstruire).
sudo tests/intrusion-lot3/lancer.sh --vm-jetable --sans-construction
```

Options : `--travail <rép>` (répertoire de travail, défaut `mktemp`), `--cible <rép>` (binaires, défaut
`target/release`), `--delai <s>` (délai d'une tentative, défaut 3), `--accepter-hote-physique`.

Codes de retour : `0` tout passe · `1` au moins un `not ok` · `2` banc non installable (`Bail out!`).
Le répertoire de travail conserve les arrêtés, les règles générées, ce que chaque écouteur a reçu
(`recu/`) et les captures NFLOG (`journaux/nflog_<espace>.pcap`, lisibles avec `tshark -V`, champ `nflog.prefix`).

## Ce que fait le banc

1. **Construction** : `cargo build --release -p miyudouane -p miyuportail`, puis l'exemple
   `miyudouane --example arretes_essai`.
2. **Génération (sans privilège)** :
   - `arretes_essai` tire une clé **jetable** (`/dev/urandom`, jamais écrite) et scelle trois arrêtés
     (Sceau v1.1, valides 24 h) :

     | Frontière | Voie | Flux |
     |---|---|---|
     | portail-interne | `voie-a` | COG → Franche, tcp 7443 |
     | portail-interne | `voie-b` | COG → Franche, tcp 7080 |
     | portail-local | `capteur` | COG → Franche locale, tcp 1883 |
     | externe | `voie-a` | International (203.0.113.0/24) → Franche, tcp 443 |

   - `miyudouane regles` (une instance par frontière) et `miyudouane zone` (pont filtré de chaque zone,
     compilé depuis les arrêtés de toutes les frontières qui la touchent) vérifient chaque arrêté contre
     l'ancre et auditent le texte produit ;
   - `miyudouane mise-en-place` / `retrait` : scripts de mise en place et de retrait des zones ;
   - contrôle de cohérence entre [`scenario.sh`](scenario.sh) et le plan généré (adresses, ports de pont).
3. **Mise en place (root)** : `nft -c -f` de chaque fichier dans un espace jetable, installation dans
   `/etc/miyukini/douane`, puis le script **généré** par la Douane. L'« Internet » du banc
   (`mcog-t-inet`, 203.0.113.1 en liste blanche et 192.0.2.1 hors liste) est relié à la frontière externe
   par le mécanisme prévu pour la carte physique (`MCOG_IF_PHYSIQUE` → `ext0`).
4. **Sondes** (ci-dessous), avec des écouteurs sur chaque cible, y compris les cibles interdites : un flux
   interdit qui passerait serait vu (jeton reçu).
5. **Coupure, fermeture, rétablissement** par la CLI de la Douane (`--appliquer` : audit, `nft -c`, `nft -f`
   dans l'espace de la frontière).
6. **Retrait**.

### Verdict d'une sonde

- **Flux déclaré** : la connexion aboutit **et** le jeton unique envoyé arrive à l'écouteur attendu.
- **Flux interdit** : la connexion expire ou n'a pas de route (ou EPERM, rejet par la chaîne de sortie
  locale). Un **refus TCP** (RST) est un échec : il prouverait que le paquet a atteint un hôte. Le jeton ne
  doit être reçu par **aucun** écouteur. Puis une ligne TAP vérifie le **journal Douane** : le compteur nommé
  `rejets` de l'espace a progressé (rejet effectif) **et** la capture NFLOG de l'espace contient au moins une
  ligne de plus portant le préfixe attendu `MDOU|<lieu>|<chaîne>|<MOTIF>|` (`group` 101–103 frontières,
  110–112 zones). La journalisation est bornée (A-016 : une ligne par flux et par seconde, 50 lignes/s par
  table) : le banc attend **au moins une** ligne par sonde, pas une par paquet. La règle `log` ne porte pas
  de compteur ; sans `tcpdump` nflog, seuls le compteur `rejets` et la présence de la règle du préfixe sont
  vérifiés.

> Pourquoi pas `journalctl -k` / `dmesg` : les règles de la Douane journalisent en **NFLOG** (`log … group N`),
> destiné aux Vigies, et non dans le journal noyau ; de plus le journal noyau est désactivé par défaut hors de
> l'espace initial (`nf_log_all_netns=0`). Le banc lit donc les groupes NFLOG (`tcpdump -i nflog:N`) et le
> compteur nommé `rejets`, sans modifier les règles générées.

## Sondes et exigences couvertes

Correspondances : **EX-12** sortie de zone COG = incident journalisé · **EX-17** tout flux non déclaré refusé ·
**EX-19** une instance par frontière, vérification finale côté zone COG · **EX-20** aucune route vers/depuis la
zone COG · **EX-21** zone COG segmentée, une sandbox par service · **EX-29** espace isolé par service en Franche ·
**EX-44** voies à sens unique par service, rupture protocolaire · **EX-49** coupure d'une voie sans impact ·
INV-Z2/Z4/Z6/Z7 du [modèle de zones](../../docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md).

### Structure

| Contrôle | Prouve |
|---|---|
| `table inet miyudouane` chargée, 3 chaînes `policy drop`, dans chaque espace de frontière | EX-19 : une instance par frontière, refus par défaut |
| `table bridge miyudouane_zone` chargée dans chaque zone | EX-21 : pont de zone filtré |
| Sandboxes COG et Franche locale : aucune route par défaut, aucune route vers Internet | EX-20, INV-Z4 |
| Frontière externe : aucune route vers la zone COG | EX-20 |
| Portails : `ip_forward=0` | EX-44 : rupture protocolaire, rien ne transite |
| IPv6 désactivé dans tous les espaces | surface IPv6 nulle |

### Flux déclarés (doivent passer)

| Sonde | Exigence |
|---|---|
| voie A : COG A → Portail interne :7443 ; voie B : COG B → :7080 | EX-44 |
| sorties des voies : Portail interne → Franche A :7443, → Franche B :7080 | EX-44 |
| capteur : COG → Portail local :1883, puis Portail local → Franche locale :1883 | EX-19 |
| International (liste blanche) → Franche A :443 via la frontière externe | EX-19, INV-Z6 |

### Flux non déclarés (doivent échouer, journal attendu)

| Sonde | Journal attendu | Exigence |
|---|---|---|
| latéral COG A → COG B (TCP et ping) | `zone-cog … LATERAL` | EX-21 |
| COG A → Portail sur le port de la voie B, sur 22, vers le Portail local (voie du capteur) | `zone-cog … SORTIE-COG` | EX-21, EX-17 |
| ping COG A → Portail | `zone-cog … SORTIE-COG` | EX-12 |
| COG A → Internet sans route | — (absence de route) | EX-20, INV-Z4 |
| COG A → Internet et → Franche avec **route par défaut forcée** | `zone-cog … SORTIE-COG` | EX-12 (défense en profondeur) |
| latéral Franche A → Franche B | `zone-franche … LATERAL` | EX-29 |
| Franche A → COG A direct | `zone-franche … NON-DECLARE` | INV-Z2 |
| Franche A ouvre vers le Portail interne | `zone-franche … NON-DECLARE` | INV-Z7 (seule la zone COG ouvre) |
| Franche A → Internet non déclaré | `zone-franche … NON-DECLARE` | EX-29, EX-17 |
| International → Franche A :22 | `externe|transit|NON-DECLARE` | EX-17 |
| International hors liste blanche (192.0.2.1) → Franche A :443 | `externe|transit|NON-DECLARE` | INV-Z6 |
| International → COG A | — (aucune route dans la frontière) | EX-20 |
| Franche locale → Internet sans route, puis avec route forcée ; → COG avec route forcée ; ouverture vers le Portail local | `zone-locale … NON-DECLARE` | EX-29 |
| **IPv6 forcé** (réactivé pour l'essai) : COG A → COG B, COG A → Portail en IPv6 sur le port de la voie A | `zone-cog … LATERAL` ou `NON-DECLARE` | refus de toute famille non déclarée |
| balayages `nmap` (facultatifs) : COG A → Portail = {7443} ; Franche A → Portail = {} ; International → Franche A = {443} | — | EX-17 |
| bilan : aucun des jetons interdits reçu par un écouteur | — | EX-17 |

### Coupure, fermeture, rétablissement (EX-49, EX-19)

| Étape | Attendu |
|---|---|
| `miyudouane regles --frontiere portail-interne … --couper voie-a --appliquer` | la voie A est coupée **dans les deux sens** : COG A → Portail bloqué (`portail-interne|entree|SORTIE-COG`), Portail → Franche A bloqué (`portail-interne|sortie|NON-DECLARE`) |
| pendant la coupure | voie B intacte (deux sens), Portail local intact, frontière externe intacte (même nom de voie `voie-a`, autre instance) |
| `miyudouane fermeture --frontiere portail-local --appliquer` | capteur bloqué (`portail-local|entree|FERME`), Portail interne non affecté |
| `miyudouane regles … --appliquer` (sans coupure) | voie A et capteur rétablis, voie B toujours intacte |

## Limites connues

- Le **Portail** n'a pas encore de processus réseau (seul `portail-sas` existe) : ses points d'arrivée sont des
  écouteurs de test dans l'espace du Portail. Le banc éprouve la **couche réseau** (Douane, zones), pas la
  rupture protocolaire MSCE ni la purge des files lors d'une coupure.
- La coupure n'est appliquée qu'à l'instance de frontière ; le pont de la zone COG garde la règle de la voie
  (la frontière fait la vérification finale). `miyudouane zone --couper` permet de couper aussi au pont.
- Une connexion **déjà établie** survit à une coupure tant que son état conntrack existe ; la purge
  (`conntrack -D`) relève de l'application de la coupure par WorrySentinel, hors de ce banc.
- La mise en forme du débit (`tc`) et les plafonds `DEBIT` / `TAILLE` ne sont pas sondés ici (couverts par les
  tests de l'IPS).
- Les unités systemd des sandboxes (`miyudouane zones`) ne sont pas installées : les sandboxes sont de simples
  espaces de noms.

## Fichiers

- [`lancer.sh`](lancer.sh) — orchestration (pré-vol, construction, génération, mise en place, sondes, retrait)
- [`lib.sh`](lib.sh) — exécution / `--dry-run`, TAP, écouteurs, client, compteurs, NFLOG
- [`scenario.sh`](scenario.sh) — espaces, adresses, ports, voies (aligné sur le plan de référence)
- [`crates/miyudouane/examples/arretes_essai.rs`](../../crates/miyudouane/examples/arretes_essai.rs) — arrêtés d'essai signés par une clé jetable
