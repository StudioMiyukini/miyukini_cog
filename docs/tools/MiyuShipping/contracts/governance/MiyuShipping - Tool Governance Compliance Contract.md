# MiyuShipping — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.commerce.shipping`

---

## Obligations spécifiques MiyuShipping

- **Décision** (création étiquette, expédition) = StrongFather ; aucun Tool n'exécute de décision métier.
- **Toute écriture** (expédition, état commande) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Schéma et périmètre** (zones, règles de livraison, état des commandes et expéditions) = KindMother ; les règles sont fournies par KindMother ou dans le flux.
- **Niveau de sécurité :** tarifs/suivi 0–1, étiquettes/expéditions 2 ; cohérent WorrySentinel.
- MiyuStore inclut les Tools de base (rate, zones.resolve) pour le checkout ; MiyuShipping agrège l'ensemble des Tools livraison pour le Service complet de livraison et d'expédition.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
