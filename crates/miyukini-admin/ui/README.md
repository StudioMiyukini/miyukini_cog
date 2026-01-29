# MiyukiniAdmin — UI (Daynight Admin)

## Bornage et modularité

L'interface est inspirée du template **Daynight Admin** (TemplateMo 608) : thème dual (Snow / Carbon), police DM Sans, accent Ice Blue, **HTML/CSS/JS vanilla** sans dépendances externes (hormis Google Fonts). MiyukiniAdmin reste **autonome** et **modulaire** (INV-MA-7 : UI propre, isolée, non réutilisable).

## Structure

- **index.html** — Page d'accueil / dashboard (servie à `/`)
- **database.html** — Page Database CRUD (servie à `/database`)
- **tests.html** — Page Tests de flux (servie à `/tests`)
- **css/theme.css** — Variables de thème (clair/sombre), layout sidebar + main, cartes, boutons, tableaux
- **js/theme.js** — Bascule thème (localStorage, pas de flash blanc)
- **js/app.js** — Menu mobile (sidebar coulissant)
- **assets/** — Réservé pour images ou futurs assets

## Règles

- Aucune dépendance JavaScript lourde (pas de React, Vue, build Node).
- Préférence thème persistée en `localStorage` (`miyukini-admin-theme`).
- Assets statiques servis sous `/ui/` (ex. `/ui/css/theme.css`, `/ui/js/theme.js`).

## Référence

- Template d'inspiration : [Daynight Admin (TemplateMo 608)](https://templatemo.com/tm-608-daynight-admin) — HTML/CSS/vanilla JS, dual theme, gratuit.
