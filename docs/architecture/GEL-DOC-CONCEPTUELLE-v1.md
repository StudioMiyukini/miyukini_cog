# Gel de la doc conceptuelle — v1.0

<!-- @id arch.gel.v1 -->
<!-- @role documentation-freeze -->
<!-- @layer 0-9+zones -->
<!-- @human Périmètre, empreintes et règles d'amendement de la doc conceptuelle COG gelée le 2026-10-24 -->
<!-- @do freeze_conceptual_docs -->

- **Date** : 2026-10-24 · **Version** : 1.3 · **Étiquettes git** : `doc-conceptuelle-v1.0`, `doc-conceptuelle-v1.1`, `doc-conceptuelle-v1.2`, `doc-conceptuelle-v1.3`
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
