Miyukini Framework - Tests & Monitoring

## Contexte

Pour garantir la stabilité de l’ossature Miyukini (UI, API, auth, super admin), il faut un plan de tests & monitoring qui couvre chaque couche, documente les intersections critiques et incite à ajouter des logs là où le risque est élevé (auth, role switch, navigation super admin). Ce document formalise les suites, les métriques à surveiller et les emplacements de logging obligatoires.

## Portée / Scope

- Définir les suites de tests (UI/storybook, intégration API, auth flows, super admin).
- Identifier les métriques à monitorer et les dashboards associés.
- Spécifier les intersections critiques où des logs doivent être présents (auth, role switch, back office actions, UI Kit errors).
- Fournir la checklist “loggable” à appliquer à chaque nouvelle feature.


## 1. Tests

### 1.1 UI / stories / responsivité

- Storybook + MDX pour chaque composant (atoms/molecules/organisms/pages). Automatiser via `chromatic` ou `percy` pour visual diff.
- Tests responsives : vérifier `hero`, `GestionLayout`, `BottomNav` sur `<800px` et `>=800px`.
- Tests d’accessibilité (axe) sur `Layout`, `HomePage`, `GestionBottomNav`.
- E2E légers (Cypress/Playwright) couvrent les flows :
  - authentification (login/logout/MFA),
  - navigation `header/body/bottom`,
  - role switch + bottom nav visibility.

### 1.2 API / contrats

- Contrats OpenAPI/Swagger pour `/api/auth/*`, `/api/accounts`, `/api/admin/*`, `/api/superadmin/*`, `/api/layout/*`.
- Tests d’intégration (Jest/Supertest) sur API critiques : `auth login`, `role-switch`, `admin actions`.
- Tests MSW pour front : mock `/api/layout` pour valider `header/bottom/body`.
- Tests de charge légers sur endpoints super admin (audit, impersonate).

### 1.3 Auth & comptes

- Tester `RoleSimulationContext` : switch rôle succeed/fail.
- Tester refresh token, session expiry, MFA fallback (mock server).
- Vérifier que `BottomNav` suit les permissions (visitor vs admin).

### 1.4 Super admin & back office

- Tests UI du back office (menus, bottom nav, actions critiques).
- Tests API `admin/categories/:id/actions`, `superadmin/users/:id/impersonate`.
- Vérifier que les dashboards `superadmin/metrics` présentent les données attendues (mocks).


## 2. Monitoring

### 2.1 Logs

- **Auth / role switch** : logger success/failure (source IP, user_id, role). Backends `POST /api/auth/*`, `role-switch`.
- **Bottom nav / layout sync** : log mismatch when backend returns `rolesAllowed` incohérent.
- **UI Kit errors** : `loggingClient` capture les erreurs de rendu (themes manquants, tokens absents).
- **Back office / super admin actions** : chaque action critique (`create`, `delete`, `impersonate`, `export`) émet un log audit (user, action, timestamp).
- **Webhooks** : log inbound payload + processing outcome (success/fail).

### 2.2 Metrics & alertes

- Dashboard (Observability) :
  - Auth failures rate (> 5% sur 5 min) = alerte (Slack).
  - API 5xx (admin, superadmin) = alert.
  - UI errors (loggingClient tags) = ticket creation.
- Monitorer la latence des endpoints `/api/layout/*` pour éviter que le header/body/bottom freeze.
- Superviser les jobs cron (exports, notifications) depuis infra (logs + status page).

### 2.3 Monitoring front

- `loggingClient` capture erreurs React (UI Kit, theme). Tag : `component`, `story`.
- Reporter les event `RoleSimulationButton` failures.
- Surveiller la disponibilité de Storybook/stories (CI).


## 3. Checklist logs obligatoires

- [ ] Auth : login/logout/refresh/role-switch loggés (success + error).
- [ ] Layout sync : backend `rolesAllowed` retourne, front log mismatch.
- [ ] Back office actions : `POST /api/admin/...` log action.
- [ ] Super admin actions : impersonate/export/logs loggés.
- [ ] UI Kit : chaque nouveau composant lève `loggingClient` si tokens manquants.
- [ ] Webhooks : inbound + traitement loggé.


## 4. Observabilité & démarche

- Agréger les logs dans un tool (Chronicle/Datadog/Logflare) avec tags `layer:ui/api/auth/superadmin`.
- Documenter les dashboards (`docs/qa/monitoring`) et mettre à jour quand un nouvel indicateur est ajouté.
- Revue après incident : ajouter un cas de test si un incident a causé une rupture.

> Ce plan de tests & monitoring institutionnalise un regard multidimensionnel (UI/API/auth/super admin). Les logs sont exigés sur toutes les intersections critiques pour détecter les anomalies rapidement et prioriser les corrections.
