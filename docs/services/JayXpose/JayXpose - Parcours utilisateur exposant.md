# JayXpose — Parcours utilisateur exposant

## Contexte

Ce document décrit les **parcours utilisateur exposant** côté **JayXpose** : création du profil, gestion de la fiche entreprise, catalogue de produits, site vitrine, coffre-fort documentaire, fiche publique (annuaire) et interactions avec JayFestival. Il s'appuie sur les écrans et cycles exposant de JayFestival et sur les données alpha Supabase.

**Références** : [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md), [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md), [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md), [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md).

## Portée / Scope

- **Périmètre** : Parcours complets côté profil, catalogue, vitrine, documents, annuaire ; mécaniques de synchronisation JayFestival.
- **Hors périmètre** : Parcours candidatures et participations détaillés (documents JayFestival Exposants) ; facturation (JayKonta).

---

## 1. Vue d'ensemble des parcours

| Phase | Description | Écrans / actions |
|-------|-------------|-------------------|
| **Onboarding** | Création du compte exposant et du profil JayXpose (fiche entreprise). | Inscription exposant → création profiles + exposants. |
| **Fiche entreprise** | Mise à jour complète du profil (identité, contacts, juridique, visuels, réseaux). | Mon compte → Fiche entreprise. |
| **Catalogue** | Gestion des produits : création, modification, catégories, visuels, mise en avant. | Mon catalogue → liste produits, fiche produit, catégories. |
| **Vitrine** | Configuration et publication du site vitrine (pages, personnalisation, SEO, activation). | Ma vitrine → pages, paramètres, prévisualisation, publication. |
| **Documents** | Upload, gestion et partage des documents professionnels. | Mes documents → coffre-fort, upload, partage, alertes. |
| **Fiche publique** | Gestion de la visibilité dans l'annuaire et contrôle des champs affichés. | Ma fiche publique → aperçu, visibilité, champs autorisés. |
| **JayFestival** | Synchronisation : pré-remplissage candidatures, partage documents, historique. | Depuis JayFestival → lecture profil, documents, catalogue. |

---

## 2. Parcours détaillés

### 2.1 Inscription exposant (création du profil JayXpose)

**Contexte** : Premier usage ; l'utilisateur s'inscrit en tant qu'exposant depuis le catalogue JayFestival, depuis l'annuaire, ou depuis une page dédiée.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à l'écran « Créer un compte exposant ». | Lien depuis landing, fiche événement « Candidater », ou annuaire. |
| 2 | Saisie email, mot de passe, confirmation. | Supabase Auth : `signUp` ; création `auth.users`. |
| 3 | Saisie fiche entreprise minimale : nom entreprise, activité/secteur, contact principal (email, téléphone), adresse. | Création `profiles` (user_type = 'exhibitor'). Création `exposants` (id = auth.uid()). |
| 4 | Acceptation CGU + clic « S'inscrire ». | Insert `exposants`. |
| 5 | Redirection vers le dashboard exposant. | Session établie ; l'exposant peut compléter son profil, créer son catalogue, configurer sa vitrine. |

**Mécanique** : À l'inscription, un enregistrement `exposants` est créé avec `id = profile.id` (1:1). Seuls les champs minimaux sont requis à l'inscription ; le reste est complété progressivement.

### 2.2 Fiche entreprise (profil complet)

**Contexte** : Exposant connecté ; il complète ou modifie sa fiche entreprise.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Mon compte » → onglet « Fiche entreprise ». | Navigation menu. |
| 2 | **Section Identité** : Raison sociale, forme juridique, slogan, description courte et longue. | Champs `company_name`, `legal_form`, `slogan`, `description_short`, `description_long`. |
| 3 | **Section Juridique** : SIRET, SIREN, code APE, numéro d'immatriculation. | Champs `siret`, `siren`, `code_ape`, `num_immatriculation`. Validation format SIRET (14 chiffres). |
| 4 | **Section Contacts** : Contact principal (nom, email, téléphone) + contacts facturation et logistique (optionnels). | Champs `contact_*` multiples. |
| 5 | **Section Adresses** : Adresse siège + adresses de correspondance. | Champs `adresse_siege`, `adresse_correspondance`. |
| 6 | **Section Visuels** : Upload logo, bannière. | Stockage (Supabase Storage alpha) ; mise à jour `logo_url`, `banner_url`. |
| 7 | **Section Réseaux sociaux** : Liens Facebook, Instagram, LinkedIn, TikTok, YouTube, Pinterest, X. | Champs `social_*`. |
| 8 | **Section Activité** : Secteur, tags / mots-clés. | Champs `secteur`, `tags`. |
| 9 | Clic « Enregistrer ». | UPDATE `exposants` WHERE id = auth.uid(). |
| 10 | Confirmation visuelle. | Toast de confirmation ; données à jour dans l'annuaire et la vitrine. |

