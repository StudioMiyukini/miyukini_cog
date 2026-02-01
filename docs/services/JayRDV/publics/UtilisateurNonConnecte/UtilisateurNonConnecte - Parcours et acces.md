# Utilisateur non connecté — Parcours et accès

## Contexte

Ce document détaille le **parcours** et les **règles d’accès** du public cible **Utilisateur non connecté** dans le cadre du service JayRDV. Il complète le [document fondateur](../../JayRDV%20-%20Document%20Fondateur.md) et s’appuie sur l’[analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md). L’utilisateur non connecté accède à la **Façade publique gouvernée** : il consulte et réserve **sans compte** (parcours guest).

## Portée / Scope

- **Public** : Utilisateur non connecté (toute personne accédant à un lien de réservation ou à un widget sans être authentifiée).
- **Périmètre** : Parcours détaillé (accès lien/widget, choix service, choix créneau, formulaire guest, confirmation, rappels, annulation et modification via lien email), règles d’accès à la Façade publique, limites et gouvernance.
- **Hors périmètre** : Spécifications techniques d’implémentation (Opérateurs, Kits, API) ; parcours avec compte client (documenté dans [Clients — Parcours, capacités et livrables](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md)).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Toute personne qui souhaite réserver un rendez-vous chez un professionnel sans créer de compte ni se connecter. |
| **Compte** | Aucun — parcours **guest** exclusif pour ce document. |
| **Accès** | Accès au **lien de réservation** du professionnel (partagé par email, site, réseaux sociaux) ou au **widget** intégré sur le site du professionnel. Aucune authentification. |
| **Espace** | Page de réservation publique (lien pro ou widget) ; page de confirmation à l’écran ; email/SMS de confirmation et de rappel ; page d’annulation ou de modification (lien dans l’email, token sécurisé). Pas d’accès à « Mes RDV » (réservé aux clients avec compte). |

L’utilisateur non connecté partage avec le public **Clients** le même parcours de réservation guest (choix service → créneau → formulaire → confirmation). La distinction est : avec compte client, il peut ensuite accéder à « Mes RDV », au profil et à la pré-remplissage ; sans compte, il reste limité à la Façade publique (réservation, lien annulation/modification).

---

## 2. Parcours détaillés

### 2.1 Parcours réservation guest (sans compte)

| Étape | Action | Résultat | Règle d’accès |
|-------|--------|----------|---------------|
| **1. Accès** | L’utilisateur clique sur le lien de réservation du professionnel (ou ouvre la page contenant le widget). | La page de réservation s’affiche avec les services proposés (nom, durée, tarif optionnel). | Accès **public** : aucune authentification. Lien ou widget exposé par le professionnel. |
| **2. Choix du service** | L’utilisateur sélectionne un service dans la liste (ex. « Consultation 30 min »). | Passage à l’étape « Choix du créneau » ; les créneaux disponibles pour ce service s’affichent. | Affichage des **seuls services actifs** configurés par le professionnel. |
| **3. Choix du créneau** | L’utilisateur consulte le calendrier ou la liste des prochains jours et sélectionne une date et une heure. | Le créneau est sélectionné ; passage au formulaire. Les créneaux sont mis à jour en temps réel (pas de double réservation). | Affichage des **seules disponibilités** ; pas d’accès à l’agenda détaillé du professionnel ni aux noms des autres clients. |
| **4. Formulaire** | L’utilisateur saisit nom, email, téléphone, remarque (optionnel). Aucune création de compte. | Formulaire validé (format email, champs obligatoires). Bouton « Confirmer le rendez-vous » activé. | Données collectées **uniquement** pour la réservation, la confirmation et les rappels ; pas de revente (RGPD). |
| **5. Confirmation** | L’utilisateur clique sur « Confirmer ». | Page de confirmation à l’écran : « Votre RDV est confirmé. Vous allez recevoir un email de confirmation. » Email (et/ou SMS) envoyé avec récapitulatif, lien « Ajouter à mon agenda », lien « Annuler ou modifier le RDV ». | Une seule réservation créée (idempotence si double clic). Créneau verrouillé côté serveur. |

**Livrables sollicités** : Page de réservation (lien pro ou widget), choix service, choix créneau, formulaire guest, page de confirmation, email/SMS de confirmation, lien ajout agenda, lien annulation/modification.

**Objectif parcours court** : Maximum 4 à 5 étapes ; temps moyen de réservation < 60 secondes (référence marché).

### 2.2 Parcours rappel (automatique)

| Étape | Action | Résultat | Règle d’accès |
|-------|--------|----------|---------------|
| **Rappel** | Le système envoie automatiquement un rappel au délai configuré par le professionnel (ex. 24 h et/ou 2 h avant le RDV). | L’utilisateur reçoit un email (et/ou SMS) avec la date, l’heure, le lieu, le service. Optionnel : lien « Annuler ou modifier le RDV ». | Canal et contenu définis par le professionnel (Miyunotify ou équivalent). |

Aucune action de l’utilisateur non connecté n’est requise pour le rappel ; il est **passif** (réception du message). L’objectif est la réduction du no-show (référence : division par 5).

### 2.3 Parcours annulation (depuis le lien email)

