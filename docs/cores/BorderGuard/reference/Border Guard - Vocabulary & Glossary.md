# Border Guard â€” Vocabulary & Glossary

## Contexte

Ce document est le **glossaire canonique** de Border Guard. Il dÃ©finit de maniÃ¨re prÃ©cise, stable et non ambiguÃ« tous les termes utilisÃ©s dans la documentation Border Guard.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut :** Document de rÃ©fÃ©rence â€” Informatif et normatif pour la terminologie

---

## PortÃ©e / Scope

- **Applicable Ã  :** Toute la documentation Border Guard et les implÃ©mentations associÃ©es
- **Objectif :** Garantir une comprÃ©hension commune et non ambiguÃ« des termes
- **Usage :** RÃ©fÃ©rence pour la rÃ©daction de documents, revues de code, audits

---

## Vocabulaire canonique

### A

#### Action de franchissement

**DÃ©finition :** Comportement dÃ©clenchÃ© lorsqu'une rÃ¨gle de franchissement Ã©choue.

**Actions canoniques :**
- `DENY` â€” Refuser le franchissement
- `DEFER` â€” Soumettre Ã  StrongFather pour dÃ©cision
- `DEGRADE` â€” Autoriser avec restrictions
- `ALERT` â€” Autoriser mais alerter
- `LOG` â€” Autoriser et journaliser

**RÃ©fÃ©rence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 7

---

### C

#### Classification

**DÃ©finition :** Acte d'attribuer un niveau de confiance Ã  une source, une destination, ou une interaction. La classification est une responsabilitÃ© exclusive de Border Guard.

**CaractÃ©ristiques :**
- SystÃ©matique â€” Toute entitÃ© traversant une frontiÃ¨re doit Ãªtre classifiÃ©e
- DÃ©faut sÃ©curitaire â€” Si non classifiÃ©e explicitement, le niveau est "unknown"
- TraÃ§able â€” Chaque classification est traÃ§able (INV-BG-8)

**Invariant associÃ© :** INV-BG-4 â€” Classification exhaustive

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

---

#### CohÃ©rence globale

**DÃ©finition :** PropriÃ©tÃ© garantissant l'absence de contradiction entre les dÃ©finitions de Border Guard (frontiÃ¨res, niveaux de confiance, rÃ¨gles).

**Types de cohÃ©rence :**
- CohÃ©rence frontiÃ¨re-zone â€” Une frontiÃ¨re sÃ©pare exactement deux zones
- CohÃ©rence niveau-rÃ¨gle â€” Les rÃ¨gles sont compatibles avec les niveaux
- CohÃ©rence rÃ¨gle-rÃ¨gle â€” Les rÃ¨gles ne se contredisent pas
- CohÃ©rence temporelle â€” L'ordre chronologique est respectÃ©

**Invariant associÃ© :** INV-BG-9 â€” CohÃ©rence globale

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.3

---

#### Condition dÃ©clarative

**DÃ©finition :** Expression de ce qui est requis pour un franchissement, sans spÃ©cifier comment le vÃ©rifier techniquement.

**Exemples :**
- âœ… DÃ©claratif : "Niveau de confiance minimum : verified"
- âŒ ProcÃ©dural : "VÃ©rifier le token JWT et valider la signature"

**Invariant associÃ© :** INV-BG-6 â€” RÃ¨gles dÃ©claratives

**RÃ©fÃ©rence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 3

---

#### Contexte de frontiÃ¨re

**DÃ©finition :** Ensemble des informations relatives aux frontiÃ¨res traversÃ©es par une interaction, fourni par Border Guard aux autres cores.

**Contenu :**
- Quelles frontiÃ¨res sont traversÃ©es
- Quel niveau de confiance de la source
- Quelles rÃ¨gles de franchissement sont applicables
- Quel est l'Ã©tat des intÃ©grations concernÃ©es

**Usage :** Fourni Ã  StrongFather pour ses dÃ©cisions, Ã  BondingBrother pour l'application

**RÃ©fÃ©rence :** Documentation Fondatrice - Section 9

---

### D

#### DÃ©finition/Application (sÃ©paration)

