# Border Guard — Vocabulary & Glossary

## Contexte

Ce document est le **glossaire canonique** de Border Guard. Il définit de manière précise, stable et non ambiguë tous les termes utilisés dans la documentation Border Guard.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut :** Document de référence — Informatif et normatif pour la terminologie

---

## Portée / Scope

- **Applicable à :** Toute la documentation Border Guard et les implémentations associées
- **Objectif :** Garantir une compréhension commune et non ambiguë des termes
- **Usage :** Référence pour la rédaction de documents, revues de code, audits

---

## Vocabulaire canonique

### A

#### Action de franchissement

**Définition :** Comportement déclenché lorsqu'une règle de franchissement échoue.

**Actions canoniques :**
- `DENY` — Refuser le franchissement
- `DEFER` — Soumettre à StrongFather pour décision
- `DEGRADE` — Autoriser avec restrictions
- `ALERT` — Autoriser mais alerter
- `LOG` — Autoriser et journaliser

**Référence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 7

---

### C

#### Classification

**Définition :** Acte d'attribuer un niveau de confiance à une source, une destination, ou une interaction. La classification est une responsabilité exclusive de Border Guard.

**Caractéristiques :**
- Systématique — Toute entité traversant une frontière doit être classifiée
- Défaut sécuritaire — Si non classifiée explicitement, le niveau est "unknown"
- Traçable — Chaque classification est traçable (INV-BG-8)

**Invariant associé :** INV-BG-4 — Classification exhaustive

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

---

#### Cohérence globale

**Définition :** Propriété garantissant l'absence de contradiction entre les définitions de Border Guard (frontières, niveaux de confiance, règles).

**Types de cohérence :**
- Cohérence frontière-zone — Une frontière sépare exactement deux zones
- Cohérence niveau-règle — Les règles sont compatibles avec les niveaux
- Cohérence règle-règle — Les règles ne se contredisent pas
- Cohérence temporelle — L'ordre chronologique est respecté

**Invariant associé :** INV-BG-9 — Cohérence globale

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.3

---

#### Condition déclarative

**Définition :** Expression de ce qui est requis pour un franchissement, sans spécifier comment le vérifier techniquement.

**Exemples :**
- ✅ Déclaratif : "Niveau de confiance minimum : verified"
- ❌ Procédural : "Vérifier le token JWT et valider la signature"

**Invariant associé :** INV-BG-6 — Règles déclaratives

**Référence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 3

---

#### Contexte de frontière

**Définition :** Ensemble des informations relatives aux frontières traversées par une interaction, fourni par Border Guard aux autres cores.

**Contenu :**
- Quelles frontières sont traversées
- Quel niveau de confiance de la source
- Quelles règles de franchissement sont applicables
- Quel est l'état des intégrations concernées

**Usage :** Fourni à StrongFather pour ses décisions, à BondingBrother pour l'application

**Référence :** Documentation Fondatrice - Section 9

---

### D

#### Définition/Application (séparation)

**Définition :** Principe architectural fondamental établissant que Border Guard définit les frontières et règles, tandis que BondingBrother les applique techniquement.

**Schéma :**
```
Border Guard → Définit (frontières, règles, niveaux)
                    ↓
             Contrat d'interface
                    ↓
BondingBrother → Applique (filtrage, vérification, contrôle)
```

**Invariant associé :** INV-BG-7 — Séparation définition/application

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.1

---

### F

#### Franchissement

**Définition :** Acte de traverser une frontière. Chaque franchissement est soumis aux règles définies pour la frontière concernée.

**États possibles :**
- Autorisé — Toutes les règles sont satisfaites
- Conditionnel — Certaines règles nécessitent des vérifications supplémentaires
- Interdit — Une ou plusieurs règles critiques ne sont pas satisfaites

**Référence :** Documentation Fondatrice - Section 4 (Concepts fondamentaux)

---

#### Frontière

**Définition :** Démarcation conceptuelle qui sépare deux zones de confiance différentes. Elle représente le point de transition entre un niveau de confiance et un autre.

**Propriétés obligatoires :**
| Propriété | Description |
|-----------|-------------|
| Identifiant | Identifiant unique et stable |
| Nom | Nom descriptif et non ambigu |
| Description | Description et raison d'être |
| Type | Externe, interne, ou intégration |
| Direction | Entrée, sortie, ou bidirectionnelle |
| Perméabilité | Ouverte, contrôlée, ou fermée |
| Règles | Règles de franchissement associées |
| Traçabilité | Origine, date, historique |

**Invariant associé :** INV-BG-5 — Frontières explicites

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md)

---

#### Frontière d'intégration

**Définition :** Type de frontière qui sépare l'écosystème Miyukini d'un système externe avec lequel il interagit de manière contrôlée.

