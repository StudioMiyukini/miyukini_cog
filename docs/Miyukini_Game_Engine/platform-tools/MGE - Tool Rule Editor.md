# MGE — Tool Rule Editor

## Contexte

Outil d'écriture des règles gameplay en format déclaratif. Le moteur lit ces règles via un plugin rule-engine. L'IA peut générer, simplifier, détecter conflits.

## Portée / Scope

- **Applicable à :** Règles combat, morale, économie Allumina.
- **Statut :** Spécification.

---

## Rôle

Écrire les règles gameplay en format déclaratif.

## Exemple

```
If morale < 30 → state = routing
If HP < 10% → retreat
If in_formation → bonus = flank_multiplier
```

Le moteur lit ces règles via plugin **rule-engine**.

## IA peut

- Générer règles à partir de descriptions
- Proposer simplifications
- Détecter conflits entre règles

## Règles

- Format déclaratif (pas de code)
- Export vers Export Pipeline
- Le plugin rule-engine interprète au runtime

---

**Référence** : [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md)
