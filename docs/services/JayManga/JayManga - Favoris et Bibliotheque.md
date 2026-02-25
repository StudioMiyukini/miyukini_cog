# JayManga — Favoris et Bibliotheque

## Contexte

Ce document detaille les fonctionnalites de **favoris cross-COG**, de **bibliotheque lecteur**, de **telechargement hors-ligne** et de **presence MWS** de JayManga. Ces fonctionnalites constituent l'experience cote lecteur : suivre ses lectures, retrouver ses oeuvres favorites, savoir si un vendeur est en ligne, et lire hors connexion.

Les favoris et la bibliotheque sont geres sur le **COG du lecteur** (LOI-3 — etat local souverain). L'information de presence provient du **MWS**.

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Favoris cross-COG

### 1.1 Principe

Un lecteur possedant un COG peut mettre en favoris des oeuvres provenant de **n'importe quel COG vendeur** proposant JayManga. Les favoris sont **stockes localement** sur le COG du lecteur. Ils referencent les oeuvres par une paire d'identifiants : `seller_cog_id` (COG vendeur) + `work_id` (oeuvre sur ce COG).

Ce mecanisme est **decentralise** : aucun serveur central ne connait la liste de favoris d'un lecteur.

### 1.2 Donnees stockees par favori

| Champ | Source | Description |
|-------|--------|-------------|
| `seller_cog_id` | COG vendeur | Identifiant unique du COG hebergeant l'oeuvre. |
| `work_id` | COG vendeur | Identifiant de l'oeuvre sur le COG vendeur. |
| `cached_title` | Cache local | Titre de l'oeuvre (mis en cache lors de l'ajout ou de la derniere synchronisation). |
| `cached_cover` | Cache local | Image de couverture en cache (miniature). |
| `cached_authors` | Cache local | Auteurs en cache. |
| `cached_format` | Cache local | Format de lecture (`manga`, `webtoon`, etc.). |
| `purchase_status` | Local | `demo` (pas achete) / `purchased` (licence active) / `downloaded` (telecharge localement). |
| `last_read_chapter` | Local | Dernier chapitre lu. |
| `last_read_page` | Local | Derniere page lue dans le chapitre. |
| `reading_progress` | Local | Pourcentage global de progression (0.0 a 1.0). |
| `added_at` | Local | Date d'ajout aux favoris. |
| `last_synced_at` | Local | Date de la derniere synchronisation des metadonnees depuis le COG vendeur. |

### 1.3 Ajout aux favoris

| Contexte | Action |
|----------|--------|
| Depuis la fiche oeuvre (Portail) | Bouton « Ajouter aux favoris ». Necessite que le lecteur soit authentifie (identite COG). |
| Depuis l'ecran de fin de demonstration | Bouton « Ajouter aux favoris » sur l'ecran d'incitation a l'achat. |
| Depuis la bibliotheque (Central) | Le lecteur peut ajouter un favori manuellement en saisissant un lien ou en scannant un QR code. |

### 1.4 Suppression des favoris

Le lecteur peut retirer un favori a tout moment. La suppression du favori ne supprime pas les fichiers telecharges (si existants) ; ceux-ci restent accessibles dans la section « Telechargements ».

---

## 2. Bibliotheque lecteur

### 2.1 Vue d'ensemble

La bibliotheque est la vue unifiee de toutes les oeuvres que le lecteur a mises en favoris, achetees ou telechargees. Elle est accessible depuis **Miyukini Central** sur le COG du lecteur.

### 2.2 Sections de la bibliotheque

| Section | Contenu |
|---------|---------|
| **Favoris** | Toutes les oeuvres en favoris, triees par date d'ajout (defaut) ou par titre, auteur, progression. |
| **Achats** | Oeuvres pour lesquelles le lecteur possede une licence active. Sous-ensemble des favoris (les achats sont automatiquement ajoutes aux favoris). |
| **Telechargements** | Oeuvres telechargees localement. Lisibles hors-ligne. |
| **En cours** | Oeuvres avec une progression de lecture > 0% et < 100%. Tri par derniere lecture. |
| **Termines** | Oeuvres avec une progression de lecture = 100%. |

### 2.3 Affichage d'un favori

Chaque favori dans la bibliotheque affiche :

```
┌────────────────────────────────────────────────┐
│  [Cover]  Titre de l'Oeuvre                    │
│           Auteur(s)                            │
│           Format : Manga                       │
│           ░░░░░░░░░░▓▓▓░░  42%                │  ← Progression
│           🟢 En ligne    OU    ⚫ Hors ligne   │  ← Statut MWS
│           [Demo] [Achete] [Telecharge]         │  ← Badges statut
└────────────────────────────────────────────────┘
```

### 2.4 Actions sur un favori

