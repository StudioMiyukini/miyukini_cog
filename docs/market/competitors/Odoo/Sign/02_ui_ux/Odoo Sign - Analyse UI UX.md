# Odoo Sign — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Sign** d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les composants d'interface, patterns de navigation, formulaires, dashboard et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 — Sign (Productivity)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dashboard et vues principales (documents, templates)
- Éditeur de document (champs, rôles, options)
- Interface signataire (portail/public)
- Configuration (rôles, types de champs, tags)
- Patterns de navigation et feedback
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Dashboard et Vues Principales

### 1.1 Dashboard Sign

**Rôle :** Point d'entrée pour l'initiateur (documents récents, templates, actions rapides).

**Éléments typiques :**
- **Upload a PDF to sign** : signature unique (one-time)
- **Upload a PDF template** : création de template
- **Templates** : liste des templates visibles par défaut
- Pour chaque template : boutons **Send** (envoyer) et **Sign Now** (signer immédiatement soi-même)
- Accès rapide à **Documents ‣ All Documents** pour voir tous les documents et statuts des signatures

**Patterns :**
- Actions principales en haut (upload one-time, upload template)
- Cartes ou listes pour les templates avec actions contextuelles (Send, Sign Now)
- Lien vers la liste complète des documents (Documents ‣ All Documents)

**Recommandations Miyukini :**
- Dashboard unifié : « Envoyer un document », « Créer un template », « Mes documents », « Mes templates »
- Filtres rapides : En attente, Signés, Refusés, Expirés
- Recherche par nom, tag, date

### 1.2 Vue Documents (Documents ‣ All Documents)

**Rôle :** Liste de tous les documents (demandes envoyées) avec statut des signatures.

**Colonnes / champs typiques :**
- Nom ou référence du document
- Statut : shared, sent, signed, refused, canceled, expired
- Signataires et progression (ex. 2/3 signé)
- Date d'envoi, date de validité
- Tags
- Actions : ⋮ (menu contextuel) → Details, Template, etc.

**Filtres :**
- Par statut (sent, signed, refused, etc.)
- Par tag
- Par date
- Par initiateur (si multi-utilisateurs)

**Recommandations Miyukini :**
- Vue liste avec colonnes : Document, Statut, Signataires (progression), Validité, Tags, Actions
- Indicateur visuel de progression (ex. barre 2/3)
- Décoration selon statut (success signed, danger refused, muted expired)

---

## 2. Éditeur de Document (Préparation)

### 2.1 Vue PDF + panneau champs

**Structure :**
- **Zone centrale** : affichage du PDF avec zones de champs positionnées (glisser-déposer)
- **Panneau gauche** : liste des types de champs (Signature, Initial, Text, Multiline Text, Checkbox, Selection)
- **Champ sélectionné** : modification du rôle assigné (clic sur le champ → sélection du rôle)
- **Options** (avant envoi) : Valid Until, Reminders (toggle + nombre de jours), Sign order (toggle + ordre des signataires)

**Interactions :**
- Glisser-déposer d’un type de champ depuis le panneau vers le PDF
- Clic sur un champ placé → attribution du rôle (dropdown ou sélecteur)
- Couleur par rôle pour identifier visuellement les champs (Configuration ‣ Roles ‣ Color)

**Recommandations Miyukini :**
- Éditeur type « PDF viewer + overlay » avec champs repositionnables
- Palette de champs à gauche avec icônes par type
- Clic sur champ → panneau latéral ou popover : rôle, optionnel (placeholder, tip, auto-fill partenaire)
- Aperçu « côté signataire » (optionnel) avant envoi

### 2.2 Template Properties

**Éléments (après « Template Properties » sur un template) :**
- **Tags** : sélection multiple de tags pour catégoriser le template
- **Signed Document Workspace** : espace Documents où seront archivés les PDF signés
- **Signed Document Tags** : tags appliqués automatiquement aux documents signés
- **Redirect Link** : URL affichée dans le message de confirmation après signature
- **Authorized Users** : restriction du template à certains utilisateurs ou groupes

**Recommandations Miyukini :**
- Formulaire ou modal « Propriétés du template » avec ces champs
- Authorized Users : Many2many users/groups avec recherche

