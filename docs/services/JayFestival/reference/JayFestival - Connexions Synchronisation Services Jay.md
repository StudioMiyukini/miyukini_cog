# JayFestival — Connexions, dépendances, liaisons et synchronisation avec les services Jay

## Contexte

Ce document décrit de façon **exhaustive** les **connexions** entre JayFestival et les services Jay (JayKoa, JayXpose, JayKonta) : dépendances techniques, liaisons métier, bornes d’implémentation et **implémentation de la synchronisation**. Il complète [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) avec les détails d’architecture et de code.

**Références** : [JayXpose - Synchronisation JayFestival](../../JayXpose/JayXpose%20-%20Synchronisation%20JayFestival.md), [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md), [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md).

---

## 1. Vue d’ensemble des connexions

### 1.1 Graphe de dépendances techniques (Cargo)

```
                    ┌──────────────────┐
                    │  apps/central     │
                    │  (Miyukini Central)│
                    └────────┬─────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
         ▼                   ▼                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ jayfestival │     │   jaykoa    │     │  jayxpose   │
└──────┬──────┘     └──────▲──────┘     └─────────────┘
       │                   │
       │  (aucune dep      │  (aucune dep
       │   vers jaykoa     │   vers jayfestival
       │   ou jayxpose)    │   au niveau crate)
       │                   │
       ▼                   │
┌──────────────────────────┴─────────────────────┐
│  jayfestival dépend de :                        │
│  kindmother, kindmother-client                  │
│  miyubooking, miyuinvoice, miyunotify, miyuclock│
│  rusqlite (legacy-sqlite)                       │
└────────────────────────────────────────────────┘
```

| Crate | Dépend de JayFestival ? | Dépend de JayKoa ? | Dépend de JayXpose ? | Dépend de JayKonta ? |
|-------|-------------------------|--------------------|----------------------|----------------------|
| **jayfestival** | — | Non | Non | Non |
| **jaykoa** | Non | — | Non | Non |
| **jayxpose** | Non | Non | — | Non |
| **jaykonta** | Non | Non | Non | — |
| **apps/central** | Oui | Oui | Oui | Oui |

**Point d’orchestration** : **Miyukini Central** (apps/central) est le seul composant qui agrège jayfestival, jaykoa, jayxpose et jaykonta. Les liaisons inter-services sont réalisées au niveau de l’application, pas au niveau des crates.

---

### 1.2 Graphe des flux métier

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          JAYFESTIVAL (événements, éditions, exposants)       │
└─────────────────────────────────────────────────────────────────────────────┘
        │                    │                    │                    │
        │ LECTURE            │ PUBLIE             │ LIT                │ ÉMET
        │ annuaire           │ éditions           │ devis/factures     │
        ▼                    ▼                    │                    ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│   JAYXPOSE    │    │    JAYKOA     │    │   JAYKONTA    │    │  MIYUINVOICE   │
│ (profil       │    │ (calendrier   │    │ (compta       │    │ (façade       │
│  exposant)    │    │  agrégé)      │    │  unifiée)     │    │  devis/        │
│               │    │               │    │               │    │  factures)     │
└───────▲───────┘    └───────▲───────┘    └───────────────┘    └───────────────┘
        │                    │
        │ Source vérité      │ Récepteur temporel
        │ profil, catalogue  │ reflets lecture seule
