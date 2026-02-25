# JayManga — UI Web Portal

## Contexte

Ce document specifie l'interface utilisateur de JayManga sur le **Miyukini Web Portal** (Portail). Le Portail est la surface web publique de chaque COG — c'est la vitrine accessible a tous les visiteurs, y compris ceux qui ne possedent pas de COG.

> **Regle canonique :** Central = COG, Portail = Web. L'interface Web Portal est destinee aux **lecteurs externes** : decouverte, demonstration, achat, lecture en ligne. Les fonctions d'administration restent dans Central.

Le Portail JayManga doit offrir une experience comparable aux plateformes web de reference (Mangadraft, Manga.io, WEBTOON web, Manga Plus) tout en respectant la souverainete et l'identite de chaque COG vendeur.

Ce document couvre deux surfaces web distinctes :
1. **Portail vendeur** : le catalogue et la liseuse d'un COG vendeur specifique.
2. **Portail Agrege** : l'interface inter-COG unifiee (Type 3), documentee dans [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) pour l'architecture — ici pour l'UI.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md) et au guide transversal [Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md).

---

## 1. Portail vendeur — Catalogue et liseuse

### 1.1 Page d'accueil du catalogue

```
┌──────────────────────────────────────────────────────────────────┐
│  [Logo/Avatar]  Nom de la librairie            [🔍 Recherche]    │
│  "Description courte de la librairie"                             │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │              [Banniere / Oeuvre mise en avant]            │    │
│  │              "Titre" — Nouvelle publication !             │    │
│  │                     [ Lire la demo → ]                    │    │
│  └──────────────────────────────────────────────────────────┘    │
│                                                                    │
│  Nouveautes                                                        │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐              │
│  │cover│ │cover│ │cover│ │cover│ │cover│ │cover│              │
│  │Titre│ │Titre│ │Titre│ │Titre│ │Titre│ │Titre│              │
│  │Grat.│ │3,99€│ │Grat.│ │1,99€│ │Grat.│ │5,99€│              │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘              │
│                                                                    │
│  Par genre                                                         │
│  [Action] [Romance] [Fantasy] [Sci-Fi] [Horreur] [Comedie]      │
│                                                                    │
│  Catalogue complet (42 oeuvres)                                    │
│  [Filtres: Genre ▼  Format ▼  Prix ▼  Langue ▼]  [Tri: ▼]       │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐              │
│  │cover│ │cover│ │cover│ │cover│ │cover│ │cover│              │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘              │
│  ... (pagination)                                                  │
│                                                                    │
│  ─────────────────────────────────────────────────                 │
│  Propulse par JayManga · Miyukini COG                              │
└──────────────────────────────────────────────────────────────────┘
```

### 1.2 Responsive design

| Breakpoint | Layout |
|------------|--------|
| **Desktop** (> 1024px) | Grille 6 colonnes, banniere large, sidebar filtres. |
| **Tablette** (768-1024px) | Grille 4 colonnes, filtres en dropdown. |
| **Mobile** (< 768px) | Grille 2 colonnes, filtres en bottom sheet, banniere plein ecran. |

### 1.3 Personnalisation par le vendeur

Le vendeur configure l'apparence de son Portail depuis Central (`SellerConfig.theme`) :

| Element | Personnalisable |
|---------|----------------|
| **Logo / Avatar** | Image uploadee par le vendeur. |
| **Banniere** | Image de banniere pour la page d'accueil. |
| **Couleurs** | Couleur d'accent, couleur de fond (selection parmi des palettes predefinies ou personnalisee). |
| **Oeuvre mise en avant** | Le vendeur choisit l'oeuvre affichee en hero. |
| **Description** | Texte libre pour presenter la librairie. |
| **Ordre des sections** | Le vendeur peut reordonner : Nouveautes, Par genre, Populaires, Gratuits. |

---

## 2. Fiche oeuvre (Portail vendeur)

### 2.1 Layout desktop

