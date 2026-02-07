# JayKoa - Axes Amelioration et Plan Implementation

## Contexte

Ce document constitue l'audit d'implementation complet du Service JayKoa, croisant l'etat actuel du code (crate jaykoa worktree kxh) avec la documentation fondatrice validee. Il identifie tous les axes de developpement necessaires pour obtenir un livrable complet.

## Portee / Scope

- Applicable a : Implementation du crate jaykoa et son integration dans miyukini-central
- Audience : Developpeurs, agents IA, architectes
- Base de reference : Worktree kxh crates/jaykoa/
- Documentation de reference : docs/services/JayKoa/

---

## Etat actuel

### Ce qui est implemente

| Domaine | Etat | Detail |
|---------|------|--------|
| Modele domaine | OK | Agenda, TemporalEntry, EntryType, TemporalStatus, EventSource, TemporalConflict, UserSettings |
| Persistance SQLite | OK | Schema, CRUD agendas, CRUD entries, reflets externes, settings, detect_conflicts |
| UI calendrier egui | OK | Vues semaine, jour, mois, planning. Header, sidebar, mini-calendrier |
| Composants Atomic Design | OK | Atoms (4), Molecules (4), Organisms (3) |
| Adaptateurs sync | STUB | JayFestivalAdapter et JayRDVAdapter avec mock data hardcodee |
| Integration Central | BASIQUE | JayKoaService wrapping JayKoaApp, tab fonctionnelle |

### Ce qui manque

- Gouvernance (WriteIntent, StrongFather, MasterButler, WorrySentinel) = 0%
- Export iCal = 0%
- Conflits cross-agendas = 0%
- Liaison Central profile_service_refs = 0%
- Tests = 0%
- Filtres UI avances = 0%

---

## AXE 1 - Gouvernance Cores

Criticite : HAUTE | Ref : Bornage CF-MVP-5

### 1.1 WriteIntent via KindMother (CF-MVP-5)

Toutes les ecritures SQLite dans kindmother_db.rs sont directes. Aucune ne passe par WriteIntent ni CoreDataAPI. API disponible dans kindmother : WriteIntent, CoreDataAPI submit_write_intent, WriteOperation Create/Update/Delete.

Fichier : crates/jaykoa/src/data/kindmother_db.rs lignes 122-288

Action : Wrapper chaque methode CRUD pour construire un WriteIntent avant SQL. Flow : construire WriteIntent, soumettre a CoreDataAPI, appliquer SQL si accepte.

Priorite : MVP obligatoire

### 1.2 StrongFather - Decisions

Aucune validation de decision avant les operations. Crate : crates/strongfather/ (Decision, Intent, PolicyEngine, Validator).

Priorite : Post-MVP immediat

### 1.3 MasterButler - Capacites et Permissions

Les capacites JayKoa ne sont pas declarees. Crate : crates/masterbutler/.

Action : Declarer les capacites (entries.create, entries.read, conflicts.check, export.ical) et verifier les permissions.

Priorite : Post-MVP immediat

### 1.4 WorrySentinel - Niveaux de securite

Aucune gestion des niveaux S0-S4 ni des etats T0-T4. Crate : crates/worrysentinel/.

Action : Taguer les entrees par niveau, restreindre en etat degrade (AGD-SEC-4), filtrer export (AGD-SEC-3).

Priorite : Post-MVP immediat

### 1.5 Mandat de Permission inter-Services

Les adaptateurs sync mentionnent BondingBrother en commentaire. Crate : crates/bondingbrother/.

Priorite : Post-MVP immediat

---

## AXE 2 - Export iCal (CF-MVP-4)

Criticite : HAUTE | Ref : CF-MVP-4, AGD-UI-03, AGD-SEC-3

### 2.1 Generation iCal

Aucune fonctionnalite d'export. Le champ recurrence_rule existe mais inexploite.

Action : Creer src/export/ical.rs generant un .ics RFC 5545. Respecter AGD-SEC-3.

Priorite : MVP obligatoire

### 2.2 Export PDF

Priorite : Phase 2

### 2.3 Bloc export UI (AGD-UI-03)

Aucun composant UI pour declencher l'export.

Action : Creer src/screens/export.rs.

Priorite : MVP obligatoire

---

