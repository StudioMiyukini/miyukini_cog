# JayXpose - Interfaces Inter-Services

## 1. Objectif

Specifer les interfaces JayXpose pour les implementations futures.

## 2. Convention

- format payload: JSON snake_case
- horodatage: RFC3339 UTC
- identifiants: UUID string
- audit requis sur chaque echange ecriture

## 3. Contrat IFS-01 JayFestival

### Usage

- pre-remplissage candidature exposant
- publication fiche annuaire evenement

### Payload sortant JayXpose

```json
{
  "exposant_id": "uuid",
  "company_name": "string",
  "description_short": "string",
  "catalogue_preview": [
    {
      "product_id": "uuid",
      "name": "string",
      "price": 12.5,
      "availability": "disponible"
    }
  ],
  "vitrine_slug": "string"
}
```

### Securite

- champs soumis a confidentialite_profil
- documents transmis uniquement via mandat

## 4. Contrat IFS-02 JayKonta

### Usage

- facturation exposant
- verification pieces administratives

### Payload sortant JayXpose

```json
{
  "exposant_id": "uuid",
  "company_name": "string",
  "billing_contact": {
    "name": "string",
    "email": "string",
    "phone": "string"
  },
  "shared_documents": [
    {
      "document_id": "uuid",
      "doc_type": "rib",
      "status": "valide"
    }
  ]
}
```

### Securite

- partage base sur `documents_partages`
- audit source `jaykonta`

## 5. Contrat IFS-03 JayRDV

### Usage

- couplage catalogue service -> reservation

### Payload sortant JayXpose

```json
{
  "exposant_id": "uuid",
  "services": [
    {
      "product_id": "uuid",
      "name": "string",
      "description": "string",
      "price": 42.0
    }
  ],
  "vitrine_url": "/vitrine/slug"
}
```

## 6. Contrat IFS-04 MiyukiniPoS

### Usage

- sync stock

### Payload sortant JayXpose

```json
{
  "exposant_id": "uuid",
  "sync_type": "stock_push",
  "items": [
    {
      "produit_id": "uuid",
      "pos_sku": "JXP-ABCD-001",
      "stock_qty": 12
    }
  ]
}
```

### Payload entrant PoS

```json
{
  "sync_type": "stock_pull",
  "items": [
    {
      "pos_sku": "JXP-ABCD-001",
      "stock_qty": 7
    }
  ]
}
```

## 7. Contrat IFS-05 Central

### Usage

- hebergement de JayXpose dans shell Central

### Interface

- trait: `ServiceUi`
- id service: `ServiceId::JayXpose`
- methode rendu: `show(&mut self, ui: &mut egui::Ui)`

## 8. Journalisation standard

Tout appel inter-service doit produire:
- `sync_source`
- `sync_type`
- `status`
- `payload_json`
- `created_at`

Table cible: `sync_logs`

## 9. Codes statut recommandes

- `ok`
- `partial`
- `error`
- `denied`

## 10. Erreurs standardisees

- `ERR_MANDATE_REQUIRED`
- `ERR_PAYLOAD_INVALID`
- `ERR_TARGET_UNAVAILABLE`
- `ERR_TIMEOUT`
- `ERR_CONFLICT`

## 11. Versionnement contrats

- champ optionnel conseille: `contract_version`
- version initiale: `1.0.0`

## 12. Plan implementation

1. Exposer adaptateurs `BondingBrother` par service cible
2. Ajouter validateurs payload en entree/sortie
3. Ajouter tests de contrat par fixture JSON
4. Ajouter replay outils audit depuis `sync_logs`
