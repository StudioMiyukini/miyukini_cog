# Ever Buddy â€” Debt Tracking Contract

## 1. Contexte

Ce document dÃ©finit le **contrat de surveillance de la dette structurelle** gouvernÃ© par Ever Buddy. La dette structurelle est une rÃ©alitÃ© inÃ©vitable de tout systÃ¨me qui Ã©volue : elle reprÃ©sente l'ensemble des Ã©lÃ©ments obsolÃ¨tes ou en fin de vie qui persistent pour assurer la continuitÃ©.

Ever Buddy est **exclusivement responsable** de la surveillance de cette dette. Il ne l'Ã©limine pas, il ne la corrige pas â€” il l'observe, la mesure, et alerte quand elle devient excessive.

**Document de rÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

### Ce que ce contrat couvre

- DÃ©finition de la dette structurelle
- MÃ©triques de surveillance de la dette
- Seuils d'alerte et niveaux de gravitÃ©
- Processus de dÃ©tection et d'alerte
- Actions de nettoyage recommandÃ©es
- ConformitÃ© aux invariants d'Ever Buddy

### Ce que ce contrat ne couvre PAS

- L'exÃ©cution des nettoyages (responsabilitÃ© des produits et de KindMother)
- Les stratÃ©gies de migration technique (hors pÃ©rimÃ¨tre Ever Buddy)
- Les dÃ©cisions d'archivage forcÃ© (Ever Buddy recommande, il ne force pas)
- Les mÃ©triques de performance technique (domaine de Caring Nanny)

---

## 3. DÃ©finition de la dette structurelle

### 3.1 Qu'est-ce que la dette structurelle ?

La **dette structurelle** est l'ensemble des Ã©lÃ©ments en Ã©tat **DEPRECATED** ou **RETIRED** qui persistent dans le systÃ¨me. Cette dette n'est pas nÃ©cessairement nÃ©gative â€” elle est le **prix de la continuitÃ©**.

```mermaid
graph LR
    subgraph DettesStructurelle[Dette Structurelle]
        DEP[DEPRECATED<br/>Ã‰lÃ©ments obsolÃ¨tes<br/>mais fonctionnels]
        RET[RETIRED<br/>Ã‰lÃ©ments retirÃ©s<br/>mais conservÃ©s]
    end
    
    subgraph Actifs[Base Active]
        ACT[ACTIVE<br/>Ã‰lÃ©ments en usage<br/>normal]
    end
    
    DEP -->|"Transition<br/>planifiÃ©e"| RET
    RET -->|"Archivage<br/>futur"| ARC[ARCHIVED]
    
    classDef debt fill:#ffecb3
    classDef active fill:#c8e6c9
    classDef archived fill:#e0e0e0
    
    class DEP,RET debt
    class ACT active
    class ARC archived
```

### 3.2 Pourquoi la dette existe

La dette structurelle existe parce que :

| Raison | Explication |
|--------|-------------|
| **ContinuitÃ©** | Les consommateurs existants ont besoin de temps pour migrer |
| **CompatibilitÃ©** | Les versions antÃ©rieures doivent rester accessibles pendant la transition |
| **TraÃ§abilitÃ©** | L'historique des Ã©volutions doit Ãªtre conservÃ© |
| **Prudence** | Les transitions brutales crÃ©ent des ruptures inacceptables |

### 3.3 Quand la dette devient problÃ©matique

La dette devient problÃ©matique quand elle :

| SymptÃ´me | Description |
|----------|-------------|
| **Accumulation** | Le ratio dette/actif dÃ©passe les seuils acceptables |
| **Stagnation** | Les Ã©lÃ©ments DEPRECATED ne transitionnent pas vers RETIRED |
| **Blocage** | Les consommateurs ne migrent pas malgrÃ© les alertes |
| **IncomprÃ©hension** | La multiplication des versions crÃ©e de la confusion |
| **CoÃ»t** | La maintenance des Ã©lÃ©ments obsolÃ¨tes consomme des ressources excessives |

