# BondingBrother - Product-to-Ecosystem Flow

## 1. Contexte

Ce document définit le flux contractuel détaillé des intentions depuis les produits vers l'écosystème via Bonding Brother. Il spécifie les étapes précises, les transformations, les validations, et les garanties associées au flux Produit → Écosystème.

Ce document complète la Section 5 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour la vue d'ensemble, l'[Intent Model Contract](./BondingBrother%20-%20Intent%20Model%20Contract.md) pour la structure des intentions, et le [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md) pour les règles de traduction.

Ce flux respecte les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : il fonctionne même en mode offline (**LOI-2**), et les intentions sont préservées localement même sans connexion aux autorités (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- Le flux complet Produit → Écosystème (étape par étape)
- Les transformations appliquées à chaque étape
- Les validations et vérifications effectuées
- Les règles de routage vers les autorités
- Les garanties de traitement
- Les cas d'erreur et leur gestion

Ce document **ne couvre pas** :
- Le flux inverse Écosystème → Produit (voir Ecosystem-to-Product Flow)
- Les détails de traduction (voir Translation Contract)
- Les règles de filtrage (voir Filtering & Projection Contract)
- La gestion des erreurs (voir Error & Rejection Model)
- Les protocoles d'intégration avec les autorités (voir les contrats d'intégration)

---

## 3. Principe fondamental

**Le flux Produit → Écosystème est unidirectionnel, asymétrique, et toujours adaptatif.**

Les produits expriment des intentions dans leur vocabulaire. Bonding Brother adapte ces intentions au vocabulaire et aux contraintes des autorités, sans jamais demander aux autorités de s'adapter aux produits.

---

## 4. Vue d'ensemble du flux

Le flux Produit → Écosystème traverse les étapes suivantes dans l'ordre strict :

```
PRODUIT
  │
  ▼
[1] Réception de l'intention
  │
  ▼
[2] Validation structurelle
  │
  ▼
[3] Traduction intention → demande
  │
  ▼
[4] Filtrage d'entrée
  │
  ▼
[5] Journalisation
  │
  ▼
[6] Routage vers autorité
  │
  ▼
[7] Transmission à l'autorité
  │
  ▼
[8] Attente de réponse
  │
  ▼
[9] Réception de la réponse
  │
  ▼
[10] Traduction réponse → résultat
  │
  ▼
[11] Filtrage de sortie
  │
  ▼
[12] Transmission du résultat au produit
  │
  ▼
PRODUIT
```

---

## 5. Étapes détaillées

### 5.1 Étape 1 : Réception de l'intention

**Déclencheur :** Le produit soumet une intention à Bonding Brother via l'interface `IIntentSubmission`.

**Action :** Bonding Brother reçoit l'intention dans le format et le vocabulaire du produit.

**Validation :** Aucune validation à ce stade, uniquement réception.

**Résultat :** Intention reçue, état `CRÉÉE`.

**Règle REC-01 : Réception immédiate**

Bonding Brother accepte immédiatement toute intention structurellement valide (JSON valide), même si la validation sémantique échoue plus tard.

**Règle REC-02 : Pas de rejet précoce**

Aucun rejet n'est effectué à cette étape, sauf si l'intention n'est pas un JSON valide.

---

### 5.2 Étape 2 : Validation structurelle

**Déclencheur :** Intention reçue et parsée.

**Action :** Bonding Brother valide la structure de l'intention selon le schéma défini dans l'Intent Model Contract.

**Validations effectuées :**
- Format JSON valide
- Présence des champs obligatoires (`id`, `produit_id`, `type`, `payload`, `contexte`, `timestamp`, `version`)
- Types de données conformes
- Version du schéma supportée
- Type d'intention reconnu

**Résultat :**
- Si validation réussie : État `VALIDÉE`, passage à l'étape suivante
- Si validation échoue : État `REJETÉE`, transmission d'un résultat d'erreur au produit

**Règle VAL-01 : Validation stricte**

Toute intention non conforme est rejetée immédiatement, sans tentative de correction ou d'inférence.

**Règle VAL-02 : Pas de validation métier**

Bonding Brother ne valide pas le contenu métier du payload. Cette validation appartient aux autorités.

---

### 5.3 Étape 3 : Traduction intention → demande

**Déclencheur :** Intention validée structurellement.

**Action :** Bonding Brother traduit l'intention (vocabulaire produit) en demande (vocabulaire autorité) selon les règles du Translation Contract.

**Transformations appliquées :**
- Mapping du type d'intention vers le type de demande
- Traduction champ par champ du payload
- Préservation intégrale du contexte
- Ajout de métadonnées techniques (intention_id, timestamp_demande)

