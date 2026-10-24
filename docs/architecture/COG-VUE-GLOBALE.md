# Miyukini COG — vue globale consolidée

<!-- @id arch.cog.vue-globale -->
<!-- @role architecture-consolidation -->
<!-- @layer 0-9+zones -->
<!-- @human Consolidation de toute la documentation COG (≈ 900 fichiers) : inventaire, flux, échelles de confiance, contradictions à trancher, spécification du grand schéma global -->
<!-- @do consolidate_cog_documentation -->

- **Statut** : v1, 2026-10-24 — base du grand schéma global
- **Sources** : `kernel/`, `cores/` (10 Cores), `admin/`, `contrats/`, `tools/` (49 kits), `services/`, `miyukini-webway-system/`, `architecture/`, `securite/`, code (`apps/`, `crates/`)
- **Documents cibles** : [zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) · [services hybrides](SERVICES-HYBRIDES-ET-PORTAIL.md) · [mémoire MAIA](MEMOIRE-MAIA.md) · [IA fédérée](IA-FEDEREE.md) · [modèles locaux](MODELES-IA-PETITES-CONFIGS.md) · [lot 2](../securite/ARCHITECTURE-lot2-identite-signee.md) · [menaces](../securite/MODELE-MENACES-COG.md)

## 1. Spécification du grand schéma

<!-- @id arch.vue.schema-spec -->
<!-- @role diagram-specification -->
<!-- @do specify_global_diagram -->

Un schéma global en **6 vues** superposables (une page chacune, même légende) :

