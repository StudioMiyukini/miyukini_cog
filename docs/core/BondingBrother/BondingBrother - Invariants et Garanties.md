# BondingBrother - Invariants et Garanties

## 1. Contexte

Ce document formalise les invariants techniques et les garanties de Bonding Brother. Il étend la Section 10 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) en détaillant les propriétés non négociables et les engagements mesurables.

## 2. Portée / Scope

Ce document couvre :
- Les invariants structurels (toujours vrais par construction)
- Les invariants comportementaux (toujours respectés à l'exécution)
- Les garanties envers les produits
- Les garanties envers les autorités
- Les mécanismes de vérification

Ce document **ne couvre pas** :
- Les violations et anti-patterns (voir document dédié)
- Les détails d'implémentation
- Les cas d'erreur (voir Error & Rejection Model)

---

## 3. Définitions

### 3.1 Invariant

Un **invariant** est une propriété qui doit toujours être vraie. Elle ne peut jamais être violée, quelles que soient les circonstances. Un invariant est vérifié par construction (architecture) ou par assertion (code).

**Caractéristiques d'un invariant :**
- Non négociable : aucune exception possible
- Non configurable : pas d'option pour le désactiver
- Non contournable : aucun chemin de code ne peut l'éviter
- Vérifiable : son respect peut être prouvé

### 3.2 Garantie

Une **garantie** est un engagement de Bonding Brother envers ses consommateurs. Elle décrit un comportement promis que les consommateurs peuvent considérer comme acquis.

**Caractéristiques d'une garantie :**
- Contractuelle : formellement documentée
- Mesurable : son respect peut être vérifié
- Stable : ne change pas sans changement de version majeure

---

## 4. Invariants de nature (ce que Bonding Brother EST)

Ces invariants définissent la nature fondamentale de Bonding Brother. Ils sont vrais par définition et ne peuvent être remis en question.

### 4.1 INV-NAT-01 : Médiateur, pas autorité

**Énoncé :** Bonding Brother est un médiateur. Il n'est pas, et ne peut jamais devenir, une autorité.

**Implications :**
- Aucun composant de BB ne détient de vérité
- Aucun composant de BB ne prend de décision finale
- Aucun composant de BB ne définit de règle

**Vérification :** Revue architecturale. Aucun composant ne possède de méthode `decide()`, `rule()`, ou `store_truth()`.

---

### 4.2 INV-NAT-02 : Traducteur, pas exécuteur

**Énoncé :** Bonding Brother traduit et transmet. Il n'exécute jamais d'action métier.

**Implications :**
- BB ne modifie pas les données métier
- BB ne crée pas d'entités métier
- BB ne supprime pas d'entités métier

**Vérification :** Les composants de traduction sont des fonctions pures sans effet de bord.

---

### 4.3 INV-NAT-03 : Filtre, pas source

**Énoncé :** Bonding Brother filtre l'information. Il n'est jamais la source de l'information.

**Implications :**
- Toute donnée transmise par BB provient d'une autorité
- BB ne génère pas de données
- BB ne fabrique pas de réponses

**Vérification :** Traçabilité complète de toute donnée sortante vers sa source (autorité).

---

## 5. Invariants de non-action (ce que Bonding Brother NE FAIT JAMAIS)

Ces invariants définissent les actions que Bonding Brother refuse structurellement d'effectuer.

### 5.1 INV-NEG-01 : Jamais de décision

**Énoncé :** Bonding Brother ne prend jamais de décision stratégique, politique, ou opérationnelle.

**Exemples de décisions interdites :**
- Autoriser ou refuser un accès
- Valider ou invalider une donnée métier
- Choisir entre plusieurs options métier
- Définir une priorité métier

**Ce qui est autorisé :**
- Décisions techniques de routage (vers KM ou SF)
- Décisions de format (quel traducteur utiliser)
- Décisions de filtrage (appliquer une règle définie par une autorité)

**Vérification :** Revue de code. Aucune logique conditionnelle basée sur des critères métier.

---

### 5.2 INV-NEG-02 : Jamais de stockage de vérité

**Énoncé :** Bonding Brother ne stocke jamais l'état des données, des identités, ou des permissions.

**Stockages interdits :**
- Cache de données métier
- Cache de permissions
- Cache d'identités
- Réplique d'état d'autorité

**Stockages autorisés :**
- Journal des interactions (immutable, sans valeur de vérité)
- Buffer offline (temporaire, en attente de transmission)
- Configuration (immuable après démarrage)

**Vérification :** Audit des structures de données. Aucune structure ne représente un "état courant" métier.

---

### 5.3 INV-NEG-03 : Jamais de création de règle

**Énoncé :** Bonding Brother ne crée, ne modifie, et ne supprime jamais de règle.

**Ce que BB ne fait pas :**
- Définir qui peut accéder à quoi
- Définir quel format est valide
- Définir quelles données sont cohérentes

**Ce que BB fait :**
- Appliquer les règles définies par les autorités
- Transmettre les règles aux produits (si demandé par une autorité)

**Vérification :** Les règles sont chargées depuis une source externe (autorité ou configuration), jamais générées.

---

### 5.4 INV-NEG-04 : Jamais de contournement d'autorité

**Énoncé :** Bonding Brother ne permet jamais à un produit d'accéder directement aux autorités en le contournant.

**Implications :**
- Toute interaction produit-autorité passe par BB
- Aucune API directe vers les autorités n'est exposée
- Aucun mode "bypass" n'existe

**Vérification :** Analyse réseau et API. Les autorités ne sont accessibles que via BB.

---

### 5.5 INV-NEG-05 : Jamais de modification de décision

**Énoncé :** Bonding Brother ne modifie jamais une décision d'autorité.

**Ce que BB ne fait pas :**
- Transformer un "refusé" en "accepté"
- Ajouter des permissions non accordées
- Supprimer des restrictions imposées

**Ce que BB fait :**
- Transmettre fidèlement la décision
- Traduire le format (sans changer le sens)
- Filtrer les informations non nécessaires (sans changer la décision)

**Vérification :** Comparaison automatisée entre décision reçue et décision transmise.

---

### 5.6 INV-NEG-06 : Jamais de masquage d'origine

**Énoncé :** Bonding Brother ne cache jamais l'origine d'une intention aux autorités.

**Informations toujours transmises :**
- Identité du produit émetteur
- Identité de l'utilisateur (si applicable)
- Timestamp de l'intention
- Contexte complet fourni par le produit

**Vérification :** Audit des demandes transmises aux autorités. Toutes contiennent le contexte complet.

---

## 6. Invariants de flux (comment les données transitent)

Ces invariants définissent les propriétés du transit des données à travers Bonding Brother.

### 6.1 INV-FLUX-01 : Séquence complète

**Énoncé :** Toute intention suit la séquence complète de traitement, sans saut d'étape.

**Séquence obligatoire (Produit → Autorité) :**
1. Réception
2. Validation structurelle
3. Traduction
4. Filtrage d'entrée
5. Journalisation
6. Transmission à l'autorité

**Séquence obligatoire (Autorité → Produit) :**
1. Réception de la réponse
2. Traduction
3. Filtrage de sortie
4. Journalisation
5. Émission au produit

**Vérification :** Chaque étape est tracée. Une trace incomplète déclenche une alerte.

---

### 6.2 INV-FLUX-02 : Journalisation systématique

**Énoncé :** Toute interaction est journalisée, sans exception.

**Éléments journalisés :**
- Intention reçue (avec contexte complet)
- Demande transmise (avec timestamp)
- Réponse reçue (avec timestamp)
- Résultat émis (avec timestamp)
- Erreurs survenues (avec détails)

**Ce qui n'est jamais journalisé :**
- Secrets (mots de passe, tokens)
- Données personnelles sensibles (configurable selon RGPD)

**Vérification :** Audit du journal. Toute interaction a une entrée correspondante.

---

### 6.3 INV-FLUX-03 : Ordre préservé

**Énoncé :** Les intentions d'un même produit sont traitées dans leur ordre d'arrivée.

**Implications :**
- Pas de réordonnancement
- Pas de traitement parallèle intra-produit (sauf si explicitement autorisé)
- En mode offline, l'ordre est préservé dans le buffer

**Vérification :** Comparaison des timestamps d'arrivée et de traitement.

---

### 6.4 INV-FLUX-04 : Aucune perte

**Énoncé :** Aucune intention n'est perdue, même en cas d'erreur ou de déconnexion.

**Mécanismes de protection :**
- Journalisation avant transmission
- Buffer offline en cas de déconnexion
- Retry automatique configurable
- Notification en cas d'échec définitif

**Vérification :** Réconciliation périodique entre intentions reçues et résultats émis.

**Conformité autonomie :** Cet invariant garantit le respect de **LOI-2** (isolement comme état normal) et **LOI-3** (état local souverain) : les intentions sont préservées localement même en déconnexion, et leur état local est considéré comme valide. Voir les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md).

