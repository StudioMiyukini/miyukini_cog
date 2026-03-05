# Jay1Tribu â€” Todo Audit

## Contexte

Ce document est le **plan de remÃ©diation** dÃ©rivÃ© de lâ€™audit Jay1Tribu rÃ©alisÃ© le 2026-02-16. Il liste les tÃ¢ches prioritaires pour atteindre une implÃ©mentation conforme Ã  la documentation (Document Fondateur, SpÃ©cification Fonctionnelle, Guide dâ€™implÃ©mentation, Contraintes et Invariants).

**RÃ©fÃ©rence :** Audit Jay1Tribu (complÃ©tion, fonctionnement, qualitÃ© de code, protocole MIP, implantation vs doc).

## PortÃ©e / Scope

- **Applicable Ã  :** Crate `jay1tribu`, intÃ©gration Central/Miou, conformitÃ© MIP.
- **Audience :** DÃ©veloppeurs, architectes, Ã©quipe produit.
- **Statut :** Plan dâ€™action normatif â€” Ã  traiter par prioritÃ©.

---

## SynthÃ¨se de lâ€™audit

| Dimension | Ã‰tat | ComplÃ©tion estimÃ©e |
|-----------|:----:|:------------------:|
| ComplÃ©tion fonctionnelle | Partielle | ~45â€“50 % |
| Bon fonctionnement (lecture locale) | âœ… | OK |
| QualitÃ© du code | âœ… Bonne | OK |
| Protocole MIP (MSCM) | âŒ | 0 % |
| Implantation vs doc | âš ï¸ Ã‰carts | Partiel |

---

## Sync avec JayXpose et JayKoa

Cette section dÃ©crit le **flux de donnÃ©es et les responsabilitÃ©s** entre Jay1Tribu et les services **JayXpose**, **JayKoa** et **JayRDV**, pour les parcours oÃ¹ un professionnel propose des crÃ©neaux de rÃ©servation et oÃ¹ les agendas sont unifiÃ©s.

### RÃ´le de chaque service

| Service | RÃ´le | En bref |
|---------|------|--------|
| **JayXpose** | Vitrine / portfolio du professionnel | Alimente **JayRDV** avec les **informations du professionnel** qui propose ses services (prÃ©sentation, offre, lien public, widget). Les donnÃ©es exposÃ©es par JayXpose (profil pro, services proposÃ©s, page publique) sont la source pour configurer ce que le client voit cÃ´tÃ© rÃ©servation. |
| **JayKoa** | Calendrier universel, agenda unifiÃ© | **Organise les agendas de chacun** : agrÃ¨ge et reflÃ¨te les engagements temporels (Ã©vÃ©nements internes, reflets JayFestival, reflets JayRDV). JayKoa ne crÃ©e pas dâ€™Ã©vÃ©nement externe ; il reflÃ¨te les RDV confirmÃ©s issus de JayRDV pour afficher un calendrier unifiÃ© par utilisateur. |
| **JayRDV** | Service rendez-vous et rÃ©servation | **Organise les Ã©crans, les vues, le flux de rÃ©servation** : CRUD des crÃ©neaux (slots), gestion des rendez-vous (crÃ©ation, modification), **annulations** (cÃ´tÃ© client ou cÃ´tÃ© pro), **rappels** (confirmations, rappels avant RDV). JayRDV consomme les infos pro fournies par JayXpose et peut exposer des reflets vers JayKoa pour lâ€™agenda. |

### Flux de donnÃ©es (schÃ©ma)

```
JayXpose (infos professionnel, offre, page publique)
    â”‚
    â–¼
JayRDV  â† reÃ§oit : qui propose quoi, lien/widget, services
    â”‚
    â”œâ”€â”€ Ã‰crans / vues / flux rÃ©servation
    â”œâ”€â”€ CRUD crÃ©neaux (slots), RDV
    â”œâ”€â”€ Annulations (client ou pro)
    â”œâ”€â”€ Rappels (confirmation, rappel avant RDV)
    â”‚
    â–¼
JayKoa  â† reÃ§oit : reflets des RDV confirmÃ©s (lecture seule)
    â”‚
    â””â”€â”€ Organise les agendas de chacun (vue calendrier unifiÃ©e)
```

- **JayXpose â†’ JayRDV** : les informations du professionnel (profil, services proposÃ©s, paramÃ¨tres de la page de rÃ©servation) alimentent la configuration de JayRDV (quel pro, quels services, quel lien/widget). Sync ou lecture rÃ©flÃ©chie selon lâ€™implÃ©mentation (adaptateur ou API).
- **JayRDV â†’ JayKoa** : les rendez-vous **confirmÃ©s** sont synchronisÃ©s en reflets dans JayKoa (via `JayRDVAdapter::sync_appointments_from_store` ou Ã©quivalent) pour que lâ€™agenda unifiÃ© (JayKoa) affiche les RDV sans modifier les donnÃ©es source.

### Points dâ€™intÃ©gration Jay1Tribu

