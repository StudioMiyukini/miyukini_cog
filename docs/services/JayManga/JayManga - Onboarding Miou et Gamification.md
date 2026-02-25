# JayManga — Onboarding Miou et Gamification

## Contexte

Ce document definit les mecanismes d'**onboarding assiste par Miou** et de **gamification par progression lecteur** pour JayManga. Ces mecanismes sont **transversaux aux trois interfaces** (Central/Stable, Mobile/Terminal, Web Portal) et s'adaptent au contexte de chaque plateforme.

L'objectif est double : **guider le nouvel utilisateur** vers la valeur du service des ses premieres interactions (onboarding) et **maintenir son engagement** sur la duree via un systeme de progression non intrusif (gamification).

Les leaders du marche (WEBTOON, Tachiyomi, Crunchyroll Manga, Manga Plus) proposent des mecanismes d'engagement limites (Daily Pass, challenges coins). JayManga se demarque en integrant un **assistant narratif** (Miou) et un systeme de **progression intrinsèque** (pas de monnaie virtuelle, pas de mur payant artificiel).

> **Principe fondamental :** La gamification de JayManga recompense la lecture et la curiosite, jamais la depense. Le lecteur progresse en lisant, pas en achetant.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Onboarding assiste par Miou

### 1.1 Qui est Miou dans JayManga ?

Miou est le **compagnon audio et contextuel** de l'ecosysteme Miyukini. Dans JayManga, Miou intervient comme guide lors des premieres interactions du lecteur et du vendeur, puis s'efface progressivement pour laisser l'utilisateur autonome.

| Aspect | Description |
|--------|-------------|
| **Role** | Guide d'onboarding, assistant contextuel, narrateur de progression. |
| **Ton** | Amical, concis, jamais condescendant. Evoque un compagnon manga (mascotte de bibliotheque). |
| **Modalites** | Voix audio (synthese vocale ou fichiers pre-enregistres), bulles de texte illustrees, animations de mascotte. |
| **Discretion** | Miou se retire apres les 3-5 premieres sessions. Il reste accessible via un bouton d'aide mais ne se manifeste plus spontanement sauf pour les evenements de progression. |

### 1.2 Onboarding lecteur (Portail / Mobile)

L'onboarding lecteur se deroule en **4 etapes progressives** lors de la premiere visite :

#### Etape 1 — Accueil et decouverte (0-30 secondes)

```
┌──────────────────────────────────────────────────────┐
│                                                        │
│          [Mascotte Miou animee]                        │
│                                                        │
│   Miou : "Bienvenue dans [Nom de la librairie] !      │
│    Ici tu peux lire des manga librement.               │
│    Laisse-moi te montrer."                             │
│                                                        │
│          [ Commencer la visite ]   [ Explorer seul ]   │
│                                                        │
└──────────────────────────────────────────────────────┘
```

| Action Miou | Declencheur |
|-------------|-------------|
| Message d'accueil personnalise avec le nom de la librairie | Premiere visite sur un Portail JayManga. |
| Presentation du concept (3 secondes audio ou bulle) | Si le lecteur choisit « Commencer la visite ». |
| Aucune interruption | Si le lecteur choisit « Explorer seul ». |

#### Etape 2 — Premiere interaction guidee (30-90 secondes)

Miou guide le lecteur vers sa premiere oeuvre :

| Guidage | Description |
|---------|-------------|
| **Highlight du catalogue** | Mise en surbrillance de la section « Gratuit » ou « Tendances ». Miou : « Commence par la — ces manga sont gratuits. » |
| **Ouverture d'une fiche** | Miou pointe vers une oeuvre populaire. « Clique sur cette couverture pour decouvrir l'histoire. » |
| **Tooltip contextuel** | Survol des elements : prix, pages de demo, bouton « Lire ». |

#### Etape 3 — Premiere lecture (90 secondes - 3 minutes)

Miou intervient **dans la liseuse** lors de la premiere lecture :

