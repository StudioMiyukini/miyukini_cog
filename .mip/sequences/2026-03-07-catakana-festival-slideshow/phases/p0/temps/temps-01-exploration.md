# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : TERMINE
- Phase : P0 Temps 1
- Responsable principal : Maria
- Horodatage T0 : 2026-03-07 07:35

## TL;DR

Slideshow TypeScript kiosque pour le Festival Catakana 2026 "La fantaisie dans la pop culture".
Diffusion publique sur ordinateur au festival (Chateau de Catala, 2-3 mai 2026).
App standalone fullscreen, auto-avancante, dynamique et moderne. Nouveau projet from scratch.
Classification : T4 / C3.

## Section 0 — Orientation

| Question | Reponse deduite | Confiance |
|----------|----------------|-----------|
| Pourquoi ? | Outil de communication visuelle au stand public du festival | haute |
| Exemple concret d'usage | Ordinateur affiche en boucle au festival, les visiteurs passent devant | haute |
| Solution existante proche ? | Aucune dans le projet — nouveau projet standalone | haute |
| Pour qui ? | Visiteurs du festival Catakana 2026, public general, familles, geeks | haute |
| Fonction Online / MWS requise ? | Non — standalone local, pas de serveur | haute |
| Open-source / from scratch ? | From scratch | haute |
| Classification estimee | T4 — app standalone multi-fichiers, animation engine, contenu riche | haute |

## Brainstorming

### Contexte festival (Catakana 2026)
- Edition 3 du Festival Catakana — "La fantaisie dans la pop culture"
- Dates : 2 & 3 mai 2026, Chateau de Catala, Saint-Orens-de-Gameville (31)
- Horaires : Samedi 10h-20h / Dimanche 10h-18h
- Objectif : 2000 visiteurs, 30+ exposants
- Organisation : Association Catakana / Studio Miyukini, direction Jean (Miyukini)

### Contenu disponible (extrait du site)
- Thematique : La fantaisie — films (LOTR, GoT, HP), anime (Dungeon Meshi, Berserk, Re:Zero...), jeux (WoW, LoL, FF)
- Animation fil rouge : "Guilde d'aventurier" — quetes, pieces, tresors
- Activites : cosplay, karaoké, quiz, jeux de role, concert, ateliers, foodtrucks
- Historique : 2024 (Le Manga, 2000 visiteurs), 2025 (40 ans geek), 2026 (Fantaisie)
- Valeurs : Accessibilite, Creativite, Ecologie, Transmission
- Equipe : Miyukini/Jean (pres), Nielu/Nico (tresorier), Melanie (secretaire), Sokola (co-pres)

### Contraintes techniques
- TypeScript obligatoire (demande explicite)
- Diffusion sur ordinateur (pas mobile, donc 16:9 / 1920x1080)
- Pas de serveur requis — doit tourner depuis dist/index.html
- Moderne et dynamique — animations, transitions fluides

### Approches techniques envisagees

A) **Vite + TypeScript vanilla** — build propre, pas de framework, bundled en dist/
   RETENU : simple, no-deps pour l'affichage, moderne

B) React/Vue + TS — inutilement lourd pour un slideshow statique

C) Reveal.js/Impress.js — frameworks slideshow, moins de controle visuel

D) HTML/CSS/JS pur — pas de TypeScript natif, moins propre

## Hypotheses retenues

1. Vite + TypeScript vanilla (pas de framework)
2. Fullscreen auto-play, boucle infinie
3. Particle background canvas (etoiles/magie) pour ambiance fantaisie
4. Transitions cross-fade + slide-up staggered pour le texte
5. Palette : deep purple/dark (#0d0820), orange (#ff6b2b), or (#ffd700), blanc
6. 9-10 slides : Hero, Theme, Univers, Guilde, Activites, Exposants, Timeline, Valeurs, Contact
7. Progress bar visible + dots de navigation
8. Auto-avance toutes les 7 secondes, navigation manuelle possible (touches fleche/clic)

## Hypotheses ecartees

- Serveur backend : inutile, tout est statique
- Images reelles : non disponibles, utiliser des gradients/icones/emojis
- Framework lourd (React) : overhead injustifie pour un slideshow
- Reveal.js : moins de controle sur le visuel

## Classification post-T1

- Classe tache (T1-T5) : **T4** — app frontend standalone, ~8 fichiers, engine animation, 10 slides
- Complexite sequence estimee (C1-C5) : **C3** — fonctionnalite complete, frontend seul, pas de backend
  > Sera confirme en T2 et valide par Denis en T8.
