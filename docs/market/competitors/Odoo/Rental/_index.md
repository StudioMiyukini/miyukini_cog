# Odoo Rental — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Rental - Logique Metier Complete.md](./00_logique_metier/Odoo%20Rental%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (commandes location, lignes, tarification, stock Rental In/Out)
- Règles métier (tarification option la moins chère, pénalités, Security Time)
- Workflows (devis → confirmé → enlèvement → retour → facturation)
- Intégrations Sales, Stock, Sign, Invoicing

### 2. Parcours Utilisateur
📄 [Odoo Rental - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Rental%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Gestionnaire location, Commercial, Magasinier, Client)
- Parcours d'onboarding (configuration produits, paramètres)
- Scénarios d'usage (devis → confirmé → enlèvement → retour → facturé, prolongation)
- Points de friction et recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Rental - Analyse UI UX.md](./02_ui_ux/Odoo%20Rental%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vues principales (commandes, lignes, produits, configuration)
- Widgets (dates, tarification, statuts)
- Documents (reçu enlèvement/retour, Sign)
- Patterns de navigation et recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Rental - Integrations Cross App.md](./03_integrations/Odoo%20Rental%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (Sales, Stock, Sign, Invoicing)
- Flux de données inter-apps
- Mécanismes d'intégration
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Rental - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Rental%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs (RentalOrderOperator, RentalPricingOperator, RentalStockOperator, RentalUI)
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Rental - Guide Integration COG.md](./05_integration_cog/Odoo%20Rental%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Rental - Guide Implementation.md](./06_guides_implementation/Odoo%20Rental%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique (crates)
- Schémas de données (RentalOrder, RentalOrderLine, config, stock)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniRental` ou `MiyuRental`

**Opérateurs :**
- **RentalOrderOperator** : Gestion des commandes et lignes de location
- **RentalPricingOperator** : Tarification (grilles, calcul prix, pénalités)
- **RentalStockOperator** : Disponibilité, Security Time, emplacements Rental In/Out
- **RentalUI** : Interface utilisateur Location

**Équipe d'Opérateurs :** `RentalService`

---

## Source d'Analyse

**Documentation :** Odoo Rental (14.0–18.0), app `sale_rental`

**Version analysée :** Odoo 14.0–18.0 (documentation officielle)

**Date d'analyse :** 2026-02-01

---

## Notes

- Application Sales (extension de Sales) : commandes de location avec dates, tarification au temps, stock Rental In/Out
- Règle de calcul prix : une seule ligne de prix, option la moins chère pour couvrir la durée
- Intégrations : Sales, Stock, Sign (optionnel), Invoicing
- Équivalent Miyukini : MiyuRental + RentalService (Opérateurs + Contrats d'équipe)
