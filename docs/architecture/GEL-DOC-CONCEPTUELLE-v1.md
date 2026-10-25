# Gel de la doc conceptuelle — v1.0

<!-- @id arch.gel.v1 -->
<!-- @role documentation-freeze -->
<!-- @layer 0-9+zones -->
<!-- @human Périmètre, empreintes et règles d'amendement de la doc conceptuelle COG gelée le 2026-10-24 -->
<!-- @do freeze_conceptual_docs -->

- **Date** : 2026-10-24 (amendé le 2026-10-25) · **Version** : 1.20 · **Étiquettes git** : `doc-conceptuelle-v1.0`, `doc-conceptuelle-v1.1`, `doc-conceptuelle-v1.2`, `doc-conceptuelle-v1.3`, `doc-conceptuelle-v1.4`, `doc-conceptuelle-v1.5`, `doc-conceptuelle-v1.6`, `doc-conceptuelle-v1.7`, `doc-conceptuelle-v1.8`, `doc-conceptuelle-v1.9`, `doc-conceptuelle-v1.10`, `doc-conceptuelle-v1.11`, `doc-conceptuelle-v1.12`, `doc-conceptuelle-v1.13`, `doc-conceptuelle-v1.14`, `doc-conceptuelle-v1.15`, `doc-conceptuelle-v1.16`, `doc-conceptuelle-v1.17`, `doc-conceptuelle-v1.18`, `doc-conceptuelle-v1.19`, `doc-conceptuelle-v1.20`
- **Empreintes** : [`GEL-DOC-CONCEPTUELLE-v1.sha256`](GEL-DOC-CONCEPTUELLE-v1.sha256) — contrôle : `sha256sum -c docs/architecture/GEL-DOC-CONCEPTUELLE-v1.sha256`
- **Suite** : [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) · [revue de code](../implementation/REVUE-CODE-PREPARATION.md) · [plan d'implémentation](../implementation/PLAN-IMPLEMENTATION.md)

## 1. Périmètre gelé

<!-- @id arch.gel.perimetre -->
<!-- @role freeze-scope -->
<!-- @do list_frozen_docs -->

- **Architecture**
  - [Vue globale](COG-VUE-GLOBALE.md) — point d'entrée, décisions D1–D9
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md)
  - [Services hybrides et Portail](SERVICES-HYBRIDES-ET-PORTAIL.md)
  - [Mémoire MAIA](MEMOIRE-MAIA.md) · [IA fédérée](IA-FEDEREE.md) · [modèles locaux](MODELES-IA-PETITES-CONFIGS.md)
  - Schémas : `schema-global/V1…V6`, `schema-service-relaye.mmd`, `../schema-conceptuel-cog.mmd`
- **Sécurité**
  - [Modèle de menaces EX-01..63](../securite/MODELE-MENACES-COG.md)
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md)
  - [Audit 2026-10-23](../securite/AUDIT-2026-10-23-auth-wan.md) (historique)
