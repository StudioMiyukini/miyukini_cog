# JayBudget - Besoins en Operateurs et Toolkits (point d'entree Purse)

## Contexte

Ce document mappe les besoins Purse vers operateurs et toolkits executables.
Il constitue la reference de design fonctionnel Purse.

## Operateurs cibles

### OP-PURSE-CORE

- role : point d'entree fonctionnel Purse
- couvre : auth contexte, dashboard, mouvements, categories, rapports
- contrats : CK-SVC-01, CK-SVC-02, CK-OP-01, CK-OP-02

### OP-PURSE-PLANNING

- role : budgets occasionnels, objectifs, alertes
- couvre : create/assign/close budget, progression objectifs
- contrats : CK-OP-03, CK-TK-61

### OP-PURSE-REMINDER (optionnel)

- role : publication rappels vers JayKoa
- couvre : echeances objectifs/budgets
- contrats : CK-INT-03

## Toolkits cibles

### TK-AUTH-CONTEXT

- operations : auth et resolution contexte Purse
- dependances : Miyauth, Master Butler

### TK-LEDGER

- operations : record/list movements, categories
- dependances : KindMother, WorrySentinel

### TK-BUDGET-PURSE

- operations : create budget occasionnel, assign movement, close budget
- dependances : KindMother

### TK-GOALS-PURSE

- operations : create goal, progress, threshold checks
- dependances : KindMother, Miyunotify

### TK-REPORTING

- operations : dashboard/synthesis/export personnel
- dependances : KindMother

### TK-REMINDERS (optionnel)

- operations : publish deadlines to JayKoa
- contraintes : reference temporelle seulement

## Matrice besoin vers operateur/toolkit

| Besoin | Operateur | Toolkit |
|--------|-----------|---------|
| PUR-01 PUR-03 | OP-PURSE-CORE | TK-AUTH-CONTEXT |
| PUR-04 PUR-07 | OP-PURSE-CORE | TK-LEDGER |
| PUR-08 PUR-10 | OP-PURSE-PLANNING | TK-BUDGET-PURSE |
| PUR-11 PUR-12 | OP-PURSE-PLANNING | TK-GOALS-PURSE |
| PUR-13 PUR-14 | OP-PURSE-CORE | TK-REPORTING |
| PUR-15 | OP-PURSE-PLANNING | TK-GOALS-PURSE |
| PUR-16 | OP-PURSE-REMINDER | TK-REMINDERS |

## Regles d'equipe operateurs

- REG-PUR-1 : OP-PURSE-CORE ne traite jamais de devis/factures
- REG-PUR-2 : OP-PURSE-PLANNING n'ecrit qu'en contexte Purse
- REG-PUR-3 : OP-PURSE-REMINDER ne transporte pas de donnees financieres
- REG-PUR-4 : ecritures sensibles auditees

## Donnees et niveaux

| Classe | Niveau | Exemples |
|--------|--------|----------|
| Purse standard | 2 | mouvements, categories, budgets, objectifs |

## Tests attendus

- tests unitaires TK-LEDGER
- tests unitaires TK-BUDGET-PURSE/TK-GOALS-PURSE
- tests export scope utilisateur
- tests contrats optionnels CK-INT-03

## References

- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/publics/Purse/Purse - Analyse des besoins.md`
- `docs/services/JayKonta/publics/Purse/Purse - Parcours Capacites Livrables.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Operateurs/toolkits enrichis
