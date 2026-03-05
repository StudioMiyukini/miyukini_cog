# TAMR â€” Error & Rejection Model

## 1. Introduction

### Contexte

TAMR (The Authority Must Rest) est le **Human Interaction Core** du Miyukini Core System. Il definit le cadre conceptuel de l'intervention humaine : ou, quand, et comment l'humain intervient. Ce contrat definit comment les erreurs et les rejets lies aux interventions humaines sont representes, communiques et traites.

### Objet du contrat

Ce document definit le **TAMR â€” Error & Rejection Model** : un contrat normatif, non negociable, et de statut FONDATION qui etablit le modele conceptuel des erreurs et des rejets dans le cadre des interventions humaines, definissant comment les erreurs sont representees, comment les rejets d'intervention sont communiques, les categories d'erreurs et de rejets, et les regles de gestion des situations exceptionnelles.

Ce contrat precise la nature des erreurs dans le cadre TAMR, la distinction entre erreur et rejet d'intervention, les categories de rejet, et les garanties associees.

### Portee / Scope

Ce contrat s'applique a **toutes les situations d'erreur et de rejet liees aux interventions humaines** definies ou encadrees par TAMR et definit de maniere absolue :

- la definition formelle d'une erreur dans le cadre TAMR,
- la distinction entre erreur et rejet d'intervention,
- les categories d'erreurs et de rejets d'intervention,
- la structure des messages d'erreur et de rejet,
- les regles de propagation,
- les invariants de gestion d'erreur.

### Statut contractuel

Ce document est **contractuel, normatif, non discutable, et de statut FONDATION**. Il etablit des regles absolues qui ne peuvent etre contournees, negociees, ou modifiees. Le contrat prime sur toute consideration pratique.

### Relation avec les autres contrats

Ce contrat complete et respecte les documents contractuels existants :

- **[TAMR â€” Documentation Fondatrice](../../foundation/TAMR%20-%20Documentation%20Fondatrice.md)** : Definition philosophique de TAMR, tracabilite et responsabilite
- **[TAMR â€” Intervention Types Contract](../intervention/TAMR%20-%20Intervention%20Types%20Contract.md)** : Types d'intervention (Approval, Override, Escalation, Supervision)
- **[TAMR â€” Intervention Points Contract](../intervention/TAMR%20-%20Intervention%20Points%20Contract.md)** : Points d'intervention, conditions, declencheurs
- **[TAMR â€” Authority Limits Contract](../boundaries/TAMR%20-%20Authority%20Limits%20Contract.md)** : Limites d'autorite humaine
- **[TAMR â€” Inviolable Limits Contract](../boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md)** : Limites infranchissables
- **[TAMR â€” Invariants & Guarantees](../governance/TAMR%20-%20Invariants%20%26%20Guarantees.md)** : INV-TAMR-1 a INV-TAMR-8
- **TAMR â€” Trace Contract** (audit) : Structure des traces d'intervention
- **[Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)** : Conformite LOI-1 a LOI-6 â€” les erreurs ne bloquent jamais le systeme en attente d'une ressource externe

Il n'introduit aucune contradiction, et constitue la definition formelle de la gestion des erreurs et rejets d'intervention dans TAMR.

---

## 2. Distinction erreur / rejet d'intervention

### 2.1. Definition d'une erreur (cadre TAMR)

Une **erreur** dans le cadre TAMR est une situation anormale qui empeche le fonctionnement correct du cadre d'intervention humaine. Une erreur represente un dysfonctionnement interne au cadre (tracabilite, point d'intervention, enregistrement), pas un resultat d'evaluation d'une intervention.

**Caracteristiques d'une erreur :**

- **Interne** : L'erreur provient du cadre TAMR ou de ses dependances (trace, point d'intervention), pas de l'intervention elle-meme
- **Inattendue** : L'erreur n'est pas un resultat d'evaluation previsible (refus par politique, limite depassee)
- **Bloquante** : L'erreur empeche l'enregistrement, la categorisation ou le traitement correct de l'intervention
- **Technique** : L'erreur concerne le fonctionnement du cadre (conceptuel dans ce contrat)

### 2.2. Definition d'un rejet d'intervention

