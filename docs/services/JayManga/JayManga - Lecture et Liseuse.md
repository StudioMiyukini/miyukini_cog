# JayManga — Lecture et Liseuse

## Contexte

Ce document detaille les fonctionnalites de **lecture en ligne et hors-ligne** de JayManga, ainsi que le composant **liseuse** (reader) qui assure l'affichage des pages manga. La liseuse existe en deux variantes : une version **web** (Portail, pour les lecteurs distants) et une version **native** (Central, pour la lecture locale et hors-ligne). Les deux partagent le meme comportement fonctionnel.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Composant liseuse — Vue d'ensemble

### 1.1 Deux variantes, un comportement

| Variante | Emplacement | Technologie | Usage |
|----------|-------------|-------------|-------|
| **Liseuse web** | Portail (surface web du COG vendeur) | HTML/CSS/JS rendu par le Portail | Lecture en ligne par les lecteurs distants. Les pages sont servies depuis le COG vendeur. |
| **Liseuse native** | Central (application desktop/mobile du COG lecteur) | Composant Dioxus (trait `ServiceUi`) | Lecture locale des oeuvres telechargees. Fonctionne hors-ligne. |

Les deux variantes implementent les memes fonctionnalites de navigation, d'affichage et de configuration.

### 1.2 Interface principale

```
┌─────────────────────────────────────────────┐
│  ← Retour   Titre - Chapitre X   ⚙ Config  │  ← Barre de navigation
├─────────────────────────────────────────────┤
│                                             │
│                                             │
│           [   Page manga   ]                │  ← Zone d'affichage
│                                             │
│                                             │
├─────────────────────────────────────────────┤
│  ◀ Prev   Page 12 / 48   ▶ Next            │  ← Barre de controle
│  ░░░░░░░░░░░░░░░▓▓▓░░░░░░░░░░░░░░░░░░░░░  │  ← Barre de progression
└─────────────────────────────────────────────┘
```

---

## 2. Modes de lecture par format

### 2.1 Mode Manga (`manga`)

