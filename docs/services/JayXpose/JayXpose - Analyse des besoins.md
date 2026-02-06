# JayXpose — Analyse des besoins

## Contexte

Ce document constitue l'**analyse des besoins** exhaustive du service **JayXpose** : profil exposant enrichi, catalogue de produits, site vitrine, coffre-fort documentaire, annuaire des exposants et **synchronisation avec JayFestival** (GFestival). Il reprend et enrichit les besoins alpha (profil basique, répertoire) avec les nouvelles fonctionnalités structurantes.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [JayFestival - Exposants Analyse des besoins](../JayFestival/publics/Exposants/Exposants%20-%20Analyse%20des%20besoins.md), [JayFestival - Reference Base de Donnees](../JayFestival/reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md).

## Portée / Scope

- **Périmètre** : Besoins fonctionnels et non fonctionnels de JayXpose (profil exposant enrichi, catalogue produits, site vitrine, coffre-fort documentaire, annuaire, synchronisation JayFestival) ; données et fiches (alpha Supabase, migration SQLite).
- **Hors périmètre** : Candidatures, participations, facturation (JayFestival / JayKonta) ; spécifications API détaillées (contrats dédiés) ; implémentation.

---

## 1. Profil du public et personas

### 1.1 Définition

Les **exposants** sont des professionnels, entreprises ou associations qui souhaitent **constituer une présence professionnelle en ligne** et/ou **participer à des événements/festivals** en tant qu'exposants. **JayXpose** gère leur **identité professionnelle complète** : profil, catalogue, vitrine, documents, annuaire.

### 1.2 Personas

| Persona | Profil | Besoin JayXpose |
|---------|--------|------------------|
| **Artisan / créateur** | Petit exposant ; 2 à 5 festivals par an ; produits faits main. | Fiche vitrine simple + catalogue produits (5-30 produits) + documents de base (assurance, SIRET). Annuaire. |
| **Entreprise / marque** | Exposant régulier ; 10 à 30 salons/an ; gamme de produits étendue. | Vitrine complète + catalogue riche (50-500 produits) + coffre-fort documentaire complet (KBIS, RIB, assurances). Référencement multi-événements. |
| **Association / collectif** | Occasionnel ; 1 à 3 événements/an ; activité non commerciale ou mixte. | Fiche entreprise + page présentation (histoire, mission) + quelques documents (statuts, assurance). Annuaire. |
| **Food truck / restaurateur** | Itinérant ; 10 à 40 événements/an ; offre alimentaire. | Catalogue menu + documents sanitaires + certificats + référencement JayFaim (phase 2). |
| **Artiste / performer** | Portfolio plutôt que catalogue produit ; prestations sur scène ou en live. | Page présentation riche (portfolio, vidéos) + vitrine sans prix. Documents (licence spectacle, assurance). |

---

## 2. Besoins fonctionnels

### 2.1 Profil exposant (fiche entreprise enrichie)

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-01 | Création du profil exposant | À l'inscription (JayFestival ou autonome), création d'un profil JayXpose : identité, contact, activité. | Données stockées ; un exposant = un profil. |
| JXP-02 | Mise à jour du profil | Modifier les informations de la fiche entreprise (tous champs). | Formulaire « Fiche entreprise » ; sauvegarde sur action « Enregistrer ». |
| JXP-03 | Logo et visuels | Upload logo, bannière, visuels de couverture. | Stockage médias ; URL exposée ; formats (PNG, JPG, WEBP) et taille max (2 Mo logo, 5 Mo bannière). |
| JXP-04 | Données de contact principal | Email, téléphone, adresse postale, site web, réseaux sociaux. | Champs configurables ; visibilité selon confidentialité (public / authentifié / organisateur / privé). |
| JXP-05 | Secteur / activité | Catégorie ou secteur d'activité (liste ou libre) + tags / mots-clés. | Utilisé pour l'annuaire (filtres) et la fiche exposant. |
| JXP-13 | Informations juridiques | Raison sociale, forme juridique, SIRET, SIREN, code APE/NAF, numéro d'immatriculation. | Champs optionnels selon type de structure ; validations de format (SIRET = 14 chiffres). |
| JXP-14 | Contacts multiples | Contact principal + contact facturation + contact logistique (nom, email, téléphone chacun). | Au moins 1 contact obligatoire (principal) ; les autres optionnels. |
| JXP-15 | Adresses multiples | Adresse siège + adresse(s) de correspondance / livraison. | Au moins 1 adresse obligatoire (siège). |
| JXP-16 | Réseaux sociaux | Liens Facebook, Instagram, LinkedIn, TikTok, YouTube, Pinterest, X (Twitter). | Champs URL validés ; affichés sur la vitrine et l'annuaire. |
| JXP-17 | Description enrichie | Description courte (accroche, max 200 caractères) + description longue (texte formaté). | Accroche = annuaire ; description longue = vitrine et fiche détaillée. |
| JXP-18 | Slogan / accroche | Phrase courte résumant l'activité (affichée en bannière vitrine). | Max 100 caractères ; optionnel. |

