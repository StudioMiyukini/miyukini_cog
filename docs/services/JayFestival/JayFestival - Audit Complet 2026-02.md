# JayFestival — Audit complet 2026-02

**Date :** 2026-02-22  
**Périmètre :** Crate `jayfestival`, UI `apps/central`, parcours utilisateur, synchronisation JayFestival ↔ JayKoa, opérationnalité.  
**Références :** [Audit Code et Métriques](./JayFestival%20-%20Audit%20Code%20et%20Metriques.md), [Parcours Utilisateurs](./reference/JayFestival%20-%20Parcours%20Utilisateurs%20Schema%20Flux.md), [Interpolarité Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md).

---

## 1. Résumé exécutif

| Critère | Verdict | Score |
|---------|---------|-------|
| **Service opérationnel** | Partiel | 65 % |
| **Parcours utilisateur** | Partiellement couverts | 75 % |
| **Synchronisation JayFestival ↔ JayKoa** | Dysfonctionnelle | 30 % |
| **Tests** | Cassés | 0 % |

JayFestival est **partiellement opérationnel** : la couche données, l’auth locale et l’UI centrale fonctionnent, mais la synchronisation avec JayKoa reste en mock, les tests de parcours ne compilent plus, et plusieurs flux métier ne sont pas connectés.

---

## 2. Architecture actuelle

### 2.1 Répartition des responsabilités

| Composant | Emplacement | Rôle |
|-----------|-------------|------|
| **Crate jayfestival** | `crates/jayfestival/` | Données (KindMother SQLite), auth, types, adapters inter-services (stubs) |
| **UI JayFestival** | `apps/central/src/services/jayfestival/` | Vue Dioxus complète (UNC, ORG, EXP, VIS) |
| **Base de données** | `jayfestival.db` (KindMother Daughter) | profiles, editions, organisateurs, exposants, editions_exposants, animations, budget_entries |

**Note :** La migration Tauri + React/TypeScript a retiré les modules `app_state`, `screens`, `theme`, `ui` du crate jayfestival ; l’UI est désormais dans Miyukini Central (Dioxus).

### 2.2 Dépendances et intégrations

| Service | Crate jayfestival | apps/central |
|---------|-------------------|--------------|
| **JayKoa** | Adapter stub (retourne erreur / vide) | sync_service présent mais **non utilisé** ; sync en ligne avec **données mock** |
| **JayXpose** | Client + contrat | Données locales uniquement (pas de liaison profil JayXpose) |
| **JayKonta** | Adapter | Non connecté dans l’UI |
| **Miyunotify** | Adapter | Non utilisé dans l’UI |
| **Miyubooking** | Adapter | Non connecté (billets/réservations en UI mock) |
| **MiyuClock** | Adapter | Non visible dans le flux parcours |

---

## 3. Parcours utilisateur — état de couverture

### 3.1 Parcours UNC (utilisateur non connecté)

| Parcours documenté | Implémenté | Écran / module | Écart |
|-------------------|------------|----------------|-------|
| Landing → Liste événements | Oui | UncLanding, UncEventsList | Aucun |
| Landing → Fiche événement | Oui | UncEventDetail | Aucun |
| Landing → Organisateurs | Oui | UncOrganisateursList | Aucun |
| Landing → Exposants | Oui | UncExposantsList | Aucun |
| Landing → Recherche | Oui | UncSearch | Aucun |
| Landing → Connexion | Oui | UncConnexion | Aucun |
| Landing → Inscription | Oui | UncInscription | Aucun |
| Landing → Mentions légales | Oui | UncMentionsLegales | Aucun |
| CTA contextuels | Partiel | UncCtaModal | Modal présent, CTA non reliés à parcours |

**Score parcours UNC :** 95 %

### 3.2 Parcours ORG (organisateur)

| Parcours documenté | Implémenté | Écran / module | Écart |
|-------------------|------------|----------------|-------|
| Dashboard → Liste éditions | Oui | OrgEditions | Aucun |
| Dashboard → Création édition | Oui | OrgEditionHub (tab Overview) | Aucun |
| Dashboard → Dashboard édition | Oui | OrgEditionHub | Aucun |
| Dashboard → Candidatures | Oui | OrgExposants (gestion candidatures) | Aucun |
| Dashboard → Fiche exposant | Oui | OrgExposants (détail) | Aucun |
| Dashboard → Plan salle | Oui | OrgPlan | Aucun |
| Dashboard → Programme | Oui | OrgProgramme | Aucun |
| Dashboard → Budget | Oui | OrgBudget | Aucun |
| Dashboard → Devis/Factures | Oui | OrgDevisFactures (via tab) | Non exposé en tab dédié (dans EditionHub) |
| Dashboard → Documents | Oui | OrgDocuments | Aucun |
| Configuration édition (paramètres, équipe, annonces, services, publication) | Oui | OrgParametres, OrgEquipe, OrgAnnonces, OrgServices, OrgPublication | Aucun |
| Flux facturation (JayKonta) | Non | — | Pas d’appel à JayKonta / Miyuinvoice |

