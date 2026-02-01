# Odoo Sign — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Sign** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 — Sign

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (signature unique, template, multi-signataires)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Initiateur (Document Owner)

**Profil :**
- Rôle : Envoie les documents à faire signer
- Responsabilités :
  - Uploader un PDF ou choisir un template
  - Placer les champs (signature, initiales, texte, etc.) et les associer aux rôles
  - Définir les signataires (un par rôle)
  - Configurer validité, relances, ordre de signature
  - Envoyer la demande et suivre le statut
  - Consulter les documents signés et les archiver

**Besoins :**
- Dashboard clair (documents récents, en attente, signés)
- Création rapide (one-time ou template)
- Suivi du statut par signataire
- Relances automatiques et date de validité
- Archivage dans Documents (workspace, tags)

**Permissions :**
- Accès à l’app Sign
- Droits sur les templates (éventuellement restreints par « Authorized Users »)
- Accès aux documents signés (selon règles entreprise)

### 1.2 Signataire (Signer)

**Profil :**
- Rôle : Reçoit le lien de signature et remplit les champs qui lui sont assignés
- Responsabilités :
  - Ouvrir le lien sécurisé (email)
  - S’identifier si nécessaire (SMS, itsme®, Aadhaar)
  - Remplir les champs de son rôle (signature, initiales, texte, cases à cocher, sélection)
  - Valider et envoyer sa partie (ou refuser)
  - Télécharger une copie du document signé (si proposé)

**Besoins :**
- Interface simple et guidée (tips, placeholders)
- Lien unique et sécurisé
- Possibilité de signer sur mobile
- Clarté sur ce qui est demandé (ordre de signature si applicable)
- Preuve de ce qui a été signé (copie, email de confirmation)

**Permissions :**
- Aucun accès Odoo requis pour signer (lien portail/public)
- Optionnel : compte portail si document lié à un partenaire existant

### 1.3 Administrateur Sign (Configuration)

**Profil :**
- Rôle : Configure rôles, types de champs, paramètres globaux
- Responsabilités :
  - Créer/modifier les rôles (Sign ‣ Configuration ‣ Roles)
  - Définir l’authentification renforcée (SMS, itsme®, Aadhaar) par rôle
  - Créer/modifier les types de champs (Sign ‣ Configuration ‣ Settings ‣ Edit field types)
  - Gérer les tags (Configuration ‣ Tags)
  - Paramétrer les crédits SMS, Aadhaar, etc.

**Besoins :**
- Interface de configuration centralisée
- Gestion des coûts (crédits IAP pour SMS / Aadhaar)
- Documentation sur la conformité juridique par pays

**Permissions :**
- Droits de configuration Sign (groupe manager/settings)

---

## 2. Parcours d'Onboarding

### 2.1 Initiateur — Première demande (signature unique)

**Scénario :**
1. **Accès :**
   - Menu Sign → Dashboard
   - Clic « Upload a PDF to sign » (one-time)

2. **Préparation du document :**
   - Sélection du fichier PDF → ouverture dans l’éditeur
   - Glisser-déposer les champs depuis la colonne gauche (Signature, Initial, Text, etc.)
   - Pour chaque champ : clic sur le champ → choix du rôle assigné
   - Ajuster ordre des rôles / signataires si besoin

3. **Envoi :**
   - Clic « Send »
   - Renseigner les champs requis (signataires, message, options)
   - Optionnel : Valid Until, Reminders (toggle + nombre de jours)
   - Envoyer → la demande apparaît dans « Documents ‣ All Documents » avec statut des signatures

**Points d’aide :**
- Champs prédéfinis (Signature, Initial, Text, etc.) avec tips et placeholders
- Message d’erreur si un rôle n’a pas de signataire assigné

### 2.2 Initiateur — Premier template

**Scénario :**
1. **Création :**
   - Dashboard → « Upload a PDF template »
   - Sélection du PDF → ajout des champs et attribution aux rôles
   - Clic « Template Properties » : Tags, Signed Document Workspace, Signed Document Tags, Redirect Link, Authorized Users
   - Sauvegarde du template

