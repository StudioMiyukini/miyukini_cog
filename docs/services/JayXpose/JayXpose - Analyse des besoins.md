# JayXpose — Analyse des besoins

## Contexte

Ce document constitue l’**analyse des besoins** du service **JayXpose** : profil exposant, vitrine en ligne, répertoire des exposants et **liaison avec JayFestival**. Il reprend les mécaniques du parcours utilisateur exposant (Catakana / JayFestival) pour la **fiche exposant** et les **données vitrine**. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [JayFestival - Exposants Analyse des besoins](../JayFestival/publics/Exposants/Exposants%20-%20Analyse%20des%20besoins.md), [JayFestival - Reference Base de Donnees](../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).

## Portée / Scope

- **Périmètre** : Besoins fonctionnels et non fonctionnels de JayXpose (profil exposant, vitrine, catalogue, répertoire, intégration JayFestival) ; données et fiches (alpha Supabase, migration SQLite).
- **Hors périmètre** : Candidatures, participations, facturation (JayFestival) ; spécifications API détaillées (contrats dédiés).

---

## 1. Profil du public et personas

### 1.1 Définition

Les **exposants** sont des professionnels, entreprises ou associations qui **participent à des événements/festivals** en tant qu’exposants. **JayXpose** gère leur **profil vitrine** : identité, catalogue, contact, portfolio. Ce profil alimente la **fiche exposant** dans JayFestival et le **répertoire des exposants** (catalogue public).

### 1.2 Personas (alignés JayFestival Exposants)

| Persona | Profil | Besoin JayXpose |
|---------|--------|------------------|
| **Artisan / créateur** | Petit exposant ; 2 à 5 festivals par an. | Fiche vitrine simple (nom, activité, logo, contact) ; visible dans le répertoire. |
| **Entreprise / marque** | Exposant régulier ; 10 à 30 salons/an. | Vitrine complète (catalogue, site web, réseaux) ; visibilité répertoire. |
| **Association / collectif** | Occasionnel ; 1 à 3 événements/an. | Fiche entreprise ; mise à jour pour le répertoire. |

---

## 2. Besoins fonctionnels

### 2.1 Profil exposant (fiche entreprise)

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-01 | Création du profil exposant | À l'inscription (JayFestival ou autonome), création d'un profil JayXpose : identité, contact, activité. | Données stockées (Supabase alpha : table `exposants` liée à `profiles`) ; un exposant = un profil. |
| JXP-02 | Mise à jour du profil | Pouvoir modifier les informations de la fiche entreprise (nom, contact, activité, logo, site web, adresse, SIRET si applicable). | Formulaire ou écran « Mon compte » / « Fiche entreprise » ; sauvegarde immédiate ou sur action « Enregistrer ». |
| JXP-03 | Logo et visuels | Upload et affichage du logo ; optionnel : visuels additionnels (portfolio, galerie). | Stockage (Supabase Storage alpha) ; URL exposée dans la fiche ; formats et taille max définis. |
| JXP-04 | Données de contact | Email, téléphone, adresse postale, site web, réseaux sociaux (optionnel). | Champs configurables ; visibilité selon paramètre « fiche publique » (répertoire). |
| JXP-05 | Secteur / activité | Catégorie ou secteur d'activité (liste ou libre). | Utilisé pour le répertoire (filtres) et la fiche exposant. |

### 2.2 Vitrine et répertoire

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-06 | Fiche publique (répertoire) | La fiche exposant peut être publiée dans le **répertoire des exposants** du catalogue (JayFestival). | Visibilité selon politique plateforme et choix exposant (option désactivation) ; champs autorisés éditables. |
| JXP-07 | Liste répertoire (catalogue) | Affichage de la liste des exposants (annuaire) : nom, secteur, logo, lien fiche. | Requête sur exposants avec flag « visible en répertoire » ; filtres (secteur, recherche). |
| JXP-08 | Fiche exposant (détail public) | Affichage de la fiche complète d'un exposant (répertoire ou depuis fiche événement JayFestival). | Données lues depuis JayXpose (profil) ; éditions participées si liaison JayFestival (requête `editions_exposants`). |
| JXP-09 | Cohérence fiche ↔ candidatures | La fiche exposant utilisée pour les **candidatures** JayFestival est la même que le profil JayXpose. | Pas de duplication : une source de vérité (table `exposants` alpha) ; JayFestival lit les données exposant pour formulaires candidature et répertoire. |

