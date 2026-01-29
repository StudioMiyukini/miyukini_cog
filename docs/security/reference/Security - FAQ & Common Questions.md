# Miyukini Security — FAQ & Common Questions

## 1. Contexte

Ce document repond aux **questions frequemment posees** concernant la securite Miyukini : concepts fondamentaux, mecanismes de protection, niveaux de securite, cas limites et clarifications conceptuelles.

**Objectif :**

> **"Clarifier, demystifier et guider la comprehension de la securite Miyukini."**

Ce document est destine aux developpeurs, architectes, operateurs et toute personne souhaitant comprendre le fonctionnement de la securite dans l'ecosysteme Miyukini.

## 2. Portee / Scope

Ce document repond aux questions sur :
- Les concepts fondamentaux de securite
- Les niveaux de securite (0-4) et de confiance (T0-T4)
- Les Security Engines et leur fonctionnement
- Les invariants et les lois systeme
- La chaine de confiance
- La gouvernance humaine
- Les cas limites et situations particulieres

Ce document **ne couvre pas** :
- Les details d'implementation technique
- Les protocoles cryptographiques specifiques
- Les configurations systeme detaillees

---

## 3. Questions Conceptuelles Fondamentales

### Q1 : Pourquoi dit-on que la securite est une "propriete structurelle" ?

**Reponse :**

Dans Miyukini, la securite n'est pas un module ajoute, une bibliotheque importee ou un service appele. Elle **emerge de l'architecture elle-meme**. Cela signifie que :

- Chaque composant est concu avec la securite en tete
- Les flux entre strates sont obligatoirement securises
- Les controles sont integres, pas optionnels
- Contourner la securite revient a casser l'architecture

**Analogie :** Dans un batiment, la structure porteuse n'est pas une option. Sans elle, le batiment s'effondre. De meme, sans la securite structurelle, le systeme Miyukini ne peut pas exister.

