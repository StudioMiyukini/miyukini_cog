# 24 - Backend MMO-ready

## Intention

Le MVP livre un ARPG solo/coop, mais le backend et le runtime doivent etre prepares a faire tourner une version MMO sans reecrire la simulation coeur.

## Principe fondateur

- une seule simulation autoritaire
- local solo = serveur embarque in-process
- coop = serveur dedie ou host autoritaire
- MMO futur = meme simulation extraite dans des services externes

## Topologie cible

### Cote client

- client jeu `Sodomight`
- prediction locale limitee
- interpolation et correction reseau

### Cote services

- gateway/login
- realm/lobby directory
- zone servers autoritaires
- service social:
  - chat
  - party
  - presence
- service progression/economie
- persistence characters/saves
- observabilite et administration

## Regles architecture

- le client n'autorise jamais lui-meme degats, drops ou progression
- commandes reseau idempotentes et bornees
- schemas de messages versionnes
- replication par snapshots + deltas
- interest management par zone, cellule ou proximite
- persistance separee de la boucle temps reel

## Briques Rust ciblees

- `tokio` pour orchestration async et services
- `quinn` comme candidat transport QUIC pour temps reel fiable + non fiable
- crates internes:
  - `mge-proto`
  - `mge-server-core`
  - `mge-replication`
  - `mge-net`

## Donnees et stockage

- comptes
- personnages
- inventaires
- progression quetes
- classements ladder
- etats de parties / instances
- journaux techniques et moderation si MMO complet plus tard

## Exigences des P3

- separer clairement commandes, events et snapshots
- garder les calculs gameplay dans le serveur/sim autoritaire
- pouvoir lancer localement un zone server de test
- tester `single-player local` contre `dedicated sim` sur un meme scenario
- documenter les frontieres entre save locale et persistence service

## Ce qui n'est pas exige au MVP produit

- megaserveur mondial
- exploitation live 24/7
- moderation communautaire complete
- economie globale en production

## Ce qui doit deja exister

- coeur simulation externalisable
- proto versionne
- chemin de boot local host, dedicated et client-only
- observabilite reseau minimum
- strategie anti-triche de base
