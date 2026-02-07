# Miyukini Conceptual References — Équivalents Boutique, CMS, Réservation et SaaS

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités équivalentes à **WordPress**, **Elementor**, **WooCommerce**, **WooCommerce Bookings** et **WooCommerce Shipping**. Il vise à permettre la création d'**outils**, **opérateurs** et **services** Miyukini permettant de délivrer :

- **Boutiques en ligne** (catalogue, panier, checkout, paiement, livraison, commandes)
- **CMS** (contenu, pages, médias, révisions, commentaires)
- **Sites de réservation en ligne** (rendez-vous, créneaux, ressources, participants)
- **La plupart des SaaS** (multi-tenant, abonnements, facturation, tableaux de bord)

La sémantique et la terminologie officielles sont celles du [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md). Les Cores gouvernent et ne font jamais d'exécution ; les Opérateurs exécutent des rôles ; les Outils (Tools) exécutent des capacités atomiques sans décision métier.

**Références croisées :** [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md).

---

## Portée / Scope

**Ce document définit :**

- La cartographie détaillée **WordPress cœur** → Outils, Opérateurs, Services Miyukini
- La cartographie **Elementor** → Outils, Opérateurs, Services Miyukini
- La cartographie **WooCommerce** → Outils, Opérateurs, Services Miyukini
- La cartographie **WooCommerce Bookings** → Outils, Opérateurs, Services Miyukini
- La cartographie **WooCommerce Shipping** → Outils, Opérateurs, Services Miyukini
- Les **Kits d'outils (Toolkits)** et **Outils (Tools)** à créer ou à réutiliser
- Les **Opérateurs** (Domaine, Interface, Service) à déployer
- Les **Services** perçus par l'utilisateur (boutique, CMS, réservation, SaaS)
- Les **patterns SaaS** (abonnements, multi-tenant, facturation) en termes COG
- Les **Équipes d'Opérateurs** et **Contrats d'équipe** pour livrer ces services
- Les **Cores** impliqués et les flux de gouvernance

**Hors scope :**

- L'implémentation technique détaillée (code, schémas DB)
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools)
- La stratégie commerciale ou marketing des services

**Statut :** Document de référence normatif — source de vérité pour la conception des équivalents Miyukini.

---

## 1. Périmètre cible et objectifs

### 1.1 Équivalents logiciels ciblés

| Équivalent | Rôle | Objectif Miyukini |
|------------|------|--------------------|
| **WordPress** | CMS, utilisateurs, rôles, thèmes, extensions | Contenu (posts, pages, médias), identité (MiyuAuth), thèmes (MiyuWeb), Opérateurs déclarés (Master Butler) |
| **Elementor** | Page builder, widgets, theme builder | Éditeur visuel de pages/thèmes, layout et widgets (MiyuWeb + Tools layout/widgets) |
| **WooCommerce** | E-commerce (produits, panier, checkout, paiement, livraison, commandes) | Opérateur Commerce + Toolkits commerce (catalogue, panier, checkout, paiement, livraison, commandes) |
| **WooCommerce Bookings** | Réservations, créneaux, ressources, participants | Opérateur Réservation + Tools booking (créneaux, disponibilités, réservation, annulation) |
| **WooCommerce Shipping** | Étiquettes, transporteurs, tarifs, suivi | Tools shipping (étiquettes, tarifs transporteurs, suivi colis) intégrés au flux commandes |

### 1.2 Services utilisateur visés

| Service | Description | Opérateurs / Tools principaux |
|--------|-------------|-------------------------------|
| **Boutique en ligne** | Catalogue, panier, checkout, paiement, livraison, commandes | Opérateur Commerce, Toolkits commerce + shipping |
| **CMS** | Contenu éditorial, pages, médias, révisions, commentaires | Opérateur Contenu, Toolkit content.cms, MiyuWeb |
| **Site de réservation** | Rendez-vous, créneaux, ressources, participants, rappels | Opérateur Réservation, Toolkit booking |
| **SaaS générique** | Multi-tenant, abonnements, facturation, tableaux de bord | Opérateurs Interface + Domaine, Tools billing/subscription/tenant, MiyuAuth |

---

## 2. Cartographie WordPress cœur → Miyukini COG

### 2.1 Fonctionnalités WordPress et équivalents

