# JayEcole — Brainstorming Round 3 : Le Pivot Construction

## Contexte

Ce document approfondit les decisions majeures prises lors du round 2 de brainstorming. Le pivot le plus structurant est l'abandon du narratif "brouillard de l'Oubli / liberation du monde" au profit d'un ton **artisan/construction** : l'enfant construit quelque chose de concret au fil de son apprentissage.

## Portee / Scope

- **Applicable a :** Phase d'approfondissement round 3, decisions structurantes.
- **Audience :** Equipe projet, utilisateur fondateur.
- **Statut :** Brainstorming approfondi, en attente de decisions round 3.
- **Prerequis :** BRAINSTORM-initial.md (round 1+2), BRAINSTORM-approfondissement.md.
- **Redige par :** Maria (Chef de Projet)
- **Date :** 2026-02-27

### Decisions round 2 qui impactent ce document

| Decision | Impact |
|----------|--------|
| Ton artisan/construction | Re-imagine tout le framework narratif (section A) |
| Miyu-sensei humanoide | Design du personnage et role dans la construction (section B) |
| Gardiens = animaux | Verba le chevalier remplace, integration construction (section C) |
| CP prioritaire | Implications UX, audio, exercices, prototype (section D) |
| DYS dans le MVP | Variantes d'exercices, parametres d'affichage (section E) |
| Service separe | Architecture technique, DB, Cores (section F) |
| Audio dans le MVP | Production, format, taille, TTS vs humain (section G) |
| Contenu from scratch | Strategie de creation de 800 exercices (section H) |

---

## A. Nouveau narratif : artisan/construction

### A.1 Pourquoi ce pivot

Le narratif "liberation du monde" a un defaut : il est **passif et abstrait**. L'enfant "dissipe du brouillard" mais ne voit pas concretement ce qu'il cree. Le pivot vers la construction resout ce probleme :

- **Tangible** : l'enfant voit son oeuvre grandir, piece par piece
- **Valorisant** : l'enfant est createur, pas sauveteur
- **Progressif** : chaque session ajoute un element visible
- **Personnel** : deux enfants n'ont pas la meme construction (choix esthetiques)
- **Motivant** : "je veux finir le toit de ma maison" est plus concret que "je veux liberer le village"

### A.2 Quatre concepts proposes

Je propose quatre concepts detailles. Chacun est viable, mais avec des forces et faiblesses differentes.

---

#### Option A : L'enfant construit un VILLAGE

**Concept :** L'enfant est le batisseur d'un petit village. Chaque exercice reussi lui fournit des materiaux (bois, pierre, tuiles) et des plans pour construire de nouveaux batiments et amenagements.

**Metaphore pedagogique :**
- Les exercices de francais fournissent des materiaux "vivants" (bois, fleurs, peinture) — ils embellissent le village
- Les exercices de maths fournissent des materiaux "solides" (pierre, brique, fer) — ils structurent le village
- Les competences sont des "plans" : chaque competence maitrisee debloque le plan d'un batiment

**Progression visuelle :**
```
Debut          : Terrain vide avec un arbre et Miyu-sensei
10 exercices   : Premiere cabane en bois
30 exercices   : Atelier de Miyu-sensei + chemin
60 exercices   : Maison, puits, cloture
100 exercices  : Place du village avec fontaine
150 exercices  : Ecole du village (!), bibliotheque
200 exercices  : Moulin, pont, jardin
300 exercices  : Eglise/mairie, place du marche
500 exercices  : Village complet avec decorations
800 exercices  : Village prospere, banniere, fete
```

**Comment les matieres contribuent :**

| Matiere | Zone du village | Batiments debloques |
|---------|----------------|---------------------|
| Francais — Lecture | Bibliotheque, librairie | Etageres de livres |
| Francais — Grammaire | Ecole du village | Tableau noir, bureaux |
| Francais — Orthographe | Atelier de calligraphie | Plumes, encriers |
| Francais — Vocabulaire | Place du marche | Etals avec mots |
| Maths — Numeration | Mairie | Horloge, calendrier |
| Maths — Calcul | Moulin, forge | Engrenages, enclume |
| Maths — Geometrie | Pont, clotures | Plans architecturaux |
| Maths — Mesures | Marche aux poids | Balance, toises |
| Maths — Problemes | Tour d'observation | Longue-vue, cartes |

**Interaction enfant :**
- Apres chaque session, l'enfant peut placer les nouveaux elements dans son village
- Choix de couleurs et de positions (personnalisation limitee mais reelle)
- Les gardiens animaux sont les habitants du village (Compti le herisson tient la forge, etc.)

**Forces :**
- Tres visuel, progression evidente
- Coherent avec le monde reel (les enfants connaissent un village)
- Chaque element a un sens educatif
- Grande surface de personnalisation

**Faiblesses :**
- Complexite graphique (beaucoup de sprites a creer)
- Risque que l'enfant se concentre sur la decoration plutot que les exercices

---

#### Option B : L'enfant construit un NAVIRE

**Concept :** L'enfant est un apprenti constructeur naval. Il construit un navire de plus en plus grand et elabore — d'un simple radeau a un magnifique galion — et peut ensuite l'utiliser pour "voyager" vers des iles thematiques (les domaines).

