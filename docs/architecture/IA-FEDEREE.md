# IA fédérée — le web pour IA entre COG

> **Gelé v1.0 — 2026-10-24** · amendé **A-007** (v1.7 : code de type des Fiches ≠ `0x08`). Toute modification passe par un amendement : [règles du gel](GEL-DOC-CONCEPTUELLE-v1.md).

<!-- @id arch.ia.federee -->
<!-- @role architecture-specification -->
<!-- @layer 8 -->
<!-- @human Publication de Fiches signées pour les IA des COG fédérés, double MAIA, marquage d'origine, défense contre injection et empoisonnement -->
<!-- @do design_federated_ai_knowledge -->

- **Statut** : v1, 2026-10-24
- **Base** : [modèle zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) · [services hybrides](SERVICES-HYBRIDES-ET-PORTAIL.md) · [mémoire MAIA](MEMOIRE-MAIA.md) · [menaces](../securite/MODELE-MENACES-COG.md)
- **Principe de sécurité** : aucun modèle n'est immunisé contre l'injection → la défense est **architecturale** : une injection peut arriver, elle ne peut pas **agir** ; un empoisonnement est **borné et traçable**

## 1. Vue d'ensemble

<!-- @id arch.ia.federee.vue -->
<!-- @role overview -->
<!-- @do summarize_federated_ai -->

- **MAIA** : interne pur (H0), inférence locale en zone COG, sans réseau, outils par intentions
- **Publication IA** (H1) : le COG publie des **Fiches** (données structurées pour LLM), au lieu de pages web pour humains
- **Service de requête déterministe** : une IA étrangère interroge une **base à requêtes typées**, jamais MAIA
- **Recherche fédérée** (H3) : une façade interroge les COG fédérés ; les Fiches reçues passent quarantaine, CDR, Portail
- **Base de connaissances étrangère** : séparée des connaissances nationales et de la mémoire de MAIA

## 2. Format « Fiche »

<!-- @id arch.ia.federee.fiche -->
<!-- @role data-format -->
<!-- @layer 6 -->
<!-- @do define_fiche_format -->

- Sceau v1, nouveau type ~~**`0x08 Fiche`**~~ **Fiche** (code à attribuer par l'amendement du lot 7, ≠ `0x08` déjà pris par la déclaration de voie ; amendé A-007), signée par le COG émetteur
- Champs typés :
  - `sujet`, `type` (fait, définition, procédure, document, tableau)
  - `texte` (≤ ~2 000 tokens, texte brut)
  - `relations` sujet–prédicat–objet
  - `sources`, `date`, `version`, `langue`, `licence`, `audience`
- **Aucun champ libre de consignes**
- Normalisation à l'entrée :
  - suppression des caractères invisibles, contrôles bidi, sauts de mise en forme
  - normalisation des homoglyphes, aucun balisage
  - aucune URL suivie automatiquement
- **Embeddings recalculés localement** : jamais de vecteurs fournis par un tiers
- Catalogue signé par COG : liste des Fiches publiées, inscrite au registre Merkle

## 3. Défense contre l'injection

<!-- @id arch.ia.federee.injection -->
<!-- @role prompt-injection-defense -->
<!-- @layer 8 -->
<!-- @do neutralize_prompt_injection -->

- **Deux MAIA** (patron « dual LLM », CaMeL) :
  - **MAIA-Planificateur** : outils, **ne voit jamais** le contenu étranger
  - **MAIA-Lecteur** : lit le contenu étranger, **aucun outil**, renvoie des extractions typées
- **Marquage d'origine** : toute valeur issue d'une Fiche porte `etranger` à travers tous les traitements
  - ne déclenche aucune action
  - n'est argument d'aucun outil sensible sans TAMR
  - n'est jamais publiée, jamais écrite en mémoire longue sans validation
- **Mode consultation** : répondre sur des connaissances fédérées se fait **sans outil** → pire cas : une mauvaise réponse
- Contenu étranger délimité et annoncé non fiable dans le contexte (défense d'appoint)
- **Réponse sourcée** : chaque affirmation cite la Fiche et le COG émetteur

## 4. Défense contre l'empoisonnement

<!-- @id arch.ia.federee.empoisonnement -->
<!-- @role data-poisoning-defense -->
<!-- @layer 7-8 -->
<!-- @do bound_data_poisoning -->

- **Provenance signée** ; poids selon statut fédéral (membre > invité), note locale, blacklist
- **Corroboration** : « établi » seulement si ≥ *k* COG indépendants concordent, sinon « source unique »
- **Registre Merkle des publications** : aucune modification silencieuse, rétractations tracées
- **Plafond par source** : au plus N extraits par COG et par réponse (anti-inondation)
- **Aucun apprentissage automatique** sur des données étrangères (ni fine-tuning, ni mémoire longue)
- **WorrySentinel** analyse le contenu : motifs d'injection, publications massives, anomalies
- Une Fiche piégée est **signée** → preuve pour la note locale et la blacklist fédérale

## 5. Publication par son propre COG

<!-- @id arch.ia.federee.publication -->
<!-- @role knowledge-publication -->
<!-- @layer 7 -->
<!-- @do publish_knowledge_safely -->

- Seules les données étiquetées **`publiable`** par l'utilisateur
- Audience par Fiche : membres, invités, COG nommés
- Requêtes **typées**, sans texte libre envoyé à un LLM
- Quotas par demandeur (anti-aspiration, anti-exfiltration)
- Aucune donnée de la mémoire de MAIA (`nationale`) n'est publiable

## 6. Risque résiduel

<!-- @id arch.ia.federee.residuel -->
<!-- @role residual-risk -->
<!-- @do state_residual_ai_risk -->

- Fiche fausse mais corroborée par plusieurs COG complices → bornée par l'indépendance des confirmateurs et la blacklist sur preuves
- Utilisateur qui valide une action trompeuse proposée par l'IA → TAMR rare et explicite

## 7. Exigences

<!-- @id arch.ia.federee.exigences -->
<!-- @role security-requirements -->
<!-- @do derive_federated_ai_requirements -->

- **EX-57** Fiches signées (type à attribuer, ≠ `0x08`, A-007), champs typés, aucun champ de consignes, normalisation à l'entrée
- **EX-58** Aucune IA étrangère n'interroge MAIA : service de requête déterministe uniquement
- **EX-59** Double MAIA : le planificateur ne voit jamais de contenu étranger
- **EX-60** Marquage `etranger` propagé ; aucune action, publication ni mémoire longue sans validation
- **EX-61** Corroboration par *k* COG indépendants, plafond d'extraits par source
- **EX-62** Embeddings toujours recalculés localement
- **EX-63** Aucun apprentissage automatique sur des données étrangères
