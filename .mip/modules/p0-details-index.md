# Module MIP — P0 Cadrage — Index (drill-down)

> **Charger cet index en premier.** Drill-down vers p0-details.md par temps si nécessaire.
> Référence complète : `.mip/modules/p0-details.md` (~447 lignes)

---

## TL;DR

P0 = 10 temps (T1-T10). Maria orchestre. T2+T3 parallèles, T4+T5 parallèles, T6→T7→T8+T9→T10.
Gate stricte : brief approuvé + mode autonomie (2 AskUserQuestion distincts).
Classification T3-T5 détermine allègement (saut T3, T9, etc.).

---

## Règles bornantes (R-P0)

| Règle | Contenu |
|-------|---------|
| R-P0-1 | AskUserQuestion obligatoire pour questions. 1 appel = 1 section. Pas de texte libre pour questions. |
| R-P0-2 | Après chaque temps : annoncer résumé 3-5 lignes dans le chat |
| R-P0-3 | Brief : 1) Écrire fichier 2) Présenter TL;DR+approches+risques 3) AskUserQuestion approbation 4) SI APPROUVÉ → AskUserQuestion autonomie |
| R-P0-4 | Carte sync : T1→T2+T3→T4+T5→T6→T7→T8+T9→T10 |
| R-P0-5 | Allègement par classe : T3 saute T3 concurrence, T3 saute T9 CI/CD ; T4-T5 complet |

---

## Carte de synchronisation

```
T1 (Maria, HUMAIN) -------------------------------- [gate : réponses utilisateur]
  +- T2 (Maria+Lise) --+
  +- T3 (Fabrice, T4+) -+
                         +- [sync : T2+T3 terminés]
T4 (Denis+Hugo+Jean) ---+
T5 (Victor) -------------+
                          +- [sync : T4+T5 terminés]
T6 (François) ------------+
                           +- [sync : T6 terminé]
T7 (Denis) ----------------+
                            +- [sync : T7 terminé]
T8 (Arianne+Jean) ----------+
T9 (Hugo) -------------------+
                              +- [sync : T8+T9 terminés]
T10 (Maria, brief) ----------+
                               +- [GATE P0 : brief + autonomie, HUMAIN]
```

---

## Drill-down par temps (Read p0-details.md avec offset/limit)

| Temps | Lignes approx | Contenu |
|-------|---------------|---------|
| T1 + Questionnaire | 80-155 | Exploration, brainstorming, sections 1-5 |
| T2 | 156-170 | Idéation Maria + Lise |
| T3 | 173-184 | Analyse concurrentielle Fabrice |
| T4 | 187-219 | Inventaire Denis+Hugo+Jean |
| T5 | 222-247 | Sécurité Victor |
| T6 | 250-265 | Spec François |
| T7 | 268-298 | Plan Denis |
| T8 | 302-325 | Audit Arianne+Jean |
| T9 | 328-337 | CI/CD Hugo |
| T10 | 341-447 | Synthèse Maria, brief, gates |

---

## Références

- **Complet** : `.mip/modules/p0-details.md`
- **Token-loading** : `.mip/skills/miyukini-mip-workflow/modules/token-loading.md` (source unique chargement par agent)
