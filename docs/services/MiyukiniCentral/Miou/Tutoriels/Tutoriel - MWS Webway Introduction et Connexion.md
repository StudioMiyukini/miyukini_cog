# Tutoriel — MWS Webway, introduction et connexion

Tutoriel d'introduction au **Miyukini Webway System (MWS)** : le réseau de présence et de découverte entre COGs.

---

## Métadonnées

| Champ | Valeur |
|-------|--------|
| **ID** | `tutoriel_mws_connexion` |
| **Durée estimée** | 3–4 min |
| **Cible** | Utilisateur découvrant le Webway |
| **Prérequis** | Connexion au COG, session active, onglet Webway ouvert |

---

## Objectifs pédagogiques

À l'issue du tutoriel, l'utilisateur :
1. Comprend ce qu'est le MWS (présence, découverte, transport)
2. Connaît le protocole de conformité (Origin → Relay → Tracker)
3. Sait se connecter au réseau MWS
4. Connaît le mode Lone (COG isolé)
5. Sait rechercher des COGs et des lobbys une fois connecté

---

## Contexte conceptuel (pour Miou)

Le MWS est la couche de **présence, découverte et transport** des COGs. Il ne transporte pas les données métier — il permet de savoir *où* et *comment* initier une visite gouvernée. Acteurs principaux :
- **Origin** : point d'origine, cumule relay + tracker
- **Relays** : vérification de conformité, délivrance du Permis de circulation
- **Trackers** : douaniers du réseau, contrôle d'identité, catalogue des lobbys

---

## Étapes détaillées

### Étape 1 — Présentation du Webway

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `mws-header` |
| **Flèche verte** | Oui, vers l'en-tête « Réseau MWS » |
| **Texte Miou** | « Le **Webway**, c'est le réseau entre COGs. Il permet de se déclarer, de découvrir qui est présent, et de rejoindre des lobbys — sans transporter tes données. » |
| **Variantes** | « Ici tu vois l'état de ton COG sur le réseau. Connecté, tu peux trouver d'autres environnements et des sessions à rejoindre. » |

### Étape 2 — Le protocole de conformité

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `mws-conformity` |
| **Flèche verte** | Oui, vers le bloc « Protocole de conformité MWS » |
| **Texte Miou** | « Avant de te connecter, ton COG passe par un protocole de conformité : résolution Origin → connexion Relay → obtention du Permis → connexion Tracker. Dix étapes pour garantir la confiance du réseau. » |
| **Variantes** | « Ces étapes vérifient que ton COG est conforme. Une fois les 10 validées, tu es sur le Webway. » |

### Étape 3 — Se connecter

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `mws-btn-connect` |
| **Flèche verte** | Oui, vers le bouton « Se connecter » |
| **Texte Miou** | « Clique sur **Se connecter** pour rejoindre le réseau. Ton COG va contacter un Relay, obtenir son Permis de circulation, puis s'annoncer auprès des Trackers. » |
| **Variantes** | « Un clic ici lance la connexion. Ça peut prendre quelques secondes. » |

### Étape 4 — Mode Lone (optionnel)

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `mws-btn-lone` |
| **Flèche verte** | Oui, vers le toggle « Lone » / « Réseau » |
| **Texte Miou** | « Le **mode Lone** isole ton COG : aucune connexion réseau. Utile si tu veux garder tes données strictement locales. Tu peux basculer à tout moment. » |
| **Variantes** | « Lone = COG en île. Réseau = connecté au Webway. » |

### Étape 5 — Recherche (une fois connecté)

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | `mws-search-input` |
| **Flèche verte** | Oui, vers le champ de recherche |
| **Texte Miou** | « Une fois connecté, tu peux rechercher des COGs ou des lobbys ici. Les lobbys sont des sessions hébergées par d'autres COGs — tu peux les rejoindre avec leur accord. » |
| **Variantes** | « Tape un nom ou laisse vide pour voir la liste. Les lobbys apparaîtront en bas. » |

### Étape 6 — Conclusion

| Champ | Valeur |
|-------|--------|
| **data-tutorial-id** | — |
| **Flèche verte** | Non |
| **Texte Miou** | « Voilà. Le Webway, c'est présence et découverte. La gouvernance reste chez toi — le réseau indique juste où aller pour initier une visite. Bonne exploration. » |

---

## Triggers de proposition

- Première ouverture de l'onglet Webway ET `tutoriel_mws_vu == false`
- Demande utilisateur : « Comment me connecter au réseau ? », « C'est quoi le Webway ? », « Comment rejoindre le MWS ? »

---

## Références doc

- `docs/miyukini-webway-system/MWS - Document Fondateur.md`
- Glossaire MWS : Origin, Relay, Tracker, Permis de circulation, accord d'hôte