---

## 4. MÃ©triques de surveillance

### 4.1 Debt Ratio â€” MÃ©trique principale

Le **debt ratio** est la mÃ©trique centrale de surveillance de la dette structurelle.

```
Debt Ratio = (Nombre d'Ã©lÃ©ments DEPRECATED + Nombre d'Ã©lÃ©ments RETIRED) / Nombre d'Ã©lÃ©ments ACTIVE
```

**InterprÃ©tation :**

| Debt Ratio | InterprÃ©tation | Action |
|------------|----------------|--------|
| **0.00 - 0.10** | Sain | Aucune action requise |
| **0.10 - 0.25** | Normal | Surveillance standard |
| **0.25 - 0.40** | Ã‰levÃ© | Alerte prÃ©ventive, plan de nettoyage recommandÃ© |
| **0.40 - 0.60** | Critique | Alerte urgente, nettoyage prioritaire |
| **> 0.60** | Excessif | Alerte bloquante, gel des nouvelles dÃ©prÃ©ciations |

### 4.2 MÃ©triques complÃ©mentaires

#### 4.2.1 Distribution par Ã©tat

| MÃ©trique | Description | Formule |
|----------|-------------|---------|
| `count_draft` | Nombre d'Ã©lÃ©ments DRAFT | Comptage direct |
| `count_active` | Nombre d'Ã©lÃ©ments ACTIVE | Comptage direct |
| `count_deprecated` | Nombre d'Ã©lÃ©ments DEPRECATED | Comptage direct |
| `count_retired` | Nombre d'Ã©lÃ©ments RETIRED | Comptage direct |
| `count_archived` | Nombre d'Ã©lÃ©ments ARCHIVED | Comptage direct |

#### 4.2.2 Ã‚ge de la dette

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| `avg_deprecation_age` | Ã‚ge moyen des Ã©lÃ©ments DEPRECATED | > 3 cycles de release |
| `max_deprecation_age` | Ã‚ge maximum d'un Ã©lÃ©ment DEPRECATED | > 6 cycles de release |
| `avg_retirement_age` | Ã‚ge moyen des Ã©lÃ©ments RETIRED | > 2 cycles de release |
| `max_retirement_age` | Ã‚ge maximum d'un Ã©lÃ©ment RETIRED | > 4 cycles de release |

#### 4.2.3 VÃ©locitÃ© de transition

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| `transitions_per_cycle` | Nombre de transitions par cycle | < 1 (stagnation) |
| `blocked_transitions` | Transitions en attente au-delÃ  de la pÃ©riode prÃ©vue | > 0 |
| `adoption_rate` | Taux d'adoption des successeurs | < 50% Ã  mi-parcours |

#### 4.2.4 SantÃ© par catÃ©gorie

| CatÃ©gorie | Debt Ratio Max | PÃ©riode de dÃ©prÃ©ciation Min |
|-----------|----------------|----------------------------|
| **Contrats fondateurs** | 0.10 | 6 cycles de release |
| **Contrats opÃ©rationnels** | 0.25 | 3 cycles de release |
| **Interfaces techniques** | 0.40 | 2 cycles de release |
| **Ã‰lÃ©ments internes** | 0.60 | 1 cycle de release |

---

## 5. Processus de dÃ©tection et d'alerte

### 5.1 Flux de dÃ©tection

```mermaid
sequenceDiagram
    participant EB as Ever Buddy
    participant REG as Registre des Ã‰tats
    participant ALT as SystÃ¨me d'Alerte
    participant CONS as Consommateurs
    
    loop Cycle de surveillance
        EB->>REG: Collecte des Ã©tats
        REG-->>EB: Comptages par Ã©tat
        EB->>EB: Calcul du debt ratio
        EB->>EB: Calcul des mÃ©triques secondaires
        
        alt Debt ratio normal
            EB->>ALT: Enregistrement (pas d'alerte)
        else Debt ratio Ã©levÃ©
            EB->>ALT: Ã‰mission alerte prÃ©ventive
            ALT->>CONS: Notification (niveau INFO)
        else Debt ratio critique
            EB->>ALT: Ã‰mission alerte urgente
            ALT->>CONS: Notification (niveau WARNING)
            EB->>EB: GÃ©nÃ©ration plan de nettoyage
        else Debt ratio excessif
            EB->>ALT: Ã‰mission alerte bloquante
            ALT->>CONS: Notification (niveau CRITICAL)
            EB->>EB: Gel des nouvelles dÃ©prÃ©ciations
        end
    end
```