2. **Utilisation :**
   - Template visible sur le dashboard
   - « Send » → choix des signataires par rôle, options (validité, relances, ordre de signature)
   - « Sign Now » si l’utilisateur veut signer lui-même immédiatement

**Points d’aide :**
- Possibilité de créer un template à partir d’un document déjà envoyé (⋮ → Template → Restore)

### 2.3 Signataire — Première signature

**Scénario :**
1. **Réception :**
   - Email avec lien unique vers le document à signer

2. **Ouverture :**
   - Clic sur le lien → ouverture (portail/public)
   - Si authentification renforcée : étape SMS / itsme® / Aadhaar selon configuration

3. **Signature :**
   - Affichage du PDF avec les champs à remplir (tips en flèche à gauche)
   - Remplissage des champs (signature : dessin, génération automatique ou image)
   - Clic « Validate & Send Completed Document »
   - Si dernière signature : document complété ; sinon : passage au signataire suivant (ordre de signature)
   - Message de confirmation + lien de redirection si configuré
   - Téléchargement de la copie signée (si proposé)

**Points de friction possibles :**
- Lien expiré ou déjà utilisé
- Authentification SMS/itsme non disponible ou crédits épuisés
- Confusion sur l’ordre de signature (qui signe après qui)

---

## 3. Scénarios d'Usage Principaux

### 3.1 Signature unique (one-time)

**Acteur :** Initiateur puis Signataire

**Flux :**
1. Initiateur : Upload PDF → ajout champs → Send → remplir signataire(s) et options
2. Signataire : reçoit email → ouvre lien → remplit champs → valide
3. Initiateur : voit statut « Signed » dans Documents ‣ All Documents → télécharge ou consulte dans workspace

**Recommandations Miyukini :**
- Parcours équivalent : upload → édition → envoi → suivi
- Lien unique et révocable
- Option validité et relances dès le MVP

### 3.2 Template avec ordre de signature

**Acteur :** Initiateur (config) puis plusieurs Signataires

**Flux :**
1. Initiateur : crée template (PDF + champs + rôles) → Template Properties (workspace, tags, redirect, authorized users)
2. Envoi : Send → « Specify Signing Order » activé → ordre 1, 2, 3… par signataire
3. Signataire 1 reçoit → signe → Signataire 2 reçoit → signe → … → document complété
4. Documents signés archivés dans le workspace configuré avec les tags

**Recommandations Miyukini :**
- Modèle « template » avec rôles et ordre de signature
- Notifications en chaîne (un signataire après l’autre)
- Archivage automatique (workspace + tags) côté Miyukini Documents

### 3.3 Authentification renforcée (SMS / itsme® / Aadhaar)

**Acteur :** Administrateur + Initiateur + Signataire

**Flux :**
1. Admin : Sign ‣ Configuration ‣ Roles → Extra Authentication Step (SMS, itsme®, Aadhaar eSign)
2. Initiateur : envoie un document avec un rôle utilisant cette authentification
3. Signataire : remplit les champs → « Validate & Send » → page « Final verification » → SMS code / itsme® / Aadhaar OTP
4. Après validation : signature enregistrée avec preuve d’identité

**Recommandations Miyukini :**
- Paramétrage par rôle (auth optionnelle ou obligatoire)
- Intégration SMS / fournisseurs d’identité selon juridiction (itsme®, Aadhaar, etc.)
- Traçabilité (horodatage, type d’auth) sans stocker de données sensibles inutiles

### 3.4 Tags et recherche

**Acteur :** Initiateur

**Flux :**
1. Configuration ‣ Tags : création de tags (nom, couleur)
2. Sur document ou template : sélection des tags dans la liste
3. Filtrage dans Documents ‣ All Documents par tag pour retrouver rapidement les documents

**Recommandations Miyukini :**
- Tags réutilisables (Sign + Documents) pour cohérence
- Filtres par tag, statut, date, initiateur

### 3.5 Création de template depuis un document envoyé

**Acteur :** Initiateur

