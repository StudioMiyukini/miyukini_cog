# Odoo Inventory — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Inventory d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (réception, livraison, transfert, inventaire)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles

### 1.1 Préparateur de Commande (Picker / Warehouse Worker)

**Profil :**
- Prépare les commandes de livraison
- Scanne les produits avec code-barres
- Valide les prélèvements
- Gère les colis et emballages

**Permissions :**
- Consultation des pickings `outgoing` assignés
- Modification des quantités faites (`quantity` sur `move_line_ids`)
- Validation des pickings
- Impression des bons de livraison

**Fréquence d'usage :** Quotidienne, plusieurs fois par jour

**Contexte d'usage :** Entrepôt, mobile/tablette avec scanner code-barres

---

### 1.2 Réceptionniste (Receiving Clerk)

**Profil :**
- Réceptionne les marchandises fournisseurs
- Vérifie les quantités reçues
- Crée les lots/numéros de série
- Valide les réceptions

**Permissions :**
- Consultation des pickings `incoming`
- Création de lots/SN lors de la réception
- Modification des quantités reçues
- Validation des réceptions

**Fréquence d'usage :** Quotidienne, plusieurs fois par jour

**Contexte d'usage :** Quai de réception, poste fixe ou mobile

---

### 1.3 Gestionnaire de Stock (Stock Manager)

**Profil :**
- Gère les emplacements et entrepôts
- Configure les règles d'approvisionnement
- Effectue les inventaires physiques
- Analyse les mouvements de stock
- Gère les ajustements

**Permissions :**
- Accès complet à tous les pickings
- Configuration des emplacements et entrepôts
- Gestion des inventaires physiques
- Consultation des rapports
- Configuration des règles de stock

**Fréquence d'usage :** Quotidienne à hebdomadaire

**Contexte d'usage :** Bureau, poste de travail

---

### 1.4 Responsable Logistique (Logistics Manager)

**Profil :**
- Supervise les opérations d'entrepôt
- Configure les routes et stratégies
- Analyse les performances
- Gère les transferts inter-entrepôts
- Optimise les processus

**Permissions :**
- Accès à tous les pickings et moves
- Configuration des entrepôts et routes
- Consultation des rapports avancés
- Gestion des transferts inter-entrepôts

**Fréquence d'usage :** Quotidienne

**Contexte d'usage :** Bureau, tableau de bord

---

### 1.5 Opérateur Transfert Interne (Internal Transfer Operator)

**Profil :**
- Effectue les transferts entre emplacements
- Gère les mouvements internes
- Optimise le rangement

**Permissions :**
- Consultation des pickings `internal`
- Création de transferts internes
- Validation des transferts

**Fréquence d'usage :** Quotidienne

**Contexte d'usage :** Entrepôt, mobile

---

## 2. Parcours d'Onboarding

### 2.1 Première Utilisation (Gestionnaire de Stock)

**Étapes :**

1. **Configuration de l'entrepôt**
   - Navigation vers "Inventaire" → "Configuration" → "Entrepôts"
   - Création d'un premier entrepôt
   - Configuration des étapes de réception (1, 2 ou 3 étapes)
   - Configuration des étapes de livraison (1, 2 ou 3 étapes)
   - Validation

2. **Configuration des emplacements**
   - Création d'emplacements hiérarchiques
   - Configuration des stratégies de retrait (FIFO, LIFO, etc.)
   - Configuration des règles de rangement (putaway)

3. **Première réception**
   - Création d'un picking `incoming` depuis une commande fournisseur
   - Réception des produits
   - Création de lots si nécessaire
   - Validation de la réception

4. **Première livraison**
   - Création d'un picking `outgoing` depuis une commande client
   - Vérification de disponibilité
   - Prélèvement des produits
   - Validation de la livraison

**Durée estimée :** 30-60 minutes

**Points d'attention :**
- Comprendre la différence entre emplacements `view` et `internal`
- Comprendre les stratégies de retrait et leur impact
- Comprendre le système de réservation

