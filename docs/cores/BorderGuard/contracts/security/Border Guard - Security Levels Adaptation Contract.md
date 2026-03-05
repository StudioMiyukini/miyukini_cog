# Border Guard - Security Levels Adaptation Contract

## 1. Contexte

Ce document dÃ©finit comment **Border Guard adapte son comportement** selon les cinq niveaux de sÃ©curitÃ© Miyukini (0-4). Il spÃ©cifie formellement les rÃ¨gles d'adaptation des frontiÃ¨res, la rigueur de classification, les seuils de dÃ©tection, et les comportements en dÃ©gradation selon le profil de risque dÃ©clarÃ©.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**RÃ©fÃ©rence principale :** [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) (Section 7.3)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice et des Security Levels.

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toutes les dÃ©finitions de frontiÃ¨res et classifications de Border Guard
- **Responsable :** Border Guard (adaptation des frontiÃ¨res selon niveau sÃ©curitÃ©)
- **Consommateurs :** Tous les OpÃ©rateurs dÃ©clarant un niveau de sÃ©curitÃ©
- **Ne couvre pas :** L'implÃ©mentation technique des contrÃ´les (responsabilitÃ© des adaptateurs)

---

## 3. Principe fondamental

### 3.1 La sÃ©curitÃ© est un paramÃ¨tre de gouvernance

**Un OpÃ©rateur :**
- âœ… DÃ©clare son profil de risque (niveau 0-4)
- âŒ N'implÃ©mente jamais sa propre sÃ©curitÃ© de frontiÃ¨re
- âœ… Subit le niveau imposÃ© par l'Ã©cosystÃ¨me

**Border Guard adapte automatiquement :**
- La rigueur des frontiÃ¨res
- Les critÃ¨res de classification de confiance
- Les seuils de dÃ©tection de menaces
- Les rÃ¨gles de franchissement

### 3.2 IndÃ©pendance niveau de sÃ©curitÃ© / niveau de confiance

| Concept | DÃ©fini par | Nature |
|---------|------------|--------|
| **Niveau de sÃ©curitÃ© (0-4)** | OpÃ©rateur | Profil de risque dÃ©clarÃ© |
| **Niveau de confiance (T0-T4)** | Caring Nanny | Ã‰tat d'intÃ©gritÃ© du systÃ¨me |
| **Niveau de confiance source** | Border Guard | Classification (TRUSTED/VERIFIED/UNKNOWN/HOSTILE) |

**Ces trois concepts sont indÃ©pendants mais interconnectÃ©s.**

---

## 4. Les cinq niveaux de sÃ©curitÃ© et Border Guard

### 4.1 Niveau 0 â€” PUBLIC / DISPLAY

**Contexte :** Site vitrine, donnÃ©es publiques, WebApp sans Ã©tat critique.

**Philosophie :** "Si Ã§a casse, ce n'est pas grave."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **FrontiÃ¨res** | Assouplies |
| **Classification** | SimplifiÃ©e |
| **DÃ©tection hostile** | Seuil haut (tolÃ©rant) |
| **DÃ©gradation** | Douce |
| **TTL VERIFIED** | Long (heures) |
| **RÃ©Ã©valuation TRUSTED** | Rare |

**RÃ¨gles de frontiÃ¨re :**
- âœ… Franchissement libre sous conditions minimales
- âœ… Validation structurelle uniquement
- âŒ Pas de vÃ©rification stricte de contexte
- âœ… TraÃ§abilitÃ© minimale

**Classification :**
- CritÃ¨res `VERIFIED` : Assouplis
- Distribution `TRUSTED` : Largement distribuÃ©e
- Seuil `HOSTILE` : Patterns d'attaque Ã©vidents uniquement

### 4.2 Niveau 1 â€” STANDARD / CMS

**Contexte :** CMS, backoffice simple, contenu Ã©ditorial.

**Philosophie :** "On protÃ¨ge l'accÃ¨s, pas le systÃ¨me."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **FrontiÃ¨res** | Standard |
| **Classification** | Normale |
| **DÃ©tection hostile** | Seuil standard |
| **DÃ©gradation** | Normale |
| **TTL VERIFIED** | Standard (minutes) |
| **RÃ©Ã©valuation TRUSTED** | Mensuelle |

**RÃ¨gles de frontiÃ¨re :**
- âœ… Franchissement contrÃ´lÃ©
- âœ… Authentification simple requise pour zones protÃ©gÃ©es
- âœ… ContrÃ´le d'intÃ©gritÃ© pÃ©riodique
- âœ… TraÃ§abilitÃ© normale

