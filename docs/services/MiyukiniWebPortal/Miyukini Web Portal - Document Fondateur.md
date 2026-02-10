# Miyukini Web Portal — Document Fondateur

## Contexte

**Miyukini Web Portal** (le **Portail**) est le **Service Fondamental** qui constitue le point d'entrée web pour les utilisateurs externes. Il est l'équivalent de Miyukini Central pour le monde extérieur : là où Central sert l'utilisateur du COG, le Portail sert les utilisateurs externes qui accèdent via un navigateur web.

Le Portail expose les **Façades Publiques Gouvernées** des Services de Type 2 (à surface web externe) sous contrôle strict de la gouvernance COG (BorderGuard, StrongFather, WorrySentinel).

**Règle canonique :**

> **Central = COG, Portail = Web.**

## Portée / Scope

- **Applicable à :** Conception, développement et exploitation du Portail Miyukini Web Portal
- **Audience :** Architectes, développeurs, designers UX/UI, responsables sécurité
- **Statut :** Document de référence fondateur — Service Fondamental

### Couvert par ce document

- Positionnement et rôle du Portail
- Relation avec Central et les Services
- Architecture et gouvernance
- Flux et sécurité
- Identification et fichage des connexions

### Hors scope

- Implémentation technique détaillée (voir guide d'implémentation)
- UX/UI détaillé des surfaces des Services (voir chaque Service)

---

## 1. Définition et Position dans la Pyramide

### 1.1 Nature du Miyukini Web Portal

| Aspect | Définition |
|--------|------------|
| **Type** | Service Fondamental — Opérateur d'Interface (Strate 7) |
| **Rôle** | Expose les Façades Publiques des Services (Type 2) aux utilisateurs externes via le web |
| **Question fondamentale** | *« Comment les utilisateurs externes accèdent-ils aux surfaces web du COG ? »* |
| **Cible** | Utilisateurs externes (clients, visiteurs, prospects) — sans COG, via navigateur |
| **Autorité** | Aucune autorité propre — applique les décisions des Cores (BorderGuard, StrongFather) |

**Phrase fondatrice :**

> **Le Portail est la porte d'entrée web du COG. L'utilisateur externe y accède ; le COG sort vers lui, jamais l'inverse.**

### 1.2 Ce que le Portail N'EST PAS

| ❌ N'est pas | Pourquoi |
|--------------|----------|
| Un serveur central unique | Chaque COG expose **son** Portail ; pas de "super-serveur" qui affiche tous les COGs |
| Un remplacement de Central | Central reste le point d'entrée pour l'utilisateur du COG |
| Une porte ouverte | Tout accès passe par BorderGuard, identification et Mandat Public d'Accès |
| Une source de vérité | La vérité est dans les Cores ; le Portail relaie et affiche |

### 1.3 Position dans la Pyramide Miyukini

```
STRATE 7 — Opérateurs
├── Miyukini Central (Service Fondamental) ← Point d'entrée COG
├── Miyukini Web Portal (Service Fondamental) ← Point d'entrée Web
├── JayXpose, JayFestival, JayRDV, JayKonta, ... (Services Type 1/2/3)
└── …

STRATE 9 — MiyukiniAdmin (exception, administration système)
```

---

## 2. Relation Central / Portail

### 2.1 Dualité fondamentale

| Aspect | Miyukini Central | Miyukini Web Portal |
|--------|------------------|---------------------|
| **Cible** | Utilisateur du COG | Utilisateurs externes (web) |
| **Accès** | Application desktop/web locale | Navigateur web distant |
| **Authentification** | Identité COG souveraine | Sans identité COG (Façade Publique) |
| **Gouvernance** | Mandat de Permission | Mandat Public d'Accès |
| **Rôle** | Gestion, administration, création | Consultation, réservation, achat |

### 2.2 Schéma de relation

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Utilisateur du COG                                │
│  (propriétaire, administrateur, gestionnaire)                       │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Miyukini Central                                  │
│  · Gestion, administration, création                                │
│  · Accès complet aux Services (Type 1, 2, 3)                        │
│  · Prépare les contenus exposés au Portail                          │
└─────────────────────────────────────────────────────────────────────┘
                            │
                            │ Les Services de Type 2 exposent
                            │ leurs capacités au Portail
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Miyukini Web Portal                               │
│  · Façades Publiques Gouvernées                                     │
│  · Surfaces web des Services (vitrine, réservation, catalogue...)  │
│  · Identification et fichage des connexions entrantes               │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Utilisateurs externes                             │
│  (clients, visiteurs, prospects — sans COG)                         │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.3 Règle de cohabitation

> **Central et Portail sont tous deux des Services Fondamentaux. Leur présence fait partie de l'environnement versionné du COG.**

Un COG sans Central ne peut pas être administré.  
Un COG sans Portail ne peut pas exposer de surfaces web (Services Type 2 limités à leur facette interne).

---

## 3. Architecture

### 3.1 Architecture logique

```
┌─────────────────────────────────────────────────────────────────────┐
│            Utilisateur externe (navigateur web)                      │
│  https://vitrine.mon-commerce.cog                                   │
│  https://rdv.kine-paris.cog                                         │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ HTTPS
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│              Miyukini Web Portal (Portail)                           │
│  · MiyuWeb (rendu HTML, layout, formulaires)                        │
│  · Routage vers les surfaces des Services                           │
│  · Identification et fichage (session, IP, scope)                   │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ BondingBrother
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    BorderGuard (Strate 4)                            │
│  · Définition des frontières et niveaux de confiance                │
│  · Mandat Public d'Accès                                            │
│  · Règles de franchissement                                         │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ StrongFather     │ │ KindMother       │ │ WorrySentinel    │
│ Décision ALLOW/  │ │ Données          │ │ Sécurité         │
│ DENY             │ │ (lecture seule)  │ │ (état, niveau)   │
└──────────────────┘ └──────────────────┘ └──────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Services (JayXpose, JayFestival, JayRDV, JayKonta)                  │
│  · Exposent leurs capacités (lecture, formulaires, actions bornées) │
│  · Jamais d'accès direct aux Cores depuis l'extérieur               │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Flux typiques

| Flux | Description |
|------|-------------|
| **Consultation vitrine** | Utilisateur externe → Portail → BorderGuard → JayXpose (lecture catalogue, pages) → rendu HTML |
| **Réservation RDV** | Utilisateur externe → Portail → BorderGuard → JayRDV (créneaux, formulaire, création RDV) |
| **Achat e-shop** | Utilisateur externe → Portail → BorderGuard → JayXpose (panier, commande) → KindMother (persist) |
| **Consultation facture** | Client (lien email) → Portail → BorderGuard (token) → JayKonta (lecture facture) |

---

## 4. Gouvernance et Sécurité

### 4.1 Principes fondamentaux

| Principe | Règle |
|----------|-------|
| **Façade Publique Gouvernée** | Le Portail expose une zone tampon ; l'utilisateur externe n'entre jamais dans le COG |
| **Mandat Public d'Accès** | Toute surface exposée est sous Mandat (quotas, rate limits, méthodes autorisées) |
| **Identification obligatoire** | Chaque connexion entrante est identifiée et fichée (session, IP, timestamp, scope) |
| **Pas d'accès aux Cores** | L'utilisateur externe n'accède jamais directement aux Cores ; uniquement via capacités exposées |

### 4.2 Identification et fichage des connexions

**Règle :**

> **Par sécurité, chaque connexion entrante est identifiée et fichée.**

| Donnée collectée | Usage |
|------------------|-------|
| **Session ID** | Identification de la session (token opaque) |
| **IP / origine** | Traçabilité, rate limiting, détection d'abus |
| **Timestamp** | Horodatage de la connexion |
| **Scope / Visa** | Niveau d'accès accordé (S1/S2 selon Mandat Public) |
| **Actions effectuées** | Audit (pages consultées, formulaires soumis) |

**Stockage :** KindMother (logs de sécurité), non exposé à l'utilisateur externe.

### 4.3 Niveaux de sécurité (WorrySentinel)

| Niveau | Usage typique sur le Portail |
|--------|------------------------------|
| **0 — Public** | Lecture catalogue, pages vitrine |
| **1 — Standard** | Formulaire de contact, réservation simple |
| **2 — Sensitive** | Paiement, données personnelles |

Les niveaux 3+ (Critical, Highest) ne sont pas exposés via le Portail (réservés à Central / Admin).

### 4.4 Dégradation et protection

| Action | Déclencheur | Effet |
|--------|-------------|-------|
| **Throttle** | Requêtes trop fréquentes | Ralentissement |
| **Downgrade** | Comportement suspect | Fonctionnalités réduites |
| **Freeze** | Anomalie détectée | Lecture seule |
| **Block** | Abus confirmé | IP / session bloquée |
| **Blackhole** | Attaque | Réponse neutre, pas d'erreur exploitable |

---

## 5. Relation avec le Maillage Webway

### 5.1 Rôle du COG Tracker

Le **COG Tracker** (Maillage Webway) peut cataloguer les Portails des COGs participants :

- **Déclaration** : un COG déclare son Portail (adresse, services exposés)
- **Découverte** : d'autres COGs ou utilisateurs peuvent découvrir les Portails disponibles
- **Pas de gouvernance** : le Tracker ne gouverne pas ; il référence et facilite la découverte

### 5.2 Portail et Inter-COG

Le Portail sert les **utilisateurs externes sans COG**.

Pour les **utilisateurs d'autres COGs** (Visite gouvernée inter-COG), c'est le mécanisme de Passeport / Visa qui s'applique, pas le Portail. Le Portail est réservé aux utilisateurs non-COG.

---

## 6. Services exposés via le Portail

### 6.1 Services de Type 2 (à surface web externe)

Seuls les Services de **Type 2** exposent une surface via le Portail.

| Service | Surface Portail | Parcours typiques |
|---------|-----------------|-------------------|
| **JayXpose** | Vitrine, e-shop, annuaire, blog | Consultation catalogue, achat, contact |
| **JayFestival** | Catalogue événements, billets, espace visiteur | Découverte, inscription, réservation |
| **JayRDV** | Page réservation, parcours guest | Choix créneau, formulaire, confirmation |
| **JayKonta** | Portail client | Consultation factures, paiement en ligne |

### 6.2 Ce qui n'est PAS exposé

| Service / Fonction | Exposition Portail |
|--------------------|--------------------|
| **JayKoa** (agenda personnel) | ❌ Interne uniquement |
| **Administration comptable** (JayKonta) | ❌ Central uniquement |
| **Gestion exposant** (JayXpose) | ❌ Central uniquement |
| **MiyukiniAdmin** | ❌ Jamais exposé |

---

## 7. Type de Service et Espaces

| Aspect | Valeur |
|--------|--------|
| **Type** | Service Fondamental |
| **Description** | Point d'entrée web pour les utilisateurs externes |
| **Espace Central** | ✅ Administration du Portail (paramètres, logs, monitoring) |
| **Espace Portail** | ✅ Le Portail lui-même (point d'entrée web) |
| **Espace Inter-COG** | ❌ Non concerné (les visites inter-COG passent par Passeport/Visa, pas par le Portail) |

---

## 8. Outils et Kits d'Outils

Le Portail utilise les Outils suivants (Strate 6) :

| Outil / Kit | Usage |
|-------------|-------|
| **MiyuWeb** | Rendu HTML, layout, formulaires, événements |
| **MiyuValidate** | Validation des entrées utilisateur |
| **MiyuAntiSpam** | Rate limiting, captcha, flood protection |
| **BorderGuard (capacités)** | Règles de franchissement, niveaux de confiance |

---

## 9. Synthèse

| Dimension | Miyukini Web Portal |
|-----------|---------------------|
| **Type** | Service Fondamental — Opérateur d'Interface (Strate 7) |
| **Rôle** | Point d'entrée web pour les utilisateurs externes |
| **Cible** | Clients, visiteurs, prospects — sans COG |
| **Gouvernance** | BorderGuard + Mandat Public d'Accès + Identification/Fichage |
| **Services exposés** | Type 2 (JayXpose, JayFestival, JayRDV, JayKonta portail client) |
| **Outils** | MiyuWeb, MiyuValidate, MiyuAntiSpam |
| **Relation Central** | Central = COG, Portail = Web |

---

## 10. Références

| Document | Lien |
|----------|------|
| **Miyukini Central** | [Miyukini Central Hub Services](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) |
| **Types de Services** | [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) |
| **Glossaire** | [Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Façade Publique** | [Glossaire § Façade Publique Gouvernée](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **BorderGuard** | [docs/core/BorderGuard](../../core/BorderGuard/) |
| **Souveraineté** | [Souveraineté Environnement](../../reference/Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** Document de référence fondateur — Service Fondamental Miyukini Web Portal
