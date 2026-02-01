# Odoo Website Builder — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Website Builder** d'Odoo (version 19.0). Il identifie les composants d'interface, l'éditeur visuel, les building blocks, les patterns de navigation et les mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module Website

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Éditeur visuel (mode Edit) et barre d’outils
- Building blocks (catégories, inner content)
- Panneau Customize (par bloc)
- Propriétés de page (Site ‣ Properties)
- Menus (header, footer) et navigation
- Thèmes et personnalisation
- Formulaires et champs
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Éditeur Visuel (Frontend)

### 1.1 Mode Edit

**Accès :** Bouton **Edit** sur le site en frontend (visible aux utilisateurs ayant les droits d’édition).

**Comportement :**
- Bascule en mode édition WYSIWYG
- Barre d’outils en haut (ou contextuelle) : Save, annuler, options globales
- Zones éditables délimitées (bordures turquoise pour les blocs)
- Clic sur un bloc : sélection et ouverture du panneau **Customize**

### 1.2 Barre d’outils et actions globales

**Éléments typiques :**
- **Save** : enregistrer les modifications
- **Mobile / Desktop** : prévisualisation responsive (icône mobile pour adapter colonnes)
- **Site ‣ Properties** : accès aux propriétés de la page (URL, menu, publication, visibilité, SEO)
- **Blocks** : panneau d’insertion de blocs (catégories et inner content)
- Sortie du mode Edit : retour à la vue visiteurs

### 1.3 Insertion de blocs

**Workflow :**
1. Clic sur **Blocks** (ou équivalent) pour ouvrir le panneau.
2. **Catégories** : Basic, About, Landing Pages, Gallery, Services, Pricing, Team, Contact & Forms, Custom.
3. **Recherche** : barre de recherche pour trouver un bloc.
4. **Drag & drop** : glisser un bloc de catégorie vers la page à l’emplacement souhaité.
5. **Inner Content** : blocs « contenu » (vidéo, image, réseaux sociaux, etc.) glissés **dans** un bloc de catégorie déjà placé.
6. Popup de sélection : pour certaines catégories, choix entre plusieurs templates avant insertion.

---

## 2. Building Blocks — Détail UI

### 2.1 Catégories de blocs

| Catégorie | Usage UI |
|-----------|----------|
| Basic | Page vierge, blocs multi-usage |
| About | À propos, marque |
| Landing Pages | Résumé contenu / offre |
| Gallery | Médias, photos |
| Services | Offres, contact |
| Pricing Plans | Abonnements, tarifs |
| Team | Équipe |
| Contact & Forms | Formulaires (contact, CRM, recrutement, etc.) |
| Custom | Blocs enregistrés par l’utilisateur |

### 2.2 Actions sur un bloc (Customize)

**Disponibles selon le type de bloc :**
- **Background** : couleur, image, vidéo, forme (arrière-plan)
- **Layout** : Grid (repositionnement, redimensionnement) ou Cols (colonnes par ligne)
- **Add Elements** : Image, Text, Button (souvent en layout Grid)
- **Column** : paramètres par colonne (Cols)
- **Move / reorder** : flèches haut/bas, gauche/droite ; chevrons pour colonnes
- **Espacement** : bordures turquoise pour augmenter/diminuer l’espace en haut/bas
- **Duplicate** : icône dupliquer (Customize)
- **Delete** : icône corbeille
- **Switch category** : icône échange pour changer de catégorie de bloc
- **Link / anchor** : icône lien pour créer une ancre et copier l’URL

### 2.3 Bloc Formulaire (UI)

**Onglet Customize ‣ Form :**
- **Action** : liste déroulante (Email, Apply for a Job, Create a Customer, Create a Ticket, Create an Opportunity, Subscribe to Newsletter, Create a Task, etc.) selon apps installées.
- **On Success** : URL de redirection, Nothing, Show Message.
- **+ Field** : ajout de champ (Form ou Field section).
- **Field section** : pour chaque champ — Type, Label, Position, Description, Placeholder, Default value, Required, Visibility, Animation.
- **Existing Field** : liaison à un champ du modèle (selon l’action).

### 2.4 Bloc Embed

- **Customize ‣ Edit** : zone de code (iframe ou embed).
- Remplacement du placeholder par le code personnalisé.
- Avertissement documentation : ne pas coller de code non maîtrisé (risque sécurité).

---

## 3. Propriétés de Page (Site ‣ Properties)

**Accès :** En mode Edit ‣ **Site ‣ Properties** (ou backend Website ‣ Site ‣ Pages ‣ ouvrir la page).

