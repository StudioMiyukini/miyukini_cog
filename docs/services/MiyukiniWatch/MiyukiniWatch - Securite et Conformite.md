# MiyukiniWatch â€” SÃ©curitÃ© et ConformitÃ©

## Contexte

**MiyukiniWatch** manipule des donnÃ©es d'usage personnelles (habitudes, sessions, amis contactÃ©s). Bien que ces donnÃ©es ne contiennent jamais de contenu (messages, textes, fichiers), elles constituent des **mÃ©tadonnÃ©es comportementales** qui requiÃ¨rent un niveau de protection appropriÃ©. Ce document dÃ©finit les exigences de sÃ©curitÃ©, la classification des donnÃ©es, le chiffrement au repos, les contrÃ´les d'accÃ¨s et la conformitÃ© vis-Ã -vis des Cores de gouvernance.

## PortÃ©e / Scope

- **Applicable Ã  :** SÃ©curitÃ© des donnÃ©es, classification, chiffrement, contrÃ´les d'accÃ¨s, audit sÃ©curitÃ©.
- **Audience :** Ã‰quipes sÃ©curitÃ©, architectes, dÃ©veloppeurs, auditeurs.
- **Statut :** Document normatif â€” rÃ©fÃ©rence sÃ©curitÃ© du Service MiyukiniWatch.

---

## 1. Classification des donnÃ©es

### 1.1 Niveau de sÃ©curitÃ©

ConformÃ©ment aux niveaux de sÃ©curitÃ© Miyukini (0â€“4), les donnÃ©es MiyukiniWatch sont classifiÃ©es :

| DonnÃ©es | Niveau | Justification |
|---------|--------|---------------|
| MÃ©triques de sessions (horodatages, durÃ©es) | **1 â€” Standard** | DonnÃ©es d'usage personnelles, non sensibles au sens strict, mais privÃ©es. |
| MÃ©triques de services (quels services, frÃ©quence) | **1 â€” Standard** | Habitudes d'usage, pas de contenu. |
| MÃ©triques d'amis (identifiants de contacts, timing) | **2 â€” Sensitive** | MÃ©tadonnÃ©es de relations sociales ; plus sensibles que des donnÃ©es d'usage pure. |
| AgrÃ©gats exposÃ©s Ã  Miou | **1 â€” Standard** | RÃ©sumÃ©s anonymisÃ©s, moins granulaires que les donnÃ©es brutes. |
| Journal d'audit | **1 â€” Standard** | Historique des actions (effacements, dÃ©sactivations). |
| Compteurs globaux | **0 â€” Public** | Informations non identifiantes (nombre total de sessions). |

### 1.2 Matrice de classification

| Dimension | Contenu | Niveau |
|-----------|---------|--------|
| **Quand** | Horodatages, durÃ©es | 1 â€” Standard |
| **OÃ¹** | Identifiants de services, onglets | 1 â€” Standard |
| **Qui** | Identifiants techniques de contacts | 2 â€” Sensitive |
| **Combien** | Compteurs, agrÃ©gats numÃ©riques | 0â€“1 |

---

## 2. Chiffrement

### 2.1 Chiffrement au repos

| Exigence | Description |
|----------|-------------|
| **Toutes les donnÃ©es MiyukiniWatch sont chiffrÃ©es au repos.** | Le stockage KindMother utilise le chiffrement par dÃ©faut de la couche de persistance du COG. |
| **ClÃ© de chiffrement** | GÃ©rÃ©e par WorrySentinel / KindMother. LiÃ©e au profil utilisateur. |
| **Algorithme** | DÃ©fini par la politique de sÃ©curitÃ© du COG (spÃ©cification WorrySentinel). |

### 2.2 Chiffrement en transit

| Exigence | Description |
|----------|-------------|
| **Non applicable pour le rÃ©seau** | MiyukiniWatch ne communique pas sur le rÃ©seau. Aucun chiffrement rÃ©seau requis. |
| **Communications internes** | Les Ã©changes entre OpÃ©rateurs, BondingBrother et Cores sont des appels internes au processus. Si une architecture multi-processus est utilisÃ©e, les IPC sont sÃ©curisÃ©s par le Kernel. |

---

## 3. ContrÃ´les d'accÃ¨s

### 3.1 Matrice d'accÃ¨s

| Acteur | Lecture mÃ©triques | Ã‰criture mÃ©triques | Effacement | Lecture agrÃ©gats | Configuration |
|--------|-------------------|-------------------|------------|-----------------|---------------|
| **Utilisateur (via MiyukiniWatch UI)** | Oui | Non (automatique) | Oui | Oui | Oui |
| **MiyukiniWatchCollector** | Non | Oui (WriteIntent) | Non | Non | Non |
| **MiyukiniWatchAggregator** | Oui (ReadIntent) | Oui (agrÃ©gats) | Oui (purge) | Non | Non |
| **MiyukiniWatchPresenter** | Oui (ReadIntent) | Non | Oui (DeleteIntent) | Non | Non |
| **Miou** | Non | Non | Non | Oui (lecture agrÃ©gats) | Non |
| **Autres services** | Non | Non | Non | Non | Non |
| **Autres COGs** | Non | Non | Non | Non | Non |
| **MWS (relays, trackers)** | Non | Non | Non | Non | Non |