---

### 2.2 Première Utilisation (Préparateur)

**Étapes :**

1. **Accès à la liste des commandes**
   - Navigation vers "Inventaire" → "Opérations" → "Livraisons"
   - Filtre sur "Prêt" (`state` = `assigned`)
   - Sélection d'une commande

2. **Préparation de la commande**
   - Consultation des lignes à préparer
   - Scan des produits avec code-barres
   - Vérification des quantités
   - Mise en colis si nécessaire

3. **Validation**
   - Clic sur "Valider"
   - Confirmation (ou création de backorder)
   - Impression du bon de livraison

**Durée estimée :** 10-15 minutes

**Points d'attention :**
- Comprendre l'interface de préparation
- Maîtriser le scan code-barres
- Comprendre les backorders

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Réception Marchandise Fournisseur

**Acteur :** Réceptionniste

**Contexte :** Arrivée d'une livraison fournisseur, commande d'achat déjà créée

**Étapes :**

1. **Accès à la réception**
   - Navigation vers "Inventaire" → "Opérations" → "Réceptions"
   - Filtre sur "En attente" (`state` = `confirmed` ou `assigned`)
   - Sélection du picking correspondant à la livraison

2. **Vérification des quantités**
   - Consultation des lignes de réception
   - Comparaison avec le bon de livraison fournisseur
   - Modification des quantités reçues si différent

3. **Gestion des lots/SN (si traçabilité)**
   - Si `use_create_lots` = True : création des lots/SN
   - Saisie des numéros de série ou noms de lots
   - Vérification des doublons

4. **Mise en emplacement**
   - Application automatique des règles de rangement (putaway)
   - Vérification de l'emplacement suggéré
   - Modification si nécessaire

5. **Validation**
   - Clic sur "Valider"
   - Si quantités partielles : choix de créer un backorder ou non
   - Confirmation de la réception
   - Impression du rapport de réception (si configuré)

**Durée estimée :** 5-15 minutes selon nombre de lignes et traçabilité

**Points de friction :**
- Saisie manuelle des lots/SN peut être fastidieuse
- Gestion des écarts de quantité nécessite une décision
- Règles de rangement peuvent suggérer des emplacements non optimaux

**Améliorations possibles :**
- Import batch de lots/SN depuis fichier
- Suggestions intelligentes d'emplacements
- Validation rapide si quantités conformes

---

### 3.2 Scénario : Préparation Commande Client

**Acteur :** Préparateur

**Contexte :** Commande client confirmée, picking `outgoing` créé automatiquement

**Étapes :**

1. **Accès à la commande**
   - Navigation vers "Inventaire" → "Opérations" → "Livraisons"
   - Filtre sur "Prêt" (`state` = `assigned`)
   - Sélection de la commande à préparer
   - Ouverture depuis le Kanban des opérations

2. **Vérification de disponibilité**
   - Consultation du statut de disponibilité (`products_availability`)
   - Si "Not Available" : vérification de la date prévue (`forecast_expected_date`)
   - Décision de préparer partiellement ou d'attendre

3. **Prélèvement des produits**
   - Mode liste ou vue détaillée des opérations
   - Scan code-barres de chaque produit
   - Vérification automatique de la quantité
   - Prélèvement selon stratégie de retrait (FIFO, LIFO, etc.)

4. **Gestion des lots/SN (si traçabilité)**
   - Si `use_existing_lots` = True : sélection des lots/SN
   - Scan des numéros de série
   - Vérification de la disponibilité du lot/SN à l'emplacement source

5. **Mise en colis**
   - Si nécessaire : création de colis
   - Scan des produits à mettre dans le colis
   - Attribution d'un nom/référence au colis
   - Impression de l'étiquette colis (si configuré)

6. **Validation**
   - Vérification des quantités prélevées
   - Clic sur "Valider"
   - Si quantités partielles : choix de créer un backorder
   - Confirmation de la livraison
   - Impression du bon de livraison (si configuré)