**Résultat :**
- Si traduction réussie : Demande créée, état `TRADUITE`
- Si traduction échoue : État `REJETÉE`, transmission d'un résultat d'erreur au produit

**Règle TRAD-01 : Fidélité sémantique**

La traduction préserve intégralement la sémantique de l'intention. Aucune interprétation ni enrichissement métier n'est autorisé.

**Règle TRAD-02 : Déterminisme**

Pour une même intention, la traduction produit toujours la même demande.

**Règle TRAD-03 : Identification de l'autorité**

La traduction identifie l'autorité cible (Kind Mother ou Strong Father) en fonction du type d'intention.

---

### 5.4 Étape 4 : Filtrage d'entrée

**Déclencheur :** Demande traduite et prête.

**Action :** Bonding Brother applique les règles de filtrage d'entrée définies dans le Filtering & Projection Contract.

**Filtrages appliqués :**
- Rejet des demandes manifestement invalides
- Vérification des contraintes pré-transmission
- Application des règles de sécurité d'entrée

**Résultat :**
- Si filtrage accepte : État `FILTRÉE`, passage à l'étape suivante
- Si filtrage rejette : État `REJETÉE`, transmission d'un résultat d'erreur au produit

**Règle FILT-01 : Filtrage préventif**

Le filtrage d'entrée protège les autorités en rejetant les demandes invalides avant transmission.

**Règle FILT-02 : Pas de décision métier**

Le filtrage ne prend pas de décision métier. Il applique uniquement des règles techniques et de sécurité.

---

### 5.5 Étape 5 : Journalisation

**Déclencheur :** Demande filtrée et prête pour transmission.

**Action :** Bonding Brother journalise l'intention complète dans le journal d'audit.

**Informations journalisées :**
- Intention complète (structure + payload)
- Contexte complet
- Timestamp de réception
- Identité du produit
- État actuel (`JOURNALISÉE`)

**Résultat :** Intention journalisée, état `JOURNALISÉE`, passage à l'étape suivante.

**Règle JOUR-01 : Journalisation systématique**

Toute intention qui atteint cette étape est journalisée, sans exception.

**Règle JOUR-02 : Immuabilité**

Une fois journalisée, l'intention ne peut être modifiée. Les corrections se font par nouvelle intention.

**Règle JOUR-03 : Traçabilité complète**

Le journal permet de tracer l'intention complète depuis sa réception jusqu'à sa résolution.

---

### 5.6 Étape 6 : Routage vers autorité

**Déclencheur :** Intention journalisée et prête pour transmission.

**Action :** Bonding Brother détermine l'autorité cible et route la demande vers l'adaptateur approprié.

**Règles de routage :**
- Intentions de type données (CREATE_CONTENT, UPDATE_CONTENT, etc.) → Kind Mother
- Intentions de type hiérarchie (CREATE_NODE, MOVE_NODE, etc.) → Kind Mother
- Intentions de type identité (AUTHENTICATE, AUTHORIZE, etc.) → Strong Father
- Intentions de type session (CREATE_SESSION, REVOKE_SESSION) → Strong Father

**Résultat :** Autorité identifiée, adaptateur sélectionné, passage à l'étape suivante.

**Règle ROUT-01 : Routage déterministe**

Le routage est déterministe : un type d'intention mappe toujours vers la même autorité.

**Règle ROUT-02 : Pas d'intentions multi-autorités**

Une intention ne peut cibler qu'une seule autorité. Les intentions multi-autorités sont interdites.

---

### 5.7 Étape 7 : Transmission à l'autorité

**Déclencheur :** Autorité identifiée et adaptateur sélectionné.

**Action :** Bonding Brother transmet la demande à l'autorité via l'adaptateur approprié (KindMotherAdapter ou StrongFatherAdapter).

**Transmission :**
- Format : Demande traduite dans le vocabulaire de l'autorité
- Contexte : Contexte complet préservé
- Métadonnées : Métadonnées techniques ajoutées

**Résultat :**
- Si transmission réussie : État `TRANSMISE`, passage à l'étape suivante
- Si transmission échoue (offline) : État `EN_ERREUR`, mise en buffer pour retry ultérieur

**Règle TRANS-01 : Transmission fidèle**

La demande est transmise intégralement, sans modification ni interprétation.

**Règle TRANS-02 : Gestion offline**

En cas d'indisponibilité de l'autorité, l'intention est mise en buffer et retentée lors de la reconnexion.

**Règle TRANS-03 : Pas de modification**

Bonding Brother ne modifie jamais la demande avant transmission. Toute adaptation a été faite lors de la traduction.

---

### 5.8 Étape 8 : Attente de réponse

**Déclencheur :** Demande transmise avec succès à l'autorité.

