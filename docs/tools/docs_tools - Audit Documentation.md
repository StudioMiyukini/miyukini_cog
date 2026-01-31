# docs/tools — Audit de la documentation

## Contexte

Audit de la documentation des Kits d'Outils (Toolkits) dans **docs/tools/** : recherche de redondances, évaluation de la qualité, propositions d’amélioration.

**Périmètre :** tous les dossiers et fichiers sous `docs/tools/`.  
**Date :** 2026-01-30.

---

## 1. Synthèse

| Critère | État | Commentaire |
|--------|------|-------------|
| **Cohérence structurelle** | Bonne | Schéma Contexte / Portée / Définition canonique / Identifiant / Outils / Gouvernance / Sécurité / KindMother / Références respecté sur la majorité des Doc Fondatrices. |
| **Redondances** | Présentes | Doublons entre index principal et liste, entre Doc Fondatrice et contrats gouvernance, et répétition du flux de gouvernance. |
| **Complétude par kit** | Inégale | Deux niveaux : kits « complets » (index + doc fondatrice + reference outils + contrat) vs kits « minimal » (doc fondatrice seule). |
| **Liens et références** | Corrects | Liens vers Glossaire, Master Butler, Équivalents ; quelques références à des fichiers « phase ultérieure ». |

---

## 2. Redondances identifiées

### 2.1 Index principal (`docs/tools/_index.md`)

- **Liste « Kits documentés » (l. 18–40)** et **tableau « Structure — Kits documentés » (l. 46–73)** décrivent les mêmes kits avec les mêmes descriptions. Une seule représentation suffit (conserver le tableau avec lien et description courte).

### 2.2 Documentation Fondatrice vs Contrat Governance

- **Section 6 (Gouvernance)** et **section 8 (Relation KindMother)** de chaque Documentation Fondatrice répètent les mêmes principes que le **Tool Governance Compliance Contract** du kit (catalogue Master Butler, WriteIntent KindMother, pas de décision métier).
- **Recommandation :** soit la Doc Fondatrice renvoie au contrat pour les obligations (« conformité détaillée : voir [Tool Governance Compliance Contract](./contracts/...) »), soit le contrat reste un rappel court et la Doc Fondatrice reste la référence ; éviter de dupliquer les mêmes phrases.

### 2.3 Flux de gouvernance

- **12+ Documentations Fondatrices** contiennent une ligne du type :  
  `Flux standard : Opérateur → BondingBrother → Master Butler → … → Exécution Tool ; toute écriture = WriteIntent KindMother.`
- Seuls le détail (ex. « règles alertes », « permissions employé ») change. Le flux générique est déjà décrit dans [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md).
- **Recommandation :** dans chaque Doc Fondatrice, remplacer le paragraphe complet par une phrase du type :  
  *« Flux de gouvernance standard (voir [Tools et Toolkits](../reference/...)). Spécificité : [règles alertes / permissions employé / etc.]. »*

### 2.4 Contrats « Tool Governance Compliance »

- Tous les contrats **Tool Governance Compliance Contract** partagent la même structure : Contexte (une phrase + ToolkitId), Obligations (3–4 puces). Seules changent l’identifiant du kit et une ou deux obligations métier.
- **Recommandation :** rédiger un **contrat-type** (ou une section dans Master Butler) qui liste les obligations communes, puis chaque kit n’a qu’un court addendum « Obligations spécifiques MiyuXXX : … ». Cela réduit la duplication et facilite les mises à jour.

### 2.5 Index de kit (`_index.md`) vs Documentation Fondatrice

- Lorsqu’un `_index.md` existe, la section **Contexte** reprend en résumé le § 1 de la Documentation Fondatrice. Ce n’est pas une redondance grave (entrée vs détail), mais on peut préciser dans l’index : *« Voir [Documentation Fondatrice](./...) pour la définition complète. »* pour clarifier la hiérarchie.

---

## 3. Qualité de la documentation actuelle

### 3.1 Points forts

- **Alignement terminologique** : recours systématique au Glossaire (Outil, Kit d’Outils, KindMother, WriteIntent, etc.).
- **Format ToolkitId** : `toolkit.<domain>.<name>` respecté partout.
- **Sections prévisibles** : Contexte, Portée, Définition canonique, Identifiant, Liste outils, Gouvernance, Sécurité, Relation KindMother, Références — navigation homogène.
- **Références croisées** : liens vers Master Butler, Équivalents métier, Glossaire.

