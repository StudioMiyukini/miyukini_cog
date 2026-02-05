# JayXpose — Parcours utilisateur exposant

## Contexte

Ce document décrit le **parcours utilisateur exposant** côté **JayXpose** : création du profil, mise à jour de la fiche entreprise, gestion de la **fiche publique** (répertoire) et mécaniques alignées sur **Catakana** / JayFestival. Il s’appuie sur les écrans et cycles exposant de JayFestival (EXP-E03, EXP-E17, EXP-E18) et sur les données stockées en alpha Supabase.

**Références** : [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md), [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md), [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

## Portée / Scope

- **Périmètre** : Parcours côté profil/vitrine/répertoire (création compte exposant, fiche entreprise, fiche publique) ; mécaniques Catakana reprises pour l’alpha.
- **Hors périmètre** : Parcours candidatures, participations, facturation (documents JayFestival Exposants).

---

## 1. Vue d’ensemble du parcours (JayXpose)

| Phase | Description | Écrans / actions |
|-------|-------------|-------------------|
| **Onboarding** | Création du compte exposant et du **profil JayXpose** (fiche entreprise). | Inscription exposant (EXP-E03) → création `profiles` + `exposants`. |
| **Mon compte / Fiche entreprise** | Mise à jour du profil (nom, contact, activité, logo, site web). | Mon compte (EXP-E17) → formulaire fiche entreprise ; sauvegarde `exposants`. |
| **Fiche publique (répertoire)** | Gestion de la visibilité et des champs affichés dans le répertoire des exposants. | Fiche publique (EXP-E18) → aperçu + édition champs autorisés + option visibilité. |
| **Consommation par JayFestival** | Lecture du profil pour fiche exposant (organisateur), répertoire (catalogue), candidatures. | Écrans JayFestival (liste exposants, fiche exposant, formulaire candidature) lisent `exposants`. |

---

## 2. Parcours détaillés (mécaniques Catakana)

### 2.1 Inscription exposant (création du profil JayXpose)

**Contexte** : Premier usage ; l’utilisateur s’inscrit en tant qu’exposant depuis le catalogue (JayFestival) ou depuis une page dédiée.

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à l’écran « Créer un compte exposant » (EXP-E03). | Lien depuis landing ou fiche événement « Candidater ». |
| 2 | Saisie email, mot de passe, confirmation. | Supabase Auth : `signUp` ; création entrée `auth.users`. |
| 3 | Saisie fiche entreprise : nom entreprise, activité/secteur, contact (téléphone, adresse), site web (optionnel). | Création ou mise à jour **profiles** (trigger ou service) : `user_type = 'exhibitor'`. Création **exposants** : `id = auth.uid()`, `company_name`, `contact_*`, `secteur`, etc. (voir requêtes SQL en référence). |
| 4 | Acceptation CGU + clic « S’inscrire ». | Insert `exposants` si pas encore créé ; sinon update. |
| 5 | Redirection vers dashboard exposant ou page de confirmation. | Session établie ; exposant peut candidater (JayFestival) et modifier sa fiche (JayXpose). |

**Mécanique Catakana** : À l’inscription, un enregistrement **exposants** est créé avec `id = profile.id` (1:1). Les champs du formulaire d’inscription exposant correspondent aux colonnes de la table `exposants`.

### 2.2 Mon compte — Fiche entreprise (mise à jour du profil)

**Contexte** : Exposant connecté ; il consulte ou modifie sa fiche entreprise (écran EXP-E17, onglet « Fiche entreprise »).

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Mon compte » depuis le dashboard exposant. | Navigation menu. |
| 2 | Onglet « Fiche entreprise » : affichage des champs actuels (nom, contact, activité, logo, site web, adresse, SIRET…). | SELECT sur `exposants` WHERE `id = auth.uid()`. |
| 3 | Modification des champs et clic « Enregistrer ». | UPDATE `exposants` SET … WHERE `id = auth.uid()` ; RLS : seul le propriétaire peut modifier. |
| 4 | (Optionnel) Upload logo. | Upload vers Supabase Storage (bucket `logos` ou `exposants`) ; mise à jour `exposants.logo_url`. |
| 5 | Confirmation visuelle (toast ou message). | Données à jour ; répertoire et fiche exposant (JayFestival) reflètent les changements à la prochaine lecture. |

**Mécanique Catakana** : Les services Catakana (`exposantService`) exposent `getExposantByUserId`, `updateExposant` ; en alpha JayFestival/JayXpose on utilise les mêmes tables et RLS.

### 2.3 Fiche publique — Répertoire (visibilité et champs publiés)

**Contexte** : Exposant connecté ; il consulte l’aperçu de sa fiche telle qu’elle apparaît dans le **répertoire des exposants** et peut activer/désactiver la visibilité ou éditer les champs autorisés (écran EXP-E18).

| Étape | Action | Données / mécanique |
|-------|--------|----------------------|
| 1 | Accès à « Ma fiche publique » depuis Mon compte ou le dashboard. | Navigation. |
| 2 | Affichage de l’aperçu (mode lecture) : comme vu par un visiteur du répertoire. | Données lues depuis `exposants` (champs avec `visible_repertoire` ou politique) ; masquage des champs non publiés. |
| 3 | Option « Visible dans le répertoire » : case à cocher. | UPDATE `exposants` SET `visible_repertoire = true/false` WHERE `id = auth.uid()`. |
| 4 | Édition des champs autorisés pour le répertoire (nom, description, logo, site web, réseaux). | Même table `exposants` ; champs « publics » définis par politique (ex. company_name, description, logo_url, site_web). |
| 5 | Enregistrement. | UPDATE `exposants` ; liste répertoire (catalogue) filtre sur `visible_repertoire = true`. |

**Mécanique Catakana** : Le répertoire public (catalogue) interroge les exposants avec un filtre type `visible_repertoire = true` (ou équivalent selon schéma Catakana). La fiche détail exposant (UNC-E09, ORG-E11) lit les mêmes champs.

### 2.4 Consommation par JayFestival (lecture seule)

| Cas d’usage | Acteur | Données lues | Requête type |
|-------------|--------|--------------|--------------|
| Liste répertoire (catalogue) | Utilisateur non connecté / visiteur | Exposants visibles | SELECT exposants WHERE visible_repertoire = true (+ jointures si besoin). |
| Fiche exposant (détail public) | UNC, visiteur | Fiche complète (champs publics) | SELECT exposants WHERE id = ?. |
| Fiche exposant (organisateur) | Organisateur édition | Fiche + statut participation | SELECT exposants + editions_exposants WHERE edition_id = ? AND exposant_id = ?. |
| Formulaire candidature (pré-remplissage) | Exposant | Son propre profil | SELECT exposants WHERE id = auth.uid(). |
| Liste exposants par édition | Organisateur / catalogue | Exposants ayant une participation validée pour l’édition | SELECT exposants JOIN editions_exposants ON … WHERE edition_id = ? AND is_validated = true. |

Détail des requêtes SQL dans [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

---

## 3. Flux résumé (schéma)

```
[Inscription exposant] → Auth + profiles (user_type=exhibitor) + exposants (création)
        ↓
[Mon compte - Fiche entreprise] → UPDATE exposants (propriétaire)
        ↓
[Fiche publique] → UPDATE visible_repertoire + champs publiés
        ↓
[JayFestival] → Lecture exposants (répertoire, fiche exposant, candidatures, liste par édition)
```

---

## 4. Références

- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)

---

**Document** : JayXpose — Parcours utilisateur exposant  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Référence produit
