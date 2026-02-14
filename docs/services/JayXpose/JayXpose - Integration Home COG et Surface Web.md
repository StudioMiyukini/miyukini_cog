# JayXpose — Intégration Home COG et surface web

## Contexte

Ce document décrit l’**intégration du service JayXpose** avec la **page Home du COG** (MWS) et la **surface web** : exposition optionnelle de la vitrine sur la Home, carte « Découvrir », et alignement des pages vitrine (présentation, catalogue avec prix et disponibilité en temps réel).

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Pages Web Publiques Vitrine](./JayXpose%20-%20Pages%20Web%20Publiques%20Vitrine.md), [MWS - Home COG et carte Tracker](../../miyukini-webway-system/MWS%20-%20Home%20COG%20et%20carte%20Tracker.md).

---

## 1. Principe

- La **Home du COG** est servie par le **mini serveur HTTP** intégré à Central (natif), lorsque le COG est annoncé sur le Tracker et que `home_http_bind` est configuré.
- JayXpose peut **proposer sur cette surface** un **mini-site vitrine** : présentation de l’exposant, catalogue des produits avec **présentation, prix et disponibilité en temps réel**.
- **L’utilisateur du COG** décide s’il souhaite **rendre disponibles** ces pages (paramètre « Exposer la vitrine JayXpose sur la Home »).
- Si activé et qu’une vitrine est **publiée** (`vitrine_status = 'publiee'`), une **carte** est affichée sur la Home avec :
  - le **nom du service** (JayXpose),
  - une **courte description**,
  - un bouton **« Découvrir »** qui mène vers les pages vitrine (servies par le service WEB Origin).

---

## 2. Architecture

| Composant | Rôle |
|-----------|------|
| **Central (natif)** | Sert la page Home (GET /). Affiche la carte JayXpose si `expose_jayxpose_vitrine` est activé et qu’un slug vitrine publiée existe. Le lien « Découvrir » pointe vers l’URL de base du Tracker (Origin) + `/vitrine/{slug}`. |
| **CentralMwsConfig** | Nouveaux champs : `expose_jayxpose_vitrine` (bool), `jayxpose_vitrine_base_url` (Option&lt;String&gt;) — URL publique du serveur web Origin (ex. `http://origin.example.com:8080`). |
| **Origin** | Sert les routes `/vitrine`, `/vitrine/{slug}`, `/vitrine/{slug}/catalogue`, etc. Données en **lecture seule** depuis la base JayXpose (KindMother). **Prix et disponibilité** sont lus à chaque requête (temps réel). |
| **Crate jayxpose** | Méthode `first_published_vitrine_slug()` : retourne le premier `vitrine_slug` tel que `vitrine_status = 'publiee'` (pour afficher la carte et construire le lien). |

---

## 3. Comportement utilisateur

1. **Activation** : L’utilisateur du COG active « Exposer la vitrine JayXpose sur la Home » (paramètre MWS ou écran JayXpose).
2. **Condition** : Au moins un exposant doit avoir une vitrine **publiée** (`vitrine_status = 'publiee'`) et un `vitrine_slug` renseigné.
3. **Home** : Lors de la connexion MWS, la Home affiche la section « Services disponibles » et, si JayXpose est exposé, une **carte** :
   - Titre : **JayXpose**
   - Description : **Vitrine et catalogue — Découvrez notre présentation et nos produits.**
   - Bouton : **Découvrir** → `{jayxpose_vitrine_base_url}/vitrine/{slug}`
4. **Pages vitrine** (sur Origin) : Présentation, catalogue avec **prix** et **disponibilité** à jour (lecture DB à chaque requête).

---

## 4. Données temps réel (catalogue)

- Les pages `/vitrine/{slug}/catalogue` et `/vitrine/{slug}/catalogue/{id}` utilisent les champs :
  - `price` (affiché en € ou « Sur demande »),
  - `availability` : `disponible` / `rupture` / `sur_commande` (affiché sur la fiche et la liste).
- Aucune mise en cache côté serveur pour ces données : chaque requête relit la base (temps réel).

---

## 5. Implémentation technique (référence)

| Élément | Détail |
|--------|--------|
| **Config MWS** | `expose_jayxpose_vitrine: bool` (défaut `false`), `jayxpose_vitrine_base_url: Option<String>`. |
| **Création du gestionnaire Home** | Lors du démarrage du serveur Home, Central fournit le `vitrine_slug` (via `first_published_vitrine_slug()` sur la base JayXpose) et la config. |
| **Génération HTML Home** | Si `jayxpose` dans services, `expose_jayxpose_vitrine` et slug et base_url présents : insérer une carte avec lien « Découvrir ». |
| **Origin** | Afficher `availability` sur les cartes catalogue et la fiche produit (libellé lisible : Disponible / Rupture / Sur commande). |

---

## 6. Voir aussi

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Pages Web Publiques Vitrine](./JayXpose%20-%20Pages%20Web%20Publiques%20Vitrine.md)
- [JayXpose - Site Vitrine Specification](./JayXpose%20-%20Site%20Vitrine%20Specification.md)
- [MWS - Home COG et carte Tracker](../../miyukini-webway-system/MWS%20-%20Home%20COG%20et%20carte%20Tracker.md)

---

**Document** : JayXpose — Intégration Home COG et surface web  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Référence — spécification d’intégration