**DÃ©finition :** Principe architectural fondamental Ã©tablissant que Border Guard dÃ©finit les frontiÃ¨res et rÃ¨gles, tandis que BondingBrother les applique techniquement.

**SchÃ©ma :**
```
Border Guard â†’ DÃ©finit (frontiÃ¨res, rÃ¨gles, niveaux)
                    â†“
             Contrat d'interface
                    â†“
BondingBrother â†’ Applique (filtrage, vÃ©rification, contrÃ´le)
```

**Invariant associÃ© :** INV-BG-7 â€” SÃ©paration dÃ©finition/application

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.1

---

### F

#### Franchissement

**DÃ©finition :** Acte de traverser une frontiÃ¨re. Chaque franchissement est soumis aux rÃ¨gles dÃ©finies pour la frontiÃ¨re concernÃ©e.

**Ã‰tats possibles :**
- AutorisÃ© â€” Toutes les rÃ¨gles sont satisfaites
- Conditionnel â€” Certaines rÃ¨gles nÃ©cessitent des vÃ©rifications supplÃ©mentaires
- Interdit â€” Une ou plusieurs rÃ¨gles critiques ne sont pas satisfaites

**RÃ©fÃ©rence :** Documentation Fondatrice - Section 4 (Concepts fondamentaux)

---

#### FrontiÃ¨re

**DÃ©finition :** DÃ©marcation conceptuelle qui sÃ©pare deux zones de confiance diffÃ©rentes. Elle reprÃ©sente le point de transition entre un niveau de confiance et un autre.

**PropriÃ©tÃ©s obligatoires :**
| PropriÃ©tÃ© | Description |
|-----------|-------------|
| Identifiant | Identifiant unique et stable |
| Nom | Nom descriptif et non ambigu |
| Description | Description et raison d'Ãªtre |
| Type | Externe, interne, ou intÃ©gration |
| Direction | EntrÃ©e, sortie, ou bidirectionnelle |
| PermÃ©abilitÃ© | Ouverte, contrÃ´lÃ©e, ou fermÃ©e |
| RÃ¨gles | RÃ¨gles de franchissement associÃ©es |
| TraÃ§abilitÃ© | Origine, date, historique |

**Invariant associÃ© :** INV-BG-5 â€” FrontiÃ¨res explicites

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md)

---

#### FrontiÃ¨re d'intÃ©gration

**DÃ©finition :** Type de frontiÃ¨re qui sÃ©pare l'Ã©cosystÃ¨me Miyukini d'un systÃ¨me externe avec lequel il interagit de maniÃ¨re contrÃ´lÃ©e.

**CaractÃ©ristiques :**
| Aspect | SpÃ©cification |
|--------|---------------|
| Zone source | Ã‰cosystÃ¨me Miyukini ou systÃ¨me externe intÃ©grÃ© |
| Zone destination | SystÃ¨me externe intÃ©grÃ© ou Ã©cosystÃ¨me Miyukini |
| Confiance par dÃ©faut | Selon classification de l'intÃ©gration (verified typiquement) |
| Direction typique | Bidirectionnelle |
| PermÃ©abilitÃ© typique | ContrÃ´lÃ©e |

**Exemples :**
- FrontiÃ¨re Supabase
- FrontiÃ¨re API partenaire
- FrontiÃ¨re service de paiement (Stripe)

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.3

---

#### FrontiÃ¨re externe

**DÃ©finition :** Type de frontiÃ¨re qui sÃ©pare l'Ã©cosystÃ¨me Miyukini du monde extÃ©rieur (internet, systÃ¨mes tiers, utilisateurs non authentifiÃ©s).

**CaractÃ©ristiques :**
| Aspect | SpÃ©cification |
|--------|---------------|
| Zone source | Monde extÃ©rieur (unknown ou hostile par dÃ©faut) |
| Zone destination | Ã‰cosystÃ¨me Miyukini |
| Confiance par dÃ©faut | Unknown |
| Direction typique | EntrÃ©e |
| PermÃ©abilitÃ© typique | ContrÃ´lÃ©e |

**Exemples :**
- FrontiÃ¨re API publique
- FrontiÃ¨re utilisateur non authentifiÃ©
- FrontiÃ¨re webhook

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.1

---

