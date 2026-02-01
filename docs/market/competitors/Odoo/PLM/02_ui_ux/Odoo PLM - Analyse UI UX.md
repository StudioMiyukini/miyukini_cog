# Odoo PLM — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **PLM (Product Lifecycle Management)** d'Odoo (version 19.0). Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 PLM, écrans décrits (Overview, ECO, BoM, Version control)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vue d'ensemble (PLM Overview)
- Formulaires ECO et révision BoM
- Vues liste et Kanban ECO
- Onglets de comparaison (BoM Changes, Operation Changes)
- Gestion des documents et pièces jointes
- Patterns de navigation et breadcrumbs
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vue d'Ensemble (PLM Overview)

### 1.1 Structure

- **Page d'accueil PLM** : tableau de bord avec cartes Kanban par **type d'ECO**
- Chaque carte = un type d'ECO (ex. BOM Updates, New Product Introduction)
- Sur chaque carte : indicateurs (ex. **# Engineering Changes** = nombre d'ECO de ce type)
- Clic sur la carte ou sur le bouton **# Engineering Changes** → ouverture de la **liste / Kanban des ECO** de ce type

**Objectif :** Accès rapide par processus (type d'ECO) et quantification du volume de changements.

### 1.2 Éléments UI

- **Kanban cards** : une carte par type d'ECO
- **Smart button** (ex. "Engineering Changes") : compteur + lien vers la liste des ECO du type
- Navigation : PLM app → Overview → sélection type → liste ECO

---

## 2. Formulaire ECO (Engineering Change Order)

### 2.1 En-tête et champs principaux

**Champs visibles sur le formulaire :**
- **Description** : résumé de l'amélioration (texte)
- **Type** : type d'ECO (projet / catégorie)
- **Apply on** : Bill of Materials / Product Only (boutons radio)
- **Product** : produit concerné (Many2one)
- **Bill of Materials** : BoM modifiée (liste déroulante ; auto-remplie si produit a une BoM)
- **Company** : entreprise (multi-company, optionnel)
- **Responsible** : responsable de l'ECO (optionnel)
- **Effective** : As soon as possible / At Date (avec date si At Date)
- **Tags** : tags pour priorisation (création possible : taper le nom → Create)

**Actions principales :**
- **Start Revision** : démarre la révision (crée la copie BoM, affiche Documents et Revisions)
- **Apply Changes** : visible après passage en stage de vérification et approbations ; applique la révision en production
- **Apply Rebase** : visible lorsqu'une base obsolète est détectée (conflit avec un ECO déjà appliqué)

### 2.2 Stages (coin supérieur droit)

- **Stages de l'ECO Type** affichés en haut à droite du formulaire
- Affichage après **Start Revision**
- Permet de déplacer l'ECO d'un stage à l'autre (ex. Nouveau → En cours → Vérification → Clôture)
- Stages de **vérification** : approbation requise avant **Apply Changes**

### 2.3 Smart buttons

- **Revisions** : accès à la révision BoM (visible uniquement si Apply on = Bill of Materials et Start Revision effectué)
- **Documents** : pièces jointes de l'ECO (ajout, modification, suppression) ; après Apply Changes, synchronisation avec la BoM de production

### 2.4 Onglets

- **BoM Changes** : comparaison révision vs BoM de production
  - Texte **bleu** : composants ajoutés dans la révision
  - Texte **noir** : commun aux deux
  - Texte **rouge** : composants supprimés dans la révision
- **Operation Changes** : comparaison des opérations (Add, Remove, Update)
  - Colonnes : Operation, Step, Step Type, Type (Add/Remove/Update), Work Center, Manual Duration Change
  - Même code couleur (bleu / noir / rouge) pour cohérence
- **Previous Eco Bom Changes** : visible uniquement en cas de base obsolète (autre ECO déjà appliqué) ; affiche les différences entre BoM de production actuelle et base de l'ECO courant

**Pattern :** Comparaison côte à côte (production vs révision) avec code couleur pour faciliter la revue avant approbation.

---

## 3. Révision BoM (Formulaire BoM archivée)

### 3.1 Indication visuelle « Archived »

- **Bannière / tag « Archived »** (grande étiquette rouge) sur la révision BoM
- Objectif : distinguer clairement la révision (test) de la BoM de production
- Après **Apply Changes** : cette révision perd le statut Archived et devient la BoM de production ; la bannière disparaît

### 3.2 Navigation depuis l'ECO

- **Breadcrumb** : en haut à gauche, lien cliquable sur le nom de l'ECO (ex. « ECO005: Improve... ») pour revenir à l'ECO depuis la révision
- Depuis PLM Overview : type d'ECO → sélection d'un ECO → Revisions → révision

### 3.3 Onglets sur la BoM

- **Components** : lignes de composants (quantité, produit) — Add a line, icône poubelle pour supprimer
- **Operations** : opérations de fabrication (si Work Orders activé)
  - Clic sur une opération → pop-up **Open: Operations** (durée, poste, instructions)
  - **Instructions** (smart button dans la pop-up) : liste des steps / Quality Control Points
  - Add a line / Archive Operation pour ajouter ou retirer des opérations
- **Miscellaneous** : version de la BoM (après Apply Changes, version mise à jour)

### 3.4 Quality Control Points (Quality app)

- Dans une opération : **Instructions** (smart button) → liste des steps
- Réordonnancement : **icône draggable** (grip) à gauche de la ligne ; glisser-déposer pour réordonner
- Ex. déplacer « Check for broken switches » en deuxième position

---

## 4. Documents et Pièces Jointes

### 4.1 Sur la BoM (production)

- **Chatter** (panneau droit ou bas) : icône **trombone (📎)** pour attacher des fichiers
- Section **Files** : liste des pièces jointes ; **Attach files** pour ajouter

### 4.2 Dans l'ECO (smart button Documents)

- Page **Attachments** : liste des fichiers de l'ECO
- Sur chaque fichier : **menu ⋮** (trois points) → Edit, Remove, Download
- **Remove** : archive le fichier (toujours accessible dans l'ECO, retiré de la BoM après Apply Changes)
- **Upload** : ajout de nouveaux fichiers
- Les changements restent dans l'ECO jusqu'à **Apply Changes** ; ensuite liaison automatique à la BoM de production

---

## 5. Version et Historique BoM

### 5.1 Version courante

- **PLM → Master Data → Bill of Materials** (ou Manufacturing → Products → Bills of Materials)
- Sélection d'une BoM → onglet **Miscellaneous** → champ **Version** (ex. 1, 2, 3)

### 5.2 Historique des versions (liste ECO)

- Sur la BoM : smart button **ECO** → liste des ECO liés au produit
- **Filtre** (▼ dans la barre de recherche) : **Done** pour voir l'historique des révisions
- Colonnes utiles : ECO, Responsible, Effective Date, stage
- Clic sur un ECO → détail de cette version (composants, opérations, documents)

### 5.3 Effective Date

- Si **Effective** = "At Date", la date est enregistrée et visible dans l'historique
- Si "As soon as possible", pas de date affichée dans la liste ; contournement documenté : regarder le Chatter (heure de passage en stage clôture)

---

## 6. Patterns de Navigation

### 6.1 Entrées principales

- **Menu** : Supply Chain → PLM (ou app PLM)
- **Overview** → par type d'ECO → liste/Kanban ECO → formulaire ECO
- **Master Data → Bill of Materials** : accès direct aux BoM (version, smart button ECO)

### 6.2 Breadcrumbs

- Depuis la révision BoM : breadcrumb avec nom de l'ECO → retour à l'ECO
- Contexte conservé (quel ECO, quel type) pour éviter la perte de repère

### 6.3 Retour après Apply Changes

- Depuis l'ECO : clic **Revisions** → ouverture de la révision ; après Apply Changes, la bannière **Archived** est retirée
- Vérification : Manufacturing → Products → produit → Bill of Materials → BoM → onglet Miscellaneous → **Version** mise à jour

---

## 7. Design et Accessibilité

### 7.1 Code couleur

- **Bleu** : ajout (révision vs production)
- **Noir** : commun
- **Rouge** : suppression ; bannière Archived
- Cohérence dans BoM Changes et Operation Changes

### 7.2 Messages et avertissements

- **Important / Note** dans la doc : Product obligatoire avant BoM ; Revisions visible seulement après Start Revision ; Apply Changes après approbation
- Recommandation Miyukini : messages inline (tooltips, bannières courtes) pour guider sans bloquer

### 7.3 Responsive

- Documentation ne détaille pas le mobile ; usage PLM typiquement desktop (ingénierie, qualité)
- Recommandation Miyukini : priorité desktop ; consultation et approbation possibles sur tablette

---

## 8. Synthèse et Recommandations Miyukini

- **Overview** : conserver une entrée par type de processus (équivalent type d'ECO) avec indicateurs et accès direct aux listes.
- **ECO** : formulaire clair avec stages visibles, actions Start Revision / Apply Changes / Apply Rebase bien exposées, smart buttons Revisions et Documents.
- **Comparaison** : onglets type BoM Changes / Operation Changes avec code couleur (ajout / commun / suppression) et si possible vue côte à côte.
- **Révision vs Production** : distinction visuelle forte (bannière « Révision — pas en production »), breadcrumb ECO ↔ Révision.
- **Documents** : un flux unifié « documents de l’ECO » avec synchronisation explicite après application (traçabilité).
- **Historique versions** : version + date effective systématiques ; liste ECO filtrée « Done » avec Responsible et Effective Date.
- **Rebase** : alerte visible quand la base est obsolète + bouton Apply Rebase avec courte explication.

---

**Document rédigé selon la méthodologie d'analyse Odoo.**
