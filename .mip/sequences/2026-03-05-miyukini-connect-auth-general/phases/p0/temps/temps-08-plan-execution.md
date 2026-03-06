# P0 Temps 8 - Plan execution

## Statut

- Etat : Termine
- Phase : P0 Temps 8
- Responsable principal : Denis
- Date : 2026-03-05

## TL;DR

Plan d'execution detaille produit pour P3: 5 etapes operationnelles, dependances explicites, gates securite, criteres de sortie et repartition agents. Le plan est aligne sur l'option D (local-first durci + step-up + anti-2-temps).

## Strategie d'execution

1. Livrer d'abord le chemin critique local (LOI-1/LOI-2).
2. Ajouter ensuite les facteurs forts et le step-up sensible.
3. Integrer Central et Origin sans creer de dependance externe critique.
4. Fermer par hardening + PASS securite + validation P5.

## Decoupage retenu (P3)

1. Etape 01: Foundation service Connect.
2. Etape 02: MFA forte + step-up + AAL tiers.
3. Etape 03: Couplage Central + Origin capabilities.
4. Etape 04: Isolation hardening + anti-2-temps.
5. Etape 05: Validation complete + readiness P4/P5.

## Gates obligatoires

- Gate G1 (fin etape 01): login offline + introspection fonctionnels.
- Gate G2 (fin etape 02): step-up et AAL tiers imposes.
- Gate G3 (fin etape 03): bootstrap Central/Connect stable online/offline.
- Gate G4 (fin etape 04): tests attaque 2-temps passes.
- Gate G5 (fin etape 05): PASS-0/PASS-01 atteints, dossier P4 pret.

## Repartition agentique (derivee T7)

- Francois: backend API + session/policy engine.
- Victor: securite, controles MUST, PASS.
- Lise: UI auth embed/modal/full + UX etats runtime.
- Denis: orchestration, integration, migration.
- Hugo: run/dev infra, scripts validation.
- George/Jean/Arianne: audit, efficience, validation finale.

## Decision T8

- T8 termine.
- Plan execution detaille valide.
- Passage recommande vers T9 (audit de faisabilite).
