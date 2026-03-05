# WorrySentinel - Security Levels Governance Contract

## 1. Contexte

Ce document dÃ©finit le **contrat de gouvernance des niveaux de sÃ©curitÃ©** dans WorrySentinel. Il formalise les rÃ¨gles d'attribution, de gestion, et d'application des niveaux de sÃ©curitÃ© (0-4) Ã  travers l'Ã©cosystÃ¨me Miyukini, ainsi que les contraintes que ces niveaux imposent aux cores fonctionnels et aux produits.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice (Section 6 - Gouvernance des niveaux de sÃ©curitÃ©) et respecte les invariants INV-GOV-1 et INV-GOV-6.

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Tout produit, composant, Tool, Toolkit, ou service de l'Ã©cosystÃ¨me Miyukini
- **Responsable :** WorrySentinel (autoritÃ© exclusive de gouvernance des niveaux de sÃ©curitÃ©)
- **Consommateurs :** Tous les cores fonctionnels (StrongFather, KindMother, MasterButler, CaringNanny, EverBuddy, BorderGuard, TAMR, LogisticsSteward), tous les adaptateurs produits, tous les produits
- **Ne couvre pas :** Les Ã©tats de confiance T0-T4 (voir [Trust States Governance Contract](./WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)), les dÃ©tails d'implÃ©mentation des contrÃ´les de sÃ©curitÃ©

---

## 3. DÃ©finition des niveaux de sÃ©curitÃ©

### 3.1 Principe fondamental

**"La sÃ©curitÃ© est un paramÃ¨tre de gouvernance, pas un choix applicatif."**

Les niveaux de sÃ©curitÃ© caractÃ©risent le **profil de risque** d'un produit ou composant. Ils dÃ©terminent :
- Les contraintes de sÃ©curitÃ© applicables
- Le comportement des cores fonctionnels
- Les restrictions d'accÃ¨s inter-composants
- Les exigences de traÃ§abilitÃ©

Un produit **dÃ©clare** son profil de risque, mais **ne choisit pas** ses contraintes de sÃ©curitÃ©. WorrySentinel gouverne, les cores appliquent.

### 3.2 Les cinq niveaux de sÃ©curitÃ©

#### Niveau 0 â€” Public / Display

**Profil de risque :** Minimal

| Aspect | SpÃ©cification |
|--------|---------------|
| **DÃ©signation** | Public |
| **DonnÃ©es** | Publiques, aucune sensibilitÃ© |
| **Contraintes** | Aucune contrainte de sÃ©curitÃ© stricte |
| **Fonctionnement** | Normal sans restrictions |
| **TraÃ§abilitÃ©** | Minimale |
| **Impact performance** | ðŸŸ¢ Quasi nul |

**Cas d'usage typiques :**
- Site vitrine
- Affichage de donnÃ©es publiques
- Dashboards en lecture seule
- WebApp sans Ã©tat critique

**Principe directeur :** *"Si Ã§a casse, ce n'est pas grave."*

#### Niveau 1 â€” Standard / CMS

**Profil de risque :** Faible

| Aspect | SpÃ©cification |
|--------|---------------|
| **DÃ©signation** | Standard |
| **DonnÃ©es** | Standard, sensibilitÃ© faible |
| **Contraintes** | Contraintes de sÃ©curitÃ© de base |
| **Fonctionnement** | Normal avec vÃ©rifications de base |
| **TraÃ§abilitÃ©** | Normale |
| **Impact performance** | ðŸŸ¢ Faible |

**Cas d'usage typiques :**
- CMS
- Backoffice simple
- Contenu Ã©ditorial
- OpÃ©rateurs B2C classiques

**Principe directeur :** *"On protÃ¨ge l'accÃ¨s, pas le systÃ¨me."*

#### Niveau 2 â€” Sensitive Data

**Profil de risque :** ModÃ©rÃ©

| Aspect | SpÃ©cification |
|--------|---------------|
| **DÃ©signation** | Sensitive Data |
| **DonnÃ©es** | Sensibles, protection requise |
| **Contraintes** | Contraintes de sÃ©curitÃ© renforcÃ©es |
| **Fonctionnement** | Avec restrictions modÃ©rÃ©es |
| **TraÃ§abilitÃ©** | ComplÃ¨te |
| **Impact performance** | ðŸŸ¡ ModÃ©rÃ© mais contrÃ´lÃ© |