## AXE 3 - Detection de conflits

Criticite : MOYENNE-HAUTE | Ref : CF-MVP-2, AGD-UI-02/06, AGD-SEC-6

### 3.1 Conflits cross-agendas

detect_conflicts() ligne 341 filtre a.agenda_id != b.agenda_id, ignorant les conflits entre agendas differents.

Action : Supprimer le filtre pour comparer toutes les entrees.

Priorite : MVP obligatoire

### 3.2 API de requete de conflits

Pas de point d'entree pour les services consommateurs.

Action : Exposer check_conflict(profile_id, start, end) retournant ConflictResult.

Priorite : MVP obligatoire (CF-MVP-2)

### 3.3 Conflits presence physique (AGD-SEC-6)

Regle : ne pas bloquer, notifier, pousser resolution via indicateurs rouges clignotants.

Priorite : MVP

### 3.4 Alerte conflit avant validation (AGD-UI-02)

Pas d'alerte avant soumission d'action.

Action : Dans event_create.rs, appeler check_conflict() avant save_entry().

Priorite : MVP (CF-MVP-2)

---

## AXE 4 - Integration Central

Criticite : HAUTE | Ref : Plan etape 5

### 4.1 Liaison profil Central vers JayKoa

La table profile_service_refs existe dans Central mais JayKoa utilise un profile_id hardcode.

Action : JayKoaService recoit le profile_id actif du Central.

Priorite : MVP obligatoire

### 4.2 Chargement automatique des agendas

Priorite : MVP obligatoire

### 4.3 Metadata catalog

Version marquee 0.1.0 (mock). Priorite : Basse

---

## AXE 5 - Synchronisation inter-Services

Criticite : HAUTE | Ref : Plan etape 4

### 5.1 JayFestival Adapter reel

Contient 2 editions hardcodees.

Action : Requete via BondingBrother sous Mandat.

Priorite : MVP (CF-MVP-1)

### 5.2 JayRDV Adapter reel

Priorite : Phase 2

### 5.3 Publication d'entrees (flux push)

Priorite : Post-MVP

### 5.4 Sync differee / offline

Priorite : Post-MVP

---

## AXE 6 - Composants UI manquants

Criticite : MOYENNE

### 6.1 Vue Annee

Placeholder dans calendar.rs ligne 36. Priorite : Phase 3

### 6.2 Filtres source/type (AGD-UI-04)

Action : Panneau de filtres sidebar/header : source_service, entry_type, statut, periode.

Priorite : MVP

### 6.3 Indicateur prochaine entree (AGD-UI-05)

Priorite : Post-MVP

### 6.4 Vue Liste/Agenda (AGD-UI-07)

Priorite : Phase 2

### 6.5 Indicateur libre/occupe (AGD-UI-08)

Priorite : Phase 2

### 6.6 Catalogue evenements publics (AGD-UI-09/10)

Priorite : Phase 2

---

## AXE 7 - Operateurs et Kits d'Outils (Strate 6-7)

Criticite : MOYENNE

### 7.1 Structuration en Operateurs

Code structure en modules techniques, pas en Operateurs gouvernes.

Action : Creer src/operators/ avec entries.rs, conflicts.rs, view_export.rs, public_events.rs.

Priorite : Post-MVP

### 7.2 Kits d'Outils

Priorite : Post-MVP

---

## AXE 8 - Securite et Protection (AGD-SEC)

Criticite : MOYENNE

### 8.1 AGD-SEC-1 : References seulement

Reflets stockent titre, description, lieu au lieu de references opaques. Priorite : Post-MVP

### 8.2 AGD-SEC-2 : Mandat pour agregation

Priorite : Post-MVP

### 8.3 AGD-SEC-3 : Export filtre par niveau

Priorite : MVP (couple a AXE 2)

### 8.4 AGD-SEC-5 : Niveau declare par source

Priorite : Post-MVP

---

## AXE 9 - Qualite et Maintenabilite

### 9.1 Tests unitaires

0 test dans le crate. Action : Tests pour types.rs, kindmother_db.rs, ical.rs, adaptateurs.

Priorite : MVP

### 9.2 Tests d'integration

Priorite : Post-MVP

### 9.3 README du crate

Aucun README.md. Priorite : MVP

