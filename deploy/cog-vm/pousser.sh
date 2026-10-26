#!/usr/bin/env bash
# Exécuté SUR L'HÔTE de développement : prépare un paquet à partir des binaires déjà
# construits (target/release), le copie dans la VM jetable cog-essai, puis lance
# l'installateur DANS la VM. Ne installe rien sur l'hôte.
# VM jetable cog-essai UNIQUEMENT : jamais vers un hôte réel (voir README.md).
#
#   deploy/cog-vm/pousser.sh [--sans-installation] [--modele FICHIER.mmia | --sans-modele]
#
#   --sans-installation  copie et vérifie seulement le paquet (et le banc) dans la VM
#   --modele F.mmia      artefact MMIA du mini modèle (A-026), livré À PART du paquet (≈ 1,1 Go) :
#                        copié dans /home/cog/cog-modele (seulement s'il diffère), puis installé par
#                        l'installateur via le canal (maia-llm installer-modele, ancre épinglée).
#                        Défaut : $COG_MODELE, sinon /var/tmp/mrc/modeles/<composant>.mmia s'il existe.
#   --sans-modele        n'envoie aucun artefact (modèle déjà installé dans la VM, ou aucun)
#
# Binaires : maia-llm est construit SANS CUDA pour la VM (pas de GPU) :
#   cargo build --release -p maia-llm --no-default-features
# Artefact MMIA (poste de signature, jamais dans le dépôt ni dans le paquet) :
#   maia-llm emballer-modele --modele <X.gguf> --ancre <ancre> --sortie <X.mmia>
# Outils d'essai (scenarios.sh) : outils/portail_essai, outils/alicia_proposition_ia (binaire du test
# d'intégration crates/miyualicia/tests/proposition_ia.rs : cargo test --release -p miyualicia
# --test proposition_ia --no-run ; chemin donné par COG_ALICIA_ESSAI ou trouvé dans target).
#
# Intégrité (B7) : MANIFESTE = SHA-256 de chaque fichier du paquet, lié au commit
# (en-tête), affiché ici avec sa propre empreinte, puis vérifié DANS la VM
# (empreinte du manifeste reçu + sha256sum --check) avant toute installation.
#
# Variables : COG_VM (défaut ~/vm/cog-essai), CARGO_TARGET_DIR (défaut <dépôt>/target).
set -euo pipefail

ICI=$(cd "$(dirname "$0")" && pwd)
RACINE=$(cd "$ICI/../.." && pwd)
VM=${COG_VM:-$HOME/vm/cog-essai}
REL=${CARGO_TARGET_DIR:-$RACINE/target}/release
DEST=/home/cog/cog-paquet
SANS_INSTALLATION=0
MODELE=${COG_MODELE:-}
SANS_MODELE=0
MODELE_DEST=/home/cog/cog-modele

log() { printf '[pousser] %s\n' "$*"; }
die() { printf '[pousser] ERREUR: %s\n' "$*" >&2; exit 1; }

while (($#)); do
	case $1 in
		--sans-installation) SANS_INSTALLATION=1 ;;
		--modele) MODELE=${2:?--modele : fichier attendu}; shift ;;
		--sans-modele) SANS_MODELE=1 ;;
		-h | --help) sed -n '2,31p' "$0"; exit 0 ;;
		*) die "option inconnue : $1" ;;
	esac
	shift
done

[[ -x $VM/ssh-vm.sh ]] || die "accès VM introuvable : $VM/ssh-vm.sh"
SSH_OPTS=(-q -o BatchMode=yes -o StrictHostKeyChecking=no -o "UserKnownHostsFile=$VM/known_hosts" -i "$VM/cle_vm" -P 2222)
ssh_vm() { "$VM/ssh-vm.sh" "$@"; }

# ------------------------------------------------------------------ vérifications locales

BINAIRES=(kindmother-server maia maia-llm miyucloud-server cog-web-portal miyudouane portail-sas portail-entree portail-sortie)
for b in "${BINAIRES[@]}"; do
	[[ -x $REL/$b ]] || die "binaire manquant : $REL/$b (cargo build --release -p <paquet>)"
