# TAMR - Migration & Compatibility Contract

## 1. Introduction

### Objet du contrat

Ce document definit le **TAMR - Migration & Compatibility Contract** : un contrat normatif qui etablit les regles de migration progressive vers TAMR (ou d'une version TAMR a une autre), la compatibilite avec les systemes sans cadre d'intervention humaine formalise, et les strategies de coexistence.

TAMR etant purement conceptuel, la migration concerne l'adoption des concepts (types d'intervention, points d'intervention, tracabilite) par les produits et les cores, sans deploiement technique de TAMR en tant que composant.

### Portee

Ce contrat s'applique a :
- l'introduction de TAMR dans un ecosysteme qui n'avait pas de cadre d'intervention humaine formalise,
- le passage d'une version majeure de TAMR a une autre,
- la compatibilite des traces d'intervention entre versions.

### Statut contractuel

Ce document est **contractuel, normatif, et de statut FONDATION**.

### References

- [TAMR - Documentation Fondatrice](../foundation/TAMR%20-%20Documentation%20Fondatrice.md)
- [TAMR - Versioning & Evolution Contract](./TAMR%20-%20Versioning%20&%20Evolution%20Contract.md)
- [TAMR - Integration Readiness Contract](../architecture/TAMR%20-%20Integration%20Readiness%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. Contexte de migration

### 2.1. Situation sans TAMR formalise

Avant l'adoption de TAMR, un systeme peut :
- avoir des validations manuelles ad hoc sans types ni tracabilite uniforme,
- ne pas distinguer approbation, override, escalade, supervision,
- ne pas tracer systematiquement l'identite et le contexte des interventions.

### 2.2. Objectif de migration

L'objectif est de :
- categoriser toutes les interventions humaines selon les quatre types TAMR,
- declarer les points d'intervention selon les categories TAMR,
- garantir la tracabilite selon le [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md),
- faire evaluer les autorisations d'intervention par StrongFather et transiter les intentions par BondingBrother.

### 2.3. Contraintes

- **ContinuitÃ©** : Les interventions existantes doivent pouvoir etre mappees vers les types TAMR sans rupture.
- **Compatibilite** : Les traces historiques (si elles existent) doivent rester lisibles ou migrables.
- **Progression graduelle** : L'adoption peut etre progressive (par processus ou par type d'intervention).

---

## 3. Migration progressive vers TAMR

### 3.1. Phase 1 - Inventaire

1. Recenser tous les points ou l'humain intervient (validation, derogation, escalade, surveillance).
2. Les mapper vers les types TAMR (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION).
3. Identifier les ecarts avec les exigences TAMR (tracabilite, justification pour override, non-blocage des escalades).

### 3.2. Phase 2 - Alignement conceptuel

1. Declarer les points d'intervention selon [TAMR - Intervention Points Contract](../contracts/intervention/TAMR%20-%20Intervention%20Points%20Contract.md).
2. S'assurer que chaque intervention produit une trace conforme au [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md).
3. Integrer avec StrongFather (autorisation) et BondingBrother (mediation) selon les contrats d'integration TAMR.

### 3.3. Phase 3 - Conformite

1. Verifier les invariants INV-TAMR-* (traÃ§abilite, responsabilite, limites infranchissables, justification override, escalade non bloquante).
2. Valider l'adaptation aux niveaux de confiance (T0-T4) et niveaux de securite (0-4) selon [TAMR - Security Contract](../contracts/security/TAMR%20-%20Security%20Contract.md).

---

## 4. Migration entre versions majeures de TAMR

### 4.1. Principe

Lors d'une evolution MAJEUR de TAMR (nouveau type d'intervention, modification d'invariant, changement de structure de trace), les produits et cores qui persistent des traces d'intervention doivent pouvoir :
- continuer a lire les traces existantes (compatibilite lecture),
- produire des traces conformes a la nouvelle version.

### 4.2. Guide de migration

Pour chaque version MAJEUR, le contrat [TAMR - Versioning & Evolution Contract](./TAMR%20-%20Versioning%20&%20Evolution%20Contract.md) exige un guide de migration precisant :
- les changements incompatibles,
- le mapping des anciennes traces vers la nouvelle structure (si applicable),
- les adaptations requises dans les produits.

### 4.3. Coexistence

Pendant une periode de transition, les deux versions peuvent coexister au niveau documentation ; l'implementation (produits) doit viser la nouvelle version pour les nouveaux flux et peut conserver la lecture des anciennes traces selon le guide de migration.

---

## 5. Compatibilite avec les Lois d'Autonomie

Les migrations vers ou entre versions TAMR DOIVENT preserver la conformite aux Lois d'Autonomie Systeme : interventions possibles en mode isole (LOI-2), tracabilite locale (LOI-3), pas de dependance externe critique pour le cadre conceptuel (LOI-1).

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** FONDATION  
**Reference :** TAMR Documentation Fondatrice, Versioning & Evolution Contract

