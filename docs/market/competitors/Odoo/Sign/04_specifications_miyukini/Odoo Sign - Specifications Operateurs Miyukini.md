# Odoo Sign — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Sign** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Sign
- Contrats d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d’ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SignRequestOperator** | Gestion du cycle de vie des demandes de signature | Opérateur de Service |
| **SignTemplateOperator** | Gestion des modèles (PDF, champs, rôles, tags) | Opérateur de Service |
| **SignItemOperator** | Définition des types de champs et mapping partenaire | Opérateur de Service |
| **SignRoleOperator** | Gestion des rôles et authentification renforcée | Opérateur de Service |
| **SignComplianceOperator** | Audit, hash, horodatage, preuves d’intégrité (lecture) | Opérateur de Service |
| **SignUI** | Interface utilisateur (dashboard, envoi, signature) | Opérateur d’Interface |

### 1.2 Équipe d’Opérateurs : SignService

**Définition :**
> **SignService est une Équipe d’Opérateurs qui collabore sous règles explicites pour délivrer le service de signature électronique.**

**Composition :**
- SignRequestOperator (niveau sécurité 3)
- SignTemplateOperator (niveau sécurité 2–3)
- SignItemOperator (niveau sécurité 2)
- SignRoleOperator (niveau sécurité 2)
- SignComplianceOperator (niveau sécurité 3, lecture seule)
- SignUI (niveau sécurité 1–2)

---

## 2. Opérateurs Détaillés

### 2.1 SignRequestOperator

**Rôle :** Gestion des demandes de signature (création, envoi, annulation, expiration).

**Capacités :**
- Création / modification de demandes (one-time ou depuis template)
- Envoi des demandes (assignation signataires par rôle, ordre, validité, relances)
- Réception des signatures (enregistrement des champs remplis, passage au signataire suivant)
- Gestion des états : shared, sent, signed, refused, canceled, expired
- Annulation et révocation des liens
- Déclenchement des relances et de l’expiration (jobs planifiés)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision d’envoyer, d’annuler, de révoquer
- **KindMother** : Persistance des demandes et des données de signature (WriteIntent)
- **Master Butler** : Permissions d’envoi, de consultation, d’annulation
- **WorrySentinel** : Niveau sécurité, audit des accès par token, conformité
- **Ever Buddy** : Cycle de vie (expiration, archivage)

**Contrat d’équipe :**
- Consomme : SignTemplateOperator (template), MiyuContacts (signataires), MiyuNotify (emails, relances), MiyuDocuments (archivage)
- Expose : `sign_request.create`, `sign_request.send`, `sign_request.cancel`, `sign_request.get_signer_page`

**Mandat de Permission requis :**
- Création demande : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Envoi : Mandat avec MiyuNotify (envoi) + SignTemplateOperator (lecture template)
- Annulation : Mandat avec StrongFather (décision) + KindMother (WriteIntent)
- Accès page signataire : Mandat limité (token valide, pas de décision métier)

### 2.2 SignTemplateOperator

**Rôle :** Gestion des modèles de documents (PDF, champs, rôles, tags, workspace, authorized users).

**Capacités :**
- Création / modification de templates (PDF, champs, attribution aux rôles)
- Propriétés template : tags, Signed Document Workspace, Signed Document Tags, Redirect Link, Authorized Users
- Création de template depuis un document existant (Template + Restore)
- Restriction d’usage par utilisateurs autorisés
- Liste des templates visibles sur le dashboard

**Niveau de sécurité :** 2–3 (Sensitive à Critical selon données)

**Gouvernance :**
- **StrongFather** : Décision de créer/modifier template, d’autoriser des utilisateurs
- **KindMother** : Persistance des templates et des items (WriteIntent)
- **Master Butler** : Permissions de création/modification ; vérification Authorized Users
- **WorrySentinel** : Niveau sécurité sur les documents sensibles

**Contrat d’équipe :**
- Consommé par : SignRequestOperator
- Consomme : SignItemOperator (types de champs), SignRoleOperator (rôles), MiyuDocuments (workspace)
- Expose : `template.create`, `template.update`, `template.get_for_send`, `template.duplicate_from_request`