done
# maia-llm : binaire CPU exigé (la VM n'a pas de GPU ; un binaire CUDA y échouerait au chargement).
if ldd "$REL/maia-llm" 2>/dev/null | grep -qE 'libcuda|libcudart|libcublas'; then
	die "maia-llm lié à CUDA : construire sans CUDA (cargo build --release -p maia-llm --no-default-features)"
fi
# Essai d'Alicia avec le vrai modèle (scenarios.sh, section 13) : binaire du test d'intégration.
ALICIA_ESSAI=${COG_ALICIA_ESSAI:-$(ls -t "$REL"/deps/proposition_ia-* 2>/dev/null | grep -v '\.d$' | head -n 1)}
[[ -x $ALICIA_ESSAI ]] || die "essai d'Alicia manquant (cargo test --release -p miyualicia --test proposition_ia --no-run)"
if ((SANS_MODELE)); then
	MODELE=""
elif [[ -z $MODELE && -f /var/tmp/mrc/modeles/qwen2.5-1.5b-instruct-q4_k_m.mmia ]]; then
	MODELE=/var/tmp/mrc/modeles/qwen2.5-1.5b-instruct-q4_k_m.mmia
fi
[[ -z $MODELE || -f $MODELE ]] || die "artefact MMIA introuvable : $MODELE"
[[ -x $REL/examples/arretes_essai ]] || die "exemple manquant : $REL/examples/arretes_essai"
# Outil d'essai du Portail scindé (Cores, façade, moteur ; clés jetables) : scenarios.sh, section 11 bis.
[[ -x $REL/examples/portail_essai ]] || die "exemple manquant : $REL/examples/portail_essai (cargo build --release -p miyuportail --example portail_essai)"

# ------------------------------------------------------------------ paquet (hors dépôt)

STAGE=$(mktemp -d "${TMPDIR:-/tmp}/cog-paquet.XXXXXX")
trap 'rm -rf "$STAGE"' EXIT
mkdir -p "$STAGE/bin" "$STAGE/config" "$STAGE/static" "$STAGE/apparmor" "$STAGE/banc/tests" "$STAGE/banc/cible/examples" "$STAGE/outils"

for b in "${BINAIRES[@]}"; do
	cp "$REL/$b" "$STAGE/bin/$b"
done
cp "$RACINE/bridge.toml" "$STAGE/config/bridge.toml"
cp -r "$RACINE/config/strongfather" "$STAGE/config/strongfather"
mkdir -p "$STAGE/config/maia"
cp "$RACINE/config/maia/maia-llm.toml" "$STAGE/config/maia/maia-llm.toml"
cp -r "$RACINE/apps/miyucloud/static" "$STAGE/static/static"
cp -r "$RACINE/apps/miyucloud/static-dist" "$STAGE/static/static-dist"
cp "$ICI/installer.sh" "$ICI/tester.sh" "$ICI/commun.sh" "$STAGE/"
cp "$RACINE"/deploy/apparmor/miyukini-* "$STAGE/apparmor/"
cp "$RACINE/config/maia/apparmor-miyukini-maia-llm" "$STAGE/apparmor/miyukini-maia-llm"
cp -r "$RACINE/tests/intrusion-lot3" "$STAGE/banc/tests/intrusion-lot3"
cp "$REL/miyudouane" "$STAGE/banc/cible/miyudouane"
cp "$REL/examples/arretes_essai" "$STAGE/banc/cible/examples/arretes_essai"
cp "$REL/examples/portail_essai" "$STAGE/outils/portail_essai"
cp "$ALICIA_ESSAI" "$STAGE/outils/alicia_proposition_ia"
# Chemin des politiques d'Alicia compilé dans l'essai (env!(CARGO_MANIFEST_DIR)) : scenarios.sh y
# présente, dans un espace de montage privé, la politique INSTALLÉE (/etc/miyukini/strongfather).
printf '%s\n' "$RACINE/crates/miyualicia/../../config/strongfather/alicia.toml" >"$STAGE/outils/alicia_proposition_ia.politique"
chmod 0755 "$STAGE/installer.sh" "$STAGE/tester.sh"
find "$STAGE/banc" -name '*.sh' -exec chmod 0755 {} +