```
┌──────────────────────────────────────────────────────────────────┐
│  ← Retour au catalogue                                           │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  ┌────────────┐   Titre de l'Oeuvre                               │
│  │            │   par Auteur(s)                                    │
│  │ [Couverture│   Genre(s) · Format : Webtoon · FR                │
│  │   grande]  │                                                    │
│  │            │   12 chapitres · 248 pages · 10 pages demo        │
│  │            │                                                    │
│  │            │   ┌─────────────────────────────────┐             │
│  │            │   │  📖 Lire la demo (10 pages)      │             │
│  └────────────┘   └─────────────────────────────────┘             │
│                    ┌─────────────────────────────────┐             │
│                    │  🛒 Acheter — 3,99 €              │             │
│                    └─────────────────────────────────┘             │
│                    [ ♡ Favoris ]  [ ↗ Partager ]                   │
│                                                                    │
│  Synopsis                                                          │
│  Lorem ipsum dolor sit amet, consectetur adipiscing elit...        │
│                                                                    │
│  Chapitres                                                         │
│  ┌──────────────────────────────────────────────────────┐         │
│  │  Ch.1 — Titre du chapitre   24 pages   [Lire →]     │         │
│  │  Ch.2 — Titre du chapitre   22 pages   [Lire →]     │         │
│  │  Ch.3 — Titre du chapitre   20 pages   [🔒 Acheter]  │         │
│  │  ...                                                   │         │
│  └──────────────────────────────────────────────────────┘         │
│                                                                    │
│  Oeuvres du meme auteur                                            │
│  ┌─────┐ ┌─────┐ ┌─────┐                                         │
│  └─────┘ └─────┘ └─────┘                                         │
│                                                                    │
└──────────────────────────────────────────────────────────────────┘
```

### 2.2 Liste des chapitres

| Etat | Affichage |
|------|-----------|
| **Demo** | Chapitres dans la plage de demo : bouton « Lire » actif, pas d'icone verrou. |
| **Payant non achete** | Icone verrou 🔒, bouton « Acheter ». |
| **Achete** | Bouton « Lire » actif, badge « Achete ». |
| **Lecture en cours** | Indicateur de progression (page X / Y). |

### 2.3 Bouton favoris

| Etat lecteur | Comportement |
|-------------|-------------|
| **Lecteur avec COG** | Tap → ajout aux favoris sur le COG du lecteur. Synchronisation via MWS. |
| **Lecteur visiteur** | Tap → favoris en localStorage du navigateur (ephemere). Tooltip : « Cree un COG pour sauvegarder tes favoris definitivement. » (une seule fois). |

---

## 3. Liseuse web (Portail)

### 3.1 Principes

La liseuse web est **le composant central** du Portail. Elle doit etre :

| Principe | Description |
|----------|-------------|
| **Rapide** | Chargement instantane des pages grace aux variantes optimisees (srcset/picture). Pre-chargement des pages suivantes. |
| **Responsive** | Fonctionne sur tous les ecrans (desktop, tablette, mobile). |
| **Immersive** | Mode plein ecran disponible. Fond sombre. Interface minimale. |
| **Accessible** | Navigation clavier et souris sur desktop, gestes tactiles sur mobile. |

### 3.2 Layout liseuse desktop

```
Mode manga (desktop) :

┌──────────────────────────────────────────────────────────────────┐
│  ← Retour    "Titre" — Ch.5         [Mode sombre] [Plein ecran] │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│    [←]    ┌──────────────────────────────────┐    [→]             │
│           │                                    │                   │
│           │         [Page manga]               │                   │
│           │                                    │                   │
│           │                                    │                   │
│           └──────────────────────────────────┘                    │
│                                                                    │
├──────────────────────────────────────────────────────────────────┤
│  ◄ ══════════════════════●══════════════════════ ►                │
│  Page 12 / 24   Ch.5 / 12            +1 XP  🔥 14                │
└──────────────────────────────────────────────────────────────────┘
```

```
Mode webtoon (desktop) :

┌──────────────────────────────────────────────────────────────────┐
│  ← Retour    "Titre" — Ch.3         [Mode sombre] [Plein ecran] │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│          ┌────────────────────────────┐                           │
│          │    [Bande webtoon]          │                           │
│          │    ...                      │                           │
│          │    [Image 1]               │                           │
│          │    [Image 2]               │                           │
│          │    [Image 3]               │                           │
│          │    ...                      │                           │
│          └────────────────────────────┘                           │
│                   ↕ defilement                                    │
│                                                                    │
├──────────────────────────────────────────────────────────────────┤
│  Page 15 / 40   Ch.3 / 8             +1 XP  🔥 14                │
└──────────────────────────────────────────────────────────────────┘
```