**Flux :**
1. Documents ‣ All Documents → choix du document
2. ⋮ (menu) → Template
3. ⋮ → Restore → le document réapparaît sur le dashboard comme template réutilisable

**Recommandations Miyukini :**
- Action « Dupliquer en template » depuis une demande terminée
- Conserver uniquement structure (champs + rôles), pas les données signataires

---

## 4. Points de Friction Identifiés

### 4.1 Un signataire par rôle

**Problème :** Erreur « You must specify one signer for each role of your sign template » si un rôle n’a pas de contact assigné.

**Impact :** Envoi bloqué jusqu’à ce que tous les rôles soient renseignés.

**Recommandations Miyukini :**
- Validation avant envoi avec message explicite listant les rôles manquants
- Suggestion de contacts récents par rôle

### 4.2 Crédits SMS / Aadhaar

**Problème :** Authentification SMS ou Aadhaar nécessite des crédits IAP ; si crédits épuisés, l’étape est ignorée.

**Impact :** Risque de confusion (signataire s’attend à un SMS qui n’arrive pas) ou conformité affaiblie.

**Recommandations Miyukini :**
- Alertes claires quand les crédits sont bas
- Option « Ne pas envoyer si authentification impossible » pour les documents sensibles

### 4.3 Ordre de signature

**Problème :** Si l’ordre n’est pas activé, tous reçoivent en même temps ; si activé, le délai dépend du signataire précédent. Pas toujours clair pour l’initiateur.

**Impact :** Retards ou incompréhension (« Je n’ai pas reçu le document » alors qu’il est en attente du signataire 1).

**Recommandations Miyukini :**
- Indication visuelle du flux (1 → 2 → 3) et du signataire « en cours »
- Notifications à l’initiateur à chaque étape (signé par X, en attente de Y)

### 4.4 Conformité juridique par pays

**Problème :** Validité des signatures électroniques variable selon les pays ; Odoo fournit des pages par pays mais la responsabilité reste à l’utilisateur.

**Impact :** Besoin de vérification juridique pour certains secteurs ou pays.

**Recommandations Miyukini :**
- Documentation « à usage informatif » + lien vers conseil juridique
- Paramétrage « niveau de preuve » (simple, avancé, qualifié) si intégrations itsme®/Aadhaar

### 4.5 Modification des tags d’un document signé

**Problème :** Les tags d’un document déjà signé se modifient via Documents ‣ All Documents ‣ ⋮ ‣ Details ‣ Tags. Peu visible.

**Recommandations Miyukini :**
- Édition des métadonnées (tags, workspace) depuis la fiche « Demande de signature » ou le document signé, avec audit log

---

## 5. Recommandations pour Miyukini

### 5.1 Simplification des parcours

- Dashboard unique : documents en attente, signés, templates
- Création en un clic : « Envoyer un document » (one-time) vs « Partir d’un template »
- Validation avant envoi : checklist (tous les rôles assignés, validité cohérente, relances optionnelles)

### 5.2 Expérience signataire

- Interface signataire épurée (PDF + champs + boutons Valider / Refuser)
- Tips et placeholders par défaut
- Mobile-first pour la signature
- Copie du document signé systématiquement proposée (email + lien de téléchargement)

### 5.3 Conformité et preuves

- Hash et horodatage sur chaque signature
- Logs d’accès (IP, date, type d’auth) sans exposer de données personnelles inutiles
- Export « preuve d’intégrité » pour audit (document + métadonnées signées)

### 5.4 Intégrations

- Documents : workspace et tags pour les documents signés
- Contacts : choix des signataires depuis les contacts, auto-complétion des champs
- Sales / CRM : envoi depuis une opportunité ou un devis (lien document ↔ enregistrement métier)
- Notifications : relances configurables, notifications de progression pour l’initiateur

### 5.5 Administration

- Rôles et types de champs configurables
- Authentification renforcée par rôle (SMS, itsme®, Aadhaar selon disponibilité)
- Gestion des crédits / quotas (SMS, Aadhaar) avec alertes

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