```

| Liaison | Direction | Nature | Données échangées |
|---------|-----------|--------|-------------------|
| JayFestival → JayXpose | Lecture | JayFestival lit JayXpose | Profil exposant, catalogue, documents partagés |
| JayFestival → JayKoa | Publication | JayFestival publie vers JayKoa | Éditions, participations (reflets agenda) |
| JayFestival → JayKonta | Émission | JayFestival émet via Miyuinvoice | Devis, factures, mouvements budget |
| JayXpose → JayFestival | (Indirect) | JayFestival lit données exposant | Annuaire exposants, fiche par édition |

---

## 2. Bornes d’implémentation (alpha vs post-alpha)

### 2.1 Tableau des bornes par service

| Service | Borne alpha | Borne post-alpha | Implémentation actuelle |
|---------|-------------|------------------|-------------------------|
| **JayXpose** | Annuaire local : JayFestival possède sa table `exposants` ; lecture locale uniquement. Optionnel : partage tables Supabase. | JayXpose = source de vérité ; JayFestival lit via BondingBrother / contrat. | Client JayXpose lit `JayFestivalDb.exposants` (données locales) |
| **JayKoa** | Sync manuelle : bouton « Synchroniser JayFestival » dans JayKoa ; éditions → reflets. | Sync automatique à la création/ modification d’édition ; conflits temps réel. | sync_service (Central) implémenté ; bouton utilise encore mock (à corriger) |
| **JayKonta** | Miyuinvoice en façade ; appels `quote.create`, `invoice.create` depuis JayFestival. | JayKonta backend ; opérateurs `budget.movements.record`, etc. | Adapter JayKonta (miyuinvoice) présent ; UI non connectée |
| **JayFaim** | Hors scope alpha | Phase 2 | Non implémenté |

### 2.2 Décisions P0 / P1 (tranchées)

| Décision | Contenu |
|----------|---------|
| **P0** | Miyuinvoice + JayKonta — facturation exposants = Miyuinvoice en façade, JayKonta en backend |
| **P0** | JayXpose dans l’alpha — parcours demande de stands et annuaire exposants en dépendent |
| **P1** | JayKoa organise les données et fait l’interface ; MiyuClock atteste l’horaire et la date IRL |
| **P1** | En alpha : données exposant peuvent être locales (table JayFestival) ou partagées Supabase |

---

## 3. Synchronisation JayFestival ↔ JayKoa

### 3.1 Principe : lecture réfléchie

JayKoa est le **récepteur temporel** : il reçoit des **reflets** (copies en lecture seule) des entrées agenda produites par JayFestival. Les reflets ne sont pas modifiables côté JayKoa.

### 3.2 Flux de synchronisation

```
[JayFestival]                    [JayKoa]
     │                               │
     │  editions_list()               │
     │  (JayFestivalDb)               │
     │                               │
     │  Pour chaque édition :         │
     │  → TemporalEntry              │
     │    entry_type: reflect_jayfestival
     │    source_service: jayfestival
     │    source_event_id: edition_id
     │                               │
     │  reflect_upsert(entry)        │
     └──────────────────────────────►│
                                     │
                              [Agenda "JayFestival"]
                              (agenda_id, visible)
