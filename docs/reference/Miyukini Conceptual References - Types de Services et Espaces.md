# Miyukini Conceptual References — Types de Services et Espaces

## Contexte

Ce document établit la **classification empirique des Services** dans l'écosystème Miyukini COG. Tout Service doit se ranger dans l'un des trois types définis et prévoir les espaces correspondants (Central, Portail, Inter-COG).

Cette classification est **fondamentale** pour la conception, la documentation et l'implémentation de tout nouveau Service.

## Portée / Scope

- **Applicable à :** Tout Service documenté ou implémenté dans l'écosystème Miyukini
- **Audience :** Architectes, développeurs, concepteurs produit, rédacteurs de documentation
- **Statut :** Document de référence normatif — CLASSIFICATION OFFICIELLE

---

## 1. Les Trois Types de Services

### 1.1 Définition canonique

| Type | Nom | Définition | Point d'entrée utilisateur |
|------|-----|------------|---------------------------|
| **Type 1** | **Service interne COG** | Service destiné uniquement à l'utilisateur du COG. Gestion, administration, création. Aucune surface consommée par un utilisateur final externe. | **Central** uniquement |
| **Type 2** | **Service à surface web externe** | Service avec gestion dans le COG + **surface destinée à l'utilisateur final** (lecture, réservation, achat, formulaire, etc.) accessible depuis le web par des utilisateurs externes. | **Central** (gestion) + **Portail** (façade publique) |
| **Type 3** | **Service Inter-COG** | Service destiné aux interactions entre COGs (ex. jeux multijoueur, fédération, collaboration). Les "utilisateurs" sont d'autres COGs ou des utilisateurs issus d'autres COGs. | **Central** (paramétrage local) + **Protocoles Webway / Inter-COG** |

### 1.2 Règle fondamentale

> **Tout Service doit déclarer son type (1, 2 ou 3) et prévoir les espaces correspondants.**

---

## 2. Détail des Types

### 2.1 Type 1 — Service interne COG

**Définition étendue :**

Un Service de Type 1 est entièrement destiné à l'utilisateur propriétaire du COG. Il n'expose aucune surface à des utilisateurs externes (ni web, ni inter-COG). Toute l'administration, gestion et utilisation se fait via Miyukini Central.

**Caractéristiques :**

| Aspect | Règle |
|--------|-------|
| **Point d'entrée** | Miyukini Central uniquement |
| **Accès externe** | ❌ Aucun |
| **Façade publique** | ❌ Aucune |
| **Inter-COG** | ❌ Non concerné (sauf si évolution vers Type 3) |
| **Données** | Souveraines, locales, non exposées |

**Exemples :**

- **JayKonta** (hors portail client) : comptabilité personnelle et professionnelle
- **Outils internes** de gestion, planification, etc.
- **Services utilitaires** sans exposition externe

**Espaces à prévoir :**

```
┌─────────────────────────────────────────────────────────────┐
│                    Miyukini Central                         │
│  (Gestion, administration, utilisation complète du Service) │
└─────────────────────────────────────────────────────────────┘
```

---

### 2.2 Type 2 — Service à surface web externe

**Définition étendue :**

Un Service de Type 2 possède deux facettes :
1. **Facette interne** : gestion, administration, création — accessible via Miyukini Central
2. **Facette externe** : surface web destinée aux utilisateurs finaux (clients, visiteurs, prospects) — accessible via Miyukini Web Portal (le Portail)

L'utilisateur du COG prépare et gère dans Central ; les utilisateurs finaux consomment via le Portail.

**Caractéristiques :**

| Aspect | Règle |
|--------|-------|
| **Point d'entrée gestion** | Miyukini Central |
| **Point d'entrée consommation externe** | Miyukini Web Portal (Portail) |
| **Accès externe** | ✅ Oui, via Façade Publique Gouvernée |
| **Façade publique** | ✅ Oui, sous Mandat Public d'Accès |
| **Connexions entrantes** | Identifiées et fichées (BorderGuard) |

**Exemples :**

