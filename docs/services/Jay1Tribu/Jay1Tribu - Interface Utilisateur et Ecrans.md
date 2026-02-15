# Jay1Tribu — Interface Utilisateur et Écrans

## Contexte

**Jay1Tribu** est le Service de messagerie pair-à-pair (P2P) de l'écosystème Miyukini COG. L'interface utilisateur combine une **fenêtre de chat de type Steam** (liste d'amis + zone de conversation) et une **gestion des tribus et des salons inspirée de Discord** : CRUD tribu, gestion des membres et des rôles, gestion des salons (canaux) au sein de chaque tribu.

Ce document décrit la structure des panneaux, les écrans, la navigation et les interactions UI sans imposer de choix techniques (framework, thème détaillé).

## Portée / Scope

- **Applicable à :** Spécification des écrans, panneaux, composants UI, flux de navigation, CRUD tribu/salons/membres/rôles.
- **Audience :** Développeurs frontend, designers UX/UI, équipes produit.
- **Statut :** Spécification fonctionnelle — référence UI du Service Jay1Tribu.

### Hors périmètre

- Choix techniques d'implémentation (Dioxus, composants précis).
- Style visuel détaillé (couleurs, polices) — aligné sur le thème Central.

---

## 1. Accès au service

### 1.1 Point d'entrée

Jay1Tribu apparaît dans la **liste des services** de Miyukini Central (Salon / Bibliothèque). À l'ouverture, le service s'affiche dans la zone de contenu principal avec la disposition en trois panneaux décrite ci-dessous.

| Attribut | Valeur |
|----------|--------|
| **Nom affiché** | Jay1Tribu |
| **Icône** | Icône thématique messagerie / tribu (à définir par le design) |
| **Description courte** | « Tribus, amis et discussions — tout reste chez toi. » |

---

## 2. Architecture globale de l'interface

L'interface est organisée en **trois panneaux principaux**, sur le modèle **Steam (liste d'amis + chat)** et **Discord (tribus = serveurs, salons = canaux)** :

```
┌─────────────────┬──────────────────────────────┬────────────────────────────────────┐
│  Panneau 1      │  Panneau 2                   │  Panneau 3                          │
│  Liste d'amis   │  Tribus + Salons             │  Zone de chat                       │
│  + Tribus       │  (ou conversations directes) │  Messages + barre de saisie         │
│  (Steam-like)   │  (Discord-like)              │  (Steam-like)                       │
└─────────────────┴──────────────────────────────┴────────────────────────────────────┘
```

- **Panneau 1** : Liste d'amis (présence, pseudo, statut) + raccourci vers les tribus (icônes type Discord).
- **Panneau 2** : Selon le contexte — soit **liste des tribus et salons** de la tribu sélectionnée (Discord), soit **liste des conversations directes** (amis).
- **Panneau 3** : Contenu du salon ou de la conversation sélectionné (messages) + barre de saisie.

---

## 3. Panneau 1 — Liste d'amis et tribus (inspiration Steam + Discord)

### 3.1 Liste d'amis (type Steam)

| Zone | Contenu | Comportement |
|------|---------|--------------|
| **En-tête** | Titre « Amis » ou icône + libellé | Fixe en haut du panneau. |
| **Liste** | Pour chaque ami : avatar, pseudo (résolu par Jay1Tribu), indicateur de présence (en ligne / hors ligne / absent), statut optionnel (texte court). | Clic sur un ami : ouvre ou crée la conversation directe dans le panneau 3 ; le panneau 2 peut afficher « Conversations directes » avec la conversation sélectionnée. |
| **Tri** | Option : grouper par « En ligne » puis « Hors ligne », ou alphabétique. | Configurable (paramètres ou menu contextuel). |
| **Recherche** | Champ de recherche (optionnel) pour filtrer la liste d'amis. | Filtrage en temps réel. |
| **Actions** | Bouton « Ajouter un ami » / « Demande d'ami » | Ouvre un flux d'invitation (saisie identifiant COG ou découverte MWS). |