- **Jay1Tribu** (amis, tribus, salons, messagerie) peut croiser les usages avec la rÃ©servation : par exemple afficher Â« Mes amis Â» ou Â« Ma tribu Â» dans un contexte oÃ¹ un pro (membre de la tribu) propose des crÃ©neaux via JayRDV ; ou lier notifications / rappels Ã  la messagerie si besoin.
- La **sync JayXpose / JayKoa / JayRDV** est dÃ©crite ici pour cohÃ©rence avec la doc produit et le [Suivi Audit et Todo](..//..//_index.md) ; les tÃ¢ches dâ€™implÃ©mentation des adaptateurs ou de lâ€™UI restent dans les crates respectifs (jayxpose, jaykoa, jayrdv).

---

## Todo par prioritÃ©

### PrioritÃ© haute

| # | TÃ¢che | Livrable | RÃ©fÃ©rence |
|---|------|----------|-----------|
| H1 | **ImplÃ©menter lâ€™envoi de messages via MWS** | Module `transport/` ou intÃ©gration MiyuWebwayParticipant ; remplacer le TODO dans `send_message()` par lâ€™appel effectif au transport | âœ… Fait (MwsTransportSender, set_mws_transport_sender, Central enregistre sender Ã  la connexion MWS) |
| H2 | **ImplÃ©menter la livraison diffÃ©rÃ©e (tribu)** | MÃ©canisme de sync Ã  la reconnexion : file dâ€™attente locale, reprise Ã  la connexion si Ã©metteur connectÃ© | âœ… Fait (pending_deliveries, process_pending_deliveries, enqueue dans send_message/send_file, appel depuis vue Jay1Tribu) |
| H3 | **Ajouter les balises MSCM et gÃ©nÃ©rer lâ€™index MIP** | Annotations `@id`, `@do`, `@role`, `@layer` sur lib.rs, data/, domain/ ; entrÃ©e dans `mscm_index/` | [Skill miyukini-mscm-mip](../../_index.md) ; JayKoa en rÃ©fÃ©rence |

### PrioritÃ© moyenne

| # | TÃ¢che | Livrable | RÃ©fÃ©rence |
|---|------|----------|-----------|
| M1 | **IntÃ©grer Miou avec get_online_friends / get_friends_list** | Connexion du contexte applicatif Miou Ã  Jay1Tribu ; dÃ©gradation gracieuse si indisponible | âœ… Fait |
| M2 | **ImplÃ©menter lâ€™envoi de fichiers et dâ€™images** | Flux chiffrÃ©, restriction amis (dÃ©jÃ  vÃ©rifiÃ© via `check_can_transfer_file`), archivage local | âœ… Fait (send_file domain, message_attachment_create, dispatch/enqueue) |
| M3 | **ImplÃ©menter kindmother-only** | `kindmother_client_db.rs`, feature `kindmother-only` fonctionnelle ; option db-encryption | âœ… Fait (feature kindmother-only, kindmother_client_db.rs, API synchrone block_on ; db-encryption en option legacy-sqlite) |
| M4 | **Invitations tribu** | CrÃ©ation / acceptation / refus dâ€™invitations ; transport via MWS | âœ… Fait (invite_to_tribe, accept/refuse/list ; transport MWS Ã  brancher) |
| M5 | **Clarifier C-4 (Persistance via KindMother)** | Documenter que legacy-sqlite = accÃ¨s direct SQLite (pattern acceptÃ©) OU migrer vers WriteIntent si exigÃ© | [Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) C-4 |

### PrioritÃ© basse

| # | TÃ¢che | Livrable | RÃ©fÃ©rence |
|---|------|----------|-----------|
| B1 | **Exporter find_direct_salon_between et get_or_create_direct_salon** | Ajouter Ã  `lib.rs` dans `pub use domain::` | âœ… Fait |
| B2 | **Supprimer #![allow(missing_docs)] et documenter** | Documentation des items publics (modules, fonctions, types) | En cours (docs sur types principaux ; warnings restants sur champs/mÃ©thodes DB) |
| B3 | **Module auth/ (optionnel)** | Permissions, RLS si nÃ©cessaire pour tribus/salons | [Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) Â§2 |
| B4 | **Module services/ (optionnel)** | Adaptateurs inter-services si lecture rÃ©flÃ©chie requise | [Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) Â§2 |

---

## Matrice de conformitÃ© (objectif)

| Contrainte | Actuel | Cible |
|------------|:------:|:-----:|
| C-1 Pas dâ€™archives centrales | âœ… | âœ… |
| C-2 Transit cryptÃ© | âŒ | âœ… |
| C-3 HÃ©bergement utilisateur | âœ… | âœ… |
| C-4 Persistance via KindMother | âš ï¸ | âœ… |
| C-5 Type 3 | âœ… | âœ… |
| C-6 Livraison diffÃ©rÃ©e | âŒ | âœ… |
| C-7 RÃ´les gouvernÃ©s | âš ï¸ | âœ… |
| C-8 PrÃ©sence via MWS | âœ… | âœ… |

---

## RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Jay1Tribu - Document Fondateur](./Jay1Tribu%20-%20Document%20Fondateur.md) | Vision, principes |
| [Jay1Tribu - SpÃ©cification Fonctionnelle](./Jay1Tribu%20-%20Specification%20Fonctionnelle.md) | Cas dâ€™usage, rÃ¨gles mÃ©tier |
| [Jay1Tribu - Guide Implementation](./Jay1Tribu%20-%20Guide%20Implementation.md) | Structure crate, phases |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | C-1 Ã  C-8, invariants |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat Miou |
| [Miyukini COG - Suivi Audit et Todo](..//..//_index.md) | Suivi global projet |
| [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service rendez-vous, sync avec JayKoa |
| [JayRDV - Professionnels OpÃ©rateurs et Toolkits](../JayRDV/publics/Professionnels/Professionnels%20-%20Operateurs%20et%20Toolkits.md) | JayXpose / JayRDV Pro, flux rÃ©servation |

---

**Document** : Jay1Tribu â€” Todo Audit  
**Version** : 1.0  
**Date** : 2026-02-16  
**Statut** : Plan dâ€™action normatif


