# MiyukiniWatch — Sécurité et Conformité

## Contexte

**MiyukiniWatch** manipule des données d'usage personnelles (habitudes, sessions, amis contactés). Bien que ces données ne contiennent jamais de contenu (messages, textes, fichiers), elles constituent des **métadonnées comportementales** qui requièrent un niveau de protection approprié. Ce document définit les exigences de sécurité, la classification des données, le chiffrement au repos, les contrôles d'accès et la conformité vis-à-vis des Cores de gouvernance.

## Portée / Scope

- **Applicable à :** Sécurité des données, classification, chiffrement, contrôles d'accès, audit sécurité.
- **Audience :** Équipes sécurité, architectes, développeurs, auditeurs.
- **Statut :** Document normatif — référence sécurité du Service MiyukiniWatch.

---

## 1. Classification des données

### 1.1 Niveau de sécurité

Conformément aux niveaux de sécurité Miyukini (0–4), les données MiyukiniWatch sont classifiées :

| Données | Niveau | Justification |
|---------|--------|---------------|
| Métriques de sessions (horodatages, durées) | **1 — Standard** | Données d'usage personnelles, non sensibles au sens strict, mais privées. |
| Métriques de services (quels services, fréquence) | **1 — Standard** | Habitudes d'usage, pas de contenu. |
| Métriques d'amis (identifiants de contacts, timing) | **2 — Sensitive** | Métadonnées de relations sociales ; plus sensibles que des données d'usage pure. |
| Agrégats exposés à Miou | **1 — Standard** | Résumés anonymisés, moins granulaires que les données brutes. |
| Journal d'audit | **1 — Standard** | Historique des actions (effacements, désactivations). |
| Compteurs globaux | **0 — Public** | Informations non identifiantes (nombre total de sessions). |

### 1.2 Matrice de classification

| Dimension | Contenu | Niveau |
|-----------|---------|--------|
| **Quand** | Horodatages, durées | 1 — Standard |
| **Où** | Identifiants de services, onglets | 1 — Standard |
| **Qui** | Identifiants techniques de contacts | 2 — Sensitive |
| **Combien** | Compteurs, agrégats numériques | 0–1 |

---

## 2. Chiffrement

### 2.1 Chiffrement au repos

| Exigence | Description |
|----------|-------------|
| **Toutes les données MiyukiniWatch sont chiffrées au repos.** | Le stockage KindMother utilise le chiffrement par défaut de la couche de persistance du COG. |
| **Clé de chiffrement** | Gérée par WorrySentinel / KindMother. Liée au profil utilisateur. |
| **Algorithme** | Défini par la politique de sécurité du COG (spécification WorrySentinel). |

### 2.2 Chiffrement en transit

| Exigence | Description |
|----------|-------------|
| **Non applicable pour le réseau** | MiyukiniWatch ne communique pas sur le réseau. Aucun chiffrement réseau requis. |
| **Communications internes** | Les échanges entre Opérateurs, BondingBrother et Cores sont des appels internes au processus. Si une architecture multi-processus est utilisée, les IPC sont sécurisés par le Kernel. |

---

## 3. Contrôles d'accès

### 3.1 Matrice d'accès

| Acteur | Lecture métriques | Écriture métriques | Effacement | Lecture agrégats | Configuration |
|--------|-------------------|-------------------|------------|-----------------|---------------|
| **Utilisateur (via MiyukiniWatch UI)** | Oui | Non (automatique) | Oui | Oui | Oui |
| **MiyukiniWatchCollector** | Non | Oui (WriteIntent) | Non | Non | Non |
| **MiyukiniWatchAggregator** | Oui (ReadIntent) | Oui (agrégats) | Oui (purge) | Non | Non |
| **MiyukiniWatchPresenter** | Oui (ReadIntent) | Non | Oui (DeleteIntent) | Non | Non |
| **Miou** | Non | Non | Non | Oui (lecture agrégats) | Non |
| **Autres services** | Non | Non | Non | Non | Non |
| **Autres COGs** | Non | Non | Non | Non | Non |
| **MWS (relays, trackers)** | Non | Non | Non | Non | Non |