| Fonctionnalité WordPress | Équivalent Miyukini | Type | Détail |
|--------------------------|---------------------|------|--------|
| **Posts / Pages** | Opérateur de Domaine **Contenu (CMS)** | Opérateur | Gère publication, révisions, planification. Données : KindMother. Tools : `tool.content.create`, `tool.content.update`, `tool.content.publish`, `tool.content.schedule`, `tool.content.revision.*` |
| **Média (upload, galeries)** | Tools médias + KindMother | Tools + Core | `tool.media.upload`, `tool.media.serve`, `tool.media.transform` (miniatures). Données médias : KindMother. Peut être regroupé dans `toolkit.content.media` ou `toolkit.content.cms`. |
| **Utilisateurs et rôles** | MiyuAuth + Master Butler + StrongFather | Toolkit + Cores | Pas de nouvel Opérateur « utilisateurs » : identité et rôle (citoyen / visiteur / externe) = MiyuAuth ; permissions = Master Butler ; décision ALLOW/DENY = StrongFather. |
| **Thèmes** | MiyuWeb + KindMother | Toolkit + Core | `tool.web.theme.resolve`, `tool.web.layout.render`. Définitions de thèmes (nom, couleurs, tokens) = données KindMother. |
| **Extensions (plugins)** | Opérateurs / Tools déclarés au Master Butler | Gouvernance | En Miyukini, pas de « plugin » libre : tout Opérateur ou Tool est déclaré au Master Butler ; collaboration = Contrat d'équipe + Mandat de Permission. |
| **Commentaires** | Opérateur de Domaine **Discussion** ou sous-domaine Contenu | Opérateur | Tools : `tool.content.comment.create`, `tool.content.comment.moderate`, `tool.content.comment.list`. Persistance : KindMother. |
| **Révisions / brouillons** | Ever Buddy (états de vie) + KindMother | Cores | Tools : `tool.content.revision.list`, `tool.content.revision.restore`, `tool.content.revision.compare`. Pas de décision métier dans le Tool. |

### 2.2 Opérateur de Domaine — Contenu (CMS)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère la publication et le cycle de vie des contenus (posts, pages, révisions, médias, commentaires). |
| **Service perçu** | « Site éditorial / CMS » |
| **Tools utilisés** | `tool.content.*`, `tool.media.*`, `tool.content.revision.*`, `tool.content.comment.*` ; affichage via MiyuWeb (données fournies dans le flux). |
| **Données** | KindMother (contenus, médias, révisions, commentaires). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; écriture = WriteIntent vers KindMother. |

### 2.3 Outils WordPress cœur (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.content.create` | Crée un brouillon de contenu à partir de données fournies ; ne décide pas de la politique de publication | 1–2 |
| `tool.content.update` | Met à jour un contenu existant à partir de données fournies | 1–2 |
| `tool.content.publish` | Marque un contenu comme publié (exécution seule ; décision = StrongFather) | 2 |
| `tool.content.schedule` | Planifie une publication à une date/heure fournie | 1–2 |
| `tool.content.revision.list` | Liste les révisions d'un contenu | 0–1 |
| `tool.content.revision.restore` | Restaure une révision donnée (exécution ; décision = StrongFather) | 2 |
| `tool.content.revision.compare` | Compare deux révisions (lecture seule) | 0–1 |
| `tool.content.comment.create` | Crée un commentaire à partir de données fournies | 0–2 |
| `tool.content.comment.moderate` | Applique une action de modération (approuver, rejeter) ; décision = StrongFather | 2 |
| `tool.content.comment.list` | Liste les commentaires d'un contenu (filtres fournis dans le flux) | 0–1 |
| `tool.media.upload` | Enregistre un média à partir du flux ; persistance = KindMother | 1–2 |
| `tool.media.serve` | Sert un média (stream ou métadonnées) à partir de données fournies | 0–1 |
| `tool.media.transform` | Produit une variante (miniature, recadrage) à partir de données fournies | 0–1 |

---

## 3. Cartographie Elementor → Miyukini COG

### 3.1 Fonctionnalités Elementor et équivalents

| Fonctionnalité Elementor | Équivalent Miyukini | Type | Détail |
|-------------------------|---------------------|------|--------|
| **Page builder (drag & drop)** | Opérateur d'Interface **Éditeur de pages** | Opérateur | Utilise MiyuWeb (`tool.web.layout.render`, `tool.web.html.render`, `tool.web.theme.resolve`) et `tool.web.layout.apply`, `tool.web.input.capture` pour le glisser-déposer. Données de layout fournies dans le flux (KindMother en amont). |
| **Widgets** | Tools de rendu de blocs | Tools | `tool.web.widget.<type>.render` (texte, image, bouton, grille, etc.). Chaque widget = Tool atomique ; pas de décision métier. Composition possible dans `toolkit.web.widgets`. |
| **Bibliothèque de templates** | Données KindMother + MiyuWeb | Core + Toolkit | Templates = structures (blocs, zones) persistées par KindMother. Résolution et rendu = `tool.web.layout.render`, `tool.web.html.render`. |
| **Theme Builder (header, footer, single, archive, 404)** | Données KindMother + MiyuWeb | Core + Toolkit | Parties de thème = données ; `tool.web.theme.resolve`, `tool.web.layout.render`. Opérateur d'Interface « Éditeur de thème » édite ces définitions via gouvernance. |
| **Design (couleurs, typo, marges)** | Tokens / thème dans KindMother + `tool.web.theme.resolve` | Core + Tool | Design system = données ; Tool résout le thème applicable. |
| **Pages WooCommerce dans Elementor** | Zones réservées au Service commerce ; contenu fourni par Opérateur Commerce | Collaboration | L'Opérateur Interface rend des layouts ; les zones « boutique » sont alimentées par l'Opérateur de Domaine Commerce (flux gouverné). |