#### FrontiÃ¨re interne

**DÃ©finition :** Type de frontiÃ¨re qui sÃ©pare diffÃ©rentes zones de confiance au sein de l'Ã©cosystÃ¨me Miyukini.

**CaractÃ©ristiques :**
| Aspect | SpÃ©cification |
|--------|---------------|
| Zone source | Zone interne avec niveau de confiance X |
| Zone destination | Zone interne avec niveau de confiance Y (X â‰  Y) |
| Confiance par dÃ©faut | HÃ©ritÃ©e de la zone source |
| Direction typique | Bidirectionnelle |
| PermÃ©abilitÃ© typique | Variable |

**Exemples :**
- FrontiÃ¨re admin/utilisateur
- FrontiÃ¨re donnÃ©es sensibles
- FrontiÃ¨re module critique

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.2

---

### G

#### Garantie

**DÃ©finition :** Engagement formel que Border Guard prend envers les autres cores et le systÃ¨me global. Contrairement aux invariants (rÃ¨gles absolues), les garanties sont des promesses de service.

**Garanties de Border Guard :**
1. **ExhaustivitÃ©** â€” Toute frontiÃ¨re du systÃ¨me est explicitement dÃ©finie
2. **Classification complÃ¨te** â€” Toute source et interaction est classifiÃ©e
3. **CohÃ©rence** â€” Les dÃ©finitions sont globalement cohÃ©rentes
4. **TraÃ§abilitÃ©** â€” Toute dÃ©finition est traÃ§able
5. **NeutralitÃ© technique** â€” Les dÃ©finitions sont indÃ©pendantes de l'implÃ©mentation
6. **SÃ©paration stricte** â€” La dÃ©finition est sÃ©parÃ©e de l'application

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 7

---

#### Gouvernance d'intÃ©gration

**DÃ©finition :** Ensemble des rÃ¨gles et processus qui encadrent la relation avec les systÃ¨mes externes. Cette gouvernance dÃ©finit les conditions d'Ã©tablissement, de maintien, et de rÃ©vocation des intÃ©grations.

**ResponsabilitÃ© exclusive :** Border Guard

**RÃ©fÃ©rence :** Documentation Fondatrice - Section 5

---

### H

#### Hostile (niveau de confiance)

**DÃ©finition :** Niveau de confiance indiquant une confiance nulle â€” la source ou destination a Ã©tÃ© identifiÃ©e comme malveillante, compromise, ou violant les rÃ¨gles.

| Aspect | SpÃ©cification |
|--------|---------------|
| Code | `HOSTILE` |
| IcÃ´ne | ðŸ”´ |
| Signification | Confiance nulle, menace identifiÃ©e |
| VÃ©rification | Aucune â€” blocage direct |
| Restrictions | Aucune interaction autorisÃ©e |
| RÃ©vocabilitÃ© | Uniquement par processus formel de rÃ©habilitation |

**CritÃ¨res d'attribution :**
- Source blacklistÃ©e
- Pattern d'attaque dÃ©tectÃ©
- Compromission confirmÃ©e
- Violation grave

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.4

---

### I

#### IntÃ©gration

**DÃ©finition :** Relation Ã©tablie entre l'Ã©cosystÃ¨me Miyukini et un systÃ¨me externe. Une intÃ©gration est classifiÃ©e par Border Guard selon son niveau de confiance initial, les frontiÃ¨res qu'elle traverse, les rÃ¨gles applicables, et son Ã©tat.

**Ã‰tats possibles :**
- Active â€” IntÃ©gration fonctionnelle
- Suspendue â€” IntÃ©gration temporairement dÃ©sactivÃ©e
- RÃ©voquÃ©e â€” IntÃ©gration dÃ©finitivement terminÃ©e

**RÃ©fÃ©rence :** Documentation Fondatrice - Section 4

---

#### Invariant

**DÃ©finition :** RÃ¨gle absolue qui ne peut jamais Ãªtre violÃ©e, quel que soit le contexte. Les invariants dÃ©finissent les limites non nÃ©gociables de Border Guard.

