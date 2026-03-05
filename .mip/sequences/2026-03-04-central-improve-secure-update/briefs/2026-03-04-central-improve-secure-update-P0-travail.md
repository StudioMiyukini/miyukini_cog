# P0 Travail — central-improve-secure-update

> Document de travail P0. Brief final en T10.
> **Changement de scope** : Audit complet COG hors MGE — inventaire, monolithiques, MSCM, conformité, réorganisation.

## Synthèse T1 — Réponses Section 1 (COMPRENDRE)

### Contexte stratégique
- **Agents MIP** : montée en compétences
- **Central** : interface des services COG manipulant des données sensibles (JayKonta, MiyuCloud)
- **Objectif** : Niveau de sécurité renforcé + capacité d’ajouter des services à sécurité critique avec chiffrement fort

### Problème / besoin (1.1)
Central doit intégrer un niveau de sécurité durci et supporter des services critiques (données sensibles) avec chiffrement fort.

### Déclencheur (1.2)
Montée en compétences des agents MIP ; Central devient le hub de services à données sensibles (compta JayKonta, stockage MiyuCloud).

### Utilisateur final (1.3)
Utilisateurs de JayKonta, MiyuCloud et futurs services à données sensibles via Central.

### Périmètre cible
- Niveau sécurité : **durci** (vs standard actuel)
- Services à sécurité critique : chiffrement fort (at-rest, en transit)
- Extensibilité : capacité d'ajouter de nouveaux services critiques

---

## Synthèse T1 — Section 2 (CADRER)

| # | Réponse |
|---|---------|
| 2.1 | Contraintes : aucune pour le moment |
| 2.2 | Périmètre : **Miyukini COG** (écosystème complet) |
| 2.3 | Priorité : **(b) complétude + (c) nice-to-have** |
| 2.4 | Échéance : **ASAP** |
| 2.5 | Ressources : rassemblées par Maria (voir ressources/) |

---

## Synthèse T1 — Section 3 (IMAGINER)

| # | Réponse |
|---|---------|
| 3.1 | Étendre WorrySentinel ; from scratch, le plus robuste possible |
| 3.2 | Patterns MiyuCloud (crypto), SecurityLevel, BorderGuard |
| 3.3–3.6 | Au plus pertinent ; recherche web + Opus pour finaliser P0 |