### 3.3 Navigation clavier (desktop)

| Touche | Action |
|--------|--------|
| **Fleche droite** / **Espace** | Page suivante (LTR) ou precedente (RTL manga). |
| **Fleche gauche** | Page precedente (LTR) ou suivante (RTL manga). |
| **Fleche haut/bas** | Scroll (webtoon). |
| **F** | Plein ecran (toggle). |
| **D** | Mode sombre (toggle). |
| **Echap** | Quitter le plein ecran ou la liseuse. |
| **+/-** | Zoom avant / arriere. |
| **B** | Marque-page. |
| **Home/End** | Premiere / derniere page du chapitre. |

### 3.4 Barre de progression et XP

La barre inferieure affiche en permanence :

| Element | Description |
|---------|-------------|
| **Slider de pages** | Navigation rapide dans le chapitre. |
| **Indicateur page/chapitre** | « Page 12 / 24 — Ch.5 / 12 ». |
| **XP** | Petit compteur « +1 XP » qui apparait brievement a chaque page (disparait en 1s). |
| **Streak** | Flamme et compteur de jours (discret, coin droit). |

Pour les **lecteurs visiteurs** (sans COG), les XP et streak sont stockes en localStorage et affiches de maniere identique. L'incitation a creer un COG n'apparait qu'une fois (apres le niveau 2).

### 3.5 Ecran de fin de demo

A la fin des pages de demonstration, un ecran s'intercale :

```
┌──────────────────────────────────────────────────────────────────┐
│                                                                    │
│                Fin des pages de demonstration                      │
│                                                                    │
│                Tu as lu 10 pages sur 120                           │
│                                                                    │
│     [cover]    "Titre de l'Oeuvre"                                 │
│                par Auteur(s)                                       │
│                12 chapitres · 120 pages restantes                  │
│                                                                    │
│                ┌───────────────────────────────┐                  │
│                │  🛒 Acheter l'oeuvre — 3,99 €   │                  │
│                └───────────────────────────────┘                  │
│                                                                    │
│                [ ♡ Ajouter aux favoris ]                           │
│                                                                    │
│                [ ← Revenir au catalogue ]                          │
│                                                                    │
│                Miou : "Tu as aime ? L'aventure                     │
│                 continue pour 3,99 €."                             │
│                (premiere visite uniquement)                         │
│                                                                    │
└──────────────────────────────────────────────────────────────────┘
```

---

## 4. Portail Agrege — Interface web

### 4.1 Differenciation visuelle

Le Portail Agrege a une **identite visuelle distincte** du Portail vendeur :