---

## 7. Garanties envers les produits

Ces garanties sont les engagements de Bonding Brother envers les produits qui l'utilisent.

### 7.1 GAR-PROD-01 : Interface stable

**Engagement :** L'interface de Bonding Brother ne change pas de manière rétro-incompatible sans changement de version majeure et période de dépréciation.

**Implications :**
- Les produits existants continuent de fonctionner
- Les nouvelles fonctionnalités sont additives
- Les breaking changes sont documentés et planifiés

**Mesure :** Zéro breaking change entre versions mineures.

---

### 7.2 GAR-PROD-02 : Traduction fidèle

**Engagement :** La sémantique des intentions est préservée lors de la traduction.

**Implications :**
- Ce que le produit veut faire est compris par l'autorité
- La réponse de l'autorité est comprise par le produit
- Aucune information essentielle n'est perdue

**Mesure :** Tests de round-trip (traduction aller-retour).

---

### 7.3 GAR-PROD-03 : Résultat filtré et sûr

**Engagement :** Les résultats transmis aux produits ne contiennent que des informations autorisées et nécessaires.

**Implications :**
- Pas de fuite d'informations d'autres produits
- Pas d'exposition de détails internes des autorités
- Pas de données au-delà du périmètre demandé

**Mesure :** Tests de pénétration et audits de sécurité.

