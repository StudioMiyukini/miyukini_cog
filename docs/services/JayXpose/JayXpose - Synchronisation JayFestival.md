# JayXpose — Synchronisation JayFestival

## Contexte

Ce document spécifie le **contrat d'intégration** entre **JayXpose** et **JayFestival** (GFestival). JayXpose est le service d'identité professionnelle de l'exposant ; JayFestival est le service de gestion d'événements et festivals. Les deux services partagent des données (profil, catalogue, documents) selon des règles de gouvernance strictes.

**Principe fondateur** : Un exposant = un profil JayXpose = N participations JayFestival. Pas de duplication.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md), [JayFestival - Interpolarite Services Jay](../JayFestival/reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md).

## Portée / Scope

- **Périmètre** : Données partagées, flux d'intégration, pré-remplissage candidatures, partage documents, catalogue dans répertoire, historique participations, notifications croisées.
- **Hors périmètre** : Implémentation technique détaillée (API endpoints) ; logique métier JayFestival (candidatures, stands, budget).

---

## 1. Principes d'intégration

| Principe | Description |
|----------|-------------|
| **Source unique** | JayXpose est la source de vérité pour le profil exposant, le catalogue et les documents. JayFestival ne duplique jamais ces données. |
| **Lecture depuis JayFestival** | JayFestival lit les données JayXpose (profil, catalogue, documents partagés). JayFestival n'écrit jamais dans JayXpose. |
| **Écriture JayFestival → JayXpose** | Limitée aux demandes (demande de partage de document, notifications). Jamais de modification de données exposant. |
| **Gouvernance** | Tout échange est encadré par le Kit « Liaison JayFestival » et des Mandats de Permission. |
| **Alpha** | En alpha, JayFestival et JayXpose partagent les mêmes tables Supabase. Post-alpha : communication via KindMother ou contrats dédiés. |

---

## 2. Données partagées

### 2.1 JayXpose → JayFestival (lecture)

| Catégorie | Données | Usage JayFestival | Condition |
|-----------|---------|-------------------|-----------|
| **Profil public** | company_name, logo_url, secteur, description_short, slogan, site_web, social_*. | Annuaire, fiche exposant, répertoire par édition. | Toujours (données publiques). |
| **Profil étendu** | contact_email, contact_phone, adresse, legal_form, siret. | Fiche exposant côté organisateur ; formulaire candidature pré-rempli. | Selon politique de confidentialité (niveau `organisateur` ou `public`). |
| **Catalogue** | Produits vedettes (nom, image, prix, catégorie). | Aperçu dans la fiche exposant (annuaire/répertoire). Lien vers vitrine. | Si catalogue visible. |
| **Documents partagés** | Type, statut, URL signée. | Fiche exposant organisateur (ORG-E11) ; validation candidature. | Si partage accepté (documents_partages). |
| **Historique participations** | editions_exposants (edition_id, is_validated, is_accepted). | Fiche exposant détail (éditions participées). | Lecture directe (même base alpha). |

### 2.2 JayFestival → JayXpose (notifications / demandes)

| Donnée | Usage | Mécanisme |
|--------|-------|-----------|
| **Demande de document** | L'organisateur demande un document pour une candidature. | INSERT `documents_partages` (status = 'demande'). |
| **Changement statut candidature** | Notification à l'exposant (acceptée, refusée, en attente). | Notification in-app JayXpose. |
| **Validation/rejet document** | L'organisateur valide ou rejette un document partagé. | UPDATE `documents_professionnels` (status, validated_by, rejection_reason). |
| **Notification événement** | Rappel, changement de date, message organisateur. | Notification in-app JayXpose. |

---

## 3. Flux d'intégration

### 3.1 Candidature exposant (pré-remplissage)

```
[Exposant] → Candidater à une édition JayFestival
    ↓
[JayFestival] → Charge formulaire candidature
    ↓
[Kit Liaison JayFestival] → Lit profil JayXpose (exposant.profile.get)
    ↓
[Formulaire] → Champs pré-remplis :
    - company_name, contact_email, contact_phone
    - secteur, description_short
    - logo_url
    ↓
[Exposant] → Complète les champs spécifiques candidature (message, besoins stand, pièces jointes)
    ↓
[JayFestival] → Soumet la candidature
    ↓
[Si documents requis] → Demande de partage (voir 3.2)
```

### 3.2 Partage de documents pour candidature

```
[Organisateur] → Configure les documents requis pour la candidature
    (ex: assurance, KBIS obligatoires ; licence optionnel)
    ↓
[Exposant soumet candidature] → Voit la liste des documents demandés
    ↓
[Pour chaque document demandé] :
    → L'exposant ouvre son coffre-fort JayXpose
    → Sélectionne le document correspondant
    → Accepte le partage
    ↓
[documents_partages] → status = 'accepte'
    ↓
[Organisateur] → Consulte les documents dans la fiche exposant (ORG-E11)
    → Peut valider ou rejeter
    ↓
[Si rejeté] → Notification à l'exposant + motif → remplacement et re-partage
```

### 3.3 Annuaire / Répertoire

```
[Annuaire global JayXpose]
    → SELECT exposants WHERE visible_annuaire = true
    → Affichage : logo, nom, secteur, description, lien vitrine
    ↓
[Répertoire par édition JayFestival]
    → SELECT exposants JOIN editions_exposants
       WHERE edition_id = ? AND is_validated = true
    → Même affichage + badge « Participe à [édition] »
    → Aperçu produits vedettes (catalogue JayXpose)
```

