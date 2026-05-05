# Services retirés du projet Miyukini COG

Ce fichier documente les services qui ont été retirés du périmètre du projet,
les raisons de leur retrait et les conséquences techniques pour les services
restants.

---

## JayXpose — retiré le 2026-04-29

### Description (historique)

JayXpose était le service dédié à l'identité professionnelle des exposants,
artisans et petites entreprises. Il couvrait :

- Profil entreprise et fiche publique
- Catalogue produits (photos, prix, disponibilités)
- Page builder de vitrine personnalisée
- Coffre-fort documentaire (CGV, certifications)
- Annuaire des exposants
- Vitrine publique servie par Origin sur `/vitrine/{slug}`

### Périmètre supprimé

- `crates/jayxpose/` (crate domaine + manifest + screens)
- `apps/jayxpose/` (binaire Dioxus standalone)
- `apps/central/src/services/jayxpose/` (12 fichiers UI dans Central)
- Routes `/vitrine/*` dans `apps/origin`
- Pages vitrine HTML dans `crates/miyukini-central/src/mws/mod.rs`
  (`jayxpose_layout`, `jayxpose_home_html`, `jayxpose_catalogue_html`,
  `jayxpose_product_html`, `jayxpose_contact_html`)
- Champs config `expose_jayxpose_vitrine` et `jayxpose_vitrine_base_url` dans
  `CentralMwsConfig`
- API forum_auth `set_jayxpose_linked`, `sync_profile_with_jayxpose`,
  champ `has_jayxpose_profile`
- Adaptateur `crates/jaymanga/src/services/jayxpose/`
- Documentation `docs/services/JayXpose/`

### Conséquences pour les services restants

- **JayKonta** : la facturation B2B des exposants n'est plus reliée à un
  service de fiche publique ; les `context_ref` de type `exposant:*` ne
  sont plus produits automatiquement.
- **JayShop** : la synchronisation de catalogue produit qui passait par
  JayXpose est désormais absente ; JayShop reste autonome.
- **JayManga** : l'adaptateur `services/jayxpose/` a été retiré ; la
  publication d'œuvres vers une vitrine externe doit passer par d'autres
  canaux (export direct, MWS).
- **forum_auth** (Origin) : la colonne SQL `has_jayxpose` reste présente
  dans les bases existantes (no-op). Pour les nouvelles bases, elle n'est
  plus créée. Aucune migration `DROP COLUMN` n'a été appliquée.

---

## JayFestival — retiré le 2026-04-29

### Description (historique)

JayFestival était le service B2B2C de gestion d'événements et festivals.
Il couvrait trois personas (Organisateur, Exposant, Visiteur) avec des
parcours croisés et environ 35 écrans répartis comme suit :

- **Organisateur** : éditions, programme, exposants, plan, billetterie,
  budget, équipe, documents, publication
- **Exposant** : candidatures, participations, agenda, factures, fiche
  publique, notifications
- **Visiteur** : catalogue d'éditions, billets, réservations, agenda,
  activités, dashboard

### Périmètre supprimé

- `crates/jayfestival/` (crate domaine + manifest + adapters)
- `apps/jayfestival/` (binaire Dioxus standalone)
- `apps/central/src/services/jayfestival/` (~35 fichiers UI dans Central)
- `apps/central/src/services/jayfestival_view.rs` (vue d'entrée)
- `crates/jaykoa/src/services/jayfestival/` (adaptateur synchronisation)
- `apps/central/src/services/jaykoa/sync_service.rs` (sync JayFestival
  vers JayKoa)
- Variantes d'enum `EventSource::JayFestival` et
  `EntryType::ReflectJayFestival` dans `crates/jaykoa/src/data/types.rs`
- Évènements d'intégration `JayFestivalEvent` (CK-INT-01) et payload
  `ReportByEditionPayload` dans `crates/jaykonta/src/integrations/`
- Méthode `JayKontaBackend::ingest_jayfestival_event` et bootstrap
  `bootstrap_int_01`
- Documentation `docs/services/JayFestival/`

### Conséquences pour les services restants

- **JayKoa** : la synchronisation des éditions JayFestival vers
  l'agenda universel n'existe plus. L'enum `EventSource` ne contient
  plus la variante `JayFestival` ; les bases JayKoa existantes qui
  contiendraient des entries de type `reflect_jayfestival` les liraient
  comme `EntryType::Internal` (fallback).
- **JayKonta** : le contrat **CK-INT-01 (JayFestival → JayKonta)** est
  retiré. Les contrats CK-INT-02 (JayRDV) et CK-INT-03 (JayKoa) restent
  actifs. Les bootstraps de seed n'incluent plus d'éditions de festival.
- **JayFaim** : l'intégration "stands restauration sur événements" qui
  s'appuyait sur JayFestival n'a plus de service appelant.
- **Origin tracker / catalog** : les fixtures de test qui utilisaient
  `"jayfestival"` comme service ID exemple ont été basculées vers
  `"jaykoa"`.

---

## Process suivi pour le retrait

1. **Inventaire** — scan complet du workspace (175 fichiers `.rs`,
   80+ docs, indexes mscm) pour identifier toutes les références
2. **Suppression des dossiers dédiés** — crates et apps supprimés en
   bloc, entrées workspace `Cargo.toml` retirées
3. **Refactor des sites d'usage** — chaque crate dépendant a été nettoyé
   pour retirer les imports, types, méthodes, fixtures de test
4. **Nettoyage de la documentation** — `docs/services/JayXpose/` et
   `docs/services/JayFestival/` supprimés ; références dans les docs
   inter-services nettoyées ; site officiel mis à jour
5. **Vérification** — `cargo check --workspace` (zéro erreur)
6. **Indexes MSCM régénérés** — files.json, layers.json, domains.json,
   blocks.json nettoyés

## Commit de référence

```
c2f32ecf chore(workspace): suppression des services JayXpose et JayFestival
```

342 fichiers modifiés, ~65 000 lignes nettes supprimées.

---

> Si un service futur devait reprendre certaines fonctionnalités (vitrine,
> billetterie, gestion d'événements), il devra s'inscrire dans le périmètre
> Miyukini en suivant le process Document Fondateur → Spec → Implémentation,
> et non pas restaurer ces dossiers tels quels.
