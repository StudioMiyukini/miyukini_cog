# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : Termine
- Phase : P0 Temps 1
- Responsable principal : Maria
- Date : 2026-03-05

## TL;DR

Exploration du besoin "Miyukini Connect" confirmee: service d'authentification transverse, couple a Central, capable d'operer online et en isolation, avec niveaux d'authentification standardises pour piloter les permissions de session.

## Besoin utilisateur reformule

1. Authentifier l'utilisateur a l'entree de session Central.
2. Attribuer un niveau de permission selon la robustesse du moyen d'auth.
3. Imposer des facteurs forts pour donnees/actions sensibles.
4. Exposer des API que les autres services peuvent appeler.
5. Fournir une UI/UX integrable (frame, modal, ecran).
6. Interroger Origin au boot pour activer les capacites renforcees.

## Contraintes detectees

| Contrainte | Impact architecture |
|------------|---------------------|
| LOI-1 (pas de dependance externe critique) | Auth locale obligatoire meme sans internet |
| LOI-2 (isolement normal) | Mode isolation traite comme etat nominal |
| Couplage Central + Connect | Handshake de bootstrap necessaire |
| Services sensibles | Moteur de step-up obligatoire |

## Hypotheses de travail T1

1. Connect est la facade produit d'authentification.
2. L'autorisation metier finale reste gouvernee par StrongFather/MasterButler.
3. Connect publie des claims de session normalises (`aal`, `methods`, `permission_tier`).
4. Origin enrichit les capacites, mais ne doit jamais bloquer un login local legitime.

## Sorties T1

- Perimetre fonctionnel valide.
- Contraintes LOI alignees.
- Passage au Temps 2 pour ideation solutionnelle.