| Étape | Action | Résultat | Règle d’accès |
|-------|--------|----------|---------------|
| **1. Accès** | L’utilisateur clique sur le lien « Annuler ou modifier le RDV » dans l’email de confirmation. | Une page s’ouvre **sans connexion** avec le récapitulatif du RDV (date, heure, service, professionnel, lieu) et les boutons « Annuler le RDV » et « Modifier le créneau ». | Lien **unique**, **temporaire** et **non devinable** (token sécurisé). Expiration configurable (ex. J-1 ou 7 jours). |
| **2. Confirmation** | L’utilisateur clique sur « Annuler le RDV », puis confirme (motif optionnel). | Le RDV est annulé ; page « Votre RDV a bien été annulé » ; email de confirmation d’annulation ; créneau libéré côté professionnel. | Token **invalidé** après utilisation. Si lien expiré : message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **3. Fin** | — | L’utilisateur ne peut plus réutiliser le même lien. | Pas d’accès à « Mes RDV » sans compte. |

**Livrables sollicités** : Lien dans l’email (token), page d’annulation, confirmation à l’écran et par email, libération du créneau, notification au professionnel (si configurée).

### 2.4 Parcours modification de créneau (depuis le lien email)

| Étape | Action | Résultat | Règle d’accès |
|-------|--------|----------|---------------|
| **1. Accès** | L’utilisateur clique sur « Modifier le créneau » (depuis la page ouverte via le lien email). | Page avec récapitulatif du RDV et **liste des créneaux disponibles** pour le même service. | Même token que pour l’annulation ; expiration identique. |
| **2. Choix** | L’utilisateur sélectionne un nouveau créneau et valide. | Le RDV est mis à jour ; page « Votre RDV a bien été modifié » ; email de confirmation de modification ; ancien créneau libéré, nouveau créneau réservé. | Token invalidé après utilisation (ou conservé pour une seule modification selon règle métier). |
| **3. Fin** | — | Le professionnel est notifié si configuré. | Pas d’accès à l’historique des RDV sans compte. |

**Livrables sollicités** : Page de modification (créneaux disponibles), confirmation à l’écran et par email, notification au professionnel.

### 2.5 Passerelles vers compte client (optionnel)

| Étape | Action | Résultat | Règle d’accès |
|-------|--------|----------|---------------|
| **Proposition** | Sur la page de réservation ou de confirmation, un lien « Créer un compte » ou « Se connecter » est affiché (non bloquant). | L’utilisateur peut, **s’il le souhaite**, être redirigé vers l’inscription ou la connexion (Miyauth). | **Aucune obligation** pour réserver. Le parcours guest reste complet sans compte. |
| **Après inscription/connexion** | Contexte conservé (ex. retour à la page de réservation ou accès à « Mes RDV »). | Les prochaines réservations pourront être pré-remplies et gérées depuis « Mes RDV ». | Passage dans le périmètre du public **Clients** (documenté dans Clients — Parcours, capacités et livrables). |

---

## 3. Règles d’accès à la Façade publique

### 3.1 Principe (Glossaire Miyukini)

Selon le glossaire Miyukini, la **Façade publique gouvernée** est la zone tampon d’exposition permettant aux utilisateurs externes d’interagir avec un COG **sans y entrer**. Règles :

- **Strictement unidirectionnelle** : le COG sort vers l’utilisateur (affichage des disponibilités, formulaire de réservation, confirmation) ; l’utilisateur ne pénètre pas dans les cores ni dans l’état souverain.
- **Sans identité persistante obligatoire** : l’utilisateur non connecté n’a pas de compte ; il est identifié uniquement par les données saisies (nom, email, téléphone) et par le token d’annulation/modification.
- **Sans accès aux cores** : pas d’accès à StrongFather, KindMother, Master Butler, etc. ; uniquement les capacités exposées par le Mandat public d’accès (réservation, annulation/modification via lien).

### 3.2 Ce qui est exposé (Utilisateur non connecté)

| Capacité exposée | Description | Limite |
|------------------|-------------|--------|
| **Affichage des services** | Liste des services actifs du professionnel (nom, durée, tarif si affiché). | Services configurés par le professionnel ; pas de données internes. |
| **Affichage des créneaux** | Créneaux disponibles pour un service donné (date, heure). Mise à jour en temps réel. | **Seules les disponibilités** ; pas d’agenda détaillé, pas de noms d’autres clients. |
| **Formulaire de réservation** | Saisie nom, email, téléphone, remarque. Soumission pour créer une réservation. | Données utilisées uniquement pour la réservation, la confirmation et les rappels (RGPD). |
| **Page de confirmation** | Récapitulatif du RDV après validation. | Affichage à l’écran uniquement ; pas d’historique global. |
| **Lien « Ajouter à mon agenda »** | Dans l’email de confirmation ; téléchargement iCal ou redirection Google/Outlook/Apple. | Lien généré pour ce RDV uniquement. |
| **Lien « Annuler ou modifier le RDV »** | Dans l’email de confirmation ; ouverture d’une page dédiée (token sécurisé). | Token unique, temporaire, non devinable ; expiration configurable ; invalidé après utilisation. |

### 3.3 Ce qui n’est pas exposé

| Non exposé | Raison |
|------------|--------|
| **Agenda détaillé du professionnel** | Confidentialité ; seules les plages « libres » sont affichées. |
| **Noms ou coordonnées des autres clients** | RGPD ; pas de fuite de données. |
| **« Mes RDV »** | Réservé aux clients avec compte (authentification). |
| **Paramètres du professionnel** | Gestion interne (créneaux, rappels, politique d’annulation) ; l’utilisateur voit uniquement le résultat (créneaux proposés, messages reçus). |
| **Données de paiement** (si activé) | Traitées par une page sécurisée (PCI, 3D Secure) ; pas de stockage des numéros de carte sur la Façade. |

### 3.4 Sécurité des liens (token annulation/modification)

