# JayManga — Document fondateur

## Contexte

**JayManga** est le **service Miyukini dedie a la lecture et a la vente de manga en ligne** au sein de l'ecosysteme COG. Il permet a tout proprietaire de COG de **publier sa collection de manga**, de la **proposer en lecture** (gratuite ou payante) sur la surface web de son COG, et de **gerer les ventes** depuis son environnement Host. Les lecteurs externes accedent aux oeuvres via le **Miyukini Web Portal** (Portail) du COG proposant le service.

JayManga s'appuie sur le **Miyukini Webway System (MWS)** pour la **detection de presence** des COGs vendeurs : un lecteur peut savoir en temps reel si le COG hebergeant une oeuvre est en ligne. Les oeuvres achetees peuvent etre **telechargees pour une lecture hors-ligne** si le vendeur l'autorise, respectant ainsi la souverainete de chaque COG sur son contenu.

Ce document est le **document fondateur** du service : il en fixe la raison d'etre, la portee, les principes directeurs, les fonctionnalites structurantes, le modele de donnees et l'integration avec les autres services et le MWS. Il s'adresse aux equipes produit, technique et aux parties prenantes.

## Portee / Scope

- **Perimetre** : Definition du service JayManga — positionnement, fonctionnalites (publication, lecture, demonstration, achat, favoris, telechargement hors-ligne, presence vendeur), module de paiement integre, modele de donnees, integration MWS, niveaux de securite.
- **Hors perimetre** : Specifications techniques detaillees (implementation crate, endpoints API), creation de contenu manga (JayManga ne fournit pas d'editeur de manga), hebergement de fichiers (delegue a KindMother), comptabilite detaillee (JayKonta si necessaire a terme).
- **References** : Glossaire Miyukini, [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md), [Miyukini Conceptual References - Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md).

### Decisions structurantes (mini log)

| Id | Decision | Justification |
|----|----------|---------------|
| **DS-01** | Service de Type 2 (surface web externe). | La lecture de manga est destinee aux visiteurs externes via le Portail ; la gestion de la collection et des ventes est reservee a l'admin dans Central. |
| **DS-02** | Module de paiement integre (pas d'adaptateur JayKonta en V1). | JayManga gere ses propres transactions ; les oeuvres numeriques ont un cycle de vente plus simple que le commerce physique de JayShop. |
| **DS-03** | Chaque COG est souverain sur son catalogue et ses conditions de vente. | LOI-3 (etat local souverain) : le vendeur decide des prix, de la gratuite, et de l'autorisation de telechargement hors-ligne. |
| **DS-04** | Pages de demonstration gratuites pour chaque oeuvre. | Le vendeur definit un nombre de pages accessibles sans achat, permettant au lecteur de decouvrir l'oeuvre avant d'acheter. |
| **DS-05** | Detection de presence du vendeur via le MWS. | Le MWS fournit deja la presence des COGs ; JayManga expose cette information via une API pour indiquer si le COG hebergeant une oeuvre est en ligne. |
| **DS-06** | Telechargement hors-ligne conditionnel. | Le vendeur controle si les oeuvres achetees sont telechargeables. Cela respecte la souverainete du COG sur son contenu (LOI-3) et les droits d'auteur. |
| **DS-07** | Les fichiers manga sont stockes localement sur le COG vendeur (KindMother). | Pas de stockage centralise ; chaque COG heberge ses propres fichiers. La lecture distante passe par la surface web du COG. |
| **DS-08** | Systeme de favoris cross-COG pour les lecteurs. | Les favoris sont stockes localement sur le COG du lecteur ; ils referencent des oeuvres par identifiant COG + identifiant oeuvre. |
| **DS-09** | Aucune limite de stockage imposee par le service. | Le stockage est limite uniquement par le hardware du COG (LOI-5). JayManga ne fixe aucun quota artificiel. |
| **DS-10** | Liberte totale de format de lecture (Manga, Webtoon, 16:9, libre). | JayManga ne se limite pas au format manga classique. Le vendeur choisit le format de son oeuvre ; la liseuse s'adapte automatiquement. |
| **DS-11** | Outil integre de redimensionnement et compression des pages. | A l'import, les pages sont automatiquement optimisees pour la lecture web (variantes de resolution). Les fichiers originaux sont preserves. |
| **DS-12** | Portail Agrege : interface inter-COG unifiee emulant un catalogue en ligne (type Mangadraft/Manga.io). | Tout COG peut heberger un Portail Agrege qui collecte les catalogues des COGs JayManga via le MWS, les presente dans une interface unifiee, et grise les COGs hors-ligne. Les fichiers ne quittent jamais leur COG d'origine. |
| **DS-13** | Trois interfaces UI distinctes : Central/Stable (Dioxus natif), Mobile/Terminal (Dioxus natif tactile), Web Portal (HTML/CSS/JS). | Chaque plateforme a des contraintes UX specifiques (clavier/souris vs tactile vs navigateur). Le pattern MWS "Central = COG, Portail = Web" impose la separation. |
| **DS-14** | Onboarding via Miou et engagement via gamification (progression lecteur). | L'onboarding guide par Miou reduit le time-to-value. La gamification (XP, niveaux, streaks, badges) fidelise les lecteurs. Inspire des leaders du marche (WEBTOON, Duolingo). Les donnees de progression restent sur le COG du lecteur (LOI-3). |
| **DS-15** | Guide d'implementation technique couvrant la structure crate, les types de domaine, la persistance (feature-gated), les APIs REST, l'integration MWS, les composants UI (Dioxus + Web), la gamification et la securite. | Fournit un cadre technique unifie pour l'ensemble des equipes de developpement, aligne sur les patterns standard des services Jay (miyukini-services, miyukini-rust-patterns). |
| **DS-16** | Plan d'implementation en 8 phases sequentielles (Fondation → Catalogue → Optimisation → Lecteur → Paiement → Favoris/MWS → Gamification → Portail Agrege) avec dependances, criteres de validation et jalons. | Permet un developpement incremental avec livrables testables a chaque phase. Chaque phase produit un sous-ensemble fonctionnel autonome. |
| **Dependance systeme** | MWS (presence, decouverte des COGs proposant JayManga). | — |
| **Dependance optionnelle** | JayKonta (comptabilite avancee, a terme). JayXpose (si le vendeur souhaite lier ses oeuvres a un catalogue vitrine). | — |

---

## 1. Raison d'etre

### 1.1 Proposition de valeur

**JayManga** permet a tout proprietaire de COG de :

- **Publier sa collection de manga** : importer des oeuvres (images, chapitres, metadonnees), les organiser en series et volumes.
- **Proposer la lecture en ligne** : les visiteurs lisent directement depuis la surface web du COG, avec une liseuse integree au Portail.
- **Offrir des pages de demonstration** : chaque oeuvre expose un nombre configurable de pages gratuites pour permettre la decouverte.
- **Vendre des oeuvres** : le lecteur peut acheter l'acces complet a une oeuvre via le module de paiement integre.
- **Autoriser le telechargement hors-ligne** : le vendeur peut permettre aux acheteurs de telecharger l'oeuvre sur leur COG pour une lecture sans connexion.
- **Proposer la lecture gratuite** : le vendeur peut choisir de rendre certaines ou toutes ses oeuvres entierement gratuites.

**JayManga** permet aux **lecteurs** de :

- **Decouvrir des manga** : parcourir les catalogues des COGs proposant JayManga via le MWS.
- **Lire en ligne** : liseuse web integree avec navigation par pages, zoom, plein ecran, marque-page.
- **Mettre en favoris** : sauvegarder des oeuvres et des series dans sa bibliotheque personnelle, avec suivi de la progression de lecture.
- **Acheter des oeuvres** : paiement securise via le module integre du COG vendeur.
- **Lire hors-ligne** : telecharger les oeuvres achetees sur son propre COG (si le vendeur l'autorise).
- **Verifier la disponibilite** : savoir si le COG hebergeant une oeuvre est en ligne grace a l'API de presence MWS.

### 1.2 Avantages distinctifs

| Avantage | Description |
|----------|-------------|
| **Aucune limite de stockage** | JayManga n'impose aucun quota de stockage. La seule limite est la capacite materielle du COG du vendeur (LOI-5 : cout proportionnel au hardware). Un vendeur disposant de 2 To peut publier 2 To de manga. Pas d'abonnement, pas de paliers, pas de restrictions artificielles. |
| **Liberte totale de format** | JayManga supporte nativement tous les formats de lecture numerique : **Manga** (pages classiques, lecture droite-a-gauche), **Webtoon** (bande verticale continue, defilement infini), **16:9** (format paysage/cinema, ideal pour les double-pages et les illustrations panoramiques), **Comics** (lecture gauche-a-droite), et tout **format libre** (ratio d'aspect personnalise). Le vendeur declare le format de chaque oeuvre ; la liseuse s'adapte automatiquement. |
| **Outil d'optimisation integre** | A l'import, JayManga genere automatiquement des variantes optimisees de chaque page (resolutions adaptees aux ecrans mobiles, tablettes et desktop, compression WebP/AVIF). Les fichiers originaux sont preserves ; les variantes accelerent l'affichage pour les lecteurs distants avec des connexions variables. Le vendeur peut aussi declencher manuellement l'optimisation avec des parametres personnalises. |
| **Souverainete totale du vendeur** | Aucune plateforme intermediaire, aucune commission tierce, aucune censure centralisee. Le vendeur decide de tout : prix, gratuite, formats, conditions de telechargement, politique de remboursement. |
| **Decouverte decentralisee** | Les lecteurs decouvrent les collections via le MWS sans passer par un agregateur centralise. Chaque COG est une librairie independante, et le MWS permet de les trouver sans les centraliser. |

### 1.3 Positionnement

| Espace | Description |
|--------|-------------|
| **Central (Admin/Vendeur)** | Publication, gestion du catalogue manga, configuration des prix et des pages de demonstration, gestion des ventes, autorisations de telechargement. |
| **Portail (Lecteur)** | Catalogue public, liseuse en ligne, demonstration, achat, favoris, bibliotheque personnelle, telechargement hors-ligne. |

### 1.3 Phrase fondatrice

> **JayManga est la surface de lecture et de vente de manga de l'ecosysteme Miyukini. Chaque COG est une librairie souveraine — le lecteur decouvre, lit, achete et emporte ses oeuvres, gouverne par le COG.**

---

## 2. Fonctionnalites structurantes

Les sections ci-dessous donnent une vue synthetique de chaque bloc fonctionnel. Le detail complet est documente dans des documents dedies :

| Document de detail | Couverture |
|--------------------|------------|
| [JayManga - Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md) | Import, formats, outil d'optimisation/compression, metadonnees, organisation, statuts. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Liseuse web et locale, modes de lecture (Manga, Webtoon, 16:9, Comics), navigation, demonstration. |
| [JayManga - Achat et Paiement](./JayManga%20-%20Achat%20et%20Paiement.md) | Module de paiement integre, panier, licences, remboursements, promotions, administration des ventes. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Favoris cross-COG, bibliotheque lecteur, telechargement hors-ligne, presence MWS, cache. |
| [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) | Interface inter-COG unifiee, collecte de catalogues, navigation agregee, COGs hors-ligne grises, API de federation. |
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Onboarding lecteur/vendeur via Miou, progression (XP, niveaux, streaks, badges). Transversal aux 3 interfaces. |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Interface Dioxus native pour COG STABLE : navigation, ecrans, liseuse native, theme, composants. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Interface Dioxus native pour COG TERMINAL (mobile) : navigation tactile, liseuse gestuelle, sync, notifications. |
| [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Interface web pour Portail vendeur et Portail Agrege : responsive, liseuse web, SEO, personnalisation. |
| [JayManga - Guide Implementation](./JayManga%20-%20Guide%20Implementation.md) | Guide technique : structure crate, types, persistance, APIs, MWS, UI, gamification, securite, tests. |
| [JayManga - Plan Implementation](./JayManga%20-%20Plan%20Implementation.md) | Plan en 8 phases (Fondation → Portail Agrege) : dependances, modules, criteres, jalons. |

### 2.1 Publication et gestion du catalogue (Admin/Vendeur — Central)

| Fonctionnalite | Description |
|----------------|-------------|
| Import d'oeuvres | L'admin importe des fichiers images (pages) organisees par chapitre et volume. Formats acceptes : JPEG, PNG, WebP, AVIF. Aucune limite de taille ou de nombre de pages. |
| Format de lecture | L'admin declare le format de l'oeuvre : `manga` (pages classiques), `webtoon` (bande verticale continue), `landscape` (16:9 paysage), `comics` (pages classiques LTR), `free` (ratio libre). La liseuse s'adapte automatiquement. |
| Optimisation automatique | A l'import, l'outil integre genere des variantes optimisees (resolutions multiples, compression WebP/AVIF) pour accelerer l'affichage. Les originaux sont preserves. L'admin peut re-optimiser manuellement avec des parametres personnalises. |
| Metadonnees | Titre, auteur(s), genre(s), synopsis, couverture, langue, date de publication, tags. |
| Organisation en series | Regroupement d'oeuvres en series avec ordre des volumes et des chapitres. |
| Pages de demonstration | L'admin definit le nombre de pages accessibles gratuitement par oeuvre (defaut configurable). |
| Politique de prix | Prix par oeuvre (volume ou chapitre) en centimes, devise configurable. L'admin peut rendre une oeuvre entierement gratuite. |
| Politique de telechargement | L'admin active ou desactive l'autorisation de telechargement hors-ligne par oeuvre ou globalement. |
| Statut de publication | `draft` (brouillon), `published` (publie), `unlisted` (non liste mais accessible par lien), `archived` (archive). |
| Gestion des fichiers | Les fichiers manga sont stockes localement via KindMother. L'admin peut reorganiser, remplacer ou supprimer des pages. |

### 2.2 Lecture en ligne (Lecteur — Portail)

| Fonctionnalite | Description |
|----------------|-------------|
| Catalogue public | Page liste des oeuvres disponibles avec filtres (genre, auteur, prix, gratuit/payant), recherche textuelle, tri (popularite, date, prix). |
| Fiche oeuvre | Couverture, titre, auteur(s), synopsis, genres, nombre de chapitres/volumes, prix, nombre de pages de demonstration, note moyenne (Phase 2). |
| Liseuse integree | Navigation page par page ou double-page, zoom, mode plein ecran, sens de lecture configurable (gauche-a-droite ou droite-a-gauche), mode sombre. |
| Pages de demonstration | Le lecteur peut lire les N premieres pages de chaque oeuvre sans achat ni authentification. Un ecran d'incitation a l'achat s'affiche a la fin de la demonstration. |
| Marque-page | Sauvegarde automatique de la page de lecture en cours pour reprendre plus tard. |
| Progression de lecture | Indicateur de progression (chapitre X / Y, page X / Y) sauvegarde localement et dans les favoris. |

### 2.3 Systeme de favoris et bibliotheque lecteur

| Fonctionnalite | Description |
|----------------|-------------|
| Favoris cross-COG | Le lecteur peut mettre en favoris des oeuvres provenant de differents COGs. Les favoris sont stockes localement sur le COG du lecteur (LOI-3). |
| Reference d'oeuvre | Un favori reference : `cog_id` (COG vendeur) + `work_id` (identifiant oeuvre) + metadonnees en cache (titre, couverture, progression). |
| Statut de disponibilite | Chaque favori affiche si le COG vendeur est actuellement en ligne (via API MWS). |
| Bibliotheque personnelle | Vue unifiee de toutes les oeuvres en favoris, avec indication du statut (demo, achete, hors-ligne disponible). |
| Synchronisation cache | Les metadonnees des oeuvres en favoris sont mises en cache localement pour un affichage rapide meme si le COG vendeur est hors-ligne. |

### 2.4 Achat d'oeuvres (Module de paiement integre)

| Fonctionnalite | Description |
|----------------|-------------|
| Panier | Le lecteur ajoute des oeuvres (volumes, chapitres ou series completes) a un panier. |
| Checkout | Recap du panier, montant total, selection du mode de paiement. |
| Modes de paiement | Configurables par le vendeur : virement, carte bancaire (via passerelle externe configurable), autre. Le vendeur reste souverain sur les moyens acceptes. |
| Confirmation d'achat | Apres paiement valide, le lecteur obtient l'acces complet a l'oeuvre. Un recu est genere. |
| Historique d'achats | Le lecteur peut consulter ses achats passes. Le vendeur voit l'historique des ventes. |
| Gestion des licences | Chaque achat genere une licence (acheteur + oeuvre + date + droits accordes). La licence est stockee sur le COG vendeur et une copie sur le COG acheteur. |

### 2.5 Telechargement hors-ligne

| Fonctionnalite | Description |
|----------------|-------------|
| Autorisation vendeur | Le vendeur active ou desactive le telechargement par oeuvre ou globalement. |
| Telechargement | Si autorise, le lecteur ayant achete l'oeuvre peut la telecharger sur son COG. Les fichiers sont stockes localement via KindMother sur le COG du lecteur. |
| Lecture hors-ligne | La liseuse integree dans Central permet de lire les oeuvres telechargees sans connexion. |
| Integrite | Les fichiers telecharges incluent une signature (hash) pour verification d'integrite. |
| Mise a jour | Si le vendeur met a jour une oeuvre (correction de pages), le lecteur est notifie et peut re-telecharger. |
| Revocation | Si le vendeur revoque l'autorisation de telechargement, les futures tentatives sont bloquees. Les fichiers deja telecharges restent accessibles localement (respect LOI-3). |

### 2.6 Presence, decouverte et Portail Agrege

| Fonctionnalite | Description |
|----------------|-------------|
| Declaration de service | Le COG proposant JayManga declare le service dans son Passeport COG. Les trackers indexent cette information. |
| Decouverte | Un lecteur peut decouvrir les COGs proposant JayManga via les catalogues des trackers MWS. |
| API de presence | Endpoint permettant de verifier si un COG vendeur specifique est en ligne. Utilise par la bibliotheque lecteur et le Portail Agrege pour afficher le statut. |
| **Portail Agrege** | Interface inter-COG (Type 3) hebergee par un COG qui collecte et affiche les catalogues de tous les COGs JayManga connus via le MWS. Le lecteur parcourt un catalogue unifie — les oeuvres de COGs hors-ligne sont visibles (metadonnees en cache) mais grisees. Emule l'experience d'un catalogue en ligne centralise (Mangadraft, Manga.io) tout en restant decentralise. |
| API de federation catalogue | Protocole inter-COG permettant a un COG d'exposer un resume de son catalogue (metadonnees, couvertures) pour aggregation par les Portails Agreges. Le vendeur controle l'opt-in. |

### 2.7 Administration des ventes (Admin/Vendeur — Central)

| Fonctionnalite | Description |
|----------------|-------------|
| Tableau de bord | Synthese : nombre de ventes, revenus du jour/semaine/mois, oeuvres les plus vendues, lecteurs actifs. |
| Historique des transactions | Liste chronologique des achats avec details (acheteur, oeuvre, montant, date, mode de paiement). |
| Gestion des licences | Vue des licences emises, possibilite de revoquer une licence (cas de fraude ou remboursement). |
| Remboursement | Remboursement total ou partiel d'un achat. Revocation de la licence si remboursement total. |
| Export | Export CSV ou PDF des ventes (periodes, filtres). |
| Configuration des prix | Modification des prix, creation de promotions temporaires (remises en pourcentage ou montant fixe sur une periode). |

---

## 3. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance COG** | Le service fonctionne sous gouvernance COG : StrongFather (decisions de publication et de vente), KindMother (persistance des fichiers et donnees), Master Butler (capacites/permissions de lecture et telechargement), WorrySentinel (securite des transactions), Border Guard (frontieres du contenu). |
| **Souverainete du vendeur** | Chaque COG est souverain sur son catalogue, ses prix, ses conditions de lecture et de telechargement (LOI-3). Aucune autorite centrale ne dicte les conditions de vente. |
| **Contenu decentralise** | Les fichiers manga restent sur le COG vendeur (LOI-1, LOI-7). Il n'existe pas de stockage centralise. La lecture passe par la surface web du COG ou par telechargement autorise. |
| **Hors-ligne d'abord** | Les oeuvres telechargees sont lisibles sans connexion (LOI-1, LOI-2). Les favoris et progressions sont stockes localement. |
| **Presence via MWS** | La disponibilite du vendeur est determinee par le MWS. JayManga n'invente pas son propre systeme de presence ; il consomme celui du MWS (LOI-6). |
| **Demonstration avant achat** | Chaque oeuvre propose des pages de demonstration pour encourager la decouverte tout en protegeant le contenu payant. |
| **Simplicite du paiement** | Le module de paiement est integre et leger. En V1, pas d'integration JayKonta ; le vendeur gere ses revenus directement. |
| **Interpolarite** | JayManga peut a terme s'integrer avec JayXpose (vitrine du catalogue), JayKonta (comptabilite), et les autres services Jay. Ces integrations sont optionnelles et gouvernees. |

---

## 4. Acteurs et roles

| Acteur | Description |
|--------|-------------|
| **Vendeur (Admin COG)** | Proprietaire du COG. Publie sa collection, fixe les prix, configure les pages de demonstration, gere les ventes, autorise ou non le telechargement. Interagit via **Central**. |
| **Lecteur authentifie** | Utilisateur ayant un COG propre. Peut mettre en favoris, acheter, telecharger, et lire hors-ligne. Son COG stocke ses favoris et ses fichiers telecharges. |
| **Lecteur visiteur** | Utilisateur sans COG ou non authentifie. Peut parcourir le catalogue et lire les pages de demonstration. Ne peut ni acheter, ni mettre en favoris, ni telecharger. |
| **MWS (Trackers)** | Fournissent la decouverte des COGs proposant JayManga et l'information de presence en ligne. |

---

## 5. Flux utilisateur principaux

### 5.1 Flux vendeur — publication

```
Admin → Central → JayManga (gestion)
  → Importer fichiers manga (images par chapitre)
  → Saisir metadonnees (titre, auteur, genre, synopsis)
  → Organiser en serie / volumes / chapitres
  → Definir pages de demonstration (ex. 10 pages)
  → Fixer le prix (ou gratuit)
  → Configurer autorisation telechargement
  → Publier → disponible sur le Portail
```

### 5.2 Flux lecteur — decouverte et lecture

```
Lecteur → Portail COG Vendeur (ou decouverte via MWS)
  → Parcourir le catalogue manga
  → Ouvrir une fiche oeuvre
  → Lire les pages de demonstration (liseuse)
  → [Si interessé] → Ajouter au panier → Checkout → Paiement
  → Acces complet a l'oeuvre
  → [Optionnel] Mettre en favoris
  → [Si autorise] Telecharger sur son COG
```

### 5.3 Flux lecteur — lecture hors-ligne

```
Lecteur → Central (sur son propre COG)
  → Bibliotheque personnelle
  → Oeuvres telechargees
  → Liseuse locale (pas de connexion necessaire)
```

### 5.4 Flux presence et favoris

```
Lecteur → Central → Bibliotheque (favoris)
  → Affichage des oeuvres favorites
  → Pour chaque favori : requete MWS → statut presence COG vendeur
  → Indicateur : "En ligne" / "Hors ligne"
  → Si en ligne : lien direct vers la liseuse sur le Portail du vendeur
  → Si hors-ligne : lecture locale si telecharge, sinon "Indisponible"
```

---

## 6. Modele de donnees (orientation)

### 6.1 Oeuvre (Work)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de l'oeuvre. |
| series_id | UUID (FK, optionnel) | Serie parente. |
| title | TEXT | Titre de l'oeuvre. |
| authors | JSON | Liste des auteurs (nom, role). |
| genres | JSON | Liste des genres (action, romance, fantasy, etc.). |
| synopsis | TEXT | Description / resume. |
| cover_image_path | TEXT | Chemin local vers l'image de couverture (KindMother). |
| language | TEXT | Langue de l'oeuvre (ISO 639-1). |
| volume_number | INTEGER (optionnel) | Numero de volume dans la serie. |
| status | TEXT | `draft` / `published` / `unlisted` / `archived`. |
| pricing_model | TEXT | `free` / `paid`. |
| price | NUMERIC | Prix en centimes (0 si gratuit). |
| currency | TEXT | Devise (defaut EUR). |
| demo_pages_count | INTEGER | Nombre de pages de demonstration gratuites. |
| reading_format | TEXT | Format de lecture : `manga` / `webtoon` / `landscape` / `comics` / `free`. |
| allow_download | BOOLEAN | Autorisation de telechargement hors-ligne pour les acheteurs. |
| total_pages | INTEGER | Nombre total de pages. |
| tags | JSON | Tags libres pour la recherche. |
| created_at | TEXT | ISO 8601. |
| updated_at | TEXT | ISO 8601. |

### 6.2 Chapitre (Chapter)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique du chapitre. |
| work_id | UUID (FK) | Oeuvre parente. |
| chapter_number | INTEGER | Numero du chapitre dans l'oeuvre. |
| title | TEXT (optionnel) | Titre du chapitre. |
| page_count | INTEGER | Nombre de pages du chapitre. |
| sort_order | INTEGER | Ordre d'affichage. |
| created_at | TEXT | ISO 8601. |

### 6.3 Page

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la page. |
| chapter_id | UUID (FK) | Chapitre parent. |
| page_number | INTEGER | Numero de page dans le chapitre. |
| original_image_path | TEXT | Chemin local vers le fichier image original (KindMother). |
| optimized_variants | JSON | Variantes optimisees : `[{resolution, format, path, file_size}]`. |
| width | INTEGER | Largeur originale en pixels. |
| height | INTEGER | Hauteur originale en pixels. |
| file_size | INTEGER | Taille originale en octets. |
| optimization_status | TEXT | `pending` / `optimized` / `skipped`. |
| sort_order | INTEGER | Ordre global dans l'oeuvre (pour la navigation lineaire). |

### 6.4 Serie (Series)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la serie. |
| title | TEXT | Titre de la serie. |
| synopsis | TEXT | Description de la serie. |
| cover_image_path | TEXT | Couverture de la serie. |
| status | TEXT | `ongoing` / `completed` / `hiatus`. |
| created_at | TEXT | ISO 8601. |
| updated_at | TEXT | ISO 8601. |

### 6.5 Licence d'achat (PurchaseLicense)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la licence. |
| buyer_cog_id | TEXT | Identifiant du COG acheteur. |
| buyer_identity | TEXT | Identite de l'acheteur (LSI, VID ou WID). |
| work_id | UUID (FK) | Oeuvre achetee. |
| purchase_type | TEXT | `work` (oeuvre complete) / `chapter` (chapitre individuel) / `series` (serie complete). |
| target_id | UUID | ID de la cible (work_id, chapter_id ou series_id selon purchase_type). |
| amount_paid | NUMERIC | Montant paye en centimes. |
| currency | TEXT | Devise. |
| payment_method | TEXT | Mode de paiement utilise. |
| download_allowed | BOOLEAN | Telechargement autorise au moment de l'achat. |
| status | TEXT | `active` / `refunded` / `revoked`. |
| purchased_at | TEXT | ISO 8601. |
| refunded_at | TEXT (optionnel) | ISO 8601, si applicable. |

### 6.6 Transaction de paiement (PaymentTransaction)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la transaction. |
| license_id | UUID (FK) | Licence associee. |
| buyer_cog_id | TEXT | COG acheteur. |
| amount | NUMERIC | Montant en centimes. |
| currency | TEXT | Devise. |
| method | TEXT | `card` / `transfer` / `other`. |
| status | TEXT | `pending` / `completed` / `failed` / `refunded`. |
| external_ref | TEXT (optionnel) | Reference de la passerelle de paiement externe. |
| created_at | TEXT | ISO 8601. |
| completed_at | TEXT (optionnel) | ISO 8601. |

### 6.7 Favori lecteur (ReaderFavorite) — stocke sur le COG du lecteur

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique du favori. |
| seller_cog_id | TEXT | Identifiant du COG vendeur. |
| work_id | TEXT | Identifiant de l'oeuvre sur le COG vendeur. |
| cached_title | TEXT | Titre en cache. |
| cached_cover_url | TEXT | URL ou chemin de la couverture en cache. |
| cached_authors | JSON | Auteurs en cache. |
| purchase_status | TEXT | `demo` / `purchased` / `downloaded`. |
| last_read_chapter | INTEGER (optionnel) | Dernier chapitre lu. |
| last_read_page | INTEGER (optionnel) | Derniere page lue. |
| reading_progress | REAL | Pourcentage de progression (0.0 a 1.0). |
| added_at | TEXT | ISO 8601. |
| last_synced_at | TEXT | ISO 8601 — derniere synchronisation des metadonnees. |

### 6.8 Configuration vendeur (SellerConfig)

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique. |
| shop_name | TEXT | Nom de la librairie manga affiche sur le Portail. |
| shop_description | TEXT | Description de la librairie. |
| default_demo_pages | INTEGER | Nombre de pages de demonstration par defaut (applicable aux nouvelles oeuvres). |
| default_allow_download | BOOLEAN | Autorisation de telechargement par defaut. |
| accepted_payment_methods | JSON | Modes de paiement acceptes. |
| currency | TEXT | Devise par defaut. |
| reading_direction | TEXT | `rtl` (droite a gauche, manga japonais) / `ltr` (gauche a droite, comics/manhua). |
| theme | JSON | Personnalisation visuelle du catalogue sur le Portail (couleurs, banniere). |
| created_at | TEXT | ISO 8601. |
| updated_at | TEXT | ISO 8601. |

---

## 7. Integration MWS — Presence et decouverte

### 7.1 Declaration du service dans le Passeport COG

Lorsque JayManga est active sur un COG, le service est declare dans le champ `services` du Passeport COG presente aux relays et trackers du MWS. Cela permet aux trackers d'indexer les COGs proposant JayManga dans leurs catalogues.

### 7.2 API de presence

| Endpoint | Description |
|----------|-------------|
| `GET /api/jaymanga/presence/{cog_id}` | Retourne le statut de presence du COG vendeur (`online` / `offline` / `unknown`). Interroge le MWS (tracker local ou cache). |
| `GET /api/jaymanga/discover` | Liste les COGs proposant JayManga connus du tracker local. Retourne : `cog_id`, `shop_name`, `work_count`, `online_status`. |

### 7.3 Lecture distante

La lecture en ligne passe par la surface web du COG vendeur. Le lecteur accede au Portail du vendeur ; les pages manga sont servies directement depuis le COG vendeur. Si le COG est hors-ligne, le contenu est inaccessible (sauf si telecharge localement).

### 7.4 Telechargement inter-COG

Lorsqu'un lecteur telecharge une oeuvre achetee :

1. Le COG lecteur envoie une requete au COG vendeur avec la licence d'achat.
2. Le COG vendeur verifie la licence et l'autorisation de telechargement.
3. Si valide, les fichiers sont transmis via le tunnel MWS (ou connexion directe).
4. Le COG lecteur stocke les fichiers via KindMother localement.
5. Une signature (hash SHA-256) est verifiee pour chaque fichier recu.

---

## 8. Types de service

| Espace | Description |
|--------|-------------|
| **Central** | Interface admin/vendeur : publication de manga, gestion du catalogue, configuration des prix et demonstrations, suivi des ventes, gestion des licences. Service interne COG (Type 1) pour la partie admin. |
| **Portail** | Catalogue manga public du COG, liseuse en ligne, pages de demonstration, panier et achat. Service a surface web externe (Type 2) pour la partie lecteur. |
| **Portail Agrege** | Interface inter-COG (Type 3) unifiee : collecte les catalogues de tous les COGs JayManga connus via le MWS, navigation et recherche agregees, COGs hors-ligne grises. Emule l'experience d'un Mangadraft/Manga.io decentralise. Heberge par tout COG souhaitant offrir cette vue globale. |

**Regle** : JayManga est un **Service de Type 2** (surface web externe pour la lecture et l'achat) avec une composante **Type 1** (gestion et publication reservees a l'admin dans Central) et une composante **Type 3** (Portail Agrege inter-COG pour la decouverte unifiee).

---

## 9. Niveaux de securite (orientation)

| Categorie de donnees | Niveau | Justification |
|----------------------|--------|---------------|
| Catalogue public (titres, couvertures, metadonnees) | **Public (0)** | Contenu destine a etre decouvert par les lecteurs. |
| Pages de demonstration | **Public (0)** | Contenu explicitement offert en acces libre. |
| Pages payantes (contenu complet) | **Sensitive (2)** | Contenu protege par achat, acces controle par licence. |
| Licences d'achat | **Sensitive (2)** | Preuves de transaction entre acheteur et vendeur. |
| Transactions de paiement | **Critical (3)** | Donnees financieres sensibles. |
| Fichiers manga originaux (stockage KindMother) | **Standard (1)** a **Sensitive (2)** | Fichiers source du vendeur. |
| Favoris et progression lecteur | **Standard (1)** | Donnees personnelles non sensibles. |
| Configuration vendeur | **Standard (1)** | Configuration metier non sensible. |
| Cache catalogue agrege (metadonnees) | **Public (0)** a **Standard (1)** | Metadonnees publiques mises en cache sur le COG aggregateur. |

---

## 10. Contraintes et regles metier

| Regle | Description |
|-------|-------------|
| **RM-01** | Un lecteur ne peut acceder aux pages au-dela de la demonstration que s'il possede une licence active pour l'oeuvre. |
| **RM-02** | Le telechargement n'est possible que si la licence est active ET que le vendeur a autorise le telechargement pour cette oeuvre. |
| **RM-03** | Les fichiers telecharges restent sur le COG du lecteur meme si le vendeur desactive le telechargement ulterieurement (LOI-3 — etat local souverain). |
| **RM-04** | Un remboursement total entraine la revocation de la licence. Le lecteur perd l'acces en ligne mais conserve les fichiers deja telecharges (LOI-3). |
| **RM-05** | Les prix sont en centimes pour eviter les erreurs d'arrondi. |
| **RM-06** | Un vendeur ne peut pas modifier le prix d'une oeuvre retroactivement pour les licences deja emises. |
| **RM-07** | Le nombre de pages de demonstration doit etre au minimum de 1 et ne peut pas depasser 50% du nombre total de pages de l'oeuvre. |
| **RM-08** | Le catalogue d'un COG hors-ligne est inaccessible en lecture distante. Les metadonnees en cache dans les favoris et le Portail Agrege restent visibles (grisees). |
| **RM-09** | Le Portail Agrege ne stocke que des metadonnees en cache (titre, couverture, auteur, genre, prix, statut). Les fichiers manga ne transitent jamais par le COG aggregateur. La lecture passe toujours par le COG vendeur d'origine. |
| **RM-10** | Un vendeur peut refuser l'indexation de son catalogue par les Portails Agreges (`allow_aggregation = false`). Son catalogue reste accessible uniquement via son propre Portail. |

---

## 11. Prochaines etapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser.
2. **Specifier** : Documenter les Operateurs et Kits JayManga (lecteur, catalogue, paiement, telechargement).
3. **Liseuse** : Specifier le composant liseuse (navigation, zoom, modes de lecture, responsive).
4. **Integration MWS** : Formaliser la declaration de service dans le Passeport COG et les endpoints de presence/decouverte.
5. **Paiement** : Specifier le module de paiement integre (passerelle, securite, confirmations).
6. **Securite** : Formaliser la protection des pages payantes (authentification, controle d'acces, anti-scraping).
7. **Implementation** : Developper le crate `crates/jaymanga/` en suivant le pattern standard des services.
8. **Phase 2** : Notes et avis lecteurs, catalogue agrege via trackers MWS, integration JayKonta pour la comptabilite, systeme de recommandation.

---

## 12. References

| Document | Role |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Operateur, Mandat, COG, Niveaux de securite). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Systeme de presence, decouverte et transport des COGs. |
| [Miyukini Conceptual References - Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) | Classification Type 1 / Type 2 / Type 3. |
| [JayShop - Document Fondateur](../JayShop/JayShop%20-%20Document%20Fondateur.md) | Reference pour les patterns de vente et paiement. |
| [JayXpose - Document Fondateur](../JayXpose/JayXpose%20-%20Document%20Fondateur.md) | Reference pour la gestion de catalogue (integration optionnelle). |
| [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) | Reference pour la comptabilite (integration Phase 2). |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | Principe d'interpolarite et couplage entre services Jay. |

---

**Document** : JayManga — Document fondateur
**Version** : 1.4
**Date** : 2026-02-24
**Statut** : Document de reference — enrichi avec UI/UX, onboarding Miou, gamification, guide et plan d'implementation.