---

### 7.4 GAR-PROD-04 : Transparence des erreurs

**Engagement :** En cas d'erreur, le produit reçoit une information claire et actionnable.

**Informations fournies :**
- Type d'erreur (validation, autorité, système)
- Message compréhensible
- Identifiant de corrélation pour support

**Informations non fournies :**
- Stack traces internes
- Détails d'implémentation
- Informations d'autres produits

**Mesure :** Revue des messages d'erreur par UX.

---

### 7.5 GAR-PROD-05 : Traçabilité accessible

**Engagement :** Un produit peut obtenir l'historique de ses propres interactions.

**Accès fourni :**
- Liste des intentions soumises
- Résultats obtenus
- Erreurs rencontrées

**Accès non fourni :**
- Interactions d'autres produits
- Détails internes du traitement
- Réponses brutes des autorités

**Mesure :** API de consultation du journal avec filtrage par produit.

---

## 8. Garanties envers les autorités

Ces garanties sont les engagements de Bonding Brother envers Kind Mother et Strong Father.

### 8.1 GAR-AUTH-01 : Contexte complet

**Engagement :** Les autorités reçoivent toujours le contexte complet nécessaire à leur décision.

**Informations toujours transmises :**
- Identité du produit
- Identité de l'utilisateur
- Timestamp
- Permissions déclarées
- Environnement d'exécution

**Mesure :** Validation automatique de la complétude du contexte.

---

### 8.2 GAR-AUTH-02 : Demandes valides

