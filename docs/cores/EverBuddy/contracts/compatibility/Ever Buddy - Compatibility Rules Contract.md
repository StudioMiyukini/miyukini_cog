# Ever Buddy - Compatibility Rules Contract

## 1. Contexte

Ce document dÃ©finit les **rÃ¨gles de compatibilitÃ©** gouvernÃ©es par Ever Buddy dans l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie les niveaux de compatibilitÃ©, les critÃ¨res d'Ã©valuation, et les obligations associÃ©es Ã  chaque type de changement.

**Document fondateur :** [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : CompatibilitÃ©).

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Tous les Ã©lÃ©ments du systÃ¨me possÃ©dant un cycle de vie (contrats, structures, interfaces, Ã©lÃ©ments internes)
- **Responsable :** Ever Buddy (responsabilitÃ© exclusive de dÃ©finition des rÃ¨gles de compatibilitÃ© - Section 5)
- **Consommateurs :** Tous les cores, opÃ©rateurs, et produits de l'Ã©cosystÃ¨me Miyukini
- **Ne couvre pas :** L'exÃ©cution technique de la compatibilitÃ© (responsabilitÃ© des implÃ©mentations)

---

## 3. DÃ©finition canonique de la compatibilitÃ©

### 3.1 Qu'est-ce que la compatibilitÃ© ?

La **compatibilitÃ©** est la capacitÃ© d'un Ã©lÃ©ment Ã  fonctionner avec des Ã©lÃ©ments d'autres versions. Elle caractÃ©rise la relation entre diffÃ©rentes versions d'un mÃªme Ã©lÃ©ment ou entre Ã©lÃ©ments interdÃ©pendants.

**RÃ©fÃ©rence glossaire :** La compatibilitÃ© n'est pas une garantie technique â€” c'est une **rÃ¨gle de gouvernance** dÃ©finissant les attentes et obligations lors des Ã©volutions.

### 3.2 ResponsabilitÃ© d'Ever Buddy

Ever Buddy est **exclusivement responsable** de la dÃ©finition des rÃ¨gles de compatibilitÃ© entre versions. Cette responsabilitÃ© inclut :

- DÃ©finir ce qui constitue un changement rÃ©trocompatible
- DÃ©finir ce qui constitue une rupture de compatibilitÃ©
- DÃ©finir les pÃ©riodes de transition minimales pour chaque type de changement
- DÃ©finir les exceptions autorisÃ©es (et leurs conditions strictes)

**Invariant associÃ© :** INV-EB-5 â€” Toute Ã©volution est **prÃ©sumÃ©e rÃ©trocompatible** sauf dÃ©claration explicite contraire.

---

## 4. Niveaux de compatibilitÃ©

Ever Buddy distingue trois niveaux de compatibilitÃ©, chacun avec des implications et des obligations spÃ©cifiques.

### 4.1 RÃ©trocompatible (Backward Compatible)

**DÃ©finition :** Le nouveau fonctionne avec l'ancien. Les consommateurs existants continuent de fonctionner sans modification.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Direction** | Nouveau â†’ Ancien |
| **Impact consommateur** | Aucune modification requise |
| **Migration** | Optionnelle |
| **Coexistence** | Naturelle et illimitÃ©e |
| **Version** | Changement mineur ou correctif |

**CritÃ¨res de rÃ©trocompatibilitÃ© :**

1. **Interface prÃ©servÃ©e** â€” Toutes les interfaces existantes restent fonctionnelles
2. **Comportement prÃ©servÃ©** â€” Les comportements existants produisent les mÃªmes rÃ©sultats
3. **Contrats prÃ©servÃ©s** â€” Les contrats Ã©tablis restent valides
4. **DonnÃ©es prÃ©servÃ©es** â€” Les structures de donnÃ©es existantes restent exploitables

**Exemples de changements rÃ©trocompatibles :**

- Ajout d'un champ optionnel Ã  une structure
- Ajout d'une nouvelle mÃ©thode Ã  une interface
- Correction d'un bug sans changement de comportement documentÃ©
- AmÃ©lioration de performance sans changement d'interface
- Ajout d'une nouvelle fonctionnalitÃ© indÃ©pendante

