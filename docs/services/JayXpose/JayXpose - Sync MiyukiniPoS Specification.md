# JayXpose - Sync MiyukiniPoS Specification

## 1. Objectif

Definir le protocole M7 de synchronisation entre JayXpose et MiyukiniPoS.

## 2. Perimetre MVP

- Synchronisation manuelle depuis JayXpose
- Cible: stock produit
- Trace obligatoire: audit `sync_logs`

## 3. Entites

- `ProduitCatalogue`
- `PosStockLink`
- `SyncLog`

## 4. Contrats fonctionnels

### 4.1 `SYNC-POS-01` - Push stock manuel

Declencheur:
- Bouton `Sync stock PoS` sur XP-E03

Effets:
- upsert `pos_stock_links`
- recalcul `produits_catalogue.availability`
- ecriture `sync_logs` (`sync_source=miyukinipos`, `sync_type=stock_push`)

### 4.2 `SYNC-POS-02` - Lecture etat sync

Declencheur:
- Chargement dashboard/catologue

Effets:
- lecture `pos_stock_links_by_exposant`
- lecture `sync_logs_by_exposant`

### 4.3 `SYNC-POS-03` - Audit payload type

Payload minimal:
- `connector`
- `mode`
- `synced_products`
- `timestamp`

Statut:
- `ok`
- `partial`
- `error`

## 5. Mapping stock

- cle produit JayXpose: `produit_id`
- cle PoS: `pos_sku`
- quantite: `stock_qty`
- disponibilite derivee:
- `stock_qty <= 0` -> `rupture`
- `stock_qty > 0` -> `disponible`

## 6. Modes de sync

### 6.1 Manuel (MVP)

- Action utilisateur explicite
- Bon pour controle metier et supervision

### 6.2 Batch (future)

- Traitement planifie
- utile pour gros catalogues

### 6.3 Quasi temps-reel (future)

- webhook ou message bus
- utile pour PoS volumineux

## 7. Gestion des conflits

MVP:
- dernier ecrivain gagnant cote JayXpose

Cible:
- regles par source autoritaire:
- prix autoritaire JayXpose
- stock autoritaire PoS

## 8. Securite et gouvernance

- Mandat requis pour sync ecriture
- Journalisation obligatoire des erreurs
- PII non exposee dans payload sync

## 9. Exemples payload

### 9.1 Success

```json
{
  "connector": "miyukinipos",
  "mode": "manual",
  "synced_products": 18,
  "timestamp": "2026-02-07T11:22:33Z"
}
```

### 9.2 Error

```json
{
  "connector": "miyukinipos",
  "mode": "manual",
  "synced_products": 0,
  "error_code": "POS_TIMEOUT",
  "timestamp": "2026-02-07T11:25:02Z"
}
```

## 10. Interfaces code

- `crates/jayxpose/src/screens/exp/e03_catalogue_liste.rs`
- `crates/jayxpose/src/data/kindmother_db.rs`
- `crates/jayxpose/src/data/types.rs`

## 11. KPI sync

- taux succes sync
- nb produits sync par run
- delai moyen sync
- taux conflits

## 12. Evolution

1. Ajouter endpoint Pull PoS -> JayXpose
2. Ajouter comparaison delta avant ecriture
3. Ajouter ecran de resolution de conflits
4. Ajouter alertes en cas de desynchronisation prolongee