### 2.2 Catalogue de produits

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-20 | Création fiche produit | L'exposant crée une fiche produit : nom, description, prix (optionnel), catégorie, visuels. | Formulaire création produit ; au moins nom et 1 visuel obligatoires. |
| JXP-21 | Modification fiche produit | Modifier une fiche produit existante. | Formulaire modification ; sauvegarde sur action. |
| JXP-22 | Suppression fiche produit | Supprimer (ou archiver) une fiche produit. | Confirmation avant suppression ; archivage possible (soft delete). |
| JXP-23 | Visuels multiples par produit | Jusqu'à 5 photos par fiche produit (galerie). | Upload multiples ; ordre personnalisable ; image principale désignée. |
| JXP-24 | Catégories / collections | Organiser les produits en catégories ou collections thématiques. | Création, modification, suppression de catégories ; un produit = 1 catégorie minimum. |
| JXP-25 | Produits vedettes | Marquer certains produits comme « vedettes » (mis en avant sur la vitrine). | Flag « vedette » ; max configurable (ex. 6 produits vedettes). |
| JXP-26 | Disponibilité | Indiquer si un produit est disponible, en rupture, sur commande. | Statut affiché sur la fiche produit et le catalogue. |
| JXP-27 | Prix optionnel | Le prix est optionnel (certains exposants ne veulent pas afficher de prix). | Champ prix nullable ; si absent, afficher « Sur demande » ou masquer. |
| JXP-28 | Catalogue public | Le catalogue est consultable depuis la vitrine et depuis le répertoire JayFestival. | Lecture publique des fiches produits pour les exposants ayant un catalogue visible. |
| JXP-29 | Recherche dans le catalogue | Recherche par nom de produit, catégorie, mot-clé. | Filtre texte + filtre catégorie ; résultats paginés. |

### 2.3 Site vitrine

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-30 | Page d'accueil | Bannière, accroche/slogan, produits vedettes, lien vers catalogue et contact. | Composée automatiquement à partir des données profil et catalogue. |
| JXP-31 | Page catalogue | Liste des produits filtrable par catégorie, avec visuels et description courte. | Pagination ; filtres ; clic → fiche produit détaillée. |
| JXP-32 | Page présentation | Histoire, savoir-faire, engagements, valeurs — contenu riche (texte formaté, images, vidéos). | Éditeur de contenu (type CMS simplifié) ; sauvegarde ; prévisualisation. |
| JXP-33 | Page contact | Formulaire de contact + coordonnées affichées selon confidentialité. | Formulaire (nom, email, message) ; envoi notification à l'exposant ; coordonnées filtrées par visibilité. |
| JXP-34 | URL unique (slug) | Chaque exposant a une URL unique personnalisable (ex. `/vitrine/mon-atelier`). | Slug unique validé (alphanumérique + tirets) ; modifiable ; redirection si changement. |
| JXP-35 | Personnalisation visuelle | Couleurs principales (accent, fond), choix de sections affichées, ordre des sections. | Palette configurable ; prévisualisation avant publication. |
| JXP-36 | Responsive | Adapté mobile, tablette, desktop. | Points de rupture standard (mobile < 768px, tablette < 1024px). |
| JXP-37 | SEO basique | Balises title et meta description personnalisables, mots-clés, données structurées (schema.org LocalBusiness). | Champs SEO dans les paramètres vitrine ; rendu côté serveur ou pré-rendu. |
| JXP-38 | Activation / désactivation vitrine | L'exposant peut activer ou désactiver sa vitrine (brouillon / publiée). | Statut vitrine : brouillon, publiée, suspendue. |