### 5.2 Niveaux d'alerte

| Niveau | Code | Condition | Action systÃ¨me |
|--------|------|-----------|----------------|
| **INFO** | `DEBT-INFO` | Debt ratio > 0.10 | Enregistrement, pas de notification |
| **NOTICE** | `DEBT-NOTICE` | Debt ratio > 0.25 | Notification aux administrateurs |
| **WARNING** | `DEBT-WARN` | Debt ratio > 0.40 | Notification + plan de nettoyage |
| **CRITICAL** | `DEBT-CRIT` | Debt ratio > 0.60 | Notification + gel des nouvelles dÃ©prÃ©ciations |

### 5.3 Contenu d'une alerte de dette

Chaque alerte de dette structurelle contient :

| Champ | Description | Obligatoire |
|-------|-------------|-------------|
| `alert_id` | Identifiant unique de l'alerte | âœ… |
| `alert_level` | Niveau d'alerte (INFO, NOTICE, WARNING, CRITICAL) | âœ… |
| `debt_ratio` | Valeur actuelle du debt ratio | âœ… |
| `threshold_exceeded` | Seuil dÃ©passÃ© | âœ… |
| `deprecated_count` | Nombre d'Ã©lÃ©ments DEPRECATED | âœ… |
| `retired_count` | Nombre d'Ã©lÃ©ments RETIRED | âœ… |
| `active_count` | Nombre d'Ã©lÃ©ments ACTIVE | âœ… |
| `top_contributors` | Liste des Ã©lÃ©ments contribuant le plus Ã  la dette | âœ… |
| `recommended_actions` | Actions de nettoyage recommandÃ©es | âœ… |
| `timestamp` | Horodatage de l'alerte | âœ… |
| `previous_alert_id` | RÃ©fÃ©rence Ã  l'alerte prÃ©cÃ©dente (si escalade) | âŒ |

---

## 6. Actions de nettoyage recommandÃ©es

### 6.1 Principes de nettoyage

Ever Buddy **recommande** des actions de nettoyage mais **ne les exÃ©cute jamais**. L'exÃ©cution est la responsabilitÃ© des produits et de KindMother.

| Principe | Description |
|----------|-------------|
| **ProgressivitÃ©** | Nettoyage par Ã©tapes, pas de suppression massive |
| **Priorisation** | Ã‰lÃ©ments les plus anciens d'abord |
| **VÃ©rification** | Confirmation de l'absence de consommateurs avant archivage |
| **TraÃ§abilitÃ©** | Documentation de chaque action de nettoyage |
| **RÃ©versibilitÃ©** | PossibilitÃ© de restaurer en cas d'erreur (jusqu'Ã  ARCHIVED) |

### 6.2 Actions par niveau de dette

#### Debt ratio Ã©levÃ© (0.25 - 0.40)

| Action | Description | Responsable |
|--------|-------------|-------------|
| **Revue des RETIRED** | Identifier les Ã©lÃ©ments RETIRED Ã©ligibles Ã  l'archivage | Administrateur |
| **AccÃ©lÃ©ration des transitions** | Contacter les consommateurs retardataires | BondingBrother |
| **Communication** | Rappel des dates de fin de support | Ever Buddy |

#### Debt ratio critique (0.40 - 0.60)