### 4.2 Compatible en amont (Forward Compatible)

**DÃ©finition :** L'ancien fonctionne avec le nouveau. Les anciennes versions peuvent consommer les nouvelles fonctionnalitÃ©s (rare, souvent impossible).

| Aspect | SpÃ©cification |
|--------|---------------|
| **Direction** | Ancien â†’ Nouveau |
| **Impact consommateur** | Aucune modification requise pour l'ancien |
| **Migration** | Non nÃ©cessaire |
| **Coexistence** | Requiert une conception explicite |
| **Version** | Cas particulier, rarement applicable |

**CritÃ¨res de compatibilitÃ© en amont :**

1. **ExtensibilitÃ© conÃ§ue** â€” L'Ã©lÃ©ment ancien a Ã©tÃ© conÃ§u pour ignorer les extensions inconnues
2. **DÃ©gradation gracieuse** â€” L'absence de nouvelles fonctionnalitÃ©s n'empÃªche pas le fonctionnement
3. **Protocole ouvert** â€” Le protocole de communication permet l'ajout de nouveaux Ã©lÃ©ments

**Exemples de compatibilitÃ© en amont :**

- Format de donnÃ©es avec champs ignorÃ©s si inconnus (JSON extensible)
- Protocole de communication avec version nÃ©gociÃ©e
- Interface avec mÃ©thodes optionnelles

**Avertissement :** La compatibilitÃ© en amont est **exceptionnelle**. Elle requiert une conception anticipÃ©e et ne peut Ãªtre garantie rÃ©troactivement.

### 4.3 Incompatible (Breaking)

**DÃ©finition :** Le nouveau ne fonctionne pas avec l'ancien. Une migration est obligatoire.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Direction** | Aucune coexistence naturelle |
| **Impact consommateur** | Modification obligatoire |
| **Migration** | Obligatoire avec chemin documentÃ© |
| **Coexistence** | Temporaire, pÃ©riode de transition |
| **Version** | Changement majeur obligatoire |

**CritÃ¨res d'incompatibilitÃ© (un seul suffit) :**

1. **Interface rompue** â€” Une interface existante est modifiÃ©e ou supprimÃ©e
2. **Comportement modifiÃ©** â€” Un comportement existant produit des rÃ©sultats diffÃ©rents
3. **Contrat violÃ©** â€” Un contrat Ã©tabli n'est plus respectÃ©
4. **DonnÃ©es incompatibles** â€” Les structures de donnÃ©es existantes ne sont plus exploitables

**Exemples de changements incompatibles :**

- Suppression d'un champ obligatoire
- Modification de la sÃ©mantique d'une mÃ©thode
- Changement de type d'un paramÃ¨tre
- Renommage d'une interface publique
- Modification du format de sÃ©rialisation

---

## 5. Obligations selon le niveau de compatibilitÃ©

### 5.1 Obligations pour les changements rÃ©trocompatibles

| Obligation | Requis | Description |
|------------|--------|-------------|
| Annonce prÃ©alable | âŒ Non | Peut Ãªtre publiÃ© sans annonce formelle |
| PÃ©riode de transition | âŒ Non | Pas de pÃ©riode de transition requise |
| Documentation | âœ… Oui | Changement documentÃ© dans les notes de version |
| Chemin de migration | âŒ Non | Pas de migration nÃ©cessaire |
| Test de non-rÃ©gression | âœ… Oui | VÃ©rification que l'existant fonctionne |

### 5.2 Obligations pour les changements incompatibles

| Obligation | Requis | Description |
|------------|--------|-------------|
| Annonce prÃ©alable | âœ… Oui | Communication formelle avant mise en Å“uvre |
| PÃ©riode de transition | âœ… Oui | PÃ©riode de dÃ©prÃ©ciation obligatoire (INV-EB-4) |
| Documentation | âœ… Oui | Documentation complÃ¨te des diffÃ©rences |
| Chemin de migration | âœ… Oui | Guide de migration fourni |
| Justification | âœ… Oui | Raison documentÃ©e de la rupture |
| Impact Ã©valuÃ© | âœ… Oui | Analyse d'impact sur les consommateurs |

