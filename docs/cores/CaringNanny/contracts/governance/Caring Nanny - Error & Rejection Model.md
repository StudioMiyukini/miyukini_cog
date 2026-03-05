# Caring Nanny - Error & Rejection Model

## 1. Contexte

Ce document dÃ©finit le **modÃ¨le d'erreur et de rejet** de Caring Nanny dans le Miyukini Core System. Caring Nanny Ã©tant un **observateur pur** (INV-CN-1), son modÃ¨le d'erreur est fondamentalement diffÃ©rent de celui des autres cores : Caring Nanny ne rejette jamais d'opÃ©rations, ne bloque jamais le systÃ¨me, et ne prend jamais de dÃ©cision corrective.

**Question fondamentale :**

> *"Comment Caring Nanny gÃ¨re-t-elle les erreurs tout en restant un observateur passif qui ne bloque jamais ?"*

Ce document rÃ©pond Ã  cette question en dÃ©finissant :
- Les catÃ©gories d'erreurs que Caring Nanny peut rencontrer
- Les conditions d'invaliditÃ© dans le processus d'observation
- Les stratÃ©gies de dÃ©gradation gracieuse
- Les garanties de continuitÃ© mÃªme en cas d'erreur

**RÃ©fÃ©rences normatives :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) â€” Invariants fondateurs (Section 7)
- [Caring Nanny - Invariants et Garanties](./Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) â€” Garanties contractuelles
- [Caring Nanny - Violations & Anti-Patterns](./Caring%20Nanny%20-%20Violations%20%26%20Anti-Patterns.md) â€” Violations Ã  Ã©viter
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) â€” LOI-1 Ã  LOI-6

## 2. PortÃ©e / Scope

Ce document couvre :
- Le modÃ¨le d'erreur interne au processus d'observation
- Les conditions d'invaliditÃ© des observations
- Les erreurs dans le flux de propagation
- Les stratÃ©gies de rÃ©ponse qui prÃ©servent l'invariant non-bloquant (INV-CN-6)
- La classification des Ã©tats d'erreur du point de vue de l'observateur

Ce document **ne couvre pas** :
- Les erreurs des composants observÃ©s (elles sont des faits Ã  observer, pas des erreurs de Caring Nanny)
- Les violations d'invariants (voir Violations & Anti-Patterns)
- Les stratÃ©gies de correction (Caring Nanny ne corrige jamais â€” INV-NEG-CN-03)
- Les erreurs de dÃ©cision (responsabilitÃ© de StrongFather)

---

## 3. Principes fondamentaux du modÃ¨le d'erreur

### 3.1 Caring Nanny ne bloque jamais

**Invariant absolu (INV-CN-6) :** Caring Nanny ne bloque **jamais** les opÃ©rations du systÃ¨me. MÃªme en cas d'erreur interne, le systÃ¨me continue de fonctionner.

**ConsÃ©quences pour le modÃ¨le d'erreur :**
- Aucune erreur ne peut arrÃªter le flux d'observation
- Aucune erreur ne peut empÃªcher une rÃ©ponse Ã  une consultation
- Aucune erreur ne peut bloquer une propagation
- Les erreurs sont enregistrÃ©es, rapportÃ©es, mais jamais bloquantes

### 3.2 Caring Nanny ne rejette pas au sens traditionnel

Le terme "rejet" dans ce contexte est diffÃ©rent du rejet de StrongFather :

| Core | Nature du rejet |
|------|-----------------|
| **StrongFather** | Refuse une intention, bloque une action |
| **KindMother** | Refuse une Ã©criture non conforme |
| **Caring Nanny** | **N'existe pas** â€” Caring Nanny accepte tout ce qu'elle peut observer |

Pour Caring Nanny, "rejet" signifie :
- ImpossibilitÃ© de traiter une observation (mais observation enregistrÃ©e quand mÃªme)
- DÃ©tection d'une condition invalide (mais condition rapportÃ©e quand mÃªme)
- IncapacitÃ© Ã  classifier un Ã©tat (mais Ã©tat "unknown" retournÃ© quand mÃªme)

