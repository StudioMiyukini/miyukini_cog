# Border Guard - Threat Model Contract

## 1. Contexte

Ce document dÃ©finit le **modÃ¨le de menaces** gouvernÃ© par Border Guard dans l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie formellement les catÃ©gories de menaces, les vecteurs d'attaque, les rÃ©ponses conceptuelles, et les rÃ¨gles de dÃ©tection que Border Guard applique pour protÃ©ger les frontiÃ¨res du systÃ¨me.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**RÃ©fÃ©rences principales :**
- [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - External Signal Trust Reinforcement Contract](..//..//..//..//miyukini-webway-system//reference//_index.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice et des protocoles de sÃ©curitÃ©.

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toutes les frontiÃ¨res dÃ©finies par Border Guard
- **Responsable :** Border Guard (dÃ©finition des menaces et rÃ©ponses conceptuelles)
- **Consommateurs :** BondingBrother (application), StrongFather (dÃ©cision), Caring Nanny (observation)
- **Ne couvre pas :** L'implÃ©mentation technique des mÃ©canismes de protection

---

## 3. Philosophie de sÃ©curitÃ©

### 3.1 Principe fondamental

**"La sÃ©curitÃ© n'est pas un mur. C'est un systÃ¨me nerveux. Il ressent, Ã©value, s'adapte, se dÃ©grade, se protÃ¨ge."**

### 3.2 Posture de Border Guard

Border Guard adopte une posture de **dÃ©fense en profondeur conceptuelle** :

1. **DÃ©finition** â€” Border Guard dÃ©finit ce qui est une menace
2. **Classification** â€” Border Guard classifie les sources selon leur niveau de confiance
3. **Conseil** â€” Border Guard informe les autres cores sur les menaces
4. **Jamais d'exÃ©cution** â€” Border Guard ne bloque pas lui-mÃªme

### 3.3 Internet comme source de signaux

**Principe fondamental :**

> **"Internet n'a jamais raison. Il peut seulement confirmer ou infirmer ce que le systÃ¨me croit dÃ©jÃ ."**

- Internet est un **capteur, pas un cerveau**
- Les signaux externes sont des **informations, pas des autoritÃ©s**
- L'Ã©tat local **prime toujours** sur les signaux externes

---

## 4. CatÃ©gories de menaces

### 4.1 Menaces aux frontiÃ¨res externes

Les menaces ciblant la frontiÃ¨re entre l'Ã©cosystÃ¨me Miyukini et le monde extÃ©rieur.

#### THREAT-EXT-1 : Injection de donnÃ©es malveillantes

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative d'injecter des donnÃ©es non valides ou malveillantes via les frontiÃ¨res externes |
| **Vecteurs** | API publiques, webhooks, formulaires, imports |
| **Indicateurs** | DonnÃ©es hors format, patterns d'injection connus, sÃ©quences suspectes |
| **Impact potentiel** | Corruption de donnÃ©es, compromission systÃ¨me |
| **RÃ©ponse Border Guard** | Classification UNKNOWN â†’ HOSTILE si pattern confirmÃ© |

#### THREAT-EXT-2 : Usurpation d'identitÃ©

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative de se faire passer pour une source lÃ©gitime |
| **Vecteurs** | Tokens volÃ©s, sessions hijackÃ©es, replay d'authentification |
| **Indicateurs** | Contexte incohÃ©rent, device inconnu, localisation suspecte |
| **Impact potentiel** | AccÃ¨s non autorisÃ©, actions frauduleuses |
| **RÃ©ponse Border Guard** | DÃ©gradation confiance VERIFIED â†’ UNKNOWN, notification StrongFather |

#### THREAT-EXT-3 : Attaque par dÃ©ni de service

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative de saturer les frontiÃ¨res pour bloquer le systÃ¨me |
| **Vecteurs** | RequÃªtes massives, patterns rÃ©pÃ©titifs, amplification |
| **Indicateurs** | Volume anormal, rythme anormal, sources multiples coordonnÃ©es |
| **Impact potentiel** | IndisponibilitÃ© du systÃ¨me |
| **RÃ©ponse Border Guard** | Classification HOSTILE, isolation de la frontiÃ¨re ciblÃ©e |

#### THREAT-EXT-4 : Exfiltration de donnÃ©es

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative d'extraire des donnÃ©es au-delÃ  des frontiÃ¨res autorisÃ©es |
| **Vecteurs** | RequÃªtes massives, scraping, tunneling |
| **Indicateurs** | Volume de sortie anormal, patterns d'extraction |
| **Impact potentiel** | Fuite de donnÃ©es sensibles |
| **RÃ©ponse Border Guard** | Resserrement des rÃ¨gles de sortie, notification Caring Nanny |

### 4.2 Menaces aux frontiÃ¨res internes

Les menaces ciblant les frontiÃ¨res entre zones de confiance internes.

#### THREAT-INT-1 : Escalade de privilÃ¨ges

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative d'accÃ©der Ã  une zone de confiance supÃ©rieure sans autorisation |
| **Vecteurs** | Exploitation de failles, contournement de rÃ¨gles |
| **Indicateurs** | Tentatives de franchissement non autorisÃ©es, patterns d'exploration |
| **Impact potentiel** | AccÃ¨s Ã  des zones sensibles |
| **RÃ©ponse Border Guard** | Renforcement de la frontiÃ¨re, classification HOSTILE si rÃ©pÃ©tÃ© |

#### THREAT-INT-2 : Mouvement latÃ©ral

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Tentative de se propager entre zones internes |
| **Vecteurs** | Exploitation de relations de confiance, rebond |
| **Indicateurs** | AccÃ¨s inhabituels entre zones, patterns de propagation |
| **Impact potentiel** | Compromission Ã©tendue |
| **RÃ©ponse Border Guard** | Isolation des zones, resserrement des franchissements |

#### THREAT-INT-3 : Corruption de donnÃ©es internes

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Modification non autorisÃ©e de donnÃ©es dans une zone de confiance |
| **Vecteurs** | Injection interne, race conditions, manipulation d'Ã©tat |
| **Indicateurs** | IncohÃ©rences de donnÃ©es, signatures invalides |
| **Impact potentiel** | Perte d'intÃ©gritÃ© |
| **RÃ©ponse Border Guard** | Notification Caring Nanny, gel de la zone concernÃ©e |

### 4.3 Menaces aux frontiÃ¨res d'intÃ©gration

Les menaces ciblant les relations avec les systÃ¨mes externes.

#### THREAT-INTEG-1 : Compromission d'intÃ©gration

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Un systÃ¨me externe intÃ©grÃ© devient malveillant ou compromis |
| **Vecteurs** | IntÃ©gration lÃ©gitime devenue hostile, piratage du partenaire |
| **Indicateurs** | Comportement anormal de l'intÃ©gration, signaux externes de compromission |
| **Impact potentiel** | Canal de confiance devenu canal d'attaque |
| **RÃ©ponse Border Guard** | Suspension de l'intÃ©gration, classification HOSTILE |

#### THREAT-INTEG-2 : Abus d'intÃ©gration

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Un systÃ¨me intÃ©grÃ© dÃ©passe les limites de son contrat |
| **Vecteurs** | RequÃªtes hors scope, accÃ¨s non autorisÃ©s, volume excessif |
| **Indicateurs** | Ã‰carts par rapport au contrat d'intÃ©gration |
| **Impact potentiel** | Surcharge, accÃ¨s non autorisÃ© |
| **RÃ©ponse Border Guard** | DÃ©gradation confiance, restriction des accÃ¨s |

#### THREAT-INTEG-3 : Injection via intÃ©gration

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Utilisation d'une intÃ©gration lÃ©gitime pour injecter des donnÃ©es malveillantes |
| **Vecteurs** | Webhooks manipulÃ©s, rÃ©ponses API altÃ©rÃ©es |
| **Indicateurs** | DonnÃ©es incohÃ©rentes, signatures invalides |
| **Impact potentiel** | Corruption via canal de confiance |
| **RÃ©ponse Border Guard** | Validation renforcÃ©e, suspension si rÃ©pÃ©tÃ© |

### 4.4 Menaces rÃ©seau et signaux externes

Les menaces liÃ©es aux communications rÃ©seau et aux signaux Internet.

#### THREAT-NET-1 : Signal externe malveillant

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Signal Internet tentant d'imposer un Ã©tat ou une action |
| **Vecteurs** | Update signals falsifiÃ©s, compliance signals manipulÃ©s |
| **Indicateurs** | Signal non vÃ©rifiable, contradiction avec Ã©tat local |
| **Impact potentiel** | Manipulation de l'Ã©tat local |
| **RÃ©ponse Border Guard** | Rejet du signal, marquage AMBIGU |

#### THREAT-NET-2 : RÃ©seau compromis

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Le rÃ©seau lui-mÃªme est devenu hostile (MITM, DNS poisoning) |
| **Vecteurs** | Interception, modification en transit, redirection |
| **Indicateurs** | Certificats invalides, rÃ©ponses incohÃ©rentes |
| **Impact potentiel** | Toute communication compromise |
| **RÃ©ponse Border Guard** | Isolation automatique, fonctionnement local |

#### THREAT-NET-3 : DÃ©pendance externe exploitÃ©e

| Aspect | DÃ©finition |
|--------|------------|
| **Description** | Exploitation d'une dÃ©pendance Ã  un service externe |
| **Vecteurs** | Service tiers compromis, indisponibilitÃ© forcÃ©e |
| **Indicateurs** | Comportement anormal du service externe |
| **Impact potentiel** | Perte d'autonomie |
| **RÃ©ponse Border Guard** | Activation du mode dÃ©gradÃ©, aucune dÃ©pendance critique |

---

## 5. Vecteurs d'attaque et rÃ©ponses

### 5.1 Matrice vecteur / rÃ©ponse

| Vecteur | Classification rÃ©sultante | Action Border Guard | Core notifiÃ© |
|---------|---------------------------|---------------------|--------------|
| Pattern d'injection | UNKNOWN â†’ HOSTILE | Blocage dÃ©finition | StrongFather |
| Contexte incohÃ©rent | VERIFIED â†’ UNKNOWN | RÃ©Ã©valuation | StrongFather |
| Volume anormal | UNKNOWN / HOSTILE | Isolation frontiÃ¨re | Caring Nanny |
| Signature invalide | HOSTILE | Blocage dÃ©finition | StrongFather, TAMR |
| Signal externe contradictoire | AMBIGU | Marquage, pas d'action | Caring Nanny |
| Certificat invalide | HOSTILE (rÃ©seau) | Isolation rÃ©seau | Caring Nanny |

### 5.2 RÃ©ponses graduÃ©es

| Niveau de menace | RÃ©ponse Border Guard |
|------------------|---------------------|
| **Suspicion** | Surveillance accrue, pas de changement de classification |
| **Anomalie confirmÃ©e** | DÃ©gradation de confiance (ex: VERIFIED â†’ UNKNOWN) |
| **Pattern hostile** | Classification HOSTILE, notification cores |
| **Compromission** | Isolation frontiÃ¨re, gel zone, notification TAMR |

---

## 6. External Confidence Signals (ECS)

### 6.1 Traitement des signaux externes

Border Guard traite les signaux Internet selon le contrat External Signal Trust :

| Type de signal | Traitement Border Guard |
|----------------|------------------------|
| **Update signal** | Validation format, passage Ã  Ever Buddy |
| **Compliance signal** | Comparaison Ã©tat local, passage Ã  Caring Nanny |
| **Alert signal** | Ã‰valuation gravitÃ©, notification StrongFather |
| **Metadata signal** | Validation structure, stockage si conforme |

### 6.2 RÃ¨gles de traitement ECS

1. **Isolation** â€” Tout signal est isolÃ© avant traitement
2. **Validation format** â€” Rejet si format invalide
3. **Comparaison locale** â€” Comparaison avec l'Ã©tat connu
4. **Classification** â€” Attribution d'un niveau de confiance au signal
5. **Transmission** â€” Passage aux cores concernÃ©s avec classification

### 6.3 Matrice signal / Ã©tat local

| Signal externe | Ã‰tat local | Effet Border Guard |
|----------------|------------|-------------------|
| Conforme | Sain | Aucun changement |
| Conforme | DÃ©gradÃ© | Peut aider Ã  remonter (via Caring Nanny) |
| Non conforme | Sain | Suspicion lÃ©gÃ¨re, surveillance |
| Non conforme | DÃ©gradÃ© | Renforce dÃ©gradation |
| Contradictoire | Tout Ã©tat | MarquÃ© AMBIGU, pas d'action automatique |

---

## 7. Bootstrap sÃ©curisÃ© (premier contact rÃ©seau)

### 7.1 RÃ¨gles absolues

Le premier contact avec Internet aprÃ¨s dÃ©marrage suit des rÃ¨gles strictes :

| RÃ¨gle | Exigence |
|-------|----------|
| âŒ Aucune clÃ© privÃ©e transmise | Les secrets restent locaux |
| âŒ Aucun Ã©tat interne exposÃ© | Pas de fuite d'information |
| âŒ Aucun module activÃ© | Pas d'activation Ã  distance |
| âŒ Aucune dÃ©cision modifiÃ©e | Le rÃ©seau n'impose rien |

### 7.2 Informations autorisÃ©es

| Information | AutorisÃ©e | Raison |
|-------------|-----------|--------|
| Hash public du systÃ¨me | âœ… | VÃ©rification d'intÃ©gritÃ© |
| Version dÃ©clarative | âœ… | CompatibilitÃ© |
| CapacitÃ©s exposÃ©es | âœ… | Via Master Butler |
| Ã‰tat de confiance (T0-T4) | âœ… | AnonymisÃ© |
| ClÃ©s privÃ©es | âŒ | Secret absolu |
| Ã‰tat interne dÃ©taillÃ© | âŒ | Information sensible |
| Logs complets | âŒ | Information sensible |

---

## 8. Handshake de conformitÃ©

### 8.1 Processus de reconnexion

Lors du retour Internet aprÃ¨s une pÃ©riode hors ligne :

```
1. Border Guard : isolation du canal rÃ©seau
2. Ã‰change de conformitÃ© :
   - Version du noyau
   - Version des cores
   - IntÃ©gritÃ© locale
   - Ã‰tat de confiance
3. Border Guard : validation de conformitÃ©
4. Si conforme : levÃ©e progressive de l'isolation
5. Si non conforme : maintien de l'isolation
```

### 8.2 RÃ©sultats possibles

| RÃ©sultat | Action Border Guard |
|----------|---------------------|
| **Conforme** | FrontiÃ¨res normales selon niveau sÃ©curitÃ© |
| **Partiellement conforme** | FrontiÃ¨res restreintes, surveillance |
| **Non conforme** | Isolation maintenue, notification TAMR |
| **Signal suspect** | Isolation renforcÃ©e, enquÃªte |

---

## 9. Comportements en dÃ©gradation rÃ©seau

### 9.1 Situations et rÃ©ponses

| Situation | Comportement Border Guard |
|-----------|---------------------------|
| **Pas d'Internet** | Fonctionnement normal, frontiÃ¨res locales actives |
| **RÃ©seau instable** | Aucune panique, file d'attente des signaux |
| **RÃ©seau compromis** | Isolation automatique, frontiÃ¨res fermÃ©es vers rÃ©seau |
| **Signaux incohÃ©rents** | MarquÃ©s AMBIGU, pas d'action automatique |
| **Tentative d'injection** | Classification HOSTILE, isolation immÃ©diate |

### 9.2 Principe d'autonomie

**MÃªme sans Internet, le systÃ¨me :**
- âœ”ï¸ Fonctionne (frontiÃ¨res locales actives)
- âœ”ï¸ DÃ©cide (StrongFather opÃ©rationnel)
- âœ”ï¸ Se protÃ¨ge (Border Guard dÃ©finit les menaces locales)
- âœ”ï¸ Se dÃ©grade (graduellement si nÃ©cessaire)

**Internet amÃ©liore la confiance, jamais la capacitÃ©.**

---

## 10. DÃ©tection et indicateurs

### 10.1 Indicateurs de menace par catÃ©gorie

| CatÃ©gorie | Indicateurs surveillÃ©s |
|-----------|----------------------|
| **Injection** | Patterns connus, donnÃ©es hors format, sÃ©quences suspectes |
| **Usurpation** | Contexte incohÃ©rent, device inconnu, replay dÃ©tectÃ© |
| **DoS** | Volume anormal, rythme anormal, patterns rÃ©pÃ©titifs |
| **Escalade** | Tentatives non autorisÃ©es, exploration de frontiÃ¨res |
| **Compromission** | Comportement anormal, signaux externes de compromission |

### 10.2 Niveaux de dÃ©tection

| Niveau | Description | Seuil par dÃ©faut |
|--------|-------------|------------------|
| **INFO** | Ã‰vÃ©nement normal Ã  tracer | Toujours |
| **WARNING** | Ã‰vÃ©nement inhabituel | Configuration dÃ©pendante |
| **ALERT** | Menace potentielle | Notification StrongFather |
| **CRITICAL** | Menace confirmÃ©e | Notification StrongFather + TAMR |
| **EMERGENCY** | Compromission active | Action immÃ©diate + notification tous cores |

### 10.3 Adaptation des seuils selon niveau de sÃ©curitÃ©

Les seuils de dÃ©tection s'adaptent au niveau de sÃ©curitÃ© dÃ©clarÃ© :

| Niveau de sÃ©curitÃ© | SensibilitÃ© | Comportement |
|--------------------|-------------|--------------|
| **0 - PUBLIC** | Basse | WARNING rarement, ALERT sur patterns Ã©vidents |
| **1 - STANDARD** | Standard | Seuils par dÃ©faut |
| **2 - SENSITIVE** | Haute | WARNING frÃ©quent, ALERT sur anomalies |
| **3 - CRITICAL** | TrÃ¨s haute | ALERT sur suspicions, CRITICAL rapidement |
| **4 - HARDENED** | Maximale | Toute anomalie = CRITICAL minimum |

---

## 11. Invariants de ce contrat

### INV-TMC-1 : Border Guard ne bloque jamais

Border Guard **dÃ©finit** les menaces et les rÃ©ponses conceptuelles. Il ne **bloque jamais** lui-mÃªme. L'application est dÃ©lÃ©guÃ©e Ã  BondingBrother.

### INV-TMC-2 : Internet n'est pas une autoritÃ©

Aucun signal Internet ne peut **imposer** une action au systÃ¨me. Les signaux sont des **informations** traitÃ©es selon l'Ã©tat local.

### INV-TMC-3 : Classification exhaustive des menaces

Toute menace dÃ©tectÃ©e **doit** conduire Ã  une classification de la source (UNKNOWN, HOSTILE, ou maintien avec surveillance).

### INV-TMC-4 : DÃ©gradation graduÃ©e

La rÃ©ponse Ã  une menace suit toujours une **gradation** (suspicion â†’ anomalie â†’ menace â†’ compromission), sauf compromission flagrante en niveau de sÃ©curitÃ© 4.

### INV-TMC-5 : Autonomie prÃ©servÃ©e

Le systÃ¨me **reste fonctionnel** mÃªme sans Internet. Les frontiÃ¨res locales restent actives et Border Guard continue de dÃ©finir les menaces.

### INV-TMC-6 : TraÃ§abilitÃ© des dÃ©tections

Toute dÃ©tection de menace est **traÃ§able** avec l'indicateur, le niveau de dÃ©tection, et la rÃ©ponse appliquÃ©e.

---

## 12. Interaction avec les autres cores

### 12.1 Flux vers StrongFather

| Ã‰vÃ©nement | Information transmise |
|-----------|----------------------|
| Menace dÃ©tectÃ©e | Type, indicateurs, classification source |
| Signal externe | Classification, comparaison Ã©tat local |
| Anomalie de franchissement | FrontiÃ¨re concernÃ©e, source, indicateurs |

**StrongFather dÃ©cide.** Border Guard informe.

### 12.2 Flux vers BondingBrother

| Ã‰vÃ©nement | Information transmise |
|-----------|----------------------|
| Classification HOSTILE | Source, raison, rÃ¨gles de blocage Ã  appliquer |
| RÃ¨gles de franchissement modifiÃ©es | Nouvelles rÃ¨gles pour la frontiÃ¨re |
| Isolation de frontiÃ¨re | FrontiÃ¨re concernÃ©e, niveau d'isolation |

**BondingBrother applique.** Border Guard dÃ©finit.

### 12.3 Flux vers Caring Nanny

| Ã‰vÃ©nement | Information transmise |
|-----------|----------------------|
| Ã‰tat des frontiÃ¨res | Saines, sous pression, compromises |
| Signaux de conformitÃ© | RÃ©sultat de comparaison avec Ã©tat local |
| Anomalies rÃ©seau | Type, gravitÃ©, impact potentiel |

**Caring Nanny observe.** Border Guard signale.

### 12.4 Flux vers TAMR

| Ã‰vÃ©nement | Information transmise |
|-----------|----------------------|
| Menace CRITICAL ou EMERGENCY | DÃ©tails complets pour l'humain |
| RÃ©habilitation requise | Source HOSTILE Ã  rÃ©habiliter |
| DÃ©cision humaine requise | Cas ambigu nÃ©cessitant intervention |

**TAMR implique l'humain.** Border Guard fournit le contexte.

---

## 13. RÃ©fÃ©rences croisÃ©es

### Invariants associÃ©s (Documentation Fondatrice - Section 7)

| Invariant | Ã‰noncÃ© | Relation |
|-----------|--------|----------|
| INV-BG-1 | Aucune capacitÃ© d'exÃ©cution | Border Guard dÃ©finit, ne bloque pas |
| INV-BG-3 | Aucune dÃ©cision autonome | Border Guard informe, StrongFather dÃ©cide |
| INV-BG-4 | Classification exhaustive | Toute source dÃ©tectÃ©e est classifiÃ©e |
| INV-BG-8 | TraÃ§abilitÃ© complÃ¨te | Toute dÃ©tection est traÃ§able |

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md) | Protocoles temps rÃ©el et asynchrones |
| [Miyukini Conceptual References - External Signal Trust](..//..//..//..//miyukini-webway-system//reference//_index.md) | Traitement des signaux Internet |
| [Border Guard - Security Levels Adaptation Contract](./Border%20Guard%20-%20Security%20Levels%20Adaptation%20Contract.md) | Adaptation des seuils |
| [Border Guard - Trust Level Classification Contract](../boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Classification des sources |
| [Border Guard - StrongFather Integration Contract](../integration/Border%20Guard%20-%20StrongFather%20Integration%20Contract.md) | Flux de dÃ©cision |
| [Border Guard - CaringNanny Integration Contract](../integration/Border%20Guard%20-%20CaringNanny%20Integration%20Contract.md) | Flux d'observation |

---

## 14. SynthÃ¨se contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Menaces catÃ©gorisÃ©es** â€” 13 types de menaces formellement dÃ©finies
2. **RÃ©ponses graduÃ©es** â€” De la suspicion Ã  la compromission
3. **Internet non autoritaire** â€” Signaux traitÃ©s comme informations
4. **Autonomie prÃ©servÃ©e** â€” Fonctionnement sans Internet
5. **DÃ©tection adaptative** â€” Seuils selon niveau de sÃ©curitÃ©
6. **TraÃ§abilitÃ© complÃ¨te** â€” Toute dÃ©tection documentÃ©e

### Phrase de synthÃ¨se

> **Border Guard dÃ©finit le modÃ¨le de menaces en catÃ©gorisant les attaques aux frontiÃ¨res externes, internes, d'intÃ©gration et rÃ©seau, en Ã©tablissant des rÃ©ponses graduÃ©es de la suspicion Ã  la compromission, tout en garantissant que les signaux Internet restent des informations et jamais des autoritÃ©s, prÃ©servant l'autonomie du systÃ¨me.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** Border Guard v1.5, Security Protocols v1.0, External Signal Trust v1.0  
**Type :** Contrat de modÃ¨le de menaces

