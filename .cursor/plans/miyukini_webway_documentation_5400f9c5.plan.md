---
name: Miyukini Webway documentation
overview: "Ecrire la documentation complete du Miyukini Webway System : consolider les 15+ documents existants, combler les sections \"a creer\" (systemes passifs/actifs Tracker, norme de declaration securisee, contrats), et documenter le relay custom (architecture, protocole, securite, deploiement Oracle Cloud)."
todos:
  - id: phase1-master
    content: "Phase 1 : Document maitre consolide (Miyukini Webway System Complet.md)"
    status: in_progress
  - id: phase2-relay-arch
    content: "Phase 2a : Documentation architecture relay custom (Miyukini Webway Relay.md)"
    status: pending
  - id: phase2-relay-proto
    content: "Phase 2b : Specification protocole relay (Miyukini Webway Relay Protocol.md)"
    status: pending
  - id: phase3-norme
    content: "Phase 3a : Combler section 5.3 norme de declaration securisee dans MWS principal"
    status: pending
  - id: phase3-passif
    content: "Phase 3b : Combler section 7.1 systemes passifs Trackers dans MWS principal"
    status: pending
  - id: phase3-actif
    content: "Phase 3c : Combler section 7.2 systemes actifs Trackers dans MWS principal"
    status: pending
  - id: phase3-normes-standards
    content: "Phase 3d : Finaliser schemas, bindings, matrice dans Normes et Standards"
    status: pending
  - id: phase3-outils
    content: "Phase 3e : Formaliser contrats outils + outils relay dans Outils et Operateurs"
    status: pending
  - id: phase4-oracle
    content: "Phase 4a : Completer guide Oracle Cloud (Rust, systemd, firewall, monitoring)"
    status: completed
  - id: phase4-deploy
    content: "Phase 4b : Guide deploiement relay complet (Webway Relay Deployment Guide.md)"
    status: completed
  - id: phase5-contrats
    content: "Phase 5 : Contrats systemes passifs et actifs Tracker (2 fichiers)"
    status: completed
  - id: phase6-glossaire
    content: "Phase 6 : Mise a jour glossaire + references croisees dans tous les docs MWS"
    status: completed
isProject: false
---

# Documentation complete du Miyukini Webway System

## Etat des lieux

### Documentation existante (15+ fichiers)

- 3 references conceptuelles : MWS principal, Normes et Standards, Outils et Operateurs
- 10 fichiers docs/tools/ : Participant + Tracker (fondatrice, reference, implementation, governance, index)
- 2 crates Rust : `miyuwebway_participant` + `miyuwebway_tracker` (squelettes Phase 1, toutes fonctions `Unimplemented`)
- 1 guide setup : Oracle Cloud Instance Webway Relay
- Glossaire avec termes MWS

### Ce qui manque

- Norme de declaration securisee (marquee "a creer")
- Systemes passifs et actifs des Trackers (marques "a creer")
- Documentation du **relay custom** (architecture, protocole, securite)
- Guide de deploiement complet (relay sur Oracle Cloud)
- Document consolidateur (vue d'ensemble unifiee)

---

## Plan de travail (12 documents a creer/modifier)

### Phase 1 — Document maitre consolide

**1. `docs/reference/Miyukini Conceptual References - Miyukini Webway System Complet.md**`

- Vision unifiee du MWS en un seul document
- Sections : Vue d'ensemble, Acteurs, Protocole, Relay, Transport, Decouverte, Securite, Deploiement
- Diagrammes Mermaid : flux de connexion, topologie reseau, sequence d'enregistrement
- References croisees vers tous les autres docs
- Ce document sert d'entree unique pour comprendre tout le systeme

### Phase 2 — Documentation du relay custom

**2. `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md**`

- Architecture du relay custom (bore etendu multi-tenant)
- Protocole d'enregistrement (COG -> relay, auth par token/secret)
- Routing par `cog_id` (multi-COG, multi-service)
- Securite : TLS, authentification, isolation, rate limiting, audit
- Robustesse : reconnexion, timeouts, backpressure, graceful shutdown
- Diagrammes : topologie relay, flux d'enregistrement, routing multi-service
- Integration avec MWS (adresses annoncees = relay_host:port + token)

**3. `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay Protocol.md**`

- Specification du protocole relay (messages de controle, format binaire)
- Handshake : authentification, enregistrement tunnel
- Messages : `REGISTER`, `CONNECT`, `DATA`, `HEARTBEAT`, `CLOSE`, `ERROR`
- Securite : TLS obligatoire, token auth, replay protection
- Versioning du protocole

### Phase 3 — Combler les sections "a creer"

**4. Mise a jour de `docs/reference/Miyukini Conceptual References - Miyukini Webway System.md**`

- Section 5.3 : Formaliser la norme de declaration securisee (schema, signature, verification)
- Section 7.1 : Definir les systemes passifs des Trackers (validation, filtrage, journalisation)
- Section 7.2 : Definir les systemes actifs des Trackers (blocage, signalement, degradation)
- Mettre a jour la section "Evolutions futures" (cocher les items realises)

**5. Mise a jour de `docs/reference/Miyukini Conceptual References - Miyukini Webway System Normes et Standards.md**`

- Finaliser les schemas de messages (JSON canonique, champs obligatoires/optionnels)
- Preciser les bindings de transport (TCP + TLS comme binding principal)
- Formaliser la matrice des statuts (regles de transition Trusted/Neutral/Under review/Distrusted/Rejected)
- Ajouter section sur l'integration avec le relay (annonce d'adresse relay)

