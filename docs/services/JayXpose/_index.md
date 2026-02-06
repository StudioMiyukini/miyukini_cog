# JayXpose — Identité professionnelle exposant, vitrine et coffre-fort

## Contexte

**JayXpose** est le service Miyukini dédié à l'**identité professionnelle de l'exposant** : profil complet, catalogue de produits, site vitrine, coffre-fort documentaire et référencement dans l'annuaire des exposants. Il permet à un utilisateur de devenir exposant en constituant une présence en ligne complète et de centraliser toutes ses informations professionnelles pour les exploiter au sein de l'écosystème Jay — en particulier **JayFestival** (GFestival).

Une **vitrine autonome** (sans événement festival) est également possible.

## Documentation principale

| Document | Rôle |
|----------|------|
| [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md) | Contexte, portée, raison d'être, fonctionnalités structurantes, principes directeurs, intégration écosystème Jay. |
| [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non fonctionnels exhaustifs (profil, catalogue, vitrine, coffre-fort, annuaire, synchronisation JayFestival) ; données et champs. |
| [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md) | Parcours complets : inscription, fiche entreprise, catalogue, vitrine, documents, fiche publique, synchronisation JayFestival. |
| [JayXpose - Operateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs (Profil, Catalogue, Vitrine, Documents, Annuaire) et Kits d'Outils spécifiques JayXpose. |
| [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md) | Écrans et composants UI : espace exposant (12 écrans), écrans publics (6 écrans), intégration JayFestival. |

## Documentation spécifique par module

| Document | Rôle |
|----------|------|
| [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md) | Spécification complète du module catalogue : fiches produits, catégories, visuels, vedettes, intégration vitrine et JayFestival. |
| [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md) | Spécification du coffre-fort documentaire : types de documents, upload, versioning, statuts, alertes expiration, partage gouverné. |
| [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md) | Spécification du site vitrine : pages, personnalisation, URL unique, SEO, responsive, statuts de publication. |
| [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md) | Politique de confidentialité des données exposant, matrice de visibilité par rôle/service, gouvernance du partage. |
| [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md) | Contrat d'intégration détaillé avec JayFestival : données partagées, flux, pré-remplissage candidatures, partage documents, notifications. |

## Référence technique

| Document | Rôle |
|----------|------|
| [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md) | Schéma complet des tables (exposants, produits, documents, vitrine, partages, audit), RLS, requêtes SQL alpha, buckets Storage, stratégie migration SQLite + KindMother. |
| [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Classification des données par niveau de sécurité (WorrySentinel), politique de résidence, règles d'accès, chiffrement, audit. |
| [reference/_index.md](./reference/_index.md) | Index des documents de référence JayXpose. |

## Voir aussi

- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) — service dans lequel JayXpose s'intègre (fiche exposant, répertoire)
- [JayFestival - Interpolarite Services Jay](../JayFestival/reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md) — couplages entre services Jay
- [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) — principe global d'interpolarité
- [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) — facturation exposant (RIB partagé depuis coffre-fort)
- [JayRDV - Document Fondateur](../JayRDV/JayRDV%20-%20Document%20Fondateur.md) — prise de rendez-vous (lien depuis vitrine)
