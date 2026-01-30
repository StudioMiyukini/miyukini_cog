# MiyuBilling — Documentation Fondatrice

## 1. Contexte

**MiyuBilling** est le **kit d'outils (Toolkit)** de facturation et d'abonnements SaaS (souscriptions, factures, enregistrement de paiements, résolution tenant multi-tenant) de l'écosystème Miyukini. Il intègre les outils de création, mise à jour et annulation de souscriptions, de génération et liste de factures, d'enregistrement de paiements reçus, et de résolution du contexte tenant pour l'isolation multi-tenant, alignés sur KindMother pour la persistance des données.

L'autorité sur les données (offres d'abonnement, souscriptions, factures, paiements, périmètres tenant) appartient à **KindMother** (Core de données, Strate 4). MiyuBilling expose des capacités d'exécution gouvernée (subscription.*, invoice.*, payment.record, tenant.resolve) sans remplacer KindMother ni StrongFather ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuBilling
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother (persistance souscriptions, factures, tenant)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (gateways paiement récurrent, génération PDF factures)
- Toute décision d'autorisation de souscription, de renouvellement ou de résiliation — celle-ci reste du ressort de StrongFather et des Cores
- La politique multi-tenant (Border Guard, périmètres) — MiyuBilling fournit la résolution du contexte tenant (tool.tenant.resolve)

---

## 3. Définition canonique

> **MiyuBilling est une composition officielle d'outils de facturation et d'abonnements SaaS (souscriptions, factures, enregistrement de paiements, résolution tenant), déclarée et gouvernée par l'environnement.**

- MiyuBilling **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuBilling **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (créer/mettre à jour/annuler souscription, générer/lister factures, enregistrer paiement, résoudre tenant) sans décider des autorisations ni de la politique de facturation.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Toute persistance (souscriptions, factures, paiements) et toute décision (autoriser souscription, renouveler, résilier) sont sous autorité KindMother (WriteIntent) et StrongFather (ALLOW/DENY).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.billing.saas` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `billing` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuBilling est composé des Tools suivants (format canonique `tool.billing.<sous-domaine>.<action>` ou `tool.billing.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) sera décrit dans MiyuBilling - Reference Outils (phase ultérieure).

| ToolId | Description courte |
|--------|---------------------|
| `tool.billing.subscription.create` | Crée une souscription à partir de données fournies ; WriteIntent ; décision = StrongFather |
| `tool.billing.subscription.update` | Met à jour une souscription (renouvellement, changement offre) |
| `tool.billing.subscription.cancel` | Annule / résilie une souscription |
| `tool.billing.subscription.status` | Retourne le statut d'une souscription |
| `tool.billing.invoice.generate` | Génère une facture selon règles fournies |
| `tool.billing.invoice.list` | Liste les factures selon filtres fournis |
| `tool.billing.payment.record` | Enregistre un paiement reçu (exécution ; décision = StrongFather) |
| `tool.billing.tenant.resolve` | Résout le contexte tenant (identifiant, périmètre) pour une requête ; isolation multi-tenant |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBilling en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : WorrySentinel applique le niveau de sécurité facturation ; décision (création souscription, enregistrement paiement, résiliation) = StrongFather ; toute écriture (souscription, facture, paiement) = WriteIntent KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 3** selon opération (liste factures 1–2, création souscription, enregistrement paiement 2–3) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon Toolkit Composition Contract) |

---

## 8. Relation avec KindMother et multi-tenant

- **KindMother** est l'autorité sur les offres d'abonnement, les souscriptions, les factures et les paiements. MiyuBilling exécute des capacités (subscription.*, invoice.*, payment.record, tenant.resolve) **sans décider** de l'autorisation (StrongFather) ni de la politique de facturation ; les règles sont fournies par KindMother ou dans le flux.
- **Multi-tenant** : `tool.billing.tenant.resolve` permet de résoudre le contexte tenant (identifiant, périmètre) pour une requête ; les requêtes sont ensuite filtrées par ce périmètre (KindMother / Master Butler). L'isolation des données par tenant relève de KindMother et Border Guard ; MiyuBilling fournit la capacité de résolution du contexte tenant.

**Référence :** [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuBilling sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `billing` — cohérent avec la projection domains.json (blocs du domaine « billing »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuBilling est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Équivalents Boutique CMS Reservation SaaS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
