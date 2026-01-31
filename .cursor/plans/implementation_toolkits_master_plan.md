# Plan directeur — Implémentation des Toolkits Miyukini

**Version :** 1.0  
**Date :** 2026-01-30  
**Statut :** Plan directeur — exécution par phases et distribution agents  
**Références :** [docs/tools - Verification Pret Implementation Bornes](../docs/tools/docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md), [MiyuSQL crate](../../crates/miyusql), [MiyuCalc Documentation Fondatrice](../docs/tools/MiyuCalc/MiyuCalc%20-%20Documentation%20Fondatrice.md)

---

## 1. Contexte et objectif

**Objectif :** Planifier, organiser, structurer et effectuer l’implémentation de **tous les Toolkits** documentés sous `docs/tools/`, en s’appuyant sur le pattern établi par **MiyuSQL** (seul toolkit actuellement implémenté en crate Rust), et en **distribuant les tâches** à plusieurs agents ou développeurs.

**Périmètre :** 49 Toolkits documentés ; 1 implémenté (MiyuSQL). Les 48 restants doivent être implémentés comme crates Rust dans `crates/<nom-crate>/`, avec exposition à MiyukiniAdmin via une **admin_cell** et conformité aux contrats (BOUND-*, Tool Governance Compliance, Reference Outils).

---

## 2. Architecture d’implémentation

### 2.1 Principe : une crate par Toolkit

| Élément | Règle |
|--------|--------|
| **Nom crate** | `miyucalc`, `miyutext`, `miyauth`, … (snake_case, préfixe `miyu` si cohérent avec le ToolkitId) |
| **Emplacement** | `crates/<nom-crate>/` |
| **Workspace** | Chaque crate est membre du workspace racine `Cargo.toml` |
| **Dépendances** | `miyukini-kernel` obligatoire ; pas de dépendance vers un Opérateur ni vers BondingBrother (le toolkit est invoqué par le flux gouverné, il ne décide pas). |

### 2.2 Pattern type (aligné MiyuSQL)

Structure minimale d’une crate Toolkit :

```text
crates/<toolkit>/
├── Cargo.toml
└── src/
    ├── lib.rs           # Point d'entrée ; réexport des modules et TOOLKIT_ID
    ├── admin_cell.rs     # TOOLKIT_ID, AdminCell (identification, test_manifest, integrity) pour MiyukiniAdmin
    ├── context.rs       # GovernedContext (mandate_id, security_level) — pas d'identité Opérateur
    ├── errors.rs        # Types d'erreur contractuels (NoMandate, technique, sans fuite métier)
    └── <module>.rs      # Un module par famille de Tools (ex. expression, number, unit, round pour MiyuCalc)
```

**Invariants :**

- **admin_cell** : exposée uniquement à MiyukiniAdmin ; contient `TOOLKIT_ID`, identification, manifeste de test (si contrat Cycle Tests), intégrité (contrats référencés).
- **context** : lecture seule ; `GovernedContext { mandate_id, security_level }` ; pas de champ « Opérateur » (BOUND-5).
- **errors** : pas de message exposant des données métier sensibles (BOUND-*).
- **Tools** : chaque ToolId déclaré dans la Documentation Fondatrice / Reference Outils est implémenté comme fonction ou struct + trait ; vérification `ctx.has_mandate()` en entrée ; pas de décision ALLOW/DENY (BOUND-1), pas de choix métier (BOUND-2).

### 2.3 Intégration avec les Cores

- Le toolkit **n’appelle pas** StrongFather ni BondingBrother : il est **invoqué** après décision ALLOW.
- Contexte fourni par le flux gouverné (ex. BondingBrother) : `GovernedContext` construit en amont.
- Si le toolkit produit des écritures métier : **WriteIntent** vers KindMother (pas d’accès direct à la base) — BOUND-3.
- **Kernel** : usage autorisé pour Id, Logger, Clock, Config, Lifecycle (technique uniquement).

---

## 3. Phases de développement

### Phase 0 — Fondations (réalisée / en cours)