**RÃ¨gle absolue (INV-EB-4) :** Aucun Ã©lÃ©ment ACTIVE ne peut passer directement Ã  RETIRED ou ARCHIVED. La transition par DEPRECATED est **obligatoire**. Cela s'applique Ã  tous les changements incompatibles.

---

## 6. FenÃªtre de compatibilitÃ© (Compatibility Window)

### 6.1 DÃ©finition

La **fenÃªtre de compatibilitÃ©** est la plage de versions avec lesquelles un Ã©lÃ©ment garantit la compatibilitÃ©.

**Format :** `[version_min, version_max]` ou `[version_min, *)` pour les versions ouvertes

**Exemples :**

- `[v2.0, v2.4]` â€” Compatible avec les versions 2.0 Ã  2.4 incluses
- `[v3.0, *)` â€” Compatible avec toutes les versions Ã  partir de 3.0

### 6.2 RÃ¨gles de fenÃªtre

| RÃ¨gle | Description |
|-------|-------------|
| **RÃˆGLE-COMPAT-1** | Toute fenÃªtre de compatibilitÃ© est **explicite et documentÃ©e** |
| **RÃˆGLE-COMPAT-2** | La fermeture d'une fenÃªtre requiert une **pÃ©riode de transition** |
| **RÃˆGLE-COMPAT-3** | L'extension d'une fenÃªtre est **toujours autorisÃ©e** sans formalitÃ© |
| **RÃˆGLE-COMPAT-4** | La rÃ©duction d'une fenÃªtre est un **changement incompatible** |

### 6.3 Gestion des fenÃªtres par catÃ©gorie

| CatÃ©gorie | FenÃªtre minimale recommandÃ©e | Fermeture |
|-----------|------------------------------|-----------|
| Contrats fondateurs (FONDATION) | 3 gÃ©nÃ©rations majeures | Quasi interdite |
| Contrats opÃ©rationnels | 2 gÃ©nÃ©rations majeures | Avec justification |
| Interfaces techniques | 1 gÃ©nÃ©ration majeure | Avec documentation |
| Ã‰lÃ©ments internes | Aucune garantie | Libre |

---

## 7. Ã‰valuation de la compatibilitÃ©

### 7.1 Processus d'Ã©valuation

Toute Ã©volution doit Ãªtre Ã©valuÃ©e pour dÃ©terminer son niveau de compatibilitÃ© :

```
1. Identification des changements
   â†“
2. Analyse d'impact sur les interfaces
   â†“
3. Analyse d'impact sur les comportements
   â†“
4. Analyse d'impact sur les contrats
   â†“
5. Analyse d'impact sur les donnÃ©es
   â†“
6. Classification du niveau de compatibilitÃ©
   â†“
7. DÃ©termination des obligations associÃ©es
```

### 7.2 Questions d'Ã©valuation

Pour chaque changement proposÃ©, rÃ©pondre aux questions suivantes :

**Interface :**
- [ ] Les interfaces existantes sont-elles prÃ©servÃ©es ?
- [ ] Les signatures de mÃ©thodes sont-elles inchangÃ©es ?
- [ ] Les points d'entrÃ©e existants restent-ils fonctionnels ?

**Comportement :**
- [ ] Les comportements documentÃ©s produisent-ils les mÃªmes rÃ©sultats ?
- [ ] Les effets de bord sont-ils identiques ?
- [ ] Les erreurs sont-elles levÃ©es dans les mÃªmes conditions ?

**Contrat :**
- [ ] Les invariants existants sont-ils toujours respectÃ©s ?
- [ ] Les garanties documentÃ©es sont-elles maintenues ?
- [ ] Les prÃ©/post-conditions sont-elles inchangÃ©es ?

