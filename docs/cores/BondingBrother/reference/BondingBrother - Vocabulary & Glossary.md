# BondingBrother - Vocabulary & Glossary

## 1. Contexte

Ce document Ã©tend et prÃ©cise le vocabulaire canonique introduit dans la Section 11 de la [Documentation Fondatrice](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md). Il Ã©tablit le dictionnaire complet et dÃ©finitif de tous les termes utilisÃ©s dans l'Ã©cosystÃ¨me Bonding Brother.

Les termes liÃ©s Ã  l'autonomie (isolement, Ã©tat local, synchronisation) sont dÃ©finis conformÃ©ment aux [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md).

**Navigation :** [Index BondingBrother](../_index.md)

## 2. PortÃ©e / Scope

Ce document couvre :
- Les termes fondamentaux hÃ©ritÃ©s du document fondateur
- Les termes architecturaux dÃ©rivÃ©s de la structure technique
- Les termes opÃ©rationnels utilisÃ©s dans les flux
- Les termes contractuels utilisÃ©s dans les spÃ©cifications

Ce document **Ã©tablit** :
- La dÃ©finition canonique et unique de chaque terme
- Les relations entre termes
- Les usages autorisÃ©s et interdits

---

## 3. RÃ¨gles terminologiques

### 3.1 RÃ¨gle d'unicitÃ©

Chaque concept a **un seul terme** autorisÃ©. Les synonymes sont interdits dans la documentation officielle.

### 3.2 RÃ¨gle de prÃ©cision

Chaque terme a **une seule dÃ©finition**. Aucune interprÃ©tation contextuelle n'est autorisÃ©e.

### 3.3 RÃ¨gle de stabilitÃ©

Les termes sont **versionnÃ©s** avec la documentation. Un terme ne peut changer de sens qu'avec un changement de version majeure.

### 3.4 RÃ¨gle d'usage

L'usage d'un terme non dÃ©fini dans ce glossaire est **interdit** dans la documentation contractuelle.

---

## 4. Termes fondamentaux

### 4.1 Intention

**DÃ©finition :** Expression structurÃ©e par un produit de sa volontÃ© d'effectuer une action dans l'Ã©cosystÃ¨me.

**CaractÃ©ristiques :**
- N'est pas une commande
- N'est pas une instruction d'exÃ©cution
- Est une dÃ©claration de volontÃ©
- Requiert une Ã©valuation par une autoritÃ©

**Forme canonique :**
```
Intention {
    type: TypeIntention,
    payload: DonnÃ©esSpÃ©cifiques,
    contexte: Contexte,
    origine: IdentitÃ©Produit
}
```