### 2.3 Intégration JayFestival

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-10 | Fiche exposant côté organisateur | L'organisateur consulte la fiche exposant (nom, contact, activité, logo) pour candidatures et participations. | Données exposant lues depuis la même source (Supabase `exposants`) ; pas de doublon. |
| JXP-11 | Répertoire par événement | Liste des exposants **par édition** (participations validées) dans le catalogue public. | Requête exposants join editions_exposants (édition publiée, is_validated) ; affichage fiche courte + lien fiche complète. |
| JXP-12 | Identité unique | Un exposant = un `profile_id` = un enregistrement `exposants`. Participation à plusieurs éditions = plusieurs lignes `editions_exposants`, même `exposant_id`. | Contrainte 1:1 profiles–exposants (ou 1:1 user–exposant) ; édition = N participations. |

---

## 3. Besoins non fonctionnels

| Id | Besoin | Critères d'acceptation |
|----|--------|------------------------|
| NFR-JXP-01 | Performance | Chargement fiche exposant < 1 s ; liste répertoire (paginated) < 2 s. |
| NFR-JXP-02 | Sécurité | Données profil au moins niveau Sensitive (2) ; accès en écriture = propriétaire (exposant) ou admin ; lecture répertoire = public ou authentifié selon politique. |
| NFR-JXP-03 | Résidence | Alpha : données dans Supabase (exception pré-COG). Post-alpha : migration SQLite + KindMother (documentée). |
| NFR-JXP-04 | Disponibilité | Aligné sur JayFestival (alpha Supabase). |

---

## 4. Données et champs (alignés Catakana / Supabase)

Les champs ci-dessous correspondent à la table **exposants** (Catakana / alpha Supabase). Détail des types et requêtes dans [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

| Champ (logique) | Usage | Répertoire public |
|-----------------|-------|-------------------|
| id (UUID, FK profiles) | Identifiant unique ; lien 1:1 avec compte utilisateur. | — |
| company_name | Nom de l'entreprise ou de la structure. | Oui |
| stand_name | Nom du stand (optionnel, pour édition). | Selon contexte |
| contact_email | Email de contact. | Selon paramètre |
| contact_phone | Téléphone. | Selon paramètre |
| adresse | Adresse postale. | Optionnel |
| logo_url | URL du logo (Storage). | Oui |
| site_web | Site web. | Oui |
| siret | SIRET (si applicable). | Optionnel |
| secteur / category | Secteur ou catégorie d'activité. | Oui (filtres) |
| description | Description courte (vitrine). | Oui |
| visible_repertoire | Flag : afficher dans le répertoire des exposants. | — |
| created_at, updated_at | Horodatage. | — |

---

## 5. Priorisation (MoSCoW)

### Must have (alpha)

- JXP-01, JXP-02 (création et mise à jour profil).
- JXP-04, JXP-05 (contact, secteur).
- JXP-06, JXP-07, JXP-08 (fiche publique, liste répertoire, fiche détail).
- JXP-09, JXP-10, JXP-11, JXP-12 (cohérence, intégration JayFestival, identité unique).
- NFR-JXP-02, NFR-JXP-03 (sécurité, résidence).

### Should have

- JXP-03 (logo et visuels).
- NFR-JXP-01, NFR-JXP-04.

### Could have

- Portfolio / galerie (visuels additionnels).
- Réseaux sociaux (liens).

---

## 6. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayFestival - Exposants Analyse des besoins](../JayFestival/publics/Exposants/Exposants%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)

---

**Document** : JayXpose — Analyse des besoins  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Référence produit