### 2.4 Documents professionnels (coffre-fort)

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-40 | Upload document | L'exposant uploade un document professionnel (PDF, image). | Types autorisés : PDF, PNG, JPG ; taille max 10 Mo. |
| JXP-41 | Types de documents | Types prédéfinis : RIB, attestation d'assurance, KBIS, certificat d'immatriculation, licence/autorisation, attestation URSSAF, carte professionnelle, diplôme/certification, autre. | Liste de types avec libellé ; « autre » = champ libre. |
| JXP-42 | Versioning documents | Un document peut être remplacé (nouvelle version) ; l'historique des versions est conservé. | Upload nouvelle version → version précédente archivée ; accès à l'historique. |
| JXP-43 | Statut du document | Chaque document a un statut : en attente, validé, expiré, rejeté. | Statut visible par l'exposant ; modification du statut par administrateur ou organisateur (via Mandat). |
| JXP-44 | Date d'expiration | Certains documents ont une date d'expiration (assurance, KBIS). | Champ date optionnel ; si renseigné, le système calcule le statut « expiré ». |
| JXP-45 | Alertes expiration | Notification à l'exposant avant expiration d'un document (30j, 15j, 7j). | Notifications in-app et/ou email ; configurable par l'exposant. |
| JXP-46 | Partage gouverné | L'exposant accepte de partager un document avec un organisateur JayFestival pour une candidature. | Demande de partage → acceptation/refus par l'exposant → accès temporaire pour l'organisateur (Mandat de Permission). |
| JXP-47 | Centralisation | Un document uploadé une fois sert pour N candidatures / N événements. | Pas de ré-upload ; l'exposant sélectionne un document existant pour chaque demande. |
| JXP-48 | Suppression document | L'exposant peut supprimer un document (sauf s'il est lié à une candidature en cours). | Vérification des liens actifs avant suppression ; archivage sinon. |
| JXP-49 | Consultation organisateur | L'organisateur consulte les documents partagés pour une candidature. | Lecture seule ; pas de téléchargement sauf autorisation explicite (politique à définir). |

### 2.5 Annuaire des exposants (répertoire enrichi)

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-06 | Fiche publique (annuaire) | La fiche exposant est publiée dans l'annuaire des exposants. | Visibilité selon choix exposant (opt-out possible). |
| JXP-07 | Liste annuaire | Affichage de la liste des exposants : nom, secteur, logo, accroche, lien fiche/vitrine. | Exposants avec flag « visible en annuaire » ; filtres ; pagination. |
| JXP-08 | Fiche exposant détaillée | Fiche complète : profil, catalogue (aperçu), éditions participées, lien vitrine. | Données JayXpose + liaison JayFestival (éditions participées). |
| JXP-09 | Cohérence fiche / candidatures | La fiche utilisée pour les candidatures JayFestival = profil JayXpose. | Une source de vérité ; pas de duplication. |
| JXP-50 | Filtres annuaire enrichis | Filtres : secteur, localisation (ville/département/région), mots-clés, type d'activité, événement. | Filtres combinables ; résultats paginés. |
| JXP-51 | Recherche texte | Recherche full-text dans l'annuaire (nom, description, mots-clés, produits). | Résultats pertinents triés par score ; suggestions. |
| JXP-52 | Référencement multi-événements | Un exposant visible dans l'annuaire global ET dans le répertoire par édition JayFestival. | Requête annuaire global ; requête par édition (jointure editions_exposants). |
| JXP-53 | Lien vitrine depuis annuaire | La fiche annuaire affiche un lien vers le site vitrine complet de l'exposant. | Bouton/lien « Voir la vitrine » → URL vitrine. |

### 2.6 Synchronisation JayFestival

