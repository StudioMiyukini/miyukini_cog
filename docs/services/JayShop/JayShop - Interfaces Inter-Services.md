# JayShop — Interfaces Inter-Services

## Contexte

Ce document specifie les **interfaces contractuelles** du service **JayShop** avec les services consommateurs et fournisseurs : **JayXpose** (catalogue, stocks), **JayKonta** (comptabilite) et **Miyukini Central** (hebergement UI).

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), [JayXpose - Interfaces Inter-Services](../JayXpose/JayXpose%20-%20Interfaces%20Inter-Services.md), [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md).

## Portee / Scope

- **Perimetre** : Contrats d'interface JayShop ↔ JayXpose, JayShop ↔ JayKonta, JayShop ↔ Central. Payloads, securite, journalisation.
- **Hors perimetre** : Implementation technique, adaptateurs BondingBrother, tests de contrat.

---

## 1. Convention

- Format payload : JSON `snake_case`
- Horodatage : RFC3339 UTC
- Identifiants : UUID string
- Audit requis sur chaque echange ecriture
- Versionnement : champ optionnel `contract_version` (initiale : `1.0.0`)

---

## 2. Contrat IFS-JSH-01 — JayXpose (Lecture catalogue)

### Usage

- Lecture du catalogue produits, categories, visuels, prix, disponibilite et stocks depuis JayXpose.
- Source de verite pour toutes les donnees produit affichees dans JayShop.

### Payload entrant (JayXpose → JayShop)

```json
{
  "contract_version": "1.0.0",
  "exposant_id": "uuid",
  "products": [
    {
      "product_id": "uuid",
      "name": "string",
      "description": "string",
      "price": 350,
      "currency": "EUR",
      "category_id": "uuid",
      "category_name": "string",
      "availability": "disponible",
      "is_featured": true,
      "stock_qty": 42,
      "primary_image_url": "string",
      "images": [
        {
          "url": "string",
          "alt_text": "string",
          "is_primary": true
        }
      ]
    }
  ],
  "categories": [
    {
      "category_id": "uuid",
      "name": "string",
      "sort_order": 1
    }
  ]
}
```

### Securite

- Lecture gouvernee par Mandat de Permission (Master Butler).
- Donnees catalogue : niveau 1 (Standard).
- Donnees prix/stock : niveau 1 a 2 (Standard a Sensitive) selon politique de l'exposant.

---

## 3. Contrat IFS-JSH-02 — JayXpose (Ecriture catalogue)

### Usage

- CRUD produits et categories depuis JayShop, relaye a JayXpose (source de verite).
- L'admin ne quitte pas JayShop pour gerer son catalogue.

### Payload sortant (JayShop → JayXpose)

#### Creation produit

```json
{
  "contract_version": "1.0.0",
  "action": "create_product",
  "exposant_id": "uuid",
  "product": {
    "name": "string",
    "description": "string",
    "price": 350,
    "currency": "EUR",
    "category_id": "uuid",
    "availability": "disponible",
    "is_featured": false
  },
  "source": "jayshop",
  "timestamp": "2026-02-11T14:30:00Z"
}
```

#### Mise a jour produit

```json
{
  "contract_version": "1.0.0",
  "action": "update_product",
  "exposant_id": "uuid",
  "product_id": "uuid",
  "fields": {
    "price": 400,
    "availability": "rupture"
  },
  "source": "jayshop",
  "timestamp": "2026-02-11T14:35:00Z"
}
```

#### Suppression produit (soft delete)

```json
{
  "contract_version": "1.0.0",
  "action": "delete_product",
  "exposant_id": "uuid",
  "product_id": "uuid",
  "source": "jayshop",
  "timestamp": "2026-02-11T14:40:00Z"
}
```

#### CRUD categorie

```json
{
  "contract_version": "1.0.0",
  "action": "create_category | update_category | delete_category",
  "exposant_id": "uuid",
  "category": {
    "name": "string",
    "sort_order": 3
  },
  "source": "jayshop",
  "timestamp": "2026-02-11T14:45:00Z"
}
```