| Action | Condition | Description |
|--------|-----------|-------------|
| **Lire** | COG vendeur en ligne | Ouvre la liseuse web sur le Portail du vendeur. |
| **Lire (local)** | Oeuvre telechargee | Ouvre la liseuse native dans Central. |
| **Acheter** | Licence absente | Redirige vers la fiche oeuvre sur le Portail du vendeur. |
| **Telecharger** | Licence active + autorisation vendeur | Lance le telechargement sur le COG local. |
| **Mettre a jour** | Nouvelle version disponible | Re-telecharge les fichiers mis a jour. |
| **Retirer des favoris** | — | Supprime le favori. Les telechargements sont conserves. |
| **Synchroniser** | COG vendeur en ligne | Met a jour les metadonnees en cache (titre, couverture, nombre de chapitres). |

---

## 3. Presence MWS

### 3.1 Role dans JayManga

La presence MWS permet au lecteur de savoir si le COG hebergeant une oeuvre est actuellement **en ligne** (accessible via le reseau) ou **hors-ligne** (inaccessible). Cette information est essentielle pour :

- Afficher un indicateur de disponibilite dans la bibliotheque.
- Determiner si la lecture en ligne est possible.
- Determiner si le telechargement est possible.

### 3.2 Mecanisme

```
Bibliotheque lecteur (Central)
  → Pour chaque favori : requete locale au module MWS du COG lecteur
  → Le module MWS utilise QUERY_PRESENCE_BATCH au Tracker
    (voir MWS - Manifestes de Services, section 5.3)
  → Retour : online / offline / unknown pour chaque COG vendeur
  → Affichage de l'indicateur dans la bibliotheque
```

La requete batch de presence du Tracker permet de verifier la disponibilite de tous les COGs favoris en une seule requete, sans contacter chaque COG individuellement.

### 3.3 API de presence

| Endpoint | Methode | Description |
|----------|---------|-------------|
| `/api/jaymanga/presence/{cog_id}` | GET | Statut de presence d'un COG vendeur specifique. |
| `/api/jaymanga/presence/batch` | POST | Statut de presence de plusieurs COGs en une requete (body : `{"cog_ids": [...]}`). Utilise par la bibliotheque pour mettre a jour tous les favoris. |
| `/api/jaymanga/discover` | GET | Liste des COGs proposant JayManga connus du tracker local. |

### 3.4 Statuts de presence

| Statut | Affichage | Description |
|--------|-----------|-------------|
| `online` | 🟢 En ligne | Le COG vendeur est connecte et accessible. Lecture et telechargement possibles. |
| `offline` | ⚫ Hors ligne | Le COG vendeur est deconnecte. Seule la lecture locale (si telechargee) est possible. |
| `unknown` | ⚪ Inconnu | Le statut n'a pas pu etre determine (tracker inaccessible, cache expire). |

### 3.5 Cache de presence

Pour eviter des requetes MWS trop frequentes, la bibliotheque applique un **cache local** :

| Parametre | Valeur defaut | Description |
|-----------|---------------|-------------|
| Duree du cache | 5 minutes | Duree pendant laquelle le statut est considere valide sans re-interrogation. |
| Rafraichissement automatique | A l'ouverture de la bibliotheque | Tous les statuts sont rafraichis a l'ouverture de la bibliotheque. |
| Rafraichissement a la demande | Bouton « Actualiser » | Le lecteur peut forcer la mise a jour de tous les statuts. |

---

## 4. Telechargement hors-ligne

### 4.1 Conditions de telechargement

Le telechargement d'une oeuvre necessite que **toutes** les conditions suivantes soient remplies :