| Id | Besoin | Description | Critères d'acceptation |
|----|--------|-------------|------------------------|
| JXP-10 | Fiche exposant côté organisateur | L'organisateur consulte la fiche exposant (profil JayXpose) pour candidatures et participations. | Données exposant depuis JayXpose ; pas de doublon. |
| JXP-11 | Répertoire par événement | Liste des exposants par édition (participations validées) dans le catalogue public. | Requête exposants + editions_exposants (is_validated). |
| JXP-12 | Identité unique | Un exposant = un profil JayXpose = N participations JayFestival. | Contrainte 1:1 profil-exposant. |
| JXP-60 | Pré-remplissage candidatures | Le formulaire candidature JayFestival est pré-rempli depuis le profil JayXpose. | Champs profil injectés automatiquement ; modifiables par l'exposant si besoin. |
| JXP-61 | Demande de documents | L'organisateur demande des documents via JayFestival ; la demande arrive dans JayXpose. | Notification à l'exposant ; liste des documents demandés ; acceptation/refus unitaire. |
| JXP-62 | Historique participations | L'exposant voit la liste de ses participations (passées, en cours, à venir) depuis JayXpose. | Lecture editions_exposants + editions ; affichage chronologique. |
| JXP-63 | Notifications croisées | Notifications JayFestival → JayXpose : acceptation candidature, demande document, changement statut. | Notifications in-app ; optionnel email. |
| JXP-64 | Catalogue dans JayFestival | Les produits de l'exposant sont consultables depuis le répertoire JayFestival. | Lien ou encart catalogue dans la fiche exposant JayFestival. |

---

## 3. Besoins non fonctionnels

| Id | Besoin | Critères d'acceptation |
|----|--------|------------------------|
| NFR-JXP-01 | Performance | Chargement fiche exposant < 1 s ; liste annuaire (paginée) < 2 s ; vitrine < 3 s (premier chargement). |
| NFR-JXP-02 | Sécurité | Profil = Sensitive (2) ; documents = Critical (3) ; vitrine publique = Public (0) à Standard (1). Accès en écriture = propriétaire uniquement. |
| NFR-JXP-03 | Résidence | Alpha : données dans Supabase (exception pré-COG). Post-alpha : migration SQLite + KindMother (documentée). |
| NFR-JXP-04 | Disponibilité | Aligné sur JayFestival (alpha Supabase) ; vitrine cible 99,5 % disponibilité. |
| NFR-JXP-05 | Scalabilité catalogue | Jusqu'à 500 produits par exposant ; 10 000 exposants en annuaire sans dégradation. |
| NFR-JXP-06 | Stockage documents | Max 50 Mo de documents par exposant (alpha) ; extensible. |
| NFR-JXP-07 | Confidentialité | Granularité de visibilité par champ (public, authentifié, organisateur, privé). Aucun champ privé exposé par défaut. |
| NFR-JXP-08 | Accessibilité | Vitrine conforme WCAG 2.1 niveau AA (cible). |
| NFR-JXP-09 | Internationalisation | Contenu exposant en langue libre ; interface JayXpose en français (alpha), i18n prévu. |
| NFR-JXP-10 | Auditabilité | Toute opération sur les documents professionnels est tracée (qui, quand, quoi). Gouvernance WorrySentinel. |

---

## 4. Données et champs

### 4.1 Profil exposant (table `exposants` enrichie)