### Securite

- Ecriture soumise a Mandat (StrongFather).
- Audit complet : source, action, payload, horodatage.
- Seul l'admin proprietaire peut ecrire.

---

## 4. Contrat IFS-JSH-03 — JayXpose (Synchronisation stocks)

### Usage

- Decrementation du stock a chaque vente (push JayShop → JayXpose).
- Reception des mises a jour de stock depuis JayXpose (pull JayXpose → JayShop).
- Resolution de conflits configurable.

### Payload sortant (JayShop → JayXpose) — Stock push

```json
{
  "contract_version": "1.0.0",
  "sync_type": "stock_push",
  "source": "jayshop",
  "exposant_id": "uuid",
  "items": [
    {
      "product_id": "uuid",
      "pos_sku": "JXP-ABCD-001",
      "quantity_sold": 2,
      "new_stock_qty": 40,
      "ticket_id": "uuid"
    }
  ],
  "timestamp": "2026-02-11T15:00:00Z"
}
```

### Payload entrant (JayXpose → JayShop) — Stock pull

```json
{
  "contract_version": "1.0.0",
  "sync_type": "stock_pull",
  "source": "jayxpose",
  "items": [
    {
      "product_id": "uuid",
      "pos_sku": "JXP-ABCD-001",
      "stock_qty": 38
    }
  ],
  "timestamp": "2026-02-11T15:05:00Z"
}
```

### Politique de conflit

| Politique | Description |
|-----------|-------------|
| `prefer_pos` | En cas de conflit, le stock PoS/JayShop fait autorite. |
| `prefer_local` | En cas de conflit, la valeur JayXpose fait autorite. |
| `manual_review` | Les conflits sont mis en file d'attente pour resolution manuelle. |

### Securite

- Sync soumise a Mandat.
- Audit : table `sync_logs` avec `sync_source`, `sync_type`, `status`, `payload_json`, `created_at`.
- Alignement avec le protocole de sync existant (IFS-04 de JayXpose).

---

## 5. Contrat IFS-JSH-04 — JayKonta (Transmission comptable)

### Usage

- Transmission des ventes cloturees a JayKonta pour ecriture comptable.
- Transmission des remboursements comme ecritures inverses.
- Transmission des syntheses de cloture de caisse.

### Payload sortant (JayShop → JayKonta) — Vente

```json
{
  "contract_version": "1.0.0",
  "event_type": "sale_closed",
  "source": "jayshop",
  "seller_id": "uuid",
  "ticket": {
    "ticket_id": "uuid",
    "ticket_number": "JSHOP-2026-00042",
    "source": "pos",
    "subtotal_ht": 942,
    "tax_total": 188,
    "total_ttc": 1130,
    "currency": "EUR",
    "tax_details": [
      {
        "rate": 20.0,
        "base": 942,
        "amount": 188
      }
    ],
    "payments": [
      {
        "method": "cash",
        "amount": 1130
      }
    ],
    "closed_at": "2026-02-11T15:30:00Z"
  }
}
```

### Payload sortant (JayShop → JayKonta) — Remboursement

```json
{
  "contract_version": "1.0.0",
  "event_type": "refund",
  "source": "jayshop",
  "seller_id": "uuid",
  "refund": {
    "refund_ticket_id": "uuid",
    "original_ticket_id": "uuid",
    "amount_ht": 500,
    "tax_amount": 100,
    "amount_ttc": 600,
    "currency": "EUR",
    "method": "cash",
    "created_at": "2026-02-11T16:00:00Z"
  }
}
```

### Payload sortant (JayShop → JayKonta) — Cloture de caisse

