# MiyukiniTerminal — Spécification Actions Simples

## Contexte

Ce document décrit les **actions déléguées au parent** : saisie dépense JayKonta, création événement JayKoa. Flow : Terminal → queue → sync → parent exécute. Format requête et confirmation.

**Références :**

- [Spec Services Consultatifs](./MiyukiniTerminal%20-%20Spec%20Services%20Consultatifs.md)
- [Spec Queue Actions Offline](./MiyukiniTerminal%20-%20Spec%20Queue%20Actions%20Offline.md)
- [Spec Synchronisation Parent](./MiyukiniTerminal%20-%20Spec%20Synchronisation%20Parent.md)

---

## Portée / Scope

- Actions : dépense JayKonta, événement JayKoa
- Flow Terminal → parent
- Format requête
- Confirmation

---

## 1. Action : Saisie dépense JayKonta

### 1.1 Données requises

| Champ | Type | Obligatoire |
|-------|------|-------------|
| purse_id | string | Oui |
| amount | number | Oui (négatif) |
| label | string | Oui |
| date | string | Optionnel (défaut today) |
| category_id | string | Optionnel |

### 1.2 Format requête (queue / API)

```json
{
  "action_type": "jaykonta.expense",
  "payload": {
    "purse_id": "uuid",
    "amount": -25.50,
    "label": "Déjeuner",
    "date": "2026-02-22"
  }
}
```

### 1.3 Flow

1. Utilisateur remplit formulaire
2. Clic "Enregistrer"
3. Si Online : envoi direct au parent
4. Si Offline : enregistrement queue
5. Confirmation : "Dépense enregistrée" ou "Sera synchronisée à la reconnexion"

---

## 2. Action : Création événement JayKoa

### 2.1 Données requises

| Champ | Type | Obligatoire |
|-------|------|-------------|
| title | string | Oui |
| start | string (ISO) | Oui |
| end | string (ISO) | Oui |
| calendar_id | string | Optionnel |
| location | string | Optionnel |

### 2.2 Format requête

```json
{
  "action_type": "jaykoa.event",
  "payload": {
    "title": "RDV médecin",
    "start": "2026-02-25T10:00:00",
    "end": "2026-02-25T11:00:00"
  }
}
```

### 2.3 Flow

Identique à dépense : formulaire → envoi/queue → confirmation.

---

## 3. Confirmation côté parent

| Réponse | Code | Action Terminal |
|---------|------|-----------------|
| Succès | 200/201 | Marquer sent ; afficher "Enregistré" |
| Erreur validation | 400 | Afficher message ; garder dans queue pour retry si corrigeable |
| Conflit | 409 | Voir Spec Queue (merge/TAMR) |
| Erreur serveur | 5xx | Retry (queue) |

---

## 4. Références

- [Spec Queue Actions Offline](./MiyukiniTerminal%20-%20Spec%20Queue%20Actions%20Offline.md)
- [Spec Services Consultatifs](./MiyukiniTerminal%20-%20Spec%20Services%20Consultatifs.md)
