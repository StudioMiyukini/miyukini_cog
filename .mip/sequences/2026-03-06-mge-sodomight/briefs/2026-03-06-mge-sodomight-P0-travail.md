# Travail P0 - mge-sodomight

## Statut

- Etat : Complete pour la gate P0
- Phase : P0
- Responsable principal : Maria

## TL;DR

Ce fichier centralise le travail preparatoire P0 et renvoie vers les modules detailles. Le coeur du contenu exhaustif est volontairement decentralise dans `ressources/requirements/` pour respecter la regle MIP des documents courts.

## Contexte et bornes

- Classe : T5
- Perimetre : moteur `mge/` autonome + jeu `Sodomight` + integration `Central`
- MVP cible : toutes les fonctionnalites systemiques de D2, avec contenu borne au camp de depart et a l'Acte 1 complet
- Contrainte additionnelle : rendu moderne/robuste/scalable + backend MMO-ready
- Interdit avant gate : tout code moteur ou jeu

## Synthese de travail par temps

- T1 : cadrage et reformulation
- T2 : vision produit et direction UX
- T3 : references D2, reverse-engineering et benchmark
- T4 : prerequis techniques, packaging et contraintes depot
- T5 : menaces et securite
- T6 : specification modulaire
- T7 : generation manifeste agents
- T8 : plan P3 par etapes
- T9 : audit faisabilite
- T10 : verification CI/CD
- T11 : brief et gate

## Cartographie documentaire

- Brief : `briefs/2026-03-06-mge-sodomight.md`
- Index besoins : `ressources/requirements/00-index.md`
- MVP scope : `ressources/requirements/18-mvp-camp-act1.md`
- Feature matrix : `ressources/requirements/19-feature-matrix-d2-mvp.md`
- Content bible Act 1 : `ressources/requirements/20-act1-content-bible.md`
- Doc stack implementation : `ressources/requirements/21-implementation-doc-stack.md`
- Render architecture : `ressources/requirements/22-render-modern-robust-scalable.md`
- Asset visual bible : `ressources/requirements/23-sodomight-asset-style-bible.md`
- MMO backend readiness : `ressources/requirements/24-mmo-backend-readiness.md`
- Spec principale : `specs/2026-03-06-mge-sodomight-spec.md`
- Plan P3 : `plans_p3/2026-03-06-mge-sodomight-plan.md`
- GPI : `gpi/2026-03-06-mge-sodomight-gpi.md`
- Sources : `ressources/sources-web.md`

## Resultat attendu si P3 est approuvee

1. workspace `mge/` cree
2. crates moteur et jeu bootstrappees
3. systems D2 jouables dans `Sodomight`
4. camp de depart + Acte 1 complets
5. package installable et lancable depuis `Central`
6. rendu original proche de D2 sans copie exacte
7. fondations backend pretes pour une evolution MMO