**Mandat de Permission requis :**
- Création template : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification template : Mandat avec KindMother (WriteIntent) + Master Butler (authorized users)
- Utilisation pour envoi : Mandat avec Master Butler (vérification authorized users)

### 2.3 SignItemOperator

**Rôle :** Définition des types de champs (signature, initial, texte, checkbox, sélection) et mapping partenaire pour l’auto-complétion.

**Capacités :**
- CRUD des types de champs (field types) : nom, type, auto_fill_partner_field, dimensions par défaut, tip, placeholder
- Mapping des champs Sign vers les champs technique du partenaire (res.partner)
- Fourniture de la liste des champs disponibles pour l’éditeur (glisser-déposer)
- Calcul des valeurs suggérées pour un signataire (partenaire) lors de la signature

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de créer/modifier un type de champ
- **KindMother** : Persistance des définitions (WriteIntent)
- **Master Butler** : Permissions de configuration (souvent réservé admin)

**Contrat d’équipe :**
- Consommé par : SignTemplateOperator, SignRequestOperator (pour affichage et auto-fill)
- Consomme : MiyuContacts (modèle partenaire pour noms de champs)
- Expose : `item_type.list`, `item_type.get_autofill_value(partner_id, field_name)`

**Mandat de Permission requis :**
- Modification types de champs : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Lecture et auto-fill : Mandat avec SignRequestOperator ou SignTemplateOperator

### 2.4 SignRoleOperator

**Rôle :** Gestion des rôles de signataires et des options d’authentification renforcée (SMS, itsme®, Aadhaar).

**Capacités :**
- CRUD des rôles (nom, couleur, change_authorized)
- Configuration Extra Authentication Step par rôle : aucune, SMS, itsme®, Aadhaar eSign
- Vérification des crédits / quotas (SMS, Aadhaar) avant envoi
- Fourniture de la liste des rôles pour l’éditeur et le wizard d’envoi

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de créer/modifier un rôle, d’activer une auth renforcée
- **KindMother** : Persistance des rôles (WriteIntent)
- **Master Butler** : Permissions de configuration
- **WorrySentinel** : Conformité (coût SMS, pays supportés itsme®/Aadhaar)

**Contrat d’équipe :**
- Consommé par : SignTemplateOperator, SignRequestOperator
- Consomme : Prestataires SMS / itsme® / Aadhaar (abstraction fournisseurs)
- Expose : `role.list`, `role.get_authentication_config`, `role.check_credits`

**Mandat de Permission requis :**
- Modification rôles : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Lecture : Mandat avec SignTemplateOperator ou SignRequestOperator

### 2.5 SignComplianceOperator

**Rôle :** Audit, hash signataire, horodatage, preuves d’intégrité (lecture seule, pas de modification des documents signés).

**Capacités :**
- Génération du hash à chaque signature (empreinte unique de l’opération)
- Horodatage et enregistrement des métadonnées (IP, date, type d’auth) pour preuve
- Détection des altérations du document après signature (vérification hash)
- Export « preuve d’intégrité » pour audit (document + métadonnées signées)
- Pas de modification du contenu signé ; lecture et attestation uniquement

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **KindMother** : Lecture des demandes et des documents signés ; écriture uniquement des logs de preuve (WriteIntent limité)
- **WorrySentinel** : Niveau sécurité maximal, audit des accès aux preuves
- **TAMR** : Point d’intervention humaine pour litiges (consultation preuves)

**Contrat d’équipe :**
- Consommé par : SignRequestOperator (à la signature), initiateur (export preuve)
- Consomme : SignRequestOperator (lecture demande, document signé)
- Expose : `compliance.generate_hash`, `compliance.record_signature_evidence`, `compliance.verify_integrity`, `compliance.export_proof`

**Mandat de Permission requis :**
- Enregistrement preuve : Mandat avec KindMother (WriteIntent limité aux logs) + SignRequestOperator
- Export preuve : Mandat avec Master Butler (permission audit) + WorrySentinel (niveau sécurité)

### 2.6 SignUI

**Rôle :** Interface utilisateur (dashboard, éditeur, envoi, page signataire, configuration).