**6. Mise a jour de `docs/reference/Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs.md**`

- Formaliser les contrats d'Outils MWS (signatures, preconditions, postconditions)
- Ajouter les outils relay (relay.register, relay.connect, relay.heartbeat)
- Mettre a jour la synthese des Kits (Participant + Tracker + Relay)

### Phase 4 — Documentation de deploiement et operations

**7. Mise a jour de `docs/setup/Miyukini - Oracle Cloud Instance Webway Relay.md**`

- Ajouter : installation Rust sur la VM Oracle Linux
- Ajouter : compilation et deploiement du binaire relay
- Ajouter : configuration systemd (service relay, demarrage automatique)
- Ajouter : configuration du firewall OS (firewalld sur Oracle Linux)
- Ajouter : monitoring et logs

**8. `docs/setup/Miyukini - Webway Relay Deployment Guide.md**`

- Guide pas-a-pas : de la VM vierge au relay fonctionnel
- Installation Rust, compilation du relay, configuration TLS
- Configuration systemd, logs, monitoring
- Tests de connectivite (depuis Windows, depuis Android)
- Troubleshooting

### Phase 5 — Contrats manquants

**9. `docs/tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker - Passive Systems Contract.md**`

- Contrat des systemes passifs : validation syntaxique, verification signature, filtrage par statut, journalisation
- Preconditions, postconditions, invariants

**10. `docs/tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker - Active Systems Contract.md**`

- Contrat des systemes actifs : blocage, signalement, degradation, alerte
- Declencheurs, actions, limites

### Phase 6 — Glossaire et references croisees

**11. Mise a jour de `docs/reference/Miyukini Conceptual References - Glossaire.md**`

- Ajouter termes : Relay Webway, Tunnel, Enregistrement relay, Heartbeat, Token d'authentification relay
- Mettre a jour les termes existants (Tracker, Participant) avec references au relay

**12. Mise a jour des references croisees dans tous les docs MWS**

- Ajouter liens vers le nouveau doc relay
- Ajouter liens vers les contrats systemes passifs/actifs
- Ajouter liens vers le guide de deploiement

---

## Fichiers cles a modifier

- `[docs/reference/Miyukini Conceptual References - Miyukini Webway System.md](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)` — sections 5.3, 7.1, 7.2
- `[docs/reference/Miyukini Conceptual References - Miyukini Webway System Normes et Standards.md](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)` — schemas, bindings, matrice
- `[docs/reference/Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs.md](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md)` — contrats, outils relay
- `[docs/setup/Miyukini - Oracle Cloud Instance Webway Relay.md](docs/setup/Miyukini%20-%20Oracle%20Cloud%20Instance%20Webway%20Relay.md)` — deploiement complet
- `[docs/reference/Miyukini Conceptual References - Glossaire.md](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)` — termes relay

## Fichiers cles a creer

- `docs/reference/Miyukini Conceptual References - Miyukini Webway System Complet.md` — document maitre
- `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md` — architecture relay
- `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay Protocol.md` — protocole relay
- `docs/setup/Miyukini - Webway Relay Deployment Guide.md` — guide deploiement
- `docs/tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker - Passive Systems Contract.md`
- `docs/tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker - Active Systems Contract.md`