**Reference :** [Doctrine Securite Fondamentale - Section 3](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

### Q2 : Quelle est la difference entre un niveau de securite (0-4) et un niveau de confiance (T0-T4) ?

**Reponse :**

Ces deux systemes sont **independants et complementaires** :

| Aspect | Niveaux de Securite (0-4) | Niveaux de Confiance (T0-T4) |
|--------|---------------------------|------------------------------|
| **Quoi** | Profil de risque de l'Operateur | Etat d'integrite du systeme |
| **Qui decide** | L'Operateur declare | Le systeme evalue |
| **Quand** | A la configuration | En temps reel |
| **Consequence** | Comportement des Cores | Capacites disponibles |

**Exemple concret :**
- Un Operateur niveau 2 (Sensitive Data) en T0 (Normal) → Fonctionnement normal avec protection des donnees
- Un Operateur niveau 2 en T2 (Degrade) → Restrictions supplementaires, fonctions non essentielles desactivees

**Reference :** [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md), [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md)

---

### Q3 : Qu'est-ce que le STA et l'OSV exactement ?

**Reponse :**

**STA (System Truth Anchor)** — C'est le **porteur de verite officiel** du systeme :
- Contient les empreintes MSCM, MIP, Graph checksums
- Represente l'etat certifie actuel
- Sert de reference pour toute validation

**OSV (Official Secure Version)** — C'est la **version officielle sure** :
- Snapshot certifie du systeme
- Validee, auditee, signee, figee
- Restaurable en cas de besoin

**Relation :**
- Le STA est l'etat de verite **courant**
- L'OSV est une **version figee** de cet etat
- Toute version non OSV est consideree comme non certifiee

**Analogie :** Le STA est comme le registre d'etat civil (verite actuelle), l'OSV est comme une constitution signee (version officielle de reference).

**Reference :** [Doctrine Securite Fondamentale - Section 6](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

### Q4 : Pourquoi la securite Miyukini protege-t-elle la "cognition" ?

**Reponse :**

La securite Miyukini protege 4 domaines : **Verite, Structure, Memoire et Cognition**.

La protection cognitive est essentielle car :

1. **Les agents IA sont des vecteurs potentiels de derive** — Ils peuvent prendre des decisions erronees si non contraints
2. **Les feedback loops peuvent s'emballer** — Sans limite, une IA peut s'auto-amplifier vers des etats indesirables
3. **Le consensus peut etre simule** — Plusieurs IA alimentees par la meme source donnent une fausse impression de validation croisee

**Mecanismes de protection :**
- Cognitive Guard surveille les decisions IA
- Multi-agents contradictoires detectent les biais
- Seuils de confiance limitent les actions autonomes

**Reference :** [Doctrine Securite Fondamentale - Section 8.4](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

## 4. Questions sur les Niveaux de Securite

### Q5 : Peut-on changer de niveau de securite dynamiquement ?

**Reponse :**

**Non, le niveau de securite est declare a la configuration.** C'est un parametre de gouvernance, pas un choix applicatif.

**Raison :** Si un Operateur pouvait changer son niveau de securite a la volee, cela ouvrirait des failles :
- Un attaquant pourrait baisser le niveau pour contourner les controles
- Les garanties de securite seraient imprevisibles
- L'audit serait compromis

**Ce qui peut changer dynamiquement :** Le niveau de confiance (T0-T4), car il represente l'etat reel du systeme, pas une declaration.

---

### Q6 : Quel niveau de securite choisir pour mon application ?

**Reponse :**

| Type d'Application | Niveau Recommande |
|--------------------|-------------------|
| Site vitrine, affichage public | **Niveau 0** |
| CMS, backoffice simple | **Niveau 1** |
| Donnees personnelles, profils utilisateurs | **Niveau 2** |
| Auth, paiement, decisions critiques | **Niveau 3** |
| Environnement isole, infra critique | **Niveau 4** |

**Regle d'or :** Choisir le niveau minimal qui couvre les risques reels. Un niveau trop eleve impacte les performances sans benefice.

**Reference :** [Security Levels - Section 4](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)

---

### Q7 : Que se passe-t-il si je declare un niveau de securite trop bas ?

**Reponse :**

Si le niveau declare est insuffisant pour le type de donnees traitees :

1. **Border Guard peut refuser** — Si les donnees entrantes necessitent une protection superieure
2. **Les garanties ne sont pas fournies** — Pas d'auth renforcee, pas de signatures d'intentions au niveau 0-1
3. **Responsabilite operateur** — L'Operateur assume la responsabilite de la declaration

**Conseil :** En cas de doute, choisir le niveau superieur. L'impact performance est proportionnel au risque.

---

## 5. Questions sur les Security Engines

### Q8 : Les 8 Security Engines sont-ils tous obligatoires ?

**Reponse :**

**Oui, tous les engines sont actifs.** Ils constituent une strate d'infrastructure systemique obligatoire.

| Engine | Toujours Actif | Intensite Variable |
|--------|----------------|-------------------|
| Integrity Engine | ✅ | Selon niveau securite |
| Validation Engine | ✅ | Selon niveau securite |
| Policy Engine | ✅ | Selon regles |
| Consensus Engine | ✅ | Selon criticite decisions |
| Audit Engine | ✅ | Selon niveau securite |
| Sandbox Engine | ✅ | Selon risque |
| Cognitive Guard | ✅ | Selon presence IA |
| Recovery Engine | ✅ | Toujours pret |

Ce qui varie est l'**intensite** des controles, pas leur presence.

**Reference :** [Security - Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md)

---

### Q9 : Comment Cognitive Guard detecte-t-il une derive IA ?

**Reponse :**

Cognitive Guard utilise plusieurs mecanismes :

1. **Detection de biais** — Analyse statistique des decisions
2. **Anti-feedback-loop** — Detection des boucles d'amplification
3. **Multi-agents contradictoires** — Confrontation de perspectives
4. **Seuils de confiance** — Limites sur les actions autonomes
5. **Surveillance comportementale** — Detection des anomalies de pattern

**Declencheurs d'alerte :**
- Decisions repetitives sans variation
- Decisions qui s'ecartent des normes historiques
- Absence de contradiction dans un contexte qui en necessite
- Depassement des seuils de confiance

---

### Q10 : Que fait Recovery Engine en cas de blocage T4 ?

**Reponse :**

En mode T4 (Bloque), Recovery Engine :

1. **Preserve l'etat** — Aucune action ne peut degrader davantage
2. **Active les diagnostics** — Permet l'analyse de la situation
3. **Prepare la restauration** — Identifie les options de rollback
4. **Attend l'intervention humaine** — TAMR doit valider toute action

**Capacites en T4 :**
- ✅ Lecture des logs et diagnostics
- ✅ Preparation de restauration OSV
- ❌ Aucune decision operationnelle
- ❌ Aucune ecriture non supervisee

**Garantie absolue :** Jamais de corruption, jamais d'execution sauvage en T4.

---

## 6. Questions sur les Invariants et Lois

### Q11 : Que se passe-t-il exactement si je viole la loi L3 (bypass des Cores) ?

**Reponse :**

Une violation de L3 declenche la sequence suivante :

1. **Detection immediate** — Validation Engine detecte le flux non conforme
2. **Invalidation de l'operation** — L'action n'est pas executee
3. **Transition vers T3** — Niveau de confiance restreint
4. **Blocage du composant fautif** — Isolation pour eviter propagation
5. **Journalisation** — Trace complete de l'incident
6. **Audit** — Analyse de tous les flux du composant

**Gravite :** V4 (Critique)

**Remediation :** Identifier le bypass, bloquer, retablir le flux correct, audit de conformite.

**Reference :** [Violations & Anti-Patterns - Section 4.3](../contracts/governance/Security%20-%20Violations%20&%20Anti-Patterns.md)

---

### Q12 : La loi L2 (source de verite unique) empeche-t-elle le cache ?

**Reponse :**

**Non, le cache est autorise tant qu'il ne devient pas source de verite.**

**Cache conforme :**
- ✅ Cache qui accelere l'acces aux donnees STA
- ✅ Cache invalide quand la source change
- ✅ Cache qui ne repond pas si STA indisponible

**Cache non conforme (viole L2) :**
- ❌ Cache qui devient source primaire si STA tombe
- ❌ Cache qui diverge de STA sans synchronisation
- ❌ Cache qui persiste des decisions comme verite

**Regle :** Le cache est un miroir, jamais une source.

---

### Q13 : Comment garantir L4 (tracabilite) dans un contexte haute performance ?

**Reponse :**

Plusieurs strategies permettent de maintenir la tracabilite sans degrader les performances :

1. **Journalisation asynchrone** — Ecriture dans un buffer, persistance en arriere-plan
2. **Batching** — Regroupement des traces avant ecriture
3. **Compression** — Traces compressees pour stockage
4. **Niveaux de detail** — Plus de details pour niveau securite eleve

**Ce qui est toujours trace :**
- Decisions critiques (synchrone)
- Modifications de donnees (synchrone)
- Changements de configuration (synchrone)

**Ce qui peut etre asynchrone :**
- Lectures (niveau 0-1)
- Metriques de performance
- Logs de debug

**Invariant :** Meme asynchrone, aucune trace n'est jamais perdue.

---

## 7. Questions sur la Gouvernance Humaine

### Q14 : Quand TAMR intervient-il automatiquement ?

**Reponse :**

TAMR est notifie ou requis dans les cas suivants :

| Situation | Notification | Intervention Requise |
|-----------|--------------|---------------------|
| Violation V1 (Mineure) | Non | Non |
| Violation V2 (Significative) | Si recidive | Non |
| Violation V3 (Majeure) | Oui (sous 1h) | Non (sauf escalade) |
| Violation V4 (Critique) | Immediate | Obligatoire |
| Transition vers T4 | Immediate | Obligatoire |
| Conflit non resolu | Oui | Arbitrage |
| Override de decision critique | Apres action | Validation |

**Principe :** L'humain intervient pour les decisions que le systeme ne peut pas prendre seul.

---

### Q15 : L'humain peut-il faire des erreurs ? Comment le systeme gere-t-il cela ?

**Reponse :**

**Oui, l'humain est explicitement reconnu comme surface d'attaque potentielle.**

Les risques humains :
- Social engineering
- Erreur involontaire
- Malveillance interne

**Mecanismes de protection :**

1. **Tracabilite des interventions TAMR** — Toute action humaine est journalisee
2. **Double validation** — Pour les actions critiques
3. **Audit des overrides** — Revue des decisions humaines
4. **Contraintes temporelles** — Delais de reflexion pour decisions majeures
5. **Separation des pouvoirs** — Un humain ne peut pas tout faire seul

**Principe :** L'humain gouverne, mais sous contraintes de tracabilite et de validation.

---

### Q16 : Peut-on desactiver la supervision humaine pour un deploiement totalement automatise ?

**Reponse :**

**Non, la supervision humaine est un invariant systeme (G1).**

Ce qui est possible :
- ✅ Automatiser les operations normales
- ✅ Reduire les interventions humaines au minimum
- ✅ Deleguer les decisions non critiques

Ce qui reste humain :
- ❌ Validation des OSV
- ❌ Arbitrage des conflits
- ❌ Override des blocages T4
- ❌ Decisions de rollback critique

**Raison :** L'humain est le dernier recours, l'arbitre final. Sans lui, le systeme perd sa source ultime de legitimite.

---

## 8. Questions sur les Cas Limites

### Q17 : Que se passe-t-il en mode offline / deconnecte ?

**Reponse :**

En mode offline, le systeme s'adapte automatiquement :

**Mode declare (ENV_MODE = CLOSED) :**
- Validation interne renforcee
- STA et OSV locales
- Consensus interne
- Audits locaux

**Contraintes :**
- Pas de decision finale cote mobile/offline
- Actions non engagees tant que deconnecte
- Revalidation complete a la reconnexion

**A la reconnexion (ENV_MODE = RECONNECTING) :**
1. Auto-diagnostic
2. Verification integrite locale
3. Comparaison STA local vs distant
4. Recertification dynamique
5. Reintegration dans le mesh

**Reference :** [Doctrine Securite Fondamentale - Section 13](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

### Q18 : Deux instances Miyukini de versions differentes peuvent-elles communiquer ?

**Reponse :**

**Non directement.** Les versions de cores sont des **dogmes incompatibles**.

```
CoreSet v5 ≠ CoreSet v6
```

**Consequences :**
- Pas de confiance automatique entre versions
- Pas d'interop directe
- Necessite de passerelles de traduction
- Certification conditionnelle

**Protocole de communication inter-versions :**
1. Identification des versions
2. Detection d'incompatibilite
3. Si passerelle disponible → traduction
4. Si pas de passerelle → isolement mutuel

**Reference :** [Doctrine Securite Fondamentale - Section 12.5](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

### Q19 : Que faire si le systeme reste bloque en T4 apres intervention ?

**Reponse :**

Si le systeme ne peut pas sortir de T4 apres intervention TAMR :

**Procedure :**

1. **Analyse forensique** — Comprendre pourquoi le retour echoue
2. **Evaluation de l'etendue** — Identifier les composants corrompus
3. **Decision de restauration** — Rollback vers OSV anterieure
4. **Reconstruction** — Depuis OSV vers etat souhaite
5. **Recertification** — Nouveau STA, nouvelle OSV

**Options extremes :**
- Restauration complete depuis OSV
- Reconstruction partielle avec donnees recuperees
- Nouveau deploiement (cas ultime)

**Garantie :** Meme en cas d'echec de remediation, les donnees OSV sont toujours restaurables.

---

### Q20 : Un Core peut-il tomber en panne ? Que se passe-t-il alors ?

**Reponse :**

**Oui, un Core peut devenir indisponible.** Le systeme reagit par degradation progressive.

| Core Indisponible | Impact | Reaction Systeme |
|-------------------|--------|------------------|
| StrongFather | Aucune decision possible | T3 → T4 rapide |
| KindMother | Pas de persistance | Gel des ecritures |
| Border Guard | Pas de classification | Rejet externe |
| Caring Nanny | Pas de monitoring | Surveillance aveugle |
| BondingBrother | Pas de mediation | Isolation produits |

**Mecanismes de resilience :**
- Recovery Engine detecte l'absence
- Caring Nanny alerte
- Degradation progressive (pas de blocage brutal)
- Attente de restauration ou intervention

**Prevention :**
- Redondance des Cores critiques
- Healthchecks continus
- Fallback automatique si possible

---

## 9. Questions Pratiques pour Developpeurs

### Q21 : Comment savoir si mon code viole un invariant ?

**Reponse :**

**Methodes de detection :**

1. **Validation MSCM** — Le code doit etre conforme aux balises MSCM
2. **Tests d'invariants** — Executes a chaque deploiement
3. **Sondes comportementales** — Detection des patterns suspects
4. **Revue architecturale** — A chaque changement majeur

**Indicateurs d'alerte dans les logs :**
- `[INVARIANT_VIOLATION]` — Violation detectee
- `[CHAIN_BREAK]` — Rupture de chaine de confiance
- `[LAW_VIOLATION]` — Loi systeme violee

**Conseil :** Utiliser les outils de validation MSCM avant tout commit.

---

### Q22 : Comment integrer un nouveau composant dans la chaine de confiance ?

**Reponse :**

**Etapes obligatoires :**

1. **Indexation MIP** — Creer l'entree dans le MIP
2. **Conformite MSCM** — Baliser le code selon MSCM
3. **Declaration des dependances** — Dans le Graph
4. **Definition des capacites** — Via Master Butler
5. **Classification externe** — Si interactions externes, via Border Guard
6. **Tests d'integration** — Valider la conformite
7. **Certification** — Validation par Integrity Engine

**Checklist de conformite :**
- [ ] Entree MIP creee
- [ ] Balises MSCM presentes
- [ ] Dependances declarees
- [ ] Capacites definies
- [ ] Classification effectuee (si externe)
- [ ] Tests passes
- [ ] Audit de conformite valide

---

### Q23 : Puis-je creer un Operateur qui ne respecte pas les niveaux de securite ?

**Reponse :**

**Non.** Les niveaux de securite sont gouvernes par les Cores, pas par les Operateurs.

**Ce que l'Operateur declare :**
- Son profil de risque (niveau 0-4)
- Ses besoins (offline, degradation, etc.)

**Ce que l'Operateur ne peut pas faire :**
- Implementer sa propre securite
- Contourner les controles du niveau declare
- Changer de niveau dynamiquement
- Ignorer les regles de Border Guard

**Consequence d'une tentative :** Violation de L3 (bypass des Cores), gravite V4.

---

## 10. Questions de Clarification Terminologique

### Q24 : Quelle est la difference entre "invariant" et "garantie" ?

**Reponse :**

| Concept | Definition | Exemple |
|---------|------------|---------|
| **Invariant** | Propriete toujours vraie | "StrongFather ne persiste jamais" |
| **Garantie** | Ce que le systeme fournit | "Niveau 2 : signatures d'intentions" |

**Relation :**
- Les invariants sont les **regles du systeme**
- Les garanties sont les **benefices fournis** grace au respect des invariants

**Formulation :** "Les lois sont absolues. Les contraintes sont universelles. Les garanties sont contextuelles."

---

### Q25 : Qu'est-ce qu'un "postulat" dans la doctrine securite ?

**Reponse :**

Les postulats sont les **axiomes fondamentaux** sur lesquels repose toute la doctrine :

| Postulat | Signification |
|----------|---------------|
| **P1** | Les vulnerabilites sont aux interfaces, pas au coeur |
| **P2** | La securite technique ne suffit pas sans securite structurelle |
| **P3** | La securite du code ne suffit pas sans securite cognitive |
| **P4** | La protection perimetrique ne suffit pas sans protection de la verite |
| **P5** | La securite emerge de l'architecture |

**Difference avec les lois :**
- Les **postulats** sont des principes philosophiques
- Les **lois** sont des regles operationnelles

---

### Q26 : Que signifie "Zero-Trust" dans le contexte Miyukini ?

**Reponse :**

Dans Miyukini, Zero-Trust signifie :

**Principe :** Ne jamais presupposer la validite de quoi que ce soit.

**Application concrete :**
- Tout appelant est verifie (meme interne)
- Toute donnee est validee
- Toute decision est evaluee
- Toute action est tracee

**Niveaux d'application :**
- Niveau 0-1 : Zero-trust assoupli
- Niveau 2 : Zero-trust standard
- **Niveau 3-4 : Zero-trust strict**

**A partir du niveau 3 :** Verifications croisees obligatoires, aucune confiance accordee sans preuve.

---

## 11. Documentation Associee

### Documents de Reference Conceptuels

| Document | Contenu |
|----------|---------|
| [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md) | Fondation philosophique et architecturale |
| [Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) | Niveaux de securite (0-4) |
| [Integrity Degradation System](../../reference/Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md) | Niveaux de confiance (T0-T4) |
| [Security Protocols](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md) | Protocoles temps reel et asynchrone |

### Documents Operationnels (docs/security)

| Document | Contenu |
|----------|---------|
| [Documentation Fondatrice](../foundation/Security%20-%20Documentation%20Fondatrice.md) | Vision operationnelle |
| [Architecture & Components](../architecture/Security%20-%20Architecture%20&%20Components.md) | Security Engines |
| [Invariants & Guarantees](../contracts/governance/Security%20-%20Invariants%20&%20Guarantees.md) | Lois et garanties |
| [Violations & Anti-Patterns](../contracts/governance/Security%20-%20Violations%20&%20Anti-Patterns.md) | Violations et remediation |

### Autres Documents de Reference

| Document | Contenu |
|----------|---------|
| [Vocabulary & Glossary](./Security%20-%20Vocabulary%20&%20Glossary.md) | Definitions des termes |
| [Examples & Use Cases](./Security%20-%20Examples%20&%20Use%20Cases.md) | Scenarios concrets |

---

## 12. Synthese

### Points Cles a Retenir

1. **La securite est structurelle** — Elle emerge de l'architecture, pas d'un module
2. **Niveaux de securite (0-4)** — Profil de risque declare par l'Operateur
3. **Niveaux de confiance (T0-T4)** — Etat d'integrite evalue par le systeme
4. **Les 8 Security Engines** — Toujours actifs, intensite variable
5. **Les 6 Lois (L1-L6)** — Absolues et non negociables
6. **La gouvernance humaine** — Dernier recours, arbitre final
7. **Zero-Trust** — Ne jamais presupposer la validite

### Questions Non Couvertes Ici

Si vous avez des questions sur :
- **Implementation technique** → Voir [Reference Implementation Guidelines](../implementation/Security%20-%20Reference%20Implementation%20Guidelines.md)
- **Procedures operationnelles** → Voir [Operational Runbook](../operations/Security%20-%20Operational%20Runbook.md)
- **Menaces et surfaces d'attaque** → Voir [Threat Model Summary](../operations/Security%20-%20Threat%20Model%20Summary.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0  
**Statut :** REFERENCE — Document de reference operationnel  
**Reference :** [Doctrine Securite Fondamentale](../../reference/Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)

---

## 13. Mini Log de Generation

### Decisions structurantes

- Organisation en 10 categories thematiques de questions
- Questions numerotees pour reference facile (Q1-Q26)
- Chaque reponse inclut une reference vers la documentation source
- Format FAQ standard avec question/reponse claire
- Cas limites inclus (offline, inter-versions, blocage T4)

### Sources utilisees

- Doctrine Securite Fondamentale : Concepts fondamentaux, postulats, lois
- Security Levels : Questions sur les niveaux de securite
- Integrity Degradation System : Questions sur les niveaux de confiance
- Invariants & Guarantees : Questions sur les lois et garanties
- Violations & Anti-Patterns : Questions sur les consequences

### Verification de coherence

- ✅ Coherence avec la Doctrine Securite Fondamentale
- ✅ Coherence avec les documents de docs/security
- ✅ References correctes vers docs/reference
- ✅ Structure conforme au plan de documentation
- ✅ Questions couvrant les cas limites

**Aucune contradiction detectee.**