- [x] Documenter les 49 Toolkits (Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract).
- [x] Ajouter Reference Implementation Guidelines pour 21 kits (dont 11 récents).
- [x] Établir le plan directeur et le pattern (ce document).
- [x] Créer le **squelette modèle MiyuCalc** (crate `miyucalc`) pour servir de référence aux agents.

### Phase 1 — Squelettes (crates vides ou stubs) ✅ RÉALISÉE

**Objectif :** Avoir une crate par Toolkit avec :

- `Cargo.toml` (name, version, edition, description, dependency `miyukini-kernel`),
- `lib.rs` (modules, réexport, `TOOLKIT_ID`),
- `admin_cell.rs`, `context.rs`, `errors.rs`,
- Un fichier par famille de Tools avec des **stubs** (signatures conformes aux ToolIds, corps retournant une erreur ou une valeur neutre).

**Livrable :** 48 crates (hors MiyuSQL déjà fait) compilent ; `cargo build --workspace` vert.

**Réalisation (2026-01-30) :** 49 crates toolkit (miyusql + miyucalc + 47 squelettes) dans le workspace ; `cargo build --workspace` vert. Générateur `tools/toolkit-skeleton` disponible pour futurs kits.

**Distribution suggérée :** voir section 5 (Lots agents).

### Phase 2 — Implémentation progressive par domaine

**Objectif :** Remplacer les stubs par une logique réelle, en respectant les contrats (Reference Outils, Tool Governance Compliance, Reference Implementation Guidelines ou Runtime Boundary).

**Réalisation (2026-01-31) — Lot 1 :**

- **MiyuClock** : implémentation réelle — `time.now` (Kernel Clock), `time.delta` (durée entre instants) ; modules `time.rs`, suppression de `tools.rs` ; `cargo build -p miyuclock` vert.
- **MiyuNotify** : structure par ToolId — modules `email.rs` (tool.notify.email.send), `push.rs` (tool.notify.push.send), `inbox.rs` (tool.notify.inbox.write + InboxWriteResult) ; stubs Unimplemented ; signatures conformes Reference Outils.
- **MiyuAuth** : structure par ToolId — modules `types.rs` (IdentityContext, IdentityRole, IdentityArtefacts, Attestation, VerificationResult), `resolve.rs`, `attest.rs`, `verify.rs`, `role.rs` ; `identity_role` implémenté (lecture du contexte) ; autres stubs Unimplemented.

**Réalisation (2026-01-31) — Lot 2 (MIP/MSCM) :**

- **MiyuSearch** : structure par ToolId — modules `index.rs` (tool.search.index.update), `query.rs` (tool.search.query.execute + QueryResult), `suggest.rs` (tool.search.suggest) ; balisage MSCM complet (@id, @role, @layer, @do, @human) sur chaque outil ; domaine `search`, layer tool.
- **MiyuWeb** : structure par ToolId — modules `html.rs`, `layout.rs`, `theme.rs`, `script.rs` (execute + compile), `asset.rs`, `form.rs`, `event.rs`, `input.rs` ; 9 tools conformes Reference Outils ; balisage MSCM ; domaine `web`.
- **MiyuPM** : structure par ToolId — modules `message.rs` (send, list, get), `folder.rs` (list, create, update), `draft.rs` (create, update, list), `conversation.rs` (list, get), `export.rs` ; 12 tools ; balisage MSCM ; domaine communication/pm.
- **MiyuForum** : structure par ToolId — modules `category.rs`, `board.rs`, `topic.rs` (dont export_pdf, export_text), `post.rs`, `readtrack.rs` ; 20 tools ; balisage MSCM ; domaine community/forum.

Tous les blocs outils sont balisés pour alimenter **blocks.json**, **domains.json**, **layers.json** (Protocole MIP v1).

**Réalisation (2026-01-31) — Lot 3 (commerce / booking, MIP/MSCM) :**