### 2.3 Catalogue de produits

**Contexte** : Exposant connecté ; il gère son catalogue de produits.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Mon catalogue » depuis le dashboard. | Navigation menu. |
| 2 | **Vue liste** : Liste des produits avec nom, catégorie, prix, statut, image principale. | SELECT produits_catalogue WHERE exposant_id = auth.uid(). |
| 3 | **Créer un produit** : Clic « Ajouter un produit ». | Formulaire création. |
| 4 | Saisie : nom, description, prix (optionnel), catégorie, disponibilité. | Champs obligatoires : nom. |
| 5 | Upload visuels (1 à 5 images) ; désigner l'image principale. | INSERT produits_visuels ; Supabase Storage. |
| 6 | Marquer comme « vedette » (optionnel). | Flag `is_featured`. |
| 7 | Clic « Enregistrer ». | INSERT produits_catalogue + produits_visuels. |
| 8 | **Modifier un produit** : Clic sur un produit dans la liste → édition. | UPDATE produits_catalogue. |
| 9 | **Supprimer un produit** : Confirmation → suppression (soft delete). | DELETE ou archive. |
| 10 | **Gérer les catégories** : Créer, renommer, réordonner, supprimer des catégories. | CRUD categories_produits. |

### 2.4 Site vitrine

**Contexte** : Exposant connecté ; il configure et publie son site vitrine.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Ma vitrine » depuis le dashboard. | Navigation menu. |
| 2 | **Paramètres** : Choisir le slug URL, les couleurs, le titre SEO, la meta description. | Champs `vitrine_slug`, `vitrine_colors`, `seo_title`, `seo_description`, `seo_keywords`. |
| 3 | **Page Accueil** : Activation + aperçu (bannière, accroche, produits vedettes). | Composée automatiquement depuis profil + produits vedettes. |
| 4 | **Page Catalogue** : Activation + aperçu (liste filtrée des produits). | Composée depuis `produits_catalogue`. |
| 5 | **Page Présentation** : Rédaction contenu (histoire, savoir-faire, valeurs) via éditeur. | Table `vitrine_pages` (page_type = 'presentation'). |
| 6 | **Page Contact** : Activation + choix des coordonnées affichées. | Coordonnées filtrées par la politique de confidentialité. |
| 7 | **Prévisualisation** : L'exposant visualise sa vitrine telle qu'elle sera vue par un visiteur. | Rendu lecture seule avec données actuelles. |
| 8 | **Publication** : Clic « Publier ma vitrine ». | UPDATE `exposants` SET vitrine_status = 'publiee'. |
| 9 | **Désactivation** : L'exposant peut repasser en brouillon ou suspendre. | UPDATE vitrine_status. |

### 2.5 Documents professionnels (coffre-fort)

