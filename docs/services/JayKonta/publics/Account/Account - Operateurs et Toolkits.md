# JayKonta - Besoins en Operateurs et Toolkits (point d'entree Account)

## Contexte

Ce document mappe les besoins Account vers operateurs et toolkits executables.
Il sert de reference d'architecture fonctionnelle pour l'implementation.

## Operateurs cibles

### OP-ACCOUNT-CORE

- role : point d'entree fonctionnel Account
- couvre : auth contexte, dashboard, mouvements, rapports
- contrats : CK-SVC-01, CK-SVC-02, CK-TK-01, CK-TK-11, CK-TK-51

### OP-ACCOUNT-BILLING

- role : cycle devis/facture/paiement
- couvre : creation devis, conversion, emission facture, relances, encaissements
- contrats : CK-OP-11, CK-OP-12, CK-OP-13, CK-OP-14

### OP-ACCOUNT-INTEGRATION

- role : exposition contractuelle vers JayFestival/JayRDV
- couvre : endpoints CK-INT et controle ownership
- contrats : CK-INT-01, CK-INT-02

## Toolkits cibles

### TK-AUTH-CONTEXT

- operations : auth, resolution contexte, permissions
- dependances : Miyauth, Master Butler

### TK-LEDGER

- operations : record/list/correct/void mouvements
- dependances : KindMother, WorrySentinel

### TK-QUOTES

- operations : create/send/accept/reject/convert
- dependances : KindMother, Miyunotify

### TK-INVOICES

- operations : emit/send/remind/status
- dependances : KindMother, Miyunotify

### TK-PAYMENTS

- operations : record/link/status
- contraintes : tokenisation paiement obligatoire

### TK-REPORTING

- operations : dashboard/balance/legal/export
- contraintes : scope export minimal plus audit

## Matrice besoin vers operateur/toolkit

| Besoin | Operateur | Toolkit |
|--------|-----------|---------|
| MAC-01 MAC-03 | OP-ACCOUNT-CORE | TK-AUTH-CONTEXT |
| MAC-04 MAC-06 | OP-ACCOUNT-CORE | TK-LEDGER |
| MAC-07 MAC-10 | OP-ACCOUNT-BILLING | TK-QUOTES |
| MAC-11 MAC-14 | OP-ACCOUNT-BILLING | TK-INVOICES, TK-PAYMENTS |
| MAC-15 MAC-17 | OP-ACCOUNT-CORE | TK-REPORTING |
| MAC-18 MAC-19 | OP-ACCOUNT-INTEGRATION | TK-LEDGER, TK-REPORTING |

## Regles d'equipe operateurs

- REG-OP-1 : OP-ACCOUNT-BILLING n'ecrit jamais hors mandat actif
- REG-OP-2 : OP-ACCOUNT-INTEGRATION n'expose que les contrats CK-INT
- REG-OP-3 : OP-ACCOUNT-CORE est source de contexte et permissions
- REG-OP-4 : toute action critique produit un event audit

## Donnees et niveaux

| Classe | Niveau | Exemples |
|--------|--------|----------|
| Standard Account | 2 | mouvements, devis, factures |
| Critique Account | 3 | donnees paiement, pieces sensibles |

## Tests attendus

- tests unitaires toolkit TK-LEDGER
- tests unitaires toolkit TK-QUOTES/TK-INVOICES
- tests contrats integration CK-INT-01 et CK-INT-02
- tests erreurs contractuelles (mandat, scope, statut)

## References

- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/publics/Account/Account - Analyse des besoins.md`
- `docs/services/JayKonta/publics/Account/Account - Parcours Capacites Livrables.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Operateurs/toolkits enrichis
