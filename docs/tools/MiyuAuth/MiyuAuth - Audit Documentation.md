# MiyuAuth â€” Audit de la documentation

## Contexte

Ce document constitue l'**audit de la documentation MiyuAuth** au regard des rÃ©fÃ©rences [docs/reference](..//..//_index.md) et du modÃ¨le [docs/tools/MiyuSQL](..//..//_index.md). Il identifie les points conformes et les amÃ©liorations possibles.

**Date d'audit :** 2026-01-30  
**RÃ©fÃ©rences utilisÃ©es :** Glossaire, Tools et Toolkits, Connexion Inter-COG, Security Levels, Standardisation NumÃ©ration Invariants, MiyuSQL (structure et contrats).

---

## 1. ConformitÃ© dÃ©jÃ  acquise

| Domaine | Ã‰tat | DÃ©tail |
|--------|------|--------|
| **Structure** | âœ… | Arborescence alignÃ©e sur MiyuSQL : Fondatrice, Reference Outils, _index, contracts (governance, boundaries, security, integration, testing), dependencies, implementation. |
| **Terminologie** | âœ… | Glossaire respectÃ© : Passeport Utilisateur, Visa de Connexion, COG HÃ©bergeur, COG Origine, Utilisateur Visiteur, Utilisateur Externe, citoyen. |
| **Flux de gouvernance** | âœ… | BondingBrother â†’ Master Butler â†’ WorrySentinel â†’ Caring Nanny â†’ StrongFather â†’ ExÃ©cution ; cohÃ©rent avec Tools et Toolkits (MiyuAuth explicite StrongFather, la rÃ©fÃ©rence schÃ©matise sans le dÃ©tailler). |
| **Relation KindMother** | âœ… | KindMother = validateur unique de la confiance ; MiyuAuth exÃ©cute sans dÃ©cider ; invariants INV-KM-* clairs. |
| **Relation MiyuSQL** | âœ… | Section 8bis Documentation Fondatrice + 6bis KindMother Integration : persistance = KindMother + MiyuSQL ; MiyuAuth opÃ¨re sur donnÃ©es fournies. |
| **ToolkitId / ToolIds** | âœ… | Format `toolkit.identity.miyauth`, `tool.identity.<action>` conforme Master Butler. |
| **SÃ©curitÃ© et Ã©tats** | âœ… | Niveau 2 ou 3, HEALTHY/DEGRADED autorisÃ©s, SECURITY_LOCKDOWN/MAINTENANCE interdits ; alignement WorrySentinel et Caring Nanny. |
| **MIP** | âœ… | Domaine `identity`, layer Strate 6, blocs futurs (id, do, role, layer). |

---

## 2. AmÃ©liorations possibles

### 2.1 RÃ©fÃ©rences documentaires manquantes ou Ã  renforcer

| RÃ©fÃ©rence | OÃ¹ lâ€™ajouter | Raison |
|-----------|--------------|--------|
| **Security Levels** ([Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md)) | Security and States Contract, section RÃ©fÃ©rences croisÃ©es | RÃ©fÃ©rence conceptuelle des niveaux 0â€“4 ; justification explicite du niveau 2 ou 3 (Niveau 2 = Sensitive Data, Niveau 3 = Critical System / Auth). |
| **Ever Buddy** (Tools et Toolkits Â§ 5.2, Â§ 6) | Documentation Fondatrice Â§ 6 ou Dependencies Contract | Ever Buddy gÃ¨re cycle de vie, versions et compatibilitÃ© des Outils ; MiyuAuth est dÃ©clarÃ© et compatibilisÃ© par lâ€™environnement (Toolkit Composition). |
| **Standardisation NumÃ©ration Invariants** | _index ou Implementation Guidelines | Documenter les prÃ©fixes dâ€™invariants MiyuAuth (INV-BOUND-*, INV-DEP-*, INV-KM-*, INV-SEC-*, etc.) et leur lien Ã©ventuel au standard (format INV-&lt;PREFIX&gt;-&lt;NUMERO&gt;). Les contrats MiyuAuth utilisent des prÃ©fixes catÃ©goriels (BOUND, DEP, SEC) ; le standard couvre les Cores ; option : ajouter une note dans _index ou Dependencies. |
| **SouverainetÃ© Environnement / Pyramide** | RÃ©fÃ©rences croisÃ©es Documentation Fondatrice ou _index | Contexte Strate 6 et souverainetÃ© applicative (bibliothÃ¨que dâ€™outils gouvernÃ©e). |
| **Connexion Inter-COG â€” champs Passeport / Visa** | Reference Outils (Â§ 4.3, 4.4) | Rappeler que les champs des artefacts (ex. Passeport Â§ 3.1, Visa Â§ 3.3 et niveaux S1â€“S5) sont ceux sur lesquels `verify` / `role` opÃ¨rent, sans dÃ©cider de lâ€™autorisation. |