**Action :** Bonding Brother attend la réponse de l'autorité.

**Caractéristiques :**
- État : `EN_ATTENTE`
- Timeout : Configurable par intention ou par défaut
- Mode asynchrone : Bonding Brother peut traiter d'autres intentions pendant l'attente

**Résultat :**
- Si réponse reçue : Passage à l'étape suivante
- Si timeout : État `ABANDONNÉE`, transmission d'un résultat d'erreur au produit

**Règle ATT-01 : Pas d'interruption**

Bonding Brother n'interrompt jamais l'autorité. Il attend patiemment la réponse.

**Règle ATT-02 : Timeout configurable**

Chaque intention peut spécifier un timeout. Si non spécifié, le timeout par défaut s'applique.

**Règle ATT-03 : Mode asynchrone**

L'attente est asynchrone. Bonding Brother continue de traiter d'autres intentions pendant l'attente.

---

### 5.9 Étape 9 : Réception de la réponse

**Déclencheur :** Autorité a fourni une réponse (acceptée, refusée, ou erreur).

**Action :** Bonding Brother reçoit la réponse de l'autorité dans son vocabulaire natif.

**Contenu de la réponse :**
- Statut : Acceptée, refusée, ou erreur
- Données : Données retournées (si applicable)
- Erreurs : Messages d'erreur (si applicable)
- Métadonnées : Métadonnées de l'autorité

**Résultat :** Réponse reçue, état `ÉVALUÉE`, passage à l'étape suivante.

**Règle RECP-01 : Préservation intégrale**

La réponse de l'autorité est préservée intégralement, sans modification ni interprétation.

**Règle RECP-02 : Pas de validation**

Bonding Brother ne valide pas la réponse de l'autorité. Il la transmet telle quelle (après traduction).

---

### 5.10 Étape 10 : Traduction réponse → résultat

**Déclencheur :** Réponse reçue de l'autorité.

**Action :** Bonding Brother traduit la réponse (vocabulaire autorité) en résultat (vocabulaire produit) selon les règles du Translation Contract.

**Transformations appliquées :**
- Mapping du statut vers le vocabulaire produit
- Traduction champ par champ des données
- Traduction des erreurs dans le vocabulaire produit
- Préservation de la décision de l'autorité

**Résultat :** Résultat traduit, prêt pour filtrage.

**Règle TRAD-R-01 : Préservation de la décision**

La décision de l'autorité (acceptée, refusée, erreur) est préservée intégralement. Aucune modification n'est autorisée.

**Règle TRAD-R-02 : Fidélité sémantique**

La traduction préserve la sémantique de la réponse. Les données sont traduites, pas interprétées.

---

### 5.11 Étape 11 : Filtrage de sortie

**Déclencheur :** Résultat traduit et prêt.

**Action :** Bonding Brother applique les règles de filtrage de sortie définies dans le Filtering & Projection Contract.

**Filtrages appliqués :**
- Suppression des informations sensibles non autorisées
- Adaptation des données selon les permissions du produit
- Projection des champs nécessaires uniquement

**Résultat :** Résultat filtré, prêt pour transmission au produit.

**Règle FILT-S-01 : Filtrage protecteur**

Le filtrage de sortie protège les autorités en ne transmettant que les informations autorisées.

**Règle FILT-S-02 : Respect des permissions**

Le filtrage respecte les permissions du produit. Les informations non autorisées sont omises.

---

### 5.12 Étape 12 : Transmission du résultat au produit

**Déclencheur :** Résultat filtré et prêt.

**Action :** Bonding Brother transmet le résultat au produit via l'interface `IResultConsumption`.

**Contenu transmis :**
- Statut : SUCCÈS, REFUSÉ, ou ERREUR
- Données : Données filtrées (si applicable)
- Erreurs : Messages d'erreur traduits (si applicable)
- Métadonnées : Métadonnées de traçabilité

**Résultat :** Résultat transmis, état `RÉSOLUE`, cycle complet terminé.

**Règle TRANS-R-01 : Transmission complète**

Le résultat est transmis intégralement au produit, sans modification supplémentaire.

**Règle TRANS-R-02 : Journalisation finale**

La transmission du résultat est journalisée pour compléter la traçabilité.

---

## 6. Garanties du flux

### 6.1 Garantie d'ordre

**Engagement :** Les étapes du flux sont exécutées dans l'ordre strict défini. Aucune étape ne peut être sautée ou réordonnée.

**Exception :** En cas d'erreur, le flux peut être interrompu et un résultat d'erreur peut être transmis au produit.

### 6.2 Garantie de traçabilité