| Règle | Description |
|-------|-------------|
| **Génération** | Un token **unique**, **non séquentiel** et **non devinable** est généré à la création du RDV et associé au lien « Annuler ou modifier le RDV » dans l’email. |
| **Stockage** | Le token est stocké côté serveur avec l’identifiant du RDV ; vérification à chaque ouverture du lien. |
| **Expiration** | Date de fin de validité configurable (ex. J-1 avant le RDV, ou 7 jours après l’envoi de l’email). Après expiration : message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **Invalidation** | Après utilisation (annulation ou modification), le token est invalidé ; le lien ne fonctionne plus. |
| **Pas de données sensibles dans l’URL** | L’URL ne contient que le token (ou un identifiant opaque) ; pas d’email, pas d’ID métier en clair. |

### 3.5 Quotas et limites (Façade publique)

| Limite | Description | Objectif |
|--------|-------------|----------|
| **Rate limiting (affichage créneaux)** | Nombre maximal de requêtes « créneaux disponibles » par IP et par lien professionnel sur une fenêtre glissante (ex. 60 req/min). | Limiter le scraping et les abus. |
| **Rate limiting (soumission formulaire)** | Nombre maximal de réservations soumises par IP et par lien pro sur une fenêtre (ex. 10 réservations/heure). | Limiter le spam et les réservations abusives. |
| **Rate limiting (lien annulation/modification)** | Nombre maximal de requêtes sur une URL de lien token par IP (ex. 20 req/min). | Limiter les attaques par force brute sur les tokens. |
| **Taille des champs** | Nom, email, téléphone, remarque : longueur max définie (ex. 200 caractères pour la remarque). | Éviter les abus et les injections. |

Les seuils exacts sont à définir en implémentation ; le principe est que la Façade publique reste **disponible** pour les utilisateurs légitimes tout en limitant les abus.

### 3.6 Responsive et accessibilité

| Règle | Description |
|-------|-------------|
| **Responsive** | La page de réservation, la page de confirmation et les pages d’annulation/modification (lien email) doivent être utilisables sur **mobile**, **tablette** et **desktop**. |
| **Accessibilité** | Respect des critères WCAG 2.1 niveau AA : contraste, focus visible, labels associés aux champs, messages d’erreur annoncés. |

---

## 4. Capacités et livrables (par étape)

### 4.1 Page de réservation (lien pro ou widget)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Affichage des services** | Liste des services proposés par le professionnel (nom, durée, tarif optionnel). | Bloc « Choisir un service » : cartes ou liste ; clic pour sélectionner. |
| **Affichage des créneaux** | Créneaux disponibles pour le service choisi ; mise à jour en temps réel. | Calendrier ou liste des prochains jours ; créneaux cliquables ; pas de créneau déjà pris (vérification côté serveur). |
| **Choix du praticien** | Si le professionnel a plusieurs praticiens, choix du praticien ou « Premier disponible ». | Liste déroulante ou onglets ; créneaux filtrés selon le choix. |
| **Formulaire guest** | Champs nom, email, téléphone (obligatoires), remarque (optionnel). Pas de mot de passe ni de création de compte. | Formulaire avec validation (format email, champs obligatoires) ; bouton « Confirmer le rendez-vous ». |
| **Paiement en ligne** (optionnel, P1) | Si le professionnel l’a configuré, page de paiement après saisie des infos. | Page de paiement sécurisée (PCI, 3D Secure) ; confirmation du paiement et du RDV. |

La page doit se charger en **moins de 3 secondes** (NFR-UNC-01) et les créneaux doivent être à jour en **temps réel** (latence < 2 s après une réservation par un autre client — NFR-UNC-02).

### 4.2 Confirmation et rappels

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Confirmation à la réservation** | Envoi automatique d’un email (et/ou SMS) après la prise de RDV. | Email/SMS avec récapitulatif (date, heure, service, lieu, professionnel), lien « Ajouter à mon agenda », lien « Annuler ou modifier le RDV ». Modèle personnalisable par le professionnel. |
| **Rappel automatique** | Envoi d’un rappel la veille et/ou quelques heures avant le RDV. | Email/SMS au délai configuré par le pro (ex. 24 h et 2 h avant) ; contenu : date, heure, lieu, service ; lien annulation/modification optionnel. |

Les notifications s’appuient sur Miyunotify (ou équivalent) ; les canaux et modèles sont configurables par le professionnel.

### 4.3 Annulation et modification (lien email)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Page dédiée (lien email)** | Ouverture sans connexion ; récapitulatif du RDV ; boutons « Annuler le RDV » et « Modifier le créneau ». | Page avec récapitulatif ; boutons ; token sécurisé ; expiration. |
| **Annulation** | Confirmation avant annulation (motif optionnel) ; libération du créneau ; email de confirmation d’annulation. | Flux : clic Annuler → confirmation → validation → message « Votre RDV a bien été annulé » ; email. |
| **Modification** | Affichage des créneaux disponibles ; sélection d’un nouveau créneau ; confirmation ; email de confirmation de modification. | Flux : clic Modifier → créneaux → sélection → validation → message « Votre RDV a bien été modifié » ; email. |
| **Lien expiré** | Si le token est expiré ou déjà utilisé : message clair. | « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |

### 4.4 Passerelles compte client (optionnel)

| Capacité | Description | Livrable |
|----------|-------------|----------|
| **Lien « Créer un compte »** | Visible sur la page de réservation ou de confirmation ; redirection vers l’inscription (Miyauth). | Lien non bloquant ; après inscription, contexte conservé ou accès à « Mes RDV ». |
| **Lien « Se connecter »** | Visible sur la page de réservation ; redirection vers la connexion. | Après connexion, pré-remplissage possible pour les prochaines réservations. |

---

## 5. Points de sortie