### 3.3 DÃ©gradation gracieuse obligatoire

Tout scÃ©nario d'erreur doit aboutir Ã  un Ã©tat dÃ©gradÃ© documentÃ©, jamais Ã  un blocage :

```
Erreur dÃ©tectÃ©e â†’ StratÃ©gie de dÃ©gradation â†’ RÃ©ponse dÃ©gradÃ©e
                                           â†’ Enregistrement de l'erreur
                                           â†’ Propagation si pertinente
```

---

## 4. Taxonomie des erreurs

### 4.1 Classification par origine

| Code | Origine | Description |
|------|---------|-------------|
| **ERR-OBS** | Observation | Erreur dans le processus de dÃ©tection d'une condition |
| **ERR-CLS** | Classification | Erreur dans la catÃ©gorisation d'un Ã©tat |
| **ERR-AGR** | AgrÃ©gation | Erreur dans l'agrÃ©gation des Ã©tats partiels |
| **ERR-PRO** | Propagation | Erreur dans la transmission d'un changement d'Ã©tat |
| **ERR-HIS** | Historisation | Erreur dans l'enregistrement de l'historique |
| **ERR-CON** | Consultation | Erreur dans la rÃ©ponse Ã  une demande d'Ã©tat |
| **ERR-INT** | Interne | Erreur technique interne Ã  Caring Nanny |

### 4.2 Classification par sÃ©vÃ©ritÃ©

| Niveau | Code | Description | Impact |
|--------|------|-------------|--------|
| **Critique** | SEV-CRIT | Erreur compromettant l'intÃ©gritÃ© de l'observation | DÃ©gradation majeure, alerte immÃ©diate |
| **Majeure** | SEV-MAJ | Erreur affectant la qualitÃ© de l'observation | DÃ©gradation partielle, alerte |
| **Mineure** | SEV-MIN | Erreur sans impact significatif | Enregistrement, pas d'alerte |
| **Info** | SEV-INFO | Condition anormale non bloquante | Enregistrement seul |

### 4.3 Classification par rÃ©cupÃ©rabilitÃ©

| Type | Description | StratÃ©gie |
|------|-------------|-----------|
| **Transitoire** | Erreur temporaire, rÃ©solution automatique probable | Retry avec backoff, fallback temporaire |
| **Permanente** | Erreur persistante jusqu'Ã  intervention | DÃ©gradation stable, alerte pour intervention |
| **Intermittente** | Erreur qui apparaÃ®t et disparaÃ®t | Historisation du pattern, dÃ©gradation adaptative |

---

## 5. Erreurs d'observation (ERR-OBS)

### 5.1 ERR-OBS-001 : Source d'observation indisponible

**Description :** Le composant Ã  observer ne rÃ©pond pas ou n'est pas accessible.

**Causes possibles :**
- Composant en cours de dÃ©marrage
- Composant dÃ©faillant
- Connexion locale rompue
- Ressource temporairement verrouillÃ©e

**RÃ©ponse NON-BLOQUANTE :**
1. Retourner l'Ã©tat `unknown` pour ce composant
2. Enregistrer la tentative d'observation avec l'Ã©chec
3. Propager l'incertitude si pertinent
4. Planifier une nouvelle tentative (sans blocage)

**Ce que Caring Nanny NE fait PAS :**
- âŒ Bloquer en attente de la source
- âŒ Retry infini bloquant
- âŒ DÃ©clencher une action corrective
- âŒ Ignorer silencieusement l'Ã©chec

### 5.2 ERR-OBS-002 : Format d'observation invalide

**Description :** Les donnÃ©es reÃ§ues de la source ne correspondent pas au format attendu.