**Engagement :** Toute intention qui traverse le flux est traçable de bout en bout. Le journal contient toutes les informations nécessaires pour reconstruire le flux complet.

### 6.3 Garantie de fidélité

**Engagement :** La sémantique de l'intention est préservée lors de la traduction et de la transmission. La décision de l'autorité est transmise fidèlement au produit.

### 6.4 Garantie de non-modification

**Engagement :** Bonding Brother ne modifie jamais la décision de l'autorité. Il transmet fidèlement ce que l'autorité a décidé.

---

## 7. Gestion des erreurs

### 7.1 Points d'échec

Le flux peut échouer aux étapes suivantes :
- **Étape 2** : Validation structurelle échouée → Rejet immédiat
- **Étape 3** : Traduction échouée → Rejet immédiat
- **Étape 4** : Filtrage d'entrée rejeté → Rejet immédiat
- **Étape 7** : Transmission échouée → Mise en buffer (mode offline)
- **Étape 8** : Timeout → Abandon
- **Étape 9** : Réponse d'erreur de l'autorité → Transmission de l'erreur au produit

### 7.2 Traitement des erreurs

**Règle ERR-01 : Notification immédiate**

Toute erreur détectée est notifiée immédiatement au produit via un résultat d'erreur.

**Règle ERR-02 : Journalisation des erreurs**

Toutes les erreurs sont journalisées pour audit et analyse.

**Règle ERR-03 : Pas de retry automatique**

Les erreurs de validation, traduction, ou filtrage ne sont pas retentées automatiquement (ce ne sont pas des erreurs transitoires).

**Règle ERR-04 : Retry pour erreurs de transmission**

Les erreurs de transmission sont retentées lors de la reconnexion (mode offline).

---

## 8. Mode offline

### 8.1 Comportement en mode offline

En mode offline, les étapes 7 à 9 peuvent être différées :

- **Étape 7** : La transmission est mise en buffer
- **Étape 8** : L'attente est différée jusqu'à la reconnexion
- **Étape 9** : La réception est différée jusqu'à la reconnexion

Les étapes 1 à 6 et 10 à 12 continuent de fonctionner normalement.

### 8.2 Synchronisation à la reconnexion

Lors de la reconnexion, Bonding Brother :
1. Transmet toutes les intentions en buffer
2. Attend les réponses
3. Transmet les résultats aux produits

Voir Sync & Reconnection Contract pour les détails.

---

## 9. Performance et limites

### 9.1 Délais

**Délai de traitement :** Le délai total dépend de :
- Temps de validation (instantané)
- Temps de traduction (instantané)
- Temps de filtrage (instantané)
- Temps de transmission à l'autorité (variable)
- Temps d'évaluation par l'autorité (variable)
- Temps de traduction de la réponse (instantané)
- Temps de filtrage de sortie (instantané)

**Timeout par défaut :** 30 secondes (configurable)

### 9.2 Limites

**Taille maximale d'intention :** 1 MB (configurable)
**Taille maximale de contexte :** 100 KB (configurable)
**Nombre d'intentions en attente :** Illimité (sous réserve de ressources)

---

## 10. Exemples

### 10.1 Flux complet : Création de contenu

```
1. Produit soumet intention CREATE_CONTENT
2. Validation structurelle : ✅
3. Traduction : CREATE_CONTENT → create_content (Kind Mother)
4. Filtrage d'entrée : ✅
5. Journalisation : ✅
6. Routage : → Kind Mother
7. Transmission : ✅
8. Attente : 2 secondes
9. Réception : Acceptée, content_id = "content-123"
10. Traduction : Acceptée → SUCCÈS, id = "content-123"
11. Filtrage de sortie : ✅
12. Transmission au produit : ✅
```

### 10.2 Flux avec erreur : Validation échouée

```
1. Produit soumet intention CREATE_CONTENT (champ obligatoire manquant)
2. Validation structurelle : ❌ (champ "payload.titre" manquant)
→ Rejet immédiat, résultat d'erreur transmis au produit
```

### 10.3 Flux avec erreur : Autorité refuse

```
1-7. (identique à l'exemple 10.1)
8. Attente : 1 seconde
9. Réception : Refusée (permission insuffisante)
10. Traduction : Refusée → REFUSÉ
11. Filtrage de sortie : ✅
12. Transmission au produit : REFUSÉ avec message d'erreur
```

---

## 11. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le flux détaillé que Bonding Brother doit respecter pour traiter les intentions des produits vers l'écosystème.

Toute implémentation du flux Produit → Écosystème doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 5)
- Bilateral Flow Contract v1.0
- Intent Model Contract v1.0
- Translation Contract v1.0
- Filtering & Projection Contract v1.0
- Error & Rejection Model v1.0
