# Odoo Invoicing — Analyse UI/UX

## Contexte

Ce document analyse l’**interface utilisateur et l’expérience utilisateur** de l’application **Invoicing** d’Odoo (périmètre facturation du module account). Il identifie les vues, composants, patterns de navigation et mécanismes d’interaction pour servir de référence à un équivalent Miyukini.

**Source d'analyse :** Code source Odoo 19.0 (addon account — vues et contrôlers facturation).

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales factures (Liste, Kanban, Formulaire)
- Composants et widgets facturation
- Patterns de navigation et actions contextuelles
- Formulaires, validations, statuts visuels
- Rapports et exports liés à la facturation

**Hors scope :**
- Vues grand livre / rapprochement bancaire (voir Accounting)
- Implémentation technique détaillée

---

## 1. Vues Principales (Facturation)

### 1.1 Vue Liste — Factures (`account.move`)

**Usage :** Liste des factures clients et fournisseurs (filtrée par move_type).

**Colonnes typiques :**
- Partenaire (client/fournisseur)
- Numéro de facture (name)
- Date facture / Date d’échéance
- Montant total (amount_total) + devise
- Statut de paiement (payment_state) : Not paid, Partial, Paid, etc.
- État (state) : Draft, Posted
- Journal (optionnel)

**Décorations :**
- Échéance dépassée : couleur d’alerte (date_maturity < today)
- Brouillon : style distinct (gris ou italique)
- Payée : indicateur vert ou badge

**Filtres :**
- Type : Factures clients / Factures fournisseurs / Avoirs
- État : Brouillon / Validées
- Paiement : Non payées / Partiellement payées / Payées
- Période (date facture ou échéance)
- Partenaire, journal

**Groupements :**
- Par partenaire, par mois, par journal

### 1.2 Vue Kanban — Factures

**Usage :** Vue type cartes, adaptée mobile ou tableau de bord.

**Contenu carte :**
- En-tête : Partenaire ou numéro de facture
- Montant total + devise
- Badge statut : Draft / Posted
- Badge paiement : Not paid / Partial / Paid
- Pied : Date, activités (suivi)

**Widgets :**
- Activités (kanban_activity)
- Sélection d’état (label_selection)
- Actions rapides (envoyer, imprimer)

### 1.3 Vue Formulaire — Facture (`account.move`)

**Structure :**
- **Header :** Boutons d’action selon état et type (client/fournisseur)
- **Statut :** Barre d’état Draft → Posted ; badges Sent, Paid, Partial, Reversed, Blocked
- **Corps :** Groupes de champs et lignes de facture (sous-formulaire)

**Boutons d’action (facturation) :**
- **Valider** (Post / Confirm) : Draft → Posted
- **Envoyer par email** : Envoi avec PDF
- **Imprimer** : Téléchargement / impression PDF
- **Enregistrer un paiement** : Ouverture wizard paiement
- **Créer un avoir** : Génération avoir lié
- **Annuler** : Passage à l’état Cancel
- **Remettre en brouillon** : Posted → Draft (si autorisé)
- **Marquer comme vérifié** : Pour workflow de contrôle (si activé)

**Ribbons / badges :**
- Sent, Paid, In Payment, Partial, Reversed, Blocked (couleurs distinctes)
- Alertes : doublons de référence, crédits non réconciliés, devise inactive

**Groupes de champs :**
- En-tête : Partenaire, date facture, date d’échéance, conditions de paiement, référence, journal
- Lignes : Table éditable (produit, description, quantité, prix, remise, taxes, montant)
- Totaux : HT, taxes, TTC (calculés)
- Pied : Note, termes et conditions (selon config)

**Lignes de facture (sous-formulaire) :**
- Colonnes : Produit/Service, Description, Quantité, Prix unitaire, Remise %, Taxes, Montant
- Types de ligne : Ligne section, note (affichage seul)
- Calcul automatique des totaux et des lignes de taxe

### 1.4 Vue Liste — Lignes d’écriture (Journal Items)

**Usage :** Consultation des lignes comptables des factures (débit/crédit, compte). Souvent masquée ou en lecture seule dans un périmètre "Invoicing only" pour ne pas surcharger l’utilisateur.

---

## 2. Composants et Widgets

### 2.1 Widgets spécifiques facturation

- **Montant résiduel** : Affichage du restant à payer (amount_residual) avec devise
- **État de paiement** : Badge ou label (Not paid, Partial, Paid, etc.)
- **Référence de paiement** : Champ calculé pour communication au client (référence structurée)
- **Conditions de paiement** : Sélection avec prévisualisation des échéances (dates + montants)
- **Sélection de taxes** : Liste ou tags des taxes appliquées par ligne

### 2.2 Formulaires et validations