| Intervention | Description |
|--------------|-------------|
| **Geste de navigation** | Animation montrant le geste (clic/swipe) pour passer a la page suivante. Disparait apres la 2e page. |
| **Mode de lecture** | Si le format est webtoon : « Defile vers le bas pour lire. » Si manga : « Clique a gauche pour la page suivante. » |
| **Fin de demo** | A la fin des pages de demonstration : « Tu as aime ? Tu peux acheter la suite ou mettre en favoris pour plus tard. » |

#### Etape 4 — Premiere action sociale (3-5 minutes)

| Action | Miou |
|--------|------|
| **Favoris** | « Ajoute cette oeuvre a tes favoris pour la retrouver facilement. » (tooltip sur le bouton favori). |
| **Exploration** | « Il y a [N] autres manga ici. Explore par genre ou par format. » |
| **Retour** | Miou ne se manifeste plus apres cette etape. Un badge « Premiere visite » est attribue silencieusement. |

### 1.3 Onboarding vendeur (Central)

L'onboarding vendeur se deroule lors de l'**activation de JayManga** dans Central :

| Etape | Miou | Action |
|-------|------|--------|
| **1. Activation du service** | « Tu viens d'activer JayManga. Configurons ta librairie. » | Ouverture du formulaire de configuration (shop_name, description, devise). |
| **2. Premiere oeuvre** | « Importe ta premiere oeuvre — choisis des images, donne-lui un titre. » | Guidage pas a pas de l'import (fichiers → metadonnees → format → prix → demo pages). |
| **3. Publication** | « Ta premiere oeuvre est prete. Publie-la pour que les lecteurs puissent la decouvrir. » | Bouton « Publier » avec confirmation. |
| **4. Tableau de bord** | « Ton manga est en ligne. Reviens ici pour voir les statistiques. » | Redirection vers le tableau de bord des ventes. |
| **5. Fin** | « C'est tout. Tu es pret. Si tu as besoin d'aide, je suis toujours la. » | Badge « Premiere publication ». |

### 1.4 Principes d'onboarding