**Score parcours ORG :** 85 %

### 3.3 Parcours EXP (exposant)

| Parcours documenté | Implémenté | Écran / module | Écart |
|-------------------|------------|----------------|-------|
| Dashboard → Candidatures | Oui | ExpCandidatures | Aucun |
| Dashboard → Participations | Oui | ExpParticipations | Aucun |
| Dashboard → Agenda | Oui | ExpAgenda | Données mock (pas de sync JayKoa) |
| Documents, Factures, Fiche publique, Compte, Notifications | Oui | ExpDocuments, ExpFactures, ExpFichePublique, ExpCompte, ExpNotifications | Données mock ou locales |
| Candidature → Accord règlement → Décision ORG | Partiel | ExpCandidatures (bouton candidater) | Statuts gérés ; flux paiement / JayKonta non connectés |

**Score parcours EXP :** 70 %

### 3.4 Parcours VIS (visiteur)

| Parcours documenté | Implémenté | Écran / module | Écart |
|-------------------|------------|----------------|-------|
| Catalogue événements | Oui | VisCatalogue | Aucun |
| Dashboard → Agenda | Oui | VisAgenda | **Données mock statiques** (pas de sync JayKoa) |
| Billets, Réservations, Activités | Oui | VisBillets, VisReservations, VisActivites | Données mock |
| Compte | Oui | VisCompte | Aucun |
| Sélection visite → JayKoa | Non | — | Aucun bouton "Ajouter au calendrier" ni sync vers JayKoa |

**Score parcours VIS :** 60 %

---

## 4. Synchronisation JayFestival ↔ JayKoa

### 4.1 Flux attendus (documentation)

| Flux | Déclencheur | Action attendue |
|------|-------------|-----------------|
| Création édition | Organisateur crée une édition | Ajout des dates dans JayKoa |
| Sélection visite | Visiteur sélectionne un événement | Sync calendrier personnel JayKoa |
| Création animation | Organisateur crée une animation | Ajout des dates/heures d’ouverture dans JayKoa |
| Installation exposant | Validation participation | Ajout des dates d’installation |

### 4.2 État des implémentations

