# Odoo Invoicing — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d’Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l’application **Invoicing** d’Odoo, en respectant l’architecture COG et la gouvernance Miyukini (Glossaire).

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Invoicing - Logique Métier](../00_logique_metier/Odoo%20Invoicing%20-%20Logique%20Metier%20Complete.md)
- MiyuInvoice (crate existante) : facturation

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Invoicing
- Équipe d’Opérateurs et Contrat d’Équipe
- Mandats de Permission (Standard, Validation, Envoi, Paiement)
- Niveaux de sécurité et intégration avec les Cores

**Hors scope :**
- Implémentation technique détaillée (voir Guide d’Implémentation)
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d’ensemble

L’équivalent **Invoicing** dans Miyukini s’appuie sur **MiyuInvoice** (facturation) et sur les Cores pour la gouvernance. Le périmètre est centré sur la facturation (création, validation, envoi, paiement) sans couvrir le grand livre complet ni le rapprochement bancaire (voir Accounting).

**Opérateurs identifiés (périmètre Invoicing) :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **InvoiceLedger** | Création, modification et validation des factures | Opérateur de Service |
| **InvoicePayment** | Enregistrement des paiements et réconciliation facture/paiement | Opérateur de Service |
| **InvoiceSend** | Envoi des factures (email, PDF, portail) | Opérateur de Service |
| **InvoiceTerms** | Conditions de paiement et échéanciers | Opérateur de Service |
| **InvoiceUI** | Interface utilisateur facturation | Opérateur d’Interface |

**Réutilisation :**
- **MiyuInvoice** : Kits et outils facturation (lignes, taxes, montants)
- **KindMother** : Persistance (WriteIntent) des factures et paiements
- **StrongFather** : Décisions (validation, envoi, réconciliation)
- **Ever Buddy** : Séquences (numéros de facture)
- **Master Butler** : Permissions (Mandats)
- **WorrySentinel** : Niveaux de sécurité

### 1.2 Équipe d’Opérateurs : InvoiceService

**Définition :**
> **InvoiceService est une Équipe d’Opérateurs qui collabore sous règles explicites pour délivrer le service de facturation (création, validation, envoi, paiement).**

**Composition :**
- InvoiceLedger (niveau sécurité 2)
- InvoicePayment (niveau sécurité 2–3)
- InvoiceSend (niveau sécurité 1–2)
- InvoiceTerms (niveau sécurité 2)
- InvoiceUI (niveau sécurité 1)

**Contrat d’Équipe :** Voir section 2.

---

## 2. Contrat d’Équipe InvoiceService

### 2.1 Opérateurs membres

- InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI

### 2.2 Flux autorisés

| De | Vers | Flux |
|----|------|------|
| InvoiceUI | InvoiceLedger | Création / modification / validation facture |
| InvoiceUI | InvoicePayment | Enregistrement paiement, réconciliation |
| InvoiceUI | InvoiceSend | Envoi facture (email, PDF) |
| InvoiceUI | InvoiceTerms | Consultation / configuration conditions de paiement |
| InvoiceLedger | KindMother | WriteIntent facture / lignes |
| InvoiceLedger | StrongFather | Décision validation |
| InvoiceLedger | Ever Buddy | Séquence numéro facture |
| InvoicePayment | KindMother | WriteIntent paiement, réconciliation |
| InvoicePayment | StrongFather | Décision réconciliation |
| InvoiceSend | KindMother | Lecture facture (génération PDF) |
| InvoiceSend | StrongFather | Décision envoi (optionnel selon politique) |
| InvoiceTerms | KindMother | Lecture / WriteIntent conditions de paiement |

### 2.3 Types d’échanges

- Requêtes lecture (facture, lignes, conditions de paiement)
- WriteIntent (création facture, validation, paiement, réconciliation)
- Décisions (StrongFather : validation, envoi, réconciliation)
- Séquences (Ever Buddy : numéro de facture)

### 2.4 Niveau de validation requis

- Création / modification facture brouillon : Mandat Standard (InvoiceLedger)
- Validation facture : Mandat Validation (StrongFather)
- Envoi facture : Mandat Envoi (InvoiceSend, éventuellement StrongFather)
- Enregistrement paiement : Mandat Paiement (InvoicePayment + StrongFather si seuil)
- Modification conditions de paiement : Mandat Configuration (InvoiceTerms)

### 2.5 Conditions préalables

- Environnement COG initialisé
- Plan comptable / journaux disponibles (via Accounting ou configuration minimale Invoicing)
- Partenaires (clients / fournisseurs) gérés par le système