**Durée estimée :** 10-30 minutes selon nombre de lignes et complexité

**Points de friction :**
- Prélèvement manuel peut être long pour grandes commandes
- Gestion des lots/SN peut ralentir le processus
- Backorders créent de la complexité

**Améliorations possibles :**
- Mode "batch picking" pour préparer plusieurs commandes simultanément
- Optimisation automatique du parcours de prélèvement
- Suggestions intelligentes de lots/SN selon dates d'expiration

---

### 3.3 Scénario : Transfert Interne entre Emplacements

**Acteur :** Opérateur Transfert Interne

**Contexte :** Besoin de déplacer des produits d'un emplacement à un autre

**Étapes :**

1. **Création du transfert**
   - Navigation vers "Inventaire" → "Opérations" → "Transferts Internes"
   - Clic sur "Nouveau"
   - Sélection du type d'opération `internal`
   - Sélection de l'emplacement source
   - Sélection de l'emplacement destination

2. **Ajout des produits**
   - Ajout de lignes produits
   - Quantités à transférer
   - Vérification de disponibilité à l'emplacement source

3. **Confirmation**
   - Clic sur "Confirmer"
   - Réservation automatique des produits
   - État passe à "Prêt"

4. **Exécution du transfert**
   - Prélèvement à l'emplacement source
   - Scan des produits
   - Déplacement physique
   - Mise en place à l'emplacement destination
   - Application des règles de rangement

5. **Validation**
   - Clic sur "Valider"
   - Confirmation du transfert

**Durée estimée :** 5-15 minutes

**Points de friction :**
- Création manuelle peut être fastidieuse
- Pas de suggestion automatique d'emplacements optimaux

**Améliorations possibles :**
- Création depuis un emplacement source (vue quants)
- Suggestions d'emplacements destination selon règles de rangement

---

### 3.4 Scénario : Inventaire Physique

**Acteur :** Gestionnaire de Stock

**Contexte :** Inventaire cyclique ou ponctuel planifié

**Étapes :**

1. **Planification**
   - Navigation vers "Inventaire" → "Opérations" → "Inventaire"
   - Filtre sur l'emplacement à inventorier
   - Ou création d'une session d'inventaire

2. **Comptage**
   - Mode inventaire activé (`inventory_mode` = True)
   - Consultation des quants à l'emplacement
   - Saisie des quantités comptées (`inventory_quantity`)
   - Assignation à un utilisateur si nécessaire (`user_id`)

3. **Détection des écarts**
   - Calcul automatique des écarts (`inventory_diff_quantity`)
   - Identification des quants avec écarts significatifs
   - Vérification des mouvements depuis dernier comptage (`is_outdated`)

4. **Résolution des conflits**
   - Si `is_outdated` = True : wizard de conflit
   - Choix de conserver le comptage ou d'annuler
   - Explication des écarts

5. **Application des ajustements**
   - Clic sur "Appliquer" pour chaque quant
   - Ou application globale via "Appliquer tout"
   - Création automatique des `stock.move` d'ajustement
   - Validation des moves créés

6. **Finalisation**
   - Vérification des ajustements appliqués
   - Mise à jour de `last_inventory_date` sur l'emplacement
   - Calcul de la prochaine date d'inventaire (`next_inventory_date`)

**Durée estimée :** 30 minutes à plusieurs heures selon taille de l'inventaire

**Points de friction :**
- Comptage manuel peut être long et sujet à erreurs
- Gestion des conflits peut être complexe
- Import depuis fichier peut être nécessaire pour grands inventaires

**Améliorations possibles :**
- Import/export Excel pour comptage hors ligne
- Scan code-barres pour comptage rapide
- Suggestions automatiques d'ajustements selon historique

---

### 3.5 Scénario : Gestion des Backorders

**Acteur :** Préparateur ou Gestionnaire

**Contexte :** Validation d'un picking avec quantités partielles