- **Validation côté client :** Champs requis (partenaire, au moins une ligne avec montant)
- **Validation côté serveur :** Équilibre comptable, cohérence journal/type, verrouillage fiscal
- **Messages d’erreur :** "La facture n’est pas équilibrée", "Date antérieure au verrouillage", etc.
- **Avertissements :** "Facture déjà envoyée", "Paiement partiel existant"

### 2.3 Wizard Paiement

**Étapes :**
1. Type : Paiement client / Paiement fournisseur
2. Partenaire, montant, date, journal (banque/caisse)
3. Factures à régler : sélection et répartition du montant
4. Confirmation → Création du paiement et réconciliation des lignes

**Affichage :** Montant dû par facture, montant saisi, solde après paiement.

---

## 3. Patterns de Navigation

### 3.1 Menus (Invoicing)

- **Factures** (ou Factures clients) : Liste factures clients
- **Factures fournisseurs** : Liste factures fournisseurs
- **Paiements** : Liste des paiements
- **Clients** / **Fournisseurs** : Fiche partenaire avec onglet Factures / Paiements
- **Configuration** : Journaux, conditions de paiement, taxes (si droits)

### 3.2 Navigation contextuelle

- Depuis une facture : Lien vers partenaire, vers avoir créé, vers paiements enregistrés
- Depuis un partenaire : Liste des factures et paiements
- Depuis une commande (Sales) : Lien vers factures créées depuis la commande

### 3.3 Recherche et filtres

- Recherche globale : Partenaire, numéro de facture, référence
- Filtres prédéfinis : "Mes factures brouillon", "À envoyer", "À relancer", "Payées ce mois"
- Tri : Date, partenaire, montant, statut

---

## 4. Rapports et Exports (Facturation)

### 4.1 Rapports typiques

- **Factures clients** : Liste avec totaux, filtres date/partenaire
- **Factures fournisseurs** : Idem
- **Revenus / Dépenses** : Agrégation par période (si rapports P&amp;L activés)
- **À relancer** : Factures non payées avec échéance dépassée (ou proche)

### 4.2 Exports

- PDF : Facture individuelle (template imprimable)
- Export liste : CSV, Excel (colonnes configurables)
- Envoi en masse : Envoi par email à partir d’une sélection de factures

---

## 5. Design Responsive et Accessibilité

### 5.1 Responsive

- Liste : Colonnes masquables ou réorganisables sur petit écran
- Kanban : Cartes empilées sur mobile
- Formulaire : Groupes empilés verticalement ; lignes de facture en tableau scrollable ou cartes

### 5.2 Accessibilité

- Labels explicites pour champs et boutons
- Contraste des statuts (couleurs + texte ou icônes)
- Raccourcis clavier pour Valider, Envoyer, Annuler (selon implémentation Odoo)
- Messages d’erreur annoncés (ARIA ou équivalent)

---

## 6. Points d’Attention pour Miyukini

### 6.1 Simplification "Invoicing only"

- Masquer ou réduire la visibilité des champs purement comptables (journal, compte) dans les vues principales facture
- Proposer une vue "Factures" unifiée (clients + fournisseurs) avec filtre type, plutôt que deux applications séparées si souhaité
- Statut unique lisible : Brouillon / Validée / Envoyée / Payée / Partielle / Avoir

### 6.2 Cohérence avec MiyuInvoice et COG

- Réutiliser les concepts MiyuInvoice (facture, ligne, taxe, conditions de paiement) dans l’UI
- Actions "Valider" et "Envoyer" déclencher des flux gouvernés (StrongFather, KindMother)
- Bouton "Enregistrer paiement" → workflow avec Mandat et réconciliation gouvernée

### 6.3 Recommandations

- Indicateur visuel fort pour les factures à relancer (échéance + amount_residual)
- Wizard paiement simple : montant, date, facture(s) à régler, répartition claire
- Prévisualisation PDF avant envoi
- Historique des envois et des paiements visible sur la fiche facture

---

## 7. Conclusion

L’UI/UX Invoicing d’Odoo repose sur des **vues Liste/Kanban/Formulaire** standard, des **statuts de paiement et d’état** bien visibles, et des **wizards** (paiement, envoi). Pour Miyukini, il faut **adapter l’interface au périmètre facturation** (moins de comptabilité visible), **aligner les actions sur la gouvernance COG**, et **s’appuyer sur MiyuInvoice** pour les modèles et calculs tout en offrant une expérience claire et orientée facturation.

**Prochaines étapes :** Voir [Intégrations Cross-App](../03_integrations/Odoo%20Invoicing%20-%20Integrations%20Cross%20App.md) et [Guide Implémentation](../06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md).

---

**Document** : Odoo Invoicing — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