**Caractéristiques :**
| Aspect | Spécification |
|--------|---------------|
| Zone source | Écosystème Miyukini ou système externe intégré |
| Zone destination | Système externe intégré ou écosystème Miyukini |
| Confiance par défaut | Selon classification de l'intégration (verified typiquement) |
| Direction typique | Bidirectionnelle |
| Perméabilité typique | Contrôlée |

**Exemples :**
- Frontière Supabase
- Frontière API partenaire
- Frontière service de paiement (Stripe)

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.3

---

#### Frontière externe

**Définition :** Type de frontière qui sépare l'écosystème Miyukini du monde extérieur (internet, systèmes tiers, utilisateurs non authentifiés).

**Caractéristiques :**
| Aspect | Spécification |
|--------|---------------|
| Zone source | Monde extérieur (unknown ou hostile par défaut) |
| Zone destination | Écosystème Miyukini |
| Confiance par défaut | Unknown |
| Direction typique | Entrée |
| Perméabilité typique | Contrôlée |

**Exemples :**
- Frontière API publique
- Frontière utilisateur non authentifié
- Frontière webhook

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.1

---

#### Frontière interne

**Définition :** Type de frontière qui sépare différentes zones de confiance au sein de l'écosystème Miyukini.

**Caractéristiques :**
| Aspect | Spécification |
|--------|---------------|
| Zone source | Zone interne avec niveau de confiance X |
| Zone destination | Zone interne avec niveau de confiance Y (X ≠ Y) |
| Confiance par défaut | Héritée de la zone source |
| Direction typique | Bidirectionnelle |
| Perméabilité typique | Variable |

**Exemples :**
- Frontière admin/utilisateur
- Frontière données sensibles
- Frontière module critique

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 5.2

---

### G

#### Garantie

**Définition :** Engagement formel que Border Guard prend envers les autres cores et le système global. Contrairement aux invariants (règles absolues), les garanties sont des promesses de service.

**Garanties de Border Guard :**
1. **Exhaustivité** — Toute frontière du système est explicitement définie
2. **Classification complète** — Toute source et interaction est classifiée
3. **Cohérence** — Les définitions sont globalement cohérentes
4. **Traçabilité** — Toute définition est traçable
5. **Neutralité technique** — Les définitions sont indépendantes de l'implémentation
6. **Séparation stricte** — La définition est séparée de l'application

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 7

---

#### Gouvernance d'intégration

**Définition :** Ensemble des règles et processus qui encadrent la relation avec les systèmes externes. Cette gouvernance définit les conditions d'établissement, de maintien, et de révocation des intégrations.

**Responsabilité exclusive :** Border Guard

**Référence :** Documentation Fondatrice - Section 5

---

### H

#### Hostile (niveau de confiance)

**Définition :** Niveau de confiance indiquant une confiance nulle — la source ou destination a été identifiée comme malveillante, compromise, ou violant les règles.

| Aspect | Spécification |
|--------|---------------|
| Code | `HOSTILE` |
| Icône | 🔴 |
| Signification | Confiance nulle, menace identifiée |
| Vérification | Aucune — blocage direct |
| Restrictions | Aucune interaction autorisée |
| Révocabilité | Uniquement par processus formel de réhabilitation |

**Critères d'attribution :**
- Source blacklistée
- Pattern d'attaque détecté
- Compromission confirmée
- Violation grave

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.4

---

### I

#### Intégration

**Définition :** Relation établie entre l'écosystème Miyukini et un système externe. Une intégration est classifiée par Border Guard selon son niveau de confiance initial, les frontières qu'elle traverse, les règles applicables, et son état.

**États possibles :**
- Active — Intégration fonctionnelle
- Suspendue — Intégration temporairement désactivée
- Révoquée — Intégration définitivement terminée

**Référence :** Documentation Fondatrice - Section 4

---

#### Invariant

**Définition :** Règle absolue qui ne peut jamais être violée, quel que soit le contexte. Les invariants définissent les limites non négociables de Border Guard.

**Caractéristiques :**
- Non négociable — Aucune exception possible
- Vérifiable — On peut toujours déterminer si l'invariant est respecté
- Indépendant du contexte — S'applique toujours
- Absolu — Aucune considération pratique ne justifie une violation

**Liste des invariants Border Guard :**
| Invariant | Catégorie | Description courte |
|-----------|-----------|-------------------|
| INV-BG-1 | Identité | Aucune capacité d'exécution |
| INV-BG-2 | Comportement | Aucune persistance directe |
| INV-BG-3 | Identité | Aucune décision autonome |
| INV-BG-4 | Comportement | Classification exhaustive |
| INV-BG-5 | Comportement | Frontières explicites |
| INV-BG-6 | Comportement | Règles déclaratives |
| INV-BG-7 | Qualité | Séparation définition/application |
| INV-BG-8 | Qualité | Traçabilité complète |
| INV-BG-9 | Qualité | Cohérence globale |
| INV-BG-10 | Qualité | Neutralité conceptuelle |

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md)

