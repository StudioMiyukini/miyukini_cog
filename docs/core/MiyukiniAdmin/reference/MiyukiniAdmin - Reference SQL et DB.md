# MiyukiniAdmin — Reference SQL et DB

## 1. Contexte

Ce document definit la **reference technique** pour les sujets SQL et base de donnees dans MiyukiniAdmin, enrichie depuis Supabase et PostgreSQL. Il couvre schemas, tables, colonnes et types, requetes, resultats et analyse, Realtime, roles et connexion, en les reliant aux contrats MiyukiniAdmin et au COG.

**Service hors-bord :** MiyukiniAdmin est un **service hors-bord**. Il contourne tout RLS (Row Level Security) et toute Auth externe (ex. Supabase). L'authentification et l'autorisation applicables a MiyukiniAdmin sont **les siennes**, a definir et implementer (auth propre du service admin). Aucune notion de RLS type Supabase n'est retenue dans la doc MiyukiniAdmin.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portee / Scope

Ce document definit :
- Schemas et tables (notion de schema, liste tables/vues, creation table).
- Colonnes et types (types pris en charge, valeur par defaut, primary key, nullability).
- Requetes (SELECT en mode normal, timeout, LIMIT, validation StrongFather ; ecriture en Recovery).
- Resultats et analyse (affichage tabulaire, export, Explain, Chart).
- Realtime (transposition COG).
- Roles et connexion (environnement COG, acces BondingBrother).

Ce document **ne remplace pas** les contrats ; il les complete et sert de reference pour l'interface et la documentation.

---

## 3. Schemas et tables

### 3.1 Notion de schema

- **Schema** : espace de nommage pour les objets DB (tables, vues, etc.). Exemple courant : `public`.
- **Equivalent MiyukiniAdmin :** l'environnement COG peut exposer un ou plusieurs schemas ; la selection de schema (ex. dropdown "schema public") permet de filtrer les tables/vues affichees.
- **Liste tables/vues :** exploration via KindMother (READ-001, READ-002). Voir [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md).

### 3.2 Creation de table

- **Champs usuels :** nom de la table, description (optionnelle).
- **Creation effective :** via scripts de migration (MIG-001) ou mode Recovery selon contrat ; pas de creation directe en mode normal sans validation StrongFather. Voir [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) et [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md).

---

## 4. Colonnes et types

### 4.1 Types pris en charge (sous-ensemble Postgres / Supabase)

| Type | Description | Usage courant |
|------|-------------|---------------|
| **int2** | Entier signe 2 octets | Petits entiers |
| **int4** | Entier signe 4 octets | Entiers standards |
| **int8** | Entier signe 8 octets | IDs, grands entiers |
| **float4** | Flottant simple precision (4 octets) | Reels |
| **float8** | Flottant double precision (8 octets) | Reels precis |
| **numeric** | Numerique exact (precision choisie) | Montants, calculs exacts |
| **json** | JSON textuel | Donnees semi-structurees |
| **jsonb** | JSON binaire (decompose) | Indexation, requetes JSON |
| **text** | Chaine longueur variable | Texte libre |
| **varchar(n)** | Chaine longueur variable limitee | Texte borne |
| **uuid** | Identifiant universel unique | IDs externes |
| **timestamptz** | Date/heure avec fuseau | Horodatage |
| **timestamp** | Date/heure sans fuseau | Horodatage local |
| **bool** | Booleen | Vrai/faux |

*Reference style Supabase : "About data types" — documenter ces types dans l'UI Table Editor (creation de colonnes).*

### 4.2 Proprietes des colonnes

- **Valeur par defaut :** ex. `now()` pour timestamptz, `NULL`, expression SQL.
- **Primary key :** designation d'une ou plusieurs colonnes comme cle primaire.
- **Nullability :** colonne nullable ou NOT NULL.
- **Foreign keys :** references vers d'autres tables ; a documenter dans le formulaire "Create a new table" (section Foreign keys) si dans le perimetre.