**Cas d'usage typiques :**
- DonnÃ©es personnelles
- Comptes utilisateurs
- Profils et prÃ©fÃ©rences
- Historique

**Principe directeur :** *"On protÃ¨ge les donnÃ©es."*

#### Niveau 3 â€” Critical System

**Profil de risque :** Ã‰levÃ©

| Aspect | SpÃ©cification |
|--------|---------------|
| **DÃ©signation** | Critical System |
| **DonnÃ©es** | Critiques, protection maximale |
| **Contraintes** | Contraintes de sÃ©curitÃ© strictes |
| **Fonctionnement** | Avec restrictions importantes |
| **TraÃ§abilitÃ©** | Absolue avec signatures |
| **Impact performance** | ðŸŸ  AcceptÃ© mais maÃ®trisÃ© |

**Cas d'usage typiques :**
- Authentification
- Paiement
- Autorisations
- DÃ©cisions structurantes
- Cores internes

**Principe directeur :** *"On protÃ¨ge le systÃ¨me avant l'UX."*

#### Niveau 4 â€” Hardened / Isolated

**Profil de risque :** Maximal

| Aspect | SpÃ©cification |
|--------|---------------|
| **DÃ©signation** | Hardened / Isolated |
| **DonnÃ©es** | SÃ©curitÃ© maximale, protection absolue |
| **Contraintes** | Contraintes de sÃ©curitÃ© maximales |
| **Fonctionnement** | Avec restrictions maximales |
| **TraÃ§abilitÃ©** | Absolue avec signatures cryptographiques |
| **Impact performance** | ðŸ”´ Secondaire |

**Cas d'usage typiques :**
- Environnement isolÃ©
- Hardware non fiable
- Contexte hostile
- Infrastructure critique
- Mode survie

**Principe directeur :** *"On protÃ¨ge l'intÃ©gritÃ© coÃ»te que coÃ»te."*

### 3.3 Matrice synthÃ©tique des niveaux

| Niveau | DÃ©signation | DonnÃ©es | Contraintes | TraÃ§abilitÃ© | Impact perf |
|--------|-------------|---------|-------------|-------------|-------------|
| **0** | Public | Publiques | Aucune | Minimale | ðŸŸ¢ Quasi nul |
| **1** | Standard | Standard | De base | Normale | ðŸŸ¢ Faible |
| **2** | Sensitive | Sensibles | RenforcÃ©es | ComplÃ¨te | ðŸŸ¡ ModÃ©rÃ© |
| **3** | Critical | Critiques | Strictes | Absolue + signatures | ðŸŸ  AcceptÃ© |
| **4** | Hardened | Max sÃ©curitÃ© | Maximales | Absolue + crypto | ðŸ”´ Secondaire |

---

## 4. RÃ¨gles de gouvernance des niveaux

### 4.1 RÃˆGLE-SEC-1 : Attribution de niveau

**Ã‰noncÃ© :**

> WorrySentinel gouverne l'attribution des niveaux de sÃ©curitÃ© aux produits et composants. Cette attribution est **explicite**, **immuable pendant l'exÃ©cution**, et **traÃ§able**.

| Aspect | SpÃ©cification |
|--------|---------------|
| **ApplicabilitÃ©** | Tout produit, composant, Tool, ou Toolkit |
| **PortÃ©e** | Absolue |
| **VÃ©rification** | Chaque entitÃ© possÃ¨de un niveau de sÃ©curitÃ© dÃ©fini |
| **Invariant associÃ©** | INV-GOV-1 (Niveaux de sÃ©curitÃ© explicites) |

**PropriÃ©tÃ©s de l'attribution :**

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **Explicite** | Chaque produit et composant possÃ¨de un niveau de sÃ©curitÃ© dÃ©fini |
| **Immuable pendant l'exÃ©cution** | Le niveau de sÃ©curitÃ© ne change pas pendant l'exÃ©cution d'une opÃ©ration |
| **TraÃ§able** | Toute attribution de niveau est tracÃ©e avec justification |
| **DÃ©clarative** | L'attribution est dÃ©clarÃ©e dans le profil du produit |
| **ValidÃ©e** | L'attribution est validÃ©e par les cores (BorderGuard, StrongFather) |