### 2.3 Page d'envoi (Send)

**Après clic « Send » sur un document ou template :**
- **Signataires** : pour chaque rôle, sélection d’un contact (recherche partenaire)
- **Ordre de signature** : si « Specify Signing Order » activé, colonne ordre (1, 2, 3…) par signataire
- **Options** : Valid Until (date), Reminders (toggle + jours)
- **Message** : sujet et corps du message (optionnel)
- Bouton **Send** final

**Validations :**
- Un signataire par rôle obligatoire
- Message d’erreur explicite si rôle sans signataire

**Recommandations Miyukini :**
- Wizard ou formulaire « Envoyer » avec étapes ou sections : Signataires, Ordre, Options, Message
- Validation avant envoi avec liste des rôles manquants
- Récapitulatif avant envoi (document, signataires, validité, relances)

---

## 3. Interface Signataire (Portail / Public)

### 3.1 Page de signature

**Structure :**
- **PDF** : affichage du document avec les champs à remplir pour ce signataire (selon son rôle)
- **Panneau gauche** : tips en flèche (ex. « Sign here », « Fill in your birthdate ») pour guider
- **Champs** : placeholder affiché dans le champ avant saisie
- **Signature** : dessin à la souris/doigt, ou génération automatique depuis le nom, ou upload d’image
- **Initial** : même principe que signature, format initiales
- **Text / Multiline** : saisie texte
- **Checkbox** : case à cocher
- **Selection** : liste déroulante
- **Boutons** : « Validate & Send Completed Document » (ou équivalent), éventuellement « Refuse »

**Authentification renforcée :**
- Si rôle configuré avec SMS / itsme® / Aadhaar : après « Validate & Send », page **Final verification** (saisie téléphone + code SMS, ou redirection itsme®, ou Aadhaar OTP)
- Hash signataire : option « Frame » pour afficher le début du hash à côté de la signature

**Recommandations Miyukini :**
- Page dédiée signataire : pas de menu Odoo, uniquement document + champs + boutons
- Tips et placeholders systématiques
- Signature : dessin + option « Générer depuis mon nom » + upload image
- Bouton Refuser clairement séparé (style secondaire)
- Message de confirmation après signature (avec redirect link si configuré) + proposition de téléchargement de la copie signée

### 3.2 Responsive signataire

- Interface utilisable sur mobile (champs tactiles, signature au doigt)
- Formulaire adapté (champs en pleine largeur sur petit écran)
- Boutons accessibles (taille minimale, contraste)

**Recommandations Miyukini :**
- Mobile-first pour la page signataire
- Signature tactile fluide (canvas ou équivalent)
- Pas de dépendance à des plugins lourds (éviter Flash, etc.)

---

## 4. Configuration

### 4.1 Sign ‣ Configuration ‣ Roles

**Vue liste des rôles :**
- Nom du rôle
- Extra Authentication Step (aucune, Unique Code Via SMS, Via itsme®, Via Aadhaar eSign)
- Change Authorized (oui/non)
- Color (couleur pour les champs du rôle)

**Création / édition :**
- New → Role Name, Extra Authentication Step, Change Authorized, Color
- Sauvegarde

**Recommandations Miyukini :**
- CRUD rôles avec champs : name, extra_authentication (enum), change_authorized, color
- Aide contextuelle sur SMS/itsme®/Aadhaar (crédits, pays supportés)

### 4.2 Sign ‣ Configuration ‣ Settings ‣ Edit field types

**Gestion des types de champs (signature item types) :**
- Liste des types existants (Signature, Initial, Text, etc.)
- Création / édition : Field Name, Field Type (Signature, Initial, Text, Multiline Text, Checkbox, Selection), Auto-fill Partner Field (nom technique `res.partner`), Default Width/Height (ratio), Tip, Placeholder

**Recommandations Miyukini :**
- Types de champs configurables (nom, type, auto_fill_partner_field, dimensions par défaut, tip, placeholder)
- Aide sur les noms techniques des champs partenaire (lien doc ou tooltip)

### 4.3 Configuration ‣ Tags