---

### N

#### Neutralité conceptuelle

**Définition :** Propriété garantissant que Border Guard ne fait jamais de supposition sur la technologie d'implémentation. Les définitions sont purement conceptuelles et peuvent être implémentées par n'importe quelle technologie.

**Exemples :**
| ✅ Neutre | ❌ Couplé |
|-----------|-----------|
| "Authentification requise" | "Token JWT requis" |
| "Données chiffrées" | "AES-256-GCM requis" |
| "Connexion sécurisée" | "HTTPS/TLS 1.3 requis" |

**Invariant associé :** INV-BG-10 — Neutralité conceptuelle

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.4

---

#### Niveau de confiance

**Définition :** Classification qui indique le degré de fiabilité accordé à une source, une destination, ou une interaction. Border Guard définit quatre niveaux canoniques.

**Niveaux canoniques :**
| Niveau | Code | Icône | Signification |
|--------|------|-------|---------------|
| Trusted | `TRUSTED` | 🟢 | Confiance totale — cercle de confiance absolu |
| Verified | `VERIFIED` | 🔵 | Confiance vérifiée — authentifié et validé |
| Unknown | `UNKNOWN` | 🟡 | Confiance inconnue — niveau par défaut |
| Hostile | `HOSTILE` | 🔴 | Confiance nulle — source malveillante |

**Invariant associé :** INV-BG-4 — Classification exhaustive

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md)

---

### P

#### Perméabilité

**Définition :** Caractéristique d'une frontière qui indique sa propension à autoriser le franchissement.

**Niveaux de perméabilité :**
| Niveau | Description | Niveau de sécurité associé |
|--------|-------------|---------------------------|
| **Ouverte** | Franchissement libre sous conditions minimales | 0 (PUBLIC) |
| **Contrôlée** | Franchissement soumis à vérification | 1-3 (STANDARD à CRITICAL) |
| **Fermée** | Franchissement interdit sauf exceptions | 4 (HARDENED) |

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 6

---

#### Priorité (de règle)

**Définition :** Ordre d'évaluation des règles de franchissement. Les règles avec priorité basse (1-10) sont évaluées en premier.

**Plages de priorité :**
| Plage | Description | Exemples |
|-------|-------------|----------|
| 1-10 | Règles de sécurité critiques | Blocage hostile, rate limiting |
| 11-30 | Règles de niveau de confiance | Vérification TRUSTED, VERIFIED |
| 31-50 | Règles d'authentification | Session valide, MFA |
| 51-70 | Règles de données | Classification, types |
| 71-90 | Règles d'action | Lecture seule, impact |
| 91-100 | Règles temporelles et autres | Heures, quotas |

**Référence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) - Section 6

---

### R

#### Règle de franchissement

**Définition :** Condition déclarative qui doit être satisfaite pour qu'une interaction puisse traverser une frontière. Elle exprime ce qui est requis, pas comment le vérifier techniquement.

**Types de règles :**
| Type | Code | Description |
|------|------|-------------|
| Niveau de confiance | `niveau_confiance` | Niveau requis pour franchir |
| Authentification | `authentification` | État d'authentification requis |
| Données | `donnees` | Nature des données autorisées |
| Action | `action` | Actions autorisées |
| Temporel | `temporel` | Contraintes de temps |

**Invariant associé :** INV-BG-6 — Règles déclaratives

**Référence :** [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md)

---

### T

#### Traçabilité

**Définition :** Capacité à retracer l'origine, l'historique et la justification de toute définition dans Border Guard.

**Métadonnées obligatoires :**
| Métadonnée | Description |
|------------|-------------|
| Origine | Qui a créé/modifié l'élément |
| Date de création | Horodatage de création |
| Date de modification | Horodatage de dernière modification |
| Justification | Pourquoi cet élément existe |
| Historique | Journal des modifications |
| Version | Numéro de version |

**Invariant associé :** INV-BG-8 — Traçabilité complète

**Référence :** [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) - Section 6.2

---

#### Transition (de niveau de confiance)

**Définition :** Passage d'un niveau de confiance à un autre, soumis à des règles strictes.

**Règles de transition :**
| Règle | Description |
|-------|-------------|
| TRANS-1 | UNKNOWN → VERIFIED → TRUSTED (progression obligatoire) |
| TRANS-2 | * → HOSTILE (transition immédiate possible) |
| TRANS-3 | HOSTILE → UNKNOWN (réhabilitation obligatoire via UNKNOWN) |
| TRANS-4 | VERIFIED expiré → UNKNOWN (pas HOSTILE) |
| TRANS-5 | Toute transition est traçable |

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 6