**Format d'attribution :**

```
Attribution Niveau de SÃ©curitÃ©:
  entitÃ©: "MonProduit"
  niveau: 2
  justification: "Gestion de donnÃ©es personnelles utilisateurs"
  validÃ©_par: "BorderGuard"
  date_attribution: "2026-01-28"
  immuable_jusqu'Ã : "fin_opÃ©ration"
```

### 4.2 RÃˆGLE-SEC-2 : Adaptation comportementale

**Ã‰noncÃ© :**

> WorrySentinel gouverne les rÃ¨gles selon lesquelles les composants doivent adapter leur comportement selon le niveau de sÃ©curitÃ©.

| Aspect | SpÃ©cification |
|--------|---------------|
| **ApplicabilitÃ©** | Tous les cores fonctionnels |
| **PortÃ©e** | Absolue |
| **VÃ©rification** | Chaque core adapte son comportement selon le niveau |
| **Invariant associÃ©** | INV-WS-7 (Gouvernance explicite) |

**Adaptation par tranche de niveau :**

| Tranche | Comportement attendu |
|---------|---------------------|
| **Niveau 0-1** | Comportement normal, restrictions minimales |
| **Niveau 2** | Restrictions modÃ©rÃ©es, vÃ©rifications renforcÃ©es |
| **Niveau 3-4** | Restrictions importantes, vÃ©rifications maximales |

### 4.3 RÃˆGLE-SEC-3 : CohÃ©rence inter-composants

**Ã‰noncÃ© :**

> WorrySentinel garantit la cohÃ©rence des niveaux de sÃ©curitÃ© entre composants qui interagissent. Un composant de niveau N ne peut pas accÃ©der directement Ã  un composant de niveau > N sans mÃ©diation.

| Aspect | SpÃ©cification |
|--------|---------------|
| **ApplicabilitÃ©** | Toute interaction entre composants |
| **PortÃ©e** | Absolue |
| **VÃ©rification** | Matrice d'accÃ¨s respectÃ©e |
| **Invariant associÃ©** | INV-GOV-6 (CohÃ©rence inter-composants) |

**Matrice d'accÃ¨s inter-niveaux :**

| Source \ Cible | N0 | N1 | N2 | N3 | N4 |
|----------------|----|----|----|----|----| 
| **N0** | âœ… | âŒ | âŒ | âŒ | âŒ |
| **N1** | âœ… | âœ… | âŒ | âŒ | âŒ |
| **N2** | âœ… | âœ… | âœ… | âŒ | âŒ |
| **N3** | âœ… | âœ… | âœ… | âœ… | âŒ |
| **N4** | âœ… | âœ… | âœ… | âœ… | âœ… |

**RÃ¨gle de mÃ©diation :** Les accÃ¨s aux niveaux supÃ©rieurs nÃ©cessitent une mÃ©diation explicite gouvernÃ©e par WorrySentinel et validÃ©e par StrongFather.

### 4.4 RÃˆGLE-SEC-4 : ImmuabilitÃ© opÃ©rationnelle

**Ã‰noncÃ© :**

> Le niveau de sÃ©curitÃ© d'un composant est immuable pendant toute la durÃ©e d'une opÃ©ration. Aucune modification de niveau n'est autorisÃ©e pendant l'exÃ©cution.

| Aspect | SpÃ©cification |
|--------|---------------|
| **ApplicabilitÃ©** | Toute opÃ©ration en cours |
| **PortÃ©e** | Absolue |
| **VÃ©rification** | Niveau constant du dÃ©but Ã  la fin de l'opÃ©ration |
| **ConsÃ©quence de violation** | IncohÃ©rence de sÃ©curitÃ©, comportement imprÃ©visible |

**Ce que cela signifie concrÃ¨tement :**

| AutorisÃ© | Interdit |
|----------|----------|
| âœ… Modifier le niveau entre deux opÃ©rations | âŒ Modifier le niveau pendant une opÃ©ration |
| âœ… Planifier un changement de niveau | âŒ Changer de niveau pour contourner une restriction |
| âœ… Attribuer un niveau au dÃ©marrage | âŒ RÃ©trograder le niveau pour performance |