log "paquet préparé : $(du -sh "$STAGE" | cut -f1)"

# ------------------------------------------------------------------ manifeste (B7)

COMMIT=$(git -C "$RACINE" rev-parse HEAD 2>/dev/null) || die "commit introuvable (dépôt git requis)"
MODIFS=$(git -C "$RACINE" status --porcelain --untracked-files=no 2>/dev/null | wc -l)
if ((MODIFS > 0)); then
	ARBRE="modifié ($MODIFS fichier(s) suivis non commités : binaires NON reproductibles depuis le commit)"
else
	ARBRE="propre"
fi
{
	printf '# paquet cog-vm (VM jetable cog-essai uniquement)\n'
	printf '# commit %s\n' "$COMMIT"
	printf '# arbre %s\n' "$ARBRE"
	printf '# prepare_le %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
	(cd "$STAGE" && find . -type f ! -path ./MANIFESTE -print0 | LC_ALL=C sort -z | xargs -0 sha256sum)
} >"$STAGE/MANIFESTE"
EMPREINTE=$(sha256sum "$STAGE/MANIFESTE" | cut -d' ' -f1)
log "commit $COMMIT ; arbre $ARBRE"
log "binaires envoyés :"
grep '  \./bin/' "$STAGE/MANIFESTE" | sed 's/^/[pousser]   /'
log "MANIFESTE : $(grep -vc '^#' "$STAGE/MANIFESTE") fichiers, sha256 $EMPREINTE"

# ------------------------------------------------------------------ envoi

log "envoi vers la VM : $DEST"
ssh_vm "rm -rf '$DEST' && mkdir -p '$DEST'"
scp -r "${SSH_OPTS[@]}" "$STAGE/." "cog@127.0.0.1:$DEST/"

# Vérification DANS la VM : empreinte du manifeste reçu, puis chaque fichier.
RECU=$(ssh_vm "sha256sum '$DEST/MANIFESTE' | cut -d' ' -f1")
[[ $RECU == "$EMPREINTE" ]] || die "MANIFESTE reçu dans la VM ($RECU) ≠ manifeste envoyé ($EMPREINTE)"
ssh_vm "cd '$DEST' && grep -v '^#' MANIFESTE | sha256sum --check --strict --quiet -" ||
	die "vérification du paquet dans la VM : SHA-256 différent du MANIFESTE"
log "paquet vérifié dans la VM (manifeste $EMPREINTE)"

if ((SANS_INSTALLATION)); then
	log "paquet déposé ; installation ignorée (--sans-installation)"
	exit 0
fi

# ------------------------------------------------------------------ installation dans la VM

# Artefact MMIA (paquet modèle séparé) : envoyé seulement s'il n'est pas déjà dans la VM.
OPTION_MODELE=""
if [[ -n $MODELE ]]; then
	NOM_MODELE=$(basename "$MODELE")
	SHA_MODELE=$(sha256sum "$MODELE" | cut -d' ' -f1)
	if [[ $(ssh_vm "sha256sum '$MODELE_DEST/$NOM_MODELE' 2>/dev/null | cut -d' ' -f1") != "$SHA_MODELE" ]]; then
		log "envoi de l'artefact MMIA $NOM_MODELE ($(du -h "$MODELE" | cut -f1), sha256 $SHA_MODELE)"
		ssh_vm "mkdir -p '$MODELE_DEST'"
		scp "${SSH_OPTS[@]}" "$MODELE" "cog@127.0.0.1:$MODELE_DEST/$NOM_MODELE"
		[[ $(ssh_vm "sha256sum '$MODELE_DEST/$NOM_MODELE' | cut -d' ' -f1") == "$SHA_MODELE" ]] ||
			die "artefact MMIA altéré pendant l'envoi"
	else
		log "artefact MMIA $NOM_MODELE déjà dans la VM (sha256 $SHA_MODELE)"
	fi
	OPTION_MODELE="--modele '$MODELE_DEST/$NOM_MODELE'"
fi

log "installation dans la VM"
ssh_vm "sudo bash '$DEST/installer.sh' --vm-jetable --bundle '$DEST' --avec-banc --manifeste-sha256 '$EMPREINTE' $OPTION_MODELE"
log "terminé"