- **MiyuBilling** : structure par ToolId — modules `subscription.rs` (create, update, cancel, status), `invoice.rs` (generate, list), `payment.rs` (record), `tenant.rs` (resolve) ; 8 tools ; balisage MSCM ; domaine `billing`.
- **MiyuStore** : structure par ToolId — modules `product.rs`, `cart.rs`, `checkout.rs`, `payment.rs`, `shipping.rs`, `order.rs` ; 19 tools ; balisage MSCM ; domaine `commerce`.
- **MiyuShipping** : structure par ToolId — modules `rate.rs` (rate, rates_compare), `zones.rs`, `label.rs` (create, print), `tracking.rs`, `shipment.rs` (create, list) ; 8 tools ; balisage MSCM ; domaine commerce/shipping.
- **MiyuBooking** : structure par ToolId — modules `slots.rs` (list, resolve), `booking.rs` (create, update, cancel), `resource.rs` (resolve, availability), `price.rs`, `participants.rs` ; 9 tools ; balisage MSCM ; domaine `booking`.

**Réalisation (2026-01-31) — Lot 4 (compta / finance, MIP/MSCM) :**

- **MiyuInvoice** : structure par ToolId — modules `quote.rs` (create, update, to_invoice), `invoice.rs` (create, send), `electronic.rs`, `reminder.rs`, `payment_link.rs`, `customer.rs` (resolve, list) ; 10 tools ; balisage MSCM ; domaine `invoice`.
- **MiyuComptaLedger** : structure par ToolId — modules `bank.rs` (sync), `transaction.rs` (categorize, vat_resolve), `reconciliation.rs` (suggest, record), `company.rs` (structure_resolve, structure_register, siret_resolve) ; 8 tools ; balisage MSCM ; domaine `compta`.
- **MiyuExpense** : structure par ToolId — modules `receipt.rs` (capture, extract), `claim.rs` (create, update, list, validate, export), `mileage.rs` (calculate, export) ; 9 tools ; balisage MSCM ; domaine `expense`.
- **MiyuTreasury** : structure par ToolId — modules `dashboard.rs` (aggregate), `forecast.rs` (compute), `alert.rs` (check) ; 3 tools ; balisage MSCM ; domaine `treasury`.

**Réalisation (2026-01-31) — Lot 5 (contenu / webway, MIP/MSCM) — clôture des 21 kits « prêts » :**

- **MiyuCMS** : structure par ToolId — modules `content.rs` (create, update, publish, schedule), `revision.rs` (list, restore, compare), `comment.rs` (create, moderate, list), `media.rs` (upload, serve, transform) ; 14 tools ; balisage MSCM ; domaine `content`.
- **MiyuMedia** : structure par ToolId — module `media.rs` (upload, serve, transform) ; 3 tools ; balisage MSCM ; domaine content/media.
- **MiyuWidgets** : structure par ToolId — modules `layout.rs` (apply), `widget.rs` (text_render, image_render, button_render, grid_render, container_render), `template.rs` (resolve) ; 7 tools ; balisage MSCM ; domaine web/widgets.
- **MiyuWebwayParticipant** : structure par ToolId MWS — modules `declaration.rs` (build, sign, validate, verify), `transport.rs` (send), `discovery.rs` (request_build, request_send), `cog_list.rs` (get, update, merge), `port.rs` (check), `address.rs` (tracker_default) ; 12 tools ; balisage MSCM ; domaine `webway`.
- **MiyuWebwayTracker** : structure par ToolId MWS — modules `declaration.rs` (validate, verify), `transport.rs` (receive, send), `discovery.rs` (response_build, response_send), `cog_list.rs` (get, update, merge, filter), `port.rs` (check), `address.rs` (tracker_default) ; 12 tools ; balisage MSCM ; domaine `webway`.

**Les 21 kits « prêts sans déviation » sont désormais structurés par ToolId avec balisage MIP/MSCM.**

**Réalisation (2026-01-31) — Lot 6 (kits « avec précautions » prioritaires, MIP/MSCM) :**