---

#### Trusted (niveau de confiance)

**Définition :** Niveau de confiance indiquant une confiance totale — la source ou destination fait partie du cercle de confiance absolu.

| Aspect | Spécification |
|--------|---------------|
| Code | `TRUSTED` |
| Icône | 🟢 |
| Signification | Confiance absolue, cercle interne |
| Vérification | Aucune vérification supplémentaire requise |
| Restrictions | Aucune restriction par défaut |
| Révocabilité | Révocable (mais rare) |

**Exemples de sources Trusted :**
- StrongFather, KindMother, Border Guard (cores système)
- Modules internes certifiés du noyau

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.1

---

### U

#### Unknown (niveau de confiance)

**Définition :** Niveau de confiance indiquant une confiance inconnue — la source ou destination n'a pas encore été classifiée. C'est le niveau par défaut pour tout ce qui arrive de l'extérieur.

| Aspect | Spécification |
|--------|---------------|
| Code | `UNKNOWN` |
| Icône | 🟡 |
| Signification | Confiance non établie, prudence requise |
| Vérification | Vérifications systématiques requises |
| Restrictions | Règles restrictives par défaut |
| Évolution | Peut évoluer vers VERIFIED ou HOSTILE |

**Règle fondamentale :** "Unknown" n'est pas "hostile". C'est un état d'attente qui peut évoluer.

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.3

---

### V

#### Verified (niveau de confiance)

**Définition :** Niveau de confiance indiquant une confiance vérifiée — la source ou destination a été authentifiée et validée selon des critères stricts.

| Aspect | Spécification |
|--------|---------------|
| Code | `VERIFIED` |
| Icône | 🔵 |
| Signification | Confiance accordée après vérification |
| Vérification | Vérifications effectuées, résultat positif |
| Restrictions | Selon le contexte et les règles |
| Révocabilité | Révocable à tout moment |

**Exemples de sources Verified :**
- Utilisateur authentifié avec session valide
- API partenaire avec authentification valide
- Intégration Supabase avec credentials valides

**Référence :** [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) - Section 4.2

---

### Z

#### Zone de confiance

**Définition :** Espace conceptuel délimité par des frontières, où tous les éléments partagent un même niveau de confiance.

**Propriétés :**
| Propriété | Description |
|-----------|-------------|
| Identifiant | Identifiant unique de la zone |
| Niveau de confiance | Niveau homogène (trusted, verified, unknown, hostile) |
| Frontières | Liste des frontières délimitant la zone |
| Contenu | Composants, données, services contenus |

**Hiérarchie des zones :**
```
ZONE EXTERNE (hostile/unknown)
  └─ ZONE PÉRIPHÉRIQUE (unknown/verified)
      └─ ZONE UTILISATEUR (verified)
          └─ ZONE ADMIN (verified+)
              └─ ZONE SYSTÈME (trusted)
                  └─ ZONE CRITIQUE (trusted isolé)
```

**Référence :** [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) - Section 8

---

## Index alphabétique rapide

| Terme | Section |
|-------|---------|
| Action de franchissement | A |
| Classification | C |
| Cohérence globale | C |
| Condition déclarative | C |
| Contexte de frontière | C |
| Définition/Application | D |
| Franchissement | F |
| Frontière | F |
| Frontière d'intégration | F |
| Frontière externe | F |
| Frontière interne | F |
| Garantie | G |
| Gouvernance d'intégration | G |
| Hostile | H |
| Intégration | I |
| Invariant | I |
| Neutralité conceptuelle | N |
| Niveau de confiance | N |
| Perméabilité | P |
| Priorité | P |
| Règle de franchissement | R |
| Traçabilité | T |
| Transition | T |
| Trusted | T |
| Unknown | U |
| Verified | V |
| Zone de confiance | Z |

---

## Documents de référence

| Document | Relation |
|----------|----------|
| [Documentation Fondatrice](../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Définitions originales (Section 9) |
| [Boundary Definition Contract](../contracts/boundaries/Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | Termes de frontières |
| [Trust Level Classification Contract](../contracts/boundaries/Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Termes de niveaux de confiance |
| [Crossing Rules Contract](../contracts/boundaries/Border%20Guard%20-%20Crossing%20Rules%20Contract.md) | Termes de règles |
| [Invariants & Guarantees](../contracts/governance/Border%20Guard%20-%20Invariants%20&%20Guarantees.md) | Termes de gouvernance |
| [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Glossaire général Miyukini |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Document de référence — Glossaire canonique  
**Référence :** Border Guard v1.5, Documentation Fondatrice Section 9