| Principe | Description |
|----------|-------------|
| **Progressive disclosure** | Chaque etape ne montre que ce qui est necessaire. Pas de surcharge d'information. |
| **Skip always available** | Le lecteur peut toujours sauter l'onboarding. Miou ne bloque jamais la navigation. |
| **Context-aware** | Miou adapte son message au contexte (format de l'oeuvre, type de lecteur, premiere visite ou retour). |
| **Once-and-done** | L'onboarding ne se repete pas. Il est marque comme complete dans le stockage local (cookie sur le Portail, donnee locale dans Central/Mobile). |
| **Audio optionnel** | L'audio Miou est desactivable. Les bulles de texte restent toujours disponibles. |

---

## 2. Gamification — Systeme de progression lecteur

### 2.1 Vision

Le systeme de progression de JayManga est inspire de Duolingo (streaks, XP, niveaux) et adapte a l'univers manga. Il recompense la **lecture** (temps passe, pages lues, oeuvres terminees) et la **curiosite** (decouverte de nouveaux genres, vendeurs, formats).

| Axe | Ce qu'il recompense | Ce qu'il ne recompense PAS |
|-----|---------------------|---------------------------|
| **Lecture** | Pages lues, chapitres termines, oeuvres terminees | Achats, depenses |
| **Regularite** | Streaks de lecture quotidienne, retour apres absence | Connexions sans lecture |
| **Exploration** | Nouveaux genres, nouveaux vendeurs, nouveaux formats | Repetition du meme contenu |

### 2.2 XP de lecture

Chaque action de lecture genere des **points d'experience (XP)** :

| Action | XP | Condition |
|--------|-----|-----------|
| Lire une page | 1 XP | Temps minimum sur la page (3 secondes manga, 2 secondes webtoon). Empeche le farming. |
| Terminer un chapitre | 10 XP (bonus) | En plus des XP par page. |
| Terminer une oeuvre complete | 50 XP (bonus) | Toutes les pages lues. |
| Premiere lecture du jour | 5 XP (bonus quotidien) | Premier acces a la liseuse dans la journee. |
| Lire une oeuvre d'un nouveau genre | 15 XP | Genre jamais lu auparavant. |
| Lire sur un nouveau COG vendeur | 10 XP | Premier acces au catalogue d'un vendeur jamais visite. |
| Lire un nouveau format | 20 XP | Premier webtoon, premier manga, premier 16:9, etc. |

### 2.3 Niveaux de lecteur

Les XP accumules definissent le **niveau du lecteur** :

| Niveau | Nom | XP requis | Illustration |
|--------|-----|-----------|--------------|
| 1 | Curieux | 0 | Un personnage qui ouvre un premier livre. |
| 2 | Lecteur | 100 | Le personnage s'installe confortablement avec un manga. |
| 3 | Passione | 500 | Etagere avec quelques manga. |
| 4 | Devore | 1 500 | Pile de manga grandissante. |
| 5 | Otaku | 5 000 | Bibliotheque remplie. |
| 6 | Connaisseur | 15 000 | Critique manga, lunettes, stylo. |
| 7 | Sage | 40 000 | Maitre assis devant un mur de livres. |
| 8 | Legendaire | 100 000 | Personnage entour d'un halo manga, bibliotheque cosmique. |

Les niveaux debloquent des **recompenses cosmetiques** (voir 2.5).

### 2.4 Streaks de lecture

Le systeme de streaks encourage la lecture reguliere :

| Mecanisme | Description |
|-----------|-------------|
| **Streak quotidien** | Le compteur s'incremente chaque jour ou le lecteur lit au moins 5 pages. |
| **Affichage** | Flamme avec compteur de jours dans la bibliotheque et le profil. |
| **Protection** | Un « bouclier de streak » (automatique, 1 par semaine) protege contre la perte en cas de jour manque. |
| **Bonus streak** | Tous les 7 jours de streak continu : bonus de 25 XP. Tous les 30 jours : 100 XP. |
| **Reset gracieux** | Un streak perdu n'efface pas les XP acquis. Le compteur repart a 0 mais le niveau reste. |

```
┌─────────────────────────────────────────┐
│  🔥 14 jours de lecture                  │
│  ■ ■ ■ ■ ■ ■ ■  ■ ■ ■ ■ ■ ■ ■        │
│  L M M J V S D  L M M J V S D          │
│                                          │
│  Prochain bonus : dans 7 jours (25 XP)  │
└─────────────────────────────────────────┘
```

### 2.5 Badges et accomplissements

Les badges celebrent des **jalons specifiques** et sont affiches dans le profil du lecteur :

#### Badges de lecture

| Badge | Condition | Icone |
|-------|-----------|-------|
| Premiere Page | Lire sa premiere page de manga. | Livre ouvert. |
| Premier Chapitre | Terminer un premier chapitre complet. | Signet. |
| Premiere Oeuvre | Terminer une oeuvre complete. | Etoile doree. |
| Marathonien | Lire 100 pages en une seule session. | Personnage en sprint. |
| Bibliophile | Terminer 10 oeuvres differentes. | Pile de 10 livres. |
| Centurion | Lire 1 000 pages au total. | Chiffre romain C. |
| Mille-Pages | Lire 10 000 pages au total. | Chiffre romain M. |

#### Badges de regularite

| Badge | Condition | Icone |
|-------|-----------|-------|
| Fidele | 7 jours de streak consecutifs. | Flamme bronze. |
| Assidu | 30 jours de streak consecutifs. | Flamme argent. |
| Inebranble | 100 jours de streak consecutifs. | Flamme doree. |
| Eternel | 365 jours de streak consecutifs. | Flamme arc-en-ciel. |

#### Badges d'exploration

| Badge | Condition | Icone |
|-------|-----------|-------|
| Explorateur | Lire des oeuvres de 3 genres differents. | Boussole. |
| Polyglotte | Lire des oeuvres en 3 langues differentes. | Globe. |
| Nomade | Lire sur 5 COGs vendeurs differents. | Carte au tresor. |
| Omnivore | Lire dans les 5 formats (manga, webtoon, landscape, comics, free). | Pentagon colore. |
| Globe-Trotteur | Lire sur 20 COGs vendeurs differents. | Avion manga. |

### 2.6 Profil lecteur et vitrine

Le profil lecteur affiche la progression de maniere visuelle :

```
┌──────────────────────────────────────────────────────┐
│  [Avatar]    Nom du lecteur                           │
│              Niveau 5 — Otaku                         │
│              ████████████░░░ 7 200 / 15 000 XP       │
│              🔥 14 jours de streak                     │
│                                                        │
│  Badges recents :                                      │
│  [Centurion] [Fidele] [Explorateur] [Omnivore]        │
│                                                        │
│  Statistiques :                                        │
│  📖 2 340 pages lues                                   │
│  📚 18 oeuvres terminees                               │
│  🔥 Record streak : 42 jours                           │
│  🎯 6 genres explores                                  │
│  🌐 8 COGs visites                                     │
│                                                        │
│  [ Voir tous les badges ]  [ Historique de lecture ]   │
└──────────────────────────────────────────────────────┘
```

### 2.7 Notifications de progression (Miou)

Miou intervient lors des **evenements de progression** significatifs :

| Evenement | Intervention Miou |
|-----------|-------------------|
| Montee de niveau | Animation Miou + message : « Niveau 5 — Otaku ! Ta bibliotheque est impressionnante. » |
| Badge debloque | Notification discrete : « Nouveau badge : Centurion — 1 000 pages lues. » |
| Streak milestone (7, 30, 100) | Miou anime : « 30 jours de lecture d'affilee. Tu es inebranble. » |
| Retour apres absence | Miou bienveillant : « Content de te revoir. Tu avais un streak de 14 jours — tu veux repartir ? » |

Ces interventions sont **courtes** (3-5 secondes), **non bloquantes** (toast ou notification flottante), et **desactivables** dans les preferences.

---

## 3. Stockage des donnees de progression

### 3.1 Ou sont stockees les donnees ?

| Donnee | Stockage | Justification |
|--------|----------|---------------|
| XP total, niveau | COG du lecteur (KindMother) | LOI-3 — donnees personnelles souveraines. |
| Streak compteur | COG du lecteur (KindMother) | Donnee locale, pas de centralisation. |
| Badges obtenus | COG du lecteur (KindMother) | Collecte personnelle. |
| Historique de lecture (pages, dates) | COG du lecteur (KindMother) | Vie privee du lecteur. |
| Statistiques agregees (genres, COGs visites) | COG du lecteur (KindMother) | Calcul local uniquement. |
| Flag onboarding complete | Cookie (Portail) / KindMother (Central/Mobile) | Etat local. |

### 3.2 Modele de donnees supplementaire

#### ReaderProgression (sur le COG du lecteur)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique. |
| total_xp | INTEGER | XP accumules. |
| current_level | INTEGER | Niveau actuel (1-8). |
| current_streak | INTEGER | Nombre de jours de streak en cours. |
| longest_streak | INTEGER | Record de streak. |
| streak_shield_available | BOOLEAN | Si le bouclier de streak est disponible cette semaine. |
| last_read_date | TEXT (ISO 8601) | Date de la derniere lecture (pour le calcul de streak). |
| total_pages_read | INTEGER | Pages lues au total. |
| total_works_completed | INTEGER | Oeuvres terminees. |
| total_chapters_completed | INTEGER | Chapitres termines. |
| genres_explored | JSON | Liste des genres lus au moins une fois. |
| formats_explored | JSON | Liste des formats lus au moins une fois. |
| cogs_visited | JSON | Liste des COGs vendeurs visites. |
| languages_read | JSON | Liste des langues lues. |
| onboarding_completed | BOOLEAN | Onboarding termine. |
| created_at | TEXT | ISO 8601. |
| updated_at | TEXT | ISO 8601. |

#### ReaderBadge (sur le COG du lecteur)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique. |
| badge_id | TEXT | Identifiant du badge (ex. `first_page`, `centurion`, `streak_30`). |
| badge_name | TEXT | Nom affiche. |
| badge_category | TEXT | `reading` / `regularity` / `exploration`. |
| earned_at | TEXT | ISO 8601 — date d'obtention. |

### 3.3 Lecteur visiteur (sans COG)

Les lecteurs visiteurs (sans COG propre) voient un systeme de **progression ephemere** :

| Aspect | Comportement |
|--------|-------------|
| **XP et streaks** | Stockes dans le localStorage du navigateur (Portail) ou en memoire (session). |
| **Persistance** | Les donnees sont perdues si le cache est vide. |
| **Incitation** | Apres le niveau 2 (100 XP), Miou suggere : « Tu lis beaucoup ici. Avec un COG, ta progression serait sauvegardee. » — une seule fois, jamais intrusif. |
| **Pas de mur** | Le lecteur visiteur peut continuer a accumuler des XP sans creer de COG. La suggestion reste optionnelle. |

---

## 4. Integration vendeur

### 4.1 Statistiques de progression sur le tableau de bord

Le vendeur voit des **statistiques anonymisees** sur l'engagement des lecteurs de son catalogue :

| Statistique | Description |
|-------------|-------------|
| Pages lues (total / jour / semaine) | Volume de lecture sur le catalogue du vendeur. |
| Oeuvres terminees | Nombre d'oeuvres lues integralement. |
| Taux de conversion demo → achat | Pourcentage de lecteurs ayant achete apres la demo. |
| Genres les plus lus | Repartition des genres dans les lectures. |
| Retention | Pourcentage de lecteurs revenant dans les 7 jours. |

Ces statistiques sont **calculees localement** sur le COG vendeur a partir des logs de lecture. Aucune donnee personnelle du lecteur n'est partagee.

### 4.2 Le vendeur ne gamifie pas

Le systeme de gamification est **cote lecteur uniquement**. Le vendeur n'a pas d'XP, de niveaux ou de badges. Son engagement est mesure par ses ventes et ses statistiques, pas par un systeme de points.

---

## 5. Adaptation par interface

Ce document definit les mecanismes transversaux. L'adaptation visuelle et ergonomique est detaillee dans les documents specifiques :

| Interface | Document | Adaptation principale |
|-----------|----------|-----------------------|
| Central / Stable | [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Profil et progression dans la barre laterale. Miou via le systeme audio natif Dioxus. Badges dans la section « Ma Bibliotheque ». |
| Mobile / Terminal | [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Streak en header. Notifications push de progression. Miou via notifications systeme. |
| Web Portal | [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Onboarding Miou en overlay. Progression en pied de liseuse. Badges optionnels (lecteur visiteur). |

---

## 6. Inspirations marche et differenciation

| Plateforme | Ce qu'on reprend | Ce qu'on evite |
|------------|-----------------|----------------|
| **WEBTOON** | Daily engagement, challenges thematiques, decouverte par genres. | Coins/monnaie virtuelle, Daily Pass payant, murs de paiement artificiels. |
| **Duolingo** | Streaks, XP, niveaux, bouclier de streak, retour apres absence bienveillant. | Publicites intrusives, pression sociale excessive, comparaisons entre utilisateurs. |
| **Tachiyomi** | Personnalisation de l'interface, suivi de progression, integration tracking (MyAnimeList). | Absence totale de gamification (trop passif pour l'engagement). |
| **Crunchyroll Manga** | Acces fluide au catalogue, integration compte unifie. | Dependance a l'abonnement, catalogue ferme. |
| **Manga Plus** | Lecture gratuite, catalogue officiel, qualite. | Modele publicitaire, pas de progression lecteur. |

**Differenciation JayManga :**
- Miou comme guide narratif (aucun concurrent n'a d'assistant embarque).
- Progression qui recompense la lecture, pas la depense.
- Donnees de progression stockees localement (souverainete LOI-3).
- Pas de monnaie virtuelle, pas de paywall artificiel.
- Systeme ouvert : chaque COG vendeur est une librairie independante, pas un catalogue centralise.

---

## 7. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Bibliotheque lecteur et progression. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Liseuse (ou les XP sont accumules). |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Adaptation UI Central/Stable. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Adaptation UI Mobile. |
| [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Adaptation UI Web Portal. |

---

**Document** : JayManga — Onboarding Miou et Gamification
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle transversale.
