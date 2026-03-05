# Master Butler â€” Tool Governance Compliance Template

## Contexte

Ce document dÃ©finit les **obligations communes** de conformitÃ© pour tout Toolkit documentÃ© sous docs/tools. Chaque kit (MiyuXXX) dÃ©clare sa conformitÃ© Ã  ce template et ajoute uniquement ses **obligations spÃ©cifiques** dans son propre contrat `MiyuXXX - Tool Governance Compliance Contract.md`.

**RÃ©fÃ©rence :** [Master Butler - Tool Governance Contract](./Master%20Butler%20-%20Tool%20Governance%20Contract.md), [Master Butler - Toolkit Composition Contract](./Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)

---

## Obligations communes (tous les Toolkits)

- Le Toolkit et chaque Tool composant sont **dÃ©clarÃ©s** au Master Butler.
- Toute utilisation passe par le **catalogue** et la **gouvernance** (BondingBrother â†’ Master Butler â†’ WorrySentinel â†’ Caring Nanny â†’ StrongFather).
- Aucun Tool n'exÃ©cute de **dÃ©cision mÃ©tier** : les dÃ©cisions (autorisation, validation, politique) relÃ¨vent de StrongFather et des Cores.
- Toute Ã©criture de donnÃ©es mÃ©tier = **WriteIntent** vers KindMother (sauf cas explicitement documentÃ©s en lecture seule).

---

## Usage par kit

Dans `docs/tools/<MiyuXXX>/contracts/governance/MiyuXXX - Tool Governance Compliance Contract.md` :

1. RÃ©fÃ©rencer ce template : Â« ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md). Â»
2. Indiquer le **ToolkitId** du kit.
3. Lister uniquement les **obligations spÃ©cifiques** du kit (ex. : rÃ¨gles alertes, permissions employÃ©, validation rapprochement, etc.).

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat-type de conformitÃ©