### 2.2 Contenu Ã  prÃ©ciser

| Sujet | Suggestion |
|-------|------------|
| **Niveaux Visa (S1â€“S5)** | Dans Reference Outils (tool.identity.verify) ou Security Contract : prÃ©ciser que la vÃ©rification peut porter sur le `security_level` (S1â€“S5) du Visa (Connexion Inter-COG Â§ 4) sans que MiyuAuth dÃ©cide de lâ€™autorisation. |
| **RÃ´le Â« externe Â»** | Dans Reference Outils Â§ 4.4 (tool.identity.role) : ajouter une phrase reliant Â« externe Â» Ã  Utilisateur Externe et Mandat Public dâ€™AccÃ¨s (Connexion Inter-COG Â§ 11), pour cohÃ©rence avec la rÃ©fÃ©rence. |
| **_index â€” description Fondation** | Dans _index, ligne Â« Documentation Fondatrice Â» : ajouter Â« relation MiyuSQL (donnÃ©es identitÃ©, Passeport, Visa) Â» pour reflÃ©ter la section 8bis. |
| **Flux schÃ©matique** | Documentation Fondatrice Â§ 6 : le flux liste bien StrongFather ; la rÃ©fÃ©rence Tools et Toolkits ne le montre pas dans le schÃ©ma ASCII. Conserver la formulation MiyuAuth (complÃ¨te) ; pas de changement requis. |

### 2.3 Invariants et numÃ©rotation

| Ã‰lÃ©ment | Ã‰tat | Recommandation |
|--------|------|----------------|
| **INV-KM-*** | UtilisÃ© dans KindMother Integration | KM = KindMother dans le standard ; ici ce sont des invariants du *contrat* dâ€™intÃ©gration. Pas dâ€™ambiguÃ¯tÃ© si le contrat est lu en entier ; option : prÃ©fixe INV-MAUTH-KM-* pour distinguer Â« contrat MiyuAuthâ€“KindMother Â» du core KindMother. |
| **INV-BOUND-*, INV-DEP-*, INV-SEC-*** | PrÃ©fixes catÃ©goriels | CohÃ©rents en interne ; le standard cible les Cores. Ajouter une courte note dans Runtime Boundary ou dans _index : Â« Invariants de bornage (BOUND), dÃ©pendances (DEP), sÃ©curitÃ© (SEC) ; voir Standardisation NumÃ©ration Invariants pour le format canonique des Cores. Â» |
| **INV-UT-MAUTH-*, INV-CT-MAUTH-*** | Tests | Clairs ; pas de changement requis. |

### 2.4 Alignement MiyuSQL (modÃ¨le)

| Aspect | MiyuSQL | MiyuAuth | Action |
|--------|---------|----------|--------|
| RÃ©fÃ©rence Â« Acces DB et Droits Agents IA Â» | Oui (Fondatrice) | N/A (MiyuAuth ne touche pas Ã  la DB) | â€” |
| RÃ©fÃ©rence MiyukiniAdmin Cycle Tests | Oui (_index) | Mention Â« exÃ©cutable par MiyukiniAdmin Â» dans contrats | Option : ajouter dans _index un lien vers MiyukiniAdmin - Cycle Tests Contract si un test MiyuAuth y est dÃ©fini. |
| RÃ©fÃ©rence Security Levels | Non explicite dans MiyuSQL | Idem | Ajouter dans MiyuAuth Security and States Contract. |

---

## 3. Plan dâ€™actions recommandÃ©