| Service | Facette interne (Central) | Facette externe (Portail) |
|---------|---------------------------|--------------------------|
| **JayXpose** | Profil entreprise, catalogue produits, page builder | Vitrine, e-shop, annuaire public, blog |
| **JayFestival** | Gestion éditions, exposants, budget, programme | Catalogue événements, inscriptions, billets, espace visiteur |
| **JayRDV** | Gestion agenda, services, paramètres | Page de réservation publique, parcours guest |
| **JayKonta** (portail client) | Gestion comptable | Portail client : consultation factures, paiement |

**Espaces à prévoir :**

```
┌─────────────────────────────────────────────────────────────┐
│                    Miyukini Central                         │
│  (Gestion, administration, création — utilisateur COG)      │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Expose les capacités
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Miyukini Web Portal                        │
│  (Façade publique gouvernée — utilisateurs externes)        │
│  · Vitrine, e-shop, réservation, catalogue public, etc.    │
│  · Identification et fichage des connexions entrantes       │
│  · BorderGuard + Visa + Mandat Public d'Accès               │
└─────────────────────────────────────────────────────────────┘
```

---

### 2.3 Type 3 — Service Inter-COG

**Définition étendue :**

Un Service de Type 3 est conçu pour des interactions entre plusieurs COGs. Les "utilisateurs" peuvent être d'autres COGs (participants, adversaires, partenaires) ou des utilisateurs issus d'autres COGs (via Visite gouvernée).

**Caractéristiques :**

| Aspect | Règle |
|--------|-------|
| **Point d'entrée local** | Miyukini Central (paramétrage, partie locale) |
| **Interactions** | Via Maillage Webway, Bridge inter-COG, Visite gouvernée |
| **Gouvernance** | Chaque COG reste souverain ; interactions sous contrat explicite |
| **Passeport / Visa** | Requis pour les sessions inter-COG |

**Exemples :**

- **Jeux multijoueur** : les joueurs viennent d'autres COGs, sessions hébergées sur un COG Hébergeur
- **Collaboration expert JayKonta** : un expert-comptable (autre COG) accède à la compta d'un client
- **Fédération inter-COG** : consolidation multi-entités souveraines

**Espaces à prévoir :**

```
┌─────────────────────────────────────────────────────────────┐
│                    Miyukini Central                         │
│  (Paramétrage local, configuration du service Inter-COG)    │
└─────────────────────────────────────────────────────────────┘
                              │
                              │ Protocoles Inter-COG
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Maillage Webway / Bridge inter-COG             │
│  · Déclaration d'hébergement de session                     │
│  · Passeport Utilisateur / Visa de Connexion               │
│  · Visite gouvernée inter-COG                               │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. Services Fondamentaux

### 3.1 Définition

Les **Services Fondamentaux** sont les points d'entrée structurels de l'écosystème COG. Leur présence fait partie de l'environnement versionné du COG. Ils ne sont pas optionnels.

| Service Fondamental | Rôle | Cible |
|---------------------|------|-------|
| **Miyukini Central** | Hub de gestion des Services — point d'entrée utilisateur COG | Utilisateur du COG |
| **Miyukini Web Portal** | Hub des surfaces web — point d'entrée utilisateurs externes | Utilisateurs externes (web) |

### 3.2 Règle canonique

> **Central = COG, Portail = Web.**
>
> Miyukini Central est le point d'accès pour l'utilisateur du COG.  
> Miyukini Web Portal est le point d'accès pour les utilisateurs externes via le web.
>
> Ces deux Services Fondamentaux font partie intégrante de l'environnement versionné du COG.

### 3.3 Position dans l'environnement

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Environnement COG (versionné)                     │
│                                                                      │
│  ┌────────────────────────────┐  ┌────────────────────────────┐    │
│  │   Miyukini Central         │  │   Miyukini Web Portal      │    │
│  │   (Service Fondamental)    │  │   (Service Fondamental)    │    │
│  │   · Point d'entrée COG     │  │   · Point d'entrée Web     │    │
│  │   · Utilisateur COG        │  │   · Utilisateurs externes  │    │
│  └────────────────────────────┘  └────────────────────────────┘    │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    Services (Type 1, 2, 3)                    │  │
│  │   · JayXpose, JayFestival, JayRDV, JayKonta, ...             │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    Strate Cores (immuable)                    │  │
│  │   · StrongFather, KindMother, BorderGuard, ...               │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Règles pour les Nouveaux Services

### 4.1 Checklist obligatoire

Lors de la création ou de la documentation d'un nouveau Service :

| # | Question | Action |
|---|----------|--------|
| 1 | **Quel est le type du Service ?** | Déclarer Type 1, 2 ou 3 dans le document fondateur |
| 2 | **Quels espaces sont prévus ?** | Lister : Central uniquement / Central + Portail / Central + Inter-COG |
| 3 | **Si Type 2 : quelle surface web ?** | Décrire la façade publique (quels parcours, quelles capacités exposées) |
| 4 | **Si Type 3 : quels protocoles ?** | Décrire les interactions inter-COG (Webway, Visite gouvernée, contrats) |

### 4.2 Section à ajouter dans le Document Fondateur

Tout document fondateur de Service doit inclure :

```markdown
## Type de Service et Espaces