```

### 3.3 Implémentation technique

| Composant | Emplacement | Rôle |
|-----------|-------------|------|
| **Sync service** | `apps/central/src/services/jaykoa/sync_service.rs` | `JayFestivalSync::sync_all(koa_db, festival_db, profile_id)` : lit `festival_db.editions_list()`, crée l’agenda JayFestival si besoin, appelle `koa_db.reflect_upsert()` pour chaque édition |
| **Bouton sync** | `apps/central/src/services/jaykoa/mod.rs` | `on_sync_jayfestival` : **à l’heure actuelle** utilise des données mock en dur au lieu d’appeler `sync_service::JayFestivalSync::sync_all()` |
| **Adapter crate jayfestival** | `crates/jayfestival/src/services/jaykoa/adapter.rs` | `jaykoa_publish_edition()` → Err (stub) ; `jaykoa_get_conflicts()` → Vec vide (stub). Non utilisé par Central |
| **Adapter crate jaykoa** | `crates/jaykoa/src/services/jayfestival/adapter.rs` | `JayFestivalAdapter::sync_editions()` : utilise des éditions mock (Festival Printemps 2026, etc.) ; ne lit pas JayFestivalDb. Utilisé par le crate jaykoa seul, pas par Central |

### 3.4 Données synchronisées

| Champ TemporalEntry | Source JayFestival |
|--------------------|--------------------|
| `title` | `edition.name` |
| `start_datetime` | `edition.start_date` + `T00:00:00` |
| `end_datetime` | `edition.end_date` + `T23:59:59` |
| `location` | `edition.location` |
| `entry_type` | `reflect_jayfestival` |
| `source_service` | `jayfestival` |
| `source_event_id` | `edition.id` ou `participation_{edition_id}` |
| `status` | Confirmed ou Cancelled si status = termine/annule |

### 3.5 Actions recommandées

1. **P0** : Dans `jaykoa/mod.rs`, remplacer le code mock par l’appel à `JayFestivalSync::sync_all(&conns.jaykoa, &conns.jayfestival, profile_id)`.
2. **P2** : Sync automatique à la création/modification d’une édition (hook dans org_editions / org_edition_hub).

---

## 4. Synchronisation JayXpose et annuaire des exposants JayFestival

### 4.1 Principe fondateur

> **Un exposant = un profil JayXpose = N participations JayFestival.** Pas de duplication.

JayXpose est la **source de vérité** pour le profil exposant (identité, catalogue, documents). JayFestival **lit** ces données pour alimenter l’annuaire et les fiches exposants.

### 4.2 Architecture des données exposant

#### Alpha (implémentation actuelle)

```
┌─────────────────────────────────────────────────────────────────┐
│  JayFestivalDb (jayfestival.db)                                 │
│  ┌─────────────────┐     ┌─────────────────────────┐            │
│  │  exposants       │     │  editions_exposants     │            │
│  │  (table locale)  │◄────│  (liaison édition↔exp) │            │
│  └────────┬─────────┘     └─────────────────────────┘            │
└───────────┼──────────────────────────────────────────────────────┘
            │
            │  jayxpose_list_repertoire(db, filters)
            │  jayxpose_fiche_by_id(db, exposant_id)
            │  → Lit JayFestivalDb, pas JayXposeDb
            ▼
┌─────────────────────────────────────────────────────────────────┐
│  JayXpose "client" (crates/jayfestival/src/services/jayxpose/)  │
│  → Contract types : JayXposeProfile, RepertoireItem              │
│  → Mapping Exposant → JayXposeProfile (format affichage)         │
└─────────────────────────────────────────────────────────────────┘
```

En alpha, le **client JayXpose** du crate jayfestival lit la table `exposants` de **JayFestivalDb** et mappe vers les types de contrat (`JayXposeProfile`, `RepertoireItem`). Il n’y a pas encore de lecture réelle depuis le crate `jayxpose` ou une base JayXpose distincte.

#### Post-alpha (cible)

```
┌─────────────────┐         ┌─────────────────┐
│  JayXposeDb     │  LECTURE │  JayFestivalDb  │
│  (exposants,    │◄─────────│  (editions_     │
│   catalogue,    │  BondingBrother / contrat  │  exposants
│   documents)    │          │  uniquement)    │
└─────────────────┘          └─────────────────┘
       ▲
       │ Annuaire = SELECT exposants WHERE visible_repertoire
       │ + JOIN editions_exposants pour répertoire par édition