| Champ (logique) | Type | Usage | Annuaire public | Sécurité |
|-----------------|------|-------|-----------------|----------|
| id | UUID (FK profiles) | Identifiant unique ; lien 1:1 avec compte utilisateur. | — | — |
| company_name | TEXT | Raison sociale / nom de la structure. | Oui | Standard (1) |
| legal_form | TEXT | Forme juridique (SARL, SAS, EI, association…). | Non | Sensitive (2) |
| stand_name | TEXT | Nom du stand (optionnel, par édition). | Selon contexte | Standard (1) |
| slogan | TEXT | Accroche / slogan (max 100 caractères). | Oui | Public (0) |
| description_short | TEXT | Description courte (max 200 caractères). | Oui | Public (0) |
| description_long | TEXT | Description longue (texte formaté). | Vitrine | Standard (1) |
| contact_email | TEXT | Email de contact principal. | Selon confidentialité | Sensitive (2) |
| contact_phone | TEXT | Téléphone principal. | Selon confidentialité | Sensitive (2) |
| adresse_siege | TEXT / JSON | Adresse siège social. | Selon confidentialité | Sensitive (2) |
| adresse_correspondance | TEXT / JSON | Adresse(s) de correspondance. | Non | Sensitive (2) |
| contact_facturation_nom | TEXT | Nom du contact facturation. | Non | Sensitive (2) |
| contact_facturation_email | TEXT | Email facturation. | Non | Sensitive (2) |
| contact_facturation_phone | TEXT | Téléphone facturation. | Non | Sensitive (2) |
| contact_logistique_nom | TEXT | Nom du contact logistique. | Non | Sensitive (2) |
| contact_logistique_email | TEXT | Email logistique. | Non | Sensitive (2) |
| contact_logistique_phone | TEXT | Téléphone logistique. | Non | Sensitive (2) |
| logo_url | TEXT | URL du logo. | Oui | Public (0) |
| banner_url | TEXT | URL de la bannière. | Vitrine | Standard (1) |
| site_web | TEXT | Site web externe. | Oui | Public (0) |
| siret | TEXT (14 chiffres) | SIRET. | Non | Sensitive (2) |
| siren | TEXT (9 chiffres) | SIREN. | Non | Sensitive (2) |
| code_ape | TEXT | Code APE / NAF. | Non | Sensitive (2) |
| num_immatriculation | TEXT | Numéro d'immatriculation (RM, RCS…). | Non | Sensitive (2) |
| secteur | TEXT | Secteur d'activité principal. | Oui (filtre) | Standard (1) |
| tags | TEXT[] / JSON | Mots-clés / tags sectoriels. | Oui (recherche) | Standard (1) |
| social_facebook | TEXT | URL Facebook. | Vitrine | Public (0) |
| social_instagram | TEXT | URL Instagram. | Vitrine | Public (0) |
| social_linkedin | TEXT | URL LinkedIn. | Vitrine | Public (0) |
| social_tiktok | TEXT | URL TikTok. | Vitrine | Public (0) |
| social_youtube | TEXT | URL YouTube. | Vitrine | Public (0) |
| social_pinterest | TEXT | URL Pinterest. | Vitrine | Public (0) |
| social_x | TEXT | URL X (Twitter). | Vitrine | Public (0) |
| visible_annuaire | BOOLEAN | Affiché dans l'annuaire. | — | — |
| vitrine_slug | TEXT | Slug URL de la vitrine. | — | Standard (1) |
| vitrine_status | TEXT | brouillon / publiée / suspendue. | — | Standard (1) |
| vitrine_colors | JSON | Palette de couleurs personnalisée. | — | Standard (1) |
| seo_title | TEXT | Titre SEO personnalisé. | — | Public (0) |
| seo_description | TEXT | Meta description SEO. | — | Public (0) |
| seo_keywords | TEXT | Mots-clés SEO. | — | Public (0) |
| created_at | TIMESTAMPTZ | Création. | — | — |
| updated_at | TIMESTAMPTZ | Dernière mise à jour. | — | — |

### 4.2 Catalogue produits (table `produits_catalogue`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique du produit. |
| exposant_id | UUID (FK exposants) | Propriétaire du produit. |
| name | TEXT | Nom du produit. |
| description | TEXT | Description du produit. |
| price | NUMERIC | Prix (nullable = « Sur demande »). |
| currency | TEXT | Devise (EUR par défaut). |
| category_id | UUID (FK categories_produits) | Catégorie. |
| availability | TEXT | disponible / rupture / sur_commande. |
| is_featured | BOOLEAN | Produit vedette. |
| sort_order | INTEGER | Ordre d'affichage. |
| created_at | TIMESTAMPTZ | Création. |
| updated_at | TIMESTAMPTZ | Dernière mise à jour. |

### 4.3 Catégories produits (table `categories_produits`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique. |
| exposant_id | UUID (FK exposants) | Propriétaire. |
| name | TEXT | Nom de la catégorie. |
| sort_order | INTEGER | Ordre d'affichage. |
| created_at | TIMESTAMPTZ | Création. |

### 4.4 Visuels produits (table `produits_visuels`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique. |
| produit_id | UUID (FK produits_catalogue) | Produit associé. |
| url | TEXT | URL du visuel (Storage). |
| is_primary | BOOLEAN | Image principale. |
| sort_order | INTEGER | Ordre. |
| created_at | TIMESTAMPTZ | Création. |