**CaractÃ©ristiques :**
- Non nÃ©gociable â€” Aucune exception possible
- VÃ©rifiable â€” On peut toujours dÃ©terminer si l'invariant est respectÃ©
- IndÃ©pendant du contexte â€” S'applique toujours
- Absolu â€” Aucune considÃ©ration pratique ne justifie une violation

**Liste des invariants Border Guard :**
| Invariant | CatÃ©gorie | Description courte |
|-----------|-----------|-------------------|
| INV-BG-1 | IdentitÃ© | Aucune capacitÃ© d'exÃ©cution |
| INV-BG-2 | Comportement | Aucune persistance directe |
| INV-BG-3 | IdentitÃ© | Aucune dÃ©cision autonome |
| INV-BG-4 | Comportement | Classification exhaustive |
| INV-BG-5 | Comportement | FrontiÃ¨res explicites |
| INV-BG-6 | Comportement | RÃ¨gles dÃ©claratives |
| INV-BG-7 | QualitÃ© | SÃ©paration dÃ©finition/application |
| INV-BG-8 | QualitÃ© | TraÃ§abilitÃ© complÃ¨te |
| INV-BG-9 | QualitÃ© | CohÃ©rence globale |
| INV-BG-10 | QualitÃ© | NeutralitÃ© conceptuelle |

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md)

---

### N

#### NeutralitÃ© conceptuelle

**DÃ©finition :** PropriÃ©tÃ© garantissant que Border Guard ne fait jamais de supposition sur la technologie d'implÃ©mentation. Les dÃ©finitions sont purement conceptuelles et peuvent Ãªtre implÃ©mentÃ©es par n'importe quelle technologie.

**Exemples :**
| âœ… Neutre | âŒ CouplÃ© |
|-----------|-----------|
| "Authentification requise" | "Token JWT requis" |
| "DonnÃ©es chiffrÃ©es" | "AES-256-GCM requis" |
| "Connexion sÃ©curisÃ©e" | "HTTPS/TLS 1.3 requis" |

**Invariant associÃ© :** INV-BG-10 â€” NeutralitÃ© conceptuelle

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.4

---

#### Niveau de confiance

**DÃ©finition :** Classification qui indique le degrÃ© de fiabilitÃ© accordÃ© Ã  une source, une destination, ou une interaction. Border Guard dÃ©finit quatre niveaux canoniques.

**Niveaux canoniques :**
| Niveau | Code | IcÃ´ne | Signification |
|--------|------|-------|---------------|
| Trusted | `TRUSTED` | ðŸŸ¢ | Confiance totale â€” cercle de confiance absolu |
| Verified | `VERIFIED` | ðŸ”µ | Confiance vÃ©rifiÃ©e â€” authentifiÃ© et validÃ© |
| Unknown | `UNKNOWN` | ðŸŸ¡ | Confiance inconnue â€” niveau par dÃ©faut |
| Hostile | `HOSTILE` | ðŸ”´ | Confiance nulle â€” source malveillante |

**Invariant associÃ© :** INV-BG-4 â€” Classification exhaustive

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

---

### P

#### PermÃ©abilitÃ©

**DÃ©finition :** CaractÃ©ristique d'une frontiÃ¨re qui indique sa propension Ã  autoriser le franchissement.

**Niveaux de permÃ©abilitÃ© :**
| Niveau | Description | Niveau de sÃ©curitÃ© associÃ© |
|--------|-------------|---------------------------|
| **Ouverte** | Franchissement libre sous conditions minimales | 0 (PUBLIC) |
| **ContrÃ´lÃ©e** | Franchissement soumis Ã  vÃ©rification | 1-3 (STANDARD Ã  CRITICAL) |
| **FermÃ©e** | Franchissement interdit sauf exceptions | 4 (HARDENED) |

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 6

---

#### PrioritÃ© (de rÃ¨gle)

**DÃ©finition :** Ordre d'Ã©valuation des rÃ¨gles de franchissement. Les rÃ¨gles avec prioritÃ© basse (1-10) sont Ã©valuÃ©es en premier.