### 3.2 Gouvernance des accès

Chaque accès est gouverné par les Cores via BondingBrother :

| Core | Rôle dans le contrôle d'accès |
|------|-------------------------------|
| **Master Butler** | Maintient le registre des permissions. Vérifie que l'acteur a le droit d'effectuer l'opération demandée. |
| **StrongFather** | Émet les Mandats de Permission pour les opérations sensibles (effacement, purge, changement de configuration). |
| **Border Guard** | Bloque toute tentative d'accès aux données MiyukiniWatch depuis l'extérieur du COG. |
| **WorrySentinel** | Définit le niveau de sécurité requis pour chaque type de données et vérifie la conformité. |

### 3.3 Isolation par profil

| Règle | Description |
|-------|-------------|
| **Un profil = un jeu de données** | Chaque profil utilisateur a ses propres données MiyukiniWatch, isolées des autres. |
| **Pas de cross-profil** | Un profil ne peut pas accéder aux métriques d'un autre profil, même sur le même COG. |
| **Session authentifiée** | L'accès aux données MiyukiniWatch requiert une session Central authentifiée. |

---

## 4. États de confiance et comportement

Conformément aux états de confiance Miyukini (T0–T4), MiyukiniWatch adapte son comportement :

| État | Nom | Comportement MiyukiniWatch |
|------|-----|---------------------------|
| **T0** | Normal | Collecte active. Agrégation normale. Interface complète. |
| **T1** | Instable | Collecte active. Agrégation peut être retardée. |
| **T2** | Dégradé | Collecte réduite (uniquement sessions et services, pas de clics). Agrégation reportée. |
| **T3** | Restreint | Collecte suspendue. Données existantes consultables. Pas de nouvelle écriture. |
| **T4** | Bloqué | MiyukiniWatch inaccessible. Données préservées mais non consultables tant que le COG est en T4. |

### 4.1 Transition et recovery

| Transition | Action MiyukiniWatch |
|------------|---------------------|
| T0 → T2 | Caring Nanny signale la dégradation ; Collector réduit sa collecte. |
| T2 → T0 | Retour à la collecte complète. Les données de la période dégradée sont marquées comme potentiellement incomplètes. |
| T3 → T0 | Reprise de la collecte. Un événement d'audit « collecte reprise après restriction T3 » est enregistré. |
| T4 → T0 | Reprise complète. Audit de l'intégrité des données. |

---

## 5. Menaces et contre-mesures

### 5.1 Modèle de menaces