| Sortie | Description |
|--------|-------------|
| **Vers professionnel** | La réservation est enregistrée ; les données client (nom, email, téléphone) sont disponibles côté professionnel (fiche client, historique RDV). Annulation ou modification libère ou met à jour le créneau ; le pro peut être notifié. |
| **Vers compte client** | Si l’utilisateur clique sur « Créer un compte » ou « Se connecter », il quitte le périmètre « Utilisateur non connecté » et entre dans le périmètre **Clients** (espace « Mes RDV », profil, préférences). |
| **Vers agenda externe** | Lien « Ajouter à mon agenda » dans l’email : téléchargement iCal ou redirection Google/Outlook/Apple. |
| **Fin de parcours** | Après confirmation, l’utilisateur non connecté n’a plus d’écran dédié ; il reçoit les emails (confirmation, rappel) et peut utiliser le lien annulation/modification jusqu’à expiration. |

---

## 6. Critères d’acceptation par parcours

### 6.1 Réservation guest

- La page de réservation se charge en moins de 3 secondes (NFR-UNC-01).
- Les services affichés sont ceux configurés par le professionnel (actifs uniquement).
- Les créneaux affichés sont à jour en temps réel ; après une réservation par un autre client, le créneau disparaît en moins de 2 secondes (NFR-UNC-02).
- Le formulaire exige au minimum : nom, email, téléphone. Validation du format email. Remarque optionnelle. **Aucune** création de compte obligatoire.
- Après clic « Confirmer », la confirmation s’affiche et l’email/SMS est envoyé en moins de 5 secondes (NFR-UNC-03).
- Le lien « Ajouter à mon agenda » dans l’email permet de télécharger un fichier iCal ou d’ouvrir Google/Outlook/Apple.
- Le lien « Annuler ou modifier le RDV » dans l’email est présent et pointe vers une URL avec token sécurisé.

### 6.2 Confirmation et rappels

- La confirmation est envoyée automatiquement à la réservation (email et/ou SMS selon config pro).
- Le rappel est envoyé au délai configuré (ex. 24 h et 2 h avant).
- Le contenu (récap, lien annulation/modification) est personnalisable par le professionnel (modèles).

### 6.3 Annulation (lien email)

- Le lien dans l’email est unique, temporaire et non devinable (token sécurisé).
- La page d’annulation affiche le récapitulatif du RDV et un bouton « Annuler le RDV » ; confirmation avant annulation (motif optionnel).
- Après annulation : message « Votre RDV a bien été annulé » ; email de confirmation d’annulation ; créneau libéré côté pro.
- Après expiration ou utilisation : message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. »

### 6.4 Modification (lien email)

- La page de modification affiche les créneaux disponibles pour le même service ; sélection d’un nouveau créneau ; confirmation.
- Après modification : message « Votre RDV a bien été modifié » ; email de confirmation de modification ; ancien créneau libéré, nouveau créneau réservé.
- Après expiration ou utilisation : même message que pour l’annulation.

### 6.5 Façade publique et limites

- Aucune donnée sensible (agenda détaillé du pro, noms d’autres clients) n’est exposée.
- Rate limiting appliqué sur les endpoints publics (affichage créneaux, soumission formulaire, lien token).
- Responsive : page de réservation et pages annulation/modification utilisables sur mobile, tablette, desktop.
- Accessibilité : WCAG 2.1 niveau AA (contraste, focus, labels, messages d’erreur).

---

## 7. Cas limites et comportements attendus

| Cas | Comportement attendu |
|-----|----------------------|
| **Deux utilisateurs réservent le même créneau** | Un seul obtient la réservation ; l’autre reçoit « Ce créneau n’est plus disponible » et peut en choisir un autre. Verrouillage côté serveur. |
| **Double clic sur « Confirmer »** | Une seule réservation est créée ; message de succès une seule fois ; pas de double email (idempotence). |
| **Lien d’annulation/modification expiré** | Message « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. » |
| **Lien déjà utilisé (annulation ou modification)** | Token invalidé ; même message que pour expiration. |
| **Email invalide** | Validation du format email à la saisie ; message d’erreur si invalide ; pas d’envoi de confirmation si email invalide. |
| **Créneau plus disponible au moment de la confirmation** | Vérification côté serveur à la soumission ; si créneau pris entre-temps : « Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. » |
| **Utilisateur clique sur « Créer un compte » en milieu de parcours** | Redirection vers inscription ; après inscription, retour possible à la page de réservation (contexte conservé) ou accès à « Mes RDV ». La réservation en cours (si non finalisée) peut être reprise selon implémentation. |

---

## 8. Écrans et zones fonctionnelles

### 8.1 Écran Page de réservation (lien pro ou widget)

- **En-tête** : Nom du professionnel (ou logo). Optionnel : lien « Se connecter » ou « Créer un compte » (non bloquant).
- **Zone principale** :  
  - Étape 1 — Choix du service (liste ou cartes).  
  - Étape 2 — Choix du créneau (calendrier ou liste des jours/heures). Si multi-praticiens : choix du praticien ou « Premier disponible ».  
  - Étape 3 — Formulaire (nom, email, téléphone, remarque). Bouton « Confirmer le rendez-vous ».  
  - Si paiement activé par le pro : étape paiement après formulaire.  
- **Zone confirmation** : Après validation — « Votre RDV est confirmé » ; récapitulatif (date, heure, service, lieu) ; « Vous allez recevoir un email de confirmation. »  
- **Pied de page** : Coordonnées du professionnel (optionnel), mentions légales (lien), lien « Créer un compte » (optionnel).

### 8.2 Écran Page de confirmation (post-réservation)