| Aspect | Comportement |
|--------|-------------|
| **Sens de lecture** | Droite a gauche (RTL). |
| **Navigation** | Page par page. Clic/tap zone droite = page precedente, zone gauche = page suivante (inverse du sens occidental). |
| **Double-page (desktop)** | En mode plein ecran desktop, deux pages sont affichees cote a cote. La page impaire a droite, la page paire a gauche (convention manga). La premiere page (couverture de chapitre) est toujours affichee seule. |
| **Mobile** | Page unique en portrait. Double-page en paysage (si l'ecran le permet). |
| **Transitions** | Glissement horizontal (droite vers gauche). |

### 2.2 Mode Webtoon (`webtoon`)

| Aspect | Comportement |
|--------|-------------|
| **Sens de lecture** | Defilement vertical continu (haut en bas). |
| **Navigation** | Scroll vertical. Pas de concept de « page suivante/precedente » ; defilement fluide. |
| **Chargement** | Lazy loading : les panneaux sont charges au fur et a mesure du defilement. Un indicateur de chargement s'affiche pour les panneaux non encore charges. |
| **Espacement** | Un leger espacement vertical entre les panneaux (configurable : 0px, 4px, 8px). |
| **Progression** | Basee sur le pourcentage de defilement dans le chapitre. |
| **Chapitrage** | A la fin d'un chapitre, un bouton « Chapitre suivant » apparait. Le defilement peut enchainer automatiquement (optionnel). |

### 2.3 Mode Paysage / 16:9 (`landscape`)

| Aspect | Comportement |
|--------|-------------|
| **Sens de lecture** | Gauche a droite (LTR). |
| **Navigation** | Page par page. Optimise pour les ecrans larges. |
| **Affichage** | La page occupe toute la largeur du viewport. Redimensionnement proportionnel en hauteur. |
| **Mobile portrait** | La page est affichee en entier avec des marges noire en haut et en bas (letterbox). L'utilisateur peut zoomer. |
| **Mobile paysage** | Affichage plein ecran optimal. |
| **Usage** | Illustrations panoramiques, planches cinematiques, double-pages, BD horizontales. |

### 2.4 Mode Comics (`comics`)

| Aspect | Comportement |
|--------|-------------|
| **Sens de lecture** | Gauche a droite (LTR). |
| **Navigation** | Page par page. Clic/tap zone gauche = page precedente, zone droite = page suivante (sens occidental). |
| **Double-page** | Meme logique que le mode manga mais en sens inverse. Page impaire a gauche, page paire a droite. |
| **Transitions** | Glissement horizontal (gauche vers droite). |

### 2.5 Mode libre (`free`)

| Aspect | Comportement |
|--------|-------------|
| **Sens de lecture** | Configurable par l'oeuvre (LTR ou RTL). Defaut : LTR. |
| **Ratio variable** | Chaque page peut avoir un ratio different. La liseuse adapte l'affichage page par page. |
| **Pages portrait** | Centrees, redimensionnees en hauteur. |
| **Pages paysage** | Centrees, redimensionnees en largeur. |
| **Navigation** | Page par page (par defaut). |

---

## 3. Fonctionnalites de la liseuse

### 3.1 Navigation

| Fonctionnalite | Description |
|----------------|-------------|
| **Page suivante / precedente** | Clic/tap sur zones de navigation, touches clavier (fleches, Espace), swipe tactile. |
| **Saut de chapitre** | Boutons « Chapitre precedent / suivant » dans la barre de navigation. |
| **Saut de page** | Clic sur la barre de progression pour aller directement a une page. Saisie du numero de page. |
| **Table des matieres** | Panneau lateral listant les chapitres de l'oeuvre avec le nombre de pages et le statut de lecture (lu, en cours, non lu). |
| **Raccourcis clavier** | Fleches : navigation. F : plein ecran. Espace : page suivante. Echap : quitter plein ecran. D : mode double-page. N : mode sombre. |

### 3.2 Affichage

| Fonctionnalite | Description |
|----------------|-------------|
| **Zoom** | Zoom par pinch (tactile) ou molette (desktop). Niveaux : ajuste a la page, ajuste a la largeur, 100%, 150%, 200%. |
| **Mode plein ecran** | Masque toutes les barres du navigateur et de la liseuse. Seule la page est affichee. Les controles apparaissent au tap/mouvement de souris. |
| **Mode sombre** | Fond sombre autour de la page. Optionnel : filtre d'assombrissement leger sur la page elle-meme (configurable). |
| **Mode double-page** | Desktop uniquement. Affiche deux pages cote a cote. Activable/desactivable manuellement. |
| **Orientation automatique** | Sur mobile, la liseuse suggere l'orientation optimale (portrait pour manga/comics, paysage pour landscape). |
| **Redimensionnement adaptatif** | La taille de la page s'adapte en temps reel au redimensionnement de la fenetre. |

### 3.3 Marque-page et progression

| Fonctionnalite | Description |
|----------------|-------------|
| **Sauvegarde automatique** | La position de lecture (chapitre + page) est sauvegardee automatiquement a chaque changement de page. |
| **Reprise de lecture** | A la reouverture d'une oeuvre, la liseuse propose de reprendre a la derniere page lue. |
| **Barre de progression** | Indicateur visuel du pourcentage de lecture dans le chapitre en cours et dans l'oeuvre globale. |
| **Marque-pages manuels** | Le lecteur peut poser des marque-pages nommes sur des pages specifiques (Phase 2). |

### 3.4 Chargement et performance

| Fonctionnalite | Description |
|----------------|-------------|
| **Selection de variante** | La liseuse selectionne automatiquement la variante optimisee la plus appropriee en fonction de l'ecran et de la connexion (voir [Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md), section 3.7). |
| **Pre-chargement** | Les 2-3 pages suivantes sont pre-chargees en arriere-plan pour un affichage instantane. |
| **Placeholder** | Pendant le chargement, un placeholder (miniature floue ou squelette) est affiche a la taille exacte de la page pour eviter les sauts de mise en page. |
| **Mode basse qualite** | Sur connexion lente detectee, la liseuse bascule automatiquement vers le profil mobile (plus leger). Un bouton permet de forcer la HD. |
| **Cache navigateur** | Les pages deja lues sont mises en cache dans le navigateur pour un retour instantane. |

---

## 4. Pages de demonstration

### 4.1 Comportement dans la liseuse

La liseuse applique les regles de demonstration definies par le vendeur :

1. Les **N premieres pages** de l'oeuvre (selon `demo_pages_count`) sont accessibles sans licence.
2. La navigation est libre au sein des pages de demonstration.
3. A la derniere page de demonstration, un **ecran d'incitation** s'affiche :

```
┌─────────────────────────────────────────────┐
│                                             │
│         Fin de la demonstration             │
│                                             │
│    [Couverture]    Titre de l'oeuvre         │
│                    Auteur(s)                 │
│                    XX chapitres restants     │
│                                             │
│         ┌──────────────────────┐            │
│         │  Acheter — 4,99 €    │            │
│         └──────────────────────┘            │
│                                             │
│         Ajouter aux favoris                 │
│                                             │
└─────────────────────────────────────────────┘
```

4. Les pages au-dela de la demonstration retournent une erreur 403 si le lecteur tente d'y acceder directement (URL).

### 4.2 Indicateur visuel

Dans la barre de progression, la zone de demonstration est marquee visuellement (couleur differente ou separateur) pour que le lecteur sache ou se termine l'acces gratuit.

---

## 5. Lecture hors-ligne (Central)

### 5.1 Comportement

La liseuse native dans Central permet de lire les oeuvres telechargees sans connexion reseau. Le comportement est identique a la liseuse web, avec les differences suivantes :

| Aspect | Liseuse web (Portail) | Liseuse native (Central) |
|--------|----------------------|--------------------------|
| Source des pages | COG vendeur (reseau) | Fichiers locaux (KindMother) |
| Variante servie | Selectionne selon ecran/connexion | Original ou HD (fichier local) |
| Connexion requise | Oui | Non |
| Sauvegarde progression | Cache local navigateur + favori | Stockage local KindMother |
| Pre-chargement | Oui (reseau) | Instantane (fichiers locaux) |

### 5.2 Synchronisation de la progression

Si le lecteur lit une oeuvre hors-ligne puis se reconnecte, la progression locale est synchronisee avec le favori cross-COG (si l'oeuvre est en favoris). La progression la plus avancee est conservee en cas de conflit.

---

## 6. Accessibilite et ergonomie

| Fonctionnalite | Description |
|----------------|-------------|
| **Navigation tactile** | Swipe gauche/droite pour la navigation (manga, comics, landscape, free). Swipe vertical pour le defilement (webtoon). |
| **Zones de tap** | Zones de navigation definies : 1/3 gauche = page precedente, 1/3 droit = page suivante, 1/3 central = afficher/masquer les controles. Ajustable dans les preferences. |
| **Contraste** | Le fond autour de la page est toujours sombre (mode sombre) ou clair, selon le theme. Le contenu de la page n'est jamais modifie. |
| **Taille du texte** | Les controles et menus de la liseuse respectent les preferences de taille de texte du systeme. |
| **Orientation** | La liseuse ne verrouille jamais l'orientation de l'ecran ; elle s'adapte. |

---

## 7. Configuration utilisateur (Preferences lecteur)

| Preference | Options | Defaut |
|------------|---------|--------|
| Mode d'affichage | Page simple / Double-page / Auto | Auto |
| Mode sombre | Active / Desactive | Desactive |
| Qualite d'image | Auto / HD / SD / Mobile | Auto |
| Pre-chargement | Active / Desactive | Active |
| Espacement webtoon | 0px / 4px / 8px | 4px |
| Enchainement chapitres (webtoon) | Auto / Manuel | Manuel |
| Sens de navigation personnalise | LTR / RTL / Auto (suit le format) | Auto |
| Zones de tap | Standard / Inversees / Larges | Standard |

---

## 8. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md) | Formats, import, outil d'optimisation. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Favoris, telechargement hors-ligne, progression. |

---

**Document** : JayManga — Lecture et Liseuse
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle detaillee.