**Étapes :**

1. **Validation partielle**
   - Préparation d'une commande avec quantités partielles
   - Clic sur "Valider"
   - Détection automatique des quantités manquantes

2. **Wizard de backorder**
   - Affichage du wizard "Create Backorder?"
   - Liste des pickings avec quantités restantes
   - Choix pour chaque picking : créer backorder ou non (`to_backorder`)

3. **Décision**
   - Si "Create Backorder" : création d'un nouveau picking avec quantités restantes
   - Si "No Backorder" : annulation des quantités restantes
   - Confirmation

4. **Traitement du backorder**
   - Le backorder apparaît dans la liste des livraisons
   - Reprend le workflow normal (confirmation → réservation → préparation → validation)
   - Lien visible vers le picking original (`backorder_id`)

**Durée estimée :** 2-5 minutes

**Points de friction :**
- Décision peut être difficile si plusieurs pickings
- Gestion des backorders multiples peut être complexe

**Améliorations possibles :**
- Règles automatiques de création de backorder selon contexte
- Vue consolidée des backorders liés

---

### 3.6 Scénario : Retour Client

**Acteur :** Réceptionniste ou Gestionnaire

**Contexte :** Client retourne des produits déjà livrés

**Étapes :**

1. **Accès au picking original**
   - Navigation vers "Inventaire" → "Opérations" → "Livraisons"
   - Recherche du picking de livraison original
   - Ouverture du picking

2. **Création du retour**
   - Clic sur "Retour" (`action_return()`)
   - Ouverture du wizard de retour
   - Sélection des produits à retourner
   - Quantités à retourner (par défaut = quantités livrées)

3. **Configuration**
   - Choix entre "Retour" et "Retour pour échange"
   - Si échange : création simultanée d'une nouvelle livraison
   - Validation du wizard

4. **Traitement du retour**
   - Création d'un nouveau picking `incoming` avec `return_id` lié
   - Emplacements inversés (customer → stock)
   - Confirmation automatique
   - Réception des produits retournés
   - Validation

**Durée estimée :** 5-10 minutes

**Points de friction :**
- Identification du picking original peut être difficile
- Gestion des lots/SN retournés peut être complexe

**Améliorations possibles :**
- Recherche par référence client ou numéro de commande
- Validation automatique des lots/SN retournés

---

### 3.7 Scénario : Configuration Règles d'Approvisionnement

**Acteur :** Responsable Logistique

**Contexte :** Configuration d'un nouveau produit ou optimisation des approvisionnements

**Étapes :**

1. **Accès aux routes**
   - Navigation vers "Inventaire" → "Configuration" → "Routes"
   - Consultation des routes existantes
   - Ou création d'une nouvelle route

2. **Configuration de la route**
   - Nom de la route
   - Sélection des entrepôts applicables
   - Sélection des produits/catégories applicables
   - Activation

3. **Configuration des règles**
   - Ajout de règles dans la route
   - Configuration de chaque règle :
     - Emplacement source (`location_src_id`)
     - Emplacement destination (`location_dest_id`)
     - Type d'opération (`picking_type_id`)
     - Action (pull, push, pull_push)
     - Méthode d'approvisionnement (MTS, MTO, MTS else MTO)
   - Ordre des règles (`sequence`)

4. **Test**
   - Création d'un mouvement de test
   - Vérification de l'application des règles
   - Ajustements si nécessaire

**Durée estimée :** 15-30 minutes

**Points de friction :**
- Configuration peut être complexe pour routes multi-étapes
- Compréhension des différents types d'actions nécessite expertise

**Améliorations possibles :**
- Wizards guidés pour création de routes courantes
- Simulation avant activation

---

## 4. Points de Friction Identifiés

### 4.1 Complexité de la Réservation

**Problème :** La réservation automatique peut être difficile à comprendre et déboguer.

**Impact :** Utilisateurs confus quand produits non réservés, difficultés à identifier la cause.

