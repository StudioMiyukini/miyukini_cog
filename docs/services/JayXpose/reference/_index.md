# JayXpose — Référence technique

## Contexte

Ce dossier regroupe les documents de **référence technique** du service JayXpose (profil exposant, catalogue, vitrine, coffre-fort, annuaire).

## Documents

| Document | Rôle |
|----------|------|
| [JayXpose - Base de donnees Supabase et Migration SQLite](./JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md) | Schéma complet des tables Supabase (exposants, produits_catalogue, categories_produits, produits_visuels, documents_professionnels, documents_versions, documents_partages, documents_audit, vitrine_pages, confidentialite_profil), RLS, index, requêtes SQL alpha, buckets Storage, stratégie migration SQLite + KindMother. |
| [JayXpose - Niveaux Securite et Protection Donnees](./JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Classification des données par niveau de sécurité (WorrySentinel), politique de résidence (alpha Supabase / post-alpha SQLite + KindMother), règles d'accès, chiffrement, audit et traçabilité, impact des états de confiance. |

## Voir aussi

- [JayFestival - Reference Base de Donnees et Migration](../../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) — schéma global et migration commune.
- [JayXpose - Confidentialite et Partage Inter-Services](../JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md) — politique de confidentialité (document principal).
