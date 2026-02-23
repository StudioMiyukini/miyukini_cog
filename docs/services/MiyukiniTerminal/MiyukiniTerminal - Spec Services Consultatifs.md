# MiyukiniTerminal — Spécification Services Consultatifs

## Contexte

Ce document définit les **services exposés en vue consultative** (JayKonta, JayKoa), le format des données (JSON), la pagination et les limites (ex. derniers 30 jours).

**Références :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Spec Synchronisation Parent](./MiyukiniTerminal%20-%20Spec%20Synchronisation%20Parent.md)
- [Spec Actions Simples](./MiyukiniTerminal%20-%20Spec%20Actions%20Simples.md)

---

## Portée / Scope

- Services : JayKonta, JayKoa
- Données exposées
- Format (JSON)
- Pagination, limites

---

## 1. JayKonta

### 1.1 Données consultatives

| Donnée | Description | Limite |
|--------|-------------|--------|
| Purses (portefeuilles) | Liste avec solde, devise, nom | Tous |
| Mouvements | Montant, libellé, date, catégorie | 50 derniers par purse |
| Vue agrégée | Solde total | — |

### 1.2 Format JSON (exemple)

```json
{
  "purses": [
    {"id": "uuid", "name": "Principal", "balance": 125.50, "currency": "EUR"}
  ],
  "movements": [
    {"id": "uuid", "amount": -15.00, "label": "Déjeuner", "date": "2026-02-22", "purse_id": "uuid"}
  ]
}
```

### 1.3 Pagination

- Mouvements : `limit=50`, `offset` (ou `since` pour temps)
- Pas de pagination côté purse (liste courte)

---

## 2. JayKoa

### 2.1 Données consultatives

| Donnée | Description | Limite |
|--------|-------------|--------|
| Événements | Titre, début, fin, lieu | 30 jours à venir + 7 passés |
| Calendriers | Liste des calendriers accessibles | Tous |

### 2.2 Format JSON (exemple)

```json
{
  "events": [
    {"id": "uuid", "title": "RDV", "start": "2026-02-22T10:00", "end": "2026-02-22T11:00"}
  ],
  "calendars": [
    {"id": "uuid", "name": "Personnel"}
  ]
}
```

### 2.3 Pagination

- Fenêtre temporelle : `from`, `to` (dates ISO)
- Par défaut : 30 j à venir, 7 j passés

---

## 3. Format global sync

```json
{
  "version": 1,
  "jaykonta": {
    "purses": [...],
    "movements": [...]
  },
  "jaykoa": {
    "events": [...],
    "calendars": [...]
  }
}
```

---

## 4. Limites et performances

| Contrainte | Valeur |
|------------|--------|
| Taille réponse max | 500 KB (configurable) |
| Mouvements JayKonta | 50 par purse |
| Événements JayKoa | 200 max (fenêtre) |
| Rafraîchissement | Au plus toutes les 5 min |

---

## 5. Références

- [Spec Synchronisation Parent](./MiyukiniTerminal%20-%20Spec%20Synchronisation%20Parent.md)
- [Spec Actions Simples](./MiyukiniTerminal%20-%20Spec%20Actions%20Simples.md)
