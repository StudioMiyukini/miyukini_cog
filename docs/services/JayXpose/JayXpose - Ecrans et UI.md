# JayXpose — Écrans et UI

## Contexte

Ce document précise les **écrans et composants UI** de **JayXpose** : profil exposant enrichi, catalogue de produits, site vitrine, coffre-fort documentaire, fiche publique (annuaire) et **intégration dans les écrans JayFestival**. Il s'appuie sur le [Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md) et sur les écrans exposant de JayFestival.

**Références** : [JayFestival - Specification UI Conforme Catakana](../JayFestival/JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md).

## Portée / Scope

- **Périmètre** : Écrans et zones UI propres à JayXpose (profil, catalogue, vitrine, documents, annuaire) ; intégration dans les écrans JayFestival.
- **Hors périmètre** : Maquettes pixel ; stack egui détaillée (voir Specification UI Conforme Catakana).

---

## 1. Principe : écrans JayXpose

| Type | Description |
|------|-------------|
| **Écrans « Dashboard »** | Tableau de bord exposant : accès rapide à tous les modules (profil, catalogue, vitrine, documents, fiche publique). |
| **Écrans « Profil »** | Gestion de la fiche entreprise enrichie. |
| **Écrans « Catalogue »** | Gestion des produits, catégories, visuels. |
| **Écrans « Vitrine »** | Configuration, édition de contenu, prévisualisation, publication. |
| **Écrans « Documents »** | Coffre-fort documentaire : upload, statuts, alertes, partage. |
| **Écrans « Fiche publique »** | Gestion de la visibilité annuaire et confidentialité. |
| **Écrans JayFestival** | Consommation des données JayXpose (annuaire, fiche exposant, catalogue). |

---

## 2. Écrans JayXpose — Espace exposant

### 2.1 Dashboard exposant (XP-E01)

| Zone | Composants | Données |
|------|------------|---------|
| En-tête | Logo exposant, nom entreprise, statut vitrine (brouillon/publiée). | `exposants.logo_url`, `company_name`, `vitrine_status`. |
| Résumé profil | Jauge de complétion du profil (%) ; alertes (documents expirants, champs manquants). | Calcul depuis `exposants` + `documents_professionnels`. |
| Accès rapides | Cartes : « Fiche entreprise », « Mon catalogue (N produits) », « Ma vitrine », « Mes documents (N) », « Ma fiche publique ». | Compteurs depuis tables JayXpose. |
| Notifications | Liste des notifications récentes (demandes de partage, changements de statut JayFestival). | Notifications internes. |

### 2.2 Fiche entreprise (XP-E02)

| Zone | Composants | Données |
|------|------------|---------|
| Section Identité | Input (raison sociale), Input (forme juridique), Input (slogan), Textarea (description courte), Textarea (description longue). | `company_name`, `legal_form`, `slogan`, `description_short`, `description_long`. |
| Section Juridique | Input (SIRET), Input (SIREN), Input (code APE), Input (n° immatriculation). | `siret`, `siren`, `code_ape`, `num_immatriculation`. |
| Section Contact principal | Input (email), Input (téléphone), Textarea (adresse siège). | `contact_email`, `contact_phone`, `adresse_siege`. |
| Section Contact facturation | Input (nom), Input (email), Input (téléphone). Mention « optionnel ». | `contact_facturation_*`. |
| Section Contact logistique | Input (nom), Input (email), Input (téléphone). Mention « optionnel ». | `contact_logistique_*`. |
| Section Adresses | Textarea (adresse correspondance). Bouton « Ajouter une adresse ». | `adresse_correspondance`. |
| Section Visuels | Upload logo (aperçu), Upload bannière (aperçu). | `logo_url`, `banner_url`. |
| Section Réseaux sociaux | Input (Facebook), Input (Instagram), Input (LinkedIn), Input (TikTok), Input (YouTube), Input (Pinterest), Input (X). | `social_*`. |
| Section Activité | Input (secteur), Input tags (mots-clés, autocomplétion). | `secteur`, `tags`. |
| Actions | Button « Enregistrer ». | UPDATE `exposants`. |

**Comportement** : Chargement au montage ; validation format SIRET (14 chiffres) et URLs ; sauvegarde complète sur clic « Enregistrer ».

### 2.3 Mon catalogue — Liste des produits (XP-E03)

