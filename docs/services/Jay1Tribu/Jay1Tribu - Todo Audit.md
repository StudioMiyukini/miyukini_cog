# Jay1Tribu — Todo Audit

## Contexte

Ce document est le **plan de remédiation** dérivé de l’audit Jay1Tribu réalisé le 2026-02-16. Il liste les tâches prioritaires pour atteindre une implémentation conforme à la documentation (Document Fondateur, Spécification Fonctionnelle, Guide d’implémentation, Contraintes et Invariants).

**Référence :** Audit Jay1Tribu (complétion, fonctionnement, qualité de code, protocole MIP, implantation vs doc).

## Portée / Scope

- **Applicable à :** Crate `jay1tribu`, intégration Central/Miou, conformité MIP.
- **Audience :** Développeurs, architectes, équipe produit.
- **Statut :** Plan d’action normatif — à traiter par priorité.

---

## Synthèse de l’audit

| Dimension | État | Complétion estimée |
|-----------|:----:|:------------------:|
| Complétion fonctionnelle | Partielle | ~45–50 % |
| Bon fonctionnement (lecture locale) | ✅ | OK |
| Qualité du code | ✅ Bonne | OK |
| Protocole MIP (MSCM) | ❌ | 0 % |
| Implantation vs doc | ⚠️ Écarts | Partiel |

---

## Sync avec JayXpose et JayKoa

Cette section décrit le **flux de données et les responsabilités** entre Jay1Tribu et les services **JayXpose**, **JayKoa** et **JayRDV**, pour les parcours où un professionnel propose des créneaux de réservation et où les agendas sont unifiés.

### Rôle de chaque service

| Service | Rôle | En bref |
|---------|------|--------|
| **JayXpose** | Vitrine / portfolio du professionnel | Alimente **JayRDV** avec les **informations du professionnel** qui propose ses services (présentation, offre, lien public, widget). Les données exposées par JayXpose (profil pro, services proposés, page publique) sont la source pour configurer ce que le client voit côté réservation. |
| **JayKoa** | Calendrier universel, agenda unifié | **Organise les agendas de chacun** : agrège et reflète les engagements temporels (événements internes, reflets JayFestival, reflets JayRDV). JayKoa ne crée pas d’événement externe ; il reflète les RDV confirmés issus de JayRDV pour afficher un calendrier unifié par utilisateur. |
| **JayRDV** | Service rendez-vous et réservation | **Organise les écrans, les vues, le flux de réservation** : CRUD des créneaux (slots), gestion des rendez-vous (création, modification), **annulations** (côté client ou côté pro), **rappels** (confirmations, rappels avant RDV). JayRDV consomme les infos pro fournies par JayXpose et peut exposer des reflets vers JayKoa pour l’agenda. |

### Flux de données (schéma)

```
JayXpose (infos professionnel, offre, page publique)
    │
    ▼
JayRDV  ← reçoit : qui propose quoi, lien/widget, services
    │
    ├── Écrans / vues / flux réservation
    ├── CRUD créneaux (slots), RDV
    ├── Annulations (client ou pro)
    ├── Rappels (confirmation, rappel avant RDV)
    │
    ▼
JayKoa  ← reçoit : reflets des RDV confirmés (lecture seule)
    │
    └── Organise les agendas de chacun (vue calendrier unifiée)
```

- **JayXpose → JayRDV** : les informations du professionnel (profil, services proposés, paramètres de la page de réservation) alimentent la configuration de JayRDV (quel pro, quels services, quel lien/widget). Sync ou lecture réfléchie selon l’implémentation (adaptateur ou API).
- **JayRDV → JayKoa** : les rendez-vous **confirmés** sont synchronisés en reflets dans JayKoa (via `JayRDVAdapter::sync_appointments_from_store` ou équivalent) pour que l’agenda unifié (JayKoa) affiche les RDV sans modifier les données source.

### Points d’intégration Jay1Tribu

- **Jay1Tribu** (amis, tribus, salons, messagerie) peut croiser les usages avec la réservation : par exemple afficher « Mes amis » ou « Ma tribu » dans un contexte où un pro (membre de la tribu) propose des créneaux via JayRDV ; ou lier notifications / rappels à la messagerie si besoin.
- La **sync JayXpose / JayKoa / JayRDV** est décrite ici pour cohérence avec la doc produit et le [Suivi Audit et Todo](../implementation/Miyukini%20COG%20-%20Suivi%20Audit%20et%20Todo.md) ; les tâches d’implémentation des adaptateurs ou de l’UI restent dans les crates respectifs (jayxpose, jaykoa, jayrdv).

---

## Todo par priorité