### 4.5 RÃˆGLE-SEC-5 : Principe de non-contournement

**Ã‰noncÃ© :**

> Aucun composant ne peut contourner les contraintes de son niveau de sÃ©curitÃ©. Les contraintes sont imposÃ©es par WorrySentinel et appliquÃ©es par les cores.

| Aspect | SpÃ©cification |
|--------|---------------|
| **ApplicabilitÃ©** | Tout composant, quel que soit son niveau |
| **PortÃ©e** | Absolue |
| **VÃ©rification** | Aucune violation des contraintes de niveau |
| **ConsÃ©quence de violation** | Faute de sÃ©curitÃ©, violation de INV-GOV-6 |

**Interdictions explicites :**

| Code | Interdiction |
|------|--------------|
| **INTERD-SEC-1** | Un composant ne peut pas dÃ©clarer un niveau infÃ©rieur Ã  son profil de risque rÃ©el |
| **INTERD-SEC-2** | Un composant ne peut pas accÃ©der Ã  un niveau supÃ©rieur sans mÃ©diation |
| **INTERD-SEC-3** | Un composant ne peut pas dÃ©sactiver les contrÃ´les de son niveau |
| **INTERD-SEC-4** | Un composant ne peut pas ignorer les adaptations comportementales requises |

---

## 5. Adaptation des cores par niveau

### 5.1 StrongFather

**ResponsabilitÃ© :** Adapter la sÃ©vÃ©ritÃ© des dÃ©cisions selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation StrongFather |
|--------|------------------------|
| **0** | DÃ©cisions simplifiÃ©es, pas de vÃ©rification stricte |
| **1** | DÃ©cisions standard, validation normale |
| **2** | DÃ©cisions renforcÃ©es, validation stricte |
| **3** | DÃ©cisions strictes, vÃ©rifications croisÃ©es |
| **4** | DÃ©cisions ultra-strictes, aucune tolÃ©rance |

**RÃ¨gle :** StrongFather adapte sa sÃ©vÃ©ritÃ© selon le niveau gouvernÃ© par WorrySentinel, sans jamais contourner les contraintes.

### 5.2 MasterButler

**ResponsabilitÃ© :** Adapter les permissions selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation MasterButler |
|--------|------------------------|
| **0** | Permissions publiques uniquement |
| **1** | Permissions basiques |
| **2** | Permissions dÃ©taillÃ©es |
| **3** | Permissions critiques, vÃ©rification systÃ©matique |
| **4** | Permissions minimales, vÃ©rification constante |

**RÃ¨gle :** MasterButler accorde les permissions selon le niveau gouvernÃ© par WorrySentinel, jamais au-delÃ .

### 5.3 BorderGuard

**ResponsabilitÃ© :** Adapter les frontiÃ¨res I/O selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation BorderGuard |
|--------|------------------------|
| **0** | FrontiÃ¨res assouplies |
| **1** | FrontiÃ¨res standard |
| **2** | FrontiÃ¨res renforcÃ©es |
| **3** | FrontiÃ¨res strictes, classification renforcÃ©e |
| **4** | FrontiÃ¨res maximales, isolement strict |

**RÃ¨gle :** BorderGuard durcit les frontiÃ¨res selon le niveau gouvernÃ© par WorrySentinel.

### 5.4 CaringNanny

**ResponsabilitÃ© :** Adapter le monitoring selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation CaringNanny |
|--------|------------------------|
| **0** | Monitoring minimal |
| **1** | Monitoring normal |
| **2** | Monitoring actif, dÃ©tection anomalies |
| **3** | Monitoring intensif, sondes actives |
| **4** | Monitoring continu, sondes trÃ¨s frÃ©quentes |

**RÃ¨gle :** CaringNanny intensifie la surveillance selon le niveau gouvernÃ© par WorrySentinel.

### 5.5 TAMR

**ResponsabilitÃ© :** Adapter les interventions humaines selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation TAMR |
|--------|----------------|
| **0** | Pas d'intervention humaine requise |
| **1** | Intervention humaine optionnelle |
| **2** | Intervention humaine possible |
| **3** | Intervention humaine requise en cas de doute |
| **4** | Intervention humaine systÃ©matique |

