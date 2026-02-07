# TAMR - Operational Runbook

## 1. Introduction

### Objet du document

Ce document definit le **TAMR - Operational Runbook** : un guide operationnel pour l'exploitation des systemes qui implementent les concepts TAMR (intervention humaine, points d'intervention, tracabilite). TAMR etant purement conceptuel, ce runbook s'adresse aux SRE / Ops et aux administrateurs qui doivent garantir que les interventions humaines sont correctement declarees, tracees, et auditees.

### Portee

Ce document s'applique a **toute l'exploitation des flux d'intervention humaine** dans un ecosysteme Miyukini et couvre :
- les procedures de verification de conformite TAMR,
- le monitoring des interventions et des traces,
- les alertes liees aux interventions (override sans justification, escalade bloquante, trace manquante),
- les procedures de troubleshooting.

### Statut

Ce document est **operationnel et pratique**. Il complete les contrats FONDATION sans imposer d'outils ou d'infrastructure specifiques.

### References

- [TAMR - Documentation Fondatrice](../foundation/TAMR%20-%20Documentation%20Fondatrice.md)
- [TAMR - Architecture & Flows](../architecture/TAMR%20-%20Architecture%20&%20Flows.md)
- [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md)
- [TAMR - Security Contract](../contracts/security/TAMR%20-%20Security%20Contract.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 2. Contexte operationnel

### 2.1. Nature de TAMR

TAMR est un **cadre conceptuel** qui definit :
- les types d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION),
- les points d'intervention et leurs conditions,
- les limites d'autorite et les limites infranchissables,
- les exigences de tracabilite.

TAMR ne s'execute pas : les **produits** et les **cores** (StrongFather, KindMother, BondingBrother) implementent ces concepts. L'operation concerne donc la verification que ces implementations respectent les contrats TAMR et que les interventions sont correctement tracees et auditees.

### 2.2. Caracteristiques operationnelles critiques

**A preserver absolument :**
1. **Tracabilite absolue** (INV-TAMR-1) : Toute intervention humaine est tracee.
2. **Responsabilite explicite** (INV-TAMR-2) : L'intervenant est identifie.
3. **Justification obligatoire pour override** (INV-TAMR-7) : Tout override a une justification enregistree.
4. **Escalade non bloquante** (INV-TAMR-8) : Aucune escalade ne bloque indefiniment le systeme.

### 2.3. Niveaux de confiance (T0-T4) et niveaux de securite (0-4)

Le comportement des interventions s'adapte selon le niveau de confiance (T0-T4) et le niveau de securite (0-4). En T3, les overrides necessitent TAMR ; en T4, l'intervention humaine est le seul canal. Voir [TAMR - Security Contract](../contracts/security/TAMR%20-%20Security%20Contract.md).

---

## 3. Verification de conformite

### 3.1. Checklist pre-production

Avant mise en production d'un flux avec intervention humaine :
- [ ] Tous les points d'intervention sont declares selon [TAMR - Intervention Points Contract](../contracts/intervention/TAMR%20-%20Intervention%20Points%20Contract.md).
- [ ] Chaque type d'intervention (APPROVAL, OVERRIDE, ESCALATION, SUPERVISION) produit une trace conforme au [TAMR - Trace Contract](../contracts/audit/TAMR%20-%20Trace%20Contract.md).
- [ ] Les overrides exigent une justification et sont verifies contre les limites infranchissables ([TAMR - Inviolable Limits Contract](../contracts/boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md)).
- [ ] Les escalades ont un comportement par defaut (timeout, rejet par defaut) pour respecter INV-TAMR-8.
- [ ] L'integration avec StrongFather (autorisation) et BondingBrother (mediation) est en place selon les contrats d'integration TAMR.

### 3.2. Audit periodique

- Verifier que toutes les interventions enregistrees ont une trace complete (identite, type, moment, contexte, resultat).
- Verifier qu'aucun override n'a ete enregistre sans justification.
- Verifier qu'aucune escalade n'est restee en etat "en cours" au-dela du delai prevu sans resolution ou comportement par defaut.

---

## 4. Monitoring

### 4.1. Metriques conceptuelles a suivre

| Metrique | Description | Alerte si |
|----------|-------------|-----------|
| Interventions sans trace | Nombre d'interventions non tracees | > 0 |
| Overrides sans justification | Overrides enregistres sans champ justification | > 0 |
| Escalades non resolues | Escalades en etat "en cours" au-dela du timeout | > 0 |
| Tentatives de franchissement de limite infranchissable | Overrides refuses pour limite infranchissable | A auditer |

### 4.2. Sources de donnees

Les traces d'intervention sont persistees par KindMother selon la structure definie par TAMR. Le monitoring doit s'appuyer sur ces traces (via Audit Engine ou equivalent) pour detecter les ecarts aux invariants TAMR.

---

## 5. Alertes et reponses

### 5.1. Alerte : Intervention non tracee

**Cause possible :** Defaut d'implementation (le produit n'envoie pas la trace a KindMother) ou defaut de persistance.

**Reponse :**
1. Identifier le flux et le point d'intervention concerne.
2. Verifier que le produit emet bien une trace conforme au Trace Contract.
3. Verifier que KindMother persiste la trace (configuration, erreurs).
4. Corriger l'implementation ou la configuration ; rejouer si possible l'intervention pour generer la trace a posteriori (si autorise par la politique de rétention).

### 5.2. Alerte : Override sans justification

**Cause possible :** Champ justification non renseigne ou non persiste.

**Reponse :**
1. Identifier l'override et l'intervenant.
2. Exiger a posteriori une justification et la faire enregistrer (audit).
3. Corriger l'interface produit pour rendre la justification obligatoire avant envoi.

### 5.3. Alerte : Escalade bloquee

**Cause possible :** Timeout non configure, niveau superieur indisponible, comportement par defaut absent.

**Reponse :**
1. Identifier l'escalade et le niveau bloque.
2. Appliquer manuellement le comportement par defaut si defini (ex. rejet par defaut).
3. Corriger la configuration (timeout, delegation automatique) pour respecter INV-TAMR-8.

---

## 6. Troubleshooting

### 6.1. L'intervention n'apparait pas dans l'audit

- Verifier que l'intention d'intervention transite bien par BondingBrother et que StrongFather a autorise l'intervention.
- Verifier que la trace est emise par le produit et persistee par KindMother (logs, configuration).
- Verifier le format de la trace (conformite au Trace Contract).

### 6.2. Override refuse alors qu'attendu

- Verifier que l'override ne franchit pas une limite infranchissable ([TAMR - Inviolable Limits Contract](../contracts/boundaries/TAMR%20-%20Inviolable%20Limits%20Contract.md)).
- Verifier que StrongFather autorise l'override dans ce contexte (politiques).
- En T3, verifier que le canal TAMR pour override est bien active ([TAMR - Security Contract](../contracts/security/TAMR%20-%20Security%20Contract.md)).

### 6.3. Escalade sans reponse

- Verifier le delai de timeout et le comportement par defaut configures.
- Verifier que le niveau superieur est notifie (produit / notification).
- Appliquer le comportement par defaut si le delai est depasse pour eviter un blocage indefini (INV-TAMR-8).

---

## 7. Conformite aux Lois d'Autonomie

Les procedures operationnelles DOIVENT preserver la conformite aux Lois d'Autonomie Systeme : le monitoring et l'audit des interventions peuvent etre effectues localement (LOI-1, LOI-2) ; les traces sont persistees localement et synchronisees selon KindMother (LOI-3).

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Operationnel  
**Reference :** TAMR Documentation Fondatrice, Trace Contract, Security Contract