**Classification :**
- CritÃ¨res `VERIFIED` : Standard
- Distribution `TRUSTED` : Normale
- Seuil `HOSTILE` : Patterns d'attaque classiques

### 4.3 Niveau 2 â€” SENSITIVE DATA

**Contexte :** DonnÃ©es personnelles, comptes utilisateurs, profils, historique.

**Philosophie :** "On protÃ¨ge les donnÃ©es."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **FrontiÃ¨res** | RenforcÃ©es |
| **Classification** | RenforcÃ©e |
| **DÃ©tection hostile** | Seuil bas (sensible) |
| **DÃ©gradation** | Rapide |
| **TTL VERIFIED** | Court (minutes) |
| **RÃ©Ã©valuation TRUSTED** | Hebdomadaire |

**RÃ¨gles de frontiÃ¨re :**
- âœ… Franchissement soumis Ã  vÃ©rification stricte
- âœ… Signatures d'intentions pour donnÃ©es sensibles
- âœ… ContrÃ´les de cohÃ©rence rÃ©guliers
- âœ… TraÃ§abilitÃ© complÃ¨te
- âœ… DÃ©tection d'anomalies comportementales

**Classification :**
- CritÃ¨res `VERIFIED` : RenforcÃ©s (contexte vÃ©rifiÃ©)
- Distribution `TRUSTED` : Restreinte
- Seuil `HOSTILE` : Patterns d'attaque + comportements anormaux

### 4.4 Niveau 3 â€” CRITICAL SYSTEM

**Contexte :** Auth, paiement, autorisations, dÃ©cisions structurantes, cores internes.

**Philosophie :** "On protÃ¨ge le systÃ¨me avant l'UX."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **FrontiÃ¨res** | Strictes |
| **Classification** | Stricte avec vÃ©rifications croisÃ©es |
| **DÃ©tection hostile** | Seuil trÃ¨s bas |
| **DÃ©gradation** | Blocage rapide |
| **TTL VERIFIED** | TrÃ¨s court |
| **RÃ©Ã©valuation TRUSTED** | Quotidienne |

**RÃ¨gles de frontiÃ¨re :**
- âœ… Zero-trust strict
- âœ… Signatures obligatoires pour tout franchissement sensible
- âœ… VÃ©rifications croisÃ©es systÃ©matiques
- âœ… Gel partiel possible en cas de doute
- âœ… DÃ©gradation rapide si anomalie

**Classification :**
- CritÃ¨res `VERIFIED` : Stricts (historique + contexte + authentification forte)
- Distribution `TRUSTED` : Minimale (cores uniquement)
- Seuil `HOSTILE` : Moindre anomalie significative

### 4.5 Niveau 4 â€” HARDENED / ISOLATED

**Contexte :** Environnement isolÃ©, hardware non fiable, contexte hostile, mode survie.

**Philosophie :** "On protÃ¨ge l'intÃ©gritÃ© coÃ»te que coÃ»te."

| Aspect | Comportement Border Guard |
|--------|---------------------------|
| **FrontiÃ¨res** | Maximales (isolement strict) |
| **Classification** | Ultra-stricte, zÃ©ro tolÃ©rance |
| **DÃ©tection hostile** | Minimal (aucune tolÃ©rance) |
| **DÃ©gradation** | Blocage progressif â†’ total |
| **TTL VERIFIED** | Minimal |
| **RÃ©Ã©valuation TRUSTED** | Constante |

**RÃ¨gles de frontiÃ¨re :**
- âœ… ContrÃ´les continus
- âœ… Attestations rÃ©guliÃ¨res requises
- âœ… TrÃ¨s peu de franchissements autorisÃ©s
- âœ… Blocage progressif puis total si anomalie
- âŒ Aucune tolÃ©rance aux anomalies

**Classification :**
- CritÃ¨res `VERIFIED` : Ultra-stricts (vÃ©rification continue)
- Distribution `TRUSTED` : Quasi nulle (isolement)
- Seuil `HOSTILE` : Toute anomalie = hostilitÃ© potentielle

---

## 5. Matrice d'adaptation des frontiÃ¨res

### 5.1 PermÃ©abilitÃ© par niveau

| Type de frontiÃ¨re | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 |
|-------------------|----------|----------|----------|----------|----------|
| **Externe** | Ouverte | ContrÃ´lÃ©e | ContrÃ´lÃ©e + vÃ©rif | Stricte | FermÃ©e |
| **Interne** | Ouverte | Standard | ContrÃ´lÃ©e | Stricte | Ultra-stricte |
| **IntÃ©gration** | Permissive | Standard | RenforcÃ©e | Stricte | Minimale/Aucune |