Un **rejet d'intervention** dans TAMR est le resultat d'une evaluation qui determine qu'une intervention humaine ne peut pas etre acceptee ou executee. Un rejet est un resultat normal de l'evaluation (par StrongFather, par les limites TAMR, ou par les regles d'intervention), pas un dysfonctionnement.

**Caracteristiques d'un rejet d'intervention :**

- **Externe** : Le rejet est cause par l'intervention, son contexte, ou une decision (StrongFather, limites)
- **Previsible** : Le rejet est un resultat d'evaluation selon les politiques et limites TAMR
- **Non-bloquant** : Le rejet produit un resultat explicite (INTERVENTION_REFUSEE, INTERVENTION_HORS_LIMITES, etc.)
- **Conceptuel** : Le rejet concerne la validite de l'intervention au regard des regles et limites

### 2.3. Distinction fondamentale

| Aspect | Erreur | Rejet d'intervention |
|--------|--------|------------------------|
| Source | Interne au cadre TAMR (trace, point, enregistrement) | Externe (intervention, contexte, decision StrongFather, limites) |
| Nature | Dysfonctionnement | Resultat d'evaluation |
| Resultat | Pas de resultat d'intervention produit | Resultat produit (refusee, hors limites, invalide) |
| Traitement | Correction technique requise | Information a l'appelant, trace de la tentative |
| Tracabilite | Log d'erreur | Trace d'intervention (tentative refusee) â€” conforme INV-TAMR-1 |

---

## 3. Categories d'erreurs (cadre TAMR)

### 3.1. Erreurs de structure

**Definition :**

Les **erreurs de structure** sont des erreurs causees par une incoherence ou une malformation dans la definition des points d'intervention, des types d'intervention, ou des donnees requises pour une trace.

**Exemples conceptuels :**

- Point d'intervention malforme ou incoherent
- Type d'intervention non reconnu dans le contexte
- Structure de la trace d'intervention incomplete ou invalide
- Reference circulaire dans les definitions de points d'intervention

**Gravite :** Critique

**Consequence :** Arret du traitement de l'intervention, signalement d'erreur interne

### 3.2. Erreurs de tracabilite

**Definition :**

Les **erreurs de tracabilite** sont des erreurs causees par l'impossibilite d'enregistrer une trace d'intervention conforme a INV-TAMR-1 (tracabilite absolue).

**Exemples conceptuels :**

- Impossibilite d'enregistrer la trace (persistance indisponible, canal KindMother indisponible)
- Contexte de trace incomplet (identite intervenant, moment, type, resultat)
- Violation des exigences du Trace Contract

**Gravite :** Critique

**Consequence :** L'intervention ne peut pas etre consideree comme realisee ; signalement d'erreur, pas d'effet de bord (INV-TAMR-1 preserve par refus de traiter sans trace)

### 3.3. Erreurs de coherence

**Definition :**

Les **erreurs de coherence** sont des erreurs causees par une violation des invariants internes du cadre TAMR (INV-TAMR-1 a INV-TAMR-8).

**Exemples conceptuels :**

- Tentative d'intervention sans identite intervenant (violation tracabilite)
- Contradiction detectee entre type d'intervention et point d'intervention
- Etat du cadre incoherent (point d'intervention inexistant)

**Gravite :** Critique

**Consequence :** Arret du traitement, signalement d'erreur de coherence

### 3.4. Erreurs de ressource

**Definition :**

Les **erreurs de ressource** sont des erreurs causees par l'indisponibilite de ressources necessaires au traitement d'une intervention (canal vers StrongFather, persistance des traces, etc.).

**Exemples conceptuels :**

- Canal BondingBrother ou StrongFather indisponible pour soumettre l'intention d'intervention
- Persistance KindMother indisponible pour enregistrer la trace
- Delai depasse (timeout) avant reponse

**Gravite :** Haute

**Consequence :** Echec du traitement de l'intervention, possibilite de retry selon contexte (conforme LOI-2 : isolement comme etat normal)

---

## 4. Categories de rejets d'intervention

### 4.1. Rejet structurel (intervention invalide)

**Definition :**

Un **rejet structurel** se produit lorsque la demande d'intervention soumise est structurellement invalide selon les regles definies dans l'Intervention Types Contract et l'Intervention Points Contract.

**Causes :**

- Absence d'un composant obligatoire (identite intervenant, type d'intervention, point d'intervention)
- Type d'intervention non reconnu
- Point d'intervention inexistant ou inactif
- Contexte d'appel incomplet

**Resultat produit :** INTERVENTION_REFUSEE (structure invalide)

**Contenu du rejet :**

- Type de rejet : STRUCTUREL
- Composants manquants ou invalides
- Regles de formation violees
- Aucune evaluation de politique ou de limite (rejet avant evaluation)

### 4.2. Rejet par limite d'autorite

**Definition :**

Un **rejet par limite d'autorite** se produit lorsque l'intervention est structurellement valide mais depasse les limites d'autorite definies dans l'Authority Limits Contract pour le contexte donne.

**Causes :**

- Intervention hors du perimetre autorise pour l'intervenant
- Contexte (role, niveau de securite, domaine) insuffisant pour ce type d'intervention
- Restriction contextuelle non respectee

**Resultat produit :** INTERVENTION_REFUSEE (hors limites d'autorite)

**Contenu du rejet :**

- Type de rejet : LIMITE_AUTORITE
- Limite concernee (identifiant ou description conceptuelle)
- Contexte d'evaluation
- Justification du rejet

### 4.3. Rejet par limite inviolable

**Definition :**

Un **rejet par limite inviolable** se produit lorsque l'intervention tente de franchir une limite infranchissable definie dans l'Inviolable Limits Contract (LIM-INV-*).

**Causes :**

- Tentative d'override sur une limite inviolable
- Action interdite absolument (integrite systeme, donnees critiques, regles de securite fondamentales, contraintes legales)
- Violation de INV-TAMR-3 (limites infranchissables)

**Resultat produit :** INTERVENTION_REFUSEE (limite inviolable)

**Contenu du rejet :**

- Type de rejet : LIMITE_INVIOLABLE
- Limite inviolable concernee (LIM-INV-*)
- Aucune exception possible
- Trace obligatoire de la tentative (conformement R-DETECT-3, Inviolable Limits Contract)

### 4.4. Rejet par decision StrongFather

**Definition :**

Un **rejet par decision StrongFather** se produit lorsque StrongFather evalue l'intention d'intervention et refuse l'autorisation (decision REFUSEE selon le StrongFather Error & Rejection Model).

**Causes :**

- Politique non satisfaite
- Intention d'intervention invalide au regard des politiques StrongFather
- Contexte d'evaluation insuffisant ou ambigu

**Resultat produit :** INTERVENTION_REFUSEE (decision StrongFather)

**Contenu du rejet :**

- Type de rejet : DECISION_STRONGFATHER
- Justification liee a la decision StrongFather (sans exposer de detail technique interne)
- Contexte d'evaluation
- Trace de la tentative d'intervention (INV-TAMR-1)

### 4.5. Rejet pour ambiguite

**Definition :**

Un **rejet pour ambiguite** se produit lorsque la demande d'intervention ne peut pas etre evaluee completement car des informations sont manquantes ou insuffisamment definies.

**Causes :**

- Elements de la demande d'intervention insuffisamment definis
- Contexte insuffisant pour evaluer les limites ou les politiques
- Clarifications necessaires (role, point d'intervention, type)

**Resultat produit :** INTERVENTION_AMBIGUE

**Contenu du rejet :**

- Type de rejet : AMBIGUITE
- Elements manquants ou insuffisants
- Clarifications requises
- Non definitif : la demande peut etre reformulee et re-soumise

### 4.6. Rejet pour escalade non resolue (timeout)

**Definition :**

Un **rejet pour escalade non resolue** se produit lorsque une escalade n'est pas resolue dans le delai prevu (conformement INV-TAMR-8 : escalade non bloquante).

**Causes :**

- Timeout d'escalade atteint
- Aucune reponse du niveau superieur dans le delai
- Mecanisme de repli (rejet par defaut, delegation automatique) declenche

**Resultat produit :** INTERVENTION_REFUSEE (escalade timeout) ou resultat de repli selon contrat

**Contenu du rejet :**

- Type de rejet : ESCALADE_TIMEOUT
- Contexte de l'escalade
- Raison du rejet (timeout)
- Trace de l'escalade et du timeout

---

## 5. Structure des messages d'erreur

### 5.1. Composants obligatoires

Tout message d'erreur (cadre TAMR) DOIT contenir :

**Identifiant d'erreur :** Un identifiant unique permettant de referencer l'erreur.

**Categorie d'erreur :** La categorie de l'erreur (STRUCTURE, TRACABILITE, COHERENCE, RESSOURCE).

**Description :** Une description conceptuelle de l'erreur.

**Contexte d'erreur :** Le contexte dans lequel l'erreur s'est produite (point d'intervention, type d'intervention, intervenant si disponible).

### 5.2. Composants optionnels

**Cause racine :** La cause conceptuelle identifiee de l'erreur.

**Recommandation :** Une recommandation conceptuelle pour resoudre l'erreur.

**References :** Des references vers des documents ou des contrats pertinents.

---

## 6. Structure des messages de rejet d'intervention

### 6.1. Composants obligatoires

Tout message de rejet d'intervention DOIT contenir :

**Identifiant de la demande d'intervention :** L'identifiant de la demande rejetee.

**Type de resultat :** Le type de resultat (INTERVENTION_REFUSEE, INTERVENTION_AMBIGUE, etc.).

**Type de rejet :** La categorie de rejet (STRUCTUREL, LIMITE_AUTORITE, LIMITE_INVIOLABLE, DECISION_STRONGFATHER, AMBIGUITE, ESCALADE_TIMEOUT).

**Justification :** La justification detaillee du rejet.

**Contexte d'evaluation :** Le contexte utilise pour l'evaluation (point d'intervention, intervenant, limites concernees).

### 6.2. Composants specifiques par type

**Pour rejet STRUCTUREL :** Composants manquants, regles de formation violees.

**Pour rejet LIMITE_AUTORITE :** Limite concernee, contexte d'evaluation.

**Pour rejet LIMITE_INVIOLABLE :** Limite inviolable concernee (LIM-INV-*), reference au contrat.

**Pour rejet DECISION_STRONGFATHER :** Justification liee a la decision (niveau conceptuel).

**Pour rejet AMBIGUITE :** Elements manquants ou insuffisants, clarifications requises.

**Pour rejet ESCALADE_TIMEOUT :** Contexte escalade, delai depasse.

---

## 7. Regles de propagation

### 7.1. Propagation des erreurs

**R-PROP-ERR-1 : Non-absorption**

Les erreurs ne sont jamais absorbees silencieusement. Toute erreur doit etre signalee.

**R-PROP-ERR-2 : Remontee**

Les erreurs sont remontees a l'appelant avec leur contexte complet.

**R-PROP-ERR-3 : Pas de transformation en rejet**

Une erreur ne peut jamais etre transformee en rejet d'intervention. Les erreurs et les rejets sont distincts.

**R-PROP-ERR-4 : Arret de traitement**

Une erreur arrete le traitement de l'intervention. Aucun resultat d'intervention valide n'est produit suite a une erreur. Si la tracabilite a echoue (erreur tracabilite), l'intervention ne doit pas etre consideree comme realisee (INV-TAMR-1).

### 7.2. Propagation des rejets

**R-PROP-REJ-1 : Resultat produit**

Un rejet produit toujours un resultat explicite (INTERVENTION_REFUSEE, INTERVENTION_AMBIGUE, etc.).

**R-PROP-REJ-2 : Justification complete**

Un rejet est toujours accompagne d'une justification complete.

**R-PROP-REJ-3 : Tracabilite**

Un rejet d'intervention est toujours trace : la tentative d'intervention et son rejet sont enregistres (INV-TAMR-1, tracabilite absolue).

**R-PROP-REJ-4 : Non-blocage**

Un rejet ne bloque pas le cadre TAMR. D'autres interventions peuvent etre traitees.

---

## 8. Invariants de gestion d'erreur

### 8.1. Invariants de distinction

**INV-ERR-TAMR-1 : Distinction erreur / rejet**

Toute situation est soit une erreur, soit un rejet d'intervention, jamais les deux. La distinction est absolue.

**INV-ERR-TAMR-2 : Erreur sans resultat valide**

Une erreur ne produit jamais de resultat d'intervention valide. Les erreurs et les resultats d'intervention sont mutuellement exclusifs.

**INV-ERR-TAMR-3 : Rejet avec resultat**

Un rejet produit toujours un resultat explicite. Pas de rejet sans resultat associe.

### 8.2. Invariants de tracabilite

**INV-ERR-TAMR-4 : Tracabilite des erreurs**

Toute erreur est tracable avec son contexte et sa cause (log d'erreur).

**INV-ERR-TAMR-5 : Tracabilite des rejets**

Tout rejet d'intervention est trace : la tentative et le rejet sont enregistres (conforme INV-TAMR-1).

### 8.3. Invariants de comportement

**INV-ERR-TAMR-6 : Pas d'effet de bord sur erreur**

Une erreur ne produit jamais d'effet de bord sur le systeme (pas d'intervention consideree realisee si erreur de tracabilite).

**INV-ERR-TAMR-7 : Pas d'effet de bord non trace sur rejet**

Un rejet ne produit pas d'effet de bord non trace : le rejet lui-meme est trace (tentative refusee).

---

## 9. Regles de fermeture du contrat

### 9.1. Contrat ferme

Ce contrat est **ferme**. Seules les categories d'erreurs, les categories de rejets, les structures et les regles explicitement definies dans ce contrat sont autorisees.

### 9.2. Interdiction d'extension implicite

Aucune extension implicite de ce contrat n'est autorisee :

- **INTERD-ERR-TAMR-1** : Aucune categorie d'erreur non definie n'est reconnue
- **INTERD-ERR-TAMR-2** : Aucune categorie de rejet non definie n'est reconnue
- **INTERD-ERR-TAMR-3** : Aucune regle de propagation non definie n'est applicable
- **INTERD-ERR-TAMR-4** : Aucun invariant non defini n'est garanti

---

## 10. Conclusion contractuelle

Ce contrat etablit de maniere definitive et non negociable le modele d'erreurs et de rejets d'intervention de TAMR.

Il garantit que :

- la distinction erreur / rejet d'intervention est absolue,
- les categories d'erreurs et de rejets sont definies et fermees,
- les structures de messages sont standardisees,
- les regles de propagation sont explicites,
- les invariants sont respectes,
- le contrat est ferme et non extensible implicitement.

Ce contrat est de statut **FONDATION**. Aucune exception n'est autorisee.

---

## 11. Validation conceptuelle

### 11.1. Cas valides

Les cas suivants sont **valides** selon ce contrat :

1. **Rejet structurel** : Une demande d'intervention sans identite intervenant produit un resultat INTERVENTION_REFUSEE de type STRUCTUREL.

2. **Rejet par limite inviolable** : Une tentative d'override sur une limite inviolable produit un resultat INTERVENTION_REFUSEE de type LIMITE_INVIOLABLE, avec trace de la tentative.

3. **Rejet pour ambiguite** : Une demande d'intervention avec contexte insuffisant produit un resultat INTERVENTION_AMBIGUE de type AMBIGUITE.

4. **Erreur de tracabilite** : L'impossibilite d'enregistrer la trace arrete le traitement, aucune intervention n'est consideree realisee, erreur signalee.

### 11.2. Cas de violation

Les cas suivants **violent** ce contrat :

1. **Erreur transformee en rejet** : Une erreur de tracabilite transformee en INTERVENTION_REFUSEE. Viole R-PROP-ERR-3.

2. **Rejet sans resultat** : Un rejet qui ne produit pas de resultat explicite. Viole INV-ERR-TAMR-3.

3. **Erreur absorbee** : Une erreur qui n'est pas signalee. Viole R-PROP-ERR-1.

4. **Rejet sans trace** : Un rejet d'intervention non trace (tentative non enregistree). Viole INV-TAMR-1 et R-PROP-REJ-3.

---

## 12. References

| Reference | Description |
|-----------|-------------|
| [Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) | Terminologie TAMR (intervention, point d'intervention, limite d'autorite, etc.) |
| [Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md) | Principes de securite |
| [Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) | Conformite LOI-1 a LOI-6 |
| [Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) | Niveaux T0-T4 (contexte de confiance) |
| [Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) | Niveaux 0-4 (contexte de securite) |

---

**Document cree le :** 2026-01-28  
**Version :** 1.0  
**Statut :** FONDATION â€” Contrat normatif  
**Reference :** Miyukini Core System, TAMR Documentation Fondatrice  
**Type :** Contrat de modele d'erreur et de rejet d'intervention non negociable