**Contexte** : Exposant connecté ; il gère ses documents professionnels.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Mes documents » depuis le dashboard. | Navigation menu. |
| 2 | **Vue coffre-fort** : Liste des documents avec type, nom, statut, date d'expiration, version. | SELECT documents_professionnels WHERE exposant_id = auth.uid(). |
| 3 | **Upload document** : Clic « Ajouter un document » → choix type (RIB, assurance, KBIS…) → upload fichier. | INSERT documents_professionnels ; Supabase Storage (bucket sécurisé). |
| 4 | **Renseigner date d'expiration** (optionnel). | Champ `expires_at`. |
| 5 | **Remplacer un document** (nouvelle version) : Upload nouvelle version → version précédente archivée. | Incrémentation `version` ; archivage de l'ancienne URL. |
| 6 | **Consulter statut** : En attente, validé, expiré, rejeté. | Affichage couleur par statut. |
| 7 | **Alerte expiration** : Notification si un document expire dans 30j / 15j / 7j. | Calcul `expires_at - now()` ; notifications in-app / email. |
| 8 | **Partage pour candidature** : Une demande de partage arrive depuis JayFestival. | Notification « L'organisateur X demande votre attestation d'assurance pour l'édition Y ». |
| 9 | **Accepter / refuser le partage** : Document par document. | INSERT/UPDATE documents_partages (status = accepte / refuse). |
| 10 | **Révoquer un partage** : L'exposant retire l'accès à un document partagé. | UPDATE documents_partages SET status = 'revoque'. |

### 2.6 Fiche publique (annuaire)

**Contexte** : Exposant connecté ; il gère sa visibilité dans l'annuaire.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Ma fiche publique » depuis le dashboard. | Navigation. |
| 2 | **Aperçu** : Affichage lecture seule tel que vu dans l'annuaire. | Rendu avec champs publics uniquement. |
| 3 | **Visibilité annuaire** : Case à cocher « Visible dans l'annuaire ». | UPDATE `exposants` SET visible_annuaire. |
| 4 | **Confidentialité par champ** : Pour chaque champ (email, téléphone, adresse), choisir le niveau de visibilité (public / authentifié / organisateur / privé). | Politique de confidentialité stockée (JSON ou table dédiée). |
| 5 | **Lien vitrine** : Si la vitrine est publiée, le lien est affiché automatiquement. | Lecture vitrine_slug + vitrine_status. |
| 6 | **Enregistrement**. | Sauvegarde politique de confidentialité. |

### 2.7 Synchronisation JayFestival (consommation)

| Cas d'usage | Acteur | Données lues | Source |
|-------------|--------|--------------|--------|
| Liste annuaire global | Visiteur | Exposants visibles (profil + accroche + logo) | `exposants` WHERE visible_annuaire = true. |
| Fiche exposant détaillée | Visiteur | Profil + catalogue (aperçu) + éditions participées | `exposants` + `produits_catalogue` + `editions_exposants`. |
| Fiche exposant (organisateur) | Organisateur | Profil + statut participation + documents partagés | `exposants` + `editions_exposants` + `documents_partages`. |
| Formulaire candidature | Exposant | Pré-remplissage depuis son profil | `exposants` WHERE id = auth.uid(). |
| Demande de documents | Organisateur → Exposant | Liste des documents demandés | Notification → `documents_partages`. |
| Liste exposants par édition | Catalogue | Exposants validés pour l'édition | `exposants` JOIN `editions_exposants`. |
| Catalogue dans JayFestival | Visiteur | Produits de l'exposant (lien ou encart) | `produits_catalogue` WHERE exposant_id = ?. |
| Historique participations | Exposant | Ses éditions passées, en cours, à venir | `editions_exposants` JOIN `editions`. |

---

## 3. Flux résumé (schéma)

```
[Inscription exposant]
    → Auth + profiles (user_type=exhibitor) + exposants (création minimale)
        ↓
[Fiche entreprise]
    → Compléter identité, juridique, contacts, visuels, réseaux
        ↓
[Mon catalogue]
    → Créer produits, catégories, visuels, vedettes
        ↓
[Ma vitrine]
    → Configurer pages, personnaliser, SEO, publier
        ↓
[Mes documents]
    → Uploader RIB, KBIS, assurances... → Coffre-fort sécurisé
        ↓
[Ma fiche publique]
    → Visibilité annuaire + confidentialité par champ
        ↓
[JayFestival]
    → Candidater (pré-rempli) → Partager documents → Participer
    → Annuaire global + par édition → Catalogue visible
```

---

## 4. Références

- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)

---

**Document** : JayXpose — Parcours utilisateur exposant
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Référence produit