- **MiyuCalc** : déjà structuré — modules `expression.rs`, `number.rs`, `round.rs`, `unit.rs` ; 4 tools avec balisage MSCM ; domaine `calc`. Aucune modification (déjà conforme).
- **MiyuText** : structure par ToolId — modules `markdown.rs` (tool.text.markdown.render), `replace.rs` (tool.text.replace), `template.rs` (tool.text.template.apply), `sanitize.rs` (tool.text.sanitize) ; 4 tools ; balisage MSCM ; domaine `text`.
- **MiyuValidate** : structure par ToolId — modules `schema.rs` (tool.validate.schema.check + SchemaCheckResult), `sanitize.rs` (tool.validate.sanitize) ; 2 tools ; balisage MSCM ; domaine `validate`.
- **MiyuLocale** : structure par ToolId — modules `date.rs` (tool.locale.date.format), `number.rs` (tool.locale.number.format), `translate.rs` (tool.locale.translate) ; 3 tools ; balisage MSCM ; domaine `locale`.
- **MiyuExport** : structure par ToolId — modules `csv.rs` (tool.export.csv.generate), `xlsx.rs` (tool.export.xlsx.generate), `pdf.rs` (tool.export.pdf.render) ; 3 tools ; balisage MSCM ; domaine `export`.
- **MiyuJobs** : structure par ToolId — modules `schedule.rs` (tool.jobs.schedule.at, tool.jobs.schedule.cron), `queue.rs` (tool.jobs.queue.enqueue, tool.jobs.queue.process + EnqueueOptions, ProcessResult) ; 4 tools ; balisage MSCM ; domaine `jobs`.

**Les 6 kits « avec précautions » prioritaires sont désormais structurés par ToolId avec balisage MIP/MSCM ; `tools.rs` supprimé partout.**

**Réalisation (2026-01-31) — Lot 7 (autres kits « avec précautions », MIP/MSCM) :**

- **MiyuAntiSpam** : structure par ToolId — modules `captcha.rs` (generate, verify), `flood.rs` (check + FloodCheckResult), `rate_limit.rs` (check + RateLimitCheckResult) ; 4 tools ; balisage MSCM ; domaine `security`.
- **MiyuBookmarks** : structure par ToolId — module `bookmark.rs` (add, remove, list + BookmarkFilters, BookmarkItem) ; 3 tools ; balisage MSCM ; domaine `content`.
- **MiyuComptaReports** : structure par ToolId — modules `report.rs` (livre_recettes_generate, balance_generate, liasse_generate, cashflow_generate), `export.rs` (ledger) ; 5 tools ; balisage MSCM ; domaine `compta`.
- **MiyuContacts** : structure par ToolId — modules `friend.rs` (add, remove, list + ContactItem), `foe.rs` (add, remove, list), `contacts.rs` (list) ; 7 tools ; balisage MSCM ; domaine `communication`.
- **MiyuDeclarations** : structure par ToolId — modules `urssaf.rs` (prepare, submit + UrssafPrepareResult), `tva.rs` (prepare, submit + TvaPrepareResult), `deadline.rs` (list + DeadlineItem), `declaration.rs` (list + DeclarationFilters, DeclarationItem), `estimate.rs` (cotisations + CotisationsEstimate) ; 7 tools ; balisage MSCM ; domaine `compta`.
- **MiyuDiscovery** : structure par ToolId — modules `hashtag.rs` (list, get, trending + HashtagItem, HashtagDetail), `trending.rs` (list + TrendingItem), `discover.rs` (list + DiscoverFilters, DiscoverItem), `search.rs` (search + SearchResult) ; 6 tools ; balisage MSCM ; domaine `social`.

**Les 6 kits du Lot 7 sont structurés par ToolId avec balisage MIP/MSCM ; `tools.rs` supprimé.**

**Réalisation (2026-01-31) — Lot 8 (feeds, HR, polls, profile, story, modération forum, MIP/MSCM) :**

- **MiyuFeeds** : structure par ToolId — module `feed.rs` (atom_board, atom_forum, atom_topic) ; 3 tools ; balisage MSCM ; domaine `content`.
- **MiyuHR** : structure par ToolId — modules `time_clock.rs` (clock_in, clock_out), `schedule.rs` (get + ScheduleResult, ShiftItem) ; 3 tools ; balisage MSCM ; domaine `hr`.
- **MiyuPolls** : structure par ToolId — module `poll.rs` (create, vote, list, result + PollItem, PollResult) ; 4 tools ; balisage MSCM ; domaine `content`.
- **MiyuStory** : structure par ToolId — module `story.rs` (create, list, get, reaction_add + StoryFilters, StoryItem, StoryDetail) ; 4 tools ; balisage MSCM ; domaine `social`.
- **MiyuProfile** : structure par ToolId — modules `profile.rs` (get, update + ProfileData), `field.rs` (list, get, set), `avatar.rs` (get, set, resolve), `signature.rs` (get, set), `rank.rs` (list, resolve + RankItem), `preferences.rs` (get, set) ; 14 tools ; balisage MSCM ; domaine `identity`.
- **MiyuModerationForum** : structure par ToolId — modules `queue.rs` (list, get + QueueItem, QueueItemDetail), `report.rs` (create, list + ReportItem), `topic.rs` (lock, move, merge, split, delete, copy), `post.rs` (edit, lock, delete), `warning.rs` (create, list + WarningItem), `ban.rs` (create, list + BanItem), `usernote.rs` (create, list + UsernoteItem) ; 19 tools ; balisage MSCM ; domaine `moderation`.