- **Rendu** : [FigJam](https://www.figma.com/board/Nx7DXwWUy2jwetLGkxB0Jn) — non normatif, les `.mmd` font foi

## 2. Hiérarchie

<!-- @id arch.gel.hierarchie -->
<!-- @role precedence -->
<!-- @do define_doc_precedence -->

- La doc gelée **prime** sur tout autre document (contrats des Cores, docs de services, `doc_for_website`, `.mip/`)
- En cas de conflit : la doc subordonnée est corrigée, pas la doc gelée
- Le code se conforme à la doc gelée ; un écart constaté par la revue est un constat `R-NNN`

## 3. Règles d'amendement

<!-- @id arch.gel.amendement -->
<!-- @role change-control -->
<!-- @do amend_frozen_docs -->

- Toute modification = **amendement** `A-NNN` inscrit au §4, écrit **avant** le code (C-82)
- Contenu d'un amendement : motif, documents touchés, décision (`Dx`), impact sur EX / C / plan
- Validation : le propriétaire du projet
- Après application : version mineure (v1.1, …), empreintes régénérées, nouvelle étiquette git
- Exceptions sans amendement : coquilles, liens cassés, encodage — sans changement de sens

## 4. Registre des amendements

<!-- @id arch.gel.registre -->
<!-- @role amendment-log -->
<!-- @do log_amendments -->

| Id | Date | Objet | Version |
|---|---|---|---|
| — | 2026-10-24 | Gel initial | 1.0 |
| A-001 | 2026-10-24 | Vue globale §9 : éléments hors workspace supprimés (décision du propriétaire) | 1.1 |
| A-002 | 2026-10-24 | Lot 2 §4 : Sceau v1.1 — identifiant d'algorithme + bloc multi-signature (seuil) ; détail ci-dessous | 1.2 |
| A-003 | 2026-10-24 | Lot 2 §3.1, §5–§8 et Zones §15 : choix d'implémentation du lot 2 (tête de registre, racine, passeports et visas) ; détail ci-dessous | 1.3 |
| A-004 | 2026-10-25 | Zones §5, §10–§12, §15 · Services hybrides §3 · Lot 2 §3.1, §4.1 : choix d'implémentation du lot 3 (Portail, CDR, arrêtés, Douane, Vigies, coupure de voie) ; détail ci-dessous | 1.4 |
| A-005 | 2026-10-25 | Zones §12 : règles intégrées des Vigies sur le motif de la Douane (`LATERAL`, `NON-DECLARE` critiques, coupure de la voie fautive) ; attribution de voie par le seul côté d'entrée ; détail ci-dessous | 1.5 |
| A-006 | 2026-10-25 | Lot 2 §3, §4.1, §8, §8.1, §8.2, §9 · Zones §7, §15, §17 : choix d'implémentation du lot 4 (entrées de registre `0x04`, stockage `MRGJ`, Median-Time-Past, relevé `MCAN`, quarantaine, registre fédéral) ; têtes du registre officiel par une clé de journal d'Origin ; révocations à seuil 1 ; quarantaine persistée et scellée ; détail ci-dessous | 1.6 |
| A-007 | 2026-10-25 | Zones §7, §10, §16, §17 · Services hybrides §1, §3.1, §4.1 · Lot 2 §4.1, §8.2, §8.3, §9, §10 · IA fédérée §2, §7 · Menaces EX-32, EX-34 : intégrations du lot 4, choix des lots 5 (services hybrides) et 6 (fédération) ; délai de contestation 72 h ; registre fédéral sans Origin (≥ 3 témoins membres, **remplace** une phrase d'A-006) ; anti-rejeu du Portail par auteur ; code des Fiches ≠ `0x08` ; détail ci-dessous | 1.7 |
| A-008 | 2026-10-25 | Lot 2 §4.1, §10.1 · Services hybrides §3.1, §4.1 : nature `0x44` du recours accepté (encodage prévu par A-007) ; règles de suspension et de levée ; alignements consignés (recours, témoins membres, séquence par auteur) | 1.8 |
| A-009 | 2026-10-25 | IA fédérée §2–§5, §7 · Mémoire MAIA §4, §9, §10, §12 · Lot 2 §4.1 : choix du lot 7 (Fiche `0x09`, *k* = 3, quotas, identifiants et budgets mémoire, une base `maia-<utilisateur>`) ; **deux décisions** : republication d'une donnée étrangère après validation TAMR, notes ordinaires écrites par MAIA | 1.9 |
| A-010 | 2026-10-25 | Mémoire MAIA §9 : « note ordinaire » écrite par MAIA = note des strates N2 / N3 seulement (jamais N0 ni N1) ; précision fail-closed d'A-009 | 1.10 |
| A-011 | 2026-10-25 | Lot 2 §5 · Zones §16 · Services hybrides §4.1 · Mémoire MAIA §9 : formats et choix consignés tels qu'implémentés — fichier de clé racine `MYRC`, durcissement du processus, état fédéral `MFED`, quotas et révocations de la façade, chemin TAMR et routes de la mémoire ; **à valider par le propriétaire** | 1.11 |
| A-012 | 2026-10-25 | Lot 2 §6, §8 · Services hybrides §3 · Zones §7 : choix consignés tels qu'implémentés — clé d'appareil par COG de destination, transport des registres de révocation vers Origin et MiyuCloud, fichiers MiyuCloud `inconnu`, Landlock du sas, installateur de modèles de Central ; **à valider par le propriétaire** | 1.12 |
| A-013 | 2026-10-25 | Corrections de la revue de sécurité du 2026-10-25 : formats révisés (liaison d'appareil v2, synchro par morceaux, `MFED` v2, sas code 7), chiffrement des blobs MiyuCloud `MIYUCENC` v1, garde-fous MAIA (provenance `Inconnue`, compétence `memoire`, clé d'approbation TAMR distincte, publication par capacité) ; **à valider par le propriétaire** | 1.13 |
| A-014 | 2026-10-25 | Synchro des révocations vers Origin : chaque envoi (morceau ou tête) signé par la clé de journal du COG, débit par COG après authentification, budget global ; **à valider par le propriétaire** | 1.14 |
| A-015 | 2026-10-25 | Éditeur du registre de révocation côté COG (dépôt et envois Origin) ; étiquette `publiable` de KindMother (EX-47) et règle de dérivation des Fiches ; relais NFLOG des Vigies (format de ligne, témoin) ; **à valider par le propriétaire** | 1.15 |
| A-016 | 2026-10-25 | Table hôte de la Douane (description v2, flux de boucle locale par compte, aucun DNS), journalisation bornée, règle EX-12 des Vigies, relais (panne immédiate, ligne sans ports), étiquette `publiable` v2 (paramètres signés, propriété des bases), dérivation stricte des Fiches ; **à valider par le propriétaire** | 1.16 |
| A-017 | 2026-10-25 | Route de publication des Fiches depuis MAIA : requête signée par l'utilisateur, champs structurels calculés par le serveur, porte des Cores, débits ; clé d'identité dans le pont (à trancher) ; **à valider par le propriétaire** | 1.17 |
| A-018 | 2026-10-25 | État de publication persistant dans KindMother (base réservée `cog-federee`, une insertion par version, rechargement fail-closed), plafond décompté après ALLOW, contrôle du propriétaire de l'étiquette ; **à valider par le propriétaire** | 1.18 |
| A-019 | 2026-10-25 | État fédéral persistant (`MFED` v3 avec carnet de pairs, support KindMother `miyufederation`, rechargement revérifié, 72 h à travers les arrêts), fuzzing ; mémoire d'Alicia dans KindMother (`maia-_alicia`, `nationale`), import unique ; **à valider par le propriétaire** | 1.19 |
| A-020 | 2026-10-25 | Effacement physique dans KindMother (`secure_delete` relu, WAL vidé après suppression), effacement de l'ancienne base d'Alicia, anti-retour de l'état fédéral (`MFED` v4, compteur `MFFR`), encodage canonique du Sceau ; **à valider par le propriétaire** | 1.20 |

### A-002 — Sceau v1.1 : algorithme et multi-signature

<!-- @id arch.gel.a002 -->
<!-- @role amendment -->
<!-- @do amend_sceau_format_v1_1 -->

- **Date / validation** : 2026-10-24, propriétaire du projet · **version** 1.2
- **Motif**
  - Lot 2, incrément 2 : signature à seuil 2 sur 2 (EX-01), fraîcheur de la tête (EX-03), fuzzing du décodeur (EX-10)
  - Revue de conformité ([synthèse](../conformite/CONFORMITE-SYNTHESE.md) §2, points 4–5) : identifiant d'algorithme **avant** gel du format en production ; préparation post-quantique (ML-DSA)
  - Aucun sceau officiel émis à ce jour : pas de migration
- **Documents touchés**
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §2 (étiquette), §4.1 (format), §4.3 (API), §11, §12 (décision 1)
  - Hors gel, alignés : [fiche MiyuSceau](../tools/MiyuSceau/_index.md)
- **Décision**
  - Format **Sceau v1.1**, octet de version `0x02` (disposition incompatible avec v1.0 : un décodeur v1.0 doit la refuser, et inversement)
  - Identifiant d'algorithme dans l'entête **et** dans les octets signés : `0x01` Ed25519 (seul implémenté) ; `0x02` ML-DSA-65 et `0x03` hybride Ed25519 + ML-DSA-65 **réservés, refusés** à la vérification ; tout autre code refusé au décodage
  - Bloc de signatures `nombre || (kid, alg, longueur, signature)*`, 1 à 8 entrées, `kid` strictement croissants (doublon refusé)
  - Vérification à seuil `m` sur un jeu d'ancres ; niveau de confiance = le plus faible des signataires comptés
  - Sceau v1.0 (`0x01`) : **abandonné**, aucun décodage hérité (fail-closed, pas de voie de rétrogradation)
  - La feuille de route « Sceau v2 Ed25519 + ML-DSA » devient : Sceau v1.1 avec l'algorithme `0x03` (nouvel amendement à l'activation)
- **Impact**
  - **EX** : EX-01 rendu implémentable (seuil 2 sur 2) ; EX-10 porte sur le décodeur v1.1 ; EX-39 (agilité pilotée par seuil) préparée ; NC-STD-20d couverte
  - **C** : C-61 — vecteurs de référence régénérés (signature simple et 2 sur 2) ; C-24 inchangée (Ed25519 seul implémenté ; écart d'inventaire crypto suivi par la revue de conformité)
  - **Plan** : lot 2 incrément 2 (seuil) livré côté format ; ML-DSA et racine SLIP-39 restent à faire (incrément 3)
  - **Code** : `miyusceau` (API cassante : `Sceau`, `Verifie`), appelants `miyumarket`, `maia-llm`, `central`

### A-003 — Lot 2 : choix d'implémentation consignés

<!-- @id arch.gel.a003 -->
<!-- @role amendment -->
<!-- @do record_lot2_implementation_choices -->

- **Date / validation** : 2026-10-24, propriétaire du projet · **version** 1.3
- **Motif**
  - Lot 2, incréments 2 à 4 : choix que la doc gelée ne fixait pas, faits dans le code et signalés « à inscrire par amendement » ([plan](../implementation/PLAN-IMPLEMENTATION.md) §4)
  - Consigner l'existant, **sans nouvelle décision de conception**
- **Documents touchés**
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §3.1 (nouveau), §5, §6, §7, §8.1 et §8.2 (nouveaux), §11, §12 (décision 2)
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) : bandeau, §15 (clés par usage)
  - Hors gel, alignés : [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) C-24 révisée ; [plan](../implementation/PLAN-IMPLEMENTATION.md) §4
- **Décision** (consignation de l'implémenté)
  - **Tête de registre** : `Sceau` `0x05`, champs TLV 1–6 (`registre`, `taille`, `racine`, `sequence`, `emise_a`, `validite_max_secs`) ; âge max 24 h par défaut, plafond 7 j, tolérance 120 s
  - **Politiques** : `journal_cog` 1 signataire ; `canal_officiel` **2 sur 2**
  - **Monotonie** : recul = refus ; équivoque prouvée par les deux sceaux ; preuve de cohérence exigée à toute croissance ; tête liée au registre attendu, y compris l'état stocké (`MSRG`)
  - **Preuves Merkle** : encodage binaire propre (tailles 8 o, chemin ≤ 64)
  - **Racine** : SLIP-39 un groupe 2 sur 3, exposant 5 ; chemins SLIP-10 `m/1296256839'/u'/g'`, usages identité 0 · émission 1 · journal 2 · éditeur 3, indice jamais réattribué ; `cog_id` = base32 minuscule de SHA-256(clé racine), 26 caractères
  - **Passation** : `Sceau` `0x06` (certificat), champ `nature = 0x01`, signée par l'ancienne et la nouvelle identité
  - **Passeport** : `Sceau` `0x01` avec `destination` (un passeport par COG visité) ; `sujet` = identifiant par paire HKDF-SHA256 (sel `miyukini/miyauth/pairwise/v1`, info `"cog:" || cog_id`)
  - **Visa** : `Sceau` `0x02`, `sujet` et `cle_appareil` recopiés du passeport ; portées ⊆ passeport, comparaison exacte sans joker
  - **Durées max** : passeport 30 j, visa 15 min ; un seul signataire compté par passeport ou visa
  - **Possession** : `"miyukini/pop/v1" || défi (32) || audience || liaison de canal (32)` ; rôle accordé seulement sur `AccesProuve`
  - **Identifiants exposés** : `pw:<cog d'origine>:<hex>` ; référence de session hachée `sess:`
  - **Révocation** : fraîcheur 15 min par défaut, fail-closed
  - **Zones §15** : passeports et visas partagent la clé d'émission (comme le lot 2 §3) ; arrêtés = indice d'usage futur (≥ 4)
- **À décider ultérieurement** (points ouverts, pas des décisions)
  - Une clé d'appareil par COG visité (non-corrélation complète)
  - Format d'attestation (`attest` reste refusé)
  - Chaîne de certificats racine → clé d'émission / journal
  - Fichier racine chiffré Argon2id sur support amovible
- **Impact**
  - **EX** : EX-03 (fraîcheur, anti-gel), EX-10 (fuzzing, 4 cibles), EX-14 (révocation), EX-24 / EX-25 (racine, clé par usage), EX-36 (identifiant par COG) consignés ; aucune exigence modifiée
  - **C** : **C-24 révisée** (inventaire factuel des primitives et crates, [conformité ANSSI](../conformite/ANSSI-ET-SOUVERAINETE.md) NC-ANSSI-09, [synthèse](../conformite/CONFORMITE-SYNTHESE.md) §2 point 4)
  - **Plan** : lot 2 — la ligne « choix non fixés, à inscrire par amendement » renvoie à A-003
  - **Code** : aucun changement ; `miyusceau` (7ed62645, 6774aaa6), `miyuracine` (7e6719ca), `miyauth` (4277203c, e34f1f08)

### A-004 — Lot 3 : choix d'implémentation consignés

<!-- @id arch.gel.a004 -->
<!-- @role amendment -->
<!-- @do record_lot3_implementation_choices -->

- **Date / validation** : 2026-10-25, propriétaire du projet · **version** 1.4
- **Motif**
  - Lot 3 « Frontières » : choix que la doc gelée ne fixait pas, faits dans le code (`miyuportail`, `miyucdr`, `borderguard` arrêtés, `miyudouane`, `miyuvigie`, CaringNanny « vigies », WorrySentinel « voies ») et signalés « amendement A-004 proposé »
  - Consigner l'existant ; **deux politiques** décidées par le propriétaire (voie aveugle, quarantaine CDR de MiyuCloud) ; **une convention** de journalisation fixée
- **Documents touchés**
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) : bandeau, §5 (frontières), §10 (arrêtés), §11 (adressage, espaces de noms), §12 (Douane, Vigies, coupure), §15 (indice d'usage 4)
  - [Services hybrides et Portail](SERVICES-HYBRIDES-ET-PORTAIL.md) : bandeau, §1 (classes → sens), §3.1 (nouveau : voies, trame `0x08`, sas, CDR, quarantaine, coupure)
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §3.1 (indice 4), §4.1 (type `0x08`, natures de certificat)
  - Hors gel, alignés : [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) C-24 ; [plan](../implementation/PLAN-IMPLEMENTATION.md) §5
- **Décision** (consignation de l'implémenté)
  - **Voies** : une voie par service ; sens ouverts selon la classe — H0 aucun, H1 sortante seule, H2 / H3 les deux ; voie déclarée **désactivée** par défaut (EX-48)
  - **Trame de voie** : disposition Sceau v1.1, type **`0x08` « déclaration de voie »**, décodée par `miyuportail` seul (`miyusceau` la refuse : type inconnu) ; charge TLV tags 1–7 (`service`, `sens`, `version du traité`, `type`, `séquence`, `émise le`, `champs`) ; format interne réencodé `MPI1`
  - **Signatures** : entrante = **exactement 2** (clé de session de la façade + un auteur épinglé) ; sortante = **exactement 1** (clé de publication de la voie)
  - **Traité** : version **strictement égale** (aucune compatibilité implicite) ; anti-rejeu par **séquence strictement croissante** par session
  - **Sas** : processus séparé par lot (≤ 64 trames, délai 5 s) ; rlimits (256 Mio, CPU 5 s, 3 descripteurs, aucun fichier, aucun processus), `no_new_privs`, seccomp en liste blanche (tuer le processus) ; **deuxième validation** indépendante en zone COG, toute divergence = refus
  - **Coupure** : voie entière, **deux sens** ; session close, files purgées, génération incrémentée (lots en vol rejetés) ; **rétablissement explicite** seulement, voie remise désactivée avec une **nouvelle clé de publication**
  - **CDR** (`miyucdr`, fermé par défaut) : PNG, JPEG, GIF (1re image), WebP, BMP réencodés sans métadonnées ; texte, CSV, Markdown, JSON reconstruits ; type réel = type déclaré ; PDF, bureautique, archives, exécutables **refusés**
  - **Arrêtés** : `Sceau` `0x06` (certificat), `nature = 0x41` ; TLV `0x01`–`0x0B` ; validité ≤ **90 j** ; numéro strictement croissant par frontière ; ancres `Souverain` du COG seulement ; constitution (INV-Z1–Z7, plafonds) contrôlée au scellement et à la vérification
  - **Indice d'usage SLIP-10 `4` = arrêtés** (`m/1296256839'/4'/g'`) — **clôt** le point A-003 « arrêtés : indice d'usage futur »
  - **Douane** : trois frontières `externe`, `portail-interne`, `portail-local` ; portails en **rupture** (aucun routage) ; nftables `policy drop` ; IPS : bannissement automatique **5 blocages / 60 s → 15 min**, sources International seulement ; table de connexions ≤ 65 536 ; canal Vigies fermé ⇒ **tout bloqué**
  - **Réseau** : zone COG `10.77.0.0/24` sans passerelle · Franche `10.77.1.0/24` · Franche locale `10.77.2.0/24` · Fédération `10.78.0.0/16` ; IPv6 désactivé ; espaces de noms `mcog-z-<zone>`, `mcog-f-<frontière>`, `mcog-s-<zone>-<service>`
  - **Vigies → WorrySentinel** : règles intégrées `R-EX12-SORTIE`, `R-INV-Z7-ENTREE` (critiques, **même si la Douane a bloqué**), `R-INDETERMINE` ; jeux de règles `vigies-regles 1` (analyse stricte) ; capteur muet, périmé ou en échec → santé `Inconnu`
  - **Voies sous WorrySentinel** : `Ouverte` / `Surveillee` / `Coupee` ; seuls les signaux **critiques** coupent (et la voie aveugle, ci-dessous) ; réarmement sur **autorisation explicite** ; état global T0–T4 un cran à la fois : T1 / T2 / T3 à 1 / 2 / 3 voies coupées pour incident (un incident hors voie compte pour T2, et pour T3 avec une voie coupée), **jamais T4 automatique**, aucune désescalade automatique
- **Décision du propriétaire 1 — voie aveugle** : capteur muet, périmé ou défaillant ⇒ la voie est **coupée** (fail-closed, défaut `PolitiqueVoies.voie_aveugle = Couper`)
  - Inconvénients **acceptés** : voies coupées jusqu'au premier rapport des capteurs ; des journaux corrompus peuvent forcer une coupure
- **Décision du propriétaire 2 — quarantaine CDR (MiyuCloud, implémentation à venir)** : fichier non pris en charge par le CDR (PDF, vidéo, archives, bureautique…) ou trop volumineux ⇒ **quarantaine**
  - stocké à part, marqué « non désarmé »
  - téléchargeable seulement après une **action explicite** de l'utilisateur
  - **jamais ouvert** par un composant du COG
- **Convention fixée — journaux Douane → Vigies** : préfixe **`MDOU|<lieu>|<chaine>|<MOTIF>|` (`<chaine>` = `entree` / `transit` / `sortie`)** + groupes NFLOG 101–103 (frontières) et 110–112 (zones) = **convention canonique** ; le lecteur des Vigies (`MIYU:…`) s'y aligne
- **Impact**
  - **EX** : EX-12 (sortie de zone COG critique, même bloquée), EX-19 (une instance par frontière), EX-20 (portails sans routage, aucune route vers la zone COG), EX-21 / EX-29 (sandboxes par service, espaces de noms), EX-44 / EX-45 / EX-46 / EX-49 consignés ; aucune exigence modifiée
  - **C** : **C-24** — inventaire étendu (`miyuportail` : Ed25519, SHA-256 ; `seccompiler`, `rustix` pour le sas) ; C-12 appliquée (voie aveugle coupée, CDR fermé) ; C-26 : cibles de fuzzing MSCE et sas à écrire
  - **Plan** : lot 3 — statut « Fait / Reste »
  - **Code** : non encore commité ; alignements à faire : lecteur nftables des Vigies → `MDOU|` / NFLOG ; `miyuracine::Usage` → indice 4 « arrêtés »

### A-005 — Vigies : motif de la Douane et attribution de voie

<!-- @id arch.gel.a005 -->
<!-- @role amendment -->
<!-- @do amend_vigies_builtin_rules_douane_reason -->

- **Date / validation** : 2026-10-25, propriétaire du projet · **version** 1.5
- **Motif**
  - Écart constaté au câblage du lot 3 (469db8bf) : les règles intégrées des Vigies ne regardent que les **zones** ; un flux `LATERAL` (sandbox → sandbox d'une même zone) ou `NON-DECLARE` dans une zone (ou zone COG → adresse du Portail sur un port non déclaré) a ses deux extrémités dans la même zone : **aucun signal**, la voie fautive n'est pas coupée
  - Revue de sécurité de 469db8bf : la voie était attribuée aussi par l'interface de **sortie** (`OUT=`) et par l'adresse **source** de l'événement de l'IPS sans recoupement avec le côté d'entrée ; une adresse usurpée ou un flux **reçu** par une sandbox pouvait faire couper la voie d'une victime
- **Documents touchés**
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) : bandeau, §12 (règles intégrées, attribution de voie)
  - Hors gel, alignés : [plan](../implementation/PLAN-IMPLEMENTATION.md) §5 (à reporter)
- **Décision du propriétaire — motif de la Douane dans les règles intégrées**
  - Les règles intégrées lisent aussi le **motif** de la Douane (préfixe `MDOU|<lieu>|<chaine>|<MOTIF>|`, A-004 ; nature de l'événement de l'IPS)
  - `R-A005-LATERAL` : motif `LATERAL` (sandbox ⇄ sandbox dans un pont de zone, INV-Z8) → **critique**, catégorie `mouvement_lateral`
  - `R-A005-NON-DECLARE` : motif `NON-DECLARE` émis depuis l'**intérieur** (source en zone COG, Portail, Franche ou Franche locale), y compris **intra-zone** → **critique**, catégorie `flux_hors_contrat`
    - inclut zone COG → adresse du Portail (ou autre adresse de la zone COG) sur un port non déclaré : la Douane le journalise `SORTIE-COG` (règle `ip saddr <cog>` avant le rejet final) ; `R-EX12-SORTIE` ne le couvre pas (destination en zone COG), `R-A005-NON-DECLARE` le prend
    - hors champ : `NON-DECLARE` de source extérieure (International, Origin, Fédération) — rejet ordinaire de la frontière, traité par l'IPS (bannissement automatique) ; sans voie, il ne ferait qu'élever l'état global
  - Signal critique attribué ⇒ **coupure de la voie fautive** (elle seule, EX-49) ; non attribué ⇒ incident critique **hors voie** (`IncidentHorsVoie`) : la détection n'est jamais perdue
  - Non désactivables, identifiants réservés (comme les autres règles intégrées)
- **Décision — attribution de voie (anti-usurpation)**
  - Une voie est attribuée **seulement par le côté d'entrée** : la sandbox qui a **émis** le trafic
  - Journaux du pont de zone : interface d'**entrée** (`IN=`, port `sv<n>` de la sandbox) ; jamais l'interface de sortie (`OUT=`) ; une ligne qui n'atteint une sandbox que par `OUT=` reste **sans voie**
  - Événements de l'IPS : interface d'entrée observée jointe à la métadonnée ; voie attribuée seulement si la zone de cette interface (plan réseau de la frontière) est celle de l'adresse source, et seulement pour une **tentative de connexion nouvelle** ; `HorsEtat`, paquet d'une connexion existante, interface absente ou inconnue ⇒ **sans voie**
  - Limite : à une frontière, le recoupement est par zone ; l'usurpation entre sandboxes d'une même zone est arrêtée au pont de zone (port `sv<n>` non usurpable)
- **Impact**
  - **EX** : EX-12 (mouvement latéral et flux hors contrat de la zone COG = incidents critiques, même bloqués) ; EX-49 (seule la voie fautive est coupée ; une voie ne peut plus être coupée par le trafic d'autrui) ; aucune exigence modifiée
  - **C** : C-12 appliquée (doute sur l'attribution ⇒ pas de voie, incident hors voie)
  - **Plan** : lot 3 — ligne « Vigies : motif Douane, anti-usurpation » à reporter au §5 (hors gel)
  - **Code** : `miyuvigie` (règles intégrées, motif porté par l'événement de flux, lecteur nftables), `miyudouane` (interface d'entrée de la métadonnée), `miyufrontiere` (attribution par le côté d'entrée)

### A-006 — Lot 4 : registre et canal officiel

<!-- @id arch.gel.a006 -->
<!-- @role amendment -->
<!-- @do record_lot4_registry_channel_choices -->

- **Date / validation** : 2026-10-25, propriétaire du projet · **version** 1.6
- **Motif**
  - Lot 4 « Registre et canal officiel » : choix que la doc gelée ne fixait pas, faits dans le code (`miyuregistre`, `miyucanal`) et signalés « non spécifié par la doc gelée, à amender » dans la doc de leurs modules
  - Consigner l'existant ; **trois décisions** du propriétaire (signataire des têtes du registre officiel, signataires des révocations, persistance de la quarantaine)
- **Documents touchés**
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §3 (clé de journal d'Origin), §4.1 (natures du type `0x04`), §8, §8.1, §8.2, §9
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) : bandeau, §7 (canal officiel), §15 (clé de journal d'Origin), §17 (temps de confiance)
  - Hors gel, alignés : [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) C-24 (`hmac` → `miyucanal`) ; [plan](../implementation/PLAN-IMPLEMENTATION.md) §6 (à reporter)
- **Décision** (consignation de l'implémenté)
  - **Entrée de registre** : `Sceau` `0x04`, charge TLV — tags 1 `nature` (1 publication, 2 révocation, 3 émission, 4 décret, 5 événement) et 2 `inscrite_a` communs, puis champs propres à la nature, tous obligatoires, aucun champ en trop ; détail au [lot 2 §8](../securite/ARCHITECTURE-lot2-identite-signee.md)
  - **Feuille** Merkle = `SHA-256(0x00 || octets du sceau)` (RFC 9162) ; **identité** d'un titre scellé = SHA-256 des octets signés (stable si l'on ajoute un cosignataire)
  - **Annonce** de mise à jour = feuille `Publication` scellée **2 sur 2** sur l'ensemble **exact** {Origin, clé éditeur hors ligne} (aucun troisième signataire compté) ; inscription datée dans le futur au-delà de 120 s = refus
  - **Stockage** append-only, format fichier **`MRGJ`** : entête `"MRGJ" | 0x01` ; enregistrement `longueur (4) | octets | contrôle (8)`, contrôle = `SHA-256(longueur || octets)[0..8]` ; `fsync` à chaque ajout ; fin tronquée (écriture interrompue) retirée à l'ouverture ; enregistrement altéré au milieu = refus, aucune réparation automatique
  - **Median-Time-Past** : fenêtre de **11** têtes ; fenêtre paire (amorçage) ⇒ médiane **haute** ; horodatage **égal** au MTP accepté, strictement inférieur refusé ; un MTP **par registre** pour ses têtes, plus une **horloge de confiance globale** (monotone) alimentée par toutes les têtes acceptées (registre local, canal officiel, COG fédérés) ; état persistant `"MSMT" | 0x01 | dernier (8) | n (1) | n × horodatage (8)`
  - **Miroir** du registre officiel : copie **intégrale** ; les entrées reçues doivent reconstruire exactement la racine signée, la preuve de cohérence est recalculée sur la copie ; entrée non confirmée par une tête retirée à la réouverture
  - **Relevé** (zone COG → dépôt officiel, **pull seulement**, le dépôt ne pousse rien et reste non fiable) : demandes **`MCAN`** `"MCAN" | 0x01 | code` — `0x01` tête, `0x02` entrées `depuis (8) | nombre (4, 1..=1024)`, `0x03` artefact `empreinte (32) | taille (8)` ; décodage strict, taille exacte
  - **Quarantaine** : **48 h** comptées depuis la **première vue locale** sur l'heure de confiance, jamais depuis l'horodatage signé (une annonce antidatée ne raccourcit rien) ; plus longue configurable, plus courte refusée ; installation seulement si une tête **émise après la fin de la quarantaine** est acceptée et fraîche (toute révocation publiée pendant la quarantaine est vue), inclusion revérifiée, version strictement supérieure à l'installée
  - **Bornes** : **4 096** entrées nouvelles au plus par relevé ; **1 Mio** au plus par enregistrement stocké ; entrée reçue ≤ taille maximale d'un sceau ; artefact relevé par empreinte, **taille annoncée = borne de lecture**, taille et SHA-256 exacts exigés
  - **Registre fédéral** (EX-34) : ~~tête d'un COG acceptée seulement **cosignée** — 2 sur {clé de journal du COG, Origin}, Origin cosignant le même sceau~~ **(remplacé par A-007 : ≥ 3 témoins membres distincts, Origin = un témoin au plus)** ; clé de journal du COG ≠ clé d'Origin ; amorçage d'un COG au **premier usage** (TOFU)
- **Décision du propriétaire 1 — têtes du registre officiel**
  - Têtes signées par une **clé de journal d'Origin dédiée** (en ligne), **seuil 1** ; distincte de la clé de signature d'Origin et de la clé éditeur (configuration refusée sinon)
  - Mises à jour inchangées : **2 sur 2** {Origin, éditeur}, portées par la feuille `Publication` (annonce)
  - Politique plus stricte admise (`ConfigCanal::avec_tetes`) : **N sur N** d'un ensemble qui contient la clé de journal d'Origin, âge ≤ 24 h ; jamais plus laxiste
  - **Emplacement** : la clé de journal d'Origin n'est **pas** un usage `miyuracine` (Origin n'est pas une racine COG) ; c'est une clé d'Origin, épinglée dans la **configuration** du canal de la zone COG (`ConfigCanal`) avec Origin et l'éditeur
  - Risque accepté : la clé de journal seule ne forge aucune mise à jour ; elle peut geler le registre (refusé : tête gelée) ou présenter une vue scindée à un COG isolé (détectée par équivoque dès que deux vues se croisent ; voir journal fédéral agrégé, ci-dessous)
- **Décision du propriétaire 2 — révocations du canal** : signées par **Origin ou l'éditeur**, **seuil 1** sur l'ensemble exact {Origin, éditeur} ; la clé de journal d'Origin n'est pas comptée ; une révocation ne peut qu'**empêcher** une installation
- **Décision du propriétaire 3 — persistance de la quarantaine**
  - Liste des annonces en attente persistée avec l'heure de **première vue** (heure de confiance, bornée par le MTP) ; après redémarrage, le décompte **reprend**
  - Format **`MCQT`** : `"MCQT" | 0x01 | registre (32) | n (4) | n × (identité de l'annonce (32) | index (8) | vue_le (8) | mac (32))`, n ≤ 4 096
  - Intégrité : **HMAC-SHA256** par enregistrement sur `"MCQT" | 0x01 | registre | identité | index | vue_le` ; clé = `HMAC-SHA256(secret local du COG, "miyukini/miyucanal/quarantaine/v1")`, le secret local ne quitte pas la zone COG
  - Fail-closed : enregistrement altéré ⇒ quarantaine **complète relancée** pour cette annonce ; état illisible, d'un autre registre ou d'une autre version ⇒ relancée pour **toutes** ; jamais raccourcie ; refus signalé (événement à remonter à WorrySentinel)
  - Limite : un accès en écriture au disque peut supprimer un enregistrement (relance) ou rejouer un enregistrement authentique plus ancien de la même annonce, qui porte sa vraie première vue : jamais moins de 48 h depuis la première vue locale
- **À décider ultérieurement** (points ouverts, pas des décisions)
  - Exception « correctif critique co-signé » de la quarantaine (EX-02) : co-signataire et règle ; **non implémentée** (aucun raccourcissement)
  - Amorçage du registre officiel : ancres et première tête **épinglées dans l'image signée** ou premier usage (TOFU, implémenté)
  - Journal fédéral **agrégé** tenu par Origin (aujourd'hui : un suivi par COG, cosigné)
  - Règle de voie, à travers le CDR du Portail, des binaires dont l'intégrité tient à l'empreinte (le CDR refuse les exécutables)
  - « Sens unique » face aux demandes de relevé : positions publiques du registre officiel et empreintes publiées sortent de la zone COG (demandes génériques, mais corrélables)
- **Impact**
  - **EX** : EX-01 (2 sur 2 par la feuille), EX-02 (quarantaine 48 h persistée ; exception non implémentée), EX-03 (anti-gel), EX-14 (révocations visibles par le miroir intégral), EX-20 (relevé seulement), EX-22 (Median-Time-Past), EX-26 (OS et modèles IA par le même canal), EX-34 (registre fédéral cosigné) consignés ; aucune exigence modifiée
  - **C** : C-12 appliquée (état de quarantaine douteux ⇒ relance) ; C-24 — `hmac` 0.12 (déjà inventorié) utilisé par `miyucanal` ; C-26 : cibles de fuzzing `MRGJ`, `MCAN`, `MCQT` et entrée `0x04` à écrire
  - **Plan** : lot 4 — statut « Fait / Reste » à reporter au §6 (hors gel)
  - **Code** : `miyuregistre`, `miyucanal` (non encore commités)

### A-007 — Lots 4 (intégrations), 5 et 6 : choix consignés et décisions

<!-- @id arch.gel.a007 -->
<!-- @role amendment -->
<!-- @do record_lot4_integrations_lot5_lot6_choices -->

- **Date / validation** : 2026-10-25, propriétaire du projet · **version** 1.7
- **Motif**
  - Lot 4 : intégrations du canal officiel (Central, maia-llm, révocations miyauth) signalées « non spécifié par la doc gelée, à amender » dans leurs modules
  - Lot 5 « Services hybrides » (`miyuhybride`, `jayrdv-traite`, `jayrdv-facade`, moteur `jayrdv::hybride`) et lot 6 « Fédération » (`miyufederation`) : encodages et seuils que la doc gelée nomme sans les fixer
  - Consigner l'existant ; **trois décisions** du propriétaire (délai de contestation, registre fédéral sans Origin, services hybrides multi-appareils) ; **un conflit** de code de type résolu (Fiches ≠ `0x08`)
- **Documents touchés**
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) : bandeau, §7 (intégrations), §10 (encodage du contrat), §16 (fédération implémentée, décisions), §17 (phrase A-006 remplacée)
  - [Services hybrides et Portail](SERVICES-HYBRIDES-ET-PORTAIL.md) : bandeau, §1 (H3 refusé), §3.1 (séquence par auteur), §4.1 (nouveau)
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §4.1 (types `0x03`, `0x07`, natures `0x42`, `0x43`), §8.2 (série miyauth), §8.3 (registre fédéral), §9 (tag 5), §10 (confirmation fédérale)
  - [IA fédérée](IA-FEDEREE.md) : bandeau, §2 et §7 EX-57 (code des Fiches)
  - [Modèle de menaces](../securite/MODELE-MENACES-COG.md) : bandeau, EX-32, EX-34
  - Hors gel, alignés : [plan](../implementation/PLAN-IMPLEMENTATION.md) §6, §7, §8, §9 ; `config/strongfather/market.toml`
- **Décision — lot 4, intégrations** (consignation de l'implémenté)
  - **Opération StrongFather** `OperationMarket::MettreAJourCanal` (`mettre_a_jour_canal`, action `modification`, sujet `central.market`), distincte de la mise à jour Market ; politique **`market-canal-officiel`** : provenance `signee_officielle` **et** niveau `souverain` seulement, aucune dérogation de développement
  - **Installation des Cores** (`installer_depuis_canal`) par-dessus le canal : paquet `.msp` signé par un éditeur épinglé ; version du manifeste signé = version annoncée ; décision StrongFather **avant** l'arrêt du service, puis le chemin unique d'installation (nouvelle décision) ; un refus à une étape ⇒ aucun octet écrit
  - **Version de canal** : `version_canal(semver) = majeure × 10¹² + mineure × 10⁶ + correctif` (mineure, correctif < 10⁶) ; **pré-version ou métadonnée de build ⇒ aucune version de canal** ; version installée illisible ⇒ `u64::MAX` (aucune installation)
  - **Catégories** : Cores → Central ; **OS → refus** (aucun installateur d'OS, l'annonce reste en attente, refus signalé) ; modèle IA → installateur délégué (maia-llm), absent ⇒ refus
  - **Persistance** sous `<données>/canal-officiel/` (dossier `0700`, fichiers `0600`, écriture atomique fichier temporaire + renommage) :
    - `secret-local` : 32 o aléatoires, créé **une fois** (création exclusive) ; lien symbolique, autre taille ou droits ouverts ⇒ refus ; dérive la clé HMAC de `MCQT` (A-006)
    - `miroir.mrgj` : miroir intégral (`MRGJ`, A-006)
    - `etat.mcce` : **un seul état atomique** `"MCCE" | 0x01 | 3 × (longueur (4, BE, 1..=1 Mio) | octets)` = `EtatRegistre` (`MSRG`), fenêtre MTP du registre (`MSMT`), horloge de confiance ; taille exacte exigée ; contenu revérifié au chargement
    - `quarantaine.mcqt` : `MCQT` (A-006)
  - **Reprise** : aucun état ⇒ confiance au **premier usage** (miroir non confirmé retiré, signalé) ; état illisible, secret local douteux ou reprise refusée (signature, autre registre, stockage incohérent) ⇒ canal **désactivé**, rien n'est installé ; quarantaine absente ou altérée ⇒ relancée (A-006)
  - **Configuration** `<config>/canal-officiel.json` : `actif` (défaut faux), `registre`, `origin`, `journal_origin`, `editeur` (64 hex, clés distinctes), `periode_secs` (défaut 3 600, minimum 300) ; champ inconnu refusé ; absent, faux ou invalide ⇒ **désactivé** ; aucun cycle lancé tant que la voie de relevé du Portail n'est pas branchée
  - **Artefact « modèle »** `MMIA` : `"MMIA" | 0x01 | longueur du manifeste (4, BE, 1..=128 Kio) | manifeste signé (ManifesteModele) | octets du modèle (non vides)` ; tout vérifié **en mémoire** avant écriture (ancre `model_trust_anchors`, nom, taille, licence, SHA-256 ; nom commençant par `.` ou en `.sceau` refusé) ; écriture atomique `0644`, modèle puis `X.gguf.sceau`, contrôle de chargement rejoué ; index `.canal-modeles.json`, version strictement supérieure ; index illisible ⇒ aucune installation
  - **Révocations miyauth ← registre** (`RevocationsRegistre`, fonctionnalité `miyauth` de `miyuregistre`) : série miyauth **16 o** ↔ cible d'entrée **32 o complétée de 16 octets nuls** ; cible `Serie` à fin non nulle ignorée ; `Cle(kid)` reprise ; `Annonce` / `Artefact` ignorées ; seules les entrées au **seuil** sur les ancres fournies comptent ; horodatage = tête vérifiée ; fraîcheur 15 min ; fail-closed
  - **Refus → WorrySentinel** (`SignalCanal` + `ThreatLevel`) :
    - `High` : signature, rétrogradation, inclusion, artefact altéré, quarantaine altérée, relevé refusé (registre, sceau, temps, révoqué), échec de persistance, miroir non confirmé retiré
    - `Medium` : annonce invalide, révoquée, refus de l'installateur (StrongFather, OS, modèles, version), configuration, réponse invalide, trop d'entrées
    - `Low` : artefact indisponible, transport, révocation enregistrée, annonce annulée
    - événements ordinaires (synchronisation, quarantaine, attente, installation) : journalisés seulement
  - **Banc d'essai** `miyucanal::banc` (fonctionnalité `banc-essai`, tests seulement) : Origin, éditeur, dépôt simulés ; clés jamais persistées
- **Décision — lot 5, services hybrides** (consignation de l'implémenté ; détail : [services hybrides §4.1](SERVICES-HYBRIDES-ET-PORTAIL.md))
  - **Contrat de service** = **tag 5** du manifeste `Sceau` `0x03` (tags 1–4 du Market inchangés ; le Market le transporte **opaque**, vide refusé) ; TLV imbriqué 1–8 : genre `miyukini/contrat-service/v1`, classe, exposition par défaut (**doit valoir 0**), ressources, voie, traité, façade, entrées externes
  - **Règles** : H0 ni voie, ni traité, ni façade, ni entrée ; H1 publication seule ; H2 ≥ 1 type entrant ; **H3 refusé** tant que son encodage n'est pas spécifié ; entrées d'origine **Fédération** seule (Origin et International refusés tant qu'aucune liste blanche de pairs n'existe)
  - **Réponse du moteur** : publication seulement en **réponse** à une déclaration admise signée par son auteur, avec un **jeton à usage unique** (périmé à la désactivation ou à une nouvelle session) ; étiquette **`Session`** ; 1 à 32 messages ; chaque réponse porte `demande` = SHA-256 de la trame entrante
  - **Signature d'auteur** : l'appareil signe la charge MSCE (octets signés `0x08`) ; la façade ajoute sa clé de session par `miyuportail::msce::sceller_cosigne` (cosignature vérifiée `verify_strict`, clé faible refusée, **avant** assemblage) : la façade ne peut rien forger au nom d'un auteur
  - **Visa** : audience = identifiant du service, portée **`jayrdv:reserver`** exigée ; clé d'appareil du visa = auteur épinglé de la session
  - **Traité JayRDV v1** : entrants 1 `DemandeCreneaux`, 2 `Reservation` ; sortants 101 `CreneauDisponible`, 102 `FinCreneaux`, 103 `ReservationConfirmee`, 104 `ReservationRefusee` (motif par code, aucun texte libre)
  - **Façade jetable** : condamnée à la coupure, à la désactivation ou à l'expiration de la session (503, DB secondaire purgée) ; détruite par le `Superviseur`, reconstruite **seulement** après `retablir` **et** `activer` explicites
- **Décision — lot 6, fédération** (consignation de l'implémenté ; détail : [zones §16](MODELE-ZONES-ET-COUCHES-COG.md), [lot 2 §4.1, §10](../securite/ARCHITECTURE-lot2-identite-signee.md))
  - **Candidature** = certificat `0x06`, nature **`0x43`**, auto-signée par l'identité, validité **48 h**
  - **Confirmation** = reconnaissance `0x07`, signée par un **membre** (ni invité, ni soi-même), ≤ **24 h**, niveau 1, périmètre `federation` ; comptée à partir de sa **réception**
  - **Accusation** = certificat `0x06`, nature **`0x42`**, **un seul sceau cosigné** (≤ 8 signatures, quorum 6) ; preuve **`MFPV`** (équivoque de têtes)
  - **Preuve de calcul** : `SHA-256("miyukini/federation/candidature/v1" || 0x00 || racine || jour (8, BE) || nonce (8, BE))`, `jour = emise_a / 86 400` ; difficulté **22 bits** par défaut, plancher 8, maximum 40
  - **Quota** : **8** nouveaux COG confirmés par membre et par **30 jours**
  - **Statut membre** : compteur des 7 jours remis à zéro sous 4 ; membre acquis conservé tant qu'il reste ≥ 2 confirmations, **perdu immédiatement** sous 2 (lecture fail-closed de « durablement »), statut alors fixé par le compte ; fondateurs (graines de l'image) membres d'office
  - **Note locale** des pairs : seuil de bannissement 100, décroissance 10 points / 24 h ; jamais exportée
  - **Vue fédérale** : fraîcheur **15 min** (plus courte seulement) ; âge maximal d'une tête **24 h**
- **Décision du propriétaire 1 — délai de contestation (EX-32)**
  - Accusation au quorum de 6 ⇒ **suspension immédiate** : les titres de l'accusé sont refusés
  - **Révocation définitive après 72 h**, sauf contre-preuve signée présentée et examinée par le quorum : **6 membres distincts** signent un « recours accepté » sur la **même empreinte de preuve** ⇒ suspension levée
  - **Jamais de destruction à distance** (INV-Z11)
  - Encodage du « recours accepté » : fixé à l'implémentation, consigné par amendement ; aujourd'hui le code révoque **immédiatement** (à aligner)
- **Décision du propriétaire 2 — registre fédéral sans Origin (EX-30 prime sur A-006)**
  - Tête fédérale valide avec **≥ 3 témoins membres distincts** (clés de journal des membres actuels), en plus de la signature du COG propriétaire
  - Origin, s'il est présent, compte pour **un** témoin seulement
  - **Remplace** la phrase d'A-006 « 2 sur {clé de journal du COG, Origin} » (barrée ci-dessus)
- **Décision du propriétaire 3 — services hybrides multi-appareils**
  - Anti-rejeu du Portail **par auteur** dans une session : chaque appareil (auteur) épinglé a sa propre séquence strictement croissante
  - Plusieurs appareils partagent **une** instance de façade
  - Aujourd'hui : séquence par session (Portail et façade JayRDV, à aligner)
- **Conflit résolu — code des Fiches (lot 7)** : `0x08` reste la **déclaration de voie** (A-004) ; les Fiches de l'IA fédérée recevront un **nouveau code de type**, attribué par l'amendement du lot 7 (non choisi ici)
- **À décider ultérieurement** (points ouverts, pas des décisions)
  - Encodage de H3 : liste blanche sortante, intention StrongFather
  - Liste blanche de pairs pour les entrées Origin et International
  - Point d'application des quotas de disque et de sessions déclarés
  - Emplacement du guichet des visas
  - Rafraîchissement des révocations d'une façade (instantané remis au lancement aujourd'hui)
  - Image de façade signée réelle (aujourd'hui : SHA-256 de l'identité du crate, en processus)
  - Attestation TPM de l'indépendance des confirmateurs
  - Autres types de preuve que l'équivoque
  - Persistance fédérale des statuts (vue en mémoire aujourd'hui)
  - Transport du commérage (`inv` / `getdata`, WireGuard)
  - Transport du canal officiel par le Portail et règle des binaires vérifiés par empreinte face au CDR (A-006)
- **Impact**
  - **EX** : EX-02, EX-14, EX-20, EX-26 (canal branché sur Central et maia-llm) ; EX-16, EX-40 à EX-43, EX-47, EX-48 (lot 5) ; EX-30 à EX-35, EX-37, EX-38 (lot 6) consignés ; **EX-32** précisée (contestation 72 h), **EX-34** révisée (témoins membres, Origin facultatif)
  - **C** : C-12 appliquée (configuration ou état douteux ⇒ canal désactivé ; H3 et entrées non spécifiées ⇒ refus) ; C-24 — inventaire à compléter (`rand_core` `OsRng` : secret local, clés de session) ; C-26 : cibles de fuzzing écrites pour `MCAN`, `MCQT`, `MRGJ`, entrée `0x04`, miroir, MSCE, sas ; restent manifeste de service, titres fédéraux, `MFPV`, `MCCE`, `MMIA`
  - **Plan** : lots 4, 5, 6 — statut « Fait / Reste » ; lot 7 — « Fiches (type à attribuer, ≠ `0x08`) »
  - **Code** : non encore commité ; alignements à faire : suspension 72 h et recours (`miyufederation`), `miyuregistre::federal` (2 sur {COG, Origin}) vers ≥ 3 témoins membres, séquence par auteur (`miyuportail`, `jayrdv-facade`)

### A-008 — Recours accepté et alignements du code sur A-007

<!-- @id arch.gel.a008 -->
<!-- @role amendment -->
<!-- @do record_amendment_a008 -->

- **Motif** : A-007 prévoit que l'encodage du « recours accepté » soit fixé à l'implémentation et consigné par amendement ; le code des trois décisions d'A-007 est écrit
- **Documents touchés** : `docs/securite/ARCHITECTURE-lot2-identite-signee.md` §4.1, §10.1 ; `docs/architecture/SERVICES-HYBRIDES-ET-PORTAIL.md` §3.1, §4.1
- **Décision** (consignée telle qu'implémentée, `crates/miyufederation`)
  - **Recours accepté** : certificat `0x06`, nature **`0x44`** (« D »), un sceau cosigné par 6 à 8 membres courants distincts, **jamais** le mis en cause ; champs 1 `nature`, 2 `accuse`, 3 `type_preuve`, 4 `empreinte_preuve`, 5 `accusation` (SHA-256 des octets signés de l'accusation visée), 6 `emise_a` ; reçu avant l'échéance de 72 h
  - **Suspension** : statut `Suspendu { depuis, accusation }` ; titres du mis en cause refusés, accès fédéré, têtes et confirmations qu'il signe refusés ; il ne compte plus comme membre ; les membres peuvent continuer à le confirmer
  - **Levée** : recours accepté ⇒ statut antérieur rétabli, compteur des 7 jours **relancé** depuis la levée ; une accusation levée ne peut pas être rejouée ; une nouvelle accusation avec un quorum neuf peut suspendre à nouveau
  - **Révocation définitive** : à 72 h sans recours accepté, entrée `Revocation{cible: Cle(kid)}` produite une seule fois ; recours tardif refusé ; pénalité des confirmateurs appliquée à la révocation définitive
  - **Registre fédéral** : règle commune `verifier_temoins` (≥ 3 témoins membres distincts, clé du COG jamais comptée, Origin un témoin au plus) partagée par `miyuregistre` et `miyufederation`
  - **Portail** : anti-rejeu par auteur implémenté (marque haute conservée pour un auteur retiré ; retrait du dernier auteur refusé ; 32 épinglés, 256 connus par session)
- **Impact**
  - **EX** : EX-32 et EX-34 implémentées ; EX-44 (multi-appareils) implémentée
  - **Plan** : lot 6 — décisions d'A-007 faites ; lot 5 — multi-appareils fait
  - **Code** : `miyufederation`, `miyuregistre::federal`, `miyuportail`, `miyuhybride`, `jayrdv-facade`

### A-009 — Lot 7 : choix consignés, republication validée, notes écrites par MAIA

<!-- @id arch.gel.a009 -->
<!-- @role amendment -->
<!-- @do record_lot7_choices -->

- **Date / validation** : 2026-10-25, propriétaire du projet · **version** 1.9
- **Motif**
  - Lot 7 (`maia-federee`, `maia-memoire`, pont `miou-llm-bridge`) : formats et seuils que la doc gelée nomme sans les fixer (code des Fiches, *k*, N, quotas, budgets)
  - **Deux décisions** du propriétaire qui changent la doc gelée :
    - une donnée `etranger` peut être **republiée** après validation (IA fédérée §3 disait « jamais publiée ») ;
    - MAIA **écrit elle-même** ses notes ordinaires (Mémoire §9 disait « déductions validées en lot »)
- **Documents touchés**
  - [IA fédérée](IA-FEDEREE.md) : bandeau, §2 (code, champ `reprises`), §3 (republication), §4 (*k*, N), §5 (quotas, republication), §7 EX-57, EX-60
  - [Mémoire MAIA](MEMOIRE-MAIA.md) : bandeau, §4 (alphabet), §9 (écriture), §10 (`etranger` jamais en mémoire sans validation, aussi pour MAIA), §12 EX-53 ; budgets du §7 confirmés tels quels
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md) : bandeau, §4.1 (type `0x09`)
  - Hors gel, aligné : [plan](../implementation/PLAN-IMPLEMENTATION.md) §9
- **Décision — Fiches** (consignation de l'implémenté, `crates/maia-federee`)
  - **Type Sceau `0x09` « Fiche »**, signé par la **clé d'identité** du COG émetteur (seule clé que la vue fédérale lie au `cog_id`) ; la clé éditeur avec certificat viendra avec la validation de chaîne
  - Charge TLV, tags croissants, **tout tag inconnu refusé** : 1 `id` (`[a-z0-9._-]{1,64}`), 2 `emetteur` (32 o = signataire), 3 `sujet` (une ligne, ≤ 200 car.), 4 `type` (1 fait, 2 définition, 3 procédure, 4 document, 5 tableau), 5 `texte` (≤ 8 000 car. après normalisation, refus sans troncature), 6 `relations` (≤ 32 triplets), 7 `sources` (≤ 16, texte inerte), 8 `date`, 9 `version` (≥ 1, strictement croissante), 10 `langue`, 11 `licence` (SPDX), 12 `audience` (1 membres, 2 invités, 3 COG nommés ≤ 64), **13 `reprises`** (optionnel, voir ci-dessous)
  - **Normalisation** à la vérification, dans cet ordre : NFKC ; suppression des invisibles, contrôles bidi, C0/C1 et zones privées ; **balisage retiré** (`< > { } [ ] | * # ~` et accent grave) ; homoglyphes cyrilliques et grecs ramenés au latin ; sauts de mise en forme repliés ; une ligne pour `sujet`, relations et sources
  - **Base étrangère** : au plus 1 000 Fiches par émetteur ; émetteur non admis par la vue fédérale refusé ; embeddings recalculés localement (dimension 256)
  - **Corroboration** : *k* = **3** membres distincts par défaut (2 à 16), invité cité mais jamais compté, un COG = un ; plafond N = **3** extraits par COG et par réponse (1 à 10) ; indépendance limitée aux identités fédérales distinctes
  - **Service de requête** : 60 requêtes par heure et par demandeur, 20 résultats au plus, sujet ≤ 120 car., ≤ 12 mots ; tri déterministe ; aucun modèle
  - **MAIA-Lecteur** : ≤ 8 extraits par Fiche, termes ≤ 120 car., sortie ≤ 16 Kio ; extrait non ancré dans la Fiche écarté
- **Décision — republication d'une donnée étrangère** (propriétaire)
  - Autorisée **seulement** avec une validation explicite de l'utilisateur, **confirmée par TAMR**, liée à l'empreinte exacte de la Fiche à publier ; sans elle ⇒ refus
  - La Fiche republiée porte le champ **`reprises`** (tag 13) : ≤ 16 références `cog_id` source, `id`, `version`, empreinte SHA-256 des octets signés ; vide refusé
  - **Licence identique** à celle de chaque source ; **audience au plus aussi large** que celle de chaque source (nommés ⊂ membres ⊂ invités)
  - Une Fiche avec `reprises` est **citée** mais **ne compte jamais** pour *k* (pas de blanchiment de corroboration)
  - Inchangé : jamais de Règle ni d'apprentissage sur une donnée étrangère (EX-54, EX-63) ; la mémoire de MAIA n'est jamais publiable
- **Décision — mémoire MAIA** (consignation de l'implémenté, `crates/maia-memoire`)
  - **Identifiants** : alphabet `[a-z0-9_]`, segment commençant par une lettre, ≤ 32 car., ≤ 5 segments de branche ; numéro de note ≤ 6 chiffres, sans zéro initial
  - **Budgets** par profil : petit 600, moyen 1 500, grand et Strata 4 000 jetons ; plafonds N0 200, N1 40 par domaine, N2 150, N3 60 ; au plus 3 notes par sujet ; confiance affichée si < 7
  - **Stockage** : une base KindMother chiffrée **`maia-<utilisateur>`** par utilisateur (≤ 48 car.), étiquetée `nationale` ; utilisateurs autorisés listés dans `<données>/maia-utilisateurs` ; export `maia-memoire/v1`
  - **Confiance** : utilisateur 9 ; déduction ≤ 6 ; `etranger` ≤ 4
  - **Domaines sensibles** : `sante`, `finances`, `tiers` ⇒ validation TAMR
- **Décision — notes écrites par MAIA** (propriétaire)
  - MAIA **écrit directement** une note ordinaire si toutes les conditions tiennent : provenance `deduit` ; type ≠ Règle ; domaine non sensible ; nouvelle note, ou correction d'une note dont la version courante est d'auteur `maia`
  - Sinon **candidat** à valider, comme avant : contenu `etranger` (EX-53), domaine sensible (TAMR), correction d'une note de l'utilisateur ou validée par lui
  - Version d'auteur **`maia`**, confiance ≤ 6 ; au plus **50** écritures directes par 24 h glissantes, au-delà ⇒ candidat ; chaque écriture directe tracée (`maia_memoire::audit`)
  - MAIA n'oublie et n'efface rien ; l'utilisateur voit, corrige, exporte et oublie ces notes comme les siennes (EX-56)
- **Impact**
  - **EX** : EX-53 précisée (seule la provenance `etranger` exige la validation) ; EX-57 type `0x09` ; EX-60 republication sous TAMR, marquée `reprises`, hors corroboration ; EX-50..56, 58, 59, 61..63 implémentées au niveau bibliothèque
  - **Plan** : lot 7 — sortie atteinte au niveau bibliothèque ; restent les modèles réels, les routes et l'interface de la mémoire, le transport entre COG
  - **Code** : `maia-federee`, `maia-memoire`, `miyusceau` (`TypeSceau::Fiche`), `miou-llm-bridge`, `kindmother-service`

### A-010 — Notes de MAIA : strates N2 / N3 seulement

<!-- @id arch.gel.a010 -->
<!-- @role amendment -->
<!-- @do record_amendment_a010 -->

- **Date** : 2026-10-25 · **version** 1.10 · précision **plus stricte** d'une décision du propriétaire (A-009), signalée à lui
- **Motif** : revue de sécurité du lot 7 — une note écrite directement par MAIA en N0 (domaine `moi`, préférence d'importance ≥ 8) ou en N1 (résumé de domaine) serait injectée dans **chaque** contexte : une injection réussie dans un échange deviendrait une consigne quasi permanente
- **Documents touchés** : [Mémoire MAIA](MEMOIRE-MAIA.md) bandeau, §9
- **Décision**
  - « Note ordinaire » (A-009) = note dont la version écrite tombe en **N2 ou N3**
  - Domaine `moi`, préférence d'importance ≥ 8, résumé de domaine (N1), Règle ⇒ **candidat** à valider, comme toute autre exception d'A-009
  - Une correction qui ferait passer une note de MAIA en N0 ou N1 (importance relevée) ⇒ candidat
- **Impact** : EX-53, EX-54 renforcées ; code `maia-memoire::MemoireMaia::noter`

### A-011 — Formats consignés : clé racine, état fédéral, façade, TAMR

<!-- @id arch.gel.a011 -->
<!-- @role amendment -->
<!-- @do record_amendment_a011 -->

- **Date** : 2026-10-25 · **version** 1.11 · consignation de l'implémenté (restes des lots 2, 5, 6, 7) ; **validation du propriétaire attendue** — tout point refusé sera retiré du code
- **Documents touchés** : ce registre seulement ; les docs gelées renvoient ici pour le détail (Lot 2 §5 racine, Zones §16 fédération, Services hybrides §4.1, Mémoire MAIA §9)
- **Clé racine sur support amovible** (`crates/miyuracine`, NC-ANSSI-25)
  - Fichier **`MYRC` v1**, 122 octets exactement, gros-boutiste : magie `MYRC` (4) · version `0x0001` (2) · Argon2id `m` en Kio (4, 262 144..=4 194 304) · `t` (4, 3..=64) · `p` (4, 1..=16) · sel (32) · nonce XChaCha20 (24) · secret chiffré (32) · étiquette Poly1305 (16)
  - Clé AEAD = Argon2id v0x13 (mot de passe ≥ 12 octets, sel) ; données associées = octets 0..74 ; défaut m = 1 Gio, t = 4, p = 4
  - Toute erreur de contenu (taille, magie, version, bornes, mot de passe, altération) ⇒ **un seul refus** sans indice ; fichier ordinaire, `O_NOFOLLOW`, droits `0600`, propriétaire = uid effectif (support FAT/exFAT monté `uid=…,fmask=0177,dmask=0077`)
  - Écriture : temporaire exclusif `0600`, `fsync`, `renameat2(RENAME_NOREPLACE)` (jamais d'écrasement), `fsync` du dossier, relecture avant succès
  - **Durcissement** du processus avant toute lecture de secret : core dumps coupés (`RLIMIT_CORE` 0, `PR_SET_DUMPABLE` 0) ; `mlockall(MCL_CURRENT|MCL_FUTURE)` **exigé** par défaut (refus si impossible), `--sans-mlock` = avertissement explicite
- **État fédéral local** (`crates/miyufederation`, lot 6)
  - Fichier `<dossier>/etat.mfed` (dossier `0700`, fichier `0600`) : `"MFED" | 0x01 | n (4, ≤ 64 Mio) | corps | HMAC-SHA256 (32)` ; clé = HMAC-SHA256(secret local du COG, `miyukini/miyufederation/etat/v1`)
  - Le corps conserve les **titres signés d'origine** (candidatures `0x43`, confirmations `0x07`, accusations `0x42` et preuves `MFPV`, recours `0x44`, têtes `MSRG`), **revérifiés** au chargement ; les statuts sont **recalculés**, jamais lus ; seuls le drapeau fondateur, les heures de réception, les quotas et la note locale reposent sur le seul HMAC
  - Reprise : rejeu jusqu'à l'heure courante ; une suspension échue pendant l'arrêt devient révocation datée de l'échéance, produite une seule fois ; état illisible, altéré ou heure en recul ⇒ fédération désactivée ; fichier absent = premier usage
  - Limites : suppression du fichier ⇒ perte des statuts locaux (les révocations restent au registre) ; un état authentique plus ancien peut être rejoué
- **Façade des services hybrides** (`crates/miyuhybride`, `crates/jayrdv-facade`, lot 5 ; « à décider » d'A-007)
  - Quotas **par service**, partagés par ses façades : sessions clientes simultanées (une par clé d'appareil), requêtes simultanées, disque de la DB secondaire (table + clé + valeur, réservé **avant** écriture) ; compteurs atomiques ; refus `429 quota_sessions` / `quota_taches`, disque plein `503 db_secondaire_pleine`
  - Session libérée par `DELETE /v1/session`, inactivité de 15 min, retrait de l'appareil ou fin de la façade
  - Révocations : listes `RevocationsRegistre` par COG émetteur, remises par la zone COG ; fraîcheur ≤ 15 min (tête vérifiée) ; absente, périmée, future ou en recul ⇒ refus ; instantané de lancement valable 15 min
  - **Guichet des visas : non implémenté** — emplacement et détenteur de la clé d'émission à décider (INV-H2 : aucun secret durable en Franche)
- **Chemin TAMR et mémoire** (`apps/miou-llm-bridge`, lot 7)
  - Deux temps : demande (action figée, empreinte SHA-256 du domaine `maia/tamr/approbation/v1`, durée de vie 300 s, 32 demandes au plus) puis approbation de l'utilisateur propriétaire sur l'empreinte vue ; StrongFather tranche (`data.operation = 'tamr.approbation'`, `data.intervention = 'APPROVAL'`) ; refus, délai, autre empreinte ou autre utilisateur ⇒ rien n'est écrit ni publié
  - Seule une approbation accordée produit `Validation::UtilisateurEtTamr` (liée au candidat exact) ou `ValidationUtilisateur::confirmee_par_tamr` (liée aux octets exacts)
  - Routes `/v1/memoire/{utilisateur}/…` (consulter, historique, exporter, corriger, oublier, règles, candidats, approbations), POST / DELETE seulement, sans cookie ni CORS, **désactivées par défaut** ; authentification minimale par requête signée Ed25519 d'usage `maia/memoire` avec clés épinglées (`MAIA_MEMOIRE_CLES_UTILISATEUR`) — **provisoire**, à remplacer par passeport et visa miyauth
  - Outil `memoire_noter` (StrongFather `memoire.noter`) : jamais de Règle ; **tour contenant du contenu étranger ⇒ provenance `etranger`, candidat seulement**
  - Politiques ajoutées à `config/strongfather/maia.toml` : `maia-tamr-approbation`, `maia-memoire-noter`
- **Impact** : EX-04 (builds reproductibles, hors gel) ; NC-ANSSI-25 ; EX-32..34 (persistance) ; EX-44, EX-56 ; plan lots 2, 5, 6, 7 mis à jour

### A-012 — Clés d'appareil, révocations relayées, sas confiné, modèles

<!-- @id arch.gel.a012 -->
<!-- @role amendment -->
<!-- @do record_amendment_a012 -->

- **Date** : 2026-10-25 · **version** 1.12 · consignation de l'implémenté (restes des lots 2, 3, 4) ; **validation du propriétaire attendue** — tout point refusé sera retiré du code
- **Clé d'appareil par COG de destination** (`miyauth::appareil`, EX-14 non-corrélation)
  - Clé = graine Ed25519 issue de HKDF-SHA256 (sel `miyukini/miyauth/cle-appareil/v1`, ikm = graine d'appareil, info = `"cog:" || cog_id de destination`)
  - Liaison au COG d'origine : message `"miyukini/miyauth/liaison-appareil/v1" || 0x00 || origine || destination || clé de destination`, signé par la clé principale appairée (autorisation) **et** par la clé de destination (possession), `verify_strict` ; clé de destination égale à la clé principale refusée
  - Format du passeport inchangé ; révoquer un appareil = révoquer **chaque** clé de destination liée (non reliables entre elles)
- **Révocations relayées** (A-007 : fraîcheur ≤ 15 min, absente ou périmée ⇒ refus)
  - **Origin** : le COG pousse son registre (`POST /api/auth/revocations/sync`, `{cog_id, depuis, tete, entrees}`, ≤ 2 Mio, ≤ 4 096 entrées ; 409 si `depuis` ≠ taille du miroir) ; sécurité portée par la tête signée et la racine Merkle, sans authentification de transport ; configuration `[[forum.cogs_reconnus]]` + `registre` + `cle_journal` ; **un COG reconnu qui n'a jamais poussé son registre voit ses titres refusés**
  - **MiyuCloud** : dépôt local `tete.sceau` + `entrees.mrgj` (≤ 1 Mio, ≤ 256 Mio), relu toutes les 60 s, miroir en mémoire ; variables `MIYUCLOUD_MIYU_REGISTRE`, `…_CLE_JOURNAL`, `…_REVOCATIONS_DEPOT` (tout ou rien) ; ancre sans source ⇒ tout visa refusé
  - Clé de journal du COG : signe têtes et entrées de révocation (ancre unique, seuil 1)
- **MiyuCloud, fichiers `inconnu`** (hérités d'avant le CDR, A-004)
  - Jamais servis, même avec accord explicite (téléchargement 403 `NonReclasse`, WebDAV, ZIP exclus, blob par empreinte 404)
  - Reclassement au démarrage puis toutes les 15 min (`MIYUCLOUD_CDR_RECLASSEMENT=0` le coupe) : désarmable ⇒ `desarme` (nouveau blob) ; non désarmable ou refusé par le CDR ⇒ **quarantaine** ; erreur technique ⇒ reste `inconnu` ; écritures conditionnelles (aucune écriture de l'utilisateur écrasée)
- **Sas du Portail** (`portail-sas`, A-004)
  - Domaine **Landlock** sans aucune règle (aucun chemin accessible), avant rlimits et seccomp ; noyau sans Landlock ⇒ le sas refuse de démarrer (code de sortie 5), tout le lot est refusé, sans réglage pour lever ce refus
  - Chaque voie déclarée porte un identifiant d'instance jamais réutilisé : le contrôle « voie inchangée » compare (instance, génération) — une voie redéclarée pendant une relève ou une publication ⇒ `VoieCoupee`
- **Central, canal officiel** (A-007)
  - Champ facultatif `modeles` de `canal-officiel.json` : `{"dossier": <chemin absolu>, "ancres": [<64 hex>…]}` ; absent ⇒ tout modèle refusé ; invalide ⇒ canal désactivé ; installateur délégué = celui de maia-llm (`MMIA`)
- **Non faits, décision ou format manquant**
  - Certificat racine → clé d'émission : aucune nature de certificat `0x06` « émission » ni champs définis (A-003 « à décider ») ; seul l'épinglage explicite est accepté
  - Séparation des processus entrant / sortant du Portail : protocole interprocessus, répartition de l'état et propagation des coupures non spécifiés
  - Source des révocations des vérificateurs de `central-remote` : liste locale du COG ou registre du COG
- **Impact** : EX-14, EX-36 ; EX-44, EX-45 ; plan lots 2, 3, 4 mis à jour

### A-013 — Corrections de la revue de sécurité (2026-10-25)

<!-- @id arch.gel.a013 -->
<!-- @role amendment -->
<!-- @do record_amendment_a013 -->

- **Date** : 2026-10-25 · **version** 1.13 · formats et règles **révisés** après une revue de sécurité en quatre parties ; remplace les points correspondants d'A-011 et A-012 ; **validation du propriétaire attendue**
- **Identité et révocations** (remplace A-012 sur ces points)
  - Liaison d'appareil **v2** : domaine `miyukini/miyauth/liaison-appareil/v2`, message `0x00 || origine || destination || clé de destination || pas_apres (u64 BE)` ; échéance ≤ 5 min ; la clé principale doit être non révoquée ; registre des liaisons : révoquer un appareil = un geste unique qui révoque toutes ses clés de destination
  - Suivi d'un registre de révocation : **un seul type** (`miyuregistre::SuiviRevocationsRegistre`) partagé par Origin, MiyuCloud et la façade ; ancre de journal au niveau `Reconnu`, seuil 1 ; fraîcheur de 15 min imposée par le type ; chevauchement toléré ; ≤ 1 048 576 entrées et ≤ 256 Mio
  - Origin : synchro **par morceaux** (`tete` absente = entrées mises en attente, ≤ 15 min ; la tête valide le tout), débit 120/min par IP et 300/min par COG, rejeu identique sans recalcul ; route indépendante du forum
  - Façade : remise d'une liste seulement si elle inclut l'ancienne (même COG, date non reculée) ; session libérée au retrait d'un appareil
- **État fédéral `MFED` v2** (remplace A-011) : ancre du registre local (taille, racine) scellée dans l'état et contrôlée au chargement (anti-rejeu d'un état ancien) ; file persistée des révocations à publier, purgée seulement par accusé ; révocations inscrites au registre ré-appliquées ; v1 refusée ; dossier 0700, propriétaire vérifié, verrou exclusif ; option `etat_attendu` qui refuse l'absence de fichier
- **Sas du Portail** : aucun descripteur > 2 hérité (refusé par le parent, et par le sas : code de sortie **7**) ; `mmap` seulement anonyme et sans exécution ; chemin du binaire absolu ; verrou par voie
- **Clé racine** : tout fichier secret (part, phrase, mot de passe) lu comme le fichier `MYRC` (O_NOFOLLOW, 0600, propriétaire) ; `mlockall` tenté même sous limite basse avec privilège ; changement de mot de passe à l'export
- **Modèles** : crate `maia-modeles` partagée (plus d'inclusion de sources) ; fichier imposé `<composant>.gguf` ; installation par temporaire vérifié puis renommage, ancien modèle restauré en cas d'échec ; dérogation « non signé » décidée par le binaire, jamais par la bibliothèque
- **MiyuCloud**
  - Chiffrement des blobs **réellement actif** quand `MIYUCLOUD_ENCRYPTION_ENABLED` vaut autre chose que faux : format **`MIYUCENC` v1** = en-tête 16 o (`MIYUCENC`, `0x01`, préfixe de nonce 7 o) puis segments AES-256-GCM de ≤ 64 Kio (nonce = préfixe ‖ index u32 ‖ drapeau dernier, données associées = en-tête) ; clé de 32 octets exactement, sinon démarrage refusé ; empreinte BLAKE3 du clair ; blobs clairs hérités relus mais non migrés ; restent en clair : vignettes, transcodages, temporaires d'envoi
  - Toute modification de `blob_hash` ajuste `ref_count` dans la même transaction ; réconciliation horaire et ramasse-miettes ; tâches de fond supervisées
  - Contrôle CDR dans le service de lecture lui-même ; WOPI limité au titulaire du jeton et soumis au budget d'envoi
- **MAIA**
  - Provenance des résultats d'outils `Locale | Etranger | Inconnue` ; `Inconnue` traitée comme étranger ; liste blanche des résultats locaux ; note directe seulement si tout le tour est local
  - Note directe seulement dans un domaine **déjà établi par l'utilisateur** et non sensible ; candidats plafonnés (100 en attente, 100 par 24 h), dédoublonnés
  - Compétence `memoire` explicite (seule l'agente Alicia l'a parmi les agents intégrés) ; refus critique StrongFather d'une écriture directe sur contenu étranger
  - Approbation TAMR signée par une clé **distincte** (`MAIA_TAMR_CLES_APPROBATION`, usage `maia/tamr-approbation/<instance>/<utilisateur>`) ; la description et l'empreinte couvrent la cible et sa version ; constructeurs TAMR scellés (jeton unique par processus) ; signatures des routes liées à l'instance (en-tête `x-maia-instance`) ; débit 60/min par IP
  - Publication fédérée par **capacité** : seule une Fiche du COG local est publiable ; Fiche originale publiable seulement via l'étiquette `publiable` de KindMother (EX-47, pas encore implémentée : aucune publication originale possible) ; `reprises` sans doublon et égales aux sources
- **Impact** : EX-14, EX-36, EX-32..35, EX-44, EX-45, EX-50..56, EX-57, EX-60 ; NC-ANSSI-25

### A-014 — Enveloppe signée de la synchro des révocations

<!-- @id arch.gel.a014 -->
<!-- @role amendment -->
<!-- @do record_amendment_a014 -->

- **Date** : 2026-10-25 · **version** 1.14 · complète A-013 après une revue de sécurité : la synchro par morceaux non authentifiée permettait à un tiers de purger l'attente d'un COG ou d'épuiser son débit (déni de la fraîcheur des révocations) ; **validation du propriétaire attendue**
- **Enveloppe** : corps `{cog_id, depuis, tete?, entrees, echeance, signature}` ; signature Ed25519 (`verify_strict`) par la **clé de journal épinglée** du COG sur `"miyukini/origin/revocations/morceau/v1" || 0x00 || len(cog_id) u16 || cog_id || depuis u64 || nombre d'entrées u64 || SHA-256(entrées préfixées de leur longueur u32) || tête présente u8 || SHA-256(tête) ou 32 × 0 || echeance u64` ; exigée sur **chaque** envoi, tête comprise ; `maintenant ≤ echeance ≤ maintenant + 15 min + 120 s`
- **Comportement** : signature vérifiée avant toute lecture ou modification d'état ; non authentifié ⇒ 401 **sans purge** ; mauvaise position authentifiée ⇒ 409 sans purge ; purge seulement sur tête authentifiée refusée, bornes du COG dépassées ou attente de plus de 15 min ; enveloppe rejouée sans effet (mémorisée jusqu'à son échéance, ≤ 8 192 par COG)
- **Débit et budget** : limite par IP avant analyse, limite par COG **après** authentification ; budget global de l'attente 2 097 152 entrées et 512 Mio, en plus des bornes par COG
- **Impact** : EX-35, EX-36 ; client côté COG à écrire (format fixé ici)

### A-015 — Éditeur de révocations, étiquette `publiable`, relais NFLOG

<!-- @id arch.gel.a015 -->
<!-- @role amendment -->
<!-- @do record_amendment_a015 -->

- **Date** : 2026-10-25 · **version** 1.15 · consignation de l'implémenté (restes des lots 2, 3, 7) ; **validation du propriétaire attendue**
- **Éditeur du registre de révocation** (`miyuregistre::editeur`, binaire `miyuregistre-publier`)
  - Format de l'enveloppe A-014 défini une seule fois (`miyuregistre::enveloppe`), utilisé par Origin et par l'éditeur
  - **Dépôt** pour MiyuCloud : entrées écrites d'abord, tête ensuite (le lecteur lit la tête puis les entrées) ; chaque fichier par temporaire exclusif, `fsync`, renommage, `fsync` du dossier ; fichiers 0640, dossier 0750 appartenant à l'éditeur, aucun lien suivi, verrou exclusif ; bornes de MiyuCloud
  - **Envois Origin** : sonde par la tête seule si la position est inconnue ; morceaux ≤ 4 096 entrées ; échéance 10 min strictement croissante ; 409 ⇒ reprise à la position attendue ; 401/403 ⇒ arrêt définitif des envois vers Origin ; HTTPS obligatoire hors boucle locale ; ≥ 550 ms entre deux envois
  - Une tête émise à chaque cycle (validité 900 s, cycle 240 s par défaut, ≤ 300 s) ; clé de journal lue dans un fichier strict (0600, propriétaire, O_NOFOLLOW), jamais en argument ; écrivain unique du journal (verrou `<journal>/verrou`)
- **Étiquette `publiable`** (KindMother, EX-47)
  - Intention signée Ed25519 par une clé d'**utilisateur** (`<credentials>/utilisateurs/<u>.pub`, déposée par l'administrateur ; égale à une clé de service ⇒ démarrage refusé) ; message `miyukini/kindmother/etiquette/v1` incluant la clé du serveur ; fenêtre ±300 s ; horodatage strictement croissant par donnée ; anti-rejeu par empreinte de signature
  - Désigne une cellule de texte exacte (base, table, clé, colonne) ; empreinte du contenu `SHA-256("miyukini/kindmother/publiable/contenu/v1\0" || texte)` vérifiée à la pose ; une donnée modifiée après la pose n'est plus publiable
  - Données `nationale` (bases `maia-*` et `<data_dir>/bases-nationales`) jamais étiquetables ; état = dernier événement `posee`/`retiree` du journal d'audit chaîné (`AuditKind::Etiquette`)
  - **Règle de dérivation** : la Fiche est publiable seulement si elle est normalisée, si l'empreinte de son **texte** égale l'empreinte étiquetée, et si son sujet, ses relations et ses sources apparaissent dans ce texte ; capacité `Publiable` valable 60 s, étiquette relue à chaque attestation
  - Limites : toute clé d'utilisateur épinglée peut étiqueter toute base non `nationale` ; retrait perdu si la fin du journal d'audit est tronquée (ancrage externe à prévoir) ; pas encore de route de publication dans le pont
- **Relais NFLOG** (`crates/miyurelais`, Rust pur, sans code GPL)
  - Décodage strict et borné de `NFNL_SUBSYS_ULOG` ; ligne `<sec>.<µs6> <hôte> nflog-<G>: <préfixe> IN= OUT= SRC= DST= LEN= TTL= PROTO= [SPT= DPT=] [TYPE= CODE=] [drapeaux] [MARK=]` lue par le lecteur nftables des Vigies ; préfixe limité aux octets 0x21..0x7e ; horodatages strictement croissants ; en famille pont, `IN=`/`OUT=` = port physique (attribution de voie d'A-005)
  - Préfixe témoin `MDOU-TEMOIN|<lieu>|` : preuve de vie seulement, aucune ligne produite
  - Capteur muet, perte (trou de séquence, `ENOBUFS`) ou datagramme tronqué ⇒ relève en échec ⇒ **voie aveugle coupée** ; « voie active » déclarée par l'appelant
  - NFQUEUE (verdicts en ligne) non implémenté : décision d'architecture (chemin critique, contournement interdit par C-12)
- **Impact** : EX-12, EX-35, EX-36, EX-47, EX-57 ; plan lots 2, 3, 7

### A-016 — Table hôte de la Douane, journalisation bornée, corrections de la revue (étiquettes, relais)

<!-- @id arch.gel.a016 -->
<!-- @role amendment -->
<!-- @do record_amendment_a016 -->

- **Date** : 2026-10-25 · **version** 1.16 · consignation de l'implémenté après la revue de sécurité du lot 3 / lot 7 (constats H1, H2, M1, M3..M6, B1..B6) ; **validation du propriétaire attendue**
- **Table hôte `inet miyudouane_hote`** (`miyudouane hote`, `crates/miyudouane/src/hote.rs`)
  - En déploiement réel, la zone COG sans sortie (INV-Z1, INV-Z4, EX-12) est portée sur l'hôte par une table propre à la Douane, générée depuis une **description** bornée ; remplacement atomique de cette seule table ; plus aucune table écrite à la main par les installateurs
  - Description **version 2** (`miyudouane-hote 2` ; v1 refusée explicitement) : `groupe`, `compte`, `uid`, `port tcp|udp`, `exception <voie>` (liée à un arrêté signé), **`boucle <compte> <tcp|udp> <127.0.0.1|::1> <port>`** (port 53 refusé, ≤ 128 lignes, doublons refusés)
  - Chaîne `sortie` : boucle locale ouverte aux seuls comptes **non** COG ; pour les comptes COG, seulement les réponses (`ct state established,related`) et les flux `boucle` déclarés **pour ce compte** ; **aucune résolution DNS** ; tout le reste journalisé `MDOU|zone-cog|sortie|SORTIE-COG|` puis rejeté
  - `auditer_hote` refuse toute acceptation `oifname "lo"` hors de ces formes
- **Journalisation bornée** (toutes les tables de la Douane, anti-inondation du relais, constat H2)
  - Rejet journalisé = au plus **une ligne par flux et par seconde** (ensembles dynamiques ≤ 65 536 flux, expiration 10 s) et au plus **50 lignes/s** (rafale 100) par table ; le rejet lui-même reste inconditionnel, volume compté par le compteur nommé `rejets`
- **Règle EX-12 des Vigies** : toute ligne du lieu `zone-cog` en chaîne `sortie` (tables `miyudouane_hote` et `miyudouane_zone`) est traitée comme source en zone COG quelle que soit l'adresse ⇒ signal **critique** `R-EX12-SORTIE`
- **Relais NFLOG** (précise A-015)
  - Panne (trou de séquence, `ENOBUFS`, muet) signalée **dans la relève même** où elle survient, jamais reportée (`ReleveJournal::echec`) ; panne imputée au capteur entier
  - Tampon `SO_RCVBUFFORCE` 16 Mio (repli `SO_RCVBUF`, taille effective journalisée) ; socket vidée à chaque relève (≤ 32 768 lectures) ; datagramme d'émetteur `nl_pid ≠ 0` écarté et compté ; index d'interface résolu en nom à la réception
  - **Ligne sans ports** : fragment non initial ou en-tête de transport absent ⇒ `PROTO=6|17 FRAG|TRONQUE` sans `SPT`/`DPT` ; copie par défaut 256 octets ; toute autre combinaison sans ports refusée par le lecteur
  - Binaire : `--voie-active` ou `--sans-veille` obligatoire ; sortie cassée ⇒ arrêt, code 4
- **Étiquette `publiable` v2** (remplace la v1 d'A-015, qui est refusée)
  - Domaine `miyukini/kindmother/etiquette/v2\0`, JSON `"version": 2` ; à la pose, la signature de l'utilisateur couvre aussi `type`, `langue`, `licence`, `audience` (1 membres, 2 invités, 3 COG nommés triés) ; détail d'audit `v2;<empreinte>;<utilisateur>;<kid>;<horodatage>;<paramètres|->` (entrées v1 lisibles, jamais publiables)
  - **Propriété des bases** : `<credentials>/utilisateurs/<u>.bases` (un nom `[a-z0-9_-]{1,64}` par ligne), validé au démarrage ; étiqueter hors de ses bases ⇒ refus ; noms de base d'étiquette en minuscules, `maia-*` comparé sans casse
  - Journal : lectures autorisées non journalisées ; refus agrégés par opérateur (période 60 s) ; ≤ 120 requêtes d'étiquette par opérateur et par 60 s
  - Clés, `.bases` et révocations lus **au démarrage seulement** : tout changement exige un redémarrage
- **Règle de dérivation des Fiches originales** (précise A-015) : `id` = `km.` + 32 hex de l'empreinte de la référence de donnée ; `date` à ±300 s ; `version` = dernière publiée + 1 ; `type`, `langue`, `licence`, `audience` égaux aux paramètres signés ; ≤ 8 attestations par donnée et par 24 h
- **Éditeur du registre** (précise A-015) : une tête n'est jamais datée au-delà de `maintenant + tolérance` (horloge en recul ⇒ refus explicite) ; verrou `flock` avec délai 10 s ; graine effacée après usage
- **Impact** : EX-12, EX-35, EX-47, EX-57 ; INV-Z1, INV-Z4 ; plan lots 3, 7

### A-017 — Route de publication des Fiches depuis MAIA

<!-- @id arch.gel.a017 -->
<!-- @role amendment -->
<!-- @do record_amendment_a017 -->

- **Date** : 2026-10-25 · **version** 1.17 · consignation de l'implémenté (reste du lot 7 signalé par A-015) ; **validation du propriétaire attendue**, en particulier sur la garde de la clé d'identité (dernier point)
- **Route** `POST /v1/federee/{utilisateur}/publier` (pont `maia`, `apps/miou-llm-bridge/src/routes_federee.rs`)
  - Requête signée par une clé de l'utilisateur épinglée (`MAIA_MEMOIRE_CLES_UTILISATEUR`), usage propre `maia/federee/<instance>/<utilisateur>` : une requête de la mémoire n'est pas rejouable ici ; POST seulement, aucun cookie, aucun CORS, origine vérifiée ; corps JSON ≤ 64 Kio, champs inconnus refusés
  - Entrée : `donnee` (référence exacte de la cellule KindMother), `sujet`, `texte` ; facultatifs `relations`, `sources` ; champs structurels **déclarés** (`id`, `version`, `date`, `type`, `langue`, `licence`, `audience`, `provenance`, `reprises`) seulement **comparés**, jamais utilisés
  - Sortie : `{"publiee":{"id","version","date","empreinte","fiche"}}` ; `empreinte` = empreinte de publication (celle que cite `reprises`), distincte du hachage du sceau entier utilisé par TAMR
  - Codes : 400 corps / donnée invalides, cookie ; 401 signature requise ; 403 autre utilisateur, donnée `nationale`, non publiable, republication TAMR requise, refus de la porte ; 409 version déjà publiée ; 413 corps trop volumineux ; 422 champ structurel falsifié, non dérivée, Fiche invalide ; 429 débit IP, débit utilisateur, plafond d'attestations ; 503 publication désactivée, KindMother indisponible
- **Vérifications, dans l'ordre**
  1. Débit : 60 requêtes/min par IP, puis 30 propositions/h par utilisateur (acceptées ou refusées)
  2. Contenu étranger (`provenance` ≠ `locale`, ou `reprises` présent) ⇒ refus sans appel à KindMother ; la republication reste au chemin TAMR (non routé)
  3. Base `maia-*` ⇒ refus sans appel à KindMother, sans tenir compte de la casse
  4. Étiquette relue dans KindMother : `nationale`, absente, retirée, contenu modifié depuis la pose, ou sans paramètres signés ⇒ refus
  5. Le **serveur calcule** tous les champs structurels (règle de dérivation d'A-016) ; un champ déclaré différent ⇒ refus ; version déclarée ≤ dernière ⇒ 409 ; date déclarée à plus de 300 s ⇒ 422
  6. Fiche signée par la clé d'identité du COG ; `PublieurOriginal::attester` relit l'étiquette et applique la dérivation stricte et le plafond de 8 attestations / donnée / 24 h
  7. Décision par la **porte des Cores** (`data.operation = "federee.publier"`, refus par défaut) ; politiques `maia-federee-publier` (autorisation) et `maia-federee-publier-non-locale` (refus critique) dans `config/strongfather/maia.toml` ; seule une décision ALLOW publie (`ServiceRequete::publier` revérifie la version)
  8. Chaque publication et chaque refus tracés (`maia::federee::audit`), journal en mémoire borné à 1 024 entrées
- **Configuration** : `MAIA_FEDEREE_CLE_IDENTITE` (graine de 32 octets, même format que `identite_cog.key` de Central) ; `MAIA_FEDEREE_OPERATEUR` (opérateur KindMother en lecture sur les bases étiquetables, à enregistrer) ; sans clé ⇒ route désactivée (503)
- **Limites connues** : Fiches publiées et plafond d'attestations **en mémoire** (perdus au redémarrage, versions repartent à 1) ; un refus de la porte consomme une attestation ; aucune route de republication ; **garde de la clé d'identité dans le pont** : alternative à trancher — la laisser à Central qui signerait pour le pont
- **Impact** : EX-47, EX-57, EX-60 ; plan lot 7

### A-018 — État de publication persistant, propriétaire de l'étiquette

<!-- @id arch.gel.a018 -->
<!-- @role amendment -->
<!-- @do record_amendment_a018 -->

- **Date** : 2026-10-25 · **version** 1.18 · complète A-017 : lève la limite « état en mémoire » et corrige un constat de revue (publication d'une donnée étiquetée par un autre utilisateur) ; **validation du propriétaire attendue**
- **Persistance** : base KindMother **`cog-federee`** (opérateur propriétaire du même nom, par défaut ; `MAIA_FEDEREE_PUBLICATIONS_OPERATEUR` ; nom `maia-*` ou invalide refusé au démarrage) ; base **réservée** : jamais dans un fichier `.bases`, et une proposition dont `donnee.base` est cette base est refusée (`base_reservee`)
  - Table `fiches_publiees`, clé primaire `(fiche, version)` : `fiche`, `version` ≥ 1, `donnee` (empreinte hex de la référence), `empreinte` (empreinte hex des octets signés), `utilisateur`, `publiee_a` (horodatage = attestation consommée), `octets` (base64)
- **Ordre et garantie en cas d'arrêt brutal**
  - Sous un verrou de publication unique : rechargement si nécessaire, calcul de la version, plafond **contrôlé sans être décompté**, scellement, attestation, contrôle sans effet (`ServiceRequete::controler`)
  - Porte des Cores ; seulement sur ALLOW : **une insertion KindMother** (version, Fiche et attestation dans la même ligne), puis l'état en mémoire, puis la réponse
  - Arrêt avant l'insertion ⇒ la version reste libre ; après ⇒ rechargement, version suivante exigée ; même version, autre contenu ⇒ refus par la clé primaire
  - Démarrage : état rechargé, chaque Fiche revérifiée, versions contiguës, plafond reconstruit ; KindMother indisponible ou état incohérent ⇒ **503**, jamais de reprise à la version 1 (codes `kindmother_indisponible`, `etat_publication_incoherent`, `persistance_indisponible`) ; un changement de clé d'identité du COG bloque la route jusqu'à traitement manuel
- **Plafond** : publications sur 24 h glissantes, **décomptées seulement après ALLOW** (un refus de la porte ne consomme rien) ; requêtes concurrentes sérialisées
- **Propriétaire de l'étiquette** : la route exige que l'étiquette ait été posée par l'utilisateur authentifié, sinon 403 `non_publiable` (même code qu'une donnée non étiquetée : la route ne révèle pas qu'un autre l'a étiquetée) ; `PublieurOriginal::attester` revérifie le propriétaire à la relecture ; la propriété de la base reste contrôlée par KindMother à la pose (A-016)
- Journal des décisions toujours en mémoire (1 024 entrées)
- **Impact** : EX-47, EX-57 ; plan lot 7

### A-019 — État fédéral persistant (`MFED` v3), mémoire d'Alicia dans KindMother

<!-- @id arch.gel.a019 -->
<!-- @role amendment -->
<!-- @do record_amendment_a019 -->

- **Date** : 2026-10-25 · **version** 1.19 · consignation de l'implémenté (restes des lots 6 et 7) ; **validation du propriétaire attendue**
- **État fédéral persistant** (`crates/miyufederation`)
  - L'état scellé `MFED` (HMAC du secret local, titres signés) est stocké tel quel, en octets opaques, derrière un trait `SupportEtat` : fichier (`etat.mfed`, existant), mémoire (tests), **KindMother** (fonctionnalité `kindmother`, facultative ; la crate reste sans réseau par défaut)
  - **`MFED` v3** : champ final « carnet de pairs », `compte ≤ 4 096 × (cog_id | bloc réseau 1..64 | premier_contact u64)`, identifiants strictement croissants ; v2 lue (carnet vide) puis réécrite en v3 ; v1 refusée. Carnet plein : un nouveau venu n'évince qu'un pair inéligible, jamais un pair éligible
  - Base KindMother **`miyufederation`** (opérateur propriétaire du même nom, non `nationale`, chiffrée exigée ; `maia-*` et `cog-federee` refusés) ; tables `etat_courant(cle='courant', generation, morceaux, taille, empreinte)` et `etat_morceaux(generation, rang, octets hex ≤ 256 Kio)` ; nouvelle génération écrite, puis **bascule du pointeur** gardée par `generation = ancienne` (≤ 4 morceaux en une transaction, au-delà en lots, bascule en dernier) ; morceaux périmés purgés
  - **Rechargement** unique pour tous les supports : titres revérifiés et statuts recalculés, jamais lus tels quels ; HMAC ; l'état doit prolonger le registre local (révocations réappliquées) ; heure en recul, état altéré, tronqué ou absent alors qu'attendu ⇒ refus, aucune vue rendue ; la vue vide est écrite dès le premier usage pour qu'une disparition soit détectée
  - **Transitions** : appliquées sur une copie, scellées, écrites, puis seulement rendues visibles ; échec d'écriture ⇒ instance **hors service** jusqu'au rechargement
  - **72 h** : le début de suspension est dans les pièces revérifiées, l'échéance recalculée avec l'horloge de confiance ; une suspension échue pendant l'arrêt devient une révocation datée de l'échéance, produite une seule fois, gardée à publier jusqu'à l'accusé d'inscription au registre
  - Fuzzing : cibles `titres`, `preuve`, `etat`, `vue` (15 min chacune, aucune panique ; aucun défaut de bibliothèque) ; écart signalé : `miyusceau` encode une tête (taille 0, racine non vide) qu'il refuse au décodage
- **Mémoire d'Alicia** (`crates/miyualicia-memory`) : plus de rusqlite, exception CI retirée
  - Base KindMother **`maia-_alicia`**, classée **`nationale`** (données personnelles du foyer ; mémoire commune MAIA / Alicia, Mémoire MAIA §10–§11) ; le segment `_alicia` est volontairement invalide comme identifiant d'utilisateur MAIA : aucune base `maia-<utilisateur>` ne peut se confondre avec elle
  - Ouverture refusée sans migrations appliquées, sans classement `nationale` relu chez KindMother, ou base non chiffrée ; KindMother injoignable ⇒ erreur explicite, aucun repli sur un fichier local
  - Recherche plein texte faite par le store (tous les mots exigés, `mot*` = préfixe, casse et accents ignorés) ; plus de syntaxe FTS5
  - Import unique de l'ancienne base SQLite (moteur `kindmother-heritage`, fonctionnalité `import-heritage`, au démarrage ou par `alicia-memoire-import`) : vérification ligne à ligne, empreinte du fichier avant et après, marqueur `<base>.importe-kindmother`, idempotent ; l'ancien fichier n'est jamais modifié ni supprimé ; sans la fonctionnalité, une ancienne base non importée fait refuser l'ouverture
  - **Limite** (levée par A-020) : KindMother n'effaçait pas physiquement les pages libérées ; un oubli laissait des pages chiffrées résiduelles
- **Impact** : EX-30, EX-35, EX-37, EX-50..56 ; plan lots 6, 7

### A-020 — Effacement physique, anti-retour de l'état fédéral, encodage canonique du Sceau

<!-- @id arch.gel.a020 -->
<!-- @role amendment -->
<!-- @do record_amendment_a020 -->

- **Date** : 2026-10-25 · **version** 1.20 · corrections après revue de sécurité de l'implémenté d'A-019 et constat du fuzzing ; **validation du propriétaire attendue**
- **Effacement physique dans KindMother** (toutes bases)
  - `PRAGMA secure_delete = ON` posé et **relu** sur chaque connexion (lecture-écriture, lecture seule, journal d'audit) ; valeur relue ≠ 1 ⇒ connexion refusée
  - Tout lot comportant une suppression effective (`Delete` typé, ou `DELETE` en SQL de transition) est suivi de `wal_checkpoint(TRUNCATE)` sous le verrou d'écriture ; point de contrôle incomplet ⇒ suppression maintenue, entrée d'audit annotée, résidu vidé au prochain effacement réussi ; coût ≈ 10 ms par lot qui supprime
  - **Un oubli doit être exprimé par une suppression** : une mise à jour qui écrase une valeur n'est pas garantie effacée du WAL
  - Limites : bases antérieures (pages libérées avant ce changement : `VACUUM` unique à prévoir) ; sauvegardes `_backups/` d'avant migration (règle de conservation à fixer) ; rémanence matérielle (SSD, copie sur écriture) couverte seulement par le chiffrement
- **Ancienne base d'Alicia** (précise A-019) : après un import **vérifié** d'empreinte inchangée, l'ancienne base et ses `-wal`/`-shm`/`-journal` sont réécrits de zéros, synchronisés puis supprimés ; le marqueur garde l'empreinte, la date et les fichiers effacés ; lien symbolique ou empreinte différente ⇒ jamais effacé ; conservation seulement explicite (`--conserver-ancienne`), et `forget()` est alors **refusé** (`LegacyCopyPresent`) ; pierres tombales réappliquées après chaque import et à chaque ouverture (aucune résurrection)
- **État fédéral : anti-retour** (précise A-019)
  - **`MFED` v4** : génération monotone en fin de corps, sous le HMAC ; v2/v3 lues avec génération 0
  - **Compteur de fraîcheur `MFFR` v1**, hors de KindMother : fichier local 0600 en dossier privé verrouillé, `"MFFR" | 0x01 | génération (8) | SHA-256 de l'état (32) | mac (32)` (77 octets), clé `HMAC(secret local, "miyukini/miyufederation/fraicheur/v1")` ; ordre d'écriture : l'état puis le compteur
  - Au chargement : état plus ancien ⇒ `EtatPerime` ; même génération, autre empreinte ⇒ `EtatDivergent` ; une génération d'avance (écriture interrompue) ⇒ accepté, compteur avancé ; plus ⇒ `CompteurIncoherent` ; compteur absent ⇒ `CompteurAbsent` jusqu'à **réamorçage explicite** (après revérification complète et constat que l'état n'a pas été restauré) ; état absent alors que le compteur existe ⇒ `EtatAbsent`
  - Morceaux de la génération précédente purgés dans le lot qui suit une bascule réussie
  - Limite : un acteur capable d'écrire sur le disque local peut restaurer ensemble compteur et état (comme la paire état + registre local)
- **Sceau : encodage canonique** (`crates/miyusceau`) : tout ce que l'encodeur accepte, le décodeur l'accepte, et `decoder(encoder(x)) == x`
  - `TeteRegistre::valider` (validité `1..=plafond`, taille 0 ⇒ racine `SHA-256("")`) appelé à la construction (`TeteRegistre::nouvelle`), à l'encodage et au décodage
  - `Sceau::encoder` vérifié et faillible (charge ≤ 64 Kio, 1 à 8 signatures, algorithme et longueur de signature cohérents, `kid` strictement croissants, aucune troncature de longueur) ; `encoder_brut` réservé aux tests
  - Preuves Merkle : chemin > 64 refusé à l'encodage
  - `sceller`, `sceller_multi`, `cosigner` : la charge doit se relire et se ré-encoder à l'octet près (`EncodageNonCanonique`)
- **Impact** : EX-30, EX-35, EX-50..56 ; plan lots 2, 6, 7
