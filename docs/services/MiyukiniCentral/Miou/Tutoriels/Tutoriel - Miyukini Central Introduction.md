# Tutoriel — Miyukini Central, introduction

Tutoriel d'introduction à **Miyukini Central**, le hub de gestion des Services du COG.

---

## Métadonnées

| Champ | Valeur |
|-------|--------|
| **ID** | `tutoriel_central_intro` |
| **Durée estimée** | 2–3 min |
| **Cible** | Utilisateur découvrant Central |
| **Prérequis** | Connexion au COG, session active |

---

## Objectifs pédagogiques

À l'issue du tutoriel, l'utilisateur sait :
1. Ce qu'est Miyukini Central (vitrine du Registre d'Opérateurs)
2. Naviguer entre Salon, Bibliothèque et Webway
3. Ouvrir un service depuis une carte
4. Comprendre la TabBar (onglets ouverts)

---

## Étapes détaillées

### Étape 1 — Accueil

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `home-welcome` |
| **Flèche verte** | Oui, vers la bannière d'accueil |
| **Texte Miou** | « Bienvenue dans Miyukini Central, {pseudo}. C'est ton hub : ici tu vois les Services de ton COG et tu peux les ouvrir. » |
| **Variantes** | « Tu es sur la page d'accueil. Central, c'est la vitrine de tes Opérateurs — tout passe par ici. » |

### Étape 2 — Le Salon

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `nav-salon` |
| **Flèche verte** | Oui, vers le bouton SALON |
| **Texte Miou** | « Le **Salon** est ton lieu de vie. Tu y vois les services populaires et les nouveautés. C'est là que je suis avec toi. » |
| **Variantes** | « Le Salon, c'est ton écran d'accueil — services en vitrine, et moi pour te guider. » |

### Étape 3 — La Bibliothèque

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `nav-bibliotheque` |
| **Flèche verte** | Oui, vers le bouton BIBLIOTHÈQUE |
| **Texte Miou** | « La **Bibliothèque**, c'est tes Services installés. Tout ce que ton COG te propose, organisé par toi. » |
| **Variantes** | « Tes services sont dans la Bibliothèque. Tu peux filtrer par Tous, Installés ou Favoris. » |

### Étape 4 — Le Webway

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `nav-webway` |
| **Flèche verte** | Oui, vers le bouton WEBWAY |
| **Texte Miou** | « Le **Webway**, c'est le réseau entre COGs. Tu peux te connecter pour découvrir d'autres environnements, des lobbys, des sessions. Je t'expliquerai ça plus en détail si tu veux. » |
| **Variantes** | « Le Webway relie les COGs. Présence, découverte, lobbys — tout ça dans un autre tutoriel. » |

### Étape 5 — Ouvrir un service

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `service-card-jayxpose` (ou premier service installé disponible) |
| **Flèche verte** | Oui, vers une carte de service |
| **Texte Miou** | « Pour ouvrir un service, clique sur sa carte. Par exemple JayXpose pour ta vitrine, ou JayKoa pour ton calendrier. Un clic = un nouvel onglet. » |
| **Variantes** | « Clique sur n'importe quelle carte pour lancer le service. L'onglet s'ouvrira en haut. » |

### Étape 6 — Les onglets

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `tab-accueil` |
| **Flèche verte** | Oui, vers la TabBar |
| **Texte Miou** | « Les onglets en haut te permettent de passer d'un service à l'autre. Le bouton « + » ouvre la Bibliothèque pour en ajouter. » |
| **Variantes** | « Tes services ouverts apparaissent ici. Tu peux en fermer avec le « ✕ » sur chaque onglet. » |

### Étape 7 — Conclusion

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | — |
| **Flèche verte** | Non |
| **Texte Miou** | « Voilà, tu connais Central. Si tu as des questions, demande-moi. Et n'hésite pas à explorer le Webway pour voir les autres COGs. » |

---

## Triggers de proposition

- Première connexion de la session ET `tutoriel_central_vu == false`
- Demande utilisateur : « Explique-moi Central », « Comment ça marche ? », « C'est quoi Central ? »

---

## Références doc

- `docs/reference/Miyukini Conceptual References - Miyukini Central Hub Services.md`
- `docs/services/MiyukiniCentral/Miyukini Central - Ecrans et UI.md`
