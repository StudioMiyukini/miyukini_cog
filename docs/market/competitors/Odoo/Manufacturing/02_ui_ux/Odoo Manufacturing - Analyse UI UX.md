# Odoo Manufacturing — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Manufacturing** (MRP) d'Odoo (version 19.0). Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, patterns Manufacturing / MRP, Supply Chain.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Gantt, Pivot, Graph)
- Tableau de bord poste (Shop Floor)
- Formulaires et champs clés
- Navigation et filtres
- Rapports et indicateurs
- Design atelier (tablette, scan)

**Hors scope :**
- Implémentation technique détaillée
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Ordres de fabrication (mrp.production)

**Vue Liste :**
- Colonnes : Référence, Produit, BOM, Quantité / Produit, État, Date début planifiée, Date fin planifiée, Origine
- Filtres : Brouillon, Confirmé, En cours, À clôturer, Terminé, Annulé ; par produit, BOM, période
- Groupement : État, Produit, BOM, Semaine/Mois
- Actions : Confirmer, Marquer comme fait, Planifier, Annuler, Créer backorder

**Vue Kanban :**
- Cartes par état (Draft, Confirmed, In Progress, Done)
- Infos : produit, quantité, dates, avancement
- Glisser-déposer pour changer d’état (selon config)

**Vue Formulaire :**
- En-tête : Référence, Produit, Quantité, BOM, État, Dates planifiées / réelles
- Onglets : Composants (move_raw_ids), Produits finis (move_finished_ids), Ordres de travail (workorder_ids), Notes, Historique
- Boutons : Confirmer, Réserver, Marquer comme fait, Clôturer, Créer backorder, Annuler
- Bloc « Ordres de travail » : liste ou Gantt des WO avec état et durée

**Vue Gantt (planification) :**
- OF et WO sur timeline
- Dépendances entre WO visibles
- Glisser pour modifier dates (report)
- Couleur par état ou priorité

### 1.2 Ordres de travail (mrp.workorder)

**Vue Liste :**
- Colonnes : OF, Opération, Poste, État, Quantité à produire / produite, Date début/fin, Durée
- Filtres : En attente, Prêt, En cours, Terminé ; par poste, OF, période

**Vue Formulaire :**
- En-tête : OF, Opération, Poste, État, Quantités, Durée prévue/réelle
- Instructions (worksheet), temps, alertes qualité/maintenance
- Boutons : Démarrer, Terminer, Bloquer, Annuler

### 1.3 Nomenclatures (mrp.bom)

**Vue Liste :**
- Colonnes : Produit, Code BOM, Quantité, Unité, Type (normal/phantom/kit), Société
- Filtres : Actives, Par produit, Par type

**Vue Formulaire :**
- En-tête : Produit fabriqué, Quantité, Unité, Type, Consommation, Ready to produce
- Table des composants : Produit, Quantité, Unité, Opération (si gamme)
- Lien vers gamme (routing) si utilisée
- Boutons : Importer variantes, Dupliquer

### 1.4 Postes de travail (mrp.workcenter)

**Vue Liste :**
- Colonnes : Nom, Code, Capacité, Efficacité, Coût horaire
- Filtres : Par société

**Vue Formulaire :**
- Données de capacité, coûts, calendrier, postes alternatifs
- Onglet opérations (routing lines) utilisant ce poste

### 1.5 Gammes (mrp.routing)

**Vue Liste / Formulaire :**
- Nom de gamme
- Lignes d’opérations : Poste, Temps réglage, Temps cycle, Dépendances, Instructions

---

## 2. Tableau de bord poste (Shop Floor)

**Objectif :** Interface atelier pour exécuter les WO sur un poste (souvent tablette).

**Éléments :**
- Sélection du poste (ou utilisateur/poste par défaut)
- Liste des WO à faire : Prêt, puis En attente (avec indication des dépendances)
- Carte WO : nom opération, OF, produit, quantité à produire, instructions (worksheet)
- Boutons : Démarrer, Terminer, Mettre en pause (selon config)
- Saisie : quantité produite, lot/série (scan possible)
- Alertes : qualité, maintenance, blocage
- Indicateurs temps : durée prévue, écoulée

**Patterns :**
- Vue plein écran, peu de menus
- Scan code-barres pour WO ou composant
- Feedback visuel (couleur état, progression)
- Accès rapide aux instructions (PDF, lien)

---

## 3. Planification (MPS)

**Vue type « Plan directeur » :**
- Grille temps (semaines/jours) × produits ou OF
- Quantités demandées / prévues / disponibles
- Proposition d’OF à créer (wizard ou bouton)
- Glisser pour déplacer ou ajuster quantités

**Filtres :** Période, entrepôt, famille de produits

---

## 4. Rapports et Tableaux de bord

- **Délais :** OF/WO en retard, comparaison prévu/réel
- **Allocation :** utilisation des composants par OF, alertes rupture
- **OEE (Overall Equipment Effectiveness):** disponibilité, performance, qualité par poste
- **Analyse production :** quantités, coûts, écarts par OF, produit, période
- **Coûts OF :** détail matière + main-d’œuvre + overhead

**Vues :** Pivot (croisement dimensions), Graph (courbes, barres), Listes exportables

---

## 5. Patterns de Navigation

- **Menu Manufacturing :** Sous-menus Produits (BOM), Fabrication (OF), Ordres de travail, Postes de travail, Planification (MPS), Rapports
- **Drill-down :** Liste OF → Formulaire OF → Formulaire WO ; Liste BOM → Formulaire BOM → Lignes composants
- **Création rapide :** OF depuis commande vente (lien « Fabrication ») ; OF depuis MPS (action « Créer OF »)
- **Breadcrumb :** Manufacturing > Fabrication > [Réf OF]

---

## 6. Formulaires et Validations

- **OF :** Quantité > 0 ; BOM avec au moins une ligne ; produit de type « fabriqué »
- **BOM :** Lignes avec produit et quantité > 0 ; unicité produit/BOM/variante selon règles
- **WO :** Démarrage possible si état = ready (dépendances done) ; fin avec quantité et durée
- **Messages d’erreur :** Composants indisponibles, BOM manquante, poste surchargé (selon config)

---

## 7. Design responsive et atelier

- **Bureau :** Vues list/form complètes, Gantt, MPS, rapports
- **Tablette / poste :** Vue Shop Floor simplifiée, gros boutons, scan
- **Mobile :** Consultation OF/WO possible ; saisie limitée selon droits

---

## 8. Points d'Attention pour Miyukini

- **Opérateur d’interface Manufacturing** : vues liste/form/Gantt pour planification et suivi.
- **Opérateur d’interface Shop Floor** : vue dédiée poste (WO, temps, quantités, alertes) avec permissions restreintes (Mandat poste).
- **Cohérence** : mêmes patterns que Inventory/Sales (état, boutons d’action, filtres) pour faciliter l’adoption.
- **Accessibilité** : contrastes, libellés et retours d’erreur clairs pour l’atelier.

---

**Document** : Odoo Manufacturing — Analyse UI/UX  
**Version** : 1.0  
**Date** : 2026-02-01
