# Jay1Tribu — Spécification Fonctionnelle

## Contexte

Ce document décrit les **cas d'usage**, **parcours utilisateur** et **règles métier** du Service Jay1Tribu (messagerie P2P, tribus, salons, amis) dans l'écosystème Miyukini COG. Il complète le Document Conceptuel et le Document Fondateur en précisant le comportement attendu côté utilisateur et côté système.

## Portée / Scope

- **Applicable à :** Conception fonctionnelle, parcours, règles métier, critères d'acceptation.
- **Audience :** Équipes produit, UX, développeurs.
- **Statut :** Spécification fonctionnelle de référence.

### Hors périmètre

- Détail des protocoles de chiffrement et des schémas de base de données (voir Guide d'implémentation et Sécurité).
- Choix UI/UX précis (couleurs, composants) — principes généraux uniquement.

---

## 1. Acteurs et préconditions

| Acteur | Description |
|--------|-------------|
| **Utilisateur** | Personne utilisant un COG (Miyukini Central) et le service Jay1Tribu. |
| **COG** | Instance du Core-Orchestrated Governance Environment ; identifié de manière unique sur le MWS. |
| **Chef de tribu** | Utilisateur (ou COG) ayant créé une tribu ou ayant reçu le rôle d'administrateur ; peut gérer membres, rôles, salons. |

**Préconditions générales :**

- Le COG de l'utilisateur est configuré et peut se connecter au MWS (pour échanges en temps réel ou synchronisation à la reconnexion).
- Jay1Tribu est déployé et accessible depuis Miyukini Central.
- Pour les échanges en direct : le ou les COGs destinataires sont joignables (connectés au maillage ou via relais autorisé).

---

## 2. Salons de discussion

### 2.1 Création d'un salon direct

| Étape | Action utilisateur | Comportement système |
|-------|--------------------|----------------------|
| 1 | Depuis la liste d'amis ou la découverte, sélectionne un contact (ami ou COG découvert). | Système vérifie les permissions (Master Butler, StrongFather) et la frontière de confiance (Border Guard). |
| 2 | Demande « Démarrer une conversation ». | Création d'un salon direct (2 participants). WriteIntent KindMother pour chaque COG. |
| 3 | Saisit et envoie un premier message. | Message chiffré, transporté via MWS, archivé localement chez l'émetteur et le destinataire. |

**Règle :** Un salon direct est identifié de manière unique par la paire de COGs (ou profils). Pas de doublon pour la même paire.

### 2.2 Création d'un salon collectif (groupe)

| Étape | Action | Comportement |
|-------|--------|--------------|
| 1 | Depuis une tribu ou depuis « Nouveau salon », l'utilisateur crée un salon de groupe. | Vérification des droits (droit de créer un salon dans la tribu ou en dehors). |
| 2 | Invite un ou plusieurs membres (amis, membres de la tribu, ou COGs découverts). | Chaque invité reçoit une invitation (transport MWS). Acceptation = adhésion au salon. |
| 3 | Les messages sont envoyés à tous les membres du salon. | Chaque message est chiffré et acheminé vers chaque participant ; archivage local chez chaque participant. |

**Règle :** Les salons collectifs peuvent être liés à une tribu (salon de la tribu) ou indépendants (groupe ad hoc).

### 2.3 Envoi de messages texte

| Règle | Description |
|-------|-------------|
| **Envoi** | L'utilisateur saisit un message et l'envoie. Le message est chiffré, transmis via le MWS, et archivé localement par l'émetteur et chaque destinataire. |
| **Réception** | Si le destinataire est connecté : réception en temps réel. Sinon (tribu) : livraison à la reconnexion si l'émetteur est alors connecté. |
| **Historique** | Chaque participant ne voit que les messages qu'il a reçus ou envoyés ; l'historique est celui de sa base locale (KindMother). |

### 2.4 Envoi de fichiers et d'images

| Type | Règle |
|------|--------|
| **Restriction amis** | **Les transferts de fichier ne peuvent se faire qu'entre amis.** L'émetteur et chaque destinataire doivent être amis (relation confirmée dans la liste d'amis). |
| **Fichiers** | Tout type de fichier peut être envoyé entre amis. Transfert pair-à-pair (ou vers chaque pair), chiffré. Stockage local chez l'émetteur et les destinataires qui acceptent (politique de rétention locale). |
| **Images** | Traitées comme contenus chiffrés ; hébergement chez les participants ; pas de stockage central. Réservées aux échanges entre amis. |
| **Taille / quotas** | Limites définies par la politique de sécurité (WorrySentinel) et la capacité locale (KindMother) ; à préciser en implémentation. |

---

## 3. Tribus

### 3.1 Création d'une tribu

| Étape | Action | Comportement |
|-------|--------|--------------|
| 1 | L'utilisateur choisit « Créer une tribu ». | StrongFather et Master Butler autorisent ou refusent. |
| 2 | Saisit un nom et éventuellement une description, des paramètres (invitation ouverte / sur invitation). | KindMother : WriteIntent (création tribu, créateur = Chef de tribu). |
| 3 | La tribu apparaît dans la liste des tribus de l'utilisateur. | Le Chef de tribu peut créer des salons, inviter des membres, attribuer des rôles. |

### 3.2 Rejoindre une tribu

| Mode | Description |
|------|-------------|
| **Sur invitation** | Un membre (ou le Chef) envoie une invitation à un COG. L'invité accepte ou refuse. À l'acceptation, il devient membre de la tribu. |
| **Découverte** | Si la tribu est configurée comme « rejoignable par découverte » (règles à définir), l'utilisateur peut demander à rejoindre ; le Chef ou les administrateurs approuvent ou refusent. |

### 3.3 Rôles au sein d'une tribu

| Rôle | Permissions (vision conceptuelle) |
|------|-----------------------------------|
| **Chef de tribu** | Administration complète : créer/supprimer salons, inviter/exclure membres, attribuer des rôles, modifier paramètres de la tribu. |
| **Administrateur** | Délégation possible : gestion des membres et des salons, modération ; pas nécessairement la modification des rôles du Chef. |
| **Membre** | Participer aux salons, envoyer des messages, des fichiers et des images ; pas de gestion de la tribu sauf si rôle personnalisé l'y autorise. |
| **Rôles personnalisés** | Définis par le Chef de tribu (ex. « Modérateur », « Créateur de salons ») ; sémantique à préciser en implémentation. |

**Règle :** Les rôles sont gouvernés par Master Butler et StrongFather ; toute attribution passe par BondingBrother.

### 3.4 Partage à la reconnexion

| Règle | Description |
|-------|-------------|
| **Défaut** | Les membres d'une tribu qui n'ont pas encore reçu certains messages, fichiers ou images les reçoivent **à leur reconnexion**, **si l'émetteur est lui-même connecté** à ce moment-là. |
| **Paramétrage** | Un utilisateur peut restreindre ce qui est synchronisé à la reconnexion (ex. ne pas télécharger les fichiers lourds, limiter la rétention). Les options exactes seront précisées en implémentation. |
| **Hors ligne** | Si l'émetteur n'est pas connecté au moment de la reconnexion du destinataire, la livraison est différée jusqu'à une connexion simultanée ou un mécanisme de relais autorisé. |

---

## 4. Amis

### 4.1 Liste d'amis

| Action | Comportement |
|--------|--------------|
| **Ajouter un ami** | L'utilisateur envoie une demande d'ami à un COG (découvert ou identifié). L'autre partie accepte ou refuse. À l'acceptation, les deux apparaissent dans la liste d'amis de l'autre. |
| **Retirer un ami** | Suppression de la relation ; les salons directs existants peuvent être conservés (historique local) ou supprimés selon la politique et le choix utilisateur. |
| **Liste** | Affichage de la liste d'amis ; pour chaque ami : identifiant (ou pseudo résolu), statut de présence (en ligne / hors ligne) fourni par le MWS. |

### 4.2 Présence

| Règle | Description |
|-------|-------------|
| **Source** | La présence (en ligne / hors ligne) **s'appuie exclusivement sur le MWS**. Jay1Tribu ne duplique pas la logique de présence. |
| **Affichage** | Jay1Tribu consomme les informations de présence du MWS pour afficher le statut des amis et des membres de tribu. |
| **Mise à jour** | Dès que le MWS signale un changement de présence, l'interface peut mettre à jour l'affichage (sans lecture du contenu des messages). |

### 4.3 Discussion directe depuis un ami

L'utilisateur peut, depuis la liste d'amis, lancer une discussion directe en un clic. Comportement identique à la création d'un salon direct avec ce contact (ou ouverture du salon existant).

---

## 5. Parcours types

### 5.1 Premier contact avec un autre COG

1. Découverte du COG (via MWS ou invitation).
2. Envoi d'une demande d'ami (optionnel) ou création directe d'un salon direct.
3. Échange de messages / fichiers / images ; tout archivé localement chez les deux COGs.

### 5.2 Vie d'une tribu

1. Création de la tribu par un utilisateur (Chef de tribu).
2. Invitation de membres (ou adhésion par découverte si autorisé).
3. Création de salons au sein de la tribu.
4. Échanges dans les salons ; partage à la reconnexion pour les contenus non encore reçus (si émetteur connecté).
5. Gestion des rôles et des paramètres par le Chef (ou administrateurs).

### 5.3 Consultation et maîtrise des données

- L'utilisateur consulte ses tribus, salons et amis depuis Miyukini Central (interface Jay1Tribu).
- Les historiques sont ceux stockés localement (KindMother) ; pas de consultation d'archives centrales.
- Suppression locale : l'utilisateur peut supprimer des conversations ou des fichiers côté local ; les autres participants conservent leur propre copie. Politique de suppression à préciser (scope, révocabilité).

---

## 6. Règles métier transverses

| # | Règle | Description |
|---|-------|-------------|
| **RM-1** | Pas d'archives centrales | Aucun serveur ne conserve le contenu des conversations. |
| **RM-2** | Transit crypté | Tout message, fichier et image en transit entre COGs est crypté. |
| **RM-3** | Persistance via KindMother | Toute écriture locale (messages, fichiers, liste d'amis, paramètres tribu) passe par KindMother (WriteIntent). |
| **RM-4** | Présence via MWS | Liste d'amis et présence s'appuient sur le MWS ; pas de duplication de la logique de présence. |
| **RM-5** | Rôles gouvernés | Les rôles au sein d'une tribu sont attribués par le Chef de tribu (ou délégation) et gouvernés par Master Butler / StrongFather. |

---

## 7. Critères d'acceptation (résumé)

- Un utilisateur peut créer et rejoindre des salons directs et collectifs, envoyer et recevoir des messages, fichiers et images ; tout est archivé localement et transit crypté.
- Un utilisateur peut créer une tribu, inviter des membres, attribuer des rôles, créer des salons dans la tribu.
- Les contenus non encore reçus par un membre d'une tribu sont livrés à la reconnexion si l'émetteur est connecté (sauf paramétrage contraire).
- La liste d'amis et la présence sont cohérentes avec le MWS ; discussion directe rapide depuis la liste d'amis.
- Aucune donnée de conversation n'est stockée sur un serveur central ; conformité aux contraintes C-1 à C-8 (voir Contraintes et Invariants).

---

## 8. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts (tribus, salons, amis, rôles). |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes non négociables. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat d'intégration avec Central et Miou. |

---

**Document** : Jay1Tribu — Spécification Fonctionnelle  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Spécification fonctionnelle de référence