- Message principal : « Votre rendez-vous est confirmé. »
- Récapitulatif : date, heure, service, professionnel, lieu.
- Indication : « Vous allez recevoir un email de confirmation avec un lien pour annuler ou modifier votre RDV. »
- Lien « Ajouter à mon agenda » (optionnel sur la page, sinon uniquement dans l’email).
- Lien « Créer un compte » ou « Se connecter » (optionnel).

### 8.3 Écran Annulation / Modification (depuis lien email)

- **Récapitulatif du RDV** : Date, heure, service, professionnel, lieu.
- **Boutons** : « Annuler le RDV », « Modifier le créneau ».
- **Confirmation annulation** : « Êtes-vous sûr de vouloir annuler ? » (motif optionnel). « Oui, annuler » / « Non, garder le RDV ».
- **Modification** : Liste des créneaux disponibles ; sélection d’un nouveau créneau ; bouton « Confirmer la modification ».
- **Résultat** : « Votre RDV a bien été annulé. » ou « Votre RDV a bien été modifié. Nouvelle date : [date], [heure]. »
- **Lien expiré** : « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. »

---

## 9. Synthèse des livrables par bloc

| Bloc | Livrable principal | Objectif |
|------|--------------------|----------|
| **Page de réservation** | Lien pro ou widget : services, créneaux, formulaire guest, confirmation, lien ajout agenda (email), lien annulation/modification (email). | Permettre à l’utilisateur non connecté de réserver 24h/24 sans compte. |
| **Confirmation et rappels** | Email/SMS à la réservation ; rappel 24h et/ou 2h avant ; modèle personnalisable par le pro. | Réduire les no-show ; informer l’utilisateur. |
| **Annulation/modification (lien email)** | Page dédiée (token sécurisé) ; annulation ou modification sans connexion ; message lien expiré. | Autonomie de l’utilisateur ; moins d’appels pour le pro. |
| **Passerelles compte client** | Liens « Créer un compte » et « Se connecter » (optionnels, non bloquants). | Favoriser l’inscription sans l’imposer. |
| **Règles d’accès** | Façade publique : seules les disponibilités et le formulaire ; pas d’agenda détaillé ni de noms d’autres clients ; token sécurisé ; rate limiting. | Sécurité et confidentialité ; conformité RGPD. |

---

## 10. Références et annexes

### 10.1 Références

- [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md)
- [Utilisateur non connecté — Analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md)
- [Clients — Parcours, capacités et livrables](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md) — parcours guest partagé, compte client, Mes RDV.
- [Public Professionnels](../Professionnels/_index.md) | [Public Clients](../Clients/_index.md)
- [Fonctionnalités solutions réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md)

### 10.2 Checklist MVP (Utilisateur non connecté)

- [ ] **Accès** : Lien de réservation du professionnel accessible sans compte ; page affiche les services et les créneaux disponibles.
- [ ] **Choix service** : Liste des services (nom, durée, tarif si affiché) ; sélection d’un service ; passage à l’étape créneaux.
- [ ] **Choix créneau** : Créneaux affichés en temps réel ; sélection date/heure ; si multi-praticiens, choix du praticien ou « Premier disponible ».
- [ ] **Formulaire** : Champs nom, email, téléphone (obligatoires) ; remarque (optionnel) ; pas de mot de passe ; pas d’obligation de créer un compte.
- [ ] **Confirmation** : Message de succès à l’écran ; email et/ou SMS envoyé avec récapitulatif ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier le RDV ».
- [ ] **Rappel** : Rappel envoyé au délai configuré par le pro (24h et/ou 2h avant) ; contenu : date, heure, lieu, service.
- [ ] **Annulation** : Lien dans l’email fonctionne sans connexion ; page d’annulation avec confirmation ; motif optionnel ; email de confirmation d’annulation ; créneau libéré côté pro.
- [ ] **Modification** : Lien dans l’email fonctionne sans connexion ; page avec créneaux disponibles ; sélection nouveau créneau ; confirmation ; email de confirmation de modification.
- [ ] **Lien expiré** : Message clair « Ce lien a expiré. Connectez-vous pour annuler ou contactez le professionnel. »
- [ ] **Passerelles** : Option « Créer un compte » et « Se connecter » visibles ; pas d’obligation pour réserver.
- [ ] **Limites** : Pas d’accès à « Mes RDV » sans compte ; affichage des seules disponibilités ; RGPD.
- [ ] **Responsive** : Page de réservation et pages annulation/modification utilisables sur mobile, tablette et desktop.

### 10.3 Dépendances techniques (référence)

| Livrable (Utilisateur non connecté) | Composant / Kit | Rôle |
|-----------------------------------|-----------------|------|
| Page réservation, créneaux | Miyubooking, lien pro / widget | Page publique, calcul des créneaux disponibles. |
| Formulaire guest, réservation | Miyubooking, Miyucontacts (léger) | Enregistrement RDV, données client. |
| Confirmation, rappels | Miyunotify | Email et SMS. |
| Lien annulation/modification | Miyubooking, token sécurisé | Lien unique temporaire ; page dédiée. |
| RGPD, consentement | WorrySentinel, traçabilité | Données personnelles, droits. |
| Façade publique | Mandat public d’accès, Border Guard | Exposition des seules capacités autorisées. |

### 10.4 Récapitulatif des parcours (Utilisateur non connecté)

| Parcours | Déclencheur | Étapes clés | Livrables |
|----------|-------------|-------------|-----------|
| **Réservation guest** | Clic sur lien du pro ou widget | Choix service → Choix créneau → Saisie infos → Confirmation | Page réservation, confirmation, rappels, lien agenda, lien annulation/modification |
| **Annulation (lien email)** | Clic sur « Annuler ou modifier » dans email | Ouverture page → Confirmation annulation → Validation | Lien sécurisé, page annulation |
| **Modification (lien email)** | Clic sur « Modifier le créneau » | Affichage créneaux → Sélection nouveau → Confirmation | Page modification, confirmation |
| **Passage vers compte client** | Clic sur « Créer un compte » (optionnel) | Redirection inscription → Inscription → Retour contexte ou Mes RDV | Passerelle Miyauth, espace Clients |