| Action | Description | Responsable |
|--------|-------------|-------------|
| **Plan de nettoyage** | Ã‰tablir un calendrier de nettoyage priorisÃ© | Ever Buddy |
| **Archivage accÃ©lÃ©rÃ©** | Archiver les Ã©lÃ©ments RETIRED sans consommateurs | KindMother |
| **Audit des blocages** | Identifier pourquoi les transitions sont bloquÃ©es | Administrateur |
| **Notification urgente** | Alerter tous les consommateurs concernÃ©s | BondingBrother |

#### Debt ratio excessif (> 0.60)

| Action | Description | Responsable |
|--------|-------------|-------------|
| **Gel des dÃ©prÃ©ciations** | Aucune nouvelle dÃ©prÃ©ciation tant que la dette n'est pas rÃ©duite | Ever Buddy |
| **Nettoyage forcÃ©** | Archivage des Ã©lÃ©ments RETIRED les plus anciens | KindMother |
| **Escalade TAMR** | Intervention humaine requise pour dÃ©bloquer la situation | TAMR |
| **Audit de crise** | Analyse des causes de l'accumulation | Administrateur |

### 6.3 CritÃ¨res d'Ã©ligibilitÃ© Ã  l'archivage

Un Ã©lÃ©ment RETIRED est Ã©ligible Ã  l'archivage quand :

| CritÃ¨re | Condition | VÃ©rification |
|---------|-----------|--------------|
| **PÃ©riode de grÃ¢ce** | PÃ©riode de grÃ¢ce Ã©coulÃ©e | Automatique |
| **Absence de consommateurs** | Aucun consommateur actif | Audit |
| **Documentation complÃ¨te** | Historique complet disponible | Automatique |
| **Successeur stable** | Successeur en Ã©tat ACTIVE et stable | Automatique |

---

## 7. Surveillance par type d'Ã©lÃ©ment

### 7.1 Ã‰lÃ©ments Ã  surveiller

| Type d'Ã©lÃ©ment | GouvernÃ© par | Surveillance dette |
|----------------|--------------|-------------------|
| **Contrats de cores** | Ever Buddy | âœ… Critique |
| **Interfaces techniques** | Master Butler | âœ… Standard |
| **SchÃ©mas de donnÃ©es** | KindMother (Ã©volution par Ever Buddy) | âœ… Critique |
| **Tools** | Ever Buddy | âœ… Standard |
| **Toolkits** | Ever Buddy | âœ… Standard |
| **RÃ¨gles StrongFather** | StrongFather (Ã©volution par Ever Buddy) | âœ… Critique |

### 7.2 Exclusions de la surveillance

| Type d'Ã©lÃ©ment | Raison de l'exclusion |
|----------------|----------------------|
| **DonnÃ©es mÃ©tier** | Pas de cycle de vie structurel, domaine de KindMother |
| **Sessions utilisateur** | Ã‰phÃ©mÃ¨res, pas de dette |
| **Caches** | Ã‰phÃ©mÃ¨res, pas de dette |
| **Logs** | Archivage sÃ©parÃ©, pas de transition d'Ã©tat |

---

## 8. Invariants applicables

Ce contrat respecte et applique les invariants suivants de la [Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) :

### INV-EB-1 : Aucune exÃ©cution de migration

Ever Buddy surveille la dette mais **n'exÃ©cute jamais** de nettoyage ou d'archivage. Il recommande, il alerte, mais l'exÃ©cution est dÃ©lÃ©guÃ©e.

### INV-EB-4 : PÃ©riode de dÃ©prÃ©ciation obligatoire

La dette structurelle existe **parce que** les pÃ©riodes de dÃ©prÃ©ciation sont obligatoires. C'est le prix de la continuitÃ© et de la protection des consommateurs.

### INV-EB-6 : Vision long terme obligatoire

La surveillance de la dette garantit que les dÃ©cisions d'Ã©volution considÃ¨rent l'impact Ã  long terme. Une dette excessive est le symptÃ´me de dÃ©cisions court-termistes.

### INV-EB-7 : Documentation obligatoire

Chaque alerte de dette est documentÃ©e avec les raisons, l'impact, et les recommandations. Cette documentation est immuable.

