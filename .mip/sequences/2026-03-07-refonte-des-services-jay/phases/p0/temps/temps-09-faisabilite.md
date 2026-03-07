# P0 Temps 9 - Audit faisabilite

## Statut

- Etat : Terminé
- Phase : P0 Temps 9
- Agents : Arianne + Jean
- Date : 2026-03-07

## TL;DR

FAISABLE AVEC RÉSERVES. Scope C5 ambitieux mais réalisable en MASS. Réserve principale : scope "Autres services Jay" (MSCM audit only en P3, corrections P3 = JayFestival+JayXpose uniquement). Recommandation Jean : sonnet-4-6 pour agents de développement.

## Critères de faisabilité

| Critère | Statut | Notes |
|---------|--------|-------|
| Ressources disponibles | OK | Crates backend stables. miyuki-ui-dioxus déjà structuré. Référence miyucloud disponible. |
| Délai raisonnable | OK (réserve) | ~59 tâches MASS. Raisonnable si blocs parallèles respectés. |
| Risques acceptables | OK (réserve) | Risque scope glissement mitigé par gate MVP stricte. |
| Dépendances résolues | OK | Context7 Dioxus 0.7 résolu. axum 0.7 déjà utilisé. PortalContract = nouveau mais simple. |
| Budget tokens | OK (estimation) | C5 MASS — budget à surveiller. Jean = gardien. |

## Réserves identifiées (Arianne)

1. **Scope protection** : MSCM corrections en P3 = JayFestival + JayXpose UNIQUEMENT. Les autres services Jay (JayKoa, JayKonta, JayManga, Jay1Tribu) = audit rapport + plan corrections pour une séquence future. Sinon surcharge P3.

2. **MVP gate** : Si E03 (JayXpose) dépassait le délai, l'itération P5 peut se faire avec E00+E01+E02+E04 (MVP sans JayXpose UI refontée — mais contrats d'exposition doivent exister pour E04).

3. **COG Web Portal** : Raisonnable avec pattern miyucloud établi. Ne pas over-engineer — HTML inline + routes axum simples.

## Recommandation modèles — Jean

| Agent | Tâche | Modèle recommandé |
|-------|-------|-------------------|
| François (back) | MSCM + hardening + Portal | claude-sonnet-4-6 |
| Lise (UI Dioxus) | Refonte composants | claude-sonnet-4-6 |
| George (audit) | Lecture + rapport | claude-haiku-4-5 |
| Victor (sécu) | Audit sécurité | claude-sonnet-4-6 |
| Denis (coordination) | Checkpoints | claude-sonnet-4-6 |
| Jean (métriques) | Comptages | claude-haiku-4-5 |

## Verdict faisabilité

**FAISABLE AVEC RÉSERVES** — MVP défini, scope protection activée, MASS validé. Go pour T11.

