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
| 0 bis | Constats critiques et hauts de la revue (21) | M | R |
| C | Cores réels : StrongFather appelé, KindMother authentifié, chiffré et seul écrivain | L | 0 bis |
| 2 | Identité signée : suite | M | 0 |
| 3 | Frontières : Portail, Douane, Vigies | L | 2, C |
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
- **État 2026-10-24 (fin)** : tous les constats critiques et hauts fermés (R-001 à R-005, R-008 à R-011, R-015 à R-028) ; décisions appliquées (admin toutes personas, expiration des confirmations, airgap loopback) ; restent R-006, R-007, R-012 à R-014, R-029, R-030 (moyens et bas), CI à valider au premier push, `rust-toolchain.toml` (C-05)

## 3. Lot R — Revue de code

<!-- @id impl.plan.lotr -->
<!-- @role plan-lot -->
<!-- @do plan_review_lot -->

- **Objectif** : registre `R-NNN` complet sur P0 et P1, matrice EX-01..63
- **Méthode** : [préparation de la revue](REVUE-CODE-PREPARATION.md)
- **Sortie** : chaque exigence a un statut prouvé ; corrections P0 planifiées dans les lots suivants

## 3 bis. Résultat du lot R (2026-10-24)

<!-- @id impl.plan.lotr.resultat -->
<!-- @role review-outcome -->
<!-- @do record_lot_r_outcome -->

- **Matrice** : 1 fait · 14 partiel · 42 absent · 6 contredit — [MATRICE-EXIGENCES](MATRICE-EXIGENCES.md)
- **Constats** : 71 (5 critiques, 16 hauts) — [REVUE-CONSTATS](REVUE-CONSTATS.md)
- **Verdict** : les Cores ne décident de rien dans le code (StrongFather jamais appelé) → nouveau lot C avant les frontières

## 3 ter. Lot 0 bis — Constats critiques et hauts

<!-- @id impl.plan.lot0bis -->
<!-- @role plan-lot -->
<!-- @do plan_lot0bis -->

- **Critiques** : identité de COG usurpable sur Origin (O-01) ; SSRF de l'outil `cog_services` de MAIA (K-01) ; injection de prompt pilotant des actions (K-02) ; KindMother sans authentification de l'appelant (K-04) ; StrongFather jamais appelé (K-11, traité au lot C)
- **Hauts** : Tracker non authentifié (O-02), SSRF du proxy Origin (O-03), XSS `/visit` et `/mws` (O-04, O-05), mélange de réponses entre visiteurs (O-06), Market sans signature (CA-01), serveur Home sur `0.0.0.0` (CA-02), anti-SSRF par sous-chaîne (K-03), SQL KindMother contournable (K-05), bases non chiffrées (K-06), MiyuCloud : calendriers (M-01), Nextcloud sans propriétaire (M-02, M-03), envois sans quota (M-04)
- **Contradictions de la matrice** : EX-08 (JayKonta efface ses tables), EX-26 (modèles IA non signés), EX-30, EX-41, EX-48 (exposition par défaut), EX-52 (mémoire écrasée)
- **Sortie** : 0 critique ou haut ouvert ; contradictions EX-08, EX-26 et EX-48 corrigées
- **Statut (2026-10-24)** : **clos**. Critiques et hauts corrigés sauf K-11, K-12, K-13 (architecture, lot C) et K-06 (déplacé au lot C : une fois KindMother seul écrivain, seul `kindmother-service` chiffre). Contradictions EX-08, EX-26, EX-48 et EX-52 levées ; EX-30 (lot 6) et EX-41 (lot 5) restent à leur lot. Suivi : [registre des constats §Suivi](REVUE-CONSTATS.md)

## 3 quater. Lot C — Cores réels

<!-- @id impl.plan.lotc -->
<!-- @role plan-lot -->
<!-- @do plan_lot_c -->

- StrongFather appelé sur les vrais chemins (MAIA, Alicia, Market, Admin), moteur de règles réellement évalué, refus par défaut (K-11, K-12, C-12)
- KindMother : authentification des opérateurs (clé par service), requêtes typées à la place du SQL brut, seul écrivain des données (K-04, K-05, K-13, EX-07, EX-21)
- Chiffrement des bases par défaut dans `kindmother-service`, démarrage refusé en clair hors build de debug (K-06, venu du lot 0 bis) ; journal d'audit hors des bases opérateurs, chaîné (K-08)
- Stubs des Cores qui renvoient un faux succès → refus explicite (C-14)
- **Sortie** : un test de bout en bout montre qu'une action refusée par StrongFather n'est pas exécutée
- **Statut (2026-10-24)** : **sortie atteinte** (test `bondingbrother/tests/porte_bout_en_bout.rs`). StrongFather décide (refus par défaut) ; MAIA, Alicia, Market, cycle de vie des services et Admin passent par la porte BondingBrother ; KindMother : intentions typées, audit chaîné, SQLCipher par défaut, une clé par base ; jaykonta, jay1tribu, jaykoa, jayrdv, miyukiniwatch, miyukini-central écrits par KindMother par défaut ; règle rusqlite en CI
- **Lot C (suite), restant** :
  - import des bases SQLite héritées (garde de bascule en place : démarrage refusé tant que l'ancienne base non vide n'est pas importée)
  - apps/central, central-remote et jayclub activent encore `legacy-sqlite` ; jaymanga sans chemin KindMother
  - lectures brutes TRANSITOIRE restantes (jaykonta 10, jay1tribu 4, jaykoa 1, miyukiniwatch 3) ; chemin SQL brut de KindMother à retirer ensuite
  - test d'architecture « aucune action sans décision » ; KindMother vérifiant l'empreinte de décision jointe aux intentions
  - Alicia : `CapabilitiesRuntime::new_gouverne` à brancher dans les apps ; principal authentifié comme `caller` (MAIA et Alicia en `local`)
  - non gouvernés : arrêt de services, enregistrement des services trouvés sur disque
  - troncature de la fin du journal d'audit non détectée (ancre externe de la tête de chaîne)

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
