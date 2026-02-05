# JayKonta — Document fondateur

## Contexte

**JayKonta** est le **service Miyukini unifié du domaine comptabilité et budget** au sein de l’écosystème COG. Il couvre une **comptabilité multi-échelle** : de la tenue de budget personnelle et des budgets occasionnels (vacances, cadeaux de Noël) à la comptabilité d’entreprise au sens large (devis, facturation, suivi des revenus et dépenses).

**Un seul service COG**, avec **deux points d’entrée** distincts pour des raisons de positionnement marché :

| Point d’entrée | Nom commercial | Périmètre |
|-----------------|----------------|-----------|
| **Perso / Individuel** | **JayBudget** | Budgets personnels, budgets occasionnels (vacances, cadeaux, projets courts). |
| **Entreprise** | **JayKonta** | Comptabilité d’entreprise, devis, facturation, suivi des revenus et dépenses, rapports. |

Les deux points d’entrée s’appuient sur les **mêmes Opérateurs et Kits** du service COG ; seuls le périmètre fonctionnel, les Mandats et les niveaux de sécurité diffèrent selon le contexte (individuel vs entreprise).

Ce document est le **document fondateur** du service : il en fixe la raison d’être, les besoins stratégiques, le positionnement, l’intégration avec les autres services et les niveaux de sécurité associés à la sensibilité des données. Il s’adresse aux équipes produit, technique, sécurité et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayKonta — besoins, positionnement stratégique, points d’entrée Purse / Account, intégration multi-services, niveaux de sécurité et solutions de protection.
- **Hors périmètre** : Spécifications techniques détaillées (API, schémas), implémentation des crates (référencés dans d’autres documents).
- **Références** : Glossaire Miyukini, [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md), [Niveaux de sécurité et protection](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) :

| Élément | Description |
|--------|-------------|
| **Documentation autorisée (liste fermée)** | Glossaire Miyukini ; Politique de résidence des données sensibles ; Document fondateur JayFestival ; Document fondateur JayRDV ; Niveaux sécurité et protection (JayKonta) ; Points d’entrée JayBudget et JayKonta ; Integration Services. |
| **Ce document ne fusionne pas** | Avec les documents référence (Niveaux sécurité, Points d’entrée, Integration) — ils restent distincts. |
| **Ce document n’anticipe pas** | Les spécifications d’Opérateurs/Kits ni l’implémentation. |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Ne pas anticiper** | Les étapes suivantes (spécifications techniques, contrats d’API, implémentation) ne sont pas rédigées dans ce document. |
| ❌ **Ne pas fusionner** | Ce document ne fusionne pas avec les documents référence (Niveaux sécurité, Points d’entrée, Integration Services). |
| ❌ **Ne pas corriger hors périmètre** | Toute modification des services consommateurs (JayFestival, JayRDV) ou du Glossaire reste hors périmètre de ce document fondateur. |
| ✅ **Source de vérité** | Ce document est la **référence** pour la raison d’être, le positionnement et les règles de sécurité du service JayKonta. |

### Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-01** | Un seul service COG, deux points d’entrée (Purse / Account) sous noms commerciaux distincts. | Positionnement marché : perso vs entreprise ; évite duplication de la logique tout en différenciant l’offre. |
| **DS-02** | Données financières au minimum niveau 2 (Sensitive) ; niveau 3 pour moyens de paiement et pièces comptables. | Alignement avec Politique de résidence et sensibilité des données (revenus, dépenses, factures, RIB). |
| **DS-03** | Résidence Purse : COG de référence ou environnement utilisateur selon politique ; Account : résidence centralisée recommandée ou obligatoire. | Purse peut accepter une copie locale pour usage perso ; Account doit garantir disponibilité pour acteurs métier (JayFestival, JayRDV). |
| **DS-04** | Les services JayFestival et JayRDV consomment les Opérateurs JayKonta (quote, invoice, budget) ; ils détiennent les données métier (exposant, professionnel), JayKonta détient les données comptables. | Réduction de la duplication ; responsabilités claires (données métier vs comptables). |
| **Dépendance critique** | Le Document fondateur JayFestival et JayRDV doivent exister pour que les sections Intégration restent cohérentes ; les contrats d’intégration seront formalisés dans un document dédié (Integration Services). | — |

---

## 1. Besoins stratégiques

### 1.1 Origine du besoin

La définition des services **Miyukini Festival Service** (budget par édition, facturation exposants, devis), **JayRDV** (facturation professionnels, abonnements) et les usages **individuels** (budget perso, vacances, cadeaux) ont fait émerger un **besoin transversal** :

