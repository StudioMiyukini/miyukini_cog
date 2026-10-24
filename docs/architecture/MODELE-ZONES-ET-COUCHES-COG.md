# Modèle de zones et de couches — Miyukini COG

<!-- @id arch.cog.zones-couches -->
<!-- @role architecture-reference -->
<!-- @layer 3-8+kernel -->
<!-- @human Référence : zone COG, Portail interne, zone Franche, BorderGuard, Fédération / Origin / International, couches 3 à 8 + Kernel -->
<!-- @do define_cog_zones_and_layers -->

- **Statut** : référence, 2026-10-24 (schéma de l'auteur, v2)
- **Liens** : [audit](../securite/AUDIT-2026-10-23-auth-wan.md) · [lot 2](../securite/ARCHITECTURE-lot2-identite-signee.md) · [schéma](../schema-conceptuel-cog.mmd)

## 1. Principes

<!-- @id arch.cog.principes -->
<!-- @role architecture-principle -->
<!-- @do state_cog_security_principles -->

- Le COG **embarque sa propre infrastructure réseau** (LAN – FW – DMZ – FW – extérieur).
- La séparation des zones est **imposée par le système**, jamais par le code applicatif.
- **Gradient de confiance** : chaque origine de trafic a un niveau d'identification.
- Chaque couche et chaque frontière a un **Core garant**.

## 2. Topologie

<!-- @id arch.cog.topologie -->
<!-- @role zone-model -->
<!-- @layer 3 -->
<!-- @do define_cog_topology -->

```
   🤝 Fédération ◀── certification globale ──▶ 🏛️ Origin          🌐 International (Internet)
          │ certification fédérale                │ certification globale     │ zero trust · liste blanche
          └──────────────────────┐                │              ┌────────────┘
╔══════════════════ COG ═════════▼════════════════▼══════════════▼═══════════════╗
║                    🛂 BorderGuard (pare-feu externe)                            ║
║                       │                          │                              ║
║                       ▼                          │ canal officiel               ║
║   🏪 Zone Franche (DMZ) — services exposables     │ (gouvernance Cores, services ║
║                       │                          │  et Market officiels, MAJ)   ║
║                       ▼                          │                              ║
║                🚪 Portail interne (pare-feu interne)                            ║
║                       │                          │                              ║
║                       ▼                          ▼                              ║
║   🏛️ Zone COG (nationale) — Kernel, Cores, Services, interfaces, stockage, DB    ║
╚═════════════════════════════════════════════════════════════════════════════════╝
```

## 3. Zones intérieures

<!-- @id arch.cog.zones -->
<!-- @role zone-model -->
<!-- @layer 3-7 -->
<!-- @do define_internal_zones -->

- 🏛️ **Zone COG (nationale)** = LAN
  - Kernel, Cores, Services, interfaces
  - stockage local, **DB principale**
  - aucune liaison avec l'extérieur hors Portail interne et canal officiel
- 🏪 **Zone Franche** = DMZ
  - services exposables : pages web, services web
  - **DB secondaire** (copies publiées uniquement)
  - **agents relais** des services internes : bureautique coopérative, explorateur web…
  - aucun secret, aucune donnée de la DB principale non publiée

## 4. Origines extérieures

<!-- @id arch.cog.origines-exterieures -->
<!-- @role trust-sources -->
<!-- @layer 3-5 -->
<!-- @do classify_external_origins -->

- 🤝 **Fédération** — COG reconnus, appareils de l'utilisateur
  - identification par **certification fédérale** (racines reconnues entre COG)
- 🏛️ **Origin** — autorité globale
  - identification par **certification globale**
  - certifie aussi la Fédération (Origin ⇄ Fédération)
  - source unique du canal officiel
- 🌐 **International** — Internet hors fédération
  - **zero trust**, liste blanche uniquement

## 5. Pare-feu

<!-- @id arch.cog.pare-feu -->
<!-- @role border-control -->
<!-- @layer 3-6 -->
<!-- @do control_zone_crossings -->

- 🛂 **BorderGuard** (pare-feu externe)
  - vérifie la certification de chaque origine (fédérale / globale / liste blanche)
  - ouvre vers la zone Franche, ou vers le canal officiel pour Origin uniquement
  - IPS, bannissement automatique, ports déclarés (zéro par défaut côté International)
- 🚪 **Portail interne** (pare-feu interne)
  - seul passage zone Franche ⇄ zone COG
  - rupture protocolaire, liste blanche de protocole `MSCE` (Sceau v1)
  - dédouanement de chaque objet

## 6. Invariants de flux

<!-- @id arch.cog.invariants-flux -->
<!-- @role security-invariant -->
<!-- @layer 3-4 -->
<!-- @do enforce_zone_flow_rules -->

- **INV-Z1** Zone COG ⇄ extérieur : interdit, **sauf canal officiel** (§7)
- **INV-Z2** **La zone COG tire** : la Franche n'ouvre jamais de connexion vers la zone COG
- **INV-Z3** Fédération ⇄ International : pas de transit par le COG
- **INV-Z4** Zone COG sans route par défaut
- **INV-Z5** Franche sans secret ; DB secondaire = copies publiées seulement
- **INV-Z6** International : rien n'entre hors liste blanche

## 7. Canal officiel

<!-- @id arch.cog.canal-officiel -->
<!-- @role trusted-update-channel -->
<!-- @layer 3-7 -->
<!-- @do deliver_official_signed_content -->

- **Contenu** : gouvernance des Cores, services officiels, Market officiel, mises à jour des Cores
- **Seul passage qui contourne la Franche** → règles strictes :
  - source unique : **Origin**, certification globale
  - **tiré par la zone COG**, jamais poussé par BorderGuard
  - sens unique : aucune donnée de la zone COG ne sort par ce canal
  - objets **scellés par Origin** (Sceau v1) + preuve d'inclusion au registre Merkle
  - vérification complète **avant** installation ; pas de retour à une version antérieure
  - aucun autre protocole ; tout écart = intrusion → WorrySentinel

## 8. Rupture protocolaire et dédouanement

<!-- @id arch.cog.dedouanement -->
<!-- @role protocol-break -->
<!-- @layer 5-6 -->
<!-- @do clear_objects_through_internal_portal -->

- Toute connexion extérieure est **terminée dans la Franche**
- Les agents relais réécrivent en **déclaration Sceau v1** → quarantaine
- La zone COG **relève** par le Portail interne, répond par la file de publication
- Chaque déclaration :
  - scellée, type autorisé, taille bornée
  - passeport ou visa valide
  - décision StrongFather (+ TAMR si sensible)
  - inscrite au registre Merkle

## 9. Réseau intégré

<!-- @id arch.cog.reseau-integre -->
<!-- @role network-infrastructure -->
<!-- @layer 3 -->
<!-- @do build_embedded_zone_network -->

- 1 network namespace par zone et par pare-feu, reliés en veth
- 1 seule carte physique, portée par BorderGuard
- Adressage indicatif : zone COG `10.77.0.0/24` sans passerelle · Franche `10.77.1.0/24` · Fédération `10.78.0.0/16`
- nftables, drop par défaut, règles générées depuis la **constitution réseau** scellée par la racine COG
- Dureté :
  1. namespaces + utilisateurs + nftables (prototype, sans toucher le réseau de l'hôte)
  2. Franche et pare-feu en micro-VM (COG OS)
  3. machines physiques séparées

## 10. IDS / IPS maison

<!-- @id arch.cog.ids-ips -->
<!-- @role intrusion-detection -->
<!-- @layer 3-7 -->
<!-- @do detect_and_block_intrusions -->

- Rust, **salle blanche** : aucun code Suricata / Snort (GPLv2 incompatible)
- Briques permissives : Rusticata, `aho-corasick`, `etherparse`, `nfq`
- Sous-ensemble de la syntaxe de règles Suricata / Snort
- BorderGuard : **IPS** bloquant
- Portail interne : **IPS liste blanche `MSCE`**
- Canal officiel : uniquement objets scellés Origin
- Franche, zone COG : IDS (mouvement latéral, tentative de sortie)
- Réponse : CaringNanny → WorrySentinel → bannissement / gel Franche / coupure Portail → TAMR → registre Merkle

## 11. Cloisonnement noyau

<!-- @id arch.cog.cloisonnement-noyau -->
<!-- @role kernel-hardening -->
<!-- @layer kernel -->
<!-- @do isolate_zones_at_kernel_level -->

- network namespaces par zone
- 1 utilisateur système + 1 slice systemd par zone
- stockage et clés distincts par zone, scellés TPM
- Landlock, seccomp, AppArmor
- sockets du Portail interne avec `SO_PEERCRED`

## 12. Couches

<!-- @id arch.cog.couches -->
<!-- @role layer-model -->
<!-- @layer 3-8+kernel -->
<!-- @do map_layers_to_cores -->

- **8 · IA** — intentions, outils, limites ; MAIA en zone COG · *TAMR, StrongFather, MasterButler*
- **7 · Application** — services, permissions, visas, paquets signés · *StrongFather, MasterButler*
- **6 · Présentation** — Sceau v1, chiffrement au repos · *KindMother, WorrySentinel*
- **5 · Session** — passeport, preuve de possession, visa, Noise · *BorderGuard*
- **4 · Transport** — ports, débit, sockets locaux · *LogisticsSteward, BorderGuard*
- **3 · Réseau** — zones, pare-feu, IDS/IPS, WireGuard · *BorderGuard, WorrySentinel*
- **Kernel** — lockdown, modules signés, dm-verity, TPM, A/B signées · *WorrySentinel, EverBuddy*
- Couches 1–2 : matériel et pilotes (hors COG)

## 13. Identité

<!-- @id arch.cog.identite -->
<!-- @role identity-trust -->
<!-- @layer 5-6 -->
<!-- @do bind_identity_to_zones -->

- Racine COG hors ligne (USB) · `cog_id` = empreinte
- Passeport 30 j (clé d'appareil) · visa 15 min · preuve de possession
- **Certification fédérale** = reconnaissance croisée des racines entre COG
- **Certification globale** = signature Origin (Market, canal officiel, Fédération)
- Appareils de l'utilisateur = pairs Fédération → aucun port ouvert côté International

## 14. Feuille de route

<!-- @id arch.cog.feuille-de-route -->
<!-- @role roadmap -->
<!-- @do plan_security_hardening_lots -->

- ✅ **Lot 1** fermeture par défaut
- 🔄 **Lot 2** identité signée (incrément 1 : `miyusceau`)
- ⬜ **Lot 3** Noise, Fédération WireGuard, appareils pairs
- ⬜ **Lot 4** réseau intégré, BorderGuard, Portail interne, canal officiel, IDS/IPS maison
- ⬜ **Lot 5** gouvernance IA (couche 8)
- ⬜ **Lot 6** COG OS

## 15. Points ouverts

<!-- @id arch.cog.points-ouverts -->
<!-- @role open-questions -->
<!-- @do track_open_decisions -->

- Domotique (Alicia, MQTT) : Franche ou « Franche locale » dédiée ?
- Agents relais : liste et périmètre exact de chaque agent
- DB secondaire : quelles données publiées, synchronisation tirée par la zone COG ?
- Licence : `LICENSE` propriétaire vs `Cargo.toml` `MIT OR Apache-2.0`
- Licences des jeux de règles IDS (ET Open, Snort Community)
- Prototype réseau isolé de l'hôte de production