### 4.5 Documents professionnels (table `documents_professionnels`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique. |
| exposant_id | UUID (FK exposants) | Propriétaire. |
| type | TEXT | rib / assurance / kbis / immatriculation / licence / urssaf / carte_pro / diplome / autre. |
| label | TEXT | Libellé personnalisé (si type = autre). |
| file_url | TEXT | URL du fichier (Storage sécurisé). |
| file_name | TEXT | Nom du fichier original. |
| file_size | INTEGER | Taille en octets. |
| mime_type | TEXT | Type MIME (application/pdf, image/png…). |
| status | TEXT | en_attente / valide / expire / rejete. |
| expires_at | TIMESTAMPTZ | Date d'expiration (nullable). |
| version | INTEGER | Numéro de version (incrémental). |
| uploaded_at | TIMESTAMPTZ | Date d'upload. |
| validated_at | TIMESTAMPTZ | Date de validation (nullable). |
| validated_by | UUID | Validé par (nullable). |

### 4.6 Partages de documents (table `documents_partages`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique. |
| document_id | UUID (FK documents_professionnels) | Document partagé. |
| exposant_id | UUID (FK exposants) | Exposant propriétaire. |
| target_user_id | UUID | Organisateur ou service destinataire. |
| target_context | TEXT | Contexte (ex. candidature_id, edition_id). |
| status | TEXT | demande / accepte / refuse / revoque. |
| requested_at | TIMESTAMPTZ | Date de demande. |
| responded_at | TIMESTAMPTZ | Date de réponse. |
| expires_at | TIMESTAMPTZ | Expiration du partage. |

### 4.7 Contenu vitrine (table `vitrine_pages`)

| Champ | Type | Usage |
|-------|------|-------|
| id | UUID (PK) | Identifiant unique. |
| exposant_id | UUID (FK exposants) | Propriétaire. |
| page_type | TEXT | accueil / presentation / contact. |
| content | TEXT / JSON | Contenu de la page (texte formaté / blocs). |
| is_visible | BOOLEAN | Page activée. |
| sort_order | INTEGER | Ordre dans la navigation. |
| updated_at | TIMESTAMPTZ | Dernière mise à jour. |

---

## 5. Priorisation (MoSCoW)

### Must have (alpha)

- JXP-01, JXP-02 (création et mise à jour profil).
- JXP-04, JXP-05 (contact, secteur).
- JXP-06, JXP-07, JXP-08 (fiche publique, liste annuaire, fiche détail).
- JXP-09, JXP-10, JXP-11, JXP-12 (cohérence, intégration JayFestival, identité unique).
- JXP-13, JXP-14 (informations juridiques, contacts multiples).
- JXP-20, JXP-21, JXP-24, JXP-28 (catalogue basique : créer, modifier, catégories, catalogue public).
- JXP-40, JXP-41, JXP-43, JXP-46, JXP-47 (coffre-fort : upload, types, statuts, partage gouverné, centralisation).
- JXP-60, JXP-61 (pré-remplissage candidatures, demande documents).
- NFR-JXP-02, NFR-JXP-03, NFR-JXP-07, NFR-JXP-10 (sécurité, résidence, confidentialité, auditabilité).

### Should have

- JXP-03, JXP-15, JXP-16, JXP-17, JXP-18 (visuels enrichis, adresses multiples, réseaux sociaux, descriptions, slogan).
- JXP-22, JXP-23, JXP-25, JXP-26, JXP-27, JXP-29 (catalogue complet).
- JXP-30, JXP-31, JXP-32, JXP-33, JXP-34 (vitrine : pages, URL).
- JXP-42, JXP-44, JXP-45 (versioning documents, expiration, alertes).
- JXP-50, JXP-51, JXP-52, JXP-53 (annuaire enrichi).
- JXP-62, JXP-63, JXP-64 (historique, notifications, catalogue dans JayFestival).
- NFR-JXP-01, NFR-JXP-04, NFR-JXP-05.

### Could have

- JXP-35, JXP-36, JXP-37, JXP-38 (personnalisation vitrine, responsive, SEO, activation).
- JXP-48, JXP-49 (suppression document, consultation organisateur).
- NFR-JXP-06, NFR-JXP-08, NFR-JXP-09.

### Won't have (cette version)

- Avis / témoignages sur la fiche exposant.
- Mise en avant / boost payant dans l'annuaire.
- Carte géographique des exposants.
- Boutique en ligne intégrée (achat direct) — renvoi vers Miyustore si besoin.

---

## 6. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md)
- [JayFestival - Exposants Analyse des besoins](../JayFestival/publics/Exposants/Exposants%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)

---

**Document** : JayXpose — Analyse des besoins
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Référence produit
