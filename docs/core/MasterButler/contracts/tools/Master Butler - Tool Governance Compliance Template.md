# Master Butler — Tool Governance Compliance Template

## Contexte

Ce document définit les **obligations communes** de conformité pour tout Toolkit documenté sous docs/tools. Chaque kit (MiyuXXX) déclare sa conformité à ce template et ajoute uniquement ses **obligations spécifiques** dans son propre contrat `MiyuXXX - Tool Governance Compliance Contract.md`.

**Référence :** [Master Butler - Tool Governance Contract](./Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Master Butler - Toolkit Composition Contract](./Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)

---

## Obligations communes (tous les Toolkits)

- Le Toolkit et chaque Tool composant sont **déclarés** au Master Butler.
- Toute utilisation passe par le **catalogue** et la **gouvernance** (BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather).
- Aucun Tool n'exécute de **décision métier** : les décisions (autorisation, validation, politique) relèvent de StrongFather et des Cores.
- Toute écriture de données métier = **WriteIntent** vers KindMother (sauf cas explicitement documentés en lecture seule).

---

## Usage par kit

Dans `docs/tools/<MiyuXXX>/contracts/governance/MiyuXXX - Tool Governance Compliance Contract.md` :

1. Référencer ce template : « Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md). »
2. Indiquer le **ToolkitId** du kit.
3. Lister uniquement les **obligations spécifiques** du kit (ex. : règles alertes, permissions employé, validation rapprochement, etc.).

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat-type de conformité