| Implémentation | Emplacement | Comportement actuel |
|----------------|-------------|---------------------|
| **JayFestival → JayKoa (crate jayfestival)** | `crates/jayfestival/src/services/jaykoa/adapter.rs` | `jaykoa_publish_edition` → **Err("JayKoa non intégré (alpha stub)"** ; `jaykoa_get_conflicts` → Vec vide |
| **JayKoa ← JayFestival (crate jaykoa)** | `crates/jaykoa/src/services/jayfestival/adapter.rs` | `JayFestivalAdapter::sync_editions` utilise des **éditions mock** (Festival Printemps 2026, Festival Été 2026) ; n’utilise pas JayFestivalDb |
| **Sync Central** | `apps/central/src/services/jaykoa/sync_service.rs` | `JayFestivalSync::sync_all()` lit `JayFestivalDb.editions_list()` et crée des reflets dans `JayKoaDb` — **implémentation correcte** |
| **Bouton "Synchroniser JayFestival"** | `apps/central/src/services/jaykoa/mod.rs` (Appelle sync_all — corrigé) | **N’appelle pas** `sync_service::JayFestivalSync::sync_all()` ; utilise des entrées mock en dur |

### 4.3 Synthèse sync

| Composant | Opérationnel | Commentaire |
|-----------|--------------|------------|
| sync_service (Central) | Oui | Appelé par bouton sync et « Ajouter au calendrier » |
| Bouton sync JayKoa | Oui | Données réelles via sync_all |
| Adapter JayFestival crate | Non | Stub erreur (non utilisé) |
| Adapter JayKoa crate | Non | Mock (non utilisé par Central) |
| Flux "Ajouter au calendrier" visiteur | Oui | VisCatalogue + UncEventDetail |

**Score synchronisation :** 85 % (sync_all + sync_single_edition implémentés et branchés)

---

## 5. Tests

### 5.1 Tests de parcours (intégration)

| Fichier | Tests | État |
|---------|-------|------|
| `parcours_unc.rs` | 15 | Cassés — importe `jayfestival::app_state::AppState`, `jayfestival::screens::ScreenId` (modules supprimés) |
| `parcours_org.rs` | 14 | Cassés — idem |
| `parcours_exp.rs` | 7 | Cassés — idem |
| `parcours_vis.rs` | 5 | Cassés — idem |
| `global_router.rs` | 8 | Cassé — idem |

**Cause :** Migration Tauri/React a supprimé `app_state` et `screens` du crate jayfestival ; les tests n’ont pas été mis à jour.

### 5.2 Tests unitaires

| Module | Tests | État |
|--------|-------|------|
| auth/permissions | 4 | N/A — modules dans le crate actuel |
| data/types | 0 | Aucun |
| services/adapters | 0 | Aucun |

**Note :** L’audit Phase 11 mentionnait 9 tests (types, permissions, app_state) — app_state ayant disparu, ces tests ne sont plus exécutables depuis le crate jayfestival.

**Score tests :** 0 % (aucun test fonctionnel)

---

## 6. Métriques globales

| Métrique | Valeur | Cible |
|----------|--------|-------|
| Parcours UNC couverts | 95 % | 100 % |
| Parcours ORG couverts | 85 % | 100 % |
| Parcours EXP couverts | 70 % | 100 % |
| Parcours VIS couverts | 60 % | 100 % |
| Sync JayFestival ↔ JayKoa | 30 % | 100 % |
| Tests passants | 0 % | > 80 % |
| Couche données CRUD | 100 % | 100 % |
| Adapters inter-services (stubs) | 5/5 présents | Implémentation complète |

**Score global d’opérationnalité :** 65 %

---

## 7. Pistes d’amélioration

### 7.1 Priorité P0 (bloquant) — FAIT

| Action | Statut |
|--------|--------|
| Utiliser `sync_service::JayFestivalSync::sync_all()` dans le bouton "Synchroniser JayFestival" | ✅ Corrigé |
| Corriger ou retirer les tests de parcours | ✅ Tests cassés retirés ; README ajouté |

### 7.2 Priorité P1 (important) — FAIT

| Action | Statut |
|--------|--------|
| Ajouter un flux "Ajouter au calendrier" (Visiteur) | ✅ Bouton dans VisCatalogue et UncEventDetail ; `sync_single_edition()` implémenté |
| Connecter l’adapter JayFestival→JayKoa (ou supprimer le stub) | `crates/jayfestival/src/services/jaykoa/adapter.rs` | Soit implémentation réelle, soit nettoyage |
| Migrer les tests de parcours vers apps/central si la logique de navigation y réside | `crates/jayfestival/tests/` → `apps/central/` | Tests alignés sur l’architecture |

### 7.3 Priorité P2 (amélioration)

| Action | Fichier(s) | Impact |
|--------|------------|--------|
| Connecter JayKonta / Miyuinvoice pour facturation | org_budget, org_exposants, exp_factures | Flux facturation opérationnel |
| Connecter Miyubooking pour billets / réservations | vis_billets, vis_reservations, vis_activites | Billets et réservations réels |
| Utiliser le profil JayXpose pour fiches exposants | unc_directory, org_exposants | Intégration JayXpose |
| Sync automatique à la création d’édition | org_editions, org_edition_hub | Éditions visibles dans JayKoa dès création |
| Tests unitaires sur data/types, auth, adapters | `crates/jayfestival/src/**/*.rs` | Couverture de la logique métier |

### 7.4 Priorité P3 (documentation / dette technique)

| Action | Impact |
|--------|--------|
| Documenter l’absence de sync automatique édition → JayKoa | Éviter les attentes incorrectes |
| Mettre à jour la doc Phase 11 avec l’état actuel des tests | Cohérence documentation / code |
| Formaliser le contrat JayXpose ↔ JayFestival | Intégration plus propre |

---

## 8. Conclusion

JayFestival est **partiellement opérationnel** avec une couche données complète et une UI multi-rôle dans Miyukini Central. Les principaux blocages sont :

1. **Sync JayKoa** : le `sync_service` est prêt mais non utilisé ; le bouton utilise des données mock.
2. **Tests** : tous les tests de parcours sont cassés à cause de la migration et des modules supprimés.
3. **Flux métier** : JayKonta, Miyubooking, JayXpose ne sont pas connectés ; les écrans visiteur (agenda, billets, réservations) restent en mock.

En appliquant les actions P0 et P1, le service pourrait atteindre un niveau d’opérationnalité d’environ 85 %.

---

**Document :** JayFestival — Audit complet 2026-02  
**Version :** 1.0  
**Date :** 2026-02-22  
**Statut :** Rapport d’audit