| Menace | Scénario | Contre-mesure |
|--------|----------|---------------|
| **Exfiltration de données** | Un service malveillant tente de lire les données MiyukiniWatch. | Border Guard bloque tout accès externe. Master Butler refuse les ReadIntent non autorisés. |
| **Injection de métriques** | Un service tente d'écrire de fausses métriques. | Seul le MiyukiniWatchCollector a le droit de WriteIntent. StrongFather vérifie l'origine. |
| **Accès non autorisé** | Un autre profil tente de lire les métriques. | Isolation par profil. Session authentifiée requise. |
| **Télémétrie cachée** | Un composant tente d'envoyer des métriques sur le réseau. | Border Guard bloque. Aucune capability réseau n'est accordée à MiyukiniWatch. |
| **Perte de données** | Crash ou corruption pendant l'écriture. | Écriture atomique via KindMother. En cas d'échec, la métrique est perdue (acceptable : ce n'est pas une donnée critique). |
| **Surcharge stockage** | Collecte excessive saturant le stockage. | Limites de volumétrie (DAT-05). Caring Nanny surveille l'espace. |

### 5.2 Mesures de défense en profondeur

```
Couche 1 : Border Guard — Aucune sortie réseau
Couche 2 : Master Butler — Permissions strictes par acteur
Couche 3 : StrongFather — Mandats de Permission pour chaque opération
Couche 4 : KindMother — Chiffrement au repos, isolation par profil
Couche 5 : WorrySentinel — Classification et politique de sécurité
Couche 6 : Caring Nanny — Réduction/suspension en cas de dégradation
```

---

## 6. Audit de sécurité

### 6.1 Événements auditables

| Événement | Niveau d'audit | Description |
|-----------|---------------|-------------|
| Collecte activée/désactivée | Standard | Changement d'état de la collecte. |
| Effacement de données | Standard | Toute suppression (partielle ou totale). |
| Purge automatique | Faible | Purge par rétention (événement normal). |
| Accès aux métriques refusé | Élevé | Tentative d'accès non autorisée (Master Butler). |
| Tentative de sortie réseau | Critique | Border Guard bloque (ne devrait jamais arriver). |
| Modification de rétention | Standard | Changement de configuration. |
| Transition d'état de confiance | Standard | Impact sur la collecte (T0 → T2, etc.). |

### 6.2 Conservation du journal d'audit

| Paramètre | Valeur |
|-----------|--------|
| Rétention du journal d'audit | 365 jours (minimum 90 jours, non configurable en dessous). |
| Stockage | KindMother (chiffré au repos). |
| Accès | Utilisateur (via l'écran Historique des actions) et WorrySentinel. |

---

## 7. Conformité architecturale

### 7.1 Règles de code

| Règle | Conformité MiyukiniWatch |
|-------|--------------------------|
| `unsafe_code = "forbid"` | Oui — aucun code `unsafe` dans les crates MiyukiniWatch. |
| Pas de dépendance réseau | Oui — aucune crate réseau (hyper, reqwest, tokio::net) importée. |
| Pas d'accès fichier direct | Oui — tout accès au stockage passe par KindMother. |
| Pas de lecture de stdin/console | Oui — pas d'entrée utilisateur directe (tout passe par l'UI Central). |

### 7.2 Vérification de conformité

La conformité sécurité de MiyukiniWatch peut être auditée via la matrice de vérification du document [Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md), section 6, complétée par les vérifications suivantes :

| Vérification | Critère | Résultat attendu |
|-------------|---------|-------------------|
| Les données de niveau 2 (Amis) sont-elles chiffrées au repos ? | WorrySentinel | **Oui** |
| Les accès refusés sont-ils journalisés ? | Audit sécurité | **Oui** |
| Le code contient-il des `unsafe` ? | Règle de code | **Non** |
| Le code importe-t-il des crates réseau ? | Règle de code | **Non** |
| Le journal d'audit est-il conservé au moins 90 jours ? | Politique | **Oui** |
| Les transitions d'état T0–T4 sont-elles gérées ? | Caring Nanny | **Oui** |

---

## 8. Références

| Document | Rôle |
|----------|------|
| [MiyukiniWatch — Document Fondateur](./MiyukiniWatch%20-%20Document%20Fondateur.md) | Principes de sécurité et gouvernance. |
| [MiyukiniWatch — Contraintes et Invariants](./MiyukiniWatch%20-%20Contraintes%20et%20Invariants.md) | Contraintes non négociables (C-05 à C-17). |
| [MiyukiniWatch — Gouvernance Données et Rétention](./MiyukiniWatch%20-%20Gouvernance%20Donnees%20et%20Retention.md) | Politique de rétention et effacement. |
| [MiyukiniWatch — Architecture et Positionnement](./MiyukiniWatch%20-%20Architecture%20et%20Positionnement.md) | Interactions Cores, flux de gouvernance. |
| [Security — Liste des Mesures de Sécurité](../../security/reference/Security%20-%20Liste%20des%20Mesures%20de%20Securite%20Miyukini%20COG%20et%20MWS.md) | Référence globale des mesures de sécurité. |
| Architecture Miyukini (skill miyukini-architecture) | États de confiance T0–T4, niveaux de sécurité 0–4. |

---

**Document** : MiyukiniWatch — Sécurité et Conformité  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document normatif — référence sécurité du Service MiyukiniWatch
