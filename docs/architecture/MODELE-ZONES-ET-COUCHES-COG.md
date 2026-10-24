# Modèle de zones et de couches — Miyukini COG

<!-- @id arch.cog.zones-couches -->
<!-- @role architecture-reference -->
<!-- @layer 3-8+kernel -->
<!-- @human Référence : 4 zones (Nationale, Franche, Fédération, International), Douanes, réseau intégré, couches 3 à 8 + Kernel -->
<!-- @do define_cog_zones_and_layers -->

- **Statut** : référence, 2026-10-24
- **Liens** : [audit](../securite/AUDIT-2026-10-23-auth-wan.md) · [lot 2](../securite/ARCHITECTURE-lot2-identite-signee.md) · [schéma](../schema-conceptuel-cog.mmd)

## 1. Principes

<!-- @id arch.cog.principes -->
<!-- @role architecture-principle -->
<!-- @do state_cog_security_principles -->

- Le COG **embarque sa propre infrastructure réseau** (LAN – FW – DMZ – FW – WAN/Internet).
- La séparation des zones est **imposée par le système**, jamais par le code applicatif.
- **Gradient de confiance** : chaque zone a un niveau BorderGuard.
- Chaque couche et chaque frontière a un **Core garant**.

## 2. Zones

<!-- @id arch.cog.zones -->
<!-- @role zone-model -->
<!-- @layer 3 -->
<!-- @do define_four_zones -->

- 🏛️ **Nationale** = LAN · `Trusted`
  - Cores, MiyukiniAdmin, données, clés, MAIA, Central
  - aucune passerelle réseau
- 🏪 **Franche** = DMZ · zone tampon
  - points d'accès, Douanes, files *quarantaine* (entrant) et *publication* (sortant)
  - aucun secret, aucune donnée nationale
- 🤝 **Fédération** = WAN privé · `Verified`
  - WireGuard / Noise entre COG reconnus
  - appareils de l'utilisateur (pairs avec passeport), Origin (face fédérale)
- 🌐 **International** = Internet · `Unknown`
  - visiteurs, services tiers, Origin (face publique)

```
                                            ┌─ 🔥 FW-FED ──▶ 🤝 FÉDÉRATION   Verified
🏛️ NATIONALE ◀── 🔥 FW-INT ──▶ 🏪 FRANCHE ──┤
   Trusted                        DMZ        └─ 🔥 FW-INTL ─▶ 🌐 INTERNATIONAL  Unknown
```

## 3. Invariants de flux

<!-- @id arch.cog.invariants-flux -->
<!-- @role security-invariant -->
<!-- @layer 3-4 -->
<!-- @do enforce_zone_flow_rules -->

- **INV-Z1** Nationale ⇄ Fédération / International : **interdit**
- **INV-Z2** **La Nationale tire** : la Franche n'ouvre jamais de connexion vers la Nationale
- **INV-Z3** Fédération ⇄ International : pas de transit
- **INV-Z4** Nationale sans route par défaut
- **INV-Z5** Franche sans secret ni donnée nationale
- Franche → Fédération : pairs reconnus, flux fédérés
- Franche → International : liste de destinations autorisées
- International → Franche : ports publics déclarés, **zéro par défaut**

## 4. Douanes (BorderGuard)

<!-- @id arch.cog.douanes -->
<!-- @role border-control -->
<!-- @layer 3-6 -->
<!-- @do control_zone_crossings -->

- **Intérieure** (FW-INT, Nationale ⇄ Franche)
  - rupture protocolaire
  - liste blanche de protocole `MSCE` (Sceau v1)
- **Fédérale** (FW-FED, Franche ⇄ Fédération)
  - pair WireGuard signé par une racine reconnue
  - passeport / visa, flux fédérés uniquement
- **Internationale** (FW-INTL, Franche ⇄ International)
  - IPS, ports déclarés, sorties en liste blanche, bannissement auto

**Rupture protocolaire**
- toute connexion est terminée dans la Franche
- réécrite en déclaration Sceau v1 → quarantaine → relevée par la Nationale
- réponse par la file de publication

**Dédouanement** (Douane intérieure)
- déclaration scellée (Sceau v1)
- type autorisé, taille bornée
- passeport ou visa valide
- décision StrongFather (+ TAMR si sensible)
- inscription au registre Merkle

## 5. Réseau intégré

<!-- @id arch.cog.reseau-integre -->
<!-- @role network-infrastructure -->
<!-- @layer 3 -->
<!-- @do build_embedded_zone_network -->

- **Réalisation**
  - 1 network namespace par zone et par pare-feu, reliés en veth
  - 1 seule carte physique, dans un espace de bordure (WireGuard → FW-FED, reste → FW-INTL)
  - adressage : Nationale `10.77.0.0/24` sans passerelle · Franche `10.77.1.0/24` · Fédération `10.78.0.0/16`