### 3.2 Opérateur d'Interface — Éditeur de pages / thème

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur d'Interface |
| **Rôle** | Expose l'édition visuelle de pages et de parties de thème (header, footer, single, archive) de façon utilisable. |
| **Service perçu** | « Création de pages et thèmes sans code » |
| **Tools utilisés** | MiyuWeb (layout, HTML, thème, formulaire, événements, input) ; `tool.web.layout.apply`, `tool.web.widget.*.render` si déclarés. |
| **Données** | Fournies dans le flux (templates, assets peuvent provenir de KindMother en amont). MiyuWeb ne lit pas la base directement. |
| **Gouvernance** | Idem flux standard Opérateur → BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. |

### 3.3 Outils Elementor (compléments MiyuWeb)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.web.layout.apply` | Applique une modification de layout (structure) à partir de données fournies ; exécution seule | 1–2 |
| `tool.web.widget.text.render` | Rend un bloc texte à partir de données fournies | 0–1 |
| `tool.web.widget.image.render` | Rend un bloc image à partir de données fournies | 0–1 |
| `tool.web.widget.button.render` | Rend un bloc bouton à partir de données fournies | 0–1 |
| `tool.web.widget.grid.render` | Rend une grille de blocs à partir de données fournies | 0–1 |
| `tool.web.widget.container.render` | Rend un conteneur (section/colonnes) à partir de données fournies | 0–1 |
| `tool.web.template.resolve` | Résout un template (structure) par identifiant ; données fournies dans le flux | 0–1 |

