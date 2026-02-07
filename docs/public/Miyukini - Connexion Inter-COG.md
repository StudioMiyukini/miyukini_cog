# Miyukini Conceptual References - Connexion Inter-COG

## Contexte

Ce document définit l'architecture de **visite gouvernée inter-COG** dans l'écosystème Miyukini. Il formalise les mécanismes permettant à un utilisateur d'un COG d'accéder temporairement aux services d'un autre COG, sous contrôle strict de la gouvernance hôte.

## Portée / Scope

- Définition des acteurs de la connexion inter-COG
- Objets conceptuels (Passeport, Visa, Demande de Visite)
- Séquence complète de connexion
- Niveaux de sécurité et cas de dégradation
- Principes fondamentaux non négociables

---

## 1. Vue d'ensemble

Miyukini supporte un modèle de **visite gouvernée inter-COG**, dans lequel :

- Un **COG Hébergeur** (Host COG) expose des Services
- Un **Utilisateur Visiteur** (Visitor User) provenant d'un **COG Origine**
- Accède **temporairement** à ces services sans importer sa gouvernance
- Sous contrôle **strict, traçable, révocable et hiérarchisé**

### Principes cardinaux

> **Le COG est l'unité souveraine.**
> **L'utilisateur n'est jamais souverain hors de son COG.**

---

## 2. Acteurs fondamentaux

### 2.1 COG Origine (Home COG)

**Rôle :**
- Autorité d'identité de l'utilisateur
- Garant de la conformité de l'environnement d'origine
- Émetteur du Passeport Utilisateur

**Responsabilités :**
- Vérifier l'intégrité locale
- Attester la version de la strate Core
- Fournir une identité vérifiable
- **Ne participe PAS à l'exécution distante**

---

### 2.2 Utilisateur Visiteur (Visitor User)

**Statut :**
- Citoyen dans son COG d'origine
- Visiteur gouverné dans le COG hôte

**Caractéristiques :**
- Conserve son identité
- Perd toute souveraineté d'exécution
- Agit uniquement via un Visa de Connexion
- Ne transporte aucun core, aucune logique, aucun état

---

### 2.3 COG Hébergeur (Host COG)

**Rôle :**
- Souverain exécutif de la session
- Unique source de vérité de l'état
- Autorité de sécurité et d'arbitrage

**Responsabilités :**
- Vérifier le visiteur
- Accorder ou refuser l'accès
- Encadrer strictement l'exécution
- Surveiller la session (WorrySentinel)
- Révoquer à tout moment

---

### 2.4 Bridge inter-COG (BondingBrother étendu)

**Rôle :**
- Canal diplomatique
- Transport des identités, intentions et autorisations
- **Aucun pouvoir décisionnel**
- **Aucun état métier**

> **Le bridge ne fait jamais confiance, il transporte.**

---

## 3. Objets conceptuels clés

### 3.1 Passeport Utilisateur (User Passport)

**Émis par :** COG Origine

**Contient :**
| Champ | Description |
|-------|-------------|
| `user_id` | Identité utilisateur unique |
| `cog_origin_id` | ID du COG d'origine |
| `core_version` | Version exacte de la strate Core |
| `integrity_hash` | Empreinte d'intégrité (Core + Kernel) |
| `issued_at` | Timestamp d'émission |
| `valid_until` | Durée de validité |
| `signature` | Signature du COG Origine |

**Garanties :**
- Non falsifiable
- Non transférable
- Lisible mais non modifiable

> **Le passeport ne donne aucun droit.**
> **Il prouve seulement qui tu es et d'où tu viens.**

---

### 3.2 Demande de Visite (Visit Intent)

**Émise par :** Utilisateur Visiteur

**Contient :**
| Champ | Description |
|-------|-------------|
| `requested_services` | Liste des Services demandés |
| `usage_nature` | Nature de l'usage (lecture, interaction, temps réel, etc.) |
| `security_level` | Niveau de sécurité requis |
| `terminal_context` | Contexte terminal (PC, mobile, web…) |

> **C'est une intention, pas une permission.**

---

### 3.3 Visa de Connexion (Connection Visa)

**Émis par :** Host COG

**Contient :**
| Champ | Description |
|-------|-------------|
| `authorized_services` | Services autorisés |
| `accessible_cores` | Cores accessibles (indirectement) |
| `security_level` | Niveau de sécurité accordé |
| `execution_rules` | Règles d'exécution |
| `time_limits` | Limites temporelles |
| `functional_limits` | Limites fonctionnelles |
| `terminal_constraints` | Contraintes terminal |
| `revocation_conditions` | Conditions de révocation |

**Caractéristiques :**
- Temporaire
- Révocable
- Non transférable
- Auditée
- Strictement interprétée

> **Le Visa définit l'univers légal du visiteur.**

---

## 4. Niveaux de sécurité du Visa

