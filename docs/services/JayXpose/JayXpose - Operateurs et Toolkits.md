# JayXpose — Opérateurs et Toolkits

## Contexte

Ce document décrit les **Opérateurs** (Strate 7) et **Toolkits** (Strate 6) du service **JayXpose** : profil exposant, vitrine, catalogue, **liaison avec JayFestival**. Il s’appuie sur l’[Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) et le [Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md).

**Références** : Glossaire Miyukini (Opérateur, Outil, Kit d’Outils, Mandat de Permission) ; [JayFestival - Exposants Operateurs et Toolkits](../JayFestival/publics/Exposants/Exposants%20-%20Operateurs%20et%20Toolkits.md).

## Portée / Scope

- **Périmètre** : Opérateurs et Kits JayXpose (profil, vitrine, répertoire, intégration JayFestival).
- **Hors périmètre** : Candidatures, participations, facturation (JayFestival) ; spécifications API détaillées (contrats dédiés).

---

## 1. Référence glossaire Miyukini

| Concept | Définition |
|---------|------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |

---

## 2. Opérateurs JayXpose

### 2.1 Opérateur « JayXpose Profil » (profil exposant)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **création et la mise à jour du profil exposant** (fiche entreprise, contact, logo, secteur). |
| **Public servi** | Exposants authentifiés (rôle exposant, Master Butler). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; permissions (Master Butler) ; persistance (KindMother) ; sécurité (WorrySentinel). |
| **Capacités exposées** | Création du profil à l’inscription ; mise à jour de la fiche entreprise (nom, contact, activité, logo, site web) ; lecture de son propre profil. |
| **Ne fait pas** | Validation des candidatures (JayFestival) ; émission des factures (JayFestival / JayKonta). |

### 2.2 Opérateur « JayXpose Répertoire » (fiche publique et annuaire)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer la **fiche publique** (répertoire) et la **liste des exposants** pour le catalogue (JayFestival). |
| **Public servi** | Exposant (édition de sa fiche publique, visibilité) ; utilisateur non connecté / visiteur / organisateur (lecture répertoire). |
| **Gouvernance** | Lecture publique ou selon Mandat ; écriture (visibilité, champs publiés) = propriétaire uniquement. |
| **Capacités exposées** | Liste des exposants visibles dans le répertoire ; fiche détail exposant (champs publics) ; mise à jour visibilité et champs autorisés par l’exposant. |
| **Ne fait pas** | Candidatures, participations (JayFestival). |

### 2.3 Synthèse des Opérateurs

| Opérateur | Usage | Livrables couverts |
|-----------|-------|--------------------|
| **JayXpose Profil** | Création et mise à jour du profil exposant (fiche entreprise). | Inscription exposant, Mon compte (fiche entreprise). |
| **JayXpose Répertoire** | Fiche publique, liste répertoire, visibilité. | Fiche publique (EXP-E18), répertoire catalogue (UNC-E08, UNC-E09), fiche exposant (JayFestival). |

---

## 3. Toolkits JayXpose

### 3.1 Kit « Profil Exposant » (JayXpose)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **lecture et l’écriture du profil exposant** (fiche entreprise). |
| **Outils agrégés (exemples)** | `exposant.profile.get` (profil par user_id), `exposant.profile.create` (création à l’inscription), `exposant.profile.update` (mise à jour fiche), `exposant.logo.upload` (logo). |
| **Consommé par** | JayXpose Profil ; JayFestival (inscription exposant, Mon compte, formulaire candidature pré-rempli). |
| **Composants sous-jacents** | KindMother (persistance) ; alpha = Supabase (table `exposants`). |

### 3.2 Kit « Répertoire Exposants » (JayXpose)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Orchestrer la **liste publique des exposants** et la **fiche détail** (répertoire catalogue). |
| **Outils agrégés (exemples)** | `exposant.list.public` (liste avec filtre visible_repertoire, secteur, recherche), `exposant.profile.get.public` (fiche détail pour catalogue), `exposant.visibility.set` (activer/désactiver répertoire). |
| **Consommé par** | JayXpose Répertoire ; JayFestival Catalogue (UNC-E08, UNC-E09) ; JayFestival Organisateur (liste exposants édition). |
| **Composants sous-jacents** | KindMother ; alpha = Supabase (`exposants`, `editions_exposants` pour liste par édition). |

### 3.3 Kit « Liaison JayFestival » (JayXpose ↔ JayFestival)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Définir le **contrat d’intégration** : quelles données JayFestival lit depuis JayXpose (fiche exposant, répertoire, candidatures). |
| **Outils agrégés (exemples)** | Lecture `exposants` pour fiche exposant (organisateur, catalogue) ; lecture `exposants` + `editions_exposants` pour liste exposants par édition ; pré-remplissage formulaire candidature depuis `exposants`. |
| **Consommé par** | JayFestival (tous écrans exposant, répertoire, fiche exposant). |
| **Données** | Alpha : même table `exposants` (Supabase) ; pas de duplication ; JayFestival et JayXpose partagent la source. |

### 3.4 Synthèse des Toolkits

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|--------------------|
| **Profil Exposant** | JayXpose Profil, JayFestival (inscription, Mon compte) | Fiche entreprise, profil. |
| **Répertoire Exposants** | JayXpose Répertoire, JayFestival Catalogue | Liste répertoire, fiche publique, visibilité. |
| **Liaison JayFestival** | JayFestival (fiche exposant, candidatures, liste par édition) | Intégration fiche exposant, répertoire. |

---

## 4. Matrice Parcours / Opérateurs / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) |
|----------------------|-----------|------------|
| Inscription exposant (création profil) | JayXpose Profil | Profil Exposant. |
| Mon compte — Fiche entreprise | JayXpose Profil | Profil Exposant. |
| Fiche publique (répertoire) | JayXpose Répertoire | Répertoire Exposants. |
| Liste répertoire (catalogue) | JayXpose Répertoire | Répertoire Exposants, Liaison JayFestival. |
| Fiche exposant (détail public / organisateur) | JayXpose Répertoire | Répertoire Exposants, Liaison JayFestival. |
| Formulaire candidature (pré-rempli) | JayXpose Profil | Profil Exposant, Liaison JayFestival. |

---

## 5. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Authentification, rôles | Miyauth, Master Butler | Compte exposant, Mandat, permissions. |
| Persistance alpha | Supabase (exposants, profiles) | Table exposants, RLS. |
| Persistance post-alpha | KindMother, SQLite | Migration documentée. |
| Sécurité, audit | WorrySentinel | Niveaux de sécurité. |
| JayFestival | JayFestival (candidatures, participations, catalogue) | Consommation profil et répertoire. |

---

## 6. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [JayFestival - Exposants Operateurs et Toolkits](../JayFestival/publics/Exposants/Exposants%20-%20Operateurs%20et%20Toolkits.md)

---

**Document** : JayXpose — Opérateurs et Toolkits  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Référence produit