**Sections / champs typiques :**
- **Page URL** : modification d’URL ; option **Redirect old URL** + type (301, 302).
- **In Menu** : case à cocher (afficher dans le menu).
- **Is Homepage** : case à cocher (utiliser comme page d’accueil).
- **Published** : bascule Published / Unpublished.
- **Publishing Date** : date/heure de publication planifiée.
- **Indexed** : case à cocher (indexation moteurs de recherche).
- **Visibility** : Public / Signed In / Restricted Group / With Password.
- **Authorized group(s)** : si Restricted Group.
- **Password** : si With Password.
- **Is a template** : sauvegarder comme bloc personnalisé (Custom).
- **Duplicate Page** : bouton pour dupliquer (saisie du nom).
- **Delete Page** : bouton ; popup avec liste des liens référents et option redirection.

---

## 4. Menus (Header / Footer)

**Concepts :**
- **Header** : menu principal (souvent `website.main_menu`).
- **Footer** : liens en bas de page.
- **Éditeur de menu** : accès depuis Structure / Header & Footer (documentation Odoo).
- Items de menu : ordre par **sequence** ; parent/enfant pour sous-menus.
- Chaque item : lien vers une page (website.page) ou URL externe.

**UI typique :**
- Liste des items, drag & drop pour réordonner.
- Édition par item : label, page ou URL, visibilité, position (header/footer).

---

## 5. Thèmes et Personnalisation

**Documentation Odoo :**
- **Thèmes** : personnalisation globale (couleurs, polices, header, footer, arrière-plans, responsive) sans modifier les fichiers cœur.
- **Options par défaut** : options de thème applicables à tout le site.
- **Assets** : CSS/JS des thèmes.
- L’éditeur de site (building blocks, Customize) reste disponible avec les thèmes.

**UI :**
- Accès aux options de thème depuis la configuration du site ou un panneau dédié (selon version).
- Choix de thème prédéfini ou personnalisation des couleurs/fonts.

---

## 6. Design Responsive et Accessibilité

### 6.1 Responsive

- **Layout Cols** : nombre de colonnes par ligne ; sur mobile, documentation indique souvent 1 colonne par défaut (lisibilité).
- **Icône mobile** : en haut de l’éditeur pour prévisualiser et adapter le nombre de colonnes sur mobile.
- **Shapes** : parfois masquées par défaut sur mobile.
- **Redimensionnement** : en layout Grid, redimensionnement par poignées sur les bords du bloc.

### 6.2 Accessibilité

- Liens et boutons : contraste et zones cliquables.
- Formulaires : labels, requis, messages d’erreur.
- Pas de détail spécifique Odoo dans la documentation consultée ; recommandation Miyukini : respecter WCAG et bonnes pratiques (contraste, focus, aria).

---

## 7. Patterns de Navigation

| Contexte | Pattern |
|----------|--------|
| Création de page | Website ‣ + New ‣ Page **ou** Website ‣ Site ‣ Pages ‣ New |
| Édition contenu | Frontend ‣ Edit ‣ clic bloc ‣ Customize |
| Propriétés page | Site ‣ Properties (frontend) ou Backend ‣ Pages ‣ ouvrir |
| Menus | Structure ‣ Header & Footer (ou équivalent) |
| Redirections | Mode développeur ‣ Website ‣ Configuration ‣ Redirects |
| Publication en masse | Backend ‣ Website ‣ Site ‣ Pages ‣ Action ‣ Publish / Unpublish |

---

## 8. Recommandations pour Miyukini

- **Écran de conception vs page livrée** : distinguer clairement l’UI d’édition (blocs, propriétés, menus) de l’UI de consultation (Façade Publique Gouvernée), en cohérence avec « Une page sert à livrer, un écran sert à concevoir ».
- **Building blocks** : modéliser comme Outils ou compositions d’Outils (Master Butler, Kits d’Outils) sans logique métier ; données de structure gouvernées par KindMother.
- **Formulaires** : champs et actions comme flux gouvernés (Mandats) entre MiyuWeb et les Opérateurs métier (MiyuContacts, MiyuCRM, etc.) ; UI de configuration des champs claire et traçable.
- **Thèmes** : options de thème comme configuration gouvernée (Ever Buddy pour versions, compatibilité) et séparable du contenu (pages/blocs).
- **Responsive et accessibilité** : intégrer dès la conception des écrans Miyukini (contraste, focus, aria, colonnes mobiles).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