| Niveau | Nom | Usage typique | Caractéristiques |
|--------|-----|---------------|------------------|
| **S1** | Observation | Lecture, spectateur | Aucun état modifiable |
| **S2** | Interaction contrôlée | UI, formulaires | Validation stricte |
| **S3** | Temps réel | Jeu, collaboration | Surveillance renforcée |
| **S4** | Sensible | Admin, finance | Audit continu |
| **S5** | Critique | MiyukiniAdmin | Arbitrage ultime |

> **Un utilisateur = un Visa = un niveau unique.**

---

## 5. Séquence complète de connexion

### Étape 1 — Pré-validation locale

Le COG Origine vérifie :
- Intégrité Kernel
- Version Core
- Conformité locale

**Résultat :** Émet le **Passeport Utilisateur**

---

### Étape 2 — Présentation au Bridge

Le visiteur transmet :
- Passeport
- Demande de Visite

**Le Bridge transmet sans interprétation**

---

### Étape 3 — Douane du Host COG

Le Host COG vérifie :
- Validité du Passeport
- Compatibilité des versions
- Politique d'accueil
- Niveau de sécurité requis
- État interne (charge, risques)

> **Refus possible sans justification détaillée**

---

### Étape 4 — Émission du Visa

Le Host COG :
- Génère un Visa strict
- Enregistre la session
- Active la surveillance (WorrySentinel)

---

### Étape 5 — Session active

- Le visiteur est traité comme un utilisateur local
- **MAIS** uniquement dans le cadre du Visa
- Toute action hors cadre = **rejet**

---

### Étape 6 — Fin ou rupture

Causes possibles :
- Expiration naturelle
- Révocation manuelle
- Dégradation détectée
- Rupture réseau
- Violation de règles

> **L'état reste 100% dans le Host COG.**

---

## 6. Diagramme de séquence

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ COG Origine │    │   Bridge    │    │  Host COG   │    │   Visitor   │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │                  │
       │◄─── Demande Passeport ─────────────────────────────────┤
       │                  │                  │                  │
       ├─── Vérification intégrité ───►     │                  │
       │                  │                  │                  │
       │──── Passeport ──────────────────────────────────────►  │
       │                  │                  │                  │
       │                  │◄── Passeport + Visit Intent ────────┤
       │                  │                  │                  │
       │                  ├─── Transport ───►│                  │
       │                  │                  │                  │
       │                  │                  ├── Vérification   │
       │                  │                  │   Douane         │
       │                  │                  │                  │
       │                  │◄─── Visa ────────┤                  │
       │                  │                  │                  │
       │                  ├─────────────────────── Visa ───────►│
       │                  │                  │                  │
       │                  │                  │◄── Actions Visa ─┤
       │                  │                  │                  │
       │                  │                  ├── Surveillance   │
       │                  │                  │   WorrySentinel  │
       │                  │                  │                  │
