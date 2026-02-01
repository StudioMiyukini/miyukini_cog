# Odoo Sign — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Sign** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 — Sign (Productivity)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps (Documents, Contacts, Mail, Sales)
- Mécanismes d'intégration (modèles partagés, wizards, portail)
- APIs et hooks typiques
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis (déduits)

**Dépendances explicites typiques :**
- `base` : Modèles de base
- `mail` : Notifications, envoi d’emails (liens de signature, relances)
- `portal` : Accès signataire sans compte complet (lien unique)
- `web` : Interface web
- `attachment_index` ou `document` : Stockage des PDF (templates, documents signés)
- `auth_signup` ou équivalent : Liens sécurisés et tokens (optionnel selon implémentation)

**Dépendances optionnelles (intégrations si installés) :**
- `documents` : Workspace et tags pour les documents signés
- `crm` : Envoi de documents à signer depuis une opportunité
- `sale` : Signature de devis / contrats (Sign complète le flux avec documents multi-signataires)
- `contacts` / `base` : res.partner pour signataires et auto-complétion des champs

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Documents (documents)

**Flux :**
```
Sign (template / request) → PDF signé → Documents (workspace + tags)
```

**Mécanismes :**
- **Signed Document Workspace** : espace Documents dans lequel les PDF signés sont déposés automatiquement.
- **Signed Document Tags** : tags appliqués aux documents signés lors de l’archivage.
- Configuration au niveau du template (Template Properties).
- Les documents signés sont visibles dans Documents ‣ All Documents (ou équivalent) et dans le workspace choisi.

**Champs / paramètres liés :**
- `sign.template.signed_document_workspace` (ou équivalent) : workspace cible
- `sign.template.signed_document_tag_ids` : tags appliqués au document signé

**Recommandations pour Miyukini :**
- Intégration native avec le module Documents (ou équivalent Miyukini) : workspace et tags configurables par template.
- Archivage automatique à l’état « signed » : génération du PDF final (avec signatures et hash) → envoi vers le workspace avec tags.
- Recherche et filtrage des documents signés dans le même arbre Documents.

### 2.2 Intégration avec Contacts (res.partner)

**Flux :**
```
res.partner → Signataires (par rôle) + Auto-complétion des champs
```

**Mécanismes :**
- **Signataires** : chaque rôle est assigné à un `res.partner` (contact). Sélection via recherche partenaire dans le wizard d’envoi.
- **Auto-complétion** : les types de champs Sign peuvent être mappés sur un champ technique de `res.partner` (name, email, phone, website, etc.). Lors de la signature, la valeur est suggérée et modifiable par le signataire.
- **Change Authorized** (rôle) : si activé, le document peut être réassigné à un autre contact pour ce rôle.

**Champs liés :**
- Signataires : lien request/template → `res.partner` (plusieurs par rôle selon modèle ; en pratique un signataire par rôle avant envoi).
- `sign.item.auto_fill_partner_field` (ou équivalent) : nom technique du champ `res.partner` pour l’auto-remplissage.

**Recommandations pour Miyukini :**
- Utiliser le même modèle Contact/Partenaire (MiyuContacts ou équivalent) pour la sélection des signataires.
- Types de champs configurables avec « Auto-fill Partner Field » = clé du champ partenaire.
- Réassignation possible si « Change Authorized » activé sur le rôle.

### 2.3 Intégration avec Mail (Notifications)

**Flux :**
```
Sign Request → Email (lien unique) → Signataire
Sign Request → Relances (reminders) → Email
Sign Request (signed/refused) → Notification → Initiateur
```

**Mécanismes :**
- Envoi des demandes de signature par email : lien sécurisé unique par signataire (token, pas de compte requis).
- Relances automatiques : envoi d’emails selon `reminder_days` tant que le document n’est pas signé ou refusé, jusqu’à `valid_until`.
- Notifications à l’initiateur : document signé, refusé ou expiré (selon configuration mail).
- Message de confirmation au signataire après signature (avec redirect link si configuré).

**Templates mail typiques (déduits) :**
- Invitation à signer (sujet + corps + lien)
- Relance (rappel signature)
- Confirmation de signature (avec redirect link)
- Notification initiateur (signed / refused / expired)

**Recommandations pour Miyukini :**
- Intégration avec MiyuNotify (ou équivalent) : envoi des invitations, relances, confirmations et notifications initiateur.
- Templates de mail paramétrables (sujet, corps, variables : document_name, signer_name, link, redirect_url).
- Lien unique et révocable (token ou équivalent).

### 2.4 Intégration avec Sales (sale)

**Flux possibles :**
```
Sale Order / Quotation → Document à signer (contrat, avenant) → Sign Request → Signataires
```

**Mécanismes :**
- Odoo Sales permet déjà une signature simple sur devis (champ signature sur le bon de commande).
- L’app Sign étend le périmètre : documents PDF arbitraires, multi-signataires, multi-champs, ordre de signature, authentification renforcée.
- Création possible d’une demande Sign depuis une vente : sélection d’un template, pré-remissage des signataires (client, commercial, etc.) depuis la commande/contact.

**Champs / liens possibles :**
- `sign.request` peut avoir un champ optionnel `sale_order_id` (ou équivalent) pour tracer l’origine vente.
- Partenaires de la commande utilisés comme signataires par défaut.

**Recommandations pour Miyukini :**
- Lien optionnel Demande de signature ↔ Commande / Devis (MiyuStore ou équivalent) pour traçabilité.
- Action contextuelle « Envoyer un document à signer » depuis la commande : choix du template, signataires pré-remplis depuis la commande/contact.

