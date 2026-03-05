# JayManga - Lecture et vente de manga en ligne

## Contexte

JayManga est le service Miyukini dedie a la **lecture et la vente de manga en ligne** :
- publication et mise en lecture de collections manga (Manga, Webtoon, 16:9, Comics, format libre)
- aucune limite de stockage â€” le seul plafond est le hardware du COG
- outil integre de redimensionnement et compression pour optimiser la vitesse d'affichage
- lecture en ligne avec pages de demonstration gratuites
- achat d'oeuvres avec module de paiement integre
- gestion de la bibliotheque lecteur (favoris, historique)
- telechargement hors-ligne des oeuvres achetees (si autorise par le vendeur)
- detection de presence du COG vendeur via le MWS
- surface web externe accessible depuis le Portail de chaque COG proposant le service
- **Portail Agrege** : interface inter-COG unifiee emulant un catalogue en ligne (type Mangadraft/Manga.io), COGs hors-ligne grises

## Documentation principale

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Vision, scope, objectifs, principes directeurs, modele de donnees, avantages. |

## Specifications fonctionnelles detaillees

| Document | Role |
|----------|------|
| [JayManga - Publication et Catalogue](./JayManga%20-%20Publication%20et%20Catalogue.md) | Import, formats de lecture, outil d'optimisation/compression, metadonnees, organisation, statuts. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Liseuse web et native, modes de lecture (Manga, Webtoon, 16:9, Comics), navigation, demonstration. |
| [JayManga - Achat et Paiement](./JayManga%20-%20Achat%20et%20Paiement.md) | Module de paiement integre, panier, licences, remboursements, promotions, administration des ventes. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Favoris cross-COG, bibliotheque lecteur, telechargement hors-ligne, presence MWS, cache. |
| [JayManga - Portail Agrege et Decouverte](./JayManga%20-%20Portail%20Agrege%20et%20Decouverte.md) | Interface inter-COG unifiee (Type 3), collecte de catalogues, navigation agregee, COGs hors-ligne grises, API de federation. |

## UI, UX et engagement

| Document | Role |
|----------|------|
| [JayManga - Onboarding Miou et Gamification](./JayManga%20-%20Onboarding%20Miou%20et%20Gamification.md) | Onboarding lecteur/vendeur via Miou, systeme de progression (XP, niveaux, streaks, badges), visiteurs. Document transversal aux 3 interfaces. |
| [JayManga - UI Central et Stable](./JayManga%20-%20UI%20Central%20et%20Stable.md) | Interface Dioxus native pour COG STABLE : navigation, ecrans vendeur/lecteur, liseuse native, theme, composants. |
| [JayManga - UI Mobile Terminal](./JayManga%20-%20UI%20Mobile%20Terminal.md) | Interface Dioxus native pour COG TERMINAL (mobile) : navigation tactile, liseuse gestuelle, sync bidirectionnelle, notifications, hors-ligne. |
| [JayManga - UI Web Portal](./JayManga%20-%20UI%20Web%20Portal.md) | Interface web pour Portail vendeur et Portail Agrege : responsive, liseuse web, SEO, personnalisation vendeur, securite anti-scraping. |

## Implementation

| Document | Role |
|----------|------|
| [JayManga - Guide Implementation](./JayManga%20-%20Guide%20Implementation.md) | Guide technique : structure crate, types de domaine, persistance, APIs REST, integration MWS, composants UI, gamification, securite, tests. |
| [JayManga - Plan Implementation](./JayManga%20-%20Plan%20Implementation.md) | Plan d'implementation en 8 phases (0-7) : dependances, modules, criteres de validation, jalons. |

## Liaisons ecosysteme

- [JayShop - Document Fondateur](../JayShop/JayShop%20-%20Document%20Fondateur.md) â€” Reference pour les patterns de vente
- [JayXpose - Document Fondateur](../JayXpose/JayXpose%20-%20Document%20Fondateur.md) â€” Reference pour la gestion de catalogue
- [Miyukini Web Portal](../MiyukiniWebPortal/) â€” Surface web du Portail
- [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) â€” Presence et decouverte
- [MWS - Trackers](../../miyukini-webway-system/acteurs/MWS%20-%20Trackers.md) â€” Tracker, manifestes de services
- [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md) â€” Protocole de distribution des manifestes (schema JayManga)
- [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md)

