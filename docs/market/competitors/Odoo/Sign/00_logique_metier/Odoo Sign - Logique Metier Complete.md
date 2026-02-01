# Odoo Sign — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Sign** (Signature électronique) d'Odoo (version 19.0), à partir de la documentation officielle et des modèles typiques du module. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 — Sign (Productivity)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (sign.request, sign.template, sign.item, sign.send.request, rôles)
- Règles métier et contraintes (validité juridique, ordre de signature)
- Workflows et transitions d'état (demande → en cours → signé / refusé)
- Types de champs et auto-complétion depuis res.partner
- Authentification renforcée (SMS, itsme®, Aadhaar eSign)
- Intégration avec Documents, CRM, Sales, Contacts

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `sign.request` (Demande de signature)

**Rôle :** Représente une **demande de signature** envoyée à un ou plusieurs signataires. Gère le cycle de vie de la signature (en attente, en cours, signé, refusé).

**Champs clés (déduits) :**

#### Identification
- `name` / `reference` : Référence ou sujet du document
- `template_id` : Many2one vers `sign.template` (modèle utilisé)
- `state` : Selection (état : shared, sent, signed, refused, canceled)
- `request_item_ids` : One2many vers éléments de la demande (signataires, champs)

#### Signataires et ordre
- Ordre de signature configurable (1, 2, 3…)
- Un signataire par rôle (au moins un signataire par rôle du template)
- Lien vers `res.partner` pour chaque signataire

#### Options
- `valid_until` : Date de validité (optionnel)
- `reminder` : Boolean (relances automatiques)
- `reminder_days` : Nombre de jours entre les relances
- `redirect_url` : URL de redirection après signature (templates)

