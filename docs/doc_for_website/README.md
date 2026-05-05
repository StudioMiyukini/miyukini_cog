# Documentation Site Web — Miyukini COG

Cette arborescence contient la **documentation structurée pour le site web** du projet Miyukini COG.

## Structure

- **index.html** — Page d’accueil listant toute la documentation (cartes filtrables + recherche par titre).
- **doc.html** — Page de lecture d’un document ; un document = une page (paramètre `?path=...`).
- **css/style.css** — Styles communs (thème sombre, cartes, toolbar, rendu Markdown).
- **js/list.js** — Logique de la liste : chargement de `index.json`, filtres (catégorie, strate, type), recherche dynamique par titre.
- **js/doc.js** — Logique de la page document : chargement du fichier `.md` et rendu via Marked.js.
- **index.json** — Configuration des cartes filtrables (filtres par catégorie, strate, type) et liste des pages.
- **presentation/** — Présentation générale, définition COG.
- **architecture/** — Lois d'autonomie, pyramide des strates, souveraineté, Kernel.
- **cores/** — Les 8 Cores (StrongFather, KindMother, TAMR, BorderGuard, WorrySentinel, LogisticsSteward).
- **interface/** — BondingBrother (Strate 5).
- **tools/** — Vue d'ensemble des toolkits et fiches (MiyuAuth, MiyuSQL, MiyuWeb, Webway, etc.).
- **services/** — Vue d'ensemble et fiches services (Central, Miou, JayKoa, JayKonta, etc.).
- **mws/** — Miyukini Webway System (fondateur, architecture, Origin, Relays, Trackers, protocole, sécurité).
- **security/** — Vue d'ensemble sécurité, niveaux, états de confiance, protection par les Cores.
- **admin/** — MiyukiniAdmin (Strate 9).
- **reference/** — Glossaire, opérateurs et terminologie.

## Utilisation sur le site (origine de la doc)

1. **Page liste** (`index.html`) : affiche toutes les entrées de `index.json` sous forme de cartes. Une **barre de recherche** filtre dynamiquement par titre (et description). Des **listes déroulantes** filtrent par Catégorie, Strate et Type. Le compteur affiche « X / N document(s) ».
2. **Un document = une page** : clic sur une carte ouvre `doc.html?path=presentation/presentation-generale.md`. La page charge le fichier `.md` correspondant et le rend en HTML (Marked.js). Le titre de la page est déduit du premier `#` du Markdown ou du nom du fichier.
3. **Servir l’origine** : pour que le chargement de `index.json` et des `.md` fonctionne, il faut servir le dossier `doc_for_website` via un serveur HTTP (éviter d’ouvrir `index.html` en `file://`). Exemple : depuis la racine du dépôt, `npx serve docs/doc_for_website` ou déployer ce dossier comme racine du site de doc.

## Exclusions

Cette doc pour le site **ignore** volontairement :

- Documents d'implémentation détaillée
- Références à des produits concurrents
- Audits
- Fichiers `_index.md`
- Dossiers `qa`, `spm-cms`, `setup`, `implementation`

Elle s'appuie sur la structure de `docs/` en ne retenant que les aspects **présentation**, **architecture**, **fonctions** et **services** destinés au public.

## Contenu enrichi

La documentation a été enrichie avec :

- **Présentation** : vision en une phrase, ampleur du projet, « Pour qui » détaillé (collectivités, festivals, professionnels, décideurs), tableau « Ce que Miyukini n'est pas ».
- **Définition COG** : décryptage de l'acronyme (C, O, G), formulations FR/EN, analogie de l'engrenage, analogie du « pays » (territoire, constitution, gouvernement, citoyens).
- **Lois d'autonomie** : section « En pratique » avec questions de vérification rapide.
- **Souveraineté** : bloc « Voir aussi » vers Définition COG, Lois, Pyramide.
- **MWS** : principes cardinaux (maillage ne fait pas confiance, Trackers officiels), consommation par les strates, principe fondateur.
- **MWS Architecture** : principe fondateur et liens « Voir aussi ».
- **Miou** : catalogue et moteurs (tutoriels, templates, gamification, bulles UI), consentement et TTS.
- **Miyukini Central** : catalogue grille/liste, onglets, sidebar, thème, parcours utilisateur typique.
- **Glossaire** : BondingBrother, Lobby (Lobbys), Passeport, précision Permis (accord relay).
