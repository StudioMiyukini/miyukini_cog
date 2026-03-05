# MiyuBilling â€” Documentation Fondatrice

## 1. Contexte

**MiyuBilling** est le **kit d'outils (Toolkit)** de facturation et d'abonnements SaaS (souscriptions, factures, enregistrement de paiements, rÃ©solution tenant multi-tenant) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de crÃ©ation, mise Ã  jour et annulation de souscriptions, de gÃ©nÃ©ration et liste de factures, d'enregistrement de paiements reÃ§us, et de rÃ©solution du contexte tenant pour l'isolation multi-tenant, alignÃ©s sur KindMother pour la persistance des donnÃ©es.

L'autoritÃ© sur les donnÃ©es (offres d'abonnement, souscriptions, factures, paiements, pÃ©rimÃ¨tres tenant) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuBilling expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (subscription.*, invoice.*, payment.record, tenant.resolve) sans remplacer KindMother ni StrongFather ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuBilling
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother (persistance souscriptions, factures, tenant)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (gateways paiement rÃ©current, gÃ©nÃ©ration PDF factures)
- Toute dÃ©cision d'autorisation de souscription, de renouvellement ou de rÃ©siliation â€” celle-ci reste du ressort de StrongFather et des Cores
- La politique multi-tenant (Border Guard, pÃ©rimÃ¨tres) â€” MiyuBilling fournit la rÃ©solution du contexte tenant (tool.tenant.resolve)

---

## 3. DÃ©finition canonique

> **MiyuBilling est une composition officielle d'outils de facturation et d'abonnements SaaS (souscriptions, factures, enregistrement de paiements, rÃ©solution tenant), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuBilling **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuBilling **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (crÃ©er/mettre Ã  jour/annuler souscription, gÃ©nÃ©rer/lister factures, enregistrer paiement, rÃ©soudre tenant) sans dÃ©cider des autorisations ni de la politique de facturation.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Toute persistance (souscriptions, factures, paiements) et toute dÃ©cision (autoriser souscription, renouveler, rÃ©silier) sont sous autoritÃ© KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.billing.saas` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `billing` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuBilling - Reference Outils](./MiyuBilling%20-%20Reference%20Outils.md). MiyuBilling est composÃ© des Tools suivants (format canonique `tool.billing.<sous-domaine>.<action>` ou `tool.billing.<action>`).

| ToolId | Description courte |
|--------|---------------------|
| `tool.billing.subscription.create` | CrÃ©e une souscription Ã  partir de donnÃ©es fournies ; WriteIntent ; dÃ©cision = StrongFather |
| `tool.billing.subscription.update` | Met Ã  jour une souscription (renouvellement, changement offre) |
| `tool.billing.subscription.cancel` | Annule / rÃ©silie une souscription |
| `tool.billing.subscription.status` | Retourne le statut d'une souscription |
| `tool.billing.invoice.generate` | GÃ©nÃ¨re une facture selon rÃ¨gles fournies |
| `tool.billing.invoice.list` | Liste les factures selon filtres fournis |
| `tool.billing.payment.record` | Enregistre un paiement reÃ§u (exÃ©cution ; dÃ©cision = StrongFather) |
| `tool.billing.tenant.resolve` | RÃ©sout le contexte tenant (identifiant, pÃ©rimÃ¨tre) pour une requÃªte ; isolation multi-tenant |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBilling en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : WorrySentinel applique le niveau de sÃ©curitÃ© facturation ; dÃ©cision (crÃ©ation souscription, enregistrement paiement, rÃ©siliation) = StrongFather ; toute Ã©criture (souscription, facture, paiement) = WriteIntent KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  3** selon opÃ©ration (liste factures 1â€“2, crÃ©ation souscription, enregistrement paiement 2â€“3) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et multi-tenant

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuBilling - Tool Governance Compliance Contract](./contracts/governance/MiyuBilling%20-%20Tool%20Governance%20Compliance%20Contract.md).

- **KindMother** est l'autoritÃ© sur les offres d'abonnement, les souscriptions, les factures et les paiements. MiyuBilling exÃ©cute des capacitÃ©s (subscription.*, invoice.*, payment.record, tenant.resolve) **sans dÃ©cider** de l'autorisation (StrongFather) ni de la politique de facturation ; les rÃ¨gles sont fournies par KindMother ou dans le flux.
- **Multi-tenant** : `tool.billing.tenant.resolve` permet de rÃ©soudre le contexte tenant (identifiant, pÃ©rimÃ¨tre) pour une requÃªte ; les requÃªtes sont ensuite filtrÃ©es par ce pÃ©rimÃ¨tre (KindMother / Master Butler). L'isolation des donnÃ©es par tenant relÃ¨ve de KindMother et Border Guard ; MiyuBilling fournit la capacitÃ© de rÃ©solution du contexte tenant.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuBilling sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). RÃ©fÃ©rence : [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

- **Domaine** : `billing` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« billing Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuBilling est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