### 5.2 RÃ¨gles de franchissement par niveau

| RÃ¨gle | Niveau 0 | Niveau 1 | Niveau 2 | Niveau 3 | Niveau 4 |
|-------|----------|----------|----------|----------|----------|
| **Auth requise** | âŒ | âœ… Simple | âœ… RenforcÃ©e | âœ… Forte | âœ… Maximale |
| **Contexte validÃ©** | âŒ | âŒ | âœ… | âœ… Strict | âœ… Continu |
| **Signature** | âŒ | âŒ | âœ… Optionnelle | âœ… Obligatoire | âœ… Cryptographique |
| **VÃ©rification croisÃ©e** | âŒ | âŒ | âŒ | âœ… | âœ… Constante |
| **Attestation** | âŒ | âŒ | âŒ | âŒ | âœ… RÃ©guliÃ¨re |

---

## 6. Adaptation de la classification de confiance

### 6.1 CritÃ¨res VERIFIED par niveau

| CritÃ¨re | Niveau 0-1 | Niveau 2 | Niveau 3 | Niveau 4 |
|---------|------------|----------|----------|----------|
| Authentification | Simple | RenforcÃ©e | Forte + MFA | Maximale |
| Contexte cohÃ©rent | Non requis | Requis | Strict | Continu |
| Historique sans incident | Non requis | SouhaitÃ© | Requis | Critique |
| Device validÃ© | Non requis | RecommandÃ© | Requis | CertifiÃ© |

### 6.2 Distribution TRUSTED par niveau

| Niveau de sÃ©curitÃ© | Distribution TRUSTED |
|--------------------|---------------------|
| **0 - PUBLIC** | Large (commoditÃ©) |
| **1 - STANDARD** | Normale (Ã©quilibre) |
| **2 - SENSITIVE** | Restreinte (prÃ©caution) |
| **3 - CRITICAL** | Minimale (cores uniquement) |
| **4 - HARDENED** | Quasi nulle (isolement) |

### 6.3 Seuil de dÃ©tection HOSTILE par niveau

| Niveau de sÃ©curitÃ© | Seuil | Comportement |
|--------------------|-------|--------------|
| **0 - PUBLIC** | Haut | Patterns Ã©vidents uniquement |
| **1 - STANDARD** | Standard | Patterns classiques |
| **2 - SENSITIVE** | Bas | Patterns + comportements anormaux |
| **3 - CRITICAL** | TrÃ¨s bas | Moindre anomalie significative |
| **4 - HARDENED** | ZÃ©ro | Toute anomalie = hostilitÃ© potentielle |

---

## 7. DÃ©gradation graduÃ©e selon niveau de sÃ©curitÃ©

### 7.1 Ã‰tats de dÃ©gradation disponibles

| Ã‰tat | Description | Action Border Guard |
|------|-------------|---------------------|
| **Nominal** | Fonctionnement normal | FrontiÃ¨res normales |
| **Doute** | Suspicion lÃ©gÃ¨re | + VÃ©rifications |
| **Suspect** | Suspicion confirmÃ©e | FrontiÃ¨res resserrÃ©es |
| **Critique** | Anomalie grave | FrontiÃ¨res minimales |
| **Compromis** | Compromission dÃ©tectÃ©e | FrontiÃ¨res fermÃ©es |

### 7.2 Ã‰tats disponibles par niveau

| Niveau de sÃ©curitÃ© | Ã‰tats disponibles |
|--------------------|-------------------|
| **0-1** | Nominal â†’ Doute â†’ Suspect |
| **2** | Nominal â†’ Doute â†’ Suspect â†’ Critique â†’ Compromis |
| **3-4** | Tous les Ã©tats + blocage progressif/total |

### 7.3 Vitesse de dÃ©gradation

| Niveau de sÃ©curitÃ© | Vitesse de dÃ©gradation |
|--------------------|------------------------|
| **0 - PUBLIC** | Lente (tolÃ©rance haute) |
| **1 - STANDARD** | Normale |
| **2 - SENSITIVE** | Rapide |
| **3 - CRITICAL** | TrÃ¨s rapide |
| **4 - HARDENED** | ImmÃ©diate |

### 7.4 Vitesse de restauration

