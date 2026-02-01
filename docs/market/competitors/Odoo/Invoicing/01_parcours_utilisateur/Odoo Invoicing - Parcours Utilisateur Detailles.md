# Odoo Invoicing — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Invoicing d'Odoo : personas, scénarios d'usage, onboarding et points de friction, pour guider l'implémentation d'un équivalent Miyukini centré sur la facturation.

**Source d'analyse :** Interface Odoo Invoicing, workflows du module account (périmètre facturation).

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (facturation)
- Parcours d'onboarding Invoicing
- Scénarios d'usage principaux (création, envoi, paiement)
- Points de friction et recommandations Miyukini

**Hors scope :**
- Parcours comptabilité complète (voir Accounting)
- Détails techniques (guide d'implémentation)

---

## 1. Personas et Rôles

### 1.1 Utilisateur Facturation (Invoice User)

**Profil :**
- Crée et envoie les factures clients
- Saisit les factures fournisseurs
- Enregistre les paiements
- Consulte l’état des factures (payées, à relancer)

**Permissions :**
- Création / modification de factures (brouillon)
- Validation des factures
- Envoi par email
- Enregistrement des paiements et réconciliation

### 1.2 Responsable / Comptable (Manager / Accountant)

**Profil :**
- Valide les factures importantes
- Configure les conditions de paiement et les modèles
- Consulte les rapports de facturation et encaissements

**Permissions :**
- Lecture de toutes les factures et paiements
- Validation (éventuellement selon seuils)
- Configuration (journaux, conditions de paiement, modèles d’email)

### 1.3 Client (Portail)

**Profil :**
- Reçoit les factures par email
- Consulte ses factures sur le portail
- Peut régler en ligne (si activé)

**Permissions :**
- Accès limité à ses propres factures et documents
- Pas d’accès à la configuration ni aux autres partenaires

---

## 2. Parcours d'Onboarding

### 2.1 Première Utilisation (Invoicing Standalone)

**Étapes :**

1. **Paramétrage minimal**
   - Entreprise : nom, devise, pays
   - Journal des ventes et des achats (souvent créés par défaut)
   - Conditions de paiement courantes (30 j, 45 j, etc.)
   - Taxes de base (TVA si applicable)

2. **Première facture client**
   - Nouvelle facture → choix du client
   - Ajout de lignes (produit/service, quantité, prix)
   - Validation → numéro de facture
   - Envoi par email ou téléchargement PDF

3. **Première facture fournisseur**
   - Nouvelle facture fournisseur → fournisseur
   - Saisie des lignes (ou import PDF selon modules)
   - Validation et enregistrement du paiement si déjà réglée

**Durée estimée :** 30 min à 1 h pour un premier usage guidé.

**Points de friction :**
- Comprendre la différence brouillon / validé / envoyé
- Saisie des comptes comptables si visibles (Invoicing peut masquer une partie)
- Gestion des taxes selon le pays

### 2.2 Onboarding Utilisateur Standard

**Objectif :** Créer et envoyer une facture client sans formation longue.

**Étapes :**
1. Accéder au menu Factures (ou Factures clients)
2. Créer une facture
3. Sélectionner le client
4. Ajouter des lignes (produit, quantité, prix)
5. Vérifier le total et les taxes
6. Valider
7. Envoyer par email ou imprimer

**Points de friction :**
- Trop de champs affichés (référence, date d’échéance, conditions de paiement)
- Vocabulaire comptable (journal, compte) si non masqué

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Émettre une facture client

**Acteur :** Utilisateur Facturation

**Étapes :**
1. Créer une facture client (brouillon)
2. Choisir le client (partenaire)
3. Renseigner date de facture, échéance ou conditions de paiement
4. Ajouter des lignes (produits/services, quantités, prix, remise)
5. Vérifier les montants HT/TTC et les taxes
6. Valider la facture (obtention du numéro)
7. Envoyer par email (avec PDF) ou télécharger le PDF
8. (Optionnel) Marquer comme envoyée / suivre les ouvertures

**Critères de succès :** Facture numérotée, envoyée au client, visible dans la liste des factures avec statut approprié.

### 3.2 Scénario : Enregistrer un paiement client

**Acteur :** Utilisateur Facturation

**Étapes :**
1. Ouvrir la facture concernée (ou le menu Paiements)
2. Cliquer sur "Enregistrer un paiement"
3. Choisir le montant, la date, le journal (banque/caisse)
4. Lier éventuellement à plusieurs factures (répartition)
5. Valider le paiement
6. Vérifier que le statut de la facture passe à "Payée" ou "Partiellement payée"

**Critères de succès :** Paiement enregistré, facture(s) réconciliée(s), statut à jour.

### 3.3 Scénario : Créer un avoir (remboursement client)

**Acteur :** Utilisateur Facturation ou Responsable

**Étapes :**
1. À partir de la facture d’origine : "Créer un avoir"
2. Ajuster les lignes ou montants si besoin (avoir partiel)
3. Valider l’avoir
4. Envoyer l’avoir au client
5. (Optionnel) Enregistrer un remboursement ou imputer sur une prochaine facture

**Critères de succès :** Avoir numéroté, lié à la facture d’origine, envoyé.

### 3.4 Scénario : Saisir et régler une facture fournisseur

**Acteur :** Utilisateur Facturation

**Étapes :**
1. Créer une facture fournisseur (brouillon)
2. Sélectionner le fournisseur
3. Saisir les lignes (manuel ou import selon modules)
4. Valider la facture
5. Enregistrer le paiement (date, montant, journal)
6. Vérifier le statut "Payée"

**Critères de succès :** Facture fournisseur enregistrée et réglée, statut cohérent.

### 3.5 Scénario : Consulter les factures à relancer

**Acteur :** Utilisateur Facturation ou Responsable

**Étapes :**
1. Filtrer les factures clients : non payées, échéance dépassée (ou proche)
2. Consulter la liste et les montants restants dus
3. Envoyer des relances (email) ou exporter la liste
4. Après encaissement, enregistrer le paiement (scénario 3.2)

**Critères de succès :** Liste fiable des impayés et suivi des relances.

---

## 4. Points de Friction Identifiés

### 4.1 Complexité perçue

- **Trop de champs** : Référence, journal, compte par défaut peuvent déstabiliser un utilisateur non comptable.
- **Vocabulaire** : "Écriture", "Journal", "Compte" — à adapter ou masquer dans une version "Invoicing only".
- **États** : Brouillon / Validé / Envoyé / Payé / Partiel — nécessite un statut visuel clair (badges, couleurs).

### 4.2 Workflow

- **Envoi** : Distinction entre "Générer PDF", "Envoyer par email" et "Marquer comme envoyé" à clarifier.
- **Paiement** : Réconciliation manuelle sur plusieurs factures peut être source d’erreurs si l’interface n’est pas guidée.
- **Avoirs** : Lien facture / avoir et impact sur les montants restants dus à rendre explicite.

### 4.3 Performance et ergonomie

- **Listes longues** : Filtres et recherche indispensables (client, date, statut, montant).
- **Saisie répétitive** : Modèles de facture, lignes par défaut, conditions de paiement par client pour réduire la charge.
- **Multi-devises** : Affichage clair de la devise de la facture et du montant en devise entreprise si pertinent.

---

## 5. Recommandations pour Miyukini

### 5.1 Parcours simplifié "Facturation uniquement"

- Proposer un parcours type : Nouvelle facture → Client → Lignes → Valider → Envoyer.
- Masquer ou dériver automatiquement les champs comptables (compte, journal) pour les utilisateurs "Facturation".
- Exposer clairement les états : Brouillon, Validée, Envoyée, Payée / Partielle.

### 5.2 Gouvernance et permissions

- Mandats de Permission pour : création facture, validation, envoi, enregistrement paiement.
- StrongFather pour validation et envoi ; KindMother pour persistance (WriteIntent).
- Niveaux de sécurité (WorrySentinel) selon données (factures, paiements, coordonnées bancaires).

### 5.3 Réutilisation des services

- S’appuyer sur **MiyuInvoice** pour la structure facture/lignes/taxes/conditions de paiement.
- Intégration avec Miyukini Sales (factures depuis commandes) et éventuellement Purchase.
- Portail client : Façade Publique Gouvernée + Mandat Public d’Accès pour consultation factures.

### 5.4 UX

- Statut visuel unique (badge ou indicateur) : Brouillon / Validée / Envoyée / Payée / Partielle / Avoir.
- Actions contextuelles évidentes : Valider, Envoyer, Enregistrer paiement, Créer avoir.
- Filtres par défaut utiles : "À envoyer", "À relancer", "Ce mois".

---

## 6. Conclusion

Les parcours Invoicing d’Odoo couvrent la **facturation de A à Z** (création, validation, envoi, paiement, avoirs) avec des personas clairs (utilisateur facturation, responsable, client). Pour Miyukini, il convient de **simplifier les parcours** (moins de champs comptables visibles), de **gouverner par Mandats et Cores**, et de **réutiliser MiyuInvoice** tout en offrant une expérience orientée "facturation" plutôt que "comptabilité complète".

**Prochaines étapes :** Voir [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Invoicing%20-%20Specifications%20Operateurs%20Miyukini.md) et [Guide Implémentation](../06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md).

---

**Document** : Odoo Invoicing — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