### Priorité haute

| # | Tâche | Livrable | Référence |
|---|------|----------|-----------|
| H1 | **Implémenter l’envoi de messages via MWS** | Module `transport/` ou intégration MiyuWebwayParticipant ; remplacer le TODO dans `send_message()` par l’appel effectif au transport | ✅ Fait (MwsTransportSender, set_mws_transport_sender, Central enregistre sender à la connexion MWS) |
| H2 | **Implémenter la livraison différée (tribu)** | Mécanisme de sync à la reconnexion : file d’attente locale, reprise à la connexion si émetteur connecté | ✅ Fait (pending_deliveries, process_pending_deliveries, enqueue dans send_message/send_file, appel depuis vue Jay1Tribu) |
| H3 | **Ajouter les balises MSCM et générer l’index MIP** | Annotations `@id`, `@do`, `@role`, `@layer` sur lib.rs, data/, domain/ ; entrée dans `mscm_index/` | [Skill miyukini-mscm-mip](../../.cursor/skills/miyukini-mscm-mip/) ; JayKoa en référence |

### Priorité moyenne

| # | Tâche | Livrable | Référence |
|---|------|----------|-----------|
| M1 | **Intégrer Miou avec get_online_friends / get_friends_list** | Connexion du contexte applicatif Miou à Jay1Tribu ; dégradation gracieuse si indisponible | ✅ Fait |
| M2 | **Implémenter l’envoi de fichiers et d’images** | Flux chiffré, restriction amis (déjà vérifié via `check_can_transfer_file`), archivage local | ✅ Fait (send_file domain, message_attachment_create, dispatch/enqueue) |
| M3 | **Implémenter kindmother-only** | `kindmother_client_db.rs`, feature `kindmother-only` fonctionnelle ; option db-encryption | ✅ Fait (feature kindmother-only, kindmother_client_db.rs, API synchrone block_on ; db-encryption en option legacy-sqlite) |
| M4 | **Invitations tribu** | Création / acceptation / refus d’invitations ; transport via MWS | ✅ Fait (invite_to_tribe, accept/refuse/list ; transport MWS à brancher) |
| M5 | **Clarifier C-4 (Persistance via KindMother)** | Documenter que legacy-sqlite = accès direct SQLite (pattern accepté) OU migrer vers WriteIntent si exigé | [Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) C-4 |

### Priorité basse

| # | Tâche | Livrable | Référence |
|---|------|----------|-----------|
| B1 | **Exporter find_direct_salon_between et get_or_create_direct_salon** | Ajouter à `lib.rs` dans `pub use domain::` | ✅ Fait |
| B2 | **Supprimer #![allow(missing_docs)] et documenter** | Documentation des items publics (modules, fonctions, types) | En cours (docs sur types principaux ; warnings restants sur champs/méthodes DB) |
| B3 | **Module auth/ (optionnel)** | Permissions, RLS si nécessaire pour tribus/salons | [Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) §2 |
| B4 | **Module services/ (optionnel)** | Adaptateurs inter-services si lecture réfléchie requise | [Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) §2 |

---

## Matrice de conformité (objectif)

| Contrainte | Actuel | Cible |
|------------|:------:|:-----:|
| C-1 Pas d’archives centrales | ✅ | ✅ |
| C-2 Transit crypté | ❌ | ✅ |
| C-3 Hébergement utilisateur | ✅ | ✅ |
| C-4 Persistance via KindMother | ⚠️ | ✅ |
| C-5 Type 3 | ✅ | ✅ |
| C-6 Livraison différée | ❌ | ✅ |
| C-7 Rôles gouvernés | ⚠️ | ✅ |
| C-8 Présence via MWS | ✅ | ✅ |

---

## Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Fondateur](./Jay1Tribu%20-%20Document%20Fondateur.md) | Vision, principes |
| [Jay1Tribu - Spécification Fonctionnelle](./Jay1Tribu%20-%20Specification%20Fonctionnelle.md) | Cas d’usage, règles métier |
| [Jay1Tribu - Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) | Structure crate, phases |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | C-1 à C-8, invariants |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat Miou |
| [Miyukini COG - Suivi Audit et Todo](../implementation/Miyukini%20COG%20-%20Suivi%20Audit%20et%20Todo.md) | Suivi global projet |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service rendez-vous, sync avec JayKoa |
| [JayRDV - Professionnels Opérateurs et Toolkits](../JayRDV/publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md) | JayXpose / JayRDV Pro, flux réservation |

---

**Document** : Jay1Tribu — Todo Audit  
**Version** : 1.0  
**Date** : 2026-02-16  
**Statut** : Plan d’action normatif
