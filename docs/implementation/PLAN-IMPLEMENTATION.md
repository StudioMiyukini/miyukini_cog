# Plan d'implémentation COG

<!-- @id impl.plan -->
<!-- @role implementation-plan -->
<!-- @layer 0-9+zones -->
<!-- @human Lots ordonnés pour passer de la doc conceptuelle gelée au code : objectifs, périmètre, exigences, critères de sortie -->
<!-- @do plan_implementation -->

- **Statut** : v1.0, 2026-10-24
- **Base** : [doc gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [contraintes](CONTRAINTES-IMPLEMENTATION.md) · [revue](REVUE-CODE-PREPARATION.md)
- **Principe** : un noyau sûr d'abord, l'étendue ensuite ; aucun lot ne démarre avec un constat critique ou haut ouvert sur son périmètre
- **Taille** : S (jours) · M (semaines) · L (mois), pour une personne assistée

## 1. Vue d'ensemble

<!-- @id impl.plan.vue -->
<!-- @role plan-overview -->
<!-- @do overview_plan -->

| Lot | Objet | Taille | Dépend de |
|---|---|---|---|
| 0 | Sécurisation immédiate (constats P0) | S | — |
| R | Revue de code et matrice EX | M | 0 (en parallèle) |
| 2 | Identité signée : suite | M | 0 |
| 3 | Frontières : Portail, Douane, Vigies | L | 2 |
| 4 | Canal officiel et registre Merkle | M | 2, 3 |
| 5 | Services hybrides (contrat, moteur, façade) | L | 3, 4 |
| 6 | Fédération | L | 2, 4 |
| 7 | MAIA : mémoire et IA fédérée | M | 5, 6 |
| 8 | Durcissement continu | — | tous |

- **Démonstrateur cible** : deux COG fédérés, un service H2 relayé par le Portail, une mise à jour officielle, un incident simulé coupé par la Douane

## 2. Lot 0 — Sécurisation immédiate

<!-- @id impl.plan.lot0 -->
<!-- @role plan-lot -->
<!-- @do plan_lot0 -->

- **Objectif** : fermer les constats critiques et hauts (R-001 à R-003, R-008 à R-010, R-016) et R-004, R-005, R-015
- **Travaux**
  - TEAM : authentification, écoute locale, CORS fermé, liste blanche des nœuds (R-001)
  - Alicia API : persona dérivée du jeton, confirmation uniquement par enregistrement en attente réclamé atomiquement (R-002, R-016, R-017)
  - `miyuantispam` : implémentation réelle ou refus explicite (R-003)
  - Jay Bureau : authentification et ACL du WebSocket, échange de clés signé, nonces internes (R-008, R-009)
  - Whisper / `alicia-mvp` : jeton obligatoire, comparaison constante, origine exacte (R-010)
  - `miyuwebway_tracker`, `miyuvalidate` : implémentation testée ou refus explicite (R-015)
  - JayMail via l'IA : destinataires complets à la confirmation, pas de `bcc` d'origine IA (R-018)
  - `[lints] workspace = true` sur les 149 paquets, corrections clippy bloquantes (R-004)
  - CI : build, tests, clippy, `cargo audit`, `cargo deny` sur le workspace (R-005)
- **Sortie** : 0 constat critique ou haut connu ; CI verte
- **État 2026-10-24** : R-001 à R-005, R-008 à R-011, R-015 à R-018 fermés ; ouverts : **R-019 (haute, pont Android)**, R-020 à R-022 (collab, avant branchement Origin), CI à valider au premier push, `rust-toolchain.toml` (C-05)

## 3. Lot R — Revue de code

<!-- @id impl.plan.lotr -->
<!-- @role plan-lot -->
<!-- @do plan_review_lot -->

- **Objectif** : registre `R-NNN` complet sur P0 et P1, matrice EX-01..63
- **Méthode** : [préparation de la revue](REVUE-CODE-PREPARATION.md)
- **Sortie** : chaque exigence a un statut prouvé ; corrections P0 planifiées dans les lots suivants

## 4. Lot 2 — Identité signée (suite)

<!-- @id impl.plan.lot2 -->
<!-- @role plan-lot -->
<!-- @do plan_lot2 -->

- **Fait** : incrément 1, `miyusceau` (format Sceau v1, 15 tests)
- **Incrément 2** : signature à seuil (2 sur 2), fraîcheur de tête de registre, fuzzing du décodeur (EX-01, EX-03, EX-10)
- **Incrément 3** : racine SLIP-39 2 sur 3 sur USB, clés SLIP-10 par usage, cérémonie de rotation (EX-24, EX-25)
- **Incrément 4** : passeport et visa émis et vérifiés par `miyauth`, révocation d'appareil (EX-14, EX-36)
- **Sortie** : plus aucune authentification par secret partagé sur les points exposés

## 5. Lot 3 — Frontières

<!-- @id impl.plan.lot3 -->
<!-- @role plan-lot -->
<!-- @do plan_lot3 -->

- **Portail** : voies à sens unique par service, rupture protocolaire (MSCE), CDR, sas de décodage (EX-44, EX-45, EX-46)
- **Douane** : pare-feu et IPS appliquant les arrêtés de BorderGuard, une instance par frontière (EX-19)
- **Vigies** : capteurs IDS, signaux vers CaringNanny → WorrySentinel ; sortie de zone COG = incident critique (EX-12)
- **Zones** : zone COG sans route sortante, Franche isolée, Franche locale (EX-20, EX-21, EX-29)
- **Sortie** : test d'intrusion local — aucun flux non déclaré ne traverse ; coupure d'une voie sans impact sur les autres (EX-49)

## 6. Lot 4 — Canal officiel et registre

<!-- @id impl.plan.lot4 -->
<!-- @role plan-lot -->
<!-- @do plan_lot4 -->

- Dépôt officiel en Franche, relevé par la zone COG via le Portail (EX-20)
- Registre Merkle local et fédéral (RFC 9162), têtes signées, Median-Time-Past (EX-22, EX-34)
- Mises à jour Cores, OS et modèles IA : 2 sur 2, quarantaine temporelle (EX-01, EX-02, EX-26)
- **Sortie** : une mise à jour signée installée de bout en bout ; une mise à jour altérée refusée

## 7. Lot 5 — Services hybrides

<!-- @id impl.plan.lot5 -->
<!-- @role plan-lot -->
<!-- @do plan_lot5 -->

- Manifeste de service signé : contrat de ressources et de flux, classe H0–H3 (EX-16, EX-40)
- Patron moteur / façade / traité ; façades jetables, DB secondaire par service (EX-41, EX-42, EX-43)
- Exposition désactivée par défaut, activée service par service (EX-48)
- **Premier service migré** : un service H2 (JayRDV proposé), puis MiyuCloud
- **Sortie** : service H2 utilisable depuis un appareil fédéré sans route vers la zone COG

## 8. Lot 6 — Fédération

<!-- @id impl.plan.lot6 -->
<!-- @role plan-lot -->
<!-- @do plan_lot6 -->

- Candidature avec preuve de calcul légère ; statuts invité (≥ 2) et membre (≥ 4 pendant 7 j) (EX-31, EX-38)
- Blacklist sur preuves signées et quorum de 6 ; sanction = révocation d'identité (EX-32, EX-33)
- Note locale des pairs, anti-éclipse, fraîcheur des révocations (EX-35, EX-37)
- Fédération fonctionnelle sans Origin (EX-30)
- **Sortie** : trois COG de test atteignent le statut membre ; un COG fautif est révoqué

## 9. Lot 7 — MAIA

<!-- @id impl.plan.lot7 -->
<!-- @role plan-lot -->
<!-- @do plan_lot7 -->

- Mémoire N0–N4 sous budget de tokens, notes versionnées, oubli à la demande (EX-50 à EX-56)
- Modèle local par profil matériel (Gemma 4 E4B sur le serveur actuel, après banc d'essai)
- IA fédérée : Fiches `0x08`, double MAIA, marquage `etranger`, corroboration (EX-57 à EX-63)
- **Sortie** : réponse sourcée sur une Fiche fédérée, injection de test sans effet

## 10. Lot 8 — Durcissement continu

<!-- @id impl.plan.lot8 -->
<!-- @role plan-lot -->
<!-- @do plan_lot8 -->

- Fuzzing continu des décodeurs ; couverture ≥ 80 % des crates sécurité (C-26, C-62)
- Compilations reproductibles des Cores et services officiels (EX-04)
- Audit externe avant toute ouverture publique
- Réduction des `unwrap` hors tests sur P0 et P1 à zéro (C-10)
