# JayXpose — Profil exposant et site vitrine

## Contexte

**JayXpose** est le service Miyukini dédié au **profil exposant** et au **site vitrine** pour artisans, artistes et petites marques. Il permet de constituer une vitrine en ligne (catalogue, contact, portfolio, lien vers réservation ou boutique) et **s’intègre dans JayFestival** : la fiche exposant et le répertoire des exposants de JayFestival peuvent s’appuyer sur JayXpose ; un exposant peut avoir une vitrine JayXpose et participer à des éditions JayFestival avec le même profil.

Une **vitrine autonome** (sans événement festival) est également possible.

## Documentation principale

| Document | Rôle |
|----------|------|
| [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md) | Contexte, portée, raison d’être, principes directeurs, intégration et interpolarité (JayFestival), références. |
| [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non fonctionnels (profil, vitrine, répertoire, intégration JayFestival) ; données et champs. |
| [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md) | Parcours inscription, fiche entreprise, fiche publique ; mécaniques Catakana ; consommation par JayFestival. |
| [JayXpose - Operateurs et Toolkits](./JayXpose%20-%20Operateurs%20et%20Toolkits.md) | Opérateurs et Kits spécifiques JayXpose (profil, répertoire, liaison JayFestival). |
| [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md) | Écrans et composants UI (profil, fiche publique) ; intégration dans les écrans JayFestival. |

## Référence technique

| Document | Rôle |
|----------|------|
| [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md) | Tables Supabase (exposants, profiles, editions_exposants), RLS, requêtes SQL alpha, stratégie migration SQLite + KindMother. |
| [reference/_index.md](./reference/_index.md) | Index des documents de référence JayXpose. |

## Voir aussi

- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) — service dans lequel JayXpose s’intègre (fiche exposant, répertoire)
- [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) — principe de couplage entre services Jay
