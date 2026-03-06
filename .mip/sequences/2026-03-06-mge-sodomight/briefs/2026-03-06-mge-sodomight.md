# Brief - MGE / Sodomight

## TL;DR

Construire un moteur ARPG Rust autonome dans `mge/` pour produire `Sodomight`, copie systemique de Diablo II, avec rendu proprietaire, pipeline assets interne, packaging Market et lancement depuis `Central`. Le MVP vise explicitement toutes les fonctionnalites systemiques de D2, mais un contenu borne au camp de depart et a l'Acte 1 complet. Le rendu doit etre moderne, robuste et scalable tout en gardant une lisibilite D2-like; le backend doit etre prepare des P0/P3 a evoluer vers une version MMO autoritaire sans reecriture du coeur gameplay. Le risque principal est l'ampleur du scope; la parade est un decoupage strict par etapes techniques, documentaires et content pipeline. Aucun code n'est lance avant validation de ce brief.

## Metadonnees

- Classe : T5
- Depot : `Miyukini-COG`
- Workspace cible : `mge/`
- Mode d'execution vise : service `Standalone` lance depuis `Central`

## Objectifs

- documenter exhaustivement les besoins D2 -> Sodomight
- specifier un moteur MGE guide par ces besoins
- expliciter un MVP "systems complete, content scoped" : camp de depart + Acte 1 complet
- expliciter un rendu proprietaire moderne et une pipeline assets originale inspiree de D2 sans copie exacte
- verrouiller une architecture backend MMO-ready meme si le produit livre reste un ARPG solo/coop
- preparer une execution P3 sans deriver du protocole MIP
- verrouiller l'integration Central/Market des le P0

## Approches comparees

### A. Moteur custom ARPG sur briques low-level Rust

- plus de controle
- meilleur alignement avec la demande utilisateur
- effort plus eleve

### B. Jeu d'abord, moteur extrait plus tard

- plus rapide au debut
- dette architecturale elevee
- risque de casser l'independance `mge/`

### C. Engine tiers complet puis adaptation

- plus rapide pour un prototype
- contraire a la demande de rendu entierement construit
- risque d'un socle surdimensionne

## Recommandation

Retenir A. Construire un moteur custom specialise ARPG a base `wgpu`/`winit`, avec un MVP qui livre les systemes D2 des le premier palier et limite le contenu jouable au camp de depart et a l'Acte 1 complet. Cette approche respecte le besoin de controle, l'independance du dossier `mge/`, la scalabilite et l'integration Central.

## Livrables P0

- sequence `.mip/sequences/2026-03-06-mge-sodomight/`
- documentation modulaire `ressources/requirements/`
- spec modulaire `specs/`
- plan P3 `plans_p3/`
- GPI securite `gpi/`

## Risques

| Risque | Impact | Reponse |
|--------|--------|---------|
| Scope trop vaste | critique | decoupage strict par systemes + contenu borne camp/Acte 1 + docs de soutien obligatoires |
| Rendu proprietaire trop lent a converger | fort | pipeline sprites simple, pas de 3D cachee |
| Couplage `Central` / jeu | fort | manifeste + process standalone uniquement |
| Duplication documentaire avec Allumina | moyen | index de sequence unique et references croisees |
| Assets trop volumineux trop tot | fort | placeholders internes et asset baker des P3 |
| MVP "toutes features D2" trop ambigu | critique | borne explicite: systems complets, contenu limite au camp + Acte 1 |
| Rendu trop proche de D2 visuellement | fort | bible visuelle originale, aucune reprise directe d'assets ou tracing |
| Backend coop impossible a faire evoluer en MMO | critique | simulation autoritaire partagee, protocoles versionnes, service boundaries des P3 |

## Carte documentaire

- Besoins gameplay : `ressources/requirements/01-08`, `12-24`
- Besoins moteur : `ressources/requirements/09-11`, `22-24`
- Spec : `specs/`
- Plan : `plans_p3/`

## Decision P0

Le projet est faisable et coherent si la premiere execution livre:

1. un workspace `mge/` compilable
2. un renderer isometrique jouable
3. toutes les fonctionnalites systemiques D2 necessaires au gameplay cible
4. le camp de depart et l'Acte 1 complet jouables de bout en bout
5. un package installable et lancable depuis `Central`
6. un rendu original D2-like sans copie exacte d'assets tiers
7. un backend capable d'externaliser la simulation vers un mode MMO futur

## Gate

Approuvee par l'utilisateur le 2026-03-06. Execution P3 lancee en mode FULL autopilot.
