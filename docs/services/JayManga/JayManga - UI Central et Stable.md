# JayManga — UI Central et Stable

## Contexte

Ce document specifie l'interface utilisateur de JayManga dans **Miyukini Central** pour les COGs de type **STABLE** (desktop/laptop). Central est le hub de gestion du COG ; JayManga y est accessible comme service integre, a la fois pour le **vendeur** (admin, publication, ventes) et pour le **lecteur** (bibliotheque, liseuse locale, progression).

L'interface est construite avec **Dioxus** (framework natif Rust), utilise le systeme de theme (`ThemePalette`, `styles`), l'etat global (`AppContext`, `use_app_state`), et le systeme audio de Miou (`audio::play_voice_background`).

> **Regle canonique :** Central = COG. L'interface Central de JayManga est reservee aux utilisateurs du COG (vendeur et lecteur authentifie). Les lecteurs externes accedent au catalogue via le **Portail** (Web Portal).

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md) et au guide transversal [Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md).

---

## 1. Architecture de navigation

### 1.1 Point d'entree dans Central

JayManga est accessible depuis le **Salon** (page d'accueil de Central) via une carte de service (`ServiceCard`) et depuis l'onglet **Bibliotheque** (`MainTab::Bibliotheque`).

```
Central → MainTab
  ├── Salon (Home)
  │     └── ServiceCard "JayManga" → JayMangaView
  ├── Bibliotheque
  │     └── Section JayManga → JayMangaLibraryView
  ├── Communaute (MWS)
  │     └── Decouverte COGs JayManga
  └── Miyukini (Settings)
        └── Configuration JayManga
```

### 1.2 Ecrans JayManga dans Central

| Ecran | Acces | Role |
|-------|-------|------|
| **JayMangaDashboard** | Salon → ServiceCard | Hub JayManga : raccourcis vers le catalogue, les ventes, la bibliotheque. |
| **JayMangaCatalogAdmin** | Dashboard → « Mon Catalogue » | Gestion du catalogue vendeur (liste, ajout, modification, suppression d'oeuvres). |
| **JayMangaWorkEditor** | Catalogue → « Ajouter / Modifier » | Editeur d'oeuvre (import, metadonnees, chapitres, pages, optimisation). |
| **JayMangaSalesAdmin** | Dashboard → « Mes Ventes » | Tableau de bord des ventes, transactions, licences, remboursements. |
| **JayMangaLibrary** | Bibliotheque tab ou Dashboard → « Ma Bibliotheque » | Bibliotheque lecteur : favoris, achats, telechargements, progression. |
| **JayMangaReader** | Bibliotheque → oeuvre → « Lire » | Liseuse locale native (Dioxus) pour les oeuvres telechargees. |
| **JayMangaProfile** | Dashboard → profil lecteur | Profil de progression : niveau, XP, streaks, badges, statistiques. |
| **JayMangaSettings** | Miyukini tab → JayManga | Configuration du service (shop_name, devise, telechargement, federation). |
| **JayMangaAggregator** | Dashboard → « Portail Agrege » | Configuration et monitoring du Portail Agrege (si active). |

---

## 2. Vue vendeur (Admin)

### 2.1 Dashboard vendeur

```
┌─────────────────────────────────────────────────────────────┐
│  JayManga                                          [⚙️]      │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │ Mon Catalogue │  │ Mes Ventes   │  │ Ma Biblio    │       │
│  │ 42 oeuvres   │  │ 156 ventes   │  │ 8 favoris    │       │
│  │ 3 brouillons │  │ ce mois      │  │ 🔥 14 jours   │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│                                                               │
│  Ventes recentes                                              │
│  ┌─────────────────────────────────────────────────────┐     │
│  │ [cover] "Titre"  par Lecteur_xxx  3,99€  il y a 2h │     │
│  │ [cover] "Titre"  par Lecteur_yyy  0,00€  il y a 5h │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                               │
│  Oeuvres populaires                                           │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                            │
│  │cover│ │cover│ │cover│ │cover│                            │
│  │Titre│ │Titre│ │Titre│ │Titre│                            │
│  │120🔵│ │85🔵 │ │64🔵 │ │31🔵 │  (lectures)               │
│  └─────┘ └─────┘ └─────┘ └─────┘                            │
│                                                               │
│  [ + Ajouter une oeuvre ]   [ Portail Agrege ]               │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Editeur d'oeuvre

L'editeur d'oeuvre est un **formulaire multi-etapes** :

| Etape | Contenu |
|-------|---------|
| **1. Fichiers** | Import des pages (drag & drop, selection de fichiers, archives ZIP/CBZ). Preview des pages importees avec reordonnancement. |
| **2. Metadonnees** | Titre, auteurs, genres (multi-selection), synopsis, tags, langue, couverture. |
| **3. Format et structure** | Format de lecture (manga/webtoon/landscape/comics/free), decoupage en chapitres, numerotation, titres de chapitres. |
| **4. Prix et demo** | Modele de tarification (gratuit/payant), prix, devise, nombre de pages de demonstration, autorisation telechargement. |
| **5. Optimisation** | Apercu des variantes optimisees, re-optimisation manuelle, parametres de compression. |
| **6. Publication** | Resume, preview du rendu Portail, bouton « Publier » / « Enregistrer en brouillon ». |

Le passage entre etapes est libre (pas de lineaire impose). Un indicateur de completion montre les etapes validees.

### 2.3 Tableau de bord des ventes

| Section | Contenu |
|---------|---------|
| **Resume** | Revenus du jour/semaine/mois, nombre de ventes, panier moyen. |
| **Graphique** | Courbe de ventes sur 30 jours (barres par jour). |
| **Transactions** | Liste filtrable (date, oeuvre, acheteur, montant, statut). Clic pour details. |
| **Licences** | Liste des licences actives. Bouton de revocation (avec confirmation). |
| **Export** | Bouton export CSV/PDF avec selection de periode. |

---

## 3. Vue lecteur (Bibliotheque)

### 3.1 Ma Bibliotheque JayManga

```
┌─────────────────────────────────────────────────────────────┐
│  Ma Bibliotheque JayManga           Niveau 5 — Otaku        │
│                                      ████████░░ 7200/15000   │
│                                      🔥 14 jours              │
├─────────────────────────────────────────────────────────────┤
│  [Favoris] [Achats] [Telecharges] [En cours] [Termines]    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  En cours de lecture (4)                                      │
│  ┌─────────────────────────────────────────────────────┐     │
│  │ [cover] "Titre A"   Ch.5 / 12   ████████░░ 65%     │     │
│  │          COG Alpha 🟢   [Continuer la lecture]       │     │
│  ├─────────────────────────────────────────────────────┤     │
│  │ [cover] "Titre B"   Ch.2 / 8    ████░░░░░░ 25%     │     │
│  │          COG Beta ⚫    [Hors-ligne — telecharge]    │     │
│  └─────────────────────────────────────────────────────┘     │
│                                                               │
│  Telecharges (6)                                              │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐         │
│  │cover│ │cover│ │cover│ │cover│ │cover│ │cover│         │
│  │ ✅  │ │ ✅  │ │ 65% │ │ ✅  │ │ 30% │ │ ✅  │         │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘         │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Onglets de la bibliotheque

| Onglet | Contenu |
|--------|---------|
| **Favoris** | Toutes les oeuvres mises en favoris, triees par date d'ajout. Statut de presence du COG vendeur. |
| **Achats** | Oeuvres achetees avec licence active. Bouton « Telecharger » si autorise. |
| **Telecharges** | Oeuvres stockees localement (KindMother). Accessibles hors-ligne. |
| **En cours** | Oeuvres dont la progression est entre 1% et 99%. Triees par derniere lecture. |
| **Termines** | Oeuvres lues a 100%. |

### 3.3 Liseuse locale (JayMangaReader)

La liseuse locale dans Central est une implementation **Dioxus native** :

| Fonctionnalite | Implementation |
|-----------------|----------------|
| **Affichage** | Rendu image natif via Dioxus. Variante optimisee selectionnee selon la taille de fenetre. |
| **Navigation manga** | Clic gauche / droite pour naviguer (RTL pour manga, LTR pour comics). Raccourcis clavier : fleches, espace, Page Up/Down. |
| **Navigation webtoon** | Defilement vertical continu. Molette, touchpad, fleches haut/bas. |
| **Mode 16:9** | Images en plein ecran paysage. Navigation par clic ou fleches. |
| **Zoom** | Ctrl+molette ou pinch (touchpad). Double-clic pour zoom adapte. |
| **Plein ecran** | F11 ou double-clic pour basculer. |
| **Mode sombre** | Fond sombre automatique ou manuel. Adapte aux images (bordures sombres, pas d'inversion). |
| **Progression** | Barre de progression en bas : chapitre X / Y, page X / Y. Sauvegarde automatique dans ReaderFavorite. |
| **XP en temps reel** | Petit indicateur « +1 XP » a chaque page (discret, coin inferieur droit, disparait en 1 seconde). |
| **Raccourcis** | `B` : marque-page, `F` : favori, `Echap` : quitter la liseuse, `M` : mode de lecture (simple/double page). |

### 3.4 Profil de progression

Accessible depuis la Bibliotheque ou le Dashboard :

| Section | Contenu |
|---------|---------|
| **Niveau et XP** | Barre de progression visuelle, nom du niveau, XP actuel / prochain niveau. |
| **Streak** | Flamme animee, compteur de jours, calendrier de la semaine (jours valides marques). |
| **Badges** | Grille de badges obtenus (couleur) et non obtenus (gris avec condition). Clic pour detail. |
| **Statistiques** | Pages lues, oeuvres terminees, genres explores, COGs visites, temps de lecture estime. |
| **Historique** | Liste chronologique des lectures recentes (date, oeuvre, pages lues, XP gagnes). |

---

## 4. Onboarding Miou dans Central

### 4.1 Premiere activation (vendeur)

Quand l'admin active JayManga pour la premiere fois :

```rust
use_effect(move || {
    if is_first_activation {
        audio::play_voice_background(&base, "jaymanga/onboarding/welcome_seller.mp3");
        // Affiche le guide d'onboarding
        state.write().jaymanga_onboarding = OnboardingStep::ConfigureShop;
    }
});
```

L'onboarding vendeur suit les 5 etapes definies dans le document transversal (configuration → premiere oeuvre → publication → tableau de bord → fin).

### 4.2 Premiere ouverture bibliotheque (lecteur)

Quand le lecteur ouvre la bibliotheque JayManga pour la premiere fois :

| Etape | Miou (audio + bulle) |
|-------|----------------------|
| Bibliotheque vide | « Ta bibliotheque est vide pour l'instant. Visite le Portail d'un COG pour decouvrir des manga et les ajouter a tes favoris. » |
| Etat vide : sample content | Afficher 2-3 suggestions de COGs JayManga connus (via MWS) comme points de depart. |
| Premiere oeuvre ajoutee | « Super ! Ton premier manga est dans ta bibliotheque. » |

### 4.3 Implementation Dioxus

L'onboarding utilise les patterns standard de Central :

| Pattern | Usage |
|---------|-------|
| `use_effect` | Declenchement de l'audio Miou et des etapes conditionnelles. |
| `use_signal` | Etat local de l'onboarding (etape courante, skip). |
| Modal overlay | Guidage pas-a-pas avec fond assombri et spotlight sur l'element cible. |
| Toast notification | Notifications de badges et XP (coin inferieur droit, disparition automatique). |

---

## 5. Theme et style

### 5.1 Palette JayManga

JayManga utilise le systeme de theme Central (`ThemePalette`) avec des couleurs specifiques au service :

| Token | Usage | Valeur (theme Gaming) |
|-------|-------|----------------------|
| `jaymanga_accent` | Couleur d'accent JayManga (boutons, liens, progression) | `#FF6B35` (orange manga) |
| `jaymanga_bg_reader` | Fond de la liseuse | `#1A1A2E` (sombre) / `#F5F0E8` (clair/papier) |
| `jaymanga_xp_bar` | Barre de progression XP | `#FFD700` (or) |
| `jaymanga_streak_fire` | Couleur de la flamme streak | `#FF4500` (rouge-orange) |
| `jaymanga_badge_border` | Bordure des badges | `#C0C0C0` (argent) / `#FFD700` (or) selon rarete |

### 5.2 Composants specifiques

| Composant | Fichier | Description |
|-----------|---------|-------------|
| `JayMangaServiceCard` | `services/jaymanga/card.rs` | Carte d'entree dans le Salon. |
| `JayMangaDashboard` | `services/jaymanga/dashboard.rs` | Hub principal. |
| `JayMangaReader` | `services/jaymanga/reader.rs` | Liseuse native. |
| `JayMangaProgressBar` | `services/jaymanga/components/progress.rs` | Barre XP et progression lecture. |
| `JayMangaBadgeGrid` | `services/jaymanga/components/badges.rs` | Grille de badges. |
| `JayMangaStreakWidget` | `services/jaymanga/components/streak.rs` | Widget streak avec flamme animee. |
| `JayMangaWorkCard` | `services/jaymanga/components/work_card.rs` | Carte d'oeuvre (cover, titre, progression, statut). |

---

## 6. Gestion d'etat

### 6.1 Extension de AppState

```rust
pub struct JayMangaState {
    pub active_view: JayMangaView,
    pub reader_state: Option<ReaderState>,
    pub onboarding_step: Option<OnboardingStep>,
    pub progression: Option<ReaderProgression>,
    pub library_tab: LibraryTab,
}

pub enum JayMangaView {
    Dashboard,
    CatalogAdmin,
    WorkEditor(UUID),
    SalesAdmin,
    Library,
    Reader(UUID),
    Profile,
    Settings,
    Aggregator,
}

pub enum LibraryTab {
    Favorites,
    Purchases,
    Downloads,
    InProgress,
    Completed,
}
```

### 6.2 Hooks JayManga

```rust
pub fn use_jaymanga_state() -> Signal<JayMangaState> { ... }
pub fn use_jaymanga_progression() -> Signal<ReaderProgression> { ... }
pub fn use_jaymanga_reader() -> Signal<Option<ReaderState>> { ... }
```

---

## 7. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Mecanismes transversaux (onboarding, XP, streaks, badges). |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Specification de la liseuse (modes, navigation, formats). |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Donnees de la bibliotheque lecteur. |

---

**Document** : JayManga — UI Central et Stable
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification UI/UX — interface native Dioxus.
