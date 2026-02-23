# JayFestival — Référence

## Contexte

Ce dossier regroupe les **documents de référence** du service JayFestival, dont l’interpolarité avec les autres services Jay.

## Portée / Scope

- **Périmètre** : Références pour l’intégration et les couplages avec les services Jay.
- **Audience** : Architectes, développeurs, product owners.

## Documents

| Document | Description |
|----------|-------------|
| [JayFestival - Parcours Utilisateurs Schema Flux](./JayFestival%20-%20Parcours%20Utilisateurs%20Schema%20Flux.md) | **Parcours utilisateurs** des 3 rôles (Organisateur, Exposant, Visiteur) extraits du schéma de flux ; points d'entrée, flux de décision, interconnexions services Jay, états et transitions, règles métier. |
| [JayFestival - Interpolarite Services Jay](./JayFestival%20-%20Interpolarite%20Services%20Jay.md) | Couplages JayFestival avec JayXpose, JayFaim, JayKoa, JayKonta ; rôle de JayFestival dans chaque couplage ; liens vers documents fondateurs. |
| [JayFestival - Connexions Synchronisation Services Jay](./JayFestival%20-%20Connexions%20Synchronisation%20Services%20Jay.md) | **Dépendances Cargo**, liaisons métier, bornes alpha/post-alpha, **implémentation sync JayFestival ↔ JayKoa**, sync JayXpose et **annuaire exposants** ; chemins de code, états actuels. |
| [JayFestival - Etat Documentation Services Interfaces](./JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | **Audit** : état de la doc de chaque service interfacé (Jay, Miyu*, Cores) pour implémentation complète UI incluse ; manques ; **ambiguïtés et choix humains** à trancher ; actions recommandées. |
| [JayFestival - Reference Base de Donnees et Migration Supabase vers SQLite](./JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | État actuel Supabase (Catakana) : tables, RLS, services ; mapping vers Kits JayFestival ; stratégie de migration Supabase → SQLite + outils maison ; version alpha (Supabase autorisé en exception pré-COG). |

## Ressources graphiques

| Fichier | Description |
|---------|-------------|
| `Untitled.jpg` à `Untitled (6).jpg` | Schéma de flux découpé en 7 images — source du document "Parcours Utilisateurs Schema Flux" |

---

*Dernière mise à jour : 2026-02-09*
