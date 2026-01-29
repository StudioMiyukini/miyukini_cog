# MiyukiniAdmin — Dashboard HyperUI Implementation

## 1. Contexte

Ce document definit comment le **Dashboard** (et les pages DB associees) de MiyukiniAdmin sont realises a partir de composants **HyperUI** **copies** dans le crate MiyukiniAdmin. Aucune reference au code HyperUI en ligne n'est autorisee : tous les composants utilises sont des copies locales dans le crate, pour respecter le **bornage** et la **modularite** (INV-MA-7 : UI propre, isolee, non reutilisable).

**Source des composants a copier :** uniquement le depot local `docs/ux_ui/ui/hyperui/components/` et `docs/ux_ui/ui/hyperui/assets/` du projet Miyukini. Pas le site HyperUI externe.

## 2. Portee / Scope

Ce document definit :
- La regle de bornage (composants copies dans le crate)
- La liste des composants HyperUI copies et leur emplacement
- La structure de la page Dashboard (header, sidebar, zone principale, footer)
- Les variantes de composants utilisees pour chaque bloc
- La reference a la philosophie UI (couleurs, typo, etats)

Ce document **ne couvre pas** :
- L'implementation backend (voir Architecture & Flows)
- Les contrats de donnees (voir Monitoring contracts)

---

## 3. Regle de bornage

### 3.1 Principe

- **Ne pas** utiliser le code HyperUI depuis la documentation en ligne (hyperui.dev ou autre).
- **Copier** dans le crate MiyukiniAdmin les fichiers HTML/CSS/JS necessaires pour que l'admin soit **autonome** et **modulaire**.
- Toute evolution des composants se fait sur les copies dans `crates/miyukini-admin/ui/` ; la doc `docs/ux_ui/ui/hyperui/` reste une reference de depart, pas une dependance runtime.

### 3.2 Emplacement dans le crate

| Dossier | Contenu |
|--------|---------|
| `crates/miyukini-admin/ui/assets/` | `component.css`, `component.js` copies depuis `docs/ux_ui/ui/hyperui/assets/` |
| `crates/miyukini-admin/ui/components/` | Sous-dossiers par type de composant (stats, tables, side-menu, modals, etc.) ; chaque sous-dossier contient les variantes HTML copiees (ex. `stats/1.html`, `stats/2.html`) |
| `crates/miyukini-admin/ui/pages/` ou `templates/` | Structure des pages (dashboard, database, etc.) assemblant les composants |

---

## 4. Composants HyperUI copies

Les composants suivants sont **copies** depuis `docs/ux_ui/ui/hyperui/components/` vers `crates/miyukini-admin/ui/components/<nom>/`.

| Usage | Composant source (dossier) | Fichiers copies | Emplacement cible |
|-------|----------------------------|------------------|--------------------|
| Layout, navigation | `application/side-menu/` | 1.html, 2.html, 3.html | `ui/components/side-menu/` |
| Metriques, KPI | `application/stats/` | 1.html a 4.html (+ dark si besoin) | `ui/components/stats/` |
| Tableaux (tables DB, migrations, backups, Operateurs) | `application/tables/` | 1.html a 5.html | `ui/components/tables/` |
| Onglets (Structure / Data / Indexes) | `application/tabs/` | 1.html a 5.html | `ui/components/tabs/` |
| Boutons, actions | `application/button-groups/` | 1.html a 5.html | `ui/components/button-groups/` |
| Badges (Trust, Security, statut) | `application/badges/` | 1.html a 5.html | `ui/components/badges/` |
| Modales (confirmations, Recovery, Backup) | `application/modals/` | 1.html a 6.html | `ui/components/modals/` |
| Fil d'Ariane | `application/breadcrumbs/` | 1.html a 5.html | `ui/components/breadcrumbs/` |
| Alertes / toasts | `application/toasts/`, `application/empty-states/` | Variantes utiles | `ui/components/toasts/`, `ui/components/empty-states/` |
| Barres de progression (CPU, RAM, disque) | `application/progress-bars/` | 1.html a 4.html | `ui/components/progress-bars/` |
| Loaders | `application/loaders/` | 1.html a 7.html | `ui/components/loaders/` |

