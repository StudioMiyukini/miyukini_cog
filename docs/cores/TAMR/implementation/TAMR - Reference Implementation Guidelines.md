# TAMR - Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un developpeur ou un architecte pour implementer les concepts TAMR correctement, sans violer les contrats FONDATION.

**Objectif :** Aider a traduire les contrats TAMR (types d'intervention, points d'intervention, tracabilite, limites) en implementation dans les produits et les cores (StrongFather, KindMother, BondingBrother).

**Avertissement :** Ce document ne cree aucune nouvelle regle contractuelle. Les contrats FONDATION priment toujours.

### References contractuelles

- [TAMR - Documentation Fondatrice](../foundation/TAMR%20-%20Documentation%20Fondatrice.md)
- [TAMR - Intervention Types Contract](../contracts/intervention/TAMR%20-%20Intervention%20Types%20Contract.md)
- [TAMR - Intervention Points Contract](../contracts/intervention/TAMR%20-%20Intervention%20Points%20Contract.md)
- [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md)
- [TAMR - Invariants & Guarantees](../contracts/governance/TAMR%20-%20Invariants%20&%20Guarantees.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 1. Principes generaux a respecter absolument

### 1.1. TAMR ne s'execute pas (INV-TAMR-4, INV-TAMR-5)

**Principe contractuel :** TAMR reste purement conceptuel. Il ne prend aucune decision, ne persiste rien, ne definit pas d'interface utilisateur.

**Traduction en implementation :**
- Il n'existe pas de "service TAMR" a deployer. Les **produits** declarent les points d'intervention et presentent les interfaces d'intervention a l'humain.
- **StrongFather** decide si une intervention est autorisee ; **KindMother** persiste les traces ; **BondingBrother** mediatise les intentions.
- L'implementation consiste a : (1) declarer les points d'intervention selon les categories TAMR, (2) emettre des intentions d'intervention via BondingBrother, (3) produire des traces conformes au Trace Contract et les faire persister via KindMother.

### 1.2. Tracabilite absolue (INV-TAMR-1)

**Principe contractuel :** Toute intervention humaine est tracee, sans exception.

**Traduction en implementation :**
- Chaque action humaine de type APPROVAL, OVERRIDE, ESCALATION, SUPERVISION DOIT generer une trace contenant au minimum : identite de l'intervenant, type d'intervention, moment (horodatage local), contexte, resultat. Pour OVERRIDE : justification obligatoire.
- La trace DOIT etre emise et persistee (via KindMother ou mecanisme d'audit conforme) dans un delai compatible avec les exigences de securite (ex. RT-SEC-5 si applicable).
- Aucun chemin de code ne doit permettre une intervention sans emission de trace.

### 1.3. Justification obligatoire pour override (INV-TAMR-7)

**Principe contractuel :** Tout override necessite une justification explicite enregistree.

**Traduction en implementation :**
- L'interface produit qui permet un override DOIT exiger un champ "justification" non vide avant envoi.
- La trace d'override DOIT contenir le champ `justification`. Un override sans justification ne doit pas etre accepte par StrongFather / persiste par KindMother (validation cote implementation).

### 1.4. Escalade non bloquante (INV-TAMR-8)

**Principe contractuel :** Une escalade ne bloque pas indefiniment le systeme.

**Traduction en implementation :**
- Tout flux d'escalade DOIT avoir un **timeout** configurable et un **comportement par defaut** explicite (ex. rejet par defaut, delegation automatique). Lorsque le timeout est atteint, le comportement par defaut est applique et trace.
- Aucune attente infinie sur une reponse humaine pour debloquer un flux critique.

### 1.5. Limites infranchissables (INV-TAMR-3)

**Principe contractuel :** Certaines limites ne peuvent jamais etre franchise, meme par un override.

**Traduction en implementation :**
- StrongFather (ou le composant qui applique les politiques) DOIT verifier les limites infranchissables ([TAMR - Inviolable Limits Contract](../contracts/boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md)) avant d'accepter un override. Si une limite est franchise, l'override est refuse meme avec justification.
- Les limites infranchissables (integrite systeme, donnees critiques, regles fondamentales, contraintes legales) doivent etre listees et verifiees dans l'implementation.

---

## 2. Declaration des points d'intervention

### 2.1. Categories de points

Les points d'intervention doivent etre declares selon les categories definies dans [TAMR - Intervention Points Contract](../contracts/intervention/TAMR%20-%20Intervention%20Points%20Contract.md) (ex. DECISION_GATE, OVERRIDE_GATE, ESCALATION_POINT, SUPERVISION_SCOPE). Chaque point possede : `point_id`, `category`, `process_id`, `intervention_types` autorises, `activation_conditions`, `required`, `security_level_min`, etc.

### 2.2. Implementation

- Maintenir un registre (config, base, ou code) des points d'intervention par processus.
- Lorsqu'un flux atteint un point d'intervention, verifier les conditions d'activation et les types autorises ; si une intervention est requise, emettre une intention vers BondingBrother et attendre la reponse (ou timeout + comportement par defaut).
- Logger l'activation du point pour audit (conformite TAMR).

---

## 3. Flux d'intervention et integration

### 3.1. Approval

1. Le processus atteint un point d'approbation ; le produit cree une demande d'approbation (intention).
2. L'intention transite par BondingBrother ; StrongFather evalue si l'approbation est requise et par qui.
3. Le produit notifie l'approbateur designe (hors scope TAMR : UI, notification).
4. L'approbateur approuve ou refuse ; le produit emet une trace conforme au Trace Contract (type APPROVAL, identite, moment, resultat, contexte) et la fait persister (KindMother).
5. Le processus reprend selon la decision.

### 3.2. Override

1. Une decision automatique (acceptee ou refusee) a ete emise ; un humain autorise demande un override.
2. L'intention d'override transite par BondingBrother ; StrongFather evalue si l'override est autorise et verifie les limites infranchissables.
3. L'humain fournit une **justification** obligatoire ; le produit emet une trace conforme (type OVERRIDE, justification, original_decision, etc.) et la fait persister.
4. StrongFather applique l'override si les limites sont respectees ; le processus reprend.

### 3.3. Escalation

1. Une situation necessitant une escalade est identifiee ; une demande d'escalade est creee (intention).
2. L'intention transite par BondingBrother ; StrongFather identifie le niveau d'escalade.
3. Le produit notifie le niveau superieur ; un **timeout** et un **comportement par defaut** sont associes. Si le delai est depasse, appliquer le comportement par defaut et tracer.
4. La trace d'escalade contient : initiator_id, motif, escalation_path, current_level, resolution (si resolue), timeout_behavior.

### 3.4. Supervision

1. Un humain active une supervision (perimetre et duree definis).
2. Le produit enregistre le debut de supervision et la trace (type SUPERVISION, supervisor_id, scope, started_at, duration_planned).
3. Si le superviseur declenche une intervention, celle-ci est typée (APPROVAL, OVERRIDE, ESCALATION) et tracee separement.
4. A la fin (explicite ou timeout), la trace est completee (ended_at, end_reason).

---

## 4. Conformite aux Lois d'Autonomie

Les implementations DOIVENT respecter les Lois d'Autonomie Systeme : interventions et traces possibles en mode isole (LOI-1, LOI-2) ; horodatage local (LOI-4) ; pas de dependance externe critique pour valider une intervention (LOI-1). La synchronisation des traces peut etre differee (KindMother offline-first).

---

## 5. Checklist de conformite

- [ ] Aucun "service TAMR" central : les responsabilites sont reparties (produit, StrongFather, KindMother, BondingBrother).
- [ ] Chaque intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) produit une trace conforme au Trace Contract.
- [ ] Tout override exige une justification et est verifie contre les limites infranchissables.
- [ ] Toute escalade a un timeout et un comportement par defaut.
- [ ] Les points d'intervention sont declares selon les categories TAMR.
- [ ] Les traces sont persistees (KindMother ou mecanisme d'audit conforme) et utilisent l'horodatage local.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** INFORMATIF  
**Reference :** TAMR Documentation Fondatrice, contrats FONDATION
