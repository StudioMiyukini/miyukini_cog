# Miou â€” Onglet Service et Mode Chatbot

Miou dispose d'un **onglet de service dÃ©diÃ©** dans Miyukini Central, offrant un **mode chatbot** de type ChatGPT pour discuter et enrichir ses connaissances sur l'utilisateur. L'interface est transparente : l'utilisateur peut consulter, modifier et supprimer les donnÃ©es de la base de connaissance.

---

## 1. Contexte et positionnement

| Aspect | Description |
|--------|-------------|
| **Type** | Onglet de service Miou (comme JayKoa, JayXpose) dans la TabBar de Central |
| **AccÃ¨s** | BibliothÃ¨que > Carte Miou, ou onglet dÃ©diÃ© si Miou est Â« installÃ©e Â» comme service |
| **RÃ´le** | Canal conversationnel privilÃ©giÃ©, enrichissement des connaissances, transparence totale |

**Phrase fondatrice :**

> **L'onglet Miou est le lieu oÃ¹ l'utilisateur converse librement avec Miou et contrÃ´le tout ce qu'elle sait de lui.**

---

## 2. Interface type ChatGPT

### 2.1 Structure gÃ©nÃ©rale

| Zone | Description |
|------|-------------|
| **En-tÃªte** | Titre Â« Miou Â», indication du palier de relation (optionnel), lien vers Â« Mes connaissances Â» |
| **Zone conversation** | Historique des Ã©changes (bulles utilisateur / rÃ©ponses Miou), scroll automatique |
| **Zone saisie** | Champ texte multiligne + bouton Envoyer (style ChatGPT / Claude) |
| **Sidebar (optionnelle)** | AccÃ¨s rapide : Â« Ce que Miou sait de moi Â», ParamÃ¨tres Miou |

### 2.2 Comportement conversationnel

| FonctionnalitÃ© | Description |
|----------------|-------------|
| **Saisie libre** | L'utilisateur tape un message, Miou rÃ©pond (Proto-IA ou LLM selon config) |
| **Contexte** | Miou a accÃ¨s au contexte : profil, connaissances stockÃ©es, session, palier |
| **Enrichissement** | Miou peut poser des questions, l'utilisateur peut divulguer des infos â€” stockage explicite |
| **Ton** | AdaptÃ© au palier d'attachement (Connaissance â†’ Grande sÅ“ur) |
| **RÃ©fÃ©rences culturelles** | Miou peut blaguer ou Ãªtre espiÃ¨gle avec des rÃ©fÃ©rences pop culture (voir [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)) |

### 2.3 Invariants

- **Aucune collecte passive** : Miou ne lit que ce que l'utilisateur envoie dans le chat.
- **Stockage explicite** : Les infos extraites sont enregistrÃ©es uniquement aprÃ¨s validation ou accord implicite (ex. Â« Tu peux retenir Ã§a Â»).
- **100 % local** : Aucune donnÃ©e envoyÃ©e hors du COG.

---

## 3. Base de connaissance â€” Transparence

### 3.1 AccÃ¨s Â« Ce que Miou sait de moi Â»

Depuis l'onglet Miou (ou ParamÃ¨tres > Miou), l'utilisateur accÃ¨de Ã  la **liste complÃ¨te** des connaissances stockÃ©es.

| Action | Description |
|--------|-------------|
| **Voir** | Affichage de chaque donnÃ©e : clÃ© (ex. `reconfort`), valeur, date d'enregistrement |
| **Modifier** | Ã‰dition inline ou formulaire â€” mise Ã  jour immÃ©diate |
| **Supprimer** | Bouton par entrÃ©e â€” confirmation lÃ©gÃ¨re, suppression dÃ©finitive |

### 3.2 Organisation de l'affichage

Les donnÃ©es sont regroupÃ©es par **catÃ©gorie** (voir [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)) :

- IdentitÃ© et prÃ©fÃ©rences pratiques
- Loisirs et habitudes
- Ã‰motions et bien-Ãªtre
- Projets et valeurs
- Soutien et prÃ©sence
- DonnÃ©es Rite d'EntrÃ©e (profil Central)

### 3.3 Options globales

| Option | Effet |
|--------|-------|
| **Effacer tout** | Â« RÃ©initialiser la connaissance de Miou Â» â€” supprime toutes les rÃ©ponses, remet le palier Ã  Inconnue (ou propose de garder les donnÃ©es tout en rÃ©initialisant le palier) |
| **Exporter** | Export JSON ou CSV des donnÃ©es (pour portabilitÃ©, backup) |
| **Importer** | Restauration depuis un export (Ã  dÃ©finir) |

---

## 4. Flux conversationnel et enrichissement

### 4.1 DÃ©clenchement de questions

Miou peut **proposer** des questions dans le chat :

- Selon le palier d'attachement
- Selon les thÃ¨mes non encore couverts
- En rÃ©ponse Ã  un message de l'utilisateur (Â« Et toi, qu'est-ce qui te fait du bien ? Â»)

### 4.2 Extraction et confirmation

Quand l'utilisateur partage une info pertinente :

| Mode | Description |
|------|-------------|
| **Explicite** | Miou demande : Â« Tu veux que je retienne Ã§a ? Â» â€” boutons Oui / Non |
| **Implicite** | Si l'utilisateur dit clairement Â« retiens queâ€¦ Â» â€” enregistrement direct |
| **DÃ©duit** | Miou ne dÃ©duit jamais sans confirmation â€” pas d'extraction passive |

### 4.3 IntÃ©gration avec les bulles

Les donnÃ©es enrichies via le chatbot sont **rÃ©utilisÃ©es** dans les bulles (pause santÃ©, accueil, retour). Voir [Miou - Roadmap et AmÃ©liorations](_index.md) â€” rÃ©utilisation des rÃ©ponses.

---

## 5. RÃ©fÃ©rences

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - PrÃ©fÃ©rences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md)
- [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)

---

**Version :** 1.0  
**Statut :** SpÃ©cification onglet service et mode chatbot