### 10.5 Index des sections

1. Contexte et portée — 2. Parcours détaillés (réservation guest, rappel, annulation, modification, passerelles) — 3. Règles d’accès à la Façade publique (exposé / non exposé, token, quotas, responsive, accessibilité) — 4. Capacités et livrables par étape — 5. Points de sortie — 6. Critères d’acceptation par parcours — 7. Cas limites — 8. Écrans et zones fonctionnelles — 9. Synthèse des livrables par bloc — 10. Références et annexes (checklist MVP, dépendances techniques, récap parcours, index).

### 10.6 Points de vigilance (parcours Utilisateur non connecté)

- **Pas d’obligation de créer un compte** : Le parcours guest doit être complet et fonctionnel sans aucune étape de création de compte.
- **Créneaux en temps réel** : Vérification côté serveur à la confirmation ; pas de double réservation ; message clair si créneau plus disponible.
- **Lien annulation/modification** : Token unique, temporaire, non devinable ; expiration configurable (ex. J-1 ou 7 jours) ; invalidé après utilisation ; message clair si expiré.
- **Parcours court** : Objectif < 60 s (référence marché) ; maximum 4 à 5 étapes.
- **Rappels** : Objectif réduction no-show (référence : division par 5) ; modèles personnalisables par le pro.
- **RGPD** : Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation.
- **Façade publique** : Affichage des seules disponibilités ; pas de fuite de données (noms des autres clients, agenda détaillé du pro).

### 10.7 Exemples de libellés (référence)

| Contexte | Libellé type |
|----------|--------------|
| **Bouton confirmation** | « Confirmer le rendez-vous » |
| **Page confirmation** | « Votre rendez-vous est confirmé. Vous allez recevoir un email de confirmation. » |
| **Lien email** | « Annuler ou modifier le rendez-vous » |
| **Message créneau non disponible** | « Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. » |
| **Lien expiré** | « Ce lien a expiré. Connectez-vous pour annuler ou modifier votre RDV, ou contactez le professionnel. » |
| **Confirmation annulation** | « Votre rendez-vous a bien été annulé. » |
| **Confirmation modification** | « Votre rendez-vous a bien été modifié. Nouvelle date : [date], [heure]. » |

### 10.8 Correspondance parcours / analyse des besoins

| Parcours (ce document) | Besoins (Analyse des besoins) |
|------------------------|------------------------------|
| Réservation guest | UNC-01 à UNC-10, UNC-13, UNC-24, UNC-25 |
| Annulation (lien email) | UNC-11, UNC-12b, UNC-22 |
| Modification (lien email) | UNC-12, UNC-12b |
| Passerelles compte client | UNC-15, UNC-17, UNC-18 |
| Règles d’accès Façade | UNC-19, UNC-21, NFR-UNC-01 à NFR-UNC-11 |

### 10.9 Synthèse exécutive (Parcours Utilisateur non connecté)

Le public **Utilisateur non connecté** dispose d’un **parcours guest** complet : (1) **Accès** au lien de réservation du professionnel (ou widget) sans compte ; (2) **Choix du service** et **du créneau** (affichage des seules disponibilités) ; (3) **Formulaire** (nom, email, téléphone, remarque) sans création de compte ; (4) **Confirmation** à l’écran et par email/SMS, avec lien « Ajouter à mon agenda » et lien « Annuler ou modifier le RDV » ; (5) **Rappel** automatique au délai configuré par le pro ; (6) **Annulation et modification** via le lien dans l’email (token sécurisé, page dédiée, sans connexion). Les **règles d’accès** à la Façade publique limitent l’exposition aux seules disponibilités et au formulaire de réservation ; pas d’accès à l’agenda détaillé du pro ni aux noms des autres clients. Le **token** d’annulation/modification est unique, temporaire et non devinable ; des **quotas** (rate limiting) protègent les endpoints publics. Les **passerelles** « Créer un compte » et « Se connecter » sont optionnelles et non bloquantes. Les **critères d’acceptation** portent sur la performance (chargement < 3 s), la disponibilité des créneaux en temps réel, la sécurité du lien et le respect du RGPD. Ce document est aligné avec l’[analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md) du public Utilisateur non connecté et avec le parcours guest partagé du public [Clients](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md).

### 10.10 Historique des versions

- **v1.0 (2026-01-31)** — Création du document ; parcours détaillés (réservation guest, rappel, annulation, modification, passerelles) ; règles d’accès à la Façade publique (exposé / non exposé, token, quotas, responsive, accessibilité) ; capacités et livrables par étape ; critères d’acceptation ; cas limites ; écrans ; annexes (checklist MVP, dépendances techniques, récap parcours, index, points de vigilance, exemples libellés, correspondance analyse, synthèse exécutive).

### 10.11 Audience et document lié

**Audience** : Équipes produit, conception (UX/UI), développement, QA ; parties prenantes du service JayRDV. Document de référence pour les parcours et les règles d’accès du public Utilisateur non connecté.

**Document lié** : [Utilisateur non connecté — Analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md) — besoins fonctionnels et non fonctionnels, user stories, priorisation MoSCoW, cas limites.

