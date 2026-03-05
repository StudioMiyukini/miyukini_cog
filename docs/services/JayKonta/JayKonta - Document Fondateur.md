# JayKonta â€” Document fondateur

## Contexte

**JayKonta** est le **service Miyukini unifiÃ© du domaine comptabilitÃ© et budget** au sein de lâ€™Ã©cosystÃ¨me COG. Il couvre une **comptabilitÃ© multi-Ã©chelle** : de la tenue de budget personnelle et des budgets occasionnels (vacances, cadeaux de NoÃ«l) Ã  la comptabilitÃ© dâ€™entreprise au sens large (devis, facturation, suivi des revenus et dÃ©penses).

**Un seul service COG**, avec **deux points dâ€™entrÃ©e** distincts pour des raisons de positionnement marchÃ© :

| Point dâ€™entrÃ©e | Nom commercial | PÃ©rimÃ¨tre |
|-----------------|----------------|-----------|
| **Perso / Individuel** | **JayBudget** | Budgets personnels, budgets occasionnels (vacances, cadeaux, projets courts). |
| **Entreprise** | **JayKonta** | ComptabilitÃ© dâ€™entreprise, devis, facturation, suivi des revenus et dÃ©penses, rapports. |

Les deux points dâ€™entrÃ©e sâ€™appuient sur les **mÃªmes OpÃ©rateurs et Kits** du service COG ; seuls le pÃ©rimÃ¨tre fonctionnel, les Mandats et les niveaux de sÃ©curitÃ© diffÃ¨rent selon le contexte (individuel vs entreprise).

