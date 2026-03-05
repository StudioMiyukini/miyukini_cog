# JayKonta â€” Points dâ€™entrÃ©e JayBudget et JayKonta

## Contexte

Le **service COG JayKonta** couvre la comptabilitÃ© multi-Ã©chelle (budget perso â†’ budgets occasionnels â†’ comptabilitÃ© dâ€™entreprise). Pour des raisons de **positionnement marchÃ©**, deux **points dâ€™entrÃ©e** distincts sont proposÃ©s sous des noms commerciaux diffÃ©rents : **JayBudget** (perso/individuel) et **JayKonta** (entreprise). Il sâ€™agit du **mÃªme service COG** ; seuls le pÃ©rimÃ¨tre fonctionnel, les Mandats et les niveaux de sÃ©curitÃ© diffÃ¨rent.

Ce document dÃ©taille la diffÃ©renciation des points dâ€™entrÃ©e, les pÃ©rimÃ¨tres fonctionnels et les rÃ¨gles de gouvernance associÃ©es.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : DÃ©finition des points dâ€™entrÃ©e JayBudget et JayKonta â€” pÃ©rimÃ¨tres, publics, donnÃ©es, rÃ©sidence, Mandats.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques des OpÃ©rateurs et Kits (rÃ©fÃ©rencÃ©s dans dâ€™autres documents).

### Cadre de travail (protocole documentation conceptuelle)