#### Audit et conformité
- Hash signataire (empreinte unique par signature)
- Horodatage, IP, géolocalisation (logs d'accès)
- Preuves cryptographiques d'intégrité du document

**Workflow d'états :**
```
shared (créé) → sent (envoyé) → [signataires signent] → signed (complété)
                              → refused (refusé par un signataire)
                              → canceled (annulé)
```

---

### 1.2 Modèle `sign.template` (Modèle de document)

**Rôle :** Représente un **modèle PDF** avec champs de signature et métadonnées. Réutilisable pour envoyer le même document plusieurs fois.

**Champs clés (déduits) :**

#### Identification
- `name` : Nom du modèle
- `attachment_id` : Many2one vers `ir.attachment` (fichier PDF)
- `active` : Boolean (actif)

#### Champs et rôles
- `item_ids` : One2many vers `sign.item` (champs à remplir : signature, initiales, texte, case à cocher, sélection)
- Rôles définis (un rôle par type de signataire : Customer, Manager, etc.)
- Chaque champ est assigné à un rôle

#### Configuration template
- `tag_ids` : Many2many — Tags pour catégoriser
- `signed_document_workspace` : Workspace Documents pour archivage (optionnel)
- `signed_document_tag_ids` : Tags appliqués au document signé
- `redirect_link` : Lien affiché dans le message de confirmation après signature
- `authorized_user_ids` : Many2many vers `res.users` — Utilisateurs autorisés à utiliser le template (restriction optionnelle)

**Règles métier :**
- Au moins un signataire doit être spécifié pour chaque rôle du template avant envoi
- Création de template possible depuis un document déjà envoyé (action « Template » depuis document)

---

### 1.3 Modèle `sign.item` (Champ / type de champ)

**Rôle :** Représente un **type de champ** (signature, initiales, texte, case à cocher, sélection) ou une instance de champ sur un document. Lié à un rôle (`responsible_id` = rôle signataire).

**Champs clés (déduits) :**

#### Identification
- `template_id` : Many2one vers `sign.template` (si champ de template)
- `name` : Nom du champ (ex. « Signature », « Initiales »)
- `type` : Selection (signature, initial, text, multiline_text, checkbox, selection)
- `responsible_id` : Many2one vers rôle — Rôle qui doit remplir ce champ

#### Position et taille
- `page` : Numéro de page (1-based)
- `posX`, `posY` : Position (ratio 0–1)
- `width`, `height` : Largeur/hauteur (ratio 0–1, ex. 0.15 = 15 % de la page)

#### Auto-complétion
- `auto_fill_partner_field` : Char — Nom technique du champ `res.partner` pour auto-remplissage (ex. `name`, `email`, `website`)
- Valeurs suggérées modifiables par le signataire

#### UX
- `placeholder` : Texte affiché dans le champ avant saisie
- `tip` : Conseil affiché à gauche pendant la signature (ex. « Signez ici »)

**Types de champs :**
- **Signature** : Dessin, génération automatique depuis le nom, ou upload image
- **Initial** : Initiales (même principe que signature)
- **Text** : Une ligne
- **Multiline Text** : Plusieurs lignes
- **Checkbox** : Case à cocher (approbation, consentement)
- **Selection** : Liste d’options (une seule choisie)

---

### 1.4 Rôles (`sign.role` ou équivalent)

**Rôle :** Définit un **rôle** de signataire (ex. Customer, Manager, Witness). Chaque champ du document est assigné à un rôle.

**Champs clés (déduits) :**
- `name` : Nom du rôle (ex. « Customer », « Manager »)
- `extra_authentication` : Selection (aucune, SMS, itsme®, Aadhaar eSign)
- `change_authorized` : Boolean — Le document peut être réassigné à un autre contact pour ce rôle
- `color` : Couleur pour identifier les champs du rôle dans l’éditeur

**Authentification renforcée :**
- **SMS** : Code unique envoyé par SMS (6 chiffres) — nécessite crédits IAP
- **itsme®** : Identification forte (Belgique, Pays-Bas)
- **Aadhaar eSign** : Signature numérique Inde (eMudhra)
- Dès qu’un rôle a une étape d’authentification, elle s’applique à tout champ de ce rôle

---

### 1.5 Modèle `sign.send.request` (Envoi)

**Rôle :** Wizard ou modèle pour **préparer et envoyer** une demande de signature (sélection template, signataires, options).

**Champs clés (déduits) :**
- `template_id` : Modèle à envoyer
- `signer_ids` : One2many vers signataires (partner_id, role_id, ordre)
- `valid_until` : Date limite de validité
- `reminder` : Boolean
- `reminder_days` : Intervalle des relances
- `subject` : Sujet du message
- `message` : Corps du message (optionnel)

**Règles métier :**
- Un signataire par rôle du template obligatoire
- Ordre de signature : chaque signataire reçoit la demande seulement après que le précédent a signé (si « Specify Signing Order » activé)

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow Demande de signature

**États typiques :**
- `shared` / `draft` : Brouillon, pas encore envoyé
- `sent` : Envoyé, en attente de signatures
- `signed` : Tous les signataires ont signé — document complété
- `refused` : Un signataire a refusé
- `canceled` : Annulé par l’initiateur
- `expired` : Dépassement de la date de validité (si `valid_until`)

**Transitions :**
```
shared → sent (clic Send, tous les rôles assignés)
sent → signed (dernier signataire valide et envoie)
sent → refused (un signataire refuse)
sent → expired (valid_until dépassée)
shared / sent → canceled (annulation par l’owner)
```

### 2.2 Ordre de signature

- Si **Sign order** activé : les signataires reçoivent la demande **dans l’ordre** (1 puis 2 puis 3…).
- Chaque destinataire reçoit la notification seulement quand le précédent a complété son action (signé ou refusé).
- Si désactivé : tous reçoivent en parallèle (ordre libre).

### 2.3 Validité et relances

- **Valid Until** : date au-delà de laquelle la demande n’est plus valide (optionnel).
- **Reminders** : relances automatiques par email (toggle + nombre de jours entre deux relances).

---

## 3. Règles Métier et Contraintes

### 3.1 Validité juridique des signatures

**UE (eIDAS 910/2014) :**
- Odoo produit des **signatures électroniques simples**.
- Légalement valides ; des preuves complémentaires peuvent être demandées.
- Preuves collectées : email/SMS, itsme®, horodatage, IP, géolocalisation, intégrité cryptographique du document.

**États-Unis (ESIGN, UETA) :**
- Cinq critères : intention de signer, consentement à l’électronique, attribution claire (métadonnées), association signature–document, conservation par toutes les parties.
- Odoo couvre ces critères (métadonnées, hash, copie téléchargeable pour le signataire).

**Autres pays :**
- Documentation par pays disponible (Algérie, Brésil, Canada, etc.) — à consulter selon la juridiction.

### 3.2 Hash signataire et intégrité

- À chaque signature, un **hash** (empreinte unique) est généré.
- Toute modification du document après signature est **détectable** (preuves cryptographiques).
- Cadre visuel optionnel « Frame » affichant le début du hash à côté de la signature.

### 3.3 Contraintes d’envoi

- **Un signataire par rôle** : pour chaque rôle du template, exactement un partenaire (ou utilisateur) doit être renseigné avant envoi.
- **Champs obligatoires** : tous les champs requis du template doivent être présents sur le PDF.
- **Fichier** : le document source doit être un PDF valide.

### 3.4 Templates

- **Création** : « Upload a PDF template » → ajout des champs → configuration (tags, workspace, redirect, authorized users).
- **Création depuis document** : Documents → All Documents → document existant → ⋮ → Template → Restore → le document apparaît comme template sur le dashboard.
- **Restriction** : « Authorized Users » limite l’usage du template à certains utilisateurs ou groupes.

---

## 4. Intégrations avec Autres Modules

### 4.1 Documents (documents)

- **Signed Document Workspace** : espace de stockage pour les PDF signés.
- **Signed Document Tags** : tags appliqués automatiquement aux documents signés.
- Archivage et recherche des documents signés dans Documents.

### 4.2 Contacts (res.partner)

- **Auto-complétion** : champs Sign mappés sur des champs technique `res.partner` (name, email, phone, etc.).
- **Signataires** : sélection des contacts comme signataires (par rôle).
- **Réassignation** : si « Change Authorized » activé sur le rôle, le signataire peut être changé pour ce rôle.

### 4.3 Mail / Notifications

- Envoi des demandes de signature par email (liens sécurisés uniques par signataire).
- Relances automatiques (reminders).
- Message de confirmation après signature (avec redirect link si configuré).

### 4.4 Sales (optionnel)

- Signature de devis / contrats depuis le portail (signature simple sur devis).
- Sign app permet des documents multi-signataires et multi-champs (contrats, avenants).

### 4.5 CRM / Autres

- Envoi de documents à signer depuis une opportunité ou un contact (workflow métier).
- Lien possible entre sign.request et sale.order / crm.lead (champs relationnels selon implémentation).

---

## 5. Considérations pour Miyukini COG

### 5.1 Architecture Opérateurs proposée

1. **SignRequestOperator** : gestion du cycle de vie des demandes (création, envoi, annulation, expiration).
2. **SignTemplateOperator** : gestion des modèles (PDF, champs, rôles, tags, workspace, authorized users).
3. **SignItemOperator** : définition des types de champs (signature, texte, checkbox, etc.) et mapping partenaire.
4. **SignRoleOperator** : gestion des rôles et options d’authentification (SMS, itsme®, Aadhaar).
5. **SignComplianceOperator** : audit, hash, horodatage, preuves d’intégrité (lecture seule, pas de modification).
6. **SignUI** : interface utilisateur (dashboard, envoi, signature côté signataire).

### 5.2 Gouvernance COG

- **StrongFather** : autorisation d’envoyer une demande, de créer/modifier un template, d’annuler une demande.
- **KindMother** : persistance des demandes, templates, items, rôles, et du document signé (WriteIntent pour créations/modifications).
- **Master Butler** : qui peut envoyer, qui peut utiliser quel template (authorized users), qui peut voir les documents signés.
- **WorrySentinel** : niveau de sécurité élevé (données sensibles, conformité juridique) ; audit des accès et des signatures.
- **Ever Buddy** : gestion des états de vie (expiration, archivage, dépréciation de types de champs ou rôles).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