**Causes possibles :**
- Version incompatible du composant observÃ©
- DonnÃ©es corrompues
- RÃ©ponse partielle
- Erreur de sÃ©rialisation

**RÃ©ponse NON-BLOQUANTE :**
1. Enregistrer le format invalide reÃ§u (pour diagnostic)
2. Tenter un parsing partiel si possible
3. Retourner `unknown` avec indication "format_invalid"
4. Journaliser avec sÃ©vÃ©ritÃ© SEV-MAJ

### 5.3 ERR-OBS-003 : Observation incomplÃ¨te

**Description :** L'observation a rÃ©ussi partiellement mais des informations manquent.

**Causes possibles :**
- Timeout partiel
- Composant en Ã©tat transitoire
- RÃ©ponse tronquÃ©e

**RÃ©ponse NON-BLOQUANTE :**
1. Utiliser les informations disponibles
2. Marquer l'observation comme "partielle"
3. ComplÃ©ter avec le dernier Ã©tat connu si disponible
4. Signaler l'incomplÃ©tude dans le contexte

---

## 6. Erreurs de classification (ERR-CLS)

### 6.1 ERR-CLS-001 : Condition non classifiable

**Description :** Une condition observÃ©e ne correspond Ã  aucun critÃ¨re de classification connu.

**Causes possibles :**
- Nouveau type de condition non prÃ©vu
- Combinaison de conditions inÃ©dite
- RÃ¨gles de classification incomplÃ¨tes

**RÃ©ponse NON-BLOQUANTE :**
1. Classifier comme `unknown` avec sous-catÃ©gorie "unclassified"
2. Enregistrer la condition brute complÃ¨te
3. Propager avec indication "requires_classification_update"
4. **Ne pas inventer** une classification

### 6.2 ERR-CLS-002 : Classification ambiguÃ«

**Description :** Une condition correspond Ã  plusieurs catÃ©gories mutuellement exclusives.

**Causes possibles :**
- RÃ¨gles de classification avec chevauchement
- Condition Ã  la frontiÃ¨re entre deux Ã©tats
- DonnÃ©es contradictoires dans la condition

**RÃ©ponse NON-BLOQUANTE :**
1. Appliquer la rÃ¨gle de prioritÃ© : `error > degraded > syncing > offline > healthy`
2. Enregistrer l'ambiguÃ¯tÃ© avec toutes les catÃ©gories candidates
3. Propager l'Ã©tat de prioritÃ© maximale
4. Journaliser pour revue des rÃ¨gles

### 6.3 ERR-CLS-003 : CritÃ¨res de classification indisponibles

**Description :** Les rÃ¨gles de classification ne sont pas chargÃ©es ou sont corrompues.

**Causes possibles :**
- Erreur au dÃ©marrage
- Configuration corrompue
- RÃ¨gles non trouvÃ©es

**RÃ©ponse NON-BLOQUANTE (Critique) :**
1. Activer le mode dÃ©gradÃ© : classification minimale (healthy/error/unknown)
2. Journaliser avec sÃ©vÃ©ritÃ© SEV-CRIT
3. Retourner des Ã©tats avec indication "degraded_classification"
4. Tenter de recharger les rÃ¨gles pÃ©riodiquement (sans blocage)

---

## 7. Erreurs d'agrÃ©gation (ERR-AGR)

### 7.1 ERR-AGR-001 : Ã‰tats partiels incohÃ©rents

**Description :** Les Ã©tats partiels des composants contiennent des contradictions lors de l'agrÃ©gation.

**Exemple :**
- Composant A rapporte que Composant B est `healthy`
- Observation directe de Composant B indique `error`

**RÃ©ponse NON-BLOQUANTE :**
1. PrioritÃ© Ã  l'observation directe sur les observations indirectes
2. Enregistrer l'incohÃ©rence avec les deux sources
3. AgrÃ©ger avec l'Ã©tat de prioritÃ© maximale (error > autres)
4. Propager l'Ã©tat avec indication "inconsistency_detected"