**DonnÃ©es :**
- [ ] Les structures existantes sont-elles toujours valides ?
- [ ] Les formats de sÃ©rialisation sont-ils compatibles ?
- [ ] Les migrations de donnÃ©es sont-elles Ã©vitÃ©es ?

**RÃ©sultat :**
- Si **toutes les rÃ©ponses sont "Oui"** â†’ Changement **rÃ©trocompatible**
- Si **une seule rÃ©ponse est "Non"** â†’ Changement **incompatible**

---

## 8. RÃ¨gles de rupture de compatibilitÃ©

### 8.1 Conditions de rupture autorisÃ©e

Une rupture de compatibilitÃ© est autorisÃ©e **uniquement** si :

1. **Justification documentÃ©e** â€” La rupture est nÃ©cessaire et les alternatives ont Ã©tÃ© Ã©valuÃ©es
2. **Impact Ã©valuÃ©** â€” L'impact sur les consommateurs est documentÃ©
3. **PÃ©riode de transition** â€” Une pÃ©riode de dÃ©prÃ©ciation est planifiÃ©e
4. **Chemin de migration** â€” Un guide de migration est fourni
5. **Communication prÃ©alable** â€” L'annonce est faite en avance (minimum 1 cycle de release)

### 8.2 Ruptures exceptionnelles

Certaines ruptures peuvent Ãªtre accÃ©lÃ©rÃ©es en cas de :

| Cas | PÃ©riode minimale | Condition |
|-----|------------------|-----------|
| Faille de sÃ©curitÃ© critique | ImmÃ©diate | Documentation post-facto |
| Violation lÃ©gale | ImmÃ©diate | Obligation rÃ©glementaire documentÃ©e |
| Corruption de donnÃ©es | 1 cycle de release | Risque de perte de donnÃ©es |

**Avertissement :** Ces exceptions sont **strictement encadrÃ©es** et requiÃ¨rent une justification formelle. Elles ne peuvent pas Ãªtre utilisÃ©es pour contourner la discipline normale.

### 8.3 Ruptures interdites

Les ruptures suivantes sont **structurellement interdites** :

| Rupture interdite | Raison |
|-------------------|--------|
| Rupture rÃ©troactive | INV-EB-11 â€” Les rÃ¨gles ne peuvent pas modifier le passÃ© |
| Rupture sans transition | INV-EB-4 â€” DEPRECATED est obligatoire |
| Rupture sans documentation | INV-EB-7 â€” Documentation obligatoire |
| Rupture discriminatoire | INV-EB-8 â€” RÃ¨gles universelles |

---

## 9. Interactions avec le versionnement

### 9.1 Correspondance compatibilitÃ©-version

| Type de changement | Version | CompatibilitÃ© |
|--------------------|---------|---------------|
| Correction de bug | Correctif (+0.0.1) | RÃ©trocompatible |
| Ajout de fonctionnalitÃ© | Mineur (+0.1.0) | RÃ©trocompatible |
| Rupture de compatibilitÃ© | Majeur (+1.0.0) | Incompatible |

**RÃ¨gle absolue :** Un changement incompatible **doit** Ãªtre accompagnÃ© d'un changement de version majeure. Un changement mineur ou correctif **ne peut jamais** Ãªtre incompatible.

### 9.2 Relation avec le contrat de sÃ©mantique de version

Les rÃ¨gles de compatibilitÃ© sont complÃ©mentaires au contrat de sÃ©mantique de version :

- **CompatibilitÃ©** dÃ©finit la relation entre versions
- **SÃ©mantique de version** dÃ©finit comment les versions sont numÃ©rotÃ©es

**RÃ©fÃ©rence :** [Ever Buddy - Version Semantics Contract](./Ever%20Buddy%20-%20Version%20Semantics%20Contract.md)

---

## 10. MÃ©triques de compatibilitÃ©

Ever Buddy surveille les mÃ©triques suivantes relatives Ã  la compatibilitÃ© :