*Les widgets peuvent être regroupés dans le **toolkit.web.widgets** (composition d'outils de rendu de blocs), déclaré au Master Butler.*

---

## 4. Cartographie WooCommerce → Miyukini COG

### 4.1 Fonctionnalités WooCommerce et équivalents

| Fonctionnalité WooCommerce | Équivalent Miyukini | Type | Détail |
|----------------------------|---------------------|------|--------|
| **Catalogue produits** | Opérateur de Domaine **Commerce** + Tools produit | Opérateur + Tools | Données produits : KindMother. Tools : `tool.commerce.product.list`, `tool.commerce.product.resolve`, `tool.commerce.product.variations` ; création/modification = WriteIntent + StrongFather. |
| **Panier** | État métier (KindMother ou flux) + Tools panier | Core / Tools | `tool.commerce.cart.add`, `tool.commerce.cart.update`, `tool.commerce.cart.remove`, `tool.commerce.cart.compute` (totaux, taxes). Décision promo/refus = StrongFather. |
| **Checkout** | Flux gouverné + Tools checkout | Tools + Cores | BondingBrother → Master Butler → WorrySentinel → StrongFather. Tools : `tool.commerce.checkout.validate`, `tool.commerce.checkout.submit`. Création commande = WriteIntent KindMother. |
| **Paiements** | Tools paiement (exécution seule) | Tools | `tool.commerce.payment.capture`, `tool.commerce.payment.refund`, `tool.commerce.payment.status`. Autorisation et niveau de confiance = StrongFather + WorrySentinel. Gateways = implémentations des Tools. |
| **Livraison (zones, tarifs)** | Tools livraison + données KindMother | Tools + Core | `tool.commerce.shipping.rate`, `tool.commerce.shipping.zones.resolve`. Règles et zones = KindMother. Voir section 6 pour étiquettes et transporteurs. |
| **Commandes** | Entités KindMother + Tools commande | Core + Tools | `tool.commerce.order.create`, `tool.commerce.order.update`, `tool.commerce.order.status`, `tool.commerce.order.list`. Workflow (validée, expédiée, remboursée) = StrongFather + Ever Buddy. |
| **Dashboard marchand** | Opérateur d'Interface **Admin Commerce** | Opérateur | Expose les Services du Opérateur Commerce (catalogue, commandes, rapports) ; utilise les mêmes Tools en lecture/écriture via gouvernance. |

### 4.2 Opérateur de Domaine — Commerce (Catalogue / Boutique)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère le catalogue, le panier, le checkout et les commandes pour délivrer le Service boutique. |
| **Service perçu** | « Boutique en ligne » |
| **Tools utilisés** | `tool.commerce.product.*`, `tool.commerce.cart.*`, `tool.commerce.checkout.*`, `tool.commerce.payment.*`, `tool.commerce.shipping.*`, `tool.commerce.order.*`. |
| **Données** | KindMother (produits, paniers, commandes, règles livraison/paiement). |
| **Gouvernance** | Flux standard ; toute écriture = WriteIntent ; toute décision = StrongFather. |

### 4.3 Opérateur d'Interface — Admin Commerce

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur d'Interface |
| **Rôle** | Expose la gestion du catalogue et des commandes de façon utilisable (dashboard marchand). |
| **Service perçu** | « Gestion de la boutique » |
| **Tools utilisés** | Mêmes Tools commerce en lecture/écriture ; permissions et Mandat = StrongFather / Master Butler. |

### 4.4 Outils WooCommerce (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.commerce.product.list` | Liste des produits selon filtres fournis | 0–1 |
| `tool.commerce.product.resolve` | Résout un produit par identifiant | 0–1 |
| `tool.commerce.product.variations` | Liste les variations d'un produit | 0–1 |
| `tool.commerce.product.create` | Crée un produit (exécution ; décision = StrongFather) | 2 |
| `tool.commerce.product.update` | Met à jour un produit | 2 |
| `tool.commerce.cart.add` | Ajoute une ligne au panier à partir de données fournies | 0–2 |
| `tool.commerce.cart.update` | Met à jour une ligne du panier | 0–2 |
| `tool.commerce.cart.remove` | Supprime une ligne du panier | 0–2 |
| `tool.commerce.cart.compute` | Calcule totaux, taxes, livraison du panier (règles fournies) | 0–1 |
| `tool.commerce.checkout.validate` | Valide les données de checkout (structure, champs) | 1–2 |
| `tool.commerce.checkout.submit` | Soumet le checkout et crée la commande (WriteIntent KindMother) | 2 |
| `tool.commerce.payment.capture` | Capture un paiement (exécution ; autorisation = StrongFather) | 3 |
| `tool.commerce.payment.refund` | Rembourse un paiement | 3 |
| `tool.commerce.payment.status` | Retourne le statut d'un paiement | 1–2 |
| `tool.commerce.shipping.rate` | Calcule le tarif de livraison pour un panier/zone fourni | 0–1 |
| `tool.commerce.shipping.zones.resolve` | Résout les zones de livraison applicables | 0–1 |
| `tool.commerce.order.create` | Crée une commande (exécution ; souvent appelé par checkout.submit) | 2 |
| `tool.commerce.order.update` | Met à jour une commande (statut, champs) | 2 |
| `tool.commerce.order.status` | Retourne le statut d'une commande | 0–1 |
| `tool.commerce.order.list` | Liste les commandes selon filtres fournis | 1–2 |

---

## 5. Cartographie WooCommerce Bookings → Miyukini COG

### 5.1 Fonctionnalités Bookings et équivalents

| Fonctionnalité Bookings | Équivalent Miyukini | Type | Détail |
|-------------------------|---------------------|------|--------|
| **Créneaux / plages horaires** | Données KindMother + Tools booking | Core + Tools | Règles de disponibilité (fixes, flexibles) = données ; `tool.booking.slots.list`, `tool.booking.slots.resolve` pour calcul des créneaux disponibles. |
| **Réservation (single / multi-day)** | Tools réservation + WriteIntent | Tools + Core | `tool.booking.create`, `tool.booking.update`, `tool.booking.cancel`. Décision (autoriser, refuser) = StrongFather. |
| **Participants (min/max)** | Données produit/ressource + `tool.booking.compute` | KindMother + Tool | Règles participants = données ; Tool calcule disponibilité restante. |
| **Tarification (groupes, week-end, early)** | Règles KindMother + `tool.booking.price.compute` | Core + Tool | Tool exécute le calcul à partir de règles fournies. |
| **Fuseau horaire / disponibilité** | Données + `tool.booking.slots.list` | Core + Tool | Affichage créneaux dans le fuseau utilisateur ; règles de disponibilité = KindMother. |
| **Confirmation / rappels / annulation** | Workflow StrongFather + Tools notification (hors scope détaillé ici) ou état commande/booking | Cores + état | Confirmation = décision StrongFather ; rappels = Opérateur Automatisation ou état Ever Buddy ; annulation = `tool.booking.cancel` + StrongFather. |
| **Ressources (salles, équipements)** | Entités KindMother + `tool.booking.resource.resolve` | Core + Tool | Ressources = données ; Tool résout disponibilité par ressource. |

### 5.2 Opérateur de Domaine — Réservation (Bookings)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère les créneaux, disponibilités, réservations et ressources pour délivrer le Service réservation (rendez-vous, locations, événements). |
| **Service perçu** | « Réservation en ligne » (rendez-vous, créneaux, ressources) |
| **Tools utilisés** | `tool.booking.slots.*`, `tool.booking.create`, `tool.booking.update`, `tool.booking.cancel`, `tool.booking.resource.*`, `tool.booking.price.compute`. |
| **Données** | KindMother (règles de créneaux, ressources, réservations, tarifs). |
| **Gouvernance** | Flux standard ; création/modification réservation = WriteIntent + StrongFather. |

### 5.3 Outils Bookings (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.booking.slots.list` | Liste les créneaux disponibles pour un contexte (ressource, date, durée) fourni | 0–1 |
| `tool.booking.slots.resolve` | Résout un créneau par identifiant | 0–1 |
| `tool.booking.create` | Crée une réservation à partir de données fournies ; WriteIntent KindMother | 1–2 |
| `tool.booking.update` | Met à jour une réservation (déplacement, prolongation) | 1–2 |
| `tool.booking.cancel` | Annule une réservation ; décision politique = StrongFather | 1–2 |
| `tool.booking.resource.resolve` | Résout une ressource (salle, équipement) et ses contraintes | 0–1 |
| `tool.booking.resource.availability` | Retourne la disponibilité d'une ressource sur une plage donnée | 0–1 |
| `tool.booking.price.compute` | Calcule le prix d'une réservation (règles fournies) | 0–1 |
| `tool.booking.participants.compute` | Calcule places restantes / participants pour un créneau | 0–1 |

---

## 6. Cartographie WooCommerce Shipping (étiquettes, transporteurs) → Miyukini COG

### 6.1 Fonctionnalités Shipping et équivalents

| Fonctionnalité Shipping | Équivalent Miyukini | Type | Détail |
|-------------------------|---------------------|------|--------|
| **Étiquettes d'expédition** | Tools shipping | Tools | `tool.commerce.shipping.label.create`, `tool.commerce.shipping.label.print` (données fournies ; intégration transporteur = implémentation du Tool). |
| **Tarifs transporteurs (USPS, UPS, DHL, etc.)** | Déjà partiellement couvert par `tool.commerce.shipping.rate` ; extension par transporteur | Tools | `tool.commerce.shipping.rate` peut appeler des règles par transporteur ; données tarifs = KindMother ou flux. |
| **Comparaison de tarifs** | Tool ou composition | Tool | `tool.commerce.shipping.rates.compare` : retourne les tarifs de plusieurs transporteurs pour un colis donné. |
| **Suivi colis** | Tool lecture seule | Tool | `tool.commerce.shipping.tracking.get` : retourne le statut de suivi à partir d'un identifiant fourni. |
| **Enlèvement / pickup** | Règles KindMother + flux commande | Core | Politique d'enlèvement = données ; pas de Tool dédié obligatoire (peut être une option dans `tool.commerce.shipping.zones.resolve` ou commande). |
| **Envoi partiel (split shipments)** | Workflow commande (plusieurs expéditions) ; état KindMother | Core + état | Une commande peut avoir plusieurs « expéditions » ; Tools : `tool.commerce.shipping.shipment.create`, `tool.commerce.shipping.shipment.list`. |

### 6.2 Outils Shipping (compléments commerce)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.commerce.shipping.label.create` | Crée une étiquette d'expédition pour une commande/colis fourni | 2 |
| `tool.commerce.shipping.label.print` | Produit les données d'impression d'une étiquette (exécution seule) | 2 |
| `tool.commerce.shipping.rates.compare` | Compare les tarifs de plusieurs transporteurs pour un colis donné | 0–1 |
| `tool.commerce.shipping.tracking.get` | Retourne le statut de suivi d'un envoi (identifiant fourni) | 0–1 |
| `tool.commerce.shipping.shipment.create` | Crée une expédition (colis) pour une commande ; WriteIntent si état commande géré | 2 |
| `tool.commerce.shipping.shipment.list` | Liste les expéditions d'une commande | 1–2 |

---

## 7. Synthèse — Kits d'outils (Toolkits)

Les Kits d'outils sont des **compositions officielles d'Outils**, déclarées au Master Butler, sans logique métier. Ils orchestrent des Tools existants pour efficience et cohérence.

| ToolkitId | Domaine | Composition (résumé) | Usage principal |
|-----------|---------|----------------------|------------------|
| `toolkit.content.cms` | content | tool.content.*, tool.media.*, tool.content.revision.*, tool.content.comment.* | CMS (posts, pages, médias, révisions, commentaires) |
| `toolkit.content.media` | content | tool.media.upload, tool.media.serve, tool.media.transform | Gestion médias seule |
| `toolkit.web.miyuweb` | web | (existant) layout, HTML, thème, script, asset, form, event, input | Affichage web |
| `toolkit.web.widgets` | web | tool.web.widget.*.render, tool.web.layout.apply, tool.web.template.resolve | Page builder / widgets |
| `toolkit.commerce.store` | commerce | tool.commerce.product.*, cart.*, checkout.*, payment.*, shipping.*, order.* | Boutique complète |
| `toolkit.commerce.shipping` | commerce | tool.commerce.shipping.rate, zones.resolve, label.*, rates.compare, tracking.get, shipment.* | Livraison et étiquettes |
| `toolkit.booking.reservations` | booking | tool.booking.slots.*, create, update, cancel, resource.*, price.compute, participants.compute | Réservations et créneaux |
| `toolkit.identity.miyauth` | identity | (existant) resolve, attest, verify, role | Identité et rôles |
| `toolkit.billing.saas` | billing | tool.billing.subscription.*, invoice.*, payment.record, tenant.resolve | Facturation, abonnements, multi-tenant |

**Invariant :** Chaque Toolkit contient au moins deux Tools. Les Toolkits sont validés par Ever Buddy (cycle de vie, versions) et déclarés au Master Butler.

---

## 8. Synthèse — Outils (Tools) catalogue complet

Format ToolId : `tool.<domain>.<action>` ou `tool.<domain>.<sous-domaine>.<action>` (conforme Master Butler).

### 8.1 Contenu (content)

| ToolId | Action courte |
|--------|----------------|
| `tool.content.create` | Crée un brouillon |
| `tool.content.update` | Met à jour un contenu |
| `tool.content.publish` | Marque comme publié |
| `tool.content.schedule` | Planifie publication |
| `tool.content.revision.list` | Liste révisions |
| `tool.content.revision.restore` | Restaure révision |
| `tool.content.revision.compare` | Compare révisions |
| `tool.content.comment.create` | Crée commentaire |
| `tool.content.comment.moderate` | Modère commentaire |
| `tool.content.comment.list` | Liste commentaires |
| `tool.media.upload` | Enregistre média |
| `tool.media.serve` | Sert média |
| `tool.media.transform` | Variante média (miniature, etc.) |

### 8.2 Web (web) — existant MiyuWeb + extensions

| ToolId | Action courte |
|--------|----------------|
| `tool.web.html.render` | Rendu HTML |
| `tool.web.layout.render` | Rendu layout |
| `tool.web.theme.resolve` | Résolution thème |
| `tool.web.script.execute` | Exécution script |
| `tool.web.script.compile` | Compilation script |
| `tool.web.asset.serve` | Service asset |
| `tool.web.form.validate` | Validation formulaire |
| `tool.web.event.dispatch` | Dispatch événement |
| `tool.web.input.capture` | Capture entrée |
| `tool.web.layout.apply` | Applique modification layout |
| `tool.web.widget.*.render` | Rendu widget (texte, image, bouton, grille, container) |
| `tool.web.template.resolve` | Résolution template |

### 8.3 Commerce (commerce)

| ToolId | Action courte |
|--------|----------------|
| `tool.commerce.product.list` | Liste produits |
| `tool.commerce.product.resolve` | Résout produit |
| `tool.commerce.product.variations` | Variations produit |
| `tool.commerce.product.create` | Crée produit |
| `tool.commerce.product.update` | Met à jour produit |
| `tool.commerce.cart.add` | Ajoute au panier |
| `tool.commerce.cart.update` | Met à jour panier |
| `tool.commerce.cart.remove` | Supprime du panier |
| `tool.commerce.cart.compute` | Calcule totaux panier |
| `tool.commerce.checkout.validate` | Valide checkout |
| `tool.commerce.checkout.submit` | Soumet checkout |
| `tool.commerce.payment.capture` | Capture paiement |
| `tool.commerce.payment.refund` | Rembourse |
| `tool.commerce.payment.status` | Statut paiement |
| `tool.commerce.shipping.rate` | Tarif livraison |
| `tool.commerce.shipping.zones.resolve` | Zones livraison |
| `tool.commerce.shipping.label.create` | Crée étiquette |
| `tool.commerce.shipping.label.print` | Impression étiquette |
| `tool.commerce.shipping.rates.compare` | Compare tarifs transporteurs |
| `tool.commerce.shipping.tracking.get` | Suivi colis |
| `tool.commerce.shipping.shipment.create` | Crée expédition |
| `tool.commerce.shipping.shipment.list` | Liste expéditions |
| `tool.commerce.order.create` | Crée commande |
| `tool.commerce.order.update` | Met à jour commande |
| `tool.commerce.order.status` | Statut commande |
| `tool.commerce.order.list` | Liste commandes |

### 8.4 Réservation (booking)

| ToolId | Action courte |
|--------|----------------|
| `tool.booking.slots.list` | Liste créneaux disponibles |
| `tool.booking.slots.resolve` | Résout créneau |
| `tool.booking.create` | Crée réservation |
| `tool.booking.update` | Met à jour réservation |
| `tool.booking.cancel` | Annule réservation |
| `tool.booking.resource.resolve` | Résout ressource |
| `tool.booking.resource.availability` | Disponibilité ressource |
| `tool.booking.price.compute` | Calcule prix réservation |
| `tool.booking.participants.compute` | Places restantes / participants |

### 8.5 Facturation et SaaS (billing)

| ToolId | Action courte |
|--------|----------------|
| `tool.billing.subscription.create` | Crée une souscription à partir de données fournies ; WriteIntent ; décision = StrongFather |
| `tool.billing.subscription.update` | Met à jour une souscription (renouvellement, changement offre) |
| `tool.billing.subscription.cancel` | Annule / résilie une souscription |
| `tool.billing.subscription.status` | Retourne le statut d'une souscription |
| `tool.billing.invoice.generate` | Génère une facture selon règles fournies |
| `tool.billing.invoice.list` | Liste les factures selon filtres fournis |
| `tool.billing.payment.record` | Enregistre un paiement reçu (exécution ; décision = StrongFather) |
| `tool.billing.tenant.resolve` | Résout le contexte tenant (identifiant, périmètre) pour une requête ; isolation multi-tenant |

*Ces Tools peuvent être regroupés dans un **toolkit.billing.saas** (composition facturation + abonnements + tenant), déclaré au Master Butler.*

---

## 9. Synthèse — Opérateurs catalogue complet

| Opérateur | Type | Service perçu | Tools principaux |
|-----------|------|----------------|------------------|
| **Contenu (CMS)** | Domaine | Site éditorial / CMS | tool.content.*, tool.media.*, tool.content.revision.*, tool.content.comment.* ; MiyuWeb pour rendu |
| **Discussion** | Domaine (ou sous-domaine Contenu) | Commentaires / modération | tool.content.comment.* |
| **Éditeur de pages / thème** | Interface | Création de pages et thèmes sans code | MiyuWeb, tool.web.layout.apply, tool.web.widget.*.render, tool.web.template.resolve |
| **Commerce (Catalogue / Boutique)** | Domaine | Boutique en ligne | tool.commerce.product.*, cart.*, checkout.*, payment.*, shipping.*, order.* |
| **Admin Commerce** | Interface | Gestion de la boutique | Mêmes Tools commerce (lecture/écriture gouvernée) |
| **Réservation (Bookings)** | Domaine | Réservation en ligne (rendez-vous, créneaux, ressources) | tool.booking.* |
| **Admin Réservation** | Interface (optionnel) | Gestion des réservations et ressources | tool.booking.* |
| **Facturation (Billing)** | Domaine (optionnel) | Facturation et abonnements SaaS | tool.billing.* |
| **Dashboard** | Interface | Tableaux de bord agrégés | Données en flux + MiyuWeb (rendu) |

*Identité et rôles : MiyuAuth + Cores (Master Butler, StrongFather) — pas d'Opérateur « utilisateurs » dédié.*

---

## 10. Synthèse — Services utilisateur

Un **Service** est ce que l'utilisateur perçoit. Il peut être porté par un seul Opérateur ou par une **Équipe d'Opérateurs**.

| Service | Opérateur(s) / Équipe | Toolkits impliqués |
|---------|------------------------|--------------------|
| **Boutique en ligne** | Commerce + Admin Commerce (optionnel) | toolkit.commerce.store, toolkit.commerce.shipping |
| **CMS / site éditorial** | Contenu + Éditeur de pages (optionnel) | toolkit.content.cms, MiyuWeb, toolkit.web.widgets |
| **Site de réservation** | Réservation + (Commerce si paiement) | toolkit.booking.reservations, éventuellement toolkit.commerce.store |
| **SaaS multi-tenant** | Plusieurs Opérateurs (Interface + Domaine) ; tenant = périmètre KindMother / environnement | MiyuAuth, toolkit.content.cms, toolkit.billing.saas, toolkits commerce/booking selon cas |
| **Tableau de bord / admin** | Opérateur(s) d'Interface (Admin Commerce, Admin Réservation, etc.) | Selon domaine (commerce, booking, content) |

---

## 11. Équipes d'Opérateurs et Contrats d'équipe

Pour délivrer un **Service** combiné (ex. site avec CMS + boutique + réservation), on constitue une **Équipe d'Opérateurs** liée par un **Contrat d'équipe** (membres, flux autorisés, types d'échanges, niveau de validation). L'équipe n'existe opérationnellement que sous un **Mandat de Permission** émis par StrongFather.

### 11.1 Exemple — Équipe « Site complet (CMS + Boutique + Réservation) »

| Membre | Rôle dans l'équipe |
|--------|---------------------|
| Opérateur Contenu (CMS) | Fournit contenus et médias ; expose pages éditoriales |
| Opérateur Éditeur de pages | Fournit layouts et thèmes ; peut inclure zones commerce et réservation |
| Opérateur Commerce | Fournit catalogue, panier, checkout, commandes ; alimente les zones « boutique » |
| Opérateur Réservation | Fournit créneaux et réservations ; alimente les zones « réservation » |
| Opérateur Admin Commerce | Gestion catalogue et commandes (flux interne) |
| Opérateur Admin Réservation (optionnel) | Gestion créneaux et réservations (flux interne) |

**Flux autorisés (exemple) :** Éditeur de pages → Contenu (lecture contenus) ; Éditeur de pages → Commerce (données catalogue pour rendu) ; Éditeur de pages → Réservation (créneaux pour rendu) ; Admin Commerce → Commerce ; Admin Réservation → Réservation. Pas de communication directe Contenu ↔ Commerce sans passer par BondingBrother et Mandat.

**Contrat d'équipe :** Définit statiquement les membres, les flux, les types de données échangeables, les conditions préalables et le niveau de validation requis. Validé par StrongFather.

---

## 12. Patterns SaaS en termes Miyukini COG

### 12.1 Multi-tenant

- **Tenant** = périmètre d'isolation (données, identité). Peut être modélisé comme un **environnement COG** distinct ou comme un périmètre de données (KindMother) au sein d'un même environnement.
- **Isolation** : Border Guard (frontières), KindMother (données par tenant), Master Butler (permissions par tenant). Aucun Opérateur ne doit accéder aux données d'un autre tenant sans gouvernance explicite (Visite inter-COG si environnements distincts).

### 12.2 Abonnements (subscriptions)

- **Offre d'abonnement** = donnée (KindMother) ; **souscription** = WriteIntent + décision StrongFather.
- **Tools** : `tool.billing.subscription.create`, `tool.billing.subscription.update`, `tool.billing.subscription.cancel`, `tool.billing.subscription.status`. Pas de décision métier dans le Tool ; décision (autoriser, renouveler, résilier) = StrongFather.
- **Facturation récurrente** : `tool.billing.invoice.generate`, `tool.billing.invoice.list` ; règles de facturation = KindMother.
- **Kit d'outils** : `toolkit.billing.saas` agrège les Tools subscription + invoice + payment.record + tenant.resolve.

### 12.3 Facturation (billing)

- **Tools** : `tool.billing.invoice.generate`, `tool.billing.invoice.list`, `tool.billing.payment.record` (enregistrer un paiement reçu). Autorité sur les montants et la politique = StrongFather ; persistance = KindMother.
- **Opérateur de Domaine « Facturation »** (optionnel) : gère offres, factures et paiements pour le Service SaaS ; utilise les Tools billing + gouvernance.
- **Multi-tenant** : `tool.billing.tenant.resolve` permet de résoudre le contexte tenant (isolation des données) ; les requêtes sont ensuite filtrées par ce périmètre (KindMother / Master Butler).

### 12.4 Tableaux de bord (dashboards)

- **Opérateur d'Interface** « Dashboard » : agrège des données fournies dans le flux (provenant d'autres Opérateurs ou de KindMother via Mandat). Utilise MiyuWeb pour le rendu (graphiques, tableaux = données + layout). Aucune décision métier ; affichage uniquement.

---

## 13. Cores impliqués et flux de gouvernance

Les Cores **ne font jamais d'exécution** ; ils gouvernent, décident ou observent.

| Core | Rôle dans le périmètre Boutique / CMS / Réservation / SaaS |
|------|------------------------------------------------------------|
| **KindMother** | Autorité sur toutes les données : contenus, médias, produits, paniers, commandes, réservations, ressources, règles livraison/paiement, thèmes, templates. WriteIntent pour toute écriture. |
| **StrongFather** | Décision finale ALLOW/DENY : publication, checkout, paiement, réservation, annulation, modération. Émission et révocation des Mandats de Permission. Validation des Contrats d'équipe. |
| **Master Butler** | Déclaration des Tools et Toolkits ; permissions et capabilities. Catalogue des capacités (commerce, content, booking, billing). |
| **BondingBrother** | Médiation des intentions (éditeur, acheteur, réservant, admin) ; traduction vers les Cores et les Opérateurs. |
| **WorrySentinel** | Niveau de sécurité (paiement, données sensibles) ; blocage si menace ou état dégradé. |
| **Caring Nanny** | État système (HEALTHY, DEGRADED, etc.) ; blocage des Tools si environnement dégradé. |
| **Ever Buddy** | Cycle de vie : révisions contenu, états de commande, dépréciation Tools/Toolkits, compatibilité versions. |
| **Border Guard** | Frontières et niveaux de confiance ; multi-tenant et inter-COG si applicable. |
| **TAMR** | Points d'intervention humaine (modération, arbitrage, validation manuelle) si définis. |

**Flux générique :** Opérateur → BondingBrother → Master Butler (existence Tool, permissions) → WorrySentinel (niveau sécurité) → Caring Nanny (état système) → StrongFather (ALLOW/DENY) → Exécution Tool ; toute persistance passe par WriteIntent KindMother.

---

## 14. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Opérateurs et Terminologie | [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| Mandats et Équipes Opérateurs | [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| Pyramide Architecture Complète | [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Objectif Final | [Miyukini Conceptual References - Objectif Final](./Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) |
| MiyuAuth Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../tools/MiyuAuth/MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../tools/MiyuWeb/MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](./Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Master Butler - Tool Governance Contract | (voir core/MasterButler/contracts/tools) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — Équivalents Boutique, CMS, Réservation et SaaS