### 3.4 Fiche exposant dans JayFestival

```
[Visiteur/Organisateur] → Clique sur un exposant dans le répertoire
    ↓
[JayFestival] → Charge la fiche exposant
    ↓
[Bloc Identité] → Lecture profil JayXpose
    (company_name, logo, description, secteur, contact selon confidentialité)
    ↓
[Bloc Catalogue] → Lecture produits vedettes JayXpose
    (nom, image, prix — max 6 produits)
    → Lien « Voir la vitrine complète »
    ↓
[Bloc Participations] → Lecture editions_exposants
    (liste des éditions avec liens)
    ↓
[Bloc Documents] (organisateur uniquement)
    → Lecture documents_partages pour cette candidature/édition
    (type, statut, date)
```

### 3.5 Historique participations (côté exposant)

```
[Exposant] → Dashboard JayXpose → « Mes participations »
    ↓
[Lecture] → SELECT editions_exposants JOIN editions
    WHERE exposant_id = auth.uid()
    ORDER BY editions.start_date DESC
    ↓
[Affichage] :
    - Éditions à venir (is_validated = true, date > now)
    - Éditions en cours
    - Éditions passées
    - Candidatures en attente (is_accepted = false)
```

---

## 4. Notifications croisées

### 4.1 JayFestival → Exposant (via JayXpose)

| Événement | Notification | Canal |
|-----------|--------------|-------|
| Candidature acceptée | « Votre candidature pour [édition] a été acceptée. » | In-app + email. |
| Candidature refusée | « Votre candidature pour [édition] a été refusée. [motif] » | In-app + email. |
| Candidature en attente | « Votre candidature pour [édition] est en cours d'examen. » | In-app. |
| Demande de document | « L'organisateur [nom] demande votre [type document] pour [édition]. » | In-app + email. |
| Document validé | « Votre [type document] a été validé pour [édition]. » | In-app. |
| Document rejeté | « Votre [type document] a été rejeté pour [édition]. Motif : [motif]. » | In-app + email. |
| Rappel événement | « L'événement [édition] commence dans [N] jours. » | In-app. |
| Message organisateur | « [Organisateur] vous a envoyé un message concernant [édition]. » | In-app + email. |
| Changement de date/lieu | « L'événement [édition] a changé de date/lieu. » | In-app + email. |

### 4.2 Configuration par l'exposant

| Paramètre | Options |
|-----------|---------|
| Notifications in-app | Toujours activées (non désactivable). |
| Notifications email | Par type : activé / désactivé. Défaut : activé pour candidature + documents. |
| Fréquence email | Immédiat / résumé quotidien. |

---

## 5. Architecture alpha (Supabase partagé)

### 5.1 Tables partagées

En alpha, JayFestival et JayXpose partagent les mêmes tables Supabase :

| Table | Propriétaire | Lecteur |
|-------|-------------|---------|
| `profiles` | Miyauth | JayXpose, JayFestival. |
| `exposants` | JayXpose | JayFestival (lecture). |
| `produits_catalogue` | JayXpose | JayFestival (lecture). |
| `categories_produits` | JayXpose | JayFestival (lecture). |
| `produits_visuels` | JayXpose | JayFestival (lecture). |
| `documents_professionnels` | JayXpose | JayFestival (lecture, si partage accepté). |
| `documents_partages` | JayXpose | JayFestival (lecture + insert demande). |
| `editions` | JayFestival | JayXpose (lecture). |
| `editions_exposants` | JayFestival | JayXpose (lecture). |
| `candidatures` | JayFestival | JayXpose (lecture statut). |

### 5.2 Post-alpha (migration)

Post-alpha, la communication sera formalisée :
- **KindMother** gère la persistance de chaque service indépendamment.
- Les échanges passent par **BondingBrother** (médiation).
- Les contrats de lecture sont formalisés (API ou contrats KindMother).
- Les Mandats de Permission sont émis par StrongFather pour chaque flux.

---

## 6. Contrat de données (résumé)

| Flux | Source | Destination | Données | Gouvernance |
|------|--------|-------------|---------|-------------|
| Profil public → Annuaire/Répertoire | JayXpose | JayFestival | Champs publics exposant. | Automatique (données publiques). |
| Profil étendu → Fiche organisateur | JayXpose | JayFestival | Champs selon confidentialité. | Politique de confidentialité exposant. |
| Catalogue → Aperçu répertoire | JayXpose | JayFestival | Produits vedettes. | Automatique (catalogue visible). |
| Documents → Candidature | JayXpose | JayFestival | Documents partagés. | Mandat de Permission (explicite, unitaire). |
| Demande document → Exposant | JayFestival | JayXpose | Type demandé, contexte. | BondingBrother. |
| Notifications → Exposant | JayFestival | JayXpose | Statuts, messages. | Notification interne. |
| Participations → Historique | JayFestival | JayXpose | editions_exposants. | Lecture directe (alpha) / contrat (post-alpha). |

---

## 7. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)
- [JayXpose - Documents Professionnels et Coffre-Fort](./JayXpose%20-%20Documents%20Professionnels%20et%20Coffre-Fort.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayFestival - Interpolarite Services Jay](../JayFestival/reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md)
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)

---

**Document** : JayXpose — Synchronisation JayFestival
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Référence produit