ConformÃ©ment au [Protocole dâ€™Ã©criture de la documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisÃ©e** â€” Document fondateur JayKonta, Niveaux sÃ©curitÃ© et protection, Integration Services, Glossaire Miyukini, Politique de rÃ©sidence. **Contraintes** : ne pas fusionner avec le Document fondateur ni avec les analyses des besoins Purse/Account ; ne pas anticiper les spÃ©cifications dâ€™OpÃ©rateurs/Kits.

---

## 1. Un service COG, deux points dâ€™entrÃ©e

### 1.1 Principe

| Aspect | Description |
|--------|-------------|
| **Service COG** | JayKonta (COG) : un seul service, une seule gouvernance (Cores), des OpÃ©rateurs et Kits communs. |
| **Points dâ€™entrÃ©e** | **JayBudget** et **JayKonta** : deux **marques / points dâ€™entrÃ©e** qui exposent un sous-ensemble des capacitÃ©s du service COG, avec des pÃ©rimÃ¨tres et des Mandats distincts. |
| **RÃ¨gle** | Lâ€™utilisateur (particulier ou professionnel) accÃ¨de au service COG via **un** point dâ€™entrÃ©e selon son contexte ; les donnÃ©es et les permissions sont gouvernÃ©es selon ce point dâ€™entrÃ©e. |

### 1.2 Tableau comparatif

| CritÃ¨re | JayBudget | JayKonta |
|---------|----------------|------------------|
| **Nom commercial** | JayBudget | JayKonta |
| **Public** | Particuliers, foyers | Professionnels, associations, TPE/PME, organisateurs |
| **PÃ©rimÃ¨tre fonctionnel** | Budgets personnels, budgets occasionnels (vacances, NoÃ«l, projets courts) | ComptabilitÃ© dâ€™entreprise, devis, facturation, rapports lÃ©gaux |
| **DonnÃ©es typiques** | Mouvements perso, catÃ©gories, objectifs, budgets par projet/occasion | Devis, factures, clients/fournisseurs, rapports, piÃ¨ces comptables |
| **Niveau de sÃ©curitÃ© minimal** | 2 (Sensitive) | 2â€“3 (Sensitive Ã  Critical) |
| **RÃ©sidence** | COG de rÃ©fÃ©rence ou environnement utilisateur selon politique | RÃ©sidence centralisÃ©e recommandÃ©e ou obligatoire |
| **IntÃ©gration** | Autonome ou articulÃ© avec dâ€™autres services (ex. agenda pour rappels) | ConsommÃ©e par JayFestival, JayRDV, etc. pour facturation et budget |
| **ConformitÃ© lÃ©gale** | Pas dâ€™exigence de facturation lÃ©gale ni de comptabilitÃ© dâ€™entreprise | ConformitÃ© facturation, TVA, rapports selon juridiction |

---

## 2. JayBudget (point dâ€™entrÃ©e perso/individuel)

### 2.1 Proposition de valeur

**JayBudget** permet Ã  un **particulier** ou un **foyer** de :

- **Tenir un budget personnel** : revenus, dÃ©penses, catÃ©gories, objectifs, alertes.
- **GÃ©rer des budgets occasionnels** : vacances, cadeaux de NoÃ«l, mariage, travaux, etc. â€” un budget dÃ©diÃ© par projet ou occasion, avec suivi des dÃ©penses et du solde.
- **Consulter des rapports et tableaux de bord** : synthÃ¨ses, Ã©volution, export (PDF, CSV) pour usage personnel.

Aucune exigence de facturation lÃ©gale ni de comptabilitÃ© dâ€™entreprise ; le pÃ©rimÃ¨tre reste **budget et suivi personnel**.

### 2.2 CapacitÃ©s exposÃ©es (sous-ensemble du service COG)

| CapacitÃ© | Description |
|----------|-------------|
| **Mouvements** | Enregistrement des revenus et dÃ©penses, catÃ©gories, date, libellÃ©. |
| **Budgets occasionnels** | CrÃ©ation dâ€™un budget par projet/occasion (vacances, NoÃ«l, etc.), suivi des dÃ©penses et du solde. |
| **Objectifs** | Objectifs dâ€™Ã©pargne ou de dÃ©pense par catÃ©gorie ou projet. |
| **Rapports** | SynthÃ¨ses, soldes, Ã©volution, export (PDF, CSV) pour usage personnel. |

Les capacitÃ©s **devis** et **facturation lÃ©gale** ne sont **pas** exposÃ©es dans le point dâ€™entrÃ©e Purse (rÃ©servÃ©es Ã  JayKonta).

### 2.3 DonnÃ©es et rÃ©sidence

- **Niveau de sÃ©curitÃ©** : au minimum 2 (Sensitive).
- **RÃ©sidence** : selon politique â€” COG de rÃ©fÃ©rence ou environnement utilisateur avec synchronisation sÃ©curisÃ©e ; la copie canonique peut rÃ©sider sur le COG de rÃ©fÃ©rence pour garantir la disponibilitÃ© et la cohÃ©rence.
- **Audit** : traÃ§abilitÃ© des accÃ¨s et des Ã©critures (Mandat, Master Butler).

---

## 3. JayKonta (point dâ€™entrÃ©e entreprise)

### 3.1 Proposition de valeur

**JayKonta** (marque) permet Ã  un **professionnel**, une **association** ou une **entreprise** de :

- **Tenir une comptabilitÃ©** au sens large : grand livre, journal, ventilation par catÃ©gorie ou projet.
- **Ã‰mettre des devis** : crÃ©ation, envoi, suivi des devis (clients, prestataires, exposants).
- **Facturer** : Ã©mission de factures, relances, suivi des encaissements, conformitÃ© lÃ©gale (TVA, numÃ©rotation, etc.).
- **Produire des rapports** : tableaux de bord, rapports lÃ©gaux, export (PDF, CSV) pour comptabilitÃ© et contrÃ´le.

Ce point dâ€™entrÃ©e est **consommÃ©** par les services mÃ©tier (JayFestival, JayRDV) pour la facturation des exposants, des professionnels, etc.

### 3.2 CapacitÃ©s exposÃ©es (sous-ensemble du service COG)

| CapacitÃ© | Description |
|----------|-------------|
| **Mouvements** | Enregistrement des revenus et dÃ©penses, ventilation par catÃ©gorie, projet, client/fournisseur. |
| **Devis** | CrÃ©ation, envoi, suivi des devis (statut, conversion en facture). |
| **Facturation** | Ã‰mission de factures, relances, suivi des encaissements, conformitÃ© (TVA, numÃ©rotation). |
| **Rapports** | SynthÃ¨ses, soldes, rapports lÃ©gaux, export (PDF, CSV) pour comptabilitÃ© et contrÃ´le. |

Les capacitÃ©s **budgets occasionnels** (type Purse) peuvent Ãªtre rÃ©utilisÃ©es en contexte entreprise (ex. budget par projet ou par Ã©dition) selon les besoins du service consommateur (ex. JayFestival).

### 3.3 DonnÃ©es et rÃ©sidence

- **Niveau de sÃ©curitÃ©** : 2â€“3 (Sensitive Ã  Critical) selon les donnÃ©es (factures, moyens de paiement, piÃ¨ces comptables).
- **RÃ©sidence** : rÃ©sidence centralisÃ©e sur COG de rÃ©fÃ©rence **recommandÃ©e ou obligatoire** (voir [Politique de rÃ©sidence](..//..//..//miyukini-webway-system//reference//_index.md)) ; les donnÃ©es sensibles ne doivent pas avoir pour seule copie un terminal ou un COG tiers.
- **Audit** : audit complet des lectures et Ã©critures ; conformitÃ© PCI-DSS / rÃ©glementation pour les donnÃ©es de paiement.

---

## 4. RÃ¨gles de gouvernance communes

| RÃ¨gle | Description |
|-------|-------------|
| **Un utilisateur, un contexte** | Un utilisateur accÃ¨de au service COG via **un** point dâ€™entrÃ©e (Purse ou Account) selon son contexte (particulier vs professionnel) ; les Mandats et les permissions sont Ã©mis en fonction de ce point dâ€™entrÃ©e. |
| **Pas de mÃ©lange non gouvernÃ©** | Les donnÃ©es Purse et Account sont sÃ©parÃ©es par contexte (identitÃ©, Mandat) ; un mÃªme utilisateur peut avoir un accÃ¨s Purse (perso) et un accÃ¨s Account (pro) sous des Mandats distincts, sans mÃ©lange des donnÃ©es sans gouvernance. |
| **Cores communs** | StrongFather, KindMother, Master Butler, WorrySentinel gouvernent les deux points dâ€™entrÃ©e ; les dÃ©cisions (Mandats, rÃ©sidence, niveaux de sÃ©curitÃ©) sont cohÃ©rentes. |

---

## 5. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKonta - Document Fondateur](..//_index.md) | Contexte, besoins, positionnement, sÃ©curitÃ© synthÃ©tique. |
| [JayKonta - Niveaux Securite et Protection Donnees](_index.md) | DÃ©tail des niveaux et mesures de protection. |
| [JayKonta - Integration Services](_index.md) | IntÃ©gration JayFestival, JayRDV, futurs services. |
| [Miyukini Prompt Protocol â€” Ã‰criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole dâ€™Ã©criture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : JayKonta â€” Points dâ€™entrÃ©e Purse et Account  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (points dâ€™entrÃ©e). Enrichi selon [Protocole dâ€™Ã©criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).