### 2.5 Intégration avec CRM (crm)

**Flux possibles :**
```
Opportunity / Lead → Document à signer (contrat, NDA) → Sign Request → Signataires
```

**Mécanismes :**
- Envoi d’un document à signer depuis une opportunité : choix du template, signataires (contact lead/opportunity, responsable, etc.).
- Lien optionnel `sign.request` ↔ `crm.lead` / `crm.opportunity` pour suivi (document signé = étape du pipeline).

**Recommandations pour Miyukini :**
- Lien optionnel Demande de signature ↔ Opportunité / Lead pour traçabilité et reporting.
- Action « Envoyer à signer » depuis la fiche opportunité avec signataires suggérés.

---

## 3. Mécanismes Techniques Typiques

### 3.1 Liens sécurisés signataire

- Chaque signataire reçoit un lien unique (token dans l’URL ou paramètre).
- Le token est associé à la demande + rôle/signataire ; pas besoin de compte Odoo pour ouvrir le lien.
- Validation du token côté serveur avant d’afficher le PDF et les champs.
- Révocation possible (annulation de la demande → liens invalides).

**Recommandations pour Miyukini :**
- Génération de token sécurisé (aléatoire, durée de vie optionnelle).
- Route dédiée (ex. `/sign/request/<token>`) pour la page signataire, sans authentification COG obligatoire (ou avec auth minimale portail).
- WorrySentinel : niveau de sécurité adapté (données sensibles, audit des accès par token).

### 3.2 Stockage des PDF

- **Templates** : fichier PDF stocké (ir.attachment ou documents). Référence depuis `sign.template`.
- **Documents signés** : PDF généré (original + signatures + hash) stocké dans le workspace Documents ou équivalent.
- **Audit** : conservation des preuves (hash, horodatage, logs d’accès) sans modifier le PDF signé.

**Recommandations pour Miyukini :**
- Stockage des templates et des PDF signés via le module Documents / Stockage (KindMother, WriteIntent pour métadonnées).
- Génération du PDF signé côté serveur (fusion champs + signatures + cadre hash) puis archivage.
- Pas de modification du PDF après signature ; lecture seule + export « preuve » si besoin.

### 3.3 Authentification renforcée (SMS, itsme®, Aadhaar)

- **SMS** : envoi d’un code à 6 chiffres au numéro du signataire (API SMS Odoo / IAP). Saisie du code sur la page « Final verification ».
- **itsme®** : redirection vers le fournisseur itsme® (Belgique, Pays-Bas) ; retour avec identité vérifiée.
- **Aadhaar eSign** : intégration avec prestataire eMudhra (Inde) ; OTP Aadhaar puis certificat numérique dans le PDF téléchargé.

**Recommandations pour Miyukini :**
- Abstraction « fournisseur d’authentification » (SMS, itsme®, Aadhaar) avec interfaces par fournisseur.
- Crédits / quotas gérés côté admin (SMS, Aadhaar) avec alertes.
- Conformité juridique documentée (à usage informatif) ; pas de conseil juridique.

---

## 4. APIs et Hooks (déduits)

### 4.1 Méthodes typiques

- **sign.request** :
  - `send_request()` ou équivalent : envoi des emails et passage en état `sent`.
  - `action_signed()` : passage en `signed`, génération du PDF final, archivage.
  - `action_refused()` : passage en `refused`, notification initiateur.
  - `action_cancel()` : annulation, invalidation des liens.
- **sign.template** :
  - Création depuis attachment (PDF).
  - Duplication depuis un document existant (action « Template » + « Restore »).
- **Cron / scheduled** :
  - Relances (reminders) : envoi d’emails selon `reminder_days` et `valid_until`.
  - Expiration : passage en `expired` si `valid_until` dépassée.

**Recommandations pour Miyukini :**
- Contrats d’API clairs pour : création demande, envoi, signature (réception des champs signés), refus, annulation, expiration.
- Jobs planifiés pour relances et expiration.
- Événements émis (signed, refused, expired) pour notifier d’autres modules (Sales, CRM, Documents).

### 4.2 Événements partagés

- **Document signé** : événement « sign.request.signed » (ou équivalent) pour déclencher archivage, mise à jour commande/opportunité, notifications.
- **Document refusé** : événement « sign.request.refused » pour notification et suivi.
- **Document expiré** : événement « sign.request.expired » pour statistiques et nettoyage.

**Recommandations pour Miyukini :**
- Publier des événements COG (ou équivalent) pour signed / refused / expired afin que Documents, Sales, CRM puissent réagir sans couplage direct.

---

## 5. Synthèse des Recommandations pour Miyukini

| Intégration   | Rôle | Recommandation |
|---------------|------|----------------|
| **Documents** | Archivage | Workspace + tags par template ; archivage auto à l’état signed. |
| **Contacts**  | Signataires + auto-fill | Partenaires comme signataires ; champs Sign mappés sur champs partenaire. |
| **Mail**      | Envoi et relances | MiyuNotify ; templates invitation, relance, confirmation, notification initiateur. |
| **Sales**     | Optionnel | Lien demande ↔ commande ; action « Envoyer à signer » depuis commande. |
| **CRM**       | Optionnel | Lien demande ↔ opportunité ; action « Envoyer à signer » depuis opportunité. |
| **Sécurité**  | Tokens, audit | Liens uniques révocables ; WorrySentinel ; audit des accès et signatures. |
| **Authentification** | SMS, itsme®, Aadhaar | Interfaces par fournisseur ; gestion crédits et conformité documentée. |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