| Element | Portail vendeur | Portail Agrege |
|---------|-----------------|----------------|
| **Logo** | Logo/avatar du vendeur | Logo du Portail Agrege (configurable par l'admin aggregateur) |
| **Couleurs** | Theme du vendeur | Theme de l'aggregateur |
| **Contenu** | Catalogue d'un seul vendeur | Catalogues de tous les COGs indexes |
| **Indicateurs de presence** | Non necessaires (c'est le COG du vendeur) | 🟢 en ligne / ⚫ hors-ligne sur chaque oeuvre et vendeur |
| **Action sur une oeuvre** | Lire / acheter directement | Redirection vers le Portail du vendeur |

### 4.2 Page d'accueil Portail Agrege (web)

L'architecture fonctionnelle est documentee dans [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md). Ici, on detaille l'adaptation UI web :

```
┌──────────────────────────────────────────────────────────────────┐
│  [Logo]  Nom du Portail Agrege          [🔍 Recherche]           │
│  "Tous les manga du reseau Miyukini"                              │
├──────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Tendances                          [ Voir tout → ]               │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐     │
│  │cover│ │cover│ │cover│ │cover│ │cover│ │cover│ │cover│     │
│  │Titre│ │Titre│ │Titre│ │Titre│ │Titre│ │Titre│ │Titre│     │
│  │  🟢 │ │  🟢 │ │  ⚫ │ │  🟢 │ │  🟢 │ │  ⚫ │ │  🟢 │     │
│  │Vend.│ │Vend.│ │Vend.│ │Vend.│ │Vend.│ │Vend.│ │Vend.│     │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘     │
│                                                                    │
│  Nouveautes                         [ Voir tout → ]               │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ...                            │
│  └─────┘ └─────┘ └─────┘ └─────┘                                 │
│                                                                    │
│  Vendeurs en ligne (12)             [ Voir tout → ]               │
│  ┌───────────────┐ ┌───────────────┐ ┌───────────────┐           │
│  │ [avatar]      │ │ [avatar]      │ │ [avatar]      │           │
│  │ COG Alpha     │ │ COG Beta      │ │ COG Gamma     │           │
│  │ 🟢 42 manga   │ │ 🟢 8 manga    │ │ 🟢 23 manga   │           │
│  │ Action, Romce │ │ Webtoon       │ │ Fantasy       │           │
│  └───────────────┘ └───────────────┘ └───────────────┘           │
│                                                                    │
│  Gratuit                            [ Voir tout → ]               │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ...                            │
│  └─────┘ └─────┘ └─────┘ └─────┘                                 │
│                                                                    │
│  Catalogue complet                                                 │
│  [Filtres: Genre ▼  Format ▼  Prix ▼  Langue ▼  Dispo ▼]        │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐              │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ ... pagination │
│                                                                    │
│  ─────────────────────────────────────────────────                 │
│  Propulse par JayManga · Portail Agrege · Miyukini COG             │
└──────────────────────────────────────────────────────────────────┘
```

### 4.3 Carte d'oeuvre Portail Agrege

Chaque carte d'oeuvre sur le Portail Agrege affiche des elements supplementaires par rapport au Portail vendeur :

| Element | Description |
|---------|-------------|
| **Pastille de presence** | 🟢 ou ⚫ selon le statut du COG vendeur. |
| **Nom du vendeur** | Sous le titre, pour identifier la provenance. |
| **Filtre de gris** | Couverture en niveaux de gris a 50% d'opacite si le COG est hors-ligne. |
| **Lien** | Clic → fiche intermediaire sur l'aggregateur → bouton « Lire / Acheter sur [COG] » qui redirige. |

### 4.4 Fiche intermediaire (Portail Agrege)

La fiche intermediaire est documentee en detail dans le [document Portail Agrege section 4.4](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md). L'adaptation UI web ajoute :

| Element | Desktop | Mobile web |
|---------|---------|------------|
| Couverture | Grande, a gauche | Plein ecran en haut. |
| Metadonnees | A droite de la couverture | Sous la couverture. |
| Bouton de redirection | Primaire, centre | Plein largeur, sticky en bas. |
| Oeuvres similaires | Grille horizontale | Carousel swipeable. |

---

## 5. Onboarding Miou sur le Portail web

### 5.1 Implementation

Sur le web, Miou est rendu en **overlay HTML/CSS** (pas d'audio par defaut sur le web pour respecter les autoplay policies des navigateurs) :

| Element | Implementation |
|---------|---------------|
| **Mascotte** | Image animee CSS (apparition douce, legere animation idle). |
| **Bulle de texte** | Tooltip stylise avec fleche vers l'element cible. |
| **Audio** | Optionnel, active par clic sur une icone son dans la bulle Miou. |
| **Stockage** | Flag `jaymanga_onboarding_done` en localStorage. |
| **Dismiss** | Bouton « × » sur chaque bulle. Bouton « Passer le guide » visible en permanence. |

### 5.2 Flux d'onboarding web

| Etape | Miou | Condition |
|-------|------|-----------|
| **1. Premiere visite** | Bulle d'accueil avec mascotte : « Bienvenue ! Ici tu peux lire des manga librement. » | Premiere visite (pas de cookie). |
| **2. Highlight catalogue** | Fleche vers la section « Gratuit » : « Commence ici — tout est gratuit. » | Si le lecteur n'a pas encore ouvert de fiche oeuvre. |
| **3. Premiere lecture** | Animation geste dans la liseuse (fleche de navigation). Disparait apres 2 pages. | Premiere ouverture de la liseuse. |
| **4. Fin** | Pas de message de cloture. L'onboarding est marque comme termine. | — |

---

## 6. Specificites techniques web

### 6.1 Performance

| Mesure | Description |
|--------|-------------|
| **SSR** | Les pages catalogue et fiches oeuvres sont rendues cote serveur (SEO et performance). |
| **Lazy load images** | Les couvertures et pages manga utilisent `loading="lazy"` et `srcset` avec les variantes optimisees. |
| **CDN-like local** | Les variantes optimisees sont servies avec des headers de cache longs (images statiques). |
| **Pre-chargement liseuse** | Les 3 pages suivantes sont pre-chargees en `<link rel="preload">`. |
| **WebP/AVIF** | Element `<picture>` avec fallback JPEG pour les navigateurs anciens. |

### 6.2 SEO

| Element | Implementation |
|---------|---------------|
| **Balises meta** | `title`, `description`, `og:image` pour chaque fiche oeuvre. |
| **Structured data** | JSON-LD `CreativeWork` pour chaque oeuvre (titre, auteur, genre, image). |
| **URLs propres** | `/jaymanga/work/{slug}`, `/jaymanga/series/{slug}`, `/jaymanga/genre/{name}`. |
| **Sitemap** | Genere automatiquement a partir du catalogue publie. |
| **Robots.txt** | Pages de demo accessibles aux robots. Pages de contenu payant bloquees. |

### 6.3 Accessibilite web

| Mesure | Description |
|--------|-------------|
| **ARIA** | Labels et roles sur tous les elements interactifs (boutons, liens, sliders, navigation). |
| **Navigation clavier** | Tab order logique. Focus visible. Raccourcis clavier dans la liseuse. |
| **Contraste** | WCAG 2.1 AA : ratio minimum 4.5:1 pour le texte. |
| **Texte alternatif** | `alt` descriptif sur les couvertures (titre + auteur). Pages manga : `alt="Page X du chapitre Y"`. |
| **Motion reduced** | Respecte `prefers-reduced-motion` pour les animations Miou et les transitions. |

### 6.4 Securite

| Mesure | Description |
|--------|-------------|
| **Anti-scraping** | Les pages payantes ne sont pas accessibles sans licence. Les images sont servies via un endpoint protege avec verification de session / licence. |
| **Hotlink protection** | Les images de pages manga ne sont pas hotlinkables (verification du referer + token). |
| **Rate limiting** | Limite de requetes par IP sur les endpoints de lecture (empeche le telechargement automatise). |
| **CSP** | Content Security Policy stricte pour le Portail. |

---

## 7. Comparaison des 3 interfaces

| Aspect | Central / Stable | Mobile / Terminal | Web Portal |
|--------|-----------------|-------------------|------------|
| **Technologie** | Dioxus natif (Rust) | App native (COG TERMINAL) | HTML/CSS/JS (SSR) |
| **Public** | Proprietaire du COG (vendeur + lecteur) | Lecteur authentifie (enfant COG Stable) | Tous (visiteurs + lecteurs authentifies) |
| **Fonctions vendeur** | Toutes (publication, ventes, config) | Aucune | Aucune |
| **Fonctions lecteur** | Bibliotheque, liseuse locale, progression | Bibliotheque, liseuse mobile, progression | Catalogue, liseuse web, demo, achat, progression ephemere |
| **Stockage progression** | KindMother (COG local) | KindMother (COG TERMINAL) + sync Stable | localStorage (visiteur) ou KindMother (authentifie) |
| **Miou** | Audio natif + bulles | Notifications push + bulles | Overlay HTML (audio opt-in) |
| **Mode hors-ligne** | Complet (oeuvres telechargees) | Complet (oeuvres telechargees) | Non (web = en ligne) |
| **Portail Agrege** | Configuration depuis Central | Exploration via onglet Explorer | Surface web complete |

---

## 8. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Mecanismes transversaux (onboarding, XP, streaks, badges). |
| [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) | Architecture du Portail Agrege (federation, cache, moderation). |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Specification de la liseuse (modes, formats). |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Interface desktop. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Interface mobile. |

---

**Document** : JayManga — UI Web Portal
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification UI/UX — interface Web Portal.