**RÃ¨gle :** TAMR exige des interventions humaines selon le niveau gouvernÃ© par WorrySentinel.

### 5.6 BondingBrother

**ResponsabilitÃ© :** Adapter la traÃ§abilitÃ© selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation BondingBrother |
|--------|------------------------|
| **0-1** | TraÃ§abilitÃ© normale |
| **2** | TraÃ§abilitÃ© complÃ¨te |
| **3** | TraÃ§abilitÃ© absolue, signatures obligatoires |
| **4** | TraÃ§abilitÃ© absolue, signatures cryptographiques |

**RÃ¨gle :** BondingBrother renforce la traÃ§abilitÃ© selon le niveau gouvernÃ© par WorrySentinel.

### 5.7 LogisticsSteward

**ResponsabilitÃ© :** Adapter l'arbitrage des ressources selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation LogisticsSteward |
|--------|---------------------------|
| **0-1** | Quotas et prioritÃ©s standards |
| **2** | Quotas ajustÃ©s, prioritÃ©s normales |
| **3** | Quotas stricts, prioritÃ© haute aux contrÃ´les de sÃ©curitÃ© |
| **4** | Quotas minimaux, prioritÃ© maximale Ã  la sÃ©curitÃ© |

**RÃ¨gle :** LogisticsSteward priorise les ressources de sÃ©curitÃ© selon le niveau gouvernÃ© par WorrySentinel.

### 5.8 Kernel

**ResponsabilitÃ© :** Adapter la frÃ©quence des sondes selon le niveau de sÃ©curitÃ©.

| Niveau | Adaptation Kernel |
|--------|------------------|
| **0-1** | Sondes normales |
| **2** | Sondes rÃ©guliÃ¨res |
| **3** | Sondes frÃ©quentes |
| **4** | Sondes trÃ¨s frÃ©quentes, attestations rÃ©guliÃ¨res |

**RÃ¨gle :** Le Kernel intensifie les sondes selon le niveau gouvernÃ© par WorrySentinel.

### 5.9 Matrice synthÃ©tique d'adaptation des cores

| Core | N0 | N1 | N2 | N3 | N4 |
|------|----|----|----|----|----| 
| **StrongFather** | SimplifiÃ© | Standard | RenforcÃ© | Strict | Ultra-strict |
| **MasterButler** | Public | Basique | DÃ©taillÃ© | Critique | Minimal |
| **BorderGuard** | Assoupli | Standard | RenforcÃ© | Strict | Isolement |
| **CaringNanny** | Minimal | Normal | Actif | Intensif | Continu |
| **TAMR** | Aucune | Optionnel | Possible | Requis | SystÃ©matique |
| **BondingBrother** | Normal | Normal | Complet | Signatures | Crypto |
| **LogisticsSteward** | Standard | Standard | AjustÃ© | Strict | PrioritÃ© max |
| **Kernel** | Normal | Normal | RÃ©gulier | FrÃ©quent | TrÃ¨s frÃ©quent |

---

## 6. Gouvernance de sÃ©curitÃ© des Tools et Toolkits

### 6.1 Principe

WorrySentinel gouverne la sÃ©curitÃ© des Tools et Toolkits en dÃ©finissant le niveau de sÃ©curitÃ© requis pour leur utilisation.

**Question fondamentale :**

> *"Le niveau de sÃ©curitÃ© actuel permet-il cet appel de Tool ?"*

### 6.2 RÃ¨gles de gouvernance des Tools

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-TOOL-SEC-1** | Chaque Tool a un niveau de sÃ©curitÃ© dÃ©fini |
| **RÃˆGLE-TOOL-SEC-2** | Un Tool de niveau N ne peut Ãªtre appelÃ© que si le niveau de sÃ©curitÃ© le permet |
| **RÃˆGLE-TOOL-SEC-3** | En Ã©tat de confiance T2+, certains Tools peuvent Ãªtre bloquÃ©s |
| **RÃˆGLE-TOOL-SEC-4** | Tout appel de Tool est auditable |

### 6.3 Attribution de niveau aux Tools

