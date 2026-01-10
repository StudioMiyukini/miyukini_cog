# Miyukini Framework - Panneau SuperAdmin

## Contexte

- Le panel SuperAdmin est le cockpit universel du Miyukini Framework, pensé comme un back-office à la WordPress pour chaque SaaS livré. Il permet au développeur principal d’accéder aux fonctions critiques backend (clés, APIs, connexions, scripts) tout en orchestrant la vue front (panneaux, sous-menus, actions dynamiques).
- Cette documentation donne les règles de construction et les points d’extension pour que le panel puisse être déployé telles quelles ou adapté dans n’importe quel produit dérivé du framework.

## Objectifs & Scope

- **Vision** : un tableau de bord unique qui expose les systèmes, autorisations, intégrations et sous-menus dynamiques pilotés par le SuperAdmin.
- **Objectifs**
  1. Lister les zones principales (maintenances, intégrations, layout des sous-menus, accès sécurité) et leurs APIs associées.
  2. Documenter les interfaces de configuration (Vue, APIs, hooks) pour que n’importe quel SaaS puisse réutiliser ou override le panel.
  3. Cataloguer les accès sensibles (clés, connexions HTTP/FTP/SMTP) pour les sécuriser et tracer leur usage.
- **Hors scope** : gestion métier spécifique (booking, inventory...) reste dans les modules concernés ; SuperAdmin orchestre les modules mais ne gère pas les données métier.

## Architecture fonctionnelle

- **Zones du panel**
  1. **Maintenance** : onglets clés/API, logs, redémarrage de services, scripts de migration, supervision des Edge Functions.
  2. **Intégrations** : gestion des connexions HTTP/REST, FTP, SMTP, webhook Supabase, accès SendGrid/Stripe.
  3. **UI Layout** : configuration dynamique des boutons de la barre bottom et des menus secondaires (`GET /ui/bottom-submenu`), ordonnancement, visibilité par rôle.
  4. **Sécurité & RGPD** : rotation des clés, gestion des consentements, audit des sessions.
  5. **Logs, Observabilité & Metrics** : journaux RGPD, erreurs, métriques d’API et de performance (latence, taux d’erreur). Les métriques sont exposées via des endpoints sécurisés destinés aux comptes `super_admin`.
- **Modules techniques**
  - `KeysManager` : CRUD clés API/secret via Supabase secrets + Vault.
  - `IntegrationService` : test de connexions HTTP/FTP/SMTP, ping Webhook.
  - `LayoutConfigurator` : orchestré par `SuperAdminLayout` UI pour ajuster sous-menus en direct.
  - `MaintenanceRunner` : execute scripts SQL/EdgeFunctions, view migrations.
  - `MetricsDashboard` : construit les graphiques et KPIs (GET `/superadmin/metrics`), expose les données uniquement après authentification Supabase (login + OTP/MFA).

## Données & Endpoints

- **Table Supabase `superadmin_config`**
  - `id`, `name`, `scope`, `payload JSONB`, `role_access`, `updated_by`, `updated_at`.
- **Endpoints**
  - `GET /superadmin/keys` / `POST /superadmin/keys` (CRUD clés).
  - `POST /superadmin/integration/test` (payload type, credentials, endpoint).
  - `PUT /superadmin/ui-layout` (ordre des boutons, visibilité).
  - `POST /superadmin/maintenance/run` (scripts SQL, edge function names).
  - `GET /superadmin/logs` (filters par module).
  - `GET /superadmin/metrics` (filtrage par module, intervalle, expose latence/erreur/taux de demande).
  - `POST /superadmin/rm-key` (rotation/invalidates).
- **Webhooks/Automation**
  - Trigger Supabase `superadmin_config_changes` notifying background sync.

## UI & UX du panel

- Layout inspiré WordPress : barre latérale à gauche pour sections, tableau principal à droite. Chaque carte utilise `AppShellScreen` (header/body/bottom) pour rester cohérent avec les écrans standards.
- **Header** : brand + bouton `Retour site`, badge `mode maintenance`, roue param back-office (accès rapide modules, FAQ rapide pour dev).
- **Main body** : `DataGrid` (keys, intégrations, panels) + éditeur JSON inline, `ToastStack` pour les résultats de test, et `MetricsDashboard` (charts, KPI) accessible après login Supabase (plus MFA).
- **Bottom** : actions rapides (Clear cache, Run migration, Valider layout) avec `ActionTray + FAB`.
- Sous-menus dynamiques : configurez la disposition des 5 zones et leurs 6 actions dans `LayoutConfigurator`, accessibles depuis le panel via drag-and-drop ; les modifications s’appliquent immédiatement sur les frontends (broadcast WebSocket).

## Sécurité & gouvernance

- Rôles nécessaires : `super_admin` (full), `admin` (lecture, layout), `maintenance` (lectures/exec scripts limités). Tout changement passe par RLS `superadmin_config_super_admin`.
- Chaque action (clé, API, configuration layout) génère un log `admin_logs` (actor_id, action, payload).
- Stockage des secrets : `KeysManager` chiffré via Supabase `services_role`, rotation programmée.
- RGPD : interface `consents` accessible, log d’export de données, `rgpd-delete-user` utilisable depuis le panel.
- Aucun accès “backdoor” ne sera documenté : les métriques ou accès back-office requièrent l’auth Supabase standard (login + possible MFA). Le panel SuperAdmin utilise ce compte sécurisé pour consommer `/superadmin/metrics`.

## Évolutivité & réutilisation

- Le panel doit s’installer dans `src/features/superadmin/` avec modules `api`, `ui`, `services`, `hooks`.
- Exporter les hooks `useSuperAdminConfig`, `useSuperAdminActions` pour les autres modules.
- Le panel fournit une API publique `SuperAdminAPI` pour scripts CLI (migrations, deploys, housekeeping).
- Documenter chaque champ dans `doc/framework/00_readme/superadmin_panel.md` pour faciliter intégration multi-SaaS.

## Tests & validation

- Tests unitaires sur `KeysManager`, `IntegrationService`.
- Tests d’intégration : `superadmin/integration/test` simulant connexions HTTP/SMTP, vérifier `toast/log`.
- Tests e2e (Playwright) : vérif panel WordPress-style, modif layout bottom, rotation clé, logs.

## FAQ pour développeurs

- Q : Comment étendre un module métier avec du config SuperAdmin ?
  R : Exposer un `superadmin_config` scope spécifique (ex. `booking_notifications`) et mettre à jour `LayoutConfigurator`.
- Q : Comment invoquer la rotation de clés ?
  R : `POST /superadmin/keys/rotate` avec `keyId`; `superadmin_api` déclenche une rotation synchronisée avec Vault.
- Q : Comment personnaliser la palette ?
  R : `GET /superadmin/ui-layout` fournit tokens ; injecter dans UI via `themeUtils`.

