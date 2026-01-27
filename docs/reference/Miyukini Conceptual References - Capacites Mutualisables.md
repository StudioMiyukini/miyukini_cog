# Capacités Mutualisables — Analyse Opérateurs

> Capacités apparaissant dans ≥3 domaines distincts.
> Candidats à la mutualisation en Outils et Kits d'Outils réutilisables.

**Note terminologique :** Dans ce document, "produit" au sens commercial (SaaS, app, site) désigne ce que Miyukini appelle un **Opérateur** (Strate 7). "Module produit" désigne un **Outil** ou **Kit d'Outils** (Strate 6).

---

## Capacités mutualisables — Niveau fort (≥4 domaines)

### Gestion des médias (images, vidéos, fichiers)
**Domaines :** CMS, Event, E-shop, Jeux, POS  
**Note :** Gestion d'assets, upload, stockage, transformations.

### Notifications transactionnelles
**Domaines :** CMS, Planity, Event, E-shop, POS, Jeux  
**Note :** Confirmations, rappels, alertes métier.

### Gestion des prix
**Domaines :** E-shop, POS, Event (tarifs), Jeux (achats in-app), CMS (plans premium)  
**Note :** Prix de base, variations, promotions, taxes.

### Gestion des stocks / inventaire
**Domaines :** E-shop, POS, Event (places), Jeux (ressources), Planity (créneaux)  
**Note :** Quantités, réservations, alertes seuil.

### Historique des transactions / actions
**Domaines :** E-shop, POS, Event, Jeux, Planity, CMS (audit)  
**Note :** Traçabilité des opérations métier.

### Rappels et notifications de délai
**Domaines :** Planity, Event, E-shop (commandes), Jeux (timers), POS (rappels)  
**Note :** Notifications temporelles, rappels automatiques.

---

## Capacités mutualisables — Niveau moyen (3 domaines)

### Hiérarchie de contenus
**Domaines :** CMS, Event (programmation), E-shop (catégories)  
**Note :** Arborescence, navigation hiérarchique.

### Taxonomies (catégories, tags)
**Domaines :** CMS, Event, E-shop  
**Note :** Classification, filtrage, organisation.

### Réservations
**Domaines :** Planity, Event, E-shop (produits réservables)  
**Note :** Réservation de ressources, créneaux, produits.

### Annulations et reports
**Domaines :** Planity, Event, E-shop  
**Note :** Annulation, report, remboursement partiel.

### Gestion des ressources (personnes, salles, équipements)
**Domaines :** Planity, Event, POS (caissiers, terminaux)  
**Note :** Ressources partagées, disponibilités.

### Gestion des articles / produits
**Domaines :** E-shop, POS, Event (merchandising)  
**Note :** Catalogue, références, variantes.

### Gestion des taxes
**Domaines :** E-shop, POS, Event (billetterie)  
**Note :** Calcul, règles fiscales, déclarations.

### Classements (leaderboards)
**Domaines :** Jeux, Event (classements), E-shop (top ventes)  
**Note :** Rankings, scores, comparaisons.

### Événements temporaires
**Domaines :** Jeux, Event, E-shop (promotions limitées)  
**Note :** Événements datés, campagnes limitées.

### Gestion de parties / sessions
**Domaines :** Jeux, Event (sessions), Planity (séances)  
**Note :** Sessions utilisateur, états temporaires.

### Promotions et coupons
**Domaines :** E-shop, Event, Jeux (bonus)  
**Note :** Codes promo, réductions, offres spéciales.

---

## Capacités génériques déjà mutualisables (1-18)

Les capacités génériques 1-18 sont **déjà mutualisables** par définition. Elles apparaissent dans tous les domaines ou presque.

**À mutualiser en priorité :**
- Gestion des utilisateurs
- Accès et rôles
- Contenu et données métier
- Navigation et organisation
- Formulaires et saisie
- Communication et notifications
- Administration produit
- Rapports et analyse métier
- Import / export et interopérabilité

**À laisser spécifiques produit :**
- Calendrier, planification et réservation (déjà couvert par Planity/Event)
- Automatisation avancée (règles complexes, circuits d'approbation lourds)
- Facturation et monétisation (logique métier trop spécifique)

---

## Capacités à mutualiser en priorité

1. **Gestion des médias** (fort — 5 domaines)
2. **Notifications transactionnelles** (fort — 6 domaines)
3. **Gestion des prix** (fort — 5 domaines)
4. **Gestion des stocks / inventaire** (fort — 5 domaines)
5. **Historique des transactions** (fort — 6 domaines)
6. **Rappels et notifications de délai** (fort — 5 domaines)
7. **Réservations** (moyen — 3 domaines)
8. **Gestion des ressources** (moyen — 3 domaines)
9. **Gestion des articles / produits** (moyen — 3 domaines)
10. **Gestion des taxes** (moyen — 3 domaines)

---

## Capacités à laisser spécifiques produit

### CMS uniquement
- Gestion de pages et articles
- Blocs de contenu modulaires
- Brouillons, prévisualisation, publication programmée
- Thèmes fonctionnels (structure)

### Planity uniquement
- Créneaux et disponibilités (spécifique rendez-vous)
- Notifications liées aux rendez-vous (spécifique)

### Event uniquement
- Programmation (scènes, horaires, activités)
- Gestion des intervenants / exposants
- Inscriptions et participations
- Billetterie conceptuelle
- Gestion des bénévoles
- Communication événementielle

### Jeux uniquement
- Comptes joueurs (profil de jeu)
- Progression et niveaux
- Ressources (gain, consommation) — spécifique gameplay
- Systèmes de récompenses — spécifique gameplay
- États persistants de jeu
- Boucles de gameplay (idle / temps réel)

### E-shop uniquement
- Panier
- Commandes
- Livraison et retrait
- Gestion des retours

### POS uniquement
- Encaissement
- Tickets et reçus
- Clôture de caisse
- Gestion multi-caisses

---

## Synthèse décisionnelle

**Outils / Kits d'Outils à créer :**
- Outil Médias
- Outil Notifications transactionnelles
- Outil Prix
- Outil Stocks / Inventaire
- Outil Historique / Traçabilité
- Outil Réservations (si ≥3 Opérateurs en ont besoin)
- Outil Ressources (si ≥3 Opérateurs en ont besoin)

**Capacités génériques 1-18 :** Déjà mutualisables, à réutiliser directement.

**Capacités spécifiques :** Laisser dans chaque Opérateur, sauf celles listées ci-dessus.