```

### 4.3 Implémentation actuelle de l’annuaire

| Élément | Emplacement | Comportement |
|---------|-------------|--------------|
| **Liste répertoire** | `jayfestival::services::jayxpose::client::jayxpose_list_repertoire(db, filters)` | Appelle `db.exposants_list(true)` (visible_repertoire), applique filtres secteur/pagination, mappe vers `RepertoireItem` |
| **Fiche exposant** | `jayfestival::services::jayxpose::client::jayxpose_fiche_by_id(db, exposant_id)` | Appelle `db.exposant_by_id()`, mappe vers `JayXposeProfile` |
| **Répertoire par édition** | `JayFestivalDb` (requêtes custom) | Liste exposants avec `editions_exposants` WHERE edition_id, is_validated |
| **UI annuaire** | `apps/central/.../unc_directory.rs`, `org_exposants.rs` | Lit `conns.read().jayfestival.exposants_list()`, `exposant_by_id()` |

### 4.4 Contrat de données (JayXposeProfile, RepertoireItem)

| Type | Champs (extrait) | Usage |
|------|------------------|-------|
| **JayXposeProfile** | id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire | Fiche exposant complète |
| **RepertoireItem** | id, company_name, secteur, category, logo_url, site_web, description | Item liste répertoire (léger) |
| **RepertoireFilters** | secteur, limit, offset | Filtres et pagination |

### 4.5 Flux documentés (JayXpose - Synchronisation JayFestival)

D’après [JayXpose - Synchronisation JayFestival](../../JayXpose/JayXpose%20-%20Synchronisation%20JayFestival.md) :

| Flux | Alpha | Post-alpha |
|------|-------|------------|
| Profil public → Annuaire | Table `exposants` JayFestival (ou Supabase partagée) | Lecture JayXpose via contrat |
| Catalogue → Aperçu répertoire | Non implémenté (pas de produits_catalogue) | Lecture `produits_catalogue` JayXpose |
| Documents partagés → Candidature | Non implémenté | Via `documents_partages` + Mandat |
| Demande document → Exposant | Non implémenté | BondingBrother |

### 4.6 Actions recommandées

1. **P1** : Documenter explicitement que l’annuaire alpha lit la table locale `exposants` de JayFestival, pas JayXposeDb.
2. **P2** : Introduire une liaison JayFestival → JayXposeDb (via Central) lorsque le profil exposant sera géré par JayXpose.
3. **P2** : Afficher un aperçu catalogue (produits vedettes) dans la fiche exposant si JayXpose est connecté.

---

## 5. Liaison JayFestival ↔ JayKonta

### 5.1 Principe

JayFestival **consomme** JayKonta via **Miyuinvoice** en façade : devis et factures pour les exposants.

### 5.2 Implémentation

| Élément | Emplacement | Rôle |
|---------|-------------|------|
| **Adapter** | `crates/jayfestival/src/services/jaykonta/adapter.rs` | `jaykonta_create_quote(payload)`, `jaykonta_emit_invoice(quote_id_or_payload, from_quote)` → appels Miyuinvoice |
| **Dépendance** | `jayfestival/Cargo.toml` | `miyuinvoice = { path = "../miyuinvoice" }` |
| **UI** | `org_budget.rs`, `org_exposants.rs`, `exp_factures.rs` | Non connectée aux fonctions adapter (données mock ou locales) |

### 5.3 Bornes

- **Alpha** : Miyuinvoice en façade ; payload JSON conforme aux contrats Miyuinvoice.
- **Post-alpha** : JayKonta opérateurs `budget.movements.record`, intégration comptabilité édition.

---

## 6. Synthèse des liaisons et chemins de code

| Liaison | Chemin de code (Central ou crate) | État |
|---------|-----------------------------------|------|
| **JayFestival → JayKoa (sync éditions)** | `sync_service::JayFestivalSync::sync_all()` | Implémenté, non appelé par le bouton |
| **JayFestival → JayXpose (annuaire)** | `jayfestival::services::jayxpose::client` + `JayFestivalDb.exposants_list` | Lecture locale (table exposants JayFestival) |
| **JayFestival → JayKonta (devis/factures)** | `jayfestival::services::jaykonta::adapter` | Adapter prêt ; UI non connectée |
| **JayKoa ← JayFestival (bouton)** | `jaykoa/mod.rs` on_sync_jayfestival | Mock ; à remplacer par sync_all |

---

## 7. Références

- [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) — vue synthétique des couplages
- [JayXpose - Synchronisation JayFestival](../../JayXpose/JayXpose%20-%20Synchronisation%20JayFestival.md) — contrat d’intégration JayXpose ↔ JayFestival
- [JayKoa - Integration Services Consommateurs](../../JayKoa/reference/JayKoa%20-%20Integration%20Services%20Consommateurs.md) — types d’entrées, reflets
- [JayKonta - Integration Services](../../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) — flux budget, devis, factures
- [JayFestival - Bornage Implementation](../JayFestival%20-%20Bornage%20Implementation.md) — périmètre alpha, phase 2
- [JayFestival - Audit Complet 2026-02](../JayFestival%20-%20Audit%20Complet%202026-02.md) — état opérationnel, métriques

---

**Document** : JayFestival — Connexions, dépendances, liaisons et synchronisation  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Document de référence technique
