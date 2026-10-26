# maia-llm et mini modèle — à intégrer par l'installateur (`deploy/`)

<!-- @id config.maia.installateur -->
<!-- @role install-handoff -->
<!-- @layer 8 -->
<!-- @human Changements attendus dans deploy/ pour installer maia-llm, le mini modèle du canal officiel, et brancher MAIA et Alicia (COG-POSTE-100 A1–A4) -->
<!-- @do handoff_maia_llm_install -->

Préparé sans modifier `deploy/` (autre propriétaire). Fichiers fournis ici : `maia-llm.toml`,
`apparmor-miyukini-maia-llm`, `catalogue-modeles.toml`.

**État (2026-10-26) : intégré** aux installateurs `deploy/cog-vm/` et `deploy/recette/` (logique dans
`deploy/cog-vm/commun.sh` : `cle_maia`, `configurer_maia_llm`, `modele_ia`, unité `miyukini-maia-llm`) et
vérifié en VM (`tester.sh`, `scenarios.sh` section 13). Écarts à ce qui suit : pas de `ReadWritePaths` pour
`/var/lib/miyukini/maia-llm` (rien n'y est écrit, le profil le refuse) ; `PrivateDevices=yes` vient du
durcissement commun ; `MemoryDenyWriteExecute=yes` conservé (binaire CPU) ; Alicia n'étant pas un service de ces
installateurs, sa clé est épinglée seulement si l'exploitant dépose `/etc/miyukini/alicia-cle-voix.pub` ;
`[moteur_ia]` écrit dans `/etc/miyukini/alicia.toml` ; profil AppArmor installé tel quel (règles GPU inertes
pour le binaire CPU) ; modèle livré en paquet séparé (artefact `MMIA`, option `--modele` des installateurs).

## 1. Paquet

- Construire `maia-llm` **CPU** pour la VM : `cargo build -p maia-llm --release --no-default-features`
  (la fonctionnalité `cuda` est par défaut ; elle exige le toolkit CUDA et un GPU)
- Installer `/opt/miyukini/bin/maia-llm` (0755, root) et le lister au `MANIFESTE`
- Ne **jamais** mettre de poids dans le paquet : le modèle arrive par le canal officiel

## 2. Comptes et dossiers

- Compte système `miyukini-maia-llm` (sans shell, sans home)
- `/var/lib/miyukini/maia-llm` (0700, miyukini-maia-llm) : `maia-llm.toml` copié depuis
  `/etc/miyukini/maia-llm.toml` (root, 0644) et monté en lecture seule
- `/var/lib/miyukini/modeles` (0755, root) : dossier du canal (`modeles.dossier` de
  `canal-officiel.json`, installateur `MMIA`) ; fichiers 0644 root (maia-llm refuse un modèle
  inscriptible par le groupe ou les autres, ou appartenant à un autre compte que root/lui-même)

## 3. Configuration

- `/etc/miyukini/maia-llm.toml` : depuis `config/maia/maia-llm.toml`, en remplaçant
  - `À_REMPLACER_CLE_MAIA` : clé publique de service de MAIA (`/var/lib/miyukini/maia/keys/maia.seed`)
  - `À_REMPLACER_CLE_ALICIA` : clé publique d'Alicia (`cle_voix`)
  - `À_REMPLACER_ANCRE_MODELES` : clé publique hex du signataire des manifestes de modèles
    (même ancre que `modeles.ancres` de `canal-officiel.json`)
  - VM / GPU partagé : `gpu_layers = 0`
- `canal-officiel.json` de Central : champ `modeles` = `{"dossier": "/var/lib/miyukini/modeles",
  "ancres": ["<ancre>"]}` (A-007)
- `bridge.toml` de MAIA : **ne pas** renseigner `upstream_url` (vide par défaut ; de toute façon
  refusé par la politique `maia-inference-externe-refusee`)
- `alicia.toml` : `[moteur_ia]` `url = "http://127.0.0.1:11436"` (et `timeout_ms = 20000` en CPU)
- Politiques StrongFather : recopier `config/strongfather/maia.toml` et `alicia.toml` (nouvelles
  règles `maia-inference-externe-refusee`, `alicia-domotique-proposition-ia`)

## 4. Unité systemd (`deploy/cog-vm/commun.sh`, `generer_unites`)

```bash
ecrire_unite miyukini-maia-llm "maia-llm — moteur d'IA local llama.cpp (COG)" \
	miyukini-maia-llm "$VAR/maia-llm" "$BIN/maia-llm" "$VAR/maia-llm" "" "" \
	"Environment=MAIA_DATA_DIR=$VAR/maia-llm
Environment=RUST_LOG=maia_llm=info
BindReadOnlyPaths=$ETC/maia-llm.toml:$VAR/maia-llm/maia-llm.toml
ReadOnlyPaths=$VAR/modeles
MemoryMax=3G
# VM sans GPU : PrivateDevices=yes. Hôte CUDA : DeviceAllow=/dev/nvidiactl rw, /dev/nvidia0 rw, /dev/nvidia-uvm rw
PrivateDevices=yes" "miyukini-maia-llm"
```

- Dans l'unité de MAIA : `Environment=MAIA_LLM_URL=http://127.0.0.1:11436` (au lieu de
  `11435`, qui est le port de MAIA elle-même) et `After=/Wants=miyukini-maia-llm.service`