**Références croisées** : [Clients — Parcours, capacités et livrables](../Clients/Clients%20-%20Parcours%20Capacites%20Livrables.md) (parcours guest partagé) ; [Professionnels — Parcours, capacités et livrables](../Professionnels/Professionnels%20-%20Parcours%20Capacites%20Livrables.md) (exposition des créneaux, lien pro, widget) ; [Document fondateur JayRDV](../../JayRDV%20-%20Document%20Fondateur.md) (vision, principes).

### 10.12 Mots-clés et validation

**Mots-clés** : JayRDV, Utilisateur non connecté, parcours guest, Façade publique, réservation sans compte, confirmation, rappels, lien annulation, lien modification, token sécurisé, règles d’accès, rate limiting, passerelles compte client, RGPD, livrables, critères d’acceptation, MVP.

**Validation** : Ce document a été rédigé dans le cadre de la construction de la structure par public du service JayRDV. Il constitue la référence des **parcours** et des **règles d’accès** pour le public Utilisateur non connecté et doit être maintenu à jour en cas d’évolution des parcours ou des règles. Les spécifications techniques (Opérateurs, Kits, API) sont à documenter dans les livrables associés.

**Document** : Utilisateur non connecté — Parcours et accès  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Référence pour le public Utilisateur non connecté

**Résumé des sections** : § 1 Profil du public — § 2 Parcours détaillés (réservation guest, rappel, annulation, modification, passerelles) — § 3 Règles d’accès à la Façade publique — § 4 Capacités et livrables par étape — § 5 Points de sortie — § 6 Critères d’acceptation — § 7 Cas limites — § 8 Écrans et zones fonctionnelles — § 9 Synthèse des livrables — § 10 Références et annexes.

**Note** : La présente documentation Parcours et accès est cohérente avec l’[analyse des besoins](./UtilisateurNonConnecte%20-%20Analyse%20des%20besoins.md) du public Utilisateur non connecté et avec le [benchmark des fonctionnalités des solutions de réservation en ligne](../../reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md). Le parcours guest est **partagé** avec le public Clients ; les livrables communs (page réservation, confirmation, rappels, lien annulation/modification) sont détaillés dans les deux documents pour traçabilité.

### 10.13 Critères d’acceptation par écran (référence QA)

| Écran | Critère d’acceptation principal |
|-------|---------------------------------|
| **Page de réservation** | Chargement < 3 s ; services et créneaux affichés ; formulaire guest (nom, email, téléphone) ; pas d’obligation de créer un compte ; confirmation à l’écran et par email ; lien « Ajouter à mon agenda » et « Annuler ou modifier » dans l’email. |
| **Page de confirmation** | Message « Votre RDV est confirmé » ; récapitulatif (date, heure, service, lieu) ; pas d’erreur si double clic sur « Confirmer ». Optionnel : lien « Créer un compte ». |
| **Page annulation (lien email)** | Récap RDV ; bouton « Annuler le RDV » ; confirmation avant annulation ; message « Votre RDV a bien été annulé » ; email de confirmation d’annulation ; créneau libéré côté pro ; lien expiré → message « Ce lien a expiré ». |
| **Page modification (lien email)** | Récap RDV ; liste des créneaux disponibles ; sélection nouveau créneau ; confirmation ; message « Votre RDV a bien été modifié » ; email de confirmation de modification ; ancien créneau libéré, nouveau créneau réservé. |

### 10.14 Matrice parcours / besoins (Utilisateur non connecté)

| Parcours (ce document) | Besoin (Analyse des besoins) | Priorité |
|-------------------------|-----------------------------|----------|
| Réservation guest (accès, service, créneau, formulaire, confirmation) | UNC-01 à UNC-05, UNC-07 à UNC-10, UNC-13, UNC-24, UNC-25 | Must |
| Rappels | UNC-08, UNC-22 | Must |
| Lien « Ajouter à mon agenda » | UNC-09 | Must |
| Annulation (lien email) | UNC-11, UNC-12b, UNC-22 | Must |
| Modification (lien email) | UNC-12, UNC-12b | Must |
| Limites Façade (pas d’agenda détaillé, pas de noms autres clients) | UNC-19, UNC-21 | Must |
| Parcours court, responsive, messages d’erreur | UNC-25, UNC-26 | Must |
| Passerelles compte client | UNC-15, UNC-17, UNC-18 | Should |
| Choix praticien (multi-praticiens) | UNC-06 | Should |
| Notification annulation pro | UNC-14 | Should |
| Paiement sans compte | UNC-23 | Should |
| NFR (performance, dispo, RGPD, lien sécurisé, utilisabilité) | NFR-UNC-01 à NFR-UNC-08, NFR-UNC-10, NFR-UNC-11 | Must |
| Paiement sécurisé | NFR-UNC-09 | Should |
| Accessibilité | UNC-27 | Should |

### 10.15 Métriques cibles (parcours Utilisateur non connecté)

| Métrique | Cible | Référence |
|----------|-------|-----------|
| **Temps moyen de réservation** | < 60 secondes | Parcours court (référence marché). |
| **Taux de complétion réservation** | > 70 % | Utilisateurs ayant cliqué sur « Confirmer » parmi ceux ayant commencé le parcours. |
| **Chargement page réservation** | < 3 secondes | NFR-UNC-01. |
| **Latence créneaux (temps réel)** | < 2 secondes | Après une réservation par un autre client, le créneau disparaît (NFR-UNC-02). |
| **Envoi confirmation (email/SMS)** | < 5 secondes | Après clic « Confirmer » (NFR-UNC-03). |
| **Réduction no-show (avec rappels)** | Division par 5 (référence) | Objectif rappels automatiques. |

### 10.16 Récapitulatif des livrables par priorité (MVP — Utilisateur non connecté)

