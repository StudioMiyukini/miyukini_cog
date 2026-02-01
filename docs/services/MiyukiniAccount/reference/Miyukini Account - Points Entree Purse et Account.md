# Miyukini Account — Points d’entrée Purse et Account

## Contexte

Le **service COG Miyukini Account** couvre la comptabilité multi-échelle (budget perso → budgets occasionnels → comptabilité d’entreprise). Pour des raisons de **positionnement marché**, deux **points d’entrée** distincts sont proposés sous des noms commerciaux différents : **Miyukini Purse** (perso/individuel) et **Miyukini Account** (entreprise). Il s’agit du **même service COG** ; seuls le périmètre fonctionnel, les Mandats et les niveaux de sécurité diffèrent.

Ce document détaille la différenciation des points d’entrée, les périmètres fonctionnels et les règles de gouvernance associées.

## Portée / Scope

- **Périmètre** : Définition des points d’entrée Miyukini Purse et Miyukini Account — périmètres, publics, données, résidence, Mandats.
- **Hors périmètre** : Spécifications techniques des Opérateurs et Kits (référencés dans d’autres documents).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisée** — Document fondateur Miyukini Account, Niveaux sécurité et protection, Integration Services, Glossaire Miyukini, Politique de résidence. **Contraintes** : ne pas fusionner avec le Document fondateur ni avec les analyses des besoins Purse/Account ; ne pas anticiper les spécifications d’Opérateurs/Kits.

---

## 1. Un service COG, deux points d’entrée

### 1.1 Principe

| Aspect | Description |
|--------|-------------|
| **Service COG** | Miyukini Account (COG) : un seul service, une seule gouvernance (Cores), des Opérateurs et Kits communs. |
| **Points d’entrée** | **Miyukini Purse** et **Miyukini Account** : deux **marques / points d’entrée** qui exposent un sous-ensemble des capacités du service COG, avec des périmètres et des Mandats distincts. |
| **Règle** | L’utilisateur (particulier ou professionnel) accède au service COG via **un** point d’entrée selon son contexte ; les données et les permissions sont gouvernées selon ce point d’entrée. |

### 1.2 Tableau comparatif

| Critère | Miyukini Purse | Miyukini Account |
|---------|----------------|------------------|
| **Nom commercial** | Miyukini Purse | Miyukini Account |
| **Public** | Particuliers, foyers | Professionnels, associations, TPE/PME, organisateurs |
| **Périmètre fonctionnel** | Budgets personnels, budgets occasionnels (vacances, Noël, projets courts) | Comptabilité d’entreprise, devis, facturation, rapports légaux |
| **Données typiques** | Mouvements perso, catégories, objectifs, budgets par projet/occasion | Devis, factures, clients/fournisseurs, rapports, pièces comptables |
| **Niveau de sécurité minimal** | 2 (Sensitive) | 2–3 (Sensitive à Critical) |
| **Résidence** | COG de référence ou environnement utilisateur selon politique | Résidence centralisée recommandée ou obligatoire |
| **Intégration** | Autonome ou articulé avec d’autres services (ex. agenda pour rappels) | Consommée par MFS, JayRDV, etc. pour facturation et budget |
| **Conformité légale** | Pas d’exigence de facturation légale ni de comptabilité d’entreprise | Conformité facturation, TVA, rapports selon juridiction |

---

## 2. Miyukini Purse (point d’entrée perso/individuel)

### 2.1 Proposition de valeur

**Miyukini Purse** permet à un **particulier** ou un **foyer** de :

- **Tenir un budget personnel** : revenus, dépenses, catégories, objectifs, alertes.
- **Gérer des budgets occasionnels** : vacances, cadeaux de Noël, mariage, travaux, etc. — un budget dédié par projet ou occasion, avec suivi des dépenses et du solde.
- **Consulter des rapports et tableaux de bord** : synthèses, évolution, export (PDF, CSV) pour usage personnel.

Aucune exigence de facturation légale ni de comptabilité d’entreprise ; le périmètre reste **budget et suivi personnel**.

### 2.2 Capacités exposées (sous-ensemble du service COG)

