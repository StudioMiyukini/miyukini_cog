# JayXpose — Écrans et UI

## Contexte

Ce document précise les **écrans et composants UI** de **JayXpose** : profil exposant, fiche entreprise, **fiche publique** (répertoire) et **intégration dans les écrans JayFestival**. Il s’appuie sur le [Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md) et sur les écrans exposant de JayFestival (EXP-E17, EXP-E18, UNC-E08, UNC-E09, ORG-E11).

**Références** : [JayFestival - Specification UI Conforme Catakana](../JayFestival/JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md).

## Portée / Scope

- **Périmètre** : Écrans et zones UI propres à JayXpose (profil, fiche publique) ; intégration dans les écrans JayFestival (liste répertoire, fiche exposant, formulaire candidature).
- **Hors périmètre** : Maquettes pixel ; stack egui détaillée (voir Specification UI Conforme Catakana).

---

## 1. Principe : écrans JayXpose vs écrans JayFestival

| Type | Description |
|------|-------------|
| **Écrans « JayXpose »** | Ceux dont la **donnée** est le profil/vitrine (exposants) : Mon compte — Fiche entreprise (EXP-E17), Ma fiche publique (EXP-E18). L’UI peut être hébergée dans l’app JayFestival ; la **source de vérité** est JayXpose (table `exposants`). |
| **Écrans JayFestival consommant JayXpose** | Liste répertoire (UNC-E08), Fiche exposant détail (UNC-E09), Fiche exposant organisateur (ORG-E11), formulaire candidature (EXP-E10) pré-rempli depuis le profil. |
| **Composants réutilisables** | Carte exposant (répertoire), bloc fiche exposant (détail), champs formulaire fiche entreprise. |

---

## 2. Écrans JayXpose (données profil / vitrine)

### 2.1 Mon compte — Fiche entreprise (EXP-E17, onglet Fiche entreprise)

| Zone | Composants (ordre) | Données (Supabase alpha) |
|------|--------------------|---------------------------|
| Titre | Label « Fiche entreprise » | — |
| Formulaire | Input (nom entreprise), Input (activité/secteur), Input (téléphone), Input (adresse), Input (site web), Upload logo (optionnel), Input (SIRET optionnel) | `exposants.company_name`, `secteur`, `contact_phone`, `adresse`, `site_web`, `logo_url`, `siret` |
| Actions | Button « Enregistrer », Button « Changer mot de passe » (profil) | UPDATE `exposants` WHERE id = auth.uid() |

**Comportement** : Chargement au montage : SELECT exposants WHERE id = auth.uid(). Sauvegarde : UPDATE exposants. Pas de soumission partielle : tous les champs éditables en un bloc.

### 2.2 Ma fiche publique — Répertoire (EXP-E18)

| Zone | Composants (ordre) | Données (Supabase alpha) |
|------|--------------------|---------------------------|
| Titre | Label « Ma fiche publique » | — |
| Aperçu | Bloc lecture seule : nom, description, logo, site web (tel qu’affiché dans le répertoire) | `exposants` (champs publics) |
| Option visibilité | Checkbox « Visible dans le répertoire » | `exposants.visible_repertoire` |
| Édition champs autorisés | Input (description courte), Input (site web), Upload logo | `exposants.description`, `site_web`, `logo_url` |
| Actions | Button « Enregistrer » | UPDATE `exposants` |

**Comportement** : Aperçu = même rendu que la fiche détail du répertoire (UNC-E09). Édition limitée aux champs « publics » selon politique.

---

## 3. Intégration dans les écrans JayFestival

### 3.1 Liste répertoire des exposants (UNC-E08)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Filtres | Input recherche, Select (secteur), Select (événement) | — |
| Liste | Cartes ou lignes : **logo**, **nom entreprise**, **secteur**, Button « Voir la fiche » | SELECT exposants WHERE visible_repertoire = true (+ jointures édition si filtre par événement) |
| Pagination | Button Précédent/Suivant, Label « Page n / N » | LIMIT/OFFSET |