| Priorité | Livrable | Critère d’acceptation principal |
|----------|----------|---------------------------------|
| **P0** | Page de réservation (lien pro ou widget) | Services et créneaux affichés ; formulaire guest ; pas d’obligation de compte ; confirmation à l’écran et par email ; lien « Ajouter à mon agenda » ; lien « Annuler ou modifier » dans l’email. |
| **P0** | Confirmation et rappels | Email/SMS à la réservation ; rappel 24h et/ou 2h avant ; modèle personnalisable par le pro. |
| **P0** | Annulation (lien email) | Lien sécurisé et temporaire ; page d’annulation sans connexion ; confirmation par email ; créneau libéré côté pro ; message lien expiré. |
| **P0** | Modification (lien email) | Page avec créneaux disponibles ; sélection nouveau créneau ; confirmation ; email de confirmation de modification. |
| **P0** | Règles d’accès Façade | Affichage des seules disponibilités ; pas d’agenda détaillé ni de noms d’autres clients ; token sécurisé ; rate limiting. |
| **P0** | Parcours court, responsive, messages | Maximum 4 à 5 étapes ; responsive ; message clair si créneau non disponible. |
| **P0** | NFR | Chargement < 3 s ; créneaux en temps réel ; RGPD ; lien annulation sécurisé ; utilisabilité. |
| **P1** | Passerelles compte client | Liens « Créer un compte » et « Se connecter » visibles et non bloquants. |
| **P1** | Choix praticien (multi-praticiens) | Liste déroulante ou onglets ; créneaux filtrés selon le choix. |
| **P1** | Notification annulation pro | Email/SMS si le pro annule ou modifie le RDV. |
| **P1** | Paiement sans compte | Page de paiement sécurisée après saisie des infos (si activé par le pro). |
| **P1** | Accessibilité | WCAG 2.1 niveau AA (contraste, focus, labels, messages d’erreur). |

### 10.17 Points de contrôle (QA — parcours Utilisateur non connecté)

1. **Réservation guest complète** (service → créneau → formulaire → confirmation) en < 60 s, sans création de compte.
2. **Email de confirmation** reçu avec lien « Ajouter à mon agenda » et lien « Annuler ou modifier ».
3. **Rappel** reçu au délai configuré (24h et/ou 2h avant).
4. **Lien annulation** fonctionne sans connexion ; après annulation, créneau libéré côté pro ; token expiré affiche message clair.
5. **Lien modification** fonctionne sans connexion ; affichage des créneaux disponibles ; après modification, email de confirmation ; token expiré affiche message clair.
6. **Passerelles** : Liens « Créer un compte » et « Se connecter » visibles ; pas d’obligation pour réserver.
7. **Responsive** : Page réservation et pages annulation/modification utilisables sur mobile.
8. **Message d’erreur** clair si créneau non disponible (« Ce créneau n’est plus disponible. Veuillez choisir un autre créneau. »).
9. **Façade publique** : Aucune donnée sensible exposée (agenda détaillé du pro, noms d’autres clients).
10. **RGPD** : Consentement et informations sur les données ; droits d’accès, rectification, effacement ; durée de conservation.

### 10.18 Références normatives

- **Glossaire Miyukini** : Façade publique gouvernée, Utilisateur externe, Mandat public d’accès, Opérateur, Service.
- **MIP v1** : Index, structure.
- **Nomenclature documentation Miyukini** : Format nommage, arborescence docs.
- **Document fondateur JayRDV** : Vision, raison d’être, principes.
- **Benchmark fonctionnalités solutions réservation en ligne** : Référence marché.

**Statut** : Document de référence pour le public Utilisateur non connecté. Aligné avec l’analyse des besoins et le benchmark marché. Maintenu à jour en cas d’évolution des parcours ou des règles d’accès. Spécifications techniques (Opérateurs, Kits, API) à documenter dans les livrables associés.

**Résumé (500+ lignes)** : Ce document décrit le **parcours** et les **règles d’accès** du public Utilisateur non connecté du service JayRDV. Il couvre la réservation guest (sans compte), la confirmation et les rappels, l’annulation et la modification via lien email (token sécurisé), les passerelles vers le compte client, les règles d’accès à la Façade publique (exposé / non exposé, token, quotas, responsive, accessibilité), les capacités et livrables par étape, les critères d’acceptation par parcours et par écran, les cas limites, les écrans et zones fonctionnelles, et les annexes (checklist MVP, dépendances techniques, récap parcours, index, points de vigilance, exemples libellés, correspondance analyse, synthèse exécutive, historique, audience, matrice parcours/besoins, métriques cibles, récap priorité MVP, points de contrôle QA, références normatives, statut). Objectif : document de référence produit pour le public Utilisateur non connecté, minimum 500 lignes.

### 10.19 Correspondance sections / analyse des besoins

| Section (ce document) | Section (Analyse des besoins) |
|----------------------|-------------------------------|
| § 1 Profil du public | § 1 Profil et personas |
| § 2 Parcours détaillés | § 4 Parcours détaillés |
| § 3 Règles d’accès Façade | § 2 Besoins fonctionnels (UNC-19, UNC-21), § 3 NFR |
| § 4 Capacités et livrables | § 2 Besoins fonctionnels (livrables par besoin) |
| § 6 Critères d’acceptation | § 2 Critères d’acceptation par besoin |
| § 7 Cas limites | § 9 Cas limites et règles métier |
| § 8 Écrans | § 2 Livrables (description des écrans) |
| § 10 Annexes | § 10 Glossaire, références, annexes |

Cette correspondance permet de tracer les parcours et les règles d’accès jusqu’aux besoins et aux user stories de l’analyse.

**Fin.**

---

*Fin du document — Utilisateur non connecté — Parcours et accès (JayRDV).*