| Aspect | Valeur |
|--------|--------|
| **Type** | [1 / 2 / 3] |
| **Description** | [Service interne COG / Service à surface web externe / Service Inter-COG] |
| **Espace Central** | ✅ [Description de la facette gestion] |
| **Espace Portail** | [✅ / ❌] [Si oui : description de la surface web] |
| **Espace Inter-COG** | [✅ / ❌] [Si oui : description des interactions] |
```

---

## 5. Tableau Récapitulatif des Services Existants

| Service | Type | Central | Portail | Inter-COG | Surface externe |
|---------|------|---------|---------|-----------|-----------------|
| **Miyukini Central** | Fondamental | ✅ (lui-même) | ❌ | ❌ | — |
| **Miyukini Web Portal** | Fondamental | ✅ (gestion) | ✅ (lui-même) | ❌ | Toutes les façades publiques |
| **JayXpose** | 2 | ✅ Gestion exposant | ✅ Vitrine, e-shop, annuaire | ❌ | Clients finaux, visiteurs web |
| **JayFestival** | 2 | ✅ Gestion éditions | ✅ Catalogue, billets, espace visiteur | ❌ | Visiteurs festivals |
| **JayRDV** | 2 | ✅ Gestion agenda | ✅ Réservation publique | ❌ | Clients (patients, etc.) |
| **JayKonta** | 1 + 2 | ✅ Comptabilité | ✅ Portail client (P2) | ❌ | Clients (factures, paiement) |
| **JayKoa** | 1 | ✅ Agenda personnel | ❌ | ❌ | — |
| **MiyukiniClicker** | 3 | ✅ Jeu local | ❌ | ✅ Multijoueur | Joueurs d'autres COGs |
| **Lord of the Castle** | 3 | ✅ Jeu local | ❌ | ✅ Multijoueur | Joueurs d'autres COGs |

---

## 6. Phrases Fondatrices

> **Central = COG, Portail = Web.**

> **Tout Service doit déclarer son type et prévoir les espaces correspondants.**

> **Les Services Fondamentaux (Central, Portail) font partie de l'environnement versionné du COG.**

> **Un Service de Type 2 expose une Façade Publique Gouvernée via le Portail, jamais directement.**

---

## 7. Références

| Document | Lien |
|----------|------|
| **Miyukini Central** | [Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) |
| **Miyukini Web Portal** | [MiyukiniWebPortal/_index.md](../services/MiyukiniWebPortal/_index.md) |
| **Glossaire** | [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Façade Publique Gouvernée** | [Glossaire § Façade Publique Gouvernée](./Miyukini%20Conceptual%20References%20-%20Glossaire.md#façade-publique-gouvernée-public-exposure-surface) |
| **Souveraineté Environnement** | [Souveraineté Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |
| **Visite gouvernée inter-COG** | [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** Document de référence normatif — CLASSIFICATION OFFICIELLE DES TYPES DE SERVICES
