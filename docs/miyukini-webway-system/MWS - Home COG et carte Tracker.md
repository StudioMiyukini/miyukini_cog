# MWS — Home COG et carte Tracker

## Résumé

Lorsqu’un COG **s’annonce sur le Tracker**, il met **automatiquement en exposition** une **page web Home** de présentation, servie par le **mini serveur web** intégré à Central (natif). Cette page est le **point d’entrée** pour les utilisateurs externes sur le service web du COG. Le **layout** de la Home est **commun à tous les COGs** ; seul le contenu (identité du COG, services exposés) varie.

Le **Tracker** (Origin) affiche une **carte par COG** avec un bouton **« Visiter »** vers la Home du COG. Les COGs **non connectés** ont leur carte **grisée** avec un bandeau **« Absent »**. Ce comportement s’inscrit dans la **philosophie Miyukini** : les utilisateurs de COGs ont, par le contrôle qui leur est offert, un **droit reconnu de déconnexion**.

---

## 1. Home COG — exposition automatique

### 1.1 Principe

- Dès qu’un COG est **annoncé sur le Tracker** (session MWS en ligne), le **mini serveur web** contenu dans **Central natif** est activé (s’il est configuré).
- Ce serveur écoute sur une adresse configurable (ex. `0.0.0.0:8080`) et sert une **page d’accueil unique** : **GET /** = **Home COG**.
- La Home présente :
  - l’**identité du COG** (cog_id, version des Cores) ;
  - les **services disponibles** dans le catalogue du Tracker pour ce COG (services exposés à l’annonce) ;
  - optionnellement une **présentation de l’utilisateur** (propriétaire du COG) lorsque l’application fournit cette donnée.

### 1.2 Layout commun

- Le **même gabarit** (structure HTML/CSS, en-tête, style Miyukini) est utilisé pour **tous les COGs**.
- Seuls changent : cog_id, core_version, liste des services, et éventuellement le libellé utilisateur.
- La Home est le **point d’entrée** pour les visiteurs externes : découverte du COG et de ses services sans avoir à installer Central.

### 1.3 Adresse publique et « Visiter »

- Lors de l’annonce, le COG envoie au Tracker son **adresse publique** (`public_address`, format `host:port`).
- Par **convention** : la **Home** du COG est à l’URL **`http://{public_address}/`** (ou `https://` si le déploiement le prévoit).
- Le Tracker affiche donc un lien **« Visiter »** pointant vers cette URL pour chaque COG **présent**.

---

## 2. Carte COG sur le Tracker

### 2.1 Affichage des COGs

- Le site web **Origin** (catalogue / section COGs) affiche une **carte par COG** enregistré dans les pools du Tracker.
- Chaque carte affiche au minimum :
  - **Identifiant** du COG (courte forme ou tronquée) ;
  - **Version** des Cores ;
  - **Nombre de services** exposés (et éventuellement les noms) ;
  - **État** : présent ou absent (voir ci-dessous) ;
  - **Bouton « Visiter »** : lien vers la Home du COG (`http://{address}/`).

### 2.2 COG présent vs absent

- **Présent** : le COG a envoyé un **heartbeat** (ou une annonce) récemment ; le Tracker le considère connecté. La carte est **normale**, le bouton **« Visiter »** est actif.
- **Absent** : le COG n’a pas été vu depuis un délai dépassant le seuil (ex. 2× l’intervalle de heartbeat). La carte est **grisée**, un **bandeau « Absent »** est affiché ; le bouton « Visiter » peut être désactivé ou masqué, selon le choix d’UX.

Cette distinction respecte le **droit de déconnexion** : un utilisateur peut éteindre son COG ou le déconnecter du réseau ; le Tracker continue d’afficher le COG (pour référence) mais signale clairement qu’il n’est **pas joignable** pour le moment.

---

## 3. Droit de déconnexion

- Les utilisateurs de COGs ont, par le **contrôle** offert par l’écosystème Miyukini, un **droit reconnu de déconnexion**.
- Aucune obligation de rester en ligne : un COG peut à tout moment se retirer du réseau (Withdraw) ou simplement s’éteindre.
- Le Tracker **conserve** les COGs connus (pour l’historique et la découverte) mais **affiche** leur état (présent / absent) de façon honnête, sans imposer la disponibilité.

---

## 4. Implémentation technique (référence)

| Composant | Rôle |
|-----------|------|
| **Central (natif)** | Mini serveur HTTP optionnel : écoute sur `home_http_bind`, actif lorsque MWS est en ligne ; sert GET / avec le layout commun et les données COG + services. |
| **CentralMwsConfig** | Champs `public_address` (annoncé au Tracker), `home_http_bind` (optionnel) pour l’écoute du serveur Home. |
| **Origin / Tracker** | API `/api/catalog` : liste des COGs avec `address`, `last_seen`, `present`. Route **GET /visit?cog_id=…** : enregistre la visite (CatalogVisitTracker), redirige 302 vers le Home du COG. Page catalogue : lien « Visiter » vers `/visit?cog_id=…`, bandeau « Absent » si non présent. |
| **Seuils** | Heartbeat : présent si `last_seen` ≤ 90 s. Visite catalogue : présent si une visite a eu lieu dans les 60 s (sinon le Tracker ne considère le COG présent que sur heartbeat). |

---

## 5. Mise en service (par défaut)

- **Central natif** : à la connexion MWS, la config utilise par défaut `home_http_bind = "0.0.0.0:8080"` et `public_address = "127.0.0.1:8080"`. La page Home est donc exposée dès que le COG est annoncé ; le lien « Visiter » du catalogue Origin pointe vers cette adresse (usage local). Pour un déploiement avec adresse publique, adapter `public_address` (ex. IP ou hostname:8080) dans la configuration MWS.

---

## 6. Carte service JayXpose sur la Home

- Si le COG expose le **service JayXpose** et qu’une **vitrine est publiée** (`vitrine_status = 'publiee'`), la Home peut afficher une **carte** dédiée : nom du service, courte description, bouton **« Découvrir »** pointant vers les pages vitrine (servies par le serveur web Origin).
- L’utilisateur du COG peut **activer ou non** cette exposition (config MWS : `expose_jayxpose_vitrine`, et optionnellement `jayxpose_vitrine_base_url` pour l’URL de base du serveur Origin). Par défaut, si une vitrine publiée existe au moment de la connexion MWS, la carte est proposée.
- Voir **JayXpose — Intégration Home COG et surface web** (docs/services/JayXpose/) pour le détail.

---

## 7. Voir aussi

- **MWS - Document Fondateur** (présence, annonce, Tracker).
- **JayXpose — Intégration Home COG et surface web** (docs/services/JayXpose/JayXpose - Integration Home COG et Surface Web.md).
- **docs/miyukini-webway-system/reference/** pour le protocole Tracker et les types de messages.