- **JayFestival** : budget par édition, revenus/dépenses, devis et factures exposants (Miyuinvoice), ventilation — besoin déjà cité dans le Document fondateur JayFestival.
- **JayRDV** : facturation des professionnels, abonnements, encaissements.
- **Individuels** : tenue de budget personnelle, budgets par projet ou occasion (vacances, Noël, mariage), sans exigence de facturation légale ni comptabilité d’entreprise.

Sans service unifié, chaque service définit sa propre logique budget/facturation, et l’utilisateur final n’a pas d’offre cohérente pour gérer son budget perso et occasionnel dans le même écosystème que les services métier.

### 1.2 Besoins fonctionnels identifiés

| Besoin | Description | Consommateurs typiques |
|--------|-------------|-------------------------|
| **Budget personnel** | Suivi des revenus et dépenses personnels, catégories, objectifs, alertes. | JayBudget (individuels) |
| **Budgets occasionnels** | Budget dédié par projet ou occasion : vacances, cadeaux de Noël, mariage, travaux. | JayBudget (individuels) |
| **Grand livre / journal** | Enregistrement des mouvements, ventilation par catégorie ou projet, historique. | Purse et Account |
| **Devis** | Création, envoi, suivi des devis (entreprises, prestataires, exposants). | JayKonta, JayFestival, JayRDV |
| **Facturation** | Émission de factures, relances, suivi des encaissements, conformité légale. | JayKonta, JayFestival, JayRDV |
| **Rapports et tableaux de bord** | Synthèses, soldes, évolution, export (PDF, CSV), niveaux de détail selon contexte. | Purse et Account |
| **Intégration multi-services** | Consommation par JayFestival (budget édition, facturation exposants), JayRDV (facturation pro), autres services. | JayFestival, JayRDV, futurs services |

### 1.3 Besoin stratégique de fond

> **Un service de comptabilité multi-échelle permet de couvrir le continuum budget perso → budgets occasionnels → comptabilité d’entreprise, avec une base COG unique, deux marques d’entrée (Purse / Account) et des niveaux de sécurité adaptés à la sensibilité des données financières.**

La création de **JayKonta** (service COG) avec **JayBudget** et **JayKonta** comme points d’entrée répond à la fois au besoin des services métier (JayFestival, JayRDV) et à l’offre marché pour les particuliers et les entreprises.

---

## 2. Positionnement stratégique

### 2.1 Raison d’être

**JayKonta** (service COG) a pour objectif de :

- **Centraliser le domaine « budget et comptabilité »** : mouvements, catégories, projets, devis, factures, rapports, avec des règles de gouvernance et de sécurité cohérentes.
- **Exposer des Opérateurs et Kits réutilisables** : les services métier (JayFestival, JayRDV, etc.) et les points d’entrée Purse / Account consomment le même socle.
- **Différencier par point d’entrée** : **JayBudget** pour le perso et les budgets occasionnels ; **JayKonta** pour l’entreprise (devis, facturation, comptabilité). Même COG, périmètres et Mandats différents.

### 2.2 Points d’entrée : JayBudget vs JayKonta

| Aspect | JayBudget | JayKonta |
|--------|----------------|------------------|
| **Public** | Particuliers, foyers. | Professionnels, associations, TPE/PME, organisateurs. |
| **Périmètre** | Budgets personnels, budgets occasionnels (vacances, Noël, projets). | Comptabilité d’entreprise, devis, facturation, rapports légaux. |
| **Données** | Données personnelles financières (niveau 2). | Données métier et légales (niveau 2–3). |
| **Résidence** | COG de référence selon politique (option perso : terminal ou COG). | Résidence centralisée recommandée ou obligatoire (niveau 2+). |
| **Intégration** | Peut rester autonome ou s’articuler avec d’autres services (ex. agenda pour rappels). | Consommée par JayFestival, JayRDV, etc. pour facturation et budget. |

**Règle** : le même **service COG** expose les capacités ; les **points d’entrée** (Purse, Account) déterminent le périmètre fonctionnel, les Mandats et le niveau de sécurité appliqué (WorrySentinel).

Détail : [JayKonta - Points Entree JayBudget et JayKonta](./reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md).

### 2.3 Positionnement dans la pyramide Miyukini

