# MiyuBilling — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.billing.saas`

---

## Obligations spécifiques MiyuBilling

- **Décision** (création souscription, enregistrement paiement, résiliation) = StrongFather ; aucun Tool n'exécute de décision métier.
- **Toute écriture** (souscription, facture, paiement) = **WriteIntent** vers KindMother ; aucun accès direct à la persistance depuis le kit.
- **Multi-tenant :** `tool.billing.tenant.resolve` fournit la résolution du contexte tenant (identifiant, périmètre) ; l'isolation des données par tenant relève de KindMother et Border Guard.
- **Niveau de sécurité :** WorrySentinel applique le niveau facturation ; liste factures 1–2, création souscription / enregistrement paiement 2–3.
- **Schéma et périmètre** (offres, souscriptions, factures, paiements) = KindMother ; le kit ne modifie pas le schéma.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