| Zone | Composants | Données |
|------|------------|---------|
| En-tête | Label « Mon catalogue » ; compteur « N produits ». Button « Ajouter un produit ». | COUNT produits_catalogue. |
| Filtres | Select (catégorie), Input (recherche). | Filtre local. |
| Liste | Lignes/cartes : image principale, nom, catégorie, prix (ou « Sur demande »), disponibilité (badge), vedette (étoile). Actions : Modifier, Supprimer. | `produits_catalogue` + `produits_visuels` (is_primary). |
| Catégories | Sidebar ou onglets : liste des catégories. Bouton « Gérer les catégories ». | `categories_produits`. |
| Pagination | Précédent / Suivant. | LIMIT/OFFSET. |

### 2.4 Fiche produit — Création / Modification (XP-E04)

| Zone | Composants | Données |
|------|------------|---------|
| Formulaire | Input (nom), Textarea (description), Input (prix, optionnel), Select (catégorie), Select (disponibilité : disponible / rupture / sur_commande). | `produits_catalogue.*`. |
| Visuels | Upload multiples (jusqu'à 5) ; drag & drop pour réordonner ; désigner image principale. | `produits_visuels`. |
| Vedette | Checkbox « Produit vedette ». | `is_featured`. |
| Actions | Button « Enregistrer », Button « Annuler ». Si modification : Button « Supprimer » (avec confirmation). | INSERT/UPDATE/DELETE. |

### 2.5 Gestion des catégories (XP-E05)

| Zone | Composants | Données |
|------|------------|---------|
| Liste | Lignes : nom catégorie, nombre de produits. Actions : Renommer, Supprimer, Réordonner (drag & drop). | `categories_produits`. |
| Ajout | Input (nom) + Button « Ajouter ». | INSERT categories_produits. |

### 2.6 Ma vitrine — Paramètres (XP-E06)

| Zone | Composants | Données |
|------|------------|---------|
| URL | Input (slug), aperçu URL complète. | `vitrine_slug`. |
| Personnalisation | Color picker (couleur accent), Color picker (couleur fond). | `vitrine_colors`. |
| SEO | Input (titre SEO), Input (meta description), Input (mots-clés). | `seo_title`, `seo_description`, `seo_keywords`. |
| Pages | Liste des pages avec toggle activation : Accueil, Catalogue, Présentation, Contact. | `vitrine_pages.is_visible`. |
| Statut | Badge (brouillon / publiée / suspendue). Buttons « Publier » / « Suspendre » / « Passer en brouillon ». | `vitrine_status`. |
| Actions | Button « Enregistrer les paramètres ». | UPDATE `exposants` + `vitrine_pages`. |

### 2.7 Ma vitrine — Page Présentation (XP-E07)

| Zone | Composants | Données |
|------|------------|---------|
| Éditeur | Éditeur de contenu riche (texte formaté, images, vidéos intégrées). | `vitrine_pages.content` (page_type = 'presentation'). |
| Aperçu | Rendu lecture seule à côté ou en dessous. | Même données, rendu final. |
| Actions | Button « Enregistrer ». | UPDATE `vitrine_pages`. |

### 2.8 Ma vitrine — Prévisualisation (XP-E08)

| Zone | Composants | Données |
|------|------------|---------|
| Rendu complet | Affichage complet de la vitrine telle que vue par un visiteur : accueil, catalogue, présentation, contact. | Toutes données exposant + catalogue + vitrine_pages. |
| Navigation | Onglets ou liens entre les pages de la vitrine. | — |
| Actions | Button « Retour aux paramètres », Button « Publier ». | — |

### 2.9 Mes documents — Coffre-fort (XP-E09)

| Zone | Composants | Données |
|------|------------|---------|
| En-tête | Label « Mes documents » ; compteur « N documents ». Button « Ajouter un document ». | COUNT documents_professionnels. |
| Alertes | Bandeau si documents expirants (dans 30j). | `expires_at` - now(). |
| Liste | Lignes : icône type, libellé, nom fichier, statut (badge couleur : vert=validé, orange=en attente, rouge=expiré/rejeté), date expiration, version. Actions : Voir, Remplacer, Supprimer. | `documents_professionnels`. |
| Partages actifs | Section : liste des partages en cours (document, destinataire, contexte, statut). Actions : Révoquer. | `documents_partages` WHERE exposant_id = auth.uid() AND status = 'accepte'. |

### 2.10 Upload document (XP-E10)

| Zone | Composants | Données |
|------|------------|---------|
| Type | Select (RIB, Assurance, KBIS, Immatriculation, Licence, URSSAF, Carte pro, Diplôme, Autre). Si « Autre » : Input (libellé). | `type`, `label`. |
| Fichier | Upload (drag & drop ou sélection). Formats : PDF, PNG, JPG. Max 10 Mo. | `file_url`, `file_name`, `file_size`, `mime_type`. |
| Expiration | Date picker (optionnel) « Date d'expiration ». | `expires_at`. |
| Actions | Button « Enregistrer ». | INSERT documents_professionnels. |

### 2.11 Demande de partage (XP-E11)

| Zone | Composants | Données |
|------|------------|---------|
| Notification | « L'organisateur X demande votre [type document] pour l'édition Y ». | `documents_partages` WHERE status = 'demande'. |
| Document | Aperçu du document demandé (type, nom, statut). | `documents_professionnels`. |
| Actions | Button « Accepter le partage », Button « Refuser ». | UPDATE documents_partages SET status = 'accepte' / 'refuse'. |

### 2.12 Ma fiche publique (XP-E12)

| Zone | Composants | Données |
|------|------------|---------|
| Aperçu | Rendu lecture seule : fiche telle que vue dans l'annuaire (nom, logo, secteur, description, lien vitrine). | Données `exposants` filtrées par politique de confidentialité. |
| Visibilité | Checkbox « Visible dans l'annuaire ». | `visible_annuaire`. |
| Confidentialité | Pour chaque champ sensible (email, téléphone, adresse) : Select (public / authentifié / organisateur / privé). | Politique de confidentialité (JSON ou table). |
| Lien vitrine | Affichage automatique si vitrine publiée. | `vitrine_slug` + `vitrine_status`. |
| Actions | Button « Enregistrer ». | UPDATE politique + `visible_annuaire`. |

---

## 3. Écrans publics — Annuaire et Vitrine

### 3.1 Annuaire des exposants (PUB-E01)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Filtres | Input recherche, Select (secteur), Select (localisation), Select (événement, si JayFestival). | — |
| Liste | Cartes : logo, nom entreprise, secteur, description courte, lien « Voir la fiche » / « Voir la vitrine ». | SELECT exposants WHERE visible_annuaire = true + filtres. |
| Pagination | Précédent / Suivant, « Page n / N ». | LIMIT/OFFSET. |

### 3.2 Fiche exposant — Détail annuaire (PUB-E02)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Bloc Identité | Logo, bannière, nom entreprise, slogan, secteur, description. | `exposants` : champs publics. |
| Bloc Catalogue | Aperçu produits vedettes (jusqu'à 6). Button « Voir le catalogue complet ». | `produits_catalogue` WHERE is_featured AND exposant_id = ?. |
| Bloc Participations | « Éditions participées » : liste liens vers fiches événement JayFestival. | `editions_exposants` JOIN `editions` WHERE is_validated = true. |
| Bloc Contact | Coordonnées selon confidentialité. Lien vitrine. | `exposants` : champs filtrés par politique. |
| Pied | Button « Retour à l'annuaire ». | — |

### 3.3 Site vitrine — Page Accueil (PUB-E03)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Bannière | Image bannière, nom entreprise, slogan. | `banner_url`, `company_name`, `slogan`. |
| Produits vedettes | Grille (jusqu'à 6 produits vedettes) : image, nom, prix. | `produits_catalogue` WHERE is_featured. |
| Présentation courte | Description courte + lien « En savoir plus ». | `description_short`. |
| Navigation | Liens : Catalogue, Présentation, Contact. | Pages vitrine activées. |

### 3.4 Site vitrine — Page Catalogue (PUB-E04)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Filtres | Select (catégorie), Input (recherche). | `categories_produits`. |
| Grille produits | Cartes : image principale, nom, prix (ou « Sur demande »), badge disponibilité. Clic → fiche produit. | `produits_catalogue` + `produits_visuels`. |
| Fiche produit (modale ou page) | Galerie images, nom, description, prix, catégorie, disponibilité. | Fiche complète. |
| Pagination | Précédent / Suivant. | LIMIT/OFFSET. |

### 3.5 Site vitrine — Page Présentation (PUB-E05)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Contenu | Rendu du contenu riche (texte, images, vidéos). | `vitrine_pages.content` (page_type = 'presentation'). |

### 3.6 Site vitrine — Page Contact (PUB-E06)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Coordonnées | Affichage coordonnées selon confidentialité (email, téléphone, adresse, réseaux sociaux, site web). | `exposants` : champs filtrés. |
| Formulaire | Input (nom), Input (email), Textarea (message), Button « Envoyer ». | Notification à l'exposant. |

---

## 4. Intégration dans les écrans JayFestival

### 4.1 Répertoire exposants (UNC-E08)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Filtres | Input recherche, Select (secteur), Select (événement). | — |
| Liste | Cartes : logo, nom, secteur, description courte, Button « Voir la fiche ». | `exposants` WHERE visible_annuaire = true (+ jointure édition si filtre). |

**Contrat** : Même données que l'annuaire JayXpose ; présenté dans le contexte JayFestival.

### 4.2 Fiche exposant détail (UNC-E09)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Identité | Logo, nom, description, secteur. | `exposants`. |
| Catalogue aperçu | Produits vedettes (3-6). Lien « Voir la vitrine ». | `produits_catalogue` WHERE is_featured. |
| Éditions participées | Liste éditions. | `editions_exposants`. |
| Contact | Coordonnées filtrées. | `exposants` : champs selon confidentialité. |

### 4.3 Fiche exposant côté organisateur (ORG-E11)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Identité | Nom, contact, secteur, informations juridiques (SIRET, SIREN si partagés). | `exposants`. |
| Documents partagés | Liste des documents partagés pour cette candidature (type, statut, date). | `documents_partages` + `documents_professionnels`. |
| Statut / Emplacement / Historique | (JayFestival). | — |

### 4.4 Formulaire candidature (EXP-E10) — Pré-remplissage

| Donnée | Source | Comportement |
|--------|--------|--------------|
| Nom entreprise, contact, activité, logo | `exposants` WHERE id = auth.uid() | Pré-remplis (éditables ou lecture selon config organisateur). |
| Documents demandés | Liste de types requis par l'organisateur | Bouton « Partager depuis mon coffre-fort » → sélection document existant. |

---

## 5. Composants UI réutilisables

| Composant | Usage | Données |
|-----------|-------|---------|
| **Carte exposant (annuaire)** | Annuaire PUB-E01, UNC-E08, liste par édition. | logo_url, company_name, secteur, description_short ; id pour lien. |
| **Bloc fiche exposant (détail)** | PUB-E02, UNC-E09, ORG-E11 (bloc identité). | Champs publics exposants + catalogue aperçu + éditions. |
| **Carte produit** | Catalogue PUB-E04, XP-E03, aperçu vedettes. | Image principale, nom, prix, disponibilité. |
| **Formulaire fiche entreprise** | XP-E02. | Tous champs exposants. |
| **Formulaire produit** | XP-E04. | Champs produit + visuels. |
| **Ligne document** | XP-E09. | Type, nom, statut (badge), expiration, version. |
| **Bandeau alerte expiration** | XP-E09 (en-tête). | Nombre de documents expirants. |
| **Sélecteur confidentialité** | XP-E12. | Select (public / authentifié / organisateur / privé) par champ. |

---

## 6. Checklist implémentation UI

### Espace exposant
- [ ] Dashboard exposant (XP-E01).
- [ ] Fiche entreprise enrichie (XP-E02).
- [ ] Liste catalogue (XP-E03).
- [ ] Fiche produit création/modification (XP-E04).
- [ ] Gestion catégories (XP-E05).
- [ ] Vitrine paramètres (XP-E06).
- [ ] Vitrine page présentation (XP-E07).
- [ ] Vitrine prévisualisation (XP-E08).
- [ ] Coffre-fort documents (XP-E09).
- [ ] Upload document (XP-E10).
- [ ] Demande de partage (XP-E11).
- [ ] Fiche publique / annuaire (XP-E12).

### Écrans publics
- [ ] Annuaire exposants (PUB-E01).
- [ ] Fiche exposant détail annuaire (PUB-E02).
- [ ] Vitrine accueil (PUB-E03).
- [ ] Vitrine catalogue (PUB-E04).
- [ ] Vitrine présentation (PUB-E05).
- [ ] Vitrine contact (PUB-E06).

### Intégration JayFestival
- [ ] Répertoire exposants (UNC-E08) : données JayXpose enrichies.
- [ ] Fiche exposant détail (UNC-E09) : catalogue aperçu.
- [ ] Fiche exposant organisateur (ORG-E11) : documents partagés.
- [ ] Formulaire candidature (EXP-E10) : pré-remplissage + partage documents.

### Composants
- [ ] Carte exposant, Carte produit, Bloc fiche exposant, Formulaire fiche entreprise, Formulaire produit, Ligne document, Bandeau alerte, Sélecteur confidentialité.

---

## 7. Références

- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [JayFestival - Specification UI Conforme Catakana](../JayFestival/JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md)
- [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)

---

**Document** : JayXpose — Écrans et UI
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Référence produit