| Élément | Rôle |
|--------|------|
| **Kernel / KindMother** | Persistance, WriteIntent, cohérence des données. |
| **JayKonta (COG)** | **Service** de domaine « budget et comptabilité » : orchestration, mouvements, devis, factures, rapports, gouverné par Cores. |
| **JayBudget** | **Point d’entrée** perso/individuel du service COG (même Opérateurs, périmètre Purse). |
| **JayKonta (marque)** | **Point d’entrée** entreprise du service COG (devis, facturation, comptabilité). |
| **JayFestival / JayRDV** | **Services métier** qui s’appuient sur JayKonta pour budget, devis et facturation. |

### 2.4 Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance** | Le service fonctionne sous gouvernance COG : StrongFather (décisions), KindMother (données), Master Butler (permissions), WorrySentinel (niveaux de sécurité, états de confiance). |
| **Un COG, deux entrées** | Un seul service COG ; JayBudget et JayKonta sont des points d’entrée (marques, périmètres, Mandats), pas des services COG distincts. |
| **Sécurité par niveau** | Les données financières sont classées par niveau de sensibilité (WorrySentinel 0–4) ; les solutions de protection (résidence, chiffrement, audit) sont alignées sur ces niveaux. |
| **Réutilisabilité** | JayFestival, JayRDV et futurs services consomment les Opérateurs et Kits du service COG pour éviter la duplication (devis, factures, budget). |

---

## 3. Intégration avec les autres services

### 3.1 Services consommateurs identifiés

| Service | Usage de JayKonta | Données concernées |
|---------|---------------------------|---------------------|
| **Miyukini Festival Service** | Budget par édition, devis et factures exposants, ventilation revenus/dépenses. | Budget édition, facturation exposants |
| **JayRDV** | Facturation professionnels, abonnements, encaissements. | Factures, encaissements |
| **JayKoa** | Optionnel : rappels ou jalons liés à des échéances budget (échéances factures, objectifs). | Références temporelles, pas de données financières canoniques |
| **Futurs services** | Tout service nécessitant devis, facturation ou suivi de budget. | À définir par service |

### 3.2 Modèle d’intégration

- **JayKonta** (COG) expose des **Opérateurs** et **Kits d’outils** (ex. : `budget.movements.record`, `quote.create`, `invoice.emit`, `report.balance`, `report.export`).
- Chaque **service consommateur** (JayFestival, JayRDV) :
  - détient les **données métier** (qui est exposant, qui est professionnel, quel contrat) ;
  - **appelle** JayKonta pour enregistrer des mouvements, émettre des devis/factures, produire des rapports, selon Mandat et permissions ;
  - **reste responsable** du niveau de sécurité des données qu’il transmet (WorrySentinel).
- **KindMother** : la résidence des données sensibles (données financières personnelles ou d’entreprise) est définie par le [contrat du service](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) et le point d’entrée (Purse vs Account).

Détail : [JayKonta - Integration Services](./reference/JayKonta%20-%20Integration%20Services.md).

### 3.3 Bénéfice multi-services

- **Cohérence** : une seule logique devis/facturation/budget pour tout l’écosystème.
- **Réduction de la duplication** : JayFestival et JayRDV ne réimplémentent pas la facturation.
- **Expérience utilisateur** : un utilisateur peut avoir un budget perso (Purse) et, en tant que professionnel ou organisateur, utiliser les mêmes bases (Account) pour la facturation et la comptabilité.

---

## 4. Niveaux de sécurité et solutions de protection

### 4.1 Sensibilité des données financières

Les données traitées par JayKonta sont **hautement sensibles** (revenus, dépenses, factures, coordonnées bancaires ou moyens de paiement, identité des clients/fournisseurs). La classification et les mesures de protection sont **critiques**.

| Type de donnée | Exemple | Sensibilité | Niveau WorrySentinel typique |
|----------------|---------|-------------|------------------------------|
| **Agrégats anonymisés** | Totaux par catégorie sans lien identité | Faible | 0–1 |
| **Mouvements personnels** | Revenus, dépenses, catégories (Purse) | Sensible | 2 |
| **Budgets occasionnels** | Montants et libellés (vacances, Noël) | Sensible | 2 |
| **Devis et factures** | Montants, TVA, identité client/fournisseur | Sensible à critique | 2–3 |
| **Données de paiement** | Références de moyen de paiement, RIB, historique encaissements | Critique | 3 |
| **Comptabilité légale** | Pièces comptables, rapports soumis à contrôle | Critique | 3 |

### 4.2 Niveaux de sécurité (rappel Glossaire)

