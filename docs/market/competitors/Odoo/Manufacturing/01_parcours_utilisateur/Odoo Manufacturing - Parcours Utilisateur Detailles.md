# Odoo Manufacturing — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Manufacturing (MRP) d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, workflows Manufacturing / MRP, patterns Supply Chain.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Responsable Production / Planificateur (Production Manager / Planner)

**Profil :**
- Pilote la charge et les délais
- Crée et confirme les ordres de fabrication
- Utilise le plan directeur (MPS)
- Gère les priorités et les reports

**Permissions :**
- Création / modification / confirmation des OF
- Consultation des BOM et gammes
- Accès au MPS et aux rapports (délais, allocation)
- Gestion des backorders et reports

### 1.2 Opérateur atelier (Shop Floor Operator)

**Profil :**
- Exécute les ordres de travail sur le poste
- Scanne lots / numéros de série
- Démarre / arrête les opérations
- Signale rebuts, maintenance, qualité

**Permissions :**
- Accès au tableau de bord poste (work center)
- Démarrage / fin des WO
- Saisie quantités produites et temps
- Création alertes qualité / maintenance
- Lecture des instructions (worksheet)

### 1.3 Méthodiste / Bureau des méthodes (Process Engineer)

**Profil :**
- Définit les nomenclatures (BOM) et gammes (routing)
- Configure les postes de travail
- Gère les variantes et sous-ensembles
- Optimise les temps et les coûts

**Permissions :**
- Création / modification BOM et lignes
- Création / modification gammes et opérations
- Configuration postes de travail
- Consultation coûts et temps standards

### 1.4 Responsable stock / Supply Chain

**Profil :**
- Assure la disponibilité des composants
- Suit les réceptions et les livraisons sous-traitance
- Pilote les inventaires et les réapprovisionnements

**Permissions :**
- Consultation OF et mouvements matières / finis
- Réception et livraison (intégration Inventory)
- Points de commande et règles d’approvisionnement
- Rapports d’allocation et de délais

### 1.5 Direction / Contrôle de gestion

**Profil :**
- Consulte les coûts et la performance
- Valide les écarts et les clôtures
- Utilise les indicateurs (OEE, délais, coûts)

**Permissions :**
- Lecture rapports (OEE, analyse production, coûts)
- Validation des clôtures (selon règles)
- Pas de modification des BOM / gammes en routine

---

## 2. Parcours d'Onboarding

### 2.1 Première installation Manufacturing

**Étapes :**

1. **Produits fabriqués**
   - Créer ou importer les produits à fabriquer
   - Définir type « fabriqué » (manufacture)
   - Unité de mesure et traçabilité (lot/série si besoin)

2. **Composants et stock**
   - Produits composants en stock (ou achetés)
   - Emplacements (stock, production, rebut)
   - Types d’opération (réception, production, livraison)

3. **Nomenclatures (BOM)**
   - Créer au moins une BOM par produit fabriqué
   - Saisir composants et quantités
   - Choisir consommation (strict / flexible) et ready to produce

4. **Postes de travail (optionnel)**
   - Créer les postes (nom, capacité, coût horaire)
   - Calendrier de disponibilité
   - Postes alternatifs si besoin

5. **Gammes (optionnel)**
   - Créer un routing par gamme
   - Lier les opérations aux postes et aux temps
   - Attacher la gamme aux BOM concernées

6. **Premier ordre de fabrication**
   - Créer un OF manuel (produit, quantité, BOM)
   - Confirmer, réserver, produire, clôturer
   - Vérifier mouvements stock et coûts

**Durée estimée :** 2 à 6 heures selon complexité (BOM simples vs gammes multi-opérations)

**Points de friction identifiés :**
- Choix entre fabrication « simple » (sans gamme) et « avec opérations » pas toujours évident
- Dépendances entre opérations à bien configurer pour éviter blocages
- Calendriers et capacités des postes à renseigner pour une planification réaliste

### 2.2 Première utilisation (Opérateur atelier)

**Étapes :**

1. **Accès au tableau de bord poste**
   - Sélection du poste (ou affectation automatique)
   - Liste des WO en attente / prêts

2. **Démarrage d’un ordre de travail**
   - Ouvrir le WO
   - Consulter instructions (worksheet)
   - Démarrer (bouton / scan)

3. **Exécution**
   - Prélever composants (scan si traçabilité)
   - Saisir quantités produites
   - Signaler rebut ou alerte si besoin

