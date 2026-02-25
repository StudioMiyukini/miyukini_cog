# JayManga — UI Mobile Terminal

## Contexte

Ce document specifie l'interface utilisateur de JayManga pour les COGs de type **TERMINAL** (Android, iOS). Un COG TERMINAL est un **enfant d'un COG Stable** du meme utilisateur : il herite de sa bibliotheque, de ses achats et de sa progression, tout en offrant une experience optimisee pour les ecrans tactiles et la lecture mobile.

L'interface mobile est le **terrain naturel de la lecture manga** : la majorite des lecteurs de manga numerique lisent sur mobile (WEBTOON : 85% mobile, Manga Plus : 90% mobile). JayManga Mobile doit offrir une experience de lecture **sans friction**, avec un acces immediat a la bibliotheque et une navigation en un pouce.

> **Principe fondamental :** L'interface mobile est centree sur la **lecture**. Les fonctions d'administration et de publication restent sur Central/Stable. Le mobile est un **lecteur pur** avec bibliotheque, progression et decouverte.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md) et au guide transversal [Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md).

---

## 1. Perimetre fonctionnel

### 1.1 Ce que le mobile fait

| Fonction | Description |
|----------|-------------|
| **Bibliotheque** | Acces a tous les favoris, achats, telechargements. Synchronise avec le COG Stable parent. |
| **Liseuse** | Lecture optimisee tactile (swipe, scroll, pinch-to-zoom). |
| **Progression** | XP, streaks, badges, niveaux — synchronises avec le COG Stable. |
| **Decouverte** | Navigation dans les catalogues via le Portail Agrege ou directement sur les Portails vendeurs. |
| **Telechargement** | Telechargement d'oeuvres achetees pour lecture hors-ligne. |
| **Notifications** | Push pour les streaks, badges, mises a jour d'oeuvres suivies, retour en ligne d'un COG favori. |

### 1.2 Ce que le mobile ne fait PAS

| Exclusion | Raison |
|-----------|--------|
| Publication / import d'oeuvres | Reserve a Central/Stable (ecran large, gestion de fichiers). |
| Administration des ventes | Reserve a Central/Stable (tableaux, exports). |
| Configuration du Portail Agrege | Reserve a Central/Stable. |
| Paiement direct (V1) | Le paiement se fait sur le Portail web du vendeur (redirection navigateur). Le mobile offre un raccourci. |

---

## 2. Architecture de navigation

### 2.1 Barre de navigation inferieure

L'interface mobile utilise une **barre de navigation inferieure a 4 onglets** (pattern standard iOS/Android, inspire de WEBTOON et Tachiyomi) :

```
┌─────────────────────────────────────────┐
│                                           │
│              [Contenu ecran]              │
│                                           │
├─────────────────────────────────────────┤
│  📖 Biblio  │  🔍 Explorer  │  🔥 Progres  │  ⚙️ Plus  │
└─────────────────────────────────────────┘
```

| Onglet | Icone | Ecran | Description |
|--------|-------|-------|-------------|
| **Biblio** | Livre ouvert | BiblioScreen | Bibliotheque personnelle : favoris, achats, telechargements, en cours, termines. |
| **Explorer** | Loupe | ExploreScreen | Decouverte : Portail Agrege integre, recherche, tendances, nouveautes. |
| **Progres** | Flamme | ProgressScreen | Streaks, niveaux, badges, statistiques de lecture. |
| **Plus** | Engrenage | MoreScreen | Preferences de lecture, compte, synchronisation, notifications. |

### 2.2 Ecrans principaux

```
Mobile App
  ├── BiblioScreen
  │     ├── En cours (default)
  │     ├── Favoris
  │     ├── Achats
  │     ├── Telecharges
  │     └── Termines
  ├── ExploreScreen
  │     ├── Recherche
  │     ├── Tendances
  │     ├── Nouveautes
  │     ├── Gratuit
  │     └── Vendeurs
  ├── ProgressScreen
  │     ├── Vue d'ensemble (niveau, XP, streak)
  │     ├── Badges
  │     └── Statistiques
  ├── MoreScreen
  │     ├── Preferences de lecture
  │     ├── Notifications
  │     ├── Synchronisation
  │     └── A propos
  └── ReaderScreen (plein ecran, hors navigation)
        ├── Mode manga (swipe horizontal RTL)
        ├── Mode webtoon (scroll vertical continu)
        ├── Mode landscape (swipe horizontal)
        └── Mode comics (swipe horizontal LTR)
```

---

## 3. Ecran Bibliotheque (BiblioScreen)