**Les 6 kits du Lot 8 sont structurés par ToolId avec balisage MIP/MSCM ; `tools.rs` supprimé.**

**Réalisation (2026-01-31) — Lot 9 (Social + PoS, MIP/MSCM) — clôture des 10 derniers kits :**

- **MiyuSocialProfile** : structure par ToolId — modules `profile.rs` (get, update), `follow.rs` (add, remove, followers_list, following_list) ; 6 tools ; balisage MSCM ; domaine `social`.
- **MiyuSocialModeration** : structure par ToolId — modules `report.rs` (create, list + ReportItem), `block.rs` (add, remove, list), `post.rs` (delete) ; 6 tools ; balisage MSCM ; domaine `social`.
- **MiyuSocialMessaging** : structure par ToolId — modules `dm.rs` (send, list, get, reaction_add, reaction_remove, readmark_set + DmItem), `conversation.rs` (list, get + ConversationItem, ConversationDetail) ; 8 tools ; balisage MSCM ; domaine `social`.
- **MiyuSocialFeed** : structure par ToolId — modules `post.rs` (create, update, delete, get + PostItem), `feed.rs` (list + FeedFilters), `reaction.rs` (add, remove, list + ReactionItem), `share.rs` (create, list + ShareItem), `comment.rs` (create, list, delete + CommentItem) ; 12 tools ; balisage MSCM ; domaine `social`.
- **MiyuPosAnalytics** : structure par ToolId — modules `sales.rs` (trend, by_item, by_employee), `cash.rs` (discrepancy + CashDiscrepancyResult), `tax.rs` (report), `shift.rs` (close), `export.rs` (spreadsheet) ; 7 tools ; balisage MSCM ; domaine `pos`.
- **MiyuPosKitchen** : structure par ToolId — modules `kitchen.rs` (print, order_push, order_update_status), `service_type.rs` (set), `ticket.rs` (preset_assign) ; 5 tools ; balisage MSCM ; domaine `pos`.
- **MiyuPosPayment** : structure par ToolId — modules `cash.rs` (record), `check.rs` (record), `split.rs` (split), `terminal.rs` (authorize, capture + TerminalAuthResult) ; 5 tools ; balisage MSCM ; domaine `pos`.
- **MiyuPosLoyalty** : structure par ToolId — modules `customer.rs` (get, list, create, update, address_get, note_add, note_list + CustomerItem, AddressItem, NoteItem), `loyalty.rs` (points_grant, points_redeem, balance_get, card_resolve + CardResolveResult) ; 11 tools ; balisage MSCM ; domaine `pos`.
- **MiyuPosInventory** : structure par ToolId — modules `stock.rs` (get, adjust + StockResult), `import.rs` (items + ImportResult), `alert.rs` (low_evaluate + AlertItem), `purchase_order.rs` (create, update, track + PurchaseOrderStatus), `transfer.rs` (create, execute, list + TransferItem), `count.rs` (start, record, reconcile + ReconcileResult), `production.rs` (record), `label.rs` (print), `history.rs` (list + MovementItem), `valuation.rs` (report) ; 16 tools ; balisage MSCM ; domaine `pos`.
- **MiyuPosSales** : structure par ToolId — modules `sale.rs` (create, add_item, remove_item), `ticket.rs` (open, save, close, list + TicketItem), `discount.rs` (apply), `refund.rs` (record), `receipt.rs` (render, print, send, list + ReceiptItem), `item.rs` (variant_resolve, modifier_apply + VariantResult), `cash_register.rs` (open, close), `cash_movement.rs` (record), `barcode.rs` (parse + BarcodeParseResult), `store.rs` (store_resolve + StoreResult), `display.rs` (push), `order.rs` (service_type_set) ; 21 tools ; balisage MSCM ; domaine `pos`.