```json
{
  "contract_version": "1.0.0",
  "event_type": "cash_session_closed",
  "source": "jayshop",
  "seller_id": "uuid",
  "session": {
    "session_id": "uuid",
    "opened_at": "2026-02-11T09:00:00Z",
    "closed_at": "2026-02-11T18:30:00Z",
    "total_sales_ttc": 125000,
    "total_refunds_ttc": 4500,
    "net_ttc": 120500,
    "payment_breakdown": {
      "cash": 62000,
      "card": 53000,
      "check": 5500
    },
    "opening_cash": 10000,
    "closing_cash_expected": 66150,
    "closing_cash_counted": 66000,
    "cash_difference": -150,
    "currency": "EUR"
  }
}
```

### Securite

- Donnees de vente : niveau 2 (Sensitive).
- Donnees de paiement : niveau 3 (Critical).
- Audit complet : source, event_type, payload, horodatage.
- Chiffrement en transit obligatoire.
- Mandat requis (StrongFather → JayKonta).

---

## 6. Contrat IFS-JSH-05 — Miyukini Central

### Usage

- Hebergement de JayShop dans le shell Miyukini Central.

### Interface

- Trait : `ServiceUi`
- Id service : `ServiceId::JayShop`
- Methode rendu : `show(&mut self, ui: &mut egui::Ui)`

### Ecrans exposes dans Central

| Ecran | Id |
|-------|-----|
| Tableau de bord | JSH-A01 |
| Point de vente | JSH-A05 |
| Liste produits | JSH-A02 |
| Historique ventes | JSH-A10 |
| Commandes en ligne | JSH-A12 |
| Configuration PoS | JSH-A07 |
| Parametres | JSH-A13 |
| **Liste evenements** | **JSH-A20** |
| **Dashboard evenement** | **JSH-A24** |

---

## 7. Contrat IFS-JSH-06 — JayFestival (Gestion evenements)

### Usage

- **Creation automatique** d'une fiche evenement dans JayShop lorsque l'organisateur JayFestival valide la candidature d'un exposant.
- **Synchronisation** des informations de l'edition (nom, dates, lieu, stand attribue) vers JayShop.
- **Annulation** : si la participation est annulee cote JayFestival, la fiche evenement est mise a jour et le stock temporaire reintegre.

### Payload entrant (JayFestival → JayShop) — Validation candidature

```json
{
  "contract_version": "1.0.0",
  "event_type": "candidature_validated",
  "source": "jayfestival",
  "timestamp": "2026-02-11T10:00:00Z",
  "edition": {
    "edition_id": "uuid",
    "name": "Salon du Livre 2026",
    "start_date": "2026-03-15",
    "end_date": "2026-03-17",
    "location": "Paris Expo Porte de Versailles",
    "organizer_id": "uuid",
    "organizer_name": "Association Livre & Culture"
  },
  "candidature": {
    "candidature_id": "uuid",
    "exposant_id": "uuid",
    "stand_number": "A42",
    "stand_zone": "Litterature jeunesse",
    "stand_size": "9m2",
    "validated_at": "2026-02-11T10:00:00Z"
  }
}
```

### Payload entrant (JayFestival → JayShop) — Mise a jour edition

```json
{
  "contract_version": "1.0.0",
  "event_type": "edition_updated",
  "source": "jayfestival",
  "timestamp": "2026-02-15T14:30:00Z",
  "edition_id": "uuid",
  "fields": {
    "start_date": "2026-03-16",
    "end_date": "2026-03-18",
    "location": "Paris Expo Porte de Versailles - Hall 7"
  }
}
```

### Payload entrant (JayFestival → JayShop) — Mise a jour stand

```json
{
  "contract_version": "1.0.0",
  "event_type": "stand_updated",
  "source": "jayfestival",
  "timestamp": "2026-02-20T09:15:00Z",
  "candidature_id": "uuid",
  "stand_number": "B15",
  "stand_zone": "Litterature jeunesse",
  "stand_size": "12m2"
}
```

### Payload entrant (JayFestival → JayShop) — Annulation participation

```json
{
  "contract_version": "1.0.0",
  "event_type": "participation_cancelled",
  "source": "jayfestival",
  "timestamp": "2026-02-25T16:00:00Z",
  "candidature_id": "uuid",
  "reason": "exposant_withdraw"
}
```