### INV-EB-12 : ResponsabilitÃ© de l'annonce

Ever Buddy est responsable d'alerter sur la dette excessive. Les consommateurs et les administrateurs sont responsables d'agir sur ces alertes.

---

## 9. ConformitÃ© aux Lois d'Autonomie SystÃ¨me

Ce contrat est conforme aux [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | La surveillance de dette est locale, aucune dÃ©pendance externe |
| **LOI-2** | âœ… | La surveillance fonctionne en mode isolÃ© |
| **LOI-3** | âœ… | Les mÃ©triques de dette locales sont souveraines |
| **LOI-4** | âœ… | La dette est mesurÃ©e en cycles de release, pas en temps absolu |
| **LOI-5** | âœ… | La surveillance est lÃ©gÃ¨re, pas de workers permanents |
| **LOI-6** | âœ… | Les mÃ©triques de dette peuvent Ãªtre fÃ©dÃ©rÃ©es via BondingBrother |

---

## 10. Relations avec les autres cores

### 10.1 Ever Buddy â†’ Caring Nanny

Ever Buddy fournit les mÃ©triques de dette structurelle Ã  Caring Nanny pour l'observation globale de la santÃ© du systÃ¨me.

| MÃ©trique fournie | Usage par Caring Nanny |
|-----------------|------------------------|
| `debt_ratio` | Indicateur de santÃ© Ã©volutive |
| `blocked_transitions` | Indicateur de stagnation |
| `alert_level` | Niveau de prÃ©occupation |

### 10.2 Ever Buddy â†’ StrongFather

Ever Buddy consulte StrongFather avant d'Ã©mettre des alertes bloquantes (gel des dÃ©prÃ©ciations).

| Consultation | Raison |
|--------------|--------|
| Gel des dÃ©prÃ©ciations | DÃ©cision stratÃ©gique nÃ©cessitant validation |
| Escalade TAMR | Intervention humaine nÃ©cessitant autorisation |

### 10.3 Ever Buddy â†’ BondingBrother

BondingBrother relaie les alertes de dette aux produits concernÃ©s.

| Action | RÃ´le de BondingBrother |
|--------|------------------------|
| Notification | Traduction et distribution des alertes |
| Communication consommateurs | Relais des messages de migration |

### 10.4 Ever Buddy â†’ KindMother

KindMother exÃ©cute les archivages recommandÃ©s par Ever Buddy.

| Action | RÃ´le de KindMother |
|--------|-------------------|
| Archivage | ExÃ©cution technique de l'archivage |
| CrÃ©ation de tombstones | Conservation des mÃ©tadonnÃ©es minimales |

---

## 11. Anti-patterns et violations

### 11.1 Violations de ce contrat

| Violation | Description | ConsÃ©quence |
|-----------|-------------|-------------|
| **VIOL-DT-1** | Ignorer les alertes de dette | Accumulation incontrÃ´lÃ©e |
| **VIOL-DT-2** | Archiver sans vÃ©rifier les consommateurs | Rupture de service |
| **VIOL-DT-3** | Contourner le gel des dÃ©prÃ©ciations | Aggravation de la dette |
| **VIOL-DT-4** | Manipulation des mÃ©triques | Perte de visibilitÃ© |
| **VIOL-DT-5** | Nettoyage massif sans progressivitÃ© | Risque de rÃ©gression |

### 11.2 Anti-patterns

| Anti-pattern | Description | Correction |
|--------------|-------------|------------|
| **DÃ©ni de dette** | ConsidÃ©rer que la dette n'est pas un problÃ¨me | Surveillance rÃ©guliÃ¨re, seuils stricts |
| **Nettoyage panique** | Archiver massivement sous la pression | Plan de nettoyage progressif |
| **Dette cachÃ©e** | Ne pas dÃ©clarer les Ã©lÃ©ments obsolÃ¨tes | Audit rÃ©gulier des Ã©tats |
| **Ã‰ternelle dÃ©prÃ©ciation** | Maintenir des Ã©lÃ©ments DEPRECATED indÃ©finiment | PÃ©riodes de dÃ©prÃ©ciation maximales |
| **Archivage prÃ©maturÃ©** | Archiver avant la fin de la pÃ©riode de grÃ¢ce | Respect strict des pÃ©riodes |

---

## 12. ScÃ©nario type : Dette excessive

Ce scÃ©nario illustre le processus complet de gestion d'une dette structurelle excessive.

### Contexte

Le systÃ¨me a accumulÃ© une dette importante suite Ã  plusieurs Ã©volutions majeures. Le debt ratio atteint 0.55 (critique).

### SÃ©quence

```mermaid
sequenceDiagram
    participant EB as Ever Buddy
    participant SF as StrongFather
    participant BB as BondingBrother
    participant PROD as Produits
    participant KM as KindMother
    participant TAMR as TAMR
    
    Note over EB: Cycle de surveillance
    EB->>EB: Calcul debt ratio = 0.55
    EB->>EB: Seuil CRITICAL (0.40) dÃ©passÃ©
    
    EB->>SF: Demande validation gel dÃ©prÃ©ciations
    SF-->>EB: Autorisation accordÃ©e
    
    EB->>BB: Ã‰mission alerte DEBT-CRIT
    BB->>PROD: Distribution notification urgente
    
    EB->>EB: GÃ©nÃ©ration plan de nettoyage
    Note over EB: Top 10 Ã©lÃ©ments RETIRED<br/>Ã©ligibles Ã  l'archivage
    
    EB->>BB: Transmission plan de nettoyage
    BB->>PROD: Communication plan
    
    PROD->>BB: Confirmation migration terminÃ©e
    BB->>EB: Rapport d'adoption
    
    EB->>KM: Recommandation archivage Ã©lÃ©ments Ã©ligibles
    KM->>KM: ExÃ©cution archivage
    KM-->>EB: Confirmation archivage
    
    EB->>EB: Recalcul debt ratio = 0.35
    Note over EB: Passage sous seuil CRITICAL
    
    EB->>BB: Ã‰mission alerte DEBT-WARN (dÃ©sescalade)
    EB->>EB: LevÃ©e gel dÃ©prÃ©ciations
    
    alt Debt ratio reste Ã©levÃ© > 30 jours
        EB->>TAMR: Escalade intervention humaine
        TAMR->>TAMR: Analyse et dÃ©cision
    end
```

### RÃ©sultat attendu

- Le debt ratio passe de 0.55 Ã  0.35
- Les Ã©lÃ©ments RETIRED Ã©ligibles sont archivÃ©s
- Les consommateurs retardataires sont notifiÃ©s
- Le gel des dÃ©prÃ©ciations est levÃ©
- La situation est documentÃ©e pour analyse future

---

## 13. Conclusion et statut contractuel

### SynthÃ¨se

La surveillance de la dette structurelle est une responsabilitÃ© exclusive d'Ever Buddy. Ce contrat garantit que :

- La dette est mesurÃ©e de maniÃ¨re cohÃ©rente et reproductible
- Les seuils d'alerte sont clairs et non nÃ©gociables
- Les actions de nettoyage sont recommandÃ©es mais jamais forcÃ©es
- La traÃ§abilitÃ© des alertes et des actions est complÃ¨te
- La conformitÃ© aux Lois d'Autonomie SystÃ¨me est assurÃ©e

### Phrase fondatrice

> **La dette structurelle est le prix de la continuitÃ©. Ever Buddy la surveille pour qu'elle reste un investissement, pas un fardeau.**

### Statut

Ce document est de statut **CONTRAT NORMATIF**. Il complÃ¨te la [Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) et fait autoritÃ© pour tout ce qui concerne la surveillance de la dette structurelle.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT NORMATIF â€” ComplÃ©ment Ã  la Documentation Fondatrice  
**RÃ©fÃ©rence :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md), [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md), [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