**Metaphore pedagogique :**
- Chaque competence ajoute une partie du navire
- Le francais construit la coque, les voiles, les cordages (la forme, l'expression)
- Les maths construisent le moteur, le gouvernail, les instruments (la logique, la precision)
- Le navire grandit et change de forme au fil des niveaux scolaires

**Progression visuelle :**
```
Debut          : Planche de bois flottante + Miyu-sensei
10 exercices   : Radeau avec un mat
30 exercices   : Petite barque avec voile
60 exercices   : Bateau a voile avec cabine
100 exercices  : Brick avec drapeau personnel
150 exercices  : Goelette avec vigie
200 exercices  : Caravelle avec canons decoratifs
300 exercices  : Fregate avec figure de proue
500 exercices  : Galion majestueux
800 exercices  : Navire legendaire avec effets lumineux
```

**Les iles-destinations :**
- Chaque domaine scolaire est une ile sur une carte maritime
- Le navire peut "naviguer" entre les iles (transition thematique)
- Les iles ont les gardiens animaux comme habitants
- Plus le navire est grand, plus il peut atteindre des iles lointaines (domaines avances)

**Forces :**
- Progression visuelle tres forte (petit radeau -> galion)
- Metaphore du voyage de la connaissance
- Les iles conservent la structure par domaine
- Moins de sprites qu'un village (un seul objet central)

**Faiblesses :**
- Moins de personnalisation qu'un village
- Le lien matiere -> partie du navire est moins intuitif
- Les enfants de 6 ans n'ont pas tous le concept de construction navale

---

#### Option C : L'enfant construit un ATELIER

**Concept :** L'enfant est un apprenti artisan dans un grand atelier. Chaque domaine scolaire correspond a une station de travail (etabli de menuiserie, table de chimie, coin lecture, forge de chiffres). L'enfant debloque et ameliore ses stations, cree des objets, et remplit des etageres.

**Metaphore pedagogique :**
- L'atelier est l'endroit ou l'on apprend en faisant
- Chaque station = un domaine
- Chaque exercice reussi produit un "objet" range sur une etagere
- Les outils se debiquent et s'ameliorent avec la maitrise

**Progression visuelle :**
```
Debut          : Piece vide avec un etabli basique + Miyu-sensei
10 exercices   : Premiers outils au mur, premiere etagere
30 exercices   : Deuxieme station de travail debloquer
60 exercices   : Etageres garnies, sol decore, lampes
100 exercices  : Toutes les stations presentes
150 exercices  : Outils dores, etageres pleines
200 exercices  : Fenetre avec vue sur l'exterieur
300 exercices  : Atelier luxueux, tapis, plantes
500 exercices  : Atelier de maitre artisan
800 exercices  : Atelier legendaire, brille
```

**Stations de travail :**

| Station | Domaine | Objets crees |
|---------|---------|-------------|
| Coin lecture | Lecture / Comprehension | Livres, parchemins |
| Pupitre d'ecriture | Grammaire / Orthographe | Lettres, phrases encadrees |
| Table des mots | Vocabulaire | Dictionnaire personnel, etiquettes |
| Forge des chiffres | Calcul / Numeration | Medailles numerotees, outils en metal |
| Table de geometrie | Geometrie / Mesures | Maquettes, formes en bois |
| Bureau des enigmes | Resolution de problemes | Cartes, plans, mecanismes |

**Forces :**
- Metaphore directe : apprendre = fabriquer
- Coherent avec le ton "artisan"
- Chaque objet cree est une trace de la reussite
- Plus intime et personnel qu'un village

**Faiblesses :**
- Visuellement plus confine (une seule piece vs un monde)
- Moins d'espace pour les gardiens animaux
- Pourrait sembler moins "aventureux" pour certains enfants

---

#### Option D : L'enfant construit une CABANE DANS LES ARBRES

**Concept :** L'enfant construit progressivement une cabane dans un grand arbre. La cabane commence comme une simple plate-forme et evolue en une structure elaborate avec plusieurs niveaux, des ponts, des decorations, un telescope, une bibliotheque perchee, etc.

**Metaphore pedagogique :**
- L'arbre est l'arbre de la connaissance (metaphore classique)
- Les racines = les fondamentaux (CP)
- Le tronc = la structure (CE1-CE2)
- Les branches = les specialisations (CM1-CM2)
- Chaque matiere ajoute une "aile" ou un "etage" a la cabane

**Progression visuelle :**
```
Debut          : Grand arbre avec une corde + Miyu-sensei au pied
10 exercices   : Plate-forme basique + echelle
30 exercices   : Murs et toit rudimentaire
60 exercices   : Fenetre, porte, plancher solide
100 exercices  : Deuxieme etage + pont vers une branche
150 exercices  : Telescope, bibliotheque, lanternes
200 exercices  : Troisieme etage + toboggan
300 exercices  : Tour d'observation, drapeaux, jardin suspendu
500 exercices  : Cabane palatiale sur plusieurs arbres relies
800 exercices  : Cite dans les arbres, lumineuse, majestueuse
```

**Comment les matieres construisent la cabane :**

| Matiere | Contribution architecturale |
|---------|---------------------------|
| Francais — Lecture | Bibliotheque perchee, coin lecture avec coussins |
| Francais — Grammaire | Panneaux indicateurs, regles affichees aux murs |
| Francais — Vocabulaire | Coffre au tresor de mots, dictionnaire geant |
| Maths — Numeration | Horloge, calendrier, compteur de visiteurs |
| Maths — Calcul | Mecanismes (poulie, treuil), engrenages |
| Maths — Geometrie | Plans architecturaux, formes decoratives |
| Maths — Mesures | Toises sur les murs, balance, thermometre |

**Les gardiens animaux :**
- Chaque gardien vit dans une "loge" speciale de la cabane
- Compti le herisson : dans un tiroir douillet pres de la forge a chiffres
- Lexia la chouette : dans la bibliotheque perchee, evidemment
- Conte le poisson-lune : dans un aquarium magique suspendu
- Somma l'ours : sur la branche la plus solide, pres des mecanismes
- Nouveau gardien grammaire : dans la tour d'observation

**Forces :**
- Universel : TOUS les enfants revent d'une cabane dans les arbres
- Progression tres visuelle (de rien a un palais dans les arbres)
- L'arbre comme metaphore de la connaissance est riche
- Les gardiens animaux s'integrent naturellement
- Sensation de "chez soi" — l'enfant s'identifie a sa cabane
- La cabane change VRAIMENT entre le debut et la fin — l'enfant voit le chemin parcouru
- Moins de sprites que le village (un seul point focal)

**Faiblesses :**
- Moins de variete que le village (un seul objet central, comme le navire)
- La metaphore arbre/connaissance est classique (mais classique = comprehensible)

---

### A.3 Tableau comparatif

| Critere | Village | Navire | Atelier | Cabane |
|---------|---------|--------|---------|--------|
| **Attrait enfant 6 ans** | Eleve | Moyen | Moyen | Tres eleve |
| **Attrait enfant 11 ans** | Moyen | Eleve | Eleve | Eleve |
| **Lien matiere-construction** | Tres bon | Moyen | Tres bon | Bon |
| **Progression visuelle** | Excellente | Excellente | Bonne | Excellente |
| **Personnalisation** | Tres elevee | Faible | Moyenne | Elevee |
| **Integration gardiens** | Naturelle | Moyenne | Forcee | Naturelle |
| **Complexite graphique** | Elevee | Moyenne | Moyenne | Moyenne |
| **Coherence ton artisan** | Bonne | Faible | Excellente | Bonne |
| **Effet "chez soi"** | Moyen | Faible | Bon | Excellent |
| **Sprites a creer** | 60-80 | 15-20 | 30-40 | 25-35 |

### A.4 Recommandation Maria

Mon classement personnel :

1. **Option D — Cabane dans les arbres** : meilleur equilibre entre attrait universel, progression visuelle, et integration narrative. La cabane est le "chez soi" de l'enfant, ce qui cree un attachement emotionnel fort. Les gardiens animaux y trouvent une place naturelle. L'arbre comme metaphore de la connaissance est coherent et riche.

2. **Option A — Village** : excellent sur la variete et la personnalisation, mais plus couteux en sprites et risque de distraction par la decoration.

3. **Option C — Atelier** : le plus coherent avec le ton "artisan", mais visuellement confine.

4. **Option B — Navire** : belle progression visuelle mais lien matiere-construction le plus faible.

**Proposition hybride** : on pourrait combiner Cabane (coeur) + Village (extension v1.0). Au MVP, l'enfant construit sa cabane dans l'arbre. En v1.0, la cabane devient le centre d'un petit village qui se developpe autour de l'arbre.

### A.5 Mecanisme de construction detaille (base Cabane)

Si l'option Cabane est retenue, voici comment le mecanisme fonctionne :

**Monnaie de construction : les Etoiles de Savoir**
- Chaque exercice reussi donne 1 a 3 etoiles (selon la qualite de la reponse)
- Les etoiles sont la monnaie unique pour debloquer des elements de construction
- Pas de monnaie secondaire (simplicite)

**Catalogue de construction :**

```
Catalogue (debloque progressivement par competences maitrisees)

  Structures (prix en etoiles) :
    - Plate-forme basique ........... 0 (gratuite au debut)
    - Echelle en bois ............... 10
    - Murs simples .................. 20
    - Toit en chaume ................ 30
    - Fenetre ronde ................. 15
    - Porte avec poignee ............ 20
    - Deuxieme etage ................ 50
    - Pont suspendu ................. 40
    - Tour d'observation ............ 80
    - Toboggan ...................... 60
    - ...

  Decorations (prix en etoiles) :
    - Lanternes ..................... 10
    - Guirlandes .................... 15
    - Drapeau personnel ............. 20
    - Fleurs suspendues ............. 10
    - Tapis ......................... 15
    - ...

  Meubles (debloques par competences) :
    - Bibliotheque .................. Debloquer "fr.lecture" niveau 0.6+
    - Tableau noir .................. Debloquer "fr.grammaire" niveau 0.6+
    - Horloge ....................... Debloquer "ma.numeration" niveau 0.6+
    - Etabli de geometrie ........... Debloquer "ma.geometrie" niveau 0.6+
    - ...
```

**Flux apres une session :**

```
1. L'enfant termine une session de 8 exercices
2. Ecran de bilan : "Tu as gagne 18 etoiles !"
3. Miyu-sensei : "Avec ces etoiles, tu peux ajouter quelque chose a ta cabane !"
4. [Bouton : "Construire maintenant" / "Plus tard"]
5. Si "Construire" :
   - Vue de la cabane avec elements debloques en surbrillance
   - L'enfant choisit un element et le place (glisser-deposer simple)
   - Animation de construction (marteau, sciure, etincelles)
   - Miyu-sensei : "Magnifique ! Ta cabane est de plus en plus belle !"
6. Retour a l'ecran d'accueil
```

**Ce qui remplace les anciens concepts :**

| Ancien concept (round 1-2) | Nouveau concept (construction) |
|----------------------------|-------------------------------|
| Carte du Savoir (regions) | L'arbre et sa cabane (lieu unique avec zones) |
| Villages a liberer | Elements de construction a debloquer |
| Brouillard de l'Oubli | Terrain vide qui se remplit |
| Jardin du Savoir | Fusionne : la cabane EST la representation de la perseverance |
| Etoiles de recompense | Etoiles de Savoir (monnaie de construction) |
| Badges de maitrise | Meubles speciaux debloques par maitrise |
| Regions = domaines | Zones de la cabane = domaines |

---

## B. Miyu-sensei humanoide

### B.1 Role dans la metaphore de construction

Miyu-sensei est le **maitre artisan** — un compagnon experimente qui guide l'enfant apprenti dans la construction de sa cabane. Ce n'est pas un professeur classique, c'est un artisan bienveillant qui apprend en faisant.

Vocabulaire de Miyu-sensei :
- "Exercice" -> "defi" ou "mission"
- "Bonne reponse" -> "bien construit" ou "solide"
- "Mauvaise reponse" -> "ca ne tient pas, on recommence"
- "Competence maitrisee" -> "tu maitrises cette technique"
- "Nouveau chapitre" -> "nouveau plan de construction"
- "Fin de session" -> "fin de la journee de chantier"

### B.2 Trois concepts d'apparence

**Concept 1 — Artisan chaleureux (style Ghibli)**
- Age apparent : jeune adulte (20-25 ans)
- Cheveux : courts, chatains, un peu en desordre (sciure dans les cheveux)
- Vetements : tablier de travail beige sur chemise blanche, manches retroussees
- Accessoire signature : un crayon a papier derriere l'oreille + un metre ruban autour du cou
- Expression par defaut : sourire doux, yeux un peu plisses
- Palette de couleurs : tons chauds (beige, brun, ocre, blanc)
- Genre : neutre/non specifie (le prenom "Miyu" est unisexe dans l'univers Miyukini)

**Concept 2 — Exploratrice inventrice (style anime moderne)**
- Age apparent : adolescente (14-16 ans)
- Cheveux : longs, attaches en queue haute, avec une meche rebelle
- Vetements : salopette bleue avec poches remplies d'outils, t-shirt colore
- Accessoire signature : lunettes de protection relevees sur le front + ceinture a outils
- Expression par defaut : enthousiaste, yeux grands ouverts
- Palette de couleurs : bleu, jaune, touches de rouge
- Genre : feminin

**Concept 3 — Compagnon sage (style livre d'enfant)**
- Age apparent : ageless (ni jeune ni vieux, style "grand frere/grande soeur")
- Cheveux : mi-longs, couleur sakura (rose clair, coherent avec Miyukini)
- Vetements : veste de travail verte foncee, bottes robustes, echarpe
- Accessoire signature : un carnet de croquis toujours en main + un marteau a la ceinture
- Expression par defaut : calme, regard bienveillant
- Palette de couleurs : vert, brun, rose pale
- Genre : neutre/non specifie

### B.3 Evolution visuelle avec la progression

Miyu-sensei evolue visuellement quand l'enfant progresse, pour renforcer la sensation d'aventure partagee :

| Progression enfant | Evolution de Miyu-sensei |
|-------------------|-------------------------|
| 0 - 50 exercices | Tenue de base, outils simples |
| 50 - 150 exercices | Nouveau tablier, outils ameliores |
| 150 - 300 exercices | Ceinture a outils complete, chapeau de chantier |
| 300 - 500 exercices | Tenue de maitre artisan, outils dores |
| 500 - 800 exercices | Tenue legendaire, aura lumineuse, cape |

L'enfant ne "debloque" pas ces tenues — elles apparaissent naturellement. C'est une recompense passive qui dit "on a grandi ensemble".

### B.4 Interactions specifiques a la construction

| Moment | Comportement de Miyu-sensei |
|--------|---------------------------|
| L'enfant gagne des etoiles | Sort un marteau et tapote la cabane avec un sourire |
| L'enfant place un element | Aide a porter l'element, fait un geste d'approbation |
| L'enfant debloque un meuble | Ouvre un plan, le deroule avec fierte |
| Nouvel etage construit | Monte sur le toit et plante un petit drapeau |
| L'enfant visite sa cabane | Assis dans un coin, lit un livre ou bricole |
| Gardien animal apparait | Accueille l'animal et le presente a l'enfant |

---

## C. Gardiens animaux — mise a jour

### C.1 Remplacement de Verba le chevalier

Verba le chevalier (region Grammaire/Conjugaison) doit etre remplace par un animal. Propositions :

**Option 1 — Verba le perroquet**
- Un perroquet colore qui repete et transforme les phrases
- Force : le perroquet parle, ce qui est parfait pour la grammaire
- Il conjugue les verbes en les "chantant" dans differentes formes
- Couleur : multicolore (coherent avec la richesse de la grammaire)

**Option 2 — Regla la tortue**
- Une vieille tortue sage qui connait toutes les regles
- Force : la lenteur evoque la patience necessaire pour la grammaire
- Les regles sont "gravees sur sa carapace"
- Couleur : vert et or

**Option 3 — Pluma le renard**
- Un renard ruse qui jongle avec les mots
- Force : le renard est malin, la grammaire est pleine de "pieges" et de subtilites
- Il "tisse" les phrases comme un renard tisse ses ruses
- Couleur : roux et blanc

**Recommandation :** Option 1 — Verba le perroquet. Le lien entre un animal qui parle et repete, et la grammaire/conjugaison, est le plus intuitif et amusant pour les enfants.

### C.2 Integration des gardiens dans la construction

Dans la metaphore de la cabane, les gardiens ne sont plus des "gardiens de region" mais des **compagnons artisans** qui vivent dans la cabane et aident l'enfant dans leur domaine :

| Gardien | Animal | Domaine | Role dans la cabane | Loge dans la cabane |
|---------|--------|---------|--------------------|--------------------|
| **Compti** | Herisson | Numeration / Calcul | Compte les etoiles, verifie les mesures | Nid douillet dans un tiroir pres de l'horloge |
| **Lexia** | Chouette | Vocabulaire / Orthographe | Garde la bibliotheque, corrige les mots | Perchoir dans la bibliotheque perchee |
| **Conte** | Poisson-lune | Lecture / Comprehension | Raconte des histoires le soir | Aquarium magique suspendu |
| **Somma** | Ours | Operations / Problemes | Porte les gros materiaux, actionne les mecanismes | Hamac geant sur la branche la plus solide |
| **Verba** | Perroquet | Grammaire / Conjugaison | Repete les regles, chante les conjugaisons | Perchoir colore pres de la fenetre |

**Mecanisme de deblocage :**
- Chaque gardien apparait quand l'enfant commence a travailler dans son domaine
- Miyu-sensei le presente : "Voici Compti ! C'est un herisson qui adore compter. Il va t'aider avec les nombres."
- Le gardien reste ensuite dans la cabane et reagit quand l'enfant travaille dans son domaine

**Interactions des gardiens :**
- Quand l'enfant fait un exercice de leur domaine, le gardien s'anime
- Bonne reponse : le gardien fait un petit geste de joie (Compti se roule en boule de bonheur, Lexia bat des ailes)
- Mauvaise reponse : le gardien fait un geste d'encouragement (Somma serre le poing, Verba penche la tete)
- Le gardien peut donner un indice specifique a son domaine

---

## D. CP comme niveau prioritaire — implications

### D.1 Le defi fondamental du CP

Un enfant de CP (6 ans) en debut d'annee **ne sait generalement pas lire**. C'est la contrainte la plus forte du projet. Toute l'interface doit pouvoir fonctionner sans que l'enfant ne lise un seul mot.

**Implications concretes :**

| Aspect | Implication pour CP |
|--------|-------------------|
| **Consignes** | 100% audio obligatoire. Le texte est present mais pour les enfants qui commencent a lire. |
| **Navigation** | 100% iconique. Pas de menus textuels. Gros boutons avec pictos. |
| **Exercices de francais** | Ce sont des exercices d'APPRENTISSAGE de la lecture, pas des exercices QUI NECESSITENT la lecture. |
| **Exercices de maths** | Representations visuelles obligatoires. Des doigts, des points, des barres, des objets. Pas de "lis le probleme". |
| **Temps de session** | 10-15 minutes maximum. L'attention a 6 ans est courte. |
| **Feedback** | Audio + visuel. Jamais de texte seul. |
| **Mascotte** | Tres presente, tres expressive, beaucoup de voix. |
| **Construction** | L'enfant glisse-depose des images, pas de texte. |

### D.2 Exercices de francais pour CP

En CP, le francais c'est avant tout **apprendre a lire**. Les exercices doivent suivre la progression phonologique classique :

**Periode 1 (septembre-octobre) : Les voyelles et premieres consonnes**

Exercices types :
- **Ecoute et montre** : "Ecoute le son [a]. Montre l'image ou tu entends [a]." (images : arbre, bus, chat -> reponse : arbre, chat)
- **Reconnaitre une lettre** : "Trouve toutes les lettres A dans ce groupe." (affichage de lettres melangees, l'enfant tapote les A)
- **Associer son et lettre** : "Ecoute [m]. Quelle lettre fait ce son ?" (3 lettres proposees en grand)
- **Combiner** : "Que font M et A ensemble ?" (ecouter "ma", "mi", "mo" et associer)

**Periode 2-3 (novembre-fevrier) : Syllabes et mots simples**

Exercices types :
- **Syllabes** : "Ecoute : PA-PA. Combien de syllabes ?" (l'enfant tapote 1, 2 ou 3 mains qui frappent)
- **Recomposer** : "Mets les syllabes dans l'ordre pour faire le mot." (images de syllabes a glisser-deposer, avec le son qui se joue au toucher)
- **Premier mot** : "Quel mot est ecrit ?" (image + mot + audio, l'enfant associe)
- **Ecriture** : "Trace la lettre M" (tracage au doigt/souris sur un modele)

**Periode 4-5 (mars-juin) : Phrases simples**

Exercices types :
- **Remettre les mots** : "Mets les mots dans l'ordre pour faire une phrase." (images-mots a ordonner, audio de chaque mot)
- **Lire et associer** : "Lis la phrase et choisis la bonne image." (phrase courte + 3 images)
- **Majuscule et point** : "Ou faut-il la majuscule ? Ou faut-il le point ?" (phrase affichee, l'enfant tapote)

### D.3 Exercices de mathematiques pour CP

**Periode 1 : Les nombres de 0 a 10**

Exercices types :
- **Compter des objets** : "Combien de pommes ?" (image avec 3-7 pommes, l'enfant choisit le chiffre)
- **Associer quantite et chiffre** : "Montre le 5." (5 doigts, 5 etoiles, le chiffre 5 — l'enfant relie)
- **Plus grand / plus petit** : "Qui a plus de bonbons ?" (deux tas visuels, l'enfant choisit)
- **Suite de nombres** : "Quel nombre vient apres 3 ?" (frise numerique visuelle avec un trou)

**Periode 2-3 : Additions et soustractions simples (< 10)**

Exercices types :
- **Addition visuelle** : "2 pommes + 3 pommes = ?" (animation de pommes qui se regroupent)
- **Calcul avec les doigts** : "Montre 2 + 1 avec tes doigts" (l'enfant voit des mains et complete)
- **Soustraction** : "Il y avait 5 oiseaux, 2 sont partis. Combien restent ?" (animation)
- **Droite numerique** : "Deplace le curseur de 3 cases vers la droite" (manipulation directe)

**Periode 4-5 : Nombres jusqu'a 100, geometrie de base**

Exercices types :
- **Compter en dizaines** : "Compte par paquets de 10" (jetons regroupes)
- **Formes** : "Trouve tous les carres" (formes melangees, l'enfant tapote les carres)
- **Symetrie** : "Complete le dessin de l'autre cote" (quadrillage avec symetrie)

### D.4 Interface CP specifique

```
+-----------------------------------------------------------+
|  [Cabane]  [Etoiles: 24]              [Pause] [Cadenas]   |
+-----------------------------------------------------------+
|                                                             |
|  Miyu-sensei : (bulle audio avec picto haut-parleur)        |
|  "Ecoute bien ! Combien de pommes vois-tu ?"               |
|                                                             |
|  +-----------------------------------------------------+   |
|  |                                                       |   |
|  |     (image : 4 pommes rouges bien espacees)           |   |
|  |                                                       |   |
|  +-----------------------------------------------------+   |
|                                                             |
|  +------+  +------+  +------+  +------+                    |
|  |  3   |  |  4   |  |  5   |  |  6   |                    |
|  | ooo  |  | oooo |  |ooooo |  |oooooo|                    |
|  +------+  +------+  +------+  +------+                    |
|                                                             |
|  [Miyu-sensei assis dans un coin, souriant]                 |
|  [Compti le herisson a cote, attentif]                      |
+-----------------------------------------------------------+
```

Points cles de cette interface :
- Les choix de reponse montrent le chiffre ET la representation en points
- L'audio joue automatiquement la consigne
- Les boutons sont tres grands (80px minimum pour CP)
- Miyu-sensei et le gardien sont visibles mais ne genent pas
- Un seul exercice a l'ecran, pas de distraction

### D.5 Comment le prototype CP sert de base

Le CP est le niveau le plus contraint. Si on le reussit, les autres niveaux sont plus faciles :

```
CP (base)
  |-- Audio obligatoire → en option pour CE1+
  |-- Navigation iconique → on ajoute du texte pour CE1+
  |-- Boutons 80px → on reduit a 64px pour CE2+
  |-- Exercices phonologiques → on ajoute la comprehension pour CE1+
  |-- Representations visuelles des nombres → on ajoute le symbolique pour CE1+
```

Le moteur d'exercices construit pour CP est un superset : tout ce qui fonctionne pour CP fonctionne pour les autres niveaux. Les niveaux superieurs ajoutent de la complexite, pas la retirent.

---

## E. Accessibilite DYS dans le MVP

### E.1 Types de DYS couverts

| Trouble | Prevalence | Impact sur JayEcole | Adaptations |
|---------|-----------|--------------------|----|
| **Dyslexie** | 5-10% des enfants | Difficulte a lire, confondre des lettres | Police OpenDyslexic, interlignage, audio |
| **Dysorthographie** | Souvent associee a la dyslexie | Difficulte a ecrire correctement | Exercices avec support visuel, pas de dictee classique |
| **Dyscalculie** | 3-7% des enfants | Difficulte avec les nombres et le calcul | Representations visuelles, manipulations, pas de calcul mental pur |
| **Dyspraxie** | 5-6% des enfants | Difficulte de coordination motrice | Zones de clic larges, pas de tracage precis, drag-drop avec snapping genereux |
| **TDAH** | 5% des enfants | Difficulte de concentration | Sessions courtes, pauses frequentes, feedback immediat (deja prevu) |

### E.2 Parametres d'affichage (menu parent)

```
Parametres d'accessibilite
  |
  |-- Police
  |     |-- Standard (Gentium Book Plus)
  |     |-- OpenDyslexic
  |     |-- Source Sans (tres lisible)
  |
  |-- Taille de police
  |     |-- Normale (18px)
  |     |-- Grande (22px)
  |     |-- Tres grande (26px)
  |
  |-- Interlignage
  |     |-- Normal (1.5)
  |     |-- Aere (2.0)
  |     |-- Tres aere (2.5)
  |
  |-- Espacement des mots
  |     |-- Normal
  |     |-- Large
  |
  |-- Mode dyscalculie
  |     |-- Desactive
  |     |-- Active : toujours montrer les representations visuelles des nombres
  |
  |-- Mode daltonien
  |     |-- Desactive
  |     |-- Active : ajout de patterns/formes en plus des couleurs
  |
  |-- Audio des consignes
  |     |-- Automatique (joue a chaque exercice)
  |     |-- Sur demande (bouton haut-parleur)
  |     |-- Desactive
  |
  |-- Zone de clic elargie
  |     |-- Normale (48px)
  |     |-- Elargie (64px)
  |     |-- Tres elargie (80px)
```

### E.3 Exercices alternatifs DYS

Certains types d'exercices sont difficiles pour les enfants DYS. Le systeme propose des variantes :

**Pour la dyslexie :**

| Exercice standard | Variante dyslexie |
|-------------------|-------------------|
| Texte a trous (ecriture) | QCM avec le mot a choisir (pas d'ecriture) |
| Lire un texte et repondre | Ecouter le texte lu a haute voix et repondre |
| Trouver l'erreur dans un mot | Le mot est lu a haute voix, l'enfant entend l'erreur |
| Classer des mots ecrits | Classer des mots lus par audio avec icones |

**Pour la dyscalculie :**

| Exercice standard | Variante dyscalculie |
|-------------------|---------------------|
| Calcul mental pur (texte) | Calcul avec representation en barres/points |
| "Quel est le resultat de 7 + 5 ?" | Animation : 7 pommes + 5 pommes se regroupent, compter |
| Comparer 347 et 374 | Barres de dizaines, unites et centaines visuelles |
| Tables de multiplication | Tables avec grille visuelle (tableau de multiplication colore) |

### E.4 Impact sur le schema de donnees

Ajout d'un champ optionnel dans le schema d'exercice :

```json
{
  "id": "fr-cp-lect-001",
  "content": { ... },
  "dys_variants": {
    "dyslexia": {
      "type": "multiple_choice",
      "instruction_override": null,
      "audio_required": true,
      "body_override": {
        "type": "multiple_choice",
        "question_audio": "assets/audio/exercises/fr-cp-lect-001-q.ogg",
        "choices": [
          {"id": "a", "text": "chat", "audio": "assets/audio/words/chat.ogg"},
          {"id": "b", "text": "chien", "audio": "assets/audio/words/chien.ogg"}
        ],
        "correct_id": "a"
      }
    },
    "dyscalculia": null
  }
}
```

Quand le parent active le mode DYS :
1. Le systeme verifie si `dys_variants.{type}` existe pour l'exercice
2. Si oui : utilise la variante
3. Si non : utilise l'exercice standard avec les parametres d'affichage DYS (police, taille, espacement, audio)

### E.5 Impact sur la taille du contenu MVP

- Exercices avec variante DYS : +30% de travail de creation (tous les exercices ne necessitent pas une variante)
- Audio supplementaire pour les variantes : +20-30 MB
- Total estime avec DYS : **80-140 MB** (contre 60-110 MB sans DYS)

L'estimation d'exercices passe de 800 a **800 exercices + ~250 variantes DYS** pour les exercices les plus impactes (lecture, calcul mental).

---

## F. Service separe — implications techniques

### F.1 Architecture binaire

JayEcole est un binaire Dioxus independant, pas un module de Miyukini Central. Cela signifie :

```
Binaire Central :
  apps/central/           -> miyukini-central.exe
  apps/central/src/main.rs

Binaire JayEcole :
  apps/jayecole/          -> jayecole.exe
  apps/jayecole/src/main.rs
  apps/jayecole/Cargo.toml
```

### F.2 Membre du workspace

JayEcole doit etre un **membre du workspace Cargo** (pas un projet separe) :

Justification :
- Partage des Cores (KindMother, StrongFather, etc.) — memes crates
- Partage des toolkits (MiyuNotify, MiyuValidate, etc.)
- Coherence de compilation (meme toolchain, memes flags Clippy)
- Un seul `cargo build --workspace` pour tout construire

Ajout dans le `Cargo.toml` racine :
```toml
[workspace]
members = [
    # ... existants ...
    "apps/jayecole",
]
```

### F.3 Base de donnees

**Option retenue : DB KindMother partagee avec Central**

Justification :
- KindMother est le Core de persistance — il gere une seule DB locale
- Les profils parents sont les profils Central (meme utilisateur)
- Le PIN parental est une extension du profil Central (StrongFather)
- Pas de duplication de donnees

JayEcole accede a la meme DB KindMother via le meme chemin que Central :
```
~/.miyukini/kindmother.db
  |-- tables Central (profils, services, etc.)
  |-- tables JayEcole (child_profiles, skill_levels, sessions, etc.)
```

Les tables JayEcole sont prefixees `je_` pour eviter les conflits :
```sql
CREATE TABLE je_child_profiles (...);
CREATE TABLE je_skill_levels (...);
CREATE TABLE je_sessions (...);
CREATE TABLE je_exercise_results (...);
CREATE TABLE je_rewards (...);
CREATE TABLE je_parent_settings (...);
```

### F.4 Cores partages

Les Cores sont des singletons locaux (LOI-7 : immuables). JayEcole les utilise de la meme maniere que Central :

```
JayEcole utilise :
  KindMother      -> Meme instance que Central (meme DB)
  StrongFather    -> Meme instance (autorisations, PIN)
  CaringNanny     -> Propre contexte d'observation (metriques JayEcole)
  MasterButler    -> Orchestration des toolkits JayEcole
  WorrySentinel   -> Propres limites (temps d'ecran)
  EverBuddy       -> Versioning des profils enfants
```

Chaque Core expose une API Rust (traits). JayEcole les importe comme dependances Cargo :
```toml
[dependencies]
kindmother = { path = "../../crates/kindmother" }
strongfather = { path = "../../crates/strongfather" }
# etc.
```

### F.5 Lancement

**Deux modes de lancement :**

1. **Depuis le bureau** : raccourci/icone JayEcole qui lance directement `jayecole.exe`
2. **Depuis Central** : bouton dans la page d'accueil des services qui lance `jayecole.exe` en processus separe

Le lancement depuis Central pourrait utiliser un simple `Command::new("jayecole")` ou passer par le systeme d'ouverture de l'OS.

### F.6 Structure du Cargo.toml

```toml
[package]
name = "jayecole"
version = "0.1.0"
edition = "2021"

[lints.clippy]
all = "warn"
pedantic = "warn"

[lints.rust]
unsafe_code = "forbid"

[dependencies]
dioxus = { version = "0.6", features = ["desktop"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }

# Cores Miyukini
kindmother = { path = "../../crates/kindmother" }
strongfather = { path = "../../crates/strongfather" }
caringnanny = { path = "../../crates/caringnanny" }

# Toolkits Miyukini (partages)
miyuvalidate = { path = "../../crates/miyuvalidate" }
miyunotify = { path = "../../crates/miyunotify" }

# Toolkits JayEcole (nouveaux)
miyuquiz = { path = "../../crates/miyuquiz" }
miyuscreenguard = { path = "../../crates/miyuscreenguard" }
```

### F.7 Distribution

JayEcole doit etre distribue avec Central (meme installeur) ou separement :

- **Option A** : inclus dans l'installeur MiyukiniCentral (un installeur, deux executables)
- **Option B** : installeur separe JayEcole
- **Option C** : portable (dossier zip sans installation)

A decider par l'utilisateur. L'option A est la plus simple pour l'utilisateur final.

---

## G. Audio dans le MVP

### G.1 Types d'audio necessaires

| Type | Volume estime | Exemples |
|------|--------------|---------|
| **Consignes d'exercices** | ~800 fichiers (1 par exercice) | "Combien de pommes vois-tu ?" |
| **Voix Miyu-sensei** | ~150 phrases | "Bravo !", "On va revoir ca.", "A demain !" |
| **Voix gardiens** | ~50 phrases (10 par gardien) | "Comptons ensemble !", "Ecoute bien ce mot." |
| **Mots individuels** | ~300-500 mots | "chat", "maison", "sept", syllabes |
| **Effets sonores** | ~20-30 sons | Clic, etoile, construction, erreur, bravo |
| **TOTAL** | ~1300-1500 fichiers | |

### G.2 TTS vs voix humaine

| Critere | TTS locale | Voix humaine enregistree |
|---------|-----------|-------------------------|
| **Cout** | Gratuit (moteurs open-source) | 1000-5000 EUR (studio + comedien) |
| **Qualite** | Correcte, voix synthetique reconnaissable | Naturelle, chaleureuse, engageante |
| **Adaptabilite** | Nouvelle phrase = nouveau rendu instantane | Nouvelle phrase = retour en studio |
| **Personnalite** | Limitee | Forte (une vraie voix pour Miyu-sensei) |
| **Taille fichiers** | Genere a la volee = 0 MB | Pre-enregistre = 50-100 MB |
| **Complexite technique** | Integrer un moteur TTS Rust | Lire des fichiers audio |
| **Impact enfant** | Les enfants detectent facilement une voix TTS | Voix humaine cree un lien emotionnel |
| **LOI-1 conformite** | TTS locale = pas de dependance | Fichiers embarques = pas de dependance |

**Proposition hybride :**
- **MVP** : TTS locale pour les consignes d'exercices (volume trop important pour l'enregistrement)
- **Miyu-sensei** : si budget disponible, voix humaine enregistree pour les ~150 phrases de la mascotte (c'est le lien emotionnel le plus important)
- **Gardiens** : TTS avec parametres de voix differents par gardien
- **Effets sonores** : fichiers audio (banques libres de droits)

**Moteurs TTS locaux envisageables :**
- **piper-rs** : moteur TTS rapide, qualite correcte, modeles francais disponibles
- **espeak-ng** : leger mais qualite faible
- **Coqui TTS** : haute qualite mais plus lourd

### G.3 Format et taille

- Format recommande : **OGG Opus** (bon ratio qualite/taille)
- Bitrate : 48 kbps (suffisant pour la voix)
- Fichier moyen : ~30 KB pour 5 secondes de voix
- Total estime si tout pre-enregistre : ~40-50 MB
- Total si TTS + quelques fichiers pre-enregistres : ~10-15 MB + taille du modele TTS (~20-50 MB)

### G.4 Architecture audio

```
assets/jayecole/audio/
  tts_model/                    -- Modele TTS (si option TTS retenue)
    fr_female_medium.onnx       -- Modele ONNX pour inference locale
    fr_female_medium.json       -- Config du modele
  mascot/                       -- Voix Miyu-sensei (pre-enregistrees si budget)
    welcome.ogg
    bravo_01.ogg
    bravo_02.ogg
    ...
  guardians/                    -- Voix gardiens (TTS ou pre-enregistrees)
    compti/
    lexia/
    ...
  effects/                      -- Effets sonores
    click.ogg
    star.ogg
    construction.ogg
    error_soft.ogg
    ...
  words/                        -- Mots individuels (pour exercices de lecture CP)
    chat.ogg
    maison.ogg
    ...
```

---

## H. Contenu from scratch — strategie de creation

### H.1 Plan de production des 800 exercices

**Approche : par niveau et par matiere, en commencant par le CP**

| Priorite | Niveau | Matiere | Exercices a creer | Variantes DYS |
|----------|--------|---------|-------------------|--------------|
| 1 (prototype) | CP | Francais | 80 | 25 |
| 1 (prototype) | CP | Maths | 80 | 25 |
| 2 | CE1 | Francais | 80 | 20 |
| 2 | CE1 | Maths | 80 | 20 |
| 3 | CE2 | Francais | 80 | 15 |
| 3 | CE2 | Maths | 80 | 15 |
| 4 | CM1 | Francais | 80 | 15 |
| 4 | CM1 | Maths | 80 | 15 |
| 5 | CM2 | Francais | 80 | 10 |
| 5 | CM2 | Maths | 80 | 10 |
| **TOTAL** | | | **800** | **~170** |

### H.2 Processus de creation d'un exercice

```
1. SPECIFICATION (Maria/Arianne)
   - Identifier la competence cible dans l'arbre
   - Definir le type d'exercice
   - Definir la difficulte (facile/moyen/difficile)
   - Ecrire la specification en langage naturel

2. REDACTION (Arianne + institutrice)
   - Rediger le contenu de l'exercice (question, reponses, distracteurs)
   - Rediger l'explication (bonne reponse, mauvaise reponse)
   - Rediger l'indice
   - Specifier l'illustration necessaire (si applicable)

3. VALIDATION PEDAGOGIQUE (institutrice)
   - L'exercice est-il conforme au programme EN ?
   - La difficulte est-elle adaptee au niveau ?
   - Les distracteurs sont-ils plausibles mais clairement faux ?
   - L'explication est-elle correcte et adaptee a l'age ?
   - Le vocabulaire est-il adapte au niveau ?

4. ENCODAGE JSON (Francois/Arianne)
   - Encoder l'exercice au format JSON normalise
   - Verifier la validite du JSON
   - Associer les skill_ids
   - Creer la variante DYS si necessaire

5. INTEGRATION ASSETS (Lise)
   - Creer ou sourcer l'illustration (SVG)
   - Generer ou enregistrer l'audio de la consigne
   - Integrer dans l'arborescence assets

6. TEST (George)
   - Verifier l'affichage dans Dioxus
   - Verifier la validation de la reponse
   - Verifier l'audio
   - Tester avec les parametres DYS actives
```

### H.3 Implication de l'institutrice

**Niveau d'implication propose : validation structuree**

L'institutrice n'ecrit pas les exercices elle-meme (trop chronophage), mais elle :

1. **Valide la structure** de l'arbre de competences par niveau
   - "Oui, c'est bien ca qu'on apprend en CP en periode 2"
   - Effort : 1-2 heures par niveau (total : 5-10 heures)

2. **Valide un echantillon** d'exercices par domaine
   - 5-10 exercices par domaine et par niveau
   - "Cet exercice est bien calibre" / "Celui-la est trop dur pour un CE1"
   - Effort : 2-3 heures par niveau (total : 10-15 heures)

3. **Repond aux questions** de l'equipe pendant la creation
   - "Comment les instits introduisent les fractions en CM1 ?"
   - Disponibilite ponctuelle, pas d'engagement regulier

**Total estime : 15-25 heures** reparties sur la duree du projet.

### H.4 Faut-il un outil interne de creation d'exercices ?

**Reponse : pas pour le MVP, mais a prevoir pour v1.0**

Pour le MVP (800 exercices) :
- Les exercices sont ecrits en JSON directement (ou via un editeur JSON quelconque)
- Un script de validation verifie le format (`cargo test` avec validation du schema JSON)
- C'est suffisant pour 800 exercices

Pour v1.0 (multiplication du contenu) :
- Un outil GUI serait utile (formulaire web ou Dioxus)
- Generation automatique de variantes (parametriques)
- Preview de l'exercice avant publication
- Cet outil pourrait etre un service Miyukini interne

### H.5 Estimation de temps pour 800 exercices

| Etape | Temps par exercice | Total pour 800 |
|-------|-------------------|---------------|
| Specification | 3 min | 40 h |
| Redaction + distracteurs | 8 min | 107 h |
| Validation pedagogique | 2 min (echantillon) | ~10 h |
| Encodage JSON | 5 min | 67 h |
| Integration assets (illustration + audio) | 10 min | 133 h |
| Test | 3 min | 40 h |
| **TOTAL** | **~31 min/exercice** | **~397 h (~50 jours)** |

Plus les ~170 variantes DYS : ~85 h (~11 jours)

**Total contenu : ~60 jours-homme** pour 800 exercices + 170 variantes DYS.

C'est coherent avec l'estimation pessimiste du brainstorm initial (60 j pour le contenu). Le poste contenu reste le plus lourd du projet.

**Strategie d'acceleration :**
- Templates parametriques pour les exercices de calcul mental (generer 50 variantes de "X + Y = ?" automatiquement)
- Reutilisation d'illustrations entre exercices similaires
- Generateur de distracteurs automatique pour les QCM
- Estimation apres acceleration : **35-45 jours-homme**

---

## I. Questions ouvertes — Round 3

Les decisions suivantes sont necessaires pour pouvoir rediger le Document Fondateur :

### Questions critiques (bloquantes pour le Document Fondateur)

1. **Concept de construction** : Quelle option retenir ?
   - Option A : Village
   - Option B : Navire
   - Option C : Atelier
   - Option D : Cabane dans les arbres (recommandation Maria)
   - Autre idee ?

2. **Apparence de Miyu-sensei** : Quel concept ?
   - Concept 1 : Artisan chaleureux (style Ghibli, neutre)
   - Concept 2 : Exploratrice inventrice (style anime, feminin)
   - Concept 3 : Compagnon sage (style livre d'enfant, neutre, cheveux sakura)
   - Autre idee ?

3. **Gardien grammaire** : Quel animal ?
   - Option 1 : Verba le perroquet (recommandation Maria)
   - Option 2 : Regla la tortue
   - Option 3 : Pluma le renard

### Questions importantes (non bloquantes mais a trancher bientot)

4. **Voix audio** :
   - TTS locale pour tout (plus simple, moins cher, mais moins chaleureux)
   - TTS + voix humaine pour Miyu-sensei (hybride recommande)
   - Voix humaine pour tout (plus cher, plus chaleureux)

5. **Implication institutrice** :
   - Validation ponctuelle (15-25h sur le projet) — recommandation Maria
   - Co-creation active (plus chronophage mais meilleur contenu)

6. **Format de distribution** :
   - Option A : inclus dans l'installeur MiyukiniCentral (un seul installeur)
   - Option B : installeur separe JayEcole
   - Option C : portable (zip sans installation)

### Questions pour plus tard (v1.0)

7. **Hybrid cabane + village** : Si cabane retenue, envisager l'extension village en v1.0 ?
8. **Outil de creation d'exercices** : A developper en interne pour v1.0 ?
9. **Voix humaine a long terme** : Budget pour un enregistrement pro ?

---

## J. Prochaines etapes

1. **Utilisateur** : repondre aux questions critiques du round 3 (concept construction, Miyu-sensei, gardien)
2. **Maria** : integrer les decisions round 3 et rediger le Document Fondateur
3. **Denis** : valider l'architecture binaire separe (section F) et le schema DB prefixe
4. **Lise** : commencer les maquettes de la cabane (ou du concept retenu) et de l'ecran CP
5. **Arianne** : contacter l'institutrice pour planifier les sessions de validation

---

*Document redige par Maria, Chef de Projet Miyukini AI Studio*
*Brainstorming round 3 — Pivot construction — JayEcole*
*Date : 2026-02-27*