---

## 3. Opérateurs détaillés

### 3.1 InvoiceLedger

**Rôle :** Gestion du cycle de vie des factures (création, modification, validation).

**Capacités :**
- Création / modification de factures (brouillon)
- Validation de factures (équilibre, séquence, calculs)
- Création d’avoirs (liés à une facture)
- Consultation des factures et lignes

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation
- **KindMother** : Persistance des factures et lignes (WriteIntent)
- **Ever Buddy** : Numéro de facture (séquence)
- **Master Butler** : Permissions (Mandat Standard / Validation)
- **WorrySentinel** : Niveau sécurité données facturation

**Contrat d’équipe :**
- Consomme : InvoiceTerms (conditions de paiement), MiyuInvoice (outils calcul)
- Expose : `invoice.create`, `invoice.update`, `invoice.validate`, `invoice.refund`

**Mandat de Permission requis :**
- Création / modification brouillon : Mandat avec InvoiceLedger
- Validation : Mandat avec StrongFather (décision)

### 3.2 InvoicePayment

**Rôle :** Enregistrement des paiements et réconciliation avec les factures.

**Capacités :**
- Enregistrement d’un paiement (client ou fournisseur)
- Réconciliation paiement ↔ facture(s) (partielle ou totale)
- Mise à jour des montants résiduels et du statut de paiement des factures

**Niveau de sécurité :** 2–3 (Sensitive à Critical selon données bancaires)

**Gouvernance :**
- **StrongFather** : Décision de réconciliation (optionnel selon seuils)
- **KindMother** : Persistance des paiements et réconciliations (WriteIntent)
- **Master Butler** : Permissions (Mandat Paiement)
- **WorrySentinel** : Niveau sécurité selon données paiement

**Contrat d’équipe :**
- Consomme : InvoiceLedger (factures, lignes créances/dettes)
- Expose : `payment.record`, `payment.reconcile`, `payment.query`

**Mandat de Permission requis :**
- Enregistrement paiement : Mandat avec InvoicePayment
- Réconciliation : Mandat avec InvoicePayment (+ StrongFather si politique de double validation)

### 3.3 InvoiceSend

**Rôle :** Envoi des factures (email, PDF, portail).

**Capacités :**
- Génération PDF facture
- Envoi par email (partenaire, pièce jointe PDF)
- Marquer comme envoyé (suivi)
- Exposition portail client (consultation facture, paiement en ligne si activé)

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données personnelles)

**Gouvernance :**
- **KindMother** : Lecture des factures (génération PDF)
- **StrongFather** : Décision d’envoi (si politique de validation envoi)
- **Master Butler** : Permissions (Mandat Envoi)
- **WorrySentinel** : Données personnelles (email, PDF)

**Contrat d’équipe :**
- Consomme : InvoiceLedger (lecture facture)
- Expose : `invoice.send`, `invoice.pdf`, `invoice.portal_url`

**Mandat de Permission requis :**
- Envoi : Mandat avec InvoiceSend

### 3.4 InvoiceTerms

**Rôle :** Conditions de paiement et échéanciers.

**Capacités :**
- Création / modification des conditions de paiement
- Calcul des échéances (dates, montants) pour une facture
- Fourniture des échéances aux factures (lignes payment_term)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des conditions de paiement (WriteIntent si configuration)
- **Master Butler** : Permissions (Mandat Configuration pour modification)
- **WorrySentinel** : Données de paiement

**Contrat d’équipe :**
- Consommé par : InvoiceLedger (calcul échéances)
- Expose : `terms.get`, `terms.compute_due_dates`, `terms.list`

### 3.5 InvoiceUI

**Rôle :** Interface utilisateur facturation (listes, formulaires, wizards).

**Capacités :**
- Affichage des listes de factures et paiements
- Formulaires de création / modification de factures
- Wizard paiement (montant, date, factures à régler)
- Actions : Valider, Envoyer, Enregistrer paiement, Créer avoir

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions (accès aux écrans)
- **WorrySentinel** : Niveau d’affichage selon données

**Contrat d’équipe :**
- Consomme : InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms
- Expose : Écrans et actions (pas d’API métier directe)

**Mandat de Permission requis :**
- Accès factures : Mandat Standard (InvoiceUI)
- Actions (Valider, Envoyer, Paiement) : Mandats respectifs (Validation, Envoi, Paiement)

---

## 4. Mandats de Permission

### 4.1 Mandat Standard (Facturation)

