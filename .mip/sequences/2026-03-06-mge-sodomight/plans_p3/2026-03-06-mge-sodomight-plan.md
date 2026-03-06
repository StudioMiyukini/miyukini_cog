# Plan P3 - mge-sodomight

## TL;DR

Execution P3 planifiee en 10 etapes avec gates techniques et documentaires.
Cible: livrer un MVP `Sodomight` avec tous les systemes majeurs de Diablo II, un contenu jouable borne au camp de depart et a l'Acte 1 complet, un rendu moderne/robuste/scalable et un backend preparable vers MMO.

## Principes de conduite

1. Le MVP signifie "parite systemique D2", pas "micro vertical slice".
2. Le contenu jouable est borne au camp de depart et a l'Acte 1, mais les systemes D2 hors Acte 1 doivent exister au niveau runtime, data et tests.
3. Chaque etape doit produire sa documentation de soutien avant ou en meme temps que le code.
4. `mge/` reste autonome; `Central` ne sert qu'a installer, lancer et superviser.
5. Le coeur simulation doit rester externalisable vers un backend autoritaire MMO.
6. Aucune etape ne doit etre executee comme un bloc monolithique: decoupage en micro-taches bornees obligatoire.

## Conditions d'entree P3

1. Le brief P0 et la spec P0 sont valides.
2. La matrice fonctionnelle D2 -> MVP est figee pour eviter le glissement de scope.
3. Le stack documentaire d'implementation est accepte comme livrable obligatoire.
4. Les conventions Rust, assets, saves et packaging sont preparees avant creation du workspace.

## Roadmap par etape

### Etape 01 - Fondation workspace et gouvernance technique

Objectif:

- Poser `mge/` comme workspace Rust autonome, compilable et maintenable.

Gate G1:

- Workspace `mge/` compilable.
- Regles lint/fmt/tests documentees.
- Carte des crates et des responsabilites redigee.

### Etape 02 - Runtime coeur, donnees et persistence

Objectif:

- Stabiliser le modele monde, les schemas data-driven et les contrats save/load.

Gate G2:

- Runtime bootable avec scene de base.
- Schemas de contenus versionnes.
- Contrat de persistence documente et teste.
- Frontieres de simulation et proto prets pour local host / dedicated.

### Etape 03 - Renderer, animation, audio et pipeline assets

Objectif:

- Livrer le socle visuel et outillage necessaires au rendu isometrique complet.

Gate G3:

- Fenetre, boucle de rendu, camera isometrique et sprite batching fonctionnels.
- Pipeline d'import/generation des assets documente.
- Placeholders et textures internes exploitables en jeu.
- Scalabilite GPU et garde-fous robustesse traces.

### Etape 04 - UI, HUD, navigation et feedback

Objectif:

- Construire toute la surface de jeu necessaire a un ARPG type D2.

Gate G4:

- HUD combat, inventaire, stash, vendors, quetes, menus et overlays disponibles.
- Input map clavier/souris et feedback audio/visuel documentes.

### Etape 05 - Stats, classes, skills et combat

Objectif:

- Implementer le coeur des regles D2: personnages, progression et combats.

Gate G5:

- Classes jouables, arbres de competences, stats, breakpoints et ressources actifs.
- Combat melee, range et cast operationnels avec formules documentees.

### Etape 06 - Items, loot, economie ville et craft

Objectif:

- Livrer la profondeur d'itemisation et les services de ville lies au loot.

Gate G6:

- Rareties, affixes, sockets, gemmes/runes/joyaux, runewords equivalents et charms actifs.
- Vendors, gamble, identify, repair, stash, cube equivalent et tables de loot documentes.

### Etape 07 - Monde, quetes, randomisation et contenu camp + Acte 1

Objectif:

- Produire le contenu jouable complet du camp de depart et de l'Acte 1.

Gate G7:

- Camp complet jouable avec ses services.
- Zones, quetes, waypoints, portails et boss final d'Acte 1 authories.
- Bible de contenu Acte 1 complete et exploitable.

### Etape 08 - Monstres, boss, mercenaire et modes meta D2

Objectif:

- Couvrir les systemes D2 transverses non limites a une seule zone.

Gate G8:

- Roster monstres/bosses Acte 1 complet avec IA et rencontres.
- Mercenaire, hardcore, party/co-op, PvP et ladder runtime/contracts implementes ou testables.
- Les features non naturellement debloquees en Acte 1 sont accessibles via harness/debug pour validation.

### Etape 09 - Integration Central, packaging et exploitation locale

Objectif:

- Installer, lancer et superviser `Sodomight` depuis `Central` sans coupler le moteur au monorepo.

Gate G9:

- Manifeste Market valide.
- Installation, update locale, lancement et gestion des saves verifies depuis `Central`.
- Documentation packaging/exploitation redigee.

### Etape 10 - Parite MVP, equilibrage, tests et freeze documentaire

Objectif:

- Clore P3 avec un build auditable et un dossier de transfert propre vers P4/P5.

Gate G10:

- Tous les systemes D2 cibles sont couverts par tests ou preuves de jouabilite.
- Camp de depart + Acte 1 completes de bout en bout.
- Matrice de tests, notes d'equilibrage et documentation finale a jour.

## Navigation

- `etapes/index.md`
- `etapes/etape-01.md`
- `etapes/etape-02.md`
- `etapes/etape-03.md`
- `etapes/etape-04.md`
- `etapes/etape-05.md`
- `etapes/etape-06.md`
- `etapes/etape-07.md`
- `etapes/etape-08.md`
- `etapes/etape-09.md`
- `etapes/etape-10.md`

## Dependances et enchainement

```text
E01 -> E02 -> E03 -> E04 -> E05 -> E06 -> E07 -> E08 -> E09 -> E10
```

Paralleles autorises:

1. Documentation de contenu Acte 1 peut avancer pendant E03 a E06.
2. Outillage assets et generation de placeholders peuvent avancer entre E02 et E04 si les formats sont figes.
3. Packaging `Central` peut etre prepare pendant E07/E08 si le contrat binaire est stable.

## Documentation obligatoire P3

1. Carte workspace et roles des crates.
2. Registre des formules gameplay.
3. Schemas data-driven et contrat save/load.
4. Architecture rendu moderne, bible assets, UI et feedback.
5. Bible contenu camp + Acte 1.
6. Roster monstres / IA / boss / mercenaire.
7. Contrat ladder / hardcore / party / PvP + trajectoire MMO-ready.
8. Contrat packaging / installation / exploitation.
9. Matrice de tests MVP et dossier d'equilibrage.

## Definition de fini P3

- `mge/` compilable
- tous les systemes D2 majeurs implementes ou verifies par harness/tests
- `sodomight` jouable du camp de depart au boss final d'Acte 1
- rendu robuste avec tiers qualite et pipeline assets original documentes
- backend externalisable vers dedicated/MMO sans reecriture de la simulation coeur
- package installable depuis `Central`
- stack documentaire d'implementation complet et maintenu
- smoke tests, tests coeur et tests de contenu verts