Ce document est le **document fondateur** du service : il en fixe la raison dâ€™Ãªtre, les besoins stratÃ©giques, le positionnement, lâ€™intÃ©gration avec les autres services et les niveaux de sÃ©curitÃ© associÃ©s Ã  la sensibilitÃ© des donnÃ©es. Il sâ€™adresse aux Ã©quipes produit, technique, sÃ©curitÃ© et aux parties prenantes.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : DÃ©finition du service JayKonta â€” besoins, positionnement stratÃ©gique, points dâ€™entrÃ©e Purse / Account, intÃ©gration multi-services, niveaux de sÃ©curitÃ© et solutions de protection.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es (API, schÃ©mas), implÃ©mentation des crates (rÃ©fÃ©rencÃ©s dans dâ€™autres documents).
- **RÃ©fÃ©rences** : Glossaire Miyukini, [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//miyukini-webway-system//reference//_index.md), [Niveaux de sÃ©curitÃ© et protection](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

### Cadre de travail (protocole documentation conceptuelle)

ConformÃ©ment au [Protocole dâ€™Ã©criture de la documentation conceptuelle](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) :

| Ã‰lÃ©ment | Description |
|--------|-------------|
| **Documentation autorisÃ©e (liste fermÃ©e)** | Glossaire Miyukini ; Politique de rÃ©sidence des donnÃ©es sensibles ; Document fondateur JayFestival ; Document fondateur JayRDV ; Niveaux sÃ©curitÃ© et protection (JayKonta) ; Points dâ€™entrÃ©e JayBudget et JayKonta ; Integration Services. |
| **Ce document ne fusionne pas** | Avec les documents rÃ©fÃ©rence (Niveaux sÃ©curitÃ©, Points dâ€™entrÃ©e, Integration) â€” ils restent distincts. |
| **Ce document nâ€™anticipe pas** | Les spÃ©cifications dâ€™OpÃ©rateurs/Kits ni lâ€™implÃ©mentation. |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| âŒ **Ne pas anticiper** | Les Ã©tapes suivantes (spÃ©cifications techniques, contrats dâ€™API, implÃ©mentation) ne sont pas rÃ©digÃ©es dans ce document. |
| âŒ **Ne pas fusionner** | Ce document ne fusionne pas avec les documents rÃ©fÃ©rence (Niveaux sÃ©curitÃ©, Points dâ€™entrÃ©e, Integration Services). |
| âŒ **Ne pas corriger hors pÃ©rimÃ¨tre** | Toute modification des services consommateurs (JayFestival, JayRDV) ou du Glossaire reste hors pÃ©rimÃ¨tre de ce document fondateur. |
| âœ… **Source de vÃ©ritÃ©** | Ce document est la **rÃ©fÃ©rence** pour la raison dâ€™Ãªtre, le positionnement et les rÃ¨gles de sÃ©curitÃ© du service JayKonta. |

### DÃ©cisions structurantes (mini log)

| Id | DÃ©cision | Justification |
|----|----------|---------------|
| **DS-01** | Un seul service COG, deux points dâ€™entrÃ©e (Purse / Account) sous noms commerciaux distincts. | Positionnement marchÃ© : perso vs entreprise ; Ã©vite duplication de la logique tout en diffÃ©renciant lâ€™offre. |
| **DS-02** | DonnÃ©es financiÃ¨res au minimum niveau 2 (Sensitive) ; niveau 3 pour moyens de paiement et piÃ¨ces comptables. | Alignement avec Politique de rÃ©sidence et sensibilitÃ© des donnÃ©es (revenus, dÃ©penses, factures, RIB). |
| **DS-03** | RÃ©sidence Purse : COG de rÃ©fÃ©rence ou environnement utilisateur selon politique ; Account : rÃ©sidence centralisÃ©e recommandÃ©e ou obligatoire. | Purse peut accepter une copie locale pour usage perso ; Account doit garantir disponibilitÃ© pour acteurs mÃ©tier (JayFestival, JayRDV). |
| **DS-04** | Les services JayFestival et JayRDV consomment les OpÃ©rateurs JayKonta (quote, invoice, budget) ; ils dÃ©tiennent les donnÃ©es mÃ©tier (exposant, professionnel), JayKonta dÃ©tient les donnÃ©es comptables. | RÃ©duction de la duplication ; responsabilitÃ©s claires (donnÃ©es mÃ©tier vs comptables). |
| **DÃ©pendance critique** | Le Document fondateur JayFestival et JayRDV doivent exister pour que les sections IntÃ©gration restent cohÃ©rentes ; les contrats dâ€™intÃ©gration seront formalisÃ©s dans un document dÃ©diÃ© (Integration Services). | â€” |

---

## 1. Besoins stratÃ©giques

### 1.1 Origine du besoin

La dÃ©finition des services **Miyukini Festival Service** (budget par Ã©dition, facturation exposants, devis), **JayRDV** (facturation professionnels, abonnements) et les usages **individuels** (budget perso, vacances, cadeaux) ont fait Ã©merger un **besoin transversal** :

- **JayFestival** : budget par Ã©dition, revenus/dÃ©penses, devis et factures exposants (Miyuinvoice), ventilation â€” besoin dÃ©jÃ  citÃ© dans le Document fondateur JayFestival.
- **JayRDV** : facturation des professionnels, abonnements, encaissements.
- **Individuels** : tenue de budget personnelle, budgets par projet ou occasion (vacances, NoÃ«l, mariage), sans exigence de facturation lÃ©gale ni comptabilitÃ© dâ€™entreprise.

Sans service unifiÃ©, chaque service dÃ©finit sa propre logique budget/facturation, et lâ€™utilisateur final nâ€™a pas dâ€™offre cohÃ©rente pour gÃ©rer son budget perso et occasionnel dans le mÃªme Ã©cosystÃ¨me que les services mÃ©tier.

### 1.2 Besoins fonctionnels identifiÃ©s

| Besoin | Description | Consommateurs typiques |
|--------|-------------|-------------------------|
| **Budget personnel** | Suivi des revenus et dÃ©penses personnels, catÃ©gories, objectifs, alertes. | JayBudget (individuels) |
| **Budgets occasionnels** | Budget dÃ©diÃ© par projet ou occasion : vacances, cadeaux de NoÃ«l, mariage, travaux. | JayBudget (individuels) |
| **Grand livre / journal** | Enregistrement des mouvements, ventilation par catÃ©gorie ou projet, historique. | Purse et Account |
| **Devis** | CrÃ©ation, envoi, suivi des devis (entreprises, prestataires, exposants). | JayKonta, JayFestival, JayRDV |
| **Facturation** | Ã‰mission de factures, relances, suivi des encaissements, conformitÃ© lÃ©gale. | JayKonta, JayFestival, JayRDV |
| **Rapports et tableaux de bord** | SynthÃ¨ses, soldes, Ã©volution, export (PDF, CSV), niveaux de dÃ©tail selon contexte. | Purse et Account |
| **IntÃ©gration multi-services** | Consommation par JayFestival (budget Ã©dition, facturation exposants), JayRDV (facturation pro), autres services. | JayFestival, JayRDV, futurs services |

### 1.3 Besoin stratÃ©gique de fond

> **Un service de comptabilitÃ© multi-Ã©chelle permet de couvrir le continuum budget perso â†’ budgets occasionnels â†’ comptabilitÃ© dâ€™entreprise, avec une base COG unique, deux marques dâ€™entrÃ©e (Purse / Account) et des niveaux de sÃ©curitÃ© adaptÃ©s Ã  la sensibilitÃ© des donnÃ©es financiÃ¨res.**

La crÃ©ation de **JayKonta** (service COG) avec **JayBudget** et **JayKonta** comme points dâ€™entrÃ©e rÃ©pond Ã  la fois au besoin des services mÃ©tier (JayFestival, JayRDV) et Ã  lâ€™offre marchÃ© pour les particuliers et les entreprises.

---

## 2. Positionnement stratÃ©gique

### 2.1 Raison dâ€™Ãªtre

**JayKonta** (service COG) a pour objectif de :

- **Centraliser le domaine Â« budget et comptabilitÃ© Â»** : mouvements, catÃ©gories, projets, devis, factures, rapports, avec des rÃ¨gles de gouvernance et de sÃ©curitÃ© cohÃ©rentes.
- **Exposer des OpÃ©rateurs et Kits rÃ©utilisables** : les services mÃ©tier (JayFestival, JayRDV, etc.) et les points dâ€™entrÃ©e Purse / Account consomment le mÃªme socle.
- **DiffÃ©rencier par point dâ€™entrÃ©e** : **JayBudget** pour le perso et les budgets occasionnels ; **JayKonta** pour lâ€™entreprise (devis, facturation, comptabilitÃ©). MÃªme COG, pÃ©rimÃ¨tres et Mandats diffÃ©rents.

### 2.2 Points dâ€™entrÃ©e : JayBudget vs JayKonta

| Aspect | JayBudget | JayKonta |
|--------|----------------|------------------|
| **Public** | Particuliers, foyers. | Professionnels, associations, TPE/PME, organisateurs. |
| **PÃ©rimÃ¨tre** | Budgets personnels, budgets occasionnels (vacances, NoÃ«l, projets). | ComptabilitÃ© dâ€™entreprise, devis, facturation, rapports lÃ©gaux. |
| **DonnÃ©es** | DonnÃ©es personnelles financiÃ¨res (niveau 2). | DonnÃ©es mÃ©tier et lÃ©gales (niveau 2â€“3). |
| **RÃ©sidence** | COG de rÃ©fÃ©rence selon politique (option perso : terminal ou COG). | RÃ©sidence centralisÃ©e recommandÃ©e ou obligatoire (niveau 2+). |
| **IntÃ©gration** | Peut rester autonome ou sâ€™articuler avec dâ€™autres services (ex. agenda pour rappels). | ConsommÃ©e par JayFestival, JayRDV, etc. pour facturation et budget. |

**RÃ¨gle** : le mÃªme **service COG** expose les capacitÃ©s ; les **points dâ€™entrÃ©e** (Purse, Account) dÃ©terminent le pÃ©rimÃ¨tre fonctionnel, les Mandats et le niveau de sÃ©curitÃ© appliquÃ© (WorrySentinel).

DÃ©tail : [JayKonta - Points Entree JayBudget et JayKonta](./reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md).

### 2.3 Positionnement dans la pyramide Miyukini

| Ã‰lÃ©ment | RÃ´le |
|--------|------|
| **Kernel / KindMother** | Persistance, WriteIntent, cohÃ©rence des donnÃ©es. |
| **JayKonta (COG)** | **Service** de domaine Â« budget et comptabilitÃ© Â» : orchestration, mouvements, devis, factures, rapports, gouvernÃ© par Cores. |
| **JayBudget** | **Point dâ€™entrÃ©e** perso/individuel du service COG (mÃªme OpÃ©rateurs, pÃ©rimÃ¨tre Purse). |
| **JayKonta (marque)** | **Point dâ€™entrÃ©e** entreprise du service COG (devis, facturation, comptabilitÃ©). |
| **JayFestival / JayRDV** | **Services mÃ©tier** qui sâ€™appuient sur JayKonta pour budget, devis et facturation. |

### 2.4 Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather (dÃ©cisions), KindMother (donnÃ©es), Master Butler (permissions), WorrySentinel (niveaux de sÃ©curitÃ©, Ã©tats de confiance). |
| **Un COG, deux entrÃ©es** | Un seul service COG ; JayBudget et JayKonta sont des points dâ€™entrÃ©e (marques, pÃ©rimÃ¨tres, Mandats), pas des services COG distincts. |
| **SÃ©curitÃ© par niveau** | Les donnÃ©es financiÃ¨res sont classÃ©es par niveau de sensibilitÃ© (WorrySentinel 0â€“4) ; les solutions de protection (rÃ©sidence, chiffrement, audit) sont alignÃ©es sur ces niveaux. |
| **RÃ©utilisabilitÃ©** | JayFestival, JayRDV et futurs services consomment les OpÃ©rateurs et Kits du service COG pour Ã©viter la duplication (devis, factures, budget). |

---

## 3. IntÃ©gration avec les autres services

### 3.1 Services consommateurs identifiÃ©s

| Service | Usage de JayKonta | DonnÃ©es concernÃ©es |
|---------|---------------------------|---------------------|
| **Miyukini Festival Service** | Budget par Ã©dition, devis et factures exposants, ventilation revenus/dÃ©penses. | Budget Ã©dition, facturation exposants |
| **JayRDV** | Facturation professionnels, abonnements, encaissements. | Factures, encaissements |
| **JayKoa** | Optionnel : rappels ou jalons liÃ©s Ã  des Ã©chÃ©ances budget (Ã©chÃ©ances factures, objectifs). | RÃ©fÃ©rences temporelles, pas de donnÃ©es financiÃ¨res canoniques |
| **Futurs services** | Tout service nÃ©cessitant devis, facturation ou suivi de budget. | Ã€ dÃ©finir par service |

### 3.2 ModÃ¨le dâ€™intÃ©gration

- **JayKonta** (COG) expose des **OpÃ©rateurs** et **Kits dâ€™outils** (ex. : `budget.movements.record`, `quote.create`, `invoice.emit`, `report.balance`, `report.export`).
- Chaque **service consommateur** (JayFestival, JayRDV) :
  - dÃ©tient les **donnÃ©es mÃ©tier** (qui est exposant, qui est professionnel, quel contrat) ;
  - **appelle** JayKonta pour enregistrer des mouvements, Ã©mettre des devis/factures, produire des rapports, selon Mandat et permissions ;
  - **reste responsable** du niveau de sÃ©curitÃ© des donnÃ©es quâ€™il transmet (WorrySentinel).
- **KindMother** : la rÃ©sidence des donnÃ©es sensibles (donnÃ©es financiÃ¨res personnelles ou dâ€™entreprise) est dÃ©finie par le [contrat du service](..//..//miyukini-webway-system//reference//_index.md) et le point dâ€™entrÃ©e (Purse vs Account).

DÃ©tail : [JayKonta - Integration Services](./reference/JayKonta%20-%20Integration%20Services.md).

### 3.3 BÃ©nÃ©fice multi-services

- **CohÃ©rence** : une seule logique devis/facturation/budget pour tout lâ€™Ã©cosystÃ¨me.
- **RÃ©duction de la duplication** : JayFestival et JayRDV ne rÃ©implÃ©mentent pas la facturation.
- **ExpÃ©rience utilisateur** : un utilisateur peut avoir un budget perso (Purse) et, en tant que professionnel ou organisateur, utiliser les mÃªmes bases (Account) pour la facturation et la comptabilitÃ©.

---

## 4. Niveaux de sÃ©curitÃ© et solutions de protection

### 4.1 SensibilitÃ© des donnÃ©es financiÃ¨res

Les donnÃ©es traitÃ©es par JayKonta sont **hautement sensibles** (revenus, dÃ©penses, factures, coordonnÃ©es bancaires ou moyens de paiement, identitÃ© des clients/fournisseurs). La classification et les mesures de protection sont **critiques**.

| Type de donnÃ©e | Exemple | SensibilitÃ© | Niveau WorrySentinel typique |
|----------------|---------|-------------|------------------------------|
| **AgrÃ©gats anonymisÃ©s** | Totaux par catÃ©gorie sans lien identitÃ© | Faible | 0â€“1 |
| **Mouvements personnels** | Revenus, dÃ©penses, catÃ©gories (Purse) | Sensible | 2 |
| **Budgets occasionnels** | Montants et libellÃ©s (vacances, NoÃ«l) | Sensible | 2 |
| **Devis et factures** | Montants, TVA, identitÃ© client/fournisseur | Sensible Ã  critique | 2â€“3 |
| **DonnÃ©es de paiement** | RÃ©fÃ©rences de moyen de paiement, RIB, historique encaissements | Critique | 3 |
| **ComptabilitÃ© lÃ©gale** | PiÃ¨ces comptables, rapports soumis Ã  contrÃ´le | Critique | 3 |

### 4.2 Niveaux de sÃ©curitÃ© (rappel Glossaire)

| Niveau | Nom | Description |
|--------|-----|-------------|
| **0** | Public | DonnÃ©es publiques, aucune contrainte stricte |
| **1** | Standard | DonnÃ©es standard, contraintes de base |
| **2** | Sensitive | DonnÃ©es sensibles, contraintes renforcÃ©es |
| **3** | Critical | DonnÃ©es critiques, contraintes strictes |
| **4** | Highest | SÃ©curitÃ© maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sÃ©curitÃ© et les Ã©tats de confiance ; Master Butler gÃ¨re les permissions ; StrongFather Ã©met les Mandats.

### 4.3 Solutions de protection par niveau

| Niveau | Mesures de protection |
|--------|------------------------|
| **0 â€“ Public** | Aucune mesure spÃ©cifique ; pas de donnÃ©es financiÃ¨res personnelles ou mÃ©tier. |
| **1 â€“ Standard** | ContrÃ´le dâ€™accÃ¨s (Mandat, Master Butler) ; traÃ§abilitÃ© des accÃ¨s ; agrÃ©gats sans identitÃ©. |
| **2 â€“ Sensitive** | RÃ©sidence centralisÃ©e sur COG de rÃ©fÃ©rence (selon [Politique de rÃ©sidence](..//..//miyukini-webway-system//reference//_index.md)) ; accÃ¨s via Visite gouvernÃ©e ou session ; audit des lectures/Ã©critures ; chiffrement en transit. |
| **3 â€“ Critical** | RÃ©sidence centralisÃ©e obligatoire ; chiffrement au repos et en transit ; audit complet ; rÃ©vocation immÃ©diate possible (StrongFather, WorrySentinel). |
| **4 â€“ Highest** | Contraintes maximales ; isolement renforcÃ© ; procÃ©dures dâ€™accÃ¨s exceptionnel (TAMR, MiyukiniAdmin). |

Pour **JayBudget** (perso) : les donnÃ©es sont au moins niveau 2 ; la rÃ©sidence peut Ãªtre sur le COG de rÃ©fÃ©rence ou, selon contrat, sur lâ€™environnement utilisateur avec synchronisation sÃ©curisÃ©e. Pour **JayKonta** (entreprise) : les donnÃ©es facturation/comptabilitÃ© sont niveau 2â€“3 ; rÃ©sidence centralisÃ©e recommandÃ©e ou obligatoire.

RÃ©fÃ©rence dÃ©taillÃ©e : [JayKonta - Niveaux Securite et Protection Donnees](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

### 4.4 RÃ¨gles de sÃ©curitÃ© spÃ©cifiques au service

| RÃ¨gle | Description |
|-------|-------------|
| **MAC-SEC-1** | Les donnÃ©es financiÃ¨res personnelles ou mÃ©tier (mouvements, factures, devis, moyens de paiement) sont classÃ©es au minimum niveau 2 (Sensitive) ; les flux sont chiffrÃ©s et soumis Ã  Mandat. |
| **MAC-SEC-2** | La rÃ©sidence des donnÃ©es sensibles (niveau 2+) est dÃ©finie par le contrat du service et le point dâ€™entrÃ©e (Purse vs Account) ; COG de rÃ©fÃ©rence dÃ©signÃ© pour Account et, selon politique, pour Purse. |
| **MAC-SEC-3** | Aucune donnÃ©e de paiement (RIB, cartes, tokens) nâ€™est stockÃ©e en clair ; rÃ©fÃ©rencement par token ou identifiant opaque, conformitÃ© PCI-DSS / rÃ©glementation en vigueur. |
| **MAC-SEC-4** | Toute Ã©mission de devis ou facture par un service consommateur (JayFestival, JayRDV) transite par les OpÃ©rateurs JayKonta avec audit et niveau de sÃ©curitÃ© dÃ©clarÃ©. |
| **MAC-SEC-5** | En Ã©tat de confiance dÃ©gradÃ© (T2â€“T4), les capacitÃ©s dâ€™Ã©criture ou dâ€™export peuvent Ãªtre restreintes (Caring Nanny, WorrySentinel). |

---

## 5. Prochaines Ã©tapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser (interne / partenaires).
2. **SpÃ©cifier** : Documenter les OpÃ©rateurs et Kits JayKonta (mouvements, devis, factures, rapports) et leurs Contrats dâ€™Ã©quipe.
3. **Points dâ€™entrÃ©e** : Finaliser les pÃ©rimÃ¨tres fonctionnels et UX JayBudget et JayKonta (voir [Points Entree Purse et Account](reference//_index.md)).
4. **SÃ©curitÃ©** : Finaliser le document [Niveaux SÃ©curitÃ© et Protection](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) et lâ€™alignement avec la Politique de rÃ©sidence.
5. **IntÃ©gration** : Formaliser les contrats dâ€™intÃ©gration avec JayFestival et JayRDV (voir [Integration Services](./reference/JayKonta%20-%20Integration%20Services.md)).
6. **ImplÃ©mentation** : DÃ©velopper les OpÃ©rateurs et Kits en sâ€™appuyant sur KindMother, WorrySentinel, Master Butler.

---

## 6. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie (OpÃ©rateur, Mandat, COG, Niveaux de sÃ©curitÃ©, WorrySentinel). |
| [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//miyukini-webway-system//reference//_index.md) | RÃ©sidence centralisÃ©e, COG de rÃ©fÃ©rence, niveaux 2+. |
| [JayKonta - Niveaux Securite et Protection Donnees](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | DÃ©tail des niveaux et mesures de protection pour le service. |
| [JayKonta - Points Entree JayBudget et JayKonta](./reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | PÃ©rimÃ¨tres et diffÃ©renciation JayBudget / JayKonta. |
| [JayKonta - Integration Services](./reference/JayKonta%20-%20Integration%20Services.md) | SchÃ©mas dâ€™intÃ©gration JayFestival, JayRDV, futurs services. |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (budget Ã©dition, facturation exposants). |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Miyukini Prompt Protocol â€” Ã‰criture documentation conceptuelle](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole dâ€™Ã©criture de la documentation conceptuelle et contractuelle (cadre de travail, contraintes, dÃ©cisions structurantes). |

---

**Document** : JayKonta â€” Document fondateur  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document fondateur â€” rÃ©fÃ©rence pour le service (besoins, positionnement, points dâ€™entrÃ©e Purse/Account, intÃ©gration multi-services, sÃ©curitÃ©). Enrichi selon [Protocole dâ€™Ã©criture documentation conceptuelle](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).