**Les 10 kits du Lot 9 sont structurés par ToolId avec balisage MIP/MSCM ; `tools.rs` supprimé. Tous les 49 Toolkits documentés sont désormais en structure par ToolId avec balisage MIP/MSCM.**

**Ordre de priorité suggéré (suite) :**

1. **Phase 3** : Intégration MiyukiniAdmin (enregistrement admin_cells), index MIP à jour.
2. **Autres kits** : par domaine selon roadmap produit.

### Phase 3 — Intégration MiyukiniAdmin et MIP ✅ RÉALISÉE (2026-01-31)

- **MIP** : `mip-generator` découvre dynamiquement tous les crates ; index MIP à jour (1302 blocs, 486 fichiers, 65 domaines).
- **Registre Toolkits** : `toolkit-registry-export` exporte les 49 admin_cells vers `mscm_index/toolkit_registry.json`. MiyukiniAdmin charge ce fichier au démarrage ; découverte et interrogation des 49 Toolkits opérationnelles.
- **Workflow** : `cargo run -p mip-generator` puis `cargo run -p toolkit-registry-export` depuis la racine ; lancer MiyukiniAdmin depuis la même racine.

### Phase 4 — Logique réelle prioritaire (en cours)

- **Lot 4a — MiyuCalc** : expression (évaluateur arithmétique sûr + - * / ( ) ), number (format décimal/percent/devise), round (HalfEven/Floor/Ceiling/Truncate), unit (conversions longueur/masse/volume). Balisage MSCM conservé.
- **Lot 4b — MiyuText** : markdown (rendu HTML via pulldown-cmark), replace (littéral), template (placeholders {{ key }}), sanitize (strip tags + escape HTML). Balisage MSCM conservé.
- **Lot 4c — MiyuValidate** : schema (validation JSON-like : required, properties/types), sanitize (trim, escape HTML, validation numérique, nettoyage liste). Balisage MSCM conservé.
- **Lot 4d — MiyuLocale** : date (format court/long fr/en via chrono, DateTime::from_timestamp), number (format décimal/devise fr/en), translate (résolution clé + fallback). Balisage MSCM conservé.
- **Lot 4e — MiyuExport** : csv (génération avec échappement champs), xlsx (rust_xlsxwriter, save_to_buffer, set_name par feuille), pdf (genpdf 0.2, set_page_decorator, polices ./fonts/LiberationSans). Erreur MiyuExportError::Io pour I/O. Balisage MSCM conservé.
- **Lot 4f — MiyuJobs** : schedule_at (id opaque at:job_id:run_at_ms), schedule_cron (validation 5 champs, id cron:job_id:expr), enqueue (task_id via UuidIdGenerator), process (ProcessResult vide sans backend). MiyuJobsError::InvalidInput. Balisage MSCM conservé.
- **Lot 4g — MiyuAntiSpam** : captcha generate (cap:uuid), verify (false sans état), flood check (within_limit true, current_count 0), rate_limit check (within_limit true, remaining 100). Balisage MSCM conservé.
- **Lot 4h — MiyuBookmarks** : add (bm:uuid), remove (Ok(())), list (vec vide). Pas de persistance dans le toolkit ; WriteIntent côté flux. Balisage MSCM conservé.
- **Lot 4i — MiyuFeeds** : atom_board, atom_forum, atom_topic — flux ATOM 1.0 minimal (title, id, updated RFC3339, échappement XML). Date UTC depuis SystemTime + unix_days_to_ymd. Balisage MSCM conservé.
- **Lot 4j — MiyuHR** : clock_in, clock_out (id opaque clock:in/out:employee_id:uuid), schedule get (ScheduleResult { shifts: vec![] }). MiyuhrError::InvalidInput. Balisage MSCM conservé.
- **Lot 4k — MiyuDiscovery** : hashtag list/get/trending (vec vide ou HashtagDetail avec tag), trending list (vec vide), discover list (vec vide), search (SearchResult vide). Pas de backend ; lecture côté flux. Balisage MSCM conservé.
- **Suite** : après chaque lot : `cargo build -p <crate>` et `cargo run -p mip-generator` pour index MIP à jour.
 l’
