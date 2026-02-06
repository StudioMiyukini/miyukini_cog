# JayXpose — Document fondateur

## Contexte

**JayXpose** est le service Miyukini dédié à l'**identité professionnelle de l'exposant** : profil complet, site vitrine, catalogue de produits, coffre-fort documentaire et référencement dans l'annuaire des exposants. Il permet à un utilisateur de **devenir exposant** en constituant une présence en ligne complète (catalogue, présentation, coordonnées, documents réglementaires) et de **centraliser toutes ses informations professionnelles** pour les exploiter au sein de l'écosystème Jay — en particulier **JayFestival** (GFestival).

Ce document est le **document fondateur** du service : il en fixe la raison d'être, la portée, les principes directeurs, les fonctionnalités structurantes et l'intégration avec les autres services Jay. Il s'adresse aux équipes produit, technique et aux parties prenantes.

## Portée / Scope

- **Périmètre** : Définition du service JayXpose, positionnement, fonctionnalités (profil, catalogue, vitrine, documents professionnels, annuaire), intégration avec JayFestival et les services Jay, vitrine autonome.
- **Hors périmètre** : Spécifications techniques détaillées, contrats d'API, implémentation (référencés dans d'autres documents).
- **Références** : Glossaire Miyukini, document fondateur JayFestival, [Interpolarité des services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

## 1. Raison d'être

### 1.1 Proposition de valeur

**JayXpose** permet à des **exposants** (artisans, artistes, petites marques, entreprises, associations) de :

- **Constituer un profil exposant complet** : identité juridique, contacts multiples, secteur d'activité, visuels, réseaux sociaux.
- **Gérer un catalogue de produits** : fiches produits, catégories, galerie photo, produits mis en avant.
- **Publier un site vitrine** : page d'accueil, page catalogue, page présentation, page contact — avec URL unique et personnalisation visuelle.
- **Renseigner et centraliser ses documents professionnels** : RIB, attestation d'assurance, KBIS, certificat d'immatriculation, licence, attestation URSSAF, carte professionnelle, diplômes — dans un coffre-fort sécurisé.
- **Être référencé dans l'annuaire des exposants** : fiche annuaire enrichie, filtres sectoriels et géographiques, multi-événements.
- **Se synchroniser avec JayFestival** (GFestival) : profil unique exploité pour les candidatures, documents partagés à la demande, historique des participations, notifications croisées.

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **Intégré JayFestival** | Profil JayXpose alimente la fiche exposant, le répertoire et les candidatures de JayFestival ; un exposant participe à plusieurs éditions avec le même profil et les mêmes documents. |
| **Vitrine autonome** | Site vitrine complet, sans événement festival ; utile pour artisans ou marques qui veulent une présence en ligne permanente, avec catalogue et contact. |

### 1.3 Phrase fondatrice

> **JayXpose est l'identité professionnelle de l'exposant dans l'écosystème Miyukini. Un profil, un catalogue, une vitrine, un coffre-fort — exploitables partout, maîtrisés par l'exposant.**

---

## 2. Fonctionnalités structurantes

### 2.1 Profil exposant enrichi

| Fonctionnalité | Description |
|----------------|-------------|
| Fiche entreprise complète | Raison sociale, forme juridique, adresse siège, adresse(s) de correspondance, SIRET, SIREN, code APE/NAF, numéro d'immatriculation. |
| Contacts multiples | Contact principal, contact facturation, contact logistique — chacun avec nom, email, téléphone. |
| Identité visuelle | Logo, bannière, visuels de couverture. |
| Description et positionnement | Description activité, mots-clés, tags sectoriels, slogan/accroche. |
| Réseaux sociaux | Facebook, Instagram, LinkedIn, TikTok, YouTube, site web externe. |
| Confidentialité granulaire | Chaque champ contrôlé : public, authentifié, organisateur seul, JayXpose seul. |

### 2.2 Catalogue de produits

| Fonctionnalité | Description |
|----------------|-------------|
| Fiches produits | Nom, description, prix (optionnel), catégorie, visuels multiples, disponibilité. |
| Catégories / collections | Organisation hiérarchique ou par collections thématiques. |
| Produits vedettes | Mise en avant de produits sur la vitrine et l'annuaire. |
| Galerie intégrée | Galerie produits consultable depuis le site vitrine et depuis JayFestival. |

### 2.3 Site vitrine

| Fonctionnalité | Description |
|----------------|-------------|
| Pages | Accueil (bannière, accroche, produits vedettes), Catalogue (liste filtrable), Présentation (histoire, savoir-faire, valeurs), Contact (formulaire, coordonnées). |
| URL unique | Slug personnalisable par exposant (ex. `vitrine.jay/mon-atelier`). |
| Personnalisation | Couleurs, mise en page minimale, choix de sections affichées. |
| SEO | Balises title/description, mots-clés, données structurées. |
| Responsive | Adapté mobile, tablette, desktop. |

### 2.4 Documents professionnels (coffre-fort)

| Fonctionnalité | Description |
|----------------|-------------|
| Types de documents | RIB, attestation d'assurance, KBIS, certificat d'immatriculation, licence/autorisation, attestation URSSAF, carte professionnelle, diplômes/certifications. |
| Stockage sécurisé | Upload, horodatage, versioning. Niveau de sécurité **Critical (3)** minimum. |
| Statuts | En attente, validé, expiré, rejeté. |
| Alertes expiration | Notification avant expiration (assurance, KBIS, etc.). |
| Partage gouverné | L'exposant partage un document avec un organisateur JayFestival via un Mandat de Permission. Document par document, acceptation explicite. |
| Centralisation | Un document uploadé une fois sert pour N candidatures / N événements. |

### 2.5 Annuaire des exposants

| Fonctionnalité | Description |
|----------------|-------------|
| Fiche annuaire | Logo, nom, secteur, description, lien vitrine, localisation. |
| Filtres | Secteur, localisation, mots-clés, type d'activité, événement. |
| Multi-événements | Un exposant visible dans l'annuaire global et par édition JayFestival. |
| Inscription | Automatique à la création du profil (opt-out possible). |

### 2.6 Synchronisation JayFestival

| Fonctionnalité | Description |
|----------------|-------------|
| Profil unique | Pas de duplication ; JayFestival lit les données JayXpose. |
| Pré-remplissage candidatures | Formulaire candidature pré-rempli depuis le profil JayXpose. |
| Partage documents | Organisateur demande des documents ; exposant accepte le partage. |
| Catalogue visible | Produits de l'exposant consultables depuis le répertoire JayFestival. |
| Historique participations | Éditions passées, en cours, à venir. |
| Notifications croisées | Acceptation candidature, demande de document, changement de statut. |

---

## 3. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance COG** | Le service fonctionne sous gouvernance COG : StrongFather (décisions), KindMother (persistance), Master Butler (capacités/permissions), WorrySentinel (sécurité), Ever Buddy (cycle de vie). |
| **Souveraineté de l'exposant** | L'exposant est propriétaire de ses données. Il contrôle la visibilité de chaque champ et le partage de chaque document. Aucun partage implicite. |
| **Réutilisabilité** | S'appuyer sur les Kits d'outils Miyukini existants (Miyauth, Miyuprofile, Miyucms, Miyumedia, Miyucontacts) et définir les Opérateurs et Kits spécifiques. |
| **Interpolarité** | Conçu pour s'intégrer dans JayFestival et les services Jay. Les couplages sont explicites et gouvernés (Mandats de Permission, niveaux de sécurité). |
| **Centralisation** | Un profil, un catalogue, un coffre-fort — exploitables depuis n'importe quel service Jay sans duplication. |
| **Confidentialité par défaut** | Les informations sensibles (documents, RIB, contacts privés) ne sont jamais exposées par défaut. Le partage est un acte explicite et traçable. |

---

## 4. Intégration et interpolarité

### 4.1 JayXpose dans JayFestival

- La **fiche exposant** de JayFestival s'appuie sur le profil JayXpose (données vitrine, catalogue, contact).
- Le **répertoire des exposants** (annuaire plateforme ou par événement) affiche les fiches JayXpose enrichies (avec catalogue).
- Un **exposant** participe à des éditions JayFestival avec le même profil ; pas de duplication d'identité ni de contenu.
- Les **documents professionnels** sont partagés à la demande pour les candidatures (partage gouverné, document par document).
- Les **candidatures** sont pré-remplies depuis le profil JayXpose.

### 4.2 JayXpose et les autres services Jay

| Service | Intégration |
|---------|-------------|
| **JayFestival** | Profil, catalogue, documents, répertoire, candidatures, notifications. |
| **JayKonta** | Facturation exposant ; RIB partagé depuis le coffre-fort JayXpose (avec Mandat). |
| **JayRDV** | Lien depuis la vitrine vers la prise de rendez-vous. |
| **JayKoa** | Agenda des événements auxquels l'exposant participe. |
| **JayFaim** | Phase 2 — si l'exposant propose de la restauration. |

### 4.3 Vitrine autonome

- JayXpose peut être utilisé **sans JayFestival** : site vitrine complet, catalogue, contact.
- Les données (profil, contenu vitrine, catalogue) sont gouvernées ; la résidence et le niveau de sécurité sont définis par le contrat du service.

### 4.4 Référence interpolarité

Voir [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) pour le principe global et les couplages entre services Jay.

---

## 5. Niveaux de sécurité (orientation)

| Catégorie de données | Niveau | Justification |
|----------------------|--------|---------------|
| Site vitrine (pages publiques) | **Public (0)** à **Standard (1)** | Contenu destiné à être visible par tous. |
| Profil entreprise (nom, secteur, description) | **Standard (1)** à **Sensitive (2)** | Données commerciales identifiantes. |
| Contacts (email, téléphone, adresse) | **Sensitive (2)** | Données personnelles / commerciales. |
| Catalogue produits | **Standard (1)** à **Sensitive (2)** | Selon prix et stratégie commerciale. |
| Documents professionnels (RIB, KBIS, assurances) | **Critical (3)** | Documents réglementaires et financiers. |
| Identifiants (SIRET, SIREN, immatriculation) | **Sensitive (2)** à **Critical (3)** | Données d'identification légale. |

Détail dans [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

---

## 6. Prochaines étapes (orientation)

1. **Fonder** : Valider ce document fondateur enrichi et le diffuser.
2. **Spécifier** : Documenter les besoins enrichis (catalogue, documents, vitrine), les Opérateurs et Kits.
3. **Intégration** : Formaliser les contrats de synchronisation avec JayFestival (profil, documents, notifications).
4. **Sécurité** : Formaliser les niveaux de sécurité et la politique de confidentialité inter-services.
5. **Implémentation** : Développer les Opérateurs et Kits en s'appuyant sur les Cores.

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Opérateur, Mandat, COG, Niveaux de sécurité). |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service dans lequel JayXpose s'intègre. |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | Principe d'interpolarité et couplage JayXpose / JayFestival. |
| [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non fonctionnels détaillés. |
| [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md) | Spécification du module catalogue. |
| [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md) | Spécification du coffre-fort documentaire. |
| [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md) | Spécification du site vitrine complet. |
| [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md) | Politique de confidentialité et partage gouverné. |
| [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md) | Contrat d'intégration détaillé avec JayFestival. |

---

**Document** : JayXpose — Document fondateur
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document de référence — non contractuel pour l'implémentation.