### 9.4 Code mort

5 helpers inutilises dans types.rs lignes 357-385 : parse_date, parse_datetime, parse_time, format_datetime, format_date.

Action : Utiliser ou supprimer. Priorite : MVP

### 9.5 Fonctions DB potentiellement inutilisees

entries_by_agenda() et reflect_clear_service() non appelees. Priorite : Basse

### 9.6 Dependance serde_json inutilisee

Declaree dans Cargo.toml mais jamais utilisee. Action : Supprimer ou utiliser. Priorite : MVP

### 9.7 MSCM annotations

@role et @human manquent dans certains fichiers. Priorite : Basse

---

## AXE 10 - Parcours utilisateurs complets

Criticite : BASSE (Phase 2+)

### 10.1 Parcours decouvrir evenements publics

Priorite : Phase 2

### 10.2 Parcours partage agenda

Priorite : Phase 2

---

## Synthese MVP strict (15 taches)

| No | Axe | Tache | Ref |
|----|-----|-------|-----|
| 1 | 1.1 | WriteIntent via KindMother | CF-MVP-5 |
| 2 | 2.1 | Export iCal | CF-MVP-4 |
| 3 | 2.3 | Bloc export UI | AGD-UI-03 |
| 4 | 3.1 | Conflits cross-agendas | CF-MVP-2 |
| 5 | 3.2 | API requete conflits | CF-MVP-2 |
| 6 | 3.3 | Conflits presence physique | AGD-SEC-6 |
| 7 | 3.4 | Alerte conflit avant validation | AGD-UI-02 |
| 8 | 4.1 | Liaison Central profile_service_refs | Etape 5 |
| 9 | 4.2 | Chargement auto agendas | Etape 5 |
| 10 | 5.1 | JayFestival adapter reel | CF-MVP-1 |
| 11 | 6.2 | Filtres source/type UI | AGD-UI-04 |
| 12 | 9.1 | Tests unitaires | Qualite |
| 13 | 9.3 | README crate | Qualite |
| 14 | 9.4 | Nettoyage code mort | Qualite |
| 15 | 9.6 | Nettoyage dependance | Qualite |

## Synthese Post-MVP immediat (11 taches)

| No | Axe | Tache |
|----|-----|-------|
| 16 | 1.2 | StrongFather decisions |
| 17 | 1.3 | MasterButler capacites |
| 18 | 1.4 | WorrySentinel securite |
| 19 | 1.5 | BondingBrother mandats |
| 20 | 5.2 | JayRDV adapter reel |
| 21 | 5.3 | Publication entrees push |
| 22 | 5.4 | Sync differee offline |
| 23 | 7.1 | Structuration Operateurs |
| 24 | 7.2 | Kits d'Outils |
| 25 | 8.1-8.4 | Regles AGD-SEC completes |
| 26 | 9.2 | Tests integration |

## Synthese Phase 2 (6 taches)

| No | Axe | Tache |
|----|-----|-------|
| 27 | 2.2 | Export PDF |
| 28 | 6.4 | Vue Liste/Agenda |
| 29 | 6.5 | Indicateur libre/occupe |
| 30 | 6.6 | Catalogue evenements publics |
| 31 | 10.1 | Parcours decouvrir evenements |
| 32 | 10.2 | Parcours partage agenda |

## Synthese Phase 3+ (2 taches)

| No | Axe | Tache |
|----|-----|-------|
| 33 | 6.1 | Vue Annee |
| 34 | 6.3 | Indicateur prochaine entree |

---

## Criteres de fin MVP

| Critere | Description | Axe |
|---------|-------------|-----|
| CF-MVP-1 | Entrees publiees par au moins un consommateur | 5.1 |
| CF-MVP-2 | Detection conflit fonctionnelle + alerte | 3.1, 3.2, 3.4 |
| CF-MVP-3 | Vue calendrier disponible | Deja OK |
| CF-MVP-4 | Export iCal disponible | 2.1, 2.3 |
| CF-MVP-5 | Gouvernance en place | 1.1 |
| CF-MVP-6 | Documentation a jour | Ce document |

---

Document : JayKoa - Axes Amelioration et Plan Implementation
Version : 1.0
Date : 2026-02-06
Statut : Document de reference operationnel