---

## 4. Priorisation des Toolkits

### 4.1 Kits « prêts sans déviation » (21)

Disposent des 3 livrables + Reference Implementation Guidelines ou Runtime Boundary Contract.

| Kit | ToolkitId | Crate proposée |
|-----|-----------|----------------|
| MiyuAuth | toolkit.identity.miyauth | miyauth |
| MiyuWeb | toolkit.web.miyuweb | miyuweb |
| MiyuSQL | toolkit.data.miyusql | miyusql (existant) |
| MiyuClock | toolkit.kernel.miyuclock | miyuclock |
| MiyuForum | toolkit.community.forum | miyuforum |
| MiyuPM | toolkit.communication.pm | miyupm |
| MiyuNotify | toolkit.notify.miyunotify | miyunotify |
| MiyuSearch | toolkit.search.miyusearch | miyusearch |
| MiyuWebwayParticipant | toolkit.webway.participant | miyuwebway-participant |
| MiyuWebwayTracker | toolkit.webway.tracker | miyuwebway-tracker |
| MiyuBilling | toolkit.billing.saas | miyubilling |
| MiyuBooking | toolkit.booking.reservations | miyubooking |
| MiyuCMS | toolkit.content.cms | miyucms |
| MiyuMedia | toolkit.content.media | miyumedia |
| MiyuShipping | toolkit.commerce.shipping | miyushipping |
| MiyuStore | toolkit.commerce.store | miyustore |
| MiyuWidgets | toolkit.web.widgets | miyuwidgets |
| MiyuInvoice | toolkit.invoice.standalone | miyuinvoice |
| MiyuComptaLedger | toolkit.compta.ledger | miyucptaledger |
| MiyuExpense | toolkit.expense.claims | miyuexpense |
| MiyuTreasury | toolkit.treasury.forecast | miyutreasury |

### 4.2 Kits « avec précautions » (28)

Ont les 3 livrables mais pas de guide dédié ni Runtime Boundary. Utiliser le [Template Reference Implementation Guidelines](../docs/tools/docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) + Documentation Fondatrice + Contrat Governance.

Liste : MiyuAntiSpam, MiyuBookmarks, **MiyuCalc**, MiyuComptaReports, MiyuContacts, MiyuDeclarations, MiyuDiscovery, MiyuExport, MiyuFeeds, MiyuHR, MiyuJobs, MiyuLocale, MiyuModerationForum, MiyuPolls, MiyuPosAnalytics, MiyuPosInventory, MiyuPosKitchen, MiyuPosLoyalty, MiyuPosPayment, MiyuPosSales, MiyuProfile, MiyuSocialFeed, MiyuSocialMessaging, MiyuSocialModeration, MiyuSocialProfile, MiyuStory, MiyuText, MiyuValidate.

**MiyuCalc** est le **kit modèle** : squelette fourni dans ce plan ; les autres agents peuvent s’en inspirer.

---

## 5. Distribution des tâches (agents / développeurs)

### 5.1 Lot A — Calcul, texte, validation, locale (squelettes)

| Agent / Lot | Crates | Tâche |
|-------------|--------|--------|
| **A1** | miyucalc | Déjà fourni en squelette dans ce plan. Vérifier compilation et compléter stubs si besoin. |
| **A2** | miyutext, miyuvalidate, miyulocale | Créer squelette (Cargo.toml, lib, admin_cell, context, errors, modules par ToolId). |
| **A3** | miyuexport, miyujobs | Idem squelette. |

**Référence pour A2/A3 :** copier la structure de `crates/miyucalc` et adapter les noms, TOOLKIT_ID, ToolIds (voir chaque Documentation Fondatrice + Reference Outils).

### 5.2 Lot B — Commerce, facturation, trésorerie (squelettes)