### 7.2 ERR-AGR-002 : Ã‰tat partiel manquant

**Description :** Un composant attendu n'a pas d'Ã©tat observable.

**RÃ©ponse NON-BLOQUANTE :**
1. ConsidÃ©rer le composant comme `unknown`
2. AgrÃ©ger les autres Ã©tats normalement
3. L'Ã©tat global reflÃ¨te l'incertitude si le composant est critique
4. Enregistrer le composant manquant

### 7.3 ERR-AGR-003 : Cycle de dÃ©pendance d'Ã©tat

**Description :** DÃ©tection d'une dÃ©pendance circulaire dans les Ã©tats des composants.

**Exemple :**
- Ã‰tat de A dÃ©pend de B, Ã©tat de B dÃ©pend de C, Ã©tat de C dÃ©pend de A

**RÃ©ponse NON-BLOQUANTE :**
1. Briser le cycle en utilisant les derniers Ã©tats connus
2. Enregistrer le cycle dÃ©tectÃ©
3. Journaliser avec SEV-MAJ (anomalie architecturale)
4. Ne pas tenter de rÃ©soudre le cycle (ce n'est pas une dÃ©cision)

---

## 8. Erreurs de propagation (ERR-PRO)

### 8.1 ERR-PRO-001 : BondingBrother indisponible

**Description :** Impossible de dÃ©lÃ©guer la propagation Ã  BondingBrother.

**Causes possibles :**
- BondingBrother non dÃ©marrÃ©
- Connexion locale rompue
- BondingBrother surchargÃ©

**RÃ©ponse NON-BLOQUANTE :**
1. Buffer local de la notification (avec limite)
2. Enregistrer la propagation comme "pending"
3. Retry pÃ©riodique (sans blocage)
4. Si buffer plein : prioritiser (error > degraded > autres), journaliser les propagations perdues

**ConformitÃ© LOI-1 :** Caring Nanny continue de fonctionner mÃªme si BondingBrother est indisponible.

### 8.2 ERR-PRO-002 : Destinataire inconnu

**Description :** Un destinataire de la propagation n'est pas identifiable.

**RÃ©ponse NON-BLOQUANTE :**
1. Propager aux destinataires connus
2. Enregistrer le destinataire inconnu
3. Ne pas bloquer la propagation pour les autres

### 8.3 ERR-PRO-003 : Propagation rejetÃ©e par BondingBrother

**Description :** BondingBrother refuse la propagation (format invalide, quota, etc.).

**RÃ©ponse NON-BLOQUANTE :**
1. Enregistrer le rejet avec la raison
2. Stocker temporairement pour analyse
3. Ne pas retry automatiquement sans correction
4. Journaliser avec SEV-MAJ

---

## 9. Erreurs d'historisation (ERR-HIS)

### 9.1 ERR-HIS-001 : Stockage d'historique saturÃ©

**Description :** L'espace allouÃ© Ã  l'historique est plein.

**RÃ©ponse NON-BLOQUANTE :**
1. Appliquer la politique de rÃ©tention (supprimer les plus anciens)
2. Si politique Ã©choue : mode dÃ©gradÃ© (historique rÃ©duit)
3. Journaliser avec SEV-MAJ
4. Continuer Ã  observer et Ã  propager

**ConformitÃ© LOI-5 :** L'historique a des limites prÃ©visibles.

### 9.2 ERR-HIS-002 : Corruption de l'historique

**Description :** Des entrÃ©es de l'historique sont corrompues ou incohÃ©rentes.

**RÃ©ponse NON-BLOQUANTE :**
1. Isoler les entrÃ©es corrompues
2. Continuer l'historisation des nouvelles observations
3. Marquer les entrÃ©es corrompues comme "corrupted"
4. Journaliser avec SEV-CRIT pour intervention

### 9.3 ERR-HIS-003 : Ã‰criture d'historique Ã©chouÃ©e

**Description :** Impossible d'Ã©crire une nouvelle entrÃ©e dans l'historique.

**RÃ©ponse NON-BLOQUANTE :**
1. Buffer temporaire en mÃ©moire
2. Retry d'Ã©criture pÃ©riodique
3. Si buffer saturÃ© : prioritiser les observations critiques
4. **Ne jamais** bloquer l'observation pour cause d'historisation

---

## 10. Erreurs de consultation (ERR-CON)

### 10.1 ERR-CON-001 : RequÃªte d'Ã©tat invalide

**Description :** Une demande d'Ã©tat ne peut pas Ãªtre interprÃ©tÃ©e.

**Exemples :**
- Identifiant de composant inconnu
- Format de requÃªte invalide
- ParamÃ¨tres manquants

**RÃ©ponse NON-BLOQUANTE :**
1. Retourner une rÃ©ponse d'erreur structurÃ©e (pas une exception)
2. Inclure l'Ã©tat global si le composant spÃ©cifique est inconnu
3. Enregistrer la requÃªte invalide
4. **Toujours** retourner une rÃ©ponse

**Format de rÃ©ponse d'erreur :**
```
{
  state: "unknown",
  error: {
    code: "ERR-CON-001",
    message: "Composant non trouvÃ©",
    requested_component: "xxx"
  },
  timestamp: <local_timestamp>,
  global_state: <Ã©tat_global_si_disponible>
}
```

### 10.2 ERR-CON-002 : Ã‰tat non disponible temporairement

**Description :** L'Ã©tat demandÃ© n'est pas encore calculÃ© ou est en cours de mise Ã  jour.

**RÃ©ponse NON-BLOQUANTE :**
1. Retourner le dernier Ã©tat connu
2. Inclure l'indication "stale" avec l'Ã¢ge de l'Ã©tat
3. **Ne jamais** bloquer en attente de l'Ã©tat frais

**Format de rÃ©ponse :**
```
{
  state: <dernier_Ã©tat_connu>,
  staleness: {
    is_stale: true,
    age_ms: <durÃ©e_depuis_derniÃ¨re_observation>,
    reason: "observation_in_progress"
  },
  timestamp: <timestamp_de_l'Ã©tat_retournÃ©>
}
```

---

## 11. Erreurs internes (ERR-INT)

### 11.1 ERR-INT-001 : Erreur technique non rÃ©cupÃ©rable

**Description :** Erreur interne qui ne peut pas Ãªtre gÃ©rÃ©e par les mÃ©canismes standard.

**Exemples :**
- Out of memory
- Stack overflow
- Corruption de l'Ã©tat interne

**RÃ©ponse :**
1. Mode de survie minimal : retourner `unknown` pour toutes les requÃªtes
2. Journaliser autant que possible
3. Alerter pour intervention urgente
4. **Ne pas tenter** de correction automatique (violation potentielle)

### 11.2 ERR-INT-002 : Configuration invalide

**Description :** La configuration de Caring Nanny est invalide ou incomplÃ¨te.

**RÃ©ponse NON-BLOQUANTE :**
1. Utiliser les valeurs par dÃ©faut quand possible
2. Mode dÃ©gradÃ© avec capacitÃ©s rÃ©duites
3. Journaliser les Ã©lÃ©ments de configuration manquants
4. Continuer Ã  fonctionner avec les capacitÃ©s disponibles

---

## 12. Ã‰tats d'erreur observÃ©s vs erreurs de Caring Nanny

### 12.1 Distinction fondamentale

Il est **critique** de distinguer :

| Type | Description | ResponsabilitÃ© de Caring Nanny |
|------|-------------|-------------------------------|
| **Ã‰tat d'erreur observÃ©** | Un composant EST en Ã©tat `error` | Observer, classifier, propager |
| **Erreur de Caring Nanny** | Caring Nanny a un problÃ¨me interne | GÃ©rer, dÃ©grader gracieusement, continuer |

**Exemple :**
- KindMother est en Ã©tat `error` â†’ Caring Nanny **observe** et **rapporte** l'Ã©tat `error`
- Caring Nanny ne peut pas contacter KindMother â†’ Caring Nanny **gÃ¨re** son erreur et retourne `unknown`

### 12.2 L'Ã©tat `error` n'est pas une erreur de Caring Nanny

Quand Caring Nanny rapporte l'Ã©tat `error` pour un composant :
- C'est une **observation correcte** d'un fait
- Ce n'est **pas** une erreur de Caring Nanny
- Caring Nanny a **rÃ©ussi** son travail d'observation

### 12.3 L'Ã©tat `unknown` signale une limite de Caring Nanny

Quand Caring Nanny retourne `unknown` :
- Caring Nanny reconnaÃ®t qu'elle **ne sait pas**
- C'est une rÃ©ponse **honnÃªte**, pas un Ã©chec
- Le consommateur peut prendre une dÃ©cision en connaissance de cause

---

## 13. Conditions de rejet (au sens Caring Nanny)

### 13.1 Ce que "rejet" signifie pour Caring Nanny

Caring Nanny ne "rejette" pas au sens traditionnel. Les conditions de rejet sont des situations oÃ¹ Caring Nanny **ne peut pas fournir** l'information demandÃ©e dans le format attendu.

### 13.2 Condition REJ-001 : Observation impossible

**Condition :** Caring Nanny ne peut absolument pas observer un composant.

**RÃ©ponse :** Ã‰tat `unknown` avec contexte `observation_impossible`

**Ce qui n'est PAS un rejet :**
- Le composant existe mais est en erreur â†’ observation rÃ©ussie (Ã©tat `error`)
- Le composant est lent Ã  rÃ©pondre â†’ observation avec timeout puis `unknown`

### 13.3 Condition REJ-002 : Classification impossible

**Condition :** Une condition ne peut Ãªtre classifiÃ©e mÃªme avec les rÃ¨gles de fallback.

**RÃ©ponse :** Ã‰tat `unknown` avec sous-catÃ©gorie `unclassifiable`

### 13.4 Condition REJ-003 : Propagation impossible

**Condition :** Impossible de propager mÃªme aprÃ¨s les stratÃ©gies de retry et de buffer.

**RÃ©ponse :** Propagation enregistrÃ©e comme `failed`, observation conservÃ©e dans l'historique.

---

## 14. DÃ©gradation gracieuse

### 14.1 Niveaux de dÃ©gradation

| Niveau | Description | CapacitÃ©s |
|--------|-------------|-----------|
| **Nominal** | Toutes fonctions opÃ©rationnelles | Observation, classification, agrÃ©gation, propagation, historisation |
| **DÃ©gradÃ© lÃ©ger** | Quelques fonctions limitÃ©es | Classification simplifiÃ©e, historique rÃ©duit |
| **DÃ©gradÃ© sÃ©vÃ¨re** | Fonctions minimales uniquement | Observation basique, Ã©tat `unknown` frÃ©quent |
| **Survie** | Mode minimal | Retourne `unknown` pour tout, journalise ce qui est possible |

### 14.2 Transitions de dÃ©gradation

Les transitions entre niveaux sont :
- **Automatiques** (vers plus dÃ©gradÃ©) : en rÃ©ponse aux erreurs
- **Automatiques** (vers moins dÃ©gradÃ©) : quand les conditions s'amÃ©liorent
- **Jamais bloquantes** : une transition ne peut pas bloquer le systÃ¨me

### 14.3 Signalement de la dÃ©gradation

Chaque rÃ©ponse de Caring Nanny inclut son niveau de dÃ©gradation :

```
{
  state: <Ã©tat_observÃ©>,
  degradation_level: "nominal" | "light" | "severe" | "survival",
  degradation_reasons: [<liste_des_raisons>]
}
```

---

## 15. ConformitÃ© aux Lois d'Autonomie

### 15.1 LOI-1 : Aucune dÃ©pendance externe critique

**ConformitÃ© :** Toutes les stratÃ©gies de gestion d'erreur fonctionnent sans dÃ©pendance externe.
- Les rÃ¨gles de classification sont locales
- L'historique est local
- La dÃ©gradation est gÃ©rÃ©e localement

### 15.2 LOI-2 : Isolement acceptÃ© comme Ã©tat normal

**ConformitÃ© :** Les erreurs de connexion ne sont pas traitÃ©es comme critiques.
- BondingBrother indisponible â†’ buffer local
- Source d'observation indisponible â†’ Ã©tat `unknown`
- Pas de retry infini bloquant

### 15.3 LOI-3 : Ã‰tat local souverain

**ConformitÃ© :** L'historique local est la source de vÃ©ritÃ©.
- Les erreurs sont enregistrÃ©es localement
- Pas de validation externe de l'Ã©tat

### 15.4 LOI-4 : Pas de temps global requis

**ConformitÃ© :** Toutes les timestamps sont locales.
- Les erreurs sont horodatÃ©es localement
- Pas de synchronisation temporelle requise

### 15.5 LOI-5 : CoÃ»t proportionnel au hardware

**ConformitÃ© :** Les stratÃ©gies de gestion d'erreur sont Ã©conomes.
- Buffer avec limites
- Pas de retry exponentiel infini
- Historique avec rÃ©tention

### 15.6 LOI-6 : Autonomie prÃ©servÃ©e avec fÃ©dÃ©ration

**ConformitÃ© :** Les erreurs de fÃ©dÃ©ration ne bloquent pas.
- Propagation vers nÅ“uds fÃ©dÃ©rÃ©s en best-effort
- Pas de dÃ©pendance au rÃ©seau de fÃ©dÃ©ration

---

## 16. ModÃ¨le de rÃ©ponse aux erreurs

### 16.1 Structure de rÃ©ponse unifiÃ©e

Toute rÃ©ponse de Caring Nanny, mÃªme en cas d'erreur, suit cette structure :

```
{
  // Toujours prÃ©sent
  timestamp: <timestamp_local>,
  degradation_level: "nominal" | "light" | "severe" | "survival",
  
  // Ã‰tat (toujours prÃ©sent, mÃªme si "unknown")
  state: "healthy" | "degraded" | "offline" | "syncing" | "error" | "unknown",
  
  // Contexte de l'Ã©tat
  state_context: {
    is_stale: <boolean>,
    age_ms: <durÃ©e_si_stale>,
    confidence: "high" | "medium" | "low" | "none"
  },
  
  // Erreurs rencontrÃ©es (optionnel)
  errors: [
    {
      code: "ERR-xxx-xxx",
      severity: "CRIT" | "MAJ" | "MIN" | "INFO",
      message: <description>,
      recoverable: <boolean>
    }
  ],
  
  // Composant spÃ©cifique (si demandÃ©)
  component: <identifiant_si_applicable>
}
```

### 16.2 Garantie de rÃ©ponse

**Invariant absolu :** Caring Nanny retourne **toujours** une rÃ©ponse conforme Ã  cette structure, mÃªme en cas d'erreur critique.

Cas extrÃªme (erreur non rÃ©cupÃ©rable) :
```
{
  timestamp: <timestamp_local>,
  degradation_level: "survival",
  state: "unknown",
  state_context: {
    is_stale: true,
    age_ms: null,
    confidence: "none"
  },
  errors: [
    {
      code: "ERR-INT-001",
      severity: "CRIT",
      message: "Erreur interne non rÃ©cupÃ©rable",
      recoverable: false
    }
  ]
}
```

---

## 17. Journalisation des erreurs

### 17.1 Format de journalisation

Toutes les erreurs sont journalisÃ©es avec :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `timestamp` | Horodatage local | Oui |
| `error_code` | Code de l'erreur (ERR-xxx-xxx) | Oui |
| `severity` | SÃ©vÃ©ritÃ© (CRIT/MAJ/MIN/INFO) | Oui |
| `context` | Contexte de l'erreur | Oui |
| `attempted_action` | Action qui a Ã©chouÃ© | Oui |
| `fallback_applied` | StratÃ©gie de dÃ©gradation appliquÃ©e | Oui |
| `outcome` | RÃ©sultat aprÃ¨s fallback | Oui |

### 17.2 RÃ©tention des journaux d'erreur

Les journaux d'erreur suivent une politique de rÃ©tention configurable :
- Erreurs CRIT : rÃ©tention maximale
- Erreurs MAJ : rÃ©tention standard
- Erreurs MIN/INFO : rÃ©tention courte ou agrÃ©gation

---

## 18. Matrice de correspondance erreurs/rÃ©ponses

| Code erreur | RÃ©ponse Ã©tat | DÃ©gradation | Action |
|-------------|--------------|-------------|--------|
| ERR-OBS-001 | `unknown` | light | Retry non-bloquant |
| ERR-OBS-002 | `unknown` | light | Journaliser |
| ERR-OBS-003 | `<partiel>` | light | ComplÃ©ter si possible |
| ERR-CLS-001 | `unknown` | light | Journaliser |
| ERR-CLS-002 | `<prioritÃ©_max>` | nominal | Journaliser ambiguÃ¯tÃ© |
| ERR-CLS-003 | `<classification_minimale>` | severe | Retry chargement |
| ERR-AGR-001 | `<prioritÃ©_max>` | light | Journaliser incohÃ©rence |
| ERR-AGR-002 | `<agrÃ©gation_partielle>` | light | Signaler manquant |
| ERR-AGR-003 | `<dernier_connu>` | light | Journaliser cycle |
| ERR-PRO-001 | N/A (propagation) | light | Buffer local |
| ERR-PRO-002 | N/A (propagation) | nominal | Propager aux autres |
| ERR-PRO-003 | N/A (propagation) | light | Journaliser |
| ERR-HIS-001 | N/A (historique) | light | Appliquer rÃ©tention |
| ERR-HIS-002 | N/A (historique) | severe | Isoler entrÃ©es |
| ERR-HIS-003 | N/A (historique) | light | Buffer mÃ©moire |
| ERR-CON-001 | `unknown` | nominal | Retourner erreur structurÃ©e |
| ERR-CON-002 | `<dernier_connu>` | nominal | Indiquer staleness |
| ERR-INT-001 | `unknown` | survival | Mode survie |
| ERR-INT-002 | `<selon_disponible>` | severe | Valeurs par dÃ©faut |

---

## 19. Statut contractuel

Ce document est **contractuel, normatif, et de statut GOUVERNANCE**. Il dÃ©finit le modÃ¨le d'erreur et de rejet de Caring Nanny, garantissant que Caring Nanny reste un observateur non-bloquant mÃªme en cas d'erreur.

Toute implÃ©mentation de Caring Nanny doit :
- ImplÃ©menter toutes les stratÃ©gies de dÃ©gradation gracieuse
- Garantir une rÃ©ponse pour toute consultation, mÃªme en cas d'erreur
- Ne jamais bloquer le systÃ¨me en raison d'une erreur interne
- Journaliser toutes les erreurs rencontrÃ©es

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** GOUVERNANCE â€” ModÃ¨le normatif  
**DÃ©pendances :**
- Caring Nanny - Documentation Fondatrice v1.6
- Caring Nanny - Invariants et Garanties v1.0
- Caring Nanny - Violations & Anti-Patterns v1.0
- Miyukini Conceptual References - Lois Autonomie Systeme v1.1