### Action JayShop pour chaque event_type

| event_type | Action JayShop |
|------------|----------------|
| `candidature_validated` | Creer une fiche evenement (`events`) avec les infos de l'edition et de la candidature. Notifier l'admin. |
| `edition_updated` | Mettre a jour la fiche evenement liee (dates, lieu). |
| `stand_updated` | Mettre a jour les informations stand dans la fiche evenement. |
| `participation_cancelled` | Passer la fiche evenement en statut `cancelled`. Si stock temporaire alloue, le reintegrer automatiquement au stock global JayXpose. |

### Payload sortant (JayShop → JayFestival) — Metriques evenement (optionnel)

JayShop peut transmettre a JayFestival un resume des metriques de l'evenement (pour le dashboard exposant cote JayFestival) :

```json
{
  "contract_version": "1.0.0",
  "event_type": "event_metrics",
  "source": "jayshop",
  "timestamp": "2026-03-18T20:00:00Z",
  "candidature_id": "uuid",
  "metrics": {
    "total_revenue": 125000,
    "total_tickets": 42,
    "status": "closed"
  }
}
```

### Securite

- Donnees evenement : niveau 1 (Standard) a 2 (Sensitive) pour les metriques financieres.
- Audit complet : source, event_type, payload, horodatage.
- Mandat requis (JayFestival → JayShop) pour les ecritures automatiques.

---

## 8. Journalisation standard

Tout appel inter-service doit produire :
- `sync_source` : identifiant du service emetteur
- `sync_type` : type d'operation
- `status` : resultat de l'operation
- `payload_json` : contenu de l'echange
- `created_at` : horodatage UTC

Table cible : `sync_logs`

---

## 9. Codes statut recommandes

- `ok` : operation reussie
- `partial` : operation partiellement reussie
- `error` : erreur technique
- `denied` : operation refusee (mandat insuffisant)

---

## 10. Erreurs standardisees

- `ERR_MANDATE_REQUIRED` : Mandat de Permission requis
- `ERR_PAYLOAD_INVALID` : Payload non conforme au contrat
- `ERR_TARGET_UNAVAILABLE` : Service cible indisponible
- `ERR_TIMEOUT` : Delai d'attente depasse
- `ERR_CONFLICT` : Conflit de donnees (stock, version)
- `ERR_STOCK_INSUFFICIENT` : Stock insuffisant pour la vente

---

## 11. Plan implementation

1. Exposer adaptateurs `BondingBrother` par service cible (JayXpose, JayKonta, JayFestival)
2. Implementer le contrat de lecture catalogue (IFS-JSH-01) en priorite
3. Implementer la sync stock (IFS-JSH-03) alignee sur le protocole existant
4. Implementer la transmission comptable (IFS-JSH-04)
5. Ajouter le contrat d'ecriture catalogue (IFS-JSH-02) en post-MVP
6. **Implementer le contrat JayFestival (IFS-JSH-06)** : reception des notifications de validation candidature, sync edition, annulation
7. Ajouter validateurs payload en entree/sortie
8. Ajouter tests de contrat par fixture JSON
9. Ajouter replay outils audit depuis `sync_logs`

---

## 12. References

- [JayXpose - Interfaces Inter-Services](../JayXpose/JayXpose%20-%20Interfaces%20Inter-Services.md)
- [JayXpose - Sync MiyukiniPoS Specification](../JayXpose/JayXpose%20-%20Sync%20MiyukiniPoS%20Specification.md)
- [JayXpose - Integration MiyukiniPoS](../JayXpose/JayXpose%20-%20Integration%20MiyukiniPoS.md)
- [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md)
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)
- [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md)

---

**Document** : JayShop — Interfaces Inter-Services
**Version** : 1.1
**Date** : 2026-02-11
**Statut** : Reference produit — enrichi avec contrat IFS-JSH-06 (JayFestival)