### 3.2 Points faibles ou incohérences

| Problème | Exemple | Suggestion |
|----------|---------|------------|
| **Section Alignement MIP** absente sur une partie des kits** | MiyuHR, MiyuTreasury, MiyuComptaLedger sans § MIP ; MiyuCMS, MiyuWidgets avec § MIP | Décider si MIP est obligatoire pour tous les Toolkits documentés ; si oui, ajouter une courte section (ou « N/A – à renseigner à l’implémentation ») partout. |
| **Gouvernance : flux détaillé vs une ligne** | MiyuCMS, MiyuWidgets ont une liste numérotée (1. Opérateur … 7. Exécution) ; les autres ont une seule ligne « Flux standard » | Uniformiser : soit tous renvoient au document de référence + une ligne de spécificité, soit tous ont le même niveau de détail (recommandation : référence + spécificité). |
| **Reference Outils manquante ou « phase ultérieure »** | MiyuCMS, MiyuWidgets indiquent « sera décrit dans … Reference Outils (phase ultérieure) » ; MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling, MiyuMedia n’ont pas de Reference Outils | Marquer clairement le statut (ex. « Reference Outils : à venir ») ou créer des Reference Outils minimales (tableau ToolId / description courte) pour les kits déjà « documentés » dans l’index. |
| **Kits sans _index ni contrat** | MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling : uniquement Documentation Fondatrice | Pour homogénéiser avec les autres kits : ajouter au minimum un `_index.md` par kit pointant vers la Doc Fondatrice (et vers Reference Outils si elle existe). |

### 3.3 Complétude par type de kit

- **Kits « complets »** (index + Doc Fondatrice + Reference Outils + contrat governance, parfois autres contrats) : MiyuSQL, MiyuAuth, MiyuWeb, MiyuClock, MiyuComptaLedger, MiyuComptaReports, MiyuDeclarations, MiyuExpense, MiyuHR, MiyuInvoice, MiyuPosSales, MiyuPosInventory, MiyuPosAnalytics, MiyuPosLoyalty, MiyuPosKitchen, MiyuPosPayment, MiyuTreasury.
- **Kits « minimal »** (Documentation Fondatrice seule, pas d’index dédié ni contrat) : MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling.
- **Cas particulier** : MiyuClock a contrats + Reference Outils mais **pas de _index.md** ; le lien depuis l’index principal pointe directement vers la Documentation Fondatrice.

---

## 4. Améliorations proposées

### 4.1 Priorité haute

1. **Dédoublonner l’index principal**  
   Supprimer la liste « Kits documentés » (l. 18–40) et ne garder que le tableau « Structure — Kits documentés » (avec éventuellement une phrase d’intro du type « Les kits listés ci-dessous sont documentés dans docs/tools. »).

2. **Factoriser le flux de gouvernance**  
   Dans chaque Documentation Fondatrice, remplacer le paragraphe long du flux par un renvoi au document de référence + une ligne sur la spécificité du kit (alertes, permissions, WriteIntent, etc.).

3. **Introduire un contrat-type Tool Governance**  
   Un document unique (ou une section dans Master Butler) pour les obligations communes ; chaque kit ne garde qu’un court fichier « Obligations spécifiques » ou un tableau récapitulatif des spécificités par ToolkitId.

### 4.2 Priorité moyenne

4. **Uniformiser la présence de la section MIP**  
   Soit exiger une section « Alignement MIP » (même courte) pour tous les Toolkits, soit documenter explicitement que certains kits n’ont pas encore d’alignement MIP (avec une phrase type).

5. **Ajouter des _index.md aux kits « minimal »**  
   Créer pour MiyuCMS, MiyuMedia, MiyuWidgets, MiyuStore, MiyuShipping, MiyuBooking, MiyuBilling un `_index.md` qui pointe au minimum vers la Documentation Fondatrice (et vers Reference Outils si créée plus tard). Mettre à jour les liens dans `docs/tools/_index.md` pour pointer vers ces `_index.md` dès qu’ils existent.