4. **Clôture du WO**
   - Terminer l’opération
   - Saisie temps et quantité finale
   - Passage au WO suivant ou fin d’OF

**Points de friction :**
- Interface poste parfois chargée ; besoin de vue épurée sur tablette
- Dépendances : l’opérateur peut ne pas comprendre pourquoi un WO reste « en attente »

---

## 3. Scénarios d'Usage Principaux

### 3.1 Fabrication « à la commande » (Make to Order)

1. Commande client reçue (Sales).
2. Besoin net calculé (demande – stock – OF en cours).
3. Planificateur crée un ou plusieurs OF (manuel ou MPS) avec origine = commande.
4. Confirmation OF → réservation composants, création WO si gamme.
5. Exécution atelier (WO) ou production simple.
6. Clôture OF → produit fini en stock, livraison possible.

**Acteurs :** Planificateur, Opérateurs, (Stock pour livraison)

### 3.2 Fabrication pour stock (Make to Stock)

1. Réapprovisionnement déclenché par point de commande ou planification (MPS).
2. OF créés automatiquement ou manuellement pour couvrir le besoin.
3. Même enchaînement : confirmation → exécution → clôture.
4. Produit fini disponible pour les ventes ou autres OF (sous-ensembles).

**Acteurs :** Planificateur, Opérateurs, Système (règles)

### 3.3 Sous-traitance

1. BOM (ou ligne) marquée sous-traitance ; partenaire et/ou route sous-traitance configurés.
2. OF confirmé → mouvements sortants (composants vers sous-traitant).
3. Commande fournisseur (ou bon de travail sous-traitant) suivie.
4. Réception produit fini (ou semi-fini) → clôture OF ou WO.

**Acteurs :** Planificateur, Achats, Stock (réceptions)

### 3.4 Gestion des retards et reports

1. Rapport « Délais » ou vue planification : OF / WO en retard.
2. Planificateur reporte les dates (OF ou WO) ou ajuste les priorités.
3. Option backorder : clôture partielle, reliquat sur nouvel OF.
4. Communication atelier (priorités affichées sur tableau de bord).

**Acteurs :** Planificateur, Opérateurs

### 3.5 Démontage (Unbuild) et rework

1. Saisie d’un ordre de démontage (produit fini → composants).
2. Mouvements : sortie produit fini, entrée composants (et rebut éventuel).
3. Utilisation : retour client, rework, reconditionnement.

**Acteurs :** Planificateur, Stock

### 3.6 Analyse et amélioration continue

1. Rapports OEE, analyse production, coûts par OF.
2. Identification des goulots et des pertes (rebut, attente).
3. Ajustement gammes, temps, postes ou BOM (méthodiste).
4. Mise à jour des standards et des coûts.

**Acteurs :** Méthodiste, Contrôle de gestion, Direction

---

## 4. Points de Friction Identifiés

- **Complexité BOM / gamme** : trop d’options (phantom, kit, consommation, dépendances) pour un premier déploiement.
- **Planification** : MPS et capacités nécessitent des données fiables (calendriers, temps) ; sinon reports manuels fréquents.
- **Interface atelier** : besoin d’une vue dédiée « poste » (tablette, scan) distincte de l’interface bureau.
- **Traçabilité** : lots/séries bien gérés en production mais configuration et formation importantes.
- **Sous-traitance** : frontière Stock / Purchase / Manufacturing à bien cadrer (qui crée quoi, quand).
- **Coûts** : coût standard vs moyen, répartition sous-produits, analytique — à aligner avec la comptabilité.

---

## 5. Recommandations pour Miyukini

- **Personas** : modéliser Planificateur, Opérateur atelier, Méthodiste, Stock/Supply Chain, Direction avec permissions distinctes (Master Butler).
- **Onboarding** : parcours guidé « Premier OF » (BOM simple puis avec gamme) et checklist configuration (postes, calendriers).
- **Parcours atelier** : Opérateur d’interface dédié « Shop Floor » (WO, temps, quantités, alertes) avec Mandats limités au poste.
- **Planification** : service MPS clair (demande, capacité, OF proposés) avec décision humaine (StrongFather) avant création OF.
- **Traçabilité** : intégration forte avec Inventory (lots/séries, mouvements) et WriteIntent pour toute modification stock.
- **Documentation** : glossaire BOM, OF, WO, poste, MPS, backorder, unbuild pour cohérence avec le reste Supply Chain.

---

**Document** : Odoo Manufacturing — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
