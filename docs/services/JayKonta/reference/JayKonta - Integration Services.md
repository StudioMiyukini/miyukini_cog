# JayKonta â€” IntÃ©gration avec les autres services

## Contexte

Le **service COG JayKonta** expose des **OpÃ©rateurs** et **Kits dâ€™outils** pour la comptabilitÃ© multi-Ã©chelle (budgets, devis, facturation, rapports). Les **services mÃ©tier** (JayFestival, JayRDV, futurs services) **consomment** JayKonta pour Ã©viter la duplication de la logique budget/facturation et garantir une cohÃ©rence gouvernÃ©e.

Ce document dÃ©crit le **modÃ¨le dâ€™intÃ©gration** : services consommateurs identifiÃ©s, flux de donnÃ©es, responsabilitÃ©s (donnÃ©es mÃ©tier vs donnÃ©es comptables), et rÃ¨gles de rÃ©sidence et de sÃ©curitÃ©.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : IntÃ©gration de JayKonta avec JayFestival, JayRDV, JayKoa (optionnel), futurs services.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es des API et contrats dâ€™OpÃ©rateurs (rÃ©fÃ©rencÃ©s dans dâ€™autres documents).

### Cadre de travail (protocole documentation conceptuelle)

ConformÃ©ment au [Protocole dâ€™Ã©criture de la documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisÃ©e** â€” Document fondateur JayKonta, Points dâ€™entrÃ©e JayBudget et JayKonta, Document fondateur JayFestival, Document fondateur JayRDV, Politique de rÃ©sidence. **Contraintes** : ne pas fusionner avec le Document fondateur ni avec les analyses des besoins ; ne pas anticiper les contrats dâ€™API dÃ©taillÃ©s.

---

## 1. Services consommateurs identifiÃ©s

### 1.1 Tableau rÃ©capitulatif

| Service | Usage de JayKonta | DonnÃ©es concernÃ©es | Point dâ€™entrÃ©e |
|---------|---------------------------|--------------------|----------------|
| **JayFestival** | Budget par Ã©dition, devis et factures exposants, ventilation revenus/dÃ©penses | Budget Ã©dition, facturation exposants | JayKonta (entreprise) |
| **JayRDV** | Facturation professionnels, abonnements, encaissements | Factures, encaissements | JayKonta (entreprise) |
| **JayKoa** | Optionnel : rappels ou jalons liÃ©s Ã  des Ã©chÃ©ances budget (Ã©chÃ©ances factures, objectifs) | RÃ©fÃ©rences temporelles, pas de donnÃ©es financiÃ¨res canoniques | N/A (rÃ©fÃ©rences uniquement) |
| **Futurs services** | Tout service nÃ©cessitant devis, facturation ou suivi de budget | Ã€ dÃ©finir par service | Purse ou Account selon contexte |

### 1.2 JayFestival (JayFestival)

| Besoin | Description |
|--------|-------------|
| **Budget par Ã©dition** | Revenus/dÃ©penses par Ã©dition (stands, inscriptions, prestations), ventilation (Miyucptaledger, Miyuexpense, Miyucomptareports â€” alignÃ©s sur JayKonta). |
| **Devis et factures exposants** | Ã‰mission de devis et factures aux exposants (emplacements, options), suivi des encaissements. |
| **Rapports** | SynthÃ¨ses budget par Ã©dition, rapports pour les organisateurs. |

**ResponsabilitÃ©** : JayFestival dÃ©tient les **donnÃ©es mÃ©tier** (qui est exposant, quelle Ã©dition, quel stand) ; JayKonta dÃ©tient les **donnÃ©es comptables** (mouvements, devis, factures, montants) et applique les rÃ¨gles de rÃ©sidence et de sÃ©curitÃ©. La **liaison** (ex. facture â†” exposant, facture â†” Ã©dition) est gÃ©rÃ©e par des rÃ©fÃ©rences opaques ou des identifiants mÃ©tier validÃ©s par StrongFather/Master Butler.

### 1.3 JayRDV

| Besoin | Description |
|--------|-------------|
| **Facturation professionnels** | Facturation des prestations (RDV, abonnements), Ã©mission de factures, relances. |
| **Encaissements** | Suivi des encaissements, moyens de paiement (tokens, pas de stockage RIB/carte en clair). |

**ResponsabilitÃ©** : JayRDV dÃ©tient les **donnÃ©es mÃ©tier** (qui est professionnel, quels RDV, quels abonnements) ; JayKonta gÃ¨re lâ€™**Ã©mission des factures** et le **suivi des encaissements** selon Mandat et niveau de sÃ©curitÃ©.

### 1.4 JayKoa (optionnel)

| Besoin | Description |
|--------|-------------|
| **Rappels / jalons** | Rappels ou jalons liÃ©s Ã  des Ã©chÃ©ances budget (ex. Ã©chÃ©ance facture, objectif dâ€™Ã©pargne). | JayKoa ne dÃ©tient **pas** la copie canonique des donnÃ©es financiÃ¨res ; il reÃ§oit des **rÃ©fÃ©rences** (date, type dâ€™Ã©chÃ©ance, identifiant opaque) pour afficher des rappels dans lâ€™agenda. La **source de vÃ©ritÃ©** reste JayKonta / KindMother. |

---

## 2. ModÃ¨le dâ€™intÃ©gration

### 2.1 Principe

- **JayKonta** (COG) expose des **OpÃ©rateurs** et **Kits** (ex. : `budget.movements.record`, `quote.create`, `invoice.emit`, `report.balance`, `report.export`).
- Chaque **service consommateur** (JayFestival, JayRDV) :
  - dÃ©tient les **donnÃ©es mÃ©tier** (identitÃ© des acteurs, contrats, Ã©ditions, RDV) ;
  - **appelle** JayKonta pour enregistrer des mouvements, Ã©mettre des devis/factures, produire des rapports, selon **Mandat de Permission** et **permissions** (Master Butler) ;
  - **dÃ©clare** le niveau de sÃ©curitÃ© (WorrySentinel) des donnÃ©es quâ€™il transmet ; JayKonta applique les rÃ¨gles de rÃ©sidence, chiffrement et audit en consÃ©quence.
- **KindMother** : la **rÃ©sidence** des donnÃ©es sensibles (donnÃ©es financiÃ¨res personnelles ou dâ€™entreprise) est dÃ©finie par le [contrat du service](..//..//..//miyukini-webway-system//reference//_index.md) et le point dâ€™entrÃ©e (Purse vs Account). Le **COG de rÃ©fÃ©rence** pour les donnÃ©es Account (entreprise) est dÃ©signÃ© par le contrat (ex. COG du service ou COG de lâ€™organisateur pour JayFestival).

### 2.2 Flux typiques

| Flux | Acteur initiateur | JayKonta | RÃ©sultat |
|------|------------------|------------------|----------|
| **CrÃ©ation devis (JayFestival)** | JayFestival (organisateur) | `quote.create` avec rÃ©fÃ©rence exposant/Ã©dition, montants, TVA | Devis enregistrÃ©, identifiant retournÃ© |
| **Ã‰mission facture (JayFestival)** | JayFestival | `invoice.emit` avec rÃ©fÃ©rence devis ou donnÃ©es facture, client (exposant) | Facture Ã©mise, suivi relances/encaissements |
| **Enregistrement mouvement budget (JayFestival)** | JayFestival | `budget.movements.record` par Ã©dition, catÃ©gorie, montant | Mouvement enregistrÃ©, rapports mis Ã  jour |
| **Facturation professionnel (JayRDV)** | JayRDV | `invoice.emit` avec rÃ©fÃ©rence professionnel, prestations, montants | Facture Ã©mise, encaissement suivi |
| **Rapport balance (Purse ou Account)** | Utilisateur (Purse ou Account) | `report.balance` selon Mandat et pÃ©rimÃ¨tre | SynthÃ¨se, export PDF/CSV selon niveau autorisÃ© |

### 2.3 RÃ¨gles dâ€™intÃ©gritÃ©

| RÃ¨gle | Description |
|-------|-------------|
| **INT-1** | Les services consommateurs ne dupliquent pas la logique devis/facturation/budget ; ils appellent les OpÃ©rateurs JayKonta. |
| **INT-2** | La liaison entre entitÃ©s mÃ©tier (exposant, professionnel, Ã©dition) et entitÃ©s comptables (devis, facture, mouvement) est gÃ©rÃ©e par des **rÃ©fÃ©rences** validÃ©es par StrongFather/Master Butler ; JayKonta ne dÃ©tient pas la copie canonique des donnÃ©es mÃ©tier des consommateurs. |
| **INT-3** | Toute donnÃ©e transmise Ã  JayKonta (montants, identitÃ© client/fournisseur) est classÃ©e et protÃ©gÃ©e selon le niveau WorrySentinel dÃ©clarÃ© ; audit et rÃ©sidence sâ€™appliquent. |
| **INT-4** | En Ã©tat de confiance dÃ©gradÃ© (T2â€“T4), les capacitÃ©s dâ€™Ã©criture ou dâ€™export peuvent Ãªtre restreintes ; les services consommateurs doivent gÃ©rer les refus ou reports (Caring Nanny, WorrySentinel). |

---

## 3. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKonta - Document Fondateur](..//_index.md) | Contexte, besoins, positionnement, sÃ©curitÃ©. |
| [JayKonta - Points Entree JayBudget et JayKonta](./JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | PÃ©rimÃ¨tres JayBudget vs JayKonta. |
| [JayFestival - Document Fondateur](../../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (budget Ã©dition, facturation exposants). |
| [JayRDV - Document Fondateur](../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md) | COG de rÃ©fÃ©rence, rÃ©sidence centralisÃ©e. |
| [Miyukini Prompt Protocol â€” Ã‰criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole dâ€™Ã©criture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : JayKonta â€” IntÃ©gration avec les autres services  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (intÃ©gration). Enrichi selon [Protocole dâ€™Ã©criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).