**Assets globaux :** copier `docs/ux_ui/ui/hyperui/assets/component.css` et `component.js` vers `crates/miyukini-admin/ui/assets/`.

---

## 5. Structure de la page Dashboard

### 5.1 Zones

| Zone | Role | Composant HyperUI utilise |
|------|------|----------------------------|
| **Header** | Logo, titre MiyukiniAdmin, alertes, utilisateur, niveau securite | Badges (Trust, Security) ; boutons/liens |
| **Sidebar** | Navigation (Dashboard, Metriques, Database, Tests, Securite, Logs) | side-menu (ex. variante 1 ou 2) |
| **Zone principale** | Contenu du Dashboard | Voir §5.2 |
| **Footer** | Version, uptime, last sync, Trust | Texte + badge Trust |

### 5.2 Zone principale — blocs et variantes

| Bloc | Contenu | Composant / variante |
|------|---------|----------------------|
| **Header statut** | Health (jauge), Trust (badge), Security (badge) | stats (ex. 1 ou 3 pour cartes chiffrees) ; badges (1 ou 2 pour Trust/Security) |
| **System Metrics** | CPU, RAM, Disk, Network (barres ou chiffres) | stats (2, 3 ou 4) ; progress-bars (1 a 4) |
| **Database Metrics** | Queries/s, Latency P95, Pool | stats (1 ou 3) — ligne de 3 cartes |
| **Operators Status** | Tableau (Name, Status, Req/s, Errors, Latency) | tables (2 ou 3) |
| **Recent Alerts** | Liste d'alertes (icone, heure, message) | toasts ou liste custom ; empty-states si aucune alerte |

Les numeros de variantes (1.html, 2.html, etc.) sont a ajuster selon le rendu souhaite ; le present document fixe le **mapping conceptuel** (stats pour KPI, tables pour tableaux, etc.).

### 5.3 Etats speciaux

- **Mode Recovery** : bandeau rouge en haut de la zone principale ; modale ou section dediee (modals). Reference : [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md) §10.1.
- **Mode degrade (T2)** : bandeau orange ; meme philosophie couleurs (voir UI Design Philosophy §5.2, §5.4).
- **Chargement** : loaders (ex. variante 1 ou 2) en overlay ou inline.

---

## 6. Reference a la philosophie UI

- **Couleurs** : Primary, Secondary, Success, Warning, Error, Info — voir [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md) §5.
- **Niveaux de securite (0–4)** et **Trust (T0–T4)** : couleurs et codes associes — voir §5.3, §5.4.
- **Typographie** : Inter (titres, corps), JetBrains Mono (code/donnees) — voir §6.
- **Boutons** : Primary, Secondary, Danger, Ghost — voir §7.1.
- **Confirmations** : actions critiques avec justification — voir §8.1.

Les composants copies peuvent etre adaptes (classes CSS, couleurs) pour coller au design system MiyukiniAdmin ; les assets `component.css` et `component.js` sont la base, pas une dependance externe.

---

## 7. Resume — checklist implementation

1. Creer `crates/miyukini-admin/ui/assets/` et y copier `component.css`, `component.js` depuis `docs/ux_ui/ui/hyperui/assets/`.
2. Creer `crates/miyukini-admin/ui/components/` et pour chaque type (side-menu, stats, tables, tabs, button-groups, badges, modals, breadcrumbs, toasts, empty-states, progress-bars, loaders) copier les fichiers indiques depuis `docs/ux_ui/ui/hyperui/components/application/<nom>/`.
3. Creer la page Dashboard (ou template) en assemblant header, sidebar (side-menu), zone principale (stats, progress-bars, tables, toasts/empty-states), footer.
4. Ne jamais referencer en runtime le dossier `docs/ux_ui/ui/hyperui/` ; tout est autonome dans le crate.

---

## 8. Documents associes

- [MiyukiniAdmin - Dashboard & Metrics Display](./MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md)
- [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md)
- [MiyukiniAdmin - Organisation Pages et UX DB](./MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md)
- [HyperUI - Index Composants](../../../ux_ui/ui/hyperui/HyperUI%20-%20Index%20Composants.md) (reference locale pour choisir les variantes a copier)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (implementation Dashboard HyperUI)
