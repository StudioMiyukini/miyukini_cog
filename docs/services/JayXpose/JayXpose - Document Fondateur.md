# JayXpose â€” Document fondateur

## Contexte

**JayXpose** est le service Miyukini dÃ©diÃ© Ã  l'**identitÃ© professionnelle de l'exposant** : profil complet, site vitrine, catalogue de produits, coffre-fort documentaire et rÃ©fÃ©rencement dans l'annuaire des exposants. Il permet Ã  un utilisateur de **devenir exposant** en constituant une prÃ©sence en ligne complÃ¨te (catalogue, prÃ©sentation, coordonnÃ©es, documents rÃ©glementaires) et de **centraliser toutes ses informations professionnelles** pour les exploiter au sein de l'Ã©cosystÃ¨me Jay â€” en particulier **JayFestival** (GFestival).

Ce document est le **document fondateur** du service : il en fixe la raison d'Ãªtre, la portÃ©e, les principes directeurs, les fonctionnalitÃ©s structurantes et l'intÃ©gration avec les autres services Jay. Il s'adresse aux Ã©quipes produit, technique et aux parties prenantes.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : DÃ©finition du service JayXpose, positionnement, fonctionnalitÃ©s (profil, catalogue, vitrine, documents professionnels, annuaire), intÃ©gration avec JayFestival et les services Jay, vitrine autonome.
- **Hors pÃ©rimÃ¨tre** : SpÃ©cifications techniques dÃ©taillÃ©es, contrats d'API, implÃ©mentation (rÃ©fÃ©rencÃ©s dans d'autres documents).
- **RÃ©fÃ©rences** : Glossaire Miyukini, document fondateur JayFestival, [InterpolaritÃ© des services Jay](..//..//miyukini-webway-system//reference//_index.md).

---

## 1. Raison d'Ãªtre

### 1.1 Proposition de valeur

**JayXpose** permet Ã  des **exposants** (artisans, artistes, petites marques, entreprises, associations) de :

- **Constituer un profil exposant complet** : identitÃ© juridique, contacts multiples, secteur d'activitÃ©, visuels, rÃ©seaux sociaux.
- **GÃ©rer un catalogue de produits** : fiches produits, catÃ©gories, galerie photo, produits mis en avant.
- **Publier un site vitrine** : page d'accueil, page catalogue, page prÃ©sentation, page contact â€” avec URL unique et personnalisation visuelle.
- **Renseigner et centraliser ses documents professionnels** : RIB, attestation d'assurance, KBIS, certificat d'immatriculation, licence, attestation URSSAF, carte professionnelle, diplÃ´mes â€” dans un coffre-fort sÃ©curisÃ©.
- **ÃŠtre rÃ©fÃ©rencÃ© dans l'annuaire des exposants** : fiche annuaire enrichie, filtres sectoriels et gÃ©ographiques, multi-Ã©vÃ©nements.
- **Se synchroniser avec JayFestival** (GFestival) : profil unique exploitÃ© pour les candidatures, documents partagÃ©s Ã  la demande, historique des participations, notifications croisÃ©es.

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **IntÃ©grÃ© JayFestival** | Profil JayXpose alimente la fiche exposant, le rÃ©pertoire et les candidatures de JayFestival ; un exposant participe Ã  plusieurs Ã©ditions avec le mÃªme profil et les mÃªmes documents. |
| **Vitrine autonome** | Site vitrine complet, sans Ã©vÃ©nement festival ; utile pour artisans ou marques qui veulent une prÃ©sence en ligne permanente, avec catalogue et contact. |

### 1.3 Phrase fondatrice

> **JayXpose est l'identitÃ© professionnelle de l'exposant dans l'Ã©cosystÃ¨me Miyukini. Un profil, un catalogue, une vitrine, un coffre-fort â€” exploitables partout, maÃ®trisÃ©s par l'exposant.**

---

## 2. FonctionnalitÃ©s structurantes

### 2.1 Profil exposant enrichi

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Fiche entreprise complÃ¨te | Raison sociale, forme juridique, adresse siÃ¨ge, adresse(s) de correspondance, SIRET, SIREN, code APE/NAF, numÃ©ro d'immatriculation. |
| Contacts multiples | Contact principal, contact facturation, contact logistique â€” chacun avec nom, email, tÃ©lÃ©phone. |
| IdentitÃ© visuelle | Logo, banniÃ¨re, visuels de couverture. |
| Description et positionnement | Description activitÃ©, mots-clÃ©s, tags sectoriels, slogan/accroche. |
| RÃ©seaux sociaux | Facebook, Instagram, LinkedIn, TikTok, YouTube, site web externe. |
| ConfidentialitÃ© granulaire | Chaque champ contrÃ´lÃ© : public, authentifiÃ©, organisateur seul, JayXpose seul. |

### 2.2 Catalogue de produits

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Fiches produits | Nom, description, prix (optionnel), catÃ©gorie, visuels multiples, disponibilitÃ©. |
| CatÃ©gories / collections | Organisation hiÃ©rarchique ou par collections thÃ©matiques. |
| Produits vedettes | Mise en avant de produits sur la vitrine et l'annuaire. |
| Galerie intÃ©grÃ©e | Galerie produits consultable depuis le site vitrine et depuis JayFestival. |

### 2.3 Site vitrine

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Pages | Accueil (banniÃ¨re, accroche, produits vedettes), Catalogue (liste filtrable), PrÃ©sentation (histoire, savoir-faire, valeurs), Contact (formulaire, coordonnÃ©es). |
| URL unique | Slug personnalisable par exposant (ex. `vitrine.jay/mon-atelier`). |
| Personnalisation | Couleurs, mise en page minimale, choix de sections affichÃ©es. |
| SEO | Balises title/description, mots-clÃ©s, donnÃ©es structurÃ©es. |
| Responsive | AdaptÃ© mobile, tablette, desktop. |

### 2.4 Documents professionnels (coffre-fort)

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Types de documents | RIB, attestation d'assurance, KBIS, certificat d'immatriculation, licence/autorisation, attestation URSSAF, carte professionnelle, diplÃ´mes/certifications. |
| Stockage sÃ©curisÃ© | Upload, horodatage, versioning. Niveau de sÃ©curitÃ© **Critical (3)** minimum. |
| Statuts | En attente, validÃ©, expirÃ©, rejetÃ©. |
| Alertes expiration | Notification avant expiration (assurance, KBIS, etc.). |
| Partage gouvernÃ© | L'exposant partage un document avec un organisateur JayFestival via un Mandat de Permission. Document par document, acceptation explicite. |
| Centralisation | Un document uploadÃ© une fois sert pour N candidatures / N Ã©vÃ©nements. |

### 2.5 Annuaire des exposants

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Fiche annuaire | Logo, nom, secteur, description, lien vitrine, localisation. |
| Filtres | Secteur, localisation, mots-clÃ©s, type d'activitÃ©, Ã©vÃ©nement. |
| Multi-Ã©vÃ©nements | Un exposant visible dans l'annuaire global et par Ã©dition JayFestival. |
| Inscription | Automatique Ã  la crÃ©ation du profil (opt-out possible). |

### 2.6 Synchronisation JayFestival

| FonctionnalitÃ© | Description |
|----------------|-------------|
| Profil unique | Pas de duplication ; JayFestival lit les donnÃ©es JayXpose. |
| PrÃ©-remplissage candidatures | Formulaire candidature prÃ©-rempli depuis le profil JayXpose. |
| Partage documents | Organisateur demande des documents ; exposant accepte le partage. |
| Catalogue visible | Produits de l'exposant consultables depuis le rÃ©pertoire JayFestival. |
| Historique participations | Ã‰ditions passÃ©es, en cours, Ã  venir. |
| Notifications croisÃ©es | Acceptation candidature, demande de document, changement de statut. |

---

## 3. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance COG** | Le service fonctionne sous gouvernance COG : StrongFather (dÃ©cisions), KindMother (persistance), Master Butler (capacitÃ©s/permissions), WorrySentinel (sÃ©curitÃ©), Ever Buddy (cycle de vie). |
| **SouverainetÃ© de l'exposant** | L'exposant est propriÃ©taire de ses donnÃ©es. Il contrÃ´le la visibilitÃ© de chaque champ et le partage de chaque document. Aucun partage implicite. |
| **RÃ©utilisabilitÃ©** | S'appuyer sur les Kits d'outils Miyukini existants (Miyauth, Miyuprofile, Miyucms, Miyumedia, Miyucontacts) et dÃ©finir les OpÃ©rateurs et Kits spÃ©cifiques. |
| **InterpolaritÃ©** | ConÃ§u pour s'intÃ©grer dans JayFestival et les services Jay. Les couplages sont explicites et gouvernÃ©s (Mandats de Permission, niveaux de sÃ©curitÃ©). |
| **Centralisation** | Un profil, un catalogue, un coffre-fort â€” exploitables depuis n'importe quel service Jay sans duplication. |
| **ConfidentialitÃ© par dÃ©faut** | Les informations sensibles (documents, RIB, contacts privÃ©s) ne sont jamais exposÃ©es par dÃ©faut. Le partage est un acte explicite et traÃ§able. |

---

## 4. IntÃ©gration et interpolaritÃ©

### 4.1 JayXpose dans JayFestival

- La **fiche exposant** de JayFestival s'appuie sur le profil JayXpose (donnÃ©es vitrine, catalogue, contact).
- Le **rÃ©pertoire des exposants** (annuaire plateforme ou par Ã©vÃ©nement) affiche les fiches JayXpose enrichies (avec catalogue).
- Un **exposant** participe Ã  des Ã©ditions JayFestival avec le mÃªme profil ; pas de duplication d'identitÃ© ni de contenu.
- Les **documents professionnels** sont partagÃ©s Ã  la demande pour les candidatures (partage gouvernÃ©, document par document).
- Les **candidatures** sont prÃ©-remplies depuis le profil JayXpose.

### 4.2 JayXpose et les autres services Jay

| Service | IntÃ©gration |
|---------|-------------|
| **JayFestival** | Profil, catalogue, documents, rÃ©pertoire, candidatures, notifications. |
| **JayKonta** | Facturation exposant ; RIB partagÃ© depuis le coffre-fort JayXpose (avec Mandat). |
| **JayRDV** | Lien depuis la vitrine vers la prise de rendez-vous. |
| **JayKoa** | Agenda des Ã©vÃ©nements auxquels l'exposant participe. |
| **JayFaim** | Phase 2 â€” si l'exposant propose de la restauration. |

### 4.3 Vitrine autonome

- JayXpose peut Ãªtre utilisÃ© **sans JayFestival** : site vitrine complet, catalogue, contact.
- Les donnÃ©es (profil, contenu vitrine, catalogue) sont gouvernÃ©es ; la rÃ©sidence et le niveau de sÃ©curitÃ© sont dÃ©finis par le contrat du service.

### 4.4 RÃ©fÃ©rence interpolaritÃ©

Voir [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) pour le principe global et les couplages entre services Jay.

---

## 5. Niveaux de sÃ©curitÃ© (orientation)

| CatÃ©gorie de donnÃ©es | Niveau | Justification |
|----------------------|--------|---------------|
| Site vitrine (pages publiques) | **Public (0)** Ã  **Standard (1)** | Contenu destinÃ© Ã  Ãªtre visible par tous. |
| Profil entreprise (nom, secteur, description) | **Standard (1)** Ã  **Sensitive (2)** | DonnÃ©es commerciales identifiantes. |
| Contacts (email, tÃ©lÃ©phone, adresse) | **Sensitive (2)** | DonnÃ©es personnelles / commerciales. |
| Catalogue produits | **Standard (1)** Ã  **Sensitive (2)** | Selon prix et stratÃ©gie commerciale. |
| Documents professionnels (RIB, KBIS, assurances) | **Critical (3)** | Documents rÃ©glementaires et financiers. |
| Identifiants (SIRET, SIREN, immatriculation) | **Sensitive (2)** Ã  **Critical (3)** | DonnÃ©es d'identification lÃ©gale. |

DÃ©tail dans [JayXpose - Niveaux Securite et Protection Donnees](./reference/JayXpose%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

---

## 6. Prochaines Ã©tapes (orientation)

1. **Fonder** : Valider ce document fondateur enrichi et le diffuser.
2. **SpÃ©cifier** : Documenter les besoins enrichis (catalogue, documents, vitrine), les OpÃ©rateurs et Kits.
3. **IntÃ©gration** : Formaliser les contrats de synchronisation avec JayFestival (profil, documents, notifications).
4. **SÃ©curitÃ©** : Formaliser les niveaux de sÃ©curitÃ© et la politique de confidentialitÃ© inter-services.
5. **ImplÃ©mentation** : DÃ©velopper les OpÃ©rateurs et Kits en s'appuyant sur les Cores.

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie (OpÃ©rateur, Mandat, COG, Niveaux de sÃ©curitÃ©). |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service dans lequel JayXpose s'intÃ¨gre. |
| [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) | Principe d'interpolaritÃ© et couplage JayXpose / JayFestival. |
| [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non fonctionnels dÃ©taillÃ©s. |
| [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md) | SpÃ©cification du module catalogue. |
| [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md) | SpÃ©cification du coffre-fort documentaire. |
| [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md) | SpÃ©cification du site vitrine complet. |
| [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md) | Politique de confidentialitÃ© et partage gouvernÃ©. |
| [JayXpose - Synchronisation JayFestival](./JayXpose%20-%20Synchronisation%20JayFestival.md) | Contrat d'intÃ©gration dÃ©taillÃ© avec JayFestival. |

---

**Document** : JayXpose â€” Document fondateur
**Version** : 2.0
**Date** : 2026-02-06
**Statut** : Document de rÃ©fÃ©rence â€” non contractuel pour l'implÃ©mentation.

