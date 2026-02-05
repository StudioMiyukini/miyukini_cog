# JayKonta — Points d’entrée (publics)

## Contexte

La documentation du service **JayKonta** est organisée par **point d’entrée** : **JayBudget** (perso/individuel) et **JayKonta** (entreprise). Chaque point d’entrée dispose d’un dossier dédié avec une présentation (_index.md) et une analyse des besoins.

**Un seul service COG** ; les deux points d’entrée exposent un sous-ensemble des capacités du service COG, avec des périmètres, Mandats et niveaux de sécurité distincts.

## Points d’entrée

| Point d’entrée | Nom commercial | Description | Documentation |
|----------------|----------------|-------------|---------------|
| **Purse** | JayBudget | Budgets personnels, budgets occasionnels (vacances, Noël, projets). | [Purse](./Purse/_index.md) |
| **Account** | JayKonta | Comptabilité d’entreprise, devis, facturation, rapports. | [Account](./Account/_index.md) |

## Vue d’ensemble

| Point d’entrée | Public | Périmètre | Niveau de sécurité minimal |
|----------------|--------|-----------|----------------------------|
| **JayBudget** | Particuliers, foyers | Mouvements perso, budgets occasionnels, objectifs, rapports | 2 (Sensitive) |
| **JayKonta** | Professionnels, associations, TPE/PME, organisateurs | Devis, facturation, comptabilité, rapports légaux | 2–3 (Sensitive à Critical) |

## Voir aussi

- [Document fondateur JayKonta](../JayKonta%20-%20Document%20Fondateur.md)
- [Points d’entrée Purse et Account (référence)](../reference/JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md)