| Niveau de sÃ©curitÃ© | Restauration aprÃ¨s dÃ©gradation |
|--------------------|-------------------------------|
| **0 - PUBLIC** | Rapide (commoditÃ©) |
| **1 - STANDARD** | Normale |
| **2 - SENSITIVE** | Progressive (prudence) |
| **3 - CRITICAL** | Lente (validation requise) |
| **4 - HARDENED** | TrÃ¨s lente (validation formelle) |

---

## 8. Impact sur les intÃ©grations

### 8.1 Gouvernance des intÃ©grations par niveau

| Niveau de sÃ©curitÃ© | IntÃ©grations autorisÃ©es |
|--------------------|------------------------|
| **0 - PUBLIC** | Toutes (responsabilitÃ© OpÃ©rateur) |
| **1 - STANDARD** | Standard (vÃ©rification basique) |
| **2 - SENSITIVE** | CertifiÃ©es (contrat requis) |
| **3 - CRITICAL** | Minimales (revue approfondie) |
| **4 - HARDENED** | Aucune ou quasi-aucune (isolement) |

### 8.2 RÃ©vocation d'intÃ©gration par niveau

| Niveau de sÃ©curitÃ© | Seuil de rÃ©vocation |
|--------------------|---------------------|
| **0 - PUBLIC** | Violation grave uniquement |
| **1 - STANDARD** | Violations rÃ©pÃ©tÃ©es |
| **2 - SENSITIVE** | Violation confirmÃ©e |
| **3 - CRITICAL** | Suspicion de violation |
| **4 - HARDENED** | Moindre anomalie |

---

## 9. Combinaison niveau de sÃ©curitÃ© + niveau de confiance systÃ¨me

### 9.1 Matrice de comportement

La combinaison du niveau de sÃ©curitÃ© dÃ©clarÃ© (0-4) et du niveau de confiance systÃ¨me (T0-T4) dÃ©termine le comportement de Border Guard :

| Confiance systÃ¨me | Niveau 0-1 | Niveau 2 | Niveau 3-4 |
|-------------------|------------|----------|------------|
| **T0 (Normal)** | FrontiÃ¨res normales | FrontiÃ¨res renforcÃ©es | FrontiÃ¨res strictes |
| **T1 (Surveillance)** | + VÃ©rifications | + Signatures | + VÃ©rifications croisÃ©es |
| **T2 (DÃ©gradÃ©)** | Suspect | Critique | Critique + gel |
| **T3 (Minimum)** | Critique | Compromis | Blocage progressif |
| **T4 (Survie)** | Compromis | Blocage | Blocage total |

### 9.2 Exemples concrets

**OpÃ©rateur Niveau 2 en T0 :**
- FrontiÃ¨res renforcÃ©es
- Classification renforcÃ©e
- TraÃ§abilitÃ© complÃ¨te

**OpÃ©rateur Niveau 2 en T2 :**
- Ã‰tat Critique
- FrontiÃ¨res minimales
- Franchissements limitÃ©s

**OpÃ©rateur Niveau 4 en T1 :**
- FrontiÃ¨res strictes + vÃ©rifications croisÃ©es constantes
- Attestations requises
- DÃ©gradation rapide au moindre doute

---

## 10. Protocoles de sÃ©curitÃ© et adaptation

### 10.1 Protocoles temps rÃ©el

| Protocole | Adaptation par niveau |
|-----------|----------------------|
| **RT-SEC-1** (Session Ã©phÃ©mÃ¨re) | TTL session : long (N0-1) â†’ minimal (N4) |
| **RT-SEC-2** (Auth en couches) | Couches : rÃ©duites (N0-1) â†’ complÃ¨tes (N3-4) |
| **RT-SEC-4** (DÃ©tection anomalie) | Seuil : haut (N0-1) â†’ zÃ©ro (N4) |

### 10.2 Protocoles asynchrones

| Protocole | Adaptation par niveau |
|-----------|----------------------|
| **AS-SEC-2** (Signature locale) | Non requise (N0-1) â†’ cryptographique (N4) |
| **AS-SEC-3** (Revalidation) | Partielle (N0-1) â†’ complÃ¨te (N3-4) |
| **AS-SEC-5** (DÃ©gradation graduÃ©e) | Ã‰tapes : 3 (N0-1) â†’ 5+ (N3-4) |

---

## 11. Invariants de ce contrat

### INV-SLAC-1 : Adaptation automatique

Border Guard **adapte toujours** son comportement au niveau de sÃ©curitÃ© dÃ©clarÃ©. Aucune exception manuelle n'est autorisÃ©e.

### INV-SLAC-2 : Niveau par dÃ©faut

En l'absence de dÃ©claration explicite, le niveau de sÃ©curitÃ© est **1 (STANDARD)**.