### 3.1 Layout

```
┌─────────────────────────────────────────┐
│  Ma Bibliotheque          🔥 14  Nv.5   │
├─────────────────────────────────────────┤
│  [En cours] [Favoris] [Achats] [⬇️] [✅]│
├─────────────────────────────────────────┤
│                                           │
│  Reprendre la lecture                     │
│  ┌──────────────────────────────────┐    │
│  │ [cover]  Titre de l'oeuvre       │    │
│  │          Ch.5 p.12  ████░░ 65%   │    │
│  │          COG Alpha 🟢            │    │
│  │          [Continuer →]           │    │
│  └──────────────────────────────────┘    │
│                                           │
│  En cours (4)                             │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
│  │cover│ │cover│ │cover│ │cover│       │
│  │ 65% │ │ 40% │ │ 25% │ │ 10% │       │
│  │  🟢 │ │  ⚫ │ │  🟢 │ │  🟢 │       │
│  └─────┘ └─────┘ └─────┘ └─────┘       │
│                                           │
│  Telecharges (6) — disponibles hors-ligne │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ...   │
│  │cover│ │cover│ │cover│ │cover│       │
│  └─────┘ └─────┘ └─────┘ └─────┘       │
│                                           │
└─────────────────────────────────────────┘
```

### 3.2 Comportement specifique

| Element | Comportement |
|---------|-------------|
| **Carte « Reprendre »** | Toujours visible en haut si une lecture est en cours. Un tap ouvre directement la liseuse a la derniere page. |
| **Indicateur de presence** | Pastille verte/grise sur chaque couverture. Rafraichi depuis le cache de presence MWS. |
| **Swipe sur une oeuvre** | Swipe gauche : supprimer des favoris. Swipe droit : marquer comme lu / non lu. |
| **Tap long** | Menu contextuel : Lire, Telecharger, Supprimer, Infos. |
| **Pull-to-refresh** | Rafraichit la presence MWS et synchronise avec le COG Stable parent. |

---

## 4. Liseuse mobile (ReaderScreen)

### 4.1 Principes de design

La liseuse est le **coeur de l'experience mobile**. Elle occupe **100% de l'ecran** (immersion totale) et masque la barre de navigation inferieure et la barre de statut du systeme.

| Principe | Description |
|----------|-------------|
| **Immersion** | Plein ecran, pas de chrome UI sauf geste de rappel. |
| **Gestes naturels** | Navigation par gestes tactiles (swipe, scroll, tap). |
| **Modes adaptatifs** | Le mode de lecture s'adapte au format de l'oeuvre. |
| **Performance** | Pre-chargement des 3 pages suivantes. Variante mobile selectionnee automatiquement (800px). |
| **Confort** | Mode sombre, luminosite adaptative, verrouillage de rotation optionnel. |

### 4.2 Modes de lecture par format

| Format | Navigation | Geste principal | Orientation |
|--------|-----------|-----------------|-------------|
| **Manga** | Swipe horizontal droite → gauche (RTL) | Swipe ou tap zones (gauche = suivant, droite = precedent) | Portrait |
| **Webtoon** | Scroll vertical continu | Defilement au doigt, inertie naturelle | Portrait |
| **Landscape / 16:9** | Swipe horizontal gauche → droite | Swipe ou tap | Paysage (rotation auto) |
| **Comics** | Swipe horizontal gauche → droite (LTR) | Swipe ou tap zones (droite = suivant, gauche = precedent) | Portrait |
| **Free** | Adapte au ratio de l'image | Scroll si vertical, swipe si horizontal | Auto |

### 4.3 Interface de la liseuse

```
Mode manga (plein ecran) :

┌─────────────────────────────────────────┐
│                                           │
│                                           │
│                                           │
│            [Page manga]                   │
│                                           │
│                                           │
│                                           │
│                                           │
│                                           │
│                                           │
│  ← precedent     [tap central]  suivant → │
│                                           │
└─────────────────────────────────────────┘

Tap au centre de l'ecran → barre d'outils :

┌─────────────────────────────────────────┐
│  ← Retour    Ch.5 / 12    ⚙️  [X]       │
├─────────────────────────────────────────┤
│                                           │
│            [Page manga]                   │
│                                           │
├─────────────────────────────────────────┤
│  ◄ ═══════════════●══════════════ ►      │
│  Page 12 / 24         +1 XP              │
└─────────────────────────────────────────┘
```