| CatÃ©gorie Tool | Niveau typique | Justification |
|----------------|----------------|---------------|
| **UI Tools** | 0-1 | Affichage, pas de donnÃ©es sensibles |
| **Data Tools** | 2 | Manipulation de donnÃ©es utilisateur |
| **Auth Tools** | 3 | Authentification, autorisations |
| **Admin Tools** | 3-4 | Administration systÃ¨me |
| **Security Tools** | 4 | ContrÃ´les de sÃ©curitÃ© critiques |

### 6.4 Blocage de Tools en Ã©tat dÃ©gradÃ©

| Ã‰tat confiance | Tools bloquÃ©s |
|----------------|---------------|
| **T0 â€” Normal** | Aucun blocage |
| **T1 â€” Instable** | Aucun blocage, traÃ§abilitÃ© renforcÃ©e |
| **T2 â€” DÃ©gradÃ©** | Tools de niveau 0 potentiellement bloquÃ©s |
| **T3 â€” Restreint** | Tools non essentiels bloquÃ©s |
| **T4 â€” BloquÃ©** | Tous les Tools bloquÃ©s sauf diagnostics |

**Exemple de blocage :**

```
UI Toolkit indisponible car environnement en Ã©tat SECURITY_LOCKDOWN (T3)
```

---

## 7. Interaction avec les Ã©tats de confiance

### 7.1 IndÃ©pendance des deux dimensions

Les niveaux de sÃ©curitÃ© (0-4) et les Ã©tats de confiance (T0-T4) sont **indÃ©pendants** mais **interagissent** :