**Présence :** La présence (en ligne / hors ligne) est fournie par le MWS ; affichage uniquement, pas de logique côté Jay1Tribu (conformité C-8).

### 3.2 Section Tribus (inspiration Discord — colonne d'icônes)

Sous ou à côté de la liste d'amis, une **colonne verticale d'icônes** représente les tribus auxquelles l'utilisateur appartient :

| Élément | Description | Comportement |
|---------|-------------|--------------|
| **Icône tribu** | Image ou initiales de la tribu (rond ou carré). | Clic : sélectionne la tribu ; le panneau 2 affiche alors les salons et catégories de cette tribu (voir § 4). |
| **Indicateur sélection** | La tribu sélectionnée est mise en évidence (bordure, fond, ou icône agrandie). | Une seule tribu sélectionnée à la fois. |
| **Bouton « + »** | En bas de la colonne. | **Créer une tribu** ou **Rejoindre une tribu** (liste d'invitations, découverte). |

**Bloc utilisateur (bas du panneau 1)** : Avatar et nom de l'utilisateur connecté, options (paramètres compte, statut). Inspiré du bloc utilisateur en bas du panneau gauche Discord / Steam.

---

## 4. Panneau 2 — Tribus et salons (inspiration Discord) ou conversations directes

Deux **modes d'affichage** selon la sélection :

- **Mode Tribu** : une tribu est sélectionnée dans le panneau 1 → affichage des salons et de la gestion de la tribu.
- **Mode Amis** : aucun contexte tribu (ou onglet « Amis ») → affichage des conversations directes (liste des DMs).

### 4.1 Mode Tribu — En-tête de la tribu

| Élément | Description | Comportement |
|---------|-------------|--------------|
| **Nom de la tribu** | Libellé de la tribu (ex. « StudioMiyukini Officiel »). | Affiché en tête du panneau 2. |
| **Menu déroulant** (flèche ou chevron) | Accès aux paramètres de la tribu. | Ouvre un menu : **Paramètres de la tribu**, **Mettre à jour la tribu**, **Quitter la tribu**, **Supprimer la tribu** (si Chef de tribu). → **CRUD Tribu** (Read = sélection, Update/Delete = menu). |
| **Bouton Inviter** | Icône silhouette + « + » ou « Inviter des membres ». | Ouvre le flux d'invitation (envoyer invitation à un COG / ami). |

### 4.2 Mode Tribu — Entrées de gestion

Sous l'en-tête, des entrées horizontales ou en liste :

| Entrée | Rôle |
|--------|------|
| **Membres** | Liste des membres de la tribu ; accès à la **gestion des membres et des rôles** (voir § 6). |
| **Chercher des salons** | Recherche dans les salons de la tribu. |

(Équivalents optionnels type Discord : objectifs, événements, etc. — hors scope minimal.)

### 4.3 Mode Tribu — Catégories et salons (canaux)

Structure **catégories repliables** contenant des **salons** :

| Élément | Description | Comportement |
|---------|-------------|--------------|
| **Catégorie** | Groupe de salons (ex. « rules », « Studio Miyukini », « Mignimaws »). Flèche pour replier / déplier. | Réduire ou développer la liste des salons. |
| **Salon texte** | Préfixe `#` + nom du salon (ex. `#moderator-only`, `#logo-studio`). | Clic : affiche les messages de ce salon dans le panneau 3. |
| **Salon vocal** (optionnel) | Icône haut-parleur + nom. | Pour les salons vocaux (hors scope minimal si non prévu). |
| **Bouton « + »** à côté d'une catégorie ou en bas de liste | Créer un salon ou une catégorie. | **Créer un salon** (nom, type texte/vocal, catégorie) — droit gouverné par Master Butler / rôles. |
| **Icône engrenage** à côté d'un salon | Paramètres du salon. | Menu ou écran : **Modifier le salon** (nom, catégorie), **Supprimer le salon**, **Gérer les permissions** (rôles autorisés à lire/écrire). → **CRUD Salons** (Create = +, Read = clic, Update/Delete = engrenage). |

**Règle :** Seuls les utilisateurs ayant le droit (rôle Chef de tribu, administrateur ou rôle personnalisé) voient les boutons « + » et engrenage ; conformité Master Butler / StrongFather.

### 4.4 Mode Amis — Conversations directes

Si l'utilisateur est en « mode Amis » ou a cliqué sur un ami :

- Le panneau 2 peut afficher une **liste des conversations directes** (salons à 2 participants).
- Clic sur une conversation : affichage des messages dans le panneau 3.
- Option « Nouvelle conversation » (ou clic sur un ami déjà listé) pour ouvrir ou créer un DM.

---

## 5. Panneau 3 — Zone de chat (inspiration Steam)

### 5.1 En-tête du canal / de la conversation

| Élément | Description |
|---------|-------------|
| **Nom** | Nom du salon (ex. `#moderator-only`) ou pseudo de l'ami (conversation directe). |
| **Icônes** | Épingle (messages épinglés), notifications (activer/désactiver), liste des membres du salon (si collectif), recherche dans le salon. |

### 5.2 Zone des messages

- **Affichage** : Messages chronologiques (auteur, horodatage, contenu). Fichiers et images inline selon le type de message.
- **Défilement** : Vers le bas pour l’historique récent ; chargement à la demande pour l’ancien (pagination ou scroll infini).
- **Recherche** : Barre de recherche en haut du panneau pour chercher dans le salon/conversation courant.

### 5.3 Barre de saisie (bas du panneau 3)

| Élément | Description |
|---------|-------------|
| **Champ de saisie** | Placeholder « Envoyer un message dans #nom-du-salon » ou « Envoyer un message à [pseudo] ». |
| **Bouton « + »** | Pièces jointes (fichiers, images). |
| **Boutons optionnels** | Emojis, GIFs, autocollants (selon design). |
| **Envoi** | Bouton envoyer ou touche Entrée. |

Les messages sont chiffrés en transit et archivés localement (C-2, C-1) ; l’UI ne modifie pas ce comportement.

---

## 6. CRUD Tribu — Gestion des membres et des rôles (inspiration Discord)

### 6.1 CRUD Tribu

| Action | Point d'entrée UI | Comportement |
|--------|-------------------|--------------|
| **Create** | Bouton « + » en bas de la colonne tribus (panneau 1). | Ouvrir un formulaire ou un assistant : nom de la tribu, description, paramètres (invitation ouverte / sur invitation). Création → l'utilisateur devient Chef de tribu. |
| **Read** | Clic sur l'icône d'une tribu. | Afficher les salons et membres dans le panneau 2 ; contenu des salons dans le panneau 3. |
| **Update** | Menu déroulant de l'en-tête de la tribu (panneau 2) → « Paramètres de la tribu » / « Modifier la tribu ». | Écran ou modal : modifier le nom, la description, l'icône, les paramètres d'invitation. Réservé au Chef de tribu (ou rôle équivalent). |
| **Delete** | Menu déroulant → « Supprimer la tribu ». | Confirmation explicite ; réservé au Chef de tribu. Suppression gouvernée par StrongFather / KindMother. |

### 6.2 Gestion des membres

| Point d'entrée | Contenu |
|----------------|---------|
| **Entrée « Membres »** (panneau 2) | Ouvre une vue ou un panneau **liste des membres** de la tribu : avatar, pseudo, rôle(s), statut en ligne/hors ligne (MWS). |
| **Actions par membre** | Menu contextuel ou bouton : **Voir le profil**, **Envoyer un message** (ouvre DM), **Attribuer un rôle** (si Chef/admin), **Exclure de la tribu** (si Chef/admin). |
| **Inviter** | Bouton « Inviter des membres » (en-tête tribu) : saisie ou sélection d'un COG/ami, envoi d'une invitation. |

### 6.3 Gestion des rôles

| Point d'entrée | Contenu |
|----------------|---------|
| **Paramètres de la tribu** | Section **Rôles** : liste des rôles (Chef de tribu, Administrateur, Membre, rôles personnalisés). |
| **Créer un rôle** | Bouton « Créer un rôle » : nom, permissions (créer salon, inviter, modérer, etc.). |
| **Modifier / Supprimer un rôle** | Icône engrenage ou menu à côté de chaque rôle (réservé Chef de tribu ou délégation). |
| **Attribution** | Depuis la liste des membres : « Attribuer un rôle » → choix du rôle. Gouverné par Master Butler / StrongFather (C-7). |

---

## 7. Gestion des salons (CRUD)

| Action | Point d'entrée | Comportement |
|--------|----------------|--------------|
| **Create** | Bouton « + » à côté d'une catégorie ou en bas de la liste des salons (panneau 2). | Modal ou formulaire : nom du salon, catégorie (existante ou nouvelle), type (texte / vocal si prévu). Réservé aux rôles ayant la permission « Créer un salon ». |
| **Read** | Clic sur un salon `#nom`. | Afficher les messages dans le panneau 3. |
| **Update** | Icône engrenage à côté du salon → « Modifier le salon ». | Modifier le nom, la catégorie, les permissions (quels rôles peuvent lire/écrire). |
| **Delete** | Icône engrenage → « Supprimer le salon ». | Confirmation ; suppression gouvernée par StrongFather / KindMother. |

---

## 8. Résumé des inspirations

| Besoin | Inspiration | Élément UI |
|--------|-------------|------------|
| Liste d'amis + présence | **Steam** | Panneau 1 : liste amis, en ligne/hors ligne, clic → conversation. |
| Fenêtre de chat | **Steam** | Panneau 3 : messages, barre de saisie, pièces jointes. |
| Tribus = communautés | **Discord** | Colonne d'icônes tribus (panneau 1) + bouton « + » pour créer/rejoindre. |
| Salons = canaux | **Discord** | Panneau 2 : catégories, salons #, bouton « + », engrenage par salon. |
| CRUD tribu, membres, rôles | **Discord** | Menu déroulant tribu (paramètres, inviter), entrée « Membres », section Rôles dans paramètres. |
| Bloc utilisateur | **Discord / Steam** | Bas du panneau 1 : avatar, nom, options. |

---

## 9. Exemples concrets — Liste d'amis et fenêtre de chat

Les exemples ci-dessous décrivent un rendu type **liste d'amis** (panneau 1) et **fenêtre de chat** (panneau 3), inspirés d’interfaces type Steam, pour servir de référence visuelle et fonctionnelle.

### 9.1 Exemple — Liste d'amis (panneau 1)

| Zone | Exemple d'affichage |
|------|---------------------|
| **En-tête** | Bandeau avec **avatar** de l'utilisateur connecté, **pseudo** (ex. « Miyukini »), **statut** (ex. « En ligne »). |
| **Contacts récents / Favoris** | Une ou deux lignes d’**avatars** avec libellé (ex. « Sgt. Pep », « Ozorval ») pour accès rapide aux conversations récentes ou aux favoris. |
| **Barre CONTACTS** | Titre « CONTACTS » avec **icône loupe** (recherche de contacts) et **icône personne +** (ajouter un ami). |
| **Section « En jeu » ou « En activité »** | Amis avec une activité signalée : pour chaque ligne — **icône** (jeu ou salon), **avatar**, **nom**, **contexte** (ex. « Caporalys — Once Human », « Folyarte — ARC Raiders », « Imperatrice — Warframe »). Pour Jay1Tribu, le contexte peut être « Dans [nom du salon] » ou « Dans [tribu] ». |
| **Section « Contacts en ligne (N) »** | Amis connectés sans activité particulière : **avatar**, **nom**, libellé **« En ligne »**. Ex. « Copainconnan de la su — En ligne », « rBow — En ligne ». |
| **Section « Chats de groupe »** | Liste des **conversations de groupe** (salons collectifs ou tribus récentes) ; **bouton « + »** pour créer un nouveau chat de groupe ou rejoindre une tribu. |
| **Design** | Thème sombre, textes blancs ou gris clair, séparateurs discrets entre les sections. |

La présence (en ligne / hors ligne / en activité) est fournie par le MWS ; l’UI se contente de l’afficher.

### 9.2 Exemple — Fenêtre de chat (panneau 3)

| Zone | Exemple d'affichage |
|------|---------------------|
| **En-tête** | **Noms des participants** à la conversation (ex. « Sgt. Pepper », « Imperatrice »). Option : **onglets** pour passer d’une conversation à l’autre (ex. onglet « Sgt. Pepper », onglet « Imperatrice »). Pour une conversation de tribu : nom du salon (ex. « #moderator-only »). |
| **Boîte d’information contextuelle** | En haut du flux, une **boîte d’info** (optionnelle) pour les règles du salon, les inspirations, un lien important, etc. Ex. « Les inspirations fortes : Songs of Syx, Empereur l'empire du milieu (city-builder) » avec lien `https://github.com/...`. Utile pour les salons de tribu (règles, annonces épinglées). |
| **Séparateurs de date** | Dans le flux de messages : **séparateurs de date** (ex. « dimanche 01/02/2026 », « lundi 2 février 2026 », « dimanche 15 février 2026 ») pour structurer la chronologie. |
| **Message type** | Pour chaque message : **avatar** de l’expéditeur, **nom**, **heure d’envoi**, **contenu** (texte, ou carte enrichie). |
| **Cartes de liens / médias** | Pour un lien ou une pièce jointe : **carte intégrée** avec logo ou image, **titre**, **description courte**, **URL** (ex. « Equipement de fabrication de semi-conducteurs et d'écrans LCD », « Site Web officiel de THK », « THK.COM »). Les messages restent chiffrés et hébergés chez les participants (C-1, C-2). |
| **Barre de saisie** | En bas : **champ de texte** (placeholder ex. « Envoyer un message »), flanqué d’icônes — **avion en papier** (envoyer), **smiley** (emojis / stickers), **trombone** (pièces jointes), **micro** (messages vocaux, si prévu). |
| **Design** | Thème cohérent avec la liste d’amis ; messages en bulles ou en lignes ; aperçus de liens bien intégrés dans le flux. |

Ces exemples servent de **référence pour les maquettes et l’implémentation** : ils ne figent pas le style graphique (couleurs, polices), aligné sur le thème Central.

---

## 10. États spéciaux et dégradation

| Situation | Affichage |
|-----------|-----------|
| **Aucun ami** | Liste vide avec message « Aucun ami pour l'instant » et bouton « Ajouter un ami ». |
| **Aucune tribu** | Colonne tribus vide avec uniquement le bouton « + » (Créer / Rejoindre une tribu). |
| **Tribu sans salon** | Panneau 2 : message « Aucun salon » + bouton « Créer un salon » (si droit). |
| **Salon vide** | Panneau 3 : message « Aucun message » + barre de saisie. |
| **MWS déconnecté** | Indicateur discret « Hors ligne » ; présence des amis non mise à jour ; envoi de messages différé (livraison à la reconnexion pour les tribus). |
| **Jay1Tribu indisponible** | Géré par Central (voir [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md)) : pas de crash, dégradation gracieuse. |

---

## 11. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Specification Fonctionnelle](./Jay1Tribu%20-%20Specification%20Fonctionnelle.md) | Parcours, cas d'usage, règles métier. |
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts tribus, salons, amis, rôles. |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes C-1 à C-8, présence via MWS. |

---

**Document** : Jay1Tribu — Interface Utilisateur et Écrans  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Spécification fonctionnelle — référence UI