### 3.2 Gouvernance des accÃ¨s

Chaque accÃ¨s est gouvernÃ© par les Cores via BondingBrother :

| Core | RÃ´le dans le contrÃ´le d'accÃ¨s |
|------|-------------------------------|
| **Master Butler** | Maintient le registre des permissions. VÃ©rifie que l'acteur a le droit d'effectuer l'opÃ©ration demandÃ©e. |
| **StrongFather** | Ã‰met les Mandats de Permission pour les opÃ©rations sensibles (effacement, purge, changement de configuration). |
| **Border Guard** | Bloque toute tentative d'accÃ¨s aux donnÃ©es MiyukiniWatch depuis l'extÃ©rieur du COG. |
| **WorrySentinel** | DÃ©finit le niveau de sÃ©curitÃ© requis pour chaque type de donnÃ©es et vÃ©rifie la conformitÃ©. |

### 3.3 Isolation par profil

| RÃ¨gle | Description |
|-------|-------------|
| **Un profil = un jeu de donnÃ©es** | Chaque profil utilisateur a ses propres donnÃ©es MiyukiniWatch, isolÃ©es des autres. |
| **Pas de cross-profil** | Un profil ne peut pas accÃ©der aux mÃ©triques d'un autre profil, mÃªme sur le mÃªme COG. |
| **Session authentifiÃ©e** | L'accÃ¨s aux donnÃ©es MiyukiniWatch requiert une session Central authentifiÃ©e. |

---

## 4. Ã‰tats de confiance et comportement

ConformÃ©ment aux Ã©tats de confiance Miyukini (T0â€“T4), MiyukiniWatch adapte son comportement :

| Ã‰tat | Nom | Comportement MiyukiniWatch |
|------|-----|---------------------------|
| **T0** | Normal | Collecte active. AgrÃ©gation normale. Interface complÃ¨te. |
| **T1** | Instable | Collecte active. AgrÃ©gation peut Ãªtre retardÃ©e. |
| **T2** | DÃ©gradÃ© | Collecte rÃ©duite (uniquement sessions et services, pas de clics). AgrÃ©gation reportÃ©e. |
| **T3** | Restreint | Collecte suspendue. DonnÃ©es existantes consultables. Pas de nouvelle Ã©criture. |
| **T4** | BloquÃ© | MiyukiniWatch inaccessible. DonnÃ©es prÃ©servÃ©es mais non consultables tant que le COG est en T4. |

### 4.1 Transition et recovery

| Transition | Action MiyukiniWatch |
|------------|---------------------|
| T0 â†’ T2 | Caring Nanny signale la dÃ©gradation ; Collector rÃ©duit sa collecte. |
| T2 â†’ T0 | Retour Ã  la collecte complÃ¨te. Les donnÃ©es de la pÃ©riode dÃ©gradÃ©e sont marquÃ©es comme potentiellement incomplÃ¨tes. |
| T3 â†’ T0 | Reprise de la collecte. Un Ã©vÃ©nement d'audit Â« collecte reprise aprÃ¨s restriction T3 Â» est enregistrÃ©. |
| T4 â†’ T0 | Reprise complÃ¨te. Audit de l'intÃ©gritÃ© des donnÃ©es. |

---

## 5. Menaces et contre-mesures

### 5.1 ModÃ¨le de menaces