**Engagement :** Les demandes transmises aux autorités sont structurellement valides.

**Validations effectuées :**
- Format correct
- Champs obligatoires présents
- Types de données corrects
- Contraintes de base respectées

**Implications :**
- Les autorités n'ont pas à gérer les erreurs de format
- Les autorités peuvent se concentrer sur la décision métier

**Mesure :** Zéro rejet pour erreur de format côté autorité.

---

### 8.3 GAR-AUTH-03 : Transmission fidèle

**Engagement :** Les réponses des autorités sont transmises fidèlement aux produits.

**Implications :**
- Le sens de la décision est préservé
- Les restrictions sont respectées
- Les autorisations ne sont pas étendues

**Mesure :** Comparaison automatisée décision/résultat transmis.

---

### 8.4 GAR-AUTH-04 : Isolation des produits

**Engagement :** Les autorités ne reçoivent que les informations nécessaires, sans pollution inter-produits.

**Implications :**
- Pas de mélange de contextes
- Pas de transmission d'informations d'autres produits
- Isolation complète des sessions

**Mesure :** Tests d'isolation et audits de sécurité.

---

## 9. Mécanismes de vérification

### 9.1 Vérification statique (au build)

| Invariant | Mécanisme | Fréquence |
|-----------|-----------|-----------|
| INV-NAT-* | Revue architecturale | Chaque PR |
| INV-NEG-01 | Analyse de code (pas de logique métier) | CI |
| INV-NEG-02 | Audit des structures de données | CI |
| INV-NEG-03 | Vérification des sources de règles | CI |

### 9.2 Vérification dynamique (au runtime)

| Invariant | Mécanisme | Fréquence |
|-----------|-----------|-----------|
| INV-FLUX-01 | Trace de chaque étape | Temps réel |
| INV-FLUX-02 | Vérification de présence dans journal | Temps réel |
| INV-FLUX-03 | Comparaison de timestamps | Temps réel |
| INV-FLUX-04 | Réconciliation intention/résultat | Batch |

### 9.3 Vérification périodique (audits)

| Garantie | Mécanisme | Fréquence |
|----------|-----------|-----------|
| GAR-PROD-01 | Tests de compatibilité | Release |
| GAR-PROD-03 | Tests de pénétration | Mensuel |
| GAR-AUTH-02 | Analyse des rejets autorité | Hebdomadaire |

---

## 10. Matrice de couverture

Cette matrice montre quels composants sont concernés par chaque invariant.

| Invariant | ProductGateway | Translator | FilterEngine | Adapter | Journal |
|-----------|----------------|------------|--------------|---------|---------|
| INV-NAT-01 | ✓ | ✓ | ✓ | ✓ | ✓ |
| INV-NAT-02 | - | ✓ | - | - | - |
| INV-NAT-03 | - | - | - | ✓ | - |
| INV-NEG-01 | ✓ | - | ✓ | - | - |
| INV-NEG-02 | - | - | - | - | ✓ |
| INV-NEG-03 | - | - | ✓ | - | - |
| INV-NEG-04 | ✓ | - | - | ✓ | - |
| INV-NEG-05 | - | ✓ | ✓ | - | - |
| INV-NEG-06 | ✓ | - | - | ✓ | - |
| INV-FLUX-01 | ✓ | ✓ | ✓ | ✓ | ✓ |
| INV-FLUX-02 | - | - | - | - | ✓ |
| INV-FLUX-03 | ✓ | - | - | - | - |
| INV-FLUX-04 | ✓ | - | - | - | ✓ |

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut INVARIANTS**. Il établit les propriétés non négociables de Bonding Brother qui doivent être vraies en toutes circonstances.

Toute implémentation de Bonding Brother doit garantir ces invariants. Toute violation est considérée comme un défaut critique. Toute modification des invariants nécessite une nouvelle version majeure et une revue architecturale complète.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** INVARIANTS — Non négociable  
**Dépendance :** Documentation Fondatrice v1.0 (Section 10)