### 10.1 MÃ©triques d'Ã©volution

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| Taux de ruptures | Ratio ruptures / Ã©volutions totales | > 20% sur 1 gÃ©nÃ©ration |
| DurÃ©e moyenne de transition | Temps entre DEPRECATED et RETIRED | < minimum dÃ©fini |
| FenÃªtre moyenne | Largeur moyenne des fenÃªtres de compatibilitÃ© | RÃ©duction tendancielle |

### 10.2 MÃ©triques d'adoption

| MÃ©trique | Description | Seuil d'alerte |
|----------|-------------|----------------|
| Taux d'adoption du successeur | % de consommateurs ayant migrÃ© | < 80% Ã  mi-transition |
| Consommateurs non migrÃ©s | Nombre de consommateurs restant sur l'ancien | > 0 Ã  fin de transition |
| Temps de migration moyen | DurÃ©e moyenne de migration par consommateur | > pÃ©riode de transition |

---

## 11. RÃ©fÃ©rences croisÃ©es

### Invariants associÃ©s (Documentation Fondatrice - Section 7)

| Invariant | Ã‰noncÃ© | Relation |
|-----------|--------|----------|
| INV-EB-4 | PÃ©riode de dÃ©prÃ©ciation obligatoire | AppliquÃ© Ã  toute rupture |
| INV-EB-5 | RÃ©trocompatibilitÃ© par dÃ©faut | PrÃ©somption de base |
| INV-EB-7 | Documentation obligatoire | Toute rupture documentÃ©e |
| INV-EB-8 | IndÃ©pendance des dÃ©cisions | RÃ¨gles universelles |
| INV-EB-9 | PrÃ©dictibilitÃ© des transitions | RÃ¨gles publiques et stables |
| INV-EB-11 | Non-rÃ©troactivitÃ© | Pas de rupture rÃ©troactive |

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [Ever Buddy - Documentation Fondatrice](../../foundation/Ever%20Buddy%20-%20Documentation%20Fondatrice.md) | Document source |
| [Ever Buddy - Version Semantics Contract](./Ever%20Buddy%20-%20Version%20Semantics%20Contract.md) | NumÃ©rotation des versions |
| [Ever Buddy - Lifecycle States Contract](../lifecycle/Ever%20Buddy%20-%20Lifecycle%20States%20Contract.md) | Ã‰tats de cycle de vie |
| [Ever Buddy - Transition Rules Contract](../lifecycle/Ever%20Buddy%20-%20Transition%20Rules%20Contract.md) | RÃ¨gles de transition |

### RÃ©fÃ©rences glossaire

| Terme | DÃ©finition |
|-------|------------|
| **RÃ©trocompatible** | Le nouveau fonctionne avec l'ancien |
| **Compatible en amont** | L'ancien fonctionne avec le nouveau |
| **Incompatible** | Le nouveau ne fonctionne pas avec l'ancien |
| **FenÃªtre de compatibilitÃ©** | Plage de versions garantissant la compatibilitÃ© |
| **Breaking change** | Changement qui rompt la compatibilitÃ© |

**Source :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 12. SynthÃ¨se contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **La compatibilitÃ© est dÃ©finie** â€” Trois niveaux clairs avec critÃ¨res explicites
2. **Les obligations sont connues** â€” Chaque niveau de compatibilitÃ© a des obligations documentÃ©es
3. **Les ruptures sont encadrÃ©es** â€” Conditions strictes pour les changements incompatibles
4. **La rÃ©trocompatibilitÃ© est la norme** â€” PrÃ©somption par dÃ©faut (INV-EB-5)
5. **Les transitions sont protÃ©gÃ©es** â€” PÃ©riode de dÃ©prÃ©ciation obligatoire (INV-EB-4)

### Phrase de synthÃ¨se

> **La compatibilitÃ© est la promesse faite aux consommateurs : ce qui fonctionne aujourd'hui fonctionnera demain, sauf annonce explicite, pÃ©riode de transition, et chemin de migration.**

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** Ever Buddy v1.0, Documentation Fondatrice Section 4  
**Type :** Contrat de compatibilitÃ©

