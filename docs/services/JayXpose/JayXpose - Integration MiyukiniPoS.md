# JayXpose - Integration MiyukiniPoS

## Scope

- mapping produit vers SKU PoS
- sync stock push/pull
- resolution conflits

## Ecran

- XP-E03

## Journal

- table `sync_logs`
- types: `stock_push`, `stock_pull`, `stock_conflict_resolve`

## Politique de conflit

- `manual_review`
- `prefer_pos`
- `prefer_local`