- **Niveaux de sÃ©curitÃ© (0-4)** : Profil de risque du produit (statique pendant l'opÃ©ration)
- **Ã‰tats de confiance (T0-T4)** : Ã‰tat d'intÃ©gritÃ© du systÃ¨me (dynamique)

### 7.2 Matrice d'interaction Niveau Ã— Ã‰tat

| Niveau \ Ã‰tat | T0 | T1 | T2 | T3 | T4 |
|---------------|----|----|----|----|----| 
| **N0** | Normal | + traces | Fonctions sensibles bridÃ©es | Lecture seule | BloquÃ© |
| **N1** | Normal | + traces | Fonctions sensibles bridÃ©es | Lecture seule | BloquÃ© |
| **N2** | Normal | + vÃ©rifications | Restrictions modÃ©rÃ©es | Restrictions importantes | BloquÃ© |
| **N3** | Normal | + vÃ©rifications renforcÃ©es | Restrictions importantes | Gel partiel | BloquÃ© |
| **N4** | Normal | + vÃ©rifications maximales | Restrictions maximales | Gel quasi-total | Diagnostics uniquement |

### 7.3 RÃ¨gle de cumul

**RÃˆGLE-CUMUL :** Les restrictions sont cumulatives.

> Niveau de sÃ©curitÃ© Ã©levÃ© + Ã‰tat de confiance dÃ©gradÃ© = Restrictions maximales

**Exemple :**
- Produit Niveau 2 (Sensitive Data) en T0 (Normal) â†’ Fonctionnement normal
- Produit Niveau 2 (Sensitive Data) en T2 (DÃ©gradÃ©) â†’ Restrictions modÃ©rÃ©es selon niveau + restrictions selon Ã©tat = Restrictions renforcÃ©es

---

## 8. DÃ©claration et validation des niveaux

### 8.1 Processus de dÃ©claration

**Ã‰tape 1 : DÃ©claration par le produit**

Le produit dÃ©clare son profil de sÃ©curitÃ© requis :

```
Product Security Profile:
  product_id: "mon-produit-v1"
  required_level: 2
  justification: "Gestion de donnÃ©es personnelles utilisateurs"
  offline_allowed: true
  degradation_allowed: true
```

**Ã‰tape 2 : Validation par BorderGuard**

BorderGuard valide que le niveau dÃ©clarÃ© est cohÃ©rent avec les capacitÃ©s du produit.

**Ã‰tape 3 : Enregistrement par WorrySentinel**

WorrySentinel enregistre l'attribution du niveau de sÃ©curitÃ© avec traÃ§abilitÃ© complÃ¨te.

**Ã‰tape 4 : Application par les cores**

Les cores adaptent leur comportement selon le niveau attribuÃ©.

### 8.2 Validation du niveau dÃ©clarÃ©

| VÃ©rification | Responsable | CritÃ¨re |
|--------------|-------------|---------|
| **CohÃ©rence profil** | BorderGuard | Le niveau correspond au type de donnÃ©es gÃ©rÃ©es |
| **CapacitÃ©s techniques** | BorderGuard | Le produit peut supporter les contraintes du niveau |
| **IntÃ©gration Ã©cosystÃ¨me** | WorrySentinel | Le niveau est cohÃ©rent avec les composants appelÃ©s |
| **Politique produit** | StrongFather | Le niveau respecte les politiques applicables |

### 8.3 Modification de niveau

**Conditions de modification :**

| Condition | Obligatoire |
|-----------|-------------|
| Aucune opÃ©ration en cours | âœ… Oui |
| Justification explicite | âœ… Oui |
| Validation par BorderGuard | âœ… Oui |
| Approbation StrongFather | âœ… Oui |
| TraÃ§abilitÃ© complÃ¨te | âœ… Oui |

**Interdiction absolue :** Modifier le niveau pendant une opÃ©ration en cours.

---

## 9. Invariants applicables

### 9.1 INV-GOV-1 : Niveaux de sÃ©curitÃ© explicites

**Application dans ce contrat :**

> Tous les produits et composants possÃ¨dent un niveau de sÃ©curitÃ© **explicite** dÃ©fini par WorrySentinel. Aucun produit ou composant ne peut fonctionner sans niveau de sÃ©curitÃ© dÃ©fini.

**VÃ©rification :**
- Chaque produit/composant possÃ¨de un niveau de sÃ©curitÃ© (0-4)
- Le niveau est dÃ©clarÃ© dans le profil du produit
- Le niveau est validÃ© par BorderGuard
- Le niveau est enregistrÃ© par WorrySentinel

### 9.2 INV-GOV-6 : CohÃ©rence inter-composants

**Application dans ce contrat :**

> Les niveaux de sÃ©curitÃ© sont **cohÃ©rents** entre composants qui interagissent. Un composant de niveau N ne peut pas accÃ©der directement Ã  un composant de niveau > N sans mÃ©diation.

**VÃ©rification :**
- Matrice d'accÃ¨s inter-niveaux respectÃ©e
- MÃ©diation explicite pour accÃ¨s Ã  niveaux supÃ©rieurs
- Aucun contournement de la classification

### 9.3 INV-WS-7 : Gouvernance explicite

**Application dans ce contrat :**

> Toutes les rÃ¨gles de gouvernance appliquÃ©es par WorrySentinel sont **explicites** et **dÃ©claratives**. Aucune rÃ¨gle implicite n'est autorisÃ©e.

**VÃ©rification :**
- RÃ¨gles RÃˆGLE-SEC-1 Ã  RÃˆGLE-SEC-5 explicitement dÃ©finies
- RÃ¨gles RÃˆGLE-TOOL-SEC-1 Ã  RÃˆGLE-TOOL-SEC-4 explicitement dÃ©finies
- Adaptations des cores explicitement documentÃ©es

### 9.4 INV-WS-8 : TraÃ§abilitÃ© complÃ¨te

**Application dans ce contrat :**

> Toute dÃ©cision de gouvernance produite par WorrySentinel est **traÃ§able** avec son contexte, ses rÃ¨gles appliquÃ©es, et sa justification.

**VÃ©rification :**
- Attribution de niveau tracÃ©e
- Modification de niveau tracÃ©e
- Blocage de Tools tracÃ©
- Adaptation des cores tracÃ©e

---

## 10. Violations et comportements interdits

### 10.1 Violations de gouvernance de niveau

| Code | Violation | Invariant violÃ© |
|------|-----------|-----------------|
| **VIOL-SEC-1** | Composant sans niveau de sÃ©curitÃ© dÃ©fini | INV-GOV-1 |
| **VIOL-SEC-2** | AccÃ¨s direct Ã  un niveau supÃ©rieur sans mÃ©diation | INV-GOV-6 |
| **VIOL-SEC-3** | Modification de niveau pendant une opÃ©ration | RÃˆGLE-SEC-4 |
| **VIOL-SEC-4** | Contournement des contraintes de niveau | RÃˆGLE-SEC-5 |
| **VIOL-SEC-5** | Attribution de niveau sans justification | INV-WS-8 |
| **VIOL-SEC-6** | Niveau dÃ©clarÃ© infÃ©rieur au profil de risque rÃ©el | INTERD-SEC-1 |
| **VIOL-SEC-7** | Core n'adaptant pas son comportement selon le niveau | RÃˆGLE-SEC-2 |

### 10.2 Anti-patterns

| Anti-pattern | Description | ConsÃ©quence |
|--------------|-------------|-------------|
| **"Security by obscurity"** | Cacher le niveau de sÃ©curitÃ© | Violation INV-GOV-1 |
| **"Level hopping"** | Changer de niveau pour contourner une restriction | Violation RÃˆGLE-SEC-4 |
| **"Downgrade attack"** | DÃ©clarer un niveau infÃ©rieur au profil rÃ©el | Violation INTERD-SEC-1 |
| **"Bypass by default"** | Ignorer les adaptations comportementales | Violation RÃˆGLE-SEC-2 |
| **"Silent access"** | AccÃ©der Ã  un niveau supÃ©rieur sans mÃ©diation | Violation INV-GOV-6 |

---

## 11. RÃ©fÃ©rences croisÃ©es

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | Document source (Section 6) |
| [WorrySentinel - Invariants & Guarantees](../governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Invariants INV-GOV-1, INV-GOV-6, INV-WS-7, INV-WS-8 |
| [WorrySentinel - Trust States Governance Contract](./WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md) | Ã‰tats de confiance T0-T4 |
| [WorrySentinel - Progressive Degradation Contract](../degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md) | DÃ©gradation progressive |
| [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) | RÃ©fÃ©rence complÃ¨te niveaux 0-4 |

### RÃ©fÃ©rences glossaire

| Terme | DÃ©finition |
|-------|------------|
| **Niveau de sÃ©curitÃ©** | Profil de risque d'un produit ou composant (0-4) |
| **Attribution** | Assignation d'un niveau de sÃ©curitÃ© Ã  un composant |
| **MÃ©diation** | Processus de validation pour accÃ¨s inter-niveaux |
| **Adaptation comportementale** | Modification du comportement des cores selon le niveau |
| **Matrice d'accÃ¨s** | RÃ¨gles dÃ©finissant les accÃ¨s autorisÃ©s entre niveaux |
| **ImmuabilitÃ© opÃ©rationnelle** | StabilitÃ© du niveau pendant l'exÃ©cution d'une opÃ©ration |

---

## 12. SynthÃ¨se contractuelle

### Engagements de ce contrat

Ce contrat Ã©tablit que :

1. **Les niveaux de sÃ©curitÃ© sont explicites** â€” 5 niveaux (0-4) caractÃ©risant le profil de risque
2. **Les rÃ¨gles sont non nÃ©gociables** â€” 5 rÃ¨gles de gouvernance (RÃˆGLE-SEC-1 Ã  5)
3. **Les adaptations sont obligatoires** â€” Tous les cores adaptent leur comportement
4. **Les accÃ¨s sont contrÃ´lÃ©s** â€” Matrice d'accÃ¨s inter-niveaux stricte
5. **Les Tools sont gouvernÃ©s** â€” 4 rÃ¨gles de gouvernance Tools (RÃˆGLE-TOOL-SEC-1 Ã  4)
6. **Les violations sont identifiÃ©es** â€” 7 violations cataloguÃ©es

### Phrase de synthÃ¨se

> **WorrySentinel gouverne les 5 niveaux de sÃ©curitÃ© (0-4) caractÃ©risant le profil de risque des produits et composants, selon 5 rÃ¨gles de gouvernance non nÃ©gociables (attribution explicite, adaptation comportementale, cohÃ©rence inter-composants, immuabilitÃ© opÃ©rationnelle, non-contournement), imposant aux 8 cores fonctionnels une adaptation stricte de leur comportement selon le niveau gouvernÃ©.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** WorrySentinel v1.2, Documentation Fondatrice Section 6  
**Type :** Contrat de gouvernance â€” Niveaux de sÃ©curitÃ©

