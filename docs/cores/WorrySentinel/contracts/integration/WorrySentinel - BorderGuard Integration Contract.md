# WorrySentinel - BorderGuard Integration Contract

## 1. Contexte

Ce document definit formellement le contrat d'integration entre **WorrySentinel** (core de gouvernance transversale de securite) et **BorderGuard** (core de definition des frontieres et classification de confiance). Il precise les flux bidirectionnels, les responsabilites respectives, les regles d'adaptation, et les invariants de cette integration strategique.

**Document fondateur WorrySentinel :** [Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Document fondateur BorderGuard :** [Border Guard - Documentation Fondatrice](..//..//..//BorderGuard//foundation//Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non negociable**. Il etablit les regles absolues de la relation entre WorrySentinel et BorderGuard.

---

## 2. Portee / Scope

### Ce document couvre

- La nature de la relation WorrySentinel-BorderGuard
- Le flux descendant de gouvernance (WorrySentinel â†’ BorderGuard)
- Le flux montant de signaux (BorderGuard â†’ WorrySentinel)
- L'adaptation des frontieres selon les niveaux de securite (0-4)
- L'adaptation des frontieres selon les etats de confiance (T0-T4)
- La matrice d'interaction entre gouvernance et definition de frontieres
- Les invariants de l'integration
- Les violations et anti-patterns

### Ce document ne couvre pas

- La definition detaillee des niveaux de securite (voir [Security Levels Governance Contract](../levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md))
- La definition detaillee des etats de confiance (voir [Trust States Governance Contract](../levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md))
- Les mecanismes de degradation progressive (voir [Progressive Degradation Contract](../degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md))
- Les details internes de BorderGuard (voir documentation BorderGuard)

---

## 3. Principe fondamental de la relation

### 3.1 Complementarite sans chevauchement

WorrySentinel et BorderGuard sont **complementaires et independants**. Chacun possede son domaine d'autorite exclusive :

| Core | Domaine d'autorite | Ce qu'il ne fait jamais |
|------|-------------------|------------------------|
| **WorrySentinel** | Gouvernance des niveaux de securite (0-4), gouvernance des etats de confiance (T0-T4), orchestration degradation | Ne definit pas de frontieres, ne classifie pas les sources |
| **BorderGuard** | Definition des frontieres, classification de confiance (trusted/verified/unknown/hostile), regles de franchissement | Ne gouverne pas les niveaux de securite, ne modifie pas l'etat global |

### 3.2 Nature de l'interaction

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     RELATION WORRYSENTINEL â†” BORDERGUARD                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  WorrySentinel                                         BorderGuard           â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                         â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€           â”‚
â”‚  â€¢ Gouverne les niveaux de securite (0-4)             â€¢ Definit les frontieresâ”‚
â”‚  â€¢ Gouverne les etats de confiance (T0-T4)            â€¢ Classifie la confianceâ”‚
â”‚  â€¢ Impose contraintes de durcissement                    (trustedâ†’hostile)   â”‚
â”‚  â€¢ Observe les signaux d'anomalie                     â€¢ Etablit regles de    â”‚
â”‚  â€¢ Ne definit JAMAIS de frontiere                       franchissement       â”‚
â”‚  â€¢ Ne classifie JAMAIS de source                      â€¢ Ne gouverne JAMAIS   â”‚
â”‚                                                          l'etat global       â”‚
â”‚                              â–¼                              â–²                â”‚
â”‚                              â”‚                              â”‚                â”‚
â”‚                    FLUX DESCENDANT                  FLUX MONTANT             â”‚
â”‚                    (durcissement)                   (signaux)                â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 3.3 Distinction entre niveaux de confiance

**IMPORTANT :** Deux concepts distincts de "confiance" coexistent :

| Concept | Defini par | Echelle | Nature |
|---------|------------|---------|--------|
| **Etat de confiance systeme** | WorrySentinel | T0 â†’ T4 | Integrite globale du systeme |
| **Niveau de confiance source** | BorderGuard | trusted â†’ hostile | Classification d'une source/destination |

Ces deux concepts sont **independants mais interconnectes**. L'etat de confiance systeme (T0-T4) influence la rigueur avec laquelle BorderGuard classifie les sources.

---

## 4. Flux descendant : Gouvernance â†’ Frontieres

WorrySentinel impose des contraintes de gouvernance sur BorderGuard selon deux axes : le niveau de securite declare et l'etat de confiance du systeme.

### 4.1 Contraintes selon niveau de securite (0-4)

WorrySentinel gouverne l'adaptation des frontieres selon le niveau de securite declare par l'Operateur :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚             CONTRAINTES WORRYSENTINEL â†’ BORDERGUARD (NIVEAU SECURITE)        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  Niveau 0 (PUBLIC)          â”€â”€â†’ Frontieres assouplies, classification simple â”‚
â”‚                                                                              â”‚
â”‚  Niveau 1 (STANDARD)        â”€â”€â†’ Frontieres standard, classification normale  â”‚
â”‚                                                                              â”‚
â”‚  Niveau 2 (SENSITIVE)       â”€â”€â†’ Frontieres renforcees, classification stricteâ”‚
â”‚                                                                              â”‚
â”‚  Niveau 3 (CRITICAL)        â”€â”€â†’ Frontieres strictes, classification rigoureuseâ”‚
â”‚                                                                              â”‚
â”‚  Niveau 4 (HARDENED)        â”€â”€â†’ Frontieres maximales, classification ultra-stricteâ”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Tableau detaille des contraintes par niveau :**

| Niveau securite | Permeabilite frontiere externe | Classification sources | Seuil hostile | TTL VERIFIED |
|-----------------|-------------------------------|----------------------|---------------|--------------|
| **0 - PUBLIC** | Ouverte | Simplifiee | Haut (tolerant) | Long (heures) |
| **1 - STANDARD** | Controlee | Normale | Standard | Standard (minutes) |
| **2 - SENSITIVE** | Controlee + verif | Renforcee | Bas (sensible) | Court |
| **3 - CRITICAL** | Stricte | Stricte | Tres bas | Tres court |
| **4 - HARDENED** | Fermee | Ultra-stricte | Zero tolerance | Minimal |

### 4.2 Contraintes selon etat de confiance (T0-T4)

WorrySentinel impose un durcissement progressif des frontieres selon l'etat de confiance global :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚             CONTRAINTES WORRYSENTINEL â†’ BORDERGUARD (ETAT CONFIANCE)         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  T0 (NORMAL)      â”€â”€â†’ Fonctionnement normal des frontieres                   â”‚
â”‚                                                                              â”‚
â”‚  T1 (INSTABLE)    â”€â”€â†’ + Verifications supplementaires                        â”‚
â”‚                        + Surveillance renforcee des frontieres               â”‚
â”‚                        + Reevaluation plus frequente des classifications     â”‚
â”‚                                                                              â”‚
â”‚  T2 (DEGRADE)     â”€â”€â†’ Frontieres resserrees                                  â”‚
â”‚                        Certains types de franchissement suspendus            â”‚
â”‚                        Classification plus restrictive                       â”‚
â”‚                                                                              â”‚
â”‚  T3 (RESTREINT)   â”€â”€â†’ Frontieres minimales                                   â”‚
â”‚                        Uniquement franchissements essentiels                 â”‚
â”‚                        Classification ultra-stricte                          â”‚
â”‚                                                                              â”‚
â”‚  T4 (BLOQUE)      â”€â”€â†’ Frontieres fermees                                     â”‚
â”‚                        Aucun franchissement externe autorise                 â”‚
â”‚                        Mode isolement total                                  â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Tableau des actions de durcissement :**

| Etat confiance | Durcissement frontieres | Action sur integrations | Impact classification |
|----------------|------------------------|------------------------|----------------------|
| **T0** | Aucun | Aucune | Normale |
| **T1** | + Verifications | Surveillance accrue | + Rigueur |
| **T2** | Resserrement | Restrictions moderees | Stricte |
| **T3** | Frontieres minimales | Suspension non essentielles | Ultra-stricte |
| **T4** | Fermees | Toutes suspendues | N/A (isole) |

### 4.3 Interface de gouvernance

WorrySentinel communique ses contraintes a BorderGuard via une interface conceptuelle :

| Interface | Description | Direction |
|-----------|-------------|-----------|
| `IBoundaryHardening` | Niveau de durcissement requis des frontieres | WS â†’ BG |
| `IClassificationRigor` | Rigueur de classification requise | WS â†’ BG |
| `IPermeabilityConstraint` | Contrainte de permeabilite des frontieres | WS â†’ BG |
| `IIntegrationRestriction` | Restrictions sur les integrations externes | WS â†’ BG |

---

## 5. Flux montant : Signaux â†’ Gouvernance

BorderGuard signale a WorrySentinel les evenements qui peuvent influencer l'etat de confiance global.

### 5.1 Types de signaux

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     SIGNAUX BORDERGUARD â†’ WORRYSENTINEL                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  BorderGuard                                              WorrySentinel      â”‚
â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€                                              â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€      â”‚
â”‚                                                                              â”‚
â”‚  â€¢ Anomalies I/O                           â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’   Correlation et      â”‚
â”‚    (patterns inhabituels, frequences                     analyse             â”‚
â”‚     anormales, volumes suspects)                                             â”‚
â”‚                                                                              â”‚
â”‚  â€¢ Violations de frontieres                â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’   Impact sur etat     â”‚
â”‚    (tentatives non autorisees,                           de confiance        â”‚
â”‚     contournements detectes)                                                 â”‚
â”‚                                                                              â”‚
â”‚  â€¢ Classifications HOSTILE declenchees     â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’   Signal d'alerte     â”‚
â”‚    (sources nouvellement hostiles)                                           â”‚
â”‚                                                                              â”‚
â”‚  â€¢ Defaillances d'integrations             â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’   Evaluation          â”‚
â”‚    (partenaires defaillants,                             degradation         â”‚
â”‚     connexions interrompues)                                                 â”‚
â”‚                                                                              â”‚
â”‚  â€¢ Patterns de franchissement anormaux     â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â†’   Detection           â”‚
â”‚    (tentatives repetees,                                 intrusion           â”‚
â”‚     escalade de privileges)                                                  â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Catalogue des signaux

| Signal | Description | Gravite | Impact potentiel sur T |
|--------|-------------|---------|------------------------|
| `SIG-BG-ANOMALY-IO` | Pattern I/O inhabituel detecte | Moderee | T0 â†’ T1 |
| `SIG-BG-VIOLATION` | Tentative de franchissement non autorise | Haute | T1 â†’ T2 |
| `SIG-BG-HOSTILE-NEW` | Nouvelle source classifiee HOSTILE | Haute | T1 â†’ T2 |
| `SIG-BG-INTEGRATION-FAIL` | Integration externe defaillante | Moderee | T0 â†’ T1 |
| `SIG-BG-ESCALATION` | Tentative d'escalade de privileges | Critique | T2 â†’ T3 |
| `SIG-BG-BREACH` | Violation confirmee de frontiere | Critique | T2 â†’ T3+ |
| `SIG-BG-PATTERN-ATTACK` | Pattern d'attaque detecte | Critique | T2 â†’ T3+ |

### 5.3 Interface de signalement

| Interface | Description | Direction |
|-----------|-------------|-----------|
| `IBoundarySignal` | Signalement de violation de frontiere | BG â†’ WS |
| `IAnomalyIOSignal` | Signalement d'anomalie I/O | BG â†’ WS |
| `IHostileClassificationSignal` | Signalement de nouvelle classification hostile | BG â†’ WS |
| `IIntegrationFailureSignal` | Signalement de defaillance d'integration | BG â†’ WS |

---

## 6. Matrice d'interaction Niveau securite Ã— Etat confiance

L'interaction entre le niveau de securite et l'etat de confiance produit un comportement combine pour BorderGuard.

### 6.1 Matrice de comportement des frontieres

```
                        NIVEAUX DE SECURITE
                    0        1        2        3        4
                â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”
         T0     â”‚ Ouvert â”‚Standardâ”‚RenforcÃ©â”‚ Strict â”‚ FermÃ©  â”‚
                â”‚        â”‚        â”‚        â”‚        â”‚sauf intâ”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
E        T1     â”‚Standardâ”‚Standardâ”‚Strict  â”‚Strict+ â”‚FermÃ©   â”‚
T               â”‚+verif  â”‚+verif  â”‚        â”‚+attestaâ”‚        â”‚
A               â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
T        T2     â”‚RenforcÃ©â”‚Strict  â”‚Minimal â”‚Minimal â”‚IsolÃ©   â”‚
S               â”‚        â”‚        â”‚        â”‚+gel    â”‚        â”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         T3     â”‚Strict  â”‚Minimal â”‚IsolÃ©   â”‚IsolÃ©   â”‚IsolÃ©   â”‚
                â”‚        â”‚        â”‚partiel â”‚total   â”‚total   â”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         T4     â”‚IsolÃ©   â”‚IsolÃ©   â”‚IsolÃ©   â”‚IsolÃ©   â”‚IsolÃ©   â”‚
                â”‚        â”‚        â”‚        â”‚        â”‚        â”‚
                â””â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”˜

Legende :
â€¢ Ouvert    : Franchissement libre sous conditions minimales
â€¢ Standard  : Franchissement controle
â€¢ RenforcÃ©  : Franchissement soumis a verification stricte
â€¢ Strict    : Zero-trust strict, signatures obligatoires
â€¢ Minimal   : Uniquement franchissements essentiels
â€¢ IsolÃ©     : Frontieres fermees, mode survie
```

### 6.2 Matrice de classification des sources

```
                        NIVEAUX DE SECURITE
                    0        1        2        3        4
                â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”
         T0     â”‚Simple  â”‚Normale â”‚RenforcÃ©eâ”‚Stricte â”‚Ultra   â”‚
                â”‚        â”‚        â”‚        â”‚        â”‚stricte â”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
E        T1     â”‚Normale â”‚Normale â”‚Stricte â”‚Stricte â”‚Ultra   â”‚
T               â”‚        â”‚+trace  â”‚        â”‚+verif  â”‚stricte â”‚
A               â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
T        T2     â”‚RenforcÃ©eâ”‚Stricteâ”‚Ultra   â”‚Ultra   â”‚Tout    â”‚
S               â”‚        â”‚        â”‚stricte â”‚stricte â”‚suspect â”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         T3     â”‚Stricte â”‚Ultra   â”‚Tout    â”‚Tout    â”‚Tout    â”‚
                â”‚        â”‚stricte â”‚suspect â”‚hostile â”‚hostile â”‚
                â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”¤
         T4     â”‚Tout    â”‚Tout    â”‚Tout    â”‚Tout    â”‚Tout    â”‚
                â”‚suspect â”‚hostile â”‚hostile â”‚hostile â”‚hostile â”‚
                â””â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 7. Regles d'integration

### 7.1 Regles de gouvernance descendante

| Regle | Description |
|-------|-------------|
| **REGLE-WS-BG-1** | WorrySentinel impose le durcissement des frontieres sans les definir |
| **REGLE-WS-BG-2** | BorderGuard adapte ses definitions selon les contraintes de WorrySentinel |
| **REGLE-WS-BG-3** | Tout changement d'etat de confiance declenche une reevaluation des frontieres |
| **REGLE-WS-BG-4** | BorderGuard ne peut pas assouplir une frontiere au-dela des contraintes imposees |
| **REGLE-WS-BG-5** | En etat T3+, BorderGuard passe en mode frontieres minimales automatiquement |
| **REGLE-WS-BG-6** | En etat T4, BorderGuard isole completement le systeme |

### 7.2 Regles de signalement montant

| Regle | Description |
|-------|-------------|
| **REGLE-BG-WS-1** | BorderGuard signale toute anomalie I/O significative |
| **REGLE-BG-WS-2** | BorderGuard signale toute violation de frontiere |
| **REGLE-BG-WS-3** | BorderGuard signale toute classification HOSTILE declenchee |
| **REGLE-BG-WS-4** | Les signaux sont emis en temps reel sans delai |
| **REGLE-BG-WS-5** | Les signaux incluent le contexte complet (source, frontiere, moment) |
| **REGLE-BG-WS-6** | BorderGuard ne decide pas de l'impact sur l'etat de confiance |

### 7.3 Regles de coherence

| Regle | Description |
|-------|-------------|
| **REGLE-COHERENCE-1** | La classification de confiance source est independante de l'etat de confiance systeme |
| **REGLE-COHERENCE-2** | Le niveau de securite influence la rigueur, pas le resultat de classification |
| **REGLE-COHERENCE-3** | Un etat de confiance degrade ne transforme pas automatiquement toutes les sources en HOSTILE |
| **REGLE-COHERENCE-4** | Les contraintes sont additives : niveau securite + etat confiance = restrictions combinees |

---

## 8. Invariants du contrat

Ces invariants sont **non negociables** et definissent les limites absolues de l'integration.

### 8.1 Invariants de WorrySentinel dans cette integration

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-WS-BG-1** | Aucune definition de frontiere | WorrySentinel ne definit jamais de frontiere |
| **INV-WS-BG-2** | Aucune classification de source | WorrySentinel ne classifie jamais trusted/verified/unknown/hostile |
| **INV-WS-BG-3** | Aucune execution de controle | WorrySentinel n'execute jamais de verification de frontiere |
| **INV-WS-BG-4** | Contraintes uniquement | WorrySentinel impose des contraintes, pas des definitions |

### 8.2 Invariants de BorderGuard dans cette integration

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-BG-WS-1** | Aucune gouvernance d'etat | BorderGuard ne gouverne jamais l'etat de confiance systeme |
| **INV-BG-WS-2** | Aucune modification d'etat | BorderGuard ne modifie jamais l'etat de confiance systeme |
| **INV-BG-WS-3** | Signalement uniquement | BorderGuard signale, WorrySentinel correle et decide |
| **INV-BG-WS-4** | Respect des contraintes | BorderGuard ne peut pas contourner les contraintes de WorrySentinel |

### 8.3 Invariants mutuels

| Code | Invariant | Description |
|------|-----------|-------------|
| **INV-MUTUAL-1** | Separation des domaines | Chaque core reste souverain dans son domaine |
| **INV-MUTUAL-2** | Pas de dependance cyclique | Les flux sont unidirectionnels par type (gouvernance vs signaux) |
| **INV-MUTUAL-3** | Tracabilite complete | Tout flux (contrainte ou signal) est trace |
| **INV-MUTUAL-4** | Zero-trust mutuel | Chaque core valide les informations recues |

---

## 9. Violations et anti-patterns

### 9.1 Violations

| Code | Violation | Invariant viole |
|------|-----------|-----------------|
| **VIOL-WS-BG-1** | WorrySentinel definit une frontiere | INV-WS-BG-1 |
| **VIOL-WS-BG-2** | WorrySentinel classifie une source | INV-WS-BG-2 |
| **VIOL-WS-BG-3** | WorrySentinel execute un controle de frontiere | INV-WS-BG-3 |
| **VIOL-BG-WS-1** | BorderGuard modifie l'etat de confiance | INV-BG-WS-2 |
| **VIOL-BG-WS-2** | BorderGuard ignore les contraintes de WorrySentinel | INV-BG-WS-4 |
| **VIOL-BG-WS-3** | BorderGuard decide de l'impact d'un signal | INV-BG-WS-3 |

### 9.2 Anti-patterns

| Anti-pattern | Description | Correction |
|--------------|-------------|------------|
| **Confusion confiance** | Confondre etat de confiance systeme (T0-T4) et classification source (trusted/hostile) | Maintenir la distinction : systeme â‰  source |
| **Bypass contraintes** | BorderGuard ignore les contraintes en invoquant la commodite | Les contraintes sont non negociables |
| **Correlation locale** | BorderGuard correle les signaux au lieu de les transmettre | WorrySentinel est seul responsable de la correlation |
| **Gouvernance de frontiere** | WorrySentinel specifie des frontieres au lieu de contraintes | WorrySentinel gouverne le durcissement, pas les definitions |
| **Decision operationnelle** | WorrySentinel prend des decisions de blocage de franchissement | StrongFather decide, BorderGuard definit, WorrySentinel gouverne |

---

## 10. Exemples concrets

### 10.1 Scenario : Degradation T0 â†’ T2

```
CONTEXTE : Operateur niveau 2 (SENSITIVE), etat initial T0

1. BorderGuard detecte des patterns I/O anormaux
   â†’ Signal SIG-BG-ANOMALY-IO emis vers WorrySentinel

2. WorrySentinel correle avec d'autres signaux
   â†’ Transition T0 â†’ T1 declaree

3. WorrySentinel impose contraintes T1 a BorderGuard :
   â†’ + Verifications supplementaires
   â†’ Reevaluation plus frequente des classifications

4. BorderGuard detecte une violation de frontiere
   â†’ Signal SIG-BG-VIOLATION emis vers WorrySentinel

5. WorrySentinel correle
   â†’ Transition T1 â†’ T2 declaree

6. WorrySentinel impose contraintes T2 a BorderGuard :
   â†’ Frontieres resserrees
   â†’ Classification ultra-stricte
   â†’ Certains types de franchissement suspendus

7. BorderGuard adapte ses definitions :
   â†’ Frontieres externes passent en mode "Strict"
   â†’ Integrations non essentielles suspendues
   â†’ Classification devient ultra-stricte
```

### 10.2 Scenario : Niveau de securite eleve

```
CONTEXTE : Operateur niveau 4 (HARDENED), etat T0

1. WorrySentinel impose contraintes niveau 4 a BorderGuard :
   â†’ Frontieres maximales (fermees sauf interne)
   â†’ Classification ultra-stricte
   â†’ Seuil hostile = zero tolerance
   â†’ Reevaluation TRUSTED constante

2. BorderGuard applique ces contraintes :
   â†’ Frontiere externe : Fermee
   â†’ Frontiere integration : Minimale/Aucune
   â†’ TTL VERIFIED : Minimal
   â†’ Criteres VERIFIED : Ultra-stricts (verification continue)
   â†’ Distribution TRUSTED : Quasi nulle (isolement)

3. Meme en T0, le systeme fonctionne en mode tres restrictif
   car le niveau de securite l'impose.
```

### 10.3 Scenario : Signal hostile

```
CONTEXTE : Operateur niveau 2, etat T1

1. BorderGuard classifie une source comme HOSTILE
   (pattern d'attaque detecte)

2. Signal SIG-BG-HOSTILE-NEW emis vers WorrySentinel
   Contenu : {source, raison, frontiere, moment}

3. WorrySentinel correle avec signaux existants :
   â†’ Plusieurs sources hostiles = pattern
   â†’ Correlation avec StrongFather (decisions refusees)

4. WorrySentinel decide de la transition :
   â†’ Si pattern confirme : T1 â†’ T2
   â†’ Si isole : T1 maintenu, surveillance accrue

5. BorderGuard ne connait PAS le resultat de la correlation
   Il continue a signaler et a appliquer les contraintes recues
```

---

## 11. Protocoles de communication

### 11.1 Protocole de contrainte descendante

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     PROTOCOLE CONTRAINTE (WS â†’ BG)                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  1. WorrySentinel evalue le contexte                                         â”‚
â”‚     â€¢ Niveau de securite de l'Operateur                                     â”‚
â”‚     â€¢ Etat de confiance actuel                                              â”‚
â”‚                                                                              â”‚
â”‚  2. WorrySentinel calcule les contraintes                                    â”‚
â”‚     â€¢ Niveau de durcissement requis                                         â”‚
â”‚     â€¢ Rigueur de classification requise                                     â”‚
â”‚     â€¢ Restrictions sur integrations                                         â”‚
â”‚                                                                              â”‚
â”‚  3. WorrySentinel emet les contraintes                                       â”‚
â”‚     â€¢ Via interfaces IBoundaryHardening, IClassificationRigor, etc.          â”‚
â”‚     â€¢ Contraintes explicites et non ambigues                                â”‚
â”‚                                                                              â”‚
â”‚  4. BorderGuard recoit et valide                                             â”‚
â”‚     â€¢ Verification coherence des contraintes                                â”‚
â”‚     â€¢ Journalisation de la reception                                        â”‚
â”‚                                                                              â”‚
â”‚  5. BorderGuard adapte ses definitions                                       â”‚
â”‚     â€¢ Application immediate des contraintes                                 â”‚
â”‚     â€¢ Reevaluation des classifications en cours                             â”‚
â”‚     â€¢ Ajustement de la permeabilite des frontieres                          â”‚
â”‚                                                                              â”‚
â”‚  6. BorderGuard confirme l'application                                       â”‚
â”‚     â€¢ Retour de confirmation vers WorrySentinel (optionnel)                  â”‚
â”‚     â€¢ Journalisation des adaptations effectuees                             â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 11.2 Protocole de signal montant

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                     PROTOCOLE SIGNAL (BG â†’ WS)                               â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                              â”‚
â”‚  1. BorderGuard detecte un evenement                                         â”‚
â”‚     â€¢ Anomalie I/O                                                          â”‚
â”‚     â€¢ Violation de frontiere                                                â”‚
â”‚     â€¢ Classification HOSTILE                                                â”‚
â”‚     â€¢ Defaillance d'integration                                             â”‚
â”‚                                                                              â”‚
â”‚  2. BorderGuard construit le signal                                          â”‚
â”‚     â€¢ Type de signal (SIG-BG-*)                                             â”‚
â”‚     â€¢ Contexte complet (source, frontiere, moment)                          â”‚
â”‚     â€¢ Gravite evaluee                                                       â”‚
â”‚     â€¢ Details techniques pertinents                                         â”‚
â”‚                                                                              â”‚
â”‚  3. BorderGuard emet le signal                                               â”‚
â”‚     â€¢ Via interface IBoundarySignal ou specifique                            â”‚
â”‚     â€¢ Emission immediate (pas de batching)                                  â”‚
â”‚     â€¢ Journalisation de l'emission                                          â”‚
â”‚                                                                              â”‚
â”‚  4. WorrySentinel recoit et journalise                                       â”‚
â”‚     â€¢ Reception confirmee                                                   â”‚
â”‚     â€¢ Signal ajoute a la correlation                                        â”‚
â”‚                                                                              â”‚
â”‚  5. WorrySentinel correle                                                    â”‚
â”‚     â€¢ Correlation avec autres signaux                                       â”‚
â”‚     â€¢ Evaluation de l'impact potentiel                                      â”‚
â”‚     â€¢ Decision de transition (ou non)                                       â”‚
â”‚                                                                              â”‚
â”‚  6. Si transition : nouvelles contraintes emises                             â”‚
â”‚     â†’ Retour au protocole de contrainte                                     â”‚
â”‚                                                                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 12. Documents associes

### Documentation WorrySentinel

- [WorrySentinel - Index de Navigation](../../_index.md)
- [WorrySentinel - Documentation Fondatrice](../../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Architecture & Flows](../../architecture/WorrySentinel%20-%20Architecture%20&%20Flows.md)
- [WorrySentinel - Security Levels Governance Contract](../levels/WorrySentinel%20-%20Security%20Levels%20Governance%20Contract.md)
- [WorrySentinel - Trust States Governance Contract](../levels/WorrySentinel%20-%20Trust%20States%20Governance%20Contract.md)
- [WorrySentinel - Progressive Degradation Contract](../degradation/WorrySentinel%20-%20Progressive%20Degradation%20Contract.md)

### Documentation BorderGuard

- [BorderGuard - Index de Navigation](..//..//..//BorderGuard//_index.md)
- [Border Guard - Documentation Fondatrice](..//..//..//BorderGuard//foundation//Border%20Guard%20-%20Documentation%20Fondatrice.md)
- [Border Guard - Security Levels Adaptation Contract](..//..//..//BorderGuard//contracts//security//Border%20Guard%20-%20Security%20Levels%20Adaptation%20Contract.md)
- [Border Guard - Boundary Definition Contract](..//..//..//BorderGuard//contracts//boundaries//Border%20Guard%20-%20Boundary%20Definition%20Contract.md)
- [Border Guard - Trust Level Classification Contract](..//..//..//BorderGuard//contracts//boundaries//Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

### Documentation transversale

- [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 13. Synthese contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Separation des domaines** â€” WorrySentinel gouverne, BorderGuard definit, aucun chevauchement
2. **Flux bidirectionnels** â€” Gouvernance descendante, signaux montants
3. **Adaptation automatique** â€” BorderGuard adapte ses definitions selon les contraintes recues
4. **Correlation centralisee** â€” WorrySentinel seul correle et decide de l'impact sur l'etat
5. **Tracabilite complete** â€” Toute interaction est tracee
6. **Coherence garantie** â€” Matrice d'interaction explicite et non ambigue

### Phrase de synthese

> **WorrySentinel impose des contraintes de durcissement des frontieres selon les niveaux de securite et les etats de confiance, tandis que BorderGuard signale les anomalies et violations qui alimentent la correlation â€” chacun souverain dans son domaine, complementaires sans chevauchement, unis par des flux explicites et traces.**

---

## 14. Mini log de generation

### Ambiguite A1 : Confusion entre types de confiance

**Ambiguite rencontree :** Risque de confusion entre "etat de confiance systeme" (T0-T4, gouverne par WorrySentinel) et "niveau de confiance source" (trusted/verified/unknown/hostile, classifie par BorderGuard).

**Decision prise :** Section 3.3 ajoutee pour clarifier explicitement cette distinction. Les deux concepts sont documentes comme independants mais interconnectes.

**Correction effectuee :** Tableau de distinction ajoute, terminologie strictement separee dans tout le document.

### Ambiguite A2 : Direction des flux

**Ambiguite rencontree :** La relation etait decrite comme "contrainte" dans la documentation fondatrice, mais sans precision sur la bidirectionnalite.

**Decision prise :** Deux flux distincts documentes : flux descendant (gouvernance â†’ durcissement) et flux montant (signaux â†’ correlation).

**Correction effectuee :** Sections 4 et 5 structurees selon cette distinction, diagrammes de flux ajoutes.

### Verification de coherence

**Verification effectuee :**
- âœ… Compatible avec WorrySentinel - Documentation Fondatrice (section 9, relation avec BorderGuard)
- âœ… Compatible avec WorrySentinel - Architecture & Flows (section 9.3)
- âœ… Compatible avec Border Guard - Documentation Fondatrice (relation avec WorrySentinel)
- âœ… Compatible avec Border Guard - Security Levels Adaptation Contract
- âœ… Invariants WorrySentinel respectes (INV-WS-1 a INV-WS-8)
- âœ… Invariants BorderGuard respectes (INV-BG-1 a INV-BG-10)
- âœ… Separation gouvernance/definition preservee
- âœ… Aucune contradiction detectee

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**Reference :** WorrySentinel v1.2, BorderGuard v1.5  
**Type :** Contrat d'integration entre cores