### INV-SLAC-3 : Pas de contournement

Un OpÃ©rateur ne peut **jamais** demander un comportement de frontiÃ¨re plus permissif que celui de son niveau dÃ©clarÃ©.

### INV-SLAC-4 : DÃ©gradation monotone

La dÃ©gradation suit toujours un chemin **monotone** (jamais de saut Nominal â†’ Compromis sans passer par les Ã©tats intermÃ©diaires), sauf en cas de compromission flagrante en niveau 4.

### INV-SLAC-5 : TraÃ§abilitÃ© des adaptations

Toute adaptation de frontiÃ¨re selon le niveau de sÃ©curitÃ© est **traÃ§able** avec le niveau dÃ©clarÃ© et la raison.

---

## 12. RÃ©fÃ©rences croisÃ©es

### Invariants associÃ©s (Documentation Fondatrice - Section 7)

| Invariant | Ã‰noncÃ© | Relation |
|-----------|--------|----------|
| INV-BG-4 | Classification exhaustive | Adaptation des critÃ¨res selon niveau |
| INV-BG-5 | FrontiÃ¨res explicites | Adaptation de la permÃ©abilitÃ© selon niveau |
| INV-BG-6 | RÃ¨gles dÃ©claratives | Les rÃ¨gles d'adaptation sont dÃ©claratives |
| INV-BG-10 | NeutralitÃ© conceptuelle | L'adaptation est conceptuelle, pas technique |

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) | DÃ©finition des niveaux (Section 7.3) |
| [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Classification adaptÃ©e |
| [Border Guard - Crossing Rules Contract](../boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | RÃ¨gles adaptÃ©es |
| [Border Guard - Threat Model Contract](./Border%20Guard%20-%20Threat%20Model%20Contract.md) | Seuils de dÃ©tection adaptÃ©s |
| [Miyukini Conceptual References - Integrity Degradation System](..//..//..//..//miyukini-webway-system//reference//_index.md) | Combinaison niveau sÃ©curitÃ© / confiance |

### Documentation de securite centrale

| Document | Description |
|----------|-------------|
| [Security - Core Integration Map](..//..//..//WorrySentinel//_index.md) | Cartographie des roles securite des Cores, points de controle |
| [Doctrine Securite Fondamentale](..//..//..//..//miyukini-webway-system//reference//_index.md) | Fondation philosophique et architecturale de la securite |
| [Security - Invariants & Guarantees](..//..//..//WorrySentinel//_index.md) | Lois L1-L6, contraintes C1-C4, garanties par niveau |

### Role de BorderGuard dans le dispositif de securite

Selon le [Core Integration Map](..//..//..//WorrySentinel//_index.md), BorderGuard est le **Gardien des Frontieres** avec :
- Definition des frontieres : Delimite l'interne de l'externe (INV-BG-1)
- Classification de confiance : Attribue les niveaux trusted/verified/unknown/hostile (INV-BG-2)
- Regles de franchissement : Definit les conditions d'entree/sortie (INV-BG-3)
- Gouvernance des integrations : Controle les relations avec l'externe (INV-BG-4)

**Protocoles concernes :** RT-SEC-1, RT-SEC-2, RT-SEC-4, AS-SEC-2, NET-SEC-1

**Point de controle :** Couche SERVICES â†’ CORES (entree) et CORES â†’ SERVICES (sortie)

---

## 13. SynthÃ¨se contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Adaptation automatique** â€” Border Guard adapte ses frontiÃ¨res selon le niveau dÃ©clarÃ©
2. **CohÃ©rence totale** â€” Comportement prÃ©visible pour chaque niveau
3. **Pas de contournement** â€” Impossible de demander un comportement plus permissif
4. **DÃ©gradation proportionnelle** â€” Vitesse de dÃ©gradation adaptÃ©e au risque
5. **Combinaison explicite** â€” Interaction claire entre niveau sÃ©curitÃ© et confiance systÃ¨me
6. **TraÃ§abilitÃ©** â€” Toute adaptation est traÃ§able

### Phrase de synthÃ¨se

> **Border Guard adapte automatiquement la rigueur de ses frontiÃ¨res, ses critÃ¨res de classification, et ses seuils de dÃ©tection selon le niveau de sÃ©curitÃ© dÃ©clarÃ© (0-4), garantissant un comportement proportionnel au profil de risque sans jamais permettre de contournement.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** Border Guard v1.5, Security Levels v1.0 Section 7.3  
**Type :** Contrat d'adaptation de frontiÃ¨res selon niveau de sÃ©curitÃ©