| Agent / Lot | Crates | Tâche |
|-------------|--------|--------|
| **B1** | miyubilling, miyustore, miyushipping, miyubooking | Squelettes. |
| **B2** | miyuinvoice, miyucptaledger, miyuexpense, miyutreasury | Squelettes. |

### 5.3 Lot C — Contenu, web, widgets (squelettes)

| Agent / Lot | Crates | Tâche |
|-------------|--------|--------|
| **C1** | miyucms, miyumedia, miyuwidgets, miyuweb | Squelettes. |
| **C2** | miyuclock, miyauth | Squelettes. |

### 5.4 Lot D — Communauté, communication, recherche (squelettes)

| Agent / Lot | Crates | Tâche |
|-------------|--------|--------|
| **D1** | miyuforum, miyupm, miyunotify, miyusearch | Squelettes. |
| **D2** | miyuwebway-participant, miyuwebway-tracker | Squelettes. |

### 5.5 Lot E — PoS, social, modération, divers (squelettes)

| Agent / Lot | Crates | Tâche |
|-------------|--------|--------|
| **E1** | miyupossales, miyuposinventory, miyuposanalytics, miyuposloyalty, miyuposkitchen, miyupospayment | Squelettes. |
| **E2** | miyuhr, miyudeclarations, miyucomptareports | Squelettes. |
| **E3** | miyufeeds, miyubookmarks, miyupolls, miyudiscovery, miyucontacts, miyuprofile | Squelettes. |
| **E4** | miyusocialfeed, miyusocialmessaging, miyusocialprofile, miyusocialmoderation, miyustory | Squelettes. |
| **E5** | miyuantispam, miyumoderationforum | Squelettes. |

### 5.6 Instructions communes pour chaque agent

1. **Lire** la Documentation Fondatrice et la Reference Outils du kit concerné pour extraire le **ToolkitId** et la liste des **ToolIds**.
2. **Créer** la crate sous `crates/<nom-crate>/` en suivant le pattern MiyuCalc/MiyuSQL (lib, admin_cell, context, errors, un module par famille de Tools).
3. **Déclarer** chaque ToolId comme fonction publique ou méthode (signature : `ctx: &GovernedContext`, paramètres d’entrée, `Result<T, E>`). Corps en stub si Phase 1 (ex. `Err(MiyuCalcError::Unimplemented)` ou valeur neutre).
4. **Ajouter** la crate au workspace racine `Cargo.toml` : `members = [ ..., "crates/<nom-crate>" ]`.
5. **Vérifier** : `cargo build -p <nom-crate>` puis `cargo build --workspace`.
6. **Ne pas** ajouter de dépendance vers un Opérateur, BondingBrother, ni de logique métier (choix de contenu, ALLOW/DENY). Respecter BOUND-1 à BOUND-6 (voir Template Reference Implementation Guidelines).

---

## 6. Critères de succès

| Phase | Critère |
|-------|--------|
| **Phase 0** | Plan rédigé ; squelette MiyuCalc créé ; `cargo build -p miyucalc` vert. |
| **Phase 1** | 49 crates (miyusql + 48 autres) présentes dans le workspace ; `cargo build --workspace` vert ; chaque crate expose TOOLKIT_ID et admin_cell. |
| **Phase 2** | Au moins les kits « prêts » prioritaires implémentés (stubs remplacés par logique conforme aux contrats). |
| **Phase 3** | MiyukiniAdmin peut découvrir et interroger les admin_cells ; MIP index à jour pour les blocs implémentés. |

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| Verification Prêt Implémentation | [docs_tools - Verification Pret Implementation Bornes](../docs/tools/docs_tools%20-%20Verification%20Pret%20Implementation%20Bornes.md) |
| Reference Implementation Guidelines Template | [docs_tools - Reference Implementation Guidelines Template](../docs/tools/docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| Index docs/tools | [docs/tools _index](../docs/tools/_index.md) |
| MiyuSQL (crate) | [crates/miyusql](../../crates/miyusql) |
| MiyuCalc Documentation Fondatrice | [MiyuCalc - Documentation Fondatrice](../docs/tools/MiyuCalc/MiyuCalc%20-%20Documentation%20Fondatrice.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Fin du plan directeur.**