**Gestion des tags :**
- Liste : Tag Name, Color Index
- New → saisie nom et couleur
- Tags utilisés sur documents et templates (dropdown dans le document)

**Recommandations Miyukini :**
- Tags partagés avec Documents si possible (cohérence)
- Création rapide depuis le document (tag inline)

### 4.4 Sign ‣ Configuration ‣ Settings

**Paramètres globaux (déduits) :**
- Authenticate by SMS : achat de crédits (Buy credits)
- Sign with Aadhaar eSign : toggle
- itsme® : activation (Belgique, Pays-Bas)
- Frame (hash) : afficher ou non le cadre de hash sur les signatures

**Recommandations Miyukini :**
- Page Settings : crédits SMS, activation Aadhaar/itsme®, option Frame
- Alertes quand crédits bas

---

## 5. Patterns de Navigation et Feedback

### 5.1 Navigation

- **Sign** : Dashboard → Upload / Templates / Documents
- **Documents ‣ All Documents** : liste → ⋮ → Details, Template, etc.
- **Configuration** : Sign ‣ Configuration ‣ Roles, Settings, Tags
- **Signataire** : lien email → page de signature (hors menu Odoo) → confirmation

**Recommandations Miyukini :**
- Breadcrumb : Sign > Dashboard / Documents / Templates / Configuration
- Retour cohérent depuis détail document vers liste ou dashboard

### 5.2 Feedback et erreurs

- **Envoi** : message d’erreur si « You must specify one signer for each role »
- **Signature** : message de succès + redirection (redirect link) + copie par email
- **Refus** : statut « Refused » visible pour l’initiateur, notification si configuré
- **Expiration** : statut « Expired » si valid_until dépassée

**Recommandations Miyukini :**
- Toasts ou messages inline pour succès/erreur
- Liste explicite des rôles sans signataire en cas d’erreur d’envoi
- Notification initiateur à chaque étape (signé par X, refusé par Y, expiré)

### 5.3 Statut des signatures

- Affichage du statut par document : shared, sent, signed, refused, canceled, expired
- Progression multi-signataires : ex. « 2/3 signé » avec indicateur visuel (barre ou badges)

**Recommandations Miyukini :**
- Badge ou libellé par statut (couleur sémantique)
- Progression 2/3 avec barre de progression ou avatars des signataires (signé / en attente / refusé)

---

## 6. Design Responsive et Accessibilité

### 6.1 Responsive

- **Dashboard** : grille adaptative (templates en cartes ou liste selon largeur)
- **Éditeur** : panneau gauche repliable sur tablette/mobile ; PDF zoomable
- **Page signataire** : pleine largeur, champs et boutons empilés sur petit écran
- **Liste documents** : colonnes masquables ou priorisées sur mobile

**Recommandations Miyukini :**
- Breakpoints cohérents avec le reste de l’app
- Éditeur utilisable sur tablette (placement des champs)

### 6.2 Accessibilité

- Contraste suffisant (texte, boutons, champs)
- Labels associés aux champs (signature, texte, checkbox)
- Focus visible (clavier)
- Messages d’erreur associés aux champs (aria-describedby ou équivalent)
- Éviter le seul retour visuel (couleur) pour le statut ; prévoir texte ou icône

**Recommandations Miyukini :**
- Respect WCAG 2.1 niveau AA pour la page signataire (usage potentiel public)
- Signature au clavier (alternative ou complément au dessin) si possible

---

## 7. Synthèse des Recommandations pour Miyukini

- **Dashboard** : actions claires (one-time, template), accès Documents et Templates, filtres et recherche.
- **Éditeur** : glisser-déposer champs, attribution rôle par champ, options (validité, relances, ordre).
- **Signataire** : page épurée, tips/placeholders, signature dessin/génération/upload, authentification renforcée, confirmation + copie.
- **Configuration** : rôles (auth renforcée), types de champs (auto-fill partenaire), tags, paramètres globaux (crédits, Frame).
- **Feedback** : statut et progression visibles, erreurs explicites (rôles manquants), notifications initiateur.
- **Responsive et accessibilité** : mobile-first signataire, contraste, clavier, labels et erreurs accessibles.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