### PrioritÃ© haute (rÃ©fÃ©rences et cohÃ©rence)

1. **Security and States Contract** : ajouter en rÃ©fÃ©rences croisÃ©es [Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md).
2. **_index** : complÃ©ter la description de la Documentation Fondatrice par Â« relation MiyuSQL (donnÃ©es identitÃ©, Passeport, Visa) Â».
3. **Reference Outils** : dans Â§ 4.4 (role), ajouter une phrase sur le rÃ´le Â« externe Â» (Utilisateur Externe / Mandat Public dâ€™AccÃ¨s).

### PrioritÃ© moyenne (enrichissement)

4. **Documentation Fondatrice ou Dependencies Contract** : mentionner Ever Buddy (cycle de vie / compatibilitÃ© des Outils, conformÃ©ment Ã  Tools et Toolkits).
5. **Reference Outils** Â§ 4.3 (verify) : prÃ©ciser que la vÃ©rification peut porter sur les champs Passeport/Visa (dont `security_level` S1â€“S5) sans dÃ©cider de lâ€™autorisation.
6. **KindMother Integration Contract** : ajouter en rÃ©fÃ©rence [Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md) si le contrat mentionne le niveau de sÃ©curitÃ©.

### PrioritÃ© basse (optionnel)

7. **Standardisation NumÃ©ration Invariants** : courte note dans _index ou Implementation Guidelines sur les prÃ©fixes dâ€™invariants MiyuAuth.
8. **RÃ©fÃ©rences SouverainetÃ© / Pyramide** : lien depuis Documentation Fondatrice ou _index pour ancrage architecture.
9. **MiyukiniAdmin - Cycle Tests Contract** : lien depuis _index MiyuAuth si un test MiyuAuth y est dÃ©crit.

---

## 3bis. Ã‰tat d'application des recommandations

| # | Recommandation | Statut |
|---|----------------|--------|
| 1 | Security and States Contract â€” rÃ©fÃ©rence Security Levels | âœ… AppliquÃ© |
| 2 | _index â€” description Documentation Fondatrice (relation MiyuSQL) | âœ… AppliquÃ© |
| 3 | Reference Outils Â§ 4.4 â€” rÃ´le Â« externe Â» (Utilisateur Externe / Mandat Public) | âœ… AppliquÃ© |
| 4 | Documentation Fondatrice / Dependencies â€” Ever Buddy | âœ… AppliquÃ© (Documentation Fondatrice Â§ 6) |
| 5 | Reference Outils Â§ 4.3 â€” verify, champs Passeport/Visa S1â€“S5 | âœ… AppliquÃ© |
| 6 | KindMother Integration Contract â€” rÃ©fÃ©rence Security Levels | âœ… AppliquÃ© |
| 7 | Standardisation NumÃ©ration Invariants â€” note _index + Implementation Guidelines | âœ… AppliquÃ© (_index Contexte ; Implementation Guidelines Â§ 5.3 ; Runtime Boundary Â§ 5) |
| 8 | SouverainetÃ© / Pyramide â€” rÃ©fÃ©rences Documentation Fondatrice et _index | âœ… AppliquÃ© |
| 9 | MiyukiniAdmin - Cycle Tests Contract â€” lien _index | âœ… AppliquÃ© |

---

## 4. SynthÃ¨se

La documentation MiyuAuth est **dÃ©jÃ  alignÃ©e** avec les rÃ©fÃ©rences (Glossaire, Tools et Toolkits, Connexion Inter-COG) et avec le modÃ¨le MiyuSQL (structure, contrats, relation KindMother, relation MiyuSQL). Les amÃ©liorations proposÃ©es portent sur :

- **RÃ©fÃ©rences explicites** : Security Levels, Ever Buddy, Ã©ventuellement SouverainetÃ© / Pyramide.
- **PrÃ©cisions de contenu** : rÃ´le Â« externe Â», champs Passeport/Visa et niveaux S1â€“S5, description _index.
- **NumÃ©rotation des invariants** : note optionnelle pour lien avec le standard.

Les modifications de prioritÃ© haute peuvent Ãªtre appliquÃ©es immÃ©diatement ; les autres au fil des mises Ã  jour.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document dâ€™audit