**Plages de prioritÃ© :**
| Plage | Description | Exemples |
|-------|-------------|----------|
| 1-10 | RÃ¨gles de sÃ©curitÃ© critiques | Blocage hostile, rate limiting |
| 11-30 | RÃ¨gles de niveau de confiance | VÃ©rification TRUSTED, VERIFIED |
| 31-50 | RÃ¨gles d'authentification | Session valide, MFA |
| 51-70 | RÃ¨gles de donnÃ©es | Classification, types |
| 71-90 | RÃ¨gles d'action | Lecture seule, impact |
| 91-100 | RÃ¨gles temporelles et autres | Heures, quotas |

**RÃ©fÃ©rence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 6

---

### R

#### RÃ¨gle de franchissement

**DÃ©finition :** Condition dÃ©clarative qui doit Ãªtre satisfaite pour qu'une interaction puisse traverser une frontiÃ¨re. Elle exprime ce qui est requis, pas comment le vÃ©rifier techniquement.

**Types de rÃ¨gles :**
| Type | Code | Description |
|------|------|-------------|
| Niveau de confiance | `niveau_confiance` | Niveau requis pour franchir |
| Authentification | `authentification` | Ã‰tat d'authentification requis |
| DonnÃ©es | `donnees` | Nature des donnÃ©es autorisÃ©es |
| Action | `action` | Actions autorisÃ©es |
| Temporel | `temporel` | Contraintes de temps |

**Invariant associÃ© :** INV-BG-6 â€” RÃ¨gles dÃ©claratives

**RÃ©fÃ©rence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md)

---

### T

#### TraÃ§abilitÃ©

**DÃ©finition :** CapacitÃ© Ã  retracer l'origine, l'historique et la justification de toute dÃ©finition dans Border Guard.

**MÃ©tadonnÃ©es obligatoires :**
| MÃ©tadonnÃ©e | Description |
|------------|-------------|
| Origine | Qui a crÃ©Ã©/modifiÃ© l'Ã©lÃ©ment |
| Date de crÃ©ation | Horodatage de crÃ©ation |
| Date de modification | Horodatage de derniÃ¨re modification |
| Justification | Pourquoi cet Ã©lÃ©ment existe |
| Historique | Journal des modifications |
| Version | NumÃ©ro de version |

**Invariant associÃ© :** INV-BG-8 â€” TraÃ§abilitÃ© complÃ¨te

**RÃ©fÃ©rence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.2

---

#### Transition (de niveau de confiance)

**DÃ©finition :** Passage d'un niveau de confiance Ã  un autre, soumis Ã  des rÃ¨gles strictes.

**RÃ¨gles de transition :**
| RÃ¨gle | Description |
|-------|-------------|
| TRANS-1 | UNKNOWN â†’ VERIFIED â†’ TRUSTED (progression obligatoire) |
| TRANS-2 | * â†’ HOSTILE (transition immÃ©diate possible) |
| TRANS-3 | HOSTILE â†’ UNKNOWN (rÃ©habilitation obligatoire via UNKNOWN) |
| TRANS-4 | VERIFIED expirÃ© â†’ UNKNOWN (pas HOSTILE) |
| TRANS-5 | Toute transition est traÃ§able |

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 6

---

#### Trusted (niveau de confiance)

**DÃ©finition :** Niveau de confiance indiquant une confiance totale â€” la source ou destination fait partie du cercle de confiance absolu.

| Aspect | SpÃ©cification |
|--------|---------------|
| Code | `TRUSTED` |
| IcÃ´ne | ðŸŸ¢ |
| Signification | Confiance absolue, cercle interne |
| VÃ©rification | Aucune vÃ©rification supplÃ©mentaire requise |
| Restrictions | Aucune restriction par dÃ©faut |
| RÃ©vocabilitÃ© | RÃ©vocable (mais rare) |

**Exemples de sources Trusted :**
- StrongFather, KindMother, Border Guard (cores systÃ¨me)
- Modules internes certifiÃ©s du noyau

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.1

---

### U

#### Unknown (niveau de confiance)

**DÃ©finition :** Niveau de confiance indiquant une confiance inconnue â€” la source ou destination n'a pas encore Ã©tÃ© classifiÃ©e. C'est le niveau par dÃ©faut pour tout ce qui arrive de l'extÃ©rieur.