**Contrat** : Chaque carte/ligne affiche au minimum : logo_url, company_name, secteur (ou category). Clic « Voir la fiche » → Fiche exposant (UNC-E09).

### 3.2 Fiche exposant — Détail public (UNC-E09)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Bloc 1 | Logo, Label (nom entreprise), Label (description), Label (secteur) | exposants : logo_url, company_name, description, secteur |
| Bloc 2 | Label « Éditions participées » ; liste liens vers fiches événement | editions_exposants JOIN editions WHERE exposant_id = ? AND is_validated = true |
| Bloc 3 | Label « Coordonnées » ; contact (email, site web, téléphone selon paramètre) | exposants : contact_email, site_web, contact_phone |
| Pied | Button « Retour liste » | — |

**Contrat** : Données lues depuis `exposants` (et `editions_exposants` + `editions` pour les éditions participées). Pas d’édition sur cet écran (lecture seule).

### 3.3 Fiche exposant — Côté organisateur (ORG-E11)

| Zone | Composants | Données JayXpose |
|------|------------|-------------------|
| Identité | Label (nom), Label (contact), Label (catégorie/secteur) | exposants (même source) |
| Statut / Emplacement / Documents / Historique | (JayFestival) | editions_exposants, stands, documents, etc. |

**Contrat** : Bloc « Identité » = lecture `exposants` pour cet exposant_id. Reste de la fiche = données JayFestival (candidature, stand, facturation).

### 3.4 Formulaire candidature (EXP-E10) — Pré-remplissage

| Donnée | Source | Comportement |
|--------|--------|--------------|
| Nom entreprise, contact, activité, logo | `exposants` WHERE id = auth.uid() | Champs pré-remplis (éditables ou en lecture selon config organisateur). |
| Pièces jointes | Upload utilisateur | Pas de pré-remplissage depuis JayXpose. |

**Contrat** : Au chargement du formulaire candidature, lecture du profil exposant (JayXpose) pour pré-remplir les champs communs (company_name, contact_*, secteur, logo_url si affiché).

---

## 4. Composants UI réutilisables (spécification)

| Composant | Usage | Données |
|-----------|--------|---------|
| **Carte exposant (répertoire)** | Liste UNC-E08, éventuellement liste par édition. | logo_url, company_name, secteur ; id pour lien. |
| **Bloc fiche exposant (détail)** | UNC-E09, ORG-E11 (bloc identité). | Tous champs publics exposants + éditions participées si contexte. |
| **Formulaire fiche entreprise** | EXP-E17 (onglet Fiche entreprise), EXP-E18 (édition champs autorisés). | Champs exposants (voir Base de donnees). |

Alignement visuel : [JayFestival - Specification UI Conforme Catakana](../JayFestival/JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) (atoms, molecules, organisms).

---

## 5. Checklist implémentation UI (JayXpose)

- [ ] Écran Mon compte — Fiche entreprise (formulaire + sauvegarde exposants).
- [ ] Écran Ma fiche publique (aperçu + visibilité + édition champs autorisés).
- [ ] Liste répertoire (UNC-E08) : données exposants + visible_repertoire.
- [ ] Fiche exposant détail (UNC-E09) : données exposants + éditions participées.
- [ ] Fiche exposant organisateur (ORG-E11) : bloc identité depuis exposants.
- [ ] Formulaire candidature (EXP-E10) : pré-remplissage depuis exposants.
- [ ] Composants : Carte exposant, Bloc fiche exposant, Formulaire fiche entreprise.

---

## 6. Références

- [JayXpose - Parcours utilisateur exposant](./JayXpose%20-%20Parcours%20utilisateur%20exposant.md)
- [JayXpose - Base de donnees Supabase et Migration SQLite](./reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md)
- [JayFestival - Specification UI Conforme Catakana](../JayFestival/JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md)
- [Exposants - Ecrans et cycle](../JayFestival/publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md)

---

**Document** : JayXpose — Écrans et UI  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Référence produit