**Recommandations Miyukini :**
- Interface claire montrant pourquoi une réservation a échoué
- Explication des stratégies de retrait appliquées
- Suggestions d'actions correctives

---

### 4.2 Gestion des Lots/SN

**Problème :** Saisie manuelle des lots/SN peut être fastidieuse et source d'erreurs.

**Impact :** Ralentissement du processus de réception, erreurs de saisie.

**Recommandations Miyukini :**
- Import batch depuis fichier
- Génération automatique selon patterns
- Validation en temps réel des doublons

---

### 4.3 Backorders Multiples

**Problème :** Gestion de plusieurs backorders liés peut être complexe.

**Impact :** Difficulté à suivre l'état global d'une commande.

**Recommandations Miyukini :**
- Vue consolidée montrant picking original + backorders
- Indicateurs visuels clairs de l'état global
- Workflow simplifié pour traitement des backorders

---

### 4.4 Performance sur Grands Volumes

**Problème :** L'interface peut être lente avec beaucoup de pickings/moves.

**Impact :** Ralentissement des opérations quotidiennes.

**Recommandations Miyukini :**
- Pagination intelligente
- Filtres optimisés
- Cache des données fréquemment consultées

---

### 4.5 Configuration Complexe

**Problème :** Configuration des entrepôts, routes, règles nécessite expertise.

**Impact :** Erreurs de configuration, sous-utilisation des fonctionnalités.

**Recommandations Miyukini :**
- Wizards guidés pour configuration courante
- Templates de configuration
- Documentation contextuelle

---

## 5. Recommandations pour Miyukini

### 5.1 Interface Mobile-First

**Recommandation :** Prioriser l'interface mobile pour les opérations d'entrepôt.

**Justification :** Les préparateurs et réceptionnistes travaillent sur mobile/tablette avec scanner.

**Implémentation :**
- Interface responsive optimisée pour mobile
- Scan code-barres natif
- Gestes tactiles pour actions rapides

---

### 5.2 Feedback en Temps Réel

**Recommandation :** Fournir un feedback immédiat sur les actions (réservation, validation).

**Justification :** Réduit les erreurs et améliore la confiance des utilisateurs.

**Implémentation :**
- Notifications visuelles claires
- Messages d'erreur explicites avec suggestions
- Indicateurs de progression

---

### 5.3 Automatisation Intelligente

**Recommandation :** Automatiser les décisions courantes (création backorder, choix emplacement).

**Justification :** Réduit la charge cognitive et accélère les processus.

**Implémentation :**
- Règles configurables pour automatisation
- Apprentissage des préférences utilisateur
- Validation humaine pour cas exceptionnels

---

### 5.4 Traçabilité Complète

**Recommandation :** Fournir une traçabilité complète et accessible de tous les mouvements.

**Justification :** Essentiel pour audits, conformité, et résolution de problèmes.

**Implémentation :**
- Historique complet visible depuis chaque entité
- Liens clairs entre pickings, moves, quants
- Export pour audits externes

---

### 5.5 Intégration Native

**Recommandation :** Intégration native avec Sales, Purchase, Accounting via Opérateurs Miyukini.

**Justification :** Réduit la friction entre modules et améliore la cohérence.

**Implémentation :**
- Opérateurs collaborant via Mandats de Permission
- Échanges de données via WriteIntent vers KindMother
- Synchronisation automatique des états

---

## 6. Conclusion

Les parcours utilisateur d'Odoo Inventory révèlent :

- **Diversité des personas** : Du préparateur terrain au responsable logistique
- **Complexité des workflows** : Multiples étapes et décisions
- **Besoin de mobilité** : Interface mobile essentielle
- **Importance de la traçabilité** : Lots, SN, historique critiques

L'implémentation Miyukini devra :
- Simplifier les parcours tout en conservant la flexibilité
- Fournir des interfaces adaptées à chaque persona
- Automatiser les décisions courantes
- Assurer une traçabilité complète via la gouvernance COG

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