6. **Ajouter un _index.md à MiyuClock**  
   Pour aligner MiyuClock sur les autres kits « complets » et permettre un lien unique depuis l’index principal.

### 4.3 Priorité basse

7. **Clarifier le rôle Doc Fondatrice vs Contrat**  
   Dans chaque Doc Fondatrice, une phrase du type : « Les obligations de conformité détaillées sont dans [Tool Governance Compliance Contract](./contracts/governance/...). » pour éviter la double définition des mêmes règles.

8. **Reference Outils pour les kits sans détail**  
   Pour MiyuCMS, MiyuWidgets, etc., soit créer une Reference Outils minimale (tableau ToolId / description), soit indiquer clairement dans la Doc Fondatrice et dans l’index : « Reference Outils : prévue (phase ultérieure) ».

9. **Tableau récapitulatif « Complétude par kit »**  
   Ajouter dans `docs/tools/_index.md` (ou dans ce rapport) un tableau : Kit | Doc Fondatrice | _index | Reference Outils | Contrat Governance | Autres contrats. Utile pour suivi et maintenance.

---

## 5. Tableau de complétude (référence)

| Kit | Doc Fondatrice | _index | Reference Outils | Contrat Governance |
|-----|----------------|--------|------------------|---------------------|
| MiyuSQL | Oui | Oui | Oui | Oui |
| MiyuAuth | Oui | Oui | Oui | Oui |
| MiyuWeb | Oui | Oui | Oui | Oui |
| MiyuClock | Oui | Non | Oui | Oui |
| MiyuCMS | Oui | Non | Non (phase ultérieure) | Non |
| MiyuMedia | Oui | Non | Non | Non |
| MiyuWidgets | Oui | Non | Non (phase ultérieure) | Non |
| MiyuStore | Oui | Non | Non | Non |
| MiyuShipping | Oui | Non | Non | Non |
| MiyuBooking | Oui | Non | Non | Non |
| MiyuBilling | Oui | Non | Non | Non |
| MiyuPosSales | Oui | Oui | Oui | Oui |
| MiyuPosInventory | Oui | Oui | Oui | Oui |
| MiyuPosAnalytics | Oui | Oui | Oui | Oui |
| MiyuPosLoyalty | Oui | Oui | Oui | Oui |
| MiyuPosKitchen | Oui | Oui | Oui | Oui |
| MiyuPosPayment | Oui | Oui | Oui | Oui |
| MiyuHR | Oui | Oui | Oui | Oui |
| MiyuInvoice | Oui | Oui | Oui | Oui |
| MiyuComptaLedger | Oui | Oui | Oui | Oui |
| MiyuComptaReports | Oui | Oui | Oui | Oui |
| MiyuDeclarations | Oui | Oui | Oui | Oui |
| MiyuExpense | Oui | Oui | Oui | Oui |
| MiyuTreasury | Oui | Oui | Oui | Oui |

---

## 6. Références

- [docs/tools/_index.md](./_index.md)
- [docs_tools - Audit Qualité Conformité Sécurité Implémentation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) — audit qualité, conformité protocoles, niveau de sécurité, guides d’implémentation
- [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Master Butler - Tool Governance Contract](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)

---

## 7. Améliorations appliquées (2026-01-30)

| Amélioration | Statut |
|--------------|--------|
| 1. Dédoublonner l'index principal | ✅ Fait |
| 2. Factoriser le flux de gouvernance | ✅ Fait (toutes les Doc Fondatrices) |
| 3. Contrat-type Tool Governance | ✅ Fait (template + contrats allégés) |
| 4. Uniformiser section MIP | ⏳ Partiel (MiyuHR, MiyuTreasury) |
| 5. _index.md kits minimal + MiyuClock | ✅ Fait (8 kits) |
| 6. Clarifier Doc Fondatrice vs Contrat | ✅ Fait (5 Doc Fondatrices) |
| 7. Reference Outils phase ultérieure | ✅ Fait (_index kits minimal) |
| 8. Tableau complétude | ✅ Fait (docs/tools/_index.md) |

---

**Date du rapport :** 2026-01-30  
**Version :** 1.1  
**Statut :** Document d’audit — améliorations appliquées (2026-01-30). Voir section 7 ci-dessous.