- **Pare-feu** : nftables, drop par défaut, règles générées depuis la constitution réseau
- **Constitution réseau** : topologie et règles scellées par la racine COG ; règle non signée = jamais appliquée
- **Dureté**
  1. namespaces + utilisateurs + nftables (prototype, sans toucher le réseau de l'hôte)
  2. Franche et pare-feu en micro-VM (COG OS)
  3. machines physiques séparées

## 6. IDS / IPS maison

<!-- @id arch.cog.ids-ips -->
<!-- @role intrusion-detection -->
<!-- @layer 3-7 -->
<!-- @do detect_and_block_intrusions -->

- Rust, **salle blanche** : aucun code Suricata / Snort (GPLv2 incompatible)
- Briques permissives : Rusticata, `aho-corasick`, `etherparse`, `nfq`
- Sous-ensemble de la syntaxe de règles Suricata / Snort
- Par point :
  - FW-INTL : **IPS** bloquant
  - FW-FED : IDS
  - FW-INT : **IPS liste blanche `MSCE`**
  - Franche, Nationale : IDS (mouvement latéral, tentative de sortie)
- Réponse : CaringNanny → WorrySentinel → bannissement / gel Franche / coupure FW-INT → TAMR alerte l'humain → registre Merkle

## 7. Cloisonnement noyau

<!-- @id arch.cog.cloisonnement-noyau -->
<!-- @role kernel-hardening -->
<!-- @layer kernel -->
<!-- @do isolate_zones_at_kernel_level -->

- network namespaces par zone
- 1 utilisateur système + 1 slice systemd par zone
- stockage et clés distincts par zone, scellés TPM
- Landlock, seccomp, AppArmor
- sockets de Douane avec `SO_PEERCRED`

## 8. Couches

<!-- @id arch.cog.couches -->
<!-- @role layer-model -->
<!-- @layer 3-8+kernel -->
<!-- @do map_layers_to_cores -->

- **8 · IA** — intentions, outils, limites ; MAIA en Nationale · *TAMR, StrongFather, MasterButler*
- **7 · Application** — services, permissions, visas, paquets signés · *StrongFather, MasterButler*
- **6 · Présentation** — Sceau v1, chiffrement au repos · *KindMother, WorrySentinel*
- **5 · Session** — passeport, preuve de possession, visa, Noise · *BorderGuard*
- **4 · Transport** — ports, débit, sockets locaux · *LogisticsSteward, BorderGuard*
- **3 · Réseau** — zones, pare-feu, IDS/IPS, WireGuard · *BorderGuard, WorrySentinel*
- **Kernel** — lockdown, modules signés, dm-verity, TPM, A/B signées · *WorrySentinel, EverBuddy*
- Couches 1–2 : matériel et pilotes (hors COG)

## 9. Identité

<!-- @id arch.cog.identite -->
<!-- @role identity-trust -->
<!-- @layer 5-6 -->
<!-- @do bind_identity_to_zones -->

- Racine COG hors ligne (USB) · `cog_id` = empreinte
- Passeport 30 j (clé d'appareil) · visa 15 min · preuve de possession
- Reconnaissance croisée des racines → `Verified` → accès Fédération
- Appareils de l'utilisateur = pairs Fédération → **aucun port ouvert sur Internet**

## 10. Origin hybride

<!-- @id arch.cog.origin-hybride -->
<!-- @role federation-node -->
<!-- @layer 7 -->
<!-- @do define_origin_dual_role -->

- **Face fédérale** : racine du Market, certificat éditeur, registre Merkle de référence, trackers signés
- **Face publique** : site, téléchargements non fédérés, documentation

## 11. Placement des services

<!-- @id arch.cog.placement-services -->
<!-- @role deployment-map -->
<!-- @layer 7 -->
<!-- @do place_services_in_zones -->

- **Nationale** : Cores, KindMother, MiyukiniAdmin, Central, Suite Jay, MiyuCloud (données), MAIA
- **Franche** : points d'accès TLS / Noise, Douanes, files, client Market, mises à jour, relais Webway
- **Franche locale** (proposé) : domotique Alicia, MQTT, LAN domestique sans Internet

## 12. Feuille de route

<!-- @id arch.cog.feuille-de-route -->
<!-- @role roadmap -->
<!-- @do plan_security_hardening_lots -->

- ✅ **Lot 1** fermeture par défaut
- 🔄 **Lot 2** identité signée (incrément 1 : `miyusceau`)
- ⬜ **Lot 3** Noise, Fédération WireGuard, appareils pairs
- ⬜ **Lot 4** réseau intégré, Douanes, IDS/IPS maison, constitution réseau
- ⬜ **Lot 5** gouvernance IA (couche 8)
- ⬜ **Lot 6** COG OS

## 13. Points ouverts

<!-- @id arch.cog.points-ouverts -->
<!-- @role open-questions -->
<!-- @do track_open_decisions -->

- Zone de la domotique (« Franche locale » ?)
- Licence : `LICENSE` propriétaire vs `Cargo.toml` `MIT OR Apache-2.0`
- Licences des jeux de règles IDS (ET Open, Snort Community)
- Prototype réseau isolé de l'hôte de production