- **Fichiers** : [V1 territoire](schema-global/V1-territoire.mmd) · [V2 gouvernance](schema-global/V2-gouvernance.mmd) · [V3 services](schema-global/V3-services.mmd) · [V4 confiance](schema-global/V4-confiance.mmd) · [V5 flux](schema-global/V5-flux.mmd) · [V6 cycle de vie](schema-global/V6-cycle-de-vie.mmd)
- **FigJam** : [COG — schéma global](https://www.figma.com/board/Nx7DXwWUy2jwetLGkxB0Jn) (6 vues, vérifiées le 2026-10-24)
  - source de vérité : les `.mmd` ; le FigJam est un rendu (sans emojis, libellés raccourcis)
  - V5 FigJam : scénarios enchaînés sans séparateur, étapes internes fusionnées dans les messages

- **V1 · Territoire** — zones et frontières
  - nœuds : International, Fédération, Origin, Douane externe, Franche (façades, DB secondaire, quarantaine, publication), Dépôt officiel, Portail interne, zone COG, Franche locale
  - liens : certification fédérale / globale / zero trust, voies du Portail, « la zone COG tire »
  - lecture : extérieur → Douane → Franche → Portail → zone COG ; la façade lit la publication écrite par le Portail
- **V2 · Gouvernance** — strates et Cores
  - nœuds : Kernel (5 modules), 10 Cores, MiyukiniAdmin, Toolkits, Services
  - liens : intention → décision → persistance, contraintes de sécurité, signaux d'observation
- **V3 · Services** — catalogue placé par zone et classe H0–H3
  - nœuds : moteurs (zone COG), façades (Franche), Central, mobile
  - liens : traité moteur ⇄ façade via Portail, dépendances
- **V4 · Confiance** — identité et fédération
  - nœuds : racine USB, clés dérivées, passeport, visa, registres Merkle, statuts fédéraux
  - liens : signe, dérive, émet, confirme, révoque
- **V5 · Flux** — parcours de bout en bout
  - dessinés : accès distant / session relayée, mise à jour officielle, incident IDS/IPS
  - à dessiner : installation Market, requête IA fédérée
- **V6 · Cycle de vie** — démarrage EIP, installation par contrat, consolidation mémoire, sanction fédérale
- **Légende commune**
  - couleurs : zone COG vert · Franche orange · Fédération bleu · International gris · pare-feu rouge · Cores violet · clés et humain jaune
  - traits : plein = flux de données · pointillé = gouvernance · épais = signature
  - formes : rectangle = composant · cylindre = stockage · hexagone = frontière · cercle = acteur externe · losange = décision

## 2. Strates (numérotation unifiée proposée)

<!-- @id arch.vue.strates -->
<!-- @role layer-inventory -->
<!-- @do list_strata -->

- **K · Kernel** — socle neutre : `config`, `id`, `time`, `log`, `lifecycle` ; sens unique Produits → Cores → Kernel
- **C · Cores** — gouvernent, n'exécutent pas (10 Cores, §3)
- **T · Toolkits** — 57 crates, 49 documentés, catalogués par MasterButler
- **S · Services / Opérateurs** — émettent des intentions vers BondingBrother
- **A · MiyukiniAdmin** — opérateur souverain, accès aux Cores via BondingBrother uniquement
- **H · TAMR** — gouvernance humaine, transversale
- **Lois d'autonomie** LOI-1 à LOI-6 : pas de dépendance externe, isolement normal, état local souverain, pas de temps global, coût proportionnel au matériel, fédération explicite
- Correspondance avec le modèle cible : K = Kernel · C/T = couches 4–7 gouvernées · S = couche 7 · IA = couche 8 · zones = couches 3–5

## 3. Cores

<!-- @id arch.vue.cores -->
<!-- @role core-inventory -->
<!-- @do summarize_cores -->

- **StrongFather** — décide (ALLOW / DENY) ; n'exécute ni ne persiste (INV-SF-1/2/3)
- **KindMother** — persiste (instances Mère / Fille / Éphémère, hors ligne d'abord) ; ne décide jamais ; seul validateur des WriteIntent
- **BondingBrother** — passerelle : traduit, filtre, transmet ; applique les règles de BorderGuard (INV-BG-7) ; tampon hors ligne
- **BorderGuard** — **définit** frontières et niveaux de confiance (TRUSTED, VERIFIED, UNKNOWN, HOSTILE) ; ne filtre pas lui-même (INV-BG-1)
- **WorrySentinel** — gouverne la sécurité : niveaux 0–4, états T0–T4 ; ne lance aucune vérification (INV-WS-2)
- **CaringNanny** — observe l'état, ne modifie rien ; signaux vers WorrySentinel
- **MasterButler** — registre des capacités et permissions, catalogue des toolkits
- **LogisticsSteward** — arbitre ressources et quotas ; le Kernel exécute
- **EverBuddy** — cycle de vie et évolution ; historique immuable
- **TAMR** — cadre de l'intervention humaine (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) ; traçabilité absolue

## 4. Services

<!-- @id arch.vue.services -->
<!-- @role service-inventory -->
<!-- @do place_services -->

- **H0 · zone COG** : Central (cœur), Miou, JayKoa, JayKonta, MiyukiniWatch, MiyukiniAdmin, MAIA, Whisper, MIPOWER, JayEcole (projet)
- **H1 · publication** : site / page publique, partage cloud en lecture
- **H2 · collaboratif** : Suite Jay Bureau (Docs, Sheets, Slides, Formulaire, Réunion, Club, Message), Jay1Tribu, JayManga, JayRDV, JayFaim, JayShop, MiyukiniBB, MiyuCloud / JayCloud, Web Portal, accès mobile, CentralRemote
- **H3 · actif** : Jay Mail (SMTP), MiyukiniSales (envoi de devis), Alicia (domotique, Franche locale), explorateur web, Origin
- **Central** : lance chaque service en processus fils (`MIYUKINI_SERVICE_ID`, `KINDMOTHER_LISTEN_ADDR`), catalogue `OFFICIAL_CATALOG` + Market
- **Retirés** : MGE, Allumina, Sodomight, MiyuClicker, Survivor (2026-10) · JayXpose, JayFestival (2026-04)

## 5. Fédération et Origin

<!-- @id arch.vue.federation -->
<!-- @role federation-inventory -->
<!-- @do map_federation_actors -->

- **Cible** : COG souverains (candidat → invité → membre → blacklisté), Origin hybride (face fédérale : certification globale, Market officiel, registre Merkle ; face publique : vitrine)
- **Existant MWS** : Origin (relay 7000, tracker 21000, web, admin), Relays (vérification A/B/C, permis), Trackers (« douaniers », pools par version), types de COG (STABLE, SPECIAL, TERMINAL, LONE)
- **Correspondance proposée** :
  - Relay → nœud de transport fédéral (plus d'autorité de vérification)
  - Tracker → annuaire fédéral alimenté par le registre Merkle
  - Permis de circulation → visa fédéral (Sceau v1)
  - « Passeport » MWS (descripteur de COG) → **déclaration de COG** (évite la collision avec le passeport personne + appareil)
  - Phase A (clé cachée) → signature de la racine ; Phase B (blocs MSCM) → empreintes de paquets ; Phase C → attestation de santé signée (conservée)
  - `cog_id` UUID → empreinte de la racine
  - COG LONE → COG sans Fédération (zone COG seule)

## 6. Flux de référence

<!-- @id arch.vue.flux -->
<!-- @role flow-inventory -->
<!-- @do list_reference_flows -->

- Opérateur → BondingBrother → StrongFather (décision) → BondingBrother → KindMother (WriteIntent : créée → validée → appliquée → archivée)
- WorrySentinel → StrongFather / BorderGuard / LogisticsSteward (contraintes) ; CaringNanny → WorrySentinel (signaux)
- Kernel (état) → LogisticsSteward (arbitrage) → StrongFather (validation) → Kernel (exécution)
- Démarrage : Kernel → MiyukiniAdmin → EIP → verrou StrongFather (Vierge / Initialisé / Compromis)
- Accès distant : appareil (Fédération) → BorderGuard → façade Franche → quarantaine → zone COG relève
- Session relayée : COG distant → BorderGuard → relais → DB secondaire → demande de sauvegarde → Portail → DB principale
- Canal officiel : Origin → BorderGuard → dépôt officiel → zone COG relève et vérifie (2 sur 2, 48 h)
- IA fédérée : façade de recherche → Fiches → quarantaine / CDR → base étrangère → MAIA-Lecteur
- Incident : capteur → WorrySentinel → blocage aux frontières → TAMR → registre Merkle

## 7. Échelles de confiance — correspondance

<!-- @id arch.vue.echelles-confiance -->
<!-- @role trust-mapping -->
<!-- @do map_trust_scales -->

- **Zone COG** = TRUSTED · **membre fédéral** = VERIFIED · **invité** = VERIFIED restreint · **International** = UNKNOWN · **blacklisté** = HOSTILE
- **WorrySentinel T0–T4** = état de confiance du COG lui-même (T3 : décisions critiques soumises à TAMR · T4 : plus aucune décision opérationnelle)
- **Niveaux 0–4** = niveau de sécurité appliqué (pilote la sévérité des frontières)
- **États d'environnement** (Vierge, Initialisé, Compromis, Vierge avec mémoire de corruption) = cycle de vie de l'installation

## 8. Décisions (validées le 2026-10-24 : recommandations retenues)

<!-- @id arch.vue.decisions -->
<!-- @role open-decisions -->
<!-- @do list_contradictions_to_resolve -->

- **D1 · Cores et exécution** (contradiction avec INV-BG-1, INV-WS-2)
  - **retenu** : les Cores gouvernent, des **opérateurs exécutent** sous leur autorité
    - **Douane** (pare-feu + IPS) : règles définies par BorderGuard, appliquées via BondingBrother
    - **Vigies** (capteurs IDS) : détectent ; signaux propagés par CaringNanny ; WorrySentinel fixe le niveau
- **D2 · « Validation TAMR »** → intervention **APPROVAL** encadrée par TAMR, tranchée par StrongFather (INV-TAMR-5)
- **D3 · MWS** : auto-destruction après blacklist → **supprimée** (révocation d'identité) ; confiance verticale → reconnaissance entre COG ; renommages du §5
- **D4 · NET-SEC et synchronisation BondingBrother** : plus de reconnexion directe → passage par le Portail, la zone COG tire
- **D5 · KindMother** : instance **Mère** = DB principale (zone COG) ; **Fille / Éphémère** = DB secondaire de session (Franche)
- **D6 · Numérotation des strates** : adopter K / C / T / S / A / H (§2)
- **D7 · MiyukiniAdmin** : en zone COG uniquement, jamais exposé (son serveur HTTP local derrière le Portail)
- **D8 · Mobile / Terminal** : pair de la Fédération (passeport d'appareil), pas COG enfant
- **D9 · Mesure des ressources** : ni LogisticsSteward ni CaringNanny ne mesurent → le **Kernel** mesure, CaringNanny observe, LogisticsSteward arbitre

## 9. Dette documentaire et technique

<!-- @id arch.vue.dette -->
<!-- @role documentation-debt -->
<!-- @do list_documentation_debt -->

- **Encodage** : ✅ réparé le 2026-10-24 — 565 fichiers de `docs/` et 2 README (UTF-8 relu en cp1252) ; `.mip/` non touché (registres de certification générés)
- **Retraits incomplets** : docs texte ✅ (README, `doc_for_website`, `DEPRECATED.md`, SKILL, présentation marquée historique) ; reste la suppression des dossiers et du code non compilé — liste dans `docs/services/DEPRECATED.md`
- **Services retirés encore cités** : JayXpose, JayFestival (≈ 20 dossiers)
- **Code mort** : modules non déclarés dans `apps/central/src/services/`, `apps/miyuclicker`
- **Hors workspace** : `lifegame-*`, `crates/jayclub` (vide), `kindmother-db-adapter`, `apps/ui-builder`
- **Sans doc** : Suite Jay Bureau, Whisper, maia-llm, TEAM, MIPOWER, `miyusceau` dans `tools/`, design system UI
- **Contradictions** : rôle de JayKoa (site web), statut JayRDV, MiyuCloud vs JayCloud, nombre de services et de Cores annoncés
- **Stubs de sécurité** : `miyuantispam` (limitation toujours « autorisée »), `miyuwebway_tracker`, `miyuvalidate`
- **Index** : ✅ `_index.md` racine aligné sur D6 ; [index des Cores](../cores/_index.md) créé