| Menace | ScÃ©nario | Contre-mesure |
|--------|----------|---------------|
| **Exfiltration de donnÃ©es** | Un service malveillant tente de lire les donnÃ©es MiyukiniWatch. | Border Guard bloque tout accÃ¨s externe. Master Butler refuse les ReadIntent non autorisÃ©s. |
| **Injection de mÃ©triques** | Un service tente d'Ã©crire de fausses mÃ©triques. | Seul le MiyukiniWatchCollector a le droit de WriteIntent. StrongFather vÃ©rifie l'origine. |
| **AccÃ¨s non autorisÃ©** | Un autre profil tente de lire les mÃ©triques. | Isolation par profil. Session authentifiÃ©e requise. |
| **TÃ©lÃ©mÃ©trie cachÃ©e** | Un composant tente d'envoyer des mÃ©triques sur le rÃ©seau. | Border Guard bloque. Aucune capability rÃ©seau n'est accordÃ©e Ã  MiyukiniWatch. |
| **Perte de donnÃ©es** | Crash ou corruption pendant l'Ã©criture. | Ã‰criture atomique via KindMother. En cas d'Ã©chec, la mÃ©trique est perdue (acceptable : ce n'est pas une donnÃ©e critique). |
| **Surcharge stockage** | Collecte excessive saturant le stockage. | Limites de volumÃ©trie (DAT-05). Caring Nanny surveille l'espace. |

### 5.2 Mesures de dÃ©fense en profondeur

```
Couche 1 : Border Guard â€” Aucune sortie rÃ©seau
Couche 2 : Master Butler â€” Permissions strictes par acteur
Couche 3 : StrongFather â€” Mandats de Permission pour chaque opÃ©ration
Couche 4 : KindMother â€” Chiffrement au repos, isolation par profil
Couche 5 : WorrySentinel â€” Classification et politique de sÃ©curitÃ©
Couche 6 : Caring Nanny â€” RÃ©duction/suspension en cas de dÃ©gradation
```

---

## 6. Audit de sÃ©curitÃ©

### 6.1 Ã‰vÃ©nements auditables

| Ã‰vÃ©nement | Niveau d'audit | Description |
|-----------|---------------|-------------|
| Collecte activÃ©e/dÃ©sactivÃ©e | Standard | Changement d'Ã©tat de la collecte. |
| Effacement de donnÃ©es | Standard | Toute suppression (partielle ou totale). |
| Purge automatique | Faible | Purge par rÃ©tention (Ã©vÃ©nement normal). |
| AccÃ¨s aux mÃ©triques refusÃ© | Ã‰levÃ© | Tentative d'accÃ¨s non autorisÃ©e (Master Butler). |
| Tentative de sortie rÃ©seau | Critique | Border Guard bloque (ne devrait jamais arriver). |
| Modification de rÃ©tention | Standard | Changement de configuration. |
| Transition d'Ã©tat de confiance | Standard | Impact sur la collecte (T0 â†’ T2, etc.). |

### 6.2 Conservation du journal d'audit

| ParamÃ¨tre | Valeur |
|-----------|--------|
| RÃ©tention du journal d'audit | 365 jours (minimum 90 jours, non configurable en dessous). |
| Stockage | KindMother (chiffrÃ© au repos). |
| AccÃ¨s | Utilisateur (via l'Ã©cran Historique des actions) et WorrySentinel. |

---

## 7. ConformitÃ© architecturale

### 7.1 RÃ¨gles de code

| RÃ¨gle | ConformitÃ© MiyukiniWatch |
|-------|--------------------------|
| `unsafe_code = "forbid"` | Oui â€” aucun code `unsafe` dans les crates MiyukiniWatch. |
| Pas de dÃ©pendance rÃ©seau | Oui â€” aucune crate rÃ©seau (hyper, reqwest, tokio::net) importÃ©e. |
| Pas d'accÃ¨s fichier direct | Oui â€” tout accÃ¨s au stockage passe par KindMother. |
| Pas de lecture de stdin/console | Oui â€” pas d'entrÃ©e utilisateur directe (tout passe par l'UI Central). |

### 7.2 VÃ©rification de conformitÃ©

La conformitÃ© sÃ©curitÃ© de MiyukiniWatch peut Ãªtre auditÃ©e via la matrice de vÃ©rification du document [Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md), section 6, complÃ©tÃ©e par les vÃ©rifications suivantes :

| VÃ©rification | CritÃ¨re | RÃ©sultat attendu |
|-------------|---------|-------------------|
| Les donnÃ©es de niveau 2 (Amis) sont-elles chiffrÃ©es au repos ? | WorrySentinel | **Oui** |
| Les accÃ¨s refusÃ©s sont-ils journalisÃ©s ? | Audit sÃ©curitÃ© | **Oui** |
| Le code contient-il des `unsafe` ? | RÃ¨gle de code | **Non** |
| Le code importe-t-il des crates rÃ©seau ? | RÃ¨gle de code | **Non** |
| Le journal d'audit est-il conservÃ© au moins 90 jours ? | Politique | **Oui** |
| Les transitions d'Ã©tat T0â€“T4 sont-elles gÃ©rÃ©es ? | Caring Nanny | **Oui** |

---

## 8. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [MiyukiniWatch â€” Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Principes de sÃ©curitÃ© et gouvernance. |
| [MiyukiniWatch â€” Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non nÃ©gociables (C-05 Ã  C-17). |
| [MiyukiniWatch â€” Gouvernance DonnÃ©es et RÃ©tention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Politique de rÃ©tention et effacement. |
| [MiyukiniWatch â€” Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Interactions Cores, flux de gouvernance. |
| [Security â€” Liste des Mesures de SÃ©curitÃ©](..//..//cores//WorrySentinel//_index.md) | RÃ©fÃ©rence globale des mesures de sÃ©curitÃ©. |
| Architecture Miyukini (skill miyukini-architecture) | Ã‰tats de confiance T0â€“T4, niveaux de sÃ©curitÃ© 0â€“4. |

---

**Document** : MiyukiniWatch â€” SÃ©curitÃ© et ConformitÃ©  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif â€” rÃ©fÃ©rence sÃ©curitÃ© du Service MiyukiniWatch