```

---

## 7. Cas de dégradation & rupture

### 7.1 Désynchronisation réseau

La session peut :
- Être suspendue
- Passer en lecture seule
- Être terminée proprement

### 7.2 Soupçon d'intrusion

- Révocation immédiate du Visa
- Journalisation
- Option de blacklist du COG Origine

### 7.3 Panne du COG Origine

- **Aucun impact direct**
- Le Host reste souverain
- La session continue ou se termine selon politique locale

---

## 8. Principes non négociables

| Interdit | Obligatoire |
|----------|-------------|
| ❌ Aucun core n'est partagé | ✅ Une seule gouvernance active |
| ❌ Aucun état n'est migré en direct | ✅ Identité ≠ autorité |
| ❌ Aucun pouvoir n'est délégué | ✅ Sécurité avant fluidité |

---

## 9. Positionnement dans la pyramide Miyukini

| Composant | Rôle dans l'inter-COG |
|-----------|----------------------|
| **Kernel** | Identité, horodatage, intégrité |
| **BorderGuard** | Frontière inter-COG |
| **BondingBrother** | Transport diplomatique |
| **WorrySentinel** | Surveillance & dégradation |
| **StrongFather** | Décisions d'accès |
| **MasterButler** | Services exposables |

---

## 10. Phrase de synthèse

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

---

---

## 11. Utilisateurs externes non certifiés

*(Public Users / Anonymous Users / Web Visitors)*

### 11.1 Positionnement conceptuel

Ces utilisateurs :
- **Ne sont PAS** des citoyens
- **Ne sont PAS** des visiteurs inter-COG
- **Ne possèdent** aucune gouvernance

> **Ils sont consommateurs de services exposés, pas participants au système.**

### 11.2 Principe fondamental

> **Un utilisateur externe n'entre jamais dans un COG.**
> **Il interagit uniquement avec une façade d'exposition gouvernée.**
> **C'est le COG qui sort vers lui, jamais l'inverse.**

---

### 11.3 Façade Publique Gouvernée (Public Exposure Surface)

**Caractéristiques :**
- Strictement unidirectionnelle
- Sans identité persistante obligatoire
- Sans accès aux cores
- Sans accès à la logique interne
- Sans état souverain

> **C'est une zone tampon, gouvernée, filtrée, instrumentée.**

---

### 11.4 Différence clé avec le Visiteur inter-COG

| Critère | Visiteur inter-COG | Utilisateur externe |
|---------|-------------------|---------------------|
| **Identité vérifiée** | ✅ Passeport | ❌ Optionnelle |
| **COG d'origine** | ✅ Oui | ❌ Non |
| **Visa** | ✅ Oui | ❌ Non |
| **Gouvernance** | Host COG | Host COG |
| **Accès cores** | Indirect | ❌ Jamais |
| **État persistant** | Possible (local) | ❌ |
| **Cas d'usage** | Jeu, outils, SaaS | Site web, vitrine |

---

### 11.5 Traitement dans Miyukini

#### Statut : Utilisateur Public

Un utilisateur externe est traité comme :
> **Un acteur sans identité souveraine, soumis à des règles publiques strictes.**

#### Mandat Public d'Accès (Public Access Mandate)

Au lieu de Passeport + Visa, on utilise un **Mandat Public d'Accès**.

**Défini par le Host COG, il spécifie :**

| Champ | Description |
|-------|-------------|
| `public_services` | Services publics accessibles |
| `allowed_methods` | Méthodes autorisées |
| `quotas` | Quotas d'utilisation |
| `rate_limits` | Limitations de fréquence |
| `security_level` | Niveau de sécurité |
| `expected_behavior` | Comportement attendu |

> **Le mandat est attaché au service, pas à l'utilisateur.**

#### Gouvernance appliquée

Même sans identité forte :

| Core | Rôle |
|------|------|
| **StrongFather** | Décide si la requête est recevable |
| **MasterButler** | Limite les capacités exposées |
| **WorrySentinel** | Surveille abus, patterns suspects, anomalies |
| **BorderGuard** | Filtre les entrées |
| **KindMother** | Peut lire (jamais écrire) |

---

### 11.6 Cas concrets

#### Site web public (CMS-like)

| Aspect | Détail |
|--------|--------|
| **Utilisateur** | Navigateur, aucun login |
| **Accès** | Lecture de contenu, recherche, navigation |
| **Sécurité** | Mandat Public S1 (Observation), Rate limiting, Anti-scraping, Détection comportementale |

#### Formulaire public

| Aspect | Détail |
|--------|--------|
| **Utilisateur** | Non certifié, données entrantes |
| **Accès** | Écriture encapsulée, validation stricte, aucune exécution directe |
| **Sécurité** | Mandat Public S2, Sandbox logique, Validation renforcée, Journalisation |

#### Démo interactive / jeu web

| Aspect | Détail |
|--------|--------|
| **Utilisateur** | Anonyme ou pseudo |
| **Accès** | Interaction temps réel, aucun état critique |
| **Sécurité** | Mandat Public S3 limité, Quotas, Session courte, Dégradation automatique |

---

### 11.7 Dégradation & blocage

Pour les utilisateurs externes, la dégradation est **beaucoup plus agressive** :

| Action | Description |
|--------|-------------|
| **Throttle** | Ralentissement |
| **Downgrade** | Moins de fonctionnalités |
| **Freeze** | Lecture seule |
| **Block** | IP / session / pattern |
| **Blackhole** | Réponse neutre, pas d'erreur exploitable |

> **Aucun droit acquis, aucune négociation.**

---

### 11.8 Garanties de robustesse

**Aucun utilisateur externe :**
- ❌ Ne peut écrire dans un core
- ❌ Ne peut influencer la gouvernance
- ❌ Ne peut provoquer une migration

**Toute exposition est :**
- ✅ Volontaire
- ✅ Bornée
- ✅ Révocable
- ✅ Observable

---

### 11.9 Positionnement dans la pyramide

```
        Utilisateur Externe
               ↓
    Façade Publique Gouvernée
               ↓
    BorderGuard + WorrySentinel
               ↓
    StrongFather / MasterButler
               ↓
        Cores (inaccessibles)
               ↓
            Kernel
```

---

### 11.10 Phrase de synthèse

> **Les utilisateurs externes ne sont pas des visiteurs. Ce sont des consommateurs de surfaces exposées, sous mandat public.**

---

## 12. Évolutions futures

- [ ] Formaliser un **Inter-COG Visit Contract**
- [ ] Définir la **matrice de compatibilité de versions**
- [ ] Intégrer au **cross-terminal** proprement
- [ ] Spécifier le protocole de **blacklist inter-COG**
- [ ] Définir les **SLA de session visiteur**
- [ ] Spécifier les **quotas standards par niveau de Mandat Public**
- [ ] Définir le **protocole de blackhole**
- [ ] Intégrer la couche de présence **Miyukini Webway System (MWS)** pour la découverte des COGs → voir [Miyukini Webway System](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)

---

## Références croisées

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) — couche de présence et découverte
- [Ecosystem Dependency Contract](./Miyukini%20Conceptual%20References%20-%20Ecosystem%20Dependency%20Contract.md)
- [Security Levels](./Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Souverainete Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)

---

*Document créé le 27/01/2026*
*Classification : Reference conceptuelle*
