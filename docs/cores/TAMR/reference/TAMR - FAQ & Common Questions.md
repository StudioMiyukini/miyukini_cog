# TAMR - FAQ & Common Questions

## Contexte

Ce document repond aux **questions frequentes** sur TAMR (The Authority Must Rest) et les interventions humaines dans le Miyukini Core System. Il est **informatif** et ne modifie aucun contrat FONDATION.

**Terminologie :** [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 1. Nature et role de TAMR

### TAMR est-il un service a deployer ?

**Non.** TAMR est un **cadre conceptuel** pur. Il ne s'execute pas, ne prend aucune decision, ne persiste rien. Les **produits** declarent les points d'intervention et presentent les interfaces ; **StrongFather** decide si une intervention est autorisee ; **KindMother** persiste les traces ; **BondingBrother** mediatise les intentions. Il n'y a pas de "service TAMR" a installer.

### Qui implemente TAMR ?

Les **concepts** TAMR sont implementes par les produits (points d'intervention, interfaces d'approbation/override/escalade/supervision) et par les cores (StrongFather pour l'autorisation, KindMother pour les traces, BondingBrother pour la mediation). Voir [TAMR - Reference Implementation Guidelines](../implementation/TAMR%20-%20Reference%20Implementation%20Guidelines.md).

### TAMR prend-il des decisions ?

**Non.** TAMR definit les **regles** (types d'intervention, limites, tracabilite). **StrongFather** prend la decision d'autoriser ou refuser une intervention concrete. INV-TAMR-5 : TAMR ne prend jamais de decision.

---

## 2. Types d'intervention

### Peut-on ajouter un nouveau type d'intervention (ex. "DELEGATION") ?

Les quatre types (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) sont **fermes** par contrat ([TAMR - Intervention Types Contract](../contracts/intervention/TAMR%20-%20Intervention%20Types%20Contract.md)). Un nouveau type necessiterait une evolution formelle du contrat (version MAJEUR). Une "delegation" peut etre modelisee comme une ESCALATION (transfert vers un niveau superieur ou un pair).

### Quelle est la difference entre APPROVAL et OVERRIDE ?

- **APPROVAL** : l'humain valide ou refuse une action **avant** son execution. Le systeme propose, l'humain decide.
- **OVERRIDE** : une **decision automatique** a deja ete emise (acceptee ou refusee) ; l'humain **contredit** cette decision (force une action refusee ou bloque une action approuvee). L'override est exceptionnel et exige une **justification** obligatoire.

### L'escalade peut-elle bloquer le systeme ?

**Non.** INV-TAMR-8 : une escalade ne bloque pas indefiniment le systeme. Toute implementation doit prevoir un **timeout** et un **comportement par defaut** (ex. rejet par defaut, delegation automatique). Si le niveau superieur ne repond pas dans le delai, le comportement par defaut est applique et trace.

---

## 3. Limites et securite

### Qu'est-ce qu'une limite infranchissable ?

Une **limite infranchissable** est une restriction que **meme un override** ne peut pas franchir. Exemples : integrite du systeme, donnees critiques de securite, regles fondamentales (L1-L6), contraintes legales. Voir [TAMR - Inviolable Limits Contract](../contracts/boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md). StrongFather refuse tout override qui franchirait une telle limite.

### TAMR s'adapte-t-il aux niveaux de confiance (T0-T4) et de securite (0-4) ?

**Oui.** En T3 (restreint), un override necessite le canal TAMR ; en T4 (bloque), l'intervention humaine est le seul canal. Par niveau de securite (0-4), l'intervention humaine passe de "non requise" (0) a "systematique" (4). Voir [TAMR - Security Contract](../contracts/security/TAMR%20-%20Security%20Contract.md).

---

## 4. Tracabilite et audit

### Que doit contenir une trace d'intervention ?

Au minimum : identite de l'intervenant, type d'intervention, moment (horodatage local), contexte, resultat. Pour un override : **justification** obligatoire. Les champs detailles par type sont dans [TAMR - Intervention Types Contract](../contracts/intervention/TAMR%20-%20Intervention%20Types%20Contract.md) et [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md).

### Qui persiste les traces ?

**KindMother** (ou un mecanisme d'audit conforme). TAMR definit la **structure** conceptuelle des traces ; KindMother assure la persistance. TAMR ne persiste rien (INTERD-TAMR-2).

### Une intervention peut-elle etre anonyme ?

**Non.** INV-TAMR-2 : l'humain qui intervient assume explicitement la responsabilite ; toute trace doit identifier l'intervenant. Une intervention anonyme est invalide au regard des contrats TAMR.

---

## 5. Integration et ecosysteme

### Comment TAMR s'articule-t-il avec StrongFather ?

TAMR definit les **regles** (types, limites, tracabilite). StrongFather **decide** si une intervention concrete est autorisee (politiques, contexte, utilisateur). Relation complementaire : TAMR dit "quoi" est possible, StrongFather dit "si" c'est autorise pour ce cas.

### Comment TAMR s'articule-t-il avec BondingBrother ?

Toute **intention** d'intervention (approbation, override, escalade, supervision) transite par **BondingBrother** vers les autorites (StrongFather, etc.). TAMR ne communique jamais directement avec les cores ; BondingBrother mediatise.

### TAMR gere-t-il les notifications (email, UI) ?

**Non.** INTERD-TAMR-7 : TAMR ne gere pas la notification. Comment l'humain est informe qu'une approbation ou une escalade est requise releve du **produit** (interfaces, workflows, notifications).

---

## 6. Conformite et tests

### Comment verifier qu'une implementation est conforme a TAMR ?

En appliquant le [TAMR - Testing & Validation Contract](../implementation/TAMR%20-%20Testing%20&%20Validation%20Contract.md) : tests d'invariants (INV-TAMR-1 a INV-TAMR-8), tests de tracabilite, tests de limites infranchissables, tests de non-blocage des escalades. Voir aussi [TAMR - Conformance & Certification Rules](../contracts/governance/TAMR%20-%20Conformance%20&%20Certification%20Rules.md).

### TAMR est-il compatible avec les Lois d'Autonomie Systeme ?

**Oui.** TAMR est purement conceptuel (LOI-1 : pas de dependance externe) ; les interventions et traces restent possibles en mode isole (LOI-2) ; horodatage local (LOI-4). Voir la section "Conformite aux Lois d'Autonomie" dans la [Documentation Fondatrice](../foundation/TAMR%20-%20Documentation%20Fondatrice.md).

---

## 7. Evolution et versioning

### Comment evoluer les contrats TAMR ?

Selon le [TAMR - Versioning & Evolution Contract](../lifecycle/TAMR%20-%20Versioning%20&%20Evolution%20Contract.md) : versioning MAJEUR.MINEUR.PATCH, compatibilite ascendante, depreciation explicite, migration conceptuelle pour changements MAJEUR. Un gel officiel peut etre declare via [TAMR - Release & Freeze Contract](../lifecycle/TAMR%20-%20Release%20&%20Freeze%20Contract.md).

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** INFORMATIF  
**Reference :** TAMR Documentation Fondatrice, contrats FONDATION