- **Opérateurs autorisés :** InvoiceLedger, InvoiceUI
- **Flux :** Création / modification facture brouillon, consultation
- **Validité :** Session ou durée définie
- **Révocation :** Fin de session, changement d’environnement, alerte WorrySentinel

### 4.2 Mandat Validation

- **Opérateurs autorisés :** InvoiceLedger, StrongFather
- **Flux :** Validation de facture (Post)
- **Validité :** Par action ou session
- **Révocation :** Idem Mandat Standard

### 4.3 Mandat Envoi

- **Opérateurs autorisés :** InvoiceSend, InvoiceUI
- **Flux :** Envoi email, génération PDF, marquer envoyé
- **Validité :** Session ou par envoi
- **Révocation :** Idem Mandat Standard

### 4.4 Mandat Paiement

- **Opérateurs autorisés :** InvoicePayment, InvoiceUI
- **Flux :** Enregistrement paiement, réconciliation
- **Validité :** Session ou par action
- **Révocation :** Idem Mandat Standard

### 4.5 Mandat Configuration

- **Opérateurs autorisés :** InvoiceTerms (modification), InvoiceLedger (configuration)
- **Flux :** Modification conditions de paiement, paramètres facturation
- **Validité :** Session ou durée définie
- **Révocation :** Idem Mandat Standard

---

## 5. Niveaux de sécurité (WorrySentinel)

| Donnée | Niveau | Justification |
|--------|--------|----------------|
| Facture (montants, partenaire) | 2 (Sensitive) | Données commerciales et personnelles |
| Paiement (montant, date, journal) | 2–3 (Sensitive à Critical) | Données de paiement / bancaires |
| Envoi (email, PDF) | 1–2 (Standard à Sensitive) | Données personnelles (email) |
| Conditions de paiement | 2 (Sensitive) | Règles métier |
| Interface (listes, formulaires) | 1 (Standard) | Affichage selon permissions |

---

## 6. Intégration avec les Cores

### 6.1 StrongFather

- Décision de validation de facture (Post)
- Décision d’envoi (si politique de validation envoi)
- Décision de réconciliation (si politique de double validation paiement)
- Émission et révocation des Mandats (Standard, Validation, Envoi, Paiement, Configuration)

### 6.2 KindMother

- Persistance des factures et lignes (WriteIntent)
- Persistance des paiements et réconciliations (WriteIntent)
- Persistance des conditions de paiement (WriteIntent si configuration)
- Lecture pour génération PDF et rapports

### 6.3 Master Butler

- Déclaration des capacités : `invoice.create`, `invoice.validate`, `payment.record`, `invoice.send`, etc.
- Vérification des Mandats avant chaque action
- Permissions par rôle (utilisateur facturation, responsable, etc.)

### 6.4 WorrySentinel

- Niveaux de sécurité (1–3) selon données (facture, paiement, email)
- États de confiance (T0–T4) : blocage des outils si environnement dégradé
- Audit des accès et des envois (données personnelles)

### 6.5 Ever Buddy

- Séquences de numéros de facture (par journal ou par type)
- Gestion du cycle de vie des numéros (réinitialisation mensuelle/annuelle si configuré)

---

## 7. Relation avec Accounting (Miyukini)

- **Invoicing** : Périmètre facturation (InvoiceService).
- **Accounting** : Grand livre, rapprochement bancaire, plan comptable, journaux (AccountService).
- **Partage :** Les factures validées peuvent être persistées comme écritures comptables (KindMother) ; InvoiceLedger peut s’appuyer sur les mêmes modèles que AccountLedger pour les écritures facture, ou déléguer à AccountLedger si le module Accounting est présent.
- **Recommandation :** MiyuInvoice + InvoiceService pour le périmètre Invoicing ; intégration optionnelle avec AccountService pour comptabilisation complète.

---

## 8. Conclusion

Les **spécifications Opérateurs Miyukini** pour Invoicing définissent une **Équipe InvoiceService** (InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI) avec **Contrat d’Équipe** et **Mandats de Permission** (Standard, Validation, Envoi, Paiement, Configuration), en réutilisant **MiyuInvoice** et les **Cores** (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy). L’intégration avec Accounting reste optionnelle pour la comptabilité complète.

**Prochaines étapes :** Voir [Guide Intégration COG](../05_integration_cog/Odoo%20Invoicing%20-%20Guide%20Integration%20COG.md) et [Guide Implémentation](../06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md).

---

**Document** : Odoo Invoicing — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