**Capacités :**
- Dashboard : upload one-time, upload template, liste templates, accès Documents
- Éditeur : affichage PDF, glisser-déposer champs, attribution rôles, options (validité, relances, ordre)
- Wizard envoi : sélection signataires par rôle, ordre, options, message
- Page signataire : affichage PDF, champs à remplir, tips, signature (dessin/génération/upload), validation / refus, authentification renforcée
- Configuration : rôles, types de champs, tags, paramètres (crédits, Frame)
- Liste documents : statut, progression, filtres, actions (Details, Template, etc.)

**Niveau de sécurité :** 1–2 (Standard à Sensitive)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs
- **Master Butler** : Vérification des permissions d’affichage (templates autorisés, documents visibles)
- **WorrySentinel** : Niveau sécurité de la page signataire (données sensibles)

**Contrat d’équipe :**
- Consomme : SignRequestOperator, SignTemplateOperator, SignItemOperator, SignRoleOperator, SignComplianceOperator (lecture), MiyuContacts, MiyuNotify, MiyuDocuments
- Expose : Vues et formulaires ; pas d’API métier directe (tout passe par les Opérateurs)

**Mandat de Permission requis :**
- Dashboard / liste : Mandat avec SignRequestOperator + SignTemplateOperator (lecture)
- Envoi : Mandat avec SignRequestOperator (send) + MiyuNotify
- Page signataire : Mandat limité (token valide, pas d’élévation de privilège)
- Configuration : Mandat avec SignRoleOperator, SignItemOperator (modification)

---

## 3. Contrats d’Équipe et Mandats

### 3.1 Contrat d’équipe SignService

**Membres :** SignRequestOperator, SignTemplateOperator, SignItemOperator, SignRoleOperator, SignComplianceOperator, SignUI.

**Flux autorisés :**
- SignUI → SignRequestOperator (création, envoi, annulation)
- SignUI → SignTemplateOperator (création, modification, liste)
- SignUI → SignItemOperator, SignRoleOperator (configuration)
- SignUI → SignComplianceOperator (export preuve)
- SignRequestOperator → SignTemplateOperator (lecture template)
- SignRequestOperator → MiyuNotify (envoi, relances)
- SignRequestOperator → MiyuContacts (signataires, auto-fill)
- SignRequestOperator → MiyuDocuments (archivage document signé)
- SignRequestOperator → SignComplianceOperator (génération hash, enregistrement preuve)
- SignTemplateOperator → SignItemOperator, SignRoleOperator (lecture rôles et types de champs)

**Niveau de sécurité maximum de l’équipe :** 3 (Critical).

**Règles :**
- Toute création/modification de demande ou template passe par StrongFather (décision) et KindMother (WriteIntent).
- La page signataire n’expose pas d’élévation de privilège ; accès par token uniquement.
- Export des preuves réservé aux utilisateurs ayant la permission audit.

### 3.2 Mandats typiques

| Action | Mandat |
|--------|--------|
| Envoyer un document | KindMother (WriteIntent demande) + StrongFather (décision envoi) + MiyuNotify (envoi emails) |
| Créer un template | KindMother (WriteIntent template) + StrongFather (décision) |
| Modifier un rôle (auth renforcée) | KindMother (WriteIntent rôle) + StrongFather (décision) |
| Annuler une demande | StrongFather (décision) + KindMother (WriteIntent état) |
| Exporter preuve d’intégrité | Master Butler (permission audit) + SignComplianceOperator (lecture) + WorrySentinel (niveau sécurité) |

---

## 4. Intégration avec les Cores

- **StrongFather** : Décisions d’envoi, d’annulation, de création/modification template et rôles.
- **KindMother** : Persistance des demandes, templates, items, rôles, et des preuves (WriteIntent).
- **Master Butler** : Permissions d’envoi, de consultation, d’annulation, authorized users, export preuves.
- **WorrySentinel** : Niveau sécurité 2–3, audit des accès par token, conformité juridique documentée.
- **Ever Buddy** : Expiration, archivage, dépréciation de types de champs ou rôles.
- **TAMR** : Intervention humaine pour litiges (consultation des preuves, support).
- **BondingBrother** : Médiation entre SignUI et les Opérateurs (pas d’autorité, traduction des intentions).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