| Aspect | SpÃ©cification |
|--------|---------------|
| Code | `UNKNOWN` |
| IcÃ´ne | ðŸŸ¡ |
| Signification | Confiance non Ã©tablie, prudence requise |
| VÃ©rification | VÃ©rifications systÃ©matiques requises |
| Restrictions | RÃ¨gles restrictives par dÃ©faut |
| Ã‰volution | Peut Ã©voluer vers VERIFIED ou HOSTILE |

**RÃ¨gle fondamentale :** "Unknown" n'est pas "hostile". C'est un Ã©tat d'attente qui peut Ã©voluer.

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.3

---

### V

#### Verified (niveau de confiance)

**DÃ©finition :** Niveau de confiance indiquant une confiance vÃ©rifiÃ©e â€” la source ou destination a Ã©tÃ© authentifiÃ©e et validÃ©e selon des critÃ¨res stricts.

| Aspect | SpÃ©cification |
|--------|---------------|
| Code | `VERIFIED` |
| IcÃ´ne | ðŸ”µ |
| Signification | Confiance accordÃ©e aprÃ¨s vÃ©rification |
| VÃ©rification | VÃ©rifications effectuÃ©es, rÃ©sultat positif |
| Restrictions | Selon le contexte et les rÃ¨gles |
| RÃ©vocabilitÃ© | RÃ©vocable Ã  tout moment |

**Exemples de sources Verified :**
- Utilisateur authentifiÃ© avec session valide
- API partenaire avec authentification valide
- IntÃ©gration Supabase avec credentials valides

**RÃ©fÃ©rence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.2

---

### Z

#### Zone de confiance

**DÃ©finition :** Espace conceptuel dÃ©limitÃ© par des frontiÃ¨res, oÃ¹ tous les Ã©lÃ©ments partagent un mÃªme niveau de confiance.

**PropriÃ©tÃ©s :**
| PropriÃ©tÃ© | Description |
|-----------|-------------|
| Identifiant | Identifiant unique de la zone |
| Niveau de confiance | Niveau homogÃ¨ne (trusted, verified, unknown, hostile) |
| FrontiÃ¨res | Liste des frontiÃ¨res dÃ©limitant la zone |
| Contenu | Composants, donnÃ©es, services contenus |

**HiÃ©rarchie des zones :**
```
ZONE EXTERNE (hostile/unknown)
  â””â”€ ZONE PÃ‰RIPHÃ‰RIQUE (unknown/verified)
      â””â”€ ZONE UTILISATEUR (verified)
          â””â”€ ZONE ADMIN (verified+)
              â””â”€ ZONE SYSTÃˆME (trusted)
                  â””â”€ ZONE CRITIQUE (trusted isolÃ©)
```

**RÃ©fÃ©rence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 8

---

## Index alphabÃ©tique rapide

| Terme | Section |
|-------|---------|
| Action de franchissement | A |
| Classification | C |
| CohÃ©rence globale | C |
| Condition dÃ©clarative | C |
| Contexte de frontiÃ¨re | C |
| DÃ©finition/Application | D |
| Franchissement | F |
| FrontiÃ¨re | F |
| FrontiÃ¨re d'intÃ©gration | F |
| FrontiÃ¨re externe | F |
| FrontiÃ¨re interne | F |
| Garantie | G |
| Gouvernance d'intÃ©gration | G |
| Hostile | H |
| IntÃ©gration | I |
| Invariant | I |
| NeutralitÃ© conceptuelle | N |
| Niveau de confiance | N |
| PermÃ©abilitÃ© | P |
| PrioritÃ© | P |
| RÃ¨gle de franchissement | R |
| TraÃ§abilitÃ© | T |
| Transition | T |
| Trusted | T |
| Unknown | U |
| Verified | V |
| Zone de confiance | Z |

---

## Documents de rÃ©fÃ©rence

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | DÃ©finitions originales (Section 9) |
| [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Termes de frontiÃ¨res |
| [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Termes de niveaux de confiance |
| [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Termes de rÃ¨gles |
| [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) | Termes de gouvernance |
| [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) | Glossaire gÃ©nÃ©ral Miyukini |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Document de rÃ©fÃ©rence â€” Glossaire canonique  
**RÃ©fÃ©rence :** Border Guard v1.5, Documentation Fondatrice Section 9

