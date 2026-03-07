# Module MIP - P0 Cadrage - Index (drill-down)

> **Charger cet index en premier.** Drill-down vers `p0-details.md` par temps si necessaire.
> Reference complete : `.mip/modules/p0-details.md`

---

## TL;DR

P0 = 11 temps (T1-T11). Maria orchestre.
T2+T3 paralleles, T4+T5 paralleles, T6 -> T7 (agents fine-tuned) -> T8 -> T9+T10 -> T11.
Gate stricte : brief approuve + mode autonomie (2 AskUserQuestion distincts).
Classification T3-T5 determine allegement (saut T3, T10, etc.).

---

## Regles bornantes (R-P0)

| Regle | Contenu |
|-------|---------|
| R-P0-1 | AskUserQuestion obligatoire pour questions. 1 appel = 1 section. Pas de texte libre pour questions. |
| R-P0-2 | Apres chaque temps : annoncer resume 3-5 lignes dans le chat |
| R-P0-3 | Brief : 1) Ecrire fichier 2) Presenter TL;DR+approches+risques 3) AskUserQuestion approbation 4) SI APPROUVE -> AskUserQuestion autonomie |
| R-P0-4 | Carte sync : T1->T2+T3->T4+T5->T6->T7->T8->T9+T10->T11 |
| R-P0-5 | Allegement par classe : T3 saute T3 concurrence, T3 saute T10 CI/CD ; T4-T5 complet |
| R-P0-6 | Temps 5 produit un RPS obligatoire + mise a jour ressources/index + volet securite dans gpi |
| R-P0-7 | Temps 7 genere les prompts agents fine-tuned de sequence dans `<sequence>/agents/` |

---

## Carte de synchronisation

```text
T1 (Maria, HUMAIN) -------------------------------- [gate : reponses utilisateur]
  +- T2 (Maria+Lise) --+
  +- T3 (Fabrice, T4+) -+
                         +- [sync : T2+T3 termines]
T4 (Denis+Hugo+Jean) ---+
T5 (Victor) -------------+
                          +- [sync : T4+T5 termines]
T6 (Francois) ------------+
                           +- [sync : T6 termine]
T7 (Maria, agents fine-tuned) -+
                                +- [sync : T7 termine]
T8 (Denis, plan) ---------------+
                                 +- [sync : T8 termine]
T9 (Arianne+Jean) --------------+
T10 (Hugo) ----------------------+
                                  +- [sync : T9+T10 termines]
T11 (Maria, brief) --------------+
                                   +- [GATE P0 : brief + autonomie, HUMAIN]
```

---

## Drill-down par temps (Read p0-details.md avec offset/limit)

> Offsets approximatifs — peuvent varier si le fichier a ete edite. Ajuster de ±10 si la section n'apparait pas.

| Temps | Contenu | Offset approx | Limit approx |
|-------|---------|:-------------:|:------------:|
| T1 + Questionnaire | Exploration, brainstorming, sections 0-5 (section 0 = orientation pre-questionnaire) | 88 | 75 |
| T2 | Ideation Maria + Lise | 163 | 20 |
| T3 | Analyse concurrentielle Fabrice | 183 | 15 |
| T4 | Inventaire Denis+Hugo+Jean | 198 | 40 |
| T5 | Securite Victor + RPS + volet GPI securite | 238 | 50 |
| T6 | Spec Francois | 288 | 20 |
| T7 | Generation agents fine-tuned de sequence | 308 | 25 |
| T8 | Plan Denis | 333 | 30 |
| T9 | Audit faisabilite Arianne+Jean | 363 | 25 |
| T10 | CI/CD Hugo | 388 | 15 |
| T11 | Synthese Maria, brief, gates | 403 | 100 |

---

## References

- **Complet** : `.mip/modules/p0-details.md`
- **Token-loading** : `.mip/skills/miyukini-mip-workflow/modules/token-loading.md` (source unique chargement par agent)