| Niveau | Nom | Description |
|--------|-----|-------------|
| **0** | Public | Données publiques, aucune contrainte stricte |
| **1** | Standard | Données standard, contraintes de base |
| **2** | Sensitive | Données sensibles, contraintes renforcées |
| **3** | Critical | Données critiques, contraintes strictes |
| **4** | Highest | Sécurité maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sécurité et les états de confiance ; Master Butler gère les permissions ; StrongFather émet les Mandats.

### 4.3 Solutions de protection par niveau

| Niveau | Mesures de protection |
|--------|------------------------|
| **0 – Public** | Aucune mesure spécifique ; pas de données financières personnelles ou métier. |
| **1 – Standard** | Contrôle d’accès (Mandat, Master Butler) ; traçabilité des accès ; agrégats sans identité. |
| **2 – Sensitive** | Résidence centralisée sur COG de référence (selon [Politique de résidence](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)) ; accès via Visite gouvernée ou session ; audit des lectures/écritures ; chiffrement en transit. |
| **3 – Critical** | Résidence centralisée obligatoire ; chiffrement au repos et en transit ; audit complet ; révocation immédiate possible (StrongFather, WorrySentinel). |
| **4 – Highest** | Contraintes maximales ; isolement renforcé ; procédures d’accès exceptionnel (TAMR, MiyukiniAdmin). |

Pour **JayBudget** (perso) : les données sont au moins niveau 2 ; la résidence peut être sur le COG de référence ou, selon contrat, sur l’environnement utilisateur avec synchronisation sécurisée. Pour **JayKonta** (entreprise) : les données facturation/comptabilité sont niveau 2–3 ; résidence centralisée recommandée ou obligatoire.

Référence détaillée : [JayKonta - Niveaux Securite et Protection Donnees](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

### 4.4 Règles de sécurité spécifiques au service

| Règle | Description |
|-------|-------------|
| **MAC-SEC-1** | Les données financières personnelles ou métier (mouvements, factures, devis, moyens de paiement) sont classées au minimum niveau 2 (Sensitive) ; les flux sont chiffrés et soumis à Mandat. |
| **MAC-SEC-2** | La résidence des données sensibles (niveau 2+) est définie par le contrat du service et le point d’entrée (Purse vs Account) ; COG de référence désigné pour Account et, selon politique, pour Purse. |
| **MAC-SEC-3** | Aucune donnée de paiement (RIB, cartes, tokens) n’est stockée en clair ; référencement par token ou identifiant opaque, conformité PCI-DSS / réglementation en vigueur. |
| **MAC-SEC-4** | Toute émission de devis ou facture par un service consommateur (JayFestival, JayRDV) transite par les Opérateurs JayKonta avec audit et niveau de sécurité déclaré. |
| **MAC-SEC-5** | En état de confiance dégradé (T2–T4), les capacités d’écriture ou d’export peuvent être restreintes (Caring Nanny, WorrySentinel). |

---

## 5. Prochaines étapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser (interne / partenaires).
2. **Spécifier** : Documenter les Opérateurs et Kits JayKonta (mouvements, devis, factures, rapports) et leurs Contrats d’équipe.
3. **Points d’entrée** : Finaliser les périmètres fonctionnels et UX JayBudget et JayKonta (voir [Points Entree Purse et Account](./reference/JayKonta%20-%20Points%20Entree%20Purse%20et%20Account.md)).
4. **Sécurité** : Finaliser le document [Niveaux Sécurité et Protection](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) et l’alignement avec la Politique de résidence.
5. **Intégration** : Formaliser les contrats d’intégration avec JayFestival et JayRDV (voir [Integration Services](./reference/JayKonta%20-%20Integration%20Services.md)).
6. **Implémentation** : Développer les Opérateurs et Kits en s’appuyant sur KindMother, WorrySentinel, Master Butler.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Opérateur, Mandat, COG, Niveaux de sécurité, WorrySentinel). |
| [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | Résidence centralisée, COG de référence, niveaux 2+. |
| [JayKonta - Niveaux Securite et Protection Donnees](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Détail des niveaux et mesures de protection pour le service. |
| [JayKonta - Points Entree JayBudget et JayKonta](./reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | Périmètres et différenciation JayBudget / JayKonta. |
| [JayKonta - Integration Services](./reference/JayKonta%20-%20Integration%20Services.md) | Schémas d’intégration JayFestival, JayRDV, futurs services. |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (budget édition, facturation exposants). |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle et contractuelle (cadre de travail, contraintes, décisions structurantes). |

---

**Document** : JayKonta — Document fondateur  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document fondateur — référence pour le service (besoins, positionnement, points d’entrée Purse/Account, intégration multi-services, sécurité). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