```
Mode webtoon (scroll vertical) :

┌─────────────────────────────────────────┐
│                                           │
│         [Bande webtoon continue]          │
│         ...                               │
│         [Image 1]                         │
│         [Image 2]                         │
│         [Image 3]                         │
│         ...                               │
│                                           │
│                      ↕ defilement libre   │
│                                           │
├─────────────────────────────────────────┤
│  Ch.3  ════════════●════  p.15/40  +1XP  │
└─────────────────────────────────────────┘
```

### 4.4 Gestes et raccourcis

| Geste | Action |
|-------|--------|
| **Tap centre** | Affiche / masque la barre d'outils et le slider de pages. |
| **Swipe horizontal** (manga/comics) | Page suivante / precedente. |
| **Scroll vertical** (webtoon) | Navigation dans la bande continue. |
| **Pinch** | Zoom avant / arriere. |
| **Double-tap** | Zoom adapte (toggle). |
| **Swipe bas** (barre d'outils visible) | Ferme la barre d'outils. |
| **Tap long** | Menu contextuel : Marque-page, Partager la page, Signaler. |
| **Slider** | Navigation rapide dans le chapitre (drag horizontal en bas). |

### 4.5 Fin de chapitre / fin d'oeuvre

```
Fin de chapitre :

┌─────────────────────────────────────────┐
│                                           │
│         Chapitre 5 termine                │
│         +10 XP (bonus chapitre)           │
│                                           │
│  [← Chapitre precedent]                   │
│                                           │
│  [Chapitre suivant →]      (prochain)     │
│                                           │
│  [Retour a la bibliotheque]               │
│                                           │
└─────────────────────────────────────────┘

Fin d'oeuvre :

┌─────────────────────────────────────────┐
│                                           │
│         🎉 Oeuvre terminee !               │
│         +50 XP (bonus oeuvre)             │
│                                           │
│         Nouveau badge :                   │
│         [Bibliophile — 10 oeuvres]        │
│                                           │
│  [Voir d'autres oeuvres du meme auteur]   │
│  [Explorer des oeuvres similaires]        │
│  [Retour a la bibliotheque]               │
│                                           │
└─────────────────────────────────────────┘
```

### 4.6 Ecran de fin de demo

```
┌─────────────────────────────────────────┐
│                                           │
│     Fin des pages de demonstration        │
│                                           │
│     Tu as lu 10 pages sur 120             │
│                                           │
│     [cover petit]  "Titre de l'oeuvre"    │
│                    3,99 €                 │
│                                           │
│     [ Acheter sur le Portail → ]          │
│       (ouvre le navigateur)               │
│                                           │
│     [ Ajouter aux favoris ♡ ]             │
│                                           │
│     [ Retour ]                            │
│                                           │
└─────────────────────────────────────────┘
```

---

## 5. Ecran Exploration (ExploreScreen)

### 5.1 Layout

L'ecran Explorer integre une vue du **Portail Agrege** optimisee pour le mobile :

```
┌─────────────────────────────────────────┐
│  Explorer                                 │
├─────────────────────────────────────────┤
│  🔍 Rechercher un manga, un auteur...     │
├─────────────────────────────────────────┤
│                                           │
│  Tendances                                │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ →     │
│  │cover│ │cover│ │cover│ │cover│        │
│  │Titre│ │Titre│ │Titre│ │Titre│        │
│  │  🟢 │ │  🟢 │ │  ⚫ │ │  🟢 │        │
│  └─────┘ └─────┘ └─────┘ └─────┘        │
│                                           │
│  Nouveautes                               │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ →     │
│  │cover│ │cover│ │cover│ │cover│        │
│  └─────┘ └─────┘ └─────┘ └─────┘        │
│                                           │
│  Gratuit                                  │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ →     │
│  └─────┘ └─────┘ └─────┘ └─────┘        │
│                                           │
│  Vendeurs en ligne (12)                   │
│  ┌──────────┐ ┌──────────┐ →             │
│  │ COG Alpha│ │ COG Beta │               │
│  │ 🟢 42    │ │ 🟢 8     │               │
│  └──────────┘ └──────────┘               │
│                                           │
└─────────────────────────────────────────┘
```

### 5.2 Recherche

| Element | Comportement |
|---------|-------------|
| **Barre de recherche** | Fixee en haut. Tap ouvre le clavier avec suggestions. |
| **Auto-completion** | Suggestions basees sur les titres, auteurs et genres indexes dans le cache aggrege. |
| **Resultats** | Grille 2 colonnes (couvertures). Filtres accessibles via un bouton « Filtres ». |
| **Filtres** | Bottom sheet : Genre (multi-selection), Format, Prix, Langue, Disponibilite. |

### 5.3 Fiche oeuvre mobile

```
┌─────────────────────────────────────────┐
│  ← Retour                                │
├─────────────────────────────────────────┤
│                                           │
│  ┌──────────────────────────────────┐    │
│  │        [Couverture large]        │    │
│  └──────────────────────────────────┘    │
│                                           │
│  Titre de l'Oeuvre                        │
│  Auteur(s) — Genre(s)                     │
│  Format : Webtoon  ·  FR  ·  🟢 En ligne  │
│  12 chapitres · 248 pages                 │
│  10 pages de demo                         │
│                                           │
│  ┌──────────────────────────────────┐    │
│  │  Lire la demo (10 pages)         │    │
│  └──────────────────────────────────┘    │
│  ┌──────────────────────────────────┐    │
│  │  Acheter — 3,99 €  (sur Portail) │    │
│  └──────────────────────────────────┘    │
│  [ ♡ Favori ]  [ ↗ Partager ]            │
│                                           │
│  Synopsis                                 │
│  Lorem ipsum dolor sit amet...            │
│  [Lire plus]                              │
│                                           │
│  Oeuvres similaires                       │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐       │
│  └─────┘ └─────┘ └─────┘ └─────┘       │
│                                           │
└─────────────────────────────────────────┘
```

---

## 6. Ecran Progression (ProgressScreen)

### 6.1 Layout

```
┌─────────────────────────────────────────┐
│  Ma Progression                           │
├─────────────────────────────────────────┤
│                                           │
│  ┌──────────────────────────────────┐    │
│  │  Niveau 5 — Otaku                │    │
│  │  ████████████░░░░░ 7 200 / 15 000│    │
│  │                                    │    │
│  │     🔥 14 jours de streak          │    │
│  │     ■ ■ ■ ■ ■ ■ ■                │    │
│  │     L M M J V S D                 │    │
│  │     🛡️ Bouclier disponible         │    │
│  └──────────────────────────────────┘    │
│                                           │
│  Statistiques                             │
│  ┌────────┐ ┌────────┐ ┌────────┐       │
│  │  2 340 │ │   18   │ │   42   │       │
│  │  pages │ │ oeuvres│ │ record │       │
│  │  lues  │ │ finies │ │ streak │       │
│  └────────┘ └────────┘ └────────┘       │
│                                           │
│  Badges recents                           │
│  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐    │
│  │ 🏅 │ │ 🔥 │ │ 🧭 │ │ ⭐ │ │ 🔒 │    │
│  └────┘ └────┘ └────┘ └────┘ └────┘    │
│  Centurion  Fidele  Explorateur ...      │
│                                           │
│  [ Voir tous les badges (12/24) ]        │
│                                           │
│  Historique de lecture                     │
│  Aujourd'hui — 45 pages, +52 XP          │
│  Hier — 30 pages, +37 XP                 │
│  ...                                      │
│                                           │
└─────────────────────────────────────────┘
```

### 6.2 Widget streak detaille

| Element | Description |
|---------|-------------|
| **Calendrier semaine** | 7 jours affiches. Jours valides en couleur, jour actuel souligne. |
| **Bouclier** | Icone bouclier si disponible. Tooltip : « Protege ton streak si tu manques un jour. » |
| **Bonus prochain** | « Prochain bonus dans 3 jours (25 XP). » |
| **Animation flamme** | La flamme grandit avec le nombre de jours de streak (petite < 7, moyenne 7-30, grande > 30). |

---

## 7. Onboarding Miou sur mobile

### 7.1 Premiere ouverture

L'onboarding mobile est **plus court** que sur desktop (attention mobile limitee) :

| Etape | Duree | Miou |
|-------|-------|------|
| **1. Ecran d'accueil** | 5 sec | Mascotte Miou animee : « Bienvenue dans JayManga. Ta bibliotheque de manga. » |
| **2. Bibliotheque vide** | 5 sec | « Ta bibliotheque est vide. Explore pour decouvrir des manga. » + highlight onglet Explorer. |
| **3. Premiere oeuvre** | Implicite | Quand le lecteur ouvre une fiche oeuvre : « Tap "Lire la demo" pour commencer. » |
| **4. Premiere lecture** | 3 sec | Animation geste de navigation (swipe ou scroll selon le format). Disparait apres 2 pages. |
| **5. Fin** | Implicite | Pas de message de fin. L'onboarding est marque comme termine. |

Total : moins de 30 secondes d'interruption.

### 7.2 Notifications push (Miou)

| Notification | Declencheur | Message |
|--------------|-------------|---------|
| **Streak en danger** | 20h sans lecture un jour de streak actif | « Tu n'as pas encore lu aujourd'hui. Garde ta flamme. » |
| **Badge debloque** | Obtention d'un badge | « Nouveau badge : [nom]. Bravo. » |
| **Montee de niveau** | Changement de niveau | « Niveau [N] — [Nom]. Continue comme ca. » |
| **Retour en ligne** | COG favori passe de offline a online | « [Nom du vendeur] est de retour en ligne. Tes favoris sont accessibles. » |
| **Mise a jour oeuvre** | Nouveau chapitre sur une oeuvre en favoris | « Nouveau chapitre de [Titre]. » |

Les notifications sont **toutes desactivables individuellement** dans les preferences.

---

## 8. Synchronisation avec le COG Stable parent

### 8.1 Donnees synchronisees

| Donnee | Direction | Methode |
|--------|-----------|---------|
| Favoris | Bidirectionnel | Sync MWS lors de la connexion au COG Stable parent. |
| Licences d'achat | Stable → Mobile | Le mobile recoit les licences du parent. |
| Progression de lecture | Bidirectionnel | XP, streaks, pages lues, dernier chapitre lu. Le plus recent l'emporte. |
| Badges | Bidirectionnel | Union des badges obtenus sur les deux plateformes. |
| Oeuvres telechargees | Independant | Chaque COG telecharge independamment (KindMother local). |
| Preferences de lecture | Bidirectionnel | Mode sombre, direction, qualite d'image. |

### 8.2 Conflit de progression

Si le lecteur lit la meme oeuvre sur Central et sur Mobile :

| Scenario | Resolution |
|----------|-----------|
| Progression differente | La progression la plus avancee l'emporte (dernier chapitre + page le plus grand). |
| XP differents | Addition des XP gagnes sur chaque plateforme depuis la derniere sync. |
| Streak | Le streak est global : une lecture sur l'une ou l'autre plateforme valide la journee. |

### 8.3 Mode hors-ligne

Le mobile fonctionne **hors-ligne par defaut** pour les oeuvres telechargees :

| Etat | Comportement |
|------|-------------|
| **Hors-ligne** | Bibliotheque telechargee accessible. Progression sauvegardee localement. Pas d'exploration ni de presence MWS. |
| **Retour en ligne** | Synchronisation automatique de la progression avec le COG Stable parent. Rafraichissement de la presence MWS. |

---

## 9. Specificites techniques mobiles

### 9.1 Performance

| Mesure | Description |
|--------|-------------|
| **Variante mobile** | Selection automatique de la variante 800px pour les images (optimisation reseau). |
| **Pre-chargement** | 3 pages suivantes pre-chargees en arriere-plan. |
| **Lazy loading** | En mode webtoon : chargement a la demande au scroll (images placeholders). |
| **Cache images** | Cache disque local pour les pages deja lues (limite configurable, defaut : 500 Mo). |
| **Compression reseau** | WebP ou AVIF priorise sur mobile pour reduire la bande passante. |

### 9.2 Accessibilite

| Mesure | Description |
|--------|-------------|
| **Taille des zones tactiles** | Minimum 44x44 points (guideline iOS/Android). |
| **Contraste** | Ratio minimum 4.5:1 pour le texte, 3:1 pour les elements interactifs. |
| **VoiceOver / TalkBack** | Labels accessibles sur tous les elements interactifs. |
| **Mode une main** | Navigation principale accessible depuis le bas de l'ecran (pouce). |
| **Orientation** | Portrait par defaut. Rotation paysage automatique pour le format landscape/16:9. Verrouillage optionnel. |

### 9.3 Stockage

| Mesure | Description |
|--------|-------------|
| **Espace oeuvres** | Stockage KindMother local. L'utilisateur peut voir l'espace utilise par JayManga dans les preferences. |
| **Gestion cache** | Bouton « Vider le cache images » dans les preferences (ne supprime pas les telechargements). |
| **Telechargements selectifs** | Le lecteur choisit quels chapitres telecharger (pas de telechargement force de l'oeuvre entiere). |

---

## 10. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Mecanismes transversaux (onboarding, XP, streaks, badges). |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Specification de la liseuse (modes, navigation, formats). |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Interface desktop pour comparaison et coherence. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Donnees de la bibliotheque lecteur. |

---

**Document** : JayManga — UI Mobile Terminal
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification UI/UX — interface mobile (COG TERMINAL).
