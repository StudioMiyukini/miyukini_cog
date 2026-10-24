# JayShop — Interfaces Inter-Services

## Contexte

Ce document specifie les **interfaces contractuelles** du service **JayShop** avec les services consommateurs et fournisseurs : **JayKonta** (comptabilite) et **Miyukini Central** (hebergement UI).

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md).

## Portee / Scope

- **Perimetre** : Contrats d'interface JayShop ↔ JayKonta, JayShop ↔ Central. Payloads, securite, journalisation.
- **Hors perimetre** : Implementation technique, adaptateurs BondingBrother, tests de contrat.

---

## 1. Convention

- Format payload : JSON `snake_case`
- Horodatage : RFC3339 UTC
- Identifiants : UUID string
- Audit requis sur chaque echange ecriture
- Versionnement : champ optionnel `contract_version` (initiale : `1.0.0`)

---

## 2. Contrat IFS-JSH-04 — JayKonta (Transmission comptable)

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

## 3. Contrat IFS-JSH-05 — Miyukini Central

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

## 4. Journalisation standard

Tout appel inter-service doit produire :
- `sync_source` : identifiant du service emetteur
- `sync_type` : type d'operation
- `status` : resultat de l'operation
- `payload_json` : contenu de l'echange
- `created_at` : horodatage UTC

Table cible : `sync_logs`

---

## 5. Codes statut recommandes

- `ok` : operation reussie
- `partial` : operation partiellement reussie
- `error` : erreur technique
- `denied` : operation refusee (mandat insuffisant)

---

## 6. Erreurs standardisees

- `ERR_MANDATE_REQUIRED` : Mandat de Permission requis
- `ERR_PAYLOAD_INVALID` : Payload non conforme au contrat
- `ERR_TARGET_UNAVAILABLE` : Service cible indisponible
- `ERR_TIMEOUT` : Delai d'attente depasse
- `ERR_CONFLICT` : Conflit de donnees (stock, version)
- `ERR_STOCK_INSUFFICIENT` : Stock insuffisant pour la vente

---

## 7. Plan implementation

1. Exposer adaptateurs `BondingBrother` par service cible (JayKonta)
2. Implementer la transmission comptable (IFS-JSH-04)
3. Ajouter validateurs payload en entree/sortie
4. Ajouter tests de contrat par fixture JSON
5. Ajouter replay outils audit depuis `sync_logs`

---

## 8. References

- [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md)
- [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md)

---

**Document** : JayShop — Interfaces Inter-Services
**Version** : 1.1
**Date** : 2026-02-11
**Statut** : Reference produit — contrats IFS-JSH-01/02/03/06 retires avec JayXpose/JayFestival (2026-04-29)