- Unité d'Alicia : `Wants=miyukini-maia-llm.service`
- `PORTS_COG_TCP` : ajouter `11436` (loopback seulement ; aucun flux sortant pour ce compte)
- Douane : aucun flux sortant pour `miyukini-maia-llm` (refus par défaut)

## 5. AppArmor

- Copier `config/maia/apparmor-miyukini-maia-llm` vers `/etc/apparmor.d/miyukini-maia-llm`
  (`apparmor_parser -Q` le valide) et ajouter `AppArmorProfile=miyukini-maia-llm`
- Machine sans GPU : les règles `/dev/nvidia*`, `libcuda*` peuvent être retirées
- `nvidia-smi` : exécution refusée (aucune règle `x` ; refus explicite) — la détection GPU de
  maia-llm passe par llama.cpp, celle de MAIA par `/proc/driver/nvidia/gpus/*/information`
- Profil de MAIA (`deploy/apparmor/miyukini-maia`) : ajouter
  `@{PROC}/driver/nvidia/gpus/ r,` et `@{PROC}/driver/nvidia/gpus/*/information r,`

## 6. Installation du modèle (recette et VM)

```bash
# Poste de signature (hors COG) — clé du canal, jamais sur le COG :
maia-llm sceller-modele --graine <cle> --modele qwen2.5-1.5b-instruct-q4_k_m.gguf \
  --licence Apache-2.0 --coupure "non publiée (estimée 2024)"
maia-llm emballer-modele --modele qwen2.5-1.5b-instruct-q4_k_m.gguf --ancre <ancre> \
  --sortie qwen2.5-1.5b-instruct-q4_k_m.mmia
# Sur le COG (canal officiel, ou à la main pour la recette) :
maia-llm installer-modele --dossier /var/lib/miyukini/modeles --ancre <ancre> \
  --composant qwen2.5-1.5b-instruct-q4_k_m --version 1 --artefact qwen2.5-1.5b-instruct-q4_k_m.mmia
```

- Refus si l'empreinte, la taille, la licence ou la signature diffèrent ; rien n'est écrit
- SHA-256 attendu : `6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e`

## 7. Contrôles de recette (`tester.sh`)

- `curl -fsS http://127.0.0.1:11436/maia/llm/health` : `"model_loaded":true`,
  `"licence":"Apache-2.0"`, `"gpu_layers":0` en VM
- `curl -fsS http://127.0.0.1:11435/maia/presentation` : `"nature":"intelligence artificielle"`
- Requête non signée sur `/v1/chat/completions` de maia-llm : 401/503
- Essai réel MAIA de bout en bout : `cargo test -p maia maia_repond_avec_le_vrai_modele -- --ignored`
  (variables dans `apps/miou-llm-bridge/src/essai_reel.rs`)

---

Date de mise a jour : 2026-10-26
