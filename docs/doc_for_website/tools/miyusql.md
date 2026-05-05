# MiyuSQL

## Toolkit d'Accès aux Bases de Données

**MiyuSQL** est le toolkit responsable de toutes les opérations de persistance dans Miyukini. Il interface avec SQLite et SQLCipher pour stocker et récupérer les données.

## Fonction

> MiyuSQL **persiste** et **récupère** les données de manière sécurisée.

MiyuSQL est l'outil de confiance utilisé par KindMother pour toutes les opérations de base de données.

## Capacités

### Lecture

| Capacité | Description |
|----------|-------------|
| SELECT | Requêtes de lecture |
| Filtrage | Conditions WHERE |
| Jointures | Relations entre tables |
| Agrégation | COUNT, SUM, AVG... |

### Écriture

| Capacité | Description |
|----------|-------------|
| INSERT | Création de données |
| UPDATE | Modification |
| DELETE | Suppression |
| Transaction | Opérations atomiques |

### Administration

| Capacité | Description |
|----------|-------------|
| Migration | Évolution des schémas |
| Backup | Sauvegarde |
| Vacuum | Optimisation |
| Intégrité | Vérification |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                    MIYUSQL                       │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │            Query Engine                   │   │
│  └──────────────────────────────────────────┘   │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │ Query  │ │Transaction│ │Migration│ │ Backup │ │
│  │Executor│ │  Manager  │ │ Engine │ │ Manager│ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
│                     │                           │
│                     ▼                           │
│  ┌──────────────────────────────────────────┐   │
│  │              SQLite + SQLCipher          │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## Base de Données

### SQLite

MiyuSQL utilise SQLite pour :
- ✓ Fonctionnement offline (LOI-1, LOI-2)
- ✓ Données souveraines (LOI-3)
- ✓ Performance sur hardware modeste (LOI-5)
- ✓ Transactions ACID

### SQLCipher

Chiffrement au repos :
- Algorithme : AES-256
- Mode : CBC
- Clé dérivée du contexte utilisateur
- Transparent pour les requêtes

## Flux de Requête

```
Instruction (via BondingBrother)
        │
        ▼
┌─────────────────┐
│ Parsing         │──► Validation syntaxe
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Préparation     │──► Paramètres sécurisés
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Exécution       │──► SQLite/SQLCipher
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Résultat        │──► Données ou confirmation
└─────────────────┘
```

## Intégration avec KindMother

```
KindMother ──► "Sauvegarder ces données"
      │
      ▼
BondingBrother ──► MiyuSQL
                      │
                      ▼
                 Exécution SQL
                      │
                      ▼
                 Résultat
                      │
      ┌───────────────┘
      ▼
KindMother ◄── {success: true, id: 123}
```

## API (via BondingBrother)

### Intentions Supportées

| Intention | Paramètres | Résultat |
|-----------|------------|----------|
| `QUERY` | sql, params | {rows} |
| `EXECUTE` | sql, params | {affected_rows} |
| `INSERT` | table, data | {id} |
| `UPDATE` | table, data, where | {affected_rows} |
| `DELETE` | table, where | {affected_rows} |
| `TRANSACTION` | operations | {success} |

### Exemple

```sql
-- Intention : QUERY
-- Paramètres : 
{
  "sql": "SELECT * FROM users WHERE status = ?",
  "params": ["active"]
}

-- Résultat :
{
  "rows": [
    {"id": 1, "name": "Alice", "status": "active"},
    {"id": 2, "name": "Bob", "status": "active"}
  ]
}
```

## Transactions

### Support ACID

| Propriété | Garantie |
|-----------|----------|
| Atomicité | Tout ou rien |
| Cohérence | État valide |
| Isolation | Pas d'interférence |
| Durabilité | Données persistées |

### Exemple Transaction

```
TRANSACTION {
  INSERT INTO orders (user_id, amount) VALUES (?, ?);
  UPDATE accounts SET balance = balance - ? WHERE user_id = ?;
  INSERT INTO history (user_id, action) VALUES (?, 'order');
}
```

Si une étape échoue, tout est annulé.

## Migrations

### Gestion des Versions

```
migrations/
├── 001_initial.sql
├── 002_add_profiles.sql
├── 003_add_permissions.sql
└── ...
```

### Processus

```
Version actuelle : 002
Cible : 003
        │
        ▼
┌─────────────────┐
│ Backup auto     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Apply 003       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Update version  │
└────────┬────────┘
         │
         ▼
Version actuelle : 003
```

## Sécurité

### Injection SQL

Protection complète :
- Requêtes préparées obligatoires
- Paramètres toujours échappés
- Pas de concaténation de strings SQL

```rust
// ❌ INTERDIT
let sql = format!("SELECT * FROM users WHERE name = '{}'", name);

// ✓ OBLIGATOIRE
let sql = "SELECT * FROM users WHERE name = ?";
let params = vec![name];
```

### Chiffrement

| Données | Protection |
|---------|------------|
| Au repos | SQLCipher (AES-256) |
| En transit | Contexte local |
| Clés | Dérivées, jamais stockées |

### Accès

- Seul BondingBrother peut invoquer
- Toutes les requêtes loguées
- Quotas par opération

## Contrats

### Contrat de Frontière

MiyuSQL **peut** :
- Exécuter des requêtes SQL
- Gérer les transactions
- Faire des migrations
- Créer des backups

MiyuSQL **ne peut pas** :
- Décider quelles données stocker
- Accéder à d'autres bases
- Modifier le schéma sans migration

### Contrat de Gouvernance

MiyuSQL respecte :
- LOI-1 : Fonctionne offline
- LOI-3 : Données locales souveraines
- LOI-5 : Performance adaptée

## Performance

### Optimisations

| Technique | Bénéfice |
|-----------|----------|
| Index | Requêtes rapides |
| Cache | Réduction I/O |
| WAL | Écriture performante |
| Vacuum | Espace optimisé |

### Limites

| Ressource | Limite |
|-----------|--------|
| Taille DB | Illimitée (disque) |
| Connexions | 1 (SQLite) |
| Requête timeout | 30 secondes |

## Cas d'Usage

### Sauvegarde Document

```
JayKonta : "Sauvegarder facture"
        │
        ▼
KindMother : persist(data)
        │
        ▼
BondingBrother ──► MiyuSQL
        │
        ▼
MiyuSQL :
  BEGIN TRANSACTION;
  INSERT INTO portfolios (...) VALUES (...);
  INSERT INTO portfolio_items (...) VALUES (...);
  COMMIT;
        │
        ▼
{success: true, id: 456}
```

## Backup et Restauration

### Backup

```
MiyuSQL crée une copie :
  1. Flush WAL
  2. Copie fichier .db
  3. Vérifie intégrité
  4. Chiffre le backup
```

### Restauration

```
MiyuSQL restaure :
  1. Vérifie intégrité backup
  2. Déchiffre
  3. Remplace la DB active
  4. Valide
```