| Capacité | Description |
|----------|-------------|
| **Mouvements** | Enregistrement des revenus et dépenses, catégories, date, libellé. |
| **Budgets occasionnels** | Création d’un budget par projet/occasion (vacances, Noël, etc.), suivi des dépenses et du solde. |
| **Objectifs** | Objectifs d’épargne ou de dépense par catégorie ou projet. |
| **Rapports** | Synthèses, soldes, évolution, export (PDF, CSV) pour usage personnel. |

Les capacités **devis** et **facturation légale** ne sont **pas** exposées dans le point d’entrée Purse (réservées à Miyukini Account).

### 2.3 Données et résidence

- **Niveau de sécurité** : au minimum 2 (Sensitive).
- **Résidence** : selon politique — COG de référence ou environnement utilisateur avec synchronisation sécurisée ; la copie canonique peut résider sur le COG de référence pour garantir la disponibilité et la cohérence.
- **Audit** : traçabilité des accès et des écritures (Mandat, Master Butler).

---

## 3. Miyukini Account (point d’entrée entreprise)

### 3.1 Proposition de valeur

**Miyukini Account** (marque) permet à un **professionnel**, une **association** ou une **entreprise** de :

- **Tenir une comptabilité** au sens large : grand livre, journal, ventilation par catégorie ou projet.
- **Émettre des devis** : création, envoi, suivi des devis (clients, prestataires, exposants).
- **Facturer** : émission de factures, relances, suivi des encaissements, conformité légale (TVA, numérotation, etc.).
- **Produire des rapports** : tableaux de bord, rapports légaux, export (PDF, CSV) pour comptabilité et contrôle.

Ce point d’entrée est **consommé** par les services métier (MFS, JayRDV) pour la facturation des exposants, des professionnels, etc.

### 3.2 Capacités exposées (sous-ensemble du service COG)

| Capacité | Description |
|----------|-------------|
| **Mouvements** | Enregistrement des revenus et dépenses, ventilation par catégorie, projet, client/fournisseur. |
| **Devis** | Création, envoi, suivi des devis (statut, conversion en facture). |
| **Facturation** | Émission de factures, relances, suivi des encaissements, conformité (TVA, numérotation). |
| **Rapports** | Synthèses, soldes, rapports légaux, export (PDF, CSV) pour comptabilité et contrôle. |

Les capacités **budgets occasionnels** (type Purse) peuvent être réutilisées en contexte entreprise (ex. budget par projet ou par édition) selon les besoins du service consommateur (ex. MFS).

### 3.3 Données et résidence

- **Niveau de sécurité** : 2–3 (Sensitive à Critical) selon les données (factures, moyens de paiement, pièces comptables).
- **Résidence** : résidence centralisée sur COG de référence **recommandée ou obligatoire** (voir [Politique de résidence](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)) ; les données sensibles ne doivent pas avoir pour seule copie un terminal ou un COG tiers.
- **Audit** : audit complet des lectures et écritures ; conformité PCI-DSS / réglementation pour les données de paiement.

---

## 4. Règles de gouvernance communes

| Règle | Description |
|-------|-------------|
| **Un utilisateur, un contexte** | Un utilisateur accède au service COG via **un** point d’entrée (Purse ou Account) selon son contexte (particulier vs professionnel) ; les Mandats et les permissions sont émis en fonction de ce point d’entrée. |
| **Pas de mélange non gouverné** | Les données Purse et Account sont séparées par contexte (identité, Mandat) ; un même utilisateur peut avoir un accès Purse (perso) et un accès Account (pro) sous des Mandats distincts, sans mélange des données sans gouvernance. |
| **Cores communs** | StrongFather, KindMother, Master Butler, WorrySentinel gouvernent les deux points d’entrée ; les décisions (Mandats, résidence, niveaux de sécurité) sont cohérentes. |

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Account - Document Fondateur](../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sécurité synthétique. |
| [Miyukini Account - Niveaux Securite et Protection Donnees](./Miyukini%20Account%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Détail des niveaux et mesures de protection. |
| [Miyukini Account - Integration Services](./Miyukini%20Account%20-%20Integration%20Services.md) | Intégration MFS, JayRDV, futurs services. |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : Miyukini Account — Points d’entrée Purse et Account  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de référence (points d’entrée). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
