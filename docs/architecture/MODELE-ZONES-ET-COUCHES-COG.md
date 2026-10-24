# Modèle de zones et de couches — Miyukini COG

<!-- @id arch.cog.zones-couches -->
<!-- @role architecture-reference -->
<!-- @layer 3-8+kernel -->
<!-- @human Référence : zone COG, Portail interne, zone Franche, BorderGuard, Fédération / Origin / International, couches 3 à 8 + Kernel -->
<!-- @do define_cog_zones_and_layers -->

- **Statut** : référence, 2026-10-24 (schéma de l'auteur, v2)
- **Liens** : [audit](../securite/AUDIT-2026-10-23-auth-wan.md) · [lot 2](../securite/ARCHITECTURE-lot2-identite-signee.md) · [schéma](../schema-conceptuel-cog.mmd) · [service relayé](schema-service-relaye.mmd) · [menaces](../securite/MODELE-MENACES-COG.md) · [services hybrides](SERVICES-HYBRIDES-ET-PORTAIL.md) · [mémoire MAIA](MEMOIRE-MAIA.md)

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
║                 🛂 BorderGuard externe (instance dédiée)                        ║
║                       │                          │ contenu Origin              ║
║                       ▼                          ▼                              ║
║   🏪 Zone Franche (DMZ)                     📦 Dépôt officiel                    ║
║   espace isolé par service                  (Cores, services, Market, MAJ OS,   ║
║                       │                      modèles IA)                        ║
║                       ▼                          │                              ║
║          🚪 Portail interne (instance dédiée) ◀───┘ voie réservée               ║
║                       ▲  la zone COG relève, vérifie chez elle                  ║
║   🏛️ Zone COG (nationale) — segmentée par service — Kernel, Cores, Services, DB  ║
║   🏠 Franche locale (domotique, LAN uniquement) ── relevée via le Portail        ║
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
  - **un espace isolé par service** (namespace / micro-VM) et une DB secondaire par service
- 🏠 **Franche locale** = seconde DMZ, LAN domestique uniquement
  - domotique : Alicia (relais), MQTT, objets connectés du foyer
  - aucun accès Internet ni Fédération ; relevée par la zone COG via le Portail
  - objets connectés considérés hostiles ; actions physiques soumises à StrongFather

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

- **Une instance BorderGuard par frontière**, chacune dans son espace (puis micro-VM), privilèges distincts : la compromission d'une instance ne donne pas les autres
- 🛂 **BorderGuard externe** (pare-feu externe)
  - vérifie la certification de chaque origine (fédérale / globale / liste blanche)
  - ouvre vers la Franche ; dépose le contenu Origin dans le **dépôt officiel** (§7)
  - IPS, bannissement automatique, ports déclarés (zéro par défaut côté International)
- 🚪 **Portail interne** (pare-feu interne)
  - seul passage Franche / dépôt officiel ⇄ zone COG
  - rupture protocolaire, liste blanche de protocole `MSCE` (Sceau v1)
  - la **vérification finale** (signature, dédouanement) est faite **côté zone COG**, jamais par une instance exposée

## 6. Invariants de flux

<!-- @id arch.cog.invariants-flux -->
<!-- @role security-invariant -->
<!-- @layer 3-4 -->
<!-- @do enforce_zone_flow_rules -->

- **INV-Z1** Zone COG ⇄ extérieur : **interdit**, y compris pour le canal officiel (relevé via le Portail, §7)
- **INV-Z2** **La zone COG tire** : la Franche n'ouvre jamais de connexion vers la zone COG
- **INV-Z3** Fédération ⇄ International : pas de transit par le COG
- **INV-Z4** Zone COG sans route par défaut
- **INV-Z5** Franche sans secret ; DB secondaire = copies publiées seulement
- **INV-Z6** International : rien n'entre hors liste blanche
- **INV-Z7** Relais (Franche) ⇄ moteur (zone COG) : **uniquement par le Portail interne**, flux ouvert par la zone COG
- **INV-Z8** Zone COG **segmentée** : une sandbox par service, accès aux données uniquement via KindMother, une clé de DB par service
- **INV-Z9** Une instance BorderGuard par frontière ; vérification finale côté zone COG
- **INV-Z10** Aucun contenu non fiable rendu tel quel en zone COG : formats assainis, WebView sans réseau, CSP stricte
- **INV-Z11** Aucune destruction à distance : une sanction porte sur l'identité, jamais sur les données

## 7. Canal officiel

<!-- @id arch.cog.canal-officiel -->
<!-- @role trusted-update-channel -->
<!-- @layer 3-7 -->
<!-- @do deliver_official_signed_content -->

- **Contenu** : gouvernance des Cores, services officiels, Market officiel, mises à jour des Cores, **correctifs de l'OS** (miroir re-signé), **modèles IA**
- **Chemin** : Origin → BorderGuard externe → **dépôt officiel** → relevé par la zone COG via le Portail (voie réservée)
- Ne contourne plus la zone Franche par une route réseau : aucune route vers la zone COG (INV-Z1, INV-Z4)
- Règles :
  - source unique : **Origin**, certification globale
  - **signature à seuil 2 sur 2** : Origin + clé éditeur hors ligne
  - preuve d'inclusion au registre Merkle + **fraîcheur** de la tête (anti-gel)
  - **quarantaine de 48 h** avant installation, sauf correctif critique co-signé
  - signature vérifiée **en zone COG, avant tout décodage**
  - version strictement croissante (anti-rollback)
  - sens unique : aucune donnée de la zone COG ne sort par ce canal
  - modèles IA chargés en sandbox
  - tout écart = intrusion → WorrySentinel (IDS), blocage BorderGuard (IPS)

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

## 9. Services relayés (agents relais)

<!-- @id arch.cog.services-relayes -->
<!-- @role relay-pattern -->
<!-- @layer 5-7 -->
<!-- @human Un service se coupe en moteur (zone COG) et relais (Franche) reliés par une DB secondaire temps réel ; exemple JayOffice -->
<!-- @do relay_service_between_cogs -->

**Principe** (détail : [services hybrides et Portail](SERVICES-HYBRIDES-ET-PORTAIL.md))
- Le **moteur** du service et ses prérequis restent en zone COG, isolés
- Un **agent relais** en Franche porte la partie collaborative externe
- Une **DB secondaire** temps réel en Franche sert de tampon
- Deux COG collaborent par un **tunnel par service et par session** (relais ⇄ relais), jusqu'à l'interface utilisateur, sans exposer le moteur

**Composants (exemple JayOffice)**
- Zone COG : *Services JayOffice* (moteur + interface interne) ⇄ *DB principale* (sync temps réel)
- Franche : *Relais JayOffice* ⇄ *DB secondaire* (temps réel)
- Fédération : relais JayOffice du COG distant
- Utilisateur local : utilise le moteur directement, **hors ligne**

**Flux**
1. Le COG distant demande à **rejoindre la session** (Fédération → BorderGuard)
2. BorderGuard vérifie la certification fédérale et délivre un **visa** limité à la session (`audience = jayoffice:<session>`)
3. **Connexion après visa** : BorderGuard → Relais
4. Le relais écrit **à chaud et temporairement** dans la DB secondaire, qui en **vérifie la santé** (schéma, taille, type)
5. **Synchronisation** moteur ⇄ relais, après authentification et **protocole de vérification**
6. La DB secondaire émet une **demande de sauvegarde** (asynchrone) au Portail interne
7. Le Portail **sauvegarde dans la DB principale seulement si OK** (dédouanement)

**Règles**
- Synchronisation moteur ⇄ relais **par le Portail interne** uniquement, sur un flux **ouvert par la zone COG** (temps réel préservé, la Nationale tire)
- La DB secondaire ne contient **que** les données partagées dans la session (publication explicite)
- DB secondaire **purgée** en fin de session ; rien n'y persiste au-delà
- Rien n'entre en DB principale sans dédouanement : type, schéma, taille, signature, analyse des fichiers, décision StrongFather
- Un participant malveillant ne peut corrompre que la **copie de session**, jamais la DB principale
- Le relais ne détient ni secret ni identifiant du moteur ; compromis, il ne donne accès qu'à la session en cours
- Tunnel de session : Noise / WireGuard, clés éphémères, fermé à l'expiration du visa
- **Un relais par session**, isolé ; un espace par service en Franche
- La « vérification de santé » en Franche n'est qu'un filtre : **la validation qui compte est faite en zone COG** à la sauvegarde
- DB principale **versionnée** : toute sauvegarde est réversible

**Autres services relayables**
- Bureautique coopérative (JayOffice : Docs, Sheets, Slides, Réunion)
- Explorateur web : le relais navigue et assainit, la zone COG affiche
- Messagerie (JayMessage), partage MiyuCloud, agenda partagé (JayKoa)

## 10. Dimensionnement par contrat

<!-- @id arch.cog.dimensionnement-contrat -->
<!-- @role resource-governance -->
<!-- @layer 3-7+kernel -->
<!-- @human Les Cores dimensionnent la Franche, les portails et BorderGuard à partir des besoins maximaux déclarés par chaque service -->
<!-- @do size_franche_and_flows_from_contracts -->

**Principe**
- Chaque service déclare ses **besoins maximaux** dans son manifeste signé (Sceau v1)
- Le COG les connaît **dès l'installation**
- Les Cores en dérivent la taille de la Franche, les flux des portails et les règles de BorderGuard
- **Ce qui n'est pas déclaré n'existe pas** : refusé par défaut

**Contrat de service (dans le manifeste)**
- Ressources max en Franche : CPU, RAM, disque, nombre de sessions
- Flux : sens, protocole, port, type de déclaration, débit max, taille max des objets
- Pairs autorisés : Fédération, Origin, liste blanche International
- Relais : besoin d'un agent relais, DB secondaire (taille max)

**Rôle des Cores**
- **MasterButler** : enregistre les capacités et flux déclarés
- **LogisticsSteward** : calcule et attribue les ressources (taille de la Franche, quotas par service et par session)
- **CaringNanny** : lit les informations du noyau (ressources disponibles, consommation, pression) et ajuste **sous** les maxima déclarés
- **StrongFather** : valide le contrat à l'installation
- **BorderGuard** (IPS) : applique — paquets autorisés, débit, taille, par service
- **WorrySentinel** (IDS) : tout dépassement de contrat = anomalie → incident

**Ce que les Cores produisent**
- Franche : un slice systemd par service (cgroups CPU / RAM / IO), espace disque borné
- Portail interne : types de déclarations et débits autorisés par service
- BorderGuard : règles nftables par flux + mise en forme du débit (tc)
- IDS : contrat attendu par flux → détection d'écart immédiate

**Hiérarchie des règles**
- **Constitution réseau** (racine COG, hors ligne) : zones, invariants, plafonds globaux
- **Arrêtés** (clé d'émission, générés par les Cores) : règles par service, **dans les plafonds** de la constitution
- Arrêté hors constitution = refusé ; installation / désinstallation = nouvel arrêté inscrit au registre Merkle

## 11. Réseau intégré

<!-- @id arch.cog.reseau-integre -->
<!-- @role network-infrastructure -->
<!-- @layer 3 -->
<!-- @do build_embedded_zone_network -->

- 1 network namespace par zone et par pare-feu, reliés en veth
- 1 seule carte physique, portée par BorderGuard
- Adressage indicatif : zone COG `10.77.0.0/24` sans passerelle · Franche `10.77.1.0/24` · Fédération `10.78.0.0/16`
- nftables, drop par défaut, règles générées depuis la **constitution réseau** (racine COG) et les **arrêtés** des Cores (§10)
- Dureté :
  1. namespaces + utilisateurs + nftables (prototype, sans toucher le réseau de l'hôte)
  2. Franche et pare-feu en micro-VM (COG OS)
  3. machines physiques séparées

## 12. IDS / IPS maison

<!-- @id arch.cog.ids-ips -->
<!-- @role intrusion-detection -->
<!-- @layer 3-7 -->
<!-- @human WorrySentinel = IDS (détecte, ne modifie pas) ; BorderGuard = IPS (bloque aux frontières) -->
<!-- @do detect_and_block_intrusions -->

**Répartition des rôles**
- **WorrySentinel = IDS** — détecte et qualifie, **ne modifie rien** (INV-WS-4)
  - capteurs dans chaque zone : Franche, zone COG, BorderGuard, Portail interne
  - compare le trafic au **contrat** de chaque service (§10)
  - fixe le niveau de sécurité du COG
- **BorderGuard = IPS** — bloque, aux frontières uniquement
  - pare-feu externe : IPS bloquant (règles, anti-scan, anti-exploitation)
  - Portail interne : IPS en liste blanche `MSCE`
  - canal officiel : uniquement objets scellés Origin
  - applique les décisions de WorrySentinel (bannissement, gel, coupure)
- **CaringNanny** — santé et ressources (noyau), distinct de la sécurité

**Moteur**
- Rust, **salle blanche** : aucun code Suricata / Snort (GPLv2 incompatible)
- Briques permissives : Rusticata, `aho-corasick`, `etherparse`, `nfq`
- Sous-ensemble de la syntaxe de règles Suricata / Snort
- Une base de détection commune, deux usages : analyse (WorrySentinel), blocage en ligne (BorderGuard)

**Chaîne de réponse**
1. Capteur → **WorrySentinel** détecte, qualifie, élève le niveau de sécurité
2. **BorderGuard** bloque : bannissement / gel de la Franche / coupure du Portail
3. **TAMR** alerte l'humain pour les décisions lourdes
4. Inscription au **registre Merkle**

**Détection prioritaire**
- Franche, zone COG : mouvement latéral, tentative de sortie de la zone COG = incident critique
- Tout flux hors contrat (§10)

## 13. Cloisonnement noyau

<!-- @id arch.cog.cloisonnement-noyau -->
<!-- @role kernel-hardening -->
<!-- @layer kernel -->
<!-- @do isolate_zones_at_kernel_level -->

- network namespaces par zone
- 1 utilisateur système + 1 slice systemd par zone
- stockage et clés distincts par zone, scellés TPM
- Landlock, seccomp, AppArmor
- sockets du Portail interne avec `SO_PEERCRED`

## 14. Couches

<!-- @id arch.cog.couches -->
<!-- @role layer-model -->
<!-- @layer 3-8+kernel -->
<!-- @do map_layers_to_cores -->

- **8 · IA** — intentions, outils, limites ; MAIA en zone COG ; [mémoire stratifiée](MEMOIRE-MAIA.md) · *TAMR, StrongFather, MasterButler*
- **7 · Application** — services, permissions, visas, paquets signés · *StrongFather, MasterButler*
- **6 · Présentation** — Sceau v1, chiffrement au repos · *KindMother, WorrySentinel*
- **5 · Session** — passeport, preuve de possession, visa, Noise · *BorderGuard*
- **4 · Transport** — ports, débit, sockets locaux · *LogisticsSteward, BorderGuard*
- **3 · Réseau** — zones, pare-feu, IPS (BorderGuard), IDS (WorrySentinel), WireGuard · *BorderGuard, WorrySentinel*
- **Kernel** — lockdown, modules signés, dm-verity, TPM, A/B signées · *WorrySentinel, EverBuddy*
- Couches 1–2 : matériel et pilotes (hors COG)

## 15. Identité

<!-- @id arch.cog.identite -->
<!-- @role identity-trust -->
<!-- @layer 5-6 -->
<!-- @do bind_identity_to_zones -->

- Racine COG hors ligne (USB) · `cog_id` = empreinte
- Passeport 30 j (clé d'appareil) · visa 15 min · preuve de possession
- **Certification fédérale** = reconnaissance croisée des racines entre COG
- **Certification globale** = signature Origin (Market, canal officiel, Fédération)
- Appareils de l'utilisateur = pairs Fédération → aucun port ouvert côté International
- **Une clé par usage** (passeports, visas, arrêtés, journal), **dérivée de la racine en SLIP-10** (dérivation durcie)
- Racine sauvegardée en **SLIP-39 2 sur 3** (parts en mots, sur USB et papier) ; cérémonie de rotation guidée
- **Identifiant différent par COG visité** (pas de traçage entre COG)

## 16. Fédération : admission et sanctions

<!-- @id arch.cog.federation-admission -->
<!-- @role federation-governance -->
<!-- @layer 5-7 -->
<!-- @human Statuts candidat / invité / membre / blacklisté, confirmations par les membres, blacklist sur preuves et quorum -->
<!-- @do govern_federation_membership -->

**Statuts**
- **Candidat** : 0–1 confirmation · aucun accès fédéré
- **Invité** : ≥ 2 confirmations · sessions sur invitation d'un membre, ne peut pas confirmer
- **Membre** : ≥ 4 confirmations **continues pendant 7 jours** · accès complet, confirme et signale
- **Blacklisté** : identité révoquée définitivement

**Confirmations**
- Seuls les **membres** confirment ; **membres fondateurs** créés par Origin
- Candidature accompagnée d'une **preuve de calcul** légère (hashcash) contre la création en masse
- Confirmation signée (Sceau v1), datée, **renouvelée toutes les 24 h**
- Compteur des 7 jours remis à zéro sous 4 confirmations ; membre durablement sous 2 → invité
- Confirmateurs **indépendants** : racines distinctes, attestation TPM, nombre de confirmations limité par membre et par mois
- Confirmateurs **responsables** : réputation réduite si un COG confirmé est blacklisté

**Blacklist**
- Signalement avec **preuves cryptographiques** (déclarations signées par le COG mis en cause)
- **Quorum de 6 membres indépendants** qui vérifient les preuves (seuil > admission)
- Délai de contestation ; faux signalement prouvé = sanction de l'auteur
- Sanction = **révocation définitive du `cog_id`**, tous les pairs refusent
- Retour possible seulement par **réinitialisation d'identité** (nouvelle racine) → repart candidat
- **Aucune destruction à distance** (INV-Z11) : les données restent au propriétaire, souvent victime d'un piratage
- Auto-détection : WorrySentinel propose quarantaine et réinitialisation au propriétaire (TAMR)

**Registre fédéral**
- Statuts, confirmations et blacklists dans un **registre Merkle fédéral**
- Têtes échangées entre pairs, contresignées par Origin en témoin → éclipse et double discours détectables
- Révocations fédérées avec **fraîcheur maximale** ; au-delà, accès fédéré refusé
- Propagation par **commérage** (annonce puis récupération à la demande)

**Défense locale**
- **Note locale** de chaque pair (ban score), indépendante de la blacklist fédérale
- **Anti-éclipse** : pairs choisis dans des réseaux différents, pairs anciens conservés
- **Évolutions du protocole** activées seulement quand un seuil de membres les signale

## 17. Confiance et résilience

<!-- @id arch.cog.confiance-resilience -->
<!-- @role resilience -->
<!-- @layer 3-8+kernel -->
<!-- @do ensure_trust_and_resilience -->

- **Temps de confiance** : horloge monotone qui ne recule jamais ; rejet de toute heure antérieure à la **médiane des 11 dernières têtes** de registre vues (Median-Time-Past) ; horodatages signés par Origin
- **Sauvegarde hors site** : DB principale chiffrée par une clé hors ligne, publiée via la Franche vers un pair fédéré (flux déclaré)
- **Amorçage** : image signée, empreinte publiée hors bande (site, papier, QR), clé Origin intégrée à l'image
- **Gouvernance d'Origin** : à seuil (plusieurs détenteurs), plan de succession ; la Fédération fonctionne sans Origin
- **Flux sortants déclarés** : alertes TAMR, arrêtés vers BorderGuard, sauvegardes = types de publication du Portail
- **Surallocation** : LogisticsSteward refuse une installation qui dépasse la capacité, ou applique des priorités
- **TAMR sans fatigue** : peu de types sensibles, validations groupées, décisions mémorisées par règle
- **Noyau partagé** : le niveau 1 (namespaces) ne sert qu'au prototype ; protection réelle à partir des micro-VM ou d'une machine dédiée

## 18. Emprunts à Bitcoin

<!-- @id arch.cog.emprunts-bitcoin -->
<!-- @role design-rationale -->
<!-- @layer 5-7 -->
<!-- @human Briques éprouvées de Bitcoin reprises par COG, et ce qui est volontairement écarté -->
<!-- @do reuse_bitcoin_mechanisms -->

**Repris**
- **Arbre de Merkle + preuves SPV** → registre Merkle, preuves d'inclusion
- **« Don't trust, verify »** → chaque COG revalide signatures, statuts, blacklists
- **SLIP-10** (BIP32 pour Ed25519, dérivation durcie) → une clé par usage dérivée de la racine
- **SLIP-39** (Shamir normalisé en mots) → sauvegarde de la racine, 2 sur 3
- **Median-Time-Past** → temps de confiance
- **Ban score** → note locale des pairs, en plus de la blacklist fédérale
- **Anti-éclipse** (addrman) → pairs diversifiés par réseau d'origine, pairs anciens conservés
- **Graines de démarrage** → membres fondateurs et Origin intégrés à l'image
- **Commérage `inv` / `getdata`** → propagation des confirmations, signalements, têtes de registre
- **Soft fork à seuil** (BIP9) → évolutions du protocole fédéral activées par signalement des membres
- **Compilations reproductibles Guix** → EX-04
- **Multisig** → signature à seuil (canal officiel, gouvernance d'Origin)
- **Hashcash** → coût de calcul léger à la candidature (anti-création en masse)

**Écarté**
- Blockchain globale et minage (énergie, latence, attaque à 51 % sur un petit réseau)
- Monnaie et jetons
- Registre public mondial (fuite de métadonnées)
- Modèle sans permission : la Fédération reste un réseau de confiance par cooptation

**Leçon retenue**
- Bitcoin n'a aucun bannissement global → la blacklist fédérale exige **preuves + quorum**, et chaque COG garde sa note locale

## 19. Feuille de route

<!-- @id arch.cog.feuille-de-route -->
<!-- @role roadmap -->
<!-- @do plan_security_hardening_lots -->

- ✅ **Lot 1** fermeture par défaut
- 🔄 **Lot 2** identité signée (incrément 1 : `miyusceau`)
- ⬜ **Lot 3** Noise, Fédération WireGuard, appareils pairs, admission et blacklist fédérales
- ⬜ **Lot 4** réseau intégré, BorderGuard, Portail interne, canal officiel, services relayés, IDS/IPS maison
- ⬜ **Lot 5** gouvernance IA (couche 8)
- ⬜ **Lot 6** COG OS

## 20. Points ouverts

<!-- @id arch.cog.points-ouverts -->
<!-- @role open-questions -->
<!-- @do track_open_decisions -->

- Agents relais : protocole de vérification moteur ⇄ relais à spécifier
- DB secondaire : moteur de base temps réel et format des demandes de sauvegarde
- Licence : `LICENSE` propriétaire vs `Cargo.toml` `MIT OR Apache-2.0`
- Licences des jeux de règles IDS (ET Open, Snort Community)
- Prototype réseau isolé de l'hôte de production