---

## 5. Requetes

### 5.1 Mode normal (Console Query)

- **SELECT uniquement :** en mode normal, seules les requetes SELECT sont autorisees (lecture seule). Parsing / whitelist, timeout et LIMIT force selon [DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) et [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md).
- **Validation StrongFather :** pour l'intention de lecture si la politique l'exige.
- **Execution :** via KindMother (BondingBrother) ; pas d'acces DB direct depuis MiyukiniAdmin.

### 5.2 Mode Recovery (ecriture)

- **Ecriture (UPDATE, INSERT, DELETE, etc.) :** uniquement en mode Recovery, sous conditions cumulatives (T3/T4, protocole renforce, MFA, justification, approbation StrongFather, fenetre limitee). Voir [Emergency DB Access Contract](../contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md).
- **Interface dediee :** Console Recovery separee du SQL Editor public ; pas de reutilisation du code "SQL Editor" pour l'ecriture.

---

## 6. Resultats et analyse

### 6.1 Affichage tabulaire

- **Resultats :** affichage des lignes retournees par une requete SELECT sous forme de tableau (colonnes = champs, lignes = enregistrements).
- **Export :** CSV, JSON (READ-004). Voir [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md).

### 6.2 Onglet Explain

- **Plan d'execution :** affichage du plan d'execution de la requete (EXPLAIN ou EXPLAIN ANALYZE) pour l'optimisation et le diagnostic de performance.
- **Usage :** analyse des requetes lentes, verification des index.

### 6.3 Onglet Chart

- **Visualisation :** representation graphique des resultats (courbes, barres) pour metriques ou donnees agregées. Optionnel.
- **Implementation :** soit tableau + export (Rust seul), soit serveur genere SVG, soit librairie JS legere cote client ; voir [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).

---

## 7. Realtime

### 7.1 Transposition COG

- **Supabase :** "Enable Realtime" sur une table permet de diffuser les changements (INSERT/UPDATE/DELETE) aux clients abonnes via WebSockets.
- **MiyukiniAdmin :** transposition = ecoute des changements (donnees ou metriques) soit par **polling** (periodique), soit par **SSE** (Server-Sent Events), soit par **WebSockets** si besoin de push instantane. Cote COG : BondingBrother / KindMother pour la source des evenements. Voir [Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md).

---

## 8. Roles et connexion

### 8.1 Role (ex. postgres)

- **Supabase :** affichage du role utilise pour executer les requetes (ex. `Role: postgres`).
- **MiyukiniAdmin :** equivalent = le contexte d'acces a la base (environnement COG) ; toute requete passe par BondingBrother vers KindMother. Le "role" affiche peut etre le compte ou le contexte utilise par l'Operateur Souverain (lecture seule en normal, Recovery en ecriture).

### 8.2 Primary Database / Source

- **Supabase :** selection de la base (Primary Database) et de la source.
- **MiyukiniAdmin :** equivalent = **Environnement** COG ; une seule "source" en general (KindMother via BondingBrother). Pas d'API publique ; voir [Invariants & Guarantees](../contracts/governance/MiyukiniAdmin%20-%20Invariants%20&%20Guarantees.md) (INV-MA-3).

### 8.3 Documentation associee

- [KindMother Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md)
- [DB Operations Contract](../contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md)
- [DB Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md)
- [Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)

---

## 9. Documents associes

- [MiyukiniAdmin - Pages et Outils Reference Supabase](./MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md)
- [MiyukiniAdmin - Gestion DB type Supabase](./MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md)
- [MiyukiniAdmin - DB Management Interface](../ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md)
- [MiyukiniAdmin - Affichage Dynamique et Metriques](../ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md)

---

**Date de creation :** 2026-01-29  
**Version :** 1.0.0  
**Statut :** Document de reference (SQL et DB)