1. Le lecteur possede une **licence active** pour l'oeuvre (statut `active`).
2. Le vendeur a **autorise le telechargement** pour cette oeuvre (`allow_download = true` sur l'oeuvre ou `default_allow_download = true` global).
3. Le **COG vendeur est en ligne** au moment du telechargement.
4. Le lecteur est **authentifie** (identite COG).

### 4.2 Processus de telechargement

```
COG Lecteur                              COG Vendeur
    │                                         │
    ├─ Requete telechargement ──────────────→ │
    │  (license_id, work_id, buyer_cog_id)    │
    │                                         ├─ Verification licence
    │                                         ├─ Verification autorisation download
    │                                         ├─ Preparation des fichiers
    │  ←───── Transmission fichiers ──────────┤
    │  (tunnel MWS ou connexion directe)      │
    ├─ Verification integrite (SHA-256)       │
    ├─ Stockage local (KindMother)            │
    ├─ Mise a jour du favori (downloaded)     │
    │                                         │
```

### 4.3 Contenu telecharge

| Element | Inclus | Description |
|---------|--------|-------------|
| Pages originales | Oui | Fichiers images originaux (qualite maximale). |
| Variantes optimisees | Non | Non telechargees ; la lecture locale utilise les originaux. |
| Metadonnees | Oui | Titre, auteurs, genres, synopsis, structure chapitres. |
| Couverture | Oui | Image de couverture. |
| Licence | Oui | Copie de la licence pour verification locale. |

### 4.4 Stockage local

Les fichiers telecharges sont stockes sur le COG du lecteur via KindMother :

```
/jaymanga/downloads/{seller_cog_id}/{work_id}/
  ├── metadata.json          (metadonnees de l'oeuvre)
  ├── license.json           (copie de la licence)
  ├── cover.jpg              (couverture)
  └── chapters/
      ├── 01/
      │   ├── page_001.jpg
      │   ├── page_002.jpg
      │   └── ...
      ├── 02/
      │   └── ...
      └── ...
```

### 4.5 Verification d'integrite

Chaque fichier telecharge est accompagne d'un hash SHA-256. Le COG lecteur verifie l'integrite de chaque fichier apres reception. En cas d'echec, le fichier est re-telecharge.

### 4.6 Mise a jour des oeuvres telechargees

| Scenario | Comportement |
|----------|-------------|
| Le vendeur ajoute des chapitres | Le lecteur est notifie (indicateur « Mise a jour disponible » dans la bibliotheque). Le lecteur peut telecharger les nouveaux chapitres. |
| Le vendeur corrige des pages | Idem. Les pages corrigees sont re-telechargees. |
| Le vendeur desactive le telechargement | Les futurs telechargements sont bloques. Les fichiers deja telecharges restent accessibles (LOI-3, RM-03). |
| Le vendeur archive l'oeuvre | Les fichiers telecharges restent lisibles localement. Pas de nouvelle licence possible. |

### 4.7 Gestion de l'espace disque

| Fonctionnalite | Description |
|----------------|-------------|
| Indicateur d'espace | La bibliotheque affiche l'espace total utilise par les telechargements JayManga et l'espace restant sur le COG. |
| Suppression locale | Le lecteur peut supprimer les fichiers telecharges d'une oeuvre pour liberer de l'espace. La licence reste active ; il pourra re-telecharger si le vendeur est en ligne et l'autorise. |

---

## 5. Synchronisation du cache des metadonnees

### 5.1 Objectif

Les metadonnees en cache dans les favoris (titre, couverture, auteurs, nombre de chapitres) peuvent devenir obsoletes si le vendeur met a jour son catalogue. La synchronisation permet de maintenir le cache a jour.

### 5.2 Declenchement

| Declencheur | Description |
|-------------|-------------|
| Ouverture de la bibliotheque | Les metadonnees des favoris dont le COG vendeur est en ligne sont synchronisees. |
| Ajout d'un favori | Les metadonnees sont chargees immediatement. |
| Synchronisation manuelle | Le lecteur peut forcer la synchronisation d'un favori ou de tous les favoris. |
| Apres un telechargement | Les metadonnees sont mises a jour avec les donnees fraiches recues. |

### 5.3 Donnees synchronisees

| Champ | Description |
|-------|-------------|
| Titre | Mis a jour si le vendeur a change le titre. |
| Couverture | Re-telechargee si mise a jour. |
| Auteurs | Mis a jour. |
| Nombre de chapitres | Mis a jour (detecte les nouveaux chapitres). |
| Nombre total de pages | Mis a jour. |
| Prix | Mis a jour (pour affichage dans la bibliotheque si non achete). |
| Statut de publication | Mis a jour (detecte les oeuvres archivees). |
| Autorisation telechargement | Mis a jour. |

### 5.4 Comportement hors-ligne

Si le COG vendeur est hors-ligne, les metadonnees en cache sont conservees telles quelles. Le `last_synced_at` n'est pas mis a jour. La bibliotheque affiche un indicateur « Derniere synchronisation : il y a X jours » si le cache est ancien (> 7 jours).

---

## 6. Decouverte de nouveaux vendeurs

### 6.1 Via le MWS

Le lecteur peut decouvrir de nouveaux COGs proposant JayManga en interrogeant les trackers MWS :

```
Lecteur → Central → Module MWS → Tracker
  → Liste des COGs avec service JayManga
  → Pour chaque COG : shop_name, work_count, online_status
  → Le lecteur peut visiter le Portail d'un COG pour parcourir son catalogue
```

### 6.2 Via lien direct

Le lecteur peut acceder directement au Portail d'un COG vendeur via une URL partagee (ex. lien recu par email, reseau social, QR code).

### 6.3 Via recommandation (Phase 2)

A terme, le systeme pourra suggerer des oeuvres similaires a celles en favoris en interrogeant les catalogues des COGs connus via les trackers.

---

## 7. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Lecture et Liseuse](./JayManga%20-%20Lecture%20et%20Liseuse.md) | Detail de la liseuse et de la lecture hors-ligne. |
| [JayManga - Achat et Paiement](./JayManga%20-%20Achat%20et%20Paiement.md) | Licences et achats. |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Systeme de presence et decouverte. |
| [MWS - Manifestes de Services](../../miyukini-webway-system/protocole/MWS%20-%20Manifestes%20de%20Services.md) | Protocole de manifestes — requete batch de presence. |

---

**Document** : JayManga — Favoris et Bibliotheque
**Version** : 1.1
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle detaillee.
