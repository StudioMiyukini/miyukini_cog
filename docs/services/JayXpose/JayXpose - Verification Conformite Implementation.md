# JayXpose — Vérification conformité implémentation

## Contexte

Ce document vérifie que l’implémentation de **JayXpose** est alignée à 100 % avec la documentation de référence (Document fondateur, Site Vitrine, Écrans et UI, etc.). Il sert de checklist de conformité et signale les écarts ou manques.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md), [Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md).

---

## 1. Périmètre documenté vs implémenté

### 1.1 Fonctionnalités structurantes (Document fondateur)

| Fonctionnalité | Documenté | Implémenté | Lieu implémentation | Note |
|----------------|-----------|------------|----------------------|------|
| Profil exposant enrichi | Oui | Oui | Central > JayXpose > Entreprise (XP-E02) | Fiche entreprise complète, contacts, visuels, réseaux, confidentialité |
| Catalogue de produits | Oui | Oui | Central > JayXpose > Catalogue + ProduitForm (XP-E03, XP-E04) | Liste, CRUD produit, visuels |
| Site vitrine (config) | Oui | Oui | Central > JayXpose > Vitrine (XP-E06, XP-E07, XP-E08) | Paramètres, page présentation, prévisualisation |
| Documents professionnels (coffre-fort) | Oui | Oui | Central > JayXpose > Documents (XP-E09, XP-E10, XP-E11) | Liste, upload, partages |
| Annuaire des exposants | Oui | Partiel | Central (données) ; annuaire public **non exposé en web** | PUB-E01 / PUB-E02 à ajouter côté service WEB (Origin) |
| Synchronisation JayFestival | Oui | Oui (contrat) | JayFestival client, répertoire exposants | Intégration côté JayFestival |

### 1.2 Écrans espace exposant (Central — natifs)

| Écran | Code doc | Implémenté | Module Central |
|-------|----------|------------|----------------|
| Dashboard exposant | XP-E01 | Oui | `dashboard.rs` |
| Fiche entreprise | XP-E02 | Oui | `entreprise.rs` |
| Liste catalogue | XP-E03 | Oui | `catalogue.rs` |
| Fiche produit création/modification | XP-E04 | Oui | `produit_form.rs` |
| Gestion des catégories | XP-E05 | Partiel | Intégré dans Catalogue (pas d’écran dédié) |
| Vitrine paramètres | XP-E06 | Oui | `vitrine.rs` |
| Vitrine page présentation | XP-E07 | Oui | `vitrine.rs` (éditeur + pages) |
| Vitrine prévisualisation | XP-E08 | Oui | Prévisualisation dans Vitrine |
| Coffre-fort documents | XP-E09 | Oui | `documents.rs` |
| Upload document | XP-E10 | Oui | `documents.rs` |
| Demande de partage | XP-E11 | Oui | `documents.rs` |
| Ma fiche publique | XP-E12 | Oui | `fiche_publique.rs` |

### 1.3 Écrans publics (vitrine web — service WEB / Origin)

| Écran | Code doc | Implémenté | Note |
|-------|----------|------------|------|
| Annuaire des exposants | PUB-E01 | Non | À implémenter dans le service WEB (Origin) |
| Fiche exposant détail annuaire | PUB-E02 | Non | À implémenter |
| **Vitrine — Page Accueil** | PUB-E03 | Oui | Origin : `/vitrine/{slug}`, layout commun, données KM |
| **Vitrine — Page Catalogue** | PUB-E04 | Oui | Origin : `/vitrine/{slug}/catalogue` et fiche produit |
| **Vitrine — Page Présentation** | PUB-E05 | Oui | Origin : `/vitrine/{slug}/presentation` |
| **Vitrine — Page Contact** | PUB-E06 | Oui | Origin : `/vitrine/{slug}/contact` |

Les pages vitrine publiques (PUB-E03 à PUB-E06) sont les **jumelles web** des écrans natifs Vitrine (XP-E06 à XP-E08) : mêmes données, source KindMother (DB JayXpose), layout commun à tous les COG, **distinct de la page d’accueil « Home »** du portail (description COG / MWS).

### 1.4 Intégration JayFestival

| Élément | Documenté | Implémenté |
|---------|-----------|------------|
| Répertoire exposants (UNC-E08) | Oui | Oui (données JayXpose) |
| Fiche exposant détail (UNC-E09) | Oui | Oui |
| Fiche exposant organisateur (ORG-E11) | Oui | Oui |
| Formulaire candidature pré-rempli (EXP-E10) | Oui | Oui |

---

## 2. Données et persistance (KindMother)

| Élément | Documenté | Implémenté |
|--------|-----------|------------|
| Schéma 8 tables+ (exposants, produits, vitrine_pages, etc.) | Oui | Oui (`kindmother_db.rs`, schéma SQLite) |
| Exposant par ID | Oui | Oui `exposant_by_id` |
| Exposant par slug vitrine (vitrine_status = publiee) | Spec Site Vitrine | Oui `exposant_by_vitrine_slug` |
| Produits par exposant / produit par ID | Oui | Oui |
| Pages vitrine par exposant | Oui | Oui `vitrine_pages_by_exposant` |
| Catégories produits | Oui | Oui |

---

## 3. Synthèse conformité

- **Central (écrans natifs)** : Conforme à la documentation pour XP-E01 à XP-E12 (avec XP-E05 regroupé dans Catalogue). Données via `JayXposeDb` (KindMother Daughter).
- **Écrans publics (vitrine web)** : Implémentés dans Origin (routes `/vitrine`, `/vitrine/{slug}`, etc., layout commun, lecture seule depuis la DB JayXpose). Annuaire global (PUB-E01, PUB-E02) à faire.
- **Distinction Home vs Vitrine** : La page « Home » du portail (Origin) décrit le COG / MWS (présentation générale). Les pages **JayXpose public** sont dédiées à la vitrine exposant (présentation/catalogue/contact) et utilisent un **layout commun vitrine** partagé entre tous les COG exposant.

---

## 4. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [JayXpose - Pages Web Publiques Vitrine](./JayXpose%20-%20Pages%20Web%20Publiques%20Vitrine.md)

---

**Document** : JayXpose — Vérification conformité implémentation  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Référence — checklist de conformité