**Termes apparentÃ©s :**
- Demande (rÃ©sultat de la traduction d'une intention)
- RequÃªte (terme interdit â€” utiliser "intention" ou "demande")

---

### 4.2 AutoritÃ©

**DÃ©finition :** EntitÃ© qui dÃ©tient la vÃ©ritÃ© et prend les dÃ©cisions dans un domaine spÃ©cifique de l'Ã©cosystÃ¨me.

**CaractÃ©ristiques :**
- DÃ©tient une vÃ©ritÃ© non contestable
- Prend des dÃ©cisions non nÃ©gociables
- DÃ©finit des rÃ¨gles dans son domaine

**Types d'autoritÃ© dans l'Ã©cosystÃ¨me :**

| AutoritÃ© | Domaine | VÃ©ritÃ© dÃ©tenue |
|----------|---------|----------------|
| Kind Mother | DonnÃ©es | Ã‰tat des donnÃ©es, persistance, cohÃ©rence |
| Strong Father | IdentitÃ©s et Permissions | Qui peut faire quoi, rÃ¨gles politiques |

**Ce qui n'est pas une autoritÃ© :**
- Bonding Brother (mÃ©diateur, pas autoritÃ©)
- Les produits (consommateurs, pas autoritÃ©s)

---

### 4.3 Contexte

**DÃ©finition :** Ensemble des informations nÃ©cessaires Ã  l'Ã©valuation d'une intention par une autoritÃ©.

**Ã‰lÃ©ments obligatoires :**
- `produit_id` : IdentitÃ© du produit Ã©metteur
- `utilisateur_id` : IdentitÃ© de l'utilisateur (si applicable)
- `timestamp` : Moment de l'intention
- `session_id` : Identifiant de session (si applicable)

**Ã‰lÃ©ments facultatifs :**
- `permissions_dÃ©clarÃ©es` : Permissions revendiquÃ©es
- `environnement` : Contexte d'exÃ©cution (dev, staging, prod)
- `metadata` : Informations complÃ©mentaires

**RÃ¨gle :** Le contexte est toujours transmis intÃ©gralement aux autoritÃ©s. Bonding Brother ne filtre jamais le contexte en entrÃ©e.

---

### 4.4 Traduction

**DÃ©finition :** Transformation d'une structure (intention ou rÃ©ponse) d'un vocabulaire vers un autre, en prÃ©servant la sÃ©mantique.

**Deux directions :**

| Direction | EntrÃ©e | Sortie |
|-----------|--------|--------|
| Ascendante | Intention (vocabulaire produit) | Demande (vocabulaire autoritÃ©) |
| Descendante | RÃ©ponse (vocabulaire autoritÃ©) | RÃ©sultat (vocabulaire produit) |

**PropriÃ©tÃ©s de la traduction :**
- **FidÃ©litÃ©** : Le sens est prÃ©servÃ©
- **ComplÃ©tude** : Aucune information essentielle n'est perdue
- **PuretÃ©** : Aucun effet de bord, aucune dÃ©cision

**Ce que la traduction n'est pas :**
- Une validation (pas de dÃ©cision oui/non)
- Un filtrage (pas de suppression d'information)
- Une exÃ©cution (pas d'action mÃ©tier)

---

### 4.5 RÃ©sultat filtrÃ©

**DÃ©finition :** RÃ©ponse d'une autoritÃ© qui a Ã©tÃ© transformÃ©e pour Ãªtre consommable par un produit.

**Transformations appliquÃ©es :**
- Traduction vers le vocabulaire du produit
- Filtrage des informations non nÃ©cessaires
- Filtrage des informations non autorisÃ©es
- Adaptation du format

**Ce qu'un rÃ©sultat filtrÃ© contient :**
- L'information demandÃ©e (si autorisÃ©e)
- Le statut de l'opÃ©ration
- Les erreurs Ã©ventuelles (dans un format produit)

**Ce qu'un rÃ©sultat filtrÃ© ne contient jamais :**
- Les dÃ©tails internes de l'autoritÃ©
- Les informations d'autres produits
- Les donnÃ©es au-delÃ  du pÃ©rimÃ¨tre de la demande

---

### 4.6 DÃ©lÃ©gation

**DÃ©finition :** Acte par lequel Bonding Brother transmet une demande Ã  une autoritÃ© et attend sa dÃ©cision, sans jamais dÃ©cider lui-mÃªme.

**CaractÃ©ristiques :**
- Bonding Brother est l'Ã©metteur de la dÃ©lÃ©gation
- L'autoritÃ© est le rÃ©cepteur de la dÃ©lÃ©gation
- La dÃ©cision appartient exclusivement Ã  l'autoritÃ©
- Bonding Brother transmet fidÃ¨lement la dÃ©cision

**Ce que la dÃ©lÃ©gation implique :**
- Absence de dÃ©cision de la part de Bonding Brother
- Transmission fidÃ¨le de la demande
- Transmission fidÃ¨le de la rÃ©ponse
- TraÃ§abilitÃ© complÃ¨te

---

## 5. Termes architecturaux

### 5.1 Couche

**DÃ©finition :** Niveau d'abstraction dans l'architecture de Bonding Brother, avec une responsabilitÃ© unique et des interfaces dÃ©finies.

**Couches de Bonding Brother :**

| Couche | ResponsabilitÃ© | Position |
|--------|----------------|----------|
| Couche Produit | Interface vers les produits | Haute |
| Couche Traduction | Transformation des structures | IntermÃ©diaire haute |
| Couche MÃ©diation | Orchestration et filtrage | IntermÃ©diaire basse |
| Couche AutoritÃ© | Interface vers les autoritÃ©s | Basse |

**RÃ¨gle :** Une couche ne communique qu'avec ses couches adjacentes.

---

### 5.2 Composant

**DÃ©finition :** UnitÃ© fonctionnelle au sein d'une couche, avec une responsabilitÃ© atomique et une interface dÃ©finie.

**Exemples :**
- ProductGateway (composant de la Couche Produit)
- IntentTranslator (composant de la Couche Traduction)
- FilterEngine (composant de la Couche MÃ©diation)
- KindMotherAdapter (composant de la Couche AutoritÃ©)

**PropriÃ©tÃ©s d'un composant :**
- ResponsabilitÃ© unique
- Interface stable
- ImplÃ©mentation encapsulÃ©e
- IndÃ©pendance fonctionnelle

---

### 5.3 Adaptateur

**DÃ©finition :** Composant spÃ©cialisÃ© qui transforme les appels entre Bonding Brother et une entitÃ© externe (autoritÃ©).

**Adaptateurs de Bonding Brother :**
- `KindMotherAdapter` : Interface avec Kind Mother
- `StrongFatherAdapter` : Interface avec Strong Father

**RÃ´le de l'adaptateur :**
- Adapter les formats d'appel
- Normaliser les rÃ©ponses
- Isoler Bonding Brother des dÃ©tails de l'autoritÃ©

---

### 5.4 Gateway

**DÃ©finition :** Point d'entrÃ©e unique pour les interactions avec Bonding Brother depuis l'extÃ©rieur.

**ProductGateway :**
- Point d'entrÃ©e unique pour les produits
- Validation structurelle des intentions
- Routage vers les composants internes

**PropriÃ©tÃ©s d'une gateway :**
- Point unique (pas de contournement possible)
- Validation prÃ©coce
- TraÃ§abilitÃ© dÃ¨s l'entrÃ©e

---

## 6. Termes opÃ©rationnels

### 6.1 Flux

**DÃ©finition :** SÃ©quence ordonnÃ©e d'Ã©tapes par lesquelles une donnÃ©e (intention ou rÃ©ponse) transite dans Bonding Brother.

**Deux flux principaux :**

| Flux | Direction | DÃ©clencheur |
|------|-----------|-------------|
| Flux ascendant | Produit â†’ Ã‰cosystÃ¨me | Intention d'un produit |
| Flux descendant | Ã‰cosystÃ¨me â†’ Produit | Notification ou rÃ©ponse |

**PropriÃ©tÃ©s d'un flux :**
- Ordre strict des Ã©tapes
- Pas de saut d'Ã©tape
- TraÃ§abilitÃ© Ã  chaque Ã©tape

---

### 6.2 Filtrage

**DÃ©finition :** Application de rÃ¨gles pour supprimer ou masquer des informations non autorisÃ©es ou non nÃ©cessaires.

**Deux types de filtrage :**

| Type | Moment | Objectif |
|------|--------|----------|
| Filtrage d'entrÃ©e | Avant transmission Ã  l'autoritÃ© | ProtÃ©ger l'autoritÃ© des demandes invalides |
| Filtrage de sortie | Avant transmission au produit | ProtÃ©ger le produit des informations non autorisÃ©es |

**Ce que le filtrage ne fait pas :**
- Ne prend pas de dÃ©cision mÃ©tier
- Ne modifie pas la sÃ©mantique
- Ne remplace pas l'autoritÃ©

---

### 6.3 Journalisation

**DÃ©finition :** Enregistrement systÃ©matique et horodatÃ© de toutes les interactions transitant par Bonding Brother.

**Ã‰lÃ©ments journalisÃ©s :**
- Toute intention reÃ§ue
- Toute demande transmise
- Toute rÃ©ponse reÃ§ue
- Tout rÃ©sultat Ã©mis
- Toute erreur survenue

**PropriÃ©tÃ©s de la journalisation :**
- **SystÃ©matique** : Aucune exception
- **ComplÃ¨te** : Toutes les informations nÃ©cessaires Ã  la reconstitution
- **Non contournable** : Pas d'option pour dÃ©sactiver
- **Immuable** : Pas de modification aprÃ¨s enregistrement

---

### 6.4 Mode offline

**DÃ©finition :** Ã‰tat de fonctionnement de Bonding Brother lorsque les autoritÃ©s ne sont pas accessibles immÃ©diatement.

**Comportement en mode offline :**
- RÃ©ception des intentions : continue normalement
- Traduction : continue normalement
- Transmission aux autoritÃ©s : diffÃ©rÃ©e
- Journalisation : continue normalement (avec marqueur offline)

**Termes associÃ©s :**
- **Buffer offline** : Stockage temporaire des demandes en attente
- **AutoritÃ© diffÃ©rÃ©e** : AutoritÃ© qui sera consultÃ©e Ã  la reconnexion
- **Synchronisation** : Processus de transmission des demandes buffÃ©es

---

### 6.5 Synchronisation

**DÃ©finition :** Processus de transmission des intentions journalisÃ©es pendant le mode offline vers les autoritÃ©s une fois la connexion rÃ©tablie.

**Ã‰tapes de la synchronisation :**
1. DÃ©tection de la reconnexion
2. RÃ©cupÃ©ration des intentions buffÃ©es (ordre chronologique)
3. Transmission sÃ©quentielle aux autoritÃ©s
4. RÃ©ception des rÃ©ponses diffÃ©rÃ©es
5. Transmission des rÃ©sultats aux produits

**PropriÃ©tÃ©s :**
- Ordre prÃ©servÃ© (FIFO)
- Aucune perte d'intention
- TraÃ§abilitÃ© complÃ¨te

---

## 7. Termes contractuels

### 7.1 Contrat

**DÃ©finition :** Document normatif qui dÃ©finit les rÃ¨gles, interfaces, ou comportements que Bonding Brother s'engage Ã  respecter.

**Types de contrats :**

| Type | PortÃ©e | Exemple |
|------|--------|---------|
| Contrat d'interface | API et formats | Product Interface Contract |
| Contrat d'intÃ©gration | Interactions avec autoritÃ©s | KindMother Integration Contract |
| Contrat de comportement | Invariants et garanties | Invariants & Guarantees |

---

### 7.2 Invariant

**DÃ©finition :** PropriÃ©tÃ© qui doit toujours Ãªtre vraie, quelles que soient les circonstances, et qui ne peut jamais Ãªtre violÃ©e.

**Exemples d'invariants de Bonding Brother :**
- Bonding Brother ne dÃ©cide jamais
- Bonding Brother ne stocke jamais la vÃ©ritÃ©
- Bonding Brother journalise toujours

**PropriÃ©tÃ©s d'un invariant :**
- Non nÃ©gociable
- Non configurable
- Non dÃ©sactivable
- VÃ©rifiÃ© structurellement

---

### 7.3 Garantie

**DÃ©finition :** Engagement de Bonding Brother envers ses consommateurs (produits ou autoritÃ©s) sur un comportement ou une propriÃ©tÃ©.

**Exemples de garanties :**
- Traduction fidÃ¨le (la sÃ©mantique est prÃ©servÃ©e)
- Transmission complÃ¨te (rien n'est perdu)
- TraÃ§abilitÃ© totale (tout est enregistrÃ©)

**DiffÃ©rence avec l'invariant :**
- L'invariant est interne (Bonding Brother s'impose Ã  lui-mÃªme)
- La garantie est externe (Bonding Brother promet aux autres)

---

### 7.4 Violation

**DÃ©finition :** Situation oÃ¹ une rÃ¨gle, un invariant, ou un contrat n'est pas respectÃ©.

**Traitement des violations :**
- Violations d'invariant : Impossible par construction (erreur de conception si dÃ©tectÃ©e)
- Violations de contrat : Rejet de l'opÃ©ration, journalisation, notification

---

## 8. Termes interdits

Les termes suivants sont **interdits** dans la documentation de Bonding Brother car ils sont ambigus ou porteurs de mauvaises connotations :

| Terme interdit | Raison | Terme Ã  utiliser |
|----------------|--------|------------------|
| RequÃªte | Ambigu (query vs request) | Intention ou Demande |
| Ordre | Implique une exÃ©cution directe | Intention |
| Commande | Implique une exÃ©cution directe | Intention |
| Cache | Implique un stockage de vÃ©ritÃ© | Buffer ou Journal |
| Base de donnÃ©es | Bonding Brother ne stocke pas | Journal |
| DÃ©cision | Bonding Brother ne dÃ©cide pas | DÃ©lÃ©gation |
| RÃ¨gle (crÃ©Ã©e par BB) | BB ne crÃ©e pas de rÃ¨gles | RÃ¨gle (dÃ©finie par autoritÃ©) |
| Proxy | Trop technique, ambigu | MÃ©diateur |

---

## 9. Index alphabÃ©tique

| Terme | Section | CatÃ©gorie |
|-------|---------|-----------|
| Adaptateur | 5.3 | Architectural |
| AutoritÃ© | 4.2 | Fondamental |
| Composant | 5.2 | Architectural |
| Contrat | 7.1 | Contractuel |
| Contexte | 4.3 | Fondamental |
| Couche | 5.1 | Architectural |
| DÃ©lÃ©gation | 4.6 | Fondamental |
| Filtrage | 6.2 | OpÃ©rationnel |
| Flux | 6.1 | OpÃ©rationnel |
| Garantie | 7.3 | Contractuel |
| Gateway | 5.4 | Architectural |
| Intention | 4.1 | Fondamental |
| Invariant | 7.2 | Contractuel |
| Journalisation | 6.3 | OpÃ©rationnel |
| Mode offline | 6.4 | OpÃ©rationnel |
| RÃ©sultat filtrÃ© | 4.5 | Fondamental |
| Synchronisation | 6.5 | OpÃ©rationnel |
| Traduction | 4.4 | Fondamental |
| Violation | 7.4 | Contractuel |

---

## 10. Statut contractuel

Ce document est **contractuel, normatif, et de statut RÃ‰FÃ‰RENCE**. Il Ã©tablit le vocabulaire officiel de Bonding Brother qui doit Ãªtre utilisÃ© dans toute documentation, code, et communication.

Tout terme utilisÃ© dans un document contractuel de Bonding Brother doit Ãªtre dÃ©fini dans ce glossaire. Toute modification terminologique nÃ©cessite une nouvelle version de ce document.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** RÃ‰FÃ‰RENCE â€” Normatif  
**DÃ©pendance :** Documentation Fondatrice v2.0 (Section 11)

